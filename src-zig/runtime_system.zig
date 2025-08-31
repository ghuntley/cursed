const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
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
    
    pub fn init() RuntimeSystem {
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
    
    /// Generate comprehensive string runtime functions
    fn generateStringRuntime(self: *RuntimeSystem, context: c.LLVMContextRef, module: c.LLVMModuleRef) RuntimeError!void {
        _ = self;
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        // Common type declarations
        const i8_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const i8_type = c.LLVMInt8TypeInContext(context);
        const i32_type = c.LLVMInt32TypeInContext(context);
        const i64_type = c.LLVMInt64TypeInContext(context);
        
        // Declare C library functions
        const strlen_type = c.LLVMFunctionType(i64_type, &[_]c.LLVMTypeRef{i8_ptr_type}, 1, 0);
        const strlen_func = c.LLVMAddFunction(module, "strlen", strlen_type);
        
        const strcpy_type = c.LLVMFunctionType(i8_ptr_type, &[_]c.LLVMTypeRef{i8_ptr_type, i8_ptr_type}, 2, 0);
        const strcpy_func = c.LLVMAddFunction(module, "strcpy", strcpy_type);
        
        const strcat_type = c.LLVMFunctionType(i8_ptr_type, &[_]c.LLVMTypeRef{i8_ptr_type, i8_ptr_type}, 2, 0);
        const strcat_func = c.LLVMAddFunction(module, "strcat", strcat_type);
        
        // === 1. STRING CONCATENATION ===
        const concat_type = c.LLVMFunctionType(i8_ptr_type, &[_]c.LLVMTypeRef{i8_ptr_type, i8_ptr_type}, 2, 0);
        const concat_func = c.LLVMAddFunction(module, "cursed_string_concat", concat_type);
        
        const concat_entry = c.LLVMAppendBasicBlockInContext(context, concat_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, concat_entry);
        
        const str1_param = c.LLVMGetParam(concat_func, 0);
        const str2_param = c.LLVMGetParam(concat_func, 1);
        
        // Get string lengths
        const len1 = c.LLVMBuildCall2(builder, i64_type, strlen_func, &[_]c.LLVMValueRef{str1_param}, 1, "len1");
        const len2 = c.LLVMBuildCall2(builder, i64_type, strlen_func, &[_]c.LLVMValueRef{str2_param}, 1, "len2");
        
        // Calculate total length (including null terminator)
        const total_len = c.LLVMBuildAdd(builder, c.LLVMBuildAdd(builder, len1, len2, "temp_len"), c.LLVMConstInt(i64_type, 1, 0), "total_len");
        
        // Allocate memory for result
        const cursed_alloc = c.LLVMGetNamedFunction(module, "cursed_alloc");
        const result_ptr = c.LLVMBuildCall2(
            builder,
            i8_ptr_type,
            cursed_alloc,
            &[_]c.LLVMTypeRef{total_len, c.LLVMConstInt(i32_type, 3, 0)}, // string type_id = 3
            2,
            "result_alloc"
        );
        
        // Copy first string
        _ = c.LLVMBuildCall2(builder, i8_ptr_type, strcpy_func, &[_]c.LLVMValueRef{result_ptr, str1_param}, 2, "");
        
        // Concatenate second string
        _ = c.LLVMBuildCall2(builder, i8_ptr_type, strcat_func, &[_]c.LLVMValueRef{result_ptr, str2_param}, 2, "");
        
        _ = c.LLVMBuildRet(builder, result_ptr);
        
        // === 2. STRING LENGTH ===
        const length_type = c.LLVMFunctionType(i32_type, &[_]c.LLVMTypeRef{i8_ptr_type}, 1, 0);
        const length_func = c.LLVMAddFunction(module, "cursed_string_length", length_type);
        
        const length_entry = c.LLVMAppendBasicBlockInContext(context, length_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, length_entry);
        
        const str_param = c.LLVMGetParam(length_func, 0);
        const str_len = c.LLVMBuildCall2(builder, i64_type, strlen_func, &[_]c.LLVMValueRef{str_param}, 1, "str_len");
        const str_len_i32 = c.LLVMBuildTrunc(builder, str_len, i32_type, "str_len_i32");
        
        _ = c.LLVMBuildRet(builder, str_len_i32);
        
        // === 3. STRING CHARACTER ACCESS ===
        const char_at_type = c.LLVMFunctionType(i8_type, &[_]c.LLVMTypeRef{i8_ptr_type, i32_type}, 2, 0);
        const char_at_func = c.LLVMAddFunction(module, "runtime_string_char_at", char_at_type);
        
        const char_at_entry = c.LLVMAppendBasicBlockInContext(context, char_at_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, char_at_entry);
        
        const str_at_param = c.LLVMGetParam(char_at_func, 0);
        const index_param = c.LLVMGetParam(char_at_func, 1);
        
        // Bounds checking
        const str_at_len = c.LLVMBuildCall2(builder, i64_type, strlen_func, &[_]c.LLVMValueRef{str_at_param}, 1, "str_at_len");
        const str_at_len_i32 = c.LLVMBuildTrunc(builder, str_at_len, i32_type, "str_at_len_i32");
        
        const bounds_check = c.LLVMBuildICmp(builder, c.LLVMIntULT, index_param, str_at_len_i32, "bounds_check");
        
        const valid_bb = c.LLVMAppendBasicBlockInContext(context, char_at_func, "valid_index");
        const invalid_bb = c.LLVMAppendBasicBlockInContext(context, char_at_func, "invalid_index");
        
        _ = c.LLVMBuildCondBr(builder, bounds_check, valid_bb, invalid_bb);
        
        // Valid index: return character
        c.LLVMPositionBuilderAtEnd(builder, valid_bb);
        const char_ptr = c.LLVMBuildGEP2(builder, i8_type, str_at_param, &[_]c.LLVMValueRef{index_param}, 1, "char_ptr");
        const char_value = c.LLVMBuildLoad2(builder, i8_type, char_ptr, "char_value");
        _ = c.LLVMBuildRet(builder, char_value);
        
        // Invalid index: return null character
        c.LLVMPositionBuilderAtEnd(builder, invalid_bb);
        const null_char = c.LLVMConstInt(i8_type, 0, 0);
        _ = c.LLVMBuildRet(builder, null_char);
        
        // === 4. CHARACTER TO STRING CONVERSION ===
        const char_to_string_type = c.LLVMFunctionType(i8_ptr_type, &[_]c.LLVMTypeRef{i8_type}, 1, 0);
        const char_to_string_func = c.LLVMAddFunction(module, "runtime_char_to_string", char_to_string_type);
        
        const char_to_str_entry = c.LLVMAppendBasicBlockInContext(context, char_to_string_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, char_to_str_entry);
        
        const char_param = c.LLVMGetParam(char_to_string_func, 0);
        
        // Allocate 2 bytes (character + null terminator)
        const char_str_len = c.LLVMConstInt(i64_type, 2, 0);
        const char_str_ptr = c.LLVMBuildCall2(
            builder,
            i8_ptr_type,
            cursed_alloc,
            &[_]c.LLVMValueRef{char_str_len, c.LLVMConstInt(i32_type, 3, 0)}, // string type_id = 3
            2,
            "char_str_alloc"
        );
        
        // Store character and null terminator
        const first_byte_ptr = c.LLVMBuildGEP2(builder, i8_type, char_str_ptr, &[_]c.LLVMValueRef{c.LLVMConstInt(i32_type, 0, 0)}, 1, "first_byte");
        const second_byte_ptr = c.LLVMBuildGEP2(builder, i8_type, char_str_ptr, &[_]c.LLVMValueRef{c.LLVMConstInt(i32_type, 1, 0)}, 1, "second_byte");
        
        _ = c.LLVMBuildStore(builder, char_param, first_byte_ptr);
        _ = c.LLVMBuildStore(builder, c.LLVMConstInt(i8_type, 0, 0), second_byte_ptr);
        
        _ = c.LLVMBuildRet(builder, char_str_ptr);
        
        // === 5. STRING COMPARISON ===
        const str_cmp_type = c.LLVMFunctionType(i32_type, &[_]c.LLVMTypeRef{i8_ptr_type, i8_ptr_type}, 2, 0);
        const str_cmp_func = c.LLVMAddFunction(module, "cursed_string_compare", str_cmp_type);
        
        const strcmp_type = c.LLVMFunctionType(i32_type, &[_]c.LLVMTypeRef{i8_ptr_type, i8_ptr_type}, 2, 0);
        const strcmp_func = c.LLVMAddFunction(module, "strcmp", strcmp_type);
        
        const str_cmp_entry = c.LLVMAppendBasicBlockInContext(context, str_cmp_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, str_cmp_entry);
        
        const str1_cmp_param = c.LLVMGetParam(str_cmp_func, 0);
        const str2_cmp_param = c.LLVMGetParam(str_cmp_func, 1);
        
        const cmp_result = c.LLVMBuildCall2(builder, i32_type, strcmp_func, &[_]c.LLVMValueRef{str1_cmp_param, str2_cmp_param}, 2, "cmp_result");
        _ = c.LLVMBuildRet(builder, cmp_result);
        
        // === 6. STRING FORMATTING FOR INTERPOLATION ===
        const sprintf_type = c.LLVMFunctionType(i32_type, &[_]c.LLVMTypeRef{i8_ptr_type, i8_ptr_type}, 2, 1); // variadic
        const sprintf_func = c.LLVMAddFunction(module, "sprintf", sprintf_type);
        
        // Format integer to string
        const int_to_str_type = c.LLVMFunctionType(i8_ptr_type, &[_]c.LLVMTypeRef{i64_type}, 1, 0);
        const int_to_str_func = c.LLVMAddFunction(module, "cursed_int_to_string", int_to_str_type);
        
        const int_to_str_entry = c.LLVMAppendBasicBlockInContext(context, int_to_str_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, int_to_str_entry);
        
        const int_param = c.LLVMGetParam(int_to_str_func, 0);
        
        // Allocate buffer (max 21 bytes for i64)
        const buffer_len = c.LLVMConstInt(i64_type, 21, 0);
        const buffer_ptr = c.LLVMBuildCall2(
            builder,
            i8_ptr_type,
            cursed_alloc,
            &[_]c.LLVMValueRef{buffer_len, c.LLVMConstInt(i32_type, 3, 0)}, // string type_id = 3
            2,
            "int_str_buffer"
        );
        
        // Create format string "%lld"
        const fmt_str = c.LLVMConstStringInContext(context, "%lld", 4, 0);
        const global_fmt = c.LLVMAddGlobal(module, c.LLVMTypeOf(fmt_str), "int_fmt");
        c.LLVMSetInitializer(global_fmt, fmt_str);
        c.LLVMSetGlobalConstant(global_fmt, 1);
        
        const zero = c.LLVMConstInt(i32_type, 0, 0);
        const fmt_indices = [_]c.LLVMValueRef{zero, zero};
        const fmt_ptr = c.LLVMConstGEP2(c.LLVMTypeOf(fmt_str), global_fmt, &fmt_indices, 2);
        
        // Call sprintf
        _ = c.LLVMBuildCall2(
            builder,
            i32_type,
            sprintf_func,
            &[_]c.LLVMValueRef{buffer_ptr, fmt_ptr, int_param},
            3,
            ""
        );
        
        _ = c.LLVMBuildRet(builder, buffer_ptr);
        
        // Format float to string
        const float_to_str_type = c.LLVMFunctionType(i8_ptr_type, &[_]c.LLVMTypeRef{c.LLVMDoubleTypeInContext(context)}, 1, 0);
        const float_to_str_func = c.LLVMAddFunction(module, "cursed_float_to_string", float_to_str_type);
        
        const float_to_str_entry = c.LLVMAppendBasicBlockInContext(context, float_to_str_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, float_to_str_entry);
        
        const float_param = c.LLVMGetParam(float_to_str_func, 0);
        
        // Allocate buffer (max 32 bytes for double)
        const float_buffer_len = c.LLVMConstInt(i64_type, 32, 0);
        const float_buffer_ptr = c.LLVMBuildCall2(
            builder,
            i8_ptr_type,
            cursed_alloc,
            &[_]c.LLVMValueRef{float_buffer_len, c.LLVMConstInt(i32_type, 3, 0)},
            2,
            "float_str_buffer"
        );
        
        // Create format string "%g"
const float_fmt_str = c.LLVMConstStringInContext(context, "%g", 2, 0);
        const global_float_fmt = c.LLVMAddGlobal(module, c.LLVMTypeOf(float_fmt_str), "float_fmt");
        c.LLVMSetInitializer(global_float_fmt, float_fmt_str);
        c.LLVMSetGlobalConstant(global_float_fmt, 1);
        
        const float_fmt_indices = [_]c.LLVMValueRef{zero, zero};
        const float_fmt_ptr = c.LLVMConstGEP2(c.LLVMTypeOf(float_fmt_str), global_float_fmt, &float_fmt_indices, 2);
        
        // Call sprintf
        _ = c.LLVMBuildCall2(
            builder,
            i32_type,
            sprintf_func,
            &[_]c.LLVMValueRef{float_buffer_ptr, float_fmt_ptr, float_param},
            3,
            ""
        );
        
        _ = c.LLVMBuildRet(builder, float_buffer_ptr);
        
        // === 6. STRING SUBSTRING ===
        const substr_type = c.LLVMFunctionType(i8_ptr_type, &[_]c.LLVMTypeRef{i8_ptr_type, i32_type, i32_type}, 3, 0);
        const substr_func = c.LLVMAddFunction(module, "cursed_string_substring", substr_type);
        
        const substr_entry = c.LLVMAppendBasicBlockInContext(context, substr_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, substr_entry);
        
        const str_substr_param = c.LLVMGetParam(substr_func, 0);
        const start_param = c.LLVMGetParam(substr_func, 1);
        const len_param = c.LLVMGetParam(substr_func, 2);
        
        // Bounds checking
        const substr_str_len = c.LLVMBuildCall2(builder, i64_type, strlen_func, &[_]c.LLVMValueRef{str_substr_param}, 1, "substr_str_len");
        const substr_str_len_i32 = c.LLVMBuildTrunc(builder, substr_str_len, i32_type, "substr_str_len_i32");
        
        const start_valid = c.LLVMBuildICmp(builder, c.LLVMIntULT, start_param, substr_str_len_i32, "start_valid");
        const len_positive = c.LLVMBuildICmp(builder, c.LLVMIntSGT, len_param, c.LLVMConstInt(i32_type, 0, 0), "len_positive");
        const bounds_valid = c.LLVMBuildAnd(builder, start_valid, len_positive, "bounds_valid");
        
        const substr_valid_bb = c.LLVMAppendBasicBlockInContext(context, substr_func, "valid_substr");
        const substr_invalid_bb = c.LLVMAppendBasicBlockInContext(context, substr_func, "invalid_substr");
        
        _ = c.LLVMBuildCondBr(builder, bounds_valid, substr_valid_bb, substr_invalid_bb);
        
        // Valid substring
        c.LLVMPositionBuilderAtEnd(builder, substr_valid_bb);
        
        // Calculate actual length (clamp to string boundaries)
        const remaining_len = c.LLVMBuildSub(builder, substr_str_len_i32, start_param, "remaining_len");
        const actual_len = c.LLVMBuildSelect(builder, 
            c.LLVMBuildICmp(builder, c.LLVMIntULT, len_param, remaining_len, "len_fits"), 
            len_param, remaining_len, "actual_len");
        
        // Allocate memory for result (actual_len + 1 for null terminator)
        const result_len = c.LLVMBuildAdd(builder, actual_len, c.LLVMConstInt(i32_type, 1, 0), "result_len");
        const result_len_i64 = c.LLVMBuildZExt(builder, result_len, i64_type, "result_len_i64");
        
        const substr_result_ptr = c.LLVMBuildCall2(
            builder,
            i8_ptr_type,
            cursed_alloc,
            &[_]c.LLVMValueRef{result_len_i64, c.LLVMConstInt(i32_type, 3, 0)}, // string type_id = 3
            2,
            "substr_result_alloc"
        );
        
        // Copy substring using strncpy-like logic (manual copy with bounds)
        const memcpy_type = c.LLVMFunctionType(i8_ptr_type, &[_]c.LLVMTypeRef{i8_ptr_type, i8_ptr_type, i64_type}, 3, 0);
        const memcpy_func = c.LLVMAddFunction(module, "memcpy", memcpy_type);
        
        const src_ptr = c.LLVMBuildGEP2(builder, i8_type, str_substr_param, &[_]c.LLVMValueRef{start_param}, 1, "src_ptr");
        const actual_len_i64 = c.LLVMBuildZExt(builder, actual_len, i64_type, "actual_len_i64");
        
        _ = c.LLVMBuildCall2(builder, i8_ptr_type, memcpy_func, &[_]c.LLVMValueRef{substr_result_ptr, src_ptr, actual_len_i64}, 3, "");
        
        // Add null terminator
        const null_term_ptr = c.LLVMBuildGEP2(builder, i8_type, substr_result_ptr, &[_]c.LLVMValueRef{actual_len}, 1, "null_term_ptr");
        _ = c.LLVMBuildStore(builder, c.LLVMConstInt(i8_type, 0, 0), null_term_ptr);
        
        _ = c.LLVMBuildRet(builder, substr_result_ptr);
        
        // Invalid substring: return empty string
        c.LLVMPositionBuilderAtEnd(builder, substr_invalid_bb);
        
        const empty_str_len = c.LLVMConstInt(i64_type, 1, 0);
        const empty_str_ptr = c.LLVMBuildCall2(
            builder,
            i8_ptr_type,
            cursed_alloc,
            &[_]c.LLVMValueRef{empty_str_len, c.LLVMConstInt(i32_type, 3, 0)}, // string type_id = 3
            2,
            "empty_str_alloc"
        );
        
        _ = c.LLVMBuildStore(builder, c.LLVMConstInt(i8_type, 0, 0), empty_str_ptr);
        _ = c.LLVMBuildRet(builder, empty_str_ptr);
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
        
        // Read line function - declare external function
        const readline_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            null, 0, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_read_line", readline_type);
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
