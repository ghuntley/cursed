const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create the parser test executable
    const exe = b.addExecutable(.{
        .name = "cursed-parser-test",
        .root_source_file = b.path("src-zig/main_parser_test.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(exe);

    // Create a run step
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run-parser", "Run the CURSED parser test");
    run_step.dependOn(&run_cmd.step);
}
