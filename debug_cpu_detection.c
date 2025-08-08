// Test LLVM CPU detection issue
#define __x86_64__ 1
#define __i386__ 0
#define _GNU_SOURCE 1

#include <stdio.h>

// Test if LLVM headers cause athlon-xp detection
#include <llvm-c/Core.h>
#include <llvm-c/Target.h>

int main() {
    printf("Testing LLVM CPU detection...\n");
    
    // Initialize LLVM
    LLVMInitializeCore(LLVMGetGlobalPassRegistry());
    LLVMInitializeNativeTarget();
    
    printf("LLVM initialization completed without athlon-xp error.\n");
    
    return 0;
}
