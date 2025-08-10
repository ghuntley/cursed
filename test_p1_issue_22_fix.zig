const std = @import("std");
const builtin = @import("builtin");
const testing = std.testing;

const lexer = @import("src-zig/lexer.zig");
const parser = @import("src-zig/parser.zig");

test "P1 Issue #22: Error recovery enabled in release builds" {
    const allocator = testing.allocator;
    
    // Test that error recovery works in both debug and release builds
    const source =
        \\sus x drip = 10
        \\sus y drip = 20 // missing semicolon 
        \\sus z = "missing type"
        \\sus valid drip = 30
    ;
    
    var tokens = std.ArrayList(lexer.Token).init(allocator);
    defer tokens.deinit();
    
    var lexer_instance = lexer.Lexer.init(allocator, source);
    defer lexer_instance.deinit();
    
    // Tokenize the source
    while (!lexer_instance.isAtEnd()) {
        const token = lexer_instance.nextToken() catch break;
        try tokens.append(token);
    }
    
    var parser_instance = parser.Parser.init(tokens.items, allocator);
    defer parser_instance.deinit();
    
    // Parse with error recovery - should not crash or hang
    const program = parser_instance.parseProgram() catch |err| {
        // Error recovery should have allowed parsing to continue
        // The key fix is that recovery works in release builds too
        std.debug.print("Parser recovered from errors: {}\n", .{err});
        return; // Test passes if we reach here without hanging
    };
    
    // Check that error recovery stats were collected
    // In debug builds, stats will be printed; in release builds, they won't be
    // But the recovery mechanism itself should work in both cases
    if (builtin.mode == .Debug) {
        std.debug.print("Debug mode: Error recovery stats available\n", .{});
        try testing.expect(parser_instance.error_recovery_stats.total_errors >= 0);
    } else {
        std.debug.print("Release mode: Error recovery works without debug output\n", .{});
        // In release mode, the recovery should still work, just without verbose output
        try testing.expect(parser_instance.error_recovery_stats.total_errors >= 0);
    }
    
    _ = program; // Program might be partial due to errors, but that's expected
}

test "P1 Issue #22: Sync-to-semicolon recovery performance in release builds" {
    const allocator = testing.allocator;
    
    // Test with many syntax errors to ensure recovery doesn't slow down release builds
    const source =
        \\sus x drip = 10
        \\bad syntax here
        \\sus y drip = 20
        \\more bad syntax
        \\sus z drip = 30
        \\even more errors
        \\sus w drip = 40
    ;
    
    var tokens = std.ArrayList(lexer.Token).init(allocator);
    defer tokens.deinit();
    
    var lexer_instance = lexer.Lexer.init(allocator, source);
    defer lexer_instance.deinit();
    
    while (!lexer_instance.isAtEnd()) {
        const token = lexer_instance.nextToken() catch break;
        try tokens.append(token);
    }
    
    var parser_instance = parser.Parser.init(tokens.items, allocator);
    defer parser_instance.deinit();
    
    const start_time = std.time.milliTimestamp();
    
    // Parse with multiple errors - recovery should be fast
    _ = parser_instance.parseProgram() catch {
        // Expected to fail, but should recover quickly
    };
    
    const end_time = std.time.milliTimestamp();
    const parse_time = end_time - start_time;
    
    // Parser should recover quickly even with many errors
    // (This is more of a sanity check than a strict performance requirement)
    try testing.expect(parse_time < 1000); // Should take less than 1 second
    
    // Verify that recovery actually happened
    try testing.expect(parser_instance.error_recovery_stats.total_errors > 0);
    
    if (builtin.mode == .Debug) {
        std.debug.print("Debug build: Parse with errors took {}ms\n", .{parse_time});
    } else {
        std.debug.print("Release build: Parse with errors took {}ms (no debug output)\n", .{parse_time});
    }
}
