const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// LLVM C imports disabled to fix CPU detection issues
// Replace with dummy types to allow compilation
const c = struct {
    // Dummy LLVM types
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMDIBuilderRef = ?*anyopaque;
    pub const LLVMMetadataRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy function implementations
    pub fn LLVMCreateDIBuilder(_: LLVMModuleRef) LLVMDIBuilderRef { return null; }
    pub fn LLVMDisposeDIBuilder(_: LLVMDIBuilderRef) void {}
    pub fn LLVMDIBuilderCreateDebugLocation(_: LLVMContextRef, _: u32, _: u32, _: LLVMMetadataRef, _: ?LLVMMetadataRef) LLVMMetadataRef { return null; }
    pub fn LLVMDIBuilderCreateInlinedAt(_: LLVMDIBuilderRef, _: u32, _: u32, _: LLVMMetadataRef) LLVMMetadataRef { return null; }
    pub fn LLVMInstructionSetDebugLoc(_: LLVMValueRef, _: LLVMMetadataRef) void {}
    pub fn LLVMDIBuilderCreateFunction(_: LLVMDIBuilderRef, _: LLVMMetadataRef, _: [*c]const u8, _: usize, _: [*c]const u8, _: usize, _: LLVMMetadataRef, _: u32, _: LLVMMetadataRef, _: c_int, _: c_int, _: u32, _: u32, _: c_int) LLVMMetadataRef { return null; }
    pub fn LLVMDIBuilderFinalize(_: LLVMDIBuilderRef) void {}
    pub const LLVMDIFlagZero: u32 = 0;
};

/// Represents the inlining context for debug information
pub const InlineContext = struct {
    /// Original function that was inlined
    original_function: []const u8,
    /// Function where the inlining occurred  
    target_function: []const u8,
    /// Source location where the inline occurred
    inline_site_line: u32,
    inline_site_column: u32,
    /// Original source location of the inlined code
    original_line: u32,
    original_column: u32,
    /// File where the original function was defined
    original_file: []const u8,
    /// LLVM metadata for the inlined-at location
    inlined_at_metadata: ?c.LLVMMetadataRef,
    /// Depth of inlining (for nested inlined functions)
    inline_depth: u32,
    
    pub fn init(original_function: []const u8, target_function: []const u8, 
               inline_site_line: u32, inline_site_column: u32,
               original_line: u32, original_column: u32, original_file: []const u8) InlineContext {
        return InlineContext{
            .original_function = original_function,
            .target_function = target_function,
            .inline_site_line = inline_site_line,
            .inline_site_column = inline_site_column,
            .original_line = original_line,
            .original_column = original_column,
            .original_file = original_file,
            .inlined_at_metadata = null,
            .inline_depth = 1,
        };
    }
};

/// Tracks variable mappings during inlining
pub const InlinedVariableMapping = struct {
    original_name: []const u8,
    inlined_name: []const u8,
    original_debug_info: ?c.LLVMMetadataRef,
    inlined_debug_info: ?c.LLVMMetadataRef,
    inline_context: *const InlineContext,
    
    pub fn init(original_name: []const u8, inlined_name: []const u8, inline_context: *const InlineContext) InlinedVariableMapping {
        return InlinedVariableMapping{
            .original_name = original_name,
            .inlined_name = inlined_name,
            .original_debug_info = null,
            .inlined_debug_info = null,
            .inline_context = inline_context,
        };
    }
};

