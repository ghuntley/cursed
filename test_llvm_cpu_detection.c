#include <stdio.h>
#include <llvm-c/Core.h>
#include <llvm-c/Target.h>
#include <llvm-c/TargetMachine.h>

int main() {
    printf("Testing LLVM CPU detection...\n");
    
    // Initialize LLVM
    LLVMInitializeNativeTarget();
    LLVMInitializeNativeAsmPrinter();
    LLVMInitializeNativeAsmParser();
    
    // Create target machine 
    char* triple = LLVMGetDefaultTargetTriple();
    printf("Default target triple: %s\n", triple);
    
    LLVMTargetRef target;
    char* error_message = NULL;
    
    if (LLVMGetTargetFromTriple(triple, &target, &error_message)) {
        printf("Error creating target: %s\n", error_message);
        LLVMDisposeMessage(error_message);
        return 1;
    }
    
    // Create target machine with default CPU
    LLVMTargetMachineRef machine = LLVMCreateTargetMachine(
        target,
        triple,
        "",  // CPU - empty means default
        "",  // Features
        LLVMCodeGenLevelDefault,
        LLVMRelocDefault,
        LLVMCodeModelDefault
    );
    
    if (!machine) {
        printf("Failed to create target machine\n");
        return 1;
    }
    
    printf("Successfully created target machine\n");
    
    // Cleanup
    LLVMDisposeTargetMachine(machine);
    LLVMDisposeMessage(triple);
    
    return 0;
}
