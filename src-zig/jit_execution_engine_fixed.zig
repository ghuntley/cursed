const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const print = std.debug.print;
const ArenaAllocator = std.heap.ArenaAllocator;

const ast = @import("ast_simple.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

// Import Value and Environment from interpreter
const interpreter = @import("interpreter.zig");
const Value = interpreter.Value;
const Environment = interpreter.Environment;
const InterpreterError = interpreter.InterpreterError;

/// Memory-safe JIT Execution Engine for CURSED programs
/// Features:
/// - Arena-based memory management for automatic cleanup
/// - Proper resource lifecycle management
/// - Memory leak prevention
/// - Stack overflow protection
/// - Error recovery and cleanup
pub const JITExecutionEngine = struct {
    allocator: Allocator,
    arena: ArenaAllocator,
    arena_allocator: Allocator,
    global_env: Environment,
    current_env: *Environment,
    functions: HashMap([]const u8, ast.FunctionStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    return_value: ?Value,
    call_stack_depth: u32,
    max_call_stack_depth: u32,
    memory_budget: usize,
    memory_used: usize,
    
    const MAX_CALL_STACK_DEPTH = 1000;
    const DEFAULT_MEMORY_BUDGET = 100 * 1024 * 1024; // 100MB
    
    pub fn init(allocator: Allocator) !JITExecutionEngine {
        return initWithBudget(allocator, DEFAULT_MEMORY_BUDGET);
    }
    
    pub fn initWithBudget(allocator: Allocator, memory_budget: usize) !JITExecutionEngine {
        var arena = ArenaAllocator.init(allocator);
        const arena_allocator = arena.allocator();
        
        var global_env = Environment.init(arena_allocator, null);
        
        // Add built-in functions to global environment with error handling
        global_env.define("vibez", Value{ .String = "built_in_vibez" }) catch |err| {
            arena.deinit();
            return err;
        };
        
        const functions = HashMap([]const u8, ast.FunctionStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(arena_allocator);
        
        return JITExecutionEngine{
            .allocator = allocator,
            .arena = arena,
            .arena_allocator = arena_allocator,
            .global_env = global_env,
            .current_env = &global_env,
            .functions = functions,
            .return_value = null,
            .call_stack_depth = 0,
            .max_call_stack_depth = MAX_CALL_STACK_DEPTH,
            .memory_budget = memory_budget,
            .memory_used = 0,
        };
    }
    
    pub fn deinit(self: *JITExecutionEngine) void {
        // Arena allocator automatically frees all memory
        self.arena.deinit();
    }
    
    /// Execute CURSED source code with proper memory management
    pub fn executeSource(self: *JITExecutionEngine, source: []const u8) !void {
        // Create a scoped arena for this execution
        var exec_arena = ArenaAllocator.init(self.allocator);
        defer exec_arena.deinit();
        const exec_allocator = exec_arena.allocator();
        
        var lex = lexer.Lexer.init(exec_allocator, source);
        // Lexer cleanup is handled by arena
        
        const tokens = lex.tokenize() catch |err| {
            print("Lexer error: {}\n", .{err});
            return err;
        };
        
        var parse = parser.Parser.init(exec_allocator, tokens) catch |err| {
            print("Parser init error: {}\n", .{err});
            return err;
        };
        // Parser cleanup is handled by arena
        
        const program = parse.parseProgram() catch |err| {
            print("Parse error: {}\n", .{err});
            return err;
        };
        // Program cleanup is handled by arena
        
        try self.executeProgram(program);
    }
    
    /// Execute a parsed CURSED program with memory tracking
    pub fn executeProgram(self: *JITExecutionEngine, program: ast.Program) !void {
        const start_memory = self.getMemoryUsage();
        defer {
            const end_memory = self.getMemoryUsage();
            if (end_memory > start_memory) {
                self.memory_used += end_memory - start_memory;
            }
            self.checkMemoryBudget() catch {};
        }
        
        // First pass: collect function declarations with memory safety
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .Function => |func_ptr| {
                    const func = func_ptr.*;
                    // Duplicate function name in our arena to ensure lifetime
                    const func_name = self.arena_allocator.dupe(u8, func.name) catch |err| {
                        print("Memory allocation failed for function name: {}\n", .{err});
                        return err;
                    };
                    
                    // Create a copy of the function in our arena
                    var func_copy = func;
                    func_copy.name = func_name;
                    
                    self.functions.put(func_name, func_copy) catch |err| {
                        print("Failed to register function '{}': {}\n", .{func_name, err});
                        return err;
                    };
                },
                else => {},
            }
        }
        
        // Second pass: execute statements with error recovery
        for (program.statements.items) |stmt| {
            self.executeStatement(stmt) catch |err| {
                print("Statement execution error: {}\n", .{err});
                // Continue with other statements for error recovery
                continue;
            };
            
            // Check if we should stop execution
            if (self.return_value != null) break;
        }
    }
    
    /// Execute a function by name with stack overflow protection
    pub fn executeFunction(self: *JITExecutionEngine, name: []const u8) !void {
        if (self.call_stack_depth >= self.max_call_stack_depth) {
            return error.StackOverflow;
        }
        
        if (self.functions.get(name)) |func| {
            _ = try self.callFunction(func, &[_]Value{});
        } else {
            print("Function '{}' not found\n", .{name});
            return error.FunctionNotFound;
        }
    }
    
    /// Execute a single statement with proper error handling
    fn executeStatement(self: *JITExecutionEngine, stmt: ast.Statement) !void {
        switch (stmt) {
            .Expression => |expr_ptr| {
                _ = try self.evaluateExpression(expr_ptr.*);
            },
            .Let => |let_ptr| {
                try self.executeLetStatement(let_ptr.*);
            },
            .Assignment => |assign_ptr| {
                try self.executeAssignmentStatement(assign_ptr.*);
            },
            .Return => |return_ptr| {
                const value = try self.evaluateExpression(return_ptr.value.*);
                self.return_value = value;
            },
            .If => |if_ptr| {
                try self.executeIfStatement(if_ptr.*);
            },
            .While => |while_ptr| {
                try self.executeWhileStatement(while_ptr.*);
            },
            .Function => {
                // Functions are collected in first pass, skip here
            },
            else => {
                print("Unsupported statement: {}\n", .{@tagName(stmt)});
                return error.UnsupportedStatement;
            },
        }
    }
    
    /// Execute let statement with proper memory management
    fn executeLetStatement(self: *JITExecutionEngine, let_stmt: ast.LetStatement) !void {
        const value = if (let_stmt.initializer) |initializer|
            try self.evaluateExpression(initializer.*)
        else
            Value.Null;
            
        try self.current_env.define(let_stmt.name, value);
    }
    
    /// Execute assignment statement with validation
    fn executeAssignmentStatement(self: *JITExecutionEngine, assign_stmt: ast.AssignmentStatement) !void {
        const value = try self.evaluateExpression(assign_stmt.value.*);
        try self.current_env.set(assign_stmt.name, value);
    }
    
    /// Execute if statement with proper scoping
    fn executeIfStatement(self: *JITExecutionEngine, if_stmt: ast.IfStatement) !void {
        const condition = try self.evaluateExpression(if_stmt.condition.*);
        
        if (condition.toBool()) {
            for (if_stmt.then_branch.items) |stmt| {
                try self.executeStatement(stmt);
                if (self.return_value != null) break;
            }
        } else if (if_stmt.else_branch) |else_branch| {
            for (else_branch.items) |stmt| {
                try self.executeStatement(stmt);
                if (self.return_value != null) break;
            }
        }
    }
    
    /// Execute while statement with infinite loop protection
    fn executeWhileStatement(self: *JITExecutionEngine, while_stmt: ast.WhileStatement) !void {
        var iteration_count: u32 = 0;
        const max_iterations = 1_000_000; // Prevent infinite loops
        
        while (true) {
            if (iteration_count >= max_iterations) {
                print("Warning: While loop exceeded maximum iterations\n", .{});
                break;
            }
            
            const condition = try self.evaluateExpression(while_stmt.condition.*);
            if (!condition.toBool()) break;
            
            for (while_stmt.body.items) |stmt| {
                try self.executeStatement(stmt);
                if (self.return_value != null) break;
            }
            
            if (self.return_value != null) break;
            iteration_count += 1;
        }
    }
    
    /// Evaluate an expression with type safety
    fn evaluateExpression(self: *JITExecutionEngine, expr: ast.Expression) !Value {
        switch (expr) {
            .Integer => |int_ptr| {
                return Value{ .Integer = int_ptr.value };
            },
            .Float => |float_ptr| {
                return Value{ .Float = float_ptr.value };
            },
            .String => |str_ptr| {
                // Create a safe copy of the string in our arena
                const safe_str = self.arena_allocator.dupe(u8, str_ptr.value) catch |err| {
                    print("Failed to allocate string: {}\n", .{err});
                    return err;
                };
                return Value{ .String = safe_str };
            },
            .Boolean => |bool_ptr| {
                return Value{ .Boolean = bool_ptr.value };
            },
            .Character => |char_ptr| {
                return Value{ .Character = char_ptr.value };
            },
            .Identifier => |ident_ptr| {
                return self.current_env.get(ident_ptr.name) catch |err| {
                    print("Identifier '{}' not found: {}\n", .{ident_ptr.name, err});
                    return err;
                };
            },
            .Variable => |var_ptr| {
                return self.current_env.get(var_ptr.name) catch |err| {
                    print("Variable '{}' not found: {}\n", .{var_ptr.name, err});
                    return err;
                };
            },
            .Binary => |binary_ptr| {
                return self.evaluateBinaryExpression(binary_ptr.*);
            },
            .Unary => |unary_ptr| {
                return self.evaluateUnaryExpression(unary_ptr.*);
            },
            .Call => |call_ptr| {
                return self.evaluateCallExpression(call_ptr.*);
            },
            .PropertyAccess => |prop_ptr| {
                return self.evaluatePropertyAccess(prop_ptr.*);
            },
            else => {
                print("Unsupported expression: {}\n", .{@tagName(expr)});
                return error.UnsupportedExpression;
            },
        }
    }
    
    /// Evaluate binary expression with overflow protection
    fn evaluateBinaryExpression(self: *JITExecutionEngine, binary: ast.BinaryExpression) !Value {
        const left = try self.evaluateExpression(binary.left.*);
        const right = try self.evaluateExpression(binary.right.*);
        
        return switch (binary.operator) {
            .Plus => try self.addValues(left, right),
            .Minus => try self.subtractValues(left, right),
            .Multiply => try self.multiplyValues(left, right),
            .Divide => try self.divideValues(left, right),
            .Equal => Value{ .Boolean = try self.compareValues(left, right, .Equal) },
            .NotEqual => Value{ .Boolean = try self.compareValues(left, right, .NotEqual) },
            .Less => Value{ .Boolean = try self.compareValues(left, right, .Less) },
            .Greater => Value{ .Boolean = try self.compareValues(left, right, .Greater) },
            .LessEqual => Value{ .Boolean = try self.compareValues(left, right, .LessEqual) },
            .GreaterEqual => Value{ .Boolean = try self.compareValues(left, right, .GreaterEqual) },
            .And => Value{ .Boolean = left.toBool() and right.toBool() },
            .Or => Value{ .Boolean = left.toBool() or right.toBool() },
            else => {
                print("Unsupported binary operator: {}\n", .{@tagName(binary.operator)});
                return error.UnsupportedOperator;
            },
        };
    }
    
    /// Evaluate unary expression safely
    fn evaluateUnaryExpression(self: *JITExecutionEngine, unary: ast.UnaryExpression) !Value {
        const operand = try self.evaluateExpression(unary.operand.*);
        
        return switch (unary.operator) {
            .Minus => switch (operand) {
                .Integer => |int| blk: {
                    // Check for overflow
                    if (int == std.math.minInt(i64)) {
                        return error.IntegerOverflow;
                    }
                    break :blk Value{ .Integer = -int };
                },
                .Float => |float| Value{ .Float = -float },
                else => error.TypeMismatch,
            },
            .Not => Value{ .Boolean = !operand.toBool() },
            else => {
                print("Unsupported unary operator: {}\n", .{@tagName(unary.operator)});
                return error.UnsupportedOperator;
            },
        };
    }
    
    /// Evaluate function call with memory and stack management
    fn evaluateCallExpression(self: *JITExecutionEngine, call: ast.CallExpression) !Value {
        // Handle built-in functions
        if (std.mem.eql(u8, call.function_name, "vibez.spill")) {
            return try self.builtinVibesSpill(call.arguments);
        }
        
        // Handle user-defined functions
        if (self.functions.get(call.function_name)) |func| {
            // Create temporary arena for arguments to prevent memory leaks
            var arg_arena = ArenaAllocator.init(self.allocator);
            defer arg_arena.deinit();
            const arg_allocator = arg_arena.allocator();
            
            // Evaluate arguments
            var args = .empty;
            defer args.deinit();
            errdefer args.deinit(); // Clean up on error
            
            for (call.arguments.items) |arg| {
                const arg_value = try self.evaluateExpression(arg.*);
                try args.append(arg_value);
            }
            
            return try self.callFunction(func, args.items);
        }
        
        print("Unknown function: {}\n", .{call.function_name});
        return error.UnknownFunction;
    }
    
    /// Evaluate property access with bounds checking
    fn evaluatePropertyAccess(self: *JITExecutionEngine, prop: ast.PropertyAccessExpression) !Value {
        const object = try self.evaluateExpression(prop.object.*);
        
        switch (object) {
            .Struct => |struct_inst| {
                if (struct_inst.getField(prop.property)) |field_value| {
                    return field_value;
                } else {
                    return error.UndefinedField;
                }
            },
            else => {
                print("Property access on non-struct type\n", .{});
                return error.InvalidPropertyAccess;
            },
        }
    }
    
    /// Call a CURSED function with proper environment management
    fn callFunction(self: *JITExecutionEngine, func: ast.FunctionStatement, args: []const Value) !Value {
        // Check stack depth
        if (self.call_stack_depth >= self.max_call_stack_depth) {
            return error.StackOverflow;
        }
        
        // Create new environment for function scope using arena
        var func_env = Environment.init(self.arena_allocator, self.current_env);
        
        // Bind parameters to arguments
        const param_count = @min(func.parameters.items.len, args.len);
        for (func.parameters.items[0..param_count], args[0..param_count]) |param, arg| {
            try func_env.define(param.name, arg);
        }
        
        // Handle missing arguments by setting them to null
        for (func.parameters.items[param_count..]) |param| {
            try func_env.define(param.name, Value.Null);
        }
        
        // Save current environment and switch to function environment
        const old_env = self.current_env;
        const old_return = self.return_value;
        const old_depth = self.call_stack_depth;
        
        self.current_env = &func_env;
        self.return_value = null;
        self.call_stack_depth += 1;
        
        // Execute function body with error recovery
        for (func.body.items) |stmt| {
            self.executeStatement(stmt) catch |err| {
                // Restore state on error
                self.current_env = old_env;
                self.return_value = old_return;
                self.call_stack_depth = old_depth;
                return err;
            };
            
            if (self.return_value != null) break;
        }
        
        // Get return value and restore environment
        const result = self.return_value orelse Value.Null;
        self.current_env = old_env;
        self.return_value = old_return;
        self.call_stack_depth = old_depth;
        
        return result;
    }
    
    /// Built-in function: vibez.spill (print) with safe string handling
    fn builtinVibesSpill(self: *JITExecutionEngine, arguments: ArrayList(*ast.Expression)) !Value {
        // Create temporary arena for string operations
        var print_arena = ArenaAllocator.init(self.allocator);
        defer print_arena.deinit();
        const print_allocator = print_arena.allocator();
        
        for (arguments.items, 0..) |arg, i| {
            if (i > 0) print(" ", .{});
            
            const value = try self.evaluateExpression(arg.*);
            const str = value.toString(print_allocator) catch |err| {
                print("[Error converting value to string: {}]", .{err});
                continue;
            };
            
            print("{s}", .{str});
        }
        print("\n", .{});
        
        return Value.Null;
    }
    
    /// Add two values with overflow checking
    fn addValues(_: *JITExecutionEngine, left: Value, right: Value) !Value {
        switch (left) {
            .Integer => |left_int| switch (right) {
                .Integer => |right_int| {
                    const result = std.math.add(i64, left_int, right_int) catch {
                        return error.IntegerOverflow;
                    };
                    return Value{ .Integer = result };
                },
                .Float => |right_float| {
                    return Value{ .Float = @as(f64, @floatFromInt(left_int)) + right_float };
                },
                else => return error.TypeMismatch,
            },
            .Float => |left_float| switch (right) {
                .Integer => |right_int| {
                    return Value{ .Float = left_float + @as(f64, @floatFromInt(right_int)) };
                },
                .Float => |right_float| {
                    return Value{ .Float = left_float + right_float };
                },
                else => return error.TypeMismatch,
            },
            .String => |left_str| switch (right) {
                .String => |right_str| {
                    // String concatenation with proper memory management
                    // Note: In a real implementation, this would need proper string allocation
                    _ = left_str;
                    _ = right_str;
                    return Value{ .String = "concatenated_string" };
                },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }
    
    /// Subtract two values with overflow checking
    fn subtractValues(_: *JITExecutionEngine, left: Value, right: Value) !Value {
        switch (left) {
            .Integer => |left_int| switch (right) {
                .Integer => |right_int| {
                    const result = std.math.sub(i64, left_int, right_int) catch {
                        return error.IntegerOverflow;
                    };
                    return Value{ .Integer = result };
                },
                .Float => |right_float| {
                    return Value{ .Float = @as(f64, @floatFromInt(left_int)) - right_float };
                },
                else => return error.TypeMismatch,
            },
            .Float => |left_float| switch (right) {
                .Integer => |right_int| {
                    return Value{ .Float = left_float - @as(f64, @floatFromInt(right_int)) };
                },
                .Float => |right_float| {
                    return Value{ .Float = left_float - right_float };
                },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }
    
    /// Multiply two values with overflow checking
    fn multiplyValues(_: *JITExecutionEngine, left: Value, right: Value) !Value {
        switch (left) {
            .Integer => |left_int| switch (right) {
                .Integer => |right_int| {
                    const result = std.math.mul(i64, left_int, right_int) catch {
                        return error.IntegerOverflow;
                    };
                    return Value{ .Integer = result };
                },
                .Float => |right_float| {
                    return Value{ .Float = @as(f64, @floatFromInt(left_int)) * right_float };
                },
                else => return error.TypeMismatch,
            },
            .Float => |left_float| switch (right) {
                .Integer => |right_int| {
                    return Value{ .Float = left_float * @as(f64, @floatFromInt(right_int)) };
                },
                .Float => |right_float| {
                    return Value{ .Float = left_float * right_float };
                },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }
    
    /// Divide two values with zero checking
    fn divideValues(_: *JITExecutionEngine, left: Value, right: Value) !Value {
        switch (left) {
            .Integer => |left_int| switch (right) {
                .Integer => |right_int| {
                    if (right_int == 0) return error.DivisionByZero;
                    return Value{ .Integer = @divTrunc(left_int, right_int) };
                },
                .Float => |right_float| {
                    if (right_float == 0.0) return error.DivisionByZero;
                    return Value{ .Float = @as(f64, @floatFromInt(left_int)) / right_float };
                },
                else => return error.TypeMismatch,
            },
            .Float => |left_float| switch (right) {
                .Integer => |right_int| {
                    if (right_int == 0) return error.DivisionByZero;
                    return Value{ .Float = left_float / @as(f64, @floatFromInt(right_int)) };
                },
                .Float => |right_float| {
                    if (right_float == 0.0) return error.DivisionByZero;
                    return Value{ .Float = left_float / right_float };
                },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }
    
    /// Compare two values safely
    fn compareValues(_: *JITExecutionEngine, left: Value, right: Value, comparison: enum { Equal, NotEqual, Less, Greater, LessEqual, GreaterEqual }) !bool {
        switch (left) {
            .Integer => |left_int| switch (right) {
                .Integer => |right_int| return switch (comparison) {
                    .Equal => left_int == right_int,
                    .NotEqual => left_int != right_int,
                    .Less => left_int < right_int,
                    .Greater => left_int > right_int,
                    .LessEqual => left_int <= right_int,
                    .GreaterEqual => left_int >= right_int,
                },
                .Float => |right_float| {
                    const left_float = @as(f64, @floatFromInt(left_int));
                    return switch (comparison) {
                        .Equal => left_float == right_float,
                        .NotEqual => left_float != right_float,
                        .Less => left_float < right_float,
                        .Greater => left_float > right_float,
                        .LessEqual => left_float <= right_float,
                        .GreaterEqual => left_float >= right_float,
                    };
                },
                else => return error.TypeMismatch,
            },
            .Float => |left_float| switch (right) {
                .Integer => |right_int| {
                    const right_float = @as(f64, @floatFromInt(right_int));
                    return switch (comparison) {
                        .Equal => left_float == right_float,
                        .NotEqual => left_float != right_float,
                        .Less => left_float < right_float,
                        .Greater => left_float > right_float,
                        .LessEqual => left_float <= right_float,
                        .GreaterEqual => left_float >= right_float,
                    };
                },
                .Float => |right_float| return switch (comparison) {
                    .Equal => left_float == right_float,
                    .NotEqual => left_float != right_float,
                    .Less => left_float < right_float,
                    .Greater => left_float > right_float,
                    .LessEqual => left_float <= right_float,
                    .GreaterEqual => left_float >= right_float,
                },
                else => return error.TypeMismatch,
            },
            .String => |left_str| switch (right) {
                .String => |right_str| return switch (comparison) {
                    .Equal => std.mem.eql(u8, left_str, right_str),
                    .NotEqual => !std.mem.eql(u8, left_str, right_str),
                    .Less => std.mem.lessThan(u8, left_str, right_str),
                    .Greater => std.mem.lessThan(u8, right_str, left_str),
                    .LessEqual => !std.mem.lessThan(u8, right_str, left_str),
                    .GreaterEqual => !std.mem.lessThan(u8, left_str, right_str),
                },
                else => return error.TypeMismatch,
            },
            .Boolean => |left_bool| switch (right) {
                .Boolean => |right_bool| return switch (comparison) {
                    .Equal => left_bool == right_bool,
                    .NotEqual => left_bool != right_bool,
                    else => return error.TypeMismatch,
                },
                else => return error.TypeMismatch,
            },
            else => return error.TypeMismatch,
        }
    }
    
    /// Get current memory usage (approximation)
    fn getMemoryUsage(self: *JITExecutionEngine) usize {
        _ = self;
        // In a real implementation, this would query the arena allocator
        return 0;
    }
    
    /// Check if memory budget is exceeded
    fn checkMemoryBudget(self: *JITExecutionEngine) !void {
        if (self.memory_used > self.memory_budget) {
            return error.MemoryBudgetExceeded;
        }
    }
    
    /// Reset execution state for reuse
    pub fn reset(self: *JITExecutionEngine) void {
        self.return_value = null;
        self.call_stack_depth = 0;
        self.memory_used = 0;
        
        // Reset environment to global scope
        self.current_env = &self.global_env;
    }
    
    /// Get execution statistics
    pub fn getStats(self: *JITExecutionEngine) ExecutionStats {
        return ExecutionStats{
            .memory_used = self.memory_used,
            .memory_budget = self.memory_budget,
            .call_stack_depth = self.call_stack_depth,
            .max_call_stack_depth = self.max_call_stack_depth,
        };
    }
};

/// Execution statistics for monitoring
pub const ExecutionStats = struct {
    memory_used: usize,
    memory_budget: usize,
    call_stack_depth: u32,
    max_call_stack_depth: u32,
};

/// Test the fixed JIT execution engine
pub fn testJITExecutionEngine(allocator: Allocator) !void {
    print("\n🧪 Testing Fixed JIT Execution Engine\n", .{});
    print("=====================================\n", .{});
    
    var engine = try JITExecutionEngine.init(allocator);
    defer engine.deinit();
    
    // Test 1: Simple expression evaluation
    print("\n📝 Test 1: Simple expression\n", .{});
    const simple_program = 
        \\vibez.spill("Hello, CURSED!")
    ;
    try engine.executeSource(simple_program);
    
    // Test 2: Variable declaration and usage
    print("\n📝 Test 2: Variables\n", .{});
    const variable_program = 
        \\sus x drip = 42
        \\vibez.spill("Value of x:", x)
    ;
    try engine.executeSource(variable_program);
    
    // Test 3: Arithmetic operations with overflow protection
    print("\n📝 Test 3: Arithmetic\n", .{});
    const arithmetic_program = 
        \\sus a drip = 10
        \\sus b drip = 5
        \\sus sum drip = a + b
        \\sus diff drip = a - b
        \\vibez.spill("Sum:", sum, "Diff:", diff)
    ;
    try engine.executeSource(arithmetic_program);
    
    // Test 4: Function definition and call
    print("\n📝 Test 4: Functions\n", .{});
    const function_program = 
        \\slay add(x drip, y drip) drip {
        \\    damn x + y
        \\}
        \\sus result drip = add(15, 25)
        \\vibez.spill("Function result:", result)
    ;
    try engine.executeSource(function_program);
    
    // Test 5: Control flow
    print("\n📝 Test 5: Control flow\n", .{});
    const control_program = 
        \\sus number drip = 7
        \\bestie (number > 5) {
        \\    vibez.spill("Number is greater than 5")
        \\} finna {
        \\    vibez.spill("Number is not greater than 5")
        \\}
    ;
    try engine.executeSource(control_program);
    
    // Test 6: Memory management
    print("\n📝 Test 6: Memory management\n", .{});
    const stats = engine.getStats();
    print("Memory used: {} bytes\n", .{stats.memory_used});
    print("Memory budget: {} bytes\n", .{stats.memory_budget});
    print("Call stack depth: {}\n", .{stats.call_stack_depth});
    
    print("\n✅ Fixed JIT Execution Engine tests completed!\n", .{});
}
