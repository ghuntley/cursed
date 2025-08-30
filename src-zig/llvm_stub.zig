// LLVM stub functions for cross-compilation
// These functions prevent linker errors but do not provide real LLVM functionality

export fn llvm_initialize_core() void {}
export fn llvm_create_context() ?*anyopaque { return null; }
export fn llvm_dispose_context(_: ?*anyopaque) void {}
export fn llvm_create_module(_: ?*anyopaque, _: [*c]const u8) ?*anyopaque { return null; }
export fn llvm_dispose_module(_: ?*anyopaque) void {}
export fn llvm_create_builder(_: ?*anyopaque) ?*anyopaque { return null; }
export fn llvm_dispose_builder(_: ?*anyopaque) void {}
export fn llvm_int32_type(_: ?*anyopaque) ?*anyopaque { return null; }
export fn llvm_int8_type(_: ?*anyopaque) ?*anyopaque { return null; }
export fn llvm_pointer_type(_: ?*anyopaque) ?*anyopaque { return null; }
export fn llvm_function_type(_: ?*anyopaque, _: [*]?*anyopaque, _: c_int, _: c_int) ?*anyopaque { return null; }
export fn llvm_add_function(_: ?*anyopaque, _: [*c]const u8, _: ?*anyopaque) ?*anyopaque { return null; }
export fn llvm_append_basic_block(_: ?*anyopaque, _: ?*anyopaque, _: [*c]const u8) ?*anyopaque { return null; }
export fn llvm_position_builder_at_end(_: ?*anyopaque, _: ?*anyopaque) void {}
export fn llvm_build_global_string_ptr(_: ?*anyopaque, _: [*c]const u8, _: [*c]const u8) ?*anyopaque { return null; }
export fn llvm_const_int(_: ?*anyopaque, _: c_ulonglong) ?*anyopaque { return null; }
export fn llvm_build_ret(_: ?*anyopaque, _: ?*anyopaque) ?*anyopaque { return null; }
export fn llvm_get_named_function(_: ?*anyopaque, _: [*c]const u8) ?*anyopaque { return null; }
export fn llvm_get_function_type(_: ?*anyopaque) ?*anyopaque { return null; }
export fn llvm_build_call2(_: ?*anyopaque, _: ?*anyopaque, _: ?*anyopaque, _: [*]?*anyopaque, _: c_int, _: [*c]const u8) ?*anyopaque { return null; }
export fn llvm_verify_module(_: ?*anyopaque) c_int { return 0; }
export fn llvm_print_module_to_string(_: ?*anyopaque) [*c]u8 { return null; }
export fn llvm_dispose_message(_: [*c]u8) void {}
export fn llvm_write_bitcode_to_file(_: ?*anyopaque, _: [*c]const u8) c_int { return 0; }
export fn llvm_build_add(_: ?*anyopaque, _: ?*anyopaque, _: ?*anyopaque, _: [*c]const u8) ?*anyopaque { return null; }
export fn llvm_build_sub(_: ?*anyopaque, _: ?*anyopaque, _: ?*anyopaque, _: [*c]const u8) ?*anyopaque { return null; }
export fn llvm_build_mul(_: ?*anyopaque, _: ?*anyopaque, _: ?*anyopaque, _: [*c]const u8) ?*anyopaque { return null; }
export fn llvm_build_div(_: ?*anyopaque, _: ?*anyopaque, _: ?*anyopaque, _: [*c]const u8) ?*anyopaque { return null; }
export fn LLVMAddInstructionCombiningPass(_: ?*anyopaque) void {}
export fn LLVMCreatePassManager() ?*anyopaque { return null; }
export fn LLVMAddReassociatePass(_: ?*anyopaque) void {}
export fn LLVMAddGVNPass(_: ?*anyopaque) void {}
export fn LLVMAddCFGSimplificationPass(_: ?*anyopaque) void {}
export fn LLVMAddPromoteMemoryToRegisterPass(_: ?*anyopaque) void {}
export fn LLVMAddDeadStoreEliminationPass(_: ?*anyopaque) void {}
export fn LLVMInitializeFunctionPassManager(_: ?*anyopaque) void {}
