const std = @import("std");
const print = std.debug.print;

/// Simple array length calculation test
pub fn main() !void {
    print("=== Dynamic Array Length Calculation Test ===\n", .{});
    
    // Test 1: Array literal parsing
    print("\nTest 1: Array literal element counting\n", .{});
    
    const array_literals = [_][]const u8{
        "[1, 2, 3]",
        "[10, 20, 30, 40, 50]", 
        "[]",
        "[42]",
        "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
    };
    
    for (array_literals) |literal| {
        const count = countArrayElements(literal);
        print("Array '{s}' -> {} elements\n", .{literal, count});
    }
    
    // Test 2: Type extraction
    print("\nTest 2: Array type extraction\n", .{});
    
    const array_types = [_][]const u8{
        "[]drip",
        "[]tea", 
        "[]lit",
        "[]normie"
    };
    
    for (array_types) |array_type| {
        if (getElementTypeName(array_type)) |element_type| {
            print("Array type '{s}' -> element type '{s}'\n", .{array_type, element_type});
        }
    }
    
    print("\n✅ Array length calculation tests completed\n", .{});
}

/// Count elements in array literal by parsing commas
fn countArrayElements(literal: []const u8) u32 {
    const start = std.mem.indexOf(u8, literal, "[") orelse return 0;
    const end = std.mem.lastIndexOf(u8, literal, "]") orelse return 0;
    
    if (end <= start + 1) return 0; // Empty array
    
    const content = literal[start + 1..end];
    if (content.len == 0) return 0;
    
    // Count commas + 1
    var count: u32 = 1;
    for (content) |char| {
        if (char == ',') count += 1;
    }
    
    return count;
}

/// Extract element type from array type
fn getElementTypeName(array_type: []const u8) ?[]const u8 {
    if (!std.mem.startsWith(u8, array_type, "[]")) return null;
    return array_type[2..];
}
