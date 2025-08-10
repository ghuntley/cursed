//! Simple test for deadlock prevention system
const std = @import("std");
const build_deadlock_prevention = @import("src-zig/build_deadlock_prevention.zig");

test "simple deadlock detection" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var scheduler = build_deadlock_prevention.BuildScheduler.init(allocator, 2);
    defer scheduler.deinit();

    // Create jobs with safe dependencies
    const job1 = try scheduler.addJob("compile_main", build_deadlock_prevention.JobPriority.normal);
    const job2 = try scheduler.addJob("compile_deps", build_deadlock_prevention.JobPriority.normal);
    const job3 = try scheduler.addJob("link_binary", build_deadlock_prevention.JobPriority.high);

    // Add safe dependencies
    try scheduler.addDependency(job3, job1); // link depends on compile_main
    try scheduler.addDependency(job3, job2); // link depends on compile_deps

    // This should succeed (no circular dependency)
    try scheduler.prepareExecution();

    std.debug.print("✅ Basic deadlock detection test passed\n", .{});
}

test "circular dependency detection" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var scheduler = build_deadlock_prevention.BuildScheduler.init(allocator, 2);
    defer scheduler.deinit();

    // Create jobs with circular dependency
    const job1 = try scheduler.addJob("job_a", build_deadlock_prevention.JobPriority.normal);
    const job2 = try scheduler.addJob("job_b", build_deadlock_prevention.JobPriority.normal);
    const job3 = try scheduler.addJob("job_c", build_deadlock_prevention.JobPriority.normal);

    // Create circular dependency: A -> B -> C -> A
    try scheduler.addDependency(job2, job1); // B depends on A
    try scheduler.addDependency(job3, job2); // C depends on B
    try scheduler.addDependency(job1, job3); // A depends on C (creates cycle)

    // This should fail with circular dependency error
    const result = scheduler.prepareExecution();
    try std.testing.expectError(error.CircularDependencyDetected, result);

    std.debug.print("✅ Circular dependency detection test passed\n", .{});
}
