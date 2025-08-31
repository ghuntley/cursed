// Oracle's Strategy Implementation - LLVM IR Validation Test
const std = @import("std");
const print = std.debug.print;
const interpreter = @import("src-zig/interpreter.zig");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    // Test cases from Oracle's strategy
    const test_cases = [_]struct {
        name: []const u8,
        code: []const u8,
        expected: i64,
    }{
        .{ .name = "mathz.add_two", .code = "yap(mathz.add_two(5, 3))", .expected = 8 },
        .{ .name = "mathz.multiply_two", .code = "yap(mathz.multiply_two(4, 7))", .expected = 28 },
        .{ .name = "mathz.abs_normie", .code = "yap(mathz.abs_normie(-10))", .expected = 10 },
        .{ .name = "mathz.max_normie", .code = "yap(mathz.max_normie(15, 7))", .expected = 15 },
        .{ .name = "mathz.min_normie", .code = "yap(mathz.min_normie(15, 7))", .expected = 7 },
    };

    print("🔍 Oracle's Strategy - Testing stdlib functions in INTERPRETER mode\n", .{});
    print("====================================================================\n", .{});

    for (test_cases) |test_case| {
        print("\n🧪 Testing {s}:\n", .{test_case.name});
        
        // Test with interpreter (known to work)
        const interp = interpreter.Interpreter.init(allocator);
        defer interp.deinit();
        
        const result = interp.interpretSource(test_case.code) catch |err| {
            print("❌ Interpreter failed for {s}: {}\n", .{ test_case.name, err });
            continue;
        };
        
        const actual_value = switch (result) {
            .Integer => |val| val,
            .Float => |val| @as(i64, @intFromFloat(val)),
            else => {
                print("❌ Unexpected result type for {s}\n", .{test_case.name});
                continue;
            },
        };
        
        if (actual_value == test_case.expected) {
            print("✅ {s} = {} (correct)\n", .{ test_case.name, actual_value });
        } else {
            print("❌ {s} = {} (expected {})\n", .{ test_case.name, actual_value, test_case.expected });
        }
    }

    print("\n🎯 Oracle's next step: Identify which specific LLVM function fails verification\n", .{});
    print("💡 Add per-function LLVMVerifyFunction calls to isolate the failing function\n", .{});
}
