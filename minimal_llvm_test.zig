const std = @import("std");

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
});

pub fn main() !void {
    std.debug.print("Testing LLVM IR generation...\n", .{});
    
    // Initialize LLVM
    _ = c.LLVMInitializeNativeTarget();
    _ = c.LLVMInitializeNativeAsmPrinter();
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("cursed_test", context);
    defer c.LLVMDisposeModule(module);
    
    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);
    
    // Create a simple function that returns 42
    const function_type = c.LLVMFunctionType(
        c.LLVMInt32TypeInContext(context), // return int
        null, // no parameters
        0, // parameter count
        0  // not variadic
    );
    
    const function = c.LLVMAddFunction(module, "get_answer", function_type);
    const entry_block = c.LLVMAppendBasicBlockInContext(context, function, "entry");
    c.LLVMPositionBuilderAtEnd(builder, entry_block);
    
    // Return 42
    const answer = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 42, 0);
    _ = c.LLVMBuildRet(builder, answer);
    
    // Create main function
    const main_type = c.LLVMFunctionType(
        c.LLVMInt32TypeInContext(context), // return int
        null, // no parameters
        0, // parameter count
        0  // not variadic
    );
    
    const main_func = c.LLVMAddFunction(module, "main", main_type);
    const main_entry = c.LLVMAppendBasicBlockInContext(context, main_func, "entry");
    c.LLVMPositionBuilderAtEnd(builder, main_entry);
    
    // Call get_answer and return its result
    const result = c.LLVMBuildCall2(
        builder,
        c.LLVMGetReturnType(c.LLVMGlobalGetValueType(function)),
        function,
        null,
        0,
        "answer_call"
    );
    _ = c.LLVMBuildRet(builder, result);
    
    // Verify the module
    var error_msg: [*c]u8 = undefined;
    if (c.LLVMVerifyModule(module, c.LLVMReturnStatusAction, &error_msg) != 0) {
        std.debug.print("Module verification failed: {s}\n", .{error_msg});
        c.LLVMDisposeMessage(error_msg);
        return;
    }
    
    // Print LLVM IR
    const ir_string = c.LLVMPrintModuleToString(module);
    if (ir_string != null) {
        std.debug.print("Generated LLVM IR:\n{s}\n", .{ir_string});
        c.LLVMDisposeMessage(ir_string);
    }
    
    // Write IR to file
    if (c.LLVMPrintModuleToFile(module, "minimal_test.ll", &error_msg) != 0) {
        std.debug.print("Failed to write IR: {s}\n", .{error_msg});
        c.LLVMDisposeMessage(error_msg);
        return;
    }
    
    std.debug.print("Successfully generated LLVM IR file: minimal_test.ll\n", .{});
    std.debug.print("You can compile it with: clang minimal_test.ll -o minimal_test\n", .{});
}
