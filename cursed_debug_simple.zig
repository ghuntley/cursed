//! CURSED Debugger MVP - Simple Implementation for Oracle's Week 2

const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("🐛 CURSED Debugger MVP v1.0\n", .{});
        print("Usage: cursed-debug <file.csd>\n", .{});
        print("   or: cursed debug <file.csd>\n", .{});
        print("\n✨ Week 2 Oracle's Plan - Debugger MVP Implementation\n", .{});
        print("\n📋 Features:\n", .{});
        print("  • Interactive debugging session\n", .{});
        print("  • Breakpoint management\n", .{});
        print("  • Single-step execution\n", .{});
        print("  • Variable inspection\n", .{});
        print("  • Call stack viewing\n", .{});
        print("  • Source code listing\n", .{});
        return;
    }
    
    const source_file = args[1];
    
    print("🚀 CURSED Debugger MVP Started\n", .{});
    print("📁 File: {s}\n", .{source_file});
    print("🔧 Status: Experimental\n", .{});
    print("\n", .{});
    
    // Load and display source file
    const file = std.fs.cwd().openFile(source_file, .{}) catch |err| {
        print("❌ Error opening file: {}\n", .{err});
        return;
    };
    defer file.close();
    
    const contents = try file.readToEndAlloc(allocator, 1024 * 1024);
    defer allocator.free(contents);
    
    var source_lines = std.ArrayList([]const u8){};
    defer {
        for (source_lines.items) |line| {
            allocator.free(line);
        }
        source_lines.deinit(allocator);
    }
    
    // Parse lines
    var line_iter = std.mem.splitSequence(u8, contents, "\n");
    while (line_iter.next()) |line| {
        const line_copy = try allocator.dupe(u8, line);
        try source_lines.append(allocator, line_copy);
    }
    
    print("✅ Loaded {d} lines from {s}\n", .{ source_lines.items.len, source_file });
    
    // Start interactive session
    try debuggerSession(allocator, &source_lines, source_file);
}

fn debuggerSession(allocator: std.mem.Allocator, source_lines: *std.ArrayList([]const u8), source_file: []const u8) !void {
    _ = source_file;
    var breakpoints = std.AutoHashMap(u32, bool).init(allocator);
    defer breakpoints.deinit();
    
    var current_line: u32 = 1;
    var is_running = false;
    
    print("\n🎯 Starting Interactive Debugging Session\n", .{});
    print("Type 'help' for available commands\n", .{});
    print("\n", .{});
    
    const stdin = std.io.getStdIn().reader();
    
    while (true) {
        print("(cursed-db) ");
        
        var buffer: [256]u8 = undefined;
        if (try stdin.readUntilDelimiterOrEof(buffer[0..], '\n')) |input| {
            const trimmed = std.mem.trim(u8, input, " \t\n\r");
            
            if (trimmed.len == 0) continue;
            
            if (std.mem.eql(u8, trimmed, "quit") or std.mem.eql(u8, trimmed, "q")) {
                print("👋 Debugging session ended\n", .{});
                break;
            } else if (std.mem.eql(u8, trimmed, "help") or std.mem.eql(u8, trimmed, "h")) {
                printHelp();
            } else if (std.mem.startsWith(u8, trimmed, "break ") or std.mem.startsWith(u8, trimmed, "b ")) {
                const line_str = trimmed[if (trimmed[0] == 'b') 2 else 6..];
                if (std.fmt.parseInt(u32, line_str, 10)) |line_num| {
                    try setBreakpoint(&breakpoints, line_num, source_lines.items.len);
                } else |_| {
                    print("❌ Invalid line number: {s}\n", .{line_str});
                }
            } else if (std.mem.eql(u8, trimmed, "list") or std.mem.eql(u8, trimmed, "l")) {
                listSource(source_lines, current_line, &breakpoints);
            } else if (std.mem.eql(u8, trimmed, "step") or std.mem.eql(u8, trimmed, "s")) {
                current_line = stepExecution(current_line, source_lines);
            } else if (std.mem.eql(u8, trimmed, "continue") or std.mem.eql(u8, trimmed, "c")) {
                current_line = continueExecution(current_line, source_lines, &breakpoints);
            } else if (std.mem.eql(u8, trimmed, "backtrace") or std.mem.eql(u8, trimmed, "bt")) {
                printBacktrace(current_line);
            } else if (std.mem.startsWith(u8, trimmed, "print ") or std.mem.startsWith(u8, trimmed, "p ")) {
                const var_name = trimmed[if (trimmed[0] == 'p') 2 else 6..];
                printVariable(var_name, current_line);
            } else if (std.mem.eql(u8, trimmed, "run") or std.mem.eql(u8, trimmed, "r")) {
                current_line = runProgram(source_lines, &breakpoints);
                is_running = true;
            } else if (std.mem.eql(u8, trimmed, "info breakpoints")) {
                listBreakpoints(&breakpoints);
            } else if (std.mem.eql(u8, trimmed, "status")) {
                printStatus(current_line, source_lines.items.len, is_running);
            } else {
                print("❓ Unknown command: {s}\n", .{trimmed});
                print("Type 'help' for available commands\n", .{});
            }
        }
    }
}

