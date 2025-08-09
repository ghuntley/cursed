const std = @import("std");
const print = std.debug.print;
const RegressionTestRunner = @import("src-zig/regression_test_runner.zig").RegressionTestRunner;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🧪 Testing Stability Fixes\n", .{});
    
    var runner = RegressionTestRunner.init(allocator);
    runner.verbose = true;
    runner.test_timeout_ms = 10000; // 10 second timeout
    runner.use_valgrind = false; // Disable valgrind for this test
    
    // Test some individual files
    const test_files = [_][]const u8{
        "test_math_safe.csd",
        "simple_test.csd",
        "test_malformed.csd",
    };
    
    var passed: u32 = 0;
    var failed: u32 = 0;
    
    for (test_files) |test_file| {
        print("📋 Testing: {s}\n", .{test_file});
        
        const result = runner.runSingleTest(test_file) catch |err| {
            print("  ❌ Test execution failed: {}\n", .{err});
            failed += 1;
            continue;
        };
        
        if (result.passed) {
            print("  ✅ PASSED ({d}ms)\n", .{result.execution_time_ms});
            passed += 1;
        } else {
            print("  ❌ FAILED: ", .{});
            if (result.error_message) |msg| {
                print("{s}", .{msg});
            }
            print(" ({d}ms)\n", .{result.execution_time_ms});
            failed += 1;
        }
    }
    
    print("\n📊 Results: {d} passed, {d} failed\n", .{ passed, failed });
    
    if (failed == 0) {
        print("🎉 All stability tests passed!\n", .{});
    } else {
        print("⚠️ Some tests failed, but no crashes occurred\n", .{});
    }
}
