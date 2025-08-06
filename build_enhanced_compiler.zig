const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Build enhanced compiler without LLVM dependencies
    const enhanced_exe = b.addExecutable(.{
        .name = "cursed-enhanced",
        .root_source_file = .{ .path = "src-zig/cursed_llvm.zig" },
        .target = target,
        .optimize = optimize,
    });
    
    enhanced_exe.linkLibC();
    b.installArtifact(enhanced_exe);
}
