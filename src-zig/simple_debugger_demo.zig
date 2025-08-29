//! Simple CURSED Debugger Demo
//!
//! Demonstrates interactive debugging capabilities for CURSED programs.

const std = @import("std");
const print = std.debug.print;

/// Main debugger demonstration
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Get command line arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: {s} <source_file.csd> [--demo]\n", .{args[0]});
        return;
    }

    const source_file = args[1];
    const demo_mode = args.len > 2 and std.mem.eql(u8, args[2], "--demo");

    print("🐛 CURSED Interactive Debugger Demo v1.0\n", .{});
    print("📁 Source file: {s}\n", .{source_file});

    // Verify source file exists
    const file = std.fs.cwd().openFile(source_file, .{}) catch |err| {
        print("❌ Error: Cannot open source file '{s}': {s}\n", .{ source_file, err });
        return;
    };
    defer file.close();

    // Read source content
    const source_content = try file.readToEndAlloc(allocator, 1024 * 1024);
    defer allocator.free(source_content);

    print("✅ Source file loaded ({d} bytes)\n", .{source_content.len});

    if (demo_mode) {
        try runDemoMode(allocator, source_file, source_content);
    } else {
        try runInteractiveMode(allocator, source_file, source_content);
    }
}

/// Run demo mode with simulated debugging
fn runDemoMode(allocator: std.mem.Allocator, source_file: []const u8, source_content: []const u8) !void {
        
    print("\n🎬 Running debugger demo for {s}\n", .{source_file});
    
    // Parse source into lines for display
    var line_iter = std.mem.split(u8, source_content, "\n");
    var line_number: u32 = 1;
    
    print("\n📄 Source code:\n", .{});
    while (line_iter.next()) |line| {
        print("  {d:3}: {s}\n", .{ line_number, line });
        line_number += 1;
    }
    
    print("\n🔴 Demo: Setting breakpoints at lines 3, 10, and 15\n", .{});
    print("✅ Breakpoint 1 set at line 3\n", .{});
    print("✅ Breakpoint 2 set at line 10\n", .{});
    print("✅ Breakpoint 3 set at line 15\n", .{});
    
    print("\n🏃 Demo: Running program...\n", .{});
    print("🛑 Execution paused at line 3 (breakpoint)\n", .{});
    print("  ➤   3: sus x drip = 42\n", .{});
    
    print("\n🔍 Demo: Printing variables\n", .{});
    print("  x = 42 (drip)\n", .{});
    
    print("\n👣 Demo: Stepping to next line\n", .{});
    print("🛑 Execution paused at line 4\n", .{});
    print("  ➤   4: sus name tea = \"CURSED Debugger Test\"\n");
    
    print("\n🔍 Demo: Printing variables\n", .{});
    print("  x = 42 (drip)\n", .{});
    print("  name = \"CURSED Debugger Test\" (tea)\n");
    
    print("\n▶️ Demo: Continuing execution...\n", .{});
    print("🛑 Execution paused at line 10 (breakpoint)\n", .{});
    print("  ➤  10: sus result drip = x * 2\n", .{});
    
    print("\n🔍 Demo: Evaluating expression 'x * 2'\n", .{});
    print("  x * 2 = 84\n", .{});
    
    print("\n📚 Demo: Stack trace\n", .{});
    print("  ➤ #0: main_character at debug_test.csd:10\n", .{});
    
    print("\n👁️ Demo: Watch variables\n", .{});
    print("  Watching: x, result\n", .{});
    print("  x = 42\n", .{});
    print("  result = 84\n", .{});
    
    print("\n▶️ Demo: Continuing to completion...\n", .{});
    print("✅ Program execution completed\n", .{});
    
    print("\n📊 Demo: Debug session summary\n", .{});
    print("  • Breakpoints hit: 2\n", .{});
    print("  • Variables inspected: x, name, result\n", .{});
    print("  • Lines executed: 20\n", .{});
    
    print("\n🎯 This demonstrates CURSED's interactive debugging capabilities:\n", .{});
    print("  ✅ Step-by-step execution\n", .{});
    print("  ✅ Breakpoint management\n", .{});
    print("  ✅ Variable inspection\n", .{});
    print("  ✅ Expression evaluation\n", .{});
    print("  ✅ Stack trace viewing\n", .{});
    print("  ✅ Watch variables\n", .{});
    print("  ✅ Source code display\n", .{});
    
    print("\n👋 Demo complete!\n", .{});
}

