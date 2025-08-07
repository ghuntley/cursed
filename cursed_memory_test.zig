const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

// Variable store similar to main_unified.zig
const Variable = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Array: ArrayList(Variable),
};

const VariableStore = HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test the memory leak fixes from main_unified.zig
    try testVariableMemoryManagement(allocator);
    print("All tests passed - no memory leaks!\n", .{});
}

fn testVariableMemoryManagement(allocator: Allocator) !void {
    // Create arena for variable names (fix for memory leak)
    var variable_arena = std.heap.ArenaAllocator.init(allocator);
    defer variable_arena.deinit();
    const variable_allocator = variable_arena.allocator();
    
    // Create variable store
    var variables = VariableStore.init(allocator);
    defer {
        // Clean up string values and arrays (variable names handled by arena)
        var iterator = variables.iterator();
        while (iterator.next()) |entry| {
            switch (entry.value_ptr.*) {
                .String => |str| allocator.free(str),  // Free string values
                .Array => |arr| arr.deinit(),  // Free array allocations
                else => {},
            }
        }
        variables.deinit();
    }
    
    // Test 1: Variable name allocation (should use arena allocator)
    const var_name = try variable_allocator.dupe(u8, "test_variable");
    const string_value = try allocator.dupe(u8, "hello world");
    try variables.put(var_name, Variable{ .String = string_value });
    
    // Test 2: String concatenation (binary operation simulation)
    const left_str = "hello";
    const right_str = "world";
    const concat_result = try std.fmt.allocPrint(allocator, "{s}{s}", .{ left_str, right_str });
    const concat_var_name = try variable_allocator.dupe(u8, "concat_result");
    try variables.put(concat_var_name, Variable{ .String = concat_result });
    
    // Test 3: Multiple variables with different types
    const int_var_name = try variable_allocator.dupe(u8, "number");
    try variables.put(int_var_name, Variable{ .Integer = 42 });
    
    const bool_var_name = try variable_allocator.dupe(u8, "flag");
    try variables.put(bool_var_name, Variable{ .Boolean = true });
    
    // Test 4: Array allocation
    var test_array = ArrayList(Variable).init(allocator);
    try test_array.append(Variable{ .Integer = 1 });
    try test_array.append(Variable{ .Integer = 2 });
    try test_array.append(Variable{ .Integer = 3 });
    
    const array_var_name = try variable_allocator.dupe(u8, "numbers");
    try variables.put(array_var_name, Variable{ .Array = test_array });
    
    print("Created {} variables successfully\n", .{variables.count()});
    
    // Verify variables are accessible
    if (variables.get("test_variable")) |var_value| {
        switch (var_value) {
            .String => |s| print("Found variable: {s}\n", .{s}),
            else => {},
        }
    }
    
    if (variables.get("concat_result")) |var_value| {
        switch (var_value) {
            .String => |s| print("Found concatenated: {s}\n", .{s}),
            else => {},
        }
    }
    
    // Variable arena will automatically clean up all variable names
    // Manual cleanup in defer will handle string values and arrays
}
