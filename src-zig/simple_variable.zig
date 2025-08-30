const std = @import("std");
const Allocator = std.mem.Allocator;

pub const SimpleVariableError = error{
    OutOfMemory,
    InvalidValue,
};

pub const SimpleVariable = struct {
    name: []const u8,
    value: ?[]const u8,
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8, value: ?[]const u8) SimpleVariableError!SimpleVariable {
        const name_copy = try allocator.dupe(u8, name);
        const value_copy = if (value) |v| try allocator.dupe(u8, v) else null;
        
        return SimpleVariable{
            .name = name_copy,
            .value = value_copy,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *SimpleVariable) void {
        self.allocator.free(self.name);
        if (self.value) |v| {
            self.allocator.free(v);
        }
    }

    pub fn setValue(self: *SimpleVariable, new_value: ?[]const u8) SimpleVariableError!void {
        if (self.value) |old_value| {
            self.allocator.free(old_value);
        }
        
        self.value = if (new_value) |v| try self.allocator.dupe(u8, v) else null;
    }

    pub fn getValue(self: *const SimpleVariable) ?[]const u8 {
        return self.value;
    }

    pub fn getName(self: *const SimpleVariable) []const u8 {
        return self.name;
    }
};

pub fn createSimpleVariable(allocator: Allocator, name: []const u8, value: ?[]const u8) SimpleVariableError!*SimpleVariable {
    const var_ptr = try allocator.create(SimpleVariable);
    var_ptr.* = try SimpleVariable.init(allocator, name, value);
    return var_ptr;
}

pub fn destroySimpleVariable(allocator: Allocator, variable: *SimpleVariable) void {
    variable.deinit();
    allocator.destroy(variable);
}
