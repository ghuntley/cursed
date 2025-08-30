const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const error_handling = @import("error_handling.zig");
const cursed_error = @import("cursed_error_runtime.zig");
const concurrency = @import("concurrency.zig");
const gc = @import("gc.zig");
const stack_trace = @import("stack_trace_runtime.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const CursedError = error_handling.CursedError;
const ErrorContext = error_handling.ErrorContext;
const safeDupeString = error_handling.safeDupeString;

// Forward declarations for struct and interface support
pub const StructInstance = struct {
    type_name: []const u8,
    fields: HashMap([]const u8, *Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, type_name: []const u8) CursedError!StructInstance {
        const type_name_copy = safeDupeString(allocator, type_name) catch |err| {
            return err;
        };
        
        return StructInstance{
            .type_name = type_name_copy,
            .fields = HashMap([]const u8, *Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *StructInstance) void {
        // Free all values stored in fields
        var iterator = self.fields.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.*.deinit(self.allocator);
            self.allocator.destroy(entry.value_ptr.*);
            self.allocator.free(entry.key_ptr.*);
        }
        self.allocator.free(self.type_name);
        self.fields.deinit();
    }
    
    pub fn setField(self: *StructInstance, name: []const u8, value: Value) !void {
        const field_name = try self.allocator.dupe(u8, name);
        const value_ptr = try self.allocator.create(Value);
        value_ptr.* = value;
        try self.fields.put(field_name, value_ptr);
    }
    
    pub fn getField(self: *StructInstance, name: []const u8) ?Value {
        if (self.fields.get(name)) |value_ptr| {
            return value_ptr.*;
        }
        return null;
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
        
        // Capture stack trace for error
        const stack_trace_str = stack_trace.cursed_runtime_get_stack_trace();
        const stack_trace_copy = if (std.mem.len(stack_trace_str) > 0) 
            allocator.dupe(u8, std.mem.span(stack_trace_str)) catch null
        else 
            null;
        
        var stack_frames: ?[][]const u8 = null;
        if (stack_trace_copy) |trace_str| {
            // Convert stack trace string to array of frames
            var frame_list = std.ArrayList([]const u8){};
            defer frame_list.deinit();
            
            var lines = std.mem.split(u8, trace_str, "\n");
            while (lines.next()) |line| {
                if (line.len > 0) {
                    const line_copy = allocator.dupe(u8, line) catch continue;
                    frame_list.append(allocator, line_copy) catch continue;
                }
            }
            
            stack_frames = frame_list.toOwnedSlice() catch null;
            allocator.free(trace_str);
        }
        
        return ErrorValue{
            .message = message_copy,
            .code = code,
            .context = null,
            .stack_trace = stack_frames,
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

pub const PointerValue = struct {
    pointee_value: *Value,
    allocator: Allocator,
    
    pub fn deinit(self: *PointerValue) void {
        self.pointee_value.deinit(self.allocator);
        self.allocator.destroy(self.pointee_value);
    }
};

const Variable = struct { name: []const u8, value: Value };

pub const ModuleInstance = struct {
    functions: std.StringHashMap(Value),
    // Keep the arena that owns the AST alive
    arena: ?std.heap.ArenaAllocator,
    
    pub fn deinit(self: *ModuleInstance, allocator: Allocator) void {
        var iterator = self.functions.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        self.functions.deinit();
        // Free the arena last - this frees the AST
        if (self.arena) |*arena| {
            arena.deinit();
        }
    }
};

pub const BuiltinFunctionValue = struct {
    name: []const u8,
    func: *const fn(*Interpreter, []Value) InterpreterError!Value,
};

pub const Value = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,
    Pointer: PointerValue,
    Struct: StructInstance,
    Interface: InterfaceInstance,
    Error: ErrorValue,
    CursedError: *cursed_error.CursedError,
    Array: []Value,
    Module: *ModuleInstance,
    BuiltinFunction: BuiltinFunctionValue,
    UserFunction: CursedFunction,
    // Tuple: *ArrayList(Value), // Temporarily disabled due to circular dependency

    pub fn deinit(self: *Value, allocator: Allocator) void {
        switch (self.*) {
            .String => |str| allocator.free(str),
            .Array => |array| {
                for (array) |*item| {
                    item.deinit(allocator);
                }
                allocator.free(array);
            },
            // }
            //     tuple.deinit();
            // },
            .Error => |*err| err.deinit(),
            .Pointer => |*ptr| ptr.deinit(),
            .Struct => |*struct_inst| struct_inst.deinit(),
            .Interface => |*interface_inst| interface_inst.deinit(),
            .CursedError => |cursed_err| {
                cursed_err.deinit();
                allocator.destroy(cursed_err);
            },
            .Module => |module_ptr| {
                module_ptr.deinit(allocator);
                allocator.destroy(module_ptr);
            },
            .BuiltinFunction => {}, // No cleanup needed
            .UserFunction => {}, // Function declarations are managed by the AST
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
            .Pointer => |ptr| return std.fmt.allocPrint(allocator, "pointer@{*}", .{ptr.pointee_value}),
            .Struct => |struct_inst| return std.fmt.allocPrint(allocator, "struct {s}", .{struct_inst.type_name}),
            .Interface => |interface_inst| return std.fmt.allocPrint(allocator, "interface {s}", .{interface_inst.vtable.interface_name}),
            .Error => |err| return std.fmt.allocPrint(allocator, "Error({s})", .{err.message}),
            .CursedError => |cursed_err| return try cursed_err.toString(),
            .Array => |array| {
                // Convert array to string representation
                if (array.len == 0) {
                    return allocator.dupe(u8, "[]");
                }
                
                // Build string manually
                var total_size: usize = 2; // for '[' and ']'
                var element_strs = try allocator.alloc([]u8, array.len);
                defer allocator.free(element_strs);
                
                for (array, 0..) |value, i| {
                    element_strs[i] = try value.toString(allocator);
                    total_size += element_strs[i].len;
                    if (i > 0) total_size += 2; // for ", "
                }
                
                var result = try allocator.alloc(u8, total_size);
                var idx: usize = 0;
                result[idx] = '[';
                idx += 1;
                
                for (element_strs, 0..) |element_str, i| {
                    if (i > 0) {
                        result[idx] = ',';
                        result[idx + 1] = ' ';
                        idx += 2;
                    }
                    @memcpy(result[idx..idx + element_str.len], element_str);
                    idx += element_str.len;
                    allocator.free(element_str);
                }
                
                result[idx] = ']';
                return result;
            },
            .Module => |module_ptr| return std.fmt.allocPrint(allocator, "module({} functions)", .{module_ptr.functions.count()}),
            .BuiltinFunction => |builtin| return std.fmt.allocPrint(allocator, "builtin function {s}", .{builtin.name}),
            .UserFunction => |user_func| return std.fmt.allocPrint(allocator, "user function {s}", .{user_func.declaration.name}),
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
            .Pointer => return true,  // Pointers are always truthy if they exist
            .Struct => return true,   // Structs are always truthy if they exist
            .Interface => return true, // Interfaces are always truthy if they exist
            .Error => return false,   // Errors are falsy
            .CursedError => return false, // CursedErrors are falsy
            .Array => |array| return array.len > 0, // Arrays are truthy if they have elements
            .Module => return true, // Modules are always truthy if they exist
            .BuiltinFunction => return true, // Builtin functions are always truthy
            .UserFunction => return true, // User functions are always truthy
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
    
    // Create Environment on heap for long-lived use
    pub fn newEnvironment(allocator: Allocator, parent: ?*Environment) !*Environment {
        const env = try allocator.create(Environment);
        env.* = .{
            .variables = HashMap([]const u8, Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .parent = parent,
            .allocator = allocator,
        };
        std.debug.print("DEBUG: Created new environment@{*} with parent@{*}\n", .{ env, parent });
        return env;
    }

    pub fn deinit(self: *Environment) void {
        self.variables.deinit();
    }

    pub fn define(self: *Environment, name: []const u8, value: Value) !void {
        try self.variables.put(name, value);
    }

    pub fn get(self: *Environment, name: []const u8) InterpreterError!Value {
        var current: ?*Environment = self;
        var hops: usize = 0;
        while (current) |env| {
            const var_count = env.variables.count();
            std.debug.print("DEBUG: Environment.get() hop {}: checking env@{*} with {} variables for '{s}'\n", .{ hops, env, var_count, name });
            
            // Safety check for corruption
            if (var_count > 1000) {
                std.debug.print("CORRUPTION DETECTED: Environment@{*} has impossible variable count: {}\n", .{ env, var_count });
                std.debug.print("Parent chain: env@{*} -> parent@{*}\n", .{ env, env.parent });
                return InterpreterError.MemoryCorruption;
            }
            
            if (env.variables.get(name)) |value| {
                std.debug.print("DEBUG: Found '{s}' in environment@{*}\n", .{ name, env });
                return value;
            }
            current = env.parent;
            hops += 1;
            std.debug.assert(hops < 1_000_000); // detect accidental cycles
        }
        std.debug.print("DEBUG: Variable '{s}' not found in any environment after {} hops\n", .{ name, hops });
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
        const key = try std.fmt.allocPrint(self.allocator, "{s}::{s}", .{struct_name, interface_name});
        try self.vtables.put(key, vtable.*);
    }
    
    pub fn getVTable(self: *TypeRegistry, struct_name: []const u8, interface_name: []const u8) ?VTable {
        const key_temp = std.fmt.allocPrint(self.allocator, "{s}::{s}", .{struct_name, interface_name}) catch return null;
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
    current_file: ?[]const u8,
    current_line: ?u32,
    current_column: ?u32,
    allocator: Allocator,

    const MAX_CALL_STACK_DEPTH = 1000;

    pub fn init(allocator: Allocator) Interpreter {
        var interp = Interpreter{
            .globals = Environment.init(allocator, null),
            .environment = undefined, // will be set correctly below
            .functions = HashMap([]const u8, CursedFunction, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .type_registry = TypeRegistry.init(allocator),
            .channel_storage = HashMap(u64, ArrayList(Value), std.hash_map.AutoContext(u64), std.hash_map.default_max_load_percentage).init(allocator),
            .next_goroutine_id = 0,
            .defer_stack = ArrayList(DeferEntry){},
            .error_handler = cursed_error.ErrorHandler.init(allocator, "main.csd"),
            .call_stack_depth = 0,
            .max_call_stack_depth = MAX_CALL_STACK_DEPTH,
            .current_file = null,
            .current_line = null,
            .current_column = null,
            .allocator = allocator,
        };
        interp.environment = &interp.globals; // Correct pointer to the persistent globals
        std.debug.print("DEBUG: Initialized interpreter with globals@{*} (parent: {*})\n", .{ &interp.globals, interp.globals.parent });
        return interp;
    }

    pub fn deinit(self: *Interpreter) void {
        // Execute any remaining deferred statements before cleanup
        self.executeAllDefers();
        
        self.globals.deinit();
        self.functions.deinit();
        self.type_registry.deinit();
        self.defer_stack.deinit(self.allocator);
        self.error_handler.deinit();
        
        // Clean up channel storage
        // var channel_iterator = self.channel_storage.iterator();
        // while (channel_iterator.next()) |entry| {
        //     // Clean up each Value in the channel's ArrayList
        //     for (entry.value_ptr.items) |*value| {
        //         value.deinit();
        //     }
        //     entry.value_ptr.deinit();
        // }
        // self.channel_storage.deinit(self.allocator);
    }

    pub fn interpret(self: *Interpreter, program: Program) InterpreterError!void {
        return self.execute(program);
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
                .Import => |import_stmt| {
                    // Process imports during the first pass
                    try self.executeImportStatement(import_stmt);
                },
                else => {}
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
        std.debug.print("DEBUG: Executing statement type: {s}\n", .{ @tagName(stmt) });
        switch (stmt) {
            .Expression => |expr| {
                _ = try self.evaluateExpression(expr);
            },
            .Let => |let| try self.executeLetStatement(let),
            .ShortDeclaration => |short_decl| try self.executeShortDeclarationStatement(short_decl),
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
            .Import => |import_stmt| try self.executeImportStatement(import_stmt),
            else => {
                std.debug.print("Unsupported statement type in interpreter: {s}\n", .{ @tagName(stmt) });
            }
        }
    }

    fn executeImportStatement(self: *Interpreter, import_stmt: ast.ImportStatement) InterpreterError!void {
        std.debug.print("DEBUG: *** EXECUTING IMPORT STATEMENT: {s} ***\n", .{import_stmt.path});
        
        // For now, use builtin modules to avoid complex parsing issues
        try self.loadBuiltinModule(import_stmt.path);
    }
    
    fn loadRealStdlibModule(self: *Interpreter, module_name: []const u8) InterpreterError!void {
        var module_arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        const temp_allocator = module_arena.allocator();
        
        // Try multiple possible paths for the module
        const possible_paths = [_][]const u8{
            try std.fmt.allocPrint(temp_allocator, "stdlib/{s}/mod.csd", .{module_name}),
            try std.fmt.allocPrint(temp_allocator, "stdlib/layer1/{s}.csd", .{module_name}),
            try std.fmt.allocPrint(temp_allocator, "stdlib/{s}/{s}.csd", .{ module_name, module_name }),
        };
        
        var loaded_successfully = false;
        
        for (possible_paths) |path_str| {
            std.debug.print("DEBUG: Trying to load module from: {s}\n", .{path_str});
            
            // Try to read the module file
            const file = std.fs.cwd().openFile(path_str, .{}) catch |err| {
                std.debug.print("DEBUG: Could not open {s}: {}\n", .{ path_str, err });
                continue;
            };
            defer file.close();
            
            const source = file.readToEndAlloc(temp_allocator, std.math.maxInt(usize)) catch |err| {
                std.debug.print("DEBUG: Could not read {s}: {}\n", .{ path_str, err });
                continue;
            };
            defer temp_allocator.free(source);
            
            std.debug.print("DEBUG: Successfully loaded source from {s}, parsing...\n", .{path_str});
            
            // Parse the module
            var module_lexer = lexer.Lexer.init(temp_allocator, source);
            
            var tokens = module_lexer.tokenize() catch |err| {
                std.debug.print("DEBUG: Tokenize error for {s}: {}\n", .{ path_str, err });
                continue;
            };
            defer tokens.deinit(temp_allocator);
            
            var module_parser = parser.Parser.init(temp_allocator, tokens.items);
            defer module_parser.deinit();
            
            var module_program = module_parser.parseProgram() catch |err| {
                std.debug.print("DEBUG: Parse error for {s}: {}\n", .{ path_str, err });
                continue;
            };
            defer module_program.deinit(temp_allocator);
            
            std.debug.print("DEBUG: Successfully parsed {s}, extracting functions...\n", .{path_str});
            
            // Extract function declarations and create a simple module representation
            try self.createModuleFromProgram(module_name, module_program, path_str, module_arena);
            loaded_successfully = true;
            break;
        }
        
        if (!loaded_successfully) {
            // Clean up arena if module loading failed
            module_arena.deinit();
            return InterpreterError.ModuleNotFound;
        }
        
        std.debug.print("DEBUG: Successfully loaded real stdlib module: {s}\n", .{module_name});
    }
    
    fn createModuleFromProgram(self: *Interpreter, module_name: []const u8, program: ast.Program, source_path: []const u8, module_arena: std.heap.ArenaAllocator) InterpreterError!void {
        var module_functions = std.StringHashMap(Value).init(self.allocator);
        
        std.debug.print("DEBUG: Creating module {s} from program with {} statements\n", .{ module_name, program.statements.items.len });
        
        // Extract function declarations from the program
        for (program.statements.items) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            switch (stmt.*) {
                .Function => |func_decl| {
                    std.debug.print("DEBUG: Found function {s} in module {s}\n", .{ func_decl.name, module_name });
                    
                    // Create a closure for this function with the module's environment
                    const func_closure = CursedFunction{
                        .declaration = func_decl,
                        .closure = &self.globals, // Use global environment as closure for stdlib functions
                    };
                    
                    // Store as a BuiltinFunction that calls the real function
                    const wrapped_func = Value{ .UserFunction = func_closure };
                    try module_functions.put(func_decl.name, wrapped_func);
                },
                else => {
                    // Skip non-function statements for now
                }
            }
        }
        
        // Create module instance on heap and store pointer in globals
        const module_ptr = try self.allocator.create(ModuleInstance);
        module_ptr.* = .{ 
            .functions = module_functions,
            .arena = module_arena, // Transfer ownership of arena to module
        };
        
        const module_value = Value{ .Module = module_ptr };
        try self.globals.define(module_name, module_value);
        
        std.debug.print("DEBUG: Created real stdlib module {s} with {} functions from {s}\n", .{ module_name, module_functions.count(), source_path });
    }
    
    fn loadBuiltinModule(self: *Interpreter, module_name: []const u8) InterpreterError!void {
        var module_functions = std.StringHashMap(Value).init(self.allocator);
        
        // Hardcode stdlib functions for now
        if (std.mem.eql(u8, module_name, "vibez")) {
            // Add vibez functions
            try module_functions.put("spill", Value{ .BuiltinFunction = .{ .name = "vibez.spill", .func = builtinVibezSpill } });
            try module_functions.put("spillln", Value{ .BuiltinFunction = .{ .name = "vibez.spillln", .func = builtinVibezSpillln } });
            try module_functions.put("print_separator", Value{ .BuiltinFunction = .{ .name = "vibez.print_separator", .func = builtinVibezPrintSeparator } });
        } else if (std.mem.eql(u8, module_name, "mathz")) {
            // Add mathz functions  
            try module_functions.put("abs_normie", Value{ .BuiltinFunction = .{ .name = "mathz.abs_normie", .func = builtinMathzAbs } });
            try module_functions.put("max_normie", Value{ .BuiltinFunction = .{ .name = "mathz.max_normie", .func = builtinMathzMax } });
            try module_functions.put("min_normie", Value{ .BuiltinFunction = .{ .name = "mathz.min_normie", .func = builtinMathzMin } });
            try module_functions.put("add", Value{ .BuiltinFunction = .{ .name = "mathz.add", .func = builtinMathzAdd } });
            try module_functions.put("sub", Value{ .BuiltinFunction = .{ .name = "mathz.sub", .func = builtinMathzSub } });
            try module_functions.put("mul", Value{ .BuiltinFunction = .{ .name = "mathz.mul", .func = builtinMathzMul } });
            try module_functions.put("div", Value{ .BuiltinFunction = .{ .name = "mathz.div", .func = builtinMathzDiv } });
            try module_functions.put("pow", Value{ .BuiltinFunction = .{ .name = "mathz.pow", .func = builtinMathzPow } });
            try module_functions.put("sqrt", Value{ .BuiltinFunction = .{ .name = "mathz.sqrt", .func = builtinMathzSqrt } });
            try module_functions.put("floor", Value{ .BuiltinFunction = .{ .name = "mathz.floor", .func = builtinMathzFloor } });
            try module_functions.put("ceil", Value{ .BuiltinFunction = .{ .name = "mathz.ceil", .func = builtinMathzCeil } });
            try module_functions.put("round", Value{ .BuiltinFunction = .{ .name = "mathz.round", .func = builtinMathzRound } });
        } else if (std.mem.eql(u8, module_name, "stringz")) {
            // Add stringz functions
            try module_functions.put("length", Value{ .BuiltinFunction = .{ .name = "stringz.length", .func = builtinStringzLength } });
            try module_functions.put("concat", Value{ .BuiltinFunction = .{ .name = "stringz.concat", .func = builtinStringzConcat } });
        }
        
        // Create module instance on heap and store pointer in globals  
        const module_ptr = try self.allocator.create(ModuleInstance);
        module_ptr.* = .{ 
            .functions = module_functions,
            .arena = null, // Builtin modules don't need arena as they don't have AST
        };
        
        const module_value = Value{ .Module = module_ptr };
        try self.environment.define(module_name, module_value);
        
        std.debug.print("DEBUG: Stored module {s} in environment@{*}, now has {} variables\n", .{ module_name, self.environment, self.environment.variables.count() });
        std.debug.print("DEBUG: self.globals is at @{*}\n", .{&self.globals});
        std.debug.print("DEBUG: Loaded builtin module {s} with {} functions\n", .{ module_name, module_functions.count() });
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
            
            switch (value) {
                // Note: Tuple variant is temporarily disabled due to circular dependency
                // .Tuple => |tuple| {
                //     // Distribute tuple values to variable names
                //     while (name_iter.next()) |raw_name| {
                //         const trimmed_name = std.mem.trim(u8, raw_name, " \t");
                //         if (name_index < tuple.items.len) {
                //             try self.environment.define(trimmed_name, tuple.items[name_index]);
                //         } else {
                //             try self.environment.define(trimmed_name, Value.Null);
                //         }
                //         name_index += 1;
                //     }
                // },
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
    
    fn executeShortDeclarationStatement(self: *Interpreter, short_decl: ast.ShortDeclarationStatement) InterpreterError!void {
        // Handle sus x = 5 style declarations
        if (short_decl.names.items.len != short_decl.values.items.len) {
            return InterpreterError.TypeMismatch;
        }
        
        // Evaluate all values first
        var evaluated_values = std.ArrayList(Value){};
        defer evaluated_values.deinit(self.allocator);
        
        for (short_decl.values.items) |value_expr| {
            const value = try self.evaluateExpression(value_expr.*);
            try evaluated_values.append(self.allocator, value);
        }
        
        // Define all variables
        for (short_decl.names.items, 0..) |name, i| {
            try self.environment.define(name, evaluated_values.items[i]);
            std.debug.print("DEBUG: Defined variable '{s}' = {any}\n", .{name, evaluated_values.items[i]});
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
                std.debug.print("Unsupported assignment target: {s}\n", .{@tagName(target_expr.*) });
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
                    .Pointer => |*ptr| {
                        // Dereference pointer and assign to field
                        switch (ptr.pointee_value.*) {
                            .Struct => |*struct_inst| {
                                try struct_inst.setField(member.property, value);
                                // Update the pointer's pointee value
                                ptr.pointee_value.* = Value{ .Struct = struct_inst.* };
                            },
                            else => {
                                std.debug.print("Cannot assign to field of dereferenced non-struct: {s}\n", .{@tagName(ptr.pointee_value.*) });
                                return InterpreterError.TypeMismatch;
                            }
                        }
                    },
                    else => {
                        std.debug.print("Cannot assign to field of non-struct type: {s}\n", .{@tagName(object_value)});
                        return InterpreterError.TypeMismatch;
                    }
                }
            },
            .Unary => |unary| {
                // Handle dereferenced member access: (*ptr).field = value
                if (std.mem.eql(u8, unary.operator, "*")) {
                    const ptr_value = try self.evaluateExpression(unary.operand.*);
                    switch (ptr_value) {
                        .Pointer => |*ptr| {
                            switch (ptr.pointee_value.*) {
                                .Struct => |*struct_inst| {
                                    try struct_inst.setField(member.property, value);
                                    ptr.pointee_value.* = Value{ .Struct = struct_inst.* };
                                },
                                else => {
                                    return InterpreterError.TypeMismatch;
                                }
                            }
                        },
                        else => {
                            return InterpreterError.TypeMismatch;
                        }
                    }
                } else {
                    return InterpreterError.TypeMismatch;
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
        std.debug.print("DEBUG: Executing interface statement for '{s}'\n", .{interface_stmt.name});
    }

    fn executeImplementationStatement(self: *Interpreter, impl_stmt: ast.ImplementationStatement) InterpreterError!void {
        std.debug.print("DEBUG: Executing implementation statement: {s} for {s}\n", .{impl_stmt.implementing_type, impl_stmt.interface_name});
        
        // Get the interface definition
        const interface_def = self.type_registry.getInterface(impl_stmt.interface_name) orelse {
            std.debug.print("ERROR: Interface '{s}' not found\n", .{impl_stmt.interface_name});
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
                    std.debug.print("DEBUG: Added method '{s}' to vtable\n", .{interface_method.name});
                    break;
                }
            }
            
            if (!method_found) {
                const error_msg = try std.fmt.allocPrint(self.allocator, "Interface method '{s}' not implemented in implementation of '{s}'", .{interface_method.name, impl_stmt.interface_name});
                defer self.allocator.free(error_msg);
                return CursedError.InterfaceNotImplemented;
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
        std.debug.print("DEBUG: Evaluating expression type: {s}\n", .{@tagName(expr)});
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
                                    std.debug.print("DEBUG: Implicit field access for '{s}' resolved to: {s}\n", .{name, @tagName(field_value.*) });
                                    return field_value.*;
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
            .Unary => |unary| return try self.evaluateUnary(unary.*),
            .Call => |call| return try self.evaluateCall(call),
            .MemberAccess => |member| return try self.evaluateMemberAccess(member.*),
            .StructLiteral => |struct_lit| return try self.evaluateStructLiteral(struct_lit),
            .Struct => |struct_expr| return try self.evaluateStructExpression(struct_expr),
            .Yikes => |yikes| return try self.evaluateYikes(yikes),
            .Shook => |shook| return try self.evaluateShook(shook),
            .Fam => |fam| return try self.evaluateFam(fam),
            .StringInterpolation => |interpolation| return try self.evaluateStringInterpolation(interpolation),
            .Match => |match| return try self.evaluateMatch(match),
            .MethodCall => |method_call| {
                // Handle vibez.spill() and other built-in method calls
                if (std.mem.eql(u8, method_call.method_name, "spill")) {
                    for (method_call.arguments.items) |arg_ptr| {
                        const arg_value = try self.evaluateExpression(arg_ptr.*);
                        const str_repr = try arg_value.toString(self.allocator);
                        defer self.allocator.free(str_repr);
                        std.debug.print("{s} ", .{str_repr});
                    }
                    std.debug.print("\n", .{});
                    return Value.Null;
                }
                // For other method calls, use existing infrastructure
                const object_as_member = ast.MemberAccessExpression{
                    .object = method_call.object,
                    .property = method_call.method_name,
                };
                return try self.evaluateMethodCall(object_as_member, method_call.arguments.items);
            },
            
            else => {
                std.debug.print("Unsupported expression type in interpreter: {s}\n", .{@tagName(expr)});
                return Value.Null;
            }
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

    fn evaluateUnary(self: *Interpreter, unary: ast.UnaryExpression) InterpreterError!Value {
        const operand = try self.evaluateExpression(unary.operand.*);
        
        if (std.mem.eql(u8, unary.operator, "-")) {
            // Unary minus
            if (operand.isNumber()) {
                const num = try operand.toNumber();
                return Value{ .Float = -num };
            } else {
                return InterpreterError.TypeMismatch;
            }
        } else if (std.mem.eql(u8, unary.operator, "+")) {
            // Unary plus (no-op for numbers)
            if (operand.isNumber()) {
                return operand;
            } else {
                return InterpreterError.TypeMismatch;
            }
        } else if (std.mem.eql(u8, unary.operator, "!")) {
            // Logical not
            return Value{ .Boolean = !operand.toBool() };
        } else if (std.mem.eql(u8, unary.operator, "*")) {
            // Dereference pointer
            switch (operand) {
                .Pointer => |ptr| {
                    // For simplicity, we'll store the pointed-to value directly
                    // In a real implementation, this would dereference memory
                    return ptr.pointee_value.*;
                },
                else => {
                    std.debug.print("ERROR: Cannot dereference non-pointer value: {s}\n", .{@tagName(operand)});
                    return InterpreterError.TypeMismatch;
                }
            }
        } else if (std.mem.eql(u8, unary.operator, "&")) {
            // Address-of operator - create a pointer to the operand
            const pointee_ptr = try self.allocator.create(Value);
            pointee_ptr.* = operand;
            return Value{ .Pointer = PointerValue{
                .pointee_value = pointee_ptr,
                .allocator = self.allocator,
            }};
        } else {
            std.debug.print("ERROR: Unsupported unary operator: {s}\n", .{unary.operator});
            return InterpreterError.TypeMismatch;
        }
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
                    std.debug.print("DEBUG: Detected method call: {s}.{s}\n", .{@tagName(member.object.*), member.property });
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
                } 
                // Environment variable functions
                else if (std.mem.eql(u8, name, "runtime_get_env")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const name_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const name_str = try name_arg.toString(self.allocator);
                    defer self.allocator.free(name_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_get_env(self.allocator, name_str) catch {
                        // Create array with two strings: empty value and error message
                        var result_array = ArrayList(Value){};
                        try result_array.append(self.allocator, Value{ .String = try self.allocator.dupe(u8, "") });
                        try result_array.append(self.allocator, Value{ .String = try self.allocator.dupe(u8, "Failed to get environment variable") });
                        return Value{ .Array = try result_array.toOwnedSlice(self.allocator) };
                    };
                    
                    // Create array with two strings: value and error
                    var result_array = ArrayList(Value){};
                    try result_array.append(self.allocator, Value{ .String = result[0] });
                    try result_array.append(self.allocator, Value{ .String = result[1] });
                    return Value{ .Array = try result_array.toOwnedSlice(self.allocator) };
                } else if (std.mem.eql(u8, name, "runtime_set_env")) {
                    if (call.arguments.items.len != 2) {
                        return InterpreterError.TypeMismatch;
                    }
                    const name_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const value_arg = try self.evaluateExpression(call.arguments.items[1].*);
                    const name_str = try name_arg.toString(self.allocator);
                    defer self.allocator.free(name_str);
                    const value_str = try value_arg.toString(self.allocator);
                    defer self.allocator.free(value_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_set_env(self.allocator, name_str, value_str) catch |err| switch (err) {
                        error.OutOfMemory => return InterpreterError.OutOfMemory,
                    };
                    
                    return Value{ .String = try self.allocator.dupe(u8, result) };
                } else if (std.mem.eql(u8, name, "runtime_unset_env")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const name_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const name_str = try name_arg.toString(self.allocator);
                    defer self.allocator.free(name_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_unset_env(self.allocator, name_str) catch |err| switch (err) {
                        error.OutOfMemory => return InterpreterError.OutOfMemory,
                        else => "Failed to unset environment variable",
                    };
                    
                    return Value{ .String = try self.allocator.dupe(u8, result) };
                } else if (std.mem.eql(u8, name, "runtime_list_env")) {
                    if (call.arguments.items.len != 0) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_list_env(self.allocator) catch {
                        // Create array with empty list and error message
                        const empty_array = try self.allocator.alloc(Value, 0);
                        const error_values = try self.allocator.alloc(Value, 2);
                        error_values[0] = Value{ .Array = empty_array };
                        error_values[1] = Value{ .String = try self.allocator.dupe(u8, "Failed to list environment variables") };
                        return Value{ .Array = error_values };
                    };
                    
                    // Convert [][]const u8 to []Value
                    const env_value_array = try self.allocator.alloc(Value, result.env_vars.len);
                    for (result.env_vars, 0..) |env_str, i| {
                        env_value_array[i] = Value{ .String = env_str };
                    }
                    
                    // Create result array with env list and error string  
                    const result_values = try self.allocator.alloc(Value, 2);
                    result_values[0] = Value{ .Array = env_value_array };
                    result_values[1] = Value{ .String = result.error_msg };
                    return Value{ .Array = result_values };
                } else if (std.mem.eql(u8, name, "runtime_expand_env")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const text_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const text_str = try text_arg.toString(self.allocator);
                    defer self.allocator.free(text_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_expand_env(self.allocator, text_str) catch |err| switch (err) {
                        error.OutOfMemory => return InterpreterError.OutOfMemory,
                    };
                    
                    return Value{ .String = result };
                }
                // Time runtime functions
                else if (std.mem.eql(u8, name, "runtime_get_current_time_ms")) {
                    if (call.arguments.items.len != 0) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const timestamp = runtime_functions.runtime_get_current_time_ms();
                    return Value{ .Integer = timestamp };
                } else if (std.mem.eql(u8, name, "runtime_sleep_ms")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const duration_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const duration_ms = switch (duration_arg) {
                        .Integer => |int_val| int_val,
                        else => return InterpreterError.TypeMismatch,
                    };
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    runtime_functions.runtime_sleep_ms(duration_ms);
                    return Value.Null;
                } else if (std.mem.eql(u8, name, "runtime_get_timezone_offset")) {
                    if (call.arguments.items.len != 0) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const offset = runtime_functions.runtime_get_timezone_offset();
                    return Value{ .Integer = offset };
                } else if (std.mem.eql(u8, name, "runtime_get_timezone_name")) {
                    if (call.arguments.items.len != 0) {
                        return InterpreterError.TypeMismatch;
                    }
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const name_ptr = runtime_functions.runtime_get_timezone_name();
                    const name_slice = std.mem.span(name_ptr);
                    const name_copy = try self.allocator.dupe(u8, name_slice);
                    return Value{ .String = name_copy };
                } else if (std.mem.eql(u8, name, "runtime_to_lowercase")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const str_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const input_str = try str_arg.toString(self.allocator);
                    defer self.allocator.free(input_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_to_lowercase(self.allocator, input_str) catch |err| switch (err) {
                        error.OutOfMemory => return InterpreterError.OutOfMemory,
                    };
                    
                    return Value{ .String = result };
                } else if (std.mem.eql(u8, name, "runtime_split_path")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const path_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const path_str = try path_arg.toString(self.allocator);
                    defer self.allocator.free(path_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_split_path(self.allocator, path_str) catch |err| switch (err) {
                        error.OutOfMemory => return InterpreterError.OutOfMemory,

        };
                    
                    // Convert ArrayList([]const u8) to ArrayList(Value)
                    var path_values = ArrayList(Value){};
                    for (result.items) |path_str_item| {
                        try path_values.append(self.allocator, Value{ .String = path_str_item });
                    }
                    
                    return Value{ .Array = try path_values.toOwnedSlice(self.allocator) };
                } else if (std.mem.eql(u8, name, "runtime_parse_int")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const str_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const input_str = try str_arg.toString(self.allocator);
                    defer self.allocator.free(input_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_parse_int(self.allocator, input_str) catch {
                        // Create array with 0 and error message
                        var result_array = ArrayList(Value){};
                        try result_array.append(self.allocator, Value{ .Integer = 0 });
                        try result_array.append(self.allocator, Value{ .String = try self.allocator.dupe(u8, "Failed to parse integer") });
                        return Value{ .Array = result_array };
                    };
                    
                    // Create array with integer and error string
                    var result_array = ArrayList(Value){};
                    try result_array.append(self.allocator, Value{ .Integer = result[0] });
                    try result_array.append(self.allocator, Value{ .String = try self.allocator.dupe(u8, result[1]) });
                    return Value{ .Array = try result_array.toOwnedSlice(self.allocator) };
                } else if (std.mem.eql(u8, name, "runtime_string_length")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const str_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const input_str = try str_arg.toString(self.allocator);
                    defer self.allocator.free(input_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_string_length(input_str);
                    return Value{ .Integer = result };
                } else if (std.mem.eql(u8, name, "runtime_read_file")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const filename_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const filename_str = try filename_arg.toString(self.allocator);
                    defer self.allocator.free(filename_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_read_file(self.allocator, filename_str) catch |err| switch (err) {
                        error.FileNotFound => "ERROR",
                        error.AccessDenied => "ERROR",
                        error.OutOfMemory => return InterpreterError.OutOfMemory,
                        else => "ERROR",
                    };
                    return Value{ .String = result };
                } else if (std.mem.eql(u8, name, "runtime_write_file")) {
                    std.debug.print("DEBUG: Intercepted runtime_write_file call\n", .{});
                    if (call.arguments.items.len != 2) {
                        return InterpreterError.TypeMismatch;
                    }
                    const filename_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const content_arg = try self.evaluateExpression(call.arguments.items[1].*);
                    const filename_str = try filename_arg.toString(self.allocator);
                    const content_str = try content_arg.toString(self.allocator);
                    defer self.allocator.free(filename_str);
                    defer self.allocator.free(content_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_write_file(filename_str, content_str) catch false;
                    return Value{ .Boolean = result };
                } else if (std.mem.eql(u8, name, "runtime_append_file")) {
                    if (call.arguments.items.len != 2) {
                        return InterpreterError.TypeMismatch;
                    }
                    const filename_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const content_arg = try self.evaluateExpression(call.arguments.items[1].*);
                    const filename_str = try filename_arg.toString(self.allocator);
                    const content_str = try content_arg.toString(self.allocator);
                    defer self.allocator.free(filename_str);
                    defer self.allocator.free(content_str);
                    
                    const runtime_functions = @import("runtime_functions.zig");
                    const result = runtime_functions.runtime_append_file(filename_str, content_str) catch false;
                    return Value{ .Boolean = result };
                } else if (std.mem.eql(u8, name, "runtime_file_exists")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const filename_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const filename_str = try filename_arg.toString(self.allocator);
                    defer self.allocator.free(filename_str);
                    
                    std.fs.cwd().access(filename_str, .{}) catch return Value{ .Boolean = false };
                    return Value{ .Boolean = true };
                } else if (std.mem.eql(u8, name, "runtime_file_size")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const filename_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const filename_str = try filename_arg.toString(self.allocator);
                    defer self.allocator.free(filename_str);
                    
                    const file = std.fs.cwd().openFile(filename_str, .{}) catch return Value{ .Integer = -1 };
                    defer file.close();
                    const stat = file.stat() catch return Value{ .Integer = -1 };
                    return Value{ .Integer = @intCast(stat.size) };
                } else if (std.mem.eql(u8, name, "runtime_delete_file")) {
                    if (call.arguments.items.len != 1) {
                        return InterpreterError.TypeMismatch;
                    }
                    const filename_arg = try self.evaluateExpression(call.arguments.items[0].*);
                    const filename_str = try filename_arg.toString(self.allocator);
                    defer self.allocator.free(filename_str);
                    
                    const result = blk: {
                        std.fs.cwd().deleteFile(filename_str) catch break :blk false;
                        break :blk true;
                    };
                    return Value{ .Boolean = result != false };
                } else {
                    // Try to find function directly first
                    if (self.functions.get(name)) |func| {
                        std.debug.print("DEBUG: Calling user function '{s}'\n", .{name});
                        // Evaluate arguments
                        var args = std.ArrayList(Value){};
                        defer args.deinit(self.allocator);
                        errdefer args.deinit(self.allocator); // Clean up on error
                        
                        for (call.arguments.items) |arg_expr| {
                            const arg = try self.evaluateExpression(arg_expr.*);
                            try args.append(self.allocator, arg);
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
            else => {}
        }
        
        return InterpreterError.UndefinedFunction;
    }

    fn evaluateMemberAccess(self: *Interpreter, member: ast.MemberAccessExpression) InterpreterError!Value {
        const object = try self.evaluateExpression(member.object.*);
        
        switch (object) {
            .Struct => |struct_inst| {
                if (struct_inst.fields.get(member.property)) |field_value| {
                    std.debug.print("DEBUG: Found field '{s}' with value type: {s}\n", .{member.property, @tagName(field_value.*)});
                    return field_value.*;
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
            .Pointer => |ptr| {
                // Automatically dereference pointer for member access
                switch (ptr.pointee_value.*) {
                    .Struct => |struct_inst| {
                        if (struct_inst.fields.get(member.property)) |field_value| {
                        std.debug.print("DEBUG: Found field '{s}' via pointer dereference with value type: {s}\n", .{member.property, @tagName(field_value.*) });
                        return field_value.*;
                        } else {
                            std.debug.print("DEBUG: Field '{s}' not found in dereferenced struct\n", .{member.property});
                            return InterpreterError.UndefinedField;
                        }
                    },
                    else => {
                        std.debug.print("DEBUG: Member access on pointer to non-struct: {s}\n", .{@tagName(ptr.pointee_value.*) });
                        return InterpreterError.TypeMismatch;
                    }
                }
            },
            else => {
                std.debug.print("DEBUG: Member access on non-struct type: {s}\n", .{@tagName(object)});
                return InterpreterError.TypeMismatch;
            }
        }
    }

    fn evaluateMethodCall(self: *Interpreter, member: ast.MemberAccessExpression, args: []*ast.Expression) InterpreterError!Value {
        std.debug.print("DEBUG: Method call - evaluating object for '{s}' method\n", .{member.property});
        std.debug.print("DEBUG: About to evaluate expression for object...\n", .{});
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
                            var method_args = std.ArrayList(Value){};
                            defer method_args.deinit(self.allocator);
                            errdefer method_args.deinit(self.allocator); // Clean up on error
                            
                            // First argument is self (the struct instance)
                            try method_args.append(self.allocator, Value{ .Struct = struct_inst });
                            
                            // Add other arguments
                            for (args) |arg_expr| {
                                const arg_val = try self.evaluateExpression(arg_expr.*);
                                try method_args.append(self.allocator, arg_val);
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
                var method_args = std.ArrayList(Value){};
                defer method_args.deinit(self.allocator);
                errdefer method_args.deinit(self.allocator); // Clean up on error
                
                // First argument is self (the underlying struct)
                try method_args.append(self.allocator, Value{ .Struct = interface_inst.underlying_struct.* });
                
                for (args) |arg_expr| {
                    const arg_val = try self.evaluateExpression(arg_expr.*);
                    try method_args.append(self.allocator, arg_val);
                }
                
                // Get method from vtable and execute it
                if (interface_inst.vtable.getMethod(member.property)) |method_func| {
                    std.debug.print("DEBUG: Found interface method '{s}' in vtable, executing...\n", .{member.property});
                    return try self.executeInterfaceMethod(method_func, method_args.items);
                } else {
                    std.debug.print("DEBUG: Method '{s}' not found in interface vtable\n", .{member.property});
                    return InterpreterError.UndefinedMethod;
                }
            },
            .Module => |module_ptr| {
                // Look for function in module
                if (module_ptr.functions.get(member.property)) |func_value| {
                    std.debug.print("DEBUG: Found function '{s}' in module\n", .{member.property});
                    
                    switch (func_value) {
                        .BuiltinFunction => |builtin_func| {
                            // Call the builtin function
                            var func_args = ArrayList(Value){};
                            defer func_args.deinit(self.allocator);
                            
                            // Evaluate arguments
                            for (args) |arg_expr| {
                                const arg_val = try self.evaluateExpression(arg_expr.*);
                                try func_args.append(self.allocator, arg_val);
                            }
                            
                            // Call the builtin function
                            return try builtin_func.func(self, func_args.items);
                        },
                        .UserFunction => |user_func| {
                            // Call the user-defined function from stdlib
                            var func_args = ArrayList(Value){};
                            defer func_args.deinit(self.allocator);
                            
                            // Evaluate arguments
                            for (args) |arg_expr| {
                                const arg_val = try self.evaluateExpression(arg_expr.*);
                                try func_args.append(self.allocator, arg_val);
                            }
                            
                            // Call the user function
                            return try self.callFunction(user_func, func_args.items);
                        },
                        else => {
                            std.debug.print("DEBUG: Module member '{s}' is not a function\n", .{member.property});
                            return InterpreterError.TypeMismatch;
                        }
                    }
                } else {
                    std.debug.print("DEBUG: Function '{s}' not found in module\n", .{member.property});
                    return InterpreterError.UndefinedFunction;
                }
            },
            else => {
                return InterpreterError.TypeMismatch;
            }
        }
    }

    fn executeInterfaceMethod(self: *Interpreter, method_func: *FunctionValue, args: []Value) InterpreterError!Value {
        // Create new environment for method execution on heap
        const method_env = try Environment.newEnvironment(self.allocator, self.environment);
        // Note: Not deinitialized here as environment may escape via closures
        
        // Bind parameters to arguments
        if (args.len != method_func.parameters.len + 1) { // +1 for 'self'
            std.debug.print("ERROR: Interface method '{s}' expects {d} params but got {d} args\n", .{method_func.name, method_func.parameters.len, args.len - 1});
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
        self.environment = method_env;
        defer self.environment = previous_env;
        
        for (method_func.body) |stmt| {
            self.executeStatement(stmt) catch |err| {
                return err;
            };
        }
        
        return Value.Null;
    }

    fn executeMethodBody(self: *Interpreter, method: ast.FunctionStatement, args: []Value) InterpreterError!Value {
        // Create new environment for method execution on heap
        const method_env = try Environment.newEnvironment(self.allocator, self.environment);
        // Note: Not deinitialized here as environment may escape via closures
        
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
        self.environment = method_env;
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
        // CRITICAL FIX: Check for generic struct syntax first
        if (try self.resolveGenericStructLiteral(struct_lit)) |generic_value| {
            return generic_value;
        }
        
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
    fn resolveGenericFunctionCall(self: *Interpreter, function_name: []const u8, arguments: []*ast.Expression) InterpreterError!?Value {
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
        var args = std.ArrayList(Value){};
        defer args.deinit(self.allocator);
        errdefer args.deinit(self.allocator); // Clean up on error
        
        for (arguments) |arg_expr| {
            const arg_value = try self.evaluateExpression(arg_expr.*);
            try args.append(self.allocator, arg_value);
        }
        
        std.debug.print("DEBUG: Calling generic function '{s}' with {d} arguments\n", 
            .{function_name, args.items.len});
        
        // CRITICAL FIX: Create monomorphized (specialized) function instance
        const specialized_func = try self.createSpecializedFunction(template_func, call_info.type_args);
        defer self.destroySpecializedFunction(specialized_func);
        
        std.debug.print("DEBUG: Created specialized function instance for types: {any}\n", .{call_info.type_args});
        
        // Call the specialized function instead of the template
        return try self.callFunction(specialized_func, args.items);
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
        var type_args = std.ArrayList([]const u8){};
        defer type_args.deinit(self.allocator);
        
        var iterator = std.mem.splitScalar(u8, type_args_str, ',');
        while (iterator.next()) |type_arg| {
            const trimmed = std.mem.trim(u8, type_arg, " \t\n");
            if (trimmed.len > 0) {
                try type_args.append(self.allocator, try self.allocator.dupe(u8, trimmed));
            }
        }
        
        return type_args.toOwnedSlice(self.allocator);
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
    
    /// CRITICAL FIX: Create specialized function with type parameter substitution
    fn createSpecializedFunction(self: *Interpreter, template_func: CursedFunction, type_args: [][]const u8) !CursedFunction {
        std.debug.print("DEBUG: Starting function specialization for types: {any}\n", .{type_args});
        
        // Create type parameter mapping
        var type_substitutions = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer type_substitutions.deinit();
        
        for (template_func.declaration.type_parameters.items, 0..) |type_param, i| {
            try type_substitutions.put(type_param.name, type_args[i]);
            std.debug.print("DEBUG: Type substitution: {s} -> {s}\n", .{type_param.name, type_args[i]});
        }
        
        // Clone the template function declaration
        var specialized_decl = ast.FunctionStatement{
            .name = try std.fmt.allocPrint(self.allocator, "{s}_specialized", .{template_func.declaration.name}),
            .parameters = .empty,
            .return_type = template_func.declaration.return_type, // Clone the return type
            .body = .empty,
            .visibility = .Public,
            .is_async = template_func.declaration.is_async,
            .type_parameters = .empty, // Empty for specialized function
            .comments = .empty,
        };
        
        // Clone parameters with type substitution
        for (template_func.declaration.parameters.items) |param| {
            const substituted_type = try self.substituteTypeInParameter(param.param_type, &type_substitutions);
            try specialized_decl.parameters.append(self.allocator, ast.Parameter{
                .name = param.name,
                .param_type = substituted_type,
                .is_mutable = param.is_mutable,
                .default_value = param.default_value,
            });
            std.debug.print("DEBUG: Parameter '{s}': original type substituted\n", .{param.name});
        }
        
        // Clone function body with type substitution  
        for (template_func.declaration.body.items) |stmt| {
            const specialized_stmt = try self.substituteTypesInStatement(stmt.*, &type_substitutions);
            const heap_stmt = try self.allocator.create(ast.Statement);
            heap_stmt.* = specialized_stmt;
            try specialized_decl.body.append(self.allocator, heap_stmt);
        }
        
        std.debug.print("DEBUG: Function body specialized with {d} statements\n", .{specialized_decl.body.items.len});
        
        // Create specialized function
        return CursedFunction{
            .declaration = specialized_decl,
            .closure = template_func.closure,
        };
    }
    
    /// Cleanup specialized function
    fn destroySpecializedFunction(self: *Interpreter, func: CursedFunction) void {
        // Free the specialized function name
        self.allocator.free(func.declaration.name);
        
        // Free parameters - skip deinit to avoid const cast issue
        // func.declaration.parameters.deinit(self.allocator);
        
        // Free body statements
        for (func.declaration.body.items) |stmt| {
            self.allocator.destroy(stmt);
        }
        // func.declaration.body.deinit(self.allocator); // Skip to avoid const issues
        
        // Free type parameters (should be empty)
        // func.declaration.type_parameters.deinit(self.allocator); // Skip to avoid const issues
        
        std.debug.print("DEBUG: Cleaned up specialized function\n", .{});
    }
    
    /// Substitute types in a parameter
    fn substituteTypeInParameter(self: *Interpreter, original_type: ast.Type, substitutions: *HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !ast.Type {
        _ = self;
        return switch (original_type) {
            .Custom => |type_name| {
                if (substitutions.get(type_name)) |concrete_type| {
                    // Map type parameter to concrete type
                    if (std.mem.eql(u8, concrete_type, "tea")) return ast.Type{ .Basic = .Tea };
                    if (std.mem.eql(u8, concrete_type, "drip")) return ast.Type{ .Basic = .Drip };
                    if (std.mem.eql(u8, concrete_type, "normie")) return ast.Type{ .Basic = .Normie };
                    if (std.mem.eql(u8, concrete_type, "lit")) return ast.Type{ .Basic = .Lit };
                    if (std.mem.eql(u8, concrete_type, "smol")) return ast.Type{ .Basic = .Smol };
                    if (std.mem.eql(u8, concrete_type, "thicc")) return ast.Type{ .Basic = .Thicc };
                    if (std.mem.eql(u8, concrete_type, "meal")) return ast.Type{ .Basic = .Meal };
                    if (std.mem.eql(u8, concrete_type, "snack")) return ast.Type{ .Basic = .Snack };
                    if (std.mem.eql(u8, concrete_type, "vibes")) return ast.Type{ .Custom = "vibes" };
                    
                    // If not a primitive, keep as identifier
                    return ast.Type{ .Custom = concrete_type };
                } else {
                    return original_type;
                }
            },
            else => original_type, // Pass through other types unchanged
        };
    }
    
    /// Substitute types in a statement (basic implementation)  
    fn substituteTypesInStatement(self: *Interpreter, original_stmt: ast.Statement, substitutions: *HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !ast.Statement {
        _ = substitutions; // For now, we'll just clone statements
        _ = self;
        
        // For basic monomorphization, we can return the statement as-is
        // since type substitution mainly affects the parameter and return types
        // which we handle in createSpecializedFunction
        return original_stmt;
    }
    
    /// CRITICAL FIX: Resolve generic struct literal
    fn resolveGenericStructLiteral(self: *Interpreter, struct_lit: ast.StructLiteralExpression) InterpreterError!?Value {
        // Parse generic struct syntax: StructName[Type1, Type2]
        const generic_info = self.parseGenericCallSyntax(struct_lit.struct_name) catch return null;
        if (generic_info == null) return null;
        
        const call_info = generic_info.?;
        defer self.allocator.free(call_info.base_name);
        defer {
            for (call_info.type_args) |arg| {
                self.allocator.free(arg);
            }
            self.allocator.free(call_info.type_args);
        }
        
        std.debug.print("DEBUG: Parsing generic struct '{s}' -> base: '{s}', type_args: {any}\n", 
            .{struct_lit.struct_name, call_info.base_name, call_info.type_args});
        
        // Find the generic template struct
        const template_struct = self.findGenericStructTemplate(call_info.base_name) orelse {
            std.debug.print("DEBUG: No generic struct template found for '{s}'\n", .{call_info.base_name});
            return null;
        };
        
        std.debug.print("DEBUG: Found generic struct template '{s}' with {d} type parameters\n", 
            .{template_struct.name, template_struct.type_parameters.items.len});
        
        // Validate type argument count
        if (call_info.type_args.len != template_struct.type_parameters.items.len) {
            std.debug.print("DEBUG: Type argument count mismatch: expected {d}, got {d}\n", 
                .{template_struct.type_parameters.items.len, call_info.type_args.len});
            return InterpreterError.TypeMismatch;
        }
        
        // Create specialized struct instance
        const specialized_struct_name = try std.fmt.allocPrint(self.allocator, "{s}_specialized", .{call_info.base_name});
        defer self.allocator.free(specialized_struct_name);
        
        std.debug.print("DEBUG: Creating specialized struct instance '{s}'\n", .{specialized_struct_name});
        
        // Create new struct instance with specialized name
        var struct_instance = try StructInstance.init(self.allocator, specialized_struct_name);
        
        // Initialize fields from literal
        for (struct_lit.fields.items) |field_assignment| {
            const field_value = try self.evaluateExpression(field_assignment.value.*);
            try struct_instance.setField(field_assignment.field_name, field_value);
            std.debug.print("DEBUG: Set field '{s}' in specialized struct\n", .{field_assignment.field_name});
        }
        
        return Value{ .Struct = struct_instance };
    }
    
    /// Find generic struct template by base name
    fn findGenericStructTemplate(self: *Interpreter, base_name: []const u8) ?*const ast.StructStatement {
        // Look for generic structs in type registry
        var iterator = self.type_registry.struct_types.iterator();
        while (iterator.next()) |entry| {
            const struct_name = entry.key_ptr.*;
            const struct_def = entry.value_ptr.*;
            
            // Check if this is a generic struct template matching the base name
            if (std.mem.eql(u8, struct_name, base_name) and 
                struct_def.type_parameters.items.len > 0) {
                return &struct_def;
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
        
        // Create new environment for function execution on heap
        const function_env = try Environment.newEnvironment(self.allocator, func.closure);
        // Note: Not deinitialized here as environment may escape via closures
        
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
        self.environment = function_env;
        defer {
            // Execute defers for this function scope in LIFO order
            self.executeDeferToSize(defer_stack_size_at_entry);
            self.environment = previous_env;
        }
        
        var return_value: Value = Value.Null;
        var has_returned = false;
        
        // Track multiple return values for tuple returns
        var return_values = std.ArrayList(Value){};
        defer return_values.deinit(self.allocator);
        errdefer return_values.deinit(self.allocator); // Clean up on error
        
        for (func.declaration.body.items) |stmt| {
            switch (stmt.*) {
                .Return => |ret| {
                    if (ret.value) |value| {
                        const expr: *ast.Expression = @ptrCast(@alignCast(value));
                        const result = try self.evaluateExpression(expr.*);
                        // Check if this is a tuple expression (multiple values)
                        switch (result) {
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

    fn compareValues(self: *Interpreter, left: *Value, right: *Value) bool {
        return self.valuesEqual(left.*, right.*);
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
            .Struct => |left_struct| {
                switch (right) {
                    .Struct => |right_struct| {
                        // Compare struct types and field values
                        if (!std.mem.eql(u8, left_struct.type_name, right_struct.type_name)) {
                            return false;
                        }
                        
                        // Compare field values
                        var left_iter = left_struct.fields.iterator();
                        while (left_iter.next()) |left_entry| {
                            const field_name = left_entry.key_ptr.*;
                            const left_value = left_entry.value_ptr.*;
                            
                            if (right_struct.fields.get(field_name)) |right_value| {
                                if (!self.compareValues(left_value, right_value)) {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                        
                        return left_struct.fields.count() == right_struct.fields.count();
                    },
                    else => return false,
                }
            },
            .Interface => |left_interface| {
                switch (right) {
                    .Interface => |right_interface| {
                        // Compare interface types - interfaces are equal if they have the same type name
                        return std.mem.eql(u8, left_interface.vtable.interface_name, right_interface.vtable.interface_name);
                    },
                    else => return false,
                }
            },
            .Error => |left_error| {
                switch (right) {
                    .Error => |right_error| {
                        // Compare error messages
                        return std.mem.eql(u8, left_error.message, right_error.message);
                    },
                    else => return false,
                }
            },
            .CursedError => |_| {
                switch (right) {
                    .CursedError => |right_err| {
                        const left_err = left.CursedError;
                        return std.mem.eql(u8, left_err.message, right_err.message) and 
                               left_err.error_type == right_err.error_type;
                    },
                    else => return false,
                }
            },
            .Pointer => |left_ptr| {
                switch (right) {
                    .Pointer => |right_ptr| {
                        // Compare pointer addresses
                        return @intFromPtr(left_ptr.pointee_value) == @intFromPtr(right_ptr.pointee_value);
                    },
                    else => return false,
                }
            },
            .Array => |left_array| {
                switch (right) {
                    .Array => |right_array| {
                        // Compare array lengths first
                        if (left_array.len != right_array.len) return false;
                        
                        // Compare elements
                        for (left_array, 0..) |left_element, i| {
                            if (!self.valuesEqual(left_element, right_array[i])) return false;
                        }
                        return true;
                    },
                    else => return false,
                }
            },
            .Module => |left_module_ptr| {
                switch (right) {
                    .Module => |right_module_ptr| {
                        // Compare pointers - same module if same pointer
                        return left_module_ptr == right_module_ptr;
                    },
                    else => return false,
                }
            },
            .BuiltinFunction => |left_builtin| {
                switch (right) {
                    .BuiltinFunction => |right_builtin| {
                        return std.mem.eql(u8, left_builtin.name, right_builtin.name);
                    },
                    else => return false,
                }
            },
            .UserFunction => |left_func| {
                switch (right) {
                    .UserFunction => |right_func| {
                        return std.mem.eql(u8, left_func.declaration.name, right_func.declaration.name);
                    },
                    else => return false,
                }
            }
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
            std.debug.print("Error formatting context: {s}\n", .{error_ctx.message});
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
                    .file = if (self.current_file) |file| file else "unknown",
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
                std.debug.print("Unhandled error in fam block: {s}\n", .{"unknown"});
                std.debug.print("Error context: {s}\n", .{error_ctx.message});
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
            std.debug.print("Unhandled error after fam block: {s}\n", .{unhandled_error.message});
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
            if (self.current_line) |line| line else 0, // Get current line number
            if (self.current_column) |col| col else 0  // Get current column
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
        
        std.debug.print("Spawned goroutine with ID: {d}\n", .{goroutine_id});
        
        // Wait a bit longer for goroutine to execute
        std.Thread.sleep(10_000_000); // 10ms
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
            .Array => |_| {
                // TODO: Fix when Tuple is properly implemented
                return false; // Temporary fix
                // if (value != .Tuple) return false;
                // const array_val = value.Tuple;

            },
            .Guard => |guard| {
                // First check if the base pattern matches
                if (!try self.matchPattern(guard.pattern.*, value)) {
                    return false;
                }
                
                // Then evaluate the guard condition
                const guard_result = try self.evaluateExpression(guard.guard.*);
                return switch (guard_result) {
                    .Boolean => |b| b,
                    else => false, // Non-boolean guard conditions are false
                };
            },
            else => {
                std.debug.print("Unsupported pattern type: {s}\n", .{@tagName(pattern)});
                return false;
            }
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
        try self.defer_stack.append(self.allocator, defer_entry);
        
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
                std.debug.print("Shook propagated error: {s}\n", .{"unknown"});
                std.debug.print("Error context: {s}\n", .{error_ctx.message});
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
            }
        }
    }

    // Enhanced channel simulation methods
    fn storeChannelValue(self: *Interpreter, channel_id: u64, value: Value) InterpreterError!void {
        if (self.channel_storage.getPtr(channel_id)) |channel_list| {
            try channel_list.append(self.allocator, value);
        } else {
            var new_list = std.ArrayList(Value){};
            errdefer new_list.deinit(self.allocator); // Clean up on error
            try new_list.append(self.allocator, value);
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
        var tuple_values = std.ArrayList(Value){};
        errdefer tuple_values.deinit(self.allocator); // Clean up on error
        
        for (tuple.elements.items) |element_expr| {
            const element_value = try self.evaluateExpression(element_expr.*);
            try tuple_values.append(self.allocator, element_value);
        }
        
        // TODO: Implement proper Tuple support
        return Value.Null; // Temporary fix
    }

    fn evaluateStringInterpolation(self: *Interpreter, interpolation: ast.StringInterpolationExpression) InterpreterError!Value {
        var result = std.ArrayList(u8){};
        defer result.deinit(self.allocator);
        
        for (interpolation.parts.items) |part| {
            if (part.expression) |expr_ptr| {
                // Evaluate expression and convert to string
                const expr: *Expression = @ptrCast(@alignCast(expr_ptr));
                const value = try self.evaluateExpression(expr.*);
                const str_value = try value.toString(self.allocator);
                defer self.allocator.free(str_value);
                try result.appendSlice(self.allocator, str_value);
            } else {
                // Literal text part
                try result.appendSlice(self.allocator, part.text);
            }
        }
        
        const final_string = try self.allocator.dupe(u8, result.items);
        return Value{ .String = final_string };
    }
};

// ===== BUILTIN STDLIB FUNCTIONS =====

// Use standard debug print for now - will be replaced with proper runtime bridge later

fn builtinVibezSpill(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter; // Mark as unused to avoid warnings
    if (args.len != 1) {
        return InterpreterError.InvalidArgumentCount;
    }
    
    const msg_value = args[0];
    switch (msg_value) {
        .String => |str| {
            std.debug.print("{s}", .{str});
            return Value{ .Boolean = true };
        },
        else => {
            return InterpreterError.TypeMismatch;
        }
    }
}

fn builtinVibezSpillln(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter; // Mark as unused to avoid warnings
    if (args.len != 1) {
        return InterpreterError.InvalidArgumentCount;
    }
    
    const msg_value = args[0];
    switch (msg_value) {
        .String => |str| {
            std.debug.print("{s}\n", .{str});
            return Value{ .Boolean = true };
        },
        else => {
            return InterpreterError.TypeMismatch;
        }
    }
}

fn builtinVibezPrintSeparator(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    _ = args;
    
    std.debug.print("--------------------------------\n", .{});
    return Value{ .Boolean = true };
}

fn builtinMathzAbs(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 1) {
        return InterpreterError.InvalidArgumentCount;
    }
    
    const val = args[0];
    switch (val) {
        .Float => |f| return Value{ .Float = if (f < 0) -f else f },
        .Integer => |i| return Value{ .Integer = if (i < 0) -i else i },
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinMathzMax(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 2) {
        return InterpreterError.InvalidArgumentCount;
    }
    
    const a = args[0];
    const b = args[1];
    
    switch (a) {
        .Float => |a_f| switch (b) {
            .Float => |b_f| return Value{ .Float = if (a_f > b_f) a_f else b_f },
            .Integer => |b_i| {
                const b_f = @as(f64, @floatFromInt(b_i));
                return Value{ .Float = if (a_f > b_f) a_f else b_f };
            },
            else => return InterpreterError.TypeMismatch,
        },
        .Integer => |a_i| switch (b) {
            .Integer => |b_i| return Value{ .Integer = if (a_i > b_i) a_i else b_i },
            .Float => |b_f| {
                const a_f = @as(f64, @floatFromInt(a_i));
                return Value{ .Float = if (a_f > b_f) a_f else b_f };
            },
            else => return InterpreterError.TypeMismatch,
        },
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinMathzMin(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 2) {
        return InterpreterError.InvalidArgumentCount;
    }
    
    const a = args[0];
    const b = args[1];
    
    switch (a) {
        .Float => |a_f| switch (b) {
            .Float => |b_f| return Value{ .Float = if (a_f < b_f) a_f else b_f },
            .Integer => |b_i| {
                const b_f = @as(f64, @floatFromInt(b_i));
                return Value{ .Float = if (a_f < b_f) a_f else b_f };
            },
            else => return InterpreterError.TypeMismatch,
        },
        .Integer => |a_i| switch (b) {
            .Integer => |b_i| return Value{ .Integer = if (a_i < b_i) a_i else b_i },
            .Float => |b_f| {
                const a_f = @as(f64, @floatFromInt(a_i));
                return Value{ .Float = if (a_f < b_f) a_f else b_f };
            },
            else => return InterpreterError.TypeMismatch,
        },
        else => return InterpreterError.TypeMismatch,
    }
}

// Additional mathz functions
fn builtinMathzAdd(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 2) return InterpreterError.InvalidArgumentCount;
    
    const a = args[0];
    const b = args[1];
    
    switch (a) {
        .Float => |a_f| switch (b) {
            .Float => |b_f| return Value{ .Float = a_f + b_f },
            .Integer => |b_i| return Value{ .Float = a_f + @as(f64, @floatFromInt(b_i)) },
            else => return InterpreterError.TypeMismatch,
        },
        .Integer => |a_i| switch (b) {
            .Integer => |b_i| return Value{ .Integer = a_i + b_i },
            .Float => |b_f| return Value{ .Float = @as(f64, @floatFromInt(a_i)) + b_f },
            else => return InterpreterError.TypeMismatch,
        },
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinMathzSub(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 2) return InterpreterError.InvalidArgumentCount;
    
    const a = args[0];
    const b = args[1];
    
    switch (a) {
        .Float => |a_f| switch (b) {
            .Float => |b_f| return Value{ .Float = a_f - b_f },
            .Integer => |b_i| return Value{ .Float = a_f - @as(f64, @floatFromInt(b_i)) },
            else => return InterpreterError.TypeMismatch,
        },
        .Integer => |a_i| switch (b) {
            .Integer => |b_i| return Value{ .Integer = a_i - b_i },
            .Float => |b_f| return Value{ .Float = @as(f64, @floatFromInt(a_i)) - b_f },
            else => return InterpreterError.TypeMismatch,
        },
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinMathzMul(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 2) return InterpreterError.InvalidArgumentCount;
    
    const a = args[0];
    const b = args[1];
    
    switch (a) {
        .Float => |a_f| switch (b) {
            .Float => |b_f| return Value{ .Float = a_f * b_f },
            .Integer => |b_i| return Value{ .Float = a_f * @as(f64, @floatFromInt(b_i)) },
            else => return InterpreterError.TypeMismatch,
        },
        .Integer => |a_i| switch (b) {
            .Integer => |b_i| return Value{ .Integer = a_i * b_i },
            .Float => |b_f| return Value{ .Float = @as(f64, @floatFromInt(a_i)) * b_f },
            else => return InterpreterError.TypeMismatch,
        },
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinMathzDiv(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 2) return InterpreterError.InvalidArgumentCount;
    
    const a = args[0];
    const b = args[1];
    
    switch (a) {
        .Float => |a_f| switch (b) {
            .Float => |b_f| {
                if (b_f == 0.0) return InterpreterError.DivisionByZero;
                return Value{ .Float = a_f / b_f };
            },
            .Integer => |b_i| {
                if (b_i == 0) return InterpreterError.DivisionByZero;
                return Value{ .Float = a_f / @as(f64, @floatFromInt(b_i)) };
            },
            else => return InterpreterError.TypeMismatch,
        },
        .Integer => |a_i| switch (b) {
            .Integer => |b_i| {
                if (b_i == 0) return InterpreterError.DivisionByZero;
                return Value{ .Float = @as(f64, @floatFromInt(a_i)) / @as(f64, @floatFromInt(b_i)) };
            },
            .Float => |b_f| {
                if (b_f == 0.0) return InterpreterError.DivisionByZero;
                return Value{ .Float = @as(f64, @floatFromInt(a_i)) / b_f };
            },
            else => return InterpreterError.TypeMismatch,
        },
        else => return InterpreterError.TypeMismatch,
    }
}

// stringz functions
fn builtinStringzLength(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 1) return InterpreterError.InvalidArgumentCount;
    
    const str_value = args[0];
    switch (str_value) {
        .String => |str| return Value{ .Integer = @as(i64, @intCast(str.len)) },
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinStringzConcat(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    if (args.len != 2) return InterpreterError.InvalidArgumentCount;
    
    const a = args[0];
    const b = args[1];
    
    switch (a) {
        .String => |str_a| switch (b) {
            .String => |str_b| {
                const result = interpreter.allocator.alloc(u8, str_a.len + str_b.len) catch {
                    return InterpreterError.OutOfMemory;
                };
                @memcpy(result[0..str_a.len], str_a);
                @memcpy(result[str_a.len..], str_b);
                return Value{ .String = result };
            },
            else => return InterpreterError.TypeMismatch,
        },
        else => return InterpreterError.TypeMismatch,
    }
}

// New mathz functions
fn builtinMathzPow(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 2) return InterpreterError.InvalidArgumentCount;
    
    const base = args[0];
    const exponent = args[1];
    
    switch (base) {
        .Float => |base_f| switch (exponent) {
            .Float => |exp_f| return Value{ .Float = std.math.pow(f64, base_f, exp_f) },
            .Integer => |exp_i| return Value{ .Float = std.math.pow(f64, base_f, @as(f64, @floatFromInt(exp_i))) },
            else => return InterpreterError.TypeMismatch,
        },
        .Integer => |base_i| switch (exponent) {
            .Integer => |exp_i| return Value{ .Float = std.math.pow(f64, @as(f64, @floatFromInt(base_i)), @as(f64, @floatFromInt(exp_i))) },
            .Float => |exp_f| return Value{ .Float = std.math.pow(f64, @as(f64, @floatFromInt(base_i)), exp_f) },
            else => return InterpreterError.TypeMismatch,
        },
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinMathzSqrt(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 1) return InterpreterError.InvalidArgumentCount;
    
    const val = args[0];
    switch (val) {
        .Float => |f| {
            if (f < 0.0) return InterpreterError.InvalidOperation;
            return Value{ .Float = std.math.sqrt(f) };
        },
        .Integer => |i| {
            if (i < 0) return InterpreterError.InvalidOperation;
            return Value{ .Float = std.math.sqrt(@as(f64, @floatFromInt(i))) };
        },
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinMathzFloor(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 1) return InterpreterError.InvalidArgumentCount;
    
    const val = args[0];
    switch (val) {
        .Float => |f| return Value{ .Integer = @as(i64, @intFromFloat(std.math.floor(f))) },
        .Integer => |i| return Value{ .Integer = i }, // Floor of integer is itself
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinMathzCeil(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 1) return InterpreterError.InvalidArgumentCount;
    
    const val = args[0];
    switch (val) {
        .Float => |f| return Value{ .Integer = @as(i64, @intFromFloat(std.math.ceil(f))) },
        .Integer => |i| return Value{ .Integer = i }, // Ceiling of integer is itself
        else => return InterpreterError.TypeMismatch,
    }
}

fn builtinMathzRound(interpreter: *Interpreter, args: []Value) InterpreterError!Value {
    _ = interpreter;
    if (args.len != 1) return InterpreterError.InvalidArgumentCount;
    
    const val = args[0];
    switch (val) {
        .Float => |f| return Value{ .Integer = @as(i64, @intFromFloat(std.math.round(f))) },
        .Integer => |i| return Value{ .Integer = i }, // Round of integer is itself
        else => return InterpreterError.TypeMismatch,
    }
}

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
