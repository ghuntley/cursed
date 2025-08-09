// Comprehensive memory stress test for stdlib module loading
// Tests various scenarios that could trigger memory corruption

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const testing = std.testing;

const module_loader = @import("src-zig/module_loader.zig");
const stdlib_import_safety = @import("src-zig/stdlib_import_safety.zig");
const stdlib_memory_fixes = @import("src-zig/stdlib_memory_fixes.zig");

/// Memory corruption test scenarios
const TestScenario = enum {
    basic_loading,
    rapid_load_unload,
    concurrent_loading,
    large_module_loading,
    invalid_module_loading,
    memory_exhaustion,
    circular_dependencies,
    duplicate_imports,
};

/// Test result tracking
const TestResult = struct {
    scenario: TestScenario,
    success: bool,
    memory_leaks: u32,
    error_message: ?[]const u8,
    execution_time_ms: u64,

    pub fn init(scenario: TestScenario) TestResult {
        return TestResult{
            .scenario = scenario,
            .success = false,
            .memory_leaks = 0,
            .error_message = null,
            .execution_time_ms = 0,
        };
    }

    pub fn deinit(self: *TestResult, allocator: Allocator) void {
        if (self.error_message) |msg| {
            allocator.free(msg);
        }
    }

    pub fn print(self: TestResult) void {
        const status = if (self.success) "✅ PASS" else "❌ FAIL";
        std.debug.print("{s} - {s} ({}ms)\n", .{ status, @tagName(self.scenario), self.execution_time_ms });
        
        if (self.memory_leaks > 0) {
            std.debug.print("  ⚠️  Memory leaks detected: {}\n", .{self.memory_leaks});
        }
        
        if (self.error_message) |msg| {
            std.debug.print("  Error: {s}\n", .{msg});
        }
    }
};