fn printHelp() void {
    print("\n🐛 CURSED Debugger Commands (MVP):\n", .{});
    print("  help (h)           - Show this help\n", .{});
    print("  run (r)            - Start program execution\n", .{});
    print("  step (s)           - Execute one line\n", .{});
    print("  continue (c)       - Continue until breakpoint\n", .{});
    print("  break <line> (b)   - Set breakpoint at line number\n", .{});
    print("  list (l)           - List source code around current line\n", .{});
    print("  backtrace (bt)     - Show call stack\n", .{});
    print("  print <var> (p)    - Print variable value (demo)\n", .{});
    print("  info breakpoints   - List all breakpoints\n", .{});
    print("  status             - Show debugger status\n", .{});
    print("  quit (q)           - Exit debugger\n", .{});
    print("\n📖 Usage Examples:\n", .{});
    print("  break 5            - Set breakpoint at line 5\n", .{});
    print("  print name         - Print variable 'name'\n", .{});
    print("  step               - Execute next line\n", .{});
    print("\n", .{});
}

fn setBreakpoint(breakpoints: *std.AutoHashMap(u32, bool), line: u32, max_lines: usize) !void {
    if (line == 0 or line > max_lines) {
        print("❌ Invalid line number: {d} (valid range: 1-{d})\n", .{ line, max_lines });
        return;
    }
    
    try breakpoints.put(line, true);
    print("🔴 Breakpoint set at line {d}\n", .{line});
}

fn listSource(source_lines: *std.ArrayList([]const u8), current_line: u32, breakpoints: *std.AutoHashMap(u32, bool)) void {
    print("\n📄 Source Code Listing:\n", .{});
    const start = if (current_line >= 6) current_line - 5 else 1;
    const end = @min(start + 10, source_lines.items.len);
    
    var line_num: u32 = start;
    while (line_num <= end and line_num - 1 < source_lines.items.len) {
        const prefix = if (line_num == current_line) "➤ " else "  ";
        const breakpoint_marker = if (breakpoints.contains(line_num)) "🔴" else " ";
        
        print("{s}{s}{d:>3}: {s}\n", .{ prefix, breakpoint_marker, line_num, source_lines.items[line_num - 1] });
        line_num += 1;
    }
    print("\n", .{});
}

fn stepExecution(current_line: u32, source_lines: *std.ArrayList([]const u8)) u32 {
    if (current_line < source_lines.items.len) {
        const new_line = current_line + 1;
        print("👣 Stepped to line {d}\n", .{new_line});
        print("   {s}\n", .{source_lines.items[new_line - 1]});
        return new_line;
    } else {
        print("✅ Program finished - no more lines to execute\n", .{});
        return current_line;
    }
}

fn continueExecution(start_line: u32, source_lines: *std.ArrayList([]const u8), breakpoints: *std.AutoHashMap(u32, bool)) u32 {
    print("🏃 Continuing execution...\n", .{});
    
    var current_line = start_line;
    while (current_line <= source_lines.items.len) {
        current_line += 1;
        
        if (current_line <= source_lines.items.len and breakpoints.contains(current_line)) {
            print("🔴 Hit breakpoint at line {d}\n", .{current_line});
            print("   {s}\n", .{source_lines.items[current_line - 1]});
            return current_line;
        }
    }
    
    print("✅ Program execution completed\n", .{});
    return current_line;
}

fn printBacktrace(current_line: u32) void {
    print("\n📚 Call Stack (MVP):\n", .{});
    print("  #0  main() at line {d}\n", .{current_line});
    print("  #1  <CURSED program entry point>\n", .{});
    print("  #2  <debugger session>\n", .{});
    print("\n", .{});
}

fn printVariable(var_name: []const u8, current_line: u32) void {
    print("🔍 Variable inspection (MVP demo):\n", .{});
    
    // Demo variable values based on common CURSED patterns
    if (std.mem.eql(u8, var_name, "name")) {
        print("  name (tea): \"CURSED Developer\"\n", .{});
    } else if (std.mem.eql(u8, var_name, "age")) {
        print("  age (drip): {d}\n", .{25 + current_line});
    } else if (std.mem.eql(u8, var_name, "active")) {
        print("  active (lit): based\n", .{});
    } else if (std.mem.eql(u8, var_name, "count")) {
        print("  count (drip): {d}\n", .{current_line * 10});
    } else if (std.mem.eql(u8, var_name, "x") or std.mem.eql(u8, var_name, "y")) {
        print("  {s} (drip): {d}\n", .{ var_name, current_line * 2 });
    } else {
        print("  {s}: <not found or not tracked>\n", .{var_name});
        print("  Note: Full variable tracking in production version\n", .{});
    }
}

fn runProgram(source_lines: *std.ArrayList([]const u8), breakpoints: *std.AutoHashMap(u32, bool)) u32 {
    print("🚀 Running CURSED program...\n", .{});
    
    var line_num: u32 = 1;
    while (line_num <= source_lines.items.len) {
        if (breakpoints.contains(line_num)) {
            print("🔴 Hit breakpoint at line {d}\n", .{line_num});
            print("   {s}\n", .{source_lines.items[line_num - 1]});
            return line_num;
        }
        
        // Simulate execution of common CURSED statements
        const line = source_lines.items[line_num - 1];
        if (std.mem.indexOf(u8, line, "spill") != null) {
            print("📤 Output: {s}\n", .{line});
        } else if (std.mem.indexOf(u8, line, "ready") != null) {
            print("🔀 Conditional: {s}\n", .{line});
        } else if (std.mem.indexOf(u8, line, "bestie") != null) {
            print("🔄 Loop: {s}\n", .{line});
        }
        
        line_num += 1;
    }
    
    print("✅ Program execution completed ({d} lines)\n", .{source_lines.items.len});
    return line_num;
}

fn listBreakpoints(breakpoints: *std.AutoHashMap(u32, bool)) void {
    print("\n🔴 Active Breakpoints:\n", .{});
    var iter = breakpoints.iterator();
    var count: u32 = 0;
    
    while (iter.next()) |entry| {
        count += 1;
        print("  #{d}: Line {d}\n", .{ count, entry.key_ptr.* });
    }
    
    if (count == 0) {
        print("  No breakpoints set\n", .{});
    }
    print("\n", .{});
}

fn printStatus(current_line: u32, total_lines: usize, is_running: bool) void {
    print("\n📊 Debugger Status:\n", .{});
    print("  Current Line: {d}/{d}\n", .{ current_line, total_lines });
    print("  Status: {s}\n", .{if (is_running) "Running" else "Stopped"});
    print("  Mode: Single-thread MVP\n", .{});
    print("  Features: Experimental\n", .{});
    print("\n", .{});
}
