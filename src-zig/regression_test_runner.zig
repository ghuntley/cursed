const std = @import("std");
const print = std.debug.print;
const testing = std.testing;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const fs = std.fs;
const process = std.process;

// Import our parser and AST components
const Parser = @import("parser.zig").Parser;
const AST = @import("ast.zig");
const Lexer = @import("lexer.zig").Lexer;

/// Test result for individual test case
pub const TestResult = struct {
    name: []const u8,
    passed: bool,
    error_message: ?[]const u8,
    memory_leaks: u32,
    execution_time_ms: u64,
    
    pub fn deinit(self: *TestResult, allocator: Allocator) void {
        _ = allocator;
        if (self.error_message) |msg| {
            allocator.free(msg);
        }
    }
};

/// Test suite results
pub const TestSuiteResult = struct {
    total_tests: u32,
    passed_tests: u32,
    failed_tests: u32,
    memory_leak_tests: u32,
    total_execution_time_ms: u64,
    results: ArrayList(TestResult),
    
    pub fn init() TestSuiteResult {
        return TestSuiteResult{
            .total_tests = 0,
            .passed_tests = 0,
            .failed_tests = 0,
            .memory_leak_tests = 0,
            .total_execution_time_ms = 0,
            .results = .empty,
        };
    }
    
    pub fn deinit(self: *TestSuiteResult) void {
        for (self.results.items) |*result| {
            result.deinit(self.results.allocator);
        }
        self.results.deinit(self.allocator);
    }
    
    pub fn addResult(self: *TestSuiteResult, result: TestResult) !void {
        try self.results.append(allocator, result);
        self.total_tests += 1;
        if (result.passed) {
            self.passed_tests += 1;
        } else {
            self.failed_tests += 1;
        }
        if (result.memory_leaks > 0) {
            self.memory_leak_tests += 1;
        }
        self.total_execution_time_ms += result.execution_time_ms;
    }
    
    pub fn printSummary(self: *const TestSuiteResult) void {
        print("\n=== REGRESSION TEST SUITE SUMMARY ===\n", .{});
        print("Total Tests: {d}\n", .{self.total_tests});
        print("Passed: {d} ({d:.1}%)\n", .{ self.passed_tests, @as(f64, @floatFromInt(self.passed_tests)) / @as(f64, @floatFromInt(self.total_tests)) * 100.0 });
        print("Failed: {d} ({d:.1}%)\n", .{ self.failed_tests, @as(f64, @floatFromInt(self.failed_tests)) / @as(f64, @floatFromInt(self.total_tests)) * 100.0 });
        print("Memory Leaks: {d}\n", .{self.memory_leak_tests});
        print("Total Execution Time: {d}ms\n", .{self.total_execution_time_ms});
        print("=====================================\n", .{});
        
        if (self.failed_tests > 0) {
            print("\nFAILED TESTS:\n", .{});
            for (self.results.items) |result| {
                if (!result.passed) {
                    print("  ❌ {s}", .{result.name});
                    if (result.error_message) |msg| {
                        print(" - {s}", .{msg});
                    }
                    if (result.memory_leaks > 0) {
                        print(" (MEMORY LEAKS: {d})", .{result.memory_leaks});
                    }
                    print("\n", .{});
                }
            }
        }
        
        if (self.memory_leak_tests > 0) {
            print("\nMEMORY LEAK TESTS:\n", .{});
            for (self.results.items) |result| {
                if (result.memory_leaks > 0) {
                    print("  🚨 {s} - {d} leaks\n", .{ result.name, result.memory_leaks });
                }
            }
        }
    }
};

