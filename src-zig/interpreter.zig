const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;

pub const InterpreterError = error{
    UndefinedVariable,
    UndefinedFunction,
    TypeMismatch,
    DivisionByZero,
    RuntimeError,
    OutOfMemory,
};

pub const Value = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,

    pub fn toString(self: Value, allocator: Allocator) ![]u8 {
        switch (self) {
            .Integer => |int| return std.fmt.allocPrint(allocator, "{}", .{int}),
            .Float => |float| return std.fmt.allocPrint(allocator, "{d}", .{float}),
            .String => |str| return allocator.dupe(u8, str),
            .Boolean => |bool_val| return allocator.dupe(u8, if (bool_val) "based" else "cap"),
            .Character => |char| return std.fmt.allocPrint(allocator, "{c}", .{char}),
            .Null => return allocator.dupe(u8, "cap"),
        }
    }

    pub fn toBool(self: Value) bool {
        switch (self) {
            .Boolean => |bool_val| return bool_val,
            .Integer => |int| return int != 0,
            .Float => |float| return float != 0.0,
            .String => |str| return str.len > 0,
            .Character => |char| return char != 0,
            .Null => return false,
        }
    }

    pub fn isNumber(self: Value) bool {
        return switch (self) {
            .Integer, .Float => true,
            else => false,
        };
    }

    pub fn toNumber(self: Value) InterpreterError!f64 {
        switch (self) {
            .Integer => |int| return @floatFromInt(int),
            .Float => |float| return float,
            else => return InterpreterError.TypeMismatch,
        }
    }
};

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
        self.variables.deinit();
    }

    pub fn define(self: *Environment, name: []const u8, value: Value) !void {
        try self.variables.put(name, value);
    }

    pub fn get(self: *Environment, name: []const u8) InterpreterError!Value {
        if (self.variables.get(name)) |value| {
            return value;
        }
        
        if (self.parent) |parent| {
            return parent.get(name);
        }
        
        return InterpreterError.UndefinedVariable;
    }

    pub fn set(self: *Environment, name: []const u8, value: Value) InterpreterError!void {
        if (self.variables.contains(name)) {
            try self.variables.put(name, value);
            return;
        }
        
        if (self.parent) |parent| {
            try parent.set(name, value);
            return;
        }
        
        return InterpreterError.UndefinedVariable;
    }
};

pub const CursedFunction = struct {
    declaration: ast.FunctionStatement,
    closure: *Environment,
};

