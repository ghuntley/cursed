//! Build System Deadlock Prevention and Safe Parallel Execution
//!
//! This module implements proper deadlock prevention, job dependency resolution,
//! and safe parallel execution for the CURSED Zig build system.
//!
//! Key Features:
//! - Deadlock detection and prevention using topological sorting
//! - Safe parallel job scheduling with proper synchronization
//! - Circular dependency detection and resolution
//! - Resource contention management
//! - Thread-safe build step execution

const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Mutex = std.Thread.Mutex;
const Condition = std.Thread.Condition;
const AtomicU32 = std.atomic.Value(u32);
const AtomicBool = std.atomic.Value(bool);

/// Build job states for tracking execution progress
pub const JobState = enum(u8) {
    pending = 0,
    ready = 1,
    running = 2,
    completed = 3,
    failed = 4,
    cancelled = 5,
    waiting_deps = 6,
    blocked = 7,
};

/// Build job priority levels
pub const JobPriority = enum(u8) {
    low = 0,
    normal = 1,
    high = 2,
    critical = 3,
};

/// Build job identifier
pub const JobId = u32;

/// Dependency relationship between jobs
pub const Dependency = struct {
    from: JobId,
    to: JobId,
    dep_type: DependencyType,
};

/// Types of dependencies between build jobs
pub const DependencyType = enum {
    sequential,    // Job B must run after Job A completes
    resource,      // Jobs share a resource and cannot run concurrently
    data,          // Job B needs output from Job A
    weak,          // Job B prefers to run after Job A but can run in parallel
};

/// Build job definition
pub const BuildJob = struct {
    id: JobId,
    name: []const u8,
    state: AtomicU32, // JobState
    priority: JobPriority,
    dependencies: ArrayList(JobId),
    dependents: ArrayList(JobId),
    resource_requirements: []const []const u8, // Resource names this job needs
    estimated_duration_ms: u64,
    actual_start_time: i64,
    actual_end_time: i64,
    worker_id: ?u32,
    retry_count: u32,
    max_retries: u32,
    allocator: Allocator,

    pub fn init(allocator: Allocator, id: JobId, name: []const u8, priority: JobPriority) BuildJob {
        return BuildJob{
            .id = id,
            .name = allocator.dupe(u8, name) catch name,
            .state = AtomicU32.init(@intFromEnum(JobState.pending)),
            .priority = priority,
            .dependencies = .empty,
            .dependents = .empty,
            .resource_requirements = &[_][]const u8{},
            .estimated_duration_ms = 1000, // Default 1 second
            .actual_start_time = 0,
            .actual_end_time = 0,
            .worker_id = null,
            .retry_count = 0,
            .max_retries = 3,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *BuildJob) void {
        self.dependencies.deinit(self.allocator);
        self.dependents.deinit(self.allocator);
        if (self.name.ptr != "unnamed".ptr) {
            self.allocator.free(self.name);
        }
    }

    pub fn getState(self: *const BuildJob) JobState {
        return @enumFromInt(self.state.load(.acquire));
    }

    pub fn setState(self: *BuildJob, new_state: JobState) void {
        self.state.store(@intFromEnum(new_state), .release);
    }

    pub fn addDependency(self: *BuildJob, dep_id: JobId) !void {
        try self.dependencies.append(self.allocator, dep_id);
    }

    pub fn addDependent(self: *BuildJob, dependent_id: JobId) !void {
        try self.dependents.append(self.allocator, dependent_id);
    }
};

/// Resource pool for managing shared build resources
pub const ResourcePool = struct {
    const Resource = struct {
        name: []const u8,
        available: AtomicU32,
        total: u32,
        waiting_jobs: ArrayList(JobId),
        mutex: Mutex,
    };

    resources: HashMap([]const u8, Resource, std.hash_map.StringContext, 80),
    allocator: Allocator,
    mutex: Mutex,

    pub fn init(allocator: Allocator) ResourcePool {
        return ResourcePool{
            .resources = HashMap([]const u8, Resource, std.hash_map.StringContext, 80).init(allocator),
            .allocator = allocator,
            .mutex = Mutex{},
        };
    }

    pub fn deinit(self: *ResourcePool) void {
        var iterator = self.resources.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.waiting_jobs.deinit(self.allocator);
        }
        self.resources.deinit(self.allocator);
    }

    pub fn addResource(self: *ResourcePool, name: []const u8, count: u32) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        const owned_name = try self.allocator.dupe(u8, name);
        try self.resources.put(owned_name, Resource{
            .name = owned_name,
            .available = AtomicU32.init(count),
            .total = count,
            .waiting_jobs = .empty,
            .mutex = Mutex{},
        });
    }

    pub fn acquireResource(self: *ResourcePool, name: []const u8, job_id: JobId) bool {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.resources.getPtr(name)) |resource| {
            resource.mutex.lock();
            defer resource.mutex.unlock();

            const available = resource.available.load(.acquire);
            if (available > 0) {
                resource.available.store(available - 1, .release);
                return true;
            } else {
                resource.waiting_jobs.append(self.allocator, job_id) catch {};
                return false;
            }
        }
        return false; // Resource doesn't exist
    }

    pub fn releaseResource(self: *ResourcePool, name: []const u8) !?JobId {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.resources.getPtr(name)) |resource| {
            resource.mutex.lock();
            defer resource.mutex.unlock();

            const available = resource.available.load(.acquire);
            resource.available.store(available + 1, .release);

            // Wake up waiting job if any
            if (resource.waiting_jobs.items.len > 0) {
                return resource.waiting_jobs.orderedRemove(0);
            }
        }
        return null;
    }
};

