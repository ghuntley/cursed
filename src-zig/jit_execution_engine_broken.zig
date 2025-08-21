const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const print = std.debug.print;

const ast = @import("ast_simple.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

// Import Value and Environment from interpreter
const interpreter = @import("interpreter.zig");
const Value = interpreter.Value;
const Environment = interpreter.Environment;
const InterpreterError = interpreter.InterpreterError;

/// JIT Execution Engine for CURSED programs
/// This is an interpreter that executes CURSED AST directly
pub const JITExecutionEngine = struct {
    allocator: Allocator,
    global_env: Environment,
    current_env: *Environment,
    functions: HashMap([]const u8, ast.FunctionStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    return_value: ?Value,
    
    pub fn init(allocator: Allocator) !JITExecutionEngine {
        var global_env = Environment.init(allocator, null);
        
        // Add built-in functions to global environment
        try global_env.define("vibez", Value{ .String = "built_in_vibez" });
        
        return JITExecutionEngine{
            .allocator = allocator,
            .global_env = global_env,
            .current_env = &global_env,
            .functions = HashMap([]const u8, ast.FunctionStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .return_value = null,
        };
    }
    
    pub fn deinit(self: *JITExecutionEngine) void {
        self.global_env.deinit();
        self.functions.deinit();
    }
    
    /// Execute CURSED source code
    pub fn executeSource(self: *JITExecutionEngine, source: []const u8) !void {
        var lex = lexer.Lexer.init(self.allocator, source);
        defer lex.deinit();
        
        const tokens = try lex.tokenize();
        
        var parse = try parser.Parser.init(self.allocator, tokens);
        defer parse.deinit();
        
        const program = try parse.parseProgram();
        defer parse.freeProgram(program);
        
        try self.executeProgram(program);
    }
    
    /// Execute a parsed CURSED program
    pub fn executeProgram(self: *JITExecutionEngine, program: ast.Program) !void {
        // First pass: collect function declarations
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .Function => |func_ptr| {
                    const func = func_ptr.*;
                    const func_name = try self.allocator.dupe(u8, func.name);
                    try self.functions.put(func_name, func);
                },
                else => {},
            }
        }
        
        // Second pass: execute statements
        for (program.statements.items) |stmt| {
            try self.executeStatement(stmt);
        }
    }
    
    /// Execute a function by name
    pub fn executeFunction(self: *JITExecutionEngine, name: []const u8) !void {
        if (self.functions.get(name)) |func| {
            _ = try self.callFunction(func, &[_]Value{});
        } else {
            print("Function '{s}' not found\n", .{name});
        }
    }
    
    /// Execute a single statement
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
                print("Unsupported statement: {s}\n", .{@tagName(stmt)});
            },
        }
    }
    
    /// Execute let statement (variable declaration)
    fn executeLetStatement(self: *JITExecutionEngine, let_stmt: ast.LetStatement) !void {
        const value = if (let_stmt.initializer) |initializer|
            try self.evaluateExpression(initializer.*)
        else
            Value.Null;
            
        try self.current_env.define(let_stmt.name, value);
    }
    
    /// Execute assignment statement
    fn executeAssignmentStatement(self: *JITExecutionEngine, assign_stmt: ast.AssignmentStatement) !void {
        const value = try self.evaluateExpression(assign_stmt.value.*);
        try self.current_env.set(assign_stmt.name, value);
    }
    
    /// Execute if statement
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
    
    /// Execute while statement
    fn executeWhileStatement(self: *JITExecutionEngine, while_stmt: ast.WhileStatement) !void {
        while (true) {
            const condition = try self.evaluateExpression(while_stmt.condition.*);
            if (!condition.toBool()) break;
            
            for (while_stmt.body.items) |stmt| {
                try self.executeStatement(stmt);
                if (self.return_value != null) break;
            }
            
            if (self.return_value != null) break;
        }
    }
    
    /// Evaluate an expression and return its value
    fn evaluateExpression(self: *JITExecutionEngine, expr: ast.Expression) !Value {
        switch (expr) {
            .Integer => |int_ptr| {
                return Value{ .Integer = int_ptr.value };
            },
            .Float => |float_ptr| {
                return Value{ .Float = float_ptr.value };
            },
            .String => |str_ptr| {
                return Value{ .String = str_ptr.value };
            },
            .Boolean => |bool_ptr| {
                return Value{ .Boolean = bool_ptr.value };
            },
            .Character => |char_ptr| {
                return Value{ .Character = char_ptr.value };
            },
            .Identifier => |ident_ptr| {
                return self.current_env.get(ident_ptr.name);
            },
            .Variable => |var_ptr| {
                return self.current_env.get(var_ptr.name);
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
                print("Unsupported expression: {s}\n", .{@tagName(expr)});
                return Value.Null;
            },
        }
    }
    
    /// Evaluate binary expression (e.g., a + b, a == b)
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
                print("Unsupported binary operator: {s}\n", .{@tagName(binary.operator)});
                return Value.Null;
            },
        };
    }
    
    /// Evaluate unary expression (e.g., -x, !x)
    fn evaluateUnaryExpression(self: *JITExecutionEngine, unary: ast.UnaryExpression) !Value {
        const operand = try self.evaluateExpression(unary.operand.*);
        
        return switch (unary.operator) {
            .Minus => switch (operand) {
                .Integer => |int| Value{ .Integer = -int },
                .Float => |float| Value{ .Float = -float },
                else => InterpreterError.TypeMismatch,
            },
            .Not => Value{ .Boolean = !operand.toBool() },
            else => {
                print("Unsupported unary operator: {s}\n", .{@tagName(unary.operator)});
                return Value.Null;
            },
        };
    }
    
    /// Evaluate function call expression
    fn evaluateCallExpression(self: *JITExecutionEngine, call: ast.CallExpression) !Value {
        // Handle built-in functions
        if (std.mem.eql(u8, call.function_name, "vibez.spill")) {
            return try self.builtinVibesSpill(call.arguments);
        }
        
        // Handle user-defined functions
        if (self.functions.get(call.function_name)) |func| {
            // Evaluate arguments
            var args = .empty;
            defer args.deinit();
            
            for (call.arguments.items) |arg| {
                const arg_value = try self.evaluateExpression(arg.*);
                try args.append(arg_value);
            }
            
            return try self.callFunction(func, args.items);
        }
        
        print("Unknown function: {s}\n", .{call.function_name});
        return Value.Null;
    }
    
    /// Evaluate property access (e.g., obj.field)
    fn evaluatePropertyAccess(self: *JITExecutionEngine, prop: ast.PropertyAccessExpression) !Value {
        const object = try self.evaluateExpression(prop.object.*);
        
        switch (object) {
            .Struct => |struct_inst| {
                if (struct_inst.getField(prop.property)) |field_value| {
                    return field_value;
                } else {
                    return InterpreterError.UndefinedField;
                }
            },
            else => {
                print("Property access on non-struct type\n", .{});
                return Value.Null;
            },
        }
    }
    
    /// Call a CURSED function
    fn callFunction(self: *JITExecutionEngine, func: ast.FunctionStatement, args: []const Value) !Value {
        // Create new environment for function scope
        var func_env = Environment.init(self.allocator, self.current_env);
        defer func_env.deinit();
        
        // Bind parameters to arguments
        for (func.parameters.items, 0..) |param, i| {
            const arg_value = if (i < args.len) args[i] else Value.Null;
            try func_env.define(param.name, arg_value);
        }
        
        // Save current environment and switch to function environment
        const old_env = self.current_env;
        const old_return = self.return_value;
        self.current_env = &func_env;
        self.return_value = null;
        
        // Execute function body
        for (func.body.items) |stmt| {
            try self.executeStatement(stmt);
            if (self.return_value != null) break;
        }
        
        // Get return value and restore environment
        const result = self.return_value orelse Value.Null;
        self.current_env = old_env;
        self.return_value = old_return;
        
        return result;
    }
    
    /// Built-in function: vibez.spill (print)
    fn builtinVibesSpill(self: *JITExecutionEngine, arguments: ArrayList(*ast.Expression)) !Value {
        for (arguments.items, 0..) |arg, i| {
            if (i > 0) print(" ", .{});
            
            const value = try self.evaluateExpression(arg.*);
            const str = try value.toString(self.allocator);
            defer self.allocator.free(str);
            
            print("{s}", .{str});
        }
        print("\n", .{});
        
        return Value.Null;
    }
    
    /// Add two values
    fn addValues(_: *JITExecutionEngine, left: Value, right: Value) !Value {
        switch (left) {
            .Integer => |left_int| switch (right) {
                .Integer => |right_int| return Value{ .Integer = left_int + right_int },
                .Float => |right_float| return Value{ .Float = @as(f64, @floatFromInt(left_int)) + right_float },
                else => return InterpreterError.TypeMismatch,
            },
            .Float => |left_float| switch (right) {
                .Integer => |right_int| return Value{ .Float = left_float + @as(f64, @floatFromInt(right_int)) },
                .Float => |right_float| return Value{ .Float = left_float + right_float },
                else => return InterpreterError.TypeMismatch,
            },
            .String => |left_str| switch (right) {
                .String => |right_str| {
                    // String concatenation - this is a simplified version
                    // In a real implementation, you'd need proper memory management
                    _ = left_str;
                    _ = right_str;
                    return Value{ .String = "concatenated_string" };
                },
                else => return InterpreterError.TypeMismatch,
            },
            else => return InterpreterError.TypeMismatch,
        }
    }
    
    /// Subtract two values
    fn subtractValues(_: *JITExecutionEngine, left: Value, right: Value) !Value {
        switch (left) {
            .Integer => |left_int| switch (right) {
                .Integer => |right_int| return Value{ .Integer = left_int - right_int },
                .Float => |right_float| return Value{ .Float = @as(f64, @floatFromInt(left_int)) - right_float },
                else => return InterpreterError.TypeMismatch,
            },
            .Float => |left_float| switch (right) {
                .Integer => |right_int| return Value{ .Float = left_float - @as(f64, @floatFromInt(right_int)) },
                .Float => |right_float| return Value{ .Float = left_float - right_float },
                else => return InterpreterError.TypeMismatch,
            },
            else => return InterpreterError.TypeMismatch,
        }
    }
    
    /// Multiply two values
    fn multiplyValues(_: *JITExecutionEngine, left: Value, right: Value) !Value {
        switch (left) {
            .Integer => |left_int| switch (right) {
                .Integer => |right_int| return Value{ .Integer = left_int * right_int },
                .Float => |right_float| return Value{ .Float = @as(f64, @floatFromInt(left_int)) * right_float },
                else => return InterpreterError.TypeMismatch,
            },
            .Float => |left_float| switch (right) {
                .Integer => |right_int| return Value{ .Float = left_float * @as(f64, @floatFromInt(right_int)) },
                .Float => |right_float| return Value{ .Float = left_float * right_float },
                else => return InterpreterError.TypeMismatch,
            },
            else => return InterpreterError.TypeMismatch,
        }
    }
    
    /// Divide two values
    fn divideValues(_: *JITExecutionEngine, left: Value, right: Value) !Value {
        switch (left) {
            .Integer => |left_int| switch (right) {
                .Integer => |right_int| {
                    if (right_int == 0) return InterpreterError.DivisionByZero;
                    return Value{ .Integer = @divTrunc(left_int, right_int) };
                },
                .Float => |right_float| {
                    if (right_float == 0.0) return InterpreterError.DivisionByZero;
                    return Value{ .Float = @as(f64, @floatFromInt(left_int)) / right_float };
                },
                else => return InterpreterError.TypeMismatch,
            },
            .Float => |left_float| switch (right) {
                .Integer => |right_int| {
                    if (right_int == 0) return InterpreterError.DivisionByZero;
                    return Value{ .Float = left_float / @as(f64, @floatFromInt(right_int)) };
                },
                .Float => |right_float| {
                    if (right_float == 0.0) return InterpreterError.DivisionByZero;
                    return Value{ .Float = left_float / right_float };
                },
                else => return InterpreterError.TypeMismatch,
            },
            else => return InterpreterError.TypeMismatch,
        }
    }
    
    /// Compare two values
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
                else => return InterpreterError.TypeMismatch,
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
                else => return InterpreterError.TypeMismatch,
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
                else => return InterpreterError.TypeMismatch,
            },
            .Boolean => |left_bool| switch (right) {
                .Boolean => |right_bool| return switch (comparison) {
                    .Equal => left_bool == right_bool,
                    .NotEqual => left_bool != right_bool,
                    else => return InterpreterError.TypeMismatch,
                },
                else => return InterpreterError.TypeMismatch,
            },
            else => return InterpreterError.TypeMismatch,
        }
    }
};

/// Test the JIT execution engine with a simple CURSED program
pub fn testJITExecutionEngine(allocator: Allocator) !void {
    print("\n🧪 Testing JIT Execution Engine\n", .{});
    print("==============================\n", .{});
    
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
    
    // Test 3: Arithmetic operations
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
    
    print("\n✅ JIT Execution Engine tests completed!\n", .{});
}
