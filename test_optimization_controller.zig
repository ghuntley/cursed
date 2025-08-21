const std = @import("std");
const OptimizationController = @import("src-zig/optimization_level_controller.zig").OptimizationController;
const testOptimizationController = @import("src-zig/optimization_level_controller.zig").testOptimizationController;

pub fn main() !void {
    try testOptimizationController();
}
