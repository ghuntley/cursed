const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const HashSet = std.HashMap(c.LLVMValueRef, void, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage);

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
});

/// Advanced dead code elimination tracker
/// Identifies and eliminates dead instructions, basic blocks, and functions
pub const DeadCodeTracker = struct {
    allocator: Allocator,
    
    // Sets to track live code
    live_instructions: HashSet,
    live_basic_blocks: HashSet,
    live_functions: HashSet,
    
    // Worklist for analysis
    instruction_worklist: ArrayList(c.LLVMValueRef),
    basic_block_worklist: ArrayList(c.LLVMBasicBlockRef),
    function_worklist: ArrayList(c.LLVMValueRef),
    
    // Analysis configuration
    config: DeadCodeConfig,
    
    // Statistics
    stats: DeadCodeStats,

    pub fn init(allocator: Allocator) !DeadCodeTracker {
        return DeadCodeTracker{
            .allocator = allocator,
            .live_instructions = HashSet.init(allocator),
            .live_basic_blocks = HashSet.init(allocator),
            .live_functions = HashSet.init(allocator),
            .instruction_worklist = ArrayList(c.LLVMValueRef).init(allocator),
            .basic_block_worklist = ArrayList(c.LLVMBasicBlockRef).init(allocator),
            .function_worklist = ArrayList(c.LLVMValueRef).init(allocator),
            .config = DeadCodeConfig.default(),
            .stats = DeadCodeStats.init(),
        };
    }

    pub fn deinit(self: *DeadCodeTracker) void {
        self.live_instructions.deinit();
        self.live_basic_blocks.deinit();
        self.live_functions.deinit();
        self.instruction_worklist.deinit();
        self.basic_block_worklist.deinit();
        self.function_worklist.deinit();
    }

    /// Find dead instructions in the module
    pub fn findDeadCode(self: *DeadCodeTracker, module: c.LLVMModuleRef) !ArrayList(c.LLVMValueRef) {
        // Clear previous analysis
        self.live_instructions.clearRetainingCapacity();
        self.live_basic_blocks.clearRetainingCapacity();
        self.live_functions.clearRetainingCapacity();
        self.instruction_worklist.clearRetainingCapacity();
        self.basic_block_worklist.clearRetainingCapacity();
        self.function_worklist.clearRetainingCapacity();
        
        // Step 1: Mark initially live code
        try self.markInitiallyLiveCode(module);
        
        // Step 2: Propagate liveness
        try self.propagateLiveness();
        
        // Step 3: Collect dead instructions
        const dead_instructions = try self.collectDeadInstructions(module);
        
        // Update statistics
        self.stats.dead_instructions_found = @as(u32, @intCast(dead_instructions.items.len));
        
        std.debug.print("✅ Dead code analysis: {} dead instructions found\n", .{dead_instructions.items.len});
        
        return dead_instructions;
    }

    /// Find dead functions in the module
    pub fn findDeadFunctions(self: *DeadCodeTracker, module: c.LLVMModuleRef) !ArrayList(c.LLVMValueRef) {
        var dead_functions = ArrayList(c.LLVMValueRef).init(self.allocator);
        
        // Collect all functions
        var function = c.LLVMGetFirstFunction(module);
        while (function != null) {
            // Skip entry points and externally visible functions
            if (!self.isFunctionLive(function.?)) {
                try dead_functions.append(function.?);
            }
            function = c.LLVMGetNextFunction(function.?);
        }
        
        // Update statistics
        self.stats.dead_functions_found = @as(u32, @intCast(dead_functions.items.len));
        
        std.debug.print("✅ Dead function analysis: {} dead functions found\n", .{dead_functions.items.len});
        
        return dead_functions;
    }

    /// Mark initially live code (entry points, externally visible symbols, etc.)
    fn markInitiallyLiveCode(self: *DeadCodeTracker, module: c.LLVMModuleRef) !void {
        // Mark entry point functions as live
        try self.markEntryPointsLive(module);
        
        // Mark externally visible functions as live
        try self.markExternallyVisibleFunctionsLive(module);
        
        // Mark functions with critical side effects as live
        try self.markCriticalFunctionsLive(module);
        
        // Mark global variables with initializers as live
        try self.markGlobalVariablesLive(module);
    }

    /// Mark entry point functions as live
    fn markEntryPointsLive(self: *DeadCodeTracker, module: c.LLVMModuleRef) !void {
        // Look for main function
        if (c.LLVMGetNamedFunction(module, "main")) |main_func| {
            try self.markFunctionLive(main_func);
        }
        
        // Look for CURSED entry points
        const cursed_entry_points = [_][]const u8{
            "cursed_main",
            "_start",
            "cursed_init",
        };
        
        for (cursed_entry_points) |entry_name| {
            if (c.LLVMGetNamedFunction(module, entry_name.ptr)) |entry_func| {
                try self.markFunctionLive(entry_func);
            }
        }
    }

    /// Mark externally visible functions as live
    fn markExternallyVisibleFunctionsLive(self: *DeadCodeTracker, module: c.LLVMModuleRef) !void {
        var function = c.LLVMGetFirstFunction(module);
        
        while (function != null) {
            const linkage = c.LLVMGetLinkage(function.?);
            
            // Keep functions with external linkage
            if (linkage == c.LLVMExternalLinkage or linkage == c.LLVMDLLExportLinkage) {
                try self.markFunctionLive(function.?);
            }
            
            // Keep functions that might be called from outside
            const name = c.LLVMGetValueName(function.?);
            const name_str = std.mem.span(name);
            
            if (self.isCriticalFunctionName(name_str)) {
                try self.markFunctionLive(function.?);
            }
            
            function = c.LLVMGetNextFunction(function.?);
        }
    }

    /// Mark functions with critical side effects as live
    fn markCriticalFunctionsLive(self: *DeadCodeTracker, module: c.LLVMModuleRef) !void {
        var function = c.LLVMGetFirstFunction(module);
        
        while (function != null) {
            if (self.functionHasCriticalSideEffects(function.?)) {
                try self.markFunctionLive(function.?);
            }
            function = c.LLVMGetNextFunction(function.?);
        }
    }

    /// Mark global variables with initializers as live
    fn markGlobalVariablesLive(self: *DeadCodeTracker, module: c.LLVMModuleRef) !void {
        var global = c.LLVMGetFirstGlobal(module);
        
        while (global != null) {
            if (c.LLVMGetInitializer(global.?) != null) {
                // Global has initializer, mark as live
                try self.live_instructions.put(global.?, {});
            }
            global = c.LLVMGetNextGlobal(global.?);
        }
    }

    /// Propagate liveness through the code
    fn propagateLiveness(self: *DeadCodeTracker) !void {
        // Process instruction worklist
        while (self.instruction_worklist.items.len > 0) {
            const instruction = self.instruction_worklist.pop();
            try self.processLiveInstruction(instruction);
        }
        
        // Process basic block worklist
        while (self.basic_block_worklist.items.len > 0) {
            const basic_block = self.basic_block_worklist.pop();
            try self.processLiveBasicBlock(basic_block);
        }
        
        // Process function worklist
        while (self.function_worklist.items.len > 0) {
            const function = self.function_worklist.pop();
            try self.processLiveFunction(function);
        }
    }

    /// Mark a function as live and add to worklist
    fn markFunctionLive(self: *DeadCodeTracker, function: c.LLVMValueRef) !void {
        if (self.live_functions.get(function) == null) {
            try self.live_functions.put(function, {});
            try self.function_worklist.append(function);
        }
    }

    /// Mark an instruction as live and add to worklist
    fn markInstructionLive(self: *DeadCodeTracker, instruction: c.LLVMValueRef) !void {
        if (self.live_instructions.get(instruction) == null) {
            try self.live_instructions.put(instruction, {});
            try self.instruction_worklist.append(instruction);
        }
    }

    /// Mark a basic block as live and add to worklist
    fn markBasicBlockLive(self: *DeadCodeTracker, basic_block: c.LLVMBasicBlockRef) !void {
        const bb_value = c.LLVMBasicBlockAsValue(basic_block);
        if (self.live_basic_blocks.get(bb_value) == null) {
            try self.live_basic_blocks.put(bb_value, {});
            try self.basic_block_worklist.append(basic_block);
        }
    }

    /// Process a live instruction
    fn processLiveInstruction(self: *DeadCodeTracker, instruction: c.LLVMValueRef) !void {
        // Mark the containing basic block as live
        const basic_block = c.LLVMGetInstructionParent(instruction);
        try self.markBasicBlockLive(basic_block);
        
        // Mark operands as live
        const num_operands = c.LLVMGetNumOperands(instruction);
        var i: u32 = 0;
        while (i < num_operands) {
            const operand = c.LLVMGetOperand(instruction, i);
            
            if (c.LLVMIsAInstruction(operand) != null) {
                try self.markInstructionLive(operand);
            } else if (c.LLVMIsAFunction(operand) != null) {
                try self.markFunctionLive(operand);
            }
            
            i += 1;
        }
        
        // Special handling for different instruction types
        const opcode = c.LLVMGetInstructionOpcode(instruction);
        switch (opcode) {
            c.LLVMCall => try self.processCallInstruction(instruction),
            c.LLVMBr, c.LLVMCondBr => try self.processBranchInstruction(instruction),
            c.LLVMSwitch => try self.processSwitchInstruction(instruction),
            else => {},
        }
    }

    /// Process a live basic block
    fn processLiveBasicBlock(self: *DeadCodeTracker, basic_block: c.LLVMBasicBlockRef) !void {
        // Mark the containing function as live
        const function = c.LLVMGetBasicBlockParent(basic_block);
        try self.markFunctionLive(function);
        
        // Mark terminator instruction as live
        const terminator = c.LLVMGetBasicBlockTerminator(basic_block);
        if (terminator != null) {
            try self.markInstructionLive(terminator.?);
        }
    }

    /// Process a live function
    fn processLiveFunction(self: *DeadCodeTracker, function: c.LLVMValueRef) !void {
        // Mark entry basic block as live
        const entry_block = c.LLVMGetEntryBasicBlock(function);
        if (entry_block != null) {
            try self.markBasicBlockLive(entry_block.?);
        }
        
        // Mark function parameters as live
        const num_params = c.LLVMCountParams(function);
        var i: u32 = 0;
        while (i < num_params) {
            const param = c.LLVMGetParam(function, i);
            try self.markInstructionLive(param);
            i += 1;
        }
    }

    /// Process call instruction
    fn processCallInstruction(self: *DeadCodeTracker, call_instruction: c.LLVMValueRef) !void {
        const called_value = c.LLVMGetCalledValue(call_instruction);
        
        if (c.LLVMIsAFunction(called_value) != null) {
            try self.markFunctionLive(called_value);
        }
    }

    /// Process branch instruction
    fn processBranchInstruction(self: *DeadCodeTracker, branch_instruction: c.LLVMValueRef) !void {
        const opcode = c.LLVMGetInstructionOpcode(branch_instruction);
        
        if (opcode == c.LLVMBr) {
            // Unconditional branch
            const target = c.LLVMGetOperand(branch_instruction, 0);
            if (c.LLVMIsABasicBlock(target) != null) {
                try self.markBasicBlockLive(c.LLVMValueAsBasicBlock(target));
            }
        } else if (opcode == c.LLVMCondBr) {
            // Conditional branch
            const true_target = c.LLVMGetOperand(branch_instruction, 2);
            const false_target = c.LLVMGetOperand(branch_instruction, 1);
            
            if (c.LLVMIsABasicBlock(true_target) != null) {
                try self.markBasicBlockLive(c.LLVMValueAsBasicBlock(true_target));
            }
            if (c.LLVMIsABasicBlock(false_target) != null) {
                try self.markBasicBlockLive(c.LLVMValueAsBasicBlock(false_target));
            }
        }
    }

    /// Process switch instruction
    fn processSwitchInstruction(self: *DeadCodeTracker, switch_instruction: c.LLVMValueRef) !void {
        // Mark all switch targets as live
        const num_operands = c.LLVMGetNumOperands(switch_instruction);
        var i: u32 = 1; // Skip condition operand
        while (i < num_operands) {
            const operand = c.LLVMGetOperand(switch_instruction, i);
            if (c.LLVMIsABasicBlock(operand) != null) {
                try self.markBasicBlockLive(c.LLVMValueAsBasicBlock(operand));
            }
            i += 2; // Switch operands come in pairs (value, target)
        }
    }

    /// Collect dead instructions from the module
    fn collectDeadInstructions(self: *DeadCodeTracker, module: c.LLVMModuleRef) !ArrayList(c.LLVMValueRef) {
        var dead_instructions = ArrayList(c.LLVMValueRef).init(self.allocator);
        
        var function = c.LLVMGetFirstFunction(module);
        while (function != null) {
            var basic_block = c.LLVMGetFirstBasicBlock(function.?);
            
            while (basic_block != null) {
                var instruction = c.LLVMGetFirstInstruction(basic_block.?);
                
                while (instruction != null) {
                    const next_instruction = c.LLVMGetNextInstruction(instruction.?);
                    
                    if (!self.isInstructionLive(instruction.?) and self.canEliminateInstruction(instruction.?)) {
                        try dead_instructions.append(instruction.?);
                    }
                    
                    instruction = next_instruction;
                }
                
                basic_block = c.LLVMGetNextBasicBlock(basic_block.?);
            }
            
            function = c.LLVMGetNextFunction(function.?);
        }
        
        return dead_instructions;
    }

    /// Check if an instruction is live
    fn isInstructionLive(self: *DeadCodeTracker, instruction: c.LLVMValueRef) bool {
        return self.live_instructions.get(instruction) != null;
    }

    /// Check if a function is live
    fn isFunctionLive(self: *DeadCodeTracker, function: c.LLVMValueRef) bool {
        return self.live_functions.get(function) != null;
    }

    /// Check if an instruction can be safely eliminated
    fn canEliminateInstruction(self: *DeadCodeTracker, instruction: c.LLVMValueRef) bool {
        _ = self;
        
        const opcode = c.LLVMGetInstructionOpcode(instruction);
        
        // Never eliminate these instruction types
        switch (opcode) {
            c.LLVMRet, c.LLVMBr, c.LLVMCondBr, c.LLVMSwitch, c.LLVMIndirectBr,
            c.LLVMInvoke, c.LLVMUnreachable, c.LLVMResume, c.LLVMCleanupRet,
            c.LLVMCatchRet, c.LLVMCatchSwitch => return false,
            else => {},
        }
        
        // Check for side effects
        if (self.instructionHasSideEffects(instruction)) {
            return false;
        }
        
        return true;
    }

    /// Check if an instruction has side effects
    fn instructionHasSideEffects(self: *DeadCodeTracker, instruction: c.LLVMValueRef) bool {
        _ = self;
        
        const opcode = c.LLVMGetInstructionOpcode(instruction);
        
        // Instructions with side effects
        switch (opcode) {
            c.LLVMStore, c.LLVMCall, c.LLVMInvoke,
            c.LLVMFence, c.LLVMAtomicCmpXchg, c.LLVMAtomicRMW => return true,
            else => return false,
        }
    }

    /// Check if a function has critical side effects
    fn functionHasCriticalSideEffects(self: *DeadCodeTracker, function: c.LLVMValueRef) bool {
        _ = self;
        
        // Check for functions that shouldn't be eliminated
        var basic_block = c.LLVMGetFirstBasicBlock(function);
        while (basic_block != null) {
            var instruction = c.LLVMGetFirstInstruction(basic_block.?);
            
            while (instruction != null) {
                if (self.instructionHasSideEffects(instruction.?)) {
                    return true;
                }
                instruction = c.LLVMGetNextInstruction(instruction.?);
            }
            
            basic_block = c.LLVMGetNextBasicBlock(basic_block.?);
        }
        
        return false;
    }

    /// Check if a function name indicates it's critical
    fn isCriticalFunctionName(self: *DeadCodeTracker, name: []const u8) bool {
        _ = self;
        
        const critical_prefixes = [_][]const u8{
            "llvm.",
            "__",
            "main",
            "init",
            "fini",
            "constructor",
            "destructor",
        };
        
        for (critical_prefixes) |prefix| {
            if (std.mem.startsWith(u8, name, prefix)) {
                return true;
            }
        }
        
        return false;
    }
};

