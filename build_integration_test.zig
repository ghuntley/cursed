const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Integration test executable
    const integration_exe = b.addExecutable(.{
        .name = "test-p0-integration", 
        .root_module = b.createModule(.{
            .root_source_file = b.path("test_p0_integration.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    // Add LLVM C wrapper
    integration_exe.addCSourceFile(.{
        .file = b.path("src-zig/llvm_wrapper.c"),
        .flags = &[_][]const u8{"-std=c99", "-I/usr/lib/llvm-18/include"},
    });

    // Link LLVM
    integration_exe.linkSystemLibrary("LLVM");
    integration_exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib" });
    integration_exe.linkLibC();

    b.installArtifact(integration_exe);

    // Run step
    const run_cmd = b.addRunArtifact(integration_exe);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run integration test");
    run_step.dependOn(&run_cmd.step);
}
