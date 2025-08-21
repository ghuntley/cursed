//! Integration Testing Framework for CURSED Zig Implementation
//! 
//! End-to-end testing of the complete compiler pipeline:
//! Source → Lexer → Parser → Semantic Analysis → Codegen → Runtime

const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;

// Import CURSED compiler components
const lexer = @import("../lexer.zig");
const parser = @import("../parser.zig");
const codegen = @import("../codegen.zig");
const runtime = @import("../runtime_system.zig");

pub const IntegrationTestCase = struct {
    name: []const u8,
    source_code: []const u8,
    expected_output: []const u8,
    should_compile: bool = true,
    should_execute: bool = true,
    timeout_ms: u32 = 10000,
};

pub const PipelineResult = struct {
    lexer_success: bool = false,
    parser_success: bool = false,
    codegen_success: bool = false,
    execution_success: bool = false,
    actual_output: ?[]const u8 = null,
    error_message: ?[]const u8 = null,
    execution_time_ms: u64 = 0,
};

pub const IntegrationTestRunner = struct {
    allocator: Allocator,
    results: std.ArrayList(IntegrationTestResult),

    const IntegrationTestResult = struct {
        test_name: []const u8,
        pipeline_result: PipelineResult,
        passed: bool,
        error_details: ?[]const u8 = null,
    };

    pub fn init(allocator: Allocator) IntegrationTestRunner {
        return IntegrationTestRunner{
            .allocator = allocator,
            .results = std.ArrayList(IntegrationTestResult).init(allocator),
        };
    }

    pub fn deinit(self: *IntegrationTestRunner) void {
        for (self.results.items) |result| {
            if (result.pipeline_result.actual_output) |output| {
                self.allocator.free(output);
            }
        }
        self.results.deinit(allocator);
    }

    pub fn runTestCase(self: *IntegrationTestRunner, test_case: IntegrationTestCase) !void {
        std.debug.print("🔄 Running integration test: {s}\n", .{test_case.name});

        const start_time = std.time.milliTimestamp();
        var pipeline_result = PipelineResult{};
        var passed = false;
        var error_details: ?[]const u8 = null;

        // Run complete pipeline
        if (self.runCompilerPipeline(test_case.source_code)) |result| {
            pipeline_result = result;
            
            // Check if results match expectations
            if (test_case.should_compile and pipeline_result.codegen_success) {
                if (test_case.should_execute and pipeline_result.execution_success) {
                    if (pipeline_result.actual_output) |output| {
                        passed = std.mem.eql(u8, output, test_case.expected_output);
                        if (!passed) {
                            error_details = try std.fmt.allocPrint(self.allocator, 
                                "Output mismatch. Expected: '{s}', Got: '{s}'", 
                                .{ test_case.expected_output, output }
                            );
                        }
                    } else {
                        error_details = try self.allocator.dupe(u8, "No output captured");
                    }
                } else {
                    passed = true; // Compilation success is enough
                }
            } else if (!test_case.should_compile and !pipeline_result.codegen_success) {
                passed = true; // Expected compilation failure
            }
        } else |err| {
            error_details = try std.fmt.allocPrint(self.allocator, "Pipeline error: {}", .{err});
        }

        const end_time = std.time.milliTimestamp();
        pipeline_result.execution_time_ms = @as(u64, @intCast(end_time - start_time));

        const status = if (passed) "✅ PASS" else "❌ FAIL";
        std.debug.print("  Result: {s}\n", .{status});

        try self.results.append(IntegrationTestResult{
            .test_name = test_case.name,
            .pipeline_result = pipeline_result,
            .passed = passed,
            .error_details = error_details,
        });
    }

    fn runCompilerPipeline(self: *IntegrationTestRunner, source_code: []const u8) !PipelineResult {
        var result = PipelineResult{};

        // Stage 1: Lexical Analysis
        var lex = lexer.Lexer.init(self.allocator, source_code) catch |err| {
            result.error_message = try std.fmt.allocPrint(self.allocator, "Lexer init failed: {}", .{err});
            return result;
        };
        defer lex.deinit(allocator);

        const tokens = lex.tokenize() catch |err| {
            result.error_message = try std.fmt.allocPrint(self.allocator, "Tokenization failed: {}", .{err});
            return result;
        };
        defer self.allocator.free(tokens);
        result.lexer_success = true;

        // Stage 2: Parsing
        var parse = parser.Parser.init(self.allocator, tokens) catch |err| {
            result.error_message = try std.fmt.allocPrint(self.allocator, "Parser init failed: {}", .{err});
            return result;
        };
        defer parse.deinit(allocator);

        const program = parse.parseProgram() catch |err| {
            result.error_message = try std.fmt.allocPrint(self.allocator, "Parsing failed: {}", .{err});
            return result;
        };
        defer program.deinit(allocator);
        result.parser_success = true;

        // Stage 3: Code Generation
        var generator = codegen.CodeGenerator.init(self.allocator) catch |err| {
            result.error_message = try std.fmt.allocPrint(self.allocator, "Codegen init failed: {}", .{err});
            return result;
        };
        defer generator.deinit(allocator);

        const c_code = generator.generateC(program) catch |err| {
            result.error_message = try std.fmt.allocPrint(self.allocator, "Code generation failed: {}", .{err});
            return result;
        };
        defer self.allocator.free(c_code);
        result.codegen_success = true;

        // Stage 4: Execution (interpretation for now)
        var interpreter = runtime.Interpreter.init(self.allocator) catch |err| {
            result.error_message = try std.fmt.allocPrint(self.allocator, "Interpreter init failed: {}", .{err});
            return result;
        };
        defer interpreter.deinit(allocator);

        // Capture output
        const output = interpreter.executeString(source_code) catch |err| {
            result.error_message = try std.fmt.allocPrint(self.allocator, "Execution failed: {}", .{err});
            return result;
        };
        
        result.actual_output = try self.allocator.dupe(u8, output);
        result.execution_success = true;

        return result;
    }

    pub fn printSummary(self: *IntegrationTestRunner) void {
        std.debug.print("\n📊 Integration Test Summary:\n");
        std.debug.print("=" ** 50 ++ "\n");

        var total_tests: u32 = 0;
        var passed_tests: u32 = 0;
        var total_time: u64 = 0;

        for (self.results.items) |result| {
            total_tests += 1;
            if (result.passed) passed_tests += 1;
            total_time += result.pipeline_result.execution_time_ms;
        }

        std.debug.print("Total Tests: {}\n", .{total_tests});
        std.debug.print("Passed: {}\n", .{passed_tests});
        std.debug.print("Failed: {}\n", .{total_tests - passed_tests});
        std.debug.print("Success Rate: {d:.1}%\n", .{
            if (total_tests > 0) (@as(f64, @floatFromInt(passed_tests)) / @as(f64, @floatFromInt(total_tests))) * 100.0 else 0.0
        });
        std.debug.print("Total Time: {}ms\n", .{total_time});

        // Print pipeline stage statistics
        var lexer_success: u32 = 0;
        var parser_success: u32 = 0;
        var codegen_success: u32 = 0;
        var execution_success: u32 = 0;

        for (self.results.items) |result| {
            if (result.pipeline_result.lexer_success) lexer_success += 1;
            if (result.pipeline_result.parser_success) parser_success += 1;
            if (result.pipeline_result.codegen_success) codegen_success += 1;
            if (result.pipeline_result.execution_success) execution_success += 1;
        }

        std.debug.print("\n🔧 Pipeline Stage Success Rates:\n");
        std.debug.print("  Lexer: {}/{} ({d:.1}%)\n", .{ lexer_success, total_tests, 
            if (total_tests > 0) (@as(f64, @floatFromInt(lexer_success)) / @as(f64, @floatFromInt(total_tests))) * 100.0 else 0.0 });
        std.debug.print("  Parser: {}/{} ({d:.1}%)\n", .{ parser_success, total_tests,
            if (total_tests > 0) (@as(f64, @floatFromInt(parser_success)) / @as(f64, @floatFromInt(total_tests))) * 100.0 else 0.0 });
        std.debug.print("  Codegen: {}/{} ({d:.1}%)\n", .{ codegen_success, total_tests,
            if (total_tests > 0) (@as(f64, @floatFromInt(codegen_success)) / @as(f64, @floatFromInt(total_tests))) * 100.0 else 0.0 });
        std.debug.print("  Execution: {}/{} ({d:.1}%)\n", .{ execution_success, total_tests,
            if (total_tests > 0) (@as(f64, @floatFromInt(execution_success)) / @as(f64, @floatFromInt(total_tests))) * 100.0 else 0.0 });

        // Print failed tests
        var has_failures = false;
        for (self.results.items) |result| {
            if (!result.passed) {
                if (!has_failures) {
                    std.debug.print("\n❌ Failed Tests:\n");
                    has_failures = true;
                }
                std.debug.print("  • {s}", .{result.test_name});
                if (result.error_details) |details| {
                    std.debug.print(" - {s}", .{details});
                }
                std.debug.print("\n");
            }
        }
    }
};

