// String processing benchmark for Zig

const std = @import("std");
const Timer = std.time.Timer;
const ArrayList = std.ArrayList;
const Random = std.rand.Random;

// Create a random string of specified length
fn createRandomString(allocator: *std.mem.Allocator, length: usize, random: Random) ![]u8 {
    const chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    var result = try allocator.alloc(u8, length);
    
    for (result) |*c| {
        c.* = chars[random.uintLessThan(usize, chars.len)];
    }
    
    return result;
}

// Process a string with various operations
fn processString(allocator: *std.mem.Allocator, input: []const u8) ![]u8 {
    // Replace all vowels with uppercase version
    var result = try allocator.alloc(u8, input.len);
    std.mem.copy(u8, result, input);
    
    // Replace vowels
    for (result) |*c| {
        switch (c.*) {
            'a' => c.* = 'A',
            'e' => c.* = 'E',
            'i' => c.* = 'I',
            'o' => c.* = 'O',
            'u' => c.* = 'U',
            else => {},
        }
    }
    
    // For other operations, we need a mutable size buffer
    var buffer = ArrayList(u8).init(allocator);
    defer buffer.deinit();
    
    try buffer.appendSlice(result);
    allocator.free(result);
    
    // Replace digits with doubled value
    var i: usize = 0;
    while (i < buffer.items.len) : (i += 1) {
        if (std.ascii.isDigit(buffer.items[i])) {
            const digit = buffer.items[i] - '0';
            const doubled = digit * 2;
            
            // Replace with doubled value as string
            var doubled_str = try std.fmt.allocPrint(allocator, "{d}", .{doubled});
            defer allocator.free(doubled_str);
            
            _ = buffer.orderedRemove(i);
            try buffer.insertSlice(i, doubled_str);
            i += doubled_str.len - 1;
        }
    }
    
    // Capitalize first letter if string is not empty
    if (buffer.items.len > 0 and std.ascii.isLower(buffer.items[0])) {
        buffer.items[0] = std.ascii.toUpper(buffer.items[0]);
    }
    
    // Reverse the string
    var reversed = try allocator.alloc(u8, buffer.items.len);
    for (buffer.items) |c, idx| {
        reversed[buffer.items.len - idx - 1] = c;
    }
    
    // Take first half
    const half_len = reversed.len / 2;
    var half = try allocator.alloc(u8, half_len);
    std.mem.copy(u8, half, reversed[0..half_len]);
    allocator.free(reversed);
    
    return half;
}

// Process multiple strings of different sizes
fn processStrings(allocator: *std.mem.Allocator, count: usize, size: usize, random: Random) ![]u8 {
    var result = ArrayList(u8).init(allocator);
    defer result.deinit();
    
    var i: usize = 0;
    while (i < count) : (i += 1) {
        var str = try createRandomString(allocator, size, random);
        defer allocator.free(str);
        
        var processed = try processString(allocator, str);
        defer allocator.free(processed);
        
        try result.appendSlice(processed);
    }
    
    return result.toOwnedSlice();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = &gpa.allocator();
    
    var timer = try Timer.start();
    const start_time = timer.lap();
    
    var prng = std.rand.DefaultPrng.init(0);
    const random = prng.random();
    
    // Process strings of different sizes
    var small = try processStrings(allocator, 10000, 10, random);    // 10,000 strings of length 10
    defer allocator.free(small);
    
    var medium = try processStrings(allocator, 1000, 100, random);   // 1,000 strings of length 100
    defer allocator.free(medium);
    
    var large = try processStrings(allocator, 100, 1000, random);    // 100 strings of length 1,000
    defer allocator.free(large);
    
    const result_length = small.len + medium.len + large.len;
    try std.io.getStdOut().writer().print("Processed string length: {d}\n", .{result_length});
    
    const end_time = timer.lap();
    const elapsed = @intToFloat(f64, end_time - start_time) / std.time.ns_per_ms;
    try std.io.getStdOut().writer().print("Time taken: {d:.2} ms\n", .{elapsed});
}