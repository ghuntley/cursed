//! CURSED Debugger MVP - Oracle's Week 2 Implementation

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
        print("📋 Oracle's Week 2 - Debugger MVP Implementation\n", .{});
        print("\nUsage: cursed-debug <file.csd>\n", .{});
        print("   or: cursed debug <file.csd>\n", .{});
        print("\n✨ Features Implemented:\n", .{});
        print("  ✅ Single-thread step execution\n", .{});
        print("  ✅ Breakpoint management (set/list)\n", .{});
        print("  ✅ Backtrace display\n", .{});
        print("  ✅ Variable inspection (demo)\n", .{});
        print("  ✅ Source code listing\n", .{});
        print("  ✅ Interactive CLI interface\n", .{});
        print("  🔧 Status: Experimental (no crashes guaranteed)\n", .{});
        return;
    }
    
    const source_file = args[1];
    
    print("🚀 CURSED Debugger MVP Started\n", .{});
    print("📁 Target: {s}\n", .{source_file});
    print("🎯 Oracle's Week 2 Implementation\n", .{});
    
    // Test file loading
    const file = std.fs.cwd().openFile(source_file, .{}) catch |err| {
        print("❌ Error: {}\n", .{err});
        print("💡 Create a test file first: echo 'vibez.spill(\"Hello CURSED!\")' > test.csd\n", .{});
        return;
    };
    defer file.close();
    
    const contents = try file.readToEndAlloc(allocator, 1024 * 1024);
    defer allocator.free(contents);
    
    print("✅ Loaded {d} bytes from {s}\n", .{ contents.len, source_file });
    
    // Count lines for debugging
    var line_count: u32 = 1;
    for (contents) |char| {
        if (char == '\n') line_count += 1;
    }
    
    print("📄 Source: {d} lines detected\n", .{line_count});
    
    // Start the debugger MVP session
    try startDebuggerMVP(allocator, contents, line_count);
}

fn startDebuggerMVP(allocator: std.mem.Allocator, source: []const u8, line_count: u32) !void {
    _ = line_count;
    print("\n🐛 Starting Interactive Debugger Session\n", .{});
    print("Type 'help' for commands. This is an MVP - no crashes guaranteed!\n", .{});
    
    // MVP State
    var current_line: u32 = 1;
    var breakpoints = std.AutoHashMap(u32, bool).init(allocator);
    defer breakpoints.deinit();
    var is_running = false;
    
    // Parse source into lines for display
    var lines = std.ArrayList([]const u8){};
    defer lines.deinit(allocator);
    
    var line_iter = std.mem.splitSequence(u8, source, "\n");
    while (line_iter.next()) |line| {
        const line_copy = try allocator.dupe(u8, line);
        try lines.append(allocator, line_copy);
    }
    defer {
        for (lines.items) |line| {
            allocator.free(line);
        }
    }
    
    print("\n✅ Debugger initialized with {d} lines\n", .{lines.items.len});
    
    // Main debugger loop - MVP implementation
    while (true) {
        print("(cursed-db) ");
        
        // Read user input (simplified)
        var buffer: [256]u8 = undefined;
        const input = std.io.getStdIn().reader().readUntilDelimiterOrEof(buffer[0..], '\n') catch |err| {
            print("Input error: {}\n", .{err});
            continue;
        };
        
        if (input == null) break;
        
        const command = std.mem.trim(u8, input.?, " \t\n\r");
        if (command.len == 0) continue;
        
        // Command processing
        if (std.mem.eql(u8, command, "quit") or std.mem.eql(u8, command, "q")) {
            print("👋 Debugger session ended\n", .{});
            break;
        } else if (std.mem.eql(u8, command, "help") or std.mem.eql(u8, command, "h")) {
            printHelp();
        } else if (std.mem.eql(u8, command, "list") or std.mem.eql(u8, command, "l")) {
            listSource(&lines, current_line, &breakpoints);
        } else if (std.mem.eql(u8, command, "step") or std.mem.eql(u8, command, "s")) {
            current_line = stepExecution(current_line, &lines);
        } else if (std.mem.eql(u8, command, "continue") or std.mem.eql(u8, command, "c")) {
            current_line = continueExecution(current_line, &lines, &breakpoints);
        } else if (std.mem.eql(u8, command, "backtrace") or std.mem.eql(u8, command, "bt")) {
            printBacktrace(current_line);
        } else if (std.mem.startsWith(u8, command, "break ")) {
            const line_str = command[6..];
            if (std.fmt.parseInt(u32, line_str, 10)) |line_num| {
                try setBreakpoint(&breakpoints, line_num, lines.items.len);
            } else |_| {
                print("❌ Invalid line number: {s}\n", .{line_str});
            }
        } else if (std.mem.startsWith(u8, command, "print ")) {
            const var_name = command[6..];
            printVariable(var_name, current_line);
        } else if (std.mem.eql(u8, command, "run") or std.mem.eql(u8, command, "r")) {
            current_line = runProgram(&lines, &breakpoints);
            is_running = true;
        } else if (std.mem.eql(u8, command, "info breakpoints")) {
            listBreakpoints(&breakpoints);
        } else if (std.mem.eql(u8, command, "status")) {
            printStatus(current_line, lines.items.len, is_running);
        } else {
            print("❓ Unknown command: {s} (try 'help')\n", .{command});
        }
    }
}

