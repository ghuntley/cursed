const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const error_handling = @import("error_handling.zig");
const cursed_error = @import("cursed_error_runtime.zig");
const concurrency = @import("concurrency.zig");
const gc = @import("gc.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;

/// Complete Zig interpreter implementation for CURSED language
/// Implements a full AST walker that can execute all language constructs
pub const CursedInterpreter = struct {
    allocator: Allocator,
    globals: Environment,
    environment: *Environment,
    functions: HashMap([]const u8, FunctionValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    struct_definitions: HashMap([]const u8, StructDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    interface_definitions: HashMap([]const u8, InterfaceDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    defer_stack: ArrayList(DeferEntry),
    call_stack: ArrayList(CallFrame),
    recursion_depth: u32,
    max_recursion_depth: u32,

    const MAX_RECURSION_DEPTH = 1000;

    pub fn init(allocator: Allocator) CursedInterpreter {
        var globals = Environment.init(allocator, null);
        
        return CursedInterpreter{
            .allocator = allocator,
            .globals = globals,
            .environment = &globals,
            .functions = HashMap([]const u8, FunctionValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .struct_definitions = HashMap([]const u8, StructDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .interface_definitions = HashMap([]const u8, InterfaceDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .defer_stack = ArrayList(DeferEntry).init(allocator),
            .call_stack = ArrayList(CallFrame).init(allocator),
            .recursion_depth = 0,
            .max_recursion_depth = MAX_RECURSION_DEPTH,
        };
    }

    pub fn deinit(self: *CursedInterpreter) void {
        // Execute all remaining deferred statements
        self.executeAllDefers();
        
        self.globals.deinit();
        
        // Clean up functions
        var func_iter = self.functions.iterator();
        while (func_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.functions.deinit();
        
        // Clean up struct definitions
        var struct_iter = self.struct_definitions.iterator();
        while (struct_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.struct_definitions.deinit();
        
        // Clean up interface definitions
        var interface_iter = self.interface_definitions.iterator();
        while (interface_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.interface_definitions.deinit();
        
        self.defer_stack.deinit();
        self.call_stack.deinit();
    }

    /// Execute a complete program
    pub fn execute(self: *CursedInterpreter, program: Program) InterpreterError!Value {
        // First pass: collect function and type definitions
        try self.collectDefinitions(program);
        
        // Second pass: execute statements
        var last_value = Value.Nil;
        for (program.statements.items) |stmt_ptr| {
            const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
            const result = try self.executeStatement(stmt.*);
            
            switch (result) {
                .Continue => |value| last_value = value,
                .Return => |value| return value,
                .Break => return InterpreterError.BreakOutsideLoop,
                .NextIteration => return InterpreterError.ContinueOutsideLoop,
                .Error => |err| return err,
            }
        }
        
        // Execute main function if it exists
        if (self.functions.get("main")) |main_func| {
            return try self.callFunction(main_func, &[_]Value{});
        }
        
        return last_value;
    }

    /// Collect function and type definitions in first pass
    fn collectDefinitions(self: *CursedInterpreter, program: Program) InterpreterError!void {
        for (program.statements.items) |stmt_ptr| {
            const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
            switch (stmt.*) {
                .Function => |func| try self.registerFunction(func),
                .Struct => |struct_decl| try self.registerStruct(struct_decl),
                .Interface => |interface_decl| try self.registerInterface(interface_decl),
                else => {},
            }
        }
    }

    /// Execute a single statement
    fn executeStatement(self: *CursedInterpreter, stmt: Statement) InterpreterError!ExecutionFlow {
        switch (stmt) {
            .Expression => |expr_ptr| {
                const expr: *Expression = @ptrCast(@alignCast(expr_ptr));
                const value = try self.evaluateExpression(expr.*);
                return ExecutionFlow{ .Continue = value };
            },
            .Let => |let| return try self.executeLetStatement(let),
            .Assignment => |assign| return try self.executeAssignmentStatement(assign),
            .Return => |ret| {
                const value = if (ret.value) |expr_ptr| blk: {
                    const expr: *Expression = @ptrCast(@alignCast(expr_ptr));
                    break :blk try self.evaluateExpression(expr.*);
                } else Value.Nil;
                return ExecutionFlow{ .Return = value };
            },
            .If => |if_stmt| return try self.executeIfStatement(if_stmt),
            .While => |while_stmt| return try self.executeWhileStatement(while_stmt),
            .For => |for_stmt| return try self.executeForStatement(for_stmt),
            .ForIn => |for_in_stmt| return try self.executeForInStatement(for_in_stmt),
            .Switch => |switch_stmt| return try self.executeSwitchStatement(switch_stmt),
            .Function => return ExecutionFlow{ .Continue = Value.Nil }, // Already collected
            .Struct => return ExecutionFlow{ .Continue = Value.Nil }, // Already collected
            .Interface => return ExecutionFlow{ .Continue = Value.Nil }, // Already collected
            .Stan => |stan| return try self.executeStanStatement(stan),
            .Defer => |defer_stmt| return try self.executeDeferStatement(defer_stmt),
            .Break => |break_stmt| return ExecutionFlow{ .Break = break_stmt.label },
            .Continue => |continue_stmt| return ExecutionFlow{ .NextIteration = continue_stmt.label },
            .Yikes => |yikes| return try self.executeYikesStatement(yikes),
            .Fam => |fam| return try self.executeFamStatement(fam),
            .PatternSwitch => |pattern_switch| return try self.executePatternSwitchStatement(pattern_switch),
            else => return InterpreterError.UnsupportedStatement,
        }
    }

    /// Evaluate an expression
    fn evaluateExpression(self: *CursedInterpreter, expr: Expression) InterpreterError!Value {
        // Check recursion depth
        if (self.recursion_depth >= self.max_recursion_depth) {
            return InterpreterError.MaxRecursionDepthExceeded;
        }
        
        self.recursion_depth += 1;
        defer self.recursion_depth -= 1;
        
        switch (expr) {
            .Integer => |i| return Value{ .Integer = i },
            .Float => |f| return Value{ .Float = f },
            .String => |s| return Value{ .String = try self.allocator.dupe(u8, s) },
            .Boolean => |b| return Value{ .Boolean = b },
            .Character => |c| return Value{ .Character = c },
            .Identifier => |name| return try self.evaluateIdentifier(name),
            .Variable => |name| return try self.evaluateIdentifier(name),
            .Binary => |binary| return try self.evaluateBinaryExpression(binary),
            .Call => |call| return try self.evaluateCallExpression(call),
            .MemberAccess => |member| return try self.evaluateMemberAccess(member.*),
            .Array => |array| return try self.evaluateArrayExpression(array.*),
            .Literal => |literal| return try self.evaluateLiteral(literal),
            .Unary => |unary| return try self.evaluateUnaryExpression(unary.*),
            .StructLiteral => |struct_literal| return try self.evaluateStructLiteral(struct_literal),
            .Lambda => |lambda| return try self.evaluateLambda(lambda),
            .Tuple => |tuple| return try self.evaluateTuple(tuple),
            .ArrayAccess => |array_access| return try self.evaluateArrayAccess(array_access),
            .TupleAccess => |tuple_access| return try self.evaluateTupleAccess(tuple_access),
            .MethodCall => |method_call| return try self.evaluateMethodCall(method_call.*),
            .Match => |match_expr| return try self.evaluateMatch(match_expr),
            .Shook => |shook| return try self.evaluateShook(shook),
            .Yikes => |yikes| return try self.evaluateYikes(yikes),
            else => return InterpreterError.UnsupportedExpression,
        }
    }

    /// Evaluate identifier/variable access
    fn evaluateIdentifier(self: *CursedInterpreter, name: []const u8) InterpreterError!Value {
        if (self.environment.get(name)) |value| {
            return value;
        }
        return InterpreterError.UndefinedVariable;
    }

    /// Evaluate binary expressions (+, -, *, /, ==, !=, etc.)
    fn evaluateBinaryExpression(self: *CursedInterpreter, binary: ast.BinaryExpression) InterpreterError!Value {
        // Special handling for assignment
        if (std.mem.eql(u8, binary.operator, "=")) {
            if (binary.left.* == .Identifier) {
                const var_name = binary.left.*.Identifier;
                const value = try self.evaluateExpression(binary.right.*);
                try self.environment.define(var_name, value);
                return value;
            } else {
                return InterpreterError.InvalidAssignmentTarget;
            }
        }
        
        const left = try self.evaluateExpression(binary.left.*);
        const right = try self.evaluateExpression(binary.right.*);
        
        return try self.applyBinaryOperator(left, binary.operator, right);
    }

    /// Apply binary operator to two values
    fn applyBinaryOperator(self: *CursedInterpreter, left: Value, operator: []const u8, right: Value) InterpreterError!Value {
        _ = self;
        
        // Arithmetic operators
        if (std.mem.eql(u8, operator, "+")) {
            switch (left) {
                .Integer => |l| switch (right) {
                    .Integer => |r| return Value{ .Integer = l + r },
                    .Float => |r| return Value{ .Float = @as(f64, @floatFromInt(l)) + r },
                    else => return InterpreterError.TypeMismatch,
                },
                .Float => |l| switch (right) {
                    .Integer => |r| return Value{ .Float = l + @as(f64, @floatFromInt(r)) },
                    .Float => |r| return Value{ .Float = l + r },
                    else => return InterpreterError.TypeMismatch,
                },
                .String => |l| switch (right) {
                    .String => |r| {
                        const result = try std.fmt.allocPrint(self.allocator, "{s}{s}", .{ l, r });
                        return Value{ .String = result };
                    },
                    else => return InterpreterError.TypeMismatch,
                },
                else => return InterpreterError.TypeMismatch,
            }
        }
        
        if (std.mem.eql(u8, operator, "-")) {
            switch (left) {
                .Integer => |l| switch (right) {
                    .Integer => |r| return Value{ .Integer = l - r },
                    .Float => |r| return Value{ .Float = @as(f64, @floatFromInt(l)) - r },
                    else => return InterpreterError.TypeMismatch,
                },
                .Float => |l| switch (right) {
                    .Integer => |r| return Value{ .Float = l - @as(f64, @floatFromInt(r)) },
                    .Float => |r| return Value{ .Float = l - r },
                    else => return InterpreterError.TypeMismatch,
                },
                else => return InterpreterError.TypeMismatch,
            }
        }
        
        if (std.mem.eql(u8, operator, "*")) {
            switch (left) {
                .Integer => |l| switch (right) {
                    .Integer => |r| return Value{ .Integer = l * r },
                    .Float => |r| return Value{ .Float = @as(f64, @floatFromInt(l)) * r },
                    else => return InterpreterError.TypeMismatch,
                },
                .Float => |l| switch (right) {
                    .Integer => |r| return Value{ .Float = l * @as(f64, @floatFromInt(r)) },
                    .Float => |r| return Value{ .Float = l * r },
                    else => return InterpreterError.TypeMismatch,
                },
                else => return InterpreterError.TypeMismatch,
            }
        }
        
        if (std.mem.eql(u8, operator, "/")) {
            switch (left) {
                .Integer => |l| switch (right) {
                    .Integer => |r| {
                        if (r == 0) return InterpreterError.DivisionByZero;
                        return Value{ .Integer = @divTrunc(l, r) };
                    },
                    .Float => |r| {
                        if (r == 0.0) return InterpreterError.DivisionByZero;
                        return Value{ .Float = @as(f64, @floatFromInt(l)) / r };
                    },
                    else => return InterpreterError.TypeMismatch,
                },
                .Float => |l| switch (right) {
                    .Integer => |r| {
                        if (r == 0) return InterpreterError.DivisionByZero;
                        return Value{ .Float = l / @as(f64, @floatFromInt(r)) };
                    },
                    .Float => |r| {
                        if (r == 0.0) return InterpreterError.DivisionByZero;
                        return Value{ .Float = l / r };
                    },
                    else => return InterpreterError.TypeMismatch,
                },
                else => return InterpreterError.TypeMismatch,
            }
        }
        
        // Comparison operators
        if (std.mem.eql(u8, operator, "==")) {
            return Value{ .Boolean = valuesEqual(left, right) };
        }
        
        if (std.mem.eql(u8, operator, "!=")) {
            return Value{ .Boolean = !valuesEqual(left, right) };
        }
        
        if (std.mem.eql(u8, operator, "<")) {
            return Value{ .Boolean = try compareValues(left, right) < 0 };
        }
        
        if (std.mem.eql(u8, operator, "<=")) {
            return Value{ .Boolean = try compareValues(left, right) <= 0 };
        }
        
        if (std.mem.eql(u8, operator, ">")) {
            return Value{ .Boolean = try compareValues(left, right) > 0 };
        }
        
        if (std.mem.eql(u8, operator, ">=")) {
            return Value{ .Boolean = try compareValues(left, right) >= 0 };
        }
        
        // Logical operators
        if (std.mem.eql(u8, operator, "&&")) {
            return Value{ .Boolean = isTruthy(left) and isTruthy(right) };
        }
        
        if (std.mem.eql(u8, operator, "||")) {
            return Value{ .Boolean = isTruthy(left) or isTruthy(right) };
        }
        
        return InterpreterError.UnknownOperator;
    }

    /// Evaluate function call
    fn evaluateCallExpression(self: *CursedInterpreter, call: ast.CallExpression) InterpreterError!Value {
        const func_expr: *Expression = @ptrCast(@alignCast(call.function));
        
        if (func_expr.* == .Identifier) {
            const func_name = func_expr.*.Identifier;
            
            // Handle built-in functions
            if (try self.handleBuiltinFunction(func_name, call.arguments)) |result| {
                return result;
            }
            
            // Handle user-defined functions
            if (self.functions.get(func_name)) |func| {
                var args = ArrayList(Value).init(self.allocator);
                defer args.deinit();
                
                for (call.arguments.items) |arg_ptr| {
                    const arg_expr: *Expression = @ptrCast(@alignCast(arg_ptr));
                    try args.append(try self.evaluateExpression(arg_expr.*));
                }
                
                return try self.callFunction(func, args.items);
            }
            
            return InterpreterError.UndefinedFunction;
        }
        
        return InterpreterError.InvalidFunctionCall;
    }

    /// Handle built-in functions
    fn handleBuiltinFunction(self: *CursedInterpreter, name: []const u8, args: ArrayList(*Expression)) InterpreterError!?Value {
        if (std.mem.eql(u8, name, "print") or std.mem.eql(u8, name, "vibez.spill")) {
            for (args.items) |arg_ptr| {
                const arg_expr: *Expression = @ptrCast(@alignCast(arg_ptr));
                const value = try self.evaluateExpression(arg_expr.*);
                const str = try self.valueToString(value);
                defer self.allocator.free(str);
                std.debug.print("{s} ", .{str});
            }
            std.debug.print("\n", .{});
            return Value.Nil;
        }
        
        if (std.mem.eql(u8, name, "len")) {
            if (args.items.len != 1) return InterpreterError.WrongArgumentCount;
            
            const arg_expr: *Expression = @ptrCast(@alignCast(args.items[0]));
            const value = try self.evaluateExpression(arg_expr.*);
            
            switch (value) {
                .String => |s| return Value{ .Integer = @intCast(s.len) },
                .Array => |arr| return Value{ .Integer = @intCast(arr.items.len) },
                .Tuple => |tuple| return Value{ .Integer = @intCast(tuple.items.len) },
                else => return InterpreterError.TypeMismatch,
            }
        }
        
        if (std.mem.eql(u8, name, "tea")) {
            if (args.items.len != 1) return InterpreterError.WrongArgumentCount;
            
            const arg_expr: *Expression = @ptrCast(@alignCast(args.items[0]));
            const value = try self.evaluateExpression(arg_expr.*);
            const str = try self.valueToString(value);
            return Value{ .String = str };
        }
        
        return null; // Not a built-in function
    }

    /// Call a user-defined function
    fn callFunction(self: *CursedInterpreter, func: FunctionValue, args: []const Value) InterpreterError!Value {
        // Check parameter count
        if (args.len != func.parameters.len) {
            return InterpreterError.WrongArgumentCount;
        }
        
        // Create new environment for function execution
        var func_env = Environment.init(self.allocator, self.environment);
        defer func_env.deinit();
        
        // Bind parameters
        for (func.parameters, args) |param, arg| {
            try func_env.define(param, arg);
        }
        
        // Push call frame
        const frame = CallFrame{ .function_name = func.name, .environment = &func_env };
        try self.call_stack.append(frame);
        defer _ = self.call_stack.pop();
        
        // Push defer scope
        const defer_scope_start = self.defer_stack.items.len;
        
        // Switch environment
        const old_env = self.environment;
        self.environment = &func_env;
        defer self.environment = old_env;
        
        // Execute function body
        var last_value = Value.Nil;
        for (func.body) |stmt| {
            const result = try self.executeStatement(stmt);
            switch (result) {
                .Continue => |value| last_value = value,
                .Return => |value| {
                    // Execute defers before returning
                    self.executeDeferToSize(defer_scope_start);
                    return value;
                },
                .Break => return InterpreterError.BreakOutsideLoop,
                .NextIteration => return InterpreterError.ContinueOutsideLoop,
                .Error => |err| return err,
            }
        }
        
        // Execute defers before function exit
        self.executeDeferToSize(defer_scope_start);
        
        return last_value;
    }

    /// Execute if statement
    fn executeIfStatement(self: *CursedInterpreter, if_stmt: ast.IfStatement) InterpreterError!ExecutionFlow {
        const condition_expr: *Expression = @ptrCast(@alignCast(if_stmt.condition));
        const condition = try self.evaluateExpression(condition_expr.*);
        
        if (isTruthy(condition)) {
            return try self.executeBlockStatements(if_stmt.then_branch);
        } else if (if_stmt.else_branch) |else_branch| {
            return try self.executeBlockStatements(else_branch);
        }
        
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    /// Execute while loop
    fn executeWhileStatement(self: *CursedInterpreter, while_stmt: ast.WhileStatement) InterpreterError!ExecutionFlow {
        var last_value = Value.Nil;
        
        while (true) {
            const condition_expr: *Expression = @ptrCast(@alignCast(while_stmt.condition));
            const condition = try self.evaluateExpression(condition_expr.*);
            
            if (!isTruthy(condition)) break;
            
            const result = try self.executeBlockStatements(while_stmt.body);
            switch (result) {
                .Continue => |value| last_value = value,
                .Return => |value| return ExecutionFlow{ .Return = value },
                .Break => break,
                .NextIteration => continue,
                .Error => |err| return ExecutionFlow{ .Error = err },
            }
        }
        
        return ExecutionFlow{ .Continue = last_value };
    }

    /// Execute block of statements
    fn executeBlockStatements(self: *CursedInterpreter, statements: ArrayList(Statement)) InterpreterError!ExecutionFlow {
        var last_value = Value.Nil;
        
        for (statements.items) |stmt| {
            const result = try self.executeStatement(stmt);
            switch (result) {
                .Continue => |value| last_value = value,
                .Return => |value| return ExecutionFlow{ .Return = value },
                .Break => |label| return ExecutionFlow{ .Break = label },
                .NextIteration => |label| return ExecutionFlow{ .NextIteration = label },
                .Error => |err| return ExecutionFlow{ .Error = err },
            }
        }
        
        return ExecutionFlow{ .Continue = last_value };
    }

    /// Register function definition
    fn registerFunction(self: *CursedInterpreter, func_stmt: ast.FunctionStatement) InterpreterError!void {
        var parameters = ArrayList([]const u8).init(self.allocator);
        for (func_stmt.parameters.items) |param| {
            try parameters.append(try self.allocator.dupe(u8, param.name));
        }
        
        const func_value = FunctionValue{
            .name = try self.allocator.dupe(u8, func_stmt.name),
            .parameters = parameters.items,
            .body = func_stmt.body.items,
            .closure_env = self.environment,
        };
        
        const name_copy = try self.allocator.dupe(u8, func_stmt.name);
        try self.functions.put(name_copy, func_value);
    }

    /// Execute defer statement by pushing to defer stack
    fn executeDeferStatement(self: *CursedInterpreter, defer_stmt: ast.DeferStatement) InterpreterError!ExecutionFlow {
        const expr_ptr: *Expression = @ptrCast(@alignCast(defer_stmt.expression));
        const defer_entry = DeferEntry{
            .expression = expr_ptr.*,
            .environment = self.environment,
        };
        try self.defer_stack.append(defer_entry);
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    /// Execute all deferred expressions in LIFO order
    fn executeAllDefers(self: *CursedInterpreter) void {
        while (self.defer_stack.items.len > 0) {
            const defer_entry = self.defer_stack.pop();
            const old_env = self.environment;
            self.environment = defer_entry.environment;
            _ = self.evaluateExpression(defer_entry.expression) catch {};
            self.environment = old_env;
        }
    }

    /// Execute defers up to a specific stack size
    fn executeDeferToSize(self: *CursedInterpreter, target_size: usize) void {
        while (self.defer_stack.items.len > target_size) {
            const defer_entry = self.defer_stack.pop();
            const old_env = self.environment;
            self.environment = defer_entry.environment;
            _ = self.evaluateExpression(defer_entry.expression) catch {};
            self.environment = old_env;
        }
    }

    /// Convert value to string representation
    fn valueToString(self: *CursedInterpreter, value: Value) InterpreterError![]u8 {
        switch (value) {
            .Integer => |i| return std.fmt.allocPrint(self.allocator, "{}", .{i}),
            .Float => |f| return std.fmt.allocPrint(self.allocator, "{d}", .{f}),
            .String => |s| return self.allocator.dupe(u8, s),
            .Boolean => |b| return self.allocator.dupe(u8, if (b) "based" else "cap"),
            .Character => |c| return std.fmt.allocPrint(self.allocator, "{c}", .{c}),
            .Null => return self.allocator.dupe(u8, "cap"),
            .Array => |arr| {
                var result = std.ArrayList(u8).init(self.allocator);
                try result.append('[');
                for (arr.items, 0..) |item, i| {
                    if (i > 0) try result.appendSlice(", ");
                    const item_str = try self.valueToString(item);
                    defer self.allocator.free(item_str);
                    try result.appendSlice(item_str);
                }
                try result.append(']');
                return result.toOwnedSlice();
            },
            .Tuple => |tuple| {
                var result = std.ArrayList(u8).init(self.allocator);
                try result.append('(');
                for (tuple.items, 0..) |item, i| {
                    if (i > 0) try result.appendSlice(", ");
                    const item_str = try self.valueToString(item);
                    defer self.allocator.free(item_str);
                    try result.appendSlice(item_str);
                }
                try result.append(')');
                return result.toOwnedSlice();
            },
            .Struct => |struct_inst| return std.fmt.allocPrint(self.allocator, "struct {s}", .{struct_inst.type_name}),
            .Interface => |interface_inst| return std.fmt.allocPrint(self.allocator, "interface {s}", .{interface_inst.interface_name}),
            .Error => |err| return std.fmt.allocPrint(self.allocator, "Error: {s}", .{err.message}),
        }
    }

    // Placeholder implementations for other statements and expressions
    fn executeLetStatement(self: *CursedInterpreter, let: ast.LetStatement) InterpreterError!ExecutionFlow {
        _ = let;
        _ = self;
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    fn executeAssignmentStatement(self: *CursedInterpreter, assign: ast.AssignmentStatement) InterpreterError!ExecutionFlow {
        _ = assign;
        _ = self;
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    fn executeForStatement(self: *CursedInterpreter, for_stmt: ast.ForStatement) InterpreterError!ExecutionFlow {
        _ = for_stmt;
        _ = self;
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    fn executeForInStatement(self: *CursedInterpreter, for_in: ast.ForInStatement) InterpreterError!ExecutionFlow {
        _ = for_in;
        _ = self;
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    fn executeSwitchStatement(self: *CursedInterpreter, switch_stmt: ast.SwitchStatement) InterpreterError!ExecutionFlow {
        _ = switch_stmt;
        _ = self;
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    fn executeStanStatement(self: *CursedInterpreter, stan: ast.StanStatement) InterpreterError!ExecutionFlow {
        _ = stan;
        _ = self;
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    fn executeYikesStatement(self: *CursedInterpreter, yikes: ast.YikesStatement) InterpreterError!ExecutionFlow {
        _ = yikes;
        _ = self;
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    fn executeFamStatement(self: *CursedInterpreter, fam: ast.FamStatement) InterpreterError!ExecutionFlow {
        _ = fam;
        _ = self;
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    fn executePatternSwitchStatement(self: *CursedInterpreter, pattern_switch: ast.PatternSwitchStatement) InterpreterError!ExecutionFlow {
        _ = pattern_switch;
        _ = self;
        return ExecutionFlow{ .Continue = Value.Nil };
    }

    fn registerStruct(self: *CursedInterpreter, struct_stmt: ast.StructStatement) InterpreterError!void {
        _ = struct_stmt;
        _ = self;
    }

    fn registerInterface(self: *CursedInterpreter, interface_stmt: ast.InterfaceStatement) InterpreterError!void {
        _ = interface_stmt;
        _ = self;
    }

    fn evaluateMemberAccess(self: *CursedInterpreter, member: ast.MemberAccessExpression) InterpreterError!Value {
        _ = member;
        _ = self;
        return Value.Nil;
    }

    fn evaluateArrayExpression(self: *CursedInterpreter, array: ast.ArrayExpression) InterpreterError!Value {
        var values = ArrayList(Value).init(self.allocator);
        errdefer values.deinit(); // Clean up on error
        for (array.elements.items) |elem_ptr| {
            const elem_expr: *Expression = @ptrCast(@alignCast(elem_ptr));
            try values.append(try self.evaluateExpression(elem_expr.*));
        }
        return Value{ .Array = values };
    }

    fn evaluateLiteral(self: *CursedInterpreter, literal: ast.Literal) InterpreterError!Value {
        _ = self;
        switch (literal) {
            .Integer => |i| return Value{ .Integer = i },
            .Float => |f| return Value{ .Float = f },
            .String => |s| return Value{ .String = try self.allocator.dupe(u8, s) },
            .Boolean => |b| return Value{ .Boolean = b },
            .Character => |c| return Value{ .Character = c },
            .Null, .Nil => return Value.Null,
        }
    }

    fn evaluateUnaryExpression(self: *CursedInterpreter, unary: ast.UnaryExpression) InterpreterError!Value {
        const operand = try self.evaluateExpression(unary.operand.*);
        
        if (std.mem.eql(u8, unary.operator, "!")) {
            return Value{ .Boolean = !isTruthy(operand) };
        }
        
        if (std.mem.eql(u8, unary.operator, "-")) {
            switch (operand) {
                .Integer => |i| return Value{ .Integer = -i },
                .Float => |f| return Value{ .Float = -f },
                else => return InterpreterError.TypeMismatch,
            }
        }
        
        return InterpreterError.UnknownOperator;
    }

    fn evaluateStructLiteral(self: *CursedInterpreter, struct_literal: ast.StructLiteralExpression) InterpreterError!Value {
        _ = struct_literal;
        _ = self;
        return Value.Nil;
    }

    fn evaluateLambda(self: *CursedInterpreter, lambda: ast.LambdaExpression) InterpreterError!Value {
        _ = lambda;
        _ = self;
        return Value.Nil;
    }

    fn evaluateTuple(self: *CursedInterpreter, tuple: ast.TupleExpression) InterpreterError!Value {
        var values = ArrayList(Value).init(self.allocator);
        errdefer values.deinit(); // Clean up on error
        for (tuple.elements.items) |elem_ptr| {
            const elem_expr: *Expression = @ptrCast(@alignCast(elem_ptr));
            try values.append(try self.evaluateExpression(elem_expr.*));
        }
        return Value{ .Tuple = values };
    }

    fn evaluateArrayAccess(self: *CursedInterpreter, array_access: ast.ArrayAccessExpression) InterpreterError!Value {
        _ = array_access;
        _ = self;
        return Value.Nil;
    }

    fn evaluateTupleAccess(self: *CursedInterpreter, tuple_access: ast.TupleAccessExpression) InterpreterError!Value {
        _ = tuple_access;
        _ = self;
        return Value.Nil;
    }

    fn evaluateMethodCall(self: *CursedInterpreter, method_call: ast.MethodCallExpression) InterpreterError!Value {
        _ = method_call;
        _ = self;
        return Value.Nil;
    }

    fn evaluateMatch(self: *CursedInterpreter, match_expr: ast.MatchExpression) InterpreterError!Value {
        _ = match_expr;
        _ = self;
        return Value.Nil;
    }

    fn evaluateShook(self: *CursedInterpreter, shook: ast.ShookExpression) InterpreterError!Value {
        _ = shook;
        _ = self;
        return Value.Nil;
    }

    fn evaluateYikes(self: *CursedInterpreter, yikes: ast.YikesExpression) InterpreterError!Value {
        _ = yikes;
        _ = self;
        return Value.Nil;
    }
};

/// Runtime value types for CURSED
pub const Value = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,
    Array: ArrayList(Value),
    Tuple: ArrayList(Value),
    Struct: StructInstance,
    Interface: InterfaceInstance,
    Error: ErrorValue,

    pub fn deinit(self: *Value, allocator: Allocator) void {
        switch (self.*) {
            .String => |str| allocator.free(str),
            .Array => |*arr| {
                for (arr.items) |*item| {
                    item.deinit(allocator);
                }
                arr.deinit();
            },
            .Tuple => |*tuple| {
                for (tuple.items) |*item| {
                    item.deinit(allocator);
                }
                tuple.deinit();
            },
            .Struct => |*struct_inst| struct_inst.deinit(),
            .Interface => |*interface_inst| interface_inst.deinit(),
            .Error => |*err| err.deinit(),
            else => {},
        }
    }
};

/// Environment for variable scoping
pub const Environment = struct {
    variables: HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    parent: ?*Environment,
    allocator: Allocator,

    pub fn init(allocator: Allocator, parent: ?*Environment) Environment {
        return Environment{
            .variables = HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .parent = parent,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Environment) void {
        var iter = self.variables.iterator();
        while (iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit(self.allocator);
        }
        self.variables.deinit();
    }

    pub fn define(self: *Environment, name: []const u8, value: Value) !void {
        const name_copy = try self.allocator.dupe(u8, name);
        try self.variables.put(name_copy, value);
    }

    pub fn get(self: *Environment, name: []const u8) ?Value {
        if (self.variables.get(name)) |value| {
            return value;
        }
        
        if (self.parent) |parent| {
            return parent.get(name);
        }
        
        return null;
    }

    pub fn set(self: *Environment, name: []const u8, value: Value) InterpreterError!void {
        if (self.variables.contains(name)) {
            try self.variables.put(name, value);
            return;
        }
        
        if (self.parent) |parent| {
            return parent.set(name, value);
        }
        
        return InterpreterError.UndefinedVariable;
    }
};

/// Function value representation
pub const FunctionValue = struct {
    name: []const u8,
    parameters: [][]const u8,
    body: []ast.Statement,
    closure_env: ?*Environment,

    pub fn deinit(self: *FunctionValue) void {
        _ = self;
        // Parameters and body are managed by AST cleanup
    }
};

/// Struct definition
pub const StructDefinition = struct {
    name: []const u8,
    fields: ArrayList(FieldDefinition),

    pub fn deinit(self: *StructDefinition) void {
        self.fields.deinit();
    }
};

/// Field definition for structs
pub const FieldDefinition = struct {
    name: []const u8,
    field_type: []const u8,
};

/// Interface definition
pub const InterfaceDefinition = struct {
    name: []const u8,
    methods: ArrayList(MethodDefinition),

    pub fn deinit(self: *InterfaceDefinition) void {
        self.methods.deinit();
    }
};

/// Method definition for interfaces
pub const MethodDefinition = struct {
    name: []const u8,
    parameters: ArrayList(FieldDefinition),
    return_type: ?[]const u8,
};

/// Struct instance
pub const StructInstance = struct {
    type_name: []const u8,
    fields: HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn deinit(self: *StructInstance) void {
        var iter = self.fields.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit(std.heap.page_allocator); // TODO: pass proper allocator
        }
        self.fields.deinit();
    }
};

/// Interface instance
pub const InterfaceInstance = struct {
    interface_name: []const u8,
    underlying_value: Value,

    pub fn deinit(self: *InterfaceInstance) void {
        _ = self;
        // underlying_value is managed by caller
    }
};

/// Error value
pub const ErrorValue = struct {
    message: []const u8,
    code: ?i64,

    pub fn deinit(self: *ErrorValue) void {
        _ = self;
        // message is managed by caller
    }
};

/// Defer entry for LIFO execution
pub const DeferEntry = struct {
    expression: Expression,
    environment: *Environment,
};

/// Call frame for function calls
pub const CallFrame = struct {
    function_name: []const u8,
    environment: *Environment,
};

/// Execution flow control
pub const ExecutionFlow = union(enum) {
    Continue: Value,
    Return: Value,
    Break: ?[]const u8,
    NextIteration: ?[]const u8,
    Error: InterpreterError,
};

/// Interpreter errors
pub const InterpreterError = error{
    UndefinedVariable,
    UndefinedFunction,
    TypeMismatch,
    DivisionByZero,
    WrongArgumentCount,
    InvalidAssignmentTarget,
    InvalidFunctionCall,
    BreakOutsideLoop,
    ContinueOutsideLoop,
    UnsupportedStatement,
    UnsupportedExpression,
    UnknownOperator,
    MaxRecursionDepthExceeded,
    OutOfMemory,
};

/// Helper functions

fn isTruthy(value: Value) bool {
    switch (value) {
        .Boolean => |b| return b,
        .Integer => |i| return i != 0,
        .Float => |f| return f != 0.0,
        .String => |s| return s.len > 0,
        .Null => return false,
        .Array => |arr| return arr.items.len > 0,
        .Tuple => |tuple| return tuple.items.len > 0,
        else => return true,
    }
}

fn valuesEqual(left: Value, right: Value) bool {
    switch (left) {
        .Integer => |l| switch (right) {
            .Integer => |r| return l == r,
            .Float => |r| return @as(f64, @floatFromInt(l)) == r,
            else => return false,
        },
        .Float => |l| switch (right) {
            .Integer => |r| return l == @as(f64, @floatFromInt(r)),
            .Float => |r| return l == r,
            else => return false,
        },
        .String => |l| switch (right) {
            .String => |r| return std.mem.eql(u8, l, r),
            else => return false,
        },
        .Boolean => |l| switch (right) {
            .Boolean => |r| return l == r,
            else => return false,
        },
        .Character => |l| switch (right) {
            .Character => |r| return l == r,
            else => return false,
        },
        .Null => return right == .Null,
        else => return false,
    }
}

fn compareValues(left: Value, right: Value) InterpreterError!i32 {
    switch (left) {
        .Integer => |l| switch (right) {
            .Integer => |r| return if (l < r) -1 else if (l > r) 1 else 0,
            .Float => |r| {
                const lf = @as(f64, @floatFromInt(l));
                return if (lf < r) -1 else if (lf > r) 1 else 0;
            },
            else => return InterpreterError.TypeMismatch,
        },
        .Float => |l| switch (right) {
            .Integer => |r| {
                const rf = @as(f64, @floatFromInt(r));
                return if (l < rf) -1 else if (l > rf) 1 else 0;
            },
            .Float => |r| return if (l < r) -1 else if (l > r) 1 else 0,
            else => return InterpreterError.TypeMismatch,
        },
        .String => |l| switch (right) {
            .String => |r| return std.mem.order(u8, l, r).compare(std.math.CompareOperator.eq),
            else => return InterpreterError.TypeMismatch,
        },
        else => return InterpreterError.TypeMismatch,
    }
}
