const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
});

/// CURSED Runtime System for native executables
pub const RuntimeSystem = struct {
    allocator: Allocator,
    
    pub const RuntimeError = error{
        InitializationError,
        MemoryError,
        ThreadError,
        ChannelError,
        GCError,
    };
    
    pub fn init(allocator: Allocator) RuntimeSystem {
        return RuntimeSystem{
            .allocator = allocator,
        };
    }
    
    /// Generate runtime library code in LLVM IR
    pub fn generateRuntimeLibrary(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        try self.generateMemoryManager(context, module);
        try self.generateGarbageCollector(context, module);
        try self.generateChannelRuntime(context, module);
        try self.generateGoroutineScheduler(context, module);
        try self.generateStringRuntime(context, module);
        try self.generateErrorHandling(context, module);
        try self.generateIORuntime(context, module);
    }
    
    /// Generate memory management runtime
    fn generateMemoryManager(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        // Declare external C functions
        const malloc_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            &[_]c.LLVMTypeRef{c.LLVMInt64TypeInContext(context)},
            1, 0
        );
        _ = c.LLVMAddFunction(module, "malloc", malloc_type);
        
        const free_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
            1, 0
        );
        _ = c.LLVMAddFunction(module, "free", free_type);
        
        // CURSED memory allocator wrapper
        const cursed_alloc_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            &[_]c.LLVMTypeRef{
                c.LLVMInt64TypeInContext(context), // size
                c.LLVMInt32TypeInContext(context), // type_id for GC
            },
            2, 0
        );
        const cursed_alloc = c.LLVMAddFunction(module, "cursed_alloc", cursed_alloc_type);
        
        // Generate function body
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const entry_block = c.LLVMAppendBasicBlockInContext(context, cursed_alloc, "entry");
        c.LLVMPositionBuilderAtEnd(builder, entry_block);
        
        const size_param = c.LLVMGetParam(cursed_alloc, 0);
        const type_id_param = c.LLVMGetParam(cursed_alloc, 1);
        
        // Add GC header size (16 bytes)
        const header_size = c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 16, 0);
        const total_size = c.LLVMBuildAdd(builder, size_param, header_size, "total_size");
        
        // Call malloc
        const malloc_func = c.LLVMGetNamedFunction(module, "malloc");
        const raw_ptr = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(malloc_func)), malloc_func, &[_]c.LLVMValueRef{total_size}, 1, "raw_ptr");
        
        // Initialize GC header
        const header_ptr = c.LLVMBuildBitCast(builder, raw_ptr, c.LLVMPointerType(c.LLVMInt64TypeInContext(context), 0), "header_ptr");
        
        // Store size
        const size_ptr = c.LLVMBuildGEP2(builder, c.LLVMInt64TypeInContext(context), header_ptr, &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0)}, 1, "size_ptr");
        _ = c.LLVMBuildStore(builder, size_param, size_ptr);
        
        // Store type_id
        const type_id_ptr = c.LLVMBuildGEP2(builder, c.LLVMInt64TypeInContext(context), header_ptr, &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 1, 0)}, 1, "type_id_ptr");
        const type_id_ext = c.LLVMBuildZExt(builder, type_id_param, c.LLVMInt64TypeInContext(context), "type_id_ext");
        _ = c.LLVMBuildStore(builder, type_id_ext, type_id_ptr);
        
        // Return user pointer
        const user_ptr = c.LLVMBuildGEP2(builder, c.LLVMInt8TypeInContext(context), raw_ptr, &[_]c.LLVMValueRef{header_size}, 1, "user_ptr");
        _ = c.LLVMBuildRet(builder, user_ptr);
    }
    
    /// Generate garbage collector runtime
    fn generateGarbageCollector(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        // GC mark function
        const mark_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
            1, 0
        );
        const mark_func = c.LLVMAddFunction(module, "cursed_gc_mark", mark_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const mark_entry = c.LLVMAppendBasicBlockInContext(context, mark_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, mark_entry);
        
        // For now, just return (marking implementation would be more complex)
        _ = c.LLVMBuildRetVoid(builder);
        
        // GC sweep function
        const sweep_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0);
        const sweep_func = c.LLVMAddFunction(module, "cursed_gc_sweep", sweep_type);
        
        const sweep_entry = c.LLVMAppendBasicBlockInContext(context, sweep_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, sweep_entry);
        
        // For now, just return (sweep implementation would be more complex)
        _ = c.LLVMBuildRetVoid(builder);
        
        // GC collect function
        const collect_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0);
        const collect_func = c.LLVMAddFunction(module, "cursed_gc_collect", collect_type);
        
        const collect_entry = c.LLVMAppendBasicBlockInContext(context, collect_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, collect_entry);
        
        // Call mark and sweep
        _ = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(sweep_func)), sweep_func, null, 0, "");
        _ = c.LLVMBuildRetVoid(builder);
    }
    
    /// Generate channel runtime system
    fn generateChannelRuntime(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        // Channel structure: { queue*, mutex, capacity, size, closed }
        const channel_fields = [_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // queue pointer
            c.LLVMInt64TypeInContext(context), // mutex (pthread_mutex_t)
            c.LLVMInt32TypeInContext(context), // capacity
            c.LLVMInt32TypeInContext(context), // size
            c.LLVMInt1TypeInContext(context),  // closed
        };
        const channel_type = c.LLVMStructTypeInContext(context, &channel_fields, 5, 0);
        
        // Channel creation function
        const create_chan_type = c.LLVMFunctionType(
            c.LLVMPointerType(channel_type, 0),
            &[_]c.LLVMTypeRef{c.LLVMInt32TypeInContext(context)}, // capacity
            1, 0
        );
        const create_chan = c.LLVMAddFunction(module, "cursed_channel_create", create_chan_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const create_entry = c.LLVMAppendBasicBlockInContext(context, create_chan, "entry");
        c.LLVMPositionBuilderAtEnd(builder, create_entry);
        
        // Allocate channel structure
        const channel_size = c.LLVMSizeOf(channel_type);
        const cursed_alloc = c.LLVMGetNamedFunction(module, "cursed_alloc");
        const chan_ptr = c.LLVMBuildCall2(
            builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(cursed_alloc)),
            cursed_alloc,
            &[_]c.LLVMValueRef{
                channel_size,
                c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 2, 0) // channel type_id
            },
            2,
            "chan_alloc"
        );
        
        const typed_chan_ptr = c.LLVMBuildBitCast(builder, chan_ptr, c.LLVMPointerType(channel_type, 0), "typed_chan");
        
        // Initialize channel fields
        const capacity_param = c.LLVMGetParam(create_chan, 0);
        
        // Set capacity
        const capacity_ptr = c.LLVMBuildStructGEP2(builder, channel_type, typed_chan_ptr, 2, "capacity_ptr");
        _ = c.LLVMBuildStore(builder, capacity_param, capacity_ptr);
        
        // Set size to 0
        const size_ptr = c.LLVMBuildStructGEP2(builder, channel_type, typed_chan_ptr, 3, "size_ptr");
        _ = c.LLVMBuildStore(builder, c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0), size_ptr);
        
        // Set closed to false
        const closed_ptr = c.LLVMBuildStructGEP2(builder, channel_type, typed_chan_ptr, 4, "closed_ptr");
        _ = c.LLVMBuildStore(builder, c.LLVMConstInt(c.LLVMInt1TypeInContext(context), 0, 0), closed_ptr);
        
        _ = c.LLVMBuildRet(builder, typed_chan_ptr);
        
        // Channel send function
        const send_type = c.LLVMFunctionType(
            c.LLVMInt1TypeInContext(context), // success/failure
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(channel_type, 0), // channel
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // data
            },
            2, 0
        );
        const send_func = c.LLVMAddFunction(module, "cursed_channel_send", send_type);
        
        const send_entry = c.LLVMAppendBasicBlockInContext(context, send_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, send_entry);
        
        // For now, just return true (full implementation would handle buffering and blocking)
        _ = c.LLVMBuildRet(builder, c.LLVMConstInt(c.LLVMInt1TypeInContext(context), 1, 0));
        
        // Channel receive function
        const receive_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // received data (null if channel closed)
            &[_]c.LLVMTypeRef{c.LLVMPointerType(channel_type, 0)}, // channel
            1, 0
        );
        const receive_func = c.LLVMAddFunction(module, "cursed_channel_receive", receive_type);
        
        const receive_entry = c.LLVMAppendBasicBlockInContext(context, receive_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, receive_entry);
        
        // For now, just return null (full implementation would handle dequeuing)
        _ = c.LLVMBuildRet(builder, c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)));
    }
    
    /// Generate goroutine scheduler
    fn generateGoroutineScheduler(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        // Goroutine function type: void (*)(void*)
        const goroutine_func_type = c.LLVMPointerType(
            c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
                1, 0
            ),
            0
        );
        
        // Spawn goroutine function
        const spawn_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(context),
            &[_]c.LLVMTypeRef{
                goroutine_func_type, // function to run
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // arguments
            },
            2, 0
        );
        const spawn_func = c.LLVMAddFunction(module, "cursed_goroutine_spawn", spawn_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const spawn_entry = c.LLVMAppendBasicBlockInContext(context, spawn_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, spawn_entry);
        
        // For now, just call the function directly (full implementation would use thread pool)
        const func_param = c.LLVMGetParam(spawn_func, 0);
        const args_param = c.LLVMGetParam(spawn_func, 1);
        
        _ = c.LLVMBuildCall2(
            builder,
            c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
                1, 0
            ),
            func_param,
            &[_]c.LLVMValueRef{args_param},
            1,
            ""
        );
        
        _ = c.LLVMBuildRetVoid(builder);
        
        // Yield function for cooperative scheduling
        const yield_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0);
        const yield_func = c.LLVMAddFunction(module, "cursed_goroutine_yield", yield_type);
        
        const yield_entry = c.LLVMAppendBasicBlockInContext(context, yield_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, yield_entry);
        
        // For now, just return (full implementation would context switch)
        _ = c.LLVMBuildRetVoid(builder);
    }
    
    /// Generate string runtime functions
    fn generateStringRuntime(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        // String concatenation function
        const concat_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // result string
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // str1
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // str2
            },
            2, 0
        );
        const concat_func = c.LLVMAddFunction(module, "cursed_string_concat", concat_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const concat_entry = c.LLVMAppendBasicBlockInContext(context, concat_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, concat_entry);
        
        // Declare strlen and strcpy
        const strlen_type = c.LLVMFunctionType(
            c.LLVMInt64TypeInContext(context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
            1, 0
        );
        const strlen_func = c.LLVMAddFunction(module, "strlen", strlen_type);
        
        const strcpy_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            },
            2, 0
        );
        const strcpy_func = c.LLVMAddFunction(module, "strcpy", strcpy_type);
        
        const strcat_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            },
            2, 0
        );
        const strcat_func = c.LLVMAddFunction(module, "strcat", strcat_type);
        
        const str1_param = c.LLVMGetParam(concat_func, 0);
        const str2_param = c.LLVMGetParam(concat_func, 1);
        
        // Get string lengths
        const len1 = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(strlen_func)), strlen_func, &[_]c.LLVMValueRef{str1_param}, 1, "len1");
        const len2 = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(strlen_func)), strlen_func, &[_]c.LLVMValueRef{str2_param}, 1, "len2");
        
        // Calculate total length (including null terminator)
        const total_len = c.LLVMBuildAdd(builder, c.LLVMBuildAdd(builder, len1, len2, "temp_len"), c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 1, 0), "total_len");
        
        // Allocate memory for result
        const cursed_alloc = c.LLVMGetNamedFunction(module, "cursed_alloc");
        const result_ptr = c.LLVMBuildCall2(
            builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(cursed_alloc)),
            cursed_alloc,
            &[_]c.LLVMValueRef{
                total_len,
                c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 3, 0) // string type_id
            },
            2,
            "result_alloc"
        );
        
        // Copy first string
        _ = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(strcpy_func)), strcpy_func, &[_]c.LLVMValueRef{result_ptr, str1_param}, 2, "");
        
        // Concatenate second string
        _ = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(strcat_func)), strcat_func, &[_]c.LLVMValueRef{result_ptr, str2_param}, 2, "");
        
        _ = c.LLVMBuildRet(builder, result_ptr);
    }
    
    /// Generate error handling runtime
    fn generateErrorHandling(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        // Error type: { error_code, message, stack_trace }
        const error_fields = [_]c.LLVMTypeRef{
            c.LLVMInt32TypeInContext(context), // error_code
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // message
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // stack_trace
        };
        const error_type = c.LLVMStructTypeInContext(context, &error_fields, 3, 0);
        
        // Create error function
        const create_error_type = c.LLVMFunctionType(
            c.LLVMPointerType(error_type, 0),
            &[_]c.LLVMTypeRef{
                c.LLVMInt32TypeInContext(context), // error_code
                c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), // message
            },
            2, 0
        );
        const create_error = c.LLVMAddFunction(module, "cursed_error_create", create_error_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const error_entry = c.LLVMAppendBasicBlockInContext(context, create_error, "entry");
        c.LLVMPositionBuilderAtEnd(builder, error_entry);
        
        // Allocate error structure
        const error_size = c.LLVMSizeOf(error_type);
        const cursed_alloc = c.LLVMGetNamedFunction(module, "cursed_alloc");
        const error_ptr = c.LLVMBuildCall2(
            builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(cursed_alloc)),
            cursed_alloc,
            &[_]c.LLVMValueRef{
                error_size,
                c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 4, 0) // error type_id
            },
            2,
            "error_alloc"
        );
        
        const typed_error_ptr = c.LLVMBuildBitCast(builder, error_ptr, c.LLVMPointerType(error_type, 0), "typed_error");
        
        // Set error fields
        const code_param = c.LLVMGetParam(create_error, 0);
        const msg_param = c.LLVMGetParam(create_error, 1);
        
        const code_ptr = c.LLVMBuildStructGEP2(builder, error_type, typed_error_ptr, 0, "code_ptr");
        _ = c.LLVMBuildStore(builder, code_param, code_ptr);
        
        const msg_ptr = c.LLVMBuildStructGEP2(builder, error_type, typed_error_ptr, 1, "msg_ptr");
        _ = c.LLVMBuildStore(builder, msg_param, msg_ptr);
        
        // Stack trace (null for now)
        const trace_ptr = c.LLVMBuildStructGEP2(builder, error_type, typed_error_ptr, 2, "trace_ptr");
        _ = c.LLVMBuildStore(builder, c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)), trace_ptr);
        
        _ = c.LLVMBuildRet(builder, typed_error_ptr);
        
        // Panic function
        const panic_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)}, // message
            1, 0
        );
        const panic_func = c.LLVMAddFunction(module, "cursed_panic", panic_type);
        
        const panic_entry = c.LLVMAppendBasicBlockInContext(context, panic_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, panic_entry);
        
        // Print error message and exit
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
            1, 1
        );
        const printf_func = c.LLVMAddFunction(module, "printf", printf_type);
        
        const exit_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(context),
            &[_]c.LLVMTypeRef{c.LLVMInt32TypeInContext(context)},
            1, 0
        );
        const exit_func = c.LLVMAddFunction(module, "exit", exit_type);
        
        const msg_param_panic = c.LLVMGetParam(panic_func, 0);
        const panic_format = c.LLVMBuildGlobalStringPtr(builder, "CURSED PANIC: %s\n", "panic_fmt");
        
        _ = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)), printf_func, &[_]c.LLVMValueRef{panic_format, msg_param_panic}, 2, "");
        _ = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(exit_func)), exit_func, &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 1, 0)}, 1, "");
        
        _ = c.LLVMBuildUnreachable(builder);
    }
    
    /// Generate I/O runtime functions
    fn generateIORuntime(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        // Print function (vibez.spill)
        const print_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
            1, 0
        );
        const print_func = c.LLVMAddFunction(module, "cursed_print", print_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const print_entry = c.LLVMAppendBasicBlockInContext(context, print_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, print_entry);
        
        // Declare printf
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
            1, 1
        );
        const printf_func = c.LLVMAddFunction(module, "printf", printf_type);
        
        const msg_param = c.LLVMGetParam(print_func, 0);
        const format_str = c.LLVMBuildGlobalStringPtr(builder, "%s\n", "print_fmt");
        
        _ = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)), printf_func, &[_]c.LLVMValueRef{format_str, msg_param}, 2, "");
        _ = c.LLVMBuildRetVoid(builder);
        
        // Read line function
        const readline_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            null, 0, 0
        );
        const readline_func = c.LLVMAddFunction(module, "cursed_readline", readline_type);
        
        const readline_entry = c.LLVMAppendBasicBlockInContext(context, readline_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, readline_entry);
        
        // For now, return empty string (full implementation would read from stdin)
        const empty_str = c.LLVMBuildGlobalStringPtr(builder, "", "empty");
        _ = c.LLVMBuildRet(builder, empty_str);
    }
    
    /// Generate runtime initialization function
    pub fn generateRuntimeInit(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        const init_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0);
        const init_func = c.LLVMAddFunction(module, "cursed_runtime_init", init_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const init_entry = c.LLVMAppendBasicBlockInContext(context, init_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, init_entry);
        
        // Initialize garbage collector
        // Initialize thread pool
        // Initialize channel system
        // etc.
        
        _ = c.LLVMBuildRetVoid(builder);
    }
    
    /// Generate runtime cleanup function
    pub fn generateRuntimeCleanup(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        const cleanup_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(context), null, 0, 0);
        const cleanup_func = c.LLVMAddFunction(module, "cursed_runtime_cleanup", cleanup_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const cleanup_entry = c.LLVMAppendBasicBlockInContext(context, cleanup_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, cleanup_entry);
        
        // Final GC collection
        const gc_collect = c.LLVMGetNamedFunction(module, "cursed_gc_collect");
        if (gc_collect != null) {
            _ = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(gc_collect)), gc_collect, null, 0, "");
        }
        
        // Cleanup thread pool
        // Cleanup channels
        // etc.
        
        _ = c.LLVMBuildRetVoid(builder);
    }
};

test "runtime system initialization" {
    const allocator = std.testing.allocator;
    
    var runtime = RuntimeSystem.init(allocator);
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_runtime", context);
    defer c.LLVMDisposeModule(module);
    
    try runtime.generateRuntimeLibrary(context, module);
    
    // Verify functions were created
    try std.testing.expect(c.LLVMGetNamedFunction(module, "cursed_alloc") != null);
    try std.testing.expect(c.LLVMGetNamedFunction(module, "cursed_gc_mark") != null);
    try std.testing.expect(c.LLVMGetNamedFunction(module, "cursed_channel_create") != null);
}
