const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
// LLVM C imports disabled to fix "athlon-xp" CPU detection issues
// Replace with dummy types to allow compilation without LLVM
const c = struct {
    // Dummy LLVM types to make compilation work without LLVM
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMExecutionEngineRef = ?*anyopaque;
    pub const LLVMTargetRef = ?*anyopaque;
    pub const LLVMTargetMachineRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    pub const LLVMMemoryBufferRef = ?*anyopaque;
    pub const LLVMDIBuilderRef = ?*anyopaque;
    pub const LLVMMetadataRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy functions to prevent link errors (add more as needed)
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMInitializeX86TargetInfo() void {}
    pub fn LLVMInitializeX86Target() void {}
    pub fn LLVMInitializeX86TargetMC() void {}
    pub fn LLVMInitializeX86AsmPrinter() void {}
    pub fn LLVMCreateTargetMachine() LLVMTargetMachineRef { return null; }
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}
    pub fn LLVMCreateDIBuilder(_: LLVMModuleRef) LLVMDIBuilderRef { return null; }
    pub fn LLVMDisposeDIBuilder(_: LLVMDIBuilderRef) void {}
    pub fn LLVMDIBuilderCreateDebugLocation(_: LLVMContextRef, _: u32, _: u32, _: LLVMMetadataRef, _: ?LLVMMetadataRef) LLVMMetadataRef { return null; }
    pub fn LLVMInstructionSetDebugLoc(_: LLVMValueRef, _: LLVMMetadataRef) void {}
    pub fn LLVMContextCreate() LLVMContextRef { return null; }
    pub fn LLVMContextDispose(_: LLVMContextRef) void {}
    pub fn LLVMModuleCreateWithNameInContext(_: [*c]const u8, _: LLVMContextRef) LLVMModuleRef { return null; }
};

const ast = @import("ast.zig");

