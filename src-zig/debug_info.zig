const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/DebugInfo.h");
    @cInclude("llvm-c/DIBuilder.h");
});

const ast = @import("ast.zig");

/// DWARF debug information generator
pub const DebugInfoGenerator = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    di_builder: c.LLVMDIBuilderRef,
    compile_unit: ?c.LLVMMetadataRef,
    file_metadata: ?c.LLVMMetadataRef,
    scope_stack: ArrayList(c.LLVMMetadataRef),
    
    // Debug type cache
    debug_types: std.HashMap([]const u8, c.LLVMMetadataRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub const DebugError = error{
        InitError,
        TypeCreationError,
        ScopeError,
        MetadataError,
        OutOfMemory,
    };
    
    pub fn init(allocator: Allocator, context: c.LLVMContextRef, module: c.LLVMModuleRef) DebugError!DebugInfoGenerator {
        const di_builder = c.LLVMCreateDIBuilder(module);
        if (di_builder == null) {
            return DebugError.InitError;
        }
        
        return DebugInfoGenerator{
            .allocator = allocator,
            .context = context,
            .module = module,
            .di_builder = di_builder,
            .compile_unit = null,
            .file_metadata = null,
            .scope_stack = ArrayList(c.LLVMMetadataRef).init(allocator),
            .debug_types = std.HashMap([]const u8, c.LLVMMetadataRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *DebugInfoGenerator) void {
        if (self.di_builder) |builder| {
            c.LLVMDisposeDIBuilder(builder);
        }
        self.scope_stack.deinit();
        self.debug_types.deinit();
    }
    
    /// Initialize debug compilation unit
    pub fn createCompileUnit(self: *DebugInfoGenerator, source_filename: []const u8, directory: []const u8) DebugError!void {
        // Create file metadata
        self.file_metadata = c.LLVMDIBuilderCreateFile(
            self.di_builder,
            source_filename.ptr,
            source_filename.len,
            directory.ptr,
            directory.len
        );
        
        if (self.file_metadata == null) {
            return DebugError.MetadataError;
        }
        
        // Create compile unit
        self.compile_unit = c.LLVMDIBuilderCreateCompileUnit(
            self.di_builder,
            c.LLVMDWARFSourceLanguageC, // Use C for now, could create custom CURSED language ID
            self.file_metadata.?,
            "CURSED Compiler v1.0", // Producer
            21, // Producer length
            0, // Optimized
            "", // Flags
            0, // Flags length
            0, // Runtime version
            "", // Split name
            0, // Split name length
            c.LLVMDWARFEmissionFull,
            0, // DWO id
            1, // Split debug inlining
            0  // Debug info for profiling
        );
        
        if (self.compile_unit == null) {
            return DebugError.MetadataError;
        }
        
        // Push compile unit as initial scope
        try self.scope_stack.append(self.compile_unit.?);
    }
    
    /// Create debug information for function
    pub fn createFunction(self: *DebugInfoGenerator, name: []const u8, linkage_name: []const u8, line: u32, func_type: c.LLVMMetadataRef, function: c.LLVMValueRef) DebugError!c.LLVMMetadataRef {
        const current_scope = self.getCurrentScope();
        
        // Create function debug info
        const di_function = c.LLVMDIBuilderCreateFunction(
            self.di_builder,
            current_scope,
            name.ptr,
            name.len,
            linkage_name.ptr,
            linkage_name.len,
            self.file_metadata.?,
            line,
            func_type,
            1, // Local to unit
            1, // Definition
            line, // Scope line
            c.LLVMDIFlagZero,
            0  // Optimized
        );
        
        if (di_function == null) {
            return DebugError.MetadataError;
        }
        
        // Attach debug info to function
        c.LLVMSetSubprogram(function, di_function);
        
        // Push function as new scope
        try self.scope_stack.append(di_function);
        
        return di_function;
    }
    
    /// Create debug information for variable
    pub fn createLocalVariable(self: *DebugInfoGenerator, name: []const u8, line: u32, di_type: c.LLVMMetadataRef, alloca: c.LLVMValueRef) DebugError!void {
        const current_scope = self.getCurrentScope();
        
        // Create local variable debug info
        const di_variable = c.LLVMDIBuilderCreateAutoVariable(
            self.di_builder,
            current_scope,
            name.ptr,
            name.len,
            self.file_metadata.?,
            line,
            di_type,
            1, // Always preserve
            c.LLVMDIFlagZero,
            0  // Alignment in bits
        );
        
        if (di_variable == null) {
            return DebugError.MetadataError;
        }
        
        // Create debug location
        const debug_loc = c.LLVMDIBuilderCreateDebugLocation(
            self.context,
            line,
            0, // Column
            current_scope,
            null // Inlined at
        );
        
        // Insert variable declaration
        const di_expr = c.LLVMDIBuilderCreateExpression(self.di_builder, null, 0);
        _ = c.LLVMDIBuilderInsertDeclareAtEnd(
            self.di_builder,
            alloca,
            di_variable,
            di_expr,
            debug_loc,
            c.LLVMGetLastBasicBlock(c.LLVMGetBasicBlockParent(c.LLVMGetInstructionParent(alloca)))
        );
    }
    
    /// Create debug information for parameter
    pub fn createParameterVariable(self: *DebugInfoGenerator, name: []const u8, arg_no: u32, line: u32, di_type: c.LLVMMetadataRef, alloca: c.LLVMValueRef) DebugError!void {
        const current_scope = self.getCurrentScope();
        
        // Create parameter variable debug info
        const di_variable = c.LLVMDIBuilderCreateParameterVariable(
            self.di_builder,
            current_scope,
            name.ptr,
            name.len,
            arg_no,
            self.file_metadata.?,
            line,
            di_type,
            1, // Always preserve
            c.LLVMDIFlagZero
        );
        
        if (di_variable == null) {
            return DebugError.MetadataError;
        }
        
        // Create debug location
        const debug_loc = c.LLVMDIBuilderCreateDebugLocation(
            self.context,
            line,
            0, // Column
            current_scope,
            null // Inlined at
        );
        
        // Insert parameter declaration
        const di_expr = c.LLVMDIBuilderCreateExpression(self.di_builder, null, 0);
        _ = c.LLVMDIBuilderInsertDeclareAtEnd(
            self.di_builder,
            alloca,
            di_variable,
            di_expr,
            debug_loc,
            c.LLVMGetLastBasicBlock(c.LLVMGetBasicBlockParent(c.LLVMGetInstructionParent(alloca)))
        );
    }
    
    /// Create debug type for CURSED basic types
    pub fn createBasicType(self: *DebugInfoGenerator, name: []const u8, size_bits: u64, encoding: c.LLVMDWARFTypeEncoding) DebugError!c.LLVMMetadataRef {
        // Check cache first
        if (self.debug_types.get(name)) |cached_type| {
            return cached_type;
        }
        
        const di_type = c.LLVMDIBuilderCreateBasicType(
            self.di_builder,
            name.ptr,
            name.len,
            size_bits,
            encoding,
            c.LLVMDIFlagZero
        );
        
        if (di_type == null) {
            return DebugError.TypeCreationError;
        }
        
        // Cache the type
        try self.debug_types.put(name, di_type);
        
        return di_type;
    }
    
    /// Create debug type for CURSED struct
    pub fn createStructType(self: *DebugInfoGenerator, name: []const u8, size_bits: u64, align_bits: u32, fields: []StructField) DebugError!c.LLVMMetadataRef {
        // Check cache first
        if (self.debug_types.get(name)) |cached_type| {
            return cached_type;
        }
        
        const current_scope = self.getCurrentScope();
        
        // Create forward declaration first
        const struct_type = c.LLVMDIBuilderCreateReplaceableCompositeType(
            self.di_builder,
            c.LLVMDWARFTagStructureType,
            name.ptr,
            name.len,
            current_scope,
            self.file_metadata.?,
            0, // Line
            0, // Runtime lang
            size_bits,
            align_bits,
            c.LLVMDIFlagZero,
            "", // Unique identifier
            0  // Unique identifier length
        );
        
        if (struct_type == null) {
            return DebugError.TypeCreationError;
        }
        
        // Create field debug info
        var field_metadata = try ArrayList(c.LLVMMetadataRef).initCapacity(self.allocator, fields.len);
        defer field_metadata.deinit();
        
        var offset_bits: u64 = 0;
        for (fields) |field| {
            const field_di = c.LLVMDIBuilderCreateMemberType(
                self.di_builder,
                struct_type,
                field.name.ptr,
                field.name.len,
                self.file_metadata.?,
                0, // Line
                field.size_bits,
                field.align_bits,
                offset_bits,
                c.LLVMDIFlagZero,
                field.di_type
            );
            
            if (field_di == null) {
                return DebugError.TypeCreationError;
            }
            
            field_metadata.appendAssumeCapacity(field_di);
            offset_bits += field.size_bits;
        }
        
        // Create the complete struct type
        const complete_struct = c.LLVMDIBuilderCreateStructType(
            self.di_builder,
            current_scope,
            name.ptr,
            name.len,
            self.file_metadata.?,
            0, // Line
            size_bits,
            align_bits,
            c.LLVMDIFlagZero,
            null, // Derived from
            field_metadata.items.ptr,
            @as(u32, @intCast(field_metadata.items.len)),
            0, // Runtime lang
            null, // VTable holder
            "", // Unique identifier
            0  // Unique identifier length
        );
        
        if (complete_struct == null) {
            return DebugError.TypeCreationError;
        }
        
        // Replace the forward declaration
        c.LLVMMetadataReplaceAllUsesWith(struct_type, complete_struct);
        
        // Cache the type
        try self.debug_types.put(name, complete_struct);
        
        return complete_struct;
    }
    
    /// Create debug type for function
    pub fn createFunctionType(self: *DebugInfoGenerator, return_type: c.LLVMMetadataRef, param_types: []c.LLVMMetadataRef) DebugError!c.LLVMMetadataRef {
        // Create parameter array (return type is first element)
        var all_types = try ArrayList(c.LLVMMetadataRef).initCapacity(self.allocator, param_types.len + 1);
        defer all_types.deinit();
        
        all_types.appendAssumeCapacity(return_type);
        for (param_types) |param_type| {
            all_types.appendAssumeCapacity(param_type);
        }
        
        const func_type = c.LLVMDIBuilderCreateSubroutineType(
            self.di_builder,
            self.file_metadata.?,
            all_types.items.ptr,
            @as(u32, @intCast(all_types.items.len)),
            c.LLVMDIFlagZero
        );
        
        if (func_type == null) {
            return DebugError.TypeCreationError;
        }
        
        return func_type;
    }
    
    /// Create debug type for pointer
    pub fn createPointerType(self: *DebugInfoGenerator, pointed_type: c.LLVMMetadataRef) DebugError!c.LLVMMetadataRef {
        const ptr_type = c.LLVMDIBuilderCreatePointerType(
            self.di_builder,
            pointed_type,
            64, // Size in bits (assuming 64-bit pointers)
            0,  // Alignment in bits
            0,  // Address space
            "", // Name
            0   // Name length
        );
        
        if (ptr_type == null) {
            return DebugError.TypeCreationError;
        }
        
        return ptr_type;
    }
    
    /// Create debug type for array
    pub fn createArrayType(self: *DebugInfoGenerator, element_type: c.LLVMMetadataRef, size: u64) DebugError!c.LLVMMetadataRef {
        // Create subrange for array bounds
        const subrange = c.LLVMDIBuilderGetOrCreateSubrange(self.di_builder, 0, @as(i64, @intCast(size)));
        if (subrange == null) {
            return DebugError.TypeCreationError;
        }
        
        const array_type = c.LLVMDIBuilderCreateArrayType(
            self.di_builder,
            size * 64, // Size in bits (assuming 64-bit elements)
            0, // Alignment
            element_type,
            &subrange,
            1 // Subscript count
        );
        
        if (array_type == null) {
            return DebugError.TypeCreationError;
        }
        
        return array_type;
    }
    
    /// Set debug location for instruction
    pub fn setDebugLocation(self: *DebugInfoGenerator, instruction: c.LLVMValueRef, line: u32, column: u32) void {
        const current_scope = self.getCurrentScope();
        
        const debug_loc = c.LLVMDIBuilderCreateDebugLocation(
            self.context,
            line,
            column,
            current_scope,
            null // Inlined at
        );
        
        c.LLVMInstructionSetDebugLoc(instruction, debug_loc);
    }
    
    /// Enter new lexical scope
    pub fn pushScope(self: *DebugInfoGenerator, scope: c.LLVMMetadataRef) DebugError!void {
        try self.scope_stack.append(scope);
    }
    
    /// Exit current lexical scope
    pub fn popScope(self: *DebugInfoGenerator) void {
        if (self.scope_stack.items.len > 1) { // Keep at least compile unit
            _ = self.scope_stack.pop();
        }
    }
    
    /// Create lexical block scope
    pub fn createLexicalBlock(self: *DebugInfoGenerator, line: u32, column: u32) DebugError!c.LLVMMetadataRef {
        const current_scope = self.getCurrentScope();
        
        const block = c.LLVMDIBuilderCreateLexicalBlock(
            self.di_builder,
            current_scope,
            self.file_metadata.?,
            line,
            column
        );
        
        if (block == null) {
            return DebugError.ScopeError;
        }
        
        return block;
    }
    
    /// Finalize debug information
    pub fn finalize(self: *DebugInfoGenerator) void {
        c.LLVMDIBuilderFinalize(self.di_builder);
    }
    
    /// Get current debug scope
    fn getCurrentScope(self: *DebugInfoGenerator) c.LLVMMetadataRef {
        if (self.scope_stack.items.len > 0) {
            return self.scope_stack.items[self.scope_stack.items.len - 1];
        }
        return self.compile_unit.?;
    }
    
    /// Helper to create standard CURSED type debug info
    pub fn createCursedTypes(self: *DebugInfoGenerator) DebugError!CursedDebugTypes {
        return CursedDebugTypes{
            .normie_type = try self.createBasicType("normie", 32, c.LLVMDWARFTypeEncodingSigned),
            .tea_type = try self.createBasicType("tea", 64, c.LLVMDWARFTypeEncodingUTF),
            .drip_type = try self.createBasicType("drip", 64, c.LLVMDWARFTypeEncodingSigned),
            .lit_type = try self.createBasicType("lit", 1, c.LLVMDWARFTypeEncodingBoolean),
            .meal_type = try self.createBasicType("meal", 64, c.LLVMDWARFTypeEncodingFloat),
            .smol_type = try self.createBasicType("smol", 8, c.LLVMDWARFTypeEncodingSigned),
            .thicc_type = try self.createBasicType("thicc", 64, c.LLVMDWARFTypeEncodingSigned),
            .sip_type = try self.createBasicType("sip", 8, c.LLVMDWARFTypeEncodingUnsigned),
        };
    }
};

/// Standard CURSED debug types
pub const CursedDebugTypes = struct {
    normie_type: c.LLVMMetadataRef,
    tea_type: c.LLVMMetadataRef,
    drip_type: c.LLVMMetadataRef,
    lit_type: c.LLVMMetadataRef,
    meal_type: c.LLVMMetadataRef,
    smol_type: c.LLVMMetadataRef,
    thicc_type: c.LLVMMetadataRef,
    sip_type: c.LLVMMetadataRef,
};

/// Struct field debug information
pub const StructField = struct {
    name: []const u8,
    di_type: c.LLVMMetadataRef,
    size_bits: u64,
    align_bits: u32,
};

/// Source location for debug info
pub const SourceLocation = struct {
    line: u32,
    column: u32,
    filename: []const u8,
};

test "debug info generator initialization" {
    const allocator = std.testing.allocator;
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    var debug_gen = try DebugInfoGenerator.init(allocator, context, module);
    defer debug_gen.deinit();
    
    try debug_gen.createCompileUnit("test.csd", "/tmp");
    
    try std.testing.expect(debug_gen.compile_unit != null);
    try std.testing.expect(debug_gen.file_metadata != null);
}

test "basic type creation" {
    const allocator = std.testing.allocator;
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    var debug_gen = try DebugInfoGenerator.init(allocator, context, module);
    defer debug_gen.deinit();
    
    try debug_gen.createCompileUnit("test.csd", "/tmp");
    
    const cursed_types = try debug_gen.createCursedTypes();
    try std.testing.expect(cursed_types.normie_type != null);
    try std.testing.expect(cursed_types.lit_type != null);
}
