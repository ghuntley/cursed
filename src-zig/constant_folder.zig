const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
});

/// Advanced constant folding and propagation engine
/// Performs compile-time evaluation of constants and simplifies expressions
pub const ConstantFolder = struct {
    allocator: Allocator,
    
    // Constant value cache
    constant_cache: HashMap(c.LLVMValueRef, c.LLVMValueRef, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage),
    
    // Expression simplification cache
    expression_cache: HashMap(ExpressionKey, c.LLVMValueRef, ExpressionContext, std.hash_map.default_max_load_percentage),
    
    // Configuration
    config: ConstantFoldingConfig,
    
    // Statistics
    stats: ConstantFoldingStats,

    pub fn init(allocator: Allocator) !ConstantFolder {
        return ConstantFolder{
            .allocator = allocator,
            .constant_cache = HashMap(c.LLVMValueRef, c.LLVMValueRef, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage).init(allocator),
            .expression_cache = HashMap(ExpressionKey, c.LLVMValueRef, ExpressionContext, std.hash_map.default_max_load_percentage).init(allocator),
            .config = ConstantFoldingConfig.default(),
            .stats = ConstantFoldingStats.init(),
        };
    }

    pub fn deinit(self: *ConstantFolder) void {
        self.constant_cache.deinit();
        self.expression_cache.deinit();
    }

    /// Fold constants throughout the module
    pub fn foldConstants(self: *ConstantFolder, module: c.LLVMModuleRef) !u32 {
        const start_time = std.time.nanoTimestamp();
        
        var folded_count: u32 = 0;
        
        // Process all functions
        var function = c.LLVMGetFirstFunction(module);
        while (function != null) {
            folded_count += try self.foldConstantsInFunction(function.?);
            function = c.LLVMGetNextFunction(function.?);
        }
        
        // Process global variables
        folded_count += try self.foldGlobalConstants(module);
        
        const end_time = std.time.nanoTimestamp();
        self.stats.folding_time_ns = end_time - start_time;
        self.stats.constants_folded = folded_count;
        
        std.debug.print("✅ Constant folding: {} constants folded\n", .{folded_count});
        
        return folded_count;
    }

    /// Fold constants in a specific function
    fn foldConstantsInFunction(self: *ConstantFolder, function: c.LLVMValueRef) !u32 {
        var folded_count: u32 = 0;
        
        var basic_block = c.LLVMGetFirstBasicBlock(function);
        while (basic_block != null) {
            var instruction = c.LLVMGetFirstInstruction(basic_block.?);
            
            while (instruction != null) {
                const next_instruction = c.LLVMGetNextInstruction(instruction.?);
                
                if (try self.foldConstantInstruction(instruction.?)) {
                    folded_count += 1;
                }
                
                instruction = next_instruction;
            }
            
            basic_block = c.LLVMGetNextBasicBlock(basic_block.?);
        }
        
        return folded_count;
    }

    /// Fold constants in global variables
    fn foldGlobalConstants(self: *ConstantFolder, module: c.LLVMModuleRef) !u32 {
        var folded_count: u32 = 0;
        
        var global = c.LLVMGetFirstGlobal(module);
        while (global != null) {
            if (try self.foldGlobalConstant(global.?)) {
                folded_count += 1;
            }
            global = c.LLVMGetNextGlobal(global.?);
        }
        
        return folded_count;
    }

    /// Attempt to fold a constant instruction
    fn foldConstantInstruction(self: *ConstantFolder, instruction: c.LLVMValueRef) !bool {
        // Check if instruction can be folded
        if (!self.canFoldInstruction(instruction)) {
            return false;
        }
        
        // Check cache first
        if (self.constant_cache.get(instruction)) |cached_constant| {
            self.replaceInstruction(instruction, cached_constant);
            return true;
        }
        
        // Attempt to fold based on instruction type
        const opcode = c.LLVMGetInstructionOpcode(instruction);
        
        const folded_value = switch (opcode) {
            c.LLVMAdd => try self.foldBinaryArithmetic(instruction, .Add),
            c.LLVMSub => try self.foldBinaryArithmetic(instruction, .Sub),
            c.LLVMMul => try self.foldBinaryArithmetic(instruction, .Mul),
            c.LLVMUDiv, c.LLVMSDiv => try self.foldBinaryArithmetic(instruction, .Div),
            c.LLVMURem, c.LLVMSRem => try self.foldBinaryArithmetic(instruction, .Rem),
            c.LLVMAnd => try self.foldBinaryArithmetic(instruction, .And),
            c.LLVMOr => try self.foldBinaryArithmetic(instruction, .Or),
            c.LLVMXor => try self.foldBinaryArithmetic(instruction, .Xor),
            c.LLVMShl => try self.foldBinaryArithmetic(instruction, .Shl),
            c.LLVMLShr, c.LLVMAShr => try self.foldBinaryArithmetic(instruction, .Shr),
            c.LLVMFAdd => try self.foldFloatingPointArithmetic(instruction, .FAdd),
            c.LLVMFSub => try self.foldFloatingPointArithmetic(instruction, .FSub),
            c.LLVMFMul => try self.foldFloatingPointArithmetic(instruction, .FMul),
            c.LLVMFDiv => try self.foldFloatingPointArithmetic(instruction, .FDiv),
            c.LLVMFRem => try self.foldFloatingPointArithmetic(instruction, .FRem),
            c.LLVMICmp => try self.foldIntegerComparison(instruction),
            c.LLVMFCmp => try self.foldFloatingPointComparison(instruction),
            c.LLVMSelect => try self.foldSelectInstruction(instruction),
            c.LLVMZExt, c.LLVMSExt, c.LLVMTrunc => try self.foldCastInstruction(instruction),
            c.LLVMBitCast => try self.foldBitCastInstruction(instruction),
            c.LLVMGetElementPtr => try self.foldGEPInstruction(instruction),
            else => null,
        };
        
        if (folded_value) |constant| {
            // Cache the result
            try self.constant_cache.put(instruction, constant);
            
            // Replace the instruction
            self.replaceInstruction(instruction, constant);
            
            self.stats.instruction_eliminations += 1;
            return true;
        }
        
        return false;
    }

    /// Fold binary arithmetic operations
    fn foldBinaryArithmetic(self: *ConstantFolder, instruction: c.LLVMValueRef, operation: ArithmeticOperation) !?c.LLVMValueRef {
        const lhs = c.LLVMGetOperand(instruction, 0);
        const rhs = c.LLVMGetOperand(instruction, 1);
        
        // Both operands must be constants
        if (!c.LLVMIsConstant(lhs) or !c.LLVMIsConstant(rhs)) {
            return try self.foldPartiallyConstantArithmetic(instruction, lhs, rhs, operation);
        }
        
        // Get integer values
        const lhs_int = c.LLVMConstIntGetSExtValue(lhs);
        const rhs_int = c.LLVMConstIntGetSExtValue(rhs);
        
        const result_int = switch (operation) {
            .Add => lhs_int +% rhs_int,
            .Sub => lhs_int -% rhs_int,
            .Mul => lhs_int *% rhs_int,
            .Div => if (rhs_int != 0) @divTrunc(lhs_int, rhs_int) else return null,
            .Rem => if (rhs_int != 0) @rem(lhs_int, rhs_int) else return null,
            .And => lhs_int & rhs_int,
            .Or => lhs_int | rhs_int,
            .Xor => lhs_int ^ rhs_int,
            .Shl => lhs_int << @as(u6, @intCast(@mod(rhs_int, 64))),
            .Shr => lhs_int >> @as(u6, @intCast(@mod(rhs_int, 64))),
            else => return null,
        };
        
        // Create constant result
        const result_type = c.LLVMTypeOf(instruction);
        const result_constant = c.LLVMConstInt(result_type, @as(u64, @bitCast(result_int)), 1);
        
        return result_constant;
    }

    /// Fold floating-point arithmetic operations
    fn foldFloatingPointArithmetic(self: *ConstantFolder, instruction: c.LLVMValueRef, operation: FloatOperation) !?c.LLVMValueRef {
        _ = self;
        
        const lhs = c.LLVMGetOperand(instruction, 0);
        const rhs = c.LLVMGetOperand(instruction, 1);
        
        // Both operands must be constants
        if (!c.LLVMIsConstant(lhs) or !c.LLVMIsConstant(rhs)) {
            return null;
        }
        
        // Get floating-point values
        const lhs_float = c.LLVMConstRealGetDouble(lhs, null);
        const rhs_float = c.LLVMConstRealGetDouble(rhs, null);
        
        const result_float = switch (operation) {
            .FAdd => lhs_float + rhs_float,
            .FSub => lhs_float - rhs_float,
            .FMul => lhs_float * rhs_float,
            .FDiv => if (rhs_float != 0.0) lhs_float / rhs_float else return null,
            .FRem => if (rhs_float != 0.0) @mod(lhs_float, rhs_float) else return null,
        };
        
        // Create constant result
        const result_type = c.LLVMTypeOf(instruction);
        const result_constant = c.LLVMConstReal(result_type, result_float);
        
        return result_constant;
    }

    /// Fold integer comparison operations
    fn foldIntegerComparison(self: *ConstantFolder, instruction: c.LLVMValueRef) !?c.LLVMValueRef {
        _ = self;
        
        const lhs = c.LLVMGetOperand(instruction, 0);
        const rhs = c.LLVMGetOperand(instruction, 1);
        
        if (!c.LLVMIsConstant(lhs) or !c.LLVMIsConstant(rhs)) {
            return null;
        }
        
        const lhs_int = c.LLVMConstIntGetSExtValue(lhs);
        const rhs_int = c.LLVMConstIntGetSExtValue(rhs);
        const predicate = c.LLVMGetICmpPredicate(instruction);
        
        const result_bool = switch (predicate) {
            c.LLVMIntEQ => lhs_int == rhs_int,
            c.LLVMIntNE => lhs_int != rhs_int,
            c.LLVMIntSLT => lhs_int < rhs_int,
            c.LLVMIntSLE => lhs_int <= rhs_int,
            c.LLVMIntSGT => lhs_int > rhs_int,
            c.LLVMIntSGE => lhs_int >= rhs_int,
            c.LLVMIntULT => @as(u64, @bitCast(lhs_int)) < @as(u64, @bitCast(rhs_int)),
            c.LLVMIntULE => @as(u64, @bitCast(lhs_int)) <= @as(u64, @bitCast(rhs_int)),
            c.LLVMIntUGT => @as(u64, @bitCast(lhs_int)) > @as(u64, @bitCast(rhs_int)),
            c.LLVMIntUGE => @as(u64, @bitCast(lhs_int)) >= @as(u64, @bitCast(rhs_int)),
            else => return null,
        };
        
        // Create boolean constant
        const i1_type = c.LLVMInt1Type();
        const result_constant = c.LLVMConstInt(i1_type, if (result_bool) 1 else 0, 0);
        
        return result_constant;
    }

    /// Fold floating-point comparison operations
    fn foldFloatingPointComparison(self: *ConstantFolder, instruction: c.LLVMValueRef) !?c.LLVMValueRef {
        _ = self;
        
        const lhs = c.LLVMGetOperand(instruction, 0);
        const rhs = c.LLVMGetOperand(instruction, 1);
        
        if (!c.LLVMIsConstant(lhs) or !c.LLVMIsConstant(rhs)) {
            return null;
        }
        
        const lhs_float = c.LLVMConstRealGetDouble(lhs, null);
        const rhs_float = c.LLVMConstRealGetDouble(rhs, null);
        const predicate = c.LLVMGetFCmpPredicate(instruction);
        
        const result_bool = switch (predicate) {
            c.LLVMRealOEQ => lhs_float == rhs_float,
            c.LLVMRealONE => lhs_float != rhs_float,
            c.LLVMRealOLT => lhs_float < rhs_float,
            c.LLVMRealOLE => lhs_float <= rhs_float,
            c.LLVMRealOGT => lhs_float > rhs_float,
            c.LLVMRealOGE => lhs_float >= rhs_float,
            else => return null, // Unordered comparisons are complex
        };
        
        // Create boolean constant
        const i1_type = c.LLVMInt1Type();
        const result_constant = c.LLVMConstInt(i1_type, if (result_bool) 1 else 0, 0);
        
        return result_constant;
    }

    /// Fold select instructions (ternary operator)
    fn foldSelectInstruction(self: *ConstantFolder, instruction: c.LLVMValueRef) !?c.LLVMValueRef {
        _ = self;
        
        const condition = c.LLVMGetOperand(instruction, 0);
        const true_value = c.LLVMGetOperand(instruction, 1);
        const false_value = c.LLVMGetOperand(instruction, 2);
        
        // If condition is constant, select the appropriate value
        if (c.LLVMIsConstant(condition)) {
            const condition_int = c.LLVMConstIntGetZExtValue(condition);
            return if (condition_int != 0) true_value else false_value;
        }
        
        // If both values are the same, return that value
        if (true_value == false_value) {
            return true_value;
        }
        
        return null;
    }

    /// Fold cast instructions
    fn foldCastInstruction(self: *ConstantFolder, instruction: c.LLVMValueRef) !?c.LLVMValueRef {
        _ = self;
        
        const operand = c.LLVMGetOperand(instruction, 0);
        
        if (!c.LLVMIsConstant(operand)) {
            return null;
        }
        
        const dest_type = c.LLVMTypeOf(instruction);
        const opcode = c.LLVMGetInstructionOpcode(instruction);
        
        switch (opcode) {
            c.LLVMZExt => return c.LLVMConstZExt(operand, dest_type),
            c.LLVMSExt => return c.LLVMConstSExt(operand, dest_type),
            c.LLVMTrunc => return c.LLVMConstTrunc(operand, dest_type),
            else => return null,
        }
    }

    /// Fold bitcast instructions
    fn foldBitCastInstruction(self: *ConstantFolder, instruction: c.LLVMValueRef) !?c.LLVMValueRef {
        _ = self;
        
        const operand = c.LLVMGetOperand(instruction, 0);
        
        if (!c.LLVMIsConstant(operand)) {
            return null;
        }
        
        const dest_type = c.LLVMTypeOf(instruction);
        return c.LLVMConstBitCast(operand, dest_type);
    }

    /// Fold GetElementPtr instructions
    fn foldGEPInstruction(self: *ConstantFolder, instruction: c.LLVMValueRef) !?c.LLVMValueRef {
        _ = self;
        
        // Check if all operands are constants
        const num_operands = c.LLVMGetNumOperands(instruction);
        var i: u32 = 0;
        while (i < num_operands) {
            const operand = c.LLVMGetOperand(instruction, i);
            if (!c.LLVMIsConstant(operand)) {
                return null;
            }
            i += 1;
        }
        
        // If all operands are constants, use LLVM's constant GEP
        const base_ptr = c.LLVMGetOperand(instruction, 0);
        const base_type = c.LLVMGetGEPSourceElementType(instruction);
        
        // Collect indices
        var indices = std.ArrayList(c.LLVMValueRef).init(self.allocator);
        defer indices.deinit();
        
        i = 1;
        while (i < num_operands) {
            try indices.append(c.LLVMGetOperand(instruction, i));
            i += 1;
        }
        
        return c.LLVMConstInBoundsGEP2(base_type, base_ptr, indices.items.ptr, @as(u32, @intCast(indices.items.len)));
    }

    /// Fold partially constant arithmetic (one operand is constant)
    fn foldPartiallyConstantArithmetic(self: *ConstantFolder, instruction: c.LLVMValueRef, lhs: c.LLVMValueRef, rhs: c.LLVMValueRef, operation: ArithmeticOperation) !?c.LLVMValueRef {
        _ = self;
        
        // Handle identity operations
        if (c.LLVMIsConstant(rhs)) {
            const rhs_int = c.LLVMConstIntGetSExtValue(rhs);
            
            switch (operation) {
                .Add => if (rhs_int == 0) return lhs, // x + 0 = x
                .Sub => if (rhs_int == 0) return lhs, // x - 0 = x
                .Mul => {
                    if (rhs_int == 0) {
                        // x * 0 = 0
                        const result_type = c.LLVMTypeOf(instruction);
                        return c.LLVMConstInt(result_type, 0, 0);
                    }
                    if (rhs_int == 1) return lhs; // x * 1 = x
                },
                .Or => if (rhs_int == 0) return lhs, // x | 0 = x
                .And => {
                    if (rhs_int == 0) {
                        // x & 0 = 0
                        const result_type = c.LLVMTypeOf(instruction);
                        return c.LLVMConstInt(result_type, 0, 0);
                    }
                },
                .Xor => if (rhs_int == 0) return lhs, // x ^ 0 = x
                else => {},
            }
        }
        
        if (c.LLVMIsConstant(lhs)) {
            const lhs_int = c.LLVMConstIntGetSExtValue(lhs);
            
            switch (operation) {
                .Add => if (lhs_int == 0) return rhs, // 0 + x = x
                .Mul => {
                    if (lhs_int == 0) {
                        // 0 * x = 0
                        const result_type = c.LLVMTypeOf(instruction);
                        return c.LLVMConstInt(result_type, 0, 0);
                    }
                    if (lhs_int == 1) return rhs; // 1 * x = x
                },
                .Or => if (lhs_int == 0) return rhs, // 0 | x = x
                .And => {
                    if (lhs_int == 0) {
                        // 0 & x = 0
                        const result_type = c.LLVMTypeOf(instruction);
                        return c.LLVMConstInt(result_type, 0, 0);
                    }
                },
                else => {},
            }
        }
        
        return null;
    }

    /// Fold a global constant
    fn foldGlobalConstant(self: *ConstantFolder, global: c.LLVMValueRef) !bool {
        _ = self;
        
        const initializer = c.LLVMGetInitializer(global);
        if (initializer == null) {
            return false;
        }
        
        // For now, just check if the initializer is already a constant
        // More sophisticated global constant folding could be implemented here
        return c.LLVMIsConstant(initializer.?) != 0;
    }

    /// Check if an instruction can be folded
    fn canFoldInstruction(self: *ConstantFolder, instruction: c.LLVMValueRef) bool {
        _ = self;
        
        const opcode = c.LLVMGetInstructionOpcode(instruction);
        
        // List of opcodes that can be folded
        switch (opcode) {
            c.LLVMAdd, c.LLVMSub, c.LLVMMul, c.LLVMUDiv, c.LLVMSDiv,
            c.LLVMURem, c.LLVMSRem, c.LLVMAnd, c.LLVMOr, c.LLVMXor,
            c.LLVMShl, c.LLVMLShr, c.LLVMAShr,
            c.LLVMFAdd, c.LLVMFSub, c.LLVMFMul, c.LLVMFDiv, c.LLVMFRem,
            c.LLVMICmp, c.LLVMFCmp, c.LLVMSelect,
            c.LLVMZExt, c.LLVMSExt, c.LLVMTrunc, c.LLVMBitCast,
            c.LLVMGetElementPtr => return true,
            else => return false,
        }
    }

    /// Replace an instruction with a constant
    fn replaceInstruction(self: *ConstantFolder, instruction: c.LLVMValueRef, constant: c.LLVMValueRef) void {
        _ = self;
        
        c.LLVMReplaceAllUsesWith(instruction, constant);
        c.LLVMInstructionEraseFromParent(instruction);
    }
};

