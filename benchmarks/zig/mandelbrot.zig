// Mandelbrot set calculation benchmark for Zig

const std = @import("std");
const Timer = std.time.Timer;

// Constants
const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const MAX_ITERATIONS: usize = 100;

// Calculate Mandelbrot set for a single point
fn mandelbrotPoint(cx: f64, cy: f64, max_iter: usize) usize {
    var zx: f64 = 0.0;
    var zy: f64 = 0.0;
    var iteration: usize = 0;
    
    while (zx*zx + zy*zy <= 4.0 and iteration < max_iter) : (iteration += 1) {
        const temp = zx*zx - zy*zy + cx;
        zy = 2.0*zx*zy + cy;
        zx = temp;
    }
    
    return iteration;
}

// Calculate the entire Mandelbrot set
fn calculateMandelbrot(allocator: *std.mem.Allocator) ![][WIDTH]usize {
    var result = try allocator.alloc([WIDTH]usize, HEIGHT);
    errdefer allocator.free(result);
    
    var y: usize = 0;
    while (y < HEIGHT) : (y += 1) {
        var x: usize = 0;
        while (x < WIDTH) : (x += 1) {
            const cx = (@intToFloat(f64, x) - @intToFloat(f64, WIDTH)/2.0) * 4.0 / @intToFloat(f64, WIDTH);
            const cy = (@intToFloat(f64, y) - @intToFloat(f64, HEIGHT)/2.0) * 4.0 / @intToFloat(f64, HEIGHT);
            result[y][x] = mandelbrotPoint(cx, cy, MAX_ITERATIONS);
        }
    }
    
    return result;
}

// Count non-black pixels
fn countNonBlack(result: [][WIDTH]usize, max_iter: usize) usize {
    var count: usize = 0;
    
    for (result) |row| {
        for (row) |pixel| {
            if (pixel < max_iter) {
                count += 1;
            }
        }
    }
    
    return count;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = &gpa.allocator();
    
    var timer = try Timer.start();
    const start_time = timer.lap();
    
    var result = try calculateMandelbrot(allocator);
    defer allocator.free(result);
    
    const non_black_count = countNonBlack(result, MAX_ITERATIONS);
    
    try std.io.getStdOut().writer().print("Mandelbrot set calculation finished.\n", .{});
    try std.io.getStdOut().writer().print("Image size: {d} x {d}\n", .{WIDTH, HEIGHT});
    try std.io.getStdOut().writer().print("Maximum iterations: {d}\n", .{MAX_ITERATIONS});
    try std.io.getStdOut().writer().print("Non-black pixels: {d}\n", .{non_black_count});
    
    const end_time = timer.lap();
    const elapsed = @intToFloat(f64, end_time - start_time) / std.time.ns_per_ms;
    try std.io.getStdOut().writer().print("Time taken: {d:.2} ms\n", .{elapsed});
}