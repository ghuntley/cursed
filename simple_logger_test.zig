const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.debug.print("✅ CURSED Optimized JSON Logger - Basic Test\n", .{});
    
    // Test basic JSON formatting
    const json_output = try formatBasicJson(allocator, "Test message", 42);
    std.debug.print("📝 JSON Output: {s}\n", .{json_output});
    
    allocator.free(json_output);
    
    std.debug.print("🎉 Basic test completed successfully!\n", .{});
}

fn formatBasicJson(allocator: std.mem.Allocator, message: []const u8, user_id: i32) ![]u8 {
    const timestamp = std.time.nanoTimestamp();
    
    const json_template = 
        \\{{"timestamp":{},"level":"INFO","message":"{s}","user_id":{}}}
    ;
    
    return std.fmt.allocPrint(allocator, json_template, .{ timestamp, message, user_id });
}