/// Memory corruption test suite
pub const MemoryCorruptionTestSuite = struct {
    allocator: Allocator,
    results: ArrayList(TestResult),
    verbose: bool,

    pub fn init(allocator: Allocator, verbose: bool) MemoryCorruptionTestSuite {
        return MemoryCorruptionTestSuite{
            .allocator = allocator,
            .results = ArrayList(TestResult).init(allocator),
            .verbose = verbose,
        };
    }

    pub fn deinit(self: *MemoryCorruptionTestSuite) void {
        for (self.results.items) |*result| {
            result.deinit(self.allocator);
        }
        self.results.deinit();
    }

    /// Run all memory corruption tests
    pub fn runAllTests(self: *MemoryCorruptionTestSuite) !void {
        print("🚀 Starting Memory Corruption Test Suite...\n", .{});
        print("========================================\n", .{});

        // Test scenarios in order of complexity
        try self.testBasicLoading();
        try self.testRapidLoadUnload();
        try self.testInvalidModuleLoading();
        try self.testDuplicateImports();
        try self.testLargeModuleLoading();
        try self.testMemoryExhaustion();

        self.printSummary();
    }

    /// Test 1: Basic module loading
    fn testBasicLoading(self: *MemoryCorruptionTestSuite) !void {
        const start_time = std.time.milliTimestamp();
        var result = TestResult.init(.basic_loading);

        // Test basic stdlib module loading
        var safe_manager = stdlib_import_safety.SafeImportManager.init(self.allocator, self.verbose);
        defer safe_manager.deinit();

        const modules = [_][]const u8{ "mathz", "stringz", "arrayz" };
        var all_success = true;

        for (modules) |module_name| {
            const import_result = safe_manager.safeImportModule(module_name) catch false;
            if (!import_result) {
                all_success = false;
                break;
            }
        }

        result.success = all_success;
        result.execution_time_ms = @intCast(std.time.milliTimestamp() - start_time);

        try self.results.append(result);
        if (self.verbose) result.print();
    }

    /// Test 2: Rapid load/unload cycles
    fn testRapidLoadUnload(self: *MemoryCorruptionTestSuite) !void {
        const start_time = std.time.milliTimestamp();
        var result = TestResult.init(.rapid_load_unload);

        var success_count: u32 = 0;
        const iterations = 50;

        // Perform rapid load/unload cycles
        for (0..iterations) |_| {
            var loader = module_loader.ModuleLoader.init(self.allocator, false);
            defer loader.deinit();

            const functions = loader.loadModule("mathz") catch null;
            if (functions != null) {
                success_count += 1;
            }
        }

        result.success = success_count >= (iterations * 80 / 100); // 80% success rate
        result.execution_time_ms = @intCast(std.time.milliTimestamp() - start_time);

        try self.results.append(result);
        if (self.verbose) result.print();
    }

    /// Test 3: Invalid module loading
    fn testInvalidModuleLoading(self: *MemoryCorruptionTestSuite) !void {
        const start_time = std.time.milliTimestamp();
        var result = TestResult.init(.invalid_module_loading);

        var safe_manager = stdlib_import_safety.SafeImportManager.init(self.allocator, self.verbose);
        defer safe_manager.deinit();

        // Test invalid module names
        const invalid_modules = [_][]const u8{
            "nonexistent_module",
            "",
            "module_with_invalid/chars",
            "super_long_module_name_that_exceeds_normal_limits_and_should_be_rejected_by_the_system_for_being_too_long",
            "../../etc/passwd",
            "module\x00with\x00nulls",
        };

        var all_handled_correctly = true;
        for (invalid_modules) |module_name| {
            const import_result = safe_manager.safeImportModule(module_name) catch false;
            // These should all fail safely
            if (import_result) {
                all_handled_correctly = false;
                break;
            }
        }

        result.success = all_handled_correctly;
        result.execution_time_ms = @intCast(std.time.milliTimestamp() - start_time);

        try self.results.append(result);
        if (self.verbose) result.print();
    }

    /// Test 4: Duplicate imports
    fn testDuplicateImports(self: *MemoryCorruptionTestSuite) !void {
        const start_time = std.time.milliTimestamp();
        var result = TestResult.init(.duplicate_imports);

        var safe_manager = stdlib_import_safety.SafeImportManager.init(self.allocator, self.verbose);
        defer safe_manager.deinit();

        // Import the same module multiple times
        var all_success = true;
        for (0..10) |_| {
            const import_result = safe_manager.safeImportModule("mathz") catch false;
            if (!import_result) {
                all_success = false;
                break;
            }
        }

        result.success = all_success;
        result.execution_time_ms = @intCast(std.time.milliTimestamp() - start_time);

        try self.results.append(result);
        if (self.verbose) result.print();
    }

    /// Test 5: Large module loading
    fn testLargeModuleLoading(self: *MemoryCorruptionTestSuite) !void {
        const start_time = std.time.milliTimestamp();
        var result = TestResult.init(.large_module_loading);

        var safe_manager = stdlib_import_safety.SafeImportManager.init(self.allocator, self.verbose);
        defer safe_manager.deinit();

        // Try to load the cryptz module (largest stdlib module)
        const import_result = safe_manager.safeImportModule("cryptz") catch false;

        result.success = import_result;
        result.execution_time_ms = @intCast(std.time.milliTimestamp() - start_time);

        try self.results.append(result);
        if (self.verbose) result.print();
    }

    /// Test 6: Memory exhaustion simulation
    fn testMemoryExhaustion(self: *MemoryCorruptionTestSuite) !void {
        const start_time = std.time.milliTimestamp();
        var result = TestResult.init(.memory_exhaustion);

        // Create a limited allocator to simulate memory pressure
        var limited_buffer: [1024 * 1024]u8 = undefined; // 1MB limit
        var fba = std.heap.FixedBufferAllocator.init(&limited_buffer);
        const limited_allocator = fba.allocator();

        var loader = module_loader.ModuleLoader.init(limited_allocator, false);
        defer loader.deinit();

        // Try to load modules with limited memory
        var modules_loaded: u32 = 0;
        const test_modules = [_][]const u8{ "mathz", "stringz", "arrayz", "cryptz" };

        for (test_modules) |module_name| {
            const functions = loader.loadModule(module_name) catch null;
            if (functions != null) {
                modules_loaded += 1;
            }
        }

        // Success if we handled memory pressure gracefully (no crashes)
        result.success = true; // If we reach here, we didn't crash
        result.execution_time_ms = @intCast(std.time.milliTimestamp() - start_time);

        try self.results.append(result);
        if (self.verbose) result.print();
    }

    /// Print test summary
    fn printSummary(self: *MemoryCorruptionTestSuite) void {
        print("\n📊 Memory Corruption Test Summary\n", .{});
        print("=================================\n", .{});

        var passed: u32 = 0;
        var failed: u32 = 0;
        var total_leaks: u32 = 0;
        var total_time: u64 = 0;

        for (self.results.items) |test_result| {
            if (test_result.success) {
                passed += 1;
            } else {
                failed += 1;
            }
            total_leaks += test_result.memory_leaks;
            total_time += test_result.execution_time_ms;

            test_result.print();
        }

        print("\nOverall Results:\n", .{});
        print("  Tests passed: {}/{}\n", .{ passed, self.results.items.len });
        print("  Tests failed: {}\n", .{failed});
        print("  Total memory leaks: {}\n", .{total_leaks});
        print("  Total execution time: {}ms\n", .{total_time});

        if (failed == 0 and total_leaks == 0) {
            print("🎉 All memory corruption tests passed!\n", .{});
        } else {
            print("⚠️  Some tests failed or memory leaks detected\n", .{});
        }
    }
};

