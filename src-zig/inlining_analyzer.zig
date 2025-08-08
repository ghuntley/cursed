const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
});

const ProfileData = @import("optimization_engine.zig").ProfileData;

/// Inlining decision for a specific call site
pub const InliningDecision = struct {
    caller: c.LLVMValueRef,
    callee: c.LLVMValueRef,
    call_site: c.LLVMValueRef,
    should_inline: bool,
    estimated_size_reduction: i32,
    cost_benefit_score: f64,
    reason: []const u8,
};

/// Advanced function inlining analyzer with heuristics
pub const InliningAnalyzer = struct {
    allocator: Allocator,
    
    // Thresholds for inlining decisions
    default_threshold: u32 = 225,
    aggressive_threshold: u32 = 325,
    small_function_threshold: u32 = 50,
    
    // Cost tracking
    total_call_sites_analyzed: u32 = 0,
    functions_considered: u32 = 0,
    inlining_decisions_made: u32 = 0,

    pub fn init(allocator: Allocator) !InliningAnalyzer {
        return InliningAnalyzer{
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *InliningAnalyzer) void {
        _ = self;
    }

    /// Analyze module for inlining opportunities
    pub fn analyzeModule(self: *InliningAnalyzer, module: c.LLVMModuleRef, pgo_data: ?ProfileData) !ArrayList(InliningDecision) {
        var decisions = ArrayList(InliningDecision).init(self.allocator);
        
        // Iterate through all functions
        var function = c.LLVMGetFirstFunction(module);
        while (function != null) {
            try self.analyzeFunctionForInlining(function.?, &decisions, pgo_data);
            self.functions_considered += 1;
            function = c.LLVMGetNextFunction(function.?);
        }
        
        std.debug.print("✅ Inlining analysis: {} functions, {} call sites, {} decisions\n",
                       .{ self.functions_considered, self.total_call_sites_analyzed, self.inlining_decisions_made });
        
        return decisions;
    }

    /// Analyze a specific function for inlining opportunities
    fn analyzeFunctionForInlining(self: *InliningAnalyzer, function: c.LLVMValueRef, decisions: *ArrayList(InliningDecision), pgo_data: ?ProfileData) !void {
        // Iterate through basic blocks
        var bb = c.LLVMGetFirstBasicBlock(function);
        while (bb != null) {
            try self.analyzeBasicBlockForCalls(function, bb.?, decisions, pgo_data);
            bb = c.LLVMGetNextBasicBlock(bb.?);
        }
    }

    /// Analyze basic block for function calls
    fn analyzeBasicBlockForCalls(self: *InliningAnalyzer, caller: c.LLVMValueRef, bb: c.LLVMBasicBlockRef, decisions: *ArrayList(InliningDecision), pgo_data: ?ProfileData) !void {
        var instruction = c.LLVMGetFirstInstruction(bb);
        
        while (instruction != null) {
            if (c.LLVMGetInstructionOpcode(instruction.?) == c.LLVMCall) {
                self.total_call_sites_analyzed += 1;
                
                // Get called function
                const called_function = c.LLVMGetCalledValue(instruction.?);
                if (called_function != null and c.LLVMIsAFunction(called_function.?) != null) {
                    const decision = try self.analyzeCallSite(caller, called_function.?, instruction.?, pgo_data);
                    try decisions.append(decision);
                    self.inlining_decisions_made += 1;
                }
            }
            instruction = c.LLVMGetNextInstruction(instruction.?);
        }
    }

    /// Analyze specific call site for inlining
    fn analyzeCallSite(self: *InliningAnalyzer, caller: c.LLVMValueRef, callee: c.LLVMValueRef, call_site: c.LLVMValueRef, pgo_data: ?ProfileData) !InliningDecision {
        // Calculate function characteristics
        const callee_size = self.calculateFunctionSize(callee);
        const callee_complexity = self.calculateFunctionComplexity(callee);
        const call_frequency = self.getCallFrequency(caller, callee, pgo_data);
        
        // Calculate cost-benefit score
        const cost_benefit = self.calculateCostBenefit(callee_size, callee_complexity, call_frequency);
        
        // Determine if should inline
        var should_inline = false;
        var reason: []const u8 = "default";
        
        // Apply inlining heuristics
        if (callee_size <= self.small_function_threshold) {
            should_inline = true;
            reason = "small function";
        } else if (call_frequency >= 0.8 and callee_size <= self.aggressive_threshold) {
            should_inline = true;
            reason = "hot call site";
        } else if (callee_complexity <= 5 and callee_size <= self.default_threshold) {
            should_inline = true;
            reason = "simple function";
        } else if (self.isLeafFunction(callee) and callee_size <= self.default_threshold) {
            should_inline = true;
            reason = "leaf function";
        } else if (self.hasOnlyOneCallSite(callee) and callee_size <= self.aggressive_threshold) {
            should_inline = true;
            reason = "single call site";
        } else {
            should_inline = false;
            reason = "too complex/large";
        }
        
        // Apply profile-guided adjustments
        if (pgo_data != null) {
            should_inline = self.applyPGOHeuristics(caller, callee, should_inline, pgo_data.?);
        }
        
        // Calculate estimated size impact
        const size_reduction = if (should_inline) @as(i32, @intCast(callee_size)) * -1 else 0;
        
        return InliningDecision{
            .caller = caller,
            .callee = callee,
            .call_site = call_site,
            .should_inline = should_inline,
            .estimated_size_reduction = size_reduction,
            .cost_benefit_score = cost_benefit,
            .reason = reason,
        };
    }

    /// Calculate function size in instructions
    fn calculateFunctionSize(self: *InliningAnalyzer, function: c.LLVMValueRef) u32 {
        _ = self;
        var size: u32 = 0;
        
        var bb = c.LLVMGetFirstBasicBlock(function);
        while (bb != null) {
            var instruction = c.LLVMGetFirstInstruction(bb.?);
            while (instruction != null) {
                size += 1;
                instruction = c.LLVMGetNextInstruction(instruction.?);
            }
            bb = c.LLVMGetNextBasicBlock(bb.?);
        }
        
        return size;
    }

    /// Calculate function complexity (control flow complexity)
    fn calculateFunctionComplexity(self: *InliningAnalyzer, function: c.LLVMValueRef) u32 {
        _ = self;
        var complexity: u32 = 1; // Base complexity
        
        var bb = c.LLVMGetFirstBasicBlock(function);
        while (bb != null) {
            // Count control flow instructions
            var instruction = c.LLVMGetFirstInstruction(bb.?);
            while (instruction != null) {
                const opcode = c.LLVMGetInstructionOpcode(instruction.?);
                switch (opcode) {
                    c.LLVMBr => {
                        if (c.LLVMIsConditional(instruction.?) != 0) complexity += 1;
                    },
                    c.LLVMSwitch => complexity += 2,
                    c.LLVMCall => complexity += 1,
                    c.LLVMInvoke => complexity += 2,
                    else => {},
                }
                instruction = c.LLVMGetNextInstruction(instruction.?);
            }
            bb = c.LLVMGetNextBasicBlock(bb.?);
        }
        
        return complexity;
    }

    /// Get call frequency from profile data
    fn getCallFrequency(self: *InliningAnalyzer, caller: c.LLVMValueRef, callee: c.LLVMValueRef, pgo_data: ?ProfileData) f64 {
        _ = self;
        _ = caller;
        _ = callee;
        
        if (pgo_data == null) return 0.5; // Default frequency
        
        // TODO: Implement actual profile data lookup
        return 0.5;
    }

    /// Calculate cost-benefit score for inlining
    fn calculateCostBenefit(self: *InliningAnalyzer, size: u32, complexity: u32, frequency: f64) f64 {
        _ = self;
        
        // Benefit increases with call frequency and decreases with size/complexity
        const benefit = frequency * 100.0;
        const cost = @as(f64, @floatFromInt(size)) + (@as(f64, @floatFromInt(complexity)) * 2.0);
        
        return benefit / cost;
    }

    /// Check if function is a leaf function (no calls)
    fn isLeafFunction(self: *InliningAnalyzer, function: c.LLVMValueRef) bool {
        _ = self;
        
        var bb = c.LLVMGetFirstBasicBlock(function);
        while (bb != null) {
            var instruction = c.LLVMGetFirstInstruction(bb.?);
            while (instruction != null) {
                if (c.LLVMGetInstructionOpcode(instruction.?) == c.LLVMCall) {
                    return false;
                }
                instruction = c.LLVMGetNextInstruction(instruction.?);
            }
            bb = c.LLVMGetNextBasicBlock(bb.?);
        }
        
        return true;
    }

    /// Check if function has only one call site
    fn hasOnlyOneCallSite(self: *InliningAnalyzer, function: c.LLVMValueRef) bool {
        _ = self;
        
        // Count users of the function
        var use = c.LLVMGetFirstUse(function);
        var call_count: u32 = 0;
        
        while (use != null) {
            const user = c.LLVMGetUser(use.?);
            if (c.LLVMGetInstructionOpcode(user) == c.LLVMCall) {
                call_count += 1;
                if (call_count > 1) return false;
            }
            use = c.LLVMGetNextUse(use.?);
        }
        
        return call_count == 1;
    }

    /// Apply profile-guided optimization heuristics
    fn applyPGOHeuristics(self: *InliningAnalyzer, caller: c.LLVMValueRef, callee: c.LLVMValueRef, current_decision: bool, pgo_data: ProfileData) bool {
        _ = self;
        _ = caller;
        _ = callee;
        _ = pgo_data;
        
        // TODO: Implement actual PGO logic
        return current_decision;
    }

    /// Set inlining thresholds
    pub fn setThresholds(self: *InliningAnalyzer, default: u32, aggressive: u32, small: u32) void {
        self.default_threshold = default;
        self.aggressive_threshold = aggressive;
        self.small_function_threshold = small;
    }
};

test "inlining analyzer initialization" {
    const allocator = std.testing.allocator;
    
    var analyzer = try InliningAnalyzer.init(allocator);
    defer analyzer.deinit();
    
    try std.testing.expect(analyzer.default_threshold == 225);
    try std.testing.expect(analyzer.aggressive_threshold == 325);
}