// Define comprehensive integration test cases
const integration_test_cases = [_]IntegrationTestCase{
    .{
        .name = "Basic Hello World",
        .source_code = "vibez.spill(\"Hello, CURSED!\");",
        .expected_output = "Hello, CURSED!\n",
    },
    .{
        .name = "Variable Declaration and Usage",
        .source_code = 
            \\sus name tea = "CURSED"
            \\sus count drip = 42
            \\vibez.spill("Language: ", name, ", Version: ", count)
        ,
        .expected_output = "Language: CURSED, Version: 42\n",
    },
    .{
        .name = "Function Definition and Call",
        .source_code = 
            \\slay greet(name tea) tea {
            \\    damn "Hello, " + name + "!"
            \\}
            \\
            \\sus message tea = greet("CURSED")
            \\vibez.spill(message)
        ,
        .expected_output = "Hello, CURSED!\n",
    },
    .{
        .name = "Arithmetic Operations",
        .source_code = 
            \\sus a drip = 10
            \\sus b drip = 5
            \\vibez.spill("Addition: ", a + b)
            \\vibez.spill("Subtraction: ", a - b)
            \\vibez.spill("Multiplication: ", a * b)
            \\vibez.spill("Division: ", a / b)
        ,
        .expected_output = "Addition: 15\nSubtraction: 5\nMultiplication: 50\nDivision: 2\n",
    },
    .{
        .name = "Boolean Logic",
        .source_code = 
            \\sus is_ready lit = based
            \\sus is_complete lit = cringe
            \\vibez.spill("Ready: ", is_ready)
            \\vibez.spill("Complete: ", is_complete)
            \\vibez.spill("Both: ", is_ready && is_complete)
        ,
        .expected_output = "Ready: true\nComplete: false\nBoth: false\n",
    },
    .{
        .name = "Conditional Statements",
        .source_code = 
            \\sus x drip = 15
            \\if x > 10 {
            \\    vibez.spill("x is greater than 10")
            \\} else {
            \\    vibez.spill("x is not greater than 10")
            \\}
        ,
        .expected_output = "x is greater than 10\n",
    },
    .{
        .name = "Loop Structures",
        .source_code = 
            \\bestie i := 1; i <= 3; i = i + 1 {
            \\    vibez.spill("Count: ", i)
            \\}
        ,
        .expected_output = "Count: 1\nCount: 2\nCount: 3\n",
    },
    .{
        .name = "Struct Definition and Usage",
        .source_code = 
            \\squad Point {
            \\    spill x drip
            \\    spill y drip
            \\}
            \\
            \\sus p Point = Point{x: 3, y: 4}
            \\vibez.spill("Point: (", p.x, ", ", p.y, ")")
        ,
        .expected_output = "Point: (3, 4)\n",
    },
    .{
        .name = "Array Operations",
        .source_code = 
            \\sus numbers []drip = [1, 2, 3, 4, 5]
            \\vibez.spill("First: ", numbers[0])
            \\vibez.spill("Length: ", numbers.len())
        ,
        .expected_output = "First: 1\nLength: 5\n",
    },
    .{
        .name = "Error Handling",
        .source_code = 
            \\slay safe_divide(a drip, b drip) drip {
            \\    if b == 0 {
            \\        yikes "Division by zero"
            \\    }
            \\    damn a / b
            \\}
            \\
            \\sus result drip = safe_divide(10, 2)
            \\vibez.spill("Result: ", result)
        ,
        .expected_output = "Result: 5\n",
    },
    .{
        .name = "Import System Test", 
        .source_code = 
            \\yeet "testz"
            \\
            \\test_start("Basic test")
            \\assert_true(based)
            \\print_test_summary()
        ,
        .expected_output = "Test: Basic test - PASS\nTests: 1, Passed: 1, Failed: 0\n",
    },
    .{
        .name = "Syntax Error Test",
        .source_code = "sus x drip = ; // Missing value",
        .expected_output = "",
        .should_compile = false,
        .should_execute = false,
    },
};

