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

void* llvm_get_function_type(void* function) {
    if (!function) return NULL;
    return LLVMGlobalGetValueType((LLVMValueRef)function);
}

void* llvm_build_call2(void* builder, void* function_type, void* function, void** args, int arg_count, const char* name) {
    // Add null pointer validation to prevent segfaults
    if (!builder || !function_type || !function) return NULL;
    if (arg_count > 0 && !args) return NULL;
    
    return LLVMBuildCall2((LLVMBuilderRef)builder, (LLVMTypeRef)function_type, (LLVMValueRef)function, (LLVMValueRef*)args, arg_count, name);
}

int llvm_verify_module(void* module) {
    char* error_message = NULL;
    // Use ReturnStatusAction to capture errors instead of printing directly
    int result = LLVMVerifyModule((LLVMModuleRef)module, LLVMReturnStatusAction, &error_message);
    
    if (result != 0 && error_message) {
        // Only print if we have meaningful error content
        size_t len = strlen(error_message);
        if (len > 0) {
            // Trim whitespace to check if message is meaningful
            char* trimmed = error_message;
            while (*trimmed == ' ' || *trimmed == '\t' || *trimmed == '\n' || *trimmed == '\r') {
                trimmed++;
            }
            if (*trimmed != '\0') {
                printf("❌ CRITICAL LLVM verification error: %s\n", error_message);
            } else {
                printf("❌ CRITICAL LLVM verification failed with unknown error\n");
            }
        } else {
            printf("❌ CRITICAL LLVM verification failed with empty error message\n");
        }
        LLVMDisposeMessage(error_message);
    } else if (result != 0) {
        // Verification failed but no error message provided
        printf("❌ CRITICAL LLVM verification failed without error details\n");
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

// Additional LLVM wrapper functions needed for codegen
void* llvm_void_type(void* context) {
    return LLVMVoidTypeInContext((LLVMContextRef)context);
}

void* llvm_build_alloca(void* builder, void* type_ref, const char* name) {
    return LLVMBuildAlloca((LLVMBuilderRef)builder, (LLVMTypeRef)type_ref, name);
}

void* llvm_build_store(void* builder, void* value, void* alloca) {
    return LLVMBuildStore((LLVMBuilderRef)builder, (LLVMValueRef)value, (LLVMValueRef)alloca);
}

void* llvm_build_load2(void* builder, void* type_ref, void* alloca, const char* name) {
    return LLVMBuildLoad2((LLVMBuilderRef)builder, (LLVMTypeRef)type_ref, (LLVMValueRef)alloca, name);
}

void* llvm_get_param(void* function, unsigned int index) {
    return LLVMGetParam((LLVMValueRef)function, index);
}

void* llvm_get_insert_block(void* builder) {
    return LLVMGetInsertBlock((LLVMBuilderRef)builder);
}

void* llvm_get_basic_block_terminator(void* block) {
    return LLVMGetBasicBlockTerminator((LLVMBasicBlockRef)block);
}

void* llvm_build_ret_void(void* builder) {
    return LLVMBuildRetVoid((LLVMBuilderRef)builder);
}

void* llvm_int64_type(void* context) {
    return LLVMInt64TypeInContext((LLVMContextRef)context);
}

void* llvm_build_add(void* builder, void* left, void* right, const char* name) {
    return LLVMBuildAdd((LLVMBuilderRef)builder, (LLVMValueRef)left, (LLVMValueRef)right, name);
}

void* llvm_build_sub(void* builder, void* left, void* right, const char* name) {
    return LLVMBuildSub((LLVMBuilderRef)builder, (LLVMValueRef)left, (LLVMValueRef)right, name);
}

void* llvm_build_mul(void* builder, void* left, void* right, const char* name) {
    return LLVMBuildMul((LLVMBuilderRef)builder, (LLVMValueRef)left, (LLVMValueRef)right, name);
}

void* llvm_build_div(void* builder, void* left, void* right, const char* name) {
    return LLVMBuildSDiv((LLVMBuilderRef)builder, (LLVMValueRef)left, (LLVMValueRef)right, name);
}

void* llvm_type_of(void* value) {
    return LLVMTypeOf((LLVMValueRef)value);
}

void* llvm_get_allocated_type(void* alloca) {
    return LLVMGetAllocatedType((LLVMValueRef)alloca);
}

// Additional functions for full LLVM integration
void* llvm_get_basic_block_parent(void* block) {
    return LLVMGetBasicBlockParent((LLVMBasicBlockRef)block);
}

void* llvm_build_cond_br(void* builder, void* condition, void* then_block, void* else_block) {
    return LLVMBuildCondBr((LLVMBuilderRef)builder, (LLVMValueRef)condition, (LLVMBasicBlockRef)then_block, (LLVMBasicBlockRef)else_block);
}

void* llvm_build_br(void* builder, void* block) {
    return LLVMBuildBr((LLVMBuilderRef)builder, (LLVMBasicBlockRef)block);
}

void* llvm_int1_type(void* context) {
    return LLVMInt1TypeInContext((LLVMContextRef)context);
}

void* llvm_global_get_value_type(void* global) {
    return LLVMGlobalGetValueType((LLVMValueRef)global);
}

int llvm_get_type_kind(void* type_ref) {
    return LLVMGetTypeKind((LLVMTypeRef)type_ref);
}
