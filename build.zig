const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Native LLVM compiler
    const native_exe = b.addExecutable(.{
        .name = "cursed-compiler",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/simple_llvm_compiler.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    // No LLVM linking needed for simple version
    
    b.installArtifact(native_exe);

    // Legacy cursed-zig alias
    const legacy_exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/simple_llvm_compiler.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    // No LLVM linking needed for simple version
    
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
