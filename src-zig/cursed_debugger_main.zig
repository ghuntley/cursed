//! CURSED Interactive Debugger Main Entry Point
//!
//! Standalone executable for debugging CURSED programs with full interactive capabilities.

const std = @import("std");
const print = std.debug.print;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const debug_integration = @import("debug_integration.zig");

/// Command line options for the debugger
const DebuggerOptions = struct {
    source_file: []const u8,
    interactive: bool = true,
    auto_run: bool = false,
    initial_breakpoints: std.ArrayList(u32),
    script_file: ?[]const u8 = null,
    verbose: bool = false,
    
    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .source_file = "",
            .initial_breakpoints = .{},
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.initial_breakpoints.deinit(self.allocator);
    }
};

/// Parse command line arguments
fn parseArguments(allocator: std.mem.Allocator, args: [][:0]u8) !DebuggerOptions {
    var options = DebuggerOptions.init(allocator);
    
    if (args.len < 2) {
        printUsage(args[0]);
        return error.InvalidArguments;
    }
    
    options.source_file = args[1];
    
    var i: usize = 2;
    while (i < args.len) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--help") or std.mem.eql(u8, arg, "-h")) {
            printUsage(args[0]);
            return error.HelpRequested;
        } else if (std.mem.eql(u8, arg, "--interactive") or std.mem.eql(u8, arg, "-i")) {
            options.interactive = true;
        } else if (std.mem.eql(u8, arg, "--auto-run") or std.mem.eql(u8, arg, "-r")) {
            options.auto_run = true;
        } else if (std.mem.eql(u8, arg, "--verbose") or std.mem.eql(u8, arg, "-v")) {
            options.verbose = true;
        } else if (std.mem.eql(u8, arg, "--breakpoint") or std.mem.eql(u8, arg, "-b")) {
            if (i + 1 >= args.len) {
                print("❌ Error: --breakpoint requires a line number\n", .{});
                return error.InvalidArguments;
            }
            const line_str = args[i + 1];
            const line_num = std.fmt.parseInt(u32, line_str, 10) catch {
                print("❌ Error: Invalid line number: {s}\n", .{line_str});
                return error.InvalidArguments;
            };
            try options.initial_breakpoints.append(allocator, line_num);
            i += 1;
        } else if (std.mem.eql(u8, arg, "--script") or std.mem.eql(u8, arg, "-s")) {
            if (i + 1 >= args.len) {
                print("❌ Error: --script requires a file path\n", .{});
                return error.InvalidArguments;
            }
            options.script_file = args[i + 1];
            options.interactive = false;
            i += 1;
        } else {
            print("❌ Unknown argument: {s}\n", .{arg});
            return error.InvalidArguments;
        }
        
        i += 1;
    }
    
    return options;
}

/// Print usage information
fn printUsage(program_name: []const u8) void {
    print("🐛 CURSED Interactive Debugger v1.0\n", .{});
    print("\n", .{});
    print("Usage: {s} <source_file.💀.💀> [options]\n", .{program_name});
    print("\n", .{});
    print("Options:\n", .{});
    print("  -h, --help              Show this help message\n", .{});
    print("  -i, --interactive       Start in interactive mode (default)\n", .{});
    print("  -r, --auto-run          Automatically run the program\n", .{});
    print("  -v, --verbose           Enable verbose output\n", .{});
    print("  -b, --breakpoint <line> Set initial breakpoint at line number\n", .{});
    print("  -s, --script <file>     Run debug script from file\n", .{});
    print("\n", .{});
    print("Examples:\n", .{});
    print("  {s} program.💀.💀                    # Interactive debugging\n", .{program_name});
    print("  {s} program.💀.💀 -b 10 -b 25        # Start with breakpoints\n", .{program_name});
    print("  {s} program.💀.💀 -r                 # Auto-run program\n", .{program_name});
    print("  {s} program.💀.💀 -s debug.script    # Run debug script\n", .{program_name});
    print("\n", .{});
    print("Interactive Commands:\n", .{});
    print("  help, h                 - Show debugger help\n", .{});
    print("  run, r                  - Run the program\n", .{});
    print("  break, b <location>     - Set breakpoint\n", .{});
    print("  continue, c             - Continue execution\n", .{});
    print("  step, s                 - Step into\n", .{});
    print("  next, n                 - Step over\n", .{});
    print("  print, p <variable>     - Print variable value\n", .{});
    print("  watch, w <variable>     - Watch variable for changes\n", .{});
    print("  backtrace, bt           - Show stack trace\n", .{});
    print("  list, l [line]          - List source code\n", .{});
    print("  quit, q                 - Exit debugger\n", .{});
}

