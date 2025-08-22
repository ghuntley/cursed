const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "cursed-minimal",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/minimal_main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    exe.linkLibC();
    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the minimal CURSED interpreter");
    run_step.dependOn(&run_cmd.step);
}
