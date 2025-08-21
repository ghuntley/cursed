//! CURSED Standalone Interactive Debugger
//!
//! Simplified debugger that works independently of the full interpreter
//! to demonstrate debugging capabilities and provide interactive debugging.

const std = @import("std");
const print = std.debug.print;

/// Simplified debugger for demonstration
const StandaloneDebugger = struct {
    allocator: std.mem.Allocator,
    source_lines: std.ArrayList([]const u8),
    current_line: u32,
    breakpoints: std.HashMap(u32, bool, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage),
    watch_variables: std.ArrayList([]const u8),
    is_running: bool,
    is_paused: bool,
    
    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator) !Self {
        return Self{
            .allocator = allocator,
            .source_lines = .empty,
            .current_line = 0,
            .breakpoints = std.HashMap(u32, bool, std.hash_map.AutoContext(u32), std.hash_map.default_max_load_percentage).init(allocator),
            .watch_variables = .empty,
            .is_running = false,
            .is_paused = false,
        };
    }
    
    pub fn deinit(self: *Self) void {
        for (self.source_lines.items) |line| {
            self.allocator.free(line);
        }
        self.source_lines.deinit(allocator);
        
        for (self.watch_variables.items) |var_name| {
            self.allocator.free(var_name);
        }
        self.watch_variables.deinit(allocator);
        
        self.breakpoints.deinit(allocator);
    }
    
    pub fn loadSourceFile(self: *Self, file_path: []const u8) !void {
        const file = std.fs.cwd().openFile(file_path, .{}) catch |err| {
            print("❌ Error loading source file: {}\n", .{err});
            return err;
        };
        defer file.close();
        
        const content = try file.readToEndAlloc(self.allocator, 1024 * 1024);
        defer self.allocator.free(content);
        
        var line_iter = std.mem.splitScalar(u8, content, '\n');
        while (line_iter.next()) |line| {
            const line_copy = try self.allocator.dupe(u8, line);
            try self.source_lines.append(self.allocator, line_copy);
        }
        
        print("✅ Loaded {} lines from {s}\n", .{ self.source_lines.items.len, file_path });
    }
    
    pub fn startSession(self: *Self, source_file: []const u8) !void {
        print("🐛 CURSED Standalone Interactive Debugger v1.0\n", .{});
        print("📁 Debugging: {s}\n", .{source_file});
        print("Type 'help' for available commands\n\n", .{});
        
        try self.loadSourceFile(source_file);
        try self.commandLoop();
    }
    
    fn commandLoop(self: *Self) !void {
        var input_buffer: [256]u8 = undefined;
        const stdin = std.fs.File.stdin().reader(input_buffer[0..]);
        
        while (true) {
            if (self.is_paused) {
                try self.showCurrentLocation();
            }
            
            print("(cursed-debug) ", .{});
            
            if (try stdin.readUntilDelimiterOrEof(input_buffer[0..], '\n')) |input| {
                const trimmed_input = std.mem.trim(u8, input, " \t\r\n");
                if (trimmed_input.len == 0) continue;
                
                const should_continue = try self.processCommand(trimmed_input);
                if (!should_continue) break;
            } else {
                break;
            }
        }
        
        print("👋 Debug session ended\n", .{});
    }
    
    fn processCommand(self: *Self, input: []const u8) !bool {
        var args = std.mem.splitScalar(u8, input, ' ');
        const command = args.next() orelse return true;
        
        if (std.mem.eql(u8, command, "help") or std.mem.eql(u8, command, "h")) {
            self.printHelp();
        } else if (std.mem.eql(u8, command, "run") or std.mem.eql(u8, command, "r")) {
            self.runProgram();
        } else if (std.mem.eql(u8, command, "break") or std.mem.eql(u8, command, "b")) {
            try self.setBreakpoint(&args);
        } else if (std.mem.eql(u8, command, "continue") or std.mem.eql(u8, command, "c")) {
            self.continueExecution();
        } else if (std.mem.eql(u8, command, "step") or std.mem.eql(u8, command, "s")) {
            self.stepExecution();
        } else if (std.mem.eql(u8, command, "next") or std.mem.eql(u8, command, "n")) {
            self.nextExecution();
        } else if (std.mem.eql(u8, command, "list") or std.mem.eql(u8, command, "l")) {
            try self.listSource(&args);
        } else if (std.mem.eql(u8, command, "print") or std.mem.eql(u8, command, "p")) {
            try self.printVariable(&args);
        } else if (std.mem.eql(u8, command, "watch") or std.mem.eql(u8, command, "w")) {
            try self.watchVariable(&args);
        } else if (std.mem.eql(u8, command, "info") or std.mem.eql(u8, command, "i")) {
            try self.infoCommand(&args);
        } else if (std.mem.eql(u8, command, "delete") or std.mem.eql(u8, command, "d")) {
            try self.deleteBreakpoint(&args);
        } else if (std.mem.eql(u8, command, "quit") or std.mem.eql(u8, command, "q")) {
            return false;
        } else {
            print("❌ Unknown command: {s}. Type 'help' for available commands.\n", .{command});
        }
        
        return true;
    }
    
    fn printHelp(self: *Self) void {
        _ = self;
        print("🐛 CURSED Debugger Commands:\n", .{});
        print("  help, h                 - Show this help\n", .{});
        print("  run, r                  - Run the program\n", .{});
        print("  break, b <line>         - Set breakpoint at line number\n", .{});
        print("  continue, c             - Continue execution\n", .{});
        print("  step, s                 - Step to next line\n", .{});
        print("  next, n                 - Step over (same as step)\n", .{});
        print("  list, l [line]          - List source code\n", .{});
        print("  print, p <var>          - Print variable (simulated)\n", .{});
        print("  watch, w <var>          - Watch variable (simulated)\n", .{});
        print("  info, i <topic>         - Show information (breakpoints)\n", .{});
        print("  delete, d <line>        - Delete breakpoint\n", .{});
        print("  quit, q                 - Exit debugger\n", .{});
    }
    
    fn runProgram(self: *Self) void {
        print("🏃 Running program...\n", .{});
        self.is_running = true;
        self.is_paused = true;
        self.current_line = 1;
        print("✅ Program started. Paused at line 1.\n", .{});
        print("💡 Use 'continue', 'step', or 'next' to control execution.\n", .{});
    }
    
    fn setBreakpoint(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const line_str = args.next() orelse {
            print("❌ Usage: break <line_number>\n", .{});
            return;
        };
        
        const line_number = std.fmt.parseInt(u32, line_str, 10) catch {
            print("❌ Invalid line number: {s}\n", .{line_str});
            return;
        };
        
        if (line_number == 0 or line_number > self.source_lines.items.len) {
            print("❌ Line {d} is out of range (1-{})\n", .{ line_number, self.source_lines.items.len });
            return;
        }
        
        try self.breakpoints.put(line_number, true);
        print("🔴 Breakpoint set at line {d}\n", .{line_number});
    }
    
    fn continueExecution(self: *Self) void {
        if (!self.is_running) {
            print("❌ Program is not running. Use 'run' first.\n", .{});
            return;
        }
        
        print("▶️ Continuing execution...\n", .{});
        
        // Find next breakpoint
        var next_bp: ?u32 = null;
        var check_line = self.current_line + 1;
        
        while (check_line <= self.source_lines.items.len) {
            if (self.breakpoints.get(check_line) == true) {
                next_bp = check_line;
                break;
            }
            check_line += 1;
        }
        
        if (next_bp) |bp_line| {
            self.current_line = bp_line;
            print("🔴 Breakpoint hit at line {d}\n", .{bp_line});
        } else {
            self.current_line = @intCast(self.source_lines.items.len);
            self.is_running = false;
            self.is_paused = false;
            print("🏁 Program execution completed\n", .{});
        }
    }
    
    fn stepExecution(self: *Self) void {
        if (!self.is_running) {
            print("❌ Program is not running. Use 'run' first.\n", .{});
            return;
        }
        
        if (self.current_line < self.source_lines.items.len) {
            self.current_line += 1;
            print("👣 Stepped to line {d}\n", .{self.current_line});
        } else {
            self.is_running = false;
            self.is_paused = false;
            print("🏁 End of program reached\n", .{});
        }
    }
    
    fn nextExecution(self: *Self) void {
        // Same as step for this simple debugger
        self.stepExecution();
    }
    
    fn listSource(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        var start_line: u32 = 1;
        
        if (args.next()) |line_str| {
            start_line = std.fmt.parseInt(u32, line_str, 10) catch {
                print("❌ Invalid line number: {s}\n", .{line_str});
                return;
            };
        } else if (self.current_line > 0) {
            start_line = if (self.current_line > 5) self.current_line - 5 else 1;
        }
        
        const end_line = @min(start_line + 9, @as(u32, @intCast(self.source_lines.items.len)));
        
        print("📄 Source code (lines {d}-{d}):\n", .{ start_line, end_line });
        
        var line_num = start_line;
        while (line_num <= end_line and line_num <= self.source_lines.items.len) : (line_num += 1) {
            const line_idx = line_num - 1;
            const line_content = self.source_lines.items[line_idx];
            
            const marker = if (line_num == self.current_line) "➤" else " ";
            const bp_marker = if (self.breakpoints.get(line_num) == true) "🔴" else " ";
            
            print("  {s}{s} {d:4}: {s}\n", .{ marker, bp_marker, line_num, line_content });
        }
    }
    
    fn printVariable(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const var_name = args.next() orelse {
            print("❌ Usage: print <variable_name>\n", .{});
            return;
        };
        
        // Simulate variable printing
        print("🔍 Variable '{s}': <simulated value>\n", .{var_name});
        print("  (Variable inspection requires full interpreter integration)\n", .{});
        _ = self;
    }
    
    fn watchVariable(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const var_name = args.next() orelse {
            self.listWatchVariables();
            return;
        };
        
        const name_copy = try self.allocator.dupe(u8, var_name);
        try self.watch_variables.append(self.allocator, name_copy);
        print("👁️ Now watching variable: {s}\n", .{var_name});
    }
    
    fn listWatchVariables(self: *Self) void {
        if (self.watch_variables.items.len == 0) {
            print("👁️ No variables being watched\n", .{});
            return;
        }
        
        print("👁️ Watched variables:\n", .{});
        for (self.watch_variables.items, 0..) |var_name, i| {
            print("  {d}: {s} = <simulated value>\n", .{ i + 1, var_name });
        }
    }
    
    fn infoCommand(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const topic = args.next() orelse {
            print("❌ Usage: info <topic> (breakpoints, watch)\n", .{});
            return;
        };
        
        if (std.mem.eql(u8, topic, "breakpoints")) {
            self.listBreakpoints();
        } else if (std.mem.eql(u8, topic, "watch")) {
            self.listWatchVariables();
        } else {
            print("❌ Unknown info topic: {s}\n", .{topic});
        }
    }
    
    fn listBreakpoints(self: *Self) void {
        if (self.breakpoints.count() == 0) {
            print("📍 No breakpoints set\n", .{});
            return;
        }
        
        print("📍 Breakpoints:\n", .{});
        var iterator = self.breakpoints.iterator();
        while (iterator.next()) |entry| {
            print("  Line {d}: enabled\n", .{entry.key_ptr.*});
        }
    }
    
    fn deleteBreakpoint(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const line_str = args.next() orelse {
            print("❌ Usage: delete <line_number>\n", .{});
            return;
        };
        
        const line_number = std.fmt.parseInt(u32, line_str, 10) catch {
            print("❌ Invalid line number: {s}\n", .{line_str});
            return;
        };
        
        if (self.breakpoints.remove(line_number)) {
            print("🗑️ Deleted breakpoint at line {d}\n", .{line_number});
        } else {
            print("❌ No breakpoint found at line {d}\n", .{line_number});
        }
    }
    
    fn showCurrentLocation(self: *Self) !void {
        if (self.current_line > 0 and self.current_line <= self.source_lines.items.len) {
            const line_content = self.source_lines.items[self.current_line - 1];
            print("📍 Current location: line {d}\n", .{self.current_line});
            print("  ➤ {d}: {s}\n", .{ self.current_line, line_content });
        }
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("Usage: {s} <source_file.csd>\n", .{args[0]});
        print("Example: {s} debug_test.csd\n", .{args[0]});
        return;
    }
    
    const source_file = args[1];
    
    var debugger = try StandaloneDebugger.init(allocator);
    defer debugger.deinit(allocator);
    
    try debugger.startSession(source_file);
}
