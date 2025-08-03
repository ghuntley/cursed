const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const type_system = @import("type_system_runtime.zig");
const RuntimeTypeInfo = type_system.RuntimeTypeInfo;
const GCTypeRegistry = type_system.GCTypeRegistry;
const TypedAllocator = type_system.TypedAllocator;
const InterfaceRegistry = type_system.InterfaceRegistry;
const TypeChecker = type_system.TypeChecker;
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/Transforms/PassManagerBuilder.h");
    @cInclude("llvm-c/Transforms/IPO.h");
    @cInclude("llvm-c/Transforms/Scalar.h");
});

const ast = @import("ast.zig");
const CodeGen = @import("codegen.zig").CodeGen;
const CodeGenError = @import("codegen.zig").CodeGenError;

/// Advanced CURSED Zig Code Generator with advanced language features
/// Handles structs, interfaces, generics, and advanced memory management
pub const AdvancedCodeGen = struct {
    base_codegen: CodeGen,
    
    // Advanced type system support
    struct_types: HashMap([]const u8, StructTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    interface_types: HashMap([]const u8, InterfaceTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    generic_instances: HashMap([]const u8, GenericInstance, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    vtables: HashMap([]const u8, VTableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Enhanced type system runtime support
    gc_type_registry: GCTypeRegistry,
    typed_allocator: TypedAllocator,
    interface_registry: InterfaceRegistry,
    type_checker: TypeChecker,
    
    // Memory management
    gc_enabled: bool,
    heap_allocator: ?c.LLVMValueRef,
    gc_mark_func: ?c.LLVMValueRef,
    gc_sweep_func: ?c.LLVMValueRef,
    
    // Optimization state
    optimization_passes: ArrayList(OptimizationPass),
    
    pub fn init(allocator: Allocator) AdvancedCodeGen {
        var gc_registry = GCTypeRegistry.init(allocator);
        var interface_registry = InterfaceRegistry.init(allocator);
        
        return AdvancedCodeGen{
            .base_codegen = CodeGen.init(allocator),
            .struct_types = HashMap([]const u8, StructTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .interface_types = HashMap([]const u8, InterfaceTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .generic_instances = HashMap([]const u8, GenericInstance, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .vtables = HashMap([]const u8, VTableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .gc_type_registry = gc_registry,
            .typed_allocator = TypedAllocator.init(allocator, &gc_registry),
            .interface_registry = interface_registry,
            .type_checker = TypeChecker.init(&gc_registry, &interface_registry),
            .gc_enabled = true,
            .heap_allocator = null,
            .gc_mark_func = null,
            .gc_sweep_func = null,
            .optimization_passes = ArrayList(OptimizationPass).init(allocator),
        };
    }

    pub fn deinit(self: *AdvancedCodeGen) void {
        self.base_codegen.deinit();
        self.struct_types.deinit();
        self.interface_types.deinit();
        self.generic_instances.deinit();
        self.vtables.deinit();
        self.gc_type_registry.deinit();
        self.typed_allocator.deinit();
        self.interface_registry.deinit();
        self.optimization_passes.deinit();
    }

    /// Generate advanced program with struct/interface/generic support
    pub fn generateAdvancedProgram(self: *AdvancedCodeGen, program: ast.Program) CodeGenError!void {
        // Initialize memory management
        try self.initializeMemoryManagement();
        
        // First pass: collect type definitions
        try self.collectTypeDefinitions(program);
        
        // Second pass: generate struct types
        try self.generateStructTypes();
        
        // Third pass: generate interface vtables
        try self.generateInterfaceVTables();
        
        // Fourth pass: process generic instantiations
        try self.processGenericInstantiations();
        
        // Fifth pass: generate code
        try self.base_codegen.generateProgram(program);
        
        // Final pass: apply optimizations
        try self.applyAdvancedOptimizations();
    }

    /// Initialize memory management system
    fn initializeMemoryManagement(self: *AdvancedCodeGen) CodeGenError!void {
        if (!self.gc_enabled) return;
        
        // Create GC-aware heap allocator
        const allocator_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), // return void*
            &[_]c.LLVMTypeRef{
                c.LLVMInt64TypeInContext(self.base_codegen.context), // size
                c.LLVMInt8TypeInContext(self.base_codegen.context),  // type_id
            },
            2, // parameter count
            0  // not variadic
        );
        
        self.heap_allocator = c.LLVMAddFunction(self.base_codegen.module, "gc_alloc", allocator_type);
        try self.base_codegen.functions.put("gc_alloc", self.heap_allocator.?);
        
        // Create GC mark function
        const mark_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.base_codegen.context), // return void
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0)}, // ptr
            1, // parameter count
            0  // not variadic
        );
        
        self.gc_mark_func = c.LLVMAddFunction(self.base_codegen.module, "gc_mark", mark_type);
        try self.base_codegen.functions.put("gc_mark", self.gc_mark_func.?);
        
        // Create GC sweep function
        const sweep_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.base_codegen.context), // return void
            null, // no parameters
            0, // parameter count
            0  // not variadic
        );
        
        self.gc_sweep_func = c.LLVMAddFunction(self.base_codegen.module, "gc_sweep", sweep_type);
        try self.base_codegen.functions.put("gc_sweep", self.gc_sweep_func.?);
        
        // Generate GC runtime support
        try self.generateGCRuntime();
    }

    /// Generate garbage collection runtime support
    fn generateGCRuntime(self: *AdvancedCodeGen) CodeGenError!void {
        // Generate gc_alloc implementation
        const alloc_func = self.heap_allocator.?;
        const alloc_entry = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, alloc_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, alloc_entry);
        
        // Get parameters
        const size_param = c.LLVMGetParam(alloc_func, 0);
        const type_id_param = c.LLVMGetParam(alloc_func, 1);
        
        // Call system malloc with GC header
        const header_size = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.base_codegen.context), 16, 0);
        const total_size = c.LLVMBuildAdd(self.base_codegen.builder, size_param, header_size, "total_size");
        
        const malloc_func = self.base_codegen.functions.get("malloc").?;
        const raw_ptr = c.LLVMBuildCall2(
            self.base_codegen.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(malloc_func)),
            malloc_func,
            &[_]c.LLVMValueRef{total_size},
            1,
            "raw_ptr"
        );
        
        // Initialize GC header
        const header_ptr = c.LLVMBuildBitCast(
            self.base_codegen.builder,
            raw_ptr,
            c.LLVMPointerType(c.LLVMInt64TypeInContext(self.base_codegen.context), 0),
            "header_ptr"
        );
        
        // Store size in header
        const size_ptr = c.LLVMBuildGEP2(
            self.base_codegen.builder,
            c.LLVMInt64TypeInContext(self.base_codegen.context),
            header_ptr,
            &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), 0, 0)},
            1,
            "size_ptr"
        );
        _ = c.LLVMBuildStore(self.base_codegen.builder, size_param, size_ptr);
        
        // Store type_id in header
        const type_id_ptr = c.LLVMBuildGEP2(
            self.base_codegen.builder,
            c.LLVMInt64TypeInContext(self.base_codegen.context),
            header_ptr,
            &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), 1, 0)},
            1,
            "type_id_ptr"
        );
        const type_id_ext = c.LLVMBuildZExt(self.base_codegen.builder, type_id_param, c.LLVMInt64TypeInContext(self.base_codegen.context), "type_id_ext");
        _ = c.LLVMBuildStore(self.base_codegen.builder, type_id_ext, type_id_ptr);
        
        // Return pointer to user data (after header)
        const user_ptr = c.LLVMBuildGEP2(
            self.base_codegen.builder,
            c.LLVMInt8TypeInContext(self.base_codegen.context),
            raw_ptr,
            &[_]c.LLVMValueRef{header_size},
            1,
            "user_ptr"
        );
        _ = c.LLVMBuildRet(self.base_codegen.builder, user_ptr);
    }

    /// Collect struct and interface type definitions from the program
    fn collectTypeDefinitions(self: *AdvancedCodeGen, program: ast.Program) CodeGenError!void {
        for (program.statements.items) |stmt| {
            switch (stmt) {
                .Struct => |struct_stmt| {
                    try self.collectStructDefinition(struct_stmt);
                },
                .Interface => |interface_stmt| {
                    try self.collectInterfaceDefinition(interface_stmt);
                },
                else => {},
            }
        }
    }

    /// Collect struct definition information
    fn collectStructDefinition(self: *AdvancedCodeGen, struct_stmt: ast.StructStatement) CodeGenError!void {
        var field_types = ArrayList(c.LLVMTypeRef).init(self.base_codegen.allocator);
        defer field_types.deinit();
        
        var field_names = ArrayList([]const u8).init(self.base_codegen.allocator);
        defer field_names.deinit();
        
        for (struct_stmt.fields.items) |field| {
            const field_type = try self.base_codegen.getLLVMType(field.field_type);
            try field_types.append(field_type);
            try field_names.append(field.name);
        }
        
        const struct_info = StructTypeInfo{
            .name = struct_stmt.name,
            .field_types = try field_types.toOwnedSlice(),
            .field_names = try field_names.toOwnedSlice(),
            .llvm_type = null, // Will be set during generation
            .methods = ArrayList(MethodInfo).init(self.base_codegen.allocator),
            .is_generic = struct_stmt.type_parameters.items.len > 0,
            .type_parameters = struct_stmt.type_parameters,
        };
        
        try self.struct_types.put(struct_stmt.name, struct_info);
    }

    /// Collect interface definition information
    fn collectInterfaceDefinition(self: *AdvancedCodeGen, interface_stmt: ast.InterfaceStatement) CodeGenError!void {
        var methods = ArrayList(InterfaceMethodInfo).init(self.base_codegen.allocator);
        
        for (interface_stmt.methods.items, 0..) |method, index| {
            const method_info = InterfaceMethodInfo{
                .name = method.name,
                .index = index,
                .signature = method, // Store full signature
            };
            try methods.append(method_info);
        }
        
        const interface_info = InterfaceTypeInfo{
            .name = interface_stmt.name,
            .methods = methods,
            .is_generic = interface_stmt.type_parameters.items.len > 0,
            .type_parameters = interface_stmt.type_parameters,
        };
        
        try self.interface_types.put(interface_stmt.name, interface_info);
    }

    /// Generate LLVM struct types
    fn generateStructTypes(self: *AdvancedCodeGen) CodeGenError!void {
        var iterator = self.struct_types.iterator();
        while (iterator.next()) |entry| {
            const struct_info = entry.value_ptr;
            
            if (struct_info.is_generic) {
                // Skip generic structs - they'll be generated during instantiation
                continue;
            }
            
            // Create LLVM struct type
            const struct_type = c.LLVMStructCreateNamed(self.base_codegen.context, struct_info.name.ptr);
            c.LLVMStructSetBody(struct_type, struct_info.field_types.ptr, @as(u32, @intCast(0));
            
            struct_info.llvm_type = struct_type;
        }
    }

    /// Generate interface vtables
    fn generateInterfaceVTables(self: *AdvancedCodeGen) CodeGenError!void {
        var interface_iterator = self.interface_types.iterator();
        while (interface_iterator.next()) |interface_entry| {
            const interface_info = interface_entry.value_ptr;
            
            if (interface_info.is_generic) {
                // Skip generic interfaces - they'll be generated during instantiation
                continue;
            }
            
            // For each struct that implements this interface
            var struct_iterator = self.struct_types.iterator();
            while (struct_iterator.next()) |struct_entry| {
                const struct_info = struct_entry.value_ptr;
                
                if (self.structImplementsInterface(struct_info, interface_info)) {
                    try self.generateVTableForImplementation(struct_info, interface_info);
                }
            }
        }
    }

    /// Check if a struct implements an interface
    fn structImplementsInterface(self: *AdvancedCodeGen, struct_info: *StructTypeInfo, interface_info: *InterfaceTypeInfo) bool {
        _ = self;
        
        // For each method in the interface
        for (interface_info.methods.items) |interface_method| {
            var found = false;
            
            // Check if struct has a method with the same signature
            for (struct_info.methods.items) |struct_method| {
                if (std.mem.eql(u8, interface_method.name, struct_method.name)) {
                    // TODO: Add proper signature checking
                    found = true;
                    break;
                }
            }
            
            if (!found) return false;
        }
        
        return true;
    }

    /// Generate vtable for struct implementing interface
    fn generateVTableForImplementation(self: *AdvancedCodeGen, struct_info: *StructTypeInfo, interface_info: *InterfaceTypeInfo) CodeGenError!void {
        var vtable_name = ArrayList(u8).init(self.base_codegen.allocator);
        defer vtable_name.deinit();
        
        try vtable_name.appendSlice(struct_info.name);
        try vtable_name.appendSlice("_");
        try vtable_name.appendSlice(interface_info.name);
        try vtable_name.appendSlice("_vtable");
        
        // Create vtable type (array of function pointers)
        const method_count = interface_info.methods.items.len;
        const func_ptr_type = c.LLVMPointerType(
            c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.base_codegen.context), // return type (placeholder)
                null, // parameters (placeholder)
                0, // parameter count
                0  // not variadic
            ),
            0
        );
        const vtable_type = c.LLVMArrayType(func_ptr_type, @as(u32, @intCast(method_count)));
        
        // Create vtable global variable
        const vtable_global = c.LLVMAddGlobal(self.base_codegen.module, vtable_type, vtable_name.items.ptr);
        c.LLVMSetLinkage(vtable_global, c.LLVMInternalLinkage);
        
        // Initialize vtable with method pointers
        var method_pointers = ArrayList(c.LLVMValueRef).init(self.base_codegen.allocator);
        defer method_pointers.deinit();
        
        for (interface_info.methods.items) |interface_method| {
            // Find corresponding method in struct
            const method_name = try self.findMethodImplementation(struct_info, interface_method.name);
            const method_func = self.base_codegen.functions.get(method_name) orelse {
                return CodeGenError.UndefinedSymbol;
            };
            
            try method_pointers.append(method_func);
        }
        
        // Create constant array initializer
        const vtable_init = c.LLVMConstArray(func_ptr_type, method_pointers.items.ptr, @as(u32, @intCast(method_pointers.items.len)));
        c.LLVMSetInitializer(vtable_global, vtable_init);
        
        // Store vtable info
        const vtable_info = VTableInfo{
            .name = try vtable_name.toOwnedSlice(),
            .interface_name = interface_info.name,
            .struct_name = struct_info.name,
            .global_value = vtable_global,
            .method_count = method_count,
        };
        
        try self.vtables.put(vtable_info.name, vtable_info);
    }

    /// Find method implementation name for struct
    fn findMethodImplementation(self: *AdvancedCodeGen, struct_info: *StructTypeInfo, method_name: []const u8) CodeGenError![]const u8 {
        _ = self;
        
        for (struct_info.methods.items) |method| {
            if (std.mem.eql(u8, method.name, method_name)) {
                return method.llvm_name;
            }
        }
        
        return CodeGenError.UndefinedSymbol;
    }

    /// Process generic instantiations
    fn processGenericInstantiations(self: *AdvancedCodeGen) CodeGenError!void {
        // This would be called when generic types are instantiated with concrete types
        // For now, we'll implement a placeholder
        _ = self;
    }

    /// Generate struct construction
    pub fn generateStructConstruction(self: *AdvancedCodeGen, struct_name: []const u8, field_values: []c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        const struct_info = self.struct_types.get(struct_name) orelse {
            return CodeGenError.UndefinedSymbol;
        };
        
        if (struct_info.llvm_type == null) {
            return CodeGenError.InvalidType;
        }
        
        // Allocate memory for struct
        const struct_size = c.LLVMSizeOf(struct_info.llvm_type.?);
        const struct_ptr = c.LLVMBuildCall2(
            self.base_codegen.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(self.heap_allocator.?)),
            self.heap_allocator.?,
            &[_]c.LLVMValueRef{
                struct_size,
                c.LLVMConstInt(c.LLVMInt8TypeInContext(self.base_codegen.context), 1, 0) // struct type_id
            },
            2,
            "struct_alloc"
        );
        
        // Cast to proper struct pointer type
        const typed_ptr = c.LLVMBuildBitCast(
            self.base_codegen.builder,
            struct_ptr,
            c.LLVMPointerType(struct_info.llvm_type.?, 0),
            "struct_ptr"
        );
        
        // Initialize fields
        for (field_values, 0..) |value, i| {
            const field_ptr = c.LLVMBuildStructGEP2(
                self.base_codegen.builder,
                struct_info.llvm_type.?,
                typed_ptr,
                @as(u32, @intCast(i)),
                "field_ptr"
            );
            _ = c.LLVMBuildStore(self.base_codegen.builder, value, field_ptr);
        }
        
        return typed_ptr;
    }

    /// Generate struct field access
    pub fn generateStructFieldAccess(self: *AdvancedCodeGen, struct_ptr: c.LLVMValueRef, struct_name: []const u8, field_name: []const u8) CodeGenError!c.LLVMValueRef {
        const struct_info = self.struct_types.get(struct_name) orelse {
            return CodeGenError.UndefinedSymbol;
        };
        
        // Find field index
        var field_index: u32 = 0;
        var found = false;
        for (struct_info.field_names, 0..) |name, i| {
            if (std.mem.eql(u8, name, field_name)) {
                field_index = @as(u32, @intCast(i));
                found = true;
                break;
            }
        }
        
        if (!found) {
            return CodeGenError.UndefinedSymbol;
        }
        
        // Generate field access
        const field_ptr = c.LLVMBuildStructGEP2(
            self.base_codegen.builder,
            struct_info.llvm_type.?,
            struct_ptr,
            field_index,
            "field_ptr"
        );
        
        return c.LLVMBuildLoad2(
            self.base_codegen.builder,
            struct_info.field_types[field_index],
            field_ptr,
            "field_value"
        );
    }

    /// Generate interface method call with dynamic dispatch
    pub fn generateInterfaceMethodCall(self: *AdvancedCodeGen, interface_ptr: c.LLVMValueRef, method_name: []const u8, args: []c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        // Extract vtable from interface object
        const vtable_ptr_ptr = c.LLVMBuildStructGEP2(
            self.base_codegen.builder,
            c.LLVMStructTypeInContext(self.base_codegen.context, null, 0, 0), // placeholder
            interface_ptr,
            0, // vtable is first field
            "vtable_ptr_ptr"
        );
        
        const vtable_ptr = c.LLVMBuildLoad2(
            self.base_codegen.builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0),
            vtable_ptr_ptr,
            "vtable_ptr"
        );
        
        // TODO: Find method index in interface
        const method_index = 0; // Placeholder
        
        // Get method function pointer from vtable
        const method_ptr_ptr = c.LLVMBuildGEP2(
            self.base_codegen.builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0),
            vtable_ptr,
            &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), method_index, 0)},
            1,
            "method_ptr_ptr"
        );
        
        const method_func = c.LLVMBuildLoad2(
            self.base_codegen.builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0),
            method_ptr_ptr,
            "method_func"
        );
        
        // Create function type (placeholder)
        const func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.base_codegen.context),
            null,
            0,
            0
        );
        
        // Call the method
        return c.LLVMBuildCall2(
            self.base_codegen.builder,
            func_type,
            method_func,
            args.ptr,
            @as(u32, @intCast(args.len)),
            "method_call"
        );
    }

    /// Apply advanced optimization passes
    fn applyAdvancedOptimizations(self: *AdvancedCodeGen) CodeGenError!void {
        // Create enhanced pass manager
        const pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(pass_manager);
        
        // Add aggressive optimization passes
        c.LLVMAddInstructionCombiningPass(pass_manager);
        c.LLVMAddReassociatePass(pass_manager);
        c.LLVMAddGVNPass(pass_manager);
        c.LLVMAddCFGSimplificationPass(pass_manager);
        c.LLVMAddPromoteMemoryToRegisterPass(pass_manager);
        
        // Add interprocedural passes
        c.LLVMAddInternalizePass(pass_manager, 1);
        c.LLVMAddFunctionInliningPass(pass_manager);
        c.LLVMAddGlobalDCEPass(pass_manager);
        c.LLVMAddGlobalOptimizerPass(pass_manager);
        
        // Add loop optimization passes
        c.LLVMAddLoopUnrollPass(pass_manager);
        c.LLVMAddLICMPass(pass_manager);
        c.LLVMAddLoopDeletionPass(pass_manager);
        
        // Run optimization passes multiple times for better results
        for (0..3) |_| {
            _ = c.LLVMRunPassManager(pass_manager, self.base_codegen.module);
        }
    }

    /// Write optimized executable with debugging information
    pub fn writeOptimizedExecutable(self: *AdvancedCodeGen, output_path: []const u8) CodeGenError!void {
        // Generate debug information
        try self.generateDebugInfo();
        
        // Write executable using base codegen
        try self.base_codegen.writeExecutable(output_path);
        
        // Additional processing for optimization reports
        try self.generateOptimizationReport(output_path);
    }

    /// Generate debug information for advanced features
    fn generateDebugInfo(self: *AdvancedCodeGen) CodeGenError!void {
        // TODO: Implement DWARF debug information generation
        // For now, just add comments to IR
        _ = self;
    }

    /// Generate optimization report
    fn generateOptimizationReport(self: *AdvancedCodeGen, base_path: []const u8) CodeGenError!void {
        var report_path = ArrayList(u8).init(self.base_codegen.allocator);
        defer report_path.deinit();
        
        try report_path.appendSlice(base_path);
        try report_path.appendSlice(".opt_report");
        
        // TODO: Generate detailed optimization report
        _ = self;
    }
};