/// Arithmetic operations for folding
const ArithmeticOperation = enum {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    Shl,
    Shr,
};

/// Floating-point operations for folding
const FloatOperation = enum {
    FAdd,
    FSub,
    FMul,
    FDiv,
    FRem,
};

/// Expression key for caching
const ExpressionKey = struct {
    opcode: c.LLVMOpcode,
    operand1: c.LLVMValueRef,
    operand2: c.LLVMValueRef,
    extra_data: u64, // For additional context like comparison predicates
};

/// Expression context for HashMap
const ExpressionContext = struct {
    pub fn hash(self: @This(), key: ExpressionKey) u64 {
        _ = self;
        var hasher = std.hash_map.DefaultHasher.init();
        hasher.update(std.mem.asBytes(&key.opcode));
        hasher.update(std.mem.asBytes(&key.operand1));
        hasher.update(std.mem.asBytes(&key.operand2));
        hasher.update(std.mem.asBytes(&key.extra_data));
        return hasher.final();
    }
    
    pub fn eql(self: @This(), a: ExpressionKey, b: ExpressionKey) bool {
        _ = self;
        return a.opcode == b.opcode and 
               a.operand1 == b.operand1 and 
               a.operand2 == b.operand2 and 
               a.extra_data == b.extra_data;
    }
};

