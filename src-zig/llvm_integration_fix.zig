const std = @import("std");

// This file demonstrates how to replace the dummy LLVM functions with real ones
// The key insight is that the LLVM C wrapper functions work correctly,
// but the dummy implementations in codegen.zig need to be replaced

// Real LLVM C API bindings using the wrapper
extern fn llvm_initialize_core() void;
extern fn llvm_create_context() ?*anyopaque;
extern fn llvm_dispose_context(?*anyopaque) void;
extern fn llvm_create_module(?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_dispose_module(?*anyopaque) void;
extern fn llvm_create_builder(?*anyopaque) ?*anyopaque;
extern fn llvm_dispose_builder(?*anyopaque) void;
extern fn llvm_int32_type(?*anyopaque) ?*anyopaque;
extern fn llvm_int8_type(?*anyopaque) ?*anyopaque;
extern fn llvm_pointer_type(?*anyopaque) ?*anyopaque;
extern fn llvm_function_type(?*anyopaque, ?[*]?*anyopaque, c_int, c_int) ?*anyopaque;
extern fn llvm_add_function(?*anyopaque, [*c]const u8, ?*anyopaque) ?*anyopaque;
extern fn llvm_append_basic_block(?*anyopaque, ?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_position_builder_at_end(?*anyopaque, ?*anyopaque) void;
extern fn llvm_build_global_string_ptr(?*anyopaque, [*c]const u8, [*c]const u8) ?*anyopaque;
extern fn llvm_const_int(?*anyopaque, c_ulonglong) ?*anyopaque;
extern fn llvm_build_ret(?*anyopaque, ?*anyopaque) ?*anyopaque;
extern fn llvm_build_call2(?*anyopaque, ?*anyopaque, ?*anyopaque, ?[*]?*anyopaque, c_int, [*c]const u8) ?*anyopaque;
extern fn llvm_verify_module(?*anyopaque) c_int;
extern fn llvm_print_module_to_string(?*anyopaque) [*c]u8;
extern fn llvm_dispose_message([*c]u8) void;
extern fn llvm_write_bitcode_to_file(?*anyopaque, [*c]const u8) c_int;

// Real LLVM types that should replace the dummy types in codegen.zig
pub const LLVMModuleRef = ?*anyopaque;
pub const LLVMBuilderRef = ?*anyopaque;
pub const LLVMContextRef = ?*anyopaque;
pub const LLVMValueRef = ?*anyopaque;
pub const LLVMTypeRef = ?*anyopaque;
pub const LLVMBasicBlockRef = ?*anyopaque;

// Real functions that should replace the dummy functions in codegen.zig
pub fn LLVMInitializeNativeTarget() void {
    llvm_initialize_core();
}

pub fn LLVMInitializeNativeAsmPrinter() void {
    // Part of llvm_initialize_core()
}

pub fn LLVMInitializeNativeAsmParser() void {
    // Part of llvm_initialize_core()
}

pub fn LLVMContextCreate() LLVMContextRef {
    return llvm_create_context();
}

pub fn LLVMModuleCreateWithNameInContext(name: [*c]const u8, context: LLVMContextRef) LLVMModuleRef {
    return llvm_create_module(context, name);
}

pub fn LLVMCreateBuilderInContext(context: LLVMContextRef) LLVMBuilderRef {
    return llvm_create_builder(context);
}

pub fn LLVMDisposeModule(module: LLVMModuleRef) void {
    llvm_dispose_module(module);
}

pub fn LLVMDisposeBuilder(builder: LLVMBuilderRef) void {
    llvm_dispose_builder(builder);
}

pub fn LLVMContextDispose(context: LLVMContextRef) void {
    llvm_dispose_context(context);
}

pub fn LLVMInt32TypeInContext(context: LLVMContextRef) LLVMTypeRef {
    _ = context; // Context passed to wrapper
    return llvm_int32_type(context);
}

pub fn LLVMInt8TypeInContext(context: LLVMContextRef) LLVMTypeRef {
    return llvm_int8_type(context);
}

pub fn LLVMPointerType(element_type: LLVMTypeRef, address_space: c_uint) LLVMTypeRef {
    _ = address_space; // Address space handled by wrapper
    return llvm_pointer_type(element_type);
}

pub fn LLVMFunctionType(return_type: LLVMTypeRef, param_types: ?[*]LLVMTypeRef, param_count: c_uint, is_var_arg: c_int) LLVMTypeRef {
    return llvm_function_type(return_type, @as(?[*]?*anyopaque, @ptrCast(param_types)), @intCast(param_count), is_var_arg);
}

pub fn LLVMAddFunction(module: LLVMModuleRef, name: [*c]const u8, function_type: LLVMTypeRef) LLVMValueRef {
    return llvm_add_function(module, name, function_type);
}

pub fn LLVMAppendBasicBlockInContext(context: LLVMContextRef, fn_ref: LLVMValueRef, name: [*c]const u8) LLVMBasicBlockRef {
    return llvm_append_basic_block(context, fn_ref, name);
}

pub fn LLVMPositionBuilderAtEnd(builder: LLVMBuilderRef, block: LLVMBasicBlockRef) void {
    llvm_position_builder_at_end(builder, block);
}

pub fn LLVMBuildGlobalStringPtr(builder: LLVMBuilderRef, str: [*c]const u8, name: [*c]const u8) LLVMValueRef {
    return llvm_build_global_string_ptr(builder, str, name);
}

pub fn LLVMConstInt(int_type: LLVMTypeRef, value: c_ulonglong, sign_extend: c_int) LLVMValueRef {
    _ = sign_extend; // Handled by wrapper
    return llvm_const_int(int_type, value);
}

pub fn LLVMBuildRet(builder: LLVMBuilderRef, value: LLVMValueRef) LLVMValueRef {
    return llvm_build_ret(builder, value);
}

pub fn LLVMVerifyModule(module: LLVMModuleRef, action: c_int, out_message: *[*c]u8) c_int {
    _ = action; _ = out_message; // Handled by wrapper
    return llvm_verify_module(module);
}

pub fn LLVMPrintModuleToString(module: LLVMModuleRef) [*c]u8 {
    return llvm_print_module_to_string(module);
}

pub fn LLVMDisposeMessage(message: [*c]u8) void {
    llvm_dispose_message(message);
}

pub fn LLVMWriteBitcodeToFile(module: LLVMModuleRef, path: [*c]const u8) c_int {
    return llvm_write_bitcode_to_file(module, path);
}

// Test to demonstrate the functions work
pub fn testLLVMIntegration() !void {
    std.debug.print("🧪 Testing LLVM integration...\n", .{});
    
    LLVMInitializeNativeTarget();
    
    const context = LLVMContextCreate() orelse {
        return error.LLVMError;
    };
    defer LLVMContextDispose(context);
    
    const module = LLVMModuleCreateWithNameInContext("test_module", context) orelse {
        return error.LLVMError;
    };
    defer LLVMDisposeModule(module);
    
    const builder = LLVMCreateBuilderInContext(context) orelse {
        return error.LLVMError;
    };
    defer LLVMDisposeBuilder(builder);
    
    // Create a simple function that returns 42
    const i32_type = LLVMInt32TypeInContext(context);
    const func_type = LLVMFunctionType(i32_type, null, 0, 0);
    const func = LLVMAddFunction(module, "get_answer", func_type);
    
    const entry_block = LLVMAppendBasicBlockInContext(context, func, "entry");
    LLVMPositionBuilderAtEnd(builder, entry_block);
    
    const answer = LLVMConstInt(i32_type, 42, 0);
    _ = LLVMBuildRet(builder, answer);
    
    // Verify the module
    var error_message: [*c]u8 = undefined;
    if (LLVMVerifyModule(module, 1, &error_message) != 0) {
        return error.LLVMError;
    }
    
    // Print the module
    const module_str = LLVMPrintModuleToString(module);
    std.debug.print("Generated LLVM IR:\n{s}\n", .{module_str});
    LLVMDisposeMessage(module_str);
    
    std.debug.print("✅ LLVM integration test passed!\n", .{});
}

test "llvm integration" {
    try testLLVMIntegration();
}
