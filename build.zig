const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Use native target for dynamic library compatibility
    const resolved_target = target;

    // Create the CURSED compiler executable with concurrency support
    const exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_source_file = b.path("src-zig/demo_simple.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    // Configure libc and system integration
    exe.linkLibC();
    
    // Add LLVM library path and includes from NixOS environment
    exe.addLibraryPath(.{ .cwd_relative = "/nix/store/rxp13pg5iidpmvlvy963n8nkkbc246iz-llvm-18.1.8-lib/lib" });
    exe.addIncludePath(.{ .cwd_relative = "/nix/store/19gmdqq62x11wv7ipni6grm5f8clcq7c-llvm-18.1.8-dev/include" });
    
    exe.linkSystemLibrary("LLVM-18");

    b.installArtifact(exe);

    // Create run step
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);

    // Create test suite
    const unit_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/main_complete.zig"),
        .target = target,
        .optimize = optimize,
    });

    unit_tests.linkLibC();
    unit_tests.linkSystemLibrary("LLVM-18");
    unit_tests.addLibraryPath(.{ .cwd_relative = "/nix/store/rxp13pg5iidpmvlvy963n8nkkbc246iz-llvm-18.1.8-lib/lib" });
    unit_tests.addIncludePath(.{ .cwd_relative = "/nix/store/19gmdqq62x11wv7ipni6grm5f8clcq7c-llvm-18.1.8-dev/include" });

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);

    // Create concurrency test suite
    const concurrency_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/concurrency.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_concurrency_tests = b.addRunArtifact(concurrency_tests);
    const concurrency_test_step = b.step("test-concurrency", "Run concurrency tests");
    concurrency_test_step.dependOn(&run_concurrency_tests.step);

    // Create concurrency benchmark executable
    const concurrency_benchmark = b.addExecutable(.{
        .name = "cursed-concurrency-benchmark",
        .root_source_file = b.path("src-zig/concurrency_benchmark.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(concurrency_benchmark);

    const run_benchmark = b.addRunArtifact(concurrency_benchmark);
    run_benchmark.step.dependOn(b.getInstallStep());

    const benchmark_step = b.step("benchmark", "Run concurrency benchmarks");
    benchmark_step.dependOn(&run_benchmark.step);

    // Create comprehensive concurrency test executable
    const concurrency_test_exe = b.addExecutable(.{
        .name = "cursed-concurrency-test",
        .root_source_file = b.path("src-zig/concurrency_test.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(concurrency_test_exe);

    const run_concurrency_test_exe = b.addRunArtifact(concurrency_test_exe);
    run_concurrency_test_exe.step.dependOn(b.getInstallStep());

    const concurrency_full_test_step = b.step("test-concurrency-full", "Run comprehensive concurrency tests");
    concurrency_full_test_step.dependOn(&run_concurrency_test_exe.step);

    // Create stdlib tests
    const stdlib_tests = b.addTest(.{
        .root_source_file = b.path("stdlib-zig/testz.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_stdlib_tests = b.addRunArtifact(stdlib_tests);
    const stdlib_test_step = b.step("test-stdlib", "Run stdlib tests");
    stdlib_test_step.dependOn(&run_stdlib_tests.step);
}
