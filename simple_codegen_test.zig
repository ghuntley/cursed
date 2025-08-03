const std = @import("std");

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
});

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    
    // Initialize LLVM
    _ = c.LLVMInitializeNativeTarget();
    _ = c.LLVMInitializeNativeAsmPrinter();
    _ = c.LLVMInitializeNativeAsmParser();
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("cursed_test", context);
    defer c.LLVMDisposeModule(module);
    
    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);
    
    // Set target triple
    const target_triple = c.LLVMGetDefaultTargetTriple();
    c.LLVMSetTarget(module, target_triple);
    
    // Declare puts function
    var puts_params = [_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)};
    const puts_type = c.LLVMFunctionType(
        c.LLVMInt32TypeInContext(context), // return int
        &puts_params,
        1, // parameter count
        0  // not variadic
    );
    const puts_func = c.LLVMAddFunction(module, "puts", puts_type);
    
    // Declare printf function
    var printf_params = [_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)};
    const printf_type = c.LLVMFunctionType(
        c.LLVMInt32TypeInContext(context), // return int
        &printf_params,
        1, // parameter count
        1  // is variadic
    );
    const printf_func = c.LLVMAddFunction(module, "printf", printf_type);
    
    // Create main_character function
    const main_char_type = c.LLVMFunctionType(
        c.LLVMVoidTypeInContext(context), // return void
        null, // no parameters
        0, // parameter count
        0  // not variadic
    );
    const main_char_func = c.LLVMAddFunction(module, "main_character", main_char_type);
    
    // Create entry block for main_character
    const entry_block = c.LLVMAppendBasicBlockInContext(context, main_char_func, "entry");
    c.LLVMPositionBuilderAtEnd(builder, entry_block);
    
    // Generate: vibez.spill("Hello from CURSED Zig!")
    const hello_str = c.LLVMBuildGlobalStringPtr(builder, "Hello from CURSED Zig!", "hello_str");
    var hello_args = [_]c.LLVMValueRef{hello_str};
    _ = c.LLVMBuildCall2(
        builder,
        c.LLVMGetReturnType(c.LLVMGlobalGetValueType(puts_func)),
        puts_func,
        @ptrCast(&hello_args),
        1,
        "puts_call1"
    );
    
    // Generate: sus x drip = 42
    const x_type = c.LLVMInt64TypeInContext(context);
    const x_alloca = c.LLVMBuildAlloca(builder, x_type, "x");
    const x_value = c.LLVMConstInt(x_type, 42, 0);
    _ = c.LLVMBuildStore(builder, x_value, x_alloca);
    
    // Generate: vibez.spill(x)
    const x_loaded = c.LLVMBuildLoad2(builder, x_type, x_alloca, "x_load");
    const format = c.LLVMBuildGlobalStringPtr(builder, "%lld\n", "int_fmt");
    var printf_args = [_]c.LLVMValueRef{ format, x_loaded };
    _ = c.LLVMBuildCall2(
        builder,
        c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)),
        printf_func,
        @ptrCast(&printf_args),
        2,
        "printf_call"
    );
    
    // Return void
    _ = c.LLVMBuildRetVoid(builder);
    
    // Create main function that calls main_character
    const main_type = c.LLVMFunctionType(
        c.LLVMInt32TypeInContext(context), // return int
        null, // no parameters
        0, // parameter count
        0  // not variadic
    );
    const main_func = c.LLVMAddFunction(module, "main", main_type);
    const main_entry = c.LLVMAppendBasicBlockInContext(context, main_func, "entry");
    c.LLVMPositionBuilderAtEnd(builder, main_entry);
    
    // Call main_character
    _ = c.LLVMBuildCall2(
        builder,
        c.LLVMGetReturnType(c.LLVMGlobalGetValueType(main_char_func)),
        main_char_func,
        null,
        0,
        "main_char_call"
    );
    
    // Return 0
    const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0);
    _ = c.LLVMBuildRet(builder, zero);
    
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
    if (c.LLVMPrintModuleToFile(module, "cursed_test.ll", &error_msg) != 0) {
        std.debug.print("Failed to write IR: {s}\n", .{error_msg});
        c.LLVMDisposeMessage(error_msg);
        return;
    }
    
    std.debug.print("\nCompiling to executable with clang...\n", .{});
    
    // Use system to compile the IR to executable
    var child = std.process.Child.init(&[_][]const u8{ "clang", "-O2", "cursed_test.ll", "-o", "cursed_test" }, allocator);
    const result = child.spawnAndWait() catch |err| {
        std.debug.print("Failed to run clang: {}\n", .{err});
        return;
    };
    
    switch (result) {
        .Exited => |code| {
            if (code == 0) {
                std.debug.print("Successfully compiled to: cursed_test\n", .{});
                std.debug.print("\nRunning the compiled program:\n", .{});
                
                // Run the compiled program
                var run_child = std.process.Child.init(&[_][]const u8{"./cursed_test"}, allocator);
                const run_result = run_child.spawnAndWait() catch |err| {
                    std.debug.print("Failed to run program: {}\n", .{err});
                    return;
                };
                
                switch (run_result) {
                    .Exited => |run_code| {
                        if (run_code == 0) {
                            std.debug.print("Program executed successfully!\n", .{});
                        } else {
                            std.debug.print("Program exited with code: {}\n", .{run_code});
                        }
                    },
                    else => {
                        std.debug.print("Program execution failed\n", .{});
                    },
                }
            } else {
                std.debug.print("Clang failed with exit code: {}\n", .{code});
            }
        },
        else => {
            std.debug.print("Clang process failed\n", .{});
        },
    }
}
