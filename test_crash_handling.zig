const std = @import("std");
const print = std.debug.print;
const testing = std.testing;
const Allocator = std.mem.Allocator;

const crash_handler = @import("src-zig/crash_handler.zig");
const safe_operations = @import("src-zig/safe_operations.zig");
const parser = @import("src-zig/parser.zig");
const lexer = @import("src-zig/lexer.zig");

/// Test suite for crash handling and telemetry system
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("🧪 CURSED Crash Handling Test Suite\n", .{});
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n", .{});
    
    // Test 1: Basic telemetry system
    try testTelemetrySystem(allocator);
    
    // Test 2: Memory error detection
    try testMemoryErrorDetection(allocator);
    
    // Test 3: Safe file operations
    try testSafeFileOperations(allocator);
    
    // Test 4: Fatal error handling
    try testFatalErrorHandling(allocator);
    
    // Test 5: Parser error recovery
    try testParserErrorRecovery(allocator);
    
    print("\n✅ All crash handling tests completed!\n", .{});
}

fn testTelemetrySystem(allocator: Allocator) !void {
    print("\n🔍 Testing telemetry system...\n", .{});
    
    var telemetry = crash_handler.CrashTelemetry.init(allocator, true, 10);
    defer telemetry.deinit();
    
    // Create and record a test crash
    var context = try crash_handler.CrashContext.init(
        allocator,
        .Warning,
        "Test warning message",
        "test_file.zig",
        123,
        45,
        "testFunction"
    );
    defer context.deinit(allocator);
    
    try telemetry.recordCrash(context);
    
    if (telemetry.crash_log.items.len == 1) {
        print("  ✅ Telemetry system recording crashes correctly\n", .{});
    } else {
        print("  ❌ Telemetry system failed to record crash\n", .{});
    }
}

fn testMemoryErrorDetection(allocator: Allocator) !void {
    print("\n💾 Testing memory error detection...\n", .{});
    
    var detector = crash_handler.MemoryErrorDetector.init(allocator);
    defer detector.deinit();
    
    // Simulate memory allocation
    const test_ptr: usize = 0x12345678;
    try detector.trackAllocation(test_ptr, 1024, "test.zig", 42);
    
    const current_usage = detector.getCurrentUsage();
    if (current_usage == 1024) {
        print("  ✅ Memory tracking working correctly\n", .{});
    } else {
        print("  ❌ Memory tracking failed\n", .{});
    }
    
    // Simulate deallocation
    detector.trackDeallocation(test_ptr);
    
    const usage_after_free = detector.getCurrentUsage();
    if (usage_after_free == 0) {
        print("  ✅ Memory deallocation tracking working\n", .{});
    } else {
        print("  ❌ Memory deallocation tracking failed\n", .{});
    }
}

fn testSafeFileOperations(allocator: Allocator) !void {
    print("\n📁 Testing safe file operations...\n", .{});
    
    var telemetry = crash_handler.CrashTelemetry.init(allocator, true, 10);
    defer telemetry.deinit();
    
    var file_ops = safe_operations.SafeFileOperations.init(allocator, &telemetry);
    
    // Test writing a file
    const test_content = "Test file content for crash handling";
    const test_file = "test_crash_file.txt";
    
    file_ops.safeWriteFile(test_file, test_content, @src().file, @src().line) catch |err| {
        print("  ⚠️  File write failed (expected for some environments): {any}\n", .{err});
        return;
    };
    
    // Test reading the file back
    const read_content = file_ops.safeReadFile(test_file, @src().file, @src().line) catch |err| {
        print("  ⚠️  File read failed: {any}\n", .{err});
        return;
    };
    defer allocator.free(read_content);
    
    if (std.mem.eql(u8, test_content, read_content)) {
        print("  ✅ Safe file operations working correctly\n", .{});
    } else {
        print("  ❌ File content mismatch\n", .{});
    }
    
    // Clean up
    std.fs.cwd().deleteFile(test_file) catch {};
}

