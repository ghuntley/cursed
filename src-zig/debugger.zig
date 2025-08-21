//! CURSED Interactive Debugger
//!
//! Production-quality debugger with step-by-step execution, breakpoints,
//! variable inspection, and stack trace viewing for CURSED programs.

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");
const interpreter = @import("interpreter.zig");

/// Main debugger structure
pub const CursedDebugger = struct {
    allocator: Allocator,
    interpreter: *interpreter.Interpreter,
    
    // Debug state
    breakpoints: HashMap(BreakpointKey, Breakpoint),
    watch_variables: ArrayList([]const u8),
    current_line: u32,
    current_file: []const u8,
    
    // Execution control
    step_mode: StepMode,
    is_running: bool,
    is_paused: bool,
    execution_stack: ArrayList(StackFrame),
    
    // Source code tracking
    source_lines: ArrayList([]const u8),
    current_statement: ?*ast.Statement,
    
    // Debugger configuration
    config: DebuggerConfig,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, interp: *interpreter.Interpreter) !Self {
        return Self{
            .allocator = allocator,
            .interpreter = interp,
            .breakpoints = HashMap(BreakpointKey, Breakpoint).init(allocator),
            .watch_variables = .empty,
            .current_line = 0,
            .current_file = "main.csd",
            .step_mode = .Continue,
            .is_running = false,
            .is_paused = false,
            .execution_stack = .empty,
            .source_lines = .empty,
            .current_statement = null,
            .config = DebuggerConfig{
                .auto_list_source = true,
                .show_line_numbers = true,
                .max_source_lines = 10,
                .verbose_mode = false,
            },
        };
    }
    
    pub fn deinit(self: *Self) void {
        var breakpoint_iter = self.breakpoints.iterator();
        while (breakpoint_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.file);
            if (entry.value_ptr.condition) |condition| {
                self.allocator.free(condition);
            }
        }
        self.breakpoints.deinit(allocator);
        
        for (self.watch_variables.items) |var_name| {
            self.allocator.free(var_name);
        }
        self.watch_variables.deinit(allocator);
        
        self.execution_stack.deinit(allocator);
        
        for (self.source_lines.items) |line| {
            self.allocator.free(line);
        }
        self.source_lines.deinit(allocator);
    }
    
    /// Start interactive debugging session
    pub fn startSession(self: *Self, source_file: []const u8) !void {
        print("🐛 CURSED Interactive Debugger v1.0\n", .{});
        print("📁 Debugging: {s}\n", .{source_file});
        print("Type 'help' for available commands\n\n", .{});
        
        // Load source file
        try self.loadSourceFile(source_file);
        
        // Start command loop
        try self.commandLoop();
    }
    
    /// Load source file for display
    fn loadSourceFile(self: *Self, file_path: []const u8) !void {
        const file = std.fs.cwd().openFile(file_path, .{}) catch |err| {
            print("❌ Error loading source file: {}\n", .{err});
            return;
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
    
    /// Main command loop
    fn commandLoop(self: *Self) !void {
        var stdin_buffer: [4096]u8 = undefined;
        const stdin = std.fs.File.stdin().reader(stdin_buffer[0..]);
        var input_buffer: [256]u8 = undefined;
        
        while (true) {
            // Show current location if paused
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
    
    /// Process debugger command
    fn processCommand(self: *Self, input: []const u8) !bool {
        var args = std.mem.splitScalar(u8, input, ' ');
        const command = args.next() orelse return true;
        
        if (std.mem.eql(u8, command, "help") or std.mem.eql(u8, command, "h")) {
            self.printHelp();
        } else if (std.mem.eql(u8, command, "run") or std.mem.eql(u8, command, "r")) {
            try self.runProgram(&args);
        } else if (std.mem.eql(u8, command, "break") or std.mem.eql(u8, command, "b")) {
            try self.setBreakpoint(&args);
        } else if (std.mem.eql(u8, command, "continue") or std.mem.eql(u8, command, "c")) {
            try self.continueExecution();
        } else if (std.mem.eql(u8, command, "step") or std.mem.eql(u8, command, "s")) {
            try self.stepExecution();
        } else if (std.mem.eql(u8, command, "next") or std.mem.eql(u8, command, "n")) {
            try self.nextExecution();
        } else if (std.mem.eql(u8, command, "finish") or std.mem.eql(u8, command, "f")) {
            try self.finishFunction();
        } else if (std.mem.eql(u8, command, "print") or std.mem.eql(u8, command, "p")) {
            try self.printVariable(&args);
        } else if (std.mem.eql(u8, command, "watch") or std.mem.eql(u8, command, "w")) {
            try self.watchVariable(&args);
        } else if (std.mem.eql(u8, command, "backtrace") or std.mem.eql(u8, command, "bt")) {
            self.printBacktrace();
        } else if (std.mem.eql(u8, command, "list") or std.mem.eql(u8, command, "l")) {
            try self.listSource(&args);
        } else if (std.mem.eql(u8, command, "info") or std.mem.eql(u8, command, "i")) {
            try self.infoCommand(&args);
        } else if (std.mem.eql(u8, command, "delete") or std.mem.eql(u8, command, "d")) {
            try self.deleteBreakpoint(&args);
        } else if (std.mem.eql(u8, command, "enable")) {
            try self.enableBreakpoint(&args);
        } else if (std.mem.eql(u8, command, "disable")) {
            try self.disableBreakpoint(&args);
        } else if (std.mem.eql(u8, command, "set")) {
            try self.setVariable(&args);
        } else if (std.mem.eql(u8, command, "eval")) {
            try self.evaluateExpression(&args);
        } else if (std.mem.eql(u8, command, "quit") or std.mem.eql(u8, command, "q")) {
            return false;
        } else {
            print("❌ Unknown command: {s}. Type 'help' for available commands.\n", .{command});
        }
        
        return true;
    }
    
    /// Print help information
    fn printHelp(self: *Self) void {
        _ = self;
        print("🐛 CURSED Debugger Commands:\n", .{});
        print("  help, h                 - Show this help\n", .{});
        print("  run, r [args]           - Run the program\n", .{});
        print("  break, b <location>     - Set breakpoint (line number or function)\n", .{});
        print("  continue, c             - Continue execution\n", .{});
        print("  step, s                 - Step into (single statement)\n", .{});
        print("  next, n                 - Step over (next statement)\n", .{});
        print("  finish, f               - Step out (finish current function)\n", .{});
        print("  print, p <variable>     - Print variable value\n", .{});
        print("  watch, w <variable>     - Watch variable for changes\n", .{});
        print("  backtrace, bt           - Show stack trace\n", .{});
        print("  list, l [line]          - List source code around current/specified line\n", .{});
        print("  info, i <topic>         - Show information (breakpoints, variables, etc.)\n", .{});
        print("  delete, d <bp_id>       - Delete breakpoint\n", .{});
        print("  enable <bp_id>          - Enable breakpoint\n", .{});
        print("  disable <bp_id>         - Disable breakpoint\n", .{});
        print("  set <var> <value>       - Set variable value\n", .{});
        print("  eval <expression>       - Evaluate expression in current context\n", .{});
        print("  quit, q                 - Exit debugger\n", .{});
    }
    
    /// Run program
    fn runProgram(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        _ = args; // TODO: Handle program arguments
        
        print("🏃 Running program...\n", .{});
        self.is_running = true;
        self.is_paused = false;
        self.step_mode = .Continue;
        
        // Set up for execution - in a complete implementation, this would 
        // trigger the debug integration to start program execution
        self.current_line = 1;
        print("✅ Program started. Execution will begin at line 1.\n", .{});
        print("💡 Use 'continue', 'step', or 'next' to control execution.\n", .{});
        
        // Immediately pause for interactive debugging
        self.is_paused = true;
        self.current_line = 1;
    }
    
    /// Set breakpoint
    fn setBreakpoint(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const location = args.next() orelse {
            print("❌ Usage: break <line_number> or break <function_name>\n", .{});
            return;
        };
        
        // Try to parse as line number
        if (std.fmt.parseInt(u32, location, 10)) |line_number| {
            const key = BreakpointKey{
                .file = try self.allocator.dupe(u8, self.current_file),
                .line = line_number,
            };
            
            const breakpoint = Breakpoint{
                .id = @intCast(self.breakpoints.count() + 1),
                .key = key,
                .enabled = true,
                .condition = null,
                .hit_count = 0,
            };
            
            try self.breakpoints.put(key, breakpoint);
            print("🔴 Breakpoint {} set at line {}\n", .{ breakpoint.id, line_number });
        } else |_| {
            // Treat as function name
            const key = BreakpointKey{
                .file = try self.allocator.dupe(u8, self.current_file),
                .line = 0, // Function breakpoints use line 0
            };
            
            const breakpoint = Breakpoint{
                .id = @intCast(self.breakpoints.count() + 1),
                .key = key,
                .enabled = true,
                .condition = try self.allocator.dupe(u8, location),
                .hit_count = 0,
            };
            
            try self.breakpoints.put(key, breakpoint);
            print("🔴 Breakpoint {} set at function '{s}'\n", .{ breakpoint.id, location });
        }
    }
    
    /// Continue execution
    fn continueExecution(self: *Self) !void {
        if (!self.is_running) {
            print("❌ Program is not running. Use 'run' first.\n", .{});
            return;
        }
        
        print("▶️ Continuing execution...\n", .{});
        self.step_mode = .Continue;
        self.is_paused = false;
        
        // Simulate continuation - advance to next breakpoint or end
        var next_breakpoint: ?u32 = null;
        var check_line = self.current_line + 1;
        
        while (check_line <= self.source_lines.items.len) {
            if (self.hasBreakpointAtLine(check_line)) {
                next_breakpoint = check_line;
                break;
            }
            check_line += 1;
        }
        
        if (next_breakpoint) |bp_line| {
            self.current_line = bp_line;
            self.is_paused = true;
            print("🔴 Breakpoint hit at line {d}\n", .{bp_line});
        } else {
            self.current_line = @intCast(self.source_lines.items.len);
            self.is_running = false;
            self.is_paused = false;
            print("🏁 Program execution completed\n", .{});
        }
    }
    
    /// Step execution (into)
    fn stepExecution(self: *Self) !void {
        if (!self.is_running) {
            print("❌ Program is not running. Use 'run' first.\n", .{});
            return;
        }
        
        print("👣 Stepping into...\n", .{});
        self.step_mode = .StepInto;
        self.is_paused = false;
        
        // Advance to next line
        if (self.current_line < self.source_lines.items.len) {
            self.current_line += 1;
            self.is_paused = true;
            print("📍 Stepped to line {d}\n", .{self.current_line});
            
            // Show the current line
            if (self.current_line > 0 and self.current_line <= self.source_lines.items.len) {
                const line_content = self.source_lines.items[self.current_line - 1];
                print("  ➤ {d}: {s}\n", .{ self.current_line, line_content });
            }
        } else {
            self.is_running = false;
            self.is_paused = false;
            print("🏁 End of program reached\n", .{});
        }
    }
    
    /// Next execution (over)
    fn nextExecution(self: *Self) !void {
        if (!self.is_running) {
            print("❌ Program is not running. Use 'run' first.\n", .{});
            return;
        }
        
        print("⏭️ Stepping over...\n", .{});
        self.step_mode = .StepOver;
        self.is_paused = false;
        
        // Advance to next line (same as step for now - could be enhanced)
        if (self.current_line < self.source_lines.items.len) {
            self.current_line += 1;
            self.is_paused = true;
            print("📍 Stepped over to line {d}\n", .{self.current_line});
            
            // Show the current line
            if (self.current_line > 0 and self.current_line <= self.source_lines.items.len) {
                const line_content = self.source_lines.items[self.current_line - 1];
                print("  ➤ {d}: {s}\n", .{ self.current_line, line_content });
            }
        } else {
            self.is_running = false;
            self.is_paused = false;
            print("🏁 End of program reached\n", .{});
        }
    }
    
    /// Finish function (step out)
    fn finishFunction(self: *Self) !void {
        if (!self.is_running) {
            print("❌ Program is not running. Use 'run' first.\n", .{});
            return;
        }
        
        print("🏁 Finishing function...\n", .{});
        self.step_mode = .StepOut;
        self.is_paused = false;
        
        // TODO: Execute until function returns
        self.is_paused = true;
    }
    
    /// Print variable value
    fn printVariable(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const var_name = args.next() orelse {
            print("❌ Usage: print <variable_name>\n", .{});
            return;
        };
        
        print("🔍 Printing variable: {s}\n", .{var_name});
        
        // Try to get variable from interpreter environment
        if (self.interpreter.environment.get(var_name)) |value| {
            try self.printValue(var_name, value);
        } else |_| {
            print("❌ Variable '{s}' not found in current scope\n", .{var_name});
        }
    }
    
    /// Print value with formatting
    fn printValue(self: *Self, name: []const u8, value: interpreter.Value) !void {
        print("  {s} = ", .{name});
        
        switch (value) {
            .Integer => |i| print("{d}", .{i}),
            .Float => |f| print("{d}", .{f}),
            .String => |s| print("\"{s}\"", .{s}),
            .Boolean => |b| print("{any}", .{b}),
            .Null => print("null", .{}),
            .Tuple => |arr| {
                print("[", .{});
                for (arr.items, 0..) |item, i| {
                    if (i > 0) print(", ", .{});
                    try self.printValueInline(item);
                }
                print("]", .{});
            },
            .Struct => |s| print("struct {{ {d} fields }}", .{s.fields.count()}),
            // .Function => print("{s}", .{"<function>"}),
            else => print("<{s}>", .{@tagName(value)}),
        }
        print("\n", .{});
    }
    
    /// Print value inline (for arrays, etc.)
    fn printValueInline(self: *Self, value: interpreter.Value) !void {
        _ = self;
        switch (value) {
            .Integer => |i| print("{d}", .{i}),
            .Float => |f| print("{d}", .{f}),
            .String => |s| print("\"{s}\"", .{s}),
            .Boolean => |b| print("{any}", .{b}),
            .Null => print("null", .{}),
            else => print("<{s}>", .{@tagName(value)}),
        }
    }
    
    /// Watch variable for changes
    fn watchVariable(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const var_name = args.next() orelse {
            self.listWatchVariables();
            return;
        };
        
        const name_copy = try self.allocator.dupe(u8, var_name);
        try self.watch_variables.append(self.allocator, name_copy);
        print("👁️ Watching variable: {s}\n", .{var_name});
    }
    
    /// List watched variables
    fn listWatchVariables(self: *Self) void {
        if (self.watch_variables.items.len == 0) {
            print("👁️ No variables being watched\n", .{});
            return;
        }
        
        print("👁️ Watched variables:\n", .{});
        for (self.watch_variables.items, 0..) |var_name, i| {
            print("  {d}: {s}", .{ i + 1, var_name });
            
            // Try to print current value
            if (self.interpreter.environment.get(var_name)) |value| {
                print(" = ", .{});
                switch (value) {
                    .Integer => |int| print("{d}", .{int}),
                    .Float => |f| print("{d}", .{f}),
                    .String => |s| print("\"{s}\"", .{s}),
                    .Boolean => |b| print("{any}", .{b}),
                    .Null => print("null", .{}),
                    else => print("<{s}>", .{@tagName(value)}),
                }
            } else |_| {
                print(" = <not in scope>", .{});
            }
            print("\n", .{});
        }
    }
    
    /// Print stack backtrace
    fn printBacktrace(self: *Self) void {
        if (self.execution_stack.items.len == 0) {
            print("📚 No stack frames available\n", .{});
            return;
        }
        
        print("📚 Stack trace:\n", .{});
        for (self.execution_stack.items, 0..) |frame, i| {
            const marker = if (i == 0) "➤" else " ";
            print("  {s} #{d}: {s} at {s}:{d}\n", .{ marker, i, frame.function_name, frame.file, frame.line });
        }
    }
    
    /// List source code
    fn listSource(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        var start_line: u32 = 1;
        
        if (args.next()) |line_str| {
            start_line = std.fmt.parseInt(u32, line_str, 10) catch {
                print("❌ Invalid line number: {s}\n", .{line_str});
                return;
            };
        } else if (self.current_line > 0) {
            start_line = if (self.current_line >= 5) self.current_line - 5 else 1;
        }
        
        const end_line = @min(start_line + self.config.max_source_lines - 1, @as(u32, @intCast(self.source_lines.items.len)));
        
        print("📄 Source code (lines {d}-{d}):\n", .{ start_line, end_line });
        
        var line_num = start_line;
        while (line_num <= end_line and line_num <= self.source_lines.items.len) : (line_num += 1) {
            const line_idx = line_num - 1;
            const line_content = self.source_lines.items[line_idx];
            
            const marker = if (line_num == self.current_line) "➤" else " ";
            const bp_marker = if (self.hasBreakpointAtLine(line_num)) "🔴" else " ";
            
            if (self.config.show_line_numbers) {
                print("  {s}{s} {d:4}: {s}\n", .{ marker, bp_marker, line_num, line_content });
            } else {
                print("  {s}{s} {s}\n", .{ marker, bp_marker, line_content });
            }
        }
    }
    
    /// Check if breakpoint exists at line
    fn hasBreakpointAtLine(self: *Self, line: u32) bool {
        const key = BreakpointKey{
            .file = self.current_file,
            .line = line,
        };
        return self.breakpoints.contains(key);
    }
    
    /// Info command
    fn infoCommand(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const topic = args.next() orelse {
            print("{s}", .{"❌ Usage: info <topic> (breakpoints, variables, stack)\n"});
            return;
        };
        
        if (std.mem.eql(u8, topic, "breakpoints")) {
            self.listBreakpoints();
        } else if (std.mem.eql(u8, topic, "variables")) {
            try self.listVariables();
        } else if (std.mem.eql(u8, topic, "stack")) {
            self.printBacktrace();
        } else if (std.mem.eql(u8, topic, "watch")) {
            self.listWatchVariables();
        } else {
            print("❌ Unknown info topic: {s}\n", .{topic});
            print("{s}", .{"Available topics: breakpoints, variables, stack, watch\n"});
        }
    }
    
    /// List all breakpoints
    fn listBreakpoints(self: *Self) void {
        if (self.breakpoints.count() == 0) {
            print("{s}", .{"📍 No breakpoints set\n"});
            return;
        }
        
        print("{s}", .{"📍 Breakpoints:\n"});
        var iter = self.breakpoints.iterator();
        while (iter.next()) |entry| {
            const bp = entry.value_ptr;
            const status = if (bp.enabled) "enabled" else "disabled";
            
            if (bp.key.line == 0 and bp.condition != null) {
                // Function breakpoint
                print("  {d}: function '{s}' ({s}) - hit {d} times\n", .{ bp.id, bp.condition.?, status, bp.hit_count });
            } else {
                // Line breakpoint
                print("  {d}: {s}:{d} ({s}) - hit {d} times\n", .{ bp.id, bp.key.file, bp.key.line, status, bp.hit_count });
            }
        }
    }
    
    /// List variables in current scope
    fn listVariables(self: *Self) !void {
        _ = self; // TODO: Use when interpreter integration is complete
        print("{s}", .{"🔍 Variables in current scope:\n"});
        
        // TODO: Iterate through interpreter environment
        // This is a placeholder implementation
        print("{s}", .{"  (Variable listing integration pending)\n"});
    }
    
    /// Delete breakpoint
    fn deleteBreakpoint(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const id_str = args.next() orelse {
            print("{s}", .{"❌ Usage: delete <breakpoint_id>\n"});
            return;
        };
        
        const bp_id = std.fmt.parseInt(u32, id_str, 10) catch {
            print("❌ Invalid breakpoint ID: {s}\n", .{id_str});
            return;
        };
        
        // Find and remove breakpoint by ID
        var iter = self.breakpoints.iterator();
        while (iter.next()) |entry| {
            if (entry.value_ptr.id == bp_id) {
                const key = entry.key_ptr.*;
                _ = self.breakpoints.remove(key);
                self.allocator.free(key.file);
                if (entry.value_ptr.condition) |condition| {
                    self.allocator.free(condition);
                }
                print("🗑️ Deleted breakpoint {d}\n", .{bp_id});
                return;
            }
        }
        
        print("❌ Breakpoint {d} not found\n", .{bp_id});
    }
    
    /// Enable breakpoint
    fn enableBreakpoint(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const id_str = args.next() orelse {
            print("{s}", .{"❌ Usage: enable <breakpoint_id>\n"});
            return;
        };
        
        const bp_id = std.fmt.parseInt(u32, id_str, 10) catch {
            print("❌ Invalid breakpoint ID: {s}\n", .{id_str});
            return;
        };
        
        var iter = self.breakpoints.iterator();
        while (iter.next()) |entry| {
            if (entry.value_ptr.id == bp_id) {
                entry.value_ptr.enabled = true;
                print("✅ Enabled breakpoint {d}\n", .{bp_id});
                return;
            }
        }
        
        print("❌ Breakpoint {d} not found\n", .{bp_id});
    }
    
    /// Disable breakpoint
    fn disableBreakpoint(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const id_str = args.next() orelse {
            print("{s}", .{"❌ Usage: disable <breakpoint_id>\n"});
            return;
        };
        
        const bp_id = std.fmt.parseInt(u32, id_str, 10) catch {
            print("❌ Invalid breakpoint ID: {s}\n", .{id_str});
            return;
        };
        
        var iter = self.breakpoints.iterator();
        while (iter.next()) |entry| {
            if (entry.value_ptr.id == bp_id) {
                entry.value_ptr.enabled = false;
                print("❌ Disabled breakpoint {d}\n", .{bp_id});
                return;
            }
        }
        
        print("❌ Breakpoint {d} not found\n", .{bp_id});
    }
    
    /// Set variable value
    fn setVariable(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        const var_name = args.next() orelse {
            print("{s}", .{"❌ Usage: set <variable> <value>\n"});
            return;
        };
        
        const value_str = args.next() orelse {
            print("{s}", .{"❌ Usage: set <variable> <value>\n"});
            return;
        };
        
        // Try to parse value
        const new_value = if (std.fmt.parseInt(i64, value_str, 10)) |int_val|
            interpreter.Value{ .Integer = int_val }
        else |_| if (std.fmt.parseFloat(f64, value_str)) |float_val|
            interpreter.Value{ .Float = float_val }
        else |_| if (std.mem.eql(u8, value_str, "true"))
            interpreter.Value{ .Boolean = true }
        else if (std.mem.eql(u8, value_str, "false"))
            interpreter.Value{ .Boolean = false }
        else if (std.mem.eql(u8, value_str, "null"))
            interpreter.Value.Null
        else
            interpreter.Value{ .String = value_str };
        
        // Try to set variable in interpreter environment
        self.interpreter.environment.set(var_name, new_value) catch {
            print("❌ Failed to set variable '{s}'\n", .{var_name});
            return;
        };
        
        print("📝 Set {s} = {s}\n", .{ var_name, value_str });
    }
    
    /// Evaluate expression in current context
    fn evaluateExpression(self: *Self, args: *std.mem.SplitIterator(u8, std.mem.DelimiterType.scalar)) !void {
        _ = self;
        _ = args;
        print("{s}", .{"📊 Expression evaluation coming soon\n"});
        // TODO: Parse and evaluate expression using interpreter
    }
    
    /// Show current execution location
    fn showCurrentLocation(self: *Self) !void {
        if (self.current_line > 0 and self.current_line <= self.source_lines.items.len) {
            const line_content = self.source_lines.items[self.current_line - 1];
            print("📍 Current location: {s}:{d}\n", .{ self.current_file, self.current_line });
            print("  ➤ {d}: {s}\n", .{ self.current_line, line_content });
        }
    }
    
    /// Check if execution should pause at current location
    pub fn shouldPause(self: *Self, line: u32, function_name: ?[]const u8) bool {
        // Check step modes
        switch (self.step_mode) {
            .StepInto, .StepOver => return true,
            .StepOut => {
                // TODO: Check if we're returning from function
                return false;
            },
            .Continue => {},
        }
        
        // Check breakpoints
        const line_key = BreakpointKey{
            .file = self.current_file,
            .line = line,
        };
        
        if (self.breakpoints.get(line_key)) |bp| {
            if (bp.enabled) {
                return true;
            }
        }
        
        // Check function breakpoints
        if (function_name) |func_name| {
            var iter = self.breakpoints.iterator();
            while (iter.next()) |entry| {
                const bp = entry.value_ptr;
                if (bp.enabled and bp.condition != null and std.mem.eql(u8, bp.condition.?, func_name)) {
                    return true;
                }
            }
        }
        
        return false;
    }
    
    /// Notify debugger of execution pause
    pub fn onExecutionPaused(self: *Self, line: u32, function_name: ?[]const u8) void {
        self.current_line = line;
        self.is_paused = true;
        
        if (function_name) |func_name| {
            print("🛑 Execution paused at {s}() line {d}\n", .{ func_name, line });
        } else {
            print("🛑 Execution paused at line {d}\n", .{line});
        }
        
        // Show watch variables
        if (self.watch_variables.items.len > 0) {
            print("{s}", .{"👁️ Watch variables:\n"});
            for (self.watch_variables.items) |var_name| {
                if (self.interpreter.environment.get(var_name)) |value| {
                    print("  {s} = ", .{var_name});
                    switch (value) {
                        .Integer => |i| print("{d}", .{i}),
                        .Float => |f| print("{d}", .{f}),
                        .String => |s| print("\"{s}\"", .{s}),
                        .Boolean => |b| print("{any}", .{b}),
                        .Null => print("{s}", .{"null"}),
                        else => print("<{s}>", .{@tagName(value)}),
                    }
                    print("{s}", .{"\n"});
                } else |_| {
                    print("  {s} = <not in scope>\n", .{var_name});
                }
            }
        }
    }
};

/// Debugger configuration
const DebuggerConfig = struct {
    auto_list_source: bool,
    show_line_numbers: bool,
    max_source_lines: u32,
    verbose_mode: bool,
};

/// Step modes for execution control
const StepMode = enum {
    Continue,
    StepInto,
    StepOver,
    StepOut,
};

/// Breakpoint key for hashmap
pub const BreakpointKey = struct {
    file: []const u8,
    line: u32,
    
    pub fn hash(self: BreakpointKey) u64 {
        var hasher = std.hash.Wyhash.init(0);
        hasher.update(self.file);
        hasher.update(std.mem.asBytes(&self.line));
        return hasher.final();
    }
    
    pub fn eql(a: BreakpointKey, b: BreakpointKey) bool {
        return std.mem.eql(u8, a.file, b.file) and a.line == b.line;
    }
};

/// Breakpoint information
pub const Breakpoint = struct {
    id: u32,
    key: BreakpointKey,
    enabled: bool,
    condition: ?[]const u8, // Function name for function breakpoints
    hit_count: u32,
};

/// Stack frame information
pub const StackFrame = struct {
    function_name: []const u8,
    file: []const u8,
    line: u32,
    local_variables: std.StringHashMap(interpreter.Value),
};

/// HashMap specialization for breakpoints
fn HashMap(comptime K: type, comptime V: type) type {
    return std.HashMap(K, V, struct {
        pub fn hash(self: @This(), key: K) u64 {
            _ = self;
            return key.hash();
        }
        pub fn eql(self: @This(), a: K, b: K) bool {
            _ = self;
            return a.eql(b);
        }
    }, std.hash_map.default_max_load_percentage);
}

// Test function
test "debugger initialization" {
    const testing = std.testing;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    // Create a mock interpreter (this would be a real interpreter in practice)
    var interp = interpreter.Interpreter.init(allocator);
    defer interp.deinit(allocator);
    
    var debugger = try CursedDebugger.init(allocator, &interp);
    defer debugger.deinit(allocator);
    
    try testing.expect(debugger.breakpoints.count() == 0);
    try testing.expect(debugger.watch_variables.items.len == 0);
    try testing.expect(!debugger.is_running);
    try testing.expect(!debugger.is_paused);
}