/// Deadlock detection using cycle detection in dependency graph
pub const DeadlockDetector = struct {
    jobs: *HashMap(JobId, BuildJob, std.hash_map.AutoContext(JobId), 80),
    allocator: Allocator,

    pub fn init(allocator: Allocator, jobs: *HashMap(JobId, BuildJob, std.hash_map.AutoContext(JobId), 80)) DeadlockDetector {
        return DeadlockDetector{
            .jobs = jobs,
            .allocator = allocator,
        };
    }

    /// Detect circular dependencies using DFS with color marking
    pub fn detectCircularDependencies(self: *DeadlockDetector) !?ArrayList(JobId) {
        var visited = HashMap(JobId, u8, std.hash_map.AutoContext(JobId), 80).init(self.allocator);
        defer visited.deinit(self.allocator);

        var recursion_stack = HashMap(JobId, bool, std.hash_map.AutoContext(JobId), 80).init(self.allocator);
        defer recursion_stack.deinit(self.allocator);

        var path = .empty;
        defer path.deinit(self.allocator);

        var job_iterator = self.jobs.iterator();
        while (job_iterator.next()) |entry| {
            const job_id = entry.key_ptr.*;
            
            if (!visited.contains(job_id)) {
                if (try self.dfsDetectCycle(job_id, &visited, &recursion_stack, &path)) {
                    // Found a cycle, return the path
                    var cycle = .empty;
                    for (path.items) |id| {
                        try cycle.append(self.allocator, id);
                    }
                    return cycle;
                }
            }
        }

        return null; // No cycles found
    }

    fn dfsDetectCycle(
        self: *DeadlockDetector,
        job_id: JobId,
        visited: *HashMap(JobId, u8, std.hash_map.AutoContext(JobId), 80),
        recursion_stack: *HashMap(JobId, bool, std.hash_map.AutoContext(JobId), 80),
        path: *ArrayList(JobId)
    ) !bool {
        try visited.put(job_id, 1);
        try recursion_stack.put(job_id, true);
        try path.append(self.allocator, job_id);

        if (self.jobs.get(job_id)) |job| {
            for (job.dependencies.items) |dep_id| {
                if (!visited.contains(dep_id)) {
                    if (try self.dfsDetectCycle(dep_id, visited, recursion_stack, path)) {
                        return true;
                    }
                } else if (recursion_stack.get(dep_id) orelse false) {
                    // Found back edge - cycle detected
                    return true;
                }
            }
        }

        _ = recursion_stack.remove(job_id);
        _ = path.pop();
        return false;
    }

    /// Perform topological sort to get safe execution order
    pub fn topologicalSort(self: *DeadlockDetector) !ArrayList(JobId) {
        var in_degree = HashMap(JobId, u32, std.hash_map.AutoContext(JobId), 80).init(self.allocator);
        defer in_degree.deinit(self.allocator);

        var queue = .empty;
        defer queue.deinit(self.allocator);

        var result = .empty;

        // Calculate in-degrees
        var job_iterator = self.jobs.iterator();
        while (job_iterator.next()) |entry| {
            const job_id = entry.key_ptr.*;
            try in_degree.put(job_id, 0);
        }

        job_iterator = self.jobs.iterator();
        while (job_iterator.next()) |entry| {
            const job = entry.value_ptr;
            for (job.dependencies.items) |dep_id| {
                if (in_degree.getPtr(dep_id)) |count| {
                    count.* += 1;
                }
            }
        }

        // Find jobs with no dependencies
        var degree_iterator = in_degree.iterator();
        while (degree_iterator.next()) |entry| {
            if (entry.value_ptr.* == 0) {
                try queue.append(self.allocator, entry.key_ptr.*);
            }
        }

        // Process queue
        while (queue.items.len > 0) {
            const current_job_id = queue.orderedRemove(0);
            try result.append(self.allocator, current_job_id);

            if (self.jobs.get(current_job_id)) |current_job| {
                for (current_job.dependents.items) |dependent_id| {
                    if (in_degree.getPtr(dependent_id)) |count| {
                        count.* -= 1;
                        if (count.* == 0) {
                            try queue.append(self.allocator, dependent_id);
                        }
                    }
                }
            }
        }

        // Check if all jobs are included (no cycles)
        if (result.items.len != self.jobs.count()) {
            return error.CircularDependencyDetected;
        }

        return result;
    }
};

