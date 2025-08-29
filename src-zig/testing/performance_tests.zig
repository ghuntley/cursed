//! Performance Testing and Benchmarking Framework for CURSED
//! 
//! Comprehensive performance validation including:
//! - Compilation speed benchmarks
//! - Runtime performance measurement
//! - Memory usage tracking
//! - Regression detection
//! - Cross-platform performance comparison

const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;

// Import CURSED components
const lexer = @import("../lexer.zig");
const parser = @import("../parser.zig");
const codegen = @import("../codegen.zig");
const runtime = @import("../runtime_system.zig");

pub const PerformanceMetrics = struct {
    compilation_time_ns: u64,
    execution_time_ns: u64,
    memory_usage_bytes: u64,
    peak_memory_bytes: u64,
    cpu_time_ns: u64,
    throughput_ops_per_sec: f64,
};

pub const BenchmarkResult = struct {
    name: []const u8,
    metrics: PerformanceMetrics,
    baseline_metrics: ?PerformanceMetrics = null,
    regression_detected: bool = false,
    performance_change_percent: f64 = 0.0,
};

pub const PerformanceTestSuite = struct {
    name: []const u8,
    source_code: []const u8,
    iterations: u32 = 100,
    warmup_iterations: u32 = 10,
    expected_throughput_min: f64 = 0.0,
    memory_limit_mb: u64 = 100,
};

