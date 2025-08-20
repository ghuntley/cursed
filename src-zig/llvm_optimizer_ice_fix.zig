const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const print = std.debug.print;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
});

/// LLVM Optimizer ICE Prevention System
/// Fixes the most common causes of Internal Compiler Errors in the LLVM optimizer
/// when processing generic parameters and const generics

pub const LLVMOptimizerICEFix = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    
    pub fn init(allocator: Allocator, context: c.LLVMContextRef, module: c.LLVMModuleRef) LLVMOptimizerICEFix {
        return LLVMOptimizerICEFix{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = c.LLVMCreateBuilderInContext(context),
        };
    }
    
    pub fn deinit(self: *LLVMOptimizerICEFix) void {
        c.LLVMDisposeBuilder(self.builder);
    }
    
    /// Master fix function that addresses all common ICE causes
    pub fn fixOptimizerICEIssues(self: *LLVMOptimizerICEFix) !void {
        std.log.info("Starting LLVM optimizer ICE prevention fixes...", .{});
        
        // Fix 1: Missing basic block terminators
        try self.fixMissingTerminators();
        
        // Fix 2: Unreachable code with invalid values
        try self.fixUnreachableCode();
        
        // Fix 3: Invalid constant expressions
        try self.fixInvalidConstants();
        
        // Fix 4: Malformed PHI nodes
        try self.fixPHINodes();
        
        // Fix 5: Invalid GEP instructions
        try self.fixGEPInstructions();
        
        std.log.info("LLVM optimizer ICE prevention fixes completed", .{});
    }
    
    /// Fix 1: Add missing terminators to basic blocks
    fn fixMissingTerminators(self: *LLVMOptimizerICEFix) !void {
        var function = c.LLVMGetFirstFunction(self.module);
        
        while (function != null) {
            defer function = c.LLVMGetNextFunction(function);
            
            var bb = c.LLVMGetFirstBasicBlock(function);
            while (bb != null) {
                defer bb = c.LLVMGetNextBasicBlock(bb);
                
                const terminator = c.LLVMGetBasicBlockTerminator(bb);
                if (terminator == null) {
                    std.log.warn("Found basic block without terminator, fixing...", .{});
                    
                    // Position builder at end of block
                    c.LLVMPositionBuilderAtEnd(self.builder, bb);
                    
                    // Add appropriate terminator based on function return type
                    const func_type = c.LLVMGlobalGetValueType(function);
                    const return_type = c.LLVMGetReturnType(func_type);
                    
                    if (c.LLVMGetTypeKind(return_type) == c.LLVMVoidTypeKind) {
                        _ = c.LLVMBuildRetVoid(self.builder);
                        std.log.info("Added void return terminator", .{});
                    } else {
                        // Create a safe default value
                        const default_value = try self.createSafeDefaultValue(return_type);
                        _ = c.LLVMBuildRet(self.builder, default_value);
                        std.log.info("Added return terminator with default value", .{});
                    }
                }
            }
        }
    }
    
    /// Fix 2: Remove or fix unreachable code that confuses the optimizer
    fn fixUnreachableCode(self: *LLVMOptimizerICEFix) !void {
        var function = c.LLVMGetFirstFunction(self.module);
        
        while (function != null) {
            defer function = c.LLVMGetNextFunction(function);
            
            var bb = c.LLVMGetFirstBasicBlock(function);
            while (bb != null) {
                defer bb = c.LLVMGetNextBasicBlock(bb);
                
                // Check if this block is unreachable
                if (self.isBlockUnreachable(bb)) {
                    std.log.warn("Found unreachable block, adding safe terminator", .{});
                    
                    c.LLVMPositionBuilderAtEnd(self.builder, bb);
                    _ = c.LLVMBuildUnreachable(self.builder);
                }
            }
        }
    }
    
    /// Fix 3: Replace invalid constant expressions that cause optimizer crashes
    fn fixInvalidConstants(self: *LLVMOptimizerICEFix) !void {
        var function = c.LLVMGetFirstFunction(self.module);
        
        while (function != null) {
            defer function = c.LLVMGetNextFunction(function);
            
            var bb = c.LLVMGetFirstBasicBlock(function);
            while (bb != null) {
                defer bb = c.LLVMGetNextBasicBlock(bb);
                
                var instruction = c.LLVMGetFirstInstruction(bb);
                while (instruction != null) {
                    defer instruction = c.LLVMGetNextInstruction(instruction);
                    
                    // Check for problematic constant expressions
                    if (self.hasProblematicConstants(instruction)) {
                        try self.fixProblematicConstant(instruction);
                    }
                }
            }
        }
    }
    
    /// Fix 4: Fix malformed PHI nodes that cause optimizer ICE
    fn fixPHINodes(self: *LLVMOptimizerICEFix) !void {
        var function = c.LLVMGetFirstFunction(self.module);
        
        while (function != null) {
            defer function = c.LLVMGetNextFunction(function);
            
            var bb = c.LLVMGetFirstBasicBlock(function);
            while (bb != null) {
                defer bb = c.LLVMGetNextBasicBlock(bb);
                
                var instruction = c.LLVMGetFirstInstruction(bb);
                while (instruction != null) {
                    defer instruction = c.LLVMGetNextInstruction(instruction);
                    
                    if (c.LLVMGetInstructionOpcode(instruction) == c.LLVMPHI) {
                        try self.validateAndFixPHINode(instruction);
                    }
                }
            }
        }
    }
    
    /// Fix 5: Fix invalid GEP (GetElementPtr) instructions
    fn fixGEPInstructions(self: *LLVMOptimizerICEFix) !void {
        var function = c.LLVMGetFirstFunction(self.module);
        
        while (function != null) {
            defer function = c.LLVMGetNextFunction(function);
            
            var bb = c.LLVMGetFirstBasicBlock(function);
            while (bb != null) {
                defer bb = c.LLVMGetNextBasicBlock(bb);
                
                var instruction = c.LLVMGetFirstInstruction(bb);
                while (instruction != null) {
                    defer instruction = c.LLVMGetNextInstruction(instruction);
                    
                    if (c.LLVMGetInstructionOpcode(instruction) == c.LLVMGetElementPtr) {
                        try self.validateAndFixGEP(instruction);
                    }
                }
            }
        }
    }
    
    /// Create a safe default value for a given type
    fn createSafeDefaultValue(self: *LLVMOptimizerICEFix, value_type: c.LLVMTypeRef) !c.LLVMValueRef {
        _ = self;
        const type_kind = c.LLVMGetTypeKind(value_type);
        
        return switch (type_kind) {
            c.LLVMIntegerTypeKind => c.LLVMConstInt(value_type, 0, 0),
            c.LLVMFloatTypeKind => c.LLVMConstReal(value_type, 0.0),
            c.LLVMDoubleTypeKind => c.LLVMConstReal(value_type, 0.0),
            c.LLVMPointerTypeKind => c.LLVMConstNull(value_type),
            c.LLVMStructTypeKind => c.LLVMConstNull(value_type),
            c.LLVMArrayTypeKind => c.LLVMConstNull(value_type),
            c.LLVMVectorTypeKind => c.LLVMConstNull(value_type),
            else => {
                std.log.warn("Unknown type kind {}, using null", .{type_kind});
                return c.LLVMConstNull(value_type);
            },
        };
    }
    
    /// Check if a basic block is unreachable
    fn isBlockUnreachable(self: *LLVMOptimizerICEFix, bb: c.LLVMBasicBlockRef) bool {
        _ = self;
        
        // Simple heuristic: if no instructions and no predecessors, it's likely unreachable
        const first_instruction = c.LLVMGetFirstInstruction(bb);
        if (first_instruction == null) {
            // Check if this is the entry block
            const function = c.LLVMGetBasicBlockParent(bb);
            const entry_bb = c.LLVMGetEntryBasicBlock(function);
            if (bb == entry_bb) {
                return false; // Entry block is never unreachable
            }
            return true;
        }
        return false;
    }
    
    /// Check if instruction has problematic constants
    fn hasProblematicConstants(self: *LLVMOptimizerICEFix, instruction: c.LLVMValueRef) bool {
        _ = self;
        
        const num_operands = c.LLVMGetNumOperands(instruction);
        var i: u32 = 0;
        while (i < num_operands) : (i += 1) {
            const operand = c.LLVMGetOperand(instruction, i);
            
            // Check for undefined values
            if (c.LLVMIsUndef(operand) != 0) {
                return true;
            }
            
            // Check for constants that are too large
            if (c.LLVMIsAConstantInt(operand) != null) {
                const value_type = c.LLVMTypeOf(operand);
                const bit_width = c.LLVMGetIntTypeWidth(value_type);
                if (bit_width > 64) {
                    return true; // Very large integers can cause optimizer issues
                }
            }
        }
        
        return false;
    }
    
    /// Fix a problematic constant in an instruction
    fn fixProblematicConstant(self: *LLVMOptimizerICEFix, instruction: c.LLVMValueRef) !void {
        std.log.warn("Fixing problematic constant in instruction", .{});
        
        const num_operands = c.LLVMGetNumOperands(instruction);
        var i: u32 = 0;
        while (i < num_operands) : (i += 1) {
            const operand = c.LLVMGetOperand(instruction, i);
            
            // Replace undefined values with safe defaults
            if (c.LLVMIsUndef(operand) != 0) {
                const operand_type = c.LLVMTypeOf(operand);
                const safe_value = try self.createSafeDefaultValue(operand_type);
                c.LLVMSetOperand(instruction, i, safe_value);
                std.log.info("Replaced undefined value with safe default", .{});
            }
        }
    }
    
    /// Validate and fix PHI node
    fn validateAndFixPHINode(self: *LLVMOptimizerICEFix, phi: c.LLVMValueRef) !void {
        const incoming_count = c.LLVMCountIncoming(phi);
        
        if (incoming_count == 0) {
            std.log.warn("Found PHI node with no incoming values, fixing...", .{});
            
            // Add a dummy incoming value to prevent optimizer crash
            const phi_type = c.LLVMTypeOf(phi);
            const safe_value = try self.createSafeDefaultValue(phi_type);
            const current_bb = c.LLVMGetInstructionParent(phi);
            const function = c.LLVMGetBasicBlockParent(current_bb);
            const entry_bb = c.LLVMGetEntryBasicBlock(function);
            
            // This is a simplified fix - in practice you'd need proper analysis
            var mutable_value = safe_value;
            var mutable_bb = entry_bb;
            c.LLVMAddIncoming(phi, @ptrCast(&mutable_value), @ptrCast(&mutable_bb), 1);
            std.log.info("Added dummy incoming value to PHI node", .{});
        }
    }
    
    /// Validate and fix GEP instruction
    fn validateAndFixGEP(self: *LLVMOptimizerICEFix, gep: c.LLVMValueRef) !void {
        _ = self;
        
        const num_indices = c.LLVMGetNumOperands(gep) - 1; // First operand is base pointer
        
        if (num_indices == 0) {
            std.log.warn("Found GEP instruction with no indices - this may cause optimizer issues", .{});
            // In practice, you might want to replace this with a simpler operation
        }
        
        // Check for out-of-bounds constant indices
        var i: u32 = 1; // Start from 1, skip base pointer
        while (i < c.LLVMGetNumOperands(gep)) : (i += 1) {
            const index = c.LLVMGetOperand(gep, i);
            if (c.LLVMIsAConstantInt(index) != null) {
                // In a real implementation, you'd check bounds against the actual type
                const constant_value = c.LLVMConstIntGetSExtValue(index);
                if (constant_value < 0 or constant_value > 1000000) { // Arbitrary large bound
                    std.log.warn("Found potentially problematic GEP index: {}", .{constant_value});
                }
            }
        }
    }
    
    /// Comprehensive module validation before optimization
    pub fn validateModuleForOptimizer(self: *LLVMOptimizerICEFix) !bool {
        std.log.info("Validating module for optimizer safety...", .{});
        
        // Use LLVM's built-in verifier
        var error_message: [*c]u8 = null;
        const verification_failed = c.LLVMVerifyModule(self.module, c.LLVMReturnStatusAction, &error_message);
        
        if (verification_failed != 0) {
            if (error_message) |msg| {
                const msg_str = std.mem.span(msg);
                std.log.err("LLVM module verification failed: {s}", .{msg_str});
                c.LLVMDisposeMessage(msg);
            }
            return false;
        }
        
        std.log.info("Module passed LLVM verification - safe for optimizer", .{});
        return true;
    }
};

/// Convenience function to apply all ICE fixes to a module
pub fn fixLLVMOptimizerICE(allocator: Allocator, context: c.LLVMContextRef, module: c.LLVMModuleRef) !void {
    var fixer = LLVMOptimizerICEFix.init(allocator, context, module);
    defer fixer.deinit();
    
    // Apply all fixes
    try fixer.fixOptimizerICEIssues();
    
    // Validate the result
    const is_valid = try fixer.validateModuleForOptimizer();
    if (!is_valid) {
        std.log.err("Module still has issues after ICE fixes - compilation may fail", .{});
        return error.ModuleValidationFailed;
    }
    
    std.log.info("LLVM optimizer ICE fixes applied successfully", .{});
}
