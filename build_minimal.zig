const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "cursed-zig",
        .target = target,
        .optimize = optimize,
    });
    
    exe.root_module.addSourceFile(.{ .path = "src-zig/main_simple.zig" });
    exe.linkLibC();

    b.installArtifact(exe);
    
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED interpreter");
    run_step.dependOn(&run_cmd.step);
}