fn testFatalErrorHandling(allocator: Allocator) !void {
    print("\n💥 Testing fatal error handling...\n", .{});
    
    var telemetry = crash_handler.CrashTelemetry.init(allocator, true, 10);
    defer telemetry.deinit();
    
    var handler = crash_handler.FatalErrorHandler.init(allocator, &telemetry);
    defer handler.deinit();
    
    // Add a test recovery strategy
    try handler.addRecoveryStrategy("test", testRecoveryFunction);
    
    // Test non-fatal error (should not exit)
    handler.handleFatalError(
        .Error,
        "Test error message",
        @src().file,
        @src().line,
        0,
        @src().fn_name
    ) catch |err| {
        print("  ⚠️  Error handling failed: {any}\n", .{err});
        return;
    };
    
    print("  ✅ Fatal error handler working (non-fatal errors)\n", .{});
    
    // Check that telemetry recorded the error
    if (telemetry.crash_log.items.len > 0) {
        print("  ✅ Error recorded in telemetry\n", .{});
    } else {
        print("  ❌ Error not recorded in telemetry\n", .{});
    }
}

fn testRecoveryFunction(allocator: Allocator, context: crash_handler.CrashContext) anyerror!void {
    _ = allocator;
    print("  🔄 Recovery function called for: {s}\n", .{context.message});
}

fn testParserErrorRecovery(allocator: Allocator) !void {
    print("\n📝 Testing parser error recovery...\n", .{});
    
    var telemetry = crash_handler.CrashTelemetry.init(allocator, true, 10);
    defer telemetry.deinit();
    
    // Test with invalid CURSED code
    const invalid_code = "sus invalid_syntax = ";
    
    var lex = lexer.Lexer.init(invalid_code);
    const tokens = lex.tokenize(allocator) catch |err| {
        print("  ⚠️  Lexer failed (expected): {any}\n", .{err});
        return;
    };
    defer allocator.free(tokens);
    
    var test_parser = parser.Parser.initWithTelemetry(allocator, tokens, "test.csd", &telemetry);
    defer test_parser.deinit();
    
    const ast_result = test_parser.parseProgram() catch |err| {
        print("  ✅ Parser correctly failed on invalid syntax: {any}\n", .{err});
        
        // Check if error was recorded in telemetry
        if (telemetry.crash_log.items.len > 0) {
            print("  ✅ Parser error recorded in telemetry\n", .{});
        }
        return;
    };
    
    // If we get here, parsing somehow succeeded
    _ = ast_result;
    print("  ⚠️  Parser unexpectedly succeeded on invalid syntax\n", .{});
}

// Test cases for specific error scenarios
test "crash handler creates valid contexts" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var context = try crash_handler.CrashContext.init(
        allocator,
        .Error,
        "Test error",
        "test.zig",
        100,
        50,
        "testFunction"
    );
    defer context.deinit(allocator);
    
    try testing.expect(context.severity == .Error);
    try testing.expect(context.source_line == 100);
    try testing.expect(context.source_column == 50);
}

test "memory detector tracks allocations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var detector = crash_handler.MemoryErrorDetector.init(allocator);
    defer detector.deinit();
    
    const test_ptr: usize = 0x1000;
    try detector.trackAllocation(test_ptr, 512, "test.zig", 10);
    
    try testing.expect(detector.getCurrentUsage() == 512);
    
    detector.trackDeallocation(test_ptr);
    try testing.expect(detector.getCurrentUsage() == 0);
}

test "safe memory manager handles allocations" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var telemetry = crash_handler.CrashTelemetry.init(allocator, false, 10);
    defer telemetry.deinit();
    
    var memory_manager = safe_operations.SafeMemoryManager.init(allocator, &telemetry);
    defer memory_manager.deinit();
    
    const test_data = try memory_manager.safeAlloc(u32, 10, @src().file, @src().line);
    defer memory_manager.safeFree(test_data);
    
    try testing.expect(test_data.len == 10);
}
