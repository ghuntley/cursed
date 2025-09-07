const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // CURSED Compiler executable (full implementation)
    const cursed_exe = b.addExecutable(.{
        .name = "cursed-compiler",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/cursed_compiler_main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    cursed_exe.linkLibC();
    
    // No longer need external LLVM libraries - using Zig's built-in LLVM IR builder
    // This enables cross-platform compilation including Windows

    // Install the executable
    b.installArtifact(cursed_exe);

    // Create a run step for the compiler
    const run_cmd = b.addRunArtifact(cursed_exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);
}
