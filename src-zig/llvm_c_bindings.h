#ifndef LLVM_C_BINDINGS_H
#define LLVM_C_BINDINGS_H

// LLVM C API headers with proper CPU target configuration
// This header prevents "athlon-xp" CPU detection issues

// Define target to avoid CPU detection problems
#ifndef LLVM_DEFAULT_TARGET_TRIPLE
#define LLVM_DEFAULT_TARGET_TRIPLE "x86_64-unknown-linux-gnu"
#endif

#ifndef LLVM_HOST_TARGET
#define LLVM_HOST_TARGET "x86-64"
#endif

// Override any CPU-specific definitions that cause athlon-xp errors
#ifdef __i386__
#undef __i386__
#endif
#ifdef __i686__
#undef __i686__
#endif

// Force x86-64 target to avoid ancient CPU issues
#define LLVM_TARGET_CPU "x86-64"
#define LLVM_TARGET_ARCH "x86_64"

// Standard LLVM C headers
#include <llvm-c/Core.h>
#include <llvm-c/Types.h>
#include <llvm-c/Target.h>
#include <llvm-c/TargetMachine.h>
#include <llvm-c/ExecutionEngine.h>
#include <llvm-c/Analysis.h>
#include <llvm-c/BitWriter.h>
#include <llvm-c/BitReader.h>
#include <llvm-c/Transforms/Scalar.h>
#include <llvm-c/Transforms/Utils.h>
#include <llvm-c/Transforms/Vectorize.h>
#include <llvm-c/Transforms/IPO.h>
#include <llvm-c/LLJIT.h>
#include <llvm-c/OrcEE.h>

// Additional headers for debugging and optimization
#ifdef LLVM_DEBUG_INFO_ENABLED
#include <llvm-c/DebugInfo.h>
#endif

// Utility macros for common LLVM operations
#define LLVM_BOOL_TRUE 1
#define LLVM_BOOL_FALSE 0

// Function to initialize LLVM with proper target configuration
static inline void llvm_initialize_with_target(void) {
    LLVMInitializeCore(LLVMGetGlobalPassRegistry());
    LLVMInitializeTransformUtils(LLVMGetGlobalPassRegistry());
    LLVMInitializeScalarOpts(LLVMGetGlobalPassRegistry());
    LLVMInitializeObjCARCOpts(LLVMGetGlobalPassRegistry());
    LLVMInitializeVectorization(LLVMGetGlobalPassRegistry());
    LLVMInitializeInstCombine(LLVMGetGlobalPassRegistry());
    LLVMInitializeIPO(LLVMGetGlobalPassRegistry());
    LLVMInitializeInstrumentation(LLVMGetGlobalPassRegistry());
    LLVMInitializeAnalysis(LLVMGetGlobalPassRegistry());
    LLVMInitializeIPA(LLVMGetGlobalPassRegistry());
    LLVMInitializeCodeGen(LLVMGetGlobalPassRegistry());
    LLVMInitializeTarget(LLVMGetGlobalPassRegistry());
    
    // Initialize native target
    LLVMInitializeNativeTarget();
    LLVMInitializeNativeAsmPrinter();
    LLVMInitializeNativeAsmParser();
}

#endif // LLVM_C_BINDINGS_H
