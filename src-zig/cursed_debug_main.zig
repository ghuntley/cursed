//! CURSED Interactive Debugger Main Entry Point
//! Week 2 Oracle's Plan - Debugger MVP Implementation

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const DebuggerState = struct {
    breakpoints: std.AutoHashMap(u32, bool),
    current_line: u32,
    is_running: bool,
    step_mode: bool,
    source_lines: ArrayList([]const u8),
    variables: std.StringHashMap([]const u8),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) DebuggerState {
        return DebuggerState{
            .breakpoints = std.AutoHashMap(u32, bool).init(allocator),
            .current_line = 1,
            .is_running = false,
            .step_mode = false,
            .source_lines = ArrayList([]const u8){},
            .variables = std.StringHashMap([]const u8).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *DebuggerState) void {
        self.breakpoints.deinit();
        for (self.source_lines.items) |line| {
            self.allocator.free(line);
        }
        self.source_lines.deinit(self.allocator);
        
        var var_iter = self.variables.iterator();
        while (var_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.variables.deinit();
    }
};

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
        return;
    }
    
    const source_file = args[1];
    
    print("🚀 Starting CURSED Debugger MVP\n", .{});
    print("📁 File: {s}\n", .{source_file});
    print("Type 'help' for commands\n\n", .{});
    
    var debugger = DebuggerState.init(allocator);
    defer debugger.deinit();
    
    // Load source file
    try loadSourceFile(&debugger, source_file);
    
    // Start interactive debugging session
    try startInteractiveSession(&debugger, source_file);
}

fn loadSourceFile(debugger: *DebuggerState, file_path: []const u8) !void {
    const file = std.fs.cwd().openFile(file_path, .{}) catch |err| {
        print("❌ Error opening file: {}\n", .{err});
        return;
    };
    defer file.close();
    
    const contents = try file.readToEndAlloc(debugger.allocator, 1024 * 1024);
    defer debugger.allocator.free(contents);
    
    var line_iter = std.mem.splitSequence(u8, contents, "\n");
    while (line_iter.next()) |line| {
        const line_copy = try debugger.allocator.dupe(u8, line);
        try debugger.source_lines.append(debugger.allocator, line_copy);
    }
    
    print("✅ Loaded {d} lines from {s}\n", .{ debugger.source_lines.items.len, file_path });
}

fn startInteractiveSession(debugger: *DebuggerState, source_file: []const u8) !void {
    const stdin = std.io.getStdIn().reader();
    
    while (true) {
        print("(cursed-db) ");
        
        var buffer: [256]u8 = undefined;
        if (try stdin.readUntilDelimiterOrEof(buffer[0..], '\n')) |input| {
            const trimmed = std.mem.trim(u8, input, " \t\n\r");
            
            if (trimmed.len == 0) continue;
            
            if (std.mem.eql(u8, trimmed, "quit") or std.mem.eql(u8, trimmed, "q")) {
                print("👋 Goodbye!\n", .{});
                break;
            } else if (std.mem.eql(u8, trimmed, "help") or std.mem.eql(u8, trimmed, "h")) {
                printHelp();
            } else if (std.mem.startsWith(u8, trimmed, "break ") or std.mem.startsWith(u8, trimmed, "b ")) {
                const line_str = trimmed[if (trimmed[0] == 'b') 2 else 6..];
                if (std.fmt.parseInt(u32, line_str, 10)) |line_num| {
                    try setBreakpoint(debugger, line_num);
                } else |_| {
                    print("❌ Invalid line number: {s}\n", .{line_str});
                }
            } else if (std.mem.eql(u8, trimmed, "list") or std.mem.eql(u8, trimmed, "l")) {
                listSource(debugger);
            } else if (std.mem.eql(u8, trimmed, "step") or std.mem.eql(u8, trimmed, "s")) {
                try stepExecution(debugger);
            } else if (std.mem.eql(u8, trimmed, "continue") or std.mem.eql(u8, trimmed, "c")) {
                try continueExecution(debugger);
            } else if (std.mem.eql(u8, trimmed, "backtrace") or std.mem.eql(u8, trimmed, "bt")) {
                printBacktrace(debugger);
            } else if (std.mem.startsWith(u8, trimmed, "print ") or std.mem.startsWith(u8, trimmed, "p ")) {
                const var_name = trimmed[if (trimmed[0] == 'p') 2 else 6..];
                printVariable(debugger, var_name);
            } else if (std.mem.eql(u8, trimmed, "run") or std.mem.eql(u8, trimmed, "r")) {
                try runProgram(debugger, source_file);
            } else if (std.mem.eql(u8, trimmed, "info breakpoints")) {
                listBreakpoints(debugger);
            } else {
                print("❓ Unknown command: {s}\n", .{trimmed});
                print("Type 'help' for available commands\n", .{});
            }
        }
    }
}

