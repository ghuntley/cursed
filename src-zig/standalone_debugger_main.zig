//! CURSED Standalone Debugger Main Entry Point
//!
//! Production-ready command-line debugger for CURSED programs.
//! Provides interactive debugging with breakpoints, step execution,
//! variable inspection, and expression evaluation.

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const debugger = @import("debugger.zig");
const interpreter = @import("interpreter.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

/// Main debugger application entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Parse command line arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("🐛 CURSED Interactive Debugger v1.0\n", .{});
        print("Usage: cursed-debug <program.csd> [options]\n", .{});
        print("\nOptions:\n", .{});
        print("  --help, -h     Show this help message\n", .{});
        print("  --version, -v  Show version information\n", .{});
        print("  --verbose      Enable verbose debugging output\n", .{});
        print("\nExamples:\n", .{});
        print("  cursed-debug myprogram.csd\n", .{});
        print("  cursed-debug --verbose complex_program.csd\n", .{});
        return;
    }

    // Handle special options
    if (std.mem.eql(u8, args[1], "--help") or std.mem.eql(u8, args[1], "-h")) {
        printHelp();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version") or std.mem.eql(u8, args[1], "-v")) {
        print("CURSED Interactive Debugger v1.0.0-beta\n", .{});
        print("Part of CURSED Language Ecosystem\n", .{});
        print("Built with Zig {s}\n", .{@import("builtin").zig_version_string});
        return;
    }

    const source_file = args[1];
    var verbose_mode = false;

    // Parse additional options
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--verbose")) {
            verbose_mode = true;
        }
    }

    // Verify source file exists
    std.fs.cwd().access(source_file, .{}) catch |err| switch (err) {
        error.FileNotFound => {
            print("❌ Error: File '{s}' not found\n", .{source_file});
            return;
        },
        else => {
            print("❌ Error: Cannot access file '{s}': {}\n", .{ source_file, err });
            return;
        },
    };

    // Initialize interpreter for debugging
    var interp = interpreter.Interpreter.init(allocator);
    defer interp.deinit();

    // Initialize debugger
    var cursed_debugger = try debugger.CursedDebugger.init(allocator, &interp);
    defer cursed_debugger.deinit();

    if (verbose_mode) {
        print("🔧 Verbose mode enabled\n", .{});
        cursed_debugger.config.verbose_mode = true;
    }

    // Start debugging session
    try cursed_debugger.startSession(source_file);
}

/// Print comprehensive help information
fn printHelp() void {
    print("🐛 CURSED Interactive Debugger v1.0.0-beta\n", .{});
    print("=====================================\n\n", .{});
    
    print("DESCRIPTION:\n", .{});
    print("  Interactive debugger for CURSED programming language.\n", .{});
    print("  Supports breakpoints, step execution, variable inspection,\n", .{});
    print("  expression evaluation, and stack trace analysis.\n\n", .{});
    
    print("USAGE:\n", .{});
    print("  cursed-debug <program.csd> [options]\n\n", .{});
    
    print("OPTIONS:\n", .{});
    print("  --help, -h     Show this help message\n", .{});
    print("  --version, -v  Show version information\n", .{});
    print("  --verbose      Enable verbose debugging output\n\n", .{});
    
    print("DEBUGGER COMMANDS:\n", .{});
    print("  help, h                 - Show debugger command help\n", .{});
    print("  run, r [args]           - Run the program\n", .{});
    print("  break, b <location>     - Set breakpoint (line number or function)\n", .{});
    print("  continue, c             - Continue execution\n", .{});
    print("  step, s                 - Step into (single statement)\n", .{});
    print("  next, n                 - Step over (next statement)\n", .{});
    print("  finish, f               - Step out (finish current function)\n", .{});
    print("  print, p <variable>     - Print variable value\n", .{});
    print("  watch, w <variable>     - Watch variable for changes\n", .{});
    print("  backtrace, bt           - Show stack trace\n", .{});
    print("  list, l [line]          - List source code\n", .{});
    print("  info, i <topic>         - Show information (breakpoints, variables)\n", .{});
    print("  delete, d <bp_id>       - Delete breakpoint\n", .{});
    print("  enable <bp_id>          - Enable breakpoint\n", .{});
    print("  disable <bp_id>         - Disable breakpoint\n", .{});
    print("  set <var> <value>       - Set variable value\n", .{});
    print("  eval <expression>       - Evaluate expression\n", .{});
    print("  quit, q                 - Exit debugger\n\n", .{});
    
    print("EXAMPLES:\n", .{});
    print("  # Debug a simple program\n", .{});
    print("  cursed-debug hello.csd\n\n", .{});
    
    print("  # Debug with verbose output\n", .{});
    print("  cursed-debug --verbose complex_program.csd\n\n", .{});
    
    print("  # Interactive debugging session\n", .{});
    print("  cursed-debug myapp.csd\n", .{});
    print("  (cursed-debug) break 10\n", .{});
    print("  (cursed-debug) run\n", .{});
    print("  (cursed-debug) print my_variable\n", .{});
    print("  (cursed-debug) continue\n\n", .{});
    
    print("BREAKPOINT SYNTAX:\n", .{});
    print("  break 42              - Set breakpoint at line 42\n", .{});
    print("  break main            - Set breakpoint at function 'main'\n", .{});
    print("  break process_data    - Set breakpoint at function 'process_data'\n\n", .{});
    
    print("VARIABLE INSPECTION:\n", .{});
    print("  print x               - Print value of variable 'x'\n", .{});
    print("  watch data_array      - Watch 'data_array' for changes\n", .{});
    print("  set counter 100       - Set 'counter' variable to 100\n\n", .{});
    
    print("EXPRESSION EVALUATION:\n", .{});
    print("  eval x + y            - Evaluate expression 'x + y'\n", .{});
    print("  eval len(my_array)    - Evaluate function call\n\n", .{});
    
    print("For more information, visit: https://cursedlang.org/docs/debugging\n", .{});
}

test "debugger main initialization" {
    const testing = std.testing;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test interpreter initialization
    var interp = try interpreter.Interpreter.init(allocator);
    defer interp.deinit();
    
    // Test debugger initialization
    var cursed_debugger = try debugger.CursedDebugger.init(allocator, &interp);
    defer cursed_debugger.deinit();
    
    try testing.expect(!cursed_debugger.is_running);
    try testing.expect(!cursed_debugger.is_paused);
}