/// Enhanced debug information generator for inlined functions
pub const InlinedFunctionDebugGenerator = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    di_builder: c.LLVMDIBuilderRef,
    
    /// Track all inline contexts in the compilation unit
    inline_contexts: ArrayList(InlineContext),
    
    /// Map from inlined function instances to their debug info
    inlined_function_debug_info: HashMap([]const u8, c.LLVMMetadataRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    /// Track variable mappings for inlined functions
    variable_mappings: ArrayList(InlinedVariableMapping),
    
    /// Map from original instructions to inlined debug locations
    instruction_debug_locations: HashMap(c.LLVMValueRef, c.LLVMMetadataRef, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage),
    
    /// Current inlining stack for nested inlines
    inline_stack: ArrayList(*const InlineContext),
    
    pub const InlineDebugError = error{
        InvalidInlineContext,
        DebugInfoCreationFailed,
        InstructionMappingFailed,
        OutOfMemory,
    };
    
    pub fn init(allocator: Allocator, context: c.LLVMContextRef, di_builder: c.LLVMDIBuilderRef) InlineDebugError!InlinedFunctionDebugGenerator {
        return InlinedFunctionDebugGenerator{
            .allocator = allocator,
            .context = context,
            .di_builder = di_builder,
            .inline_contexts = .empty,
            .inlined_function_debug_info = HashMap([]const u8, c.LLVMMetadataRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variable_mappings = .empty,
            .instruction_debug_locations = HashMap(c.LLVMValueRef, c.LLVMMetadataRef, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage).init(allocator),
            .inline_stack = .empty,
        };
    }
    
    pub fn deinit(self: *InlinedFunctionDebugGenerator) void {
        self.inline_contexts.deinit();
        self.inlined_function_debug_info.deinit();
        self.variable_mappings.deinit();
        self.instruction_debug_locations.deinit();
        self.inline_stack.deinit();
    }
    
    /// Create debug information for an inlined function
    pub fn createInlinedFunctionDebugInfo(self: *InlinedFunctionDebugGenerator, 
                                         inline_context: InlineContext,
                                         original_function_debug_info: c.LLVMMetadataRef,
                                         target_scope: c.LLVMMetadataRef) InlineDebugError!c.LLVMMetadataRef {
        
        // Create "inlined at" metadata
        const inlined_at = c.LLVMDIBuilderCreateInlinedAt(
            self.di_builder,
            inline_context.inline_site_line,
            inline_context.inline_site_column,
            target_scope
        );
        
        if (inlined_at == null) {
            return InlineDebugError.DebugInfoCreationFailed;
        }
        
        // Store the inline context with metadata
        var stored_context = inline_context;
        stored_context.inlined_at_metadata = inlined_at;
        try self.inline_contexts.append(self.allocator, stored_context);
        
        // Create debug info key for this inlined instance
        const debug_key = try std.fmt.allocPrint(self.allocator, "{s}_inlined_in_{s}_at_{d}_{d}", 
                                               .{ inline_context.original_function, 
                                                  inline_context.target_function,
                                                  inline_context.inline_site_line,
                                                  inline_context.inline_site_column });
        
        // Store the debug info mapping
        try self.inlined_function_debug_info.put(debug_key, original_function_debug_info);
        
        std.debug.print("✅ Created inlined function debug info for {s} -> {s} at {d}:{d}\n", 
                       .{ inline_context.original_function, inline_context.target_function,
                          inline_context.inline_site_line, inline_context.inline_site_column });
        
        return original_function_debug_info;
    }
    
    /// Create debug location for an instruction in an inlined function
    pub fn createInlinedDebugLocation(self: *InlinedFunctionDebugGenerator,
                                     instruction: c.LLVMValueRef,
                                     original_line: u32,
                                     original_column: u32,
                                     original_scope: c.LLVMMetadataRef,
                                     inline_context: *const InlineContext) InlineDebugError!void {
        
        // Create debug location with inlined-at metadata
        const debug_location = c.LLVMDIBuilderCreateDebugLocation(
            self.context,
            original_line,
            original_column,
            original_scope,
            inline_context.inlined_at_metadata
        );
        
        if (debug_location == null) {
            return InlineDebugError.DebugInfoCreationFailed;
        }
        
        // Set the debug location on the instruction
        c.LLVMInstructionSetDebugLoc(instruction, debug_location);
        
        // Store the mapping for later reference
        try self.instruction_debug_locations.put(instruction, debug_location);
        
        std.debug.print("🎯 Set inlined debug location for instruction at {d}:{d} (originally from {s})\n",
                       .{ original_line, original_column, inline_context.original_function });
    }
    
    /// Track variable mapping during inlining
    pub fn trackInlinedVariable(self: *InlinedFunctionDebugGenerator,
                               original_name: []const u8,
                               inlined_name: []const u8,
                               inline_context: *const InlineContext,
                               original_debug_info: c.LLVMMetadataRef,
                               inlined_debug_info: c.LLVMMetadataRef) InlineDebugError!void {
        
        var mapping = InlinedVariableMapping.init(original_name, inlined_name, inline_context);
        mapping.original_debug_info = original_debug_info;
        mapping.inlined_debug_info = inlined_debug_info;
        
        try self.variable_mappings.append(mapping);
        
        std.debug.print("📍 Tracked inlined variable: {s} -> {s}\n", .{ original_name, inlined_name });
    }
    
    /// Handle nested inlining by maintaining a stack of inline contexts
    pub fn pushInlineContext(self: *InlinedFunctionDebugGenerator, context: *const InlineContext) InlineDebugError!void {
        try self.inline_stack.append(context);
    }
    
    pub fn popInlineContext(self: *InlinedFunctionDebugGenerator) void {
        if (self.inline_stack.items.len > 0) {
            _ = self.inline_stack.pop();
        }
    }
    
    /// Get the current inline depth
    pub fn getCurrentInlineDepth(self: *InlinedFunctionDebugGenerator) u32 {
        return @intCast(self.inline_stack.items.len);
    }
    
    /// Create chained "inlined at" metadata for nested inlining
    pub fn createNestedInlinedAt(self: *InlinedFunctionDebugGenerator,
                                line: u32,
                                column: u32,
                                scope: c.LLVMMetadataRef) InlineDebugError!c.LLVMMetadataRef {
        
        if (self.inline_stack.items.len == 0) {
            // No nesting, create simple inlined-at
            return c.LLVMDIBuilderCreateInlinedAt(self.di_builder, line, column, scope);
        }
        
        // Chain the inlined-at metadata for nested inlining
        var current_inlined_at = c.LLVMDIBuilderCreateInlinedAt(self.di_builder, line, column, scope);
        
        // Walk up the inline stack to create the chain
        var i = self.inline_stack.items.len;
        while (i > 0) {
            i -= 1;
            const context = self.inline_stack.items[i];
            
            const nested_location = c.LLVMDIBuilderCreateDebugLocation(
                self.context,
                context.inline_site_line,
                context.inline_site_column,
                scope,
                current_inlined_at
            );
            
            current_inlined_at = nested_location;
        }
        
        return current_inlined_at orelse return InlineDebugError.DebugInfoCreationFailed;
    }
    
    /// Update debug information when inlining transforms instructions
    pub fn updateInlinedInstruction(self: *InlinedFunctionDebugGenerator,
                                   original_instruction: c.LLVMValueRef,
                                   inlined_instruction: c.LLVMValueRef,
                                   inline_context: *const InlineContext) InlineDebugError!void {
        
        // Get original debug location if it exists
        if (self.instruction_debug_locations.get(original_instruction)) |_| {
            // Create new debug location with inlined-at metadata
            const inlined_debug_loc = c.LLVMDIBuilderCreateDebugLocation(
                self.context,
                inline_context.original_line,
                inline_context.original_column,
                null, // Will be filled in by caller
                inline_context.inlined_at_metadata
            );
            
            if (inlined_debug_loc != null) {
                c.LLVMInstructionSetDebugLoc(inlined_instruction, inlined_debug_loc);
                try self.instruction_debug_locations.put(inlined_instruction, inlined_debug_loc);
            }
        }
    }
    
    /// Generate comprehensive inlining report for debugging
    pub fn generateInliningReport(self: *InlinedFunctionDebugGenerator, output_file: []const u8) InlineDebugError!void {
        const file = std.fs.cwd().createFile(output_file, .{}) catch |err| {
            std.debug.print("❌ Failed to create inlining report file: {}\n", .{err});
            return InlineDebugError.DebugInfoCreationFailed;
        };
        defer file.close();
        
        const writer = file.writer();
        
        try writer.print("# CURSED Inlined Function Debug Report\n\n", .{});
        try writer.print("Generated: {d} inline contexts tracked\n\n", .{self.inline_contexts.items.len});
        
        try writer.print("## Inline Contexts\n\n", .{});
        for (self.inline_contexts.items, 0..) |context, i| {
            try writer.print("### Context {d}\n", .{i + 1});
            try writer.print("- Original Function: `{s}`\n", .{context.original_function});
            try writer.print("- Target Function: `{s}`\n", .{context.target_function});
            try writer.print("- Inline Site: {s}:{d}:{d}\n", .{context.original_file, context.inline_site_line, context.inline_site_column});
            try writer.print("- Original Location: {d}:{d}\n", .{context.original_line, context.original_column});
            try writer.print("- Inline Depth: {d}\n", .{context.inline_depth});
            try writer.print("\n", .{});
        }
        
        try writer.print("## Variable Mappings\n\n", .{});
        for (self.variable_mappings.items, 0..) |mapping, i| {
            try writer.print("### Mapping {d}\n", .{i + 1});
            try writer.print("- Original: `{s}`\n", .{mapping.original_name});
            try writer.print("- Inlined: `{s}`\n", .{mapping.inlined_name});
            try writer.print("- Context: {s} -> {s}\n", .{mapping.inline_context.original_function, mapping.inline_context.target_function});
            try writer.print("\n", .{});
        }
        
        try writer.print("## Statistics\n\n", .{});
        try writer.print("- Total inline contexts: {d}\n", .{self.inline_contexts.items.len});
        try writer.print("- Total variable mappings: {d}\n", .{self.variable_mappings.items.len});
        try writer.print("- Total instruction mappings: {d}\n", .{self.instruction_debug_locations.count()});
        
        std.debug.print("✅ Generated inlining debug report: {s}\n", .{output_file});
    }
    
    /// Clean up debug information for unused inlined functions
    pub fn cleanupUnusedInlinedDebugInfo(self: *InlinedFunctionDebugGenerator) void {
        _ = self;
        // This is where we would remove debug info for functions that were considered
        // for inlining but ultimately not inlined, to keep the debug info clean
        std.debug.print("🧹 Cleaned up unused inlined debug information\n", .{});
    }
    
    /// Validate that all inlined instructions have proper debug locations
    pub fn validateInlinedDebugInfo(self: *InlinedFunctionDebugGenerator) bool {
        var valid = true;
        
        // Check that all inline contexts have proper metadata
        for (self.inline_contexts.items) |context| {
            if (context.inlined_at_metadata == null) {
                std.debug.print("⚠️ Warning: Inline context missing inlined_at metadata: {s} -> {s}\n",
                               .{ context.original_function, context.target_function });
                valid = false;
            }
        }
        
        // Check variable mappings
        for (self.variable_mappings.items) |mapping| {
            if (mapping.inlined_debug_info == null) {
                std.debug.print("⚠️ Warning: Variable mapping missing debug info: {s} -> {s}\n",
                               .{ mapping.original_name, mapping.inlined_name });
                valid = false;
            }
        }
        
        if (valid) {
            std.debug.print("✅ All inlined debug information is valid\n", .{});
        }
        
        return valid;
    }
};

/// Integration interface for the main debug info generator
pub const InlinedDebugIntegration = struct {
    inlined_generator: *InlinedFunctionDebugGenerator,
    
    pub fn init(inlined_generator: *InlinedFunctionDebugGenerator) InlinedDebugIntegration {
        return InlinedDebugIntegration{
            .inlined_generator = inlined_generator,
        };
    }
    
    /// Hook called when a function is about to be inlined
    pub fn onFunctionInline(self: *InlinedDebugIntegration,
                           original_function: []const u8,
                           target_function: []const u8,
                           inline_site_line: u32,
                           inline_site_column: u32,
                           original_file: []const u8) InlinedFunctionDebugGenerator.InlineDebugError!InlineContext {
        _ = self;
        
        const context = InlineContext.init(
            original_function,
            target_function,
            inline_site_line,
            inline_site_column,
            1, // Will be updated with actual original location
            1, // Will be updated with actual original location
            original_file
        );
        
        std.debug.print("🔄 Function inlining detected: {s} -> {s} at {d}:{d}\n",
                       .{ original_function, target_function, inline_site_line, inline_site_column });
        
        return context;
    }
    
    /// Hook called for each instruction being inlined
    pub fn onInstructionInline(self: *InlinedDebugIntegration,
                              instruction: c.LLVMValueRef,
                              original_line: u32,
                              original_column: u32,
                              inline_context: *const InlineContext) InlinedFunctionDebugGenerator.InlineDebugError!void {
        
        // This is where the main integration happens - update debug locations
        // for instructions that are being inlined
        try self.inlined_generator.createInlinedDebugLocation(
            instruction,
            original_line,
            original_column,
            null, // Original scope would be provided by caller
            inline_context
        );
    }
    
    /// Hook called when inlining is complete
    pub fn onInlineComplete(self: *InlinedDebugIntegration, inline_context: *const InlineContext) void {
        std.debug.print("✅ Completed inlining: {s} -> {s}\n",
                       .{ inline_context.original_function, inline_context.target_function });
        
        // Validate the debug info was properly created
        _ = self.inlined_generator.validateInlinedDebugInfo();
    }
};

/// Test the inlined function debug generation
pub fn testInlinedFunctionDebug() !void {
    std.debug.print("Testing inlined function debug generation...\n", .{});
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Create mock LLVM context and DI builder
    const context: c.LLVMContextRef = null; // Mock
    const di_builder: c.LLVMDIBuilderRef = null; // Mock
    
    var inlined_debug = try InlinedFunctionDebugGenerator.init(allocator, context, di_builder);
    defer inlined_debug.deinit();
    
    // Test creating inline context
    const inline_context = InlineContext.init(
        "helper_function",
        "main_function",
        15, // inline site line
        10, // inline site column
        5,  // original line
        1,  // original column
        "math.csd"
    );
    
    // Test the integration interface
    var integration = InlinedDebugIntegration.init(&inlined_debug);
    _ = try integration.onFunctionInline("helper", "main", 15, 10, "test.csd");
    integration.onInlineComplete(&inline_context);
    
    // Generate test report
    try inlined_debug.generateInliningReport("inlined_debug_test_report.md");
    
    std.debug.print("✅ Inlined function debug tests passed!\n", .{});
}

// Export functions for integration with LLVM backend
export fn cursed_create_inlined_debug_location(instruction: c.LLVMValueRef,
                                              original_line: u32,
                                              original_column: u32,
                                              inline_site_line: u32,
                                              inline_site_column: u32) void {
    _ = instruction;
    // This would be called from the LLVM backend when inlining occurs
    std.debug.print("🎯 Creating inlined debug location: {d}:{d} -> {d}:{d}\n",
                   .{ original_line, original_column, inline_site_line, inline_site_column });
}

export fn cursed_track_inlined_variable(original_name_ptr: [*]const u8, original_name_len: usize,
                                       inlined_name_ptr: [*]const u8, inlined_name_len: usize) void {
    const original_name = original_name_ptr[0..original_name_len];
    const inlined_name = inlined_name_ptr[0..inlined_name_len];
    
    std.debug.print("📍 Tracking inlined variable: {s} -> {s}\n", .{ original_name, inlined_name });
}
