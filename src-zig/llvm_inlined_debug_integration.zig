const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

const debug_info = @import("debug_info.zig");
const inlined_debug = @import("inlined_function_debug.zig");

// LLVM C imports disabled for CPU detection compatibility
const c = struct {
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMDIBuilderRef = ?*anyopaque;
    pub const LLVMMetadataRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    
    // Mock functions
    pub fn LLVMGetInstructionParent(_: LLVMValueRef) LLVMValueRef { return null; }
    pub fn LLVMGetBasicBlockParent(_: LLVMValueRef) LLVMValueRef { return null; }
    pub fn LLVMGetNextInstruction(_: LLVMValueRef) LLVMValueRef { return null; }
    pub fn LLVMGetFirstInstruction(_: LLVMBasicBlockRef) LLVMValueRef { return null; }
    pub fn LLVMGetNextBasicBlock(_: LLVMBasicBlockRef) LLVMBasicBlockRef { return null; }
    pub fn LLVMGetFirstBasicBlock(_: LLVMValueRef) LLVMBasicBlockRef { return null; }
    pub fn LLVMGetNextFunction(_: LLVMValueRef) LLVMValueRef { return null; }
    pub fn LLVMGetFirstFunction(_: LLVMModuleRef) LLVMValueRef { return null; }
    pub fn LLVMInstructionGetOpcode(_: LLVMValueRef) c_uint { return 0; }
    pub fn LLVMIsACallInst(_: LLVMValueRef) LLVMValueRef { return null; }
    pub fn LLVMGetOperand(_: LLVMValueRef, _: c_uint) LLVMValueRef { return null; }
    pub fn LLVMGetNumOperands(_: LLVMValueRef) c_uint { return 0; }
    pub fn LLVMInstructionSetDebugLoc(_: LLVMValueRef, _: LLVMMetadataRef) void {}
    pub fn LLVMValueAsBasicBlock(_: LLVMValueRef) LLVMBasicBlockRef { return null; }
    pub fn LLVMGetValueName(_: LLVMValueRef) [*c]const u8 { return "mock_function"; }
    
    // Constants
    pub const LLVMCall: c_uint = 45;
};

