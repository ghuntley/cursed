const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create main executable only
    const exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/main_ast_enabled.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    // Try standard system library linking for native builds
    exe.linkLibC();
    exe.linkSystemLibrary("m");
    exe.linkSystemLibrary("pthread");

    b.installArtifact(exe);

    // Run command
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);
}
