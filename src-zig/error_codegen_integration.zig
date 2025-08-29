const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const enhanced_error = @import("enhanced_error_system.zig");
const YikesError = enhanced_error.YikesError;
const ShookResult = enhanced_error.ShookResult;
const FamBlock = enhanced_error.FamBlock;
const StackTrace = enhanced_error.StackTrace;
const ErrorContext = enhanced_error.ErrorContext;
// LLVM C imports disabled to fix "athlon-xp" CPU detection issues
// Replace with dummy types to allow compilation without LLVM
const c = struct {
    // Dummy LLVM types to make compilation work without LLVM
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMExecutionEngineRef = ?*anyopaque;
    pub const LLVMTargetRef = ?*anyopaque;
    pub const LLVMTargetMachineRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    pub const LLVMMemoryBufferRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy functions to prevent link errors (add more as needed)
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMInitializeX86TargetInfo() void {}
    pub fn LLVMInitializeX86Target() void {}
    pub fn LLVMInitializeX86TargetMC() void {}
    pub fn LLVMInitializeX86AsmPrinter() void {}
    pub fn LLVMCreateTargetMachine() LLVMTargetMachineRef { return null; }
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}
};

/// Error Handling Code Generation Integration
/// Provides LLVM IR generation for CURSED error handling constructs

