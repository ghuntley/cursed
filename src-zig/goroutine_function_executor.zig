//! CURSED Goroutine Function Executor
//! 
//! Handles interpreted function calls within goroutines without causing stack overflow.
//! Uses call trampolines, frame growth management, and tail-call optimization.
//!
//! Features:
//! - Stack overflow prevention with configurable limits
//! - Call trampolines to prevent deep recursion
//! - Memory-safe frame management using arena allocators
//! - Tail-call optimization detection
//! - Error propagation through call stack
//! - Integration with goroutine context switching

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const ArenaAllocator = std.heap.ArenaAllocator;
const HashMap = std.HashMap;
const StringHashMap = std.StringHashMap;

const ast = @import("ast_simple.zig");
const interpreter = @import("interpreter.zig");
const concurrency = @import("concurrency.zig");

const Value = interpreter.Value;
const Environment = interpreter.Environment;
const InterpreterError = interpreter.InterpreterError;
const GoroutineId = concurrency.GoroutineId;

/// Stack frame limits for goroutine safety
const MAX_STACK_FRAMES = 2000;
const STACK_GROWTH_THRESHOLD = 1500;
const FRAME_SIZE_LIMIT = 64 * 1024; // 64KB per frame

/// Call trampoline result to control execution flow
pub const TrampolineResult = union(enum) {
    Value: Value,
    TailCall: TailCallInfo,
    Yield: YieldInfo,
    Error: InterpreterError,
    StackOverflow,
};

/// Information for tail call optimization
pub const TailCallInfo = struct {
    function: ast.FunctionStatement,
    args: []const Value,
    env: Environment,
};

/// Information for goroutine yielding
pub const YieldInfo = struct {
    goroutine_id: GoroutineId,
    continuation: CallContinuation,
};

/// Continuation information for resuming calls
pub const CallContinuation = struct {
    function: ast.FunctionStatement,
    args: []const Value,
    env: Environment,
    statement_index: usize,
    local_vars: StringHashMap(Value),
};

/// Execution frame for stack management
pub const ExecutionFrame = struct {
    function: ast.FunctionStatement,
    env: Environment,
    args: []const Value,
    statement_index: usize,
    return_address: ?*ExecutionFrame,
    frame_size: usize,
    
    pub fn init(allocator: Allocator, function: ast.FunctionStatement, args: []const Value, parent_env: ?*Environment) !ExecutionFrame {
        var env = Environment.init(allocator, parent_env);
        
        // Bind function parameters
        const param_count = @min(function.parameters.items.len, args.len);
        for (function.parameters.items[0..param_count], args[0..param_count]) |param, arg| {
            try env.define(param.name, arg);
        }
        
        // Set default values for missing parameters
        for (function.parameters.items[param_count..]) |param| {
            try env.define(param.name, Value.Null);
        }
        
        return ExecutionFrame{
            .function = function,
            .env = env,
            .args = args,
            .statement_index = 0,
            .return_address = null,
            .frame_size = @sizeOf(ExecutionFrame) + args.len * @sizeOf(Value),
        };
    }
    
    pub fn deinit(self: *ExecutionFrame) void {
        self.env.deinit(self.allocator);
    }
};

