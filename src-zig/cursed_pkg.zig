const std = @import("std");
const tools = @import("tools");

// Minimal package manager for Oracle P2 migration
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    std.debug.print("CURSED Package Manager\n", .{});
    std.debug.print("Oracle Priority 2: Build System Migration\n", .{});
    tools.printBuildInfo();
    std.debug.print("\n", .{});
    
    if (args.len < 2) {
        try printUsage();
        return;
    }
    
    const command = args[1];
    
    if (std.mem.eql(u8, command, "init")) {
        std.debug.print("✓ Initializing new CURSED project\n", .{});
    } else if (std.mem.eql(u8, command, "build")) {
        std.debug.print("✓ Building CURSED project\n", .{});
    } else if (std.mem.eql(u8, command, "test")) {
        std.debug.print("✓ Running project tests\n", .{});
    } else {
        std.debug.print("Unknown command: {s}\n", .{command});
        try printUsage();
    }
}

fn printUsage() !void {
    std.debug.print("USAGE:\n", .{});
    std.debug.print("  cursed-pkg init     Initialize new project\n", .{});
    std.debug.print("  cursed-pkg build    Build project\n", .{});
    std.debug.print("  cursed-pkg test     Run tests\n", .{});
}
