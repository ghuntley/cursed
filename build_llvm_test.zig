const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Test LLVM functionality
    const test_exe = b.addExecutable(.{
        .name = "test-llvm-p0",
        .root_module = b.createModule(.{
            .root_source_file = b.path("test_llvm_p0_validation.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    // Add LLVM C wrapper
    test_exe.addCSourceFile(.{
        .file = b.path("src-zig/llvm_wrapper.c"),
        .flags = &[_][]const u8{"-std=c99", "-I/usr/lib/llvm-18/include"},
    });

    // Link LLVM
    test_exe.linkSystemLibrary("LLVM");
    test_exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib" });
    test_exe.linkLibC();

    b.installArtifact(test_exe);

    // Run step
    const run_cmd = b.addRunArtifact(test_exe);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run LLVM P0 test");
    run_step.dependOn(&run_cmd.step);
}
