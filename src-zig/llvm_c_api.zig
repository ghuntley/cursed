const std = @import("std");

// LLVM C API bindings - simplified wrapper for optimization passes
pub const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/Transforms/PassManagerBuilder.h");
    @cInclude("llvm-c/Transforms/IPO.h");
    @cInclude("llvm-c/Transforms/Scalar.h");
    @cInclude("llvm-c/Transforms/Utils.h");
    @cInclude("llvm-c/Transforms/Vectorize.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

// Re-export LLVM functions for easier access
pub const LLVMCreatePassManager = c.LLVMCreatePassManager;
pub const LLVMCreateFunctionPassManagerForModule = c.LLVMCreateFunctionPassManagerForModule;
pub const LLVMRunPassManager = c.LLVMRunPassManager;
pub const LLVMInitializeFunctionPassManager = c.LLVMInitializeFunctionPassManager;
pub const LLVMRunFunctionPassManager = c.LLVMRunFunctionPassManager;
pub const LLVMFinalizeFunctionPassManager = c.LLVMFinalizeFunctionPassManager;

// Memory and SSA passes
pub const LLVMAddPromoteMemoryToRegisterPass = c.LLVMAddPromoteMemoryToRegisterPass;
pub const LLVMAddSROAPass = c.LLVMAddSROAPass;

// Instruction optimization passes
pub const LLVMAddInstructionCombiningPass = c.LLVMAddInstructionCombiningPass;
pub const LLVMAddReassociatePass = c.LLVMAddReassociatePass;
pub const LLVMAddCFGSimplificationPass = c.LLVMAddCFGSimplificationPass;

// Dead code elimination passes
pub const LLVMAddDeadCodeEliminationPass = c.LLVMAddDeadCodeEliminationPass;
pub const LLVMAddAggressiveDCEPass = c.LLVMAddAggressiveDCEPass;
pub const LLVMAddGlobalDCEPass = c.LLVMAddGlobalDCEPass;

// Constant propagation passes
pub const LLVMAddConstantPropagationPass = c.LLVMAddConstantPropagationPass;
pub const LLVMAddSCCPPass = c.LLVMAddSCCPPass;

// Global optimization passes
pub const LLVMAddGVNPass = c.LLVMAddGVNPass;
pub const LLVMAddGlobalOptimizerPass = c.LLVMAddGlobalOptimizerPass;

// Vectorization passes
pub const LLVMAddLoopVectorizePass = c.LLVMAddLoopVectorizePass;
pub const LLVMAddSLPVectorizePass = c.LLVMAddSLPVectorizePass;
pub const LLVMAddLoadStoreVectorizerPass = c.LLVMAddLoadStoreVectorizerPass;

// Loop optimization passes
pub const LLVMAddLICMPass = c.LLVMAddLICMPass;
pub const LLVMAddLoopUnrollPass = c.LLVMAddLoopUnrollPass;
pub const LLVMAddJumpThreadingPass = c.LLVMAddJumpThreadingPass;

// Memory optimization passes
pub const LLVMAddMemCpyOptPass = c.LLVMAddMemCpyOptPass;
pub const LLVMAddDeadStoreEliminationPass = c.LLVMAddDeadStoreEliminationPass;

// Interprocedural passes
pub const LLVMAddFunctionInliningPass = c.LLVMAddFunctionInliningPass;
pub const LLVMAddAlwaysInlinerPass = c.LLVMAddAlwaysInlinerPass;

// Cleanup passes
pub const LLVMAddTailCallEliminationPass = c.LLVMAddTailCallEliminationPass;
pub const LLVMAddStripDeadPrototypesPass = c.LLVMAddStripDeadPrototypesPass;
pub const LLVMAddStripSymbolsPass = c.LLVMAddStripSymbolsPass;

// Target-specific passes
pub const LLVMAddTargetLibraryInfoPass = c.LLVMAddTargetLibraryInfoPass;
pub const LLVMAddMergeFunctionsPass = c.LLVMAddMergeFunctionsPass;
pub const LLVMAddConstantMergePass = c.LLVMAddConstantMergePass;

// Module manipulation
pub const LLVMGetFirstFunction = c.LLVMGetFirstFunction;
pub const LLVMGetNextFunction = c.LLVMGetNextFunction;
pub const LLVMGetNamedFunction = c.LLVMGetNamedFunction;
pub const LLVMDeleteFunction = c.LLVMDeleteFunction;
pub const LLVMGetValueName = c.LLVMGetValueName;
pub const LLVMGetLinkage = c.LLVMGetLinkage;
pub const LLVMLinkModules2 = c.LLVMLinkModules2;
pub const LLVMVerifyModule = c.LLVMVerifyModule;
pub const LLVMDisposeMessage = c.LLVMDisposeMessage;
pub const LLVMWriteBitcodeToFile = c.LLVMWriteBitcodeToFile;

// Constants and types
pub const LLVMInternalLinkage = c.LLVMInternalLinkage;
pub const LLVMReturnStatusAction = c.LLVMReturnStatusAction;

// Helper functions for pass management
pub fn createOptimizedPassManager(allocator: std.mem.Allocator) !*PassManager {
        
    const pm = PassManager{
        .module_pm = c.LLVMCreatePassManager(),
        .function_pm = null,
    };
    
    return &pm;
}

pub const PassManager = struct {
    module_pm: c.LLVMPassManagerRef,
    function_pm: ?c.LLVMPassManagerRef,
    
    pub fn addBasicOptimizations(self: *PassManager) void {
        // Add basic optimization passes
        c.LLVMAddCFGSimplificationPass(self.module_pm);
        c.LLVMAddDeadCodeEliminationPass(self.module_pm);
        c.LLVMAddConstantPropagationPass(self.module_pm);
    }
    
    pub fn dispose(self: *PassManager) void {
        if (self.function_pm) |fpm| {
            c.LLVMDisposePassManager(fpm);
        }
        c.LLVMDisposePassManager(self.module_pm);
    }
};
