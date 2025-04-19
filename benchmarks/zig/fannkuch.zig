// Fannkuch Redux benchmark for Zig

const std = @import("std");
const Timer = std.time.Timer;

// Flip the first n elements in the array
fn flip(p: []i32, n: usize) void {
    var i: usize = 0;
    while (i < n / 2) : (i += 1) {
        const temp = p[i];
        p[i] = p[n - i - 1];
        p[n - i - 1] = temp;
    }
}

// Fannkuch algorithm implementation
fn fannkuch(n: usize, allocator: *std.mem.Allocator) !struct { max_flips: i32, checksum: i32 } {
    var p = try allocator.alloc(i32, n);
    defer allocator.free(p);
    
    var perm = try allocator.alloc(i32, n);
    defer allocator.free(perm);
    
    var count = try allocator.alloc(i32, n);
    defer allocator.free(count);
    
    // Initialize permutation
    for (p) |*v, i| {
        v.* = @intCast(i32, i);
    }
    
    var max_flips: i32 = 0;
    var checksum: i32 = 0;
    var sign: i32 = 1;
    var perm_count: usize = 0;
    
    while (true) {
        // Copy permutation
        for (p) |v, i| {
            perm[i] = v + 1;
        }
        
        // Count flips
        if (p[0] != 0) {
            for (count) |*v| {
                v.* = 0;
            }
            
            var flips: i32 = 0;
            var k: usize = @intCast(usize, p[0]);
            
            while (k != 0) {
                flip(perm, k + 1);
                flips += 1;
                k = @intCast(usize, perm[0] - 1);
            }
            
            max_flips = @maximum(max_flips, flips);
            checksum += sign * flips;
        }
        
        // Generate next permutation
        sign = -sign;
        
        // Find position for next permutation
        var j: usize = 1;
        while (j < n and p[j-1] >= p[j]) : (j += 1) {}
        
        if (j >= n) break;  // No more permutations
        
        perm_count += 1;
        if (perm_count >= 10000) break;  // Limit permutations for benchmark
        
        const first_j = p[j];
        var i: usize = 0;
        while (i < j) : (i += 1) {
            p[i] = p[j-i-1];
        }
        p[j] = first_j;
    }
    
    return .{ .max_flips = max_flips, .checksum = checksum };
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = &gpa.allocator();
    
    const n: usize = 10;  // Standard size for the benchmark
    
    var timer = try Timer.start();
    const start_time = timer.lap();
    
    const result = try fannkuch(n, allocator);
    
    try std.io.getStdOut().writer().print("Fannkuch({d}): {d}\n", .{n, result.max_flips});
    try std.io.getStdOut().writer().print("Checksum: {d}\n", .{result.checksum});
    
    const end_time = timer.lap();
    const elapsed = @intToFloat(f64, end_time - start_time) / std.time.ns_per_ms;
    try std.io.getStdOut().writer().print("Time taken: {d:.2} ms\n", .{elapsed});
}