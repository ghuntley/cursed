#ifndef LLVM_C_BINDINGS_H
#define LLVM_C_BINDINGS_H

// Minimal LLVM C bindings for CURSED compiler
// These are compatible with LLVM 18.x and provide the essential functions
// needed for our LLVM IR generation pipeline.

#ifdef __cplusplus
extern "C" {
#endif

// LLVM Types
typedef struct LLVMOpaqueContext *LLVMContextRef;
typedef struct LLVMOpaqueModule *LLVMModuleRef;
typedef struct LLVMOpaqueBuilder *LLVMBuilderRef;
typedef struct LLVMOpaqueValue *LLVMValueRef;
typedef struct LLVMOpaqueType *LLVMTypeRef;
typedef struct LLVMOpaqueBasicBlock *LLVMBasicBlockRef;
typedef struct LLVMOpaquePassManager *LLVMPassManagerRef;
typedef struct LLVMOpaqueTargetMachine *LLVMTargetMachineRef;
typedef struct LLVMOpaqueTarget *LLVMTargetRef;
typedef struct LLVMOpaqueExecutionEngine *LLVMExecutionEngineRef;
typedef struct LLVMOpaqueMemoryBuffer *LLVMMemoryBufferRef;
typedef struct LLVMOpaquePassRegistry *LLVMPassRegistryRef;
typedef int LLVMBool;

// Enums
typedef enum {
    LLVMVoidTypeKind,
    LLVMHalfTypeKind,
    LLVMFloatTypeKind,
    LLVMDoubleTypeKind,
    LLVMX86_FP80TypeKind,
    LLVMFP128TypeKind,
    LLVMPPC_FP128TypeKind,
    LLVMLabelTypeKind,
    LLVMIntegerTypeKind,
    LLVMFunctionTypeKind,
    LLVMStructTypeKind,
    LLVMArrayTypeKind,
    LLVMPointerTypeKind,
    LLVMVectorTypeKind,
    LLVMMetadataTypeKind,
    LLVMX86_MMXTypeKind,
    LLVMTokenTypeKind,
    LLVMScalableVectorTypeKind,
    LLVMBFloatTypeKind,
    LLVMX86_AMXTypeKind,
    LLVMTargetExtTypeKind
} LLVMTypeKind;

typedef enum {
    LLVMIntEQ = 32,
    LLVMIntNE,
    LLVMIntUGT,
    LLVMIntUGE,
    LLVMIntULT,
    LLVMIntULE,
    LLVMIntSGT,
    LLVMIntSGE,
    LLVMIntSLT,
    LLVMIntSLE
} LLVMIntPredicate;

typedef enum {
    LLVMCodeGenLevelNone,
    LLVMCodeGenLevelLess,
    LLVMCodeGenLevelDefault,
    LLVMCodeGenLevelAggressive
} LLVMCodeGenOptLevel;

typedef enum {
    LLVMRelocDefault,
    LLVMRelocStatic,
    LLVMRelocPIC,
    LLVMRelocDynamicNoPic,
    LLVMRelocROPI,
    LLVMRelocRWPI,
    LLVMRelocROPI_RWPI
} LLVMRelocMode;

typedef enum {
    LLVMCodeModelDefault,
    LLVMCodeModelJITDefault,
    LLVMCodeModelTiny,
    LLVMCodeModelSmall,
    LLVMCodeModelKernel,
    LLVMCodeModelMedium,
    LLVMCodeModelLarge
} LLVMCodeModel;

typedef enum {
    LLVMAbortProcessAction,
    LLVMPrintMessageAction,
    LLVMReturnStatusAction
} LLVMVerifierFailureAction;

typedef enum {
    LLVMModuleFlagBehaviorError,
    LLVMModuleFlagBehaviorWarning,
    LLVMModuleFlagBehaviorRequire,
    LLVMModuleFlagBehaviorOverride,
    LLVMModuleFlagBehaviorAppend,
    LLVMModuleFlagBehaviorAppendUnique,
    LLVMModuleFlagBehaviorMax
} LLVMModuleFlagBehavior;

typedef enum {
    LLVMExternalLinkage,
    LLVMAvailableExternallyLinkage,
    LLVMLinkOnceAnyLinkage,
    LLVMLinkOnceODRLinkage,
    LLVMLinkOnceODRAutoHideLinkage,
    LLVMWeakAnyLinkage,
    LLVMWeakODRLinkage,
    LLVMAppendingLinkage,
    LLVMInternalLinkage,
    LLVMPrivateLinkage,
    LLVMDLLImportLinkage,
    LLVMDLLExportLinkage,
    LLVMExternalWeakLinkage,
    LLVMGhostLinkage,
    LLVMCommonLinkage,
    LLVMLinkerPrivateLinkage,
    LLVMLinkerPrivateWeakLinkage
} LLVMLinkage;

// Function declarations - Core functions
LLVMPassRegistryRef LLVMGetGlobalPassRegistry(void);
void LLVMInitializeCore(LLVMPassRegistryRef R);
LLVMBool LLVMInitializeNativeTarget(void);
LLVMBool LLVMInitializeNativeAsmPrinter(void);
LLVMBool LLVMInitializeNativeAsmParser(void);

// Context functions
LLVMContextRef LLVMContextCreate(void);
void LLVMContextDispose(LLVMContextRef C);

// Module functions
LLVMModuleRef LLVMModuleCreateWithNameInContext(const char *ModuleID, LLVMContextRef C);
void LLVMDisposeModule(LLVMModuleRef M);
void LLVMSetTarget(LLVMModuleRef M, const char *Triple);
char *LLVMGetDefaultTargetTriple(void);
void LLVMDisposeMessage(char *Message);
LLVMBool LLVMVerifyModule(LLVMModuleRef M, LLVMVerifierFailureAction Action, char **OutMessage);
LLVMBool LLVMPrintModuleToFile(LLVMModuleRef M, const char *Filename, char **ErrorMessage);
void LLVMDumpModule(LLVMModuleRef M);
void LLVMAddModuleFlag(LLVMModuleRef M, LLVMModuleFlagBehavior Behavior, const char *Key, size_t KeyLen, LLVMValueRef Val);

// Builder functions
LLVMBuilderRef LLVMCreateBuilderInContext(LLVMContextRef C);
void LLVMDisposeBuilder(LLVMBuilderRef Builder);
void LLVMPositionBuilderAtEnd(LLVMBuilderRef Builder, LLVMBasicBlockRef Block);
LLVMBasicBlockRef LLVMGetInsertBlock(LLVMBuilderRef Builder);

// Type functions
LLVMTypeRef LLVMVoidTypeInContext(LLVMContextRef C);
LLVMTypeRef LLVMInt8TypeInContext(LLVMContextRef C);
LLVMTypeRef LLVMInt16TypeInContext(LLVMContextRef C);
LLVMTypeRef LLVMInt32TypeInContext(LLVMContextRef C);
LLVMTypeRef LLVMInt64TypeInContext(LLVMContextRef C);
LLVMTypeRef LLVMInt1TypeInContext(LLVMContextRef C);
LLVMTypeRef LLVMFloatTypeInContext(LLVMContextRef C);
LLVMTypeRef LLVMDoubleTypeInContext(LLVMContextRef C);
LLVMTypeRef LLVMPointerType(LLVMTypeRef ElementType, unsigned AddressSpace);
LLVMTypeRef LLVMArrayType(LLVMTypeRef ElementType, unsigned Count);
LLVMTypeRef LLVMFunctionType(LLVMTypeRef ReturnType, LLVMTypeRef *ParamTypes, unsigned ParamCount, LLVMBool IsVarArg);
LLVMTypeKind LLVMGetTypeKind(LLVMTypeRef Ty);
unsigned LLVMGetIntTypeWidth(LLVMTypeRef IntegerTy);
LLVMTypeRef LLVMGetElementType(LLVMTypeRef Ty);
LLVMTypeRef LLVMTypeOf(LLVMValueRef Val);
LLVMTypeRef LLVMGetAllocatedType(LLVMValueRef Alloca);

// Value functions
LLVMValueRef LLVMConstInt(LLVMTypeRef IntTy, unsigned long long N, LLVMBool SignExtend);
LLVMValueRef LLVMConstReal(LLVMTypeRef RealTy, double N);
LLVMValueRef LLVMConstStringInContext(LLVMContextRef C, const char *Str, unsigned Length, LLVMBool DontNullTerminate);
LLVMValueRef LLVMConstGEP2(LLVMTypeRef Ty, LLVMValueRef ConstantVal, LLVMValueRef *ConstantIndices, unsigned NumIndices);
LLVMValueRef LLVMValueAsMetadata(LLVMValueRef Val);
void LLVMSetValueName(LLVMValueRef Val, const char *Name);

// Function functions
LLVMValueRef LLVMAddFunction(LLVMModuleRef M, const char *Name, LLVMTypeRef FunctionTy);
LLVMValueRef LLVMGetFirstFunction(LLVMModuleRef M);
LLVMValueRef LLVMGetNextFunction(LLVMValueRef Fn);
LLVMValueRef LLVMGetParam(LLVMValueRef Fn, unsigned Index);

// Basic block functions
LLVMBasicBlockRef LLVMAppendBasicBlockInContext(LLVMContextRef C, LLVMValueRef Fn, const char *Name);
LLVMValueRef LLVMGetBasicBlockTerminator(LLVMBasicBlockRef BB);

// Instruction building functions
LLVMValueRef LLVMBuildAlloca(LLVMBuilderRef, LLVMTypeRef Ty, const char *Name);
LLVMValueRef LLVMBuildStore(LLVMBuilderRef, LLVMValueRef Val, LLVMValueRef Ptr);
LLVMValueRef LLVMBuildLoad2(LLVMBuilderRef, LLVMTypeRef Ty, LLVMValueRef PointerVal, const char *Name);
LLVMValueRef LLVMBuildAdd(LLVMBuilderRef, LLVMValueRef LHS, LLVMValueRef RHS, const char *Name);
LLVMValueRef LLVMBuildSub(LLVMBuilderRef, LLVMValueRef LHS, LLVMValueRef RHS, const char *Name);
LLVMValueRef LLVMBuildMul(LLVMBuilderRef, LLVMValueRef LHS, LLVMValueRef RHS, const char *Name);
LLVMValueRef LLVMBuildSDiv(LLVMBuilderRef, LLVMValueRef LHS, LLVMValueRef RHS, const char *Name);
LLVMValueRef LLVMBuildICmp(LLVMBuilderRef, LLVMIntPredicate Op, LLVMValueRef LHS, LLVMValueRef RHS, const char *Name);
LLVMValueRef LLVMBuildCall2(LLVMBuilderRef, LLVMTypeRef Ty, LLVMValueRef Fn, LLVMValueRef *Args, unsigned NumArgs, const char *Name);
LLVMValueRef LLVMBuildRet(LLVMBuilderRef, LLVMValueRef V);
LLVMValueRef LLVMBuildRetVoid(LLVMBuilderRef);
LLVMValueRef LLVMBuildBr(LLVMBuilderRef, LLVMBasicBlockRef Dest);
LLVMValueRef LLVMBuildCondBr(LLVMBuilderRef, LLVMValueRef If, LLVMBasicBlockRef Then, LLVMBasicBlockRef Else);
LLVMValueRef LLVMBuildStructGEP2(LLVMBuilderRef B, LLVMTypeRef Ty, LLVMValueRef Pointer, unsigned Idx, const char *Name);
LLVMValueRef LLVMBuildBitCast(LLVMBuilderRef, LLVMValueRef Val, LLVMTypeRef DestTy, const char *Name);

// Global variables
LLVMValueRef LLVMAddGlobal(LLVMModuleRef M, LLVMTypeRef Ty, const char *Name);
void LLVMSetInitializer(LLVMValueRef GlobalVar, LLVMValueRef ConstantVal);
void LLVMSetGlobalConstant(LLVMValueRef GlobalVar, LLVMBool IsConstant);
void LLVMSetLinkage(LLVMValueRef Global, LLVMLinkage Linkage);

// Target functions
LLVMBool LLVMGetTargetFromTriple(const char *Triple, LLVMTargetRef *T, char **ErrorMessage);
LLVMTargetMachineRef LLVMCreateTargetMachine(LLVMTargetRef T, const char *Triple, const char *CPU, const char *Features, LLVMCodeGenOptLevel Level, LLVMRelocMode Reloc, LLVMCodeModel CodeModel);
void LLVMDisposeTargetMachine(LLVMTargetMachineRef T);

// Pass manager functions
LLVMPassManagerRef LLVMCreateFunctionPassManagerForModule(LLVMModuleRef M);
void LLVMDisposePassManager(LLVMPassManagerRef PM);
LLVMBool LLVMInitializeFunctionPassManager(LLVMPassManagerRef FPM);
LLVMBool LLVMRunFunctionPassManager(LLVMPassManagerRef FPM, LLVMValueRef F);
LLVMBool LLVMFinalizeFunctionPassManager(LLVMPassManagerRef FPM);

// Optimization passes
void LLVMAddInstructionCombiningPass(LLVMPassManagerRef PM);
void LLVMAddReassociatePass(LLVMPassManagerRef PM);
void LLVMAddGVNPass(LLVMPassManagerRef PM);
void LLVMAddCFGSimplificationPass(LLVMPassManagerRef PM);
void LLVMAddPromoteMemoryToRegisterPass(LLVMPassManagerRef PM);
void LLVMAddDeadStoreEliminationPass(LLVMPassManagerRef PM);
void LLVMAddAggressiveDCEPass(LLVMPassManagerRef PM);
void LLVMAddLoopUnrollPass(LLVMPassManagerRef PM);
void LLVMAddLoopVectorizePass(LLVMPassManagerRef PM);
void LLVMAddSLPVectorizePass(LLVMPassManagerRef PM);

#ifdef __cplusplus
}
#endif

#endif // LLVM_C_BINDINGS_H
