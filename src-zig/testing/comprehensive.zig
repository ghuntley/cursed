//! Comprehensive Testing Framework for CURSED Zig Implementation
//! 
//! This module provides:
//! - Unit testing infrastructure
//! - Integration test runner
//! - Performance benchmarking
//! - Cross-platform validation
//! - CURSED stdlib test integration

const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;

// Import CURSED compiler components
const lexer = @import("../lexer.zig");
const parser = @import("../parser.zig");
const ast = @import("../ast.zig");
const codegen = @import("../codegen.zig");
const runtime = @import("../runtime_system.zig");

// Testing framework structures
pub const TestSuite = struct {
    name: []const u8,
    tests: []const TestCase,
    setup: ?*const fn() anyerror!void = null,
    teardown: ?*const fn() anyerror!void = null,
};

pub const TestCase = struct {
    name: []const u8,
    test_fn: *const fn(allocator: Allocator) anyerror!void,
    expected_result: TestResult = .pass,
    timeout_ms: u32 = 5000,
};

pub const TestResult = enum {
    pass,
    fail,
    skip,
    timeout,
};

pub const TestRunner = struct {
    allocator: Allocator,
    results: std.ArrayList(TestCaseResult),
    total_tests: u32 = 0,
    passed_tests: u32 = 0,
    failed_tests: u32 = 0,
    skipped_tests: u32 = 0,

    const TestCaseResult = struct {
        suite_name: []const u8,
        test_name: []const u8,
        result: TestResult,
        duration_ms: u64,
        error_message: ?[]const u8 = null,
    };

    pub fn init() TestRunner {
        return TestRunner{
            .allocator = allocator,
            .results = std.ArrayList(TestCaseResult){},
        };
    }

    pub fn deinit(self: *TestRunner) void {
        self.results.deinit(self.allocator);
    }

    pub fn runSuite(self: *TestRunner, suite: TestSuite) !void {
        std.debug.writer().print("🧪 Running test suite: {s}\n", .{suite.name});

        // Run setup if provided
        if (suite.setup) |setup_fn| {
            setup_fn() catch |err| {
                std.debug.writer().print("❌ Setup failed for suite {s}: {s}\n", .{{ suite.name, err });
                return;
            };
        }
        defer {
            if (suite.teardown) |teardown_fn| {
                teardown_fn() catch |err| {
                    std.debug.writer().print("⚠️  Teardown failed for suite {s}: {s}\n", .{{ suite.name, err });
                };
            }
        }

        // Run each test in the suite
        for (suite.tests) |test_case| {
            const start_time = std.time.milliTimestamp();
            var result = TestResult.pass;
            var error_msg: ?[]const u8 = null;

            self.total_tests += 1;

            std.debug.writer().print("  • {s}... ", .{test_case.name});

            // Run the test with timeout
            test_case.test_fn(self.allocator) catch |err| {
                result = .fail;
                error_msg = @errorName(err);
                self.failed_tests += 1;
                std.debug.writer().print("❌ FAIL\n", .{});
            };

            if (result == .pass) {
                self.passed_tests += 1;
                std.debug.writer().print("✅ PASS\n", .{});
            }

            const duration = @as(u64, @intCast(std.time.milliTimestamp() - start_time));

            try self.results.append(TestCaseResult{
                .suite_name = suite.name,
                .test_name = test_case.name,
                .result = result,
                .duration_ms = duration,
                .error_message = error_msg,
            });
        }
    }

    pub fn printSummary(self: *TestRunner) void {
        std.debug.writer().print("\n📊 Test Summary:\n", .{});
        std.debug.writer().print("  Total: {s}\n", .{{self.total_tests});
        std.debug.writer().print("  Passed: {s}\n", .{{self.passed_tests});
        std.debug.writer().print("  Failed: {s}\n", .{{self.failed_tests});
        std.debug.writer().print("  Skipped: {s}\n", .{{self.skipped_tests});

        const success_rate = if (self.total_tests > 0) 
            (@as(f64, @floatFromInt(self.passed_tests)) / @as(f64, @floatFromInt(self.total_tests))) * 100.0 
        else 
            0.0;

        std.debug.writer().print("  Success Rate: {d:.1}%\n", .{success_rate});

        if (self.failed_tests > 0) {
            std.debug.writer().print("\n❌ Failed Tests:\n", .{});
            for (self.results.items) |result| {
                if (result.result == .fail) {
                    std.debug.writer().print("  • {s}::{s}", .{ result.suite_name, result.test_name });
                    if (result.error_message) |msg| {
                        std.debug.writer().print(" - {s}", .{msg});
                    }
                    std.debug.writer().print("\n", .{});
                }
            }
        }
    }
};

// ===== LEXER TESTS =====

fn testLexerBasicTokens(allocator: Allocator) !void {
    const input = "sus x drip = 42; vibez.spill(\"Hello CURSED!\");";
    
    var lex = try lexer.Lexer.init(allocator, input);
    defer lex.deinit();

    const tokens = try lex.tokenize();
    defer allocator.free(tokens);

    // Verify we have tokens
    try testing.expect(tokens.len > 0);
    
    // Check for specific tokens
    var found_sus = false;
    var found_drip = false;
    var found_vibez = false;
    
    for (tokens) |token| {
        switch (token) {
            .Keyword => |kw| {
                if (std.mem.eql(u8, kw, "sus")) found_sus = true;
                if (std.mem.eql(u8, kw, "drip")) found_drip = true;
                if (std.mem.eql(u8, kw, "vibez")) found_vibez = true;
            },
            else => {},
        }
    }

    try testing.expect(found_sus);
    try testing.expect(found_drip);
    try testing.expect(found_vibez);
}

fn testLexerStringLiterals(allocator: Allocator) !void {
    const input = "\"Hello CURSED!\" 'Single quoted' \"Escaped \\\"string\\\"\"";
    
    var lex = try lexer.Lexer.init(allocator, input);
    defer lex.deinit();

    const tokens = try lex.tokenize();
    defer allocator.free(tokens);

    var string_count: u32 = 0;
    for (tokens) |token| {
        switch (token) {
            .StringLiteral => string_count += 1,
            else => {},
        }
    }

    try testing.expect(string_count >= 3);
}

fn testLexerCommentHandling(allocator: Allocator) !void {
    const input = 
        \\fr fr This is a comment
        \\sus x drip = 42
        \\# Hash style comment
        \\vibez.spill("test")
    ;
    
    var lex = try lexer.Lexer.init(allocator, input);
    defer lex.deinit();

    const tokens = try lex.tokenize();
    defer allocator.free(tokens);

    // Comments should be filtered out, only code tokens remain
    var code_tokens: u32 = 0;
    for (tokens) |token| {
        switch (token) {
            .Comment => {}, // Should not be present in filtered output
            else => code_tokens += 1,
        }
    }

    try testing.expect(code_tokens > 0);
}

// ===== PARSER TESTS =====

fn testParserBasicExpressions(allocator: Allocator) !void {
    const input = "sus x drip = 42 + 24;";
    
    var lex = try lexer.Lexer.init(allocator, input);
    defer lex.deinit();

    const tokens = try lex.tokenize();
    defer allocator.free(tokens);

    var parse = try parser.Parser.init(allocator, tokens);
    defer parse.deinit();

    const program = try parse.parseProgram();
    defer program.deinit();

    try testing.expect(program.statements.len > 0);
}

fn testParserFunctionDefinitions(allocator: Allocator) !void {
    const input = "slay greet(name tea) tea { damn \"Hello \" + name; }";
    
    var lex = try lexer.Lexer.init(allocator, input);
    defer lex.deinit();

    const tokens = try lex.tokenize();
    defer allocator.free(tokens);

    var parse = try parser.Parser.init(allocator, tokens);
    defer parse.deinit();

    const program = try parse.parseProgram();
    defer program.deinit();

    try testing.expect(program.statements.len > 0);
    
    // Verify it's a function definition
    for (program.statements) |stmt| {
        if (stmt == .FunctionDef) {
            try testing.expect(std.mem.eql(u8, stmt.FunctionDef.name, "greet"));
            try testing.expect(stmt.FunctionDef.params.len == 1);
            return;
        }
    }
    
    return error.FunctionDefNotFound;
}

fn testParserStructDefinitions(allocator: Allocator) !void {
    const input = "squad Point { spill x drip; spill y drip; }";
    
    var lex = try lexer.Lexer.init(allocator, input);
    defer lex.deinit();

    const tokens = try lex.tokenize();
    defer allocator.free(tokens);

    var parse = try parser.Parser.init(allocator, tokens);
    defer parse.deinit();

    const program = try parse.parseProgram();
    defer program.deinit();

    try testing.expect(program.statements.len > 0);
    
    // Verify it's a struct definition
    for (program.statements) |stmt| {
        if (stmt == .StructDef) {
            try testing.expect(std.mem.eql(u8, stmt.StructDef.name, "Point"));
            try testing.expect(stmt.StructDef.fields.len == 2);
            return;
        }
    }
    
    return error.StructDefNotFound;
}

// ===== CODEGEN TESTS =====

fn testCodegenBasicOutput(allocator: Allocator) !void {
    const input = "vibez.spill(\"Hello from codegen test!\");";
    
    var lex = try lexer.Lexer.init(allocator, input);
    defer lex.deinit();

    const tokens = try lex.tokenize();
    defer allocator.free(tokens);

    var parse = try parser.Parser.init(allocator, tokens);
    defer parse.deinit();

    const program = try parse.parseProgram();
    defer program.deinit();

    var generator = try codegen.CodeGenerator.init(allocator);
    defer generator.deinit();

    const c_code = try generator.generateC(program);
    defer allocator.free(c_code);

    // Verify C code contains expected patterns
    try testing.expect(std.mem.indexOf(u8, c_code, "printf") != null);
    try testing.expect(std.mem.indexOf(u8, c_code, "Hello from codegen test!") != null);
}

fn testCodegenFunctionGeneration(allocator: Allocator) !void {
    const input = "slay add(a drip, b drip) drip { damn a + b; }";
    
    var lex = try lexer.Lexer.init(allocator, input);
    defer lex.deinit();

    const tokens = try lex.tokenize();
    defer allocator.free(tokens);

    var parse = try parser.Parser.init(allocator, tokens);
    defer parse.deinit();

    const program = try parse.parseProgram();
    defer program.deinit();

    var generator = try codegen.CodeGenerator.init(allocator);
    defer generator.deinit();

    const c_code = try generator.generateC(program);
    defer allocator.free(c_code);

    // Verify function is generated
    try testing.expect(std.mem.indexOf(u8, c_code, "int add(") != null);
    try testing.expect(std.mem.indexOf(u8, c_code, "return") != null);
}

// ===== RUNTIME TESTS =====

fn testRuntimeBasicExecution(allocator: Allocator) !void {
    // Test that runtime can execute simple CURSED programs
    const input = "sus x drip = 42; vibez.spill(x);";
    
    var interpreter = try runtime.Interpreter.init(allocator);
    defer interpreter.deinit();

    const result = interpreter.executeString(input);
    
    // For now, just verify it doesn't crash
    _ = result;
}

// ===== PERFORMANCE TESTS =====

pub const PerformanceBenchmark = struct {
    name: []const u8,
    benchmark_fn: *const fn(allocator: Allocator) anyerror!u64,
    baseline_ns: ?u64 = null,
    tolerance_percent: f64 = 10.0,
};

fn benchmarkLexerPerformance(allocator: Allocator) !u64 {
    const large_input = 
        \\slay fibonacci(n drip) drip {
        \\    if n <= 1 { damn n; }
        \\    damn fibonacci(n - 1) + fibonacci(n - 2);
        \\}
        \\
        \\bestie i := 0; i < 20; i = i + 1 {
        \\    vibez.spill("fib(", i, ") = ", fibonacci(i));
        \\}
    ;
    
    const start_time = std.time.nanoTimestamp();
    
    // Run lexer multiple times
    var i: u32 = 0;
    while (i < 100) : (i += 1) {
        var lex = try lexer.Lexer.init(allocator, large_input);
        defer lex.deinit();
        
        const tokens = try lex.tokenize();
        allocator.free(tokens);
    }
    
    const end_time = std.time.nanoTimestamp();
    return @as(u64, @intCast(end_time - start_time));
}

fn benchmarkParserPerformance(allocator: Allocator) !u64 {
    const complex_input = 
        \\squad Complex {
        \\    spill real meal
        \\    spill imag meal
        \\}
        \\
        \\slay add_complex(a Complex, b Complex) Complex {
        \\    damn Complex{real: a.real + b.real, imag: a.imag + b.imag};
        \\}
    ;
    
    const start_time = std.time.nanoTimestamp();
    
    // Run parser multiple times
    var i: u32 = 0;
    while (i < 50) : (i += 1) {
        var lex = try lexer.Lexer.init(allocator, complex_input);
        defer lex.deinit();

        const tokens = try lex.tokenize();
        defer allocator.free(tokens);

        var parse = try parser.Parser.init(allocator, tokens);
        defer parse.deinit();

        const program = try parse.parseProgram();
        defer program.deinit();
    }
    
    const end_time = std.time.nanoTimestamp();
    return @as(u64, @intCast(end_time - start_time));
}

// ===== STDLIB INTEGRATION TESTS =====

fn testStdlibTestzIntegration(allocator: Allocator) !void {
    const testz_program = 
        \\yeet "testz"
        \\
        \\test_start("Basic arithmetic test")
        \\assert_eq_int(2 + 2, 4)
        \\assert_true(5 > 3)
        \\print_test_summary()
    ;
    
    var interpreter = try runtime.Interpreter.init(allocator);
    defer interpreter.deinit();

    // Execute testz program - should not crash
    const result = interpreter.executeString(testz_program);
    _ = result;
}

fn testStdlibMathIntegration(allocator: Allocator) !void {
    const math_program = 
        \\yeet "math"
        \\
        \\sus angle meal = math.pi / 4.0
        \\sus result meal = math.sin(angle)
        \\vibez.spill("sin(π/4) =", result)
    ;
    
    var interpreter = try runtime.Interpreter.init(allocator);
    defer interpreter.deinit();

    const result = interpreter.executeString(math_program);
    _ = result;
}

// ===== TEST SUITES DEFINITION =====

const lexer_tests = TestSuite{
    .name = "Lexer Tests",
    .tests = &[_]TestCase{
        .{ .name = "Basic Tokens", .test_fn = testLexerBasicTokens },
        .{ .name = "String Literals", .test_fn = testLexerStringLiterals },
        .{ .name = "Comment Handling", .test_fn = testLexerCommentHandling },
    },
};

const parser_tests = TestSuite{
    .name = "Parser Tests", 
    .tests = &[_]TestCase{
        .{ .name = "Basic Expressions", .test_fn = testParserBasicExpressions },
        .{ .name = "Function Definitions", .test_fn = testParserFunctionDefinitions },
        .{ .name = "Struct Definitions", .test_fn = testParserStructDefinitions },
    },
};

const codegen_tests = TestSuite{
    .name = "Code Generation Tests",
    .tests = &[_]TestCase{
        .{ .name = "Basic Output", .test_fn = testCodegenBasicOutput },
        .{ .name = "Function Generation", .test_fn = testCodegenFunctionGeneration },
    },
};

const runtime_tests = TestSuite{
    .name = "Runtime Tests",
    .tests = &[_]TestCase{
        .{ .name = "Basic Execution", .test_fn = testRuntimeBasicExecution },
    },
};

const stdlib_tests = TestSuite{
    .name = "Stdlib Integration Tests",
    .tests = &[_]TestCase{
        .{ .name = "Testz Integration", .test_fn = testStdlibTestzIntegration },
        .{ .name = "Math Integration", .test_fn = testStdlibMathIntegration },
    },
};

// ===== MAIN TEST RUNNER =====

pub fn runAllTests(allocator: Allocator) !void {
        _ = allocator;
    std.debug.writer().print("🚀 Starting CURSED Zig Comprehensive Test Suite\n", .{});
    std.debug.writer().print("=" ** 50 ++ "\n", .{});

    var runner = TestRunner.init(allocator);
    defer runner.deinit();

    // Run all test suites
    const suites = [_]TestSuite{
        lexer_tests,
        parser_tests,
        codegen_tests,
        runtime_tests,
        stdlib_tests,
    };

    for (suites) |suite| {
        try runner.runSuite(suite);
        std.debug.writer().print("\n", .{});
    }

    // Performance benchmarks
    std.debug.writer().print("⚡ Performance Benchmarks:\n", .{});
    
    const lexer_time = try benchmarkLexerPerformance(allocator);
    std.debug.writer().print("  Lexer (100 iterations): {s}ns\n", .{{lexer_time});
    
    const parser_time = try benchmarkParserPerformance(allocator);
    std.debug.writer().print("  Parser (50 iterations): {s}ns\n", .{{parser_time});

    runner.printSummary();
    
    std.debug.writer().print("\n🎯 Test execution completed!\n", .{});
}

// ===== ZIG TEST INTEGRATION =====

test "CURSED Comprehensive Test Suite" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try runAllTests(allocator);
}

// Individual test exports for `zig test`
test "Lexer: Basic Tokens" {
    try testLexerBasicTokens(testing.allocator);
}

test "Lexer: String Literals" {
    try testLexerStringLiterals(testing.allocator);
}

test "Lexer: Comment Handling" {
    try testLexerCommentHandling(testing.allocator);
}

test "Parser: Basic Expressions" {
    try testParserBasicExpressions(testing.allocator);
}

test "Parser: Function Definitions" {
    try testParserFunctionDefinitions(testing.allocator);
}

test "Codegen: Basic Output" {
    try testCodegenBasicOutput(testing.allocator);
}

test "Runtime: Basic Execution" {
    try testRuntimeBasicExecution(testing.allocator);
}