pub const PerformanceTester = struct {
    allocator: Allocator,
    results: std.ArrayList(BenchmarkResult),
    baseline_data: std.HashMap(u64, PerformanceMetrics, std.hash_map.DefaultContext),

    pub fn init() PerformanceTester {
        return PerformanceTester{
            .allocator = allocator,
            .results = std.ArrayList(BenchmarkResult){},
            .baseline_data = std.HashMap(u64, PerformanceMetrics, std.hash_map.DefaultContext){},
        };
    }

    pub fn deinit(self: *PerformanceTester) void {
        self.results.deinit(self.allocator);
        self.baseline_data.deinit(self.allocator);
    }

    pub fn runBenchmark(self: *PerformanceTester, suite: PerformanceTestSuite) !void {
        std.debug.writer().print("⚡ Running performance benchmark: {s}\n", .{suite.name});

        // Warmup phase
        std.debug.writer().print("  🔥 Warming up ({s} iterations)...\n", .{{suite.warmup_iterations});
        var i: u32 = 0;
        while (i < suite.warmup_iterations) : (i += 1) {
            _ = try self.runSingleIteration(suite.source_code);
        }

        // Benchmark phase
        std.debug.writer().print("  📊 Benchmarking ({s} iterations)...\n", .{{suite.iterations});
        var total_metrics = PerformanceMetrics{
            .compilation_time_ns = 0,
            .execution_time_ns = 0,
            .memory_usage_bytes = 0,
            .peak_memory_bytes = 0,
            .cpu_time_ns = 0,
            .throughput_ops_per_sec = 0.0,
        };

        var max_memory: u64 = 0;
        i = 0;
        while (i < suite.iterations) : (i += 1) {
            const metrics = try self.runSingleIteration(suite.source_code);
            
            total_metrics.compilation_time_ns += metrics.compilation_time_ns;
            total_metrics.execution_time_ns += metrics.execution_time_ns;
            total_metrics.memory_usage_bytes += metrics.memory_usage_bytes;
            total_metrics.cpu_time_ns += metrics.cpu_time_ns;
            total_metrics.throughput_ops_per_sec += metrics.throughput_ops_per_sec;
            
            if (metrics.peak_memory_bytes > max_memory) {
                max_memory = metrics.peak_memory_bytes;
            }
        }

        // Calculate averages
        const avg_metrics = PerformanceMetrics{
            .compilation_time_ns = total_metrics.compilation_time_ns / suite.iterations,
            .execution_time_ns = total_metrics.execution_time_ns / suite.iterations,
            .memory_usage_bytes = total_metrics.memory_usage_bytes / suite.iterations,
            .peak_memory_bytes = max_memory,
            .cpu_time_ns = total_metrics.cpu_time_ns / suite.iterations,
            .throughput_ops_per_sec = total_metrics.throughput_ops_per_sec / @as(f64, @floatFromInt(suite.iterations)),
        };

        // Check for regressions
        const suite_hash = std.hash_map.hashString(suite.name);
        var regression_detected = false;
        var performance_change: f64 = 0.0;
        var baseline_metrics: ?PerformanceMetrics = null;

        if (self.baseline_data.get(suite_hash)) |baseline| {
            baseline_metrics = baseline;
            const compilation_change = (@as(f64, @floatFromInt(avg_metrics.compilation_time_ns)) - @as(f64, @floatFromInt(baseline.compilation_time_ns))) / @as(f64, @floatFromInt(baseline.compilation_time_ns)) * 100.0;
            const execution_change = (@as(f64, @floatFromInt(avg_metrics.execution_time_ns)) - @as(f64, @floatFromInt(baseline.execution_time_ns))) / @as(f64, @floatFromInt(baseline.execution_time_ns)) * 100.0;
            
            performance_change = (compilation_change + execution_change) / 2.0;
            regression_detected = performance_change > 10.0; // 10% threshold
        }

        // Store new baseline
        try self.baseline_data.put(suite_hash, avg_metrics);

        const result = BenchmarkResult{
            .name = suite.name,
            .metrics = avg_metrics,
            .baseline_metrics = baseline_metrics,
            .regression_detected = regression_detected,
            .performance_change_percent = performance_change,
        };

        try self.results.append(allocator, result);

        // Print immediate results
        self.printBenchmarkResult(result);
    }

    fn runSingleIteration(self: *PerformanceTester, source_code: []const u8) !PerformanceMetrics {
        var metrics = PerformanceMetrics{
            .compilation_time_ns = 0,
            .execution_time_ns = 0,
            .memory_usage_bytes = 0,
            .peak_memory_bytes = 0,
            .cpu_time_ns = 0,
            .throughput_ops_per_sec = 0.0,
        };

        // Track memory before starting
        const initial_memory = try getCurrentMemoryUsage();

        // Compilation phase timing
        const compile_start = std.time.nanoTimestamp();

        // Lexer
        var lex = try lexer.Lexer.init(self.allocator, source_code);
        defer lex.deinit();

        const tokens = try lex.tokenize();
        defer self.allocator.free(tokens);

        // Parser
        var parse = try parser.Parser.init(self.allocator, tokens);
        defer parse.deinit();

        const program = try parse.parseProgram();
        defer program.deinit();

        // Codegen
        var generator = try codegen.CodeGenerator.init(self.allocator);
        defer generator.deinit();

        const c_code = try generator.generateC(program);
        defer self.allocator.free(c_code);

        const compile_end = std.time.nanoTimestamp();
        metrics.compilation_time_ns = @as(u64, @intCast(compile_end - compile_start));

        // Execution phase timing
        const exec_start = std.time.nanoTimestamp();

        var interpreter = try runtime.Interpreter.init(self.allocator);
        defer interpreter.deinit();

        _ = interpreter.executeString(source_code) catch |err| {
            std.debug.writer().print("Execution error: {s}\n", .{{err});
            return metrics;
        };

        const exec_end = std.time.nanoTimestamp();
        metrics.execution_time_ns = @as(u64, @intCast(exec_end - exec_start));

        // Memory usage calculation
        const final_memory = try getCurrentMemoryUsage();
        metrics.memory_usage_bytes = final_memory - initial_memory;
        metrics.peak_memory_bytes = final_memory;

        // CPU time approximation
        metrics.cpu_time_ns = metrics.compilation_time_ns + metrics.execution_time_ns;

        // Throughput calculation (operations per second)
        const total_time_sec = @as(f64, @floatFromInt(metrics.compilation_time_ns + metrics.execution_time_ns)) / 1_000_000_000.0;
        metrics.throughput_ops_per_sec = if (total_time_sec > 0) 1.0 / total_time_sec else 0.0;

        return metrics;
    }

    fn getCurrentMemoryUsage(self: *PerformanceTester) !u64 {
        // Simplified memory usage estimation
        // In production, this would use platform-specific APIs
        _ = self;
        return 1024 * 1024; // 1MB placeholder
    }

    fn printBenchmarkResult(self: *PerformanceTester, result: BenchmarkResult) void {
        _ = self;
        std.debug.writer().print("  📈 Results for {s}:\n", .{result.name});
        std.debug.writer().print("    Compilation Time: {d:.2}ms\n", .{@as(f64, @floatFromInt(result.metrics.compilation_time_ns)) / 1_000_000.0});
        std.debug.writer().print("    Execution Time: {d:.2}ms\n", .{@as(f64, @floatFromInt(result.metrics.execution_time_ns)) / 1_000_000.0});
        std.debug.writer().print("    Memory Usage: {d:.2}KB\n", .{@as(f64, @floatFromInt(result.metrics.memory_usage_bytes)) / 1024.0});
        std.debug.writer().print("    Throughput: {d:.1} ops/sec\n", .{result.metrics.throughput_ops_per_sec});

        if (result.regression_detected) {
            std.debug.writer().print("    ⚠️  REGRESSION DETECTED: {d:.1}% slower\n", .{result.performance_change_percent});
        } else if (result.baseline_metrics != null) {
            const change_indicator = if (result.performance_change_percent < 0) "🚀 IMPROVEMENT" else "📊 CHANGE";
            std.debug.writer().print("    {s} {d:.1}% vs baseline\n", .{{ change_indicator, std.math.fabs(result.performance_change_percent) });
        }
        std.debug.writer().print("\n", .{});
    }

    pub fn printOverallSummary(self: *PerformanceTester) void {
        std.debug.writer().print("📊 Performance Test Summary\n", .{});
        std.debug.writer().print("=" ** 50 ++ "\n");

        var total_compilation_time: u64 = 0;
        var total_execution_time: u64 = 0;
        var total_memory_usage: u64 = 0;
        var regression_count: u32 = 0;

        for (self.results.items) |result| {
            total_compilation_time += result.metrics.compilation_time_ns;
            total_execution_time += result.metrics.execution_time_ns;
            total_memory_usage += result.metrics.memory_usage_bytes;
            if (result.regression_detected) regression_count += 1;
        }

        const avg_compile_time = if (self.results.items.len > 0) 
            @as(f64, @floatFromInt(total_compilation_time)) / @as(f64, @floatFromInt(self.results.items.len)) / 1_000_000.0 
        else 0.0;
        
        const avg_exec_time = if (self.results.items.len > 0)
            @as(f64, @floatFromInt(total_execution_time)) / @as(f64, @floatFromInt(self.results.items.len)) / 1_000_000.0
        else 0.0;

        const avg_memory = if (self.results.items.len > 0)
            @as(f64, @floatFromInt(total_memory_usage)) / @as(f64, @floatFromInt(self.results.items.len)) / 1024.0
        else 0.0;

        std.debug.writer().print("Total Benchmarks: {s}\n", .{{self.results.items.len});
        std.debug.writer().print("Average Compilation Time: {d:.2}ms\n", .{avg_compile_time});
        std.debug.writer().print("Average Execution Time: {d:.2}ms\n", .{avg_exec_time});
        std.debug.writer().print("Average Memory Usage: {d:.2}KB\n", .{avg_memory});
        std.debug.writer().print("Regressions Detected: {s}\n", .{{regression_count});

        if (regression_count > 0) {
            std.debug.writer().print("\n⚠️  Performance Regressions:\n", .{});
            for (self.results.items) |result| {
                if (result.regression_detected) {
                    std.debug.writer().print("  • {s}: {d:.1}% slower\n", .{ result.name, result.performance_change_percent });
                }
            }
        }
    }
};

// Define performance test suites
const performance_test_suites = [_]PerformanceTestSuite{
    .{
        .name = "Simple Hello World",
        .source_code = "vibez.spill(\"Hello, CURSED!\");",
        .iterations = 1000,
        .expected_throughput_min = 100.0,
    },
    .{
        .name = "Arithmetic Operations",
        .source_code = 
            \\sus a drip = 100
            \\sus b drip = 50
            \\sus result drip = (a + b) * (a - b) / (a / b)
            \\vibez.spill("Result: ", result)
        ,
        .iterations = 500,
        .expected_throughput_min = 50.0,
    },
    .{
        .name = "Function Calls",
        .source_code = 
            \\slay fibonacci(n drip) drip {
            \\    if n <= 1 { damn n; }
            \\    damn fibonacci(n - 1) + fibonacci(n - 2);
            \\}
            \\
            \\sus result drip = fibonacci(10)
            \\vibez.spill("Fibonacci(10): ", result)
        ,
        .iterations = 100,
        .expected_throughput_min = 10.0,
    },
    .{
        .name = "Struct Operations",
        .source_code = 
            \\squad Point {
            \\    spill x drip
            \\    spill y drip
            \\}
            \\
            \\slay distance(p1 Point, p2 Point) meal {
            \\    sus dx meal = p1.x - p2.x
            \\    sus dy meal = p1.y - p2.y
            \\    damn sqrt(dx * dx + dy * dy)
            \\}
            \\
            \\sus p1 Point = Point{x: 3, y: 4}
            \\sus p2 Point = Point{x: 0, y: 0}
            \\sus dist meal = distance(p1, p2)
            \\vibez.spill("Distance: ", dist)
        ,
        .iterations = 200,
        .expected_throughput_min = 20.0,
    },
    .{
        .name = "Loop Performance",
        .source_code = 
            \\sus total drip = 0
            \\bestie i := 1; i <= 1000; i = i + 1 {
            \\    total = total + i
            \\}
            \\vibez.spill("Sum 1-1000: ", total)
        ,
        .iterations = 100,
        .expected_throughput_min = 5.0,
    },
    .{
        .name = "Array Operations",
        .source_code = 
            \\sus numbers []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            \\sus sum drip = 0
            \\bestie i := 0; i < numbers.len(); i = i + 1 {
            \\    sum = sum + numbers[i]
            \\}
            \\vibez.spill("Array sum: ", sum)
        ,
        .iterations = 300,
        .expected_throughput_min = 30.0,
    },
};

// Memory stress test
const memory_stress_test = PerformanceTestSuite{
    .name = "Memory Stress Test",
    .source_code = 
        \\sus large_array []drip = []
        \\bestie i := 0; i < 10000; i = i + 1 {
        \\    large_array.push(i * i)
        \\}
        \\vibez.spill("Created array with ", large_array.len(), " elements")
    ,
    .iterations = 10,
    .memory_limit_mb = 500,
};

// Compiler stress test
const compiler_stress_test = PerformanceTestSuite{
    .name = "Compiler Stress Test",
    .source_code = generateLargeProgram(),
    .iterations = 5,
    .expected_throughput_min = 1.0,
};

fn generateLargeProgram() []const u8 {
    return 
        \\fr fr Large program to stress test the compiler
        \\
        \\squad ComplexStruct {
        \\    spill id drip
        \\    spill name tea
        \\    spill value meal
        \\    spill active lit
        \\}
        \\
        \\slay process_data(data ComplexStruct) ComplexStruct {
        \\    data.value = data.value * 2.0
        \\    data.active = !data.active
        \\    damn data
        \\}
        \\
        \\slay factorial(n drip) drip {
        \\    if n <= 1 { damn 1; }
        \\    damn n * factorial(n - 1);
        \\}
        \\
        \\slay run_complex_computation() {
        \\    sus data ComplexStruct = ComplexStruct{id: 1, name: "test", value: 3.14, active: based}
        \\    
        \\    bestie i := 1; i <= 20; i = i + 1 {
        \\        data = process_data(data)
        \\        sus fact drip = factorial(i)
        \\        data.value = data.value + fact
        \\    }
        \\    
        \\    vibez.spill("Final value: ", data.value)
        \\}
        \\
        \\run_complex_computation()
    ;
}

// Main performance test runner
pub fn runAllPerformanceTests(allocator: Allocator) !void {
        _ = allocator;
    std.debug.writer().print("🚀 Starting CURSED Performance Test Suite\n", .{});
    std.debug.writer().print("=" ** 60 ++ "\n");

    var tester = PerformanceTester.init(allocator);
    defer tester.deinit();

    // Run standard performance tests
    for (performance_test_suites) |suite| {
        try tester.runBenchmark(suite);
    }

    // Run stress tests
    std.debug.writer().print("💪 Running stress tests...\n", .{});
    try tester.runBenchmark(memory_stress_test);
    try tester.runBenchmark(compiler_stress_test);

    tester.printOverallSummary();
}

// Zig test integration
test "Performance Benchmarks" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try runAllPerformanceTests(allocator);
}

test "Memory Usage Benchmark" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var tester = PerformanceTester.init(allocator);
    defer tester.deinit();

    try tester.runBenchmark(memory_stress_test);
}

test "Compiler Performance Benchmark" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var tester = PerformanceTester.init(allocator);
    defer tester.deinit();

    try tester.runBenchmark(compiler_stress_test);
}