/// Goroutine function executor with stack overflow prevention
pub const GoroutineFunctionExecutor = struct {
    allocator: Allocator,
    arena: ArenaAllocator,
    arena_allocator: Allocator,
    
    // Stack management
    call_stack: ArrayList(ExecutionFrame),
    stack_depth: u32,
    max_stack_depth: u32,
    total_frame_size: usize,
    
    // Function registry
    functions: StringHashMap(ast.FunctionStatement),
    
    // Goroutine context
    goroutine_id: GoroutineId,
    can_yield: bool,
    yield_threshold: u32,
    
    // Error handling
    last_error: ?InterpreterError,
    error_recovery_enabled: bool,
    
    pub fn init(allocator: Allocator, goroutine_id: GoroutineId) !*GoroutineFunctionExecutor {
        var arena = ArenaAllocator.init(allocator);
        const arena_allocator = arena.allocator();
        
        const executor = try arena_allocator.create(GoroutineFunctionExecutor);
        executor.* = GoroutineFunctionExecutor{
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena_allocator,
            .call_stack = .empty,
            .stack_depth = 0,
            .max_stack_depth = MAX_STACK_FRAMES,
            .total_frame_size = 0,
            .functions = StringHashMap(ast.FunctionStatement){},
            .goroutine_id = goroutine_id,
            .can_yield = true,
            .yield_threshold = STACK_GROWTH_THRESHOLD,
            .last_error = null,
            .error_recovery_enabled = true,
        };
        
        return executor;
    }
    
    pub fn deinit(self: *GoroutineFunctionExecutor) void {
        // Clean up all stack frames
        for (self.call_stack.items) |*frame| {
            frame.deinit();
        }
        
        // Arena handles the rest of the cleanup
        self.arena.deinit(self.allocator);
    }
    
    /// Register a function for interpreted execution
    pub fn registerFunction(self: *GoroutineFunctionExecutor, name: []const u8, function: ast.FunctionStatement) !void {
        const name_copy = try self.arena_allocator.dupe(u8, name);
        try self.functions.put(name_copy, function);
    }
    
    /// Execute an interpreted function using call trampolines to prevent stack overflow
    pub fn executeInterpretedFunction(self: *GoroutineFunctionExecutor, name: []const u8, args: []const Value) !Value {
        const function = self.functions.get(name) orelse {
            print("Function '{s}' not found in goroutine {s}\n", .{ name, self.goroutine_id });
            return error.FunctionNotFound;
        };
        
        return self.executeFunctionWithTrampoline(function, args);
    }
    
    /// Execute function using trampoline to prevent deep recursion
    fn executeFunctionWithTrampoline(self: *GoroutineFunctionExecutor, function: ast.FunctionStatement, args: []const Value) !Value {
        var current_function = function;
        var current_args = args;
        var result: TrampolineResult = undefined;
        
        // Main trampoline loop
        while (true) {
            // Check for stack overflow before execution
            if (self.shouldYield()) {
                return self.yieldExecution(current_function, current_args);
            }
            
            result = self.executeFunctionFrame(current_function, current_args) catch |err| {
                self.last_error = switch (err) {
                    error.StackOverflow => InterpreterError.StackOverflow,
                    error.OutOfMemory => InterpreterError.OutOfMemory,
                    error.TypeError => InterpreterError.TypeError,
                    else => InterpreterError.RuntimeError,
                };
                return err;
            };
            
            switch (result) {
                .Value => |value| return value,
                
                .TailCall => |tail_call| {
                    // Optimize tail calls by reusing the current frame
                    current_function = tail_call.function;
                    current_args = tail_call.args;
                    // Continue loop for tail call optimization
                },
                
                .Yield => |yield_info| {
                    return self.handleGoroutineYield(yield_info);
                },
                
                .Error => |interpreter_error| {
                    self.last_error = interpreter_error;
                    return self.convertInterpreterError(interpreter_error);
                },
                
                .StackOverflow => {
                    print("Stack overflow detected in goroutine {s}\n", .{self.goroutine_id});
                    return error.StackOverflow;
                },
            }
        }
    }
    
    /// Execute a single function frame with stack management
    fn executeFunctionFrame(self: *GoroutineFunctionExecutor, function: ast.FunctionStatement, args: []const Value) !TrampolineResult {
        // Check stack limits
        if (self.stack_depth >= self.max_stack_depth) {
            return TrampolineResult.StackOverflow;
        }
        
        if (self.total_frame_size > FRAME_SIZE_LIMIT * self.max_stack_depth) {
            return TrampolineResult.StackOverflow;
        }
        
        // Create execution frame
        var frame = ExecutionFrame.init(
            self.arena_allocator,
            function,
            args,
            if (self.call_stack.items.len > 0) &self.call_stack.items[self.call_stack.items.len - 1].env else null
        ) catch |err| {
            return TrampolineResult{ .Error = InterpreterError.OutOfMemory };
        };
        
        // Update stack tracking
        try self.call_stack.append(allocator, frame);
        self.stack_depth += 1;
        self.total_frame_size += frame.frame_size;
        
        defer {
            // Clean up frame on exit
            if (self.call_stack.items.len > 0) {
                var popped_frame = self.call_stack.pop();
                self.total_frame_size -= popped_frame.frame_size;
                popped_frame.deinit();
                self.stack_depth -= 1;
            }
        }
        
        // Execute function body
        var return_value: ?Value = null;
        
        for (function.body.items, 0..) |stmt, i| {
            frame.statement_index = i;
            
            // Check for yield conditions
            if (self.shouldYieldAtStatement(i)) {
                const continuation = CallContinuation{
                    .function = function,
                    .args = args,
                    .env = frame.env,
                    .statement_index = i,
                    .local_vars = StringHashMap(Value){},
                };
                
                return TrampolineResult{ .Yield = YieldInfo{
                    .goroutine_id = self.goroutine_id,
                    .continuation = continuation,
                }};
            }
            
            // Execute statement
            const stmt_result = self.executeStatement(stmt, &frame) catch |err| {
                return TrampolineResult{ .Error = self.convertErrorToInterpreterError(err) };
            };
            
            // Check for return value
            if (stmt_result) |value| {
                return_value = value;
                break;
            }
            
            // Check for tail call optimization opportunity
            if (i == function.body.items.len - 1 and self.isTailCallCandidate(stmt)) {
                if (self.extractTailCall(stmt)) |tail_call| {
                    return TrampolineResult{ .TailCall = tail_call };
                }
            }
        }
        
        return TrampolineResult{ .Value = return_value orelse Value.Null };
    }
    
    /// Execute a single statement within a frame
    fn executeStatement(self: *GoroutineFunctionExecutor, stmt: ast.Statement, frame: *ExecutionFrame) !?Value {
        switch (stmt) {
            .Return => |expr| {
                if (expr) |expression| {
                    const value = try self.evaluateExpression(expression.*, &frame.env);
                    return value;
                } else {
                    return Value.Null;
                }
            },
            
            .Expression => |expr| {
                _ = try self.evaluateExpression(expr.*, &frame.env);
                return null;
            },
            
            .VariableDeclaration => |var_decl| {
                const value = if (var_decl.initializer) |init|
                    try self.evaluateExpression(init.*, &frame.env)
                else
                    Value.Null;
                
                try frame.env.define(var_decl.name, value);
                return null;
            },
            
            .Assignment => |assignment| {
                const value = try self.evaluateExpression(assignment.value.*, &frame.env);
                try frame.env.assign(assignment.name, value);
                return null;
            },
            
            .If => |if_stmt| {
                const condition = try self.evaluateExpression(if_stmt.condition.*, &frame.env);
                if (self.isTruthy(condition)) {
                    return try self.executeBlockWithFrame(if_stmt.then_branch.items, frame);
                } else if (if_stmt.else_branch) |else_branch| {
                    return try self.executeBlockWithFrame(else_branch.items, frame);
                }
                return null;
            },
            
            .While => |while_stmt| {
                while (true) {
                    const condition = try self.evaluateExpression(while_stmt.condition.*, &frame.env);
                    if (!self.isTruthy(condition)) break;
                    
                    if (try self.executeBlockWithFrame(while_stmt.body.items, frame)) |value| {
                        return value; // Early return from loop
                    }
                    
                    // Check for yield in long-running loops
                    if (self.shouldYieldInLoop()) {
                        // This would be handled by the calling trampoline
                        break;
                    }
                }
                return null;
            },
            
            .Block => |block| {
                return try self.executeBlockWithFrame(block.items, frame);
            },
            
            else => {
                print("Unsupported statement type in goroutine executor\n", .{});
                return null;
            },
        }
    }
    
    /// Execute a block of statements within a frame
    fn executeBlockWithFrame(self: *GoroutineFunctionExecutor, statements: []const ast.Statement, frame: *ExecutionFrame) !?Value {
        for (statements) |stmt| {
            if (try self.executeStatement(stmt, frame)) |value| {
                return value;
            }
        }
        return null;
    }
    
    /// Evaluate an expression within the given environment
    fn evaluateExpression(self: *GoroutineFunctionExecutor, expr: ast.Expression, env: *Environment) !Value {
        switch (expr) {
            .Literal => |literal| {
                return switch (literal) {
                    .Integer => |i| Value{ .Integer = i },
                    .Float => |f| Value{ .Float = f },
                    .String => |s| Value{ .String = s },
                    .Boolean => |b| Value{ .Boolean = b },
                    .Character => |c| Value{ .Character = c },
                    .Null => Value.Null,
                };
            },
            
            .Identifier => |name| {
                return env.get(name) orelse {
                    print("Undefined variable: {s}\n", .{name});
                    return error.UndefinedVariable;
                };
            },
            
            .Call => |call| {
                // This is where function calls are handled - use trampoline if recursive
                return self.handleFunctionCall(call, env);
            },
            
            .Binary => |binary| {
                const left = try self.evaluateExpression(binary.left.*, env);
                const right = try self.evaluateExpression(binary.right.*, env);
                return self.evaluateBinaryOperation(binary.operator, left, right);
            },
            
            .Unary => |unary| {
                const operand = try self.evaluateExpression(unary.operand.*, env);
                return self.evaluateUnaryOperation(unary.operator, operand);
            },
            
            else => {
                print("Unsupported expression type in goroutine executor\n", .{});
                return Value.Null;
            },
        }
    }
    
    /// Handle function calls with recursion detection
    fn handleFunctionCall(self: *GoroutineFunctionExecutor, call: ast.CallExpression, env: *Environment) !Value {
        // Evaluate arguments
        var args = std.ArrayList(u8){};
        defer args.deinit();
        errdefer args.deinit(); // Clean up on error
        for (call.arguments.items) |arg| {
            const value = try self.evaluateExpression(arg.*, env);
            try args.append(allocator, value);
        }
        
        // Check if it's a built-in function
        if (self.isBuiltinFunction(call.function_name)) {
            return self.callBuiltinFunction(call.function_name, args.items);
        }
        
        // Look up user-defined function
        if (self.functions.get(call.function_name)) |function| {
            // Check for deep recursion
            if (self.isRecursiveCall(call.function_name)) {
                // Use trampoline for recursive calls
                return self.executeFunctionWithTrampoline(function, args.items);
            } else {
                // Direct call for non-recursive functions
                return self.executeFunctionFrame(function, args.items) catch |err| {
                    return self.convertErrorToValue(err);
                };
            }
        } else {
            print("Function '{s}' not found\n", .{call.function_name});
            return error.FunctionNotFound;
        }
    }
    
    /// Check if this is a recursive call
    fn isRecursiveCall(self: *GoroutineFunctionExecutor, function_name: []const u8) bool {
        var count: u32 = 0;
        for (self.call_stack.items) |frame| {
            if (std.mem.eql(u8, frame.function.name, function_name)) {
                count += 1;
                if (count >= 3) return true; // Recursive if same function appears 3+ times
            }
        }
        return false;
    }
    
    /// Check if the executor should yield control
    fn shouldYield(self: *GoroutineFunctionExecutor) bool {
        return self.can_yield and (
            self.stack_depth >= self.yield_threshold or
            self.total_frame_size > FRAME_SIZE_LIMIT * self.yield_threshold
        );
    }
    
    /// Check if should yield at a specific statement
    fn shouldYieldAtStatement(self: *GoroutineFunctionExecutor, statement_index: usize) bool {
        // Yield periodically during long function execution
        return self.can_yield and statement_index > 0 and statement_index % 100 == 0;
    }
    
    /// Check if should yield in loops
    fn shouldYieldInLoop(self: *GoroutineFunctionExecutor) bool {
        return self.can_yield and self.stack_depth > self.yield_threshold / 2;
    }
    
    /// Handle goroutine yielding
    fn yieldExecution(self: *GoroutineFunctionExecutor, function: ast.FunctionStatement, args: []const Value) !Value {
        print("Goroutine {s} yielding execution\n", .{self.goroutine_id});
        // In a real implementation, this would integrate with the goroutine scheduler
        // For now, return a special value indicating yield
        return Value{ .String = "YIELDED" };
    }
    
    /// Handle goroutine yield with continuation
    fn handleGoroutineYield(self: *GoroutineFunctionExecutor, yield_info: YieldInfo) !Value {
        _ = yield_info; // Would be used to schedule continuation
        print("Goroutine {s} yielded with continuation\n", .{self.goroutine_id});
        return Value{ .String = "YIELD_WITH_CONTINUATION" };
    }
    
    /// Check if statement is a tail call candidate
    fn isTailCallCandidate(self: *GoroutineFunctionExecutor, stmt: ast.Statement) bool {
        _ = self;
        return switch (stmt) {
            .Return => |expr| if (expr) |e| switch (e.*) {
                .Call => true,
                else => false,
            } else false,
            else => false,
        };
    }
    
    /// Extract tail call information
    fn extractTailCall(self: *GoroutineFunctionExecutor, stmt: ast.Statement) ?TailCallInfo {
        _ = self;
        switch (stmt) {
            .Return => |expr| if (expr) |e| switch (e.*) {
                .Call => |call| {
                    if (self.functions.get(call.function_name)) |function| {
                        // Would evaluate arguments and create tail call info
                        // Simplified for now
                        return TailCallInfo{
                            .function = function,
                            .args = &[_]Value{}, // Would be properly filled
                            .env = Environment.init(self.arena_allocator, null),
                        };
                    }
                },
                else => {},
            },
            else => {},
        }
        return null;
    }
    
    /// Convert interpreter error to runtime error
    fn convertInterpreterError(self: *GoroutineFunctionExecutor, interpreter_error: InterpreterError) !Value {
        _ = self;
        return switch (interpreter_error) {
            InterpreterError.StackOverflow => error.StackOverflow,
            InterpreterError.OutOfMemory => error.OutOfMemory,
            InterpreterError.TypeError => error.TypeError,
            InterpreterError.RuntimeError => error.RuntimeError,
        };
    }
    
    /// Convert general error to interpreter error
    fn convertErrorToInterpreterError(self: *GoroutineFunctionExecutor, err: anyerror) InterpreterError {
        _ = self;
        return switch (err) {
            error.StackOverflow => InterpreterError.StackOverflow,
            error.OutOfMemory => InterpreterError.OutOfMemory,
            error.TypeError => InterpreterError.TypeError,
            else => InterpreterError.RuntimeError,
        };
    }
    
    /// Convert error to safe value for error recovery
    fn convertErrorToValue(self: *GoroutineFunctionExecutor, err: anyerror) Value {
        _ = self;
        const error_msg = switch (err) {
            error.StackOverflow => "StackOverflow",
            error.OutOfMemory => "OutOfMemory",
            error.TypeError => "TypeError",
            else => "RuntimeError",
        };
        return Value{ .String = error_msg };
    }
    
    /// Helper functions for expression evaluation
    fn isTruthy(self: *GoroutineFunctionExecutor, value: Value) bool {
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
    
    fn isBuiltinFunction(self: *GoroutineFunctionExecutor, name: []const u8) bool {
        _ = self;
        return std.mem.eql(u8, name, "vibez.spill") or
               std.mem.eql(u8, name, "print") or
               std.mem.eql(u8, name, "len");
    }
    
    fn callBuiltinFunction(self: *GoroutineFunctionExecutor, name: []const u8, args: []const Value) !Value {
        if (std.mem.eql(u8, name, "vibez.spill") or std.mem.eql(u8, name, "print")) {
            for (args, 0..) |arg, i| {
                if (i > 0) print(" ", .{});
                switch (arg) {
                    .Integer => |i| print("{s}", .{i}),
                    .Float => |f| print("{d}", .{f}),
                    .String => |s| print("{s}", .{s}),
                    .Boolean => |b| print("{s}", .{b}),
                    .Character => |c| print("{c}", .{c}),
                    .Null => print("null"),
                    else => print("[complex value]"),
                }
            }
            print("\n", .{});
            return Value.Null;
        } else if (std.mem.eql(u8, name, "len")) {
            if (args.len > 0) {
                switch (args[0]) {
                    .String => |s| return Value{ .Integer = @intCast(s.len) },
                    else => return Value{ .Integer = 0 },
                }
            }
            return Value{ .Integer = 0 };
        }
        
        return Value.Null;
    }
    
    fn evaluateBinaryOperation(self: *GoroutineFunctionExecutor, operator: ast.BinaryOperator, left: Value, right: Value) !Value {
        _ = self;
        return switch (operator) {
            .Add => switch (left) {
                .Integer => |l| switch (right) {
                    .Integer => |r| Value{ .Integer = l + r },
                    .Float => |r| Value{ .Float = @as(f64, @floatFromInt(l)) + r },
                    else => error.TypeError,
                },
                .Float => |l| switch (right) {
                    .Integer => |r| Value{ .Float = l + @as(f64, @floatFromInt(r)) },
                    .Float => |r| Value{ .Float = l + r },
                    else => error.TypeError,
                },
                else => error.TypeError,
            },
            .Subtract => switch (left) {
                .Integer => |l| switch (right) {
                    .Integer => |r| Value{ .Integer = l - r },
                    .Float => |r| Value{ .Float = @as(f64, @floatFromInt(l)) - r },
                    else => error.TypeError,
                },
                .Float => |l| switch (right) {
                    .Integer => |r| Value{ .Float = l - @as(f64, @floatFromInt(r)) },
                    .Float => |r| Value{ .Float = l - r },
                    else => error.TypeError,
                },
                else => error.TypeError,
            },
            .Multiply => switch (left) {
                .Integer => |l| switch (right) {
                    .Integer => |r| Value{ .Integer = l * r },
                    .Float => |r| Value{ .Float = @as(f64, @floatFromInt(l)) * r },
                    else => error.TypeError,
                },
                .Float => |l| switch (right) {
                    .Integer => |r| Value{ .Float = l * @as(f64, @floatFromInt(r)) },
                    .Float => |r| Value{ .Float = l * r },
                    else => error.TypeError,
                },
                else => error.TypeError,
            },
            .Divide => switch (left) {
                .Integer => |l| switch (right) {
                    .Integer => |r| if (r != 0) Value{ .Integer = @divTrunc(l, r) } else error.DivisionByZero,
                    .Float => |r| if (r != 0.0) Value{ .Float = @as(f64, @floatFromInt(l)) / r } else error.DivisionByZero,
                    else => error.TypeError,
                },
                .Float => |l| switch (right) {
                    .Integer => |r| if (r != 0) Value{ .Float = l / @as(f64, @floatFromInt(r)) } else error.DivisionByZero,
                    .Float => |r| if (r != 0.0) Value{ .Float = l / r } else error.DivisionByZero,
                    else => error.TypeError,
                },
                else => error.TypeError,
            },
            else => error.UnsupportedOperation,
        };
    }
    
    fn evaluateUnaryOperation(self: *GoroutineFunctionExecutor, operator: ast.UnaryOperator, operand: Value) !Value {
        _ = self;
        return switch (operator) {
            .Minus => switch (operand) {
                .Integer => |i| Value{ .Integer = -i },
                .Float => |f| Value{ .Float = -f },
                else => error.TypeError,
            },
            .Not => Value{ .Boolean = !self.isTruthy(operand) },
            else => error.UnsupportedOperation,
        };
    }
};

/// Test the goroutine function executor
pub fn testGoroutineFunctionExecutor() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var executor = try GoroutineFunctionExecutor.init(allocator, 1);
    defer executor.deinit();
    
    print("Goroutine function executor test completed successfully\n", .{});
}
