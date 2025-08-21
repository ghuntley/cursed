const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Build cursed-zig without LLVM for interpreter mode
    const cursed_exe = b.addExecutable(.{
        .name = "cursed-zig-interpreter",
        .root_source_file = b.path("src-zig/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Don't link LLVM for interpreter-only mode
    cursed_exe.linkLibC();

    b.installArtifact(cursed_exe);

    // Also build a minimal version
    const cursed_minimal = b.addExecutable(.{
        .name = "cursed-minimal",
        .root_source_file = b.path("src-zig/main.zig"),
        .target = target,
        .optimize = .Debug,
    });

    cursed_minimal.linkLibC();
    b.installArtifact(cursed_minimal);
}
