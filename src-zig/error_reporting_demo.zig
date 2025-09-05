const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const enhanced_lexer = @import("enhanced_lexer.zig");
const error_reporting = @import("enhanced_error_reporting.zig");

// Standalone demo of enhanced error reporting capabilities
// Showcases rich diagnostics, color output, and helpful suggestions

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🎯 CURSED Enhanced Error Reporting Demo\n", .{});
    print("=========================================\n\n", .{});

    // Initialize error reporter
    var error_reporter = error_reporting.ErrorReporter.init(allocator, 10);
    defer error_reporter.deinit();
    
    error_reporter.setColors(true);
    error_reporter.setVerbose(true);

    // Test 1: Lexical errors
    print("📝 Test 1: Lexical Error Reporting\n", .{});
    print("-----------------------------------\n", .{});
    
    const source1 = "\"unterminated string\nsus x normie = 42;";
    try error_reporter.addSourceFile("test1.💀", source1);
    
    var lexer1 = enhanced_lexer.Lexer.init(allocator, source1, "test1.💀", &error_reporter) catch |err| {
        print("Failed to initialize lexer: {any}\n", .{err});
        return;
    };
    defer lexer1.deinit();

    _ = lexer1.tokenize() catch |err| {
        print("Expected lexer error: {any}\n", .{err});
    };

    // Print lexical error diagnostics
    try error_reporter.printDiagnostics(std.io.getStdErr().writer());

    // Test 2: Multiple error types
    print("\n📝 Test 2: Multiple Error Categories\n", .{});
    print("-----------------------------------\n", .{});

    // Reset error reporter for new test
    var error_reporter2 = error_reporting.ErrorReporter.init(allocator, 10);
    defer error_reporter2.deinit();
    error_reporter2.setColors(true);

    const source2 = 
        \\sus invalid_char@ normie = 42
        \\facts missing_value tea
        \\'unterminated_char
        \\/* unterminated comment
        \\sus another_var tea = "valid"
    ;
    
    try error_reporter2.addSourceFile("test2.💀", source2);
    
    var lexer2 = enhanced_lexer.Lexer.init(allocator, source2, "test2.💀", &error_reporter2) catch |err| {
        print("Failed to initialize lexer: {any}\n", .{err});
        return;
    };
    defer lexer2.deinit();

    _ = lexer2.tokenize() catch {
        // Expect errors
    };

    try error_reporter2.printDiagnostics(std.io.getStdErr().writer());

    // Test 3: Demonstrate suggestion system
    print("\n📝 Test 3: Error Suggestions Demo\n", .{});
    print("----------------------------------\n", .{});

    var error_reporter3 = error_reporting.ErrorReporter.init(allocator, 5);
    defer error_reporter3.deinit();
    error_reporter3.setColors(true);

    // Manually create diagnostic with suggestions
    const location = error_reporting.SourceLocation.init("demo.💀", 10, 15, 120);
    
    try error_reporter3.reportError(
        .E201_UndefinedVariable,
        "Variable 'user_nme' is not defined",
        location
    );

    try error_reporter3.reportError(
        .E109_InvalidFunction,
        "Invalid function declaration syntax",
        error_reporting.SourceLocation.init("demo.💀", 15, 1, 200)
    );

    try error_reporter3.reportWarning(
        .E203_TypeMismatch,
        "Implicit conversion from 'normie' to 'tea' may lose precision",
        error_reporting.SourceLocation.init("demo.💀", 20, 25, 300)
    );

    try error_reporter3.printDiagnostics(std.io.getStdErr().writer());

    // Test 4: Performance and limits
    print("\n📝 Test 4: Error Limits and Performance\n", .{});
    print("---------------------------------------\n", .{});

    var error_reporter4 = error_reporting.ErrorReporter.init(allocator, 3); // Low limit
    defer error_reporter4.deinit();
    error_reporter4.setColors(true);

    // Generate many errors to test limits
    var i: u32 = 1;
    while (i <= 5) : (i += 1) {
        const loc = error_reporting.SourceLocation.init("stress_test.💀", i, 10, i * 50);
        const message = try std.fmt.allocPrint(allocator, "Error number {d} for testing", .{i});
        defer allocator.free(message);
        
        try error_reporter4.reportError(.E104_InvalidSyntax, message, loc);
    }

    try error_reporter4.printDiagnostics(std.io.getStdErr().writer());

    print("\n✅ Error Reporting Demo Complete!\n", .{});
    print("Features demonstrated:\n", .{});
    print("  • Rich error messages with source context\n", .{});
    print("  • Color-coded severity levels\n", .{});
    print("  • Helpful suggestions for common errors\n", .{});
    print("  • Error limits and performance handling\n", .{});
    print("  • Multiple error categories (lexical, syntax, semantic)\n", .{});
}
