const std = @import("std");
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
});

pub fn main() !void {
    std.debug.print("Starting minimal LLVM test...\n", .{});
    
    // Create context and module
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("minimal_test", context);
    defer c.LLVMDisposeModule(module);
    
    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);
    
    std.debug.print("LLVM initialized successfully\n", .{});
    
    // Create main function: int main() { return 0; }
    const i32_type = c.LLVMInt32TypeInContext(context);
    const main_func_type = c.LLVMFunctionType(i32_type, null, 0, 0);
    const main_func = c.LLVMAddFunction(module, "main", main_func_type);
    
    // Create basic block
    const entry_block = c.LLVMAppendBasicBlockInContext(context, main_func, "entry");
    c.LLVMPositionBuilderAtEnd(builder, entry_block);
    
    // Return 0
    const zero = c.LLVMConstInt(i32_type, 0, 0);
    _ = c.LLVMBuildRet(builder, zero);
    
    std.debug.print("Main function created\n", .{});
    
    // Verify module
    var error_msg: [*c]u8 = null;
    const verify_result = c.LLVMVerifyModule(module, c.LLVMPrintMessageAction, &error_msg);
    if (verify_result != 0) {
        std.debug.print("Module verification failed: {s}\n", .{error_msg});
        c.LLVMDisposeMessage(error_msg);
        return;
    }
    
    std.debug.print("Module verified successfully\n", .{});
    
    // Write IR to file
    var error_msg_write: [*c]u8 = null;
    const write_result = c.LLVMPrintModuleToFile(module, "minimal_test.ll", &error_msg_write);
    if (write_result != 0) {
        std.debug.print("Failed to write IR file\n", .{});
        if (error_msg_write) |msg| {
            std.debug.print("Error: {s}\n", .{msg});
            c.LLVMDisposeMessage(msg);
        }
        return;
    }
    
    std.debug.print("LLVM IR written to minimal_test.ll\n", .{});
    std.debug.print("Minimal LLVM test completed successfully!\n", .{});
}
