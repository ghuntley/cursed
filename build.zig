const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // CURSED Unified Compiler
    const unified_exe = b.addExecutable(.{
        .name = "cursed-compiler",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/cursed_compiler_main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    b.installArtifact(unified_exe);

    // Main cursed-zig executable (unified compiler)
    const cursed_exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/cursed_compiler_main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    b.installArtifact(cursed_exe);

    // Run step
    const run_cmd = b.addRunArtifact(cursed_exe);
    run_cmd.step.dependOn(b.getInstallStep());
    
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED unified compiler");
    run_step.dependOn(&run_cmd.step);
}