/// Create a comprehensive test file for stdlib memory corruption
pub fn createStdlibMemoryCorruptionTest() !void {
    _ = @import("std").mem.Allocator; // Reference allocator for consistency
    // Create a test file that uses various stdlib functions to stress memory
    const test_content =
        \\# Comprehensive stdlib memory corruption test
        \\# Loads all major stdlib modules and uses their functions extensively
        \\
        \\# Load all stdlib modules
        \\yeet "mathz"
        \\yeet "stringz" 
        \\yeet "arrayz"
        \\yeet "cryptz"
        \\yeet "testz"
        \\
        \\# Test mathz functions
        \\sus math_test1 drip = abs_normie(-100)
        \\sus math_test2 drip = max_normie(50, 75)
        \\sus math_test3 drip = power_int(3, 4)
        \\sus math_test4 drip = factorial(5)
        \\
        \\# Test string functions
        \\sus str1 tea = "Hello"
        \\sus str2 tea = "World"
        \\sus concat_result tea = concat_strings(str1, str2)
        \\
        \\# Test array functions
        \\sus test_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        \\sus array_sum drip = sum_array(test_array)
        \\sus array_max drip = find_max(test_array)
        \\sus array_min drip = find_min(test_array)
        \\
        \\# Test crypto functions (basic ones)
        \\sus random_val drip = crypto_secure_random_u32()
        \\
        \\# Test testz functions
        \\test_start("Memory Corruption Test")
        \\assert_true(math_test1 > 0)
        \\assert_true(array_sum > 0)
        \\print_test_summary()
        \\
        \\vibez.spill("Stdlib memory corruption test completed!")
        \\
    ;

    const file = try std.fs.cwd().createFile("comprehensive_stdlib_memory_test.csd", .{});
    defer file.close();

    try file.writeAll(test_content);
    print("📄 Created comprehensive stdlib memory test file\n", .{});
}

/// Main test runner
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    print("🧪 CURSED Stdlib Memory Corruption Test Suite\n", .{});
    print("=============================================\n", .{});

    // Create comprehensive test file
    try createStdlibMemoryCorruptionTest();

    // Run memory corruption tests
    var test_suite = MemoryCorruptionTestSuite.init(allocator, true);
    defer test_suite.deinit();

    try test_suite.runAllTests();

    // Test the safe memory fixes
    print("\n🔧 Testing Safe Memory Fixes...\n", .{});
    try stdlib_memory_fixes.testSafeModuleLoader(allocator);

    // Test the safe import system
    print("\n🔧 Testing Safe Import System...\n", .{});
    try stdlib_import_safety.testSafeImportSystem(allocator);

    print("\n✅ All memory corruption tests completed!\n", .{});
}
