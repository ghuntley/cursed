const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast_simple.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;

// Forward declarations for struct and interface support
pub const StructInstance = struct {
    type_name: []const u8,
    fields: HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, type_name: []const u8) StructInstance {
        return StructInstance{
            .type_name = allocator.dupe(u8, type_name) catch @panic("Out of memory"),
            .fields = HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *StructInstance) void {
        self.allocator.free(self.type_name);
        self.fields.deinit();
    }
    
    pub fn setField(self: *StructInstance, name: []const u8, value: Value) !void {
        const field_name = try self.allocator.dupe(u8, name);
        try self.fields.put(field_name, value);
    }
    
    pub fn getField(self: *StructInstance, name: []const u8) ?Value {
        return self.fields.get(name);
    }
};

pub const InterfaceInstance = struct {
    underlying_struct: *StructInstance,
    vtable: *VTable,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, underlying: *StructInstance, vtable: *VTable) InterfaceInstance {
        return InterfaceInstance{
            .underlying_struct = underlying,
            .vtable = vtable,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *InterfaceInstance) void {
        _ = self;
        // VTable is managed separately, underlying struct is managed by caller
    }
};

pub const VTable = struct {
    interface_name: []const u8,
    methods: HashMap([]const u8, *FunctionValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, interface_name: []const u8) VTable {
        return VTable{
            .interface_name = allocator.dupe(u8, interface_name) catch @panic("Out of memory"),
            .methods = HashMap([]const u8, *FunctionValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *VTable) void {
        self.allocator.free(self.interface_name);
        self.methods.deinit();
    }
    
    pub fn setMethod(self: *VTable, name: []const u8, func: *FunctionValue) !void {
        const method_name = try self.allocator.dupe(u8, name);
        try self.methods.put(method_name, func);
    }
    
    pub fn getMethod(self: *VTable, name: []const u8) ?*FunctionValue {
        return self.methods.get(name);
    }
};

pub const FunctionValue = struct {
    name: []const u8,
    parameters: [][]const u8,
    body: []ast.Statement,
    closure_env: ?*Environment,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, name: []const u8, parameters: [][]const u8, body: []ast.Statement, env: ?*Environment) FunctionValue {
        return FunctionValue{
            .name = allocator.dupe(u8, name) catch @panic("Out of memory"),
            .parameters = parameters,
            .body = body,
            .closure_env = env,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *FunctionValue) void {
        self.allocator.free(self.name);
    }
};

pub const InterpreterError = error{
    UndefinedVariable,
    UndefinedFunction,
    TypeMismatch,
    DivisionByZero,
    RuntimeError,
    OutOfMemory,
    UndefinedStruct,
    UndefinedInterface,
    UndefinedField,
    UndefinedMethod,
    InvalidStructField,
    InterfaceNotImplemented,
};

pub const ErrorValue = struct {
    message: []const u8,
    code: i64,
    context: ?[]const u8,
    stack_trace: ?[][]const u8,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, message: []const u8, code: i64) ErrorValue {
        return ErrorValue{
            .message = allocator.dupe(u8, message) catch @panic("Out of memory"),
            .code = code,
            .context = null,
            .stack_trace = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ErrorValue) void {
        self.allocator.free(self.message);
        if (self.context) |ctx| {
            self.allocator.free(ctx);
        }
        if (self.stack_trace) |trace| {
            for (trace) |frame| {
                self.allocator.free(frame);
            }
            self.allocator.free(trace);
        }
    }
};

pub const Value = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,
    Struct: StructInstance,
    Interface: InterfaceInstance,
    Error: ErrorValue,

    pub fn toString(self: Value, allocator: Allocator) ![]u8 {
        switch (self) {
            .Integer => |int| return std.fmt.allocPrint(allocator, "{}", .{int}),
            .Float => |float| return std.fmt.allocPrint(allocator, "{d}", .{float}),
            .String => |str| return allocator.dupe(u8, str),
            .Boolean => |bool_val| return allocator.dupe(u8, if (bool_val) "based" else "cap"),
            .Character => |char| return std.fmt.allocPrint(allocator, "{c}", .{char}),
            .Null => return allocator.dupe(u8, "cap"),
            .Struct => |struct_inst| return std.fmt.allocPrint(allocator, "struct {s}", .{struct_inst.type_name}),
            .Interface => |interface_inst| return std.fmt.allocPrint(allocator, "interface {s}", .{interface_inst.vtable.interface_name}),
            .Error => |err| return std.fmt.allocPrint(allocator, "Error({s})", .{err.message}),
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
            .Struct => return true,   // Structs are always truthy if they exist
            .Interface => return true, // Interfaces are always truthy if they exist
            .Error => return false,   // Errors are falsy
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
            .Integer => |int| return @as(f64, @floatFromInt(int)),
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

pub const TypeRegistry = struct {
    struct_types: HashMap([]const u8, ast.StructStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    interface_types: HashMap([]const u8, ast.InterfaceStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    vtables: HashMap([]const u8, VTable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) TypeRegistry {
        return TypeRegistry{
            .struct_types = HashMap([]const u8, ast.StructStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .interface_types = HashMap([]const u8, ast.InterfaceStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .vtables = HashMap([]const u8, VTable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *TypeRegistry) void {
        self.struct_types.deinit();
        self.interface_types.deinit();
        self.vtables.deinit();
    }
    
    pub fn registerStruct(self: *TypeRegistry, name: []const u8, struct_decl: ast.StructStatement) !void {
        const struct_name = try self.allocator.dupe(u8, name);
        try self.struct_types.put(struct_name, struct_decl);
    }
    
    pub fn registerInterface(self: *TypeRegistry, name: []const u8, interface_decl: ast.InterfaceStatement) !void {
        const interface_name = try self.allocator.dupe(u8, name);
        try self.interface_types.put(interface_name, interface_decl);
    }
    
    pub fn getStruct(self: *TypeRegistry, name: []const u8) ?ast.StructStatement {
        return self.struct_types.get(name);
    }
    
    pub fn getInterface(self: *TypeRegistry, name: []const u8) ?ast.InterfaceStatement {
        return self.interface_types.get(name);
    }
};

pub const Interpreter = struct {
    globals: Environment,
    environment: *Environment,
    functions: HashMap([]const u8, CursedFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    type_registry: TypeRegistry,
    allocator: Allocator,

    pub fn init(allocator: Allocator) Interpreter {
        var globals = Environment.init(allocator, null);
        
        return Interpreter{
            .globals = globals,
            .environment = &globals,
            .functions = HashMap([]const u8, CursedFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .type_registry = TypeRegistry.init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Interpreter) void {
        self.globals.deinit();
        self.functions.deinit();
        self.type_registry.deinit();
    }

    pub fn execute(self: *Interpreter, program: Program) InterpreterError!void {
        // First pass: collect type and function declarations
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .Function => |func| {
                    const cursed_func = CursedFunction{
                        .declaration = func,
                        .closure = self.environment,
                    };
                    try self.functions.put(func.name, cursed_func);
                },
                .Struct => |struct_decl| {
                    try self.type_registry.registerStruct(struct_decl.name, struct_decl);
                },
                .Interface => |interface_decl| {
                    try self.type_registry.registerInterface(interface_decl.name, interface_decl);
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
            .Yikes => |yikes| try self.executeYikesStatement(yikes),
            .Fam => |fam| try self.executeFamStatement(fam),
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
            .StructLiteral => |struct_lit| return try self.evaluateStructLiteral(struct_lit),
            .Shook => |shook| return try self.evaluateShook(shook),
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
        const object = try self.evaluateExpression(member.object.*);
        
        switch (object) {
            .Struct => |struct_inst| {
                if (struct_inst.getField(member.property)) |field_value| {
                    return field_value;
                } else {
                    return InterpreterError.UndefinedField;
                }
            },
            .Interface => |interface_inst| {
                // Try to access field from underlying struct
                if (interface_inst.underlying_struct.getField(member.property)) |field_value| {
                    return field_value;
                }
                // Or call interface method
                if (interface_inst.vtable.getMethod(member.property)) |method| {
                    return Value{ .String = method.name }; // Return method name for now
                }
                return InterpreterError.UndefinedField;
            },
            else => return InterpreterError.TypeMismatch,
        }
    }
    
    fn evaluateStructLiteral(self: *Interpreter, struct_lit: ast.StructLiteralExpression) InterpreterError!Value {
        // Check if struct type exists
        if (self.type_registry.getStruct(struct_lit.struct_name) == null) {
            return InterpreterError.UndefinedStruct;
        }
        
        // Create new struct instance
        var struct_instance = StructInstance.init(self.allocator, struct_lit.struct_name);
        
        // Initialize fields from literal
        for (struct_lit.fields.items) |field_assignment| {
            const field_value = try self.evaluateExpression(field_assignment.value);
            try struct_instance.setField(field_assignment.field_name, field_value);
        }
        
        return Value{ .Struct = struct_instance };
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
                    .Float => |right_float| return @as(f64, @as(f64, @floatFromInt(left_int))) == right_float,
                    else => return false,
                }
            },
            .Float => |left_float| {
                switch (right) {
                    .Float => |right_float| return left_float == right_float,
                    .Integer => |right_int| return left_float == @as(f64, @as(f64, @floatFromInt(right_int))),
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

    // CURSED Error Handling System Interpreter Implementation
    
    fn executeYikesStatement(self: *Interpreter, yikes: ast.YikesStatement) InterpreterError!void {
        // Create error value with initial value if provided
        const error_value = if (yikes.value) |value_expr| blk: {
            const initial_value = try self.evaluateExpression(value_expr);
            break :blk switch (initial_value) {
                .String => |msg| ErrorValue.init(self.allocator, msg, 0),
                .Integer => |code| ErrorValue.init(self.allocator, "Custom error", code),
                else => ErrorValue.init(self.allocator, "Unknown error", -1),
            };
        } else ErrorValue.init(self.allocator, "Default error", -1);
        
        // Register the error type in environment
        try self.environment.define(yikes.name, Value{ .Error = error_value });
    }

    fn executeFamStatement(self: *Interpreter, fam: ast.FamStatement) InterpreterError!void {
        // Implement panic recovery using Zig's error handling
        var error_occurred: ?ErrorValue = null;
        
        // Execute main body with error catching
        for (fam.body.items) |stmt| {
            // Execute statement and catch any errors
            self.executeStatement(stmt) catch |err| {
                // Convert interpreter error to CURSED error
                error_occurred = ErrorValue.init(
                    self.allocator,
                    @errorName(err),
                    @intFromError(err)
                );
                break;
            };
        }
        
        // If error occurred and recovery body exists, execute it
        if (error_occurred != null and fam.recovery_body != null) {
            const recovery = fam.recovery_body.?;
            
            // Bind error variable if specified
            if (fam.error_variable) |error_var| {
                try self.environment.define(error_var, Value{ .Error = error_occurred.? });
            }
            
            // Execute recovery code
            for (recovery.items) |stmt| {
                try self.executeStatement(stmt);
            }
        } else if (error_occurred != null) {
            // No recovery block, propagate the error
            std.debug.print("Unhandled error in fam block: {s}\n", .{error_occurred.?.message});
        }
    }

    fn evaluateShook(self: *Interpreter, shook: ast.ShookExpression) InterpreterError!Value {
        // Evaluate the wrapped expression
        const result = self.evaluateExpression(shook.expression.*) catch |err| {
            // Convert caught error to CURSED error value
            const error_value = ErrorValue.init(
                self.allocator,
                @errorName(err),
                @intFromError(err)
            );
            return Value{ .Error = error_value };
        };
        
        // Check if result is already an error
        switch (result) {
            .Error => {
                // Propagate error up the call stack
                // In full implementation, would use proper error propagation mechanism
                std.debug.print("Error propagated by shook: {s}\n", .{result.Error.message});
                return result; // Return the error
            },
            else => {
                // Normal value, return as-is
                return result;
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
