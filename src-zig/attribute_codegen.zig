const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const StringContext = std.hash_map.StringContext;

const ast = @import("ast.zig");
const FunctionStatement = ast.FunctionStatement;
const StructStatement = ast.StructStatement;
const Type = ast.Type;

const attribute_system = @import("attribute_system.zig");
const Attribute = attribute_system.Attribute;
const AttributeType = attribute_system.AttributeType;
const AttributeList = attribute_system.AttributeList;

const advanced_codegen = @import("advanced_codegen.zig");
const AdvancedCodeGen = advanced_codegen.AdvancedCodeGen;

// Import LLVM C types
const c = @cImport({
    @cDefine("__x86_64__", "1");
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/Transforms/Scalar.h");
    @cInclude("llvm-c/Transforms/IPO.h");
});

/// Attribute-driven code generation hooks
/// Modifies LLVM IR generation based on attributes applied to declarations
pub const AttributeCodeGen = struct {
    allocator: Allocator,
    base_codegen: *AdvancedCodeGen,
    
    // Attribute processing cache to avoid recomputation
    function_attributes_cache: HashMap([]const u8, ProcessedFunctionAttributes, StringContext, std.hash_map.default_max_load_percentage),
    struct_attributes_cache: HashMap([]const u8, ProcessedStructAttributes, StringContext, std.hash_map.default_max_load_percentage),
    
    // Code generation metadata
    inline_candidates: ArrayList([]const u8),
    hot_functions: ArrayList([]const u8),
    cold_functions: ArrayList([]const u8),
    exported_functions: HashMap([]const u8, []const u8, StringContext, std.hash_map.default_max_load_percentage), // function_name -> export_name
    
    pub fn init(allocator: Allocator, base_codegen: *AdvancedCodeGen) AttributeCodeGen {
        return AttributeCodeGen{
            .allocator = allocator,
            .base_codegen = base_codegen,
            .function_attributes_cache = HashMap([]const u8, ProcessedFunctionAttributes, StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .struct_attributes_cache = HashMap([]const u8, ProcessedStructAttributes, StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .inline_candidates = ArrayList([]const u8).init(allocator),
            .hot_functions = ArrayList([]const u8).init(allocator),
            .cold_functions = ArrayList([]const u8).init(allocator),
            .exported_functions = HashMap([]const u8, []const u8, StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *AttributeCodeGen) void {
        self.function_attributes_cache.deinit();
        self.struct_attributes_cache.deinit();
        self.inline_candidates.deinit();
        self.hot_functions.deinit();
        self.cold_functions.deinit();
        self.exported_functions.deinit();
    }
    
    /// Process function attributes and apply code generation modifications
    pub fn processFunctionAttributes(self: *AttributeCodeGen, func: *const FunctionStatement, llvm_function: c.LLVMValueRef) !void {
        if (func.attributes == null) return;
        
        const attrs = func.attributes.?;
        const processed = try self.processAndCacheFunctionAttributes(func.name, &attrs);
        
        // Apply LLVM function attributes based on processed attributes
        try self.applyLLVMFunctionAttributes(llvm_function, &processed);
        
        // Update optimization lists
        if (processed.should_inline) {
            try self.inline_candidates.append(func.name);
        }
        
        if (processed.performance_level) |level| {
            switch (level) {
                .High => try self.hot_functions.append(func.name),
                .Low => try self.cold_functions.append(func.name),
                .Medium => {}, // Default optimization
            }
        }
        
        if (processed.export_name) |export_name| {
            try self.exported_functions.put(func.name, export_name);
        }
    }
    
    /// Process struct attributes and apply memory layout modifications
    pub fn processStructAttributes(self: *AttributeCodeGen, struct_stmt: *const StructStatement, llvm_struct_type: c.LLVMTypeRef) !void {
        if (struct_stmt.attributes == null) return;
        
        const attrs = struct_stmt.attributes.?;
        const processed = try self.processAndCacheStructAttributes(struct_stmt.name, &attrs);
        
        // Apply struct layout attributes
        try self.applyLLVMStructAttributes(llvm_struct_type, &processed);
    }
    
    /// Generate optimization passes based on accumulated attribute information
    pub fn generateOptimizationPasses(self: *AttributeCodeGen, pass_manager: c.LLVMPassManagerRef) !void {
        // Add inlining passes for functions marked with @inline
        if (self.inline_candidates.items.len > 0) {
            c.LLVMAddFunctionInliningPass(pass_manager);
            c.LLVMAddAlwaysInlinerPass(pass_manager);
        }
        
        // Add aggressive optimization for hot functions
        if (self.hot_functions.items.len > 0) {
            c.LLVMAddAggressiveDCEPass(pass_manager);
            c.LLVMAddLoopUnrollPass(pass_manager);
            c.LLVMAddVectorizePass(pass_manager);
        }
        
        // Add size optimization for cold functions
        if (self.cold_functions.items.len > 0) {
            c.LLVMAddCodeGenPreparePass(pass_manager);
            c.LLVMAddDeadStoreEliminationPass(pass_manager);
        }
    }
    
    /// Apply function-level debug metadata based on attributes
    pub fn applyDebugAttributes(self: *AttributeCodeGen, func: *const FunctionStatement, llvm_function: c.LLVMValueRef) !void {
        if (func.attributes == null) return;
        
        const attrs = func.attributes.?;
        
        // Check for debug attributes
        if (attrs.findByType(.Debug)) |debug_attr| {
            const enable = debug_attr.getBooleanParameter("enable") orelse true;
            if (!enable) {
                // Remove debug information for this function
                self.removeDebugInfo(llvm_function);
            }
        }
        
        if (attrs.findByType(.NoDebug)) |_| {
            self.removeDebugInfo(llvm_function);
        }
    }
    
    /// Generate export declarations based on @export attributes
    pub fn generateExportDeclarations(self: *AttributeCodeGen, module: c.LLVMModuleRef) !void {
        var iterator = self.exported_functions.iterator();
        while (iterator.next()) |entry| {
            const function_name = entry.key_ptr.*;
            const export_name = entry.value_ptr.*;
            
            // Get the LLVM function
            const llvm_function = c.LLVMGetNamedFunction(module, function_name.ptr);
            if (llvm_function != null) {
                // Set linkage to external
                c.LLVMSetLinkage(llvm_function, c.LLVMExternalLinkage);
                
                // Set the exported name if different from function name
                if (!std.mem.eql(u8, function_name, export_name)) {
                    c.LLVMSetValueName2(llvm_function, export_name.ptr, export_name.len);
                }
            }
        }
    }
    
    // Private implementation methods
    
    fn processAndCacheFunctionAttributes(self: *AttributeCodeGen, func_name: []const u8, attrs: *const AttributeList) !ProcessedFunctionAttributes {
        // Check cache first
        if (self.function_attributes_cache.get(func_name)) |cached| {
            return cached;
        }
        
        var processed = ProcessedFunctionAttributes{};
        
        // Process inline attributes
        if (attrs.findByType(.Inline)) |inline_attr| {
            const hint = inline_attr.getStringParameter("hint") orelse "hint";
            processed.should_inline = std.mem.eql(u8, hint, "always") or std.mem.eql(u8, hint, "hint");
            processed.never_inline = std.mem.eql(u8, hint, "never");
        }
        
        // Process performance attributes
        if (attrs.findByType(.Performance)) |perf_attr| {
            const level = perf_attr.getStringParameter("level") orelse "medium";
            processed.performance_level = if (std.mem.eql(u8, level, "high"))
                PerformanceLevel.High
            else if (std.mem.eql(u8, level, "low"))
                PerformanceLevel.Low
            else
                PerformanceLevel.Medium;
        }
        
        // Process optimization attributes
        if (attrs.findByType(.Optimize)) |opt_attr| {
            const target = opt_attr.getStringParameter("target") orelse "speed";
            processed.optimization_target = if (std.mem.eql(u8, target, "size"))
                OptimizationTarget.Size
            else if (std.mem.eql(u8, target, "debug"))
                OptimizationTarget.Debug
            else
                OptimizationTarget.Speed;
        }
        
        // Process export attributes
        if (attrs.findByType(.Export)) |export_attr| {
            processed.export_name = export_attr.getStringParameter("name");
        }
        
        // Process vectorization attributes
        if (attrs.findByType(.Vectorize)) |vec_attr| {
            const enable = vec_attr.getStringParameter("enable") orelse "enable";
            processed.vectorize = std.mem.eql(u8, enable, "enable");
        }
        
        // Process unroll attributes
        if (attrs.findByType(.Unroll)) |unroll_attr| {
            processed.unroll_count = unroll_attr.getIntegerParameter("count");
        }
        
        // Cache the result
        try self.function_attributes_cache.put(func_name, processed);
        
        return processed;
    }
    
    fn processAndCacheStructAttributes(self: *AttributeCodeGen, struct_name: []const u8, attrs: *const AttributeList) !ProcessedStructAttributes {
        // Check cache first
        if (self.struct_attributes_cache.get(struct_name)) |cached| {
            return cached;
        }
        
        var processed = ProcessedStructAttributes{};
        
        // Process memory layout attributes
        if (attrs.findByType(.MemoryLayout)) |layout_attr| {
            const layout = layout_attr.getStringParameter("layout") orelse "native";
            processed.layout = if (std.mem.eql(u8, layout, "packed"))
                MemoryLayout.Packed
            else if (std.mem.eql(u8, layout, "aligned"))
                MemoryLayout.Aligned
            else
                MemoryLayout.Native;
        }
        
        // Process alignment attributes
        if (attrs.findByType(.Align)) |align_attr| {
            processed.alignment = align_attr.getIntegerParameter("bytes");
        }
        
        // Process pack attributes
        if (attrs.findByType(.Pack)) |pack_attr| {
            const enable = pack_attr.getStringParameter("enable") orelse "enable";
            processed.packed = std.mem.eql(u8, enable, "enable");
        }
        
        // Cache the result
        try self.struct_attributes_cache.put(struct_name, processed);
        
        return processed;
    }
    
    fn applyLLVMFunctionAttributes(self: *AttributeCodeGen, llvm_function: c.LLVMValueRef, processed: *const ProcessedFunctionAttributes) !void {
        _ = self; // Suppress unused warning
        
        // Apply inlining attributes
        if (processed.should_inline) {
            c.LLVMAddAttributeAtIndex(llvm_function, c.LLVMAttributeFunctionIndex, 
                c.LLVMCreateEnumAttribute(c.LLVMGetGlobalContext(), c.LLVMGetEnumAttributeKindForName("alwaysinline", 12), 0));
        } else if (processed.never_inline) {
            c.LLVMAddAttributeAtIndex(llvm_function, c.LLVMAttributeFunctionIndex,
                c.LLVMCreateEnumAttribute(c.LLVMGetGlobalContext(), c.LLVMGetEnumAttributeKindForName("noinline", 8), 0));
        }
        
        // Apply optimization attributes based on performance level
        if (processed.performance_level) |level| {
            switch (level) {
                .High => {
                    // Add aggressive optimization attributes
                    c.LLVMAddAttributeAtIndex(llvm_function, c.LLVMAttributeFunctionIndex,
                        c.LLVMCreateEnumAttribute(c.LLVMGetGlobalContext(), c.LLVMGetEnumAttributeKindForName("optsize", 7), 0));
                },
                .Low => {
                    // Add size optimization attributes
                    c.LLVMAddAttributeAtIndex(llvm_function, c.LLVMAttributeFunctionIndex,
                        c.LLVMCreateEnumAttribute(c.LLVMGetGlobalContext(), c.LLVMGetEnumAttributeKindForName("minsize", 7), 0));
                },
                .Medium => {
                    // Default optimization, no special attributes
                },
            }
        }
        
        // Apply vectorization hints
        if (processed.vectorize) {
            // Add vectorization metadata to function
            // This would require more complex LLVM metadata handling
        }
    }
    
    fn applyLLVMStructAttributes(self: *AttributeCodeGen, llvm_struct_type: c.LLVMTypeRef, processed: *const ProcessedStructAttributes) !void {
        _ = self; // Suppress unused warning
        _ = llvm_struct_type; // Suppress unused warning
        
        // Apply memory layout attributes
        if (processed.layout) |layout| {
            switch (layout) {
                .Packed => {
                    // Set struct as packed in LLVM
                    // This requires modifying the struct creation to use LLVMStructCreateNamed with packed=true
                },
                .Aligned => {
                    // Apply specific alignment
                    if (processed.alignment) |align_bytes| {
                        // Set struct alignment
                        _ = align_bytes;
                    }
                },
                .Native => {
                    // Use default layout
                },
            }
        }
    }
    
    fn removeDebugInfo(self: *AttributeCodeGen, llvm_function: c.LLVMValueRef) void {
        _ = self; // Suppress unused warning
        
        // Remove debug metadata from function
        c.LLVMSetSubprogram(llvm_function, null);
        
        // This would involve walking through all instructions and removing debug intrinsics
        // For now, just remove the function-level debug info
    }
};

/// Processed function attributes for efficient code generation
const ProcessedFunctionAttributes = struct {
    should_inline: bool = false,
    never_inline: bool = false,
    performance_level: ?PerformanceLevel = null,
    optimization_target: ?OptimizationTarget = null,
    export_name: ?[]const u8 = null,
    vectorize: bool = false,
    unroll_count: ?i64 = null,
};

/// Processed struct attributes for memory layout optimization
const ProcessedStructAttributes = struct {
    layout: ?MemoryLayout = null,
    alignment: ?i64 = null,
    packed: bool = false,
};

/// Performance optimization levels
const PerformanceLevel = enum {
    Low,
    Medium,
    High,
};

/// Optimization targets
const OptimizationTarget = enum {
    Speed,
    Size,
    Debug,
};

/// Memory layout strategies
const MemoryLayout = enum {
    Native,
    Packed,
    Aligned,
};

/// Integration hooks for the main compiler pipeline

/// Hook called before function compilation to process attributes
pub fn preprocessFunctionAttributes(attr_codegen: *AttributeCodeGen, func: *const FunctionStatement) !void {
    if (func.attributes) |attrs| {
        // Pre-process attributes to determine compilation strategy
        const processed = try attr_codegen.processAndCacheFunctionAttributes(func.name, &attrs);
        
        // Adjust codegen parameters based on attributes
        if (processed.optimization_target) |target| {
            switch (target) {
                .Speed => {
                    // Configure for speed optimization
                    attr_codegen.base_codegen.optimization_config.optimize_for_speed = true;
                },
                .Size => {
                    // Configure for size optimization
                    attr_codegen.base_codegen.optimization_config.optimize_for_size = true;
                },
                .Debug => {
                    // Configure for debugging
                    attr_codegen.base_codegen.optimization_config.debug_mode = true;
                },
            }
        }
    }
}

/// Hook called after function compilation to apply post-processing
pub fn postprocessFunctionAttributes(attr_codegen: *AttributeCodeGen, func: *const FunctionStatement, llvm_function: c.LLVMValueRef) !void {
    try attr_codegen.processFunctionAttributes(func, llvm_function);
    try attr_codegen.applyDebugAttributes(func, llvm_function);
}

/// Hook called during struct compilation to apply memory layout
pub fn processStructLayoutAttributes(attr_codegen: *AttributeCodeGen, struct_stmt: *const StructStatement, llvm_struct_type: c.LLVMTypeRef) !void {
    try attr_codegen.processStructAttributes(struct_stmt, llvm_struct_type);
}

/// Hook called during module finalization to apply global optimizations
pub fn finalizeAttributeOptimizations(attr_codegen: *AttributeCodeGen, module: c.LLVMModuleRef, pass_manager: c.LLVMPassManagerRef) !void {
    try attr_codegen.generateOptimizationPasses(pass_manager);
    try attr_codegen.generateExportDeclarations(module);
}

// Example usage and testing

test "attribute-driven function optimization" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // This would require a full LLVM context setup for proper testing
    // For now, just test the attribute processing logic
    
    var attrs = AttributeList.init(allocator);
    defer attrs.deinit(allocator);
    
    // Add performance attribute
    var perf_attr = try attribute_system.createPerformanceAttribute(allocator, "high", ast.SourceLocation.unknown());
    try attrs.addAttribute(perf_attr);
    
    // Add inline attribute
    var inline_attr = try attribute_system.createInlineAttribute(allocator, "always", ast.SourceLocation.unknown());
    try attrs.addAttribute(inline_attr);
    
    // Test validation
    try attrs.validate();
    
    try std.testing.expect(attrs.hasAttribute(.Performance));
    try std.testing.expect(attrs.hasAttribute(.Inline));
}