pub const Interpreter = struct {
    globals: Environment,
    environment: *Environment,
    functions: HashMap([]const u8, CursedFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,

    pub fn init(allocator: Allocator) Interpreter {
        var globals = Environment.init(allocator, null);
        
        return Interpreter{
            .globals = globals,
            .environment = &globals,
            .functions = HashMap([]const u8, CursedFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Interpreter) void {
        self.globals.deinit();
        self.functions.deinit();
    }

    pub fn execute(self: *Interpreter, program: Program) InterpreterError!void {
        // First pass: collect function declarations
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .Function => |func| {
                    const cursed_func = CursedFunction{
                        .declaration = func,
                        .closure = self.environment,
                    };
                    try self.functions.put(func.name, cursed_func);
                },
                else => {},
            }
        }
        
        // Execute main_character function if it exists
        if (self.functions.get("main_character")) |main_func| {
            _ = try self.callFunction(main_func, &[_]Value{});
        } else {
            // Execute statements in order
            for (program.statements.items) |stmt| {
                try self.executeStatement(stmt);
            }
        }
    }

    fn executeStatement(self: *Interpreter, stmt: Statement) InterpreterError!void {
        switch (stmt) {
            .Expression => |expr| {
                _ = try self.evaluateExpression(expr);
            },
            .Let => |let| try self.executeLetStatement(let),
            .Return => |ret| {
                // Return statements are handled in function context
                _ = ret;
                // For now, just ignore returns outside functions
            },
            .If => |if_stmt| try self.executeIfStatement(if_stmt),
            .While => |while_stmt| try self.executeWhileStatement(while_stmt),
            .Function => {
                // Functions are already collected, skip execution
            },
            else => {
                std.debug.print("Unsupported statement type in interpreter: {s}\n", .{@tagName(stmt)});
            },
        }
    }

    fn executeLetStatement(self: *Interpreter, let: ast.LetStatement) InterpreterError!void {
        const value = if (let.initializer) |initializer_expr|
            try self.evaluateExpression(initializer_expr)
        else
            Value.Null;
        
        try self.environment.define(let.name, value);
    }

    fn executeIfStatement(self: *Interpreter, if_stmt: ast.IfStatement) InterpreterError!void {
        const condition = try self.evaluateExpression(if_stmt.condition);
        
        if (condition.toBool()) {
            for (if_stmt.then_branch.items) |stmt| {
                try self.executeStatement(stmt);
            }
        } else if (if_stmt.else_branch) |else_stmts| {
            for (else_stmts.items) |stmt| {
                try self.executeStatement(stmt);
            }
        }
    }

    fn executeWhileStatement(self: *Interpreter, while_stmt: ast.WhileStatement) InterpreterError!void {
        while (true) {
            const condition = try self.evaluateExpression(while_stmt.condition);
            if (!condition.toBool()) break;
            
            for (while_stmt.body.items) |stmt| {
                try self.executeStatement(stmt);
            }
        }
    }

    fn evaluateExpression(self: *Interpreter, expr: Expression) InterpreterError!Value {
        switch (expr) {
            .Integer => |int| return Value{ .Integer = int },
            .Float => |float| return Value{ .Float = float },
            .String => |str| return Value{ .String = str },
            .Boolean => |bool_val| return Value{ .Boolean = bool_val },
            .Character => |char| return Value{ .Character = char },
            .Identifier => |name| return try self.environment.get(name),
            .Binary => |bin| return try self.evaluateBinary(bin),
            .Call => |call| return try self.evaluateCall(call),
            .MemberAccess => |member| return try self.evaluateMemberAccess(member),
            else => {
                std.debug.print("Unsupported expression type in interpreter: {s}\n", .{@tagName(expr)});
                return Value.Null;
            },
        }
    }

    fn evaluateBinary(self: *Interpreter, bin: ast.BinaryExpression) InterpreterError!Value {
        const left = try self.evaluateExpression(bin.left.*);
        const right = try self.evaluateExpression(bin.right.*);
        
        if (std.mem.eql(u8, bin.operator, "+")) {
            if (left.isNumber() and right.isNumber()) {
                const left_num = try left.toNumber();
                const right_num = try right.toNumber();
                return Value{ .Float = left_num + right_num };
            }
        } else if (std.mem.eql(u8, bin.operator, "-")) {
            if (left.isNumber() and right.isNumber()) {
                const left_num = try left.toNumber();
                const right_num = try right.toNumber();
                return Value{ .Float = left_num - right_num };
            }
        } else if (std.mem.eql(u8, bin.operator, "*")) {
            if (left.isNumber() and right.isNumber()) {
                const left_num = try left.toNumber();
                const right_num = try right.toNumber();
                return Value{ .Float = left_num * right_num };
            }
        } else if (std.mem.eql(u8, bin.operator, "/")) {
            if (left.isNumber() and right.isNumber()) {
                const left_num = try left.toNumber();
                const right_num = try right.toNumber();
                if (right_num == 0.0) return InterpreterError.DivisionByZero;
                return Value{ .Float = left_num / right_num };
            }
        } else if (std.mem.eql(u8, bin.operator, "==")) {
            return Value{ .Boolean = self.valuesEqual(left, right) };
        } else if (std.mem.eql(u8, bin.operator, "!=")) {
            return Value{ .Boolean = !self.valuesEqual(left, right) };
        } else if (std.mem.eql(u8, bin.operator, "<")) {
            if (left.isNumber() and right.isNumber()) {
                const left_num = try left.toNumber();
                const right_num = try right.toNumber();
                return Value{ .Boolean = left_num < right_num };
            }
        } else if (std.mem.eql(u8, bin.operator, "<=")) {
            if (left.isNumber() and right.isNumber()) {
                const left_num = try left.toNumber();
                const right_num = try right.toNumber();
                return Value{ .Boolean = left_num <= right_num };
            }
        } else if (std.mem.eql(u8, bin.operator, ">")) {
            if (left.isNumber() and right.isNumber()) {
                const left_num = try left.toNumber();
                const right_num = try right.toNumber();
                return Value{ .Boolean = left_num > right_num };
            }
        } else if (std.mem.eql(u8, bin.operator, ">=")) {
            if (left.isNumber() and right.isNumber()) {
                const left_num = try left.toNumber();
                const right_num = try right.toNumber();
                return Value{ .Boolean = left_num >= right_num };
            }
        } else if (std.mem.eql(u8, bin.operator, "&&")) {
            return Value{ .Boolean = left.toBool() and right.toBool() };
        } else if (std.mem.eql(u8, bin.operator, "||")) {
            return Value{ .Boolean = left.toBool() or right.toBool() };
        }
        
        return InterpreterError.TypeMismatch;
    }

    fn evaluateCall(self: *Interpreter, call: ast.CallExpression) InterpreterError!Value {
        // Handle built-in functions
        switch (call.function.*) {
            .MemberAccess => |member| {
                if (std.mem.eql(u8, member.property, "spill")) {
                    // vibez.spill - print function
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const arg = try self.evaluateExpression(call.arguments.items[0]);
                    const str = try arg.toString(self.allocator);
                    defer self.allocator.free(str);
                    
                    std.debug.print("{s}\n", .{str});
                    return Value.Null;
                }
            },
            .Identifier => |name| {
                if (self.functions.get(name)) |func| {
                    // Evaluate arguments
                    var args = ArrayList(Value).init(self.allocator);
                    defer args.deinit();
                    
                    for (call.arguments.items) |arg_expr| {
                        const arg = try self.evaluateExpression(arg_expr);
                        try args.append(arg);
                    }
                    
                    return try self.callFunction(func, args.items);
                }
            },
            else => {},
        }
        
        return InterpreterError.UndefinedFunction;
    }

    fn evaluateMemberAccess(self: *Interpreter, member: ast.MemberAccessExpression) InterpreterError!Value {
        // For now, just evaluate the object
        return try self.evaluateExpression(member.object.*);
    }

    fn callFunction(self: *Interpreter, func: CursedFunction, args: []Value) InterpreterError!Value {
        // Create new environment for function execution
        var function_env = Environment.init(self.allocator, func.closure);
        defer function_env.deinit();
        
        // Bind parameters
        if (args.len != func.declaration.parameters.items.len) {
            return InterpreterError.TypeMismatch;
        }
        
        for (func.declaration.parameters.items, 0..) |param, i| {
            try function_env.define(param.name, args[i]);
        }
        
        // Execute function body
        const previous_env = self.environment;
        self.environment = &function_env;
        defer self.environment = previous_env;
        
        for (func.declaration.body.items) |stmt| {
            switch (stmt) {
                .Return => |ret| {
                    if (ret.value) |value| {
                        return try self.evaluateExpression(value);
                    } else {
                        return Value.Null;
                    }
                },
                else => try self.executeStatement(stmt),
            }
        }
        
        return Value.Null;
    }

    fn valuesEqual(self: *Interpreter, left: Value, right: Value) bool {
        _ = self;
        
        switch (left) {
            .Integer => |left_int| {
                switch (right) {
                    .Integer => |right_int| return left_int == right_int,
                    .Float => |right_float| return @as(f64, @floatFromInt(left_int)) == right_float,
                    else => return false,
                }
            },
            .Float => |left_float| {
                switch (right) {
                    .Float => |right_float| return left_float == right_float,
                    .Integer => |right_int| return left_float == @as(f64, @floatFromInt(right_int)),
                    else => return false,
                }
            },
            .String => |left_str| {
                switch (right) {
                    .String => |right_str| return std.mem.eql(u8, left_str, right_str),
                    else => return false,
                }
            },
            .Boolean => |left_bool| {
                switch (right) {
                    .Boolean => |right_bool| return left_bool == right_bool,
                    else => return false,
                }
            },
            .Character => |left_char| {
                switch (right) {
                    .Character => |right_char| return left_char == right_char,
                    else => return false,
                }
            },
            .Null => {
                switch (right) {
                    .Null => return true,
                    else => return false,
                }
            },
        }
    }
};

test "interpreter basic" {
    const allocator = std.testing.allocator;
    
    var interpreter = Interpreter.init(allocator);
    defer interpreter.deinit();
    
    // Test basic value operations
    const int_val = Value{ .Integer = 42 };
    const str = try int_val.toString(allocator);
    defer allocator.free(str);
    
    try std.testing.expect(std.mem.eql(u8, str, "42"));
}
