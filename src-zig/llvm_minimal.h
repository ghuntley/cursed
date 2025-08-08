#ifndef LLVM_MINIMAL_H
#define LLVM_MINIMAL_H

// Minimal LLVM C API wrapper to avoid CPU detection issues
// This header includes only the essential LLVM types and functions

// Force explicit CPU target to avoid athlon-xp errors
#ifndef LLVM_DEFAULT_TARGET_TRIPLE
#define LLVM_DEFAULT_TARGET_TRIPLE "x86_64-unknown-linux-gnu"
#endif

// Override any problematic CPU target definitions
#ifdef __athlon__
#undef __athlon__
#endif
#ifdef __athlon_sse__
#undef __athlon_sse__
#endif

// Set explicit target before including LLVM headers
#define __x86_64__ 1
#define __x86_64 1

// Essential LLVM C headers only
#include <llvm-c/Core.h>
#include <llvm-c/Types.h>

// Basic initialization function  
static inline void llvm_minimal_init(void) {
    LLVMInitializeCore(LLVMGetGlobalPassRegistry());
    LLVMInitializeNativeTarget();
    LLVMInitializeNativeAsmPrinter();
}

#endif // LLVM_MINIMAL_H
