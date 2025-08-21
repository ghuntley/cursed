const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create modules
    const cursed_main_module = b.addModule("cursed_main", .{
        .root_source_file = b.path("src-zig/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Create minimal interpreter-only build for RC
    const cursed_interpreter = b.addExecutable(.{
        .name = "cursed-interpreter",
        .root_module = cursed_main_module,
    });
    
    cursed_interpreter.linkLibC();
    b.installArtifact(cursed_interpreter);
}
