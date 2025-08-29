const std = @import("std");
const print = std.debug.print;
const error_reporting = @import("enhanced_error_reporting.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🧪 Simple Error Reporting Test\n", .{});

    // Test basic error reporting without lexer
    var error_reporter = error_reporting.ErrorReporter.init(allocator, 5);
    defer error_reporter.deinit();
    
    error_reporter.setColors(true);

    // Create some sample errors
    const location1 = error_reporting.SourceLocation.init("test.csd", 1, 10, 9);
    const location2 = error_reporting.SourceLocation.init("test.csd", 2, 5, 25);
    const location3 = error_reporting.SourceLocation.init("test.csd", 3, 15, 45);

    // Add source for context
    const source = "sus x normie = 42\nsus y tea = \"hello\"\nsus z lit = based";
    try error_reporter.addSourceFile("test.csd", source);

    // Report different types of errors
    try error_reporter.reportError(
        .E201_UndefinedVariable,
        "Variable 'undefined_var' is not defined",
        location1
    );

    try error_reporter.reportWarning(
        .E203_TypeMismatch,
        "Type mismatch: expected 'normie', found 'tea'",
        location2
    );

    try error_reporter.reportError(
        .E109_InvalidFunction,
        "Invalid function declaration syntax",
        location3
    );

    // Print all diagnostics
    try error_reporter.printDiagnostics(std.io.getStdErr().writer());

    print("\n✅ Error reporting test completed!\n", .{});
    print("Error count: {s}\n", .{error_reporter.getErrorCount()});
    print("Warning count: {s}\n", .{error_reporter.getWarningCount()});
}
