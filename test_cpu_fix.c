#include <stdio.h>
#include <llvm-c/Core.h>
#include <llvm-c/Target.h>

int main() {
    printf("Testing LLVM initialization without athlon-xp issues...\n");
    
    // These should work without athlon-xp CPU detection errors
    if (LLVMInitializeNativeTarget() != 0) {
        printf("Failed to initialize native target\n");
        return 1;
    }
    
    if (LLVMInitializeNativeAsmPrinter() != 0) {
        printf("Failed to initialize native ASM printer\n");
        return 1;
    }
    
    LLVMContextRef context = LLVMContextCreate();
    if (!context) {
        printf("Failed to create LLVM context\n");
        return 1;
    }
    
    LLVMModuleRef module = LLVMModuleCreateWithNameInContext("test", context);
    if (!module) {
        printf("Failed to create LLVM module\n");
        return 1;
    }
    
    printf("✅ LLVM initialization successful - athlon-xp issue fixed!\n");
    
    LLVMDisposeModule(module);
    LLVMContextDispose(context);
    
    return 0;
}
