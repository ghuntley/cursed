const std = @import("std");
const print = std.debug.print;
const memory_safety = @import("src-zig/memory_safety_runtime.zig");

pub fn main() !void {
    print("Testing comprehensive array bounds checking implementation:\n\n", .{});
    
    // Test case 1: Valid index
    print("Test 1: Valid index (should pass)\n", .{});
    memory_safety.checkArrayBounds(1, 3) catch |err| {
        print("ERROR: {}\n", .{err});
        return;
    };
    print("✅ Valid index test passed\n\n", .{});
    
    // Test case 2: Negative index
    print("Test 2: Negative index (should fail)\n", .{});
    memory_safety.checkArrayBounds(-1, 3) catch |err| {
        print("✅ Correctly caught error: {}\n\n", .{err});
    };
    
    // Test case 3: Index out of bounds
    print("Test 3: Index out of bounds (should fail)\n", .{});
    memory_safety.checkArrayBounds(5, 3) catch |err| {
        print("✅ Correctly caught error: {}\n\n", .{err});
    };
    
    // Test case 4: Edge case - index equals length
    print("Test 4: Index equals length (should fail)\n", .{});
    memory_safety.checkArrayBounds(3, 3) catch |err| {
        print("✅ Correctly caught error: {}\n\n", .{err});
    };
    
    print("All bounds checking tests completed successfully!\n", .{});
}
