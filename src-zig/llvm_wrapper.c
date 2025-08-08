// Explicitly set target before including LLVM headers to avoid athlon-xp detection
#define __x86_64__ 1
#define __i386__ 0
#define _GNU_SOURCE 1
#define LLVM_DEFAULT_TARGET_TRIPLE "x86_64-unknown-linux-gnu"
#define LLVM_HOST_TRIPLE "x86_64-unknown-linux-gnu"
#define TARGET_CPU "x86-64"

// Override any CPU auto-detection that might default to athlon-xp
#ifndef TARGET_CPU_DEFAULT
#define TARGET_CPU_DEFAULT TARGET_CPU_generic
#endif

#include <llvm-c/Core.h>
#include <llvm-c/ExecutionEngine.h>
#include <llvm-c/Target.h>
#include <llvm-c/Analysis.h>
#include <llvm-c/BitWriter.h>
#include <stdio.h>
#include <string.h>

// Wrapper functions for LLVM C API to avoid Zig @cImport issues

void llvm_initialize_core() {
    // Initialize core LLVM components
    LLVMInitializeNativeTarget();
    LLVMInitializeNativeAsmPrinter();
    LLVMInitializeNativeAsmParser();
}

void* llvm_create_context() {
    return LLVMContextCreate();
}

void llvm_dispose_context(void* context) {
    LLVMContextDispose((LLVMContextRef)context);
}

void* llvm_create_module(void* context, const char* name) {
    return LLVMModuleCreateWithNameInContext(name, (LLVMContextRef)context);
}

void llvm_dispose_module(void* module) {
    LLVMDisposeModule((LLVMModuleRef)module);
}

void* llvm_create_builder(void* context) {
    return LLVMCreateBuilderInContext((LLVMContextRef)context);
}

void llvm_dispose_builder(void* builder) {
    LLVMDisposeBuilder((LLVMBuilderRef)builder);
}

void* llvm_int32_type(void* context) {
    return LLVMInt32TypeInContext((LLVMContextRef)context);
}

void* llvm_int8_type(void* context) {
    return LLVMInt8TypeInContext((LLVMContextRef)context);
}

void* llvm_pointer_type(void* element_type) {
    return LLVMPointerType((LLVMTypeRef)element_type, 0);
}

void* llvm_function_type(void* return_type, void** param_types, int param_count, int is_var_arg) {
    return LLVMFunctionType((LLVMTypeRef)return_type, (LLVMTypeRef*)param_types, param_count, is_var_arg);
}

void* llvm_add_function(void* module, const char* name, void* function_type) {
    return LLVMAddFunction((LLVMModuleRef)module, name, (LLVMTypeRef)function_type);
}

void* llvm_append_basic_block(void* context, void* function, const char* name) {
    return LLVMAppendBasicBlockInContext((LLVMContextRef)context, (LLVMValueRef)function, name);
}

void llvm_position_builder_at_end(void* builder, void* block) {
    LLVMPositionBuilderAtEnd((LLVMBuilderRef)builder, (LLVMBasicBlockRef)block);
}

void* llvm_build_global_string_ptr(void* builder, const char* str, const char* name) {
    return LLVMBuildGlobalStringPtr((LLVMBuilderRef)builder, str, name);
}

void* llvm_const_int(void* int_type, unsigned long long value) {
    return LLVMConstInt((LLVMTypeRef)int_type, value, 0);
}

void* llvm_build_ret(void* builder, void* value) {
    return LLVMBuildRet((LLVMBuilderRef)builder, (LLVMValueRef)value);
}

void* llvm_get_named_function(void* module, const char* name) {
    return LLVMGetNamedFunction((LLVMModuleRef)module, name);
}

void* llvm_build_call2(void* builder, void* function_type, void* function, void** args, int arg_count, const char* name) {
    return LLVMBuildCall2((LLVMBuilderRef)builder, (LLVMTypeRef)function_type, (LLVMValueRef)function, (LLVMValueRef*)args, arg_count, name);
}

int llvm_verify_module(void* module) {
    char* error_message = NULL;
    int result = LLVMVerifyModule((LLVMModuleRef)module, LLVMPrintMessageAction, &error_message);
    if (error_message) {
        printf("LLVM verification error: %s\n", error_message);
        LLVMDisposeMessage(error_message);
    }
    return result;
}

char* llvm_print_module_to_string(void* module) {
    return LLVMPrintModuleToString((LLVMModuleRef)module);
}

void llvm_dispose_message(char* message) {
    LLVMDisposeMessage(message);
}

int llvm_write_bitcode_to_file(void* module, const char* path) {
    return LLVMWriteBitcodeToFile((LLVMModuleRef)module, path);
}
