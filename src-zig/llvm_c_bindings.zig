// LLVM C bindings for CURSED compiler
// These are dummy types to allow compilation without requiring full LLVM linkage

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

// Basic LLVM functions (dummy implementations)
pub fn LLVMContextCreate() LLVMContextRef {
    return null;
}

pub fn LLVMContextDispose(context: LLVMContextRef) void {
    _ = context;
}

pub fn LLVMModuleCreateWithNameInContext(name: [*c]const u8, context: LLVMContextRef) LLVMModuleRef {
    _ = name;
    _ = context;
    return null;
}

pub fn LLVMDisposeModule(module: LLVMModuleRef) void {
    _ = module;
}

pub fn LLVMCreateBuilderInContext(context: LLVMContextRef) LLVMBuilderRef {
    _ = context;
    return null;
}

pub fn LLVMDisposeBuilder(builder: LLVMBuilderRef) void {
    _ = builder;
}

pub fn LLVMInt32TypeInContext(context: LLVMContextRef) LLVMTypeRef {
    _ = context;
    return null;
}

pub fn LLVMInt64TypeInContext(context: LLVMContextRef) LLVMTypeRef {
    _ = context;
    return null;
}

pub fn LLVMFunctionType(return_type: LLVMTypeRef, param_types: [*c]LLVMTypeRef, param_count: c_uint, is_var_arg: LLVMBool) LLVMTypeRef {
    _ = return_type;
    _ = param_types;
    _ = param_count;
    _ = is_var_arg;
    return null;
}

pub fn LLVMAddFunction(module: LLVMModuleRef, name: [*c]const u8, function_type: LLVMTypeRef) LLVMValueRef {
    _ = module;
    _ = name;
    _ = function_type;
    return null;
}

pub fn LLVMAppendBasicBlockInContext(context: LLVMContextRef, function: LLVMValueRef, name: [*c]const u8) LLVMBasicBlockRef {
    _ = context;
    _ = function;
    _ = name;
    return null;
}

pub fn LLVMPositionBuilderAtEnd(builder: LLVMBuilderRef, block: LLVMBasicBlockRef) void {
    _ = builder;
    _ = block;
}

pub fn LLVMBuildAlloca(builder: LLVMBuilderRef, type_ref: LLVMTypeRef, name: [*c]const u8) LLVMValueRef {
    _ = builder;
    _ = type_ref;
    _ = name;
    return null;
}

pub fn LLVMBuildStore(builder: LLVMBuilderRef, val: LLVMValueRef, ptr: LLVMValueRef) LLVMValueRef {
    _ = builder;
    _ = val;
    _ = ptr;
    return null;
}

pub fn LLVMBuildLoad2(builder: LLVMBuilderRef, type_ref: LLVMTypeRef, ptr: LLVMValueRef, name: [*c]const u8) LLVMValueRef {
    _ = builder;
    _ = type_ref;
    _ = ptr;
    _ = name;
    return null;
}

pub fn LLVMBuildRet(builder: LLVMBuilderRef, val: LLVMValueRef) LLVMValueRef {
    _ = builder;
    _ = val;
    return null;
}

pub fn LLVMConstInt(int_type: LLVMTypeRef, val: c_ulonglong, sign_extend: LLVMBool) LLVMValueRef {
    _ = int_type;
    _ = val;
    _ = sign_extend;
    return null;
}

pub fn LLVMBuildGlobalStringPtr(builder: LLVMBuilderRef, str: [*c]const u8, name: [*c]const u8) LLVMValueRef {
    _ = builder;
    _ = str;
    _ = name;
    return null;
}

pub fn LLVMPrintModuleToString(module: LLVMModuleRef) [*c]u8 {
    _ = module;
    return "; Empty LLVM IR\n";
}

pub fn LLVMPrintModuleToFile(module: LLVMModuleRef, filename: [*c]const u8, error_message: *[*c]u8) c_int {
    _ = module;
    _ = filename;
    _ = error_message;
    return 0;
}

pub fn LLVMDisposeMessage(message: [*c]u8) void {
    _ = message;
}