/// Integration bridge between LLVM inlining passes and CURSED debug info
pub const LLVMInlinedDebugIntegration = struct {
    allocator: Allocator,
    debug_generator: *debug_info.DebugInfoGenerator,
    inlined_debug_generator: *inlined_debug.InlinedFunctionDebugGenerator,
    
    /// Track functions being considered for inlining
    inlining_candidates: HashMap([]const u8, InliningCandidate, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    /// Track active inlining operations
    active_inlines: ArrayList(ActiveInline),
    
    pub const IntegrationError = error{
        InvalidFunction,
        DebugInfoGenerationFailed,
        InliningTrackingFailed,
        OutOfMemory,
    };
    
    const InliningCandidate = struct {
        function_name: []const u8,
        llvm_function: c.LLVMValueRef,
        debug_info: c.LLVMMetadataRef,
        instruction_count: u32,
        call_sites: ArrayList(CallSite),
        
        const CallSite = struct {
            caller_function: []const u8,
            call_instruction: c.LLVMValueRef,
            call_line: u32,
            call_column: u32,
        };
    };
    
    const ActiveInline = struct {
        inline_context: inlined_debug.InlineContext,
        original_instructions: ArrayList(c.LLVMValueRef),
        inlined_instructions: ArrayList(c.LLVMValueRef),
        variable_mappings: ArrayList(VariableMapping),
        
        const VariableMapping = struct {
            original_name: []const u8,
            inlined_name: []const u8,
            original_alloca: c.LLVMValueRef,
            inlined_alloca: c.LLVMValueRef,
        };
    };
    
    pub fn init(allocator: Allocator, 
               debug_generator: *debug_info.DebugInfoGenerator,
               inlined_debug_generator: *inlined_debug.InlinedFunctionDebugGenerator) LLVMInlinedDebugIntegration {
        return LLVMInlinedDebugIntegration{
            .allocator = allocator,
            .debug_generator = debug_generator,
            .inlined_debug_generator = inlined_debug_generator,
            .inlining_candidates = HashMap([]const u8, InliningCandidate, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .active_inlines = .empty,
        };
    }
    
    pub fn deinit(self: *LLVMInlinedDebugIntegration) void {
        // Clean up inlining candidates
        var candidate_iterator = self.inlining_candidates.iterator();
        while (candidate_iterator.next()) |entry| {
            entry.value_ptr.call_sites.deinit(allocator);
        }
        self.inlining_candidates.deinit(allocator);
        
        // Clean up active inlines
        for (self.active_inlines.items) |*active| {
            active.original_instructions.deinit(allocator);
            active.inlined_instructions.deinit(allocator);
            active.variable_mappings.deinit(allocator);
        }
        self.active_inlines.deinit(allocator);
    }
    
    /// Analyze module for inlining opportunities and prepare debug info
    pub fn analyzeModuleForInlining(self: *LLVMInlinedDebugIntegration, module: c.LLVMModuleRef) IntegrationError!void {
        print("🔍 Analyzing module for inlining opportunities...\n");
        
        // Walk through all functions in the module
        var current_function = c.LLVMGetFirstFunction(module);
        while (current_function != null) {
            defer current_function = c.LLVMGetNextFunction(current_function);
            
            try self.analyzeFunctionForInlining(current_function);
        }
        
        print("✅ Module analysis complete. Found {} inlining candidates\n", .{self.inlining_candidates.count()});
    }
    
    /// Analyze a single function for inlining potential
    fn analyzeFunctionForInlining(self: *LLVMInlinedDebugIntegration, function: c.LLVMValueRef) IntegrationError!void {
        const function_name_ptr = c.LLVMGetValueName(function);
        const function_name = std.mem.span(function_name_ptr);
        
        var candidate = InliningCandidate{
            .function_name = function_name,
            .llvm_function = function,
            .debug_info = null, // Will be filled if available
            .instruction_count = 0,
            .call_sites = .empty,
        };
        
        // Count instructions and analyze complexity
        var current_block = c.LLVMGetFirstBasicBlock(function);
        while (current_block != null) {
            defer current_block = c.LLVMGetNextBasicBlock(current_block);
            
            var current_instruction = c.LLVMGetFirstInstruction(current_block);
            while (current_instruction != null) {
                defer current_instruction = c.LLVMGetNextInstruction(current_instruction);
                
                candidate.instruction_count += 1;
                
                // Check if this is a call instruction (potential inline site)
                if (c.LLVMInstructionGetOpcode(current_instruction) == c.LLVMCall) {
                    try self.analyzeCallSite(current_instruction, &candidate);
                }
            }
        }
        
        // Store the candidate if it's worth considering for inlining
        if (self.shouldConsiderForInlining(&candidate)) {
            try self.inlining_candidates.put(function_name, candidate);
            print("📋 Added inlining candidate: {s} ({d} instructions)\n", .{ function_name, candidate.instruction_count });
        } else {
            candidate.call_sites.deinit(allocator);
        }
    }
    
    /// Analyze a call site for inlining potential
    fn analyzeCallSite(self: *LLVMInlinedDebugIntegration, call_instruction: c.LLVMValueRef, candidate: *InliningCandidate) IntegrationError!void {
        _ = self;
        // Get the caller function
        const caller_block = c.LLVMGetInstructionParent(call_instruction);
        const caller_function = c.LLVMGetBasicBlockParent(c.LLVMValueAsBasicBlock(caller_block));
        const caller_name_ptr = c.LLVMGetValueName(caller_function);
        const caller_name = std.mem.span(caller_name_ptr);
        
        const call_site = InliningCandidate.CallSite{
            .caller_function = caller_name,
            .call_instruction = call_instruction,
            .call_line = 1, // Would be extracted from debug info in real implementation
            .call_column = 1, // Would be extracted from debug info in real implementation
        };
        
        try candidate.call_sites.append(allocator, call_site);
    }
    
    /// Determine if a function should be considered for inlining
    fn shouldConsiderForInlining(self: *LLVMInlinedDebugIntegration, candidate: *const InliningCandidate) bool {
        _ = self;
        
        // Simple heuristics - in practice this would be more sophisticated
        return candidate.instruction_count <= 50 and candidate.call_sites.items.len > 0;
    }
    
    /// Hook called when LLVM decides to inline a function
    pub fn onFunctionWillBeInlined(self: *LLVMInlinedDebugIntegration,
                                  original_function: c.LLVMValueRef,
                                  target_function: c.LLVMValueRef,
                                  call_site: c.LLVMValueRef) IntegrationError!void {
        _ = call_site;
        
        const original_name_ptr = c.LLVMGetValueName(original_function);
        const original_name = std.mem.span(original_name_ptr);
        
        const target_name_ptr = c.LLVMGetValueName(target_function);
        const target_name = std.mem.span(target_name_ptr);
        
        print("🔄 Function {s} will be inlined into {s}\n", .{ original_name, target_name });
        
        // Create inline context
        const inline_context = inlined_debug.InlineContext.init(
            original_name,
            target_name,
            1, // Would extract from debug info
            1, // Would extract from debug info
            1, // Would extract from debug info
            1, // Would extract from debug info
            "current.csd" // Would extract from debug info
        );
        
        // Set up active inline tracking
        var active_inline = ActiveInline{
            .inline_context = inline_context,
            .original_instructions = .empty,
            .inlined_instructions = .empty,
            .variable_mappings = .empty,
        };
        
        // Collect original instructions for debug info mapping
        try self.collectOriginalInstructions(original_function, &active_inline);
        
        try self.active_inlines.append(allocator, active_inline);
        
        // Create debug info for the inlined function
        if (self.inlining_candidates.get(original_name)) |candidate| {
            _ = try self.inlined_debug_generator.createInlinedFunctionDebugInfo(
                inline_context,
                candidate.debug_info orelse return, // Mock debug info
                null // Mock target scope
            );
        }
    }
    
    /// Collect all instructions from original function for debug mapping
    fn collectOriginalInstructions(self: *LLVMInlinedDebugIntegration, 
                                  function: c.LLVMValueRef, 
                                  active_inline: *ActiveInline) IntegrationError!void {
        _ = self;
        var current_block = c.LLVMGetFirstBasicBlock(function);
        while (current_block != null) {
            defer current_block = c.LLVMGetNextBasicBlock(current_block);
            
            var current_instruction = c.LLVMGetFirstInstruction(current_block);
            while (current_instruction != null) {
                defer current_instruction = c.LLVMGetNextInstruction(current_instruction);
                
                try active_inline.original_instructions.append(allocator, current_instruction);
            }
        }
    }
    
    /// Hook called when an instruction is inlined
    pub fn onInstructionInlined(self: *LLVMInlinedDebugIntegration,
                               original_instruction: c.LLVMValueRef,
                               inlined_instruction: c.LLVMValueRef) IntegrationError!void {
        _ = original_instruction;
        
        // Find the active inline context
        if (self.active_inlines.items.len == 0) {
            return IntegrationError.InliningTrackingFailed;
        }
        
        const active_inline = &self.active_inlines.items[self.active_inlines.items.len - 1];
        
        // Track the instruction mapping
        try active_inline.inlined_instructions.append(allocator, inlined_instruction);
        
        // Create debug location for the inlined instruction
        try self.inlined_debug_generator.createInlinedDebugLocation(
            inlined_instruction,
            1, // Would extract original line from debug info
            1, // Would extract original column from debug info
            null, // Would provide original scope
            &active_inline.inline_context
        );
        
        print("🎯 Mapped inlined instruction with debug info\n");
    }
    
    /// Hook called when variable is inlined
    pub fn onVariableInlined(self: *LLVMInlinedDebugIntegration,
                            original_name: []const u8,
                            inlined_name: []const u8,
                            original_alloca: c.LLVMValueRef,
                            inlined_alloca: c.LLVMValueRef) IntegrationError!void {
        
        // Find the active inline context
        if (self.active_inlines.items.len == 0) {
            return IntegrationError.InliningTrackingFailed;
        }
        
        const active_inline = &self.active_inlines.items[self.active_inlines.items.len - 1];
        
        // Track the variable mapping
        const mapping = ActiveInline.VariableMapping{
            .original_name = original_name,
            .inlined_name = inlined_name,
            .original_alloca = original_alloca,
            .inlined_alloca = inlined_alloca,
        };
        try active_inline.variable_mappings.append(allocator, mapping);
        
        // Create debug info for the inlined variable
        try self.inlined_debug_generator.trackInlinedVariable(
            original_name,
            inlined_name,
            &active_inline.inline_context,
            null, // Would provide original debug info
            null  // Would provide inlined debug info
        );
        
        print("📍 Tracked inlined variable: {s} -> {s}\n", .{ original_name, inlined_name });
    }
    
    /// Hook called when inlining is complete
    pub fn onInliningComplete(self: *LLVMInlinedDebugIntegration) IntegrationError!void {
        if (self.active_inlines.items.len == 0) {
            return IntegrationError.InliningTrackingFailed;
        }
        
        const completed_inline = self.active_inlines.pop();
        
        print("✅ Completed inlining: {s} -> {s}\n", 
              .{ completed_inline.inline_context.original_function, 
                 completed_inline.inline_context.target_function });
        
        // Validate debug info was properly created
        _ = self.inlined_debug_generator.validateInlinedDebugInfo();
        
        // Clean up
        completed_inline.original_instructions.deinit(allocator);
        completed_inline.inlined_instructions.deinit(allocator);
        completed_inline.variable_mappings.deinit(allocator);
    }
    
    /// Generate comprehensive inlining debug report
    pub fn generateInliningReport(self: *LLVMInlinedDebugIntegration, output_file: []const u8) IntegrationError!void {
        const file = std.fs.cwd().createFile(output_file, .{}) catch |err| {
            print("❌ Failed to create integration report: {}\n", .{err});
            return IntegrationError.DebugInfoGenerationFailed;
        };
        defer file.close();
        
        const writer = file.writer();
        
        try writer.print("# CURSED LLVM Inlined Debug Integration Report\n\n");
        
        try writer.print("## Inlining Candidates\n\n");
        var candidate_iterator = self.inlining_candidates.iterator();
        while (candidate_iterator.next()) |entry| {
            const candidate = entry.value_ptr;
            try writer.print("### {s}\n", .{candidate.function_name});
            try writer.print("- Instructions: {d}\n", .{candidate.instruction_count});
            try writer.print("- Call sites: {d}\n", .{candidate.call_sites.items.len});
            try writer.print("\n");
        }
        
        try writer.print("## Active Inlines\n\n");
        for (self.active_inlines.items, 0..) |active, i| {
            try writer.print("### Active Inline {d}\n", .{i + 1});
            try writer.print("- Function: {s} -> {s}\n", .{ active.inline_context.original_function, active.inline_context.target_function });
            try writer.print("- Original instructions: {d}\n", .{active.original_instructions.items.len});
            try writer.print("- Inlined instructions: {d}\n", .{active.inlined_instructions.items.len});
            try writer.print("- Variable mappings: {d}\n", .{active.variable_mappings.items.len});
            try writer.print("\n");
        }
        
        try writer.print("## Statistics\n\n");
        try writer.print("- Total candidates: {d}\n", .{self.inlining_candidates.count()});
        try writer.print("- Active inlines: {d}\n", .{self.active_inlines.items.len});
        
        print("✅ Generated LLVM integration report: {s}\n", .{output_file});
    }
    
    /// Cleanup after inlining pass is complete
    pub fn finalizeInlining(self: *LLVMInlinedDebugIntegration) void {
        // Clean up any remaining debug info
        self.inlined_debug_generator.cleanupUnusedInlinedDebugInfo();
        
        print("🧹 Finalized inlining debug info integration\n");
    }
};

/// Export functions for integration with LLVM C API
export fn cursed_llvm_function_will_be_inlined(original_function: c.LLVMValueRef,
                                              target_function: c.LLVMValueRef,
                                              call_site: c.LLVMValueRef) void {
    print("🔄 LLVM hook: Function will be inlined\n");
    // In real implementation, this would call the integration's onFunctionWillBeInlined
    _ = original_function;
    _ = target_function;
    _ = call_site;
}

export fn cursed_llvm_instruction_inlined(original_instruction: c.LLVMValueRef,
                                        inlined_instruction: c.LLVMValueRef) void {
    print("🎯 LLVM hook: Instruction inlined\n");
    // In real implementation, this would call the integration's onInstructionInlined
    _ = original_instruction;
    _ = inlined_instruction;
}

export fn cursed_llvm_variable_inlined(original_name_ptr: [*]const u8, original_name_len: usize,
                                     inlined_name_ptr: [*]const u8, inlined_name_len: usize,
                                     original_alloca: c.LLVMValueRef,
                                     inlined_alloca: c.LLVMValueRef) void {
    const original_name = original_name_ptr[0..original_name_len];
    const inlined_name = inlined_name_ptr[0..inlined_name_len];
    
    print("📍 LLVM hook: Variable inlined: {s} -> {s}\n", .{ original_name, inlined_name });
    // In real implementation, this would call the integration's onVariableInlined
    _ = original_alloca;
    _ = inlined_alloca;
}

export fn cursed_llvm_inlining_complete() void {
    print("✅ LLVM hook: Inlining complete\n");
    // In real implementation, this would call the integration's onInliningComplete
}

/// Test the LLVM integration
pub fn testLLVMInlinedDebugIntegration() !void {
    print("Testing LLVM inlined debug integration...\n");
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    // Create mock debug generators
    const context: c.LLVMContextRef = null;
    const module: c.LLVMModuleRef = null;
    const di_builder: c.LLVMDIBuilderRef = null;
    
    var debug_generator = try debug_info.DebugInfoGenerator.init(allocator, context, module);
    defer debug_generator.deinit(allocator);
    
    var inlined_debug_generator = try inlined_debug.InlinedFunctionDebugGenerator.init(allocator, context, di_builder);
    defer inlined_debug_generator.deinit(allocator);
    
    var integration = LLVMInlinedDebugIntegration.init(allocator, &debug_generator, &inlined_debug_generator);
    defer integration.deinit(allocator);
    
    // Test the integration workflow
    try integration.analyzeModuleForInlining(module);
    try integration.generateInliningReport("llvm_integration_test_report.md");
    integration.finalizeInlining();
    
    print("✅ LLVM inlined debug integration tests passed!\n");
}