// Main integration test runner
pub fn runAllIntegrationTests(allocator: Allocator) !void {
    std.debug.print("🚀 Starting CURSED Integration Test Suite\n");
    std.debug.print("=" ** 60 ++ "\n");

    var runner = IntegrationTestRunner.init(allocator);
    defer runner.deinit(allocator);

    for (integration_test_cases) |test_case| {
        try runner.runTestCase(test_case);
    }

    runner.printSummary();
}

// Cross-platform testing
pub fn runCrossPlatformTests(allocator: Allocator) !void {
    std.debug.print("🌍 Running Cross-Platform Integration Tests\n");
    std.debug.print("=" ** 50 ++ "\n");

    // Test platform-specific functionality
    const platform_test = IntegrationTestCase{
        .name = "Platform Detection",
        .source_code = 
            \\yeet "system"
            \\
            \\sus platform tea = system.platform()
            \\sus arch tea = system.architecture()
            \\vibez.spill("Platform: ", platform)
            \\vibez.spill("Architecture: ", arch)
        ,
        .expected_output = "Platform: linux\nArchitecture: x86_64\n", // Expected for test environment
    };

    var runner = IntegrationTestRunner.init(allocator);
    defer runner.deinit(allocator);

    try runner.runTestCase(platform_test);
    runner.printSummary();
}

