const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const error_handling = @import("error_handling.zig");
const concurrency = @import("concurrency.zig");
const gc = @import("gc.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const CursedError = error_handling.CursedError;
const ErrorContext = error_handling.ErrorContext;
const safeDupeString = error_handling.safeDupeString;

// Forward declarations for struct and interface support
pub const StructInstance = struct {
    type_name: []const u8,
    fields: HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, type_name: []const u8) CursedError!StructInstance {
        const type_name_copy = safeDupeString(allocator, type_name) catch |err| {
            return err;
        };
        
        return StructInstance{
            .type_name = type_name_copy,
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
    
    pub fn init(allocator: Allocator, interface_name: []const u8) CursedError!VTable {
        const interface_name_copy = safeDupeString(allocator, interface_name) catch |err| {
            return err;
        };
        
        return VTable{
            .interface_name = interface_name_copy,
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
    
    pub fn init(allocator: Allocator, name: []const u8, parameters: [][]const u8, body: []ast.Statement, env: ?*Environment) CursedError!FunctionValue {
        const name_copy = safeDupeString(allocator, name) catch |err| {
            return err;
        };
        
        return FunctionValue{
            .name = name_copy,
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

// Use the comprehensive error system instead of custom errors
pub const InterpreterError = CursedError;

pub const ErrorValue = struct {
    message: []const u8,
    code: i64,
    context: ?[]const u8,
    stack_trace: ?[][]const u8,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, message: []const u8, code: i64) CursedError!ErrorValue {
        const message_copy = safeDupeString(allocator, message) catch |err| {
            return err;
        };
        
        return ErrorValue{
            .message = message_copy,
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

// Defer statement entry for LIFO execution
pub const DeferEntry = struct {
    statement: Statement,
    environment: *Environment,
};

pub const Interpreter = struct {
    globals: Environment,
    environment: *Environment,
    functions: HashMap([]const u8, CursedFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    type_registry: TypeRegistry,
    channel_storage: HashMap(u64, ArrayList(Value), std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage),
    next_goroutine_id: u64,
    defer_stack: ArrayList(DeferEntry),  // LIFO defer execution stack
    allocator: Allocator,

    pub fn init(allocator: Allocator) Interpreter {
        var globals = Environment.init(allocator, null);
        
        return Interpreter{
            .globals = globals,
            .environment = &globals,
            .functions = HashMap([]const u8, CursedFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .type_registry = TypeRegistry.init(allocator),
            .channel_storage = HashMap(u64, ArrayList(Value), std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .next_goroutine_id = 0,
            .defer_stack = ArrayList(DeferEntry).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Interpreter) void {
        // Execute any remaining deferred statements before cleanup
        self.executeAllDefers();
        
        self.globals.deinit();
        self.functions.deinit();
        self.type_registry.deinit();
        self.defer_stack.deinit();
        
        // Clean up channel storage
        var channel_iterator = self.channel_storage.iterator();
        while (channel_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.channel_storage.deinit();
    }

    pub fn execute(self: *Interpreter, program: Program) InterpreterError!void {
        // First pass: collect type and function declarations
        for (program.statements.items) |stmt| {
            switch (stmt.kind) {
                .Function => {
                    if (stmt.data) |data| {
                        const func = @as(*ast.FunctionStatement, @ptrCast(@alignCast(data)));
                        const cursed_func = CursedFunction{
                            .declaration = func.*,
                            .closure = self.environment,
                        };
                        try self.functions.put(func.name, cursed_func);
                    }
                },
                .Struct => {
                    if (stmt.data) |data| {
                        const struct_decl = @as(*ast.StructStatement, @ptrCast(@alignCast(data)));
                        try self.type_registry.registerStruct(struct_decl.name, struct_decl.*);
                    }
                },
                .Interface => {
                    if (stmt.data) |data| {
                        const interface_decl = @as(*ast.InterfaceStatement, @ptrCast(@alignCast(data)));
                        try self.type_registry.registerInterface(interface_decl.name, interface_decl.*);
                    }
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
            .Stan => |stan| try self.executeStanStatement(stan),
            .Yikes => |yikes| try self.executeYikesStatement(yikes),
            .Fam => |fam| try self.executeFamStatement(fam),
            .Defer => |defer_stmt| try self.executeDeferStatement(defer_stmt),
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
            .Yikes => |yikes| return try self.evaluateYikes(yikes),
            .Shook => |shook| return try self.evaluateShook(shook),
            .Fam => |fam| return try self.evaluateFam(fam),
            .StringInterpolation => |interpolation| return try self.evaluateStringInterpolation(interpolation),
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
                    // vibez.spill - print function (can handle multiple arguments)
                    if (call.arguments.items.len == 0) {
                        std.debug.print("\n");
                        return Value.Null;
                    }
                    
                    // Print all arguments separated by spaces
                    for (call.arguments.items, 0..) |arg_expr, i| {
                        const arg = try self.evaluateExpression(arg_expr);
                        const str = try arg.toString(self.allocator);
                        defer self.allocator.free(str);
                        
                        if (i > 0) {
                            std.debug.print(" ");
                        }
                        std.debug.print("{s}", .{str});
                    }
                    std.debug.print("\n");
                    return Value.Null;
                }
            },
            .Identifier => |name| {
                // Handle concurrency built-in functions
                if (std.mem.eql(u8, name, "dm_create")) {
                    // dm_create(element_size, capacity) -> channel pointer
                    if (call.arguments.items.len != 2) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const element_size = try self.evaluateExpression(call.arguments.items[0]);
                    const capacity = try self.evaluateExpression(call.arguments.items[1]);
                    
                    const element_size_num = try element_size.toNumber();
                    const capacity_num = try capacity.toNumber();
                    
                    // Create a simple channel representation
                    const channel_id = @as(u64, @intFromFloat(element_size_num * 1000 + capacity_num));
                    return Value{ .Number = @floatFromInt(channel_id) };
                } else if (std.mem.eql(u8, name, "dm_send")) {
                    // dm_send(channel, value) -> result code
                    if (call.arguments.items.len != 2) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const channel = try self.evaluateExpression(call.arguments.items[0]);
                    const value = try self.evaluateExpression(call.arguments.items[1]);
                    
                    // Store the value in channel simulation (enhanced for concurrency)
                    const channel_id = @as(u64, @intFromFloat(try channel.toNumber()));
                    try self.storeChannelValue(channel_id, value);
                    return Value{ .Number = 0 }; // Success
                } else if (std.mem.eql(u8, name, "dm_recv")) {
                    // dm_recv(channel) -> value
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const channel = try self.evaluateExpression(call.arguments.items[0]);
                    
                    // Retrieve the value from channel simulation
                    const channel_id = @as(u64, @intFromFloat(try channel.toNumber()));
                    return self.retrieveChannelValue(channel_id) catch Value{ .Number = 0 };
                } else if (std.mem.eql(u8, name, "dm_close")) {
                    // dm_close(channel) -> void
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const channel = try self.evaluateExpression(call.arguments.items[0]);
                    _ = channel;
                    return Value.Null;
                } else if (std.mem.eql(u8, name, "dm_is_closed")) {
                    // dm_is_closed(channel) -> bool
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const channel = try self.evaluateExpression(call.arguments.items[0]);
                    _ = channel;
                    return Value{ .Boolean = true }; // Simulate closed
                } else if (std.mem.eql(u8, name, "stan")) {
                    // stan(function) -> goroutine_id
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const func_expr = try self.evaluateExpression(call.arguments.items[0]);
                    _ = func_expr;
                    
                    // Generate unique goroutine ID
                    self.next_goroutine_id += 1;
                    return Value{ .Number = @floatFromInt(self.next_goroutine_id) };
                } else if (self.functions.get(name)) |func| {
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
        var struct_instance = try StructInstance.init(self.allocator, struct_lit.struct_name);
        
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
        
        // Save defer stack size at function entry (for scope-based cleanup)
        const defer_stack_size_at_entry = self.defer_stack.items.len;
        
        // Execute function body
        const previous_env = self.environment;
        self.environment = &function_env;
        defer {
            // Execute defers for this function scope in LIFO order
            self.executeDeferToSize(defer_stack_size_at_entry);
            self.environment = previous_env;
        }
        
        var return_value: Value = Value.Null;
        var has_returned = false;
        
        for (func.declaration.body.items) |stmt| {
            switch (stmt) {
                .Return => |ret| {
                    if (ret.value) |value| {
                        return_value = try self.evaluateExpression(value);
                    } else {
                        return_value = Value.Null;
                    }
                    has_returned = true;
                    break; // Exit function body loop
                },
                else => try self.executeStatement(stmt),
            }
        }
        
        // Function completed - defers will be executed by the defer block above
        return return_value;
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
                .String => |msg| try ErrorValue.init(self.allocator, msg, 0),
                .Integer => |code| try ErrorValue.init(self.allocator, "Custom error", code),
                else => try ErrorValue.init(self.allocator, "Unknown error", -1),
            };
        } else try ErrorValue.init(self.allocator, "Default error", -1);
        
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
                ) catch break;
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

    fn evaluateYikes(self: *Interpreter, yikes: ast.YikesExpression) InterpreterError!Value {
        // Evaluate the error message
        const message_value = try self.evaluateExpression(yikes.message.*);
        const message = switch (message_value) {
            .String => |str| str,
            else => "Unknown error message",
        };
        
        // Evaluate the error code (optional)
        const code = if (yikes.code) |code_expr| blk: {
            const code_value = try self.evaluateExpression(code_expr.*);
            break :blk switch (code_value) {
                .Integer => |int| int,
                else => 0,
            };
        } else 0;
        
        // Create error value
        return Value{ .Error = try ErrorValue.init(self.allocator, message, code) };
    }

    fn evaluateFam(self: *Interpreter, fam: ast.FamExpression) InterpreterError!Value {
        var last_result = Value.Null;
        var error_occurred: ?ErrorValue = null;
        
        // Execute try body
        for (fam.try_body.items) |stmt_ptr| {
            const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
            const result = self.executeStatement(stmt.*) catch |err| {
                error_occurred = ErrorValue.init(
                    self.allocator,
                    @errorName(err),
                    @intFromError(err)
                ) catch continue; // If we can't create error, continue execution
                break;
            };
            
            if (result) |val| {
                switch (val) {
                    .Error => |err| {
                        error_occurred = err;
                        break;
                    },
                    else => last_result = val,
                }
            }
        }
        
        // Execute catch handler if error occurred
        if (error_occurred != null and fam.catch_handler != null) {
            const catch_handler = fam.catch_handler.?;
            
            // Set error variable in environment
            try self.environment.define(catch_handler.error_variable, Value{ .Error = error_occurred.? });
            
            // Execute catch body
            for (catch_handler.handler_body.items) |stmt_ptr| {
                const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
                if (self.executeStatement(stmt.*)) |result| {
                    if (result) |val| {
                        last_result = val;
                    }
                } else |_| {
                    // Ignore errors in catch handler
                }
            }
            
            error_occurred = null; // Error was handled
        }
        
        // Execute finally handler
        if (fam.finally_handler) |finally_handler| {
            for (finally_handler.finally_body.items) |stmt_ptr| {
                const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
                _ = self.executeStatement(stmt.*) catch {}; // Ignore errors in finally
            }
        }
        
        // Return error if unhandled, otherwise return last result
        if (error_occurred) |err| {
            return Value{ .Error = err };
        }
        return last_result;
    }

    fn executeStanStatement(self: *Interpreter, stan: ast.StanStatement) InterpreterError!void {
        // Execute goroutine body in a separate context
        // For now, we simulate goroutine execution by running the body immediately
        // In a full implementation, this would spawn an actual goroutine
        
        // Create a new environment for the goroutine
        var goroutine_env = Environment.init(self.allocator, self.environment);
        defer goroutine_env.deinit();
        
        const old_env = self.environment;
        self.environment = &goroutine_env;
        defer self.environment = old_env;
        
        // Execute all statements in the goroutine body
        for (stan.body.items) |stmt| {
            try self.executeStatement(stmt);
        }
    }
    
    /// Execute defer statement by pushing it onto the defer stack
    fn executeDeferStatement(self: *Interpreter, defer_stmt: ast.DeferStatement) InterpreterError!void {
        // Get the deferred statement
        const statement_ptr: *Statement = @ptrCast(@alignCast(defer_stmt.statement));
        const statement = statement_ptr.*;
        
        // Create defer entry with current environment
        const defer_entry = DeferEntry{
            .statement = statement,
            .environment = self.environment,
        };
        
        // Push onto defer stack (LIFO order)
        try self.defer_stack.append(defer_entry);
        
        std.debug.print("✅ Defer statement pushed to stack (size: {d})\n", .{self.defer_stack.items.len});
    }
    
    /// Execute all deferred statements in LIFO order
    fn executeAllDefers(self: *Interpreter) void {
        std.debug.print("Executing {d} deferred statements\n", .{self.defer_stack.items.len});
        
        // Execute in reverse order (LIFO - Last In, First Out)
        while (self.defer_stack.items.len > 0) {
            const defer_entry = self.defer_stack.pop();
            
            // Save current environment and switch to defer environment
            const saved_env = self.environment;
            self.environment = defer_entry.environment;
            
            // Execute the deferred statement
            std.debug.print("Executing deferred statement\n");
            self.executeStatement(defer_entry.statement) catch |err| {
                std.debug.print("Error executing deferred statement: {}\n", .{err});
                // Continue with other defers even if one fails
            };
            
            // Restore environment
            self.environment = saved_env;
        }
    }
    
    /// Execute defers up to a specific stack size (for function scope cleanup)
    fn executeDeferToSize(self: *Interpreter, target_size: usize) void {
        std.debug.print("Executing defers from size {d} to {d}\n", .{ self.defer_stack.items.len, target_size });
        
        while (self.defer_stack.items.len > target_size) {
            const defer_entry = self.defer_stack.pop();
            
            // Save current environment and switch to defer environment
            const saved_env = self.environment;
            self.environment = defer_entry.environment;
            
            // Execute the deferred statement
            std.debug.print("Executing scoped deferred statement\n");
            self.executeStatement(defer_entry.statement) catch |err| {
                std.debug.print("Error executing deferred statement: {}\n", .{err});
                // Continue with other defers even if one fails
            };
            
            // Restore environment
            self.environment = saved_env;
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
            ) catch {
                // If we can't create error value, return generic error
                return Value{ .Error = ErrorValue{
                    .message = "Unknown error",
                    .code = -1,
                    .context = null,
                    .stack_trace = null,
                    .allocator = self.allocator,
                }};
            };
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

    // Enhanced channel simulation methods
    fn storeChannelValue(self: *Interpreter, channel_id: u64, value: Value) InterpreterError!void {
        if (self.channel_storage.getPtr(channel_id)) |channel_list| {
            try channel_list.append(value);
        } else {
            var new_list = ArrayList(Value).init(self.allocator);
            try new_list.append(value);
            try self.channel_storage.put(channel_id, new_list);
        }
    }

    fn retrieveChannelValue(self: *Interpreter, channel_id: u64) InterpreterError!Value {
        if (self.channel_storage.getPtr(channel_id)) |channel_list| {
            if (channel_list.items.len > 0) {
                return channel_list.orderedRemove(0);
            }
        }
        return Value{ .Number = 0 }; // Default value when channel is empty
    }

    // Enhanced concurrency support
    fn executeGoroutine(self: *Interpreter, function_value: Value) InterpreterError!u64 {
        _ = self;
        _ = function_value;
        // In real implementation, this would spawn actual goroutines
        // For now, return a simulated goroutine ID
        return 1;
    }
    
    fn evaluateStringInterpolation(self: *Interpreter, interpolation: ast.StringInterpolationExpression) InterpreterError!Value {
        var result = std.ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        for (interpolation.parts.items) |part| {
            if (part.expression) |expr_ptr| {
                // Evaluate expression and convert to string
                const expr: *Expression = @ptrCast(@alignCast(expr_ptr));
                const value = try self.evaluateExpression(expr.*);
                const str_value = try value.toString(self.allocator);
                defer self.allocator.free(str_value);
                try result.appendSlice(str_value);
            } else {
                // Literal text part
                try result.appendSlice(part.text);
            }
        }
        
        const final_string = try self.allocator.dupe(u8, result.items);
        return Value{ .String = final_string };
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
