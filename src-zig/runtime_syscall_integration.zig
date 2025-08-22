const std = @import("std");
const syscall_interface = @import("syscall_interface.zig");
const runtime_system = @import("runtime_system.zig");
const concurrency_bridge = @import("concurrency_runtime_bridge_complete.zig");
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

/// CURSED Runtime Syscall Integration
/// 
/// This module integrates the real syscall interface with the CURSED runtime system,
/// providing seamless access to system calls from CURSED programs through LLVM IR generation.

pub const RuntimeSyscallIntegration = struct {
    runtime: *runtime_system.RuntimeSystem,
    allocator: std.mem.Allocator,
    
    const Self = @This();
    
    pub fn init(allocator: std.mem.Allocator, runtime: *runtime_system.RuntimeSystem) Self {
        return Self{
            .runtime = runtime,
            .allocator = allocator,
        };
    }
    
    /// Generate syscall runtime functions in LLVM IR
    pub fn generateSyscallRuntime(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        try self.generateFileSystemRuntime(context, module);
        try self.generateNetworkingRuntime(context, module);
        try self.generateProcessRuntime(context, module);
        try self.generateEnvironmentRuntime(context, module);
        try self.generateUtilityRuntime(context, module);
    }
    
    /// Generate file system syscall runtime
    fn generateFileSystemRuntime(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        // Declare external syscall functions for file operations
        const i32_type = c.LLVMInt32TypeInContext(context);
        const i64_type = c.LLVMInt64TypeInContext(context);
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const cstring_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        
        // cursed_file_open(path: [*:0]u8, mode: u32) -> i32
        const file_open_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ cstring_type, i32_type },
            2, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_file_open", file_open_type);
        
        // cursed_file_close(handle: u32) -> i32
        const file_close_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{i32_type},
            1, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_file_close", file_close_type);
        
        // cursed_file_read(handle: u32, buffer: [*]u8, size: usize) -> i64
        const file_read_type = c.LLVMFunctionType(
            i64_type,
            &[_]c.LLVMTypeRef{ i32_type, ptr_type, i64_type },
            3, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_file_read", file_read_type);
        
        // cursed_file_write(handle: u32, data: [*]const u8, size: usize) -> i64
        const file_write_type = c.LLVMFunctionType(
            i64_type,
            &[_]c.LLVMTypeRef{ i32_type, ptr_type, i64_type },
            3, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_file_write", file_write_type);
        
        // cursed_file_delete(path: [*:0]u8) -> i32
        const file_delete_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{cstring_type},
            1, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_file_delete", file_delete_type);
        
        // cursed_dir_create(path: [*:0]u8, mode: u32) -> i32
        const dir_create_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ cstring_type, i32_type },
            2, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_dir_create", dir_create_type);
        
        // cursed_dir_remove(path: [*:0]u8) -> i32
        const dir_remove_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{cstring_type},
            1, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_dir_remove", dir_remove_type);
        
        // FileStats structure for cursed_file_stat
        const file_stats_fields = [_]c.LLVMTypeRef{
            i64_type, // size
            i32_type, // mode
            i64_type, // created_time
            i64_type, // modified_time
            i64_type, // accessed_time
            c.LLVMInt1TypeInContext(context), // is_dir
            c.LLVMInt1TypeInContext(context), // is_file
            c.LLVMInt1TypeInContext(context), // is_symlink
        };
        const file_stats_type = c.LLVMStructTypeInContext(context, &file_stats_fields, 8, 0);
        
        // cursed_file_stat(path: [*:0]u8, stat: *FileStats) -> i32
        const file_stat_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ cstring_type, c.LLVMPointerType(file_stats_type, 0) },
            2, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_file_stat", file_stat_type);
        
        // Generate helper functions for CURSED file operations
        try self.generateCursedFileHelpers(context, module);
    }
    
    /// Generate networking syscall runtime
    fn generateNetworkingRuntime(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        const i32_type = c.LLVMInt32TypeInContext(context);
        const i64_type = c.LLVMInt64TypeInContext(context);
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const cstring_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const i16_type = c.LLVMInt16TypeInContext(context);
        
        // cursed_socket_create(domain: u32, type: u32, protocol: u32) -> i32
        const socket_create_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ i32_type, i32_type, i32_type },
            3, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_socket_create", socket_create_type);
        
        // cursed_socket_close(socket_id: u32) -> i32
        const socket_close_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{i32_type},
            1, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_socket_close", socket_close_type);
        
        // cursed_socket_bind(socket_id: u32, addr: [*:0]u8, port: u16) -> i32
        const socket_bind_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ i32_type, cstring_type, i16_type },
            3, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_socket_bind", socket_bind_type);
        
        // cursed_socket_listen(socket_id: u32, backlog: u32) -> i32
        const socket_listen_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ i32_type, i32_type },
            2, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_socket_listen", socket_listen_type);
        
        // HTTP runtime functions
        // cursed_http_get(url_ptr: [*]u8, url_len: usize) -> [*]u8
        const http_get_type = c.LLVMFunctionType(
            ptr_type,
            &[_]c.LLVMTypeRef{ ptr_type, i64_type },
            2, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_http_get", http_get_type);
        
        // cursed_http_post(url_ptr: [*]u8, url_len: usize, body_ptr: [*]u8, body_len: usize) -> [*]u8
        const http_post_type = c.LLVMFunctionType(
            ptr_type,
            &[_]c.LLVMTypeRef{ ptr_type, i64_type, ptr_type, i64_type },
            4, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_http_post", http_post_type);
        
        // cursed_tcp_connect(host_ptr: [*]u8, host_len: usize, port: u16) -> i32
        const tcp_connect_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ ptr_type, i64_type, i16_type },
            3, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_tcp_connect", tcp_connect_type);
        
        // cursed_socket_accept(socket_id: u32) -> i32
        const socket_accept_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{i32_type},
            1, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_socket_accept", socket_accept_type);
        
        // cursed_socket_connect(socket_id: u32, addr: [*:0]u8, port: u16) -> i32
        const socket_connect_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ i32_type, cstring_type, i16_type },
            3, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_socket_connect", socket_connect_type);
        
        // cursed_socket_send(socket_id: u32, data: [*]u8, size: usize, flags: u32) -> i64
        const socket_send_type = c.LLVMFunctionType(
            i64_type,
            &[_]c.LLVMTypeRef{ i32_type, ptr_type, i64_type, i32_type },
            4, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_socket_send", socket_send_type);
        
        // cursed_socket_recv(socket_id: u32, buffer: [*]u8, size: usize, flags: u32) -> i64
        const socket_recv_type = c.LLVMFunctionType(
            i64_type,
            &[_]c.LLVMTypeRef{ i32_type, ptr_type, i64_type, i32_type },
            4, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_socket_recv", socket_recv_type);
        
        // Generate helper functions for CURSED networking operations
        try self.generateCursedNetworkHelpers(context, module);
    }
    
    /// Generate process management syscall runtime
    fn generateProcessRuntime(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        const i32_type = c.LLVMInt32TypeInContext(context);
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const cstring_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const argv_type = c.LLVMPointerType(cstring_type, 0);
        const size_type = c.LLVMInt64TypeInContext(context);
        
        // cursed_process_spawn(command: [*:0]u8, args: [*][*:0]u8, args_count: usize) -> i32
        const process_spawn_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ cstring_type, argv_type, size_type },
            3, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_process_spawn", process_spawn_type);
        
        // cursed_process_wait(process_id: u32) -> i32
        const process_wait_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{i32_type},
            1, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_process_wait", process_wait_type);
        
        // cursed_process_kill(process_id: u32, signal: i32) -> i32
        const process_kill_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ i32_type, i32_type },
            2, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_process_kill", process_kill_type);
    }
    
    /// Generate environment variable syscall runtime
    fn generateEnvironmentRuntime(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        const i32_type = c.LLVMInt32TypeInContext(context);
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const cstring_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const size_type = c.LLVMInt64TypeInContext(context);
        
        // cursed_env_get(name: [*:0]u8, buffer: [*]u8, buffer_size: usize) -> i32
        const env_get_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ cstring_type, ptr_type, size_type },
            3, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_env_get", env_get_type);
        
        // cursed_env_set(name: [*:0]u8, value: [*:0]u8) -> i32
        const env_set_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ cstring_type, cstring_type },
            2, 0
        );
        _ = c.LLVMAddFunction(module, "cursed_env_set", env_set_type);
    }
    
    /// Generate utility runtime functions
    fn generateUtilityRuntime(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        // Generate CURSED string <-> C string conversion functions
        try self.generateStringConversionRuntime(context, module);
        
        // Generate memory management for syscall operations
        try self.generateSyscallMemoryRuntime(context, module);
        
        // Generate initialization and cleanup functions
        try self.generateSyscallInitRuntime(context, module);
    }
    
    /// Generate CURSED file operation helper functions
    fn generateCursedFileHelpers(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const void_type = c.LLVMVoidTypeInContext(context);
        const i32_type = c.LLVMInt32TypeInContext(context);
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        
        // cursed_file_read_all(path: *u8) -> *u8 (CURSED string)
        const read_all_type = c.LLVMFunctionType(
            ptr_type,
            &[_]c.LLVMTypeRef{ptr_type},
            1, 0
        );
        const read_all_func = c.LLVMAddFunction(module, "cursed_file_read_all", read_all_type);
        
        const entry_block = c.LLVMAppendBasicBlockInContext(context, read_all_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, entry_block);
        
        const path_param = c.LLVMGetParam(read_all_func, 0);
        
        // Convert CURSED string to C string
        const to_cstring_func = c.LLVMGetNamedFunction(module, "cursed_string_to_cstring");
        const c_path = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(to_cstring_func)), to_cstring_func, &[_]c.LLVMValueRef{path_param}, 1, "c_path");
        
        // Get file stats to determine size
        const file_stats_fields = [_]c.LLVMTypeRef{
            c.LLVMInt64TypeInContext(context), // size
            i32_type, // mode
            c.LLVMInt64TypeInContext(context), // created_time
            c.LLVMInt64TypeInContext(context), // modified_time
            c.LLVMInt64TypeInContext(context), // accessed_time
            c.LLVMInt1TypeInContext(context), // is_dir
            c.LLVMInt1TypeInContext(context), // is_file
            c.LLVMInt1TypeInContext(context), // is_symlink
        };
        const file_stats_type = c.LLVMStructTypeInContext(context, &file_stats_fields, 8, 0);
        
        // Allocate space for file stats
        const stats_alloca = c.LLVMBuildAlloca(builder, file_stats_type, "stats");
        
        // Call cursed_file_stat
        const file_stat_func = c.LLVMGetNamedFunction(module, "cursed_file_stat");
        const stat_result = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(file_stat_func)), file_stat_func, &[_]c.LLVMValueRef{ c_path, stats_alloca }, 2, "stat_result");
        
        // Check if stat succeeded
        const stat_success = c.LLVMBuildICmp(builder, c.LLVMIntEQ, stat_result, c.LLVMConstInt(i32_type, 0, 0), "stat_success");
        
        const success_block = c.LLVMAppendBasicBlockInContext(context, read_all_func, "success");
        const error_block = c.LLVMAppendBasicBlockInContext(context, read_all_func, "error");
        
        _ = c.LLVMBuildCondBr(builder, stat_success, success_block, error_block);
        
        // Success block: open file and read content
        c.LLVMPositionBuilderAtEnd(builder, success_block);
        
        // Get file size from stats
        const size_ptr = c.LLVMBuildStructGEP2(builder, file_stats_type, stats_alloca, 0, "size_ptr");
        const file_size = c.LLVMBuildLoad2(builder, c.LLVMInt64TypeInContext(context), size_ptr, "file_size");
        
        // Open file for reading (mode 0)
        const file_open_func = c.LLVMGetNamedFunction(module, "cursed_file_open");
        const handle = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(file_open_func)), file_open_func, &[_]c.LLVMValueRef{ c_path, c.LLVMConstInt(i32_type, 0, 0) }, 2, "handle");
        
        // Check if open succeeded
        const open_success = c.LLVMBuildICmp(builder, c.LLVMIntSGE, handle, c.LLVMConstInt(i32_type, 0, 0), "open_success");
        
        const read_block = c.LLVMAppendBasicBlockInContext(context, read_all_func, "read");
        _ = c.LLVMBuildCondBr(builder, open_success, read_block, error_block);
        
        // Read block: allocate buffer and read file
        c.LLVMPositionBuilderAtEnd(builder, read_block);
        
        // Allocate buffer for file content
        const cursed_alloc = c.LLVMGetNamedFunction(module, "cursed_alloc");
        const buffer_size = c.LLVMBuildAdd(builder, file_size, c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 1, 0), "buffer_size"); // +1 for null terminator
        const buffer = c.LLVMBuildCall2(
            builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(cursed_alloc)),
            cursed_alloc,
            &[_]c.LLVMValueRef{
                buffer_size,
                c.LLVMConstInt(i32_type, 3, 0) // string type_id
            },
            2,
            "buffer"
        );
        
        // Read file content
        const file_read_func = c.LLVMGetNamedFunction(module, "cursed_file_read");
        const bytes_read = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(file_read_func)), file_read_func, &[_]c.LLVMValueRef{ handle, buffer, file_size }, 3, "bytes_read");
        
        // Close file
        const file_close_func = c.LLVMGetNamedFunction(module, "cursed_file_close");
        _ = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(file_close_func)), file_close_func, &[_]c.LLVMValueRef{handle}, 1, "");
        
        // Null-terminate the buffer
        const null_pos = c.LLVMBuildGEP2(builder, c.LLVMInt8TypeInContext(context), buffer, &[_]c.LLVMValueRef{bytes_read}, 1, "null_pos");
        _ = c.LLVMBuildStore(builder, c.LLVMConstInt(c.LLVMInt8TypeInContext(context), 0, 0), null_pos);
        
        // Convert buffer to CURSED string
        const from_cstring_func = c.LLVMGetNamedFunction(module, "cursed_string_from_cstring");
        const result_string = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(from_cstring_func)), from_cstring_func, &[_]c.LLVMValueRef{buffer}, 1, "result_string");
        
        _ = c.LLVMBuildRet(builder, result_string);
        
        // Error block: return empty string
        c.LLVMPositionBuilderAtEnd(builder, error_block);
        const empty_string = c.LLVMBuildGlobalStringPtr(builder, "", "empty");
        const empty_cursed_string = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(from_cstring_func)), from_cstring_func, &[_]c.LLVMValueRef{empty_string}, 1, "empty_cursed");
        _ = c.LLVMBuildRet(builder, empty_cursed_string);
    }
    
    /// Generate CURSED networking helper functions
    fn generateCursedNetworkHelpers(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        const i32_type = c.LLVMInt32TypeInContext(context);
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        
        // cursed_tcp_connect(host: *u8, port: i32) -> i32 (socket_id)
        const tcp_connect_type = c.LLVMFunctionType(
            i32_type,
            &[_]c.LLVMTypeRef{ ptr_type, i32_type },
            2, 0
        );
        const tcp_connect_func = c.LLVMAddFunction(module, "cursed_tcp_connect", tcp_connect_type);
        
        const entry_block = c.LLVMAppendBasicBlockInContext(context, tcp_connect_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, entry_block);
        
        const host_param = c.LLVMGetParam(tcp_connect_func, 0);
        const port_param = c.LLVMGetParam(tcp_connect_func, 1);
        
        // Convert CURSED string to C string
        const to_cstring_func = c.LLVMGetNamedFunction(module, "cursed_string_to_cstring");
        const c_host = c.LLVMBuildCall2(builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(to_cstring_func)), to_cstring_func, &[_]c.LLVMValueRef{host_param}, 1, "c_host");
        
        // Create socket (AF_INET=2, SOCK_STREAM=1, IPPROTO_TCP=6)
        const socket_create_func = c.LLVMGetNamedFunction(module, "cursed_socket_create");
        const socket_id = c.LLVMBuildCall2(
            builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(socket_create_func)),
            socket_create_func,
            &[_]c.LLVMValueRef{
                c.LLVMConstInt(i32_type, 2, 0), // AF_INET
                c.LLVMConstInt(i32_type, 1, 0), // SOCK_STREAM
                c.LLVMConstInt(i32_type, 6, 0)  // IPPROTO_TCP
            },
            3,
            "socket_id"
        );
        
        // Check if socket creation succeeded
        const socket_success = c.LLVMBuildICmp(builder, c.LLVMIntSGE, socket_id, c.LLVMConstInt(i32_type, 0, 0), "socket_success");
        
        const connect_block = c.LLVMAppendBasicBlockInContext(context, tcp_connect_func, "connect");
        const error_block = c.LLVMAppendBasicBlockInContext(context, tcp_connect_func, "error");
        
        _ = c.LLVMBuildCondBr(builder, socket_success, connect_block, error_block);
        
        // Connect block: connect to host
        c.LLVMPositionBuilderAtEnd(builder, connect_block);
        
        const socket_connect_func = c.LLVMGetNamedFunction(module, "cursed_socket_connect");
        const port_16 = c.LLVMBuildTrunc(builder, port_param, c.LLVMInt16TypeInContext(context), "port_16");
        const connect_result = c.LLVMBuildCall2(
            builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(socket_connect_func)),
            socket_connect_func,
            &[_]c.LLVMValueRef{ socket_id, c_host, port_16 },
            3,
            "connect_result"
        );
        
        // Check if connect succeeded
        const connect_success = c.LLVMBuildICmp(builder, c.LLVMIntEQ, connect_result, c.LLVMConstInt(i32_type, 0, 0), "connect_success");
        
        const success_block = c.LLVMAppendBasicBlockInContext(context, tcp_connect_func, "success");
        _ = c.LLVMBuildCondBr(builder, connect_success, success_block, error_block);
        
        // Success block: return socket_id
        c.LLVMPositionBuilderAtEnd(builder, success_block);
        _ = c.LLVMBuildRet(builder, socket_id);
        
        // Error block: return -1
        c.LLVMPositionBuilderAtEnd(builder, error_block);
        _ = c.LLVMBuildRet(builder, c.LLVMConstInt(i32_type, @bitCast(@as(i32, -1)), 1));
    }
    
    /// Generate string conversion runtime functions
    fn generateStringConversionRuntime(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        
        // cursed_string_to_cstring(cursed_str: *u8) -> [*:0]u8
        const to_cstring_type = c.LLVMFunctionType(
            ptr_type,
            &[_]c.LLVMTypeRef{ptr_type},
            1, 0
        );
        const to_cstring_func = c.LLVMAddFunction(module, "cursed_string_to_cstring", to_cstring_type);
        
        // cursed_string_from_cstring(c_str: [*:0]u8) -> *u8
        const from_cstring_type = c.LLVMFunctionType(
            ptr_type,
            &[_]c.LLVMTypeRef{ptr_type},
            1, 0
        );
        const from_cstring_func = c.LLVMAddFunction(module, "cursed_string_from_cstring", from_cstring_type);
        
        // For now, these are placeholders - actual implementation would depend on
        // CURSED string representation format
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        // Simple implementation: just return the input (assuming compatible formats)
        const to_entry = c.LLVMAppendBasicBlockInContext(context, to_cstring_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, to_entry);
        const to_param = c.LLVMGetParam(to_cstring_func, 0);
        _ = c.LLVMBuildRet(builder, to_param);
        
        const from_entry = c.LLVMAppendBasicBlockInContext(context, from_cstring_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, from_entry);
        const from_param = c.LLVMGetParam(from_cstring_func, 0);
        _ = c.LLVMBuildRet(builder, from_param);
    }
    
    /// Generate memory management for syscall operations
    fn generateSyscallMemoryRuntime(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        const void_type = c.LLVMVoidTypeInContext(context);
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const size_type = c.LLVMInt64TypeInContext(context);
        
        // cursed_syscall_alloc(size: usize) -> *u8
        const syscall_alloc_type = c.LLVMFunctionType(
            ptr_type,
            &[_]c.LLVMTypeRef{size_type},
            1, 0
        );
        const syscall_alloc_func = c.LLVMAddFunction(module, "cursed_syscall_alloc", syscall_alloc_type);
        
        // cursed_syscall_free(ptr: *u8) -> void
        const syscall_free_type = c.LLVMFunctionType(
            void_type,
            &[_]c.LLVMTypeRef{ptr_type},
            1, 0
        );
        const syscall_free_func = c.LLVMAddFunction(module, "cursed_syscall_free", syscall_free_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        // Implement syscall_alloc using cursed_alloc
        const alloc_entry = c.LLVMAppendBasicBlockInContext(context, syscall_alloc_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, alloc_entry);
        
        const size_param = c.LLVMGetParam(syscall_alloc_func, 0);
        const cursed_alloc = c.LLVMGetNamedFunction(module, "cursed_alloc");
        const alloc_result = c.LLVMBuildCall2(
            builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(cursed_alloc)),
            cursed_alloc,
            &[_]c.LLVMValueRef{
                size_param,
                c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 5, 0) // syscall buffer type_id
            },
            2,
            "alloc_result"
        );
        _ = c.LLVMBuildRet(builder, alloc_result);
        
        // Implement syscall_free (placeholder - would integrate with GC)
        const free_entry = c.LLVMAppendBasicBlockInContext(context, syscall_free_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, free_entry);
        _ = c.LLVMBuildRetVoid(builder);
    }
    
    /// Generate syscall initialization runtime
    fn generateSyscallInitRuntime(self: *Self, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
        _ = self;
        
        const void_type = c.LLVMVoidTypeInContext(context);
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        
        // cursed_syscall_runtime_init(allocator: *anyopaque) -> void
        const init_type = c.LLVMFunctionType(
            void_type,
            &[_]c.LLVMTypeRef{ptr_type},
            1, 0
        );
        const init_func = c.LLVMAddFunction(module, "cursed_syscall_runtime_init", init_type);
        
        // cursed_syscall_runtime_cleanup() -> void
        const cleanup_type = c.LLVMFunctionType(void_type, null, 0, 0);
        const cleanup_func = c.LLVMAddFunction(module, "cursed_syscall_runtime_cleanup", cleanup_type);
        
        const builder = c.LLVMCreateBuilderInContext(context);
        defer c.LLVMDisposeBuilder(builder);
        
        // Implement init function
        const init_entry = c.LLVMAppendBasicBlockInContext(context, init_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, init_entry);
        
        const allocator_param = c.LLVMGetParam(init_func, 0);
        
        // Call external syscall init functions
        const cursed_syscall_init = c.LLVMAddFunction(module, "cursed_syscall_init", c.LLVMFunctionType(void_type, &[_]c.LLVMTypeRef{ptr_type}, 1, 0));
        _ = c.LLVMBuildCall2(builder, void_type, cursed_syscall_init, &[_]c.LLVMValueRef{allocator_param}, 1, "");
        
        const cursed_syscall_init_registries = c.LLVMAddFunction(module, "cursed_syscall_init_registries", c.LLVMFunctionType(void_type, null, 0, 0));
        _ = c.LLVMBuildCall2(builder, void_type, cursed_syscall_init_registries, null, 0, "");
        
        _ = c.LLVMBuildRetVoid(builder);
        
        // Implement cleanup function
        const cleanup_entry = c.LLVMAppendBasicBlockInContext(context, cleanup_func, "entry");
        c.LLVMPositionBuilderAtEnd(builder, cleanup_entry);
        
        const cursed_syscall_cleanup_registries = c.LLVMAddFunction(module, "cursed_syscall_cleanup_registries", c.LLVMFunctionType(void_type, null, 0, 0));
        _ = c.LLVMBuildCall2(builder, void_type, cursed_syscall_cleanup_registries, null, 0, "");
        
        const cursed_syscall_cleanup = c.LLVMAddFunction(module, "cursed_syscall_cleanup", c.LLVMFunctionType(void_type, null, 0, 0));
        _ = c.LLVMBuildCall2(builder, void_type, cursed_syscall_cleanup, null, 0, "");
        
        _ = c.LLVMBuildRetVoid(builder);
    }
};

// Export C-compatible initialization functions for the runtime
export fn cursed_runtime_init_with_syscalls(allocator: *anyopaque) void {
    // Initialize the concurrency runtime
    concurrency_bridge.cursed_runtime_init();
    
    // Initialize the syscall interface
    syscall_interface.cursed_syscall_init(allocator);
    syscall_interface.cursed_syscall_init_registries();
    
    std.debug.print("[RUNTIME] CURSED runtime with syscalls initialized\n", .{});
}

export fn cursed_runtime_cleanup_with_syscalls() void {
    // Cleanup syscall interface
    syscall_interface.cursed_syscall_cleanup_registries();
    syscall_interface.cursed_syscall_cleanup();
    
    // Cleanup concurrency runtime
    concurrency_bridge.cursed_runtime_cleanup();
    
    std.debug.print("[RUNTIME] CURSED runtime with syscalls cleaned up\n", .{});
}