pub const ErrorCodeGen = struct {
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    allocator: Allocator,
    
    // Error handling runtime functions
    error_create_func: ?c.LLVMValueRef,
    error_destroy_func: ?c.LLVMValueRef,
    stack_trace_capture_func: ?c.LLVMValueRef,
    stack_trace_destroy_func: ?c.LLVMValueRef,
    defer_push_func: ?c.LLVMValueRef,
    defer_execute_func: ?c.LLVMValueRef,
    
    // Error types
    yikes_error_type: ?c.LLVMTypeRef,
    shook_result_type: ?c.LLVMTypeRef,
    stack_trace_type: ?c.LLVMTypeRef,
    
    // Current function context
    current_function: ?c.LLVMValueRef,
    current_file: []const u8,
    
    pub fn init(context: c.LLVMContextRef, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef, allocator: Allocator) ErrorCodeGen {
        _ = allocator;
        return ErrorCodeGen{
            .context = context,
            .module = module,
            .builder = builder,
            .allocator = allocator,
            .error_create_func = null,
            .error_destroy_func = null,
            .stack_trace_capture_func = null,
            .stack_trace_destroy_func = null,
            .defer_push_func = null,
            .defer_execute_func = null,
            .yikes_error_type = null,
            .shook_result_type = null,
            .stack_trace_type = null,
            .current_function = null,
            .current_file = "unknown",
        };
    }
    
    pub fn setupRuntimeFunctions(self: *ErrorCodeGen) void {
        // YikesError type (opaque pointer)
        self.yikes_error_type = c.LLVMPointerTypeInContext(self.context, 0);
        
        // StackTrace type (opaque pointer)
        self.stack_trace_type = c.LLVMPointerTypeInContext(self.context, 0);
        
        // ShookResult type (struct with tag and union)
        const result_fields = [_]c.LLVMTypeRef{
            c.LLVMInt8TypeInContext(self.context), // tag
            c.LLVMInt64TypeInContext(self.context), // value union (simplified)
        };
        self.shook_result_type = c.LLVMStructTypeInContext(self.context, &result_fields, 2, 0);
        
        // Error creation function: YikesError* cursed_error_create(const char*, size_t, i64, u32)
        const error_create_params = [_]c.LLVMTypeRef{
            c.LLVMPointerTypeInContext(self.context, 0), // message
            c.LLVMInt64TypeInContext(self.context),      // message length
            c.LLVMInt64TypeInContext(self.context),      // error code
            c.LLVMInt32TypeInContext(self.context),      // error type
        };
        const error_create_type = c.LLVMFunctionType(
            self.yikes_error_type.?,
            &error_create_params,
            4,
            0
        );
        self.error_create_func = c.LLVMAddFunction(self.module, "cursed_error_create", error_create_type);
        
        // Error destruction function: void cursed_error_destroy(YikesError*)
        const error_destroy_params = [_]c.LLVMTypeRef{self.yikes_error_type.?};
        const error_destroy_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &error_destroy_params,
            1,
            0
        );
        self.error_destroy_func = c.LLVMAddFunction(self.module, "cursed_error_destroy", error_destroy_type);
        
        // Stack trace capture function: StackTrace* cursed_stack_trace_capture()
        const stack_trace_capture_type = c.LLVMFunctionType(
            self.stack_trace_type.?,
            null,
            0,
            0
        );
        self.stack_trace_capture_func = c.LLVMAddFunction(self.module, "cursed_stack_trace_capture", stack_trace_capture_type);
        
        // Stack trace destruction function: void cursed_stack_trace_destroy(StackTrace*)
        const stack_trace_destroy_params = [_]c.LLVMTypeRef{self.stack_trace_type.?};
        const stack_trace_destroy_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &stack_trace_destroy_params,
            1,
            0
        );
        self.stack_trace_destroy_func = c.LLVMAddFunction(self.module, "cursed_stack_trace_destroy", stack_trace_destroy_type);
        
        // Defer push function: void cursed_defer_push(void(*)(), const char*)
        const defer_push_params = [_]c.LLVMTypeRef{
            c.LLVMPointerTypeInContext(self.context, 0), // cleanup function
            c.LLVMPointerTypeInContext(self.context, 0), // context string
        };
        const defer_push_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &defer_push_params,
            2,
            0
        );
        self.defer_push_func = c.LLVMAddFunction(self.module, "cursed_defer_push", defer_push_type);
        
        // Defer execute function: void cursed_defer_execute_all()
        const defer_execute_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            null,
            0,
            0
        );
        self.defer_execute_func = c.LLVMAddFunction(self.module, "cursed_defer_execute_all", defer_execute_type);
    }
    
    /// Generate YIKES error creation
    pub fn generateYikes(
        self: *ErrorCodeGen,
        message: []const u8,
        code: i64,
        error_type: u32,
        file: []const u8,
        line: u32,
        column: u32
    ) c.LLVMValueRef {
        // Create string constant for message
        const message_global = c.LLVMAddGlobal(self.module, c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(message.len)), "error_msg");
        const message_init = c.LLVMConstStringInContext(self.context, message.ptr, @intCast(message.len), 0);
        c.LLVMSetInitializer(message_global, message_init);
        c.LLVMSetGlobalConstant(message_global, 1);
        c.LLVMSetLinkage(message_global, c.LLVMPrivateLinkage);
        
        // Get pointer to message data
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        const indices = [_]c.LLVMValueRef{ zero, zero };
        const message_ptr = c.LLVMBuildGEP2(
            self.builder,
            c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(message.len)),
            message_global,
            &indices,
            2,
            "msg_ptr"
        );
        
        // Create arguments for error creation
        const args = [_]c.LLVMValueRef{
            message_ptr,
            c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), message.len, 0),
            c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @bitCast(code), 0),
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), error_type, 0),
        };
        
        // Call error creation function
        const error_value = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGlobalGetValueType(self.error_create_func.?),
            self.error_create_func.?,
            &args,
            4,
            "yikes_error"
        );
        
        // Add debug metadata for source location
        self.addDebugLocation(error_value, file, line, column);
        
        return error_value;
    }
    
    /// Generate SHOOK error propagation
    pub fn generateShook(self: *ErrorCodeGen, expression: c.LLVMValueRef) c.LLVMValueRef {
        // Create basic block for error checking
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        const error_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "shook_error");
        const success_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "shook_success");
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "shook_merge");
        
        // Check if expression is an error (simplified check)
        const is_null = c.LLVMBuildIsNull(self.builder, expression, "is_error");
        _ = c.LLVMBuildCondBr(self.builder, is_null, error_block, success_block);
        
        // Error block: propagate error
        c.LLVMPositionBuilderAtEnd(self.builder, error_block);
        
        // Capture stack trace
        const stack_trace = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGlobalGetValueType(self.stack_trace_capture_func.?),
            self.stack_trace_capture_func.?,
            null,
            0,
            "captured_trace"
        );
        
        // TODO: Attach stack trace to error
        _ = stack_trace;
        
        // Return error (early return)
        _ = c.LLVMBuildRet(self.builder, expression);
        
        // Success block: continue execution
        c.LLVMPositionBuilderAtEnd(self.builder, success_block);
        _ = c.LLVMBuildBr(self.builder, merge_block);
        
        // Merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
        
        // Create PHI node for result
        const phi = c.LLVMBuildPhi(self.builder, c.LLVMTypeOf(expression), "shook_result");
        const phi_values = [_]c.LLVMValueRef{expression};
        const phi_blocks = [_]c.LLVMBasicBlockRef{success_block};
        c.LLVMAddIncoming(phi, &phi_values, &phi_blocks, 1);
        
        return phi;
    }
    
    /// Generate FAM panic recovery block
    pub fn generateFam(
        self: *ErrorCodeGen,
        try_block: c.LLVMBasicBlockRef,
        catch_block: c.LLVMBasicBlockRef,
        finally_block: ?c.LLVMBasicBlockRef
    ) c.LLVMValueRef {
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        const fam_entry = c.LLVMAppendBasicBlockInContext(self.context, current_func, "fam_entry");
        const fam_exit = c.LLVMAppendBasicBlockInContext(self.context, current_func, "fam_exit");
        
        // Jump to try block
        _ = c.LLVMBuildBr(self.builder, try_block);
        
        // Position at fam entry
        c.LLVMPositionBuilderAtEnd(self.builder, fam_entry);
        
        // Setup exception handling (simplified)
        // In a real implementation, this would set up exception tables
        
        // Execute try block
        c.LLVMPositionBuilderAtEnd(self.builder, try_block);
        
        // On error, jump to catch block
        // On success, jump to finally block (if any), then exit
        if (finally_block) |finally| {
            _ = c.LLVMBuildBr(self.builder, finally);
            c.LLVMPositionBuilderAtEnd(self.builder, finally);
            _ = c.LLVMBuildBr(self.builder, fam_exit);
        } else {
            _ = c.LLVMBuildBr(self.builder, fam_exit);
        }
        
        // Catch block handling
        c.LLVMPositionBuilderAtEnd(self.builder, catch_block);
        if (finally_block) |finally| {
            _ = c.LLVMBuildBr(self.builder, finally);
        } else {
            _ = c.LLVMBuildBr(self.builder, fam_exit);
        }
        
        // Exit block
        c.LLVMPositionBuilderAtEnd(self.builder, fam_exit);
        
        // Return void for now
        return c.LLVMConstNull(c.LLVMVoidTypeInContext(self.context));
    }
    
    /// Generate defer statement
    pub fn generateDefer(self: *ErrorCodeGen, cleanup_func: c.LLVMValueRef, context: []const u8) void {
        // Create string constant for context
        const context_global = c.LLVMAddGlobal(self.module, c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(context.len)), "defer_ctx");
        const context_init = c.LLVMConstStringInContext(self.context, context.ptr, @intCast(context.len), 0);
        c.LLVMSetInitializer(context_global, context_init);
        c.LLVMSetGlobalConstant(context_global, 1);
        c.LLVMSetLinkage(context_global, c.LLVMPrivateLinkage);
        
        // Get pointer to context data
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        const indices = [_]c.LLVMValueRef{ zero, zero };
        const context_ptr = c.LLVMBuildGEP2(
            self.builder,
            c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(context.len)),
            context_global,
            &indices,
            2,
            "ctx_ptr"
        );
        
        // Call defer push function
        const args = [_]c.LLVMValueRef{ cleanup_func, context_ptr };
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGlobalGetValueType(self.defer_push_func.?),
            self.defer_push_func.?,
            &args,
            2,
            ""
        );
    }
    
    /// Generate cleanup code at function exit
    pub fn generateCleanup(self: *ErrorCodeGen) void {
        // Call defer execute function
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGlobalGetValueType(self.defer_execute_func.?),
            self.defer_execute_func.?,
            null,
            0,
            ""
        );
    }
    
    /// Add debug location information
    fn addDebugLocation(self: *ErrorCodeGen, value: c.LLVMValueRef, file: []const u8, line: u32, column: u32) void {
        _ = self;
        _ = value;
        _ = file;
        _ = line;
        _ = column;
        // TODO: Add DWARF debug information
        // This would require setting up debug info builder and metadata
    }
    
    /// Set current function context
    pub fn setCurrentFunction(self: *ErrorCodeGen, function: c.LLVMValueRef, file: []const u8) void {
        self.current_function = function;
        self.current_file = file;
    }
    
    /// Generate error unwinding code
    pub fn generateUnwind(self: *ErrorCodeGen, target_scope: u32) void {
        // Generate code to unwind stack to target scope
        // This involves calling defer cleanup functions in reverse order
        
        const scope_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), target_scope, 0);
        
        // Create unwind function call
        const unwind_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMInt32TypeInContext(self.context)},
            1,
            0
        );
        const unwind_func = c.LLVMAddFunction(self.module, "cursed_unwind_to_scope", unwind_func_type);
        
        const args = [_]c.LLVMValueRef{scope_value};
        _ = c.LLVMBuildCall2(
            self.builder,
            unwind_func_type,
            unwind_func,
            &args,
            1,
            ""
        );
    }
    
    /// Generate stack unwinding for panic propagation
    pub fn generateStackUnwind(self: *ErrorCodeGen) void {
        // Generate call to stack unwind runtime function
        const unwind_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            null,
            0,
            0
        );
        const unwind_func = c.LLVMAddFunction(self.module, "cursed_stack_unwind", unwind_func_type);
        
        _ = c.LLVMBuildCall2(
            self.builder,
            unwind_func_type,
            unwind_func,
            null,
            0,
            ""
        );
    }
    
    /// Generate panic creation with context preservation
    pub fn generatePanicCreate(self: *ErrorCodeGen, message: []const u8, file: []const u8, line: u32) c.LLVMValueRef {
        // Create string constant for message
        const message_global = c.LLVMAddGlobal(self.module, c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(message.len)), "panic_msg");
        const message_init = c.LLVMConstStringInContext(self.context, message.ptr, @intCast(message.len), 0);
        c.LLVMSetInitializer(message_global, message_init);
        c.LLVMSetGlobalConstant(message_global, 1);
        c.LLVMSetLinkage(message_global, c.LLVMPrivateLinkage);
        
        // Get pointer to message data
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        const indices = [_]c.LLVMValueRef{ zero, zero };
        const message_ptr = c.LLVMBuildGEP2(
            self.builder,
            c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(message.len)),
            message_global,
            &indices,
            2,
            "panic_msg_ptr"
        );
        
        // Create panic function call
        const panic_func_type = c.LLVMFunctionType(
            c.LLVMPointerTypeInContext(self.context, 0),
            &[_]c.LLVMTypeRef{c.LLVMPointerTypeInContext(self.context, 0)},
            1,
            0
        );
        const panic_func = c.LLVMAddFunction(self.module, "cursed_panic_create", panic_func_type);
        
        const args = [_]c.LLVMValueRef{message_ptr};
        const panic_value = c.LLVMBuildCall2(
            self.builder,
            panic_func_type,
            panic_func,
            &args,
            1,
            "panic_obj"
        );
        
        // Add debug metadata for source location
        self.addDebugLocation(panic_value, file, line, 0);
        
        return panic_value;
    }
    
    /// Generate panic recovery
    pub fn generatePanicRecover(self: *ErrorCodeGen) c.LLVMValueRef {
        // Create panic recover function call
        const recover_func_type = c.LLVMFunctionType(
            c.LLVMPointerTypeInContext(self.context, 0),
            null,
            0,
            0
        );
        const recover_func = c.LLVMAddFunction(self.module, "cursed_panic_recover", recover_func_type);
        
        const recovered_panic = c.LLVMBuildCall2(
            self.builder,
            recover_func_type,
            recover_func,
            null,
            0,
            "recovered_panic"
        );
        
        return recovered_panic;
    }
    
    /// Generate defer cleanup during unwinding
    pub fn generateDeferCleanupDuringUnwind(self: *ErrorCodeGen, scope_level: u32) void {
        // Create scope level value
        const scope_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), scope_level, 0);
        
        // Create defer cleanup function call
        const cleanup_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMInt32TypeInContext(self.context)},
            1,
            0
        );
        const cleanup_func = c.LLVMAddFunction(self.module, "cursed_defer_cleanup_scope", cleanup_func_type);
        
        const args = [_]c.LLVMValueRef{scope_value};
        _ = c.LLVMBuildCall2(
            self.builder,
            cleanup_func_type,
            cleanup_func,
            &args,
            1,
            ""
        );
    }
    
    /// Generate goroutine panic propagation
    pub fn generateGoroutinePanicPropagation(self: *ErrorCodeGen, panic_obj: c.LLVMValueRef) void {
        // Create goroutine panic propagation function call
        const propagate_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMPointerTypeInContext(self.context, 0)},
            1,
            0
        );
        const propagate_func = c.LLVMAddFunction(self.module, "cursed_goroutine_panic_propagate", propagate_func_type);
        
        const args = [_]c.LLVMValueRef{panic_obj};
        _ = c.LLVMBuildCall2(
            self.builder,
            propagate_func_type,
            propagate_func,
            &args,
            1,
            ""
        );
    }
    
    /// Generate error context creation
    pub fn generateErrorContext(
        self: *ErrorCodeGen,
        function_name: []const u8,
        file: []const u8,
        line: u32,
        column: u32
    ) c.LLVMValueRef {
        // Create function name constant
        const func_name_global = c.LLVMAddGlobal(self.module, c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(function_name.len)), "func_name");
        const func_name_init = c.LLVMConstStringInContext(self.context, function_name.ptr, @intCast(function_name.len), 0);
        c.LLVMSetInitializer(func_name_global, func_name_init);
        c.LLVMSetGlobalConstant(func_name_global, 1);
        
        // Create file name constant
        const file_name_global = c.LLVMAddGlobal(self.module, c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(file.len)), "file_name");
        const file_name_init = c.LLVMConstStringInContext(self.context, file.ptr, @intCast(file.len), 0);
        c.LLVMSetInitializer(file_name_global, file_name_init);
        c.LLVMSetGlobalConstant(file_name_global, 1);
        
        // Create context structure
        const context_fields = [_]c.LLVMTypeRef{
            c.LLVMPointerTypeInContext(self.context, 0), // function_name
            c.LLVMPointerTypeInContext(self.context, 0), // file_name
            c.LLVMInt32TypeInContext(self.context),      // line
            c.LLVMInt32TypeInContext(self.context),      // column
        };
        const context_type = c.LLVMStructTypeInContext(self.context, &context_fields, 4, 0);
        
        // Allocate context on stack
        const context_alloca = c.LLVMBuildAlloca(self.builder, context_type, "error_context");
        
        // Initialize context fields
        _ = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        
        // Set function name
        const func_name_gep = c.LLVMBuildStructGEP2(self.builder, context_type, context_alloca, 0, "func_name_ptr");
        _ = c.LLVMBuildStore(self.builder, func_name_global, func_name_gep);
        
        // Set file name
        const file_name_gep = c.LLVMBuildStructGEP2(self.builder, context_type, context_alloca, 1, "file_name_ptr");
        _ = c.LLVMBuildStore(self.builder, file_name_global, file_name_gep);
        
        // Set line number
        const line_gep = c.LLVMBuildStructGEP2(self.builder, context_type, context_alloca, 2, "line_ptr");
        const line_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), line, 0);
        _ = c.LLVMBuildStore(self.builder, line_value, line_gep);
        
        // Set column number
        const column_gep = c.LLVMBuildStructGEP2(self.builder, context_type, context_alloca, 3, "column_ptr");
        const column_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), column, 0);
        _ = c.LLVMBuildStore(self.builder, column_value, column_gep);
        
        return context_alloca;
    }
};

