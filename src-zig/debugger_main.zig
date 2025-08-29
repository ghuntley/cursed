//! CURSED Debugger Main
//! Entry point for the interactive debugger tool

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const debugger = @import("debugger.zig");
const interpreter = @import("interpreter.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Parse command line arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    const filename = args[1];
    
    // Initialize interpreter
    var interp = interpreter.Interpreter.init(allocator);
    defer interp.deinit();

    // Initialize debugger
    var debug = debugger.CursedDebugger.init(allocator, &interp) catch |err| {
        print("❌ Failed to initialize debugger: {s}\n", .{err});
        return;
    };
    defer debug.deinit();

    print("🚀 CURSED Interactive Debugger v1.0.0\n", .{});
    print("📂 Loading file: {s}\n", .{filename});

    // Load source file
    debug.loadSourceFile(filename) catch |err| {
        print("❌ Failed to load source file '{s}': {s}\n", .{ filename, err });
        return;
    };

    print("✅ Source file loaded successfully\n", .{});
    print("Type 'help' for available commands, 'run' to start execution\n", .{});

    // Start interactive debugger session
    debug.run() catch |err| {
        print("❌ Debugger error: {s}\n", .{err});
        return;
    };
}

fn printUsage() void {
    print("CURSED Interactive Debugger\n\n", .{});
    print("USAGE:\n", .{});
    print("    cursed-debug <file.csd>\n\n", .{});
    print("FEATURES:\n", .{});
    print("    • Interactive debugging with breakpoints\n", .{});
    print("    • Step-by-step execution\n", .{});
    print("    • Variable inspection and watchpoints\n", .{});
    print("    • Stack trace viewing\n", .{});
    print("    • Source code listing\n\n", .{});
    print("COMMANDS:\n", .{});
    print("    run/r          Start program execution\n", .{});
    print("    step/s         Execute next statement\n", .{});
    print("    next/n         Step over function calls\n", .{});
    print("    continue/c     Continue execution\n", .{});
    print("    break <line>   Set breakpoint at line\n", .{});
    print("    list [line]    Show source code\n", .{});
    print("    print <var>    Print variable value\n", .{});
    print("    watch <var>    Add variable to watch list\n", .{});
    print("    info <topic>   Show information (breakpoints, variables, stack)\n", .{});
    print("    help/h         Show help message\n", .{});
    print("    quit/q         Exit debugger\n\n", .{});
    print("EXAMPLES:\n", .{});
    print("    cursed-debug hello.csd\n", .{});
    print("    cursed-debug fibonacci.csd\n", .{});
}