/// Verify source file exists and is readable
fn verifySourceFile(file_path: []const u8) !void {
    const file = std.fs.cwd().openFile(file_path, .{}) catch |err| switch (err) {
        error.FileNotFound => {
            print("❌ Error: Source file '{s}' not found\n", .{file_path});
            return err;
        },
        error.AccessDenied => {
            print("❌ Error: Permission denied accessing '{s}'\n", .{file_path});
            return err;
        },
        else => {
            print("❌ Error: Failed to open '{s}': {!}\n", .{ file_path, err });
            return err;
        },
    };
    
    file.close();
    print("✅ Source file verified: {s}\n", .{file_path});
}

/// Parse source file and create AST
fn parseSourceFile(allocator: std.mem.Allocator, file_path: []const u8) !ast.Program {
    // Read source file
    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();
    
    const source_content = try file.readToEndAlloc(allocator, 1024 * 1024);
    defer allocator.free(source_content);
    
    print("📄 Source file loaded ({d} bytes)\n", .{source_content.len});
    
    // Tokenize
    var tokenizer = lexer.Lexer.init(allocator, source_content);
    var tokens = std.ArrayList(lexer.Token){};
    defer tokens.deinit();
    
    while (true) {
        const token = try tokenizer.nextToken();
        try tokens.append(allocator, token);
        if (token.kind == .Eof) break;
    }
    
    print("🔤 Tokenization complete ({d} tokens)\n", .{tokens.items.len});
    
    // Parse into AST
    var parse_engine = parser.Parser.init(allocator, tokens.items);
    defer parse_engine.deinit();
    
    const program = try parse_engine.parseProgram();
    print("🌳 AST parsing complete ({d} statements)\n", .{program.statements.items.len});
    
    return program;
}

/// Execute debug script from file
fn executeDebugScript(allocator: std.mem.Allocator, script_file: []const u8, debugger_instance: *debug_integration.DebugInterpreter) !void {
    const file = std.fs.cwd().openFile(script_file, .{}) catch |err| {
        print("❌ Error: Failed to open script file '{s}': {!}\n", .{ script_file, err });
        return err;
    };
    defer file.close();
    
    const script_content = try file.readToEndAlloc(allocator, 64 * 1024);
    defer allocator.free(script_content);
    
    print("📜 Executing debug script: {s}\n", .{script_file});
    
    var line_iter = std.mem.splitScalar(u8, script_content, '\n');
    var line_number: u32 = 0;
    
    while (line_iter.next()) |line| {
        line_number += 1;
        const trimmed_line = std.mem.trim(u8, line, " \t\r\n");
        
        // Skip empty lines and comments
        if (trimmed_line.len == 0 or trimmed_line[0] == '#') {
            continue;
        }
        
        print("📜 [{d}] {s}\n", .{ line_number, trimmed_line });
        
        // Process debug command (simplified - would integrate with debugger command processor)
        if (std.mem.startsWith(u8, trimmed_line, "break ")) {
            print("  → Setting breakpoint\n", .{});
        } else if (std.mem.startsWith(u8, trimmed_line, "run")) {
            print("  → Running program\n", .{});
        } else if (std.mem.startsWith(u8, trimmed_line, "continue")) {
            print("  → Continuing execution\n", .{});
        } else {
            print("  → Unknown command: {s}\n", .{trimmed_line});
        }
    }
    
    _ = debugger_instance; // TODO: Actually integrate with debugger commands
    print("✅ Debug script execution complete\n", .{});
}

