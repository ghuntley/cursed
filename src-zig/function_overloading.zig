const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Function Overloading Implementation (Priority #20)
// Multiple signatures for same function name with proper resolution

pub const OverloadingError = error{
    NoMatchingOverload,
    AmbiguousOverload,
    DuplicateOverload,
    InvalidSignature,
    TypeMismatch,
    ArityMismatch,
};

pub const FunctionType = enum {
    Void,      // No return type
    Normie,    // int
    Tea,       // string  
    Drip,      // int64
    Lit,       // bool
    Meal,      // float64
    Smol,      // int8
    Thicc,     // int32
    Sip,       // float32
    Array,     // []T
    Struct,    // Custom struct
    Interface, // Interface type
    Generic,   // Generic type parameter
};

pub const Parameter = struct {
    name: []const u8,
    param_type: FunctionType,
    is_variadic: bool = false,
    is_optional: bool = false,
    
    pub fn init(name: []const u8, param_type: FunctionType) Parameter {
        return Parameter{
            .name = name,
            .param_type = param_type,
        };
    }
    
    pub fn matches(self: *const Parameter, other_type: FunctionType) bool {
        if (self.param_type == other_type) return true;
        
        // Allow implicit conversions
        return switch (self.param_type) {
            .Normie => other_type == .Smol or other_type == .Thicc,
            .Meal => other_type == .Sip,
            .Drip => other_type == .Normie or other_type == .Smol or other_type == .Thicc,
            .Generic => true, // Generic parameters match any type
            else => false,
        };
    }
};

pub const FunctionSignature = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: FunctionType,
    is_variadic: bool = false,
    is_generic: bool = false,
    generic_constraints: ?[]const u8 = null,
    mangled_name: []const u8,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, name: []const u8, return_type: FunctionType) !FunctionSignature {
        const mangled = try mangleFunctionName(allocator, name, &[_]FunctionType{});
        return FunctionSignature{
            .name = name,
            .parameters = ArrayList(Parameter).init(allocator),
            .return_type = return_type,
            .mangled_name = mangled,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *FunctionSignature) void {
        self.parameters.deinit();
        self.allocator.free(self.mangled_name);
    }
    
    pub fn addParameter(self: *FunctionSignature, param: Parameter) !void {
        try self.parameters.append(param);
        // Update mangled name
        self.allocator.free(self.mangled_name);
        var param_types = ArrayList(FunctionType).init(self.allocator);
        defer param_types.deinit();
        
        for (self.parameters.items) |p| {
            try param_types.append(p.param_type);
        }
        
        self.mangled_name = try mangleFunctionName(self.allocator, self.name, param_types.items);
    }
    
    pub fn matchesCall(self: *const FunctionSignature, arg_types: []const FunctionType) bool {
        // Check arity (number of arguments)
        if (!self.is_variadic and arg_types.len != self.parameters.items.len) {
            return false;
        }
        
        if (self.is_variadic and arg_types.len < self.parameters.items.len - 1) {
            return false;
        }
        
        // Check parameter types
        for (arg_types, 0..) |arg_type, i| {
            if (i < self.parameters.items.len) {
                if (!self.parameters.items[i].matches(arg_type)) {
                    return false;
                }
            } else if (self.is_variadic) {
                // For variadic functions, remaining args must match last parameter type
                const last_param = &self.parameters.items[self.parameters.items.len - 1];
                if (!last_param.matches(arg_type)) {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        return true;
    }
    
    pub fn getSpecificity(self: *const FunctionSignature, arg_types: []const FunctionType) i32 {
        var specificity: i32 = 0;
        
        for (arg_types, 0..) |arg_type, i| {
            if (i < self.parameters.items.len) {
                const param = &self.parameters.items[i];
                if (param.param_type == arg_type) {
                    specificity += 100; // Exact match
                } else if (param.matches(arg_type)) {
                    specificity += 50;  // Compatible match
                } else {
                    return -1; // No match
                }
            }
        }
        
        // Prefer non-variadic functions
        if (!self.is_variadic) {
            specificity += 10;
        }
        
        return specificity;
    }
    
    pub fn format(self: *const FunctionSignature, writer: anytype) !void {
        try writer.print("slay {}(", .{self.name});
        for (self.parameters.items, 0..) |param, i| {
            if (i > 0) try writer.print(", ");
            try writer.print("{} {}", .{ param.name, param.param_type });
        }
        try writer.print(") {}", .{self.return_type});
    }
};

pub const OverloadSet = struct {
    name: []const u8,
    overloads: ArrayList(FunctionSignature),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, name: []const u8) OverloadSet {
        return OverloadSet{
            .name = name,
            .overloads = ArrayList(FunctionSignature).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *OverloadSet) void {
        for (self.overloads.items) |*overload| {
            overload.deinit();
        }
        self.overloads.deinit();
    }
    
    pub fn addOverload(self: *OverloadSet, signature: FunctionSignature) !void {
        // Check for duplicate signatures
        for (self.overloads.items) |*existing| {
            if (existing.parameters.items.len == signature.parameters.items.len) {
                var same_types = true;
                for (existing.parameters.items, signature.parameters.items) |existing_param, new_param| {
                    if (existing_param.param_type != new_param.param_type) {
                        same_types = false;
                        break;
                    }
                }
                if (same_types) {
                    return OverloadingError.DuplicateOverload;
                }
            }
        }
        
        try self.overloads.append(signature);
    }
    
    pub fn resolveCall(self: *const OverloadSet, arg_types: []const FunctionType) OverloadingError!*const FunctionSignature {
        var candidates = ArrayList(*const FunctionSignature).init(self.allocator);
        defer candidates.deinit();
        
        // Find all matching overloads
        for (self.overloads.items) |*overload| {
            if (overload.matchesCall(arg_types)) {
                try candidates.append(overload);
            }
        }
        
        if (candidates.items.len == 0) {
            return OverloadingError.NoMatchingOverload;
        }
        
        if (candidates.items.len == 1) {
            return candidates.items[0];
        }
        
        // Resolve ambiguity by specificity
        var best_candidate: *const FunctionSignature = candidates.items[0];
        var best_specificity: i32 = best_candidate.getSpecificity(arg_types);
        
        for (candidates.items[1..]) |candidate| {
            const specificity = candidate.getSpecificity(arg_types);
            if (specificity > best_specificity) {
                best_candidate = candidate;
                best_specificity = specificity;
            } else if (specificity == best_specificity) {
                return OverloadingError.AmbiguousOverload;
            }
        }
        
        return best_candidate;
    }
};

pub const FunctionRegistry = struct {
    overload_sets: HashMap([]const u8, OverloadSet),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) FunctionRegistry {
        return FunctionRegistry{
            .overload_sets = HashMap([]const u8, OverloadSet).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *FunctionRegistry) void {
        var iter = self.overload_sets.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.overload_sets.deinit();
    }
    
    pub fn registerFunction(self: *FunctionRegistry, signature: FunctionSignature) !void {
        const result = try self.overload_sets.getOrPut(signature.name);
        if (!result.found_existing) {
            result.value_ptr.* = OverloadSet.init(self.allocator, signature.name);
        }
        try result.value_ptr.addOverload(signature);
    }
    
    pub fn resolveFunction(self: *const FunctionRegistry, name: []const u8, arg_types: []const FunctionType) OverloadingError!*const FunctionSignature {
        const overload_set = self.overload_sets.get(name) orelse return OverloadingError.NoMatchingOverload;
        return overload_set.resolveCall(arg_types);
    }
    
    pub fn listOverloads(self: *const FunctionRegistry, name: []const u8) ?*const OverloadSet {
        return self.overload_sets.getPtr(name);
    }
};

// Function name mangling for unique identification
fn mangleFunctionName(allocator: Allocator, name: []const u8, param_types: []const FunctionType) ![]u8 {
    var mangled = ArrayList(u8).init(allocator);
    defer mangled.deinit();
    
    try mangled.appendSlice("_CURSED_");
    try mangled.appendSlice(name);
    try mangled.append('_');
    
    for (param_types) |param_type| {
        const type_code = switch (param_type) {
            .Void => "v",
            .Normie => "n",
            .Tea => "t",
            .Drip => "d",
            .Lit => "l",
            .Meal => "m",
            .Smol => "s",
            .Thicc => "T",
            .Sip => "S",
            .Array => "A",
            .Struct => "R",
            .Interface => "I",
            .Generic => "G",
        };
        try mangled.appendSlice(type_code);
    }
    
    return mangled.toOwnedSlice();
}

// Helper functions for parser integration
pub fn parseParameterList(allocator: Allocator, param_string: []const u8) !ArrayList(Parameter) {
    var params = ArrayList(Parameter).init(allocator);
    
    // Simple parsing - split by commas and parse type annotations
    var iter = std.mem.split(u8, param_string, ",");
    while (iter.next()) |param_str| {
        const trimmed = std.mem.trim(u8, param_str, " \t\n");
        if (trimmed.len == 0) continue;
        
        // Parse "name type" format
        var parts = std.mem.split(u8, trimmed, " ");
        const name = parts.next() orelse continue;
        const type_str = parts.next() orelse "normie";
        
        const param_type = parseType(type_str);
        try params.append(Parameter.init(name, param_type));
    }
    
    return params;
}

fn parseType(type_str: []const u8) FunctionType {
    if (std.mem.eql(u8, type_str, "normie")) return .Normie;
    if (std.mem.eql(u8, type_str, "tea")) return .Tea;
    if (std.mem.eql(u8, type_str, "drip")) return .Drip;
    if (std.mem.eql(u8, type_str, "lit")) return .Lit;
    if (std.mem.eql(u8, type_str, "meal")) return .Meal;
    if (std.mem.eql(u8, type_str, "smol")) return .Smol;
    if (std.mem.eql(u8, type_str, "thicc")) return .Thicc;
    if (std.mem.eql(u8, type_str, "sip")) return .Sip;
    return .Normie; // Default
}

// Global function registry for runtime use
var global_registry: ?FunctionRegistry = null;

pub fn initFunctionOverloading(allocator: Allocator) !void {
    global_registry = FunctionRegistry.init(allocator);
}

pub fn deinitFunctionOverloading() void {
    if (global_registry) |*registry| {
        registry.deinit();
        global_registry = null;
    }
}

export fn cursed_register_function(name_ptr: [*]const u8, name_len: usize, 
                                  param_types_ptr: [*]const u32, param_count: usize,
                                  return_type: u32) void {
    if (global_registry == null) return;
    
    const name = name_ptr[0..name_len];
    const param_types_raw = param_types_ptr[0..param_count];
    
    var registry = &global_registry.?;
    var signature = FunctionSignature.init(registry.allocator, name, @enumFromInt(return_type)) catch return;
    
    for (param_types_raw, 0..) |param_type_raw, i| {
        const param_name = std.fmt.allocPrint(registry.allocator, "param{}", .{i}) catch return;
        const param = Parameter.init(param_name, @enumFromInt(param_type_raw));
        signature.addParameter(param) catch return;
    }
    
    registry.registerFunction(signature) catch return;
}

export fn cursed_resolve_function(name_ptr: [*]const u8, name_len: usize,
                                arg_types_ptr: [*]const u32, arg_count: usize) ?[*]const u8 {
    if (global_registry == null) return null;
    
    const name = name_ptr[0..name_len];
    const arg_types_raw = arg_types_ptr[0..arg_count];
    
    var arg_types = std.ArrayList(FunctionType).init(global_registry.?.allocator);
    defer arg_types.deinit();
    
    for (arg_types_raw) |arg_type_raw| {
        arg_types.append(@enumFromInt(arg_type_raw)) catch return null;
    }
    
    const signature = global_registry.?.resolveFunction(name, arg_types.items) catch return null;
    return signature.mangled_name.ptr;
}

// Testing
pub fn testFunctionOverloading() !void {
    print("Testing function overloading implementation...\n");
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var registry = FunctionRegistry.init(allocator);
    defer registry.deinit();
    
    // Register multiple overloads for "add"
    var sig1 = try FunctionSignature.init(allocator, "add", .Normie);
    try sig1.addParameter(Parameter.init("a", .Normie));
    try sig1.addParameter(Parameter.init("b", .Normie));
    try registry.registerFunction(sig1);
    
    var sig2 = try FunctionSignature.init(allocator, "add", .Meal);
    try sig2.addParameter(Parameter.init("a", .Meal));
    try sig2.addParameter(Parameter.init("b", .Meal));
    try registry.registerFunction(sig2);
    
    var sig3 = try FunctionSignature.init(allocator, "add", .Tea);
    try sig3.addParameter(Parameter.init("a", .Tea));
    try sig3.addParameter(Parameter.init("b", .Tea));
    try registry.registerFunction(sig3);
    
    // Test resolution
    const resolved1 = try registry.resolveFunction("add", &[_]FunctionType{ .Normie, .Normie });
    const resolved2 = try registry.resolveFunction("add", &[_]FunctionType{ .Meal, .Meal });
    const resolved3 = try registry.resolveFunction("add", &[_]FunctionType{ .Tea, .Tea });
    
    std.testing.expect(resolved1.return_type == .Normie) catch return error.TestFailed;
    std.testing.expect(resolved2.return_type == .Meal) catch return error.TestFailed;
    std.testing.expect(resolved3.return_type == .Tea) catch return error.TestFailed;
    
    print("Function overloading tests passed!\n");
}
