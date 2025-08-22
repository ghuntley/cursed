const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main CURSED interpreter (only this for validation)
    const exe = b.addExecutable(.{
        .name = "cursed-zig", 
        .target = target,
        .optimize = optimize,
    });
    exe.root_module.addSourceFile(.{ .path = "src-zig/main.zig" });
    exe.linkLibC();
    b.installArtifact(exe);

    const run_step = b.step("run", "Run the CURSED compiler");
    const run_cmd = b.addRunArtifact(exe);
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    run_step.dependOn(&run_cmd.step);
}
