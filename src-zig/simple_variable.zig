// Simple variable system for CURSED interpreter
// This provides basic variable storage and retrieval functionality

const std = @import("std");
const Allocator = std.mem.Allocator;

pub const SimpleVariable = struct {
    name: []const u8,
    value: []const u8,
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8, value: []const u8) !SimpleVariable {
        const name_copy = try allocator.dupe(u8, name);
        const value_copy = try allocator.dupe(u8, value);
        
        return SimpleVariable{
            .name = name_copy,
            .value = value_copy,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *SimpleVariable) void {
        self.allocator.free(self.name);
        self.allocator.free(self.value);
    }

    pub fn setValue(self: *SimpleVariable, new_value: []const u8) !void {
        self.allocator.free(self.value);
        self.value = try self.allocator.dupe(u8, new_value);
    }

    pub fn getValue(self: *const SimpleVariable) []const u8 {
        return self.value;
    }

    pub fn getName(self: *const SimpleVariable) []const u8 {
        return self.name;
    }
};
