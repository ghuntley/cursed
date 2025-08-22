const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Simple executable for testing plugin system
    const exe = b.addExecutable(.{
        .name = "cursed-plugin-test",
        .root_source_file = b.path("src-zig/plugin_test_main.zig"),
        .target = target,
        .optimize = optimize,
    });

    exe.linkLibC();

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the plugin test application");
    run_step.dependOn(&run_cmd.step);
}