/// Thread-safe build scheduler with deadlock prevention
pub const BuildScheduler = struct {
    jobs: HashMap(JobId, BuildJob, std.hash_map.AutoContext(JobId), 80),
    resources: ResourcePool,
    deadlock_detector: DeadlockDetector,
    ready_queue: ArrayList(JobId),
    running_jobs: HashMap(JobId, u32, std.hash_map.AutoContext(JobId), 80), // job_id -> worker_id
    completed_jobs: ArrayList(JobId),
    failed_jobs: ArrayList(JobId),
    
    allocator: Allocator,
    mutex: Mutex,
    condition: Condition,
    
    max_workers: u32,
    active_workers: AtomicU32,
    shutdown: AtomicBool,
    
    next_job_id: AtomicU32,

    pub fn init(allocator: Allocator, max_workers: u32) BuildScheduler {
        var scheduler = BuildScheduler{
            .jobs = HashMap(JobId, BuildJob, std.hash_map.AutoContext(JobId), 80).init(allocator),
            .resources = ResourcePool.init(allocator),
            .deadlock_detector = undefined,
            .ready_queue = .empty,
            .running_jobs = HashMap(JobId, u32, std.hash_map.AutoContext(JobId), 80).init(allocator),
            .completed_jobs = .empty,
            .failed_jobs = .empty,
            .allocator = allocator,
            .mutex = Mutex{},
            .condition = Condition{},
            .max_workers = max_workers,
            .active_workers = AtomicU32.init(0),
            .shutdown = AtomicBool.init(false),
            .next_job_id = AtomicU32.init(1),
        };
        
        scheduler.deadlock_detector = DeadlockDetector.init(allocator, &scheduler.jobs);
        return scheduler;
    }

    pub fn deinit(self: *BuildScheduler) void {
        self.shutdown.store(true, .release);
        
        // Clean up jobs
        var job_iterator = self.jobs.iterator();
        while (job_iterator.next()) |entry| {
            entry.value_ptr.deinit(self.allocator);
        }
        self.jobs.deinit(self.allocator);
        
        self.resources.deinit(self.allocator);
        self.ready_queue.deinit(self.allocator);
        self.running_jobs.deinit(self.allocator);
        self.completed_jobs.deinit(self.allocator);
        self.failed_jobs.deinit(self.allocator);
    }

    /// Add a new build job to the scheduler
    pub fn addJob(self: *BuildScheduler, name: []const u8, priority: JobPriority) !JobId {
        self.mutex.lock();
        defer self.mutex.unlock();

        const job_id = self.next_job_id.fetchAdd(1, .release);
        const job = BuildJob.init(self.allocator, job_id, name, priority);
        
        try self.jobs.put(job_id, job);
        return job_id;
    }

    /// Add dependency between jobs
    pub fn addDependency(self: *BuildScheduler, job_id: JobId, depends_on: JobId) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.jobs.getPtr(job_id)) |job| {
            try job.addDependency(depends_on);
        }
        
        if (self.jobs.getPtr(depends_on)) |dep_job| {
            try dep_job.addDependent(job_id);
        }
    }

    /// Check for deadlocks and prepare execution plan
    pub fn prepareExecution(self: *BuildScheduler) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        // Check for circular dependencies
        if (try self.deadlock_detector.detectCircularDependencies()) |cycle| {
            defer cycle.deinit(self.allocator);
            
            std.debug.print("🚨 Circular dependency detected in build jobs:\n", .{});
            for (cycle.items, 0..) |job_id, i| {
                if (self.jobs.get(job_id)) |job| {
                    std.debug.print("  {d}. Job {d}: {s}\n", .{ i + 1, job_id, job.name });
                }
            }
            return error.CircularDependencyDetected;
        }

        // Get topological order
        const execution_order = try self.deadlock_detector.topologicalSort();
        defer execution_order.deinit(self.allocator);

        std.debug.print("✅ Build execution order validated (no deadlocks):\n", .{});
        for (execution_order.items, 0..) |job_id, i| {
            if (self.jobs.get(job_id)) |job| {
                std.debug.print("  {d}. Job {d}: {s}\n", .{ i + 1, job_id, job.name });
            }
        }

        // Initialize ready queue with jobs that have no dependencies
        for (execution_order.items) |job_id| {
            if (self.jobs.getPtr(job_id)) |job| {
                if (job.dependencies.items.len == 0) {
                    job.setState(JobState.ready);
                    try self.ready_queue.append(self.allocator, job_id);
                } else {
                    job.setState(JobState.waiting_deps);
                }
            }
        }
    }

    /// Get next ready job for execution (thread-safe)
    pub fn getNextJob(self: *BuildScheduler) ?JobId {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.ready_queue.items.len == 0) {
            return null;
        }

        // Sort by priority (critical first)
        std.sort.block(JobId, self.ready_queue.items, self, jobPriorityLessThan);
        
        return self.ready_queue.orderedRemove(0);
    }

    fn jobPriorityLessThan(self: *BuildScheduler, a: JobId, b: JobId) bool {
        const job_a = self.jobs.get(a) orelse return false;
        const job_b = self.jobs.get(b) orelse return true;
        
        return @intFromEnum(job_a.priority) > @intFromEnum(job_b.priority);
    }

    /// Mark job as completed and update dependents
    pub fn completeJob(self: *BuildScheduler, job_id: JobId, worker_id: u32, success: bool) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        if (self.jobs.getPtr(job_id)) |job| {
            job.actual_end_time = @intCast(std.time.nanoTimestamp());
            job.worker_id = worker_id;
            
            if (success) {
                job.setState(JobState.completed);
                try self.completed_jobs.append(self.allocator, job_id);
                
                // Check if dependents are ready
                for (job.dependents.items) |dependent_id| {
                    if (self.jobs.getPtr(dependent_id)) |dependent_job| {
                        if (self.areDependenciesSatisfied(dependent_id)) {
                            dependent_job.setState(JobState.ready);
                            try self.ready_queue.append(self.allocator, dependent_id);
                        }
                    }
                }
            } else {
                if (job.retry_count < job.max_retries) {
                    job.retry_count += 1;
                    job.setState(JobState.ready);
                    try self.ready_queue.append(self.allocator, job_id);
                } else {
                    job.setState(JobState.failed);
                    try self.failed_jobs.append(self.allocator, job_id);
                }
            }
        }

        _ = self.running_jobs.remove(job_id);
        self.condition.broadcast();
    }

    fn areDependenciesSatisfied(self: *BuildScheduler, job_id: JobId) bool {
        if (self.jobs.get(job_id)) |job| {
            for (job.dependencies.items) |dep_id| {
                if (self.jobs.get(dep_id)) |dep_job| {
                    if (dep_job.getState() != JobState.completed) {
                        return false;
                    }
                }
            }
            return true;
        }
        return false;
    }

    /// Wait for all jobs to complete
    pub fn waitForCompletion(self: *BuildScheduler) void {
        while (true) {
            self.mutex.lock();
            const total_jobs = self.jobs.count();
            const completed = self.completed_jobs.items.len + self.failed_jobs.items.len;
            const all_done = completed >= total_jobs;
            
            if (all_done) {
                self.mutex.unlock();
                break;
            }
            
            self.condition.wait(&self.mutex);
            self.mutex.unlock();
        }
    }

    /// Print execution statistics
    pub fn printStatistics(self: *BuildScheduler) void {
        self.mutex.lock();
        defer self.mutex.unlock();

        const total_jobs = self.jobs.count();
        const completed = self.completed_jobs.items.len;
        const failed = self.failed_jobs.items.len;
        
        std.debug.print("\n📊 Build Execution Statistics:\n", .{});
        std.debug.print("  Total jobs: {d}\n", .{total_jobs});
        std.debug.print("  Completed: {d}\n", .{completed});
        std.debug.print("  Failed: {d}\n", .{failed});
        std.debug.print("  Success rate: {d:.1}%\n", .{if (total_jobs > 0) @as(f64, @floatFromInt(completed)) / @as(f64, @floatFromInt(total_jobs)) * 100.0 else 0.0});

        if (failed > 0) {
            std.debug.print("  Failed jobs:\n", .{});
            for (self.failed_jobs.items) |job_id| {
                if (self.jobs.get(job_id)) |job| {
                    std.debug.print("    - Job {d}: {s} (retries: {d})\n", .{ job_id, job.name, job.retry_count });
                }
            }
        }
    }
};

