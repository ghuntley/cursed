const std = @import("std");
const print = std.debug.print;

// Simple test for memory leak fixes
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test variable name arena allocator
    var variable_arena = std.heap.ArenaAllocator.init(allocator);
    defer variable_arena.deinit();
    const variable_allocator = variable_arena.allocator();

    // Test variable name allocation (should be cleaned up by arena)
    const name1 = try variable_allocator.dupe(u8, "test_var");
    const name2 = try variable_allocator.dupe(u8, "another_var");
    
    print("Variable names: {s}, {s}\n", .{name1, name2});
    
    // Test string concatenation (should be cleaned up manually)
    const str1 = "hello";
    const str2 = "world";
    const result = try std.fmt.allocPrint(allocator, "{s}{s}", .{str1, str2});
    defer allocator.free(result);
    
    print("Concatenated: {s}\n", .{result});
    print("Memory test completed successfully!\n", .{});
}