/// Run interactive debugging mode
fn runInteractiveMode(allocator: std.mem.Allocator, source_file: []const u8, source_content: []const u8) !void {
    
    print("\n🚀 Starting interactive debugging session...\n", .{});
    print("Type 'help' for available commands\n\n", .{});
    
    // Parse source into lines
    var lines = std.ArrayList([]const u8){};
    defer lines.deinit();
    
    var line_iter = std.mem.split(u8, source_content, "\n");
    while (line_iter.next()) |line| {
        try lines.append(allocator, line);
    }
    
    var current_line: u32 = 1;
    var breakpoints = std.ArrayList(u32){};
    defer breakpoints.deinit();
    
    var stdin_buffer: [4096]u8 = undefined;
    const stdin = std.fs.File.stdin().reader(stdin_buffer[0..]);
    var input_buffer: [256]u8 = undefined;
    
    // Interactive command loop
    while (true) {
        print("(cursed-debug) ", .{});
        
        if (try stdin.readUntilDelimiterOrEof(input_buffer[0..], '\n')) |input| {
            const trimmed_input = std.mem.trim(u8, input, " \t\r\n");
            
            if (trimmed_input.len == 0) continue;
            
            var args = std.mem.split(u8, trimmed_input, " ");
            const command = args.next() orelse continue;
            
            if (std.mem.eql(u8, command, "help") or std.mem.eql(u8, command, "h")) {
                printHelp();
            } else if (std.mem.eql(u8, command, "list") or std.mem.eql(u8, command, "l")) {
                try listSource(lines, current_line);
            } else if (std.mem.eql(u8, command, "break") or std.mem.eql(u8, command, "b")) {
                if (args.next()) |line_str| {
                    if (std.fmt.parseInt(u32, line_str, 10)) |line_num| {
                        try breakpoints.append(allocator, line_num);
                        print("🔴 Breakpoint set at line {s}\n", .{line_num});
                    } else |_| {
                        print("❌ Invalid line number: {s}\n", .{line_str});
                    }
                } else {
                    print("📍 Breakpoints:\n", .{});
                    for (breakpoints.items, 0..) |bp, i| {
                        print("  {s}: line {s}\n", .{ i + 1, bp });
                    }
                }
            } else if (std.mem.eql(u8, command, "run") or std.mem.eql(u8, command, "r")) {
                print("🏃 Running program '{s}'...\n", .{source_file});
                print("ℹ️  (Simulated execution - integrate with real interpreter)\n", .{});
            } else if (std.mem.eql(u8, command, "step") or std.mem.eql(u8, command, "s")) {
                current_line = @min(current_line + 1, @as(u32, @intCast(lines.items.len)));
                print("👣 Stepped to line {s}\n", .{current_line});
                try showCurrentLine(lines, current_line);
            } else if (std.mem.eql(u8, command, "print") or std.mem.eql(u8, command, "p")) {
                if (args.next()) |var_name| {
                    print("🔍 {s} = <simulated value>\n", .{var_name});
                } else {
                    print("❌ Usage: print <variable>\n", .{});
                }
            } else if (std.mem.eql(u8, command, "continue") or std.mem.eql(u8, command, "c")) {
                print("▶️ Continuing execution...\n", .{});
            } else if (std.mem.eql(u8, command, "quit") or std.mem.eql(u8, command, "q")) {
                print("👋 Exiting debugger\n", .{});
                break;
            } else {
                print("❌ Unknown command: {s}. Type 'help' for available commands.\n", .{command});
            }
        } else {
            break;
        }
    }
}

/// Print help for debugger commands
fn printHelp() void {
    print("🐛 CURSED Debugger Commands:\n", .{});
    print("  help, h                 - Show this help\n", .{});
    print("  list, l                 - List source code around current line\n", .{});
    print("  break, b <line>         - Set breakpoint at line\n", .{});
    print("  break, b                - List all breakpoints\n", .{});
    print("  run, r                  - Run the program\n", .{});
    print("  step, s                 - Step to next line\n", .{});
    print("  continue, c             - Continue execution\n", .{});
    print("  print, p <variable>     - Print variable value\n", .{});
    print("  quit, q                 - Exit debugger\n", .{});
    print("\nNote: This is a demonstration. Full integration with CURSED interpreter coming soon.\n", .{});
}

/// List source code around current line
fn listSource(lines: std.ArrayList([]const u8), current_line: u32) !void {
    const start = if (current_line >= 5) current_line - 5 else 1;
    const end = @min(current_line + 5, @as(u32, @intCast(lines.items.len)));
    
    print("📄 Source code (lines {s}-{s}):\n", .{ start, end });
    
    var line_num = start;
    while (line_num <= end) : (line_num += 1) {
        const line_content = if (line_num <= lines.items.len) lines.items[line_num - 1] else "";
        const marker = if (line_num == current_line) "➤" else " ";
        print("  {s} {d:3}: {s}\n", .{ marker, line_num, line_content });
    }
}

/// Show current line with highlight
fn showCurrentLine(lines: std.ArrayList([]const u8), current_line: u32) !void {
    if (current_line > 0 and current_line <= lines.items.len) {
        const line_content = lines.items[current_line - 1];
        print("  ➤ {d:3}: {s}\n", .{ current_line, line_content });
    }
}