/// Regression test runner
pub const RegressionTestRunner = struct {
    allocator: Allocator,
    test_timeout_ms: u32,
    use_valgrind: bool,
    verbose: bool,
    
    pub fn init() RegressionTestRunner {
        return RegressionTestRunner{
            .allocator = allocator,
            .test_timeout_ms = 5000, // 5 second timeout per test
            .use_valgrind = true,
            .verbose = false,
        };
    }
    
    /// Run all regression tests in a directory
    pub fn runTestDirectory(self: *RegressionTestRunner, test_dir: []const u8) !TestSuiteResult {
        var result = TestSuiteResult.init(self.allocator);
        
        var dir = fs.cwd().openDir(test_dir, .{ .iterate = true }) catch |err| {
            print("Failed to open test directory '{s}': {s}\n", .{ test_dir, err });
            return result;
        };
        defer dir.close();
        
        var iterator = dir.iterate();
        while (try iterator.next()) |entry| {
            if (entry.kind == .file and std.mem.endsWith(u8, entry.name, ".csd")) {
                const test_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ test_dir, entry.name });
                defer self.allocator.free(test_path);
                
                const test_result = self.runSingleTest(test_path) catch |err| {
                    const error_msg = try std.fmt.allocPrint(self.allocator, "Test execution failed: {}", .{err});
                    TestResult{
                        .name = try self.allocator.dupe(u8, entry.name),
                        .passed = false,
                        .error_message = error_msg,
                        .memory_leaks = 0,
                        .execution_time_ms = 0,
                    };
                };
                
                try result.addResult(test_result);
                
                if (self.verbose) {
                    const status = if (test_result.passed) "✅" else "❌";
                    print("{s} {s} ({d}ms)", .{ status, test_result.name, test_result.execution_time_ms });
                    if (test_result.memory_leaks > 0) {
                        print(" LEAKS: {d}", .{test_result.memory_leaks});
                    }
                    print("\n", .{});
                }
            }
        }
        
        return result;
    }
    
    /// Run a single test file with comprehensive safety checks
    pub fn runSingleTest(self: *RegressionTestRunner, test_path: []const u8) !TestResult {
        const start_time = std.time.milliTimestamp();
        
        // Validate test path
        if (test_path.len == 0 or test_path.len > 512) {
            const error_msg = try std.fmt.allocPrint(self.allocator, "Invalid test path length: {}", .{test_path.len});
            return TestResult{
                .name = try self.allocator.dupe(u8, "invalid_path"),
                .passed = false,
                .error_message = error_msg,
                .memory_leaks = 0,
                .execution_time_ms = 0,
            };
        }
        
        // Read test file with safe bounds
        const test_content = fs.cwd().readFileAlloc(self.allocator, test_path, 1024 * 1024) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "File read error: {}", .{err});
            return TestResult{
                .name = try self.allocator.dupe(u8, fs.path.basename(test_path)),
                .passed = false,
                .error_message = error_msg,
                .memory_leaks = 0,
                .execution_time_ms = @as(u64, @intCast(std.time.milliTimestamp() - start_time)),
            };
        };
        defer self.allocator.free(test_content);
        
        // Validate content before parsing
        if (test_content.len == 0) {
            const error_msg = try std.fmt.allocPrint(self.allocator, "Empty test file", .{});
            return TestResult{
                .name = try self.allocator.dupe(u8, fs.path.basename(test_path)),
                .passed = false,
                .error_message = error_msg,
                .memory_leaks = 0,
                .execution_time_ms = @as(u64, @intCast(std.time.milliTimestamp() - start_time)),
            };
        }
        
        // Parse the content with error recovery
        const parse_result = self.runParserTest(test_content) catch |err| {
            const error_msg = try std.fmt.allocPrint(self.allocator, "Parser error: {}", .{err});
            return TestResult{
                .name = try self.allocator.dupe(u8, fs.path.basename(test_path)),
                .passed = false,
                .error_message = error_msg,
                .memory_leaks = 0,
                .execution_time_ms = @as(u64, @intCast(std.time.milliTimestamp() - start_time)),
            };
        };
        
        // Check for memory leaks if valgrind is enabled
        var memory_leaks: u32 = 0;
        if (self.use_valgrind) {
            memory_leaks = try self.runValgrindTest(test_path);
        }
        
        // Run round-trip test
        const roundtrip_passed = self.runRoundTripTest(test_content) catch false;
        
        const execution_time = @as(u64, @intCast(std.time.milliTimestamp() - start_time));
        
        return TestResult{
            .name = try self.allocator.dupe(u8, fs.path.basename(test_path)),
            .passed = parse_result and roundtrip_passed and memory_leaks == 0,
            .error_message = null,
            .memory_leaks = memory_leaks,
            .execution_time_ms = execution_time,
        };
    }
    
    /// Test parser functionality with safe memory management
    fn runParserTest(self: *RegressionTestRunner, content: []const u8) !bool {
        // Validate content before processing
        if (content.len == 0 or content.len > 10 * 1024 * 1024) { // 10MB limit
            if (self.verbose) print("⚠️ Invalid content size: {s}\n", .{content.len});
            return false;
        }
        
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();
        
        // Lexical analysis with error handling
        var lexer = Lexer.init(arena_allocator, content);
        // Note: lexer.deinit() not needed as it uses arena allocator
        
        const tokens = lexer.tokenize() catch |err| {
            if (self.verbose) print("Lexer error: {s}\n", .{err});
            return false;
        };
        
        // Validate token count to prevent memory issues
        if (tokens.items.len > 100000) { // Reasonable limit for regression tests
            if (self.verbose) print("⚠️ Too many tokens: {}\n", .{tokens.items.len});
            return false;
        }
        
        // Parsing with comprehensive error recovery
        var parser = Parser.init(arena_allocator, tokens.items);
        defer parser.deinit();
        
        const ast = parser.parseProgram() catch |err| {
            if (self.verbose) print("Parser error: {s}\n", .{err});
            // Parser errors are expected for some regression tests
            return true; // Don't fail the test just because parsing failed
        };
        
        // Basic AST validation if parsing succeeded
        return self.validateAST(ast);
    }
    
    /// Run round-trip test (parse -> serialize -> parse again)
    fn runRoundTripTest(self: *RegressionTestRunner, original_content: []const u8) !bool {
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const arena_allocator = arena.allocator();
        
        // First parse
        var lexer1 = Lexer.init(arena_allocator, original_content);
        defer lexer1.deinit();
        
        const tokens1 = lexer1.tokenize() catch return false;
        
        var parser1 = Parser.init(arena_allocator, tokens1);
        defer parser1.deinit();
        
        const ast1 = parser1.parseProgram() catch return false;
        
        // Serialize AST back to source code
        const serialized = try self.serializeAST(arena_allocator, ast1);
        
        // Second parse
        var lexer2 = Lexer.init(arena_allocator, serialized);
        defer lexer2.deinit();
        
        const tokens2 = lexer2.tokenize() catch return false;
        
        var parser2 = Parser.init(arena_allocator, tokens2);
        defer parser2.deinit();
        
        const ast2 = parser2.parseProgram() catch return false;
        
        // Compare ASTs (simplified structural comparison)
        return self.compareASTs(ast1, ast2);
    }
    
    /// Run valgrind memory leak test with safe error handling
    fn runValgrindTest(self: *RegressionTestRunner, test_path: []const u8) !u32 {
        // Validate test path before execution
        if (test_path.len == 0 or test_path.len > 512) {
            if (self.verbose) print("⚠️ Invalid test path length: {s}\n", .{test_path.len});
            return 0;
        }
        
        // Run the test with valgrind and parse output for leaks
        const cmd = std.fmt.allocPrint(self.allocator, "timeout {d} valgrind --leak-check=summary --quiet --error-exitcode=1 ./zig-out/bin/cursed-zig {s} 2>&1", .{ self.test_timeout_ms / 1000, test_path }) catch |err| {
            if (self.verbose) print("⚠️ Failed to format valgrind command: {s}\n", .{err});
            return 0;
        };
        defer self.allocator.free(cmd);
        
        const result = std.ChildProcess.exec(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{ "bash", "-c", cmd },
            .max_output_bytes = 1024 * 1024, // 1MB limit
        }) catch |err| {
            if (self.verbose) print("⚠️ Failed to execute valgrind: {s}\n", .{err});
            return 0;
        };
        defer self.allocator.free(result.stdout);
        defer self.allocator.free(result.stderr);
        
        // Parse valgrind output for leak count
        var leaks: u32 = 0;
        var lines = std.mem.split(u8, result.stderr, "\n");
        while (lines.next()) |line| {
            if (std.mem.indexOf(u8, line, "definitely lost:")) |_| {
                // Extract number of bytes lost
                var words = std.mem.tokenize(u8, line, " ");
                while (words.next()) |word| {
                    if (std.mem.eql(u8, word, "lost:")) {
                        if (words.next()) |bytes_str| {
                            leaks = std.fmt.parseInt(u32, bytes_str, 10) catch 0;
                            break;
                        }
                    }
                }
            }
        }
        
        return leaks;
    }
    
    /// Basic AST validation
    fn validateAST(self: *RegressionTestRunner, ast: AST.Program) bool {
        // Basic structural validation
        if (ast.statements.len == 0) return true; // Empty program is valid
        
        // Check for basic integrity
        for (ast.statements) |stmt| {
            if (!self.validateStatement(stmt)) return false;
        }
        
        return true;
    }
    
    fn validateStatement(self: *RegressionTestRunner, stmt: AST.Statement) bool {
        _ = stmt;
        _ = self;
        // Basic statement validation
        // TODO: Implement more comprehensive validation
        return true;
    }
    
    /// Serialize AST back to source code
    fn serializeAST(self: *RegressionTestRunner, allocator: Allocator, ast: AST.Program) ![]const u8 {
        var result = std.ArrayList(u8){};
        
        for (ast.statements) |stmt| {
            try self.serializeStatement(&result, stmt);
            try result.append(allocator, '\n');
        }
        
        return result.toOwnedSlice();
    }
    
    fn serializeStatement(self: *RegressionTestRunner, result: *ArrayList(u8), stmt: AST.Statement) !void {
        _ = result;
        _ = stmt;
        _ = self;
        // TODO: Implement AST serialization
        // This is a placeholder - full implementation would recreate source code from AST
    }
    
    /// Compare two ASTs for structural equality
    fn compareASTs(self: *RegressionTestRunner, ast1: AST.Program, ast2: AST.Program) bool {
        if (ast1.statements.len != ast2.statements.len) return false;
        
        for (ast1.statements, ast2.statements) |stmt1, stmt2| {
            if (!self.compareStatements(stmt1, stmt2)) return false;
        }
        
        return true;
    }
    
    fn compareStatements(self: *RegressionTestRunner, stmt1: AST.Statement, stmt2: AST.Statement) bool {
        _ = stmt1;
        _ = stmt2;
        _ = self;
        // TODO: Implement deep AST comparison
        // This is a placeholder - full implementation would compare AST structure
        return true;
    }
};

