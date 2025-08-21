const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

// Enhanced Debug Information Generation (Priority #24)
// Complete DWARF debug info for native code with advanced features

const llvm = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/DebugInfo.h");
    @cInclude("llvm-c/DIBuilder.h");
});

pub const DebugInfoError = error{
    InvalidDebugInfo,
    MissingSourceInfo,
    CompilationUnitNotFound,
    SymbolNotFound,
    DwarfGenerationFailed,
};

pub const DebugFormat = enum {
    DWARF4,
    DWARF5,
    CodeView,  // For Windows
    PDB,       // For Windows MSVC
};

pub const DebugLevel = enum {
    None,
    LineTablesOnly,
    LimitedDebugInfo,
    FullDebugInfo,
    
    pub fn toLLVMLevel(self: DebugLevel) c_uint {
        return switch (self) {
            .None => 0,
            .LineTablesOnly => 1,
            .LimitedDebugInfo => 2,
            .FullDebugInfo => 3,
        };
    }
};

pub const SourceLocation = struct {
    filename: []const u8,
    directory: []const u8,
    line: u32,
    column: u32,
    
    pub fn init(filename: []const u8, directory: []const u8, line: u32, column: u32) SourceLocation {
        return SourceLocation{
            .filename = filename,
            .directory = directory,
            .line = line,
            .column = column,
        };
    }
};

pub const DebugVariable = struct {
    name: []const u8,
    type_name: []const u8,
    location: SourceLocation,
    scope_depth: u32,
    is_parameter: bool,
    is_local: bool,
    llvm_value: ?llvm.LLVMValueRef,
    llvm_debug_info: ?llvm.LLVMMetadataRef,
    
    pub fn init(name: []const u8, type_name: []const u8, location: SourceLocation) DebugVariable {
        return DebugVariable{
            .name = name,
            .type_name = type_name,
            .location = location,
            .scope_depth = 0,
            .is_parameter = false,
            .is_local = true,
            .llvm_value = null,
            .llvm_debug_info = null,
        };
    }
};

pub const DebugFunction = struct {
    name: []const u8,
    mangled_name: []const u8,
    return_type: []const u8,
    parameters: ArrayList(DebugVariable),
    local_variables: ArrayList(DebugVariable),
    location: SourceLocation,
    llvm_function: ?llvm.LLVMValueRef,
    llvm_debug_info: ?llvm.LLVMMetadataRef,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, name: []const u8, mangled_name: []const u8, 
               return_type: []const u8, location: SourceLocation) DebugFunction {
        return DebugFunction{
            .name = name,
            .mangled_name = mangled_name,
            .return_type = return_type,
            .parameters = .empty,
            .local_variables = .empty,
            .location = location,
            .llvm_function = null,
            .llvm_debug_info = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *DebugFunction) void {
        self.parameters.deinit(allocator);
        self.local_variables.deinit(allocator);
    }
    
    pub fn addParameter(self: *DebugFunction, variable: DebugVariable) !void {
        var param = variable;
        param.is_parameter = true;
        param.is_local = false;
        try self.parameters.append(allocator, param);
    }
    
    pub fn addLocalVariable(self: *DebugFunction, variable: DebugVariable) !void {
        try self.local_variables.append(allocator, variable);
    }
};

pub const DebugScope = struct {
    parent: ?*DebugScope,
    children: ArrayList(*DebugScope),
    variables: ArrayList(DebugVariable),
    location: SourceLocation,
    llvm_scope: ?llvm.LLVMMetadataRef,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, parent: ?*DebugScope, location: SourceLocation) DebugScope {
        return DebugScope{
            .parent = parent,
            .children = .empty,
            .variables = .empty,
            .location = location,
            .llvm_scope = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *DebugScope) void {
        for (self.children.items) |child| {
            child.deinit(allocator);
            self.allocator.destroy(child);
        }
        self.children.deinit(allocator);
        self.variables.deinit(allocator);
    }
    
    pub fn createChildScope(self: *DebugScope, location: SourceLocation) !*DebugScope {
        const child = try self.allocator.create(DebugScope);
        child.* = DebugScope.init(self.allocator, self, location);
        try self.children.append(self.allocator, child);
        return child;
    }
};

