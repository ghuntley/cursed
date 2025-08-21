const std = @import("std");
const print = std.debug.print;
const diagnostics = @import("error_diagnostics.zig");

const ErrorHandler = diagnostics.ErrorHandler;
const ErrorDiagnostic = diagnostics.ErrorDiagnostic;
const StackFrame = diagnostics.StackFrame;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Initialize error handler
    var handler = ErrorHandler.init(allocator);
    defer handler.deinit();
    
    // Sample CURSED source code with errors
    const source_code = 
        \\slay main() {
        \\    sus x normie = "string instead of number"
        \\    sus y = 
        \\    vibez.spill(undefinedVariable)
        \\    
        \\    facts constant = 42
        \\    constant = 24
        \\    
        \\    bestie (i drip = 0; i < 10; i = i + 1 {
        \\        vibez.spill("missing closing brace")
        \\    
        \\    squad Person {
        \\        name tea;
        \\        age normie;
        \\    }
        \\    
        \\    sus person Person = Person { name: "John", age: 30 }
        \\    vibez.spill(person.height)
        \\}
    ;
    
    // Add source file to engine
    try engine.addSourceFile("demo.csd", source_code);
    
    print("=== CURSED Error Diagnostics System Demo ===\n\n", .{});
    
    // Demonstrate various error types
    
    // 1. Type mismatch error
    const type_mismatch_span = SourceSpan.init("demo.csd", 2, 20, 2, 45, 39, 64);
    try engine.reportError(
        .S003_TypeMismatch,
        "Cannot assign 'tea' to variable of type 'normie'",
        type_mismatch_span
    );
    
    // 2. Missing expression error  
    const missing_expr_span = SourceSpan.init("demo.csd", 3, 13, 3, 13, 77, 77);
    try engine.reportError(
        .P005_MissingExpression,
        "Expected expression after '='",
        missing_expr_span
    );
    
    // 3. Undefined variable error
    const undefined_var_span = SourceSpan.init("demo.csd", 4, 17, 4, 32, 95, 110);
    try engine.reportError(
        .S001_UndefinedVariable,
        "Variable 'undefinedVariable' is not defined in current scope",
        undefined_var_span
    );
    
    // 4. Immutable assignment error
    const immutable_span = SourceSpan.init("demo.csd", 7, 5, 7, 13, 140, 148);
    try engine.reportError(
        .S020_ImmutableAssignment,
        "Cannot assign to immutable variable 'constant'",
        immutable_span
    );
    
    // 5. Unbalanced braces error (multi-line)
    const unbalanced_span = SourceSpan.init("demo.csd", 9, 48, 10, 36, 187, 224);
    try engine.reportError(
        .P008_UnbalancedBraces,
        "Missing closing brace for 'bestie' loop",
        unbalanced_span
    );
    
    // 6. Undefined field error
    const undefined_field_span = SourceSpan.init("demo.csd", 17, 24, 17, 30, 350, 356);
    try engine.reportError(
        .S008_UndefinedField,
        "Struct 'Person' has no field named 'height'",
        undefined_field_span
    );
    
    // 7. Warning for unused variable
    const unused_var_span = SourceSpan.init("demo.csd", 16, 9, 16, 15, 320, 326);
    try engine.reportWarning(
        .S007_UnreachableCode,
        "Variable 'person' is defined but never used",
        unused_var_span
    );
    
    // 8. Hint for better syntax
    const hint_span = SourceSpan.init("demo.csd", 2, 5, 2, 17, 24, 36);
    try engine.reportHint(
        .P007_InvalidType,
        "Consider using explicit type annotation for clarity",
        hint_span
    );
    
    // Print all diagnostics with colors
    var stdout_buffer: [4096]u8 = undefined;
    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
    try engine.printDiagnostics(stdout);
    
    // Demonstrate diagnostic counts
    print("\n=== Diagnostic Summary ===\n", .{});
    print("Total errors: {d}\n", .{engine.getErrorCount()});
    print("Total warnings: {d}\n", .{engine.getWarningCount()});
    print("Has errors: {}\n", .{engine.hasErrors()});
    print("Has warnings: {}\n", .{engine.hasWarnings()});
    
    // Test without colors for comparison
    print("\n=== Same Output Without Colors ===\n", .{});
    engine.setColors(false);
    engine.setUnicode(false);
    
    // Clear and re-add one diagnostic for demonstration
    engine.diagnostics.clearAndFree();
    engine.error_count = 0;
    engine.warning_count = 0;
    
    try engine.reportError(
        .S003_TypeMismatch,
        "Cannot assign 'tea' to variable of type 'normie'",
        type_mismatch_span
    );
    
    try engine.printDiagnostics(stdout);
}

// Test function demonstrating integration with lexer
pub fn demonstrateLexerIntegration() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var engine = DiagnosticEngine.init(allocator, 10);
    defer engine.deinit();
    
    const bad_source = "sus x tea = \"unterminated string";
    try engine.addSourceFile("lexer_test.csd", bad_source);
    
    // Simulate lexer error
    try diagnostics.lexerError(
        &engine,
        .L001_UnterminatedString,
        "String literal is missing closing quote",
        "lexer_test.csd",
        1,
        13,
        12
    );
    
    var stdout_buffer: [4096]u8 = undefined;
    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
    try engine.printDiagnostics(stdout);
}

// Test function demonstrating integration with parser
pub fn demonstrateParserIntegration() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var engine = DiagnosticEngine.init(allocator, 10);
    defer engine.deinit();
    
    const bad_source = 
        \\slay badFunction() {
        \\    sus x = 42
        \\    // missing closing brace
    ;
    
    try engine.addSourceFile("parser_test.csd", bad_source);
    
    // Simulate parser error
    try diagnostics.parserError(
        &engine,
        .P003_UnexpectedEOF,
        "Unexpected end of file, expected closing brace",
        "parser_test.csd",
        1, 1,    // start line, column
        3, 24,   // end line, column  
        0, 42    // start offset, end offset
    );
    
    var stdout_buffer: [4096]u8 = undefined;
    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
    try engine.printDiagnostics(stdout);
}

// Test function demonstrating semantic analysis integration
pub fn demonstrateSemanticIntegration() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var engine = DiagnosticEngine.init(allocator, 10);
    defer engine.deinit();
    
    const bad_source = 
        \\slay main() {
        \\    sus x normie = 42
        \\    sus y tea = x  // type mismatch
        \\}
    ;
    
    try engine.addSourceFile("semantic_test.csd", bad_source);
    
    // Primary error span
    const primary_span = SourceSpan.init("semantic_test.csd", 3, 17, 3, 18, 47, 48);
    
    // Related span for variable definition
    const related_spans = [_]SourceSpan{
        SourceSpan.init("semantic_test.csd", 2, 9, 2, 10, 20, 21),
    };
    
    // Simulate semantic error with related information
    try diagnostics.semanticError(
        &engine,
        .S003_TypeMismatch,
        "Cannot assign 'normie' to variable of type 'tea'",
        primary_span,
        &related_spans
    );
    
    var stdout_buffer: [4096]u8 = undefined;
    const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
    try engine.printDiagnostics(stdout);
}