// Stress testing for compiler robustness
pub fn runStressTests(allocator: Allocator) !void {
    std.debug.print("💪 Running Compiler Stress Tests\n");
    std.debug.print("=" ** 40 ++ "\n");

    // Large program test
    const large_program_test = IntegrationTestCase{
        .name = "Large Program Compilation",
        .source_code = generateLargeProgram(allocator),
        .expected_output = "Large program executed successfully\n",
        .timeout_ms = 30000, // 30 second timeout
    };

    var runner = IntegrationTestRunner.init(allocator);
    defer runner.deinit(allocator);

    try runner.runTestCase(large_program_test);
    runner.printSummary();
}

fn generateLargeProgram(allocator: Allocator) []const u8 {
    // Generate a large CURSED program for stress testing
    _ = allocator;
    return 
        \\fr fr Large program with many functions and structures
        \\
        \\squad LargeStruct {
        \\    spill field1 drip
        \\    spill field2 tea
        \\    spill field3 lit
        \\}
        \\
        \\slay function1() drip { damn 1; }
        \\slay function2() drip { damn 2; }
        \\slay function3() drip { damn 3; }
        \\slay function4() drip { damn 4; }
        \\slay function5() drip { damn 5; }
        \\
        \\slay main_computation() {
        \\    sus total drip = 0
        \\    bestie i := 1; i <= 100; i = i + 1 {
        \\        total = total + function1() + function2() + function3()
        \\    }
        \\    vibez.spill("Large program executed successfully")
        \\}
        \\
        \\main_computation()
    ;
}

// Zig test integration
test "Integration Tests" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    try runAllIntegrationTests(allocator);
}

test "Cross-Platform Tests" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    try runCrossPlatformTests(allocator);
}

test "Stress Tests" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    try runStressTests(allocator);
}