/// Dead code elimination configuration
const DeadCodeConfig = struct {
    eliminate_dead_functions: bool = true,
    eliminate_dead_globals: bool = true,
    aggressive_elimination: bool = false,
    preserve_debug_info: bool = true,
    
    pub fn default() DeadCodeConfig {
        return DeadCodeConfig{};
    }
    
    pub fn aggressive() DeadCodeConfig {
        return DeadCodeConfig{
            .aggressive_elimination = true,
            .preserve_debug_info = false,
        };
    }
    
    pub fn conservative() DeadCodeConfig {
        return DeadCodeConfig{
            .eliminate_dead_functions = false,
            .aggressive_elimination = false,
        };
    }
};

/// Dead code elimination statistics
const DeadCodeStats = struct {
    dead_instructions_found: u32 = 0,
    dead_functions_found: u32 = 0,
    dead_basic_blocks_found: u32 = 0,
    elimination_time_ns: i64 = 0,
    
    pub fn init() DeadCodeStats {
        return DeadCodeStats{};
    }
};

test "dead code tracker initialization" {
    const allocator = std.testing.allocator;
    
    var tracker = try DeadCodeTracker.init(allocator);
    defer tracker.deinit();
    
    try std.testing.expect(tracker.config.eliminate_dead_functions == true);
    try std.testing.expect(tracker.live_instructions.count() == 0);
}

test "dead code config variations" {
    const default_config = DeadCodeConfig.default();
    const aggressive_config = DeadCodeConfig.aggressive();
    const conservative_config = DeadCodeConfig.conservative();
    
    try std.testing.expect(default_config.eliminate_dead_functions == true);
    try std.testing.expect(aggressive_config.aggressive_elimination == true);
    try std.testing.expect(conservative_config.eliminate_dead_functions == false);
}
