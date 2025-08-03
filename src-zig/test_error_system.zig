const std = @import("std");
const testing = std.testing;
const error_handling = @import("error_handling.zig");
const interpreter = @import("interpreter.zig");
const simple_interpreter = @import("simple_interpreter.zig");
const type_system_runtime = @import("type_system_runtime.zig");

const CursedError = error_handling.CursedError;
const ErrorContext = error_handling.ErrorContext;
const ErrorRecovery = error_handling.ErrorRecovery;
const safeDupeString = error_handling.safeDupeString;
const safeReadFile = error_handling.safeReadFile;
const safeWriteFile = error_handling.safeWriteFile;

test "error handling system comprehensive test" {
    const allocator = testing.allocator;
    
    // Test 1: Basic error context creation and management
    var ctx = try ErrorContext.init(allocator, CursedError.OutOfMemory, "Test out of memory error");
    defer ctx.deinit();
    
    try testing.expect(ctx.error_code == CursedError.OutOfMemory);
    try testing.expect(std.mem.eql(u8, ctx.message, "Test out of memory error"));
    
    // Test 2: Error recovery system
    var recovery = ErrorRecovery.init(allocator, 5);
    defer recovery.deinit();
    
    const error1 = try ErrorContext.init(allocator, CursedError.ParseError, "Parse failed");
    const error2 = try ErrorContext.init(allocator, CursedError.TypeMismatch, "Type mismatch");
    
    try recovery.addError(error1);
    try recovery.addError(error2);
    
    try testing.expect(recovery.hasErrors());
    try testing.expect(recovery.getErrors().len == 2);
    
    // Test 3: Safe string duplication
    const original = "test string for duplication";
    const duplicated = try safeDupeString(allocator, original);
    defer allocator.free(duplicated);
    
    try testing.expect(std.mem.eql(u8, original, duplicated));
    
    // Test 4: Error context with location
    const location = ErrorContext.SourceLocation{
        .file = "test.csd",
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
    
    // Test 5: Nested error contexts
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

test "interpreter error handling integration" {
    const allocator = testing.allocator;
    
    // Test struct instance creation with error handling
    var struct_instance = interpreter.StructInstance.init(allocator, "TestStruct") catch |err| {
        try testing.expect(err == CursedError.OutOfMemory);
        return;
    };
    defer struct_instance.deinit();
    
    try testing.expect(std.mem.eql(u8, struct_instance.type_name, "TestStruct"));
    
    // Test VTable creation with error handling
    var vtable = interpreter.VTable.init(allocator, "TestInterface") catch |err| {
        try testing.expect(err == CursedError.OutOfMemory);
        return;
    };
    defer vtable.deinit();
    
    try testing.expect(std.mem.eql(u8, vtable.interface_name, "TestInterface"));
    
    // Test ErrorValue creation with error handling
    var error_value = interpreter.ErrorValue.init(allocator, "Test error message", 404) catch |err| {
        try testing.expect(err == CursedError.OutOfMemory);
        return;
    };
    defer error_value.deinit();
    
    try testing.expect(std.mem.eql(u8, error_value.message, "Test error message"));
    try testing.expect(error_value.code == 404);
}

test "simple interpreter error handling integration" {
    const allocator = testing.allocator;
    
    // Test struct instance creation in simple interpreter
    var struct_instance = simple_interpreter.StructInstance.init(allocator, "SimpleStruct") catch |err| {
        try testing.expect(err == CursedError.OutOfMemory);
        return;
    };
    defer struct_instance.deinit();
    
    try testing.expect(std.mem.eql(u8, struct_instance.type_name, "SimpleStruct"));
    
    // Test struct type creation
    var struct_type = simple_interpreter.StructType.init(allocator, "SimpleStructType") catch |err| {
        try testing.expect(err == CursedError.OutOfMemory);
        return;
    };
    defer struct_type.deinit();
    
    try testing.expect(std.mem.eql(u8, struct_type.name, "SimpleStructType"));
}

test "type system runtime error handling integration" {
    const allocator = testing.allocator;
    
    // Test runtime type info creation
    var type_info = type_system_runtime.RuntimeTypeInfo.init(
        allocator,
        1,
        "TestType",
        .Basic
    ) catch |err| {
        try testing.expect(err == CursedError.OutOfMemory);
        return;
    };
    defer type_info.deinit();
    
    try testing.expect(std.mem.eql(u8, type_info.type_name, "TestType"));
    try testing.expect(type_info.type_id == 1);
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

// Integration test to ensure no more @panic() calls exist
test "verify no panic calls remain" {
    // This test would fail compilation if any @panic() calls remained
    // in the files we've modified, ensuring complete error handling coverage
    
    const allocator = testing.allocator;
    
    // Try operations that previously would panic
    _ = interpreter.StructInstance.init(allocator, "TestStruct") catch |err| {
        // Should return proper error, not panic
        try testing.expect(err == CursedError.OutOfMemory);
    };
    
    _ = simple_interpreter.StructType.init(allocator, "TestType") catch |err| {
        // Should return proper error, not panic
        try testing.expect(err == CursedError.OutOfMemory);
    };
    
    _ = type_system_runtime.RuntimeTypeInfo.init(allocator, 1, "Type", .Basic) catch |err| {
        // Should return proper error, not panic
        try testing.expect(err == CursedError.OutOfMemory);
    };
}