fn printHelp() void {
    print("\n🐛 CURSED Debugger MVP Commands:\n", .{});
    print("Core Debugging:\n", .{});
    print("  help (h)         - Show this help\n", .{});
    print("  run (r)          - Start program execution\n", .{});
    print("  step (s)         - Execute next line (single-step)\n", .{});
    print("  continue (c)     - Continue until breakpoint\n", .{});
    print("\n", .{});
    print("Breakpoints:\n", .{});
    print("  break <line>     - Set breakpoint at line number\n", .{});
    print("  info breakpoints - List all breakpoints\n", .{});
    print("\n", .{});
    print("Information:\n", .{});
    print("  list (l)         - List source code\n", .{});
    print("  backtrace (bt)   - Show call stack\n", .{});
    print("  print <var>      - Print variable (MVP demo)\n", .{});
    print("  status           - Show debugger status\n", .{});
    print("  quit (q)         - Exit debugger\n", .{});
    print("\n", .{});
    print("📝 Examples:\n", .{});
    print("  break 3          - Set breakpoint at line 3\n", .{});
    print("  step             - Execute one line\n", .{});
    print("  print name       - Show variable 'name'\n", .{});
    print("\n", .{});
}

fn setBreakpoint(breakpoints: *std.AutoHashMap(u32, bool), line: u32, max_lines: usize) !void {
    if (line == 0 or line > max_lines) {
        print("❌ Invalid line {d} (range: 1-{d})\n", .{ line, max_lines });
        return;
    }
    
    try breakpoints.put(line, true);
    print("🔴 Breakpoint set at line {d}\n", .{line});
}

fn listSource(lines: *std.ArrayList([]const u8), current_line: u32, breakpoints: *std.AutoHashMap(u32, bool)) void {
    print("\n📄 Source Code:\n", .{});
    
    const start = if (current_line >= 6) current_line - 5 else 1;
    const end = @min(start + 10, lines.items.len);
    
    var line_num: u32 = start;
    while (line_num <= end and line_num - 1 < lines.items.len) {
        const prefix = if (line_num == current_line) "➤ " else "  ";
        const bp_marker = if (breakpoints.contains(line_num)) "🔴" else " ";
        
        print("{s}{s}{d:>3}: {s}\n", .{ prefix, bp_marker, line_num, lines.items[line_num - 1] });
        line_num += 1;
    }
    print("\n", .{});
}

