//! Build configuration for CURSED Error Handling and Concurrency Implementation
//! Provides proper compilation setup for the advanced features

const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Error Handling Library
    const error_handling_lib = b.addStaticLibrary(.{
        .name = "cursed_error_handling",
        .root_source_file = b.path("src-zig/advanced_error_handling.zig"),
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(error_handling_lib);

    // Concurrency Library  
    const concurrency_lib = b.addStaticLibrary(.{
        .name = "cursed_concurrency",
        .root_source_file = b.path("src-zig/advanced_concurrency.zig"),
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(concurrency_lib);

    // Integration Library
    const integration_lib = b.addStaticLibrary(.{
        .name = "cursed_integration",
        .root_source_file = b.path("src-zig/error_concurrency_integration.zig"),
        .target = target,
        .optimize = optimize,
    });
    integration_lib.linkLibrary(error_handling_lib);
    integration_lib.linkLibrary(concurrency_lib);
    b.installArtifact(integration_lib);

    // Test Suite
    const test_exe = b.addExecutable(.{
        .name = "test_error_concurrency",
        .root_source_file = b.path("src-zig/test_error_concurrency.zig"),
        .target = target,
        .optimize = optimize,
    });
    test_exe.linkLibrary(error_handling_lib);
    test_exe.linkLibrary(concurrency_lib);
    test_exe.linkLibrary(integration_lib);
    b.installArtifact(test_exe);

    // Enhanced CURSED Interpreter with Error Handling and Concurrency
    const enhanced_interpreter = b.addExecutable(.{
        .name = "cursed_enhanced",
        .root_source_file = b.path("src-zig/enhanced_main.zig"),
        .target = target,
        .optimize = optimize,
    });
    enhanced_interpreter.linkLibrary(error_handling_lib);
    enhanced_interpreter.linkLibrary(concurrency_lib);
    enhanced_interpreter.linkLibrary(integration_lib);
    enhanced_interpreter.linkLibC();
    b.installArtifact(enhanced_interpreter);

    // Demo Runner
    const demo_exe = b.addExecutable(.{
        .name = "cursed_demo_runner",
        .root_source_file = b.path("src-zig/demo_runner.zig"),
        .target = target,
        .optimize = optimize,
    });
    demo_exe.linkLibrary(integration_lib);
    demo_exe.linkLibrary(error_handling_lib);
    demo_exe.linkLibrary(concurrency_lib);
    demo_exe.linkLibC();
    b.installArtifact(demo_exe);

    // Test step
    const test_step = b.step("test-advanced", "Run advanced error handling and concurrency tests");
    const run_test = b.addRunArtifact(test_exe);
    test_step.dependOn(&run_test.step);

    // Demo step
    const demo_step = b.step("demo-advanced", "Run advanced feature demonstrations");
    const run_demo = b.addRunArtifact(demo_exe);
    demo_step.dependOn(&run_demo.step);

    // Unit tests
    const error_handling_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/advanced_error_handling.zig"),
        .target = target,
        .optimize = optimize,
    });

    const concurrency_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/advanced_concurrency.zig"),
        .target = target,
        .optimize = optimize,
    });

    const integration_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/error_concurrency_integration.zig"),
        .target = target,
        .optimize = optimize,
    });
    integration_tests.linkLibrary(error_handling_lib);
    integration_tests.linkLibrary(concurrency_lib);

    const unit_tests_step = b.step("test-unit", "Run unit tests for advanced features");
    unit_tests_step.dependOn(&b.addRunArtifact(error_handling_tests).step);
    unit_tests_step.dependOn(&b.addRunArtifact(concurrency_tests).step);
    unit_tests_step.dependOn(&b.addRunArtifact(integration_tests).step);
}
