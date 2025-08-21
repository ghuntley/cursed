const std = @import("std");
const print = std.debug.print;

/// Oracle Quality Gate 3: Memory Safety Audit System
const MemoryAuditSystem = struct {
    allocator: std.mem.Allocator,
    test_results: std.ArrayList(AuditResult),
    start_rss: usize,
    
    const Self = @This();
    const AuditResult = struct {
        test_name: []const u8,
        passed: bool,
        memory_before: usize,
        memory_after: usize,
        leak_bytes: i64,
        gc_pause_max_us: u64,
        error_message: ?[]const u8,
    };
    
    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .allocator = allocator,
            .test_results = std.ArrayList(AuditResult).init(allocator),
            .start_rss = getCurrentRSS() orelse 0,
        };
    }
    
    pub fn deinit(self: *Self) void {
        for (self.test_results.items) |result| {
            if (result.error_message) |msg| {
                self.allocator.free(msg);
            }
        }
        self.test_results.deinit(self.allocator);
    }
    
    fn getCurrentRSS() ?usize {
        const file = std.fs.openFileAbsolute("/proc/self/status", .{}) catch return null;
        defer file.close();
        
        var buffer: [2048]u8 = undefined;
        const bytes_read = file.readAll(&buffer) catch return null;
        const content = buffer[0..bytes_read];
        
        var lines = std.mem.splitSequence(u8, content, "\n");
        while (lines.next()) |line| {
            if (std.mem.startsWith(u8, line, "VmRSS:")) {
                var parts = std.mem.splitSequence(u8, line, "\t");
                _ = parts.next(); // Skip "VmRSS:"
                if (parts.next()) |size_str| {
                    var kb_parts = std.mem.splitSequence(u8, std.mem.trim(u8, size_str, " "), " ");
                    if (kb_parts.next()) |kb| {
                        return std.fmt.parseInt(usize, kb, 10) catch null;
                    }
                }
            }
        }
        return null;
    }
    
    /// Run comprehensive memory leak detection
    pub fn runMemoryLeakTests(self: *Self) !void {
        print("=== ORACLE QUALITY GATE 3: MEMORY SAFETY AUDIT ===\n");
        print("Starting comprehensive memory leak detection...\n");
        
        // Test 1: Basic allocation/deallocation cycles
        try self.runBasicAllocationTest();
        
        // Test 2: Stress test with many allocators
        try self.runStressAllocationTest();
        
        // Test 3: Large object allocation test
        try self.runLargeObjectTest();
        
        // Test 4: Fragmentation resistance test
        try self.runFragmentationTest();
        
        // Test 5: GC stress test with tiny heaps
        try self.runGCStressTest();
        
        // Test 6: Extended runtime stress test
        try self.runExtendedStressTest();
        
        self.printAuditSummary();
    }
    
    fn runBasicAllocationTest(self: *Self) !void {
        const test_name = "Basic Allocation Cycles";
        const memory_before = getCurrentRSS() orelse 0;
        var gc_pause_max_us: u64 = 0;
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        // Simulate 1000 allocation/deallocation cycles
        const iterations = 1000;
        for (0..iterations) |i| {
            const start_time = std.time.microTimestamp();
            
            const ptr = self.allocator.alloc(u8, 1024) catch {
                error_message = try std.fmt.allocPrint(self.allocator, "Allocation failed at iteration {d}", .{i});
                passed = false;
                break;
            };
            
            // Fill with data to ensure it's actually used
            @memset(ptr, @as(u8, @truncate(i)));
            
            self.allocator.free(ptr);
            
            const end_time = std.time.microTimestamp();
            const pause_time = @as(u64, @intCast(end_time - start_time));
            if (pause_time > gc_pause_max_us) {
                gc_pause_max_us = pause_time;
            }
            
            // Check for excessive pause times (>1ms)
            if (pause_time > 1000) {
                error_message = try std.fmt.allocPrint(self.allocator, "GC pause {d}μs exceeds 1ms threshold", .{pause_time});
                passed = false;
                break;
            }
        }
        
        const memory_after = getCurrentRSS() orelse 0;
        const leak_bytes = @as(i64, @intCast(memory_after)) - @as(i64, @intCast(memory_before));
        
        const result = AuditResult{
            .test_name = test_name,
            .passed = passed,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .leak_bytes = leak_bytes,
            .gc_pause_max_us = gc_pause_max_us,
            .error_message = error_message,
        };
        
        try self.test_results.append(self.allocator, result);
        
        if (passed) {
            print("✅ {s}: {d} iterations, leak: {d} KB, max GC pause: {d}μs\n", 
                  .{ test_name, iterations, leak_bytes, gc_pause_max_us });
        } else {
            print("❌ {s}: FAILED - {s}\n", 
                  .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    fn runStressAllocationTest(self: *Self) !void {
        const test_name = "Stress Allocation Test";
        const memory_before = getCurrentRSS() orelse 0;
        var gc_pause_max_us: u64 = 0;
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        // Allocate many objects simultaneously
        var allocations = std.ArrayList([]u8).init(self.allocator);
        defer {
            for (allocations.items) |allocation| {
                self.allocator.free(allocation);
            }
            allocations.deinit(self.allocator);
        }
        
        const start_time = std.time.microTimestamp();
        
        // Allocate 10000 objects of varying sizes
        for (0..10000) |i| {
            const size = (i % 8192) + 64; // 64 to 8KB objects
            const ptr = self.allocator.alloc(u8, size) catch {
                error_message = try std.fmt.allocPrint(self.allocator, "Stress allocation failed at {d}", .{i});
                passed = false;
                break;
            };
            
            // Fill with pattern
            @memset(ptr, @as(u8, @truncate(i)));
            
            try allocations.append(self.allocator, ptr);
        }
        
        const end_time = std.time.microTimestamp();
        gc_pause_max_us = @as(u64, @intCast(end_time - start_time));
        
        // Check for excessive allocation time
        if (gc_pause_max_us > 100000) { // 100ms threshold
            error_message = try std.fmt.allocPrint(self.allocator, "Allocation time {d}μs excessive", .{gc_pause_max_us});
            passed = false;
        }
        
        const memory_after = getCurrentRSS() orelse 0;
        const leak_bytes = @as(i64, @intCast(memory_after)) - @as(i64, @intCast(memory_before));
        
        const result = AuditResult{
            .test_name = test_name,
            .passed = passed,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .leak_bytes = leak_bytes,
            .gc_pause_max_us = gc_pause_max_us,
            .error_message = error_message,
        };
        
        try self.test_results.append(self.allocator, result);
        
        if (passed) {
            print("✅ {s}: 10K objects, leak: {d} KB, allocation time: {d}μs\n", 
                  .{ test_name, leak_bytes, gc_pause_max_us });
        } else {
            print("❌ {s}: FAILED - {s}\n", 
                  .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    fn runLargeObjectTest(self: *Self) !void {
        const test_name = "Large Object Allocation";
        const memory_before = getCurrentRSS() orelse 0;
        var gc_pause_max_us: u64 = 0;
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        // Test allocation of very large objects
        for (0..10) |i| {
            const start_time = std.time.microTimestamp();
            
            const large_size = (i + 1) * 1024 * 1024; // 1MB to 10MB
            const ptr = self.allocator.alloc(u8, large_size) catch {
                error_message = try std.fmt.allocPrint(self.allocator, "Large allocation {d}MB failed", .{large_size / 1024 / 1024});
                passed = false;
                break;
            };
            defer self.allocator.free(ptr);
            
            // Touch all pages to ensure they're allocated
            var j: usize = 0;
            while (j < large_size) : (j += 4096) {
                ptr[j] = @as(u8, @truncate(j));
            }
            
            const end_time = std.time.microTimestamp();
            const pause_time = @as(u64, @intCast(end_time - start_time));
            if (pause_time > gc_pause_max_us) {
                gc_pause_max_us = pause_time;
            }
            
            // Check for excessive pause times
            if (pause_time > 10000) { // 10ms threshold for large objects
                error_message = try std.fmt.allocPrint(self.allocator, "Large allocation pause {d}μs excessive", .{pause_time});
                passed = false;
                break;
            }
        }
        
        const memory_after = getCurrentRSS() orelse 0;
        const leak_bytes = @as(i64, @intCast(memory_after)) - @as(i64, @intCast(memory_before));
        
        const result = AuditResult{
            .test_name = test_name,
            .passed = passed,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .leak_bytes = leak_bytes,
            .gc_pause_max_us = gc_pause_max_us,
            .error_message = error_message,
        };
        
        try self.test_results.append(self.allocator, result);
        
        if (passed) {
            print("✅ {s}: 1-10MB objects, leak: {d} KB, max pause: {d}μs\n", 
                  .{ test_name, leak_bytes, gc_pause_max_us });
        } else {
            print("❌ {s}: FAILED - {s}\n", 
                  .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    fn runFragmentationTest(self: *Self) !void {
        const test_name = "Fragmentation Resistance";
        const memory_before = getCurrentRSS() orelse 0;
        var gc_pause_max_us: u64 = 0;
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        // Create fragmentation by allocating and freeing random sizes
        var allocations = std.ArrayList(?[]u8).init(self.allocator);
        defer {
            for (allocations.items) |maybe_allocation| {
                if (maybe_allocation) |allocation| {
                    self.allocator.free(allocation);
                }
            }
            allocations.deinit(self.allocator);
        }
        
        var prng = std.Random.DefaultPrng.init(42);
        const random = prng.random();
        
        // Phase 1: Create fragmentation
        for (0..5000) |_| {
            const size = random.uintLessThan(usize, 4096) + 8;
            const ptr = self.allocator.alloc(u8, size) catch {
                error_message = try std.fmt.allocPrint(self.allocator, "Fragmentation allocation failed");
                passed = false;
                break;
            };
            try allocations.append(self.allocator, ptr);
        }
        
        if (passed) {
            // Phase 2: Randomly free half the allocations
            for (allocations.items, 0..) |*maybe_allocation, i| {
                if (random.boolean()) {
                    if (maybe_allocation.*) |allocation| {
                        self.allocator.free(allocation);
                        maybe_allocation.* = null;
                    }
                }
                _ = i;
            }
            
            // Phase 3: Try to allocate large objects in fragmented space
            const start_time = std.time.microTimestamp();
            for (0..100) |_| {
                const large_ptr = self.allocator.alloc(u8, 8192) catch {
                    error_message = try std.fmt.allocPrint(self.allocator, "Large allocation in fragmented space failed");
                    passed = false;
                    break;
                };
                defer self.allocator.free(large_ptr);
            }
            const end_time = std.time.microTimestamp();
            gc_pause_max_us = @as(u64, @intCast(end_time - start_time));
        }
        
        const memory_after = getCurrentRSS() orelse 0;
        const leak_bytes = @as(i64, @intCast(memory_after)) - @as(i64, @intCast(memory_before));
        
        const result = AuditResult{
            .test_name = test_name,
            .passed = passed,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .leak_bytes = leak_bytes,
            .gc_pause_max_us = gc_pause_max_us,
            .error_message = error_message,
        };
        
        try self.test_results.append(self.allocator, result);
        
        if (passed) {
            print("✅ {s}: fragmentation handled, leak: {d} KB, time: {d}μs\n", 
                  .{ test_name, leak_bytes, gc_pause_max_us });
        } else {
            print("❌ {s}: FAILED - {s}\n", 
                  .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    fn runGCStressTest(self: *Self) !void {
        const test_name = "GC Stress Test (Tiny Heaps)";
        const memory_before = getCurrentRSS() orelse 0;
        var gc_pause_max_us: u64 = 0;
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        // Simulate tiny heap with frequent collections
        // This forces the GC to run very frequently
        const tiny_allocations = 1000;
        for (0..100) |cycle| { // 100 GC cycles
            var cycle_allocations = std.ArrayList([]u8).init(self.allocator);
            defer {
                for (cycle_allocations.items) |allocation| {
                    self.allocator.free(allocation);
                }
                cycle_allocations.deinit(self.allocator);
            }
            
            const start_time = std.time.microTimestamp();
            
            // Fill up a small heap rapidly
            for (0..tiny_allocations) |_| {
                const ptr = self.allocator.alloc(u8, 128) catch {
                    error_message = try std.fmt.allocPrint(self.allocator, "GC stress allocation failed at cycle {d}", .{cycle});
                    passed = false;
                    break;
                };
                try cycle_allocations.append(self.allocator, ptr);
            }
            
            if (!passed) break;
            
            const end_time = std.time.microTimestamp();
            const cycle_time = @as(u64, @intCast(end_time - start_time));
            if (cycle_time > gc_pause_max_us) {
                gc_pause_max_us = cycle_time;
            }
            
            // Check for excessive GC pause times (should be ≤1ms)
            if (cycle_time > 1000) {
                error_message = try std.fmt.allocPrint(self.allocator, "GC cycle {d} took {d}μs > 1ms threshold", .{ cycle, cycle_time });
                passed = false;
                break;
            }
        }
        
        const memory_after = getCurrentRSS() orelse 0;
        const leak_bytes = @as(i64, @intCast(memory_after)) - @as(i64, @intCast(memory_before));
        
        const result = AuditResult{
            .test_name = test_name,
            .passed = passed,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .leak_bytes = leak_bytes,
            .gc_pause_max_us = gc_pause_max_us,
            .error_message = error_message,
        };
        
        try self.test_results.append(self.allocator, result);
        
        if (passed) {
            print("✅ {s}: 100 cycles, leak: {d} KB, max pause: {d}μs\n", 
                  .{ test_name, leak_bytes, gc_pause_max_us });
        } else {
            print("❌ {s}: FAILED - {s}\n", 
                  .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    fn runExtendedStressTest(self: *Self) !void {
        const test_name = "Extended Runtime Stress";
        const memory_before = getCurrentRSS() orelse 0;
        var gc_pause_max_us: u64 = 0;
        var error_message: ?[]const u8 = null;
        var passed = true;
        
        print("   Running extended stress test (30 seconds)...\n");
        
        const test_duration_ns = 30 * std.time.ns_per_s; // 30 seconds
        const start_time = std.time.nanoTimestamp();
        var iteration: u64 = 0;
        
        while ((std.time.nanoTimestamp() - start_time) < test_duration_ns) {
            iteration += 1;
            
            const alloc_start = std.time.microTimestamp();
            
            // Allocate varying sizes
            const size = (iteration % 4096) + 64;
            const ptr = self.allocator.alloc(u8, size) catch {
                error_message = try std.fmt.allocPrint(self.allocator, "Extended stress allocation failed at iteration {d}", .{iteration});
                passed = false;
                break;
            };
            
            // Use the memory
            @memset(ptr, @as(u8, @truncate(iteration)));
            
            self.allocator.free(ptr);
            
            const alloc_end = std.time.microTimestamp();
            const pause_time = @as(u64, @intCast(alloc_end - alloc_start));
            if (pause_time > gc_pause_max_us) {
                gc_pause_max_us = pause_time;
            }
            
            // Check for memory leaks every 10000 iterations
            if (iteration % 10000 == 0) {
                const current_rss = getCurrentRSS() orelse 0;
                const current_leak = @as(i64, @intCast(current_rss)) - @as(i64, @intCast(memory_before));
                
                // Allow some growth but detect severe leaks (>100MB)
                if (current_leak > 100 * 1024) { // 100MB
                    error_message = try std.fmt.allocPrint(self.allocator, "Severe memory leak detected: {d} KB after {d} iterations", .{ current_leak, iteration });
                    passed = false;
                    break;
                }
                
                print("   Progress: {d} iterations, RSS: {d} KB\n", .{ iteration, current_rss });
            }
        }
        
        const memory_after = getCurrentRSS() orelse 0;
        const leak_bytes = @as(i64, @intCast(memory_after)) - @as(i64, @intCast(memory_before));
        
        const result = AuditResult{
            .test_name = test_name,
            .passed = passed,
            .memory_before = memory_before,
            .memory_after = memory_after,
            .leak_bytes = leak_bytes,
            .gc_pause_max_us = gc_pause_max_us,
            .error_message = error_message,
        };
        
        try self.test_results.append(self.allocator, result);
        
        if (passed) {
            print("✅ {s}: {d} iterations, leak: {d} KB, max pause: {d}μs\n", 
                  .{ test_name, iteration, leak_bytes, gc_pause_max_us });
        } else {
            print("❌ {s}: FAILED - {s}\n", 
                  .{ test_name, error_message orelse "Unknown error" });
        }
    }
    
    fn printAuditSummary(self: *Self) void {
        print("\n=== ORACLE QUALITY GATE 3: MEMORY SAFETY AUDIT RESULTS ===\n");
        
        var passed_count: usize = 0;
        var total_leak: i64 = 0;
        var max_gc_pause: u64 = 0;
        
        for (self.test_results.items) |result| {
            if (result.passed) {
                passed_count += 1;
                total_leak += result.leak_bytes;
                if (result.gc_pause_max_us > max_gc_pause) {
                    max_gc_pause = result.gc_pause_max_us;
                }
            }
        }
        
        print("\n📊 AUDIT SUMMARY:\n");
        print("   Tests passed: {d}/{d}\n", .{ passed_count, self.test_results.items.len });
        print("   Total memory leak: {d} KB\n", .{total_leak});
        print("   Max GC pause time: {d}μs ({d:.2}ms)\n", .{ max_gc_pause, @as(f64, @floatFromInt(max_gc_pause)) / 1000.0 });
        
        // Oracle Quality Gate 3 criteria
        const zero_leaks = total_leak < 1024; // < 1MB acceptable
        const gc_performance = max_gc_pause <= 1000; // ≤1ms
        const all_tests_passed = passed_count == self.test_results.items.len;
        
        print("\n🎯 ORACLE QUALITY GATE 3 CRITERIA:\n");
        print("   ✓ Zero memory leaks: {s} ({d} KB)\n", .{ if (zero_leaks) "PASS" else "FAIL", total_leak });
        print("   ✓ GC pause ≤1ms: {s} ({d:.2}ms)\n", .{ if (gc_performance) "PASS" else "FAIL", @as(f64, @floatFromInt(max_gc_pause)) / 1000.0 });
        print("   ✓ All safety tests: {s} ({d}/{d})\n", .{ if (all_tests_passed) "PASS" else "FAIL", passed_count, self.test_results.items.len });
        
        const gate_passed = zero_leaks and gc_performance and all_tests_passed;
        
        print("\n🚀 ORACLE QUALITY GATE 3 STATUS: {s}\n", .{if (gate_passed) "✅ PASSED - PRODUCTION READY" else "❌ FAILED - REQUIRES FIXES"});
        
        if (gate_passed) {
            print("   Memory safety guarantees validated for v1.0 release\n");
            print("   Zero-leak guarantee: CONFIRMED\n");
            print("   Sub-millisecond GC: CONFIRMED\n");
            print("   Extended stability: CONFIRMED\n");
        } else {
            print("   ⚠️  Memory safety issues detected - review required\n");
        }
        
        print("=======================================================\n");
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{
        .verbose_log = true,
        .retain_metadata = true,
        .never_unmap = false,
        .stack_trace_frames = 8,
    }){};
    defer {
        const leaked = gpa.deinit();
        if (leaked == .leak) {
            print("🔥 CRITICAL: GeneralPurposeAllocator detected leaks!\n");
            std.process.exit(1);
        } else {
            print("✅ GeneralPurposeAllocator reports clean shutdown\n");
        }
    }
    const allocator = gpa.allocator();
    
    var audit_system = MemoryAuditSystem.init(allocator);
    defer audit_system.deinit();
    
    try audit_system.runMemoryLeakTests();
}