/// Type information structures
const StructTypeInfo = struct {
    name: []const u8,
    field_types: []c.LLVMTypeRef,
    field_names: [][]const u8,
    llvm_type: ?c.LLVMTypeRef,
    methods: ArrayList(MethodInfo),
    is_generic: bool,
    type_parameters: ArrayList(ast.TypeParameter),
};

const InterfaceTypeInfo = struct {
    name: []const u8,
    methods: ArrayList(InterfaceMethodInfo),
    is_generic: bool,
    type_parameters: ArrayList(ast.TypeParameter),
};

const InterfaceMethodInfo = struct {
    name: []const u8,
    index: usize,
    signature: ast.MethodSignature,
};

const MethodInfo = struct {
    name: []const u8,
    llvm_name: []const u8,
    signature: ast.MethodSignature,
};

const VTableInfo = struct {
    name: []const u8,
    interface_name: []const u8,
    struct_name: []const u8,
    global_value: c.LLVMValueRef,
    method_count: usize,
};

const GenericInstance = struct {
    base_name: []const u8,
    type_arguments: [][]const u8,
    generated_name: []const u8,
    llvm_type: c.LLVMTypeRef,
};

const OptimizationPass = enum {
    FunctionInlining,
    DeadCodeElimination,
    ConstantFolding,
    LoopOptimization,
    InterfaceDevirtualization,
};

test "advanced codegen initialization" {
    const allocator = std.testing.allocator;
    
    var advanced_codegen = AdvancedCodeGen.init(allocator);
    defer advanced_codegen.deinit();
    
    try std.testing.expect(advanced_codegen.gc_enabled == true);
    try std.testing.expect(advanced_codegen.struct_types.count() == 0);
}