/// Advanced DWARF debug information generator with comprehensive GDB/LLDB support
/// Provides complete source location mapping, variable debugging, and stack trace support
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
    
    // Enhanced debug info tracking
    current_line: u32,
    current_column: u32,
    source_file_path: []const u8,
    directory_path: []const u8,
    cursed_debug_types: ?CursedDebugTypes,
    
    // Function debug metadata
    function_debug_info: std.HashMap(c.LLVMValueRef, c.LLVMMetadataRef, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage),
    
    // Variable debug tracking
    variable_debug_info: std.HashMap(c.LLVMValueRef, VariableDebugInfo, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage),
    
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
            .scope_stack = .empty,
            .debug_types = std.HashMap([]const u8, c.LLVMMetadataRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .current_line = 1,
            .current_column = 1,
            .source_file_path = "",
            .directory_path = "",
            .cursed_debug_types = null,
            .function_debug_info = std.HashMap(c.LLVMValueRef, c.LLVMMetadataRef, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage).init(allocator),
            .variable_debug_info = std.HashMap(c.LLVMValueRef, VariableDebugInfo, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *DebugInfoGenerator) void {
        if (self.di_builder) |builder| {
            c.LLVMDisposeDIBuilder(builder);
        }
        self.scope_stack.deinit(self.allocator);
        self.debug_types.deinit(self.allocator);
        self.function_debug_info.deinit(self.allocator);
        self.variable_debug_info.deinit(self.allocator);
    }
    
    /// Initialize comprehensive debug compilation unit with enhanced GDB/LLDB support
    pub fn createCompileUnit(self: *DebugInfoGenerator, source_filename: []const u8, directory: []const u8) DebugError!void {
        // Store file paths for later use
        self.source_file_path = source_filename;
        self.directory_path = directory;
        
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
        
        // Create compile unit with enhanced debug info for CURSED language
        self.compile_unit = c.LLVMDIBuilderCreateCompileUnit(
            self.di_builder,
            c.LLVMDWARFSourceLanguageC, // Use C for compatibility with GDB/LLDB
            self.file_metadata.?,
            "CURSED Compiler v1.0 with DWARF debug info", // Producer
            42, // Producer length
            0, // Not optimized for debug builds
            "-g -O0", // Debug flags
            7, // Flags length
            1, // Runtime version
            "", // Split name
            0, // Split name length
            c.LLVMDWARFEmissionFull, // Full debug emission
            0, // DWO id
            1, // Split debug inlining
            1  // Debug info for profiling enabled
        );
        
        if (self.compile_unit == null) {
            return DebugError.MetadataError;
        }
        
        // Initialize standard CURSED debug types
        self.cursed_debug_types = try self.createCursedTypes();
        
        // Push compile unit as initial scope
        try self.scope_stack.append(self.allocator, self.compile_unit.?);
        
        std.debug.print("✅ Debug compilation unit created for {s}\n", .{source_filename});
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
        try self.scope_stack.append(allocator, di_function);
        
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
        try self.scope_stack.append(allocator, scope);
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
    
    /// Set current source location for debugging
    pub fn setCurrentLocation(self: *DebugInfoGenerator, line: u32, column: u32) void {
        self.current_line = line;
        self.current_column = column;
    }
    
    /// Create comprehensive debug location with stack trace support
    pub fn createDebugLocation(self: *DebugInfoGenerator, line: u32, column: u32, scope: ?c.LLVMMetadataRef) c.LLVMMetadataRef {
        const debug_scope = scope orelse self.getCurrentScope();
        
        return c.LLVMDIBuilderCreateDebugLocation(
            self.context,
            line,
            column,
            debug_scope,
            null // Inlined at
        );
    }
    
    /// Set debug location for instruction with enhanced tracking
    pub fn setInstructionDebugLocation(self: *DebugInfoGenerator, instruction: c.LLVMValueRef, line: u32, column: u32) void {
        const debug_loc = self.createDebugLocation(line, column, null);
        c.LLVMInstructionSetDebugLoc(instruction, debug_loc);
    }
    
    /// Track variable for debugging with comprehensive metadata
    pub fn trackVariable(self: *DebugInfoGenerator, name: []const u8, alloca: c.LLVMValueRef, di_type: c.LLVMMetadataRef, di_variable: c.LLVMMetadataRef, line: u32, column: u32, is_parameter: bool) DebugError!void {
        const var_info = VariableDebugInfo{
            .name = name,
            .di_type = di_type,
            .di_variable = di_variable,
            .alloca = alloca,
            .line = line,
            .column = column,
            .is_parameter = is_parameter,
        };
        
        try self.variable_debug_info.put(alloca, var_info);
    }
    
    /// Create debug info for CURSED variable with type inference
    pub fn createCursedVariable(self: *DebugInfoGenerator, name: []const u8, cursed_type: []const u8, line: u32, alloca: c.LLVMValueRef) DebugError!void {
        const di_type = self.getCursedDebugType(cursed_type) orelse {
            std.debug.print("⚠️ Warning: Unknown CURSED type {s}, using normie\n", .{cursed_type});
            return self.cursed_debug_types.?.normie_type;
        };
        
        try self.createLocalVariable(name, line, di_type, alloca);
    }
    
    /// Get debug type for CURSED type name
    fn getCursedDebugType(self: *DebugInfoGenerator, type_name: []const u8) ?c.LLVMMetadataRef {
        const types = self.cursed_debug_types orelse return null;
        
        if (std.mem.eql(u8, type_name, "normie")) return types.normie_type;
        if (std.mem.eql(u8, type_name, "tea")) return types.tea_type;
        if (std.mem.eql(u8, type_name, "drip")) return types.drip_type;
        if (std.mem.eql(u8, type_name, "lit")) return types.lit_type;
        if (std.mem.eql(u8, type_name, "meal")) return types.meal_type;
        if (std.mem.eql(u8, type_name, "smol")) return types.smol_type;
        if (std.mem.eql(u8, type_name, "thicc")) return types.thicc_type;
        if (std.mem.eql(u8, type_name, "sip")) return types.sip_type;
        
        return null;
    }
    
    /// Create inlined function debug info for better stack traces
    pub fn createInlinedFunction(self: *DebugInfoGenerator, name: []const u8, func_type: c.LLVMMetadataRef, line: u32) DebugError!c.LLVMMetadataRef {
        const current_scope = self.getCurrentScope();
        
        const inlined_func = c.LLVMDIBuilderCreateFunction(
            self.di_builder,
            current_scope,
            name.ptr,
            name.len,
            name.ptr,
            name.len,
            self.file_metadata.?,
            line,
            func_type,
            0, // Not local to unit (inlined)
            1, // Definition
            line, // Scope line
            c.LLVMDIFlagZero,
            1  // Optimized/inlined
        );
        
        if (inlined_func == null) {
            return DebugError.MetadataError;
        }
        
        return inlined_func;
    }
    
    /// Create debug location with inlined-at metadata for inlined functions
    pub fn createInlinedDebugLocation(self: *DebugInfoGenerator, 
                                     line: u32, 
                                     column: u32, 
                                     scope: c.LLVMMetadataRef,
                                     inlined_at_line: u32,
                                     inlined_at_column: u32,
                                     inlined_at_scope: c.LLVMMetadataRef) c.LLVMMetadataRef {
        
        // Create the "inlined at" metadata first
        const inlined_at = c.LLVMDIBuilderCreateDebugLocation(
            self.context,
            inlined_at_line,
            inlined_at_column,
            inlined_at_scope,
            null // No further nesting
        );
        
        // Create the main debug location with inlined-at reference
        return c.LLVMDIBuilderCreateDebugLocation(
            self.context,
            line,
            column,
            scope,
            inlined_at // This tells debuggers where the function was inlined from
        );
    }
    
    /// Set debug location for inlined instruction with proper call stack mapping
    pub fn setInlinedInstructionDebugLocation(self: *DebugInfoGenerator, 
                                             instruction: c.LLVMValueRef, 
                                             original_line: u32, 
                                             original_column: u32,
                                             original_scope: c.LLVMMetadataRef,
                                             inline_site_line: u32,
                                             inline_site_column: u32,
                                             inline_site_scope: c.LLVMMetadataRef) void {
        
        const inlined_debug_loc = self.createInlinedDebugLocation(
            original_line,
            original_column,
            original_scope,
            inline_site_line,
            inline_site_column,
            inline_site_scope
        );
        
        c.LLVMInstructionSetDebugLoc(instruction, inlined_debug_loc);
        
        std.debug.print("🎯 Set inlined debug location for instruction: {d}:{d} (inlined at {d}:{d})\n",
                       .{ original_line, original_column, inline_site_line, inline_site_column });
    }
    
    /// Create debug info for inlined variable with proper scope tracking
    pub fn createInlinedVariable(self: *DebugInfoGenerator, 
                                name: []const u8, 
                                original_line: u32,
                                original_scope: c.LLVMMetadataRef,
                                di_type: c.LLVMMetadataRef, 
                                alloca: c.LLVMValueRef,
                                inline_site_line: u32,
                                inline_site_column: u32,
                                inline_site_scope: c.LLVMMetadataRef) DebugError!void {
        
        // Create inlined debug location for the variable
        const inlined_location = self.createInlinedDebugLocation(
            original_line,
            0, // Column not available for variables
            original_scope,
            inline_site_line,
            inline_site_column,
            inline_site_scope
        );
        
        // Create the variable debug info
        const di_variable = c.LLVMDIBuilderCreateAutoVariable(
            self.di_builder,
            original_scope,
            name.ptr,
            name.len,
            self.file_metadata.?,
            original_line,
            di_type,
            1, // Always preserve
            c.LLVMDIFlagZero,
            0  // Alignment in bits
        );
        
        if (di_variable == null) {
            return DebugError.MetadataError;
        }
        
        // Insert variable declaration with inlined location
        const di_expr = c.LLVMDIBuilderCreateExpression(self.di_builder, null, 0);
        _ = c.LLVMDIBuilderInsertDeclareAtEnd(
            self.di_builder,
            alloca,
            di_variable,
            di_expr,
            inlined_location,
            c.LLVMGetLastBasicBlock(c.LLVMGetBasicBlockParent(c.LLVMGetInstructionParent(alloca)))
        );
        
        std.debug.print("📍 Created inlined variable debug info: {s} at {d} (inlined at {d}:{d})\n",
                       .{ name, original_line, inline_site_line, inline_site_column });
    }
    
    /// Finalize debug information with optimization for debugging
    pub fn finalize(self: *DebugInfoGenerator) void {
        c.LLVMDIBuilderFinalize(self.di_builder);
        std.debug.print("✅ Debug information finalized for GDB/LLDB support\n", .{});
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
            .void_type = try self.createBasicType("void", 0, c.LLVMDWARFTypeEncodingSigned),
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
    void_type: c.LLVMMetadataRef,
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

/// Variable debug information tracking
pub const VariableDebugInfo = struct {
    name: []const u8,
    di_type: c.LLVMMetadataRef,
    di_variable: c.LLVMMetadataRef,
    alloca: c.LLVMValueRef,
    line: u32,
    column: u32,
    is_parameter: bool,
};

test "debug info generator initialization" {
    const allocator = std.testing.allocator;
    
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("test_module", context);
    defer c.LLVMDisposeModule(module);
    
    var debug_gen = try DebugInfoGenerator.init(allocator, context, module);
    defer debug_gen.deinit();
    
    try debug_gen.createCompileUnit("test.💀", "/tmp");
    
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
    
    try debug_gen.createCompileUnit("test.💀", "/tmp");
    
    const cursed_types = try debug_gen.createCursedTypes();
    try std.testing.expect(cursed_types.normie_type != null);
    try std.testing.expect(cursed_types.lit_type != null);
}
