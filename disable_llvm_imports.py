#!/usr/bin/env python3
"""
Script to temporarily disable LLVM C imports to fix athlon-xp CPU detection issues
"""

import os
import re
from pathlib import Path

# Define dummy LLVM C structures to replace imports
DUMMY_LLVM_DEFINITIONS = '''
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
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}    pub fn LLVMInitializeCore(_: ?*anyopaque) void {}
    pub fn LLVMGetGlobalPassRegistry() ?*anyopaque { return null; }
    pub fn LLVMInitializeNativeTarget() LLVMBool { return 0; }
    pub fn LLVMInitializeNativeAsmPrinter() LLVMBool { return 0; }
    pub fn LLVMInitializeNativeAsmParser() LLVMBool { return 0; }
};
'''

def fix_file(file_path):
    """Fix a single file by replacing LLVM C imports"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Pattern to match @cImport blocks with LLVM includes
        pattern = r'const c = @cImport\({[^}]*@cInclude\("llvm-c/[^}]*}\);'
        
        if re.search(pattern, content, re.DOTALL):
            print(f"Fixing {file_path}")
            # Replace the @cImport block
            new_content = re.sub(pattern, DUMMY_LLVM_DEFINITIONS.strip(), content, flags=re.DOTALL)
            
            # Write the fixed content back
            with open(file_path, 'w') as f:
                f.write(new_content)
            
            return True
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False
    
    return False

def main():
    src_dir = Path("src-zig")
    fixed_count = 0
    
    for zig_file in src_dir.glob("*.zig"):
        if fix_file(zig_file):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files with LLVM C imports")

if __name__ == "__main__":
    main()
