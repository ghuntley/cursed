// Tools module for CURSED build system
// Oracle Priority 2: Build System Migration

pub const BuildInfo = struct {
    version: []const u8,
    target: []const u8,
    optimize: []const u8,
    
    pub fn current() BuildInfo {
        return BuildInfo{
            .version = "1.0.0",
            .target = "native",
            .optimize = "debug",
        };
    }
};

pub fn printBuildInfo() void {
    const info = BuildInfo.current();
    std.debug.print("CURSED Build Tools\n", .{});
    std.debug.print("Version: {s}\n", .{info.version});
    std.debug.print("Target: {s}\n", .{info.target});
    std.debug.print("Optimize: {s}\n", .{info.optimize});
}

const std = @import("std");