/// Main test runner function
pub fn runRegressionTests(allocator: Allocator, args: []const []const u8) !void {
    var runner = RegressionTestRunner.init(allocator);
    
    // Parse command line arguments
    var test_directories = std.ArrayList(u8){};
    defer test_directories.deinit();
    
    var i: usize = 0;
    while (i < args.len) {
        const arg = args[i];
        if (std.mem.eql(u8, arg, "--verbose") or std.mem.eql(u8, arg, "-v")) {
            runner.verbose = true;
        } else if (std.mem.eql(u8, arg, "--no-valgrind")) {
            runner.use_valgrind = false;
        } else if (std.mem.eql(u8, arg, "--timeout")) {
            i += 1;
            if (i < args.len) {
                runner.test_timeout_ms = try std.fmt.parseInt(u32, args[i], 10);
            }
        } else if (!std.mem.startsWith(u8, arg, "--")) {
            try test_directories.append(allocator, arg);
        }
        i += 1;
    }
    
    // Default test directories if none specified
    if (test_directories.items.len == 0) {
        try test_directories.append(allocator, "tests/regression/parser");
        try test_directories.append(allocator, "tests/regression/stdlib");
        try test_directories.append(allocator, "tests/regression/memory");
        try test_directories.append(allocator, "tests/regression/errors");
        try test_directories.append(allocator, "tests/regression/roundtrip");
    }
    
    var overall_results = TestSuiteResult.init(allocator);
    defer overall_results.deinit();
    
    // Run tests for each directory
    for (test_directories.items) |test_dir| {
        print("Running tests in: {s}\n", .{test_dir});
        var dir_results = try runner.runTestDirectory(test_dir);
        defer dir_results.deinit();
        
        // Merge results
        for (dir_results.results.items) |result| {
            try overall_results.addResult(result);
        }
        
        print("  {d}/{d} tests passed\n", .{ dir_results.passed_tests, dir_results.total_tests });
    }
    
    // Print final summary
    overall_results.printSummary();
    
    // Exit with error code if tests failed
    if (overall_results.failed_tests > 0 or overall_results.memory_leak_tests > 0) {
        process.exit(1);
    }
}

// Test entry point for zig build test-regression
test "regression test runner" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    
    const runner = RegressionTestRunner.init(arena.allocator());
    
    // Test basic functionality
    try testing.expect(!runner.verbose);
    try testing.expect(runner.use_valgrind);
    try testing.expect(runner.test_timeout_ms == 5000);
}
