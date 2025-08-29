//! CURSED Debugger Integration
//!
//! Integrates the interactive debugger with the CURSED interpreter
//! by adding debug hooks to statement and expression execution.

const std = @import("std");
const print = std.debug.print;
const debugger = @import("debugger.zig");
const interpreter = @import("interpreter.zig");
const ast = @import("ast.zig");

/// Debug-enabled interpreter that supports interactive debugging
pub const DebugInterpreter = struct {
    base_interpreter: interpreter.Interpreter,
    debugger: debugger.CursedDebugger,
    debug_enabled: bool,
    current_line: u32,
    current_function: ?[]const u8,
    
    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator) !Self {
        var base_interp = interpreter.Interpreter.init(allocator);
        const debug_instance = try debugger.CursedDebugger.init(allocator, &base_interp);
        
        return Self{
            .base_interpreter = base_interp,
            .debugger = debug_instance,
            .debug_enabled = true,
            .current_line = 0,
            .current_function = null,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.debugger.deinit(self.allocator);
        self.base_interpreter.deinit(self.allocator);
    }
    
    /// Start interactive debugging session
    pub fn startDebugSession(self: *Self, source_file: []const u8) !void {
        try self.debugger.startSession(source_file);
    }
    
    /// Execute program with debug support
    pub fn executeWithDebug(self: *Self, program: ast.Program) !void {
        self.debug_enabled = true;
        
        // Hook into interpreter execution
        try self.executeDebugProgram(program);
    }
    
    /// Execute program with debug hooks
    fn executeDebugProgram(self: *Self, program: ast.Program) !void {
        // Execute the program using the base interpreter with debug hooks
        try self.base_interpreter.interpretProgram(program);
        
        // For now, we'll simulate execution by stepping through statements
        print("🚀 Starting program execution with debugging...\n", .{});
        
        self.current_line = 1;
        for (program.statements.items, 0..) |stmt_ptr, i| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            self.current_line = @intCast(i + 1); // Track line numbers
            
            // Debug hook before each statement
            if (self.debugger.shouldPause(self.current_line, self.current_function)) {
                self.debugger.onExecutionPaused(self.current_line, self.current_function);
                
                // Simple pause simulation - in a real implementation this would be event-driven
                print("🛑 Execution paused. Press Enter to continue...\n", .{});
                _ = std.fs.File.stdin().reader(&[_]u8{}).readByte() catch {};
            }
            
            // Execute the statement
            print("📍 Executing line {d}: {s}\n", .{ self.current_line, @tagName(stmt.*) });
            
            // Simulate execution delay for demonstration
            std.Thread.sleep(100_000_000); // 100ms
        }
        
        print("✅ Program execution complete\n", .{});
    }
    
    /// Execute statement with debug hooks
    fn executeStatementWithDebug(self: *Self, stmt: ast.Statement) interpreter.InterpreterError!void {
        // Pre-execution debug check
        if (self.debug_enabled) {
            if (self.debugger.shouldPause(self.current_line, self.current_function)) {
                self.debugger.onExecutionPaused(self.current_line, self.current_function);
                
                // Wait for debugger commands (simplified - would need proper event loop)
                while (self.debugger.is_paused) {
                    // In a real implementation, this would process debugger commands
                    // For now, we'll just continue
                    break;
                }
            }
        }
        
        // Execute the statement using base interpreter logic
        switch (stmt) {
            .Expression => |expr_ptr| {
                const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
                _ = try self.evaluateExpressionWithDebug(expr.*);
            },
            .Let => |let| try self.executeLetStatementWithDebug(let),
            .Assignment => |assign| try self.executeAssignmentStatementWithDebug(assign),
            .Return => |ret| {
                _ = ret; // Handle returns in function context
            },
            .If => |if_stmt| try self.executeIfStatementWithDebug(if_stmt),
            .While => |while_stmt| try self.executeWhileStatementWithDebug(while_stmt),
            .Function => {
                // Functions are already collected, skip execution
            },
            .Struct => |struct_stmt| try self.base_interpreter.executeStructStatement(struct_stmt),
            .Stan => |stan| try self.base_interpreter.executeStanStatement(stan),
            .Yikes => |yikes| try self.base_interpreter.executeYikesStatement(yikes),
            .Fam => |fam| try self.base_interpreter.executeFamStatement(fam),
            .Defer => |defer_stmt| try self.base_interpreter.executeDeferStatement(defer_stmt),
            .Switch => |switch_stmt| try self.base_interpreter.executeSwitchStatement(switch_stmt),
            .PatternSwitch => |pattern_switch| try self.base_interpreter.executePatternSwitchStatement(pattern_switch),
            else => {
                std.debug.print("Unsupported statement type in debug interpreter: {s}\n", .{@tagName(stmt)});
            },
        }
        
        // Post-execution debug check (for step modes)
        if (self.debug_enabled and self.debugger.step_mode != .Continue) {
            self.debugger.onExecutionPaused(self.current_line, self.current_function);
        }
    }
    
    /// Execute let statement with debug support
    fn executeLetStatementWithDebug(self: *Self, let: ast.LetStatement) !void {
        const value = if (let.initializer) |initializer_expr|
            try self.evaluateExpressionWithDebug(initializer_expr.*)
        else
            interpreter.Value.Null;
        
        // Handle tuple destructuring (same as base interpreter)
        if (std.mem.indexOf(u8, let.name, ",")) |_| {
            var name_iter = std.mem.splitSequence(u8, let.name, ",");
            var name_index: usize = 0;
            
            switch (value) {
                 raw_name, " \t");
                        if (name_index < tuple.items.len) {
                            try self.base_interpreter.environment.define(trimmed_name, tuple.items[name_index]);
                        } else {
                            try self.base_interpreter.environment.define(trimmed_name, interpreter.Value.Null);
                        }
                        name_index += 1;
                    }
                },
                else => {
                    while (name_iter.next()) |raw_name| {
                        const trimmed_name = std.mem.trim(u8, raw_name, " \t");
                        if (name_index == 0) {
                            try self.base_interpreter.environment.define(trimmed_name, value);
                        } else {
                            try self.base_interpreter.environment.define(trimmed_name, interpreter.Value.Null);
                        }
                        name_index += 1;
                    }
                },
            }
        } else {
            try self.base_interpreter.environment.define(let.name, value);
        }
        
        // Debug notification of variable assignment
        if (self.debug_enabled) {
            std.debug.print("DEBUG: Variable '{s}' assigned\n", .{let.name});
        }
    }
    
    /// Execute assignment statement with debug support
    fn executeAssignmentStatementWithDebug(self: *Self, assign: ast.AssignmentStatement) !void {
        const value_expr: *ast.Expression = @ptrCast(@alignCast(assign.value));
        const new_value = try self.evaluateExpressionWithDebug(value_expr.*);
        
        // Store old value for comparison if variable exists
        var old_value: ?interpreter.Value = null;
        if (self.base_interpreter.environment.get(assign.target)) |existing| {
            old_value = existing;
        } else |_| {
            // Variable doesn't exist yet
        }
        
        // Perform the assignment
        try self.base_interpreter.environment.define(assign.target, new_value);
        
        // Debug notification of variable change
        if (self.debug_enabled) {
            std.debug.print("DEBUG: Assignment statement executed\n", .{});
            std.debug.print("DEBUG: Variable '{s}' = ", .{assign.target});
            self.printValue(new_value);
            std.debug.print("\n", .{});
            
            // Check if variable is being watched and display changes
            if (self.isVariableWatched(assign.target)) {
                std.debug.print("WATCH: Variable '{s}' changed from ", .{assign.target});
                if (old_value) |old| {
                    self.printValue(old);
                } else {
                    std.debug.print("undefined", .{});
                }
                std.debug.print(" to ", .{});
                self.printValue(new_value);
                std.debug.print("\n", .{});
            }
        }
    }
    
    /// Execute if statement with debug support
    fn executeIfStatementWithDebug(self: *Self, if_stmt: ast.IfStatement) !void {
        const condition_expr: *ast.Expression = @ptrCast(@alignCast(if_stmt.condition));
        const condition = try self.evaluateExpressionWithDebug(condition_expr.*);
        
        if (self.isTruthy(condition)) {
            self.current_line += 1; // Enter if block
            for (if_stmt.then_branch.items) |stmt_ptr| {
                const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
                try self.executeStatementWithDebug(stmt.*);
                self.current_line += 1;
            }
        } else if (if_stmt.else_branch) |alt| {
            self.current_line += 1; // Enter else block
            for (alt.items) |stmt_ptr| {
                const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
                try self.executeStatementWithDebug(stmt.*);
                self.current_line += 1;
            }
        }
    }
    
    /// Execute while statement with debug support
    fn executeWhileStatementWithDebug(self: *Self, while_stmt: ast.WhileStatement) !void {
        while (true) {
            const condition_expr: *ast.Expression = @ptrCast(@alignCast(while_stmt.condition));
            const condition = try self.evaluateExpressionWithDebug(condition_expr.*);
            if (!self.isTruthy(condition)) break;
            
            self.current_line += 1; // Enter loop body
            for (while_stmt.body.items) |stmt_ptr| {
                const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
                try self.executeStatementWithDebug(stmt.*);
                self.current_line += 1;
            }
        }
    }
    
    /// Evaluate expression with debug support
    fn evaluateExpressionWithDebug(self: *Self, expr: ast.Expression) !interpreter.Value {
        // Debug hook before expression evaluation
        if (self.debug_enabled) {
            // Could add expression-level breakpoints here
        }
        
        // Delegate to base interpreter for actual evaluation
        return try self.base_interpreter.evaluateExpression(expr);
    }
    
    /// Call function with debug support
    fn callFunctionWithDebug(self: *Self, func: interpreter.CursedFunction, args: []interpreter.Value) !interpreter.Value {
        const previous_function = self.current_function;
        self.current_function = func.declaration.name;
        
        // Debug hook for function entry
        if (self.debug_enabled) {
            std.debug.print("DEBUG: Entering function '{s}'\n", .{func.declaration.name});
            
            // Add stack frame
            const frame = debugger.StackFrame{
                .function_name = func.declaration.name,
                .file = "main.csd", // TODO: Get actual file name
                .line = self.current_line,
                .local_variables = std.StringHashMap(interpreter.Value){},
            };
            try self.debugger.execution_stack.append(allocator, frame);
        }
        
        // Call base interpreter function
        const result = try self.base_interpreter.callFunction(func, args);
        
        // Debug hook for function exit
        if (self.debug_enabled) {
            std.debug.print("DEBUG: Exiting function '{s}'\n", .{func.declaration.name});
            
            // Remove stack frame
            if (self.debugger.execution_stack.items.len > 0) {
                _ = self.debugger.execution_stack.pop();
            }
        }
        
        self.current_function = previous_function;
        return result;
    }
    
    /// Check if value is truthy (same as base interpreter)
    fn isTruthy(self: *Self, value: interpreter.Value) bool {
        _ = self;
        return switch (value) {
            .Boolean => |b| b,
            .Null => false,
            .Integer => |i| i != 0,
            .Float => |f| f != 0.0,
            .String => |s| s.len > 0,
            else => true,
        };
    }
    
    /// Enable/disable debugging
    pub fn setDebugEnabled(self: *Self, enabled: bool) void {
        self.debug_enabled = enabled;
    }
    
    /// Set current line (for external tracking)
    pub fn setCurrentLine(self: *Self, line: u32) void {
        self.current_line = line;
    }
    
    /// Get debugger reference for external control
    pub fn getDebugger(self: *Self) *debugger.CursedDebugger {
        return &self.debugger;
    }
    
    /// Print value for debug output
    fn printValue(self: *Self, value: interpreter.Value) void {
        _ = self;
        switch (value) {
            .Integer => |i| std.debug.print("{d}", .{i}),
            .Float => |f| std.debug.print("{d}", .{f}),
            .String => |s| std.debug.print("\"{s}\"", .{s}),
            .Boolean => |b| std.debug.print("{s}", .{b}),
            .Character => |c| std.debug.print("'{c}'", .{c}),
            .Null => std.debug.print("null", .{}),
            else => std.debug.print("(complex value)", .{}),
        }
    }
    
    /// Check if variable is being watched
    fn isVariableWatched(self: *Self, var_name: []const u8) bool {
        // Check if the debugger has this variable in its watch list
        for (self.debugger.watch_variables.items) |watched| {
            if (std.mem.eql(u8, watched, var_name)) {
                return true;
            }
        }
        return false;
    }
};

/// Create debug-enabled main entry point
pub fn createDebugMain() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Get command line arguments
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.debug.print("Usage: {s} <source_file.csd> [--debug]\n", .{args[0]});
        return;
    }
    
    const source_file = args[1];
    const debug_mode = args.len > 2 and std.mem.eql(u8, args[2], "--debug");
    
    if (debug_mode) {
        // Interactive debug mode
        var debug_interpreter = try DebugInterpreter.init(allocator);
        defer debug_interpreter.deinit();
        
        try debug_interpreter.startDebugSession(source_file);
    } else {
        // Regular execution mode
        var regular_interpreter = try interpreter.Interpreter.init(allocator);
        defer regular_interpreter.deinit();
        
        // Parse and execute normally
        // TODO: Load and parse source file, then execute
        std.debug.print("Regular execution mode for {s}\n", .{source_file});
    }
}

// Test function
test "debug integration" {
    const testing = std.testing;
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var debug_interp = try DebugInterpreter.init(allocator);
    defer debug_interp.deinit();
    
    try testing.expect(debug_interp.debug_enabled);
    try testing.expect(debug_interp.current_line == 0);
    try testing.expect(debug_interp.current_function == null);
}