/// Worker thread for executing build jobs
pub const BuildWorker = struct {
    id: u32,
    scheduler: *BuildScheduler,
    thread: ?std.Thread,
    allocator: Allocator,

    pub fn init(allocator: Allocator, id: u32, scheduler: *BuildScheduler) BuildWorker {
        return BuildWorker{
            .id = id,
            .scheduler = scheduler,
            .thread = null,
            .allocator = allocator,
        };
    }

    pub fn start(self: *BuildWorker) !void {
        self.thread = try std.Thread.spawn(.{}, workerLoop, .{self});
    }

    pub fn join(self: *BuildWorker) void {
        if (self.thread) |thread| {
            thread.join();
            self.thread = null;
        }
    }

    fn workerLoop(self: *BuildWorker) void {
        while (!self.scheduler.shutdown.load(.acquire)) {
            if (self.scheduler.getNextJob()) |job_id| {
                self.executeJob(job_id);
            } else {
                // No jobs available, wait a bit
                std.time.sleep(10_000_000); // 10ms
            }
        }
    }

    fn executeJob(self: *BuildWorker, job_id: JobId) void {
        self.scheduler.mutex.lock();
        if (self.scheduler.jobs.getPtr(job_id)) |job| {
            job.setState(JobState.running);
            job.actual_start_time = @intCast(std.time.nanoTimestamp());
            job.worker_id = self.id;
            self.scheduler.running_jobs.put(job_id, self.id) catch {};
        }
        self.scheduler.mutex.unlock();

        // Simulate job execution
        const success = self.simulateJobExecution(job_id);

        // Complete the job
        self.scheduler.completeJob(job_id, self.id, success) catch |err| {
            std.debug.print("⚠️ Error completing job {d}: {}\n", .{ job_id, err });
        };
    }

    fn simulateJobExecution(self: *BuildWorker, job_id: JobId) bool {
        // In a real implementation, this would execute the actual build step
        if (self.scheduler.jobs.get(job_id)) |job| {
            std.debug.print("🔧 Worker {d} executing job {d}: {s}\n", .{ self.id, job_id, job.name });
            
            // Simulate work with random duration
            const duration_ms = job.estimated_duration_ms + (std.crypto.random.int(u32) % 500);
            std.time.sleep(duration_ms * 1_000_000);
            
            // 95% success rate for simulation
            const success = (std.crypto.random.int(u32) % 100) < 95;
            
            if (success) {
                std.debug.print("✅ Worker {d} completed job {d}: {s}\n", .{ self.id, job_id, job.name });
            } else {
                std.debug.print("❌ Worker {d} failed job {d}: {s}\n", .{ self.id, job_id, job.name });
            }
            
            return success;
        }
        return false;
    }
};

