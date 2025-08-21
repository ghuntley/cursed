const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Check what fields are available in ExecutableOptions
    const exe = b.addExecutable(.{
        .name = "test",
        // Let's see what fields are actually available
    });
    
    _ = exe;
    _ = target;
    _ = optimize;
}
