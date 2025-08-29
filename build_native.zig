const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Native LLVM compiler
    const native_exe = b.addExecutable(.{
        .name = "cursed-compiler",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/main_llvm_native.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    // Link LLVM
    native_exe.linkSystemLibrary("LLVM");
    native_exe.linkLibC();
    native_exe.linkLibCpp();
    
    // Add LLVM include paths (common locations)
    native_exe.addIncludePath(b.path("/usr/include/llvm-c"));
    native_exe.addIncludePath(b.path("/usr/include/llvm"));
    native_exe.addIncludePath(b.path("/usr/lib/llvm-16/include"));
    native_exe.addIncludePath(b.path("/usr/lib/llvm-17/include"));
    native_exe.addIncludePath(b.path("/usr/lib/llvm-18/include"));
    
    b.installArtifact(native_exe);

    // Legacy cursed-zig alias
    const legacy_exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/main_llvm_native.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    legacy_exe.linkSystemLibrary("LLVM");
    legacy_exe.linkLibC();
    legacy_exe.linkLibCpp();
    
    legacy_exe.addIncludePath(b.path("/usr/include/llvm-c"));
    legacy_exe.addIncludePath(b.path("/usr/include/llvm"));
    legacy_exe.addIncludePath(b.path("/usr/lib/llvm-16/include"));
    legacy_exe.addIncludePath(b.path("/usr/lib/llvm-17/include"));
    legacy_exe.addIncludePath(b.path("/usr/lib/llvm-18/include"));
    
    b.installArtifact(legacy_exe);

    // Run step
    const run_cmd = b.addRunArtifact(native_exe);
    run_cmd.step.dependOn(b.getInstallStep());
    
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the native CURSED compiler");
    run_step.dependOn(&run_cmd.step);
}