// Test and validation functions
test "deadlock detection and prevention" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var scheduler = BuildScheduler.init(allocator, 4);
    defer scheduler.deinit();

    // Create jobs with potential circular dependency
    const job1 = try scheduler.addJob("compile_main", JobPriority.normal);
    const job2 = try scheduler.addJob("compile_deps", JobPriority.normal);
    const job3 = try scheduler.addJob("link_binary", JobPriority.high);

    // Add safe dependencies
    try scheduler.addDependency(job3, job1); // link depends on compile_main
    try scheduler.addDependency(job3, job2); // link depends on compile_deps

    // This should succeed (no circular dependency)
    try scheduler.prepareExecution();

    std.debug.print("✅ Deadlock detection test passed\n", .{});
}

test "circular dependency detection" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var scheduler = BuildScheduler.init(allocator, 4);
    defer scheduler.deinit();

    // Create jobs with circular dependency
    const job1 = try scheduler.addJob("job_a", JobPriority.normal);
    const job2 = try scheduler.addJob("job_b", JobPriority.normal);
    const job3 = try scheduler.addJob("job_c", JobPriority.normal);

    // Create circular dependency: A -> B -> C -> A
    try scheduler.addDependency(job2, job1); // B depends on A
    try scheduler.addDependency(job3, job2); // C depends on B
    try scheduler.addDependency(job1, job3); // A depends on C (creates cycle)

    // This should fail with circular dependency error
    const result = scheduler.prepareExecution();
    try std.testing.expectError(error.CircularDependencyDetected, result);

    std.debug.print("✅ Circular dependency detection test passed\n", .{});
}

