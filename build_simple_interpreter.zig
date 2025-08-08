const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Create simple interpreter without LLVM dependencies
    const exe = b.addExecutable(.{
        .name = "cursed-simple",
        .root_source_file = b.path("src-zig/main_unified.zig"),
        .target = target,
        .optimize = optimize,
    });
    
    exe.linkLibC();
    b.installArtifact(exe);
}
