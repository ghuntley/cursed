const std = @import("std");
const print = std.debug.print;

// CURSED Testing Framework (Zig Implementation)
// This provides the same functionality as the CURSED stdlib/testz module

var test_count: u32 = 0;
var passed_count: u32 = 0;
var failed_count: u32 = 0;
var current_test_name: ?[]const u8 = null;

pub fn test_start(name: []const u8) void {
    current_test_name = name;
    test_count += 1;
    print("Running test: {s}\n", .{name});
}

pub fn assert_true(condition: bool) void {
    if (condition) {
        passed_count += 1;
        print("  ✓ PASS\n");
    } else {
        failed_count += 1;
        print("  ✗ FAIL: Expected true, got false\n");
    }
}

pub fn assert_false(condition: bool) void {
    if (!condition) {
        passed_count += 1;
        print("  ✓ PASS\n");
    } else {
        failed_count += 1;
        print("  ✗ FAIL: Expected false, got true\n");
    }
}

pub fn assert_eq_int(actual: i64, expected: i64) void {
    if (actual == expected) {
        passed_count += 1;
        print("  ✓ PASS\n");
    } else {
        failed_count += 1;
        print("  ✗ FAIL: Expected {}, got {}\n", .{ expected, actual });
    }
}

pub fn assert_eq_string(actual: []const u8, expected: []const u8) void {
    if (std.mem.eql(u8, actual, expected)) {
        passed_count += 1;
        print("  ✓ PASS\n");
    } else {
        failed_count += 1;
        print("  ✗ FAIL: Expected '{s}', got '{s}'\n", .{ expected, actual });
    }
}

pub fn assert_eq_float(actual: f64, expected: f64) void {
    const epsilon = 1e-9;
    if (@abs(actual - expected) < epsilon) {
        passed_count += 1;
        print("  ✓ PASS\n");
    } else {
        failed_count += 1;
        print("  ✗ FAIL: Expected {d}, got {d}\n", .{ expected, actual });
    }
}

pub fn print_test_summary() void {
    print("\n=== TEST SUMMARY ===\n");
    print("Total tests: {}\n", .{test_count});
    print("Passed: {}\n", .{passed_count});
    print("Failed: {}\n", .{failed_count});
    
    if (failed_count == 0) {
        print("🎉 All tests passed!\n");
    } else {
        print("❌ {} test(s) failed\n", .{failed_count});
    }
    print("==================\n");
}

// Test the testing framework itself
test "testz framework" {
    test_start("Basic assertions");
    assert_true(true);
    assert_false(false);
    assert_eq_int(42, 42);
    assert_eq_string("hello", "hello");
    assert_eq_float(3.14, 3.14);
    
    test_start("Math operations");
    assert_eq_int(2 + 2, 4);
    assert_eq_int(10 - 3, 7);
    assert_eq_int(5 * 6, 30);
    
    print_test_summary();
}
