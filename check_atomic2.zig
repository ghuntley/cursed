const std = @import("std");

pub fn main() !void {
    std.debug.print("Checking atomic orders...\n", .{});
    
    // Try seq_cst
    var val = std.atomic.Value(bool).init(false);
    val.store(true, .seq_cst);
    std.debug.print("seq_cst works\n", .{});
    
    // Try acq_rel
    val.store(false, .acq_rel);
    std.debug.print("acq_rel works\n", .{});
}