test "parallel job execution" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var scheduler = BuildScheduler.init(allocator, 2);
    defer scheduler.deinit();

    // Create independent jobs that can run in parallel
    const job1 = try scheduler.addJob("parallel_task_1", JobPriority.normal);
    const job2 = try scheduler.addJob("parallel_task_2", JobPriority.normal);
    const job3 = try scheduler.addJob("final_task", JobPriority.high);

    // Make final task depend on both parallel tasks
    try scheduler.addDependency(job3, job1);
    try scheduler.addDependency(job3, job2);

    try scheduler.prepareExecution();

    // Create workers
    var worker1 = BuildWorker.init(allocator, 1, &scheduler);
    var worker2 = BuildWorker.init(allocator, 2, &scheduler);

    try worker1.start();
    try worker2.start();

    // Wait for completion
    scheduler.waitForCompletion();

    worker1.join();
    worker2.join();

    scheduler.printStatistics();
    std.debug.print("✅ Parallel execution test completed\n", .{});
}

/// Integration function for build.zig
pub fn optimizeBuildParallelism(b: *std.Build) u32 {
    // Get CPU count for optimal parallelism
    const cpu_count = std.Thread.getCpuCount() catch 4;
    
    // Use CPU count but cap at reasonable limits to prevent resource exhaustion
    const optimal_jobs = if (cpu_count <= 2) cpu_count 
                        else if (cpu_count <= 8) cpu_count 
                        else @min(cpu_count, 12); // Cap at 12 for stability

    // Set environment variable for ninja builds
    if (std.process.getEnvVarOwned(b.allocator, "NINJA_MAX_JOBS")) |ninja_env| {
        defer b.allocator.free(ninja_env);
        // Environment variable already set, no action needed
    } else |_| {
        // Set optimal job count
        const job_count_str = std.fmt.allocPrint(b.allocator, "{d}", .{optimal_jobs}) catch "4";
        defer b.allocator.free(job_count_str);
        
        // Note: std.process.setEnvVar not available, environment should be set externally
    }

    return @intCast(optimal_jobs);
}

/// Validate build system health
pub fn validateBuildSystem(b: *std.Build) !void {
    std.debug.print("🔍 Validating build system health...\n", .{});
    
    // Check for potential issues
    const cpu_count = std.Thread.getCpuCount() catch 4;
    const recommended_jobs = @min(cpu_count, 12);
    
    if (std.process.getEnvVarOwned(b.allocator, "NINJA_MAX_JOBS") catch null) |ninja_env| {
        defer b.allocator.free(ninja_env);
        const current_jobs = std.fmt.parseInt(u32, ninja_env, 10) catch 1;
        
        if (current_jobs > cpu_count * 2) {
            std.debug.print("⚠️ Warning: NINJA_MAX_JOBS ({d}) is very high for {d} CPU cores\n", .{ current_jobs, cpu_count });
            std.debug.print("   Recommended: {d} jobs\n", .{recommended_jobs});
        }
    }
    
    std.debug.print("✅ Build system validation complete\n", .{});
}
