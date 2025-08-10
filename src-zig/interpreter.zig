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
    CursedError: *cursed_error.CursedError,
    Tuple: ArrayList(Value),

    pub fn deinit(self: *Value, allocator: Allocator) void {
        switch (self.*) {
            .String => |str| allocator.free(str),
            .Tuple => |*tuple| {
                for (tuple.items) |*item| {
                    item.deinit(allocator);
                }
                tuple.deinit();
            },
            .Error => |*err| err.deinit(),
            .Struct => |*struct_inst| struct_inst.deinit(),
            .Interface => |*interface_inst| interface_inst.deinit(),
            .CursedError => |cursed_err| {
                cursed_err.deinit();
                allocator.destroy(cursed_err);
            },
            else => {}, // Other types don't need cleanup
        }
    }

    pub fn equals(self: Value, other: Value) bool {
        switch (self) {
            .Integer => |a| switch (other) {
                .Integer => |b| return a == b,
                else => return false,
            },
            .Float => |a| switch (other) {
                .Float => |b| return a == b,
                else => return false,
            },
            .String => |a| switch (other) {
                .String => |b| return std.mem.eql(u8, a, b),
                else => return false,
            },
            .Boolean => |a| switch (other) {
                .Boolean => |b| return a == b,
                else => return false,
            },
            .Character => |a| switch (other) {
                .Character => |b| return a == b,
                else => return false,
            },
            .Null => switch (other) {
                .Null => return true,
                else => return false,
            },
            else => return false,
        }
    }

    pub fn toString(self: Value, allocator: Allocator) ![]u8 {
        switch (self) {
            .Integer => |int| return std.fmt.allocPrint(allocator, "{}", .{int}),
            .Float => |float| return std.fmt.allocPrint(allocator, "{d}", .{float}),
            .String => |str| return allocator.dupe(u8, str),
            .Boolean => |bool_val| return allocator.dupe(u8, if (bool_val) "based" else "cap"),
            .Character => |char| return std.fmt.allocPrint(allocator, "{c}", .{char}),
            .Null => return allocator.dupe(u8, "cap"),
            .Tuple => |tuple| {
                var result = std.ArrayList(u8).init(allocator);
                defer result.deinit();
                try result.append('(');
                for (tuple.items, 0..) |item, i| {
                    if (i > 0) try result.appendSlice(", ");
                    const item_str = try item.toString(allocator);
                    defer allocator.free(item_str);
                    try result.appendSlice(item_str);
                }
                try result.append(')');
                return try allocator.dupe(u8, result.items);
            },
            .Struct => |struct_inst| return std.fmt.allocPrint(allocator, "struct {s}", .{struct_inst.type_name}),
            .Interface => |interface_inst| return std.fmt.allocPrint(allocator, "interface {s}", .{interface_inst.vtable.interface_name}),
            .Error => |err| return std.fmt.allocPrint(allocator, "Error({s})", .{err.message}),
            .CursedError => |cursed_err| return try cursed_err.toString(),
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
            .CursedError => return false, // CursedErrors are falsy
            .Tuple => |tuple| return tuple.items.len > 0, // Tuples are truthy if non-empty
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
    
    pub fn registerImplementation(self: *TypeRegistry, struct_name: []const u8, interface_name: []const u8, vtable: *VTable) !void {
        // Create a key that combines struct and interface names
        const key = try std.fmt.allocPrint(self.allocator, "{}::{}", .{struct_name, interface_name});
        try self.vtables.put(key, vtable.*);
    }
    
    pub fn getVTable(self: *TypeRegistry, struct_name: []const u8, interface_name: []const u8) ?VTable {
        const key_temp = std.fmt.allocPrint(self.allocator, "{}::{}", .{struct_name, interface_name}) catch return null;
        defer self.allocator.free(key_temp);
        return self.vtables.get(key_temp);
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
    error_handler: cursed_error.ErrorHandler,
    call_stack_depth: u32,
    max_call_stack_depth: u32,
    allocator: Allocator,

    const MAX_CALL_STACK_DEPTH = 1000;

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
            .error_handler = cursed_error.ErrorHandler.init(allocator, "main.csd"),
            .call_stack_depth = 0,
            .max_call_stack_depth = MAX_CALL_STACK_DEPTH,
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
        self.error_handler.deinit();
        
        // Clean up channel storage
        var channel_iterator = self.channel_storage.iterator();
        while (channel_iterator.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.channel_storage.deinit();
    }

    pub fn execute(self: *Interpreter, program: Program) InterpreterError!void {
        // First pass: collect type and function declarations
        for (program.statements.items) |stmt_ptr| {
            const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
            switch (stmt.*) {
                .Function => |func| {
                    const cursed_func = CursedFunction{
                        .declaration = func,
                        .closure = self.environment,
                    };
                    std.debug.print("DEBUG: Registering function '{s}'\n", .{func.name});
                    try self.functions.put(func.name, cursed_func);
                },
                .Struct => |struct_decl| {
                    std.debug.print("DEBUG: Registering struct '{s}' with {d} methods\n", .{struct_decl.name, struct_decl.methods.items.len});
                    try self.type_registry.registerStruct(struct_decl.name, struct_decl);
                },
                .Interface => |interface_decl| {
                    try self.type_registry.registerInterface(interface_decl.name, interface_decl);
                },
                .Implementation => |impl_decl| {
                    // Register implementations during the first pass
                    try self.executeImplementationStatement(impl_decl);
                },
                else => {},
            }
        }
        
        // Execute main_character function if it exists
        if (self.functions.get("main_character")) |main_func| {
            _ = try self.callFunction(main_func, &[_]Value{});
        } else {
            // Execute statements in order
            for (program.statements.items) |stmt_ptr| {
                const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
                try self.executeStatement(stmt.*);
            }
        }
    }

    fn executeStatement(self: *Interpreter, stmt: Statement) InterpreterError!void {
        std.debug.print("DEBUG: Executing statement type: {}\n", .{@tagName(stmt)});
        switch (stmt) {
            .Expression => |expr_ptr| {
                const expr: *Expression = @ptrCast(@alignCast(expr_ptr));
                _ = try self.evaluateExpression(expr.*);
            },
            .Let => |let| try self.executeLetStatement(let),
            .Assignment => |assign| try self.executeAssignmentStatement(assign),
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
            .Struct => |struct_stmt| try self.executeStructStatement(struct_stmt),
            .Interface => |interface_stmt| try self.executeInterfaceStatement(interface_stmt),
            .Implementation => |impl_stmt| try self.executeImplementationStatement(impl_stmt),
            .Stan => |stan| try self.executeStanStatement(stan),
            .Yikes => |yikes| try self.executeYikesStatement(yikes),
            .Fam => |fam| try self.executeFamStatement(fam),
            .Defer => |defer_stmt| try self.executeDeferStatement(defer_stmt),
            .Switch => |switch_stmt| try self.executeSwitchStatement(switch_stmt),
            .PatternSwitch => |pattern_switch| try self.executePatternSwitchStatement(pattern_switch),
            else => {
                std.debug.print("Unsupported statement type in interpreter: {}\n", .{@tagName(stmt)});
            },
        }
    }

    fn executeLetStatement(self: *Interpreter, let: ast.LetStatement) InterpreterError!void {
        const value = if (let.initializer) |initializer_expr|
            try self.evaluateExpression(initializer_expr.*)
        else
            Value.Null;
        
        // Check if we're trying to destructure a tuple by looking for comma in name
        // This is a workaround until parser supports multiple names
        if (std.mem.indexOf(u8, let.name, ",")) |_| {
            // Handle tuple destructuring: "result, err"
            var name_iter = std.mem.splitSequence(u8, let.name, ",");
            var name_index: usize = 0;
            
            switch (value) {
                .Tuple => |tuple| {
                    while (name_iter.next()) |raw_name| {
                        const trimmed_name = std.mem.trim(u8, raw_name, " \t");
                        if (name_index < tuple.items.len) {
                            try self.environment.define(trimmed_name, tuple.items[name_index]);
                        } else {
                            try self.environment.define(trimmed_name, Value.Null);
                        }
                        name_index += 1;
                    }
                },
                else => {
                    // Single value assigned to first variable, rest get null
                    var first = true;
                    while (name_iter.next()) |raw_name| {
                        const trimmed_name = std.mem.trim(u8, raw_name, " \t");
                        if (first) {
                            try self.environment.define(trimmed_name, value);
                            first = false;
                        } else {
                            try self.environment.define(trimmed_name, Value.Null);
                        }
                    }
                }
            }
        } else {
            // Regular single variable assignment
            try self.environment.define(let.name, value);
        }
    }
    
    fn executeAssignmentStatement(self: *Interpreter, assign: ast.AssignmentStatement) InterpreterError!void {
        const target_expr: *Expression = @ptrCast(@alignCast(assign.target));
        const value_expr: *Expression = @ptrCast(@alignCast(assign.value));
        
        const value = try self.evaluateExpression(value_expr.*);
        
        switch (target_expr.*) {
            .Identifier => |name| {
                // Simple variable assignment
                try self.environment.set(name, value);
            },
            .MemberAccess => |member| {
                // Struct field assignment
                try self.assignToMemberAccess(member, value);
            },
            else => {
                std.debug.print("Unsupported assignment target: {s}\n", .{@tagName(target_expr.*)});
                return InterpreterError.TypeMismatch;
            }
        }
    }
    
    fn assignToMemberAccess(self: *Interpreter, member: *ast.MemberAccessExpression, value: Value) InterpreterError!void {
        // For struct field assignment, we need to get the variable name and update it in the environment
        switch (member.object.*) {
            .Identifier => |obj_name| {
                var object_value = try self.environment.get(obj_name);
                switch (object_value) {
                    .Struct => |*struct_inst| {
                        try struct_inst.setField(member.property, value);
                        // Update the struct instance in the environment
                        try self.environment.set(obj_name, object_value);
                    },
                    else => {
                        std.debug.print("Cannot assign to field of non-struct type: {s}\n", .{@tagName(object_value)});
                        return InterpreterError.TypeMismatch;
                    }
                }
            },
            else => {
                std.debug.print("Complex member access assignment not yet supported\n", .{});
                return InterpreterError.TypeMismatch;
            }
        }
    }
    
    pub fn executeStructStatement(self: *Interpreter, struct_stmt: ast.StructStatement) InterpreterError!void {
        // Register the struct type in the type registry
        try self.type_registry.registerStruct(struct_stmt.name, struct_stmt);
    }

    fn executeInterfaceStatement(self: *Interpreter, interface_stmt: ast.InterfaceStatement) InterpreterError!void {
        _ = self; // Interface statements are handled in the type registry during the first pass
        // Nothing to do here for execution - they're already registered in execute()
        std.debug.print("DEBUG: Executing interface statement for '{}'\n", .{interface_stmt.name});
    }

    fn executeImplementationStatement(self: *Interpreter, impl_stmt: ast.ImplementationStatement) InterpreterError!void {
        std.debug.print("DEBUG: Executing implementation statement: {} for {}\n", .{impl_stmt.implementing_type, impl_stmt.interface_name});
        
        // Get the interface definition
        const interface_def = self.type_registry.getInterface(impl_stmt.interface_name) orelse {
            std.debug.print("ERROR: Interface '{}' not found\n", .{impl_stmt.interface_name});
            return InterpreterError.UndefinedInterface;
        };
        
        // Create a vtable for this implementation
        var vtable = try VTable.init(self.allocator, impl_stmt.interface_name);
        
        // Populate vtable with implementation methods
        for (interface_def.methods.items) |interface_method| {
            var method_found = false;
            for (impl_stmt.methods.items) |impl_method| {
                if (std.mem.eql(u8, interface_method.name, impl_method.name)) {
                    // Create function value for the method
                    const func_value = try self.allocator.create(FunctionValue);
                    
                    // Convert from []*ast.Statement to []ast.Statement
                    var statements = try self.allocator.alloc(ast.Statement, impl_method.body.items.len);
                    for (impl_method.body.items, 0..) |stmt_ptr, i| {
                        statements[i] = stmt_ptr.*;
                    }
                    
                    func_value.* = try FunctionValue.init(
                        self.allocator,
                        impl_method.name,
                        &[_][]const u8{}, // Parameters will be handled in executeInterfaceMethod
                        statements,
                        self.environment
                    );
                    
                    try vtable.setMethod(interface_method.name, func_value);
                    method_found = true;
                    std.debug.print("DEBUG: Added method '{}' to vtable\n", .{interface_method.name});
                    break;
                }
            }
            
            if (!method_found) {
                std.debug.print("ERROR: Method '{}' not implemented for interface '{}'\n", .{interface_method.name, impl_stmt.interface_name});
                return InterpreterError.UndefinedMethod;
            }
        }
        
        // Store the vtable in the type registry for this struct-interface pair
        try self.type_registry.registerImplementation(impl_stmt.implementing_type, impl_stmt.interface_name, &vtable);
    }

    fn executeIfStatement(self: *Interpreter, if_stmt: ast.IfStatement) InterpreterError!void {
        const condition_expr: *Expression = @ptrCast(@alignCast(if_stmt.condition));
        const condition = try self.evaluateExpression(condition_expr.*);
        
        if (condition.toBool()) {
            for (if_stmt.then_branch.items) |stmt_ptr| {
                const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
                try self.executeStatement(stmt.*);
            }
        } else if (if_stmt.else_branch) |else_stmts| {
            for (else_stmts.items) |stmt_ptr| {
                const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
                try self.executeStatement(stmt.*);
            }
        }
    }

    fn executeWhileStatement(self: *Interpreter, while_stmt: ast.WhileStatement) InterpreterError!void {
        while (true) {
            const condition = try self.evaluateExpression(while_stmt.condition.*);
            if (!condition.toBool()) break;
            
            for (while_stmt.body.items) |stmt| {
                try self.executeStatement(stmt.*);
            }
        }
    }

    pub fn evaluateExpression(self: *Interpreter, expr: Expression) InterpreterError!Value {
        std.debug.print("DEBUG: Evaluating expression type: {}\n", .{@tagName(expr)});
        switch (expr) {
            .Integer => |int| return Value{ .Integer = int },
            .Float => |float| return Value{ .Float = float },
            .String => |str| return Value{ .String = str },
            .Boolean => |bool_val| return Value{ .Boolean = bool_val },
            .Character => |char| return Value{ .Character = char },
            .Identifier => |name| {
                // Try to find in current environment first
                if (self.environment.get(name)) |value| {
                    return value;
                } else |_| {
                    // If not found, check if this might be a field access on 'self'
                    // Look for 'self' in the current environment (first parameter in methods)
                    if (self.environment.get("self")) |self_value| {
                        switch (self_value) {
                            .Struct => |struct_inst| {
                                if (struct_inst.fields.get(name)) |field_value| {
                                    std.debug.print("DEBUG: Implicit field access for '{s}' resolved to: {s}\n", .{name, @tagName(field_value)});
                                    return field_value;
                                }
                            },
                            else => {}
                        }
                    } else |_| {}
                    
                    // If not found anywhere, return undefined variable error
                    return InterpreterError.UndefinedVariable;
                }
            },
            .Binary => |bin| return try self.evaluateBinary(bin),
            .Call => |call| return try self.evaluateCall(call),
            .MemberAccess => |member| return try self.evaluateMemberAccess(member.*),
            .StructLiteral => |struct_lit| return try self.evaluateStructLiteral(struct_lit),
            .Struct => |struct_expr| return try self.evaluateStructExpression(struct_expr),
            .Yikes => |yikes| return try self.evaluateYikes(yikes),
            .Shook => |shook| return try self.evaluateShook(shook),
            .Fam => |fam| return try self.evaluateFam(fam),
            .StringInterpolation => |interpolation| return try self.evaluateStringInterpolation(interpolation),
            .Match => |match| return try self.evaluateMatch(match),
            .Tuple => |tuple| return try self.evaluateTuple(tuple),
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
                        std.debug.print("{s}", .{"\n"});
                        return Value.Null;
                    }
                    
                    // Print all arguments separated by spaces
                    for (call.arguments.items, 0..) |arg_expr, i| {
                        const arg = try self.evaluateExpression(arg_expr.*);
                        const str = try arg.toString(self.allocator);
                        defer self.allocator.free(str);
                        
                        if (i > 0) {
                            std.debug.print("{s}", .{" "});
                        }
                        std.debug.print("{s}", .{str});
                    }
                    std.debug.print("{s}", .{"\n"});
                    return Value.Null;
                } else {
                    // Handle method calls on objects (structs/interfaces)
                    std.debug.print("DEBUG: Detected method call: {}.{}\n", .{@tagName(member.object.*), member.property});
                    return try self.evaluateMethodCall(member.*, call.arguments.items);
                }
            },
            .Identifier => |name| {
                // Handle facts() function - print function with multiple arguments
                if (std.mem.eql(u8, name, "facts")) {
                    // Print all arguments separated by spaces, similar to print() in other languages
                    for (call.arguments.items, 0..) |arg_expr, i| {
                        if (i > 0) std.debug.print(" ", .{});
                        
                        const arg = try self.evaluateExpression(arg_expr.*);
                        const str = try arg.toString(self.allocator);
                        defer self.allocator.free(str);
                        std.debug.print("{s}", .{str});
                    }
                    std.debug.print("\n", .{});
                    return Value.Null;
                }
                // Handle concurrency built-in functions
                else if (std.mem.eql(u8, name, "dm_create")) {
                    // dm_create(element_size, capacity) -> channel pointer
                    if (call.arguments.items.len != 2) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const element_size = try self.evaluateExpression(call.arguments.items[0].*);
                    const capacity = try self.evaluateExpression(call.arguments.items[1].*);
                    
                    const element_size_num = try element_size.toNumber();
                    const capacity_num = try capacity.toNumber();
                    
                    // Create a simple channel representation
                    const channel_id = @as(u64, @intFromFloat(element_size_num * 1000 + capacity_num));
                    return Value{ .Float = @floatFromInt(channel_id) };
                } else if (std.mem.eql(u8, name, "dm_send")) {
                    // dm_send(channel, value) -> result code
                    if (call.arguments.items.len != 2) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const channel = try self.evaluateExpression(call.arguments.items[0].*);
                    const value = try self.evaluateExpression(call.arguments.items[1].*);
                    
                    // Store the value in channel simulation (enhanced for concurrency)
                    const channel_id = @as(u64, @intFromFloat(try channel.toNumber()));
                    try self.storeChannelValue(channel_id, value);
                    return Value{ .Integer = 0 }; // Success
                } else if (std.mem.eql(u8, name, "dm_recv")) {
                    // dm_recv(channel) -> value
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const channel = try self.evaluateExpression(call.arguments.items[0].*);
                    
                    // Retrieve the value from channel simulation
                    const channel_id = @as(u64, @intFromFloat(try channel.toNumber()));
                    return self.retrieveChannelValue(channel_id) catch Value{ .Integer = 0 };
                } else if (std.mem.eql(u8, name, "dm_close")) {
                    // dm_close(channel) -> void
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const channel = try self.evaluateExpression(call.arguments.items[0].*);
                    _ = channel;
                    return Value.Null;
                } else if (std.mem.eql(u8, name, "dm_is_closed")) {
                    // dm_is_closed(channel) -> bool
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const channel = try self.evaluateExpression(call.arguments.items[0].*);
                    _ = channel;
                    return Value{ .Boolean = true }; // Simulate closed
                } else if (std.mem.eql(u8, name, "stan")) {
                    // stan(function) -> goroutine_id
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const func_expr = try self.evaluateExpression(call.arguments.items[0].*);
                    _ = func_expr;
                    
                    // Generate unique goroutine ID
                    self.next_goroutine_id += 1;
                    return Value{ .Integer = @intCast(self.next_goroutine_id) };
                } else {
                    // Try to find function directly first
                    if (self.functions.get(name)) |func| {
                        std.debug.print("DEBUG: Calling user function '{s}'\n", .{name});
                        // Evaluate arguments
                        var args = ArrayList(Value).init(self.allocator);
                        defer args.deinit();
                        
                        for (call.arguments.items) |arg_expr| {
                            const arg = try self.evaluateExpression(arg_expr.*);
                            try args.append(arg);
                        }
                        
                        return try self.callFunction(func, args.items);
                    }
                    
                    // Enhanced generic function call resolution
                    if (try self.resolveGenericFunctionCall(name, call.arguments.items)) |result| {
                        return result;
                    }
                    
                    std.debug.print("DEBUG: Function '{s}' not found\n", .{name});
                    return InterpreterError.UndefinedFunction;
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
                if (struct_inst.fields.get(member.property)) |field_value| {
                    std.debug.print("DEBUG: Found field '{s}' with value type: {s}\n", .{member.property, @tagName(field_value)});
                    return field_value;
                } else {
                    std.debug.print("DEBUG: Field '{s}' not found in struct\n", .{member.property});
                    return InterpreterError.UndefinedField;
                }
            },
            .Interface => |interface_inst| {
                // Try to access field from underlying struct
                if (interface_inst.underlying_struct.getField(member.property)) |field_value| {
                    return field_value;
                }
                // Interface methods should only be accessed through method calls, not direct access
                if (interface_inst.vtable.getMethod(member.property)) |_| {
                    return InterpreterError.MethodMustBeCalled;
                }
                return InterpreterError.UndefinedField;
            },
            else => {
                std.debug.print("DEBUG: Member access on non-struct type: {s}\n", .{@tagName(object)});
                return InterpreterError.TypeMismatch;
            }
        }
    }

    fn evaluateMethodCall(self: *Interpreter, member: ast.MemberAccessExpression, args: []*ast.Expression) InterpreterError!Value {
        std.debug.print("DEBUG: Method call - evaluating object for '{s}' method\n", .{member.property});
        const object = try self.evaluateExpression(member.object.*);
        std.debug.print("DEBUG: Object evaluated to type: {s}\n", .{@tagName(object)});
        
        switch (object) {
            .Struct => |struct_inst| {
                // Look for method in struct type definition
                if (self.type_registry.getStruct(struct_inst.type_name)) |struct_decl| {
                    // Check if struct has methods defined
                    for (struct_decl.methods.items) |method| {
                        if (std.mem.eql(u8, method.name, member.property)) {
                            // Call the struct method
                            var method_args = try std.ArrayList(Value).initCapacity(self.allocator, args.len + 1);
                            defer method_args.deinit();
                            
                            // First argument is self (the struct instance)
                            try method_args.append(Value{ .Struct = struct_inst });
                            
                            // Add other arguments
                            for (args) |arg_expr| {
                                const arg_val = try self.evaluateExpression(arg_expr.*);
                                try method_args.append(arg_val);
                            }
                            
                            // Execute method body
                            return try self.executeMethodBody(method, method_args.items);
                        }
                    }
                }
                return InterpreterError.UndefinedMethod;
            },
            .Interface => |interface_inst| {
                // Use interface dispatch through vtable
                var method_args = try std.ArrayList(Value).initCapacity(self.allocator, args.len + 1);
                defer method_args.deinit();
                
                // First argument is self (the underlying struct)
                try method_args.append(Value{ .Struct = interface_inst.underlying_struct.* });
                
                for (args) |arg_expr| {
                    const arg_val = try self.evaluateExpression(arg_expr.*);
                    try method_args.append(arg_val);
                }
                
                // Get method from vtable and execute it
                if (interface_inst.vtable.getMethod(member.property)) |method_func| {
                    std.debug.print("DEBUG: Found interface method '{}' in vtable, executing...\n", .{member.property});
                    return try self.executeInterfaceMethod(method_func, method_args.items);
                } else {
                    std.debug.print("DEBUG: Method '{}' not found in interface vtable\n", .{member.property});
                    return InterpreterError.UndefinedMethod;
                }
            },
            else => {
                return InterpreterError.TypeMismatch;
            }
        }
    }

    fn executeInterfaceMethod(self: *Interpreter, method_func: *FunctionValue, args: []Value) InterpreterError!Value {
        // Create new environment for method execution
        var method_env = Environment.init(self.allocator, self.environment);
        defer method_env.deinit();
        
        // Bind parameters to arguments
        if (args.len != method_func.parameters.len + 1) { // +1 for 'self'
            std.debug.print("ERROR: Interface method '{}' expects {} params but got {} args\n", .{method_func.name, method_func.parameters.len, args.len - 1});
            return InterpreterError.InvalidArgumentCount;
        }
        
        // First argument is always 'self' (the struct instance)
        if (args.len > 0) {
            try method_env.define("self", args[0]);
        }
        
        // Bind remaining parameters
        for (method_func.parameters, 1..) |param_name, i| {
            if (i < args.len) {
                try method_env.define(param_name, args[i]);
            }
        }
        
        // Execute method body
        const previous_env = self.environment;
        self.environment = &method_env;
        defer self.environment = previous_env;
        
        for (method_func.body) |stmt| {
            self.executeStatement(stmt) catch |err| {
                return err;
            };
        }
        
        return Value.Null;
    }

    fn executeMethodBody(self: *Interpreter, method: ast.FunctionStatement, args: []Value) InterpreterError!Value {
        // Create new environment for method execution
        var method_env = Environment.init(self.allocator, self.environment);
        defer method_env.deinit();
        
        // First argument is always 'self' (the struct instance)
        if (args.len > 0) {
            try method_env.define("self", args[0]);
        }
        
        // Bind other parameters to remaining arguments (skipping self)
        for (method.parameters.items, 0..) |param, i| {
            const arg_index = i + 1; // Skip self argument
            if (arg_index < args.len) {
                try method_env.define(param.name, args[arg_index]);
            }
        }
        
        // Save current environment
        const previous_env = self.environment;
        self.environment = &method_env;
        defer self.environment = previous_env;
        
        // Execute method body
        var return_value: ?Value = null;
        for (method.body.items) |stmt| {
            if (self.checkForReturn(stmt.*)) |ret_val| {
                return_value = ret_val;
                break;
            }
            try self.executeStatement(stmt.*);
        }
        
        return return_value orelse Value.Null;
    }
    
    /// Check if a statement is a return statement and extract its value
    fn checkForReturn(self: *Interpreter, stmt: Statement) ?Value {
        switch (stmt) {
            .Return => |ret| {
                if (ret.value) |value_ptr| {
                    const value_expr: *Expression = @ptrCast(@alignCast(value_ptr));
                    const result = self.evaluateExpression(value_expr.*) catch Value.Null;
                    return result;
                } else {
                    return Value.Null;
                }
            },
            else => return null,
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
            const field_value = try self.evaluateExpression(field_assignment.value.*);
            try struct_instance.setField(field_assignment.field_name, field_value);
        }
        
        return Value{ .Struct = struct_instance };
    }
    
    fn evaluateStructExpression(self: *Interpreter, struct_expr: *ast.StructExpression) InterpreterError!Value {
        // Check if struct type exists
        if (self.type_registry.getStruct(struct_expr.struct_name) == null) {
            return InterpreterError.UndefinedStruct;
        }
        
        // Create new struct instance
        var struct_instance = try StructInstance.init(self.allocator, struct_expr.struct_name);
        
        // Initialize fields from literal
        for (struct_expr.fields.items) |field_initializer| {
            const field_value = try self.evaluateExpression(field_initializer.value.*);
            try struct_instance.setField(field_initializer.field_name, field_value);
        }
        
        return Value{ .Struct = struct_instance };
    }

    /// Enhanced generic function call resolution
    /// Supports both <T> and [T] syntax for generic function calls
    fn resolveGenericFunctionCall(self: *Interpreter, function_name: []const u8, arguments: []ast.Expression) InterpreterError!?Value {
        // Parse generic call syntax - check for angle brackets first
        const generic_call_info = self.parseGenericCallSyntax(function_name) catch return null;
        if (generic_call_info == null) return null;
        
        const call_info = generic_call_info.?;
        defer self.allocator.free(call_info.base_name);
        defer {
            for (call_info.type_args) |arg| {
                self.allocator.free(arg);
            }
            self.allocator.free(call_info.type_args);
        }
        
        std.debug.print("DEBUG: Parsing generic call '{s}' -> base: '{s}', type_args: {any}\n", 
            .{function_name, call_info.base_name, call_info.type_args});
        
        // Find the generic template function
        const template_func = self.findGenericTemplate(call_info.base_name) orelse {
            std.debug.print("DEBUG: No generic template found for '{s}'\n", .{call_info.base_name});
            return null;
        };
        
        std.debug.print("DEBUG: Found generic template function '{s}' with {d} type parameters\n", 
            .{template_func.declaration.name, template_func.declaration.type_parameters.items.len});
        
        // Validate type argument count
        if (call_info.type_args.len != template_func.declaration.type_parameters.items.len) {
            std.debug.print("DEBUG: Type argument count mismatch: expected {d}, got {d}\n", 
                .{template_func.declaration.type_parameters.items.len, call_info.type_args.len});
            return InterpreterError.TypeMismatch;
        }
        
        // Evaluate function arguments
        var args = ArrayList(Value).init(self.allocator);
        defer args.deinit();
        
        for (arguments) |arg_expr| {
            const arg_value = try self.evaluateExpression(arg_expr);
            try args.append(arg_value);
        }
        
        std.debug.print("DEBUG: Calling generic function '{s}' with {d} arguments\n", 
            .{function_name, args.items.len});
        
        // For now, call the template function directly (basic monomorphization)
        // TODO: Implement proper type parameter substitution in function body
        return try self.callFunction(template_func, args.items);
    }
    
    const GenericCallInfo = struct {
        base_name: []const u8,
        type_args: [][]const u8,
    };
    
    /// Parse generic function call syntax
    fn parseGenericCallSyntax(self: *Interpreter, function_name: []const u8) !?GenericCallInfo {
        // Try angle bracket syntax: function<type1, type2>
        if (std.mem.indexOf(u8, function_name, "<")) |start| {
            if (std.mem.lastIndexOf(u8, function_name, ">")) |end| {
                const base_name = try self.allocator.dupe(u8, function_name[0..start]);
                const type_args_str = function_name[start + 1..end];
                const type_args = try self.parseTypeArguments(type_args_str);
                
                return GenericCallInfo{
                    .base_name = base_name,
                    .type_args = type_args,
                };
            }
        }
        
        // Try square bracket syntax: function[type1, type2]
        if (std.mem.indexOf(u8, function_name, "[")) |start| {
            if (std.mem.lastIndexOf(u8, function_name, "]")) |end| {
                const base_name = try self.allocator.dupe(u8, function_name[0..start]);
                const type_args_str = function_name[start + 1..end];
                const type_args = try self.parseTypeArguments(type_args_str);
                
                return GenericCallInfo{
                    .base_name = base_name,
                    .type_args = type_args,
                };
            }
        }
        
        return null;
    }
    
    /// Parse comma-separated type arguments
    fn parseTypeArguments(self: *Interpreter, type_args_str: []const u8) ![][]const u8 {
        var type_args = ArrayList([]const u8).init(self.allocator);
        defer type_args.deinit();
        
        var iterator = std.mem.splitScalar(u8, type_args_str, ',');
        while (iterator.next()) |type_arg| {
            const trimmed = std.mem.trim(u8, type_arg, " \t\n");
            if (trimmed.len > 0) {
                try type_args.append(try self.allocator.dupe(u8, trimmed));
            }
        }
        
        return type_args.toOwnedSlice();
    }
    
    /// Find generic template function by base name
    fn findGenericTemplate(self: *Interpreter, base_name: []const u8) ?CursedFunction {
        // First, try exact match for generic functions
        if (self.functions.get(base_name)) |func| {
            if (func.declaration.type_parameters.items.len > 0) {
                return func;
            }
        }
        
        // Search for functions that match the base name pattern
        var iterator = self.functions.iterator();
        while (iterator.next()) |entry| {
            const func_name = entry.key_ptr.*;
            const func = entry.value_ptr.*;
            
            // Check if this function is a generic template
            if (std.mem.eql(u8, func_name, base_name) and 
                func.declaration.type_parameters.items.len > 0) {
                return func;
            }
        }
        
        return null;
    }

    pub fn callFunction(self: *Interpreter, func: CursedFunction, args: []Value) InterpreterError!Value {
        // Check for stack overflow
        if (self.call_stack_depth >= self.max_call_stack_depth) {
            return InterpreterError.StackOverflow;
        }
        
        // Track recursion depth
        const old_depth = self.call_stack_depth;
        self.call_stack_depth += 1;
        defer self.call_stack_depth = old_depth;
        
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
        
        // Track multiple return values for tuple returns
        var return_values = ArrayList(Value).init(self.allocator);
        defer return_values.deinit();
        
        for (func.declaration.body.items) |stmt| {
            switch (stmt.*) {
                .Return => |ret| {
                    if (ret.value) |value| {
                        const expr: *ast.Expression = @ptrCast(@alignCast(value));
                        const result = try self.evaluateExpression(expr.*);
                        // Check if this is a tuple expression (multiple values)
                        switch (result) {
                            .Tuple => |_| {
                                // Return multiple values as tuple
                                return_value = result;
                            },
                            else => {
                                // Single value return
                                return_value = result;
                            }
                        }
                    } else {
                        return_value = Value.Null;
                    }
                    has_returned = true;
                    break; // Exit function body loop
                },
                else => try self.executeStatement(stmt.*),
            }
        }
        
        // Function completed - defers will be executed by the defer block above
        return return_value;
    }

    fn valuesEqual(self: *Interpreter, left: Value, right: Value) bool {
        
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
            .Struct => |_| {
                switch (right) {
                    .Struct => |_| return false, // TODO: Implement struct comparison
                    else => return false,
                }
            },
            .Interface => |_| {
                switch (right) {
                    .Interface => |_| return false, // TODO: Implement interface comparison
                    else => return false,
                }
            },
            .Error => |_| {
                switch (right) {
                    .Error => |_| return false, // TODO: Implement error comparison
                    else => return false,
                }
            },
            .CursedError => |_| {
                switch (right) {
                    .CursedError => |_| return false, // TODO: Implement cursed error comparison
                    else => return false,
                }
            },
            .Tuple => |left_tuple| {
                switch (right) {
                    .Tuple => |right_tuple| {
                        if (left_tuple.items.len != right_tuple.items.len) return false;
                        for (left_tuple.items, right_tuple.items) |left_item, right_item| {
                            if (!self.valuesEqual(left_item, right_item)) return false;
                        }
                        return true;
                    },
                    else => return false,
                }
            },
        }
    }

    // CURSED Error Handling System Interpreter Implementation
    
    pub fn executeYikesStatement(self: *Interpreter, yikes: ast.YikesStatement) InterpreterError!void {
        const error_prop = @import("error_propagation.zig");
        
        // Evaluate the error message expression
        const message_value = try self.evaluateExpression(yikes.message.*);
        const message = switch (message_value) {
            .String => |s| s,
            else => "Unknown error",
        };
        
        // Create source location if available
        const location = if (yikes.location) |loc| 
            error_prop.ErrorContext.SourceLocation{
                .file = loc.file,
                .line = loc.line,
                .column = loc.column,
            }
        else null;
        
        // Use error propagation system to create and handle error
        var error_propagator = error_prop.ErrorPropagation.init(self.allocator);
        defer error_propagator.deinit();
        
        const error_ctx = try error_propagator.createYikesError(
            message,
            yikes.error_type,
            location
        );
        
        // Propagate error immediately (yikes is like throw/panic)
        const should_continue = try error_propagator.propagateError(error_ctx, true);
        if (!should_continue) {
            // Print the error with full context
            const stdout = std.io.getStdOut().writer();
            error_ctx.format(stdout) catch |err| {
                std.debug.print("Error formatting context: {}\n", .{err});
            };
            return InterpreterError.RuntimeError;
        }
    }

    pub fn executeFamStatement(self: *Interpreter, fam: ast.FamStatement) InterpreterError!void {
        const error_prop = @import("error_propagation.zig");
        
        // Create error propagation system for this fam block
        var error_propagator = error_prop.ErrorPropagation.init(self.allocator);
        defer error_propagator.deinit();
        
        // Enter try-catch block
        try error_propagator.enterTryCatchBlock(fam.catch_blocks.items, fam.finally_block);
        
        var error_occurred: ?error_prop.ErrorContext = null;
        
        // Execute try body with error catching
        for (fam.try_body.items) |stmt| {
            self.executeStatement(stmt) catch |err| {
                // Create error context from interpreter error
                const location = error_prop.ErrorContext.SourceLocation{
                    .file = "unknown", // TODO: Get from statement location
                    .line = 0,
                    .column = 0,
                };
                
                error_occurred = error_prop.ErrorContext.initWithLocation(
                    self.allocator,
                    switch (err) {
                        InterpreterError.RuntimeError => CursedError.RuntimeError,
                        InterpreterError.UndefinedVariable => CursedError.UndefinedVariable,
                        InterpreterError.TypeMismatch => CursedError.TypeMismatch,
                        InterpreterError.DivisionByZero => CursedError.DivisionByZero,
                        else => CursedError.UnknownError,
                    },
                    @errorName(err),
                    location
                ) catch break;
                break;
            };
        }
        
        // Handle errors with catch blocks
        if (error_occurred) |error_ctx| {
            var handled = false;
            
            for (fam.catch_blocks.items) |catch_block| {
                // Check if this catch block matches the error type
                if (error_propagator.errorMatches(error_ctx, catch_block.error_type)) {
                    // Bind error variable if specified
                    if (catch_block.error_variable) |error_var| {
                        const error_msg = try self.allocator.dupe(u8, error_ctx.message);
                        try self.environment.define(error_var, Value{ .String = error_msg });
                    }
                    
                    // Execute catch block code
                    for (catch_block.body.items) |stmt| {
                        try self.executeStatement(stmt);
                    }
                    
                    handled = true;
                    break;
                }
            }
            
            if (!handled) {
                // No matching catch block, propagate the error
                const stdout = std.io.getStdOut().writer();
                stdout.print("Unhandled error in fam block: {s}\n", .{"unknown"}) catch {};
                stdout.print("Error context: {s}\n", .{error_ctx.message}) catch {};
                return InterpreterError.RuntimeError;
            }
        }
        
        // Execute finally block if it exists
        if (fam.finally_block) |finally_stmts| {
            for (finally_stmts.items) |stmt| {
                try self.executeStatement(stmt);
            }
        }
        
        // Exit try-catch block and handle any remaining errors
        if (try error_propagator.exitTryCatchBlock()) |unhandled_error| {
            const stdout = std.io.getStdOut().writer();
            stdout.print("Unhandled error after fam block: {s}\n", .{unhandled_error.message}) catch {};
            return InterpreterError.RuntimeError;
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
        
        // Create CURSED error with full context
        const error_obj = try self.error_handler.yikes(
            message,
            .Runtime,
            code,
            0, // TODO: Get actual line number
            0  // TODO: Get actual column
        );
        
        return Value{ .CursedError = error_obj };
    }

    fn evaluateFam(self: *Interpreter, fam: ast.FamExpression) InterpreterError!Value {
        const last_result = Value.Null;
        var error_occurred: ?*cursed_error.CursedError = null;
        
        // Execute try body
        for (fam.try_body.items) |stmt_ptr| {
            const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
            self.executeStatement(stmt.*) catch |err| {
                error_occurred = try self.error_handler.yikes(
                    @errorName(err),
                    .Runtime,
                    @intFromError(err),
                    0, 0
                );
                break;
            };
        }
        
        // Execute catch handler if error occurred
        if (error_occurred != null and fam.catch_handler != null) {
            const catch_handler = fam.catch_handler.?;
            
            // Set error variable in environment
            try self.environment.define(catch_handler.error_variable, Value{ .CursedError = error_occurred.? });
            
            // Execute catch body
            for (catch_handler.handler_body.items) |stmt_ptr| {
                const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
                self.executeStatement(stmt.*) catch {
                    // Ignore errors in catch handler
                };
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
            return Value{ .CursedError = err };
        }
        return last_result;
    }

    pub fn executeStanStatement(self: *Interpreter, stan: ast.StanStatement) InterpreterError!void {
        // Import concurrency runtime for goroutine spawning
        const concurrency_runtime = @import("concurrency_runtime.zig");
        
        // Create goroutine context
        const GoroutineContext = struct {
            interpreter: *Interpreter,
            statements: @TypeOf(stan.body),
            allocator: std.mem.Allocator,
            
            pub fn execute(ctx: ?*anyopaque) void {
                const context: *@This() = @ptrCast(@alignCast(ctx.?));
                
                // Create a new environment for the goroutine
                var goroutine_env = Environment.init(context.allocator, context.interpreter.environment);
                defer goroutine_env.deinit();
                
                const old_env = context.interpreter.environment;
                context.interpreter.environment = &goroutine_env;
                defer context.interpreter.environment = old_env;
                
                // Execute all statements in the goroutine body
                for (context.statements.items) |stmt_ptr| {
                    const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
                    context.interpreter.executeStatement(stmt.*) catch |err| {
                        // Handle goroutine errors gracefully
                        std.debug.print("Goroutine error: {}\n", .{err});
                        return;
                    };
                }
            }
        };
        
        // Create context for the goroutine
        const context = try self.allocator.create(GoroutineContext);
        context.* = GoroutineContext{
            .interpreter = self,
            .statements = stan.body,
            .allocator = self.allocator,
        };
        
        // Spawn the goroutine
        const goroutine_id = concurrency_runtime.executeStanFromInterpreter(context, GoroutineContext.execute) catch |err| {
            std.debug.print("Failed to spawn goroutine: {}\n", .{err});
            self.allocator.destroy(context);
            return;
        };
        
        std.debug.print("Spawned goroutine with ID: {}\n", .{goroutine_id});
        
        // Wait a bit longer for goroutine to execute
        std.time.sleep(10_000_000); // 10ms
    }
    
    /// Execute basic switch statement (simple value matching)
    pub fn executeSwitchStatement(self: *Interpreter, switch_stmt: ast.SwitchStatement) InterpreterError!void {
        const switch_expr: *Expression = @ptrCast(@alignCast(switch_stmt.expression));
        const switch_value = try self.evaluateExpression(switch_expr.*);
        
        // Find matching case
        var matched = false;
        for (switch_stmt.cases.items) |case| {
            const case_expr: *ast.Expression = @ptrCast(@alignCast(case.value));
            const case_value = try self.evaluateExpression(case_expr.*);
            
            if (switch_value.equals(case_value)) {
                matched = true;
                for (case.body.items) |stmt_ptr| {
                    const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
                    try self.executeStatement(stmt.*);
                }
                break; // Break after first match (no fallthrough)
            }
        }
        
        // Execute default case if no match
        if (!matched) {
            if (switch_stmt.default_case) |default_stmts| {
                for (default_stmts.items) |stmt_ptr| {
                    const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
                    try self.executeStatement(stmt.*);
                }
            }
        }
    }

    /// Execute pattern switch statement (advanced pattern matching)
    pub fn executePatternSwitchStatement(self: *Interpreter, pattern_switch: ast.PatternSwitchStatement) InterpreterError!void {
        const switch_value = try self.evaluateExpression(pattern_switch.expression.*);
        
        // Try each pattern case
        var matched = false;
        for (pattern_switch.patterns.items) |case| {
            // Create new scope for pattern bindings
            var pattern_env = Environment.init(self.allocator, self.environment);
            defer pattern_env.deinit();
            
            const old_env = self.environment;
            self.environment = &pattern_env;
            defer self.environment = old_env;
            
            // Try to match pattern
            if (try self.matchPattern(case.pattern, switch_value)) {
                // Check guard condition if present
                if (case.guard) |guard_expr| {
                    const guard_expr_ptr: *ast.Expression = @ptrCast(@alignCast(guard_expr));
                    const guard_value = try self.evaluateExpression(guard_expr_ptr.*);
                    if (!guard_value.toBool()) {
                        continue; // Guard failed, try next pattern
                    }
                }
                
                matched = true;
                // Execute case body
                for (case.body.items) |stmt| {
                    try self.executeStatement(stmt.*);
                }
                break; // Exit after first successful match
            }
        }
        
        // Handle default case if no pattern matched
        if (!matched and pattern_switch.default_case != null) {
            const default_case = pattern_switch.default_case.?;
            matched = true;
            
            // Execute default case body
            for (default_case.items) |stmt| {
                try self.executeStatement(stmt.*);
            }
        }
        
        // Handle unmatched patterns without default case
        if (!matched) {
            return InterpreterError.PatternMatchFailed;
        }
    }

    /// Evaluate match expression with pattern matching
    fn evaluateMatch(self: *Interpreter, match_expr: ast.MatchExpression) InterpreterError!Value {
        const match_value = try self.evaluateExpression(match_expr.expression.*);
        
        // Try each pattern case
        for (match_expr.cases.items) |case| {
            // Create new scope for pattern bindings
            var pattern_env = Environment.init(self.allocator, self.environment);
            defer pattern_env.deinit();
            
            const old_env = self.environment;
            self.environment = &pattern_env;
            defer self.environment = old_env;
            
            // Try to match pattern
            if (try self.matchPattern(case.pattern, match_value)) {
                // Check guard condition if present
                if (case.guard) |guard_expr| {
                    const guard_expr_ptr: *ast.Expression = @ptrCast(@alignCast(guard_expr));
                    const guard_value = try self.evaluateExpression(guard_expr_ptr.*);
                    if (!guard_value.toBool()) {
                        continue; // Guard failed, try next pattern
                    }
                }
                
                // Return the result expression value
                const result_ptr: *Expression = @ptrCast(@alignCast(case.result));
                return try self.evaluateExpression(result_ptr.*);
            }
        }
        
        // Check default case
        if (match_expr.default_case) |default_expr| {
            const default_ptr: *Expression = @ptrCast(@alignCast(default_expr));
            return try self.evaluateExpression(default_ptr.*);
        }
        
        // No pattern matched and no default
        return InterpreterError.PatternMatchFailed;
    }

    /// Match a pattern against a value, binding variables as needed
    fn matchPattern(self: *Interpreter, pattern: ast.Pattern, value: Value) InterpreterError!bool {
        switch (pattern) {
            .Literal => |lit| {
                return switch (lit) {
                    .Integer => |i| value.equals(Value{ .Integer = i }),
                    .Float => |f| value.equals(Value{ .Float = f }),
                    .String => |s| value.equals(Value{ .String = s }),
                    .Boolean => |b| value.equals(Value{ .Boolean = b }),
                    .Character => |c| value.equals(Value{ .Character = c }),
                    .Null, .Nil => value.equals(Value.Null),
                };
            },
            .Variable => |var_name| {
                // Bind variable to value
                try self.environment.define(var_name, value);
                return true; // Variables always match
            },
            .Wildcard => {
                return true; // Wildcard matches anything
            },
            .Tuple => |tuple_pattern| {
                if (value != .Tuple) return false;
                const array_val = value.Tuple;
                
                if (array_val.items.len != tuple_pattern.items.len) {
                    return false;
                }
                
                // Match each element
                for (tuple_pattern.items, 0..) |element_pattern, i| {
                    if (!try self.matchPattern(element_pattern, array_val.items[i])) {
                        return false;
                    }
                }
                return true;
            },
            .Array => |array_pattern| {
                if (value != .Tuple) return false;
                const array_val = value.Tuple;
                
                // For now, simple length-based matching
                if (array_pattern.items.len > 0) {
                    if (array_val.items.len != array_pattern.items.len) {
                        return false;
                    }
                }
                
                // Match each specified element
                for (array_pattern.items, 0..) |element_pattern, i| {
                    if (!try self.matchPattern(element_pattern, array_val.items[i])) {
                        return false;
                    }
                }
                
                // Rest patterns not implemented yet
                
                return true;
            },
            // Guard patterns are not implemented in current AST
            else => {
                std.debug.print("Unsupported pattern type: {s}\n", .{@tagName(pattern)});
                return false;
            },
        }
    }

    /// Execute defer statement by pushing it onto the defer stack
    pub fn executeDeferStatement(self: *Interpreter, defer_stmt: ast.DeferStatement) InterpreterError!void {
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
            self.environment = defer_entry.?.environment;
            
            // Execute the deferred statement
            std.debug.print("Executing deferred statement\n", .{});
            self.executeStatement(defer_entry.?.statement) catch |err| {
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
            self.environment = defer_entry.?.environment;
            
            // Execute the deferred statement
            std.debug.print("Executing scoped deferred statement\n", .{});
            self.executeStatement(defer_entry.?.statement) catch |err| {
                std.debug.print("Error executing deferred statement: {}\n", .{err});
                // Continue with other defers even if one fails
            };
            
            // Restore environment
            self.environment = saved_env;
        }
    }

    fn evaluateShook(self: *Interpreter, shook: ast.ShookExpression) InterpreterError!Value {
        const error_prop = @import("error_propagation.zig");
        
        // Create error propagation system
        var error_propagator = error_prop.ErrorPropagation.init(self.allocator);
        defer error_propagator.deinit();
        
        // Evaluate the wrapped expression
        const result = self.evaluateExpression(shook.expression.*) catch |err| {
            // Convert caught error to error context
            const location = error_prop.ErrorContext.SourceLocation{
                .file = "unknown", // TODO: Get from context
                .line = 0,
                .column = 0,
            };
            
            const error_ctx = try error_prop.ErrorContext.initWithLocation(
                self.allocator,
                switch (err) {
                    InterpreterError.RuntimeError => CursedError.RuntimeError,
                    InterpreterError.UndefinedVariable => CursedError.UndefinedVariable,
                    InterpreterError.TypeMismatch => CursedError.TypeMismatch,
                    InterpreterError.DivisionByZero => CursedError.DivisionByZero,
                    else => CursedError.UnknownError,
                },
                @errorName(err),
                location
            );
            
            // Use error propagation system to handle the error
            const should_continue = try error_propagator.propagateError(error_ctx, true);
            if (!should_continue) {
                // Error should be propagated up the call stack
                const stdout = std.io.getStdOut().writer();
                stdout.print("Shook propagated error: {s}\n", .{"unknown"}) catch {};
                stdout.print("Error context: {s}\n", .{error_ctx.message}) catch {};
                return InterpreterError.RuntimeError;
            }
            
            // Convert to Value for return
            const error_msg = try self.allocator.dupe(u8, error_ctx.message);
            return Value{ .String = error_msg };
        };
        
        // Check if result is already an error value
        switch (result) {
            .Error => |error_val| {
                // Convert old error format to new error context
                const location = error_prop.ErrorContext.SourceLocation{
                    .file = "unknown",
                    .line = 0,
                    .column = 0,
                };
                
                const error_ctx = try error_prop.ErrorContext.initWithLocation(
                    self.allocator,
                    CursedError.RuntimeError,
                    error_val.message,
                    location
                );
                
                // Propagate using new system
                const should_continue = try error_propagator.propagateError(error_ctx, true);
                if (!should_continue) {
                    return InterpreterError.RuntimeError;
                }
                
                const error_msg = try self.allocator.dupe(u8, error_ctx.message);
                return Value{ .String = error_msg };
            },
            .String => |str_val| {
                // Check if this is an error message (simple heuristic)
                if (std.mem.startsWith(u8, str_val, "Error:") or 
                    std.mem.startsWith(u8, str_val, "yikes:") or
                    std.mem.indexOf(u8, str_val, "error") != null) {
                    
                    // Create error context for error message
                    const error_ctx = try error_prop.ErrorContext.init(
                        self.allocator,
                        CursedError.RuntimeError,
                        str_val
                    );
                    
                    // Propagate the error
                    const should_continue = try error_propagator.propagateError(error_ctx, true);
                    if (!should_continue) {
                        return InterpreterError.RuntimeError;
                    }
                }
                
                // Regular string value, return as-is
                return result;
            },
            else => {
                // Normal value, return as-is (shook operator passes through non-errors)
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
        return Value{ .Integer = 0 }; // Default value when channel is empty
    }

    // Enhanced concurrency support
    fn executeGoroutine(self: *Interpreter, function_value: Value) InterpreterError!u64 {
        _ = self;
        _ = function_value;
        // In real implementation, this would spawn actual goroutines
        // For now, return a simulated goroutine ID
        return 1;
    }
    
    fn evaluateTuple(self: *Interpreter, tuple: ast.TupleExpression) InterpreterError!Value {
        var tuple_values = ArrayList(Value).init(self.allocator);
        
        for (tuple.elements.items) |element_expr| {
            const element_value = try self.evaluateExpression(element_expr.*);
            try tuple_values.append(element_value);
        }
        
        return Value{ .Tuple = tuple_values };
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