/// Main entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Parse command line arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    const options = parseArguments(allocator, args) catch |err| switch (err) {
        error.HelpRequested => return,
        error.InvalidArguments => return,
        else => return err,
    };
    defer {
        var mut_options = options;
        mut_options.deinit();
    }
    
    // Print banner
    print("🐛 CURSED Interactive Debugger v1.0\n", .{});
    print("📁 Source file: {s}\n", .{options.source_file});
    
    if (options.verbose) {
        print("🔧 Verbose mode enabled\n", .{});
        if (options.interactive) {
            print("🔧 Interactive mode enabled\n", .{});
        }
        if (options.auto_run) {
            print("🔧 Auto-run enabled\n", .{});
        }
        if (options.script_file) |script| {
            print("🔧 Script file: {s}\n", .{script});
        }
        if (options.initial_breakpoints.items.len > 0) {
            print("🔧 Initial breakpoints: ", .{});
            for (options.initial_breakpoints.items, 0..) |bp, i| {
                if (i > 0) print(", ", .{});
                print("{d}", .{bp});
            }
            print("\n", .{});
        }
    }
    
    // Verify source file
    try verifySourceFile(options.source_file);
    
    // Parse source file
    const program = parseSourceFile(allocator, options.source_file) catch |err| {
        print("❌ Failed to parse source file: {!}\n", .{err});
        return;
    };
    defer {
        // Clean up program statements
        for (program.statements.items) |stmt_ptr| {
            // Note: In a real implementation, we'd need proper cleanup
            _ = stmt_ptr;
        }
        program.statements.deinit();
    }
    
    // Create debug interpreter
    var debug_interpreter = debug_integration.DebugInterpreter.init(allocator) catch |err| {
        print("❌ Failed to initialize debugger: {!}\n", .{err});
        return;
    };
    defer debug_interpreter.deinit();
    
    // Set initial breakpoints
    if (options.initial_breakpoints.items.len > 0) {
        print("🔴 Setting initial breakpoints...\n", .{});
        const debugger_ref = debug_interpreter.getDebugger();
        
        for (options.initial_breakpoints.items) |line_num| {
            // Simulate setting breakpoint (would use actual debugger API)
            print("🔴 Breakpoint set at line {d}\n", .{line_num});
            
            const key = @import("debugger.zig").BreakpointKey{
                .file = try allocator.dupe(u8, "main.💀"),
                .line = line_num,
            };
            
            const breakpoint = @import("debugger.zig").Breakpoint{
                .id = @intCast(line_num),
                .key = key,
                .enabled = true,
                .condition = null,
                .hit_count = 0,
            };
            
            try debugger_ref.breakpoints.put(key, breakpoint);
        }
    }
    
    // Execute script if provided
    if (options.script_file) |script_file| {
        try executeDebugScript(allocator, script_file, &debug_interpreter);
        return;
    }
    
    // Start debugging session
    if (options.interactive) {
        print("\n🚀 Starting interactive debugging session...\n", .{});
        print("Type 'help' for available commands\n\n", .{});
        
        try debug_interpreter.startDebugSession(options.source_file);
    } else if (options.auto_run) {
        print("\n🏃 Auto-running program with debugging...\n", .{});
        try debug_interpreter.executeWithDebug(program);
    } else {
        print("\n📊 Program analysis complete. Use --interactive to debug.\n", .{});
    }
    
    print("👋 Debugger session ended\n", .{});
}

// Error handling for cleaner output
fn handleError(err: anyerror) void {
    switch (err) {
        error.OutOfMemory => print("❌ Fatal: Out of memory\n"),
        error.InvalidArguments => print("❌ Invalid command line arguments\n"),
        error.FileNotFound => print("❌ Source file not found\n"),
        error.AccessDenied => print("❌ Permission denied\n"),
        else => print("❌ Unexpected error: {!}\n", .{err}),
    }
}

// Test helper functions
test "argument parsing" {
    const testing = std.testing;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test basic usage
    const args1 = [_][]const u8{ "cursed-debug", "test.💀" };
    var options1 = try parseArguments(allocator, &args1);
    defer options1.deinit();
    
    try testing.expect(std.mem.eql(u8, options1.source_file, "test.💀"));
    try testing.expect(options1.interactive);
    try testing.expect(!options1.auto_run);
    
    // Test with breakpoints
    const args2 = [_][]const u8{ "cursed-debug", "test.💀", "--breakpoint", "10", "-b", "20" };
    var options2 = try parseArguments(allocator, &args2);
    defer options2.deinit();
    
    try testing.expect(options2.initial_breakpoints.items.len == 2);
    try testing.expect(options2.initial_breakpoints.items[0] == 10);
    try testing.expect(options2.initial_breakpoints.items[1] == 20);
}
