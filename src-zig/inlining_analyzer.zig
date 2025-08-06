const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
});

const ProfileData = @import("optimization_engine.zig").ProfileData;

/// Advanced function inlining analyzer with intelligent heuristics
pub const InliningAnalyzer = struct {
    allocator: Allocator,
    
    // Inlining heuristics configuration
    config: InliningConfig,
    
    // Function analysis cache
    function_analysis_cache: HashMap(c.LLVMValueRef, FunctionAnalysis, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage),
    
    // Call site analysis cache
    call_site_cache: HashMap(CallSiteKey, CallSiteAnalysis, CallSiteContext, std.hash_map.default_max_load_percentage),

    pub fn init(allocator: Allocator) !InliningAnalyzer {
        return InliningAnalyzer{
            .allocator = allocator,
            .config = InliningConfig.default(),
            .function_analysis_cache = HashMap(c.LLVMValueRef, FunctionAnalysis, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage).init(allocator),
            .call_site_cache = HashMap(CallSiteKey, CallSiteAnalysis, CallSiteContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }

    pub fn deinit(self: *InliningAnalyzer) void {
        self.function_analysis_cache.deinit();
        self.call_site_cache.deinit();
    }

    /// Analyze module for inlining opportunities
    pub fn analyzeModule(self: *InliningAnalyzer, module: c.LLVMModuleRef, pgo_data: ?ProfileData) !ArrayList(InliningDecision) {
        var decisions = ArrayList(InliningDecision).init(self.allocator);
        
        // First pass: Analyze all functions
        try self.analyzeFunctions(module);
        
        // Second pass: Analyze call sites
        var function = c.LLVMGetFirstFunction(module);
        while (function != null) {
            try self.analyzeCallSitesInFunction(function.?, &decisions, pgo_data);
            function = c.LLVMGetNextFunction(function.?);
        }
        
        // Third pass: Apply inlining heuristics
        try self.applyInliningHeuristics(&decisions, pgo_data);
        
        std.debug.print("✅ Inlining analysis: {} decisions made\n", .{decisions.items.len});
        
        return decisions;
    }

    /// Analyze all functions in the module
    fn analyzeFunctions(self: *InliningAnalyzer, module: c.LLVMModuleRef) !void {
        var function = c.LLVMGetFirstFunction(module);
        
        while (function != null) {
            const analysis = try self.analyzeFunctionForInlining(function.?);
            try self.function_analysis_cache.put(function.?, analysis);
            function = c.LLVMGetNextFunction(function.?);
        }
    }

    /// Analyze a function for inlining characteristics
    fn analyzeFunctionForInlining(self: *InliningAnalyzer, function: c.LLVMValueRef) !FunctionAnalysis {
        var analysis = FunctionAnalysis{
            .instruction_count = 0,
            .basic_block_count = 0,
            .call_count = 0,
            .loop_count = 0,
            .has_recursion = false,
            .has_exception_handling = false,
            .has_variable_args = false,
            .estimated_cost = 0,
            .complexity_score = 0.0,
        };

        // Count basic blocks
        var basic_block = c.LLVMGetFirstBasicBlock(function);
        while (basic_block != null) {
            analysis.basic_block_count += 1;
            
            // Count instructions in this block
            var instruction = c.LLVMGetFirstInstruction(basic_block.?);
            while (instruction != null) {
                analysis.instruction_count += 1;
                
                // Check for calls
                if (c.LLVMGetInstructionOpcode(instruction.?) == c.LLVMCall) {
                    analysis.call_count += 1;
                    
                    // Check for recursion
                    const called_function = c.LLVMGetCalledValue(instruction.?);
                    if (called_function == function) {
                        analysis.has_recursion = true;
                    }
                }
                
                instruction = c.LLVMGetNextInstruction(instruction.?);
            }
            
            basic_block = c.LLVMGetNextBasicBlock(basic_block.?);
        }

        // Check for variable arguments
        if (c.LLVMIsFunctionVarArg(c.LLVMGlobalGetValueType(function)) != 0) {
            analysis.has_variable_args = true;
        }

        // Calculate estimated inlining cost
        analysis.estimated_cost = self.calculateInliningCost(&analysis);
        
        // Calculate complexity score
        analysis.complexity_score = self.calculateComplexityScore(&analysis);

        return analysis;
    }

    /// Analyze call sites in a function
    fn analyzeCallSitesInFunction(self: *InliningAnalyzer, function: c.LLVMValueRef, decisions: *ArrayList(InliningDecision), pgo_data: ?ProfileData) !void {
        var basic_block = c.LLVMGetFirstBasicBlock(function);
        
        while (basic_block != null) {
            var instruction = c.LLVMGetFirstInstruction(basic_block.?);
            
            while (instruction != null) {
                if (c.LLVMGetInstructionOpcode(instruction.?) == c.LLVMCall) {
                    const called_function = c.LLVMGetCalledValue(instruction.?);
                    
                    if (c.LLVMIsAFunction(called_function) != null) {
                        try self.analyzeCallSite(function, called_function, instruction.?, decisions, pgo_data);
                    }
                }
                
                instruction = c.LLVMGetNextInstruction(instruction.?);
            }
            
            basic_block = c.LLVMGetNextBasicBlock(basic_block.?);
        }
    }

    /// Analyze a specific call site
    fn analyzeCallSite(self: *InliningAnalyzer, caller: c.LLVMValueRef, callee: c.LLVMValueRef, call_site: c.LLVMValueRef, decisions: *ArrayList(InliningDecision), pgo_data: ?ProfileData) !void {
        const caller_analysis = self.function_analysis_cache.get(caller) orelse return;
        const callee_analysis = self.function_analysis_cache.get(callee) orelse return;

        // Create call site key for caching
        const call_site_key = CallSiteKey{
            .caller = caller,
            .callee = callee,
            .call_site = call_site,
        };

        // Check cache
        if (self.call_site_cache.get(call_site_key)) |cached_analysis| {
            const decision = InliningDecision{
                .caller = caller,
                .callee = callee,
                .call_site = call_site,
                .should_inline = cached_analysis.should_inline,
                .confidence = cached_analysis.confidence,
                .estimated_benefit = cached_analysis.estimated_benefit,
                .estimated_size_reduction = cached_analysis.estimated_size_reduction,
                .reason = cached_analysis.reason,
            };
            try decisions.append(decision);
            return;
        }

        // Perform call site analysis
        var call_site_analysis = CallSiteAnalysis{
            .should_inline = false,
            .confidence = 0.0,
            .estimated_benefit = 0.0,
            .estimated_size_reduction = 0,
            .call_frequency = 1,
            .reason = "not analyzed",
        };

        // Get call frequency from profile data
        if (pgo_data) |profile| {
            const caller_name = c.LLVMGetValueName(caller);
            const caller_name_str = std.mem.span(caller_name);
            if (profile.call_frequencies.get(caller_name_str)) |frequency| {
                call_site_analysis.call_frequency = frequency;
            }
        }

        // Apply inlining heuristics
        const should_inline = try self.shouldInlineFunction(caller_analysis, callee_analysis, call_site_analysis);
        
        call_site_analysis.should_inline = should_inline;
        call_site_analysis.confidence = self.calculateInliningConfidence(caller_analysis, callee_analysis, call_site_analysis);
        call_site_analysis.estimated_benefit = self.calculateInliningBenefit(caller_analysis, callee_analysis, call_site_analysis);
        call_site_analysis.estimated_size_reduction = self.calculateSizeReduction(caller_analysis, callee_analysis);

        if (should_inline) {
            call_site_analysis.reason = "beneficial inlining";
        } else {
            call_site_analysis.reason = self.getInliningRejectionReason(caller_analysis, callee_analysis);
        }

        // Cache the analysis
        try self.call_site_cache.put(call_site_key, call_site_analysis);

        // Create inlining decision
        const decision = InliningDecision{
            .caller = caller,
            .callee = callee,
            .call_site = call_site,
            .should_inline = call_site_analysis.should_inline,
            .confidence = call_site_analysis.confidence,
            .estimated_benefit = call_site_analysis.estimated_benefit,
            .estimated_size_reduction = call_site_analysis.estimated_size_reduction,
            .reason = call_site_analysis.reason,
        };

        try decisions.append(decision);
    }

    /// Apply advanced inlining heuristics
    fn applyInliningHeuristics(self: *InliningAnalyzer, decisions: *ArrayList(InliningDecision), pgo_data: ?ProfileData) !void {
        _ = self;
        _ = pgo_data;
        
        // Sort decisions by estimated benefit (highest first)
        std.sort.sort(InliningDecision, decisions.items, {}, compareInliningDecisions);
        
        // Apply budget constraints and conflicts resolution
        var inlining_budget: u32 = self.config.total_inlining_budget;
        
        for (decisions.items) |*decision| {
            if (decision.should_inline) {
                const cost = @as(u32, @intFromFloat(decision.estimated_benefit * 100));
                if (cost <= inlining_budget) {
                    inlining_budget -= cost;
                } else {
                    decision.should_inline = false;
                    decision.reason = "inlining budget exceeded";
                }
            }
        }
    }

    /// Calculate inlining cost for a function
    fn calculateInliningCost(self: *InliningAnalyzer, analysis: *const FunctionAnalysis) u32 {
        var cost: u32 = 0;
        
        // Base cost from instruction count
        cost += analysis.instruction_count * self.config.instruction_cost_weight;
        
        // Additional cost for complexity
        cost += @as(u32, @intFromFloat(analysis.complexity_score * 50));
        
        // Penalty for calls (potential for further inlining)
        cost += analysis.call_count * self.config.call_cost_penalty;
        
        // Penalty for loops
        cost += analysis.loop_count * self.config.loop_cost_penalty;
        
        // High penalty for recursion
        if (analysis.has_recursion) {
            cost += self.config.recursion_penalty;
        }
        
        // Penalty for exception handling
        if (analysis.has_exception_handling) {
            cost += self.config.exception_penalty;
        }
        
        return cost;
    }

    /// Calculate complexity score for a function
    fn calculateComplexityScore(self: *InliningAnalyzer, analysis: *const FunctionAnalysis) f64 {
        _ = self;
        
        var complexity: f64 = 1.0;
        
        // Cyclomatic complexity approximation
        complexity += @as(f64, @floatFromInt(analysis.basic_block_count)) * 0.5;
        
        // Call complexity
        complexity += @as(f64, @floatFromInt(analysis.call_count)) * 0.3;
        
        // Loop complexity
        complexity += @as(f64, @floatFromInt(analysis.loop_count)) * 1.5;
        
        return complexity;
    }

    /// Determine if a function should be inlined
    fn shouldInlineFunction(self: *InliningAnalyzer, caller_analysis: FunctionAnalysis, callee_analysis: FunctionAnalysis, call_site_analysis: CallSiteAnalysis) !bool {
        // Never inline recursive functions
        if (callee_analysis.has_recursion) {
            return false;
        }
        
        // Never inline functions with variable arguments
        if (callee_analysis.has_variable_args) {
            return false;
        }
        
        // Check cost threshold
        if (callee_analysis.estimated_cost > self.config.max_inlining_cost) {
            return false;
        }
        
        // Check instruction count threshold
        if (callee_analysis.instruction_count > self.config.max_instruction_count) {
            return false;
        }
        
        // Always inline trivial functions
        if (callee_analysis.instruction_count <= self.config.trivial_function_threshold) {
            return true;
        }
        
        // Check caller growth limit
        const combined_size = caller_analysis.instruction_count + callee_analysis.instruction_count;
        if (combined_size > self.config.max_caller_growth) {
            return false;
        }
        
        // Consider call frequency
        if (call_site_analysis.call_frequency >= self.config.hot_call_threshold) {
            return true;
        }
        
        // Consider complexity
        if (callee_analysis.complexity_score <= self.config.max_complexity_for_inlining) {
            return true;
        }
        
        return false;
    }

    /// Calculate confidence in inlining decision
    fn calculateInliningConfidence(self: *InliningAnalyzer, caller_analysis: FunctionAnalysis, callee_analysis: FunctionAnalysis, call_site_analysis: CallSiteAnalysis) f64 {
        _ = self;
        _ = caller_analysis;
        
        var confidence: f64 = 0.5; // Base confidence
        
        // High confidence for trivial functions
        if (callee_analysis.instruction_count <= 5) {
            confidence += 0.4;
        }
        
        // Confidence based on call frequency
        if (call_site_analysis.call_frequency > 100) {
            confidence += 0.3;
        } else if (call_site_analysis.call_frequency > 10) {
            confidence += 0.2;
        }
        
        // Confidence based on complexity
        if (callee_analysis.complexity_score < 2.0) {
            confidence += 0.2;
        }
        
        // Reduce confidence for complex functions
        if (callee_analysis.complexity_score > 5.0) {
            confidence -= 0.3;
        }
        
        return @max(0.0, @min(1.0, confidence));
    }

    /// Calculate estimated benefit of inlining
    fn calculateInliningBenefit(self: *InliningAnalyzer, caller_analysis: FunctionAnalysis, callee_analysis: FunctionAnalysis, call_site_analysis: CallSiteAnalysis) f64 {
        _ = self;
        _ = caller_analysis;
        
        var benefit: f64 = 0.0;
        
        // Base benefit from eliminating call overhead
        benefit += 1.0;
        
        // Additional benefit based on call frequency
        benefit += @as(f64, @floatFromInt(call_site_analysis.call_frequency)) * 0.01;
        
        // Benefit from potential optimization opportunities
        if (callee_analysis.instruction_count <= 10) {
            benefit += 2.0; // Small functions benefit more
        }
        
        // Benefit from eliminating call setup/teardown
        benefit += @as(f64, @floatFromInt(callee_analysis.call_count)) * 0.1;
        
        return benefit;
    }

    /// Calculate estimated size reduction from inlining
    fn calculateSizeReduction(self: *InliningAnalyzer, caller_analysis: FunctionAnalysis, callee_analysis: FunctionAnalysis) i32 {
        _ = self;
        _ = caller_analysis;
        
        // Estimate size reduction (negative means size increase)
        var size_change: i32 = 0;
        
        // Cost of call instruction elimination (positive)
        size_change += 4; // Typical call instruction size
        
        // Cost of function body duplication (negative)
        size_change -= @as(i32, @intCast(callee_analysis.instruction_count * 2)); // Approximate instruction size
        
        return size_change;
    }

    /// Get reason for rejecting inlining
    fn getInliningRejectionReason(self: *InliningAnalyzer, caller_analysis: FunctionAnalysis, callee_analysis: FunctionAnalysis) []const u8 {
        if (callee_analysis.has_recursion) {
            return "recursive function";
        }
        
        if (callee_analysis.has_variable_args) {
            return "variable arguments";
        }
        
        if (callee_analysis.estimated_cost > self.config.max_inlining_cost) {
            return "cost too high";
        }
        
        if (callee_analysis.instruction_count > self.config.max_instruction_count) {
            return "too many instructions";
        }
        
        const combined_size = caller_analysis.instruction_count + callee_analysis.instruction_count;
        if (combined_size > self.config.max_caller_growth) {
            return "caller would grow too large";
        }
        
        if (callee_analysis.complexity_score > self.config.max_complexity_for_inlining) {
            return "too complex";
        }
        
        return "not beneficial";
    }
};

/// Inlining configuration
const InliningConfig = struct {
    max_inlining_cost: u32 = 200,
    max_instruction_count: u32 = 50,
    trivial_function_threshold: u32 = 5,
    max_caller_growth: u32 = 500,
    max_complexity_for_inlining: f64 = 3.0,
    hot_call_threshold: u64 = 10,
    total_inlining_budget: u32 = 10000,
    instruction_cost_weight: u32 = 2,
    call_cost_penalty: u32 = 10,
    loop_cost_penalty: u32 = 20,
    recursion_penalty: u32 = 1000,
    exception_penalty: u32 = 100,
    
    pub fn default() InliningConfig {
        return InliningConfig{};
    }
    
    pub fn aggressive() InliningConfig {
        return InliningConfig{
            .max_inlining_cost = 400,
            .max_instruction_count = 100,
            .max_caller_growth = 1000,
            .max_complexity_for_inlining = 5.0,
            .hot_call_threshold = 5,
        };
    }
    
    pub fn conservative() InliningConfig {
        return InliningConfig{
            .max_inlining_cost = 100,
            .max_instruction_count = 25,
            .max_caller_growth = 250,
            .max_complexity_for_inlining = 2.0,
            .hot_call_threshold = 20,
        };
    }
};

/// Function analysis result
const FunctionAnalysis = struct {
    instruction_count: u32,
    basic_block_count: u32,
    call_count: u32,
    loop_count: u32,
    has_recursion: bool,
    has_exception_handling: bool,
    has_variable_args: bool,
    estimated_cost: u32,
    complexity_score: f64,
};

/// Call site analysis result
const CallSiteAnalysis = struct {
    should_inline: bool,
    confidence: f64,
    estimated_benefit: f64,
    estimated_size_reduction: i32,
    call_frequency: u64,
    reason: []const u8,
};

/// Call site key for caching
const CallSiteKey = struct {
    caller: c.LLVMValueRef,
    callee: c.LLVMValueRef,
    call_site: c.LLVMValueRef,
};

/// Call site context for HashMap
const CallSiteContext = struct {
    pub fn hash(self: @This(), key: CallSiteKey) u64 {
        _ = self;
        var hasher = std.hash_map.DefaultHasher.init();
        hasher.update(std.mem.asBytes(&key.caller));
        hasher.update(std.mem.asBytes(&key.callee));
        hasher.update(std.mem.asBytes(&key.call_site));
        return hasher.final();
    }
    
    pub fn eql(self: @This(), a: CallSiteKey, b: CallSiteKey) bool {
        _ = self;
        return a.caller == b.caller and a.callee == b.callee and a.call_site == b.call_site;
    }
};

/// Inlining decision
pub const InliningDecision = struct {
    caller: c.LLVMValueRef,
    callee: c.LLVMValueRef,
    call_site: c.LLVMValueRef,
    should_inline: bool,
    confidence: f64,
    estimated_benefit: f64,
    estimated_size_reduction: i32,
    reason: []const u8,
};

/// Compare inlining decisions for sorting (highest benefit first)
fn compareInliningDecisions(context: void, a: InliningDecision, b: InliningDecision) bool {
    _ = context;
    return a.estimated_benefit > b.estimated_benefit;
}

test "inlining analyzer initialization" {
    const allocator = std.testing.allocator;
    
    var analyzer = try InliningAnalyzer.init(allocator);
    defer analyzer.deinit();
    
    try std.testing.expect(analyzer.config.max_inlining_cost == 200);
    try std.testing.expect(analyzer.config.trivial_function_threshold == 5);
}

test "inlining config variations" {
    const default_config = InliningConfig.default();
    const aggressive_config = InliningConfig.aggressive();
    const conservative_config = InliningConfig.conservative();
    
    try std.testing.expect(aggressive_config.max_inlining_cost > default_config.max_inlining_cost);
    try std.testing.expect(conservative_config.max_inlining_cost < default_config.max_inlining_cost);
}