fn printHelp() void {
    print("\n🐛 CURSED Debugger Commands:\n", .{});
    print("  help (h)           - Show this help\n", .{});
    print("  run (r)            - Run the program\n", .{});
    print("  break <line> (b)   - Set breakpoint at line\n", .{});
    print("  list (l)           - List source code\n", .{});
    print("  step (s)           - Step one line\n", .{});
    print("  continue (c)       - Continue execution\n", .{});
    print("  backtrace (bt)     - Show call stack\n", .{});
    print("  print <var> (p)    - Print variable value\n", .{});
    print("  info breakpoints   - List all breakpoints\n", .{});
    print("  quit (q)           - Exit debugger\n", .{});
    print("\n", .{});
}

fn setBreakpoint(debugger: *DebuggerState, line: u32) !void {
    if (line == 0 or line > debugger.source_lines.items.len) {
        print("❌ Invalid line number: {d}\n", .{line});
        return;
    }
    
    try debugger.breakpoints.put(line, true);
    print("🔴 Breakpoint set at line {d}\n", .{line});
}

fn listSource(debugger: *DebuggerState) void {
    print("\n📄 Source Code:\n", .{});
    const start = if (debugger.current_line >= 5) debugger.current_line - 5 else 0;
    const end = @min(start + 10, debugger.source_lines.items.len);
    
    for (start..end) |i| {
        const line_num = i + 1;
        const prefix = if (line_num == debugger.current_line) "➤ " else "  ";
        const breakpoint_marker = if (debugger.breakpoints.contains(@intCast(line_num))) "🔴" else " ";
        
        print("{s}{s}{d:>3}: {s}\n", .{ prefix, breakpoint_marker, line_num, debugger.source_lines.items[i] });
    }
    print("\n", .{});
}

fn stepExecution(debugger: *DebuggerState) !void {
    if (debugger.current_line < debugger.source_lines.items.len) {
        debugger.current_line += 1;
        debugger.step_mode = true;
        
        // Add demo variable for testing
        const var_name = try debugger.allocator.dupe(u8, "x");
        const var_value = try std.fmt.allocPrint(debugger.allocator, "{d}", .{debugger.current_line * 10});
        try debugger.variables.put(var_name, var_value);
        
        print("👣 Stepped to line {d}\n", .{debugger.current_line});
        
        if (debugger.source_lines.items.len > 0 and debugger.current_line <= debugger.source_lines.items.len) {
            print("   {s}\n", .{debugger.source_lines.items[debugger.current_line - 1]});
        }
    } else {
    print("✅ Program finished\n", .{});
    }
}

fn continueExecution(debugger: *DebuggerState) !void {
    debugger.step_mode = false;
    debugger.is_running = true;
    
    while (debugger.current_line <= debugger.source_lines.items.len) {
        debugger.current_line += 1;
        
        if (debugger.breakpoints.contains(debugger.current_line)) {
            print("🔴 Hit breakpoint at line {d}\n", .{debugger.current_line});
            print("   {s}\n", .{debugger.source_lines.items[debugger.current_line - 1]});
            debugger.is_running = false;
            return;
        }
    }
    
    print("✅ Program finished\n", .{});
    debugger.is_running = false;
}

fn printBacktrace(debugger: *DebuggerState) void {
    print("\n📚 Call Stack:\n", .{});
    print("  #0  main() at line {d}\n", .{debugger.current_line});
    print("  #1  <main program>\n", .{});
    print("\n", .{});
}

fn printVariable(debugger: *DebuggerState, var_name: []const u8) void {
    if (debugger.variables.get(var_name)) |value| {
        print("🔍 {s} = {s}\n", .{ var_name, value });
    } else {
        // Demo values for common CURSED variables
        if (std.mem.eql(u8, var_name, "x")) {
            print("🔍 x = {d}\n", .{debugger.current_line * 10});
        } else if (std.mem.eql(u8, var_name, "name")) {
            print("🔍 name = \"CURSED User\"\n", .{});
        } else if (std.mem.eql(u8, var_name, "active")) {
            print("🔍 active = based\n", .{});
        } else {
            print("❓ Variable '{s}' not found\n", .{var_name});
        }
    }
}

fn runProgram(debugger: *DebuggerState, source_file: []const u8) !void {
    print("🚀 Running {s}...\n", .{source_file});
    debugger.current_line = 1;
    debugger.is_running = true;
    
    // Simulate program execution
    for (debugger.source_lines.items, 0..) |line, i| {
        debugger.current_line = @intCast(i + 1);
        
        if (debugger.breakpoints.contains(debugger.current_line)) {
            print("🔴 Hit breakpoint at line {d}\n", .{debugger.current_line});
            print("   {s}\n", .{line});
            debugger.is_running = false;
            return;
        }
        
        // Simple execution simulation
        if (std.mem.indexOf(u8, line, "spill") != null) {
            print("📤 Output: {s}\n", .{line});
        }
    }
    
    print("✅ Program execution completed\n", .{});
    debugger.is_running = false;
}

fn listBreakpoints(debugger: *DebuggerState) void {
    print("\n🔴 Breakpoints:\n", .{});
    var iter = debugger.breakpoints.iterator();
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
