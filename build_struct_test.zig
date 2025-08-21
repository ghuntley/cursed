const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

// Simple struct test
const StructInstance = struct {
    type_name: []const u8,
    fields: HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator, type_name: []const u8) StructInstance {
        return StructInstance{
            .type_name = type_name,
            .fields = HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
};

const Variable = union(enum) {
    Integer: i64,
    String: []const u8,
    Struct: StructInstance,
    
    pub fn clone(self: Variable, allocator: Allocator) !Variable {
        switch (self) {
            .Integer => |v| return Variable{ .Integer = v },
            .String => |s| return Variable{ .String = try allocator.dupe(u8, s) },
            .Struct => |struct_instance| {
                var new_struct = StructInstance.init(allocator, struct_instance.type_name);
                var iter = struct_instance.fields.iterator();
                while (iter.next()) |entry| {
                    const key_copy = try allocator.dupe(u8, entry.key_ptr.*);
                    const value_copy = try entry.value_ptr.clone(allocator);
                    try new_struct.fields.put(key_copy, value_copy);
                }
                return Variable{ .Struct = new_struct };
            },
        }
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("Testing struct field access...\n", .{});
    
    // Create a Person struct
    var person = StructInstance.init(allocator, "Person");
    
    // Add fields
    const name_key = try allocator.dupe(u8, "name");
    const name_value = Variable{ .String = "Alice" };
    try person.fields.put(name_key, name_value);
    
    const age_key = try allocator.dupe(u8, "age");
    const age_value = Variable{ .Integer = 30 };
    try person.fields.put(age_key, age_value);
    
    // Test field access
    if (person.fields.get("name")) |name| {
        switch (name) {
            .String => |s| print("Person name: {s}\n", .{s}),
            else => print("Name is not a string\n", .{}),
        }
    }
    
    if (person.fields.get("age")) |age| {
        switch (age) {
            .Integer => |i| print("Person age: {}\n", .{i}),
            else => print("Age is not an integer\n", .{}),
        }
    }
    
    print("Struct field access test completed!\n", .{});
}
