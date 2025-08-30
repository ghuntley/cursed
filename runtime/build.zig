const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Build static library
    const lib = b.addStaticLibrary(.{
        .name = "cursed_stdlib",
        .root_source_file = .{ .path = "cursed_stdlib.zig" },
        .target = target,
        .optimize = optimize,
    });
    
    lib.linkLibC();
    
    b.installArtifact(lib);

    // Also build as shared library
    const shared_lib = b.addSharedLibrary(.{
        .name = "cursed_stdlib",
        .root_source_file = .{ .path = "cursed_stdlib.zig" },
        .target = target,
        .optimize = optimize,
    });
    
    shared_lib.linkLibC();
    
    b.installArtifact(shared_lib);
}