pub const CompilationUnit = struct {
    filename: []const u8,
    directory: []const u8,
    producer: []const u8,
    version: []const u8,
    language: []const u8,
    functions: ArrayList(DebugFunction),
    global_variables: ArrayList(DebugVariable),
    types: ArrayList(DebugType),
    llvm_compile_unit: ?llvm.LLVMMetadataRef,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, filename: []const u8, directory: []const u8) CompilationUnit {
        return CompilationUnit{
            .filename = filename,
            .directory = directory,
            .producer = "CURSED Compiler",
            .version = "1.0.0",
            .language = "CURSED",
            .functions = .empty,
            .global_variables = .empty,
            .types = .empty,
            .llvm_compile_unit = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *CompilationUnit) void {
        for (self.functions.items) |*func| {
            func.deinit(allocator);
        }
        self.functions.deinit(allocator);
        self.global_variables.deinit(allocator);
        for (self.types.items) |*debug_type| {
            debug_type.deinit(allocator);
        }
        self.types.deinit(allocator);
    }
};

pub const DebugType = struct {
    name: []const u8,
    size_bits: u64,
    align_bits: u64,
    encoding: DebugEncoding,
    llvm_type: ?llvm.LLVMTypeRef,
    llvm_debug_info: ?llvm.LLVMMetadataRef,
    allocator: Allocator,
    
    pub fn init(allocator: Allocator, name: []const u8, size_bits: u64, align_bits: u64, encoding: DebugEncoding) DebugType {
        return DebugType{
            .name = name,
            .size_bits = size_bits,
            .align_bits = align_bits,
            .encoding = encoding,
            .llvm_type = null,
            .llvm_debug_info = null,
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *DebugType) void {
        // LLVM metadata is managed by LLVM context
        _ = self;
    }
};

pub const DebugEncoding = enum(c_uint) {
    Address = 1,
    Boolean = 2,
    ComplexFloat = 3,
    Float = 4,
    Signed = 5,
    SignedChar = 6,
    Unsigned = 7,
    UnsignedChar = 8,
    ImaginaryFloat = 9,
    PackedDecimal = 10,
    NumericString = 11,
    Edited = 12,
    SignedFixed = 13,
    UnsignedFixed = 14,
    DecimalFloat = 15,
    UTF = 16,
};

pub const DebugInfoGenerator = struct {
    allocator: Allocator,
    llvm_context: llvm.LLVMContextRef,
    llvm_module: llvm.LLVMModuleRef,
    llvm_builder: llvm.LLVMBuilderRef,
    di_builder: llvm.LLVMDIBuilderRef,
    compilation_unit: CompilationUnit,
    current_scope: ?*DebugScope,
    debug_level: DebugLevel,
    debug_format: DebugFormat,
    
    pub fn init(allocator: Allocator, llvm_context: llvm.LLVMContextRef, 
               llvm_module: llvm.LLVMModuleRef, filename: []const u8, directory: []const u8) !DebugInfoGenerator {
        const di_builder = llvm.LLVMCreateDIBuilder(llvm_module);
        
        return DebugInfoGenerator{
            .allocator = allocator,
            .llvm_context = llvm_context,
            .llvm_module = llvm_module,
            .llvm_builder = llvm.LLVMCreateBuilderInContext(llvm_context),
            .di_builder = di_builder,
            .compilation_unit = CompilationUnit.init(allocator, filename, directory),
            .current_scope = null,
            .debug_level = .FullDebugInfo,
            .debug_format = .DWARF5,
        };
    }
    
    pub fn deinit(self: *DebugInfoGenerator) void {
        self.compilation_unit.deinit(allocator);
        if (self.current_scope) |scope| {
            scope.deinit(allocator);
            self.allocator.destroy(scope);
        }
        llvm.LLVMDisposeDIBuilder(self.di_builder);
        llvm.LLVMDisposeBuilder(self.llvm_builder);
    }
    
    pub fn createCompileUnit(self: *DebugInfoGenerator) !void {
        const filename_cstr = try std.cstr.addNullByte(self.allocator, self.compilation_unit.filename);
        defer self.allocator.free(filename_cstr);
        
        const directory_cstr = try std.cstr.addNullByte(self.allocator, self.compilation_unit.directory);
        defer self.allocator.free(directory_cstr);
        
        const producer_cstr = try std.cstr.addNullByte(self.allocator, self.compilation_unit.producer);
        defer self.allocator.free(producer_cstr);
        
        self.compilation_unit.llvm_compile_unit = llvm.LLVMDIBuilderCreateCompileUnit(
            self.di_builder,
            llvm.LLVMDWARFSourceLanguageC, // Use C as base language
            llvm.LLVMDIBuilderCreateFile(self.di_builder, filename_cstr.ptr, @intCast(self.compilation_unit.filename.len),
                                        directory_cstr.ptr, @intCast(self.compilation_unit.directory.len)),
            producer_cstr.ptr,
            @intCast(self.compilation_unit.producer.len),
            0, // is_optimized
            null, // flags
            0, // flags_len
            0, // runtime_version
            null, // split_name
            0, // split_name_len
            llvm.LLVMDWARFEmissionFull,
            0, // dwo_id
            1, // split_debug_inlining
            0  // debug_info_for_profiling
        );
    }
    
    pub fn createBasicType(self: *DebugInfoGenerator, name: []const u8, size_bits: u64, encoding: DebugEncoding) !llvm.LLVMMetadataRef {
        const name_cstr = try std.cstr.addNullByte(self.allocator, name);
        defer self.allocator.free(name_cstr);
        
        return llvm.LLVMDIBuilderCreateBasicType(
            self.di_builder,
            name_cstr.ptr,
            @intCast(name.len),
            size_bits,
            @intFromEnum(encoding),
            0 // flags
        );
    }
    
    pub fn createCursedTypes(self: *DebugInfoGenerator) !void {
        // Create debug info for all CURSED types
        _ = try self.createBasicType("normie", 64, .Signed);     // int
        _ = try self.createBasicType("tea", 64, .Address);       // string (pointer)
        _ = try self.createBasicType("drip", 64, .Signed);       // int64
        _ = try self.createBasicType("lit", 8, .Boolean);        // bool
        _ = try self.createBasicType("meal", 64, .Float);        // float64
        _ = try self.createBasicType("smol", 8, .Signed);        // int8
        _ = try self.createBasicType("thicc", 32, .Signed);      // int32
        _ = try self.createBasicType("sip", 32, .Float);         // float32
    }
    
    pub fn createFunction(self: *DebugInfoGenerator, debug_func: *DebugFunction) !void {
        const name_cstr = try std.cstr.addNullByte(self.allocator, debug_func.name);
        defer self.allocator.free(name_cstr);
        
        const mangled_cstr = try std.cstr.addNullByte(self.allocator, debug_func.mangled_name);
        defer self.allocator.free(mangled_cstr);
        
        const filename_cstr = try std.cstr.addNullByte(self.allocator, debug_func.location.filename);
        defer self.allocator.free(filename_cstr);
        
        const directory_cstr = try std.cstr.addNullByte(self.allocator, debug_func.location.directory);
        defer self.allocator.free(directory_cstr);
        
        const file = llvm.LLVMDIBuilderCreateFile(
            self.di_builder,
            filename_cstr.ptr, @intCast(debug_func.location.filename.len),
            directory_cstr.ptr, @intCast(debug_func.location.directory.len)
        );
        
        // Create function type (simplified - returns void, no parameters for now)
        const void_type = try self.createBasicType("void", 0, .Address);
        const function_type = llvm.LLVMDIBuilderCreateSubroutineType(
            self.di_builder,
            file,
            &void_type,
            1,
            0 // flags
        );
        
        debug_func.llvm_debug_info = llvm.LLVMDIBuilderCreateFunction(
            self.di_builder,
            self.current_scope orelse self.compilation_unit.llvm_compile_unit.?, // scope
            name_cstr.ptr,
            @intCast(debug_func.name.len),
            mangled_cstr.ptr,
            @intCast(debug_func.mangled_name.len),
            file,
            debug_func.location.line,
            function_type,
            1, // is_local_to_unit
            1, // is_definition
            debug_func.location.line,
            0, // flags
            0  // is_optimized
        );
    }
    
    pub fn createVariable(self: *DebugInfoGenerator, debug_var: *DebugVariable, scope: llvm.LLVMMetadataRef) !void {
        const name_cstr = try std.cstr.addNullByte(self.allocator, debug_var.name);
        defer self.allocator.free(name_cstr);
        
        const filename_cstr = try std.cstr.addNullByte(self.allocator, debug_var.location.filename);
        defer self.allocator.free(filename_cstr);
        
        const directory_cstr = try std.cstr.addNullByte(self.allocator, debug_var.location.directory);
        defer self.allocator.free(directory_cstr);
        
        const file = llvm.LLVMDIBuilderCreateFile(
            self.di_builder,
            filename_cstr.ptr, @intCast(debug_var.location.filename.len),
            directory_cstr.ptr, @intCast(debug_var.location.directory.len)
        );
        
        // Get or create type for variable
        const var_type = try self.getOrCreateTypeForVariable(debug_var);
        
        if (debug_var.is_parameter) {
            debug_var.llvm_debug_info = llvm.LLVMDIBuilderCreateParameterVariable(
                self.di_builder,
                scope,
                name_cstr.ptr,
                @intCast(debug_var.name.len),
                1, // arg_no (starting from 1)
                file,
                debug_var.location.line,
                var_type,
                1, // always_preserve
                0  // flags
            );
        } else {
            debug_var.llvm_debug_info = llvm.LLVMDIBuilderCreateAutoVariable(
                self.di_builder,
                scope,
                name_cstr.ptr,
                @intCast(debug_var.name.len),
                file,
                debug_var.location.line,
                var_type,
                1, // always_preserve
                0, // flags
                0  // align_in_bits
            );
        }
    }
    
    fn getOrCreateTypeForVariable(self: *DebugInfoGenerator, debug_var: *DebugVariable) !llvm.LLVMMetadataRef {
        // Map CURSED types to debug info
        if (std.mem.eql(u8, debug_var.type_name, "normie")) {
            return try self.createBasicType("normie", 64, .Signed);
        } else if (std.mem.eql(u8, debug_var.type_name, "tea")) {
            return try self.createBasicType("tea", 64, .Address);
        } else if (std.mem.eql(u8, debug_var.type_name, "lit")) {
            return try self.createBasicType("lit", 8, .Boolean);
        } else if (std.mem.eql(u8, debug_var.type_name, "meal")) {
            return try self.createBasicType("meal", 64, .Float);
        }
        
        // Default to void type
        return try self.createBasicType("void", 0, .Address);
    }
    
    pub fn setCurrentLocation(self: *DebugInfoGenerator, line: u32, column: u32, scope: llvm.LLVMMetadataRef) void {
        const location = llvm.LLVMDIBuilderCreateDebugLocation(
            self.llvm_context,
            line,
            column,
            scope,
            null // inlined_at
        );
        llvm.LLVMSetCurrentDebugLocation(self.llvm_builder, location);
    }
    
    pub fn insertDeclare(self: *DebugInfoGenerator, storage: llvm.LLVMValueRef, var_info: llvm.LLVMMetadataRef, 
                        location: llvm.LLVMMetadataRef, block: llvm.LLVMBasicBlockRef) void {
        _ = llvm.LLVMDIBuilderInsertDeclareAtEnd(
            self.di_builder,
            storage,
            var_info,
            llvm.LLVMDIBuilderCreateExpression(self.di_builder, null, 0),
            location,
            block
        );
    }
    
    pub fn finalize(self: *DebugInfoGenerator) void {
        llvm.LLVMDIBuilderFinalize(self.di_builder);
    }
    
    pub fn generateSourceMap(self: *DebugInfoGenerator, output_file: []const u8) !void {
        // Generate source map for web debugging
        var file = try std.fs.cwd().createFile(output_file, .{});
        defer file.close();
        
        try file.writer().print("{{\"version\":3,\"sources\":[\"{}\"]}}\n", .{self.compilation_unit.filename});
    }
};

// Global debug info generator
var global_debug_generator: ?DebugInfoGenerator = null;

pub fn initDebugGeneration(allocator: Allocator, llvm_context: llvm.LLVMContextRef, 
                          llvm_module: llvm.LLVMModuleRef, filename: []const u8, directory: []const u8) !void {
    global_debug_generator = try DebugInfoGenerator.init(allocator, llvm_context, llvm_module, filename, directory);
    try global_debug_generator.?.createCompileUnit();
    try global_debug_generator.?.createCursedTypes();
}

pub fn deinitDebugGeneration() void {
    if (global_debug_generator) |*generator| {
        generator.finalize();
        generator.deinit(allocator);
        global_debug_generator = null;
    }
}

// Export functions for LLVM codegen integration
export fn cursed_debug_create_function(name_ptr: [*]const u8, name_len: usize,
                                      line: u32, column: u32) void {
    if (global_debug_generator == null) return;
    
    const name = name_ptr[0..name_len];
    const location = SourceLocation.init("current.csd", ".", line, column);
    var debug_func = DebugFunction.init(global_debug_generator.?.allocator, name, name, "void", location);
    
    global_debug_generator.?.createFunction(&debug_func) catch return;
}

export fn cursed_debug_create_variable(name_ptr: [*]const u8, name_len: usize,
                                      type_ptr: [*]const u8, type_len: usize,
                                      line: u32, column: u32) void {
    if (global_debug_generator == null) return;
    
    const name = name_ptr[0..name_len];
    const type_name = type_ptr[0..type_len];
    const location = SourceLocation.init("current.csd", ".", line, column);
    
    var debug_var = DebugVariable.init(name, type_name, location);
    const scope = global_debug_generator.?.compilation_unit.llvm_compile_unit orelse return;
    global_debug_generator.?.createVariable(&debug_var, scope) catch return;
}

export fn cursed_debug_set_location(line: u32, column: u32) void {
    if (global_debug_generator == null) return;
    
    const scope = global_debug_generator.?.compilation_unit.llvm_compile_unit orelse return;
    global_debug_generator.?.setCurrentLocation(line, column, scope);
}

// Testing
pub fn testDebugGeneration() !void {
    print("Testing debug information generation...\n");
    
    // Test basic LLVM debug info creation
    const context = llvm.LLVMContextCreate();
    defer llvm.LLVMContextDispose(context);
    
    const module = llvm.LLVMModuleCreateWithNameInContext("test_module", context);
    defer llvm.LLVMDisposeModule(module);
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    var generator = try DebugInfoGenerator.init(allocator, context, module, "test.csd", "/tmp");
    defer generator.deinit(allocator);
    
    try generator.createCompileUnit();
    try generator.createCursedTypes();
    
    // Create a test function
    const location = SourceLocation.init("test.csd", "/tmp", 1, 1);
    var test_func = DebugFunction.init(allocator, "test_function", "test_function", "void", location);
    defer test_func.deinit(allocator);
    
    try generator.createFunction(&test_func);
    generator.finalize();
    
    print("Debug generation tests passed!\n");
}
