const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Create test executable with minimal LLVM
    const test_exe = b.addExecutable(.{
        .name = "test-llvm-minimal",
        .root_source_file = b.path("src-zig/test_llvm_minimal.zig"),
        .target = target,
        .optimize = optimize,
    });
    
    // Add LLVM includes and libraries with CPU override
    test_exe.addSystemIncludePath(.{ .cwd_relative = "src-zig" });
    test_exe.addSystemIncludePath(.{ .cwd_relative = "/usr/lib/llvm-18/include" });
    test_exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib" });
    
    // Force x86-64 target to avoid athlon-xp CPU detection
    test_exe.root_module.addCMacro("LLVM_DEFAULT_TARGET_TRIPLE", "\"x86_64-unknown-linux-gnu\"");
    test_exe.root_module.addCMacro("LLVM_HOST_TARGET", "\"x86-64\"");
    
    test_exe.linkLibC();
    test_exe.linkSystemLibrary("LLVM-18");
    
    b.installArtifact(test_exe);
    
    // Create run step
    const run_cmd = b.addRunArtifact(test_exe);
    const run_step = b.step("test-llvm", "Test LLVM minimal imports");
    run_step.dependOn(&run_cmd.step);
}
