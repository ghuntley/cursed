const std = @import("std");
const testing = std.testing;
const error_operators = @import("src-zig/error_operators.zig");

const YikesError = error_operators.YikesError;
const ShookResult = error_operators.ShookResult;
const Value = error_operators.ShookResult.Value;

test "error unwrap safety - no more panics" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create an error
    const error_value = try YikesError.init(allocator, "Test error message", 1);
    const result_error = ShookResult.err(error_value);

    // Test that unwrap returns error instead of panicking
    const unwrap_result = result_error.unwrap();
    try testing.expectError(error.ParseError, unwrap_result);

    // Test safe unwrap
    const default_value = Value{ .Integer = 42 };
    const safe_unwrap = result_error.unwrapOr(default_value);
    try testing.expect(safe_unwrap.Integer == 42);

    // Test unsafe unwrap (should log but not panic)
    const unsafe_unwrap = result_error.unwrapUnsafe();
    try testing.expect(unsafe_unwrap == Value.Void);

    // Test successful unwrap
    const success_value = Value{ .Integer = 100 };
    const result_ok = ShookResult.ok(success_value);
    const success_unwrap = try result_ok.unwrap();
    try testing.expect(success_unwrap.Integer == 100);
}

test "error code mapping" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Test different error codes map to correct CursedError types
    const test_cases = [_]struct { code: i64, expected: anyerror }{
        .{ .code = 1, .expected = error.ParseError },
        .{ .code = 2, .expected = error.TypeMismatch },
        .{ .code = 3, .expected = error.UndefinedVariable },
        .{ .code = 4, .expected = error.RuntimeError },
        .{ .code = 5, .expected = error.DivisionByZero },
        .{ .code = 6, .expected = error.IndexOutOfBounds },
        .{ .code = 7, .expected = error.NullPointerDereference },
        .{ .code = 8, .expected = error.InvalidOperation },
        .{ .code = 999, .expected = error.UnknownError },
    };

    for (test_cases) |case| {
        const error_value = try YikesError.init(allocator, "Test error", case.code);
        const result_error = ShookResult.err(error_value);
        const unwrap_result = result_error.unwrap();
        try testing.expectError(case.expected, unwrap_result);
    }
}
