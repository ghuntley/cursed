const std = @import("std");
const testing = std.testing;
const error_handling = @import("error_handling.zig");

const CursedError = error_handling.CursedError;
const ErrorContext = error_handling.ErrorContext;
const ErrorRecovery = error_handling.ErrorRecovery;
const safeDupeString = error_handling.safeDupeString;
const safeReadFile = error_handling.safeReadFile;
const safeWriteFile = error_handling.safeWriteFile;

test "basic error handling system" {
    const allocator = testing.allocator;
    
    // Test 1: Basic error context creation and management
    var ctx = try ErrorContext.init(allocator, CursedError.OutOfMemory, "Test out of memory error");
    defer ctx.deinit();
    
    try testing.expect(ctx.error_code == CursedError.OutOfMemory);
    try testing.expect(std.mem.eql(u8, ctx.message, "Test out of memory error"));
}

test "error recovery system" {
    const allocator = testing.allocator;
    
    // Test error recovery system
    var recovery = ErrorRecovery.init(allocator, 5);
    defer recovery.deinit();
    
    const error1 = try ErrorContext.init(allocator, CursedError.ParseError, "Parse failed");
    const error2 = try ErrorContext.init(allocator, CursedError.TypeMismatch, "Type mismatch");
    
    try recovery.addError(error1);
    try recovery.addError(error2);
    
    try testing.expect(recovery.hasErrors());
    try testing.expect(recovery.getErrors().len == 2);
}

test "safe string operations" {
    const allocator = testing.allocator;
    
    // Test safe string duplication
    const original = "test string for duplication";
    const duplicated = try safeDupeString(allocator, original);
    defer allocator.free(duplicated);
    
    try testing.expect(std.mem.eql(u8, original, duplicated));
}

test "error context with location" {
    const allocator = testing.allocator;
    
    // Test error context with location
    const location = ErrorContext.SourceLocation{
        .file = "test.💀",
        .line = 42,
        .column = 10,
    };
    
    var ctx_with_loc = try ErrorContext.initWithLocation(
        allocator,
        CursedError.UndefinedVariable,
        "Variable 'x' not found",
        location
    );
    defer ctx_with_loc.deinit();
    
    try testing.expect(ctx_with_loc.location != null);
    try testing.expect(ctx_with_loc.location.?.line == 42);
}

test "nested error contexts" {
    const allocator = testing.allocator;
    
    // Test nested error contexts
    const inner_ctx = try ErrorContext.init(allocator, CursedError.FileNotFound, "Input file missing");
    const inner_ptr = try allocator.create(ErrorContext);
    inner_ptr.* = inner_ctx;
    
    var outer_ctx = try ErrorContext.initWithInner(
        allocator,
        CursedError.CompilationError,
        "Compilation failed due to missing input",
        inner_ptr
    );
    defer outer_ctx.deinit();
    
    try testing.expect(outer_ctx.inner_error != null);
    try testing.expect(outer_ctx.inner_error.?.error_code == CursedError.FileNotFound);
}

test "file operation error handling" {
    const allocator = testing.allocator;
    
    // Test reading non-existent file
    const result = safeReadFile(allocator, "non_existent_file.txt");
    try testing.expectError(CursedError.FileNotFound, result);
    
    // Test writing to valid file
    const test_content = "This is test content for error handling validation";
    const test_file = "test_error_handling_output.txt";
    
    // Write file
    try safeWriteFile(allocator, test_file, test_content);
    
    // Read it back
    const read_content = try safeReadFile(allocator, test_file);
    defer allocator.free(read_content);
    
    try testing.expect(std.mem.eql(u8, test_content, read_content));
    
    // Clean up
    std.fs.cwd().deleteFile(test_file) catch {};
}

test "error context formatting" {
    const allocator = testing.allocator;
    
    var ctx = try ErrorContext.init(allocator, CursedError.ParseError, "Unexpected token");
    defer ctx.deinit();
    
    const formatted = try ctx.toString(allocator);
    defer allocator.free(formatted);
    
    try testing.expect(std.mem.containsAtLeast(u8, formatted, 1, "ParseError"));
    try testing.expect(std.mem.containsAtLeast(u8, formatted, 1, "Unexpected token"));
}

test "memory allocation safety" {
    const allocator = testing.allocator;
    
    // Test safe allocation
    const buffer = try error_handling.safeAlloc(allocator, u8, 1024);
    defer allocator.free(buffer);
    
    try testing.expect(buffer.len == 1024);
    
    // Test safe duplication
    const original_data = [_]u8{ 1, 2, 3, 4, 5 };
    const duplicated = try error_handling.safeDupe(allocator, u8, &original_data);
    defer allocator.free(duplicated);
    
    try testing.expect(std.mem.eql(u8, &original_data, duplicated));
}

test "error mapping functions" {
    // Test allocator error mapping
    const mapped_alloc_error = error_handling.mapAllocatorError(error.OutOfMemory);
    try testing.expect(mapped_alloc_error == CursedError.OutOfMemory);
    
    // Test file error mapping
    const mapped_file_error = error_handling.mapFileOpenError(error.FileNotFound);
    try testing.expect(mapped_file_error == CursedError.FileNotFound);
    
    const mapped_read_error = error_handling.mapFileReadError(error.InputOutput);
    try testing.expect(mapped_read_error == CursedError.ReadError);
    
    const mapped_write_error = error_handling.mapFileWriteError(error.AccessDenied);
    try testing.expect(mapped_write_error == CursedError.PermissionDenied);
}