/// Constant folding configuration
const ConstantFoldingConfig = struct {
    fold_arithmetic: bool = true,
    fold_comparisons: bool = true,
    fold_casts: bool = true,
    fold_gep: bool = true,
    fold_select: bool = true,
    enable_algebraic_simplification: bool = true,
    max_folding_depth: u32 = 10,
    
    pub fn default() ConstantFoldingConfig {
        return ConstantFoldingConfig{};
    }
    
    pub fn aggressive() ConstantFoldingConfig {
        return ConstantFoldingConfig{
            .max_folding_depth = 20,
        };
    }
    
    pub fn conservative() ConstantFoldingConfig {
        return ConstantFoldingConfig{
            .enable_algebraic_simplification = false,
            .max_folding_depth = 5,
        };
    }
};

/// Constant folding statistics
const ConstantFoldingStats = struct {
    constants_folded: u32 = 0,
    instruction_eliminations: u32 = 0,
    expression_simplifications: u32 = 0,
    folding_time_ns: i64 = 0,
    
    pub fn init() ConstantFoldingStats {
        return ConstantFoldingStats{};
    }
};

test "constant folder initialization" {
    const allocator = std.testing.allocator;
    
    var folder = try ConstantFolder.init(allocator);
    defer folder.deinit();
    
    try std.testing.expect(folder.config.fold_arithmetic == true);
    try std.testing.expect(folder.constant_cache.count() == 0);
}

test "constant folding config variations" {
    const default_config = ConstantFoldingConfig.default();
    const aggressive_config = ConstantFoldingConfig.aggressive();
    const conservative_config = ConstantFoldingConfig.conservative();
    
    try std.testing.expect(default_config.fold_arithmetic == true);
    try std.testing.expect(aggressive_config.max_folding_depth > default_config.max_folding_depth);
    try std.testing.expect(conservative_config.enable_algebraic_simplification == false);
}
