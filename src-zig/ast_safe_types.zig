// Safe AST Type Definitions with Reference Counting
// Prevents use-after-free and double-free issues in module dependencies

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const ast = @import("ast.zig");

// Reference-counted pointer wrapper
pub fn RefPtr(comptime T: type) type {
    return struct {
        const Self = @This();
        
        ptr: ?*T,
        ref_count: *u32,
        allocator: Allocator,
        
        pub fn init(allocator: Allocator, value: T) !Self {
            const ptr = try allocator.create(T);
            const ref_count = try allocator.create(u32);
            
            ptr.* = value;
            ref_count.* = 1;
            
            return Self{
                .ptr = ptr,
                .ref_count = ref_count,
                .allocator = allocator,
            };
        }
        
        pub fn clone(self: *const Self) Self {
            if (self.ptr != null) {
                self.ref_count.* += 1;
            }
            return Self{
                .ptr = self.ptr,
                .ref_count = self.ref_count,
                .allocator = self.allocator,
            };
        }
        
        pub fn get(self: *const Self) ?*T {
            return self.ptr;
        }
        
        pub fn deinit(self: *Self) void {
            if (self.ptr == null) return;
            
            self.ref_count.* -= 1;
            if (self.ref_count.* == 0) {
                // Last reference - safe to destroy
                if (std.meta.hasFn(T, "deinit")) {
                    self.ptr.?.deinit(allocator);
                }
                self.allocator.destroy(self.ptr.?);
                self.allocator.destroy(self.ref_count);
                self.ptr = null;
            }
        }
    };
}

// Safe versions of AST types using reference counting
pub const SafeArrayType = struct {
    element_type: RefPtr(ast.Type),
    size: ?usize,

    pub fn init(allocator: Allocator, element_type: ast.Type, size: ?usize) !SafeArrayType {
        return SafeArrayType{
            .element_type = try RefPtr(ast.Type).init(allocator, element_type),
            .size = size,
        };
    }

    pub fn deinit(self: *SafeArrayType) void {
        self.element_type.deinit(allocator);
    }

    pub fn toAstType(self: *const SafeArrayType) ast.ArrayType {
        return ast.ArrayType{
            .element_type = self.element_type.get() orelse unreachable,
            .size = self.size,
            ._owned = false, // Don't own the pointer since it's reference counted
        };
    }
};

pub const SafeMapType = struct {
    key_type: RefPtr(ast.Type),
    value_type: RefPtr(ast.Type),

    pub fn init(allocator: Allocator, key_type: ast.Type, value_type: ast.Type) !SafeMapType {
        return SafeMapType{
            .key_type = try RefPtr(ast.Type).init(allocator, key_type),
            .value_type = try RefPtr(ast.Type).init(allocator, value_type),
        };
    }

    pub fn deinit(self: *SafeMapType) void {
        self.key_type.deinit(allocator);
        self.value_type.deinit(allocator);
    }

    pub fn toAstType(self: *const SafeMapType) ast.MapType {
        return ast.MapType{
            .key_type = self.key_type.get() orelse unreachable,
            .value_type = self.value_type.get() orelse unreachable,
            ._key_owned = false,   // Don't own since reference counted
            ._value_owned = false, // Don't own since reference counted
        };
    }
};

pub const SafeFunctionType = struct {
    parameters: ArrayList(ast.Type),
    return_type: ?RefPtr(ast.Type),

    pub fn init(allocator: Allocator, parameters: ArrayList(ast.Type), return_type: ?ast.Type) !SafeFunctionType {
        var safe_return: ?RefPtr(ast.Type) = null;
        if (return_type) |ret| {
            safe_return = try RefPtr(ast.Type).init(allocator, ret);
        }

        return SafeFunctionType{
            .parameters = parameters,
            .return_type = safe_return,
        };
    }

    pub fn deinit(self: *SafeFunctionType, allocator: Allocator) void {
        for (self.parameters.items) |*param| {
            param.deinit(allocator);
        }
        self.parameters.deinit(allocator);
        
        if (self.return_type) |*ret| {
            ret.deinit(allocator);
        }
    }

    pub fn toAstType(self: *const SafeFunctionType) ast.FunctionType {
        var return_ptr: ?*ast.Type = null;
        if (self.return_type) |ret| {
            return_ptr = ret.get();
        }

        return ast.FunctionType{
            .parameters = self.parameters,
            .return_type = return_ptr,
            ._return_owned = false, // Don't own since reference counted
        };
    }
};

// Safe type creation functions
pub fn createSafeArrayType(allocator: Allocator, element_type: ast.Type, size: ?usize) !*SafeArrayType {
    const safe_type = try allocator.create(SafeArrayType);
    safe_type.* = try SafeArrayType.init(allocator, element_type, size);
    return safe_type;
}

pub fn createSafeMapType(allocator: Allocator, key_type: ast.Type, value_type: ast.Type) !*SafeMapType {
    const safe_type = try allocator.create(SafeMapType);
    safe_type.* = try SafeMapType.init(allocator, key_type, value_type);
    return safe_type;
}

pub fn createSafeFunctionType(allocator: Allocator, parameters: ArrayList(ast.Type), return_type: ?ast.Type) !*SafeFunctionType {
    const safe_type = try allocator.create(SafeFunctionType);
    safe_type.* = try SafeFunctionType.init(allocator, parameters, return_type);
    return safe_type;
}

// Tests
test "RefPtr reference counting" {
    const allocator = std.testing.allocator;
    
    var ref1 = try RefPtr(i32).init(allocator, 42);
    defer ref1.deinit(allocator);
    
    var ref2 = ref1.clone();
    defer ref2.deinit(allocator);
    
    // Both should point to the same value
    try std.testing.expect(ref1.get().?.* == 42);
    try std.testing.expect(ref2.get().?.* == 42);
    try std.testing.expect(ref1.ref_count.* == 2);
}

test "SafeArrayType lifecycle" {
    const allocator = std.testing.allocator;
    
    const element_type = ast.Type{ .Basic = "drip" };
    var array_type = try SafeArrayType.init(allocator, element_type, null);
    defer array_type.deinit(allocator);
    
    // Should be safely cloneable
    var cloned = array_type.element_type.clone();
    defer cloned.deinit(allocator);
    
    try std.testing.expect(array_type.element_type.ref_count.* == 2);
}