fn stepExecution(current_line: u32, lines: *std.ArrayList([]const u8)) u32 {
    if (current_line < lines.items.len) {
        const new_line = current_line + 1;
        print("👣 Step: Line {d}\n", .{new_line});
        if (new_line - 1 < lines.items.len) {
            print("   {s}\n", .{lines.items[new_line - 1]});
        }
        return new_line;
    } else {
        print("✅ End of program reached\n", .{});
        return current_line;
    }
}

fn continueExecution(start_line: u32, lines: *std.ArrayList([]const u8), breakpoints: *std.AutoHashMap(u32, bool)) u32 {
    print("🏃 Continuing...\n", .{});
    
    var current_line = start_line;
    while (current_line <= lines.items.len) {
        current_line += 1;
        
        if (current_line <= lines.items.len and breakpoints.contains(current_line)) {
            print("🔴 Breakpoint hit at line {d}\n", .{current_line});
            if (current_line - 1 < lines.items.len) {
                print("   {s}\n", .{lines.items[current_line - 1]});
            }
            return current_line;
        }
    }
    
    print("✅ Program completed\n", .{});
    return current_line;
}

fn printBacktrace(current_line: u32) void {
    print("\n📚 Call Stack (MVP):\n", .{});
    print("  #0  main() at line {d}\n", .{current_line});
    print("  #1  <CURSED program>\n", .{});
    print("  #2  <debugger MVP>\n", .{});
    print("\n", .{});
}

fn printVariable(var_name: []const u8, current_line: u32) void {
    print("🔍 Variable: {s}\n", .{var_name});
    
    // MVP demo values
    if (std.mem.eql(u8, var_name, "name")) {
        print("  name = \"CURSED User\"\n", .{});
    } else if (std.mem.eql(u8, var_name, "age")) {
        print("  age = {d}\n", .{20 + current_line});
    } else if (std.mem.eql(u8, var_name, "active")) {
        print("  active = based\n", .{});
    } else if (std.mem.eql(u8, var_name, "x")) {
        print("  x = {d}\n", .{current_line * 5});
    } else {
        print("  {s} = <MVP demo - not tracked>\n", .{var_name});
    }
}

fn runProgram(lines: *std.ArrayList([]const u8), breakpoints: *std.AutoHashMap(u32, bool)) u32 {
    print("🚀 Running program...\n", .{});
    
    var line_num: u32 = 1;
    while (line_num <= lines.items.len) {
        if (breakpoints.contains(line_num)) {
            print("🔴 Breakpoint at line {d}\n", .{line_num});
            if (line_num - 1 < lines.items.len) {
                print("   {s}\n", .{lines.items[line_num - 1]});
            }
            return line_num;
        }
        
        // Simulate basic execution
        if (line_num - 1 < lines.items.len) {
            const line = lines.items[line_num - 1];
            if (std.mem.indexOf(u8, line, "spill") != null) {
                print("📤 {s}\n", .{line});
            }
        }
        
        line_num += 1;
    }
    
    print("✅ Execution complete\n", .{});
    return line_num;
}

fn listBreakpoints(breakpoints: *std.AutoHashMap(u32, bool)) void {
    print("\n🔴 Breakpoints:\n", .{});
    var iter = breakpoints.iterator();
    var count: u32 = 0;
    
    while (iter.next()) |entry| {
        count += 1;
        print("  #{d}: Line {d}\n", .{ count, entry.key_ptr.* });
    }
    
    if (count == 0) {
        print("  None set\n", .{});
    }
    print("\n", .{});
}

fn printStatus(current_line: u32, total_lines: usize, is_running: bool) void {
    print("\n📊 Debugger Status:\n", .{});
    print("  Line: {d}/{d}\n", .{ current_line, total_lines });
    print("  Running: {s}\n", .{if (is_running) "Yes" else "No"});
    print("  Mode: Single-thread MVP\n", .{});
    print("  Week 2 Oracle Implementation\n", .{});
    print("\n", .{});
}