test "error codegen integration" {
    const allocator = std.testing.allocator;
    
    // Initialize LLVM
    c.LLVMInitializeNativeTarget();
    c.LLVMInitializeNativeAsmPrinter();
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);
    
    var error_codegen = ErrorCodeGen.init(context, module, builder, allocator);
    error_codegen.setupRuntimeFunctions();
    
    // Test basic setup
    try std.testing.expect(error_codegen.yikes_error_type != null);
    try std.testing.expect(error_codegen.error_create_func != null);
    
    // Create a test function
    const func_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0);
    const test_func = c.LLVMAddFunction(module, "test_func", func_type);
    const entry_block = c.LLVMAppendBasicBlockInContext(context, test_func, "entry");
    c.LLVMPositionBuilderAtEnd(builder, entry_block);
    
    error_codegen.setCurrentFunction(test_func, "test.csd");
    
    // Test error creation
    const error_value = error_codegen.generateYikes(
        "Test error message",
        42,
        0, // Runtime error
        "test.csd",
        10,
        5
    );
    try std.testing.expect(error_value != null);
    
    // Test error context
    const context_value = error_codegen.generateErrorContext(
        "test_function",
        "test.csd",
        10,
        5
    );
    try std.testing.expect(context_value != null);
    
    // Verify the module is valid
    var error_msg: [*c]u8 = undefined;
    const is_valid = c.LLVMVerifyModule(module, c.LLVMReturnStatusAction, &error_msg) == 0;
    try std.testing.expect(is_valid);
}
