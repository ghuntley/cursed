/// CURSED REPL Standalone Implementation
/// 
/// This provides a complete REPL implementation that can be called from CLI
/// or built as a standalone binary. It integrates with the existing interpreter
/// infrastructure while providing the interactive experience.

const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const repl = @import("repl.zig");
const main_unified = @import("main_unified.zig");

/// Standalone REPL main function
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{
        .stack_trace_frames = 0,
        .enable_memory_limit = false,
        .safety = false,
        .thread_safe = true,
        .never_unmap = false,
        .retain_metadata = false,
        .verbose_log = false,
    }){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Get command line arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // Parse simple flags
    var verbose = false;
    var history_file: ?[]const u8 = null;

    for (args[1..]) |arg| {
        if (std.mem.eql(u8, arg, "--verbose") or std.mem.eql(u8, arg, "-v")) {
            verbose = true;
        } else if (std.mem.startsWith(u8, arg, "--history=")) {
            history_file = arg[10..];
        } else if (std.mem.eql(u8, arg, "--help") or std.mem.eql(u8, arg, "-h")) {
            printReplHelp();
            return;
        } else {
            print("Unknown argument: {s}\n", .{arg});
            print("Use --help for usage information\n", .{});
            return;
        }
    }

    // Run the REPL
    try repl.runReplWithHistory(allocator, verbose, history_file);
}

/// Print REPL usage help
fn printReplHelp() void {
    print("CURSED Interactive REPL\n", .{});
    print("\n", .{});
    print("USAGE:\n", .{});
    print("    cursed-repl [OPTIONS]\n", .{});
    print("\n", .{});
    print("OPTIONS:\n", .{});
    print("    -v, --verbose       Enable verbose output\n", .{});
    print("    --history=<file>    Use custom history file\n", .{});
    print("    -h, --help          Show this help message\n", .{});
    print("\n", .{});
    print("REPL COMMANDS:\n", .{});
    print("    :help               Show REPL help\n", .{});
    print("    :quit, :exit, :q    Exit REPL\n", .{});
    print("    :vars               Show current variables\n", .{});
    print("    :history            Show command history\n", .{});
    print("    :clear              Clear screen\n", .{});
    print("    :version            Show version\n", .{});
    print("\n", .{});
    print("CURSED SYNTAX:\n", .{});
    print("    sus x drip = 42           # Variable declaration\n", .{});
    print("    vibez.spill(\"hello\")      # Print statement\n", .{});
    print("    x + 5                     # Expression evaluation\n", .{});
    print("    slay func() { ... }       # Function definition\n", .{});
    print("    yeet \"module\"             # Module import\n", .{});
    print("\n", .{});
}

/// API function for calling REPL from other modules
pub fn runRepl(allocator: Allocator, verbose: bool) !void {
    try repl.runRepl(allocator, verbose);
}

/// API function for calling REPL with custom history
pub fn runReplWithHistory(allocator: Allocator, verbose: bool, history_file: ?[]const u8) !void {
    try repl.runReplWithHistory(allocator, verbose, history_file);
}
