// Binary trees benchmark for Zig

const std = @import("std");
const Timer = std.time.Timer;
const Allocator = std.mem.Allocator;

// Tree node structure
const TreeNode = struct {
    left: ?*TreeNode,
    right: ?*TreeNode,
    item: i32,

    fn init(allocator: *Allocator, item: i32) !*TreeNode {
        var node = try allocator.create(TreeNode);
        node.left = null;
        node.right = null;
        node.item = item;
        return node;
    }

    fn deinit(self: *TreeNode, allocator: *Allocator) void {
        if (self.left) |left| {
            left.deinit(allocator);
            allocator.destroy(left);
        }
        if (self.right) |right| {
            right.deinit(allocator);
            allocator.destroy(right);
        }
    }
};

// Create a tree of specified depth
fn makeTree(allocator: *Allocator, depth: i32, item: i32) !*TreeNode {
    var node = try TreeNode.init(allocator, item);
    if (depth > 0) {
        node.left = try makeTree(allocator, depth - 1, 2 * item - 1);
        node.right = try makeTree(allocator, depth - 1, 2 * item);
    }
    return node;
}

// Check a tree by calculating checksum
fn checkTree(node: ?*TreeNode) i32 {
    const n = node orelse return 0;
    if (n.left == null) return n.item;
    return n.item + checkTree(n.left) - checkTree(n.right);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = &gpa.allocator();

    const min_depth: i32 = 4;
    const max_depth: i32 = 12;
    const stretch_depth: i32 = max_depth + 1;

    var timer = try Timer.start();
    const start_time = timer.lap();

    // Create and check stretch tree
    var stretch_tree = try makeTree(allocator, stretch_depth, 0);
    const stretch_check = checkTree(stretch_tree);
    try std.io.getStdOut().writer().print("stretch tree of depth {}\t check: {}\n", .{stretch_depth, stretch_check});
    stretch_tree.deinit(allocator);
    allocator.destroy(stretch_tree);

    // Create long-lived tree
    var long_lived_tree = try makeTree(allocator, max_depth, 0);

    // Process trees of increasing depths
    var depth: i32 = min_depth;
    while (depth <= max_depth) : (depth += 2) {
        const iterations: i32 = @as(i32, 1) << @intCast(u5, max_depth - depth + min_depth);
        var check: i32 = 0;

        var i: i32 = 0;
        while (i < iterations) : (i += 1) {
            var a = try makeTree(allocator, depth, i);
            var b = try makeTree(allocator, depth, -i);
            check += checkTree(a) + checkTree(b);
            a.deinit(allocator);
            allocator.destroy(a);
            b.deinit(allocator);
            allocator.destroy(b);
        }

        try std.io.getStdOut().writer().print("{} trees of depth {}\t check: {}\n", .{iterations * 2, depth, check});
    }

    // Check long-lived tree
    try std.io.getStdOut().writer().print("long lived tree of depth {}\t check: {}\n", .{max_depth, checkTree(long_lived_tree)});
    long_lived_tree.deinit(allocator);
    allocator.destroy(long_lived_tree);

    const end_time = timer.lap();
    const elapsed = @intToFloat(f64, end_time - start_time) / std.time.ns_per_ms;
    try std.io.getStdOut().writer().print("Time taken: {d:.2} ms\n", .{elapsed});
}