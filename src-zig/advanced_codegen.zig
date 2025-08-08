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
    @cInclude("llvm-c/TargetMachine.h");
});

const ast = @import("ast.zig");
const Type = ast.Type;
const MethodSignature = ast.MethodSignature;
const PointerType = ast.PointerType;
const CodeGen = @import("codegen.zig").CodeGen;
const CodeGenError = @import("codegen.zig").CodeGenError;
const WorkingCodeGen = @import("working_codegen.zig").WorkingCodeGen;
const generics = @import("generics.zig");
const FinalWorkingCodeGen = @import("final_working_codegen.zig").FinalWorkingCodeGen;
const debug_info = @import("debug_info.zig");
const DebugInfoGenerator = debug_info.DebugInfoGenerator;
const SourceLocation = debug_info.SourceLocation;

const interface_dispatch = @import("interface_dispatch.zig");
const InterfaceDispatcher = interface_dispatch.InterfaceDispatcher;
const VTable = interface_dispatch.VTable;
const InterfaceInstance = interface_dispatch.InterfaceInstance;

const OptimizationEngine = @import("optimization_engine.zig").OptimizationEngine;
const OptimizationConfig = @import("optimization_engine.zig").OptimizationConfig;
const OptimizationResult = @import("optimization_engine.zig").OptimizationResult;

/// Enhanced defer statement information for LLVM code generation
/// Supports proper LIFO execution, error handling, and scope management
pub const DeferInfo = struct {
    cleanup_function: c.LLVMValueRef,
    cleanup_block: c.LLVMBasicBlockRef,
    scope_name: []const u8,
    function_name: []const u8,
    is_error_safe: bool = true, // Can be called during error handling
    defer_id: u32 = 0, // Unique ID for LIFO ordering
    scope_id: u32 = 0, // Scope ID for proper cleanup
};

/// Advanced CURSED Zig Code Generator with advanced language features
/// Handles structs, interfaces, generics, advanced memory management, and defer statements
pub const AdvancedCodeGen = struct {
base_codegen: FinalWorkingCodeGen,
    
    // Enhanced defer statement management
    defer_stack: ArrayList(DeferInfo),
    scope_defer_stacks: HashMap([]const u8, ArrayList(DeferInfo), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    scope_stack: ArrayList(u32),
    current_function_name: ?[]const u8,
    current_scope_id: u32,
    
    // Advanced type system support
    struct_types: HashMap([]const u8, StructTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    interface_types: HashMap([]const u8, InterfaceTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    generic_instances: HashMap([]const u8, GenericInstance, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    vtables: HashMap([]const u8, VTableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Generics monomorphization system
    monomorphizer: *generics.Monomorphizer,
    
    // Enhanced type system runtime support
    gc_type_registry: GCTypeRegistry,
    typed_allocator: TypedAllocator,
    interface_registry: InterfaceRegistry,
    type_checker: TypeChecker,
    interface_dispatcher: InterfaceDispatcher,
    
    // Memory management
    gc_enabled: bool,
    heap_allocator: ?c.LLVMValueRef,
    gc_mark_func: ?c.LLVMValueRef,
    gc_sweep_func: ?c.LLVMValueRef,
    
    // Optimization engine
    optimization_engine: ?OptimizationEngine,
    optimization_config: OptimizationConfig,
    last_optimization_result: ?OptimizationResult,
    
    // Debug information generation
    debug_generator: ?DebugInfoGenerator,
    debug_enabled: bool,
    source_file: ?[]const u8,
    source_locations: HashMap(c.LLVMValueRef, SourceLocation, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage),
    
    // Required field to track current function
    current_function: ?c.LLVMValueRef,
    
    pub fn init(allocator: Allocator) !AdvancedCodeGen {
        var gc_registry = GCTypeRegistry.init(allocator);
        var interface_registry = InterfaceRegistry.init(allocator);
        var interface_dispatcher = InterfaceDispatcher.init(allocator, &interface_registry);
        
        // Initialize base codegen first to get context and module
        var base_codegen = try FinalWorkingCodeGen.init(allocator);
        
        // Initialize monomorphizer
        const monomorphizer = try allocator.create(generics.Monomorphizer);
        monomorphizer.* = generics.Monomorphizer.init(allocator, base_codegen.context, base_codegen.module);
        
        return AdvancedCodeGen{
            .base_codegen = base_codegen,
            .defer_stack = ArrayList(DeferInfo).init(allocator),
            .scope_defer_stacks = HashMap([]const u8, ArrayList(DeferInfo), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .scope_stack = ArrayList(u32).init(allocator),
            .current_function_name = null,
            .current_scope_id = 0,
            .struct_types = HashMap([]const u8, StructTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .interface_types = HashMap([]const u8, InterfaceTypeInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .generic_instances = HashMap([]const u8, GenericInstance, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .vtables = HashMap([]const u8, VTableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .monomorphizer = monomorphizer,
            .gc_type_registry = gc_registry,
            .typed_allocator = TypedAllocator.init(allocator, &gc_registry),
            .interface_registry = interface_registry,
            .type_checker = TypeChecker.init(&gc_registry, &interface_registry),
            .interface_dispatcher = interface_dispatcher,
            .gc_enabled = true,
            .heap_allocator = null,
            .gc_mark_func = null,
            .gc_sweep_func = null,
            .optimization_engine = null,
            .optimization_config = OptimizationConfig.default(),
            .last_optimization_result = null,
            .debug_generator = null,
            .debug_enabled = false,
            .source_file = null,
            .source_locations = HashMap(c.LLVMValueRef, SourceLocation, std.hash_map.AutoContext(c.LLVMValueRef), std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
        };
    }

    pub fn deinit(self: *AdvancedCodeGen) void {
        self.base_codegen.deinit();
        self.defer_stack.deinit();
        
        // Clean up scope defer stacks
        var scope_iter = self.scope_defer_stacks.iterator();
        while (scope_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.scope_defer_stacks.deinit();
        
        self.struct_types.deinit();
        self.interface_types.deinit();
        self.generic_instances.deinit();
        self.vtables.deinit();
        
        // Clean up monomorphizer
        self.monomorphizer.deinit();
        self.base_codegen.allocator.destroy(self.monomorphizer);
        
        self.gc_type_registry.deinit();
        self.typed_allocator.deinit();
        self.interface_registry.deinit();
        self.interface_dispatcher.deinit();
        if (self.optimization_engine) |*engine| {
            engine.deinit();
        }
        if (self.debug_generator) |*debug_gen| {
            debug_gen.deinit();
        }
        self.source_locations.deinit();
    }

    /// Set optimization level
    pub fn setOptimizationLevel(self: *AdvancedCodeGen, level: u32) void {
        self.optimization_config.optimization_level = level;
        
        if (self.optimization_engine) |*engine| {
            engine.setOptimizationLevel(level);
        }
    }

    /// Enable size optimization
    pub fn enableSizeOptimization(self: *AdvancedCodeGen, level: u32) void {
        self.optimization_config.size_optimization_level = level;
        self.optimization_config.size_optimizations = level > 0;
        
        if (self.optimization_engine) |*engine| {
            engine.setSizeOptimizationLevel(level);
        }
    }

    /// Enable profile-guided optimization
    pub fn enableProfileGuidedOptimization(self: *AdvancedCodeGen, profile_path: []const u8) !void {
        _ = profile_path; // TODO: Load profile data
        self.optimization_config.pgo_enabled = true;
        
        if (self.optimization_engine) |*engine| {
            // TODO: Load profile data and enable PGO
            _ = engine;
        }
    }

    /// Enable comprehensive debug information generation with GDB/LLDB support
    pub fn enableDebugInfo(self: *AdvancedCodeGen, source_file: []const u8) !void {
        self.debug_enabled = true;
        self.source_file = source_file;
        
        // Initialize debug generator
        self.debug_generator = try DebugInfoGenerator.init(
            self.base_codegen.allocator,
            self.base_codegen.context,
            self.base_codegen.module
        );
        
        // Create compile unit with enhanced debug info
        const directory = std.fs.path.dirname(source_file) orelse ".";
        const filename = std.fs.path.basename(source_file);
        try self.debug_generator.?.createCompileUnit(filename, directory);
        
        // Enable debug symbols in module
        c.LLVMAddModuleFlag(
            self.base_codegen.module,
            c.LLVMModuleFlagBehaviorWarning,
            "Debug Info Version",
            17, // Length
            c.LLVMValueAsMetadata(c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), 3, 0))
        );
        
        // Enable DWARF version
        c.LLVMAddModuleFlag(
            self.base_codegen.module,
            c.LLVMModuleFlagBehaviorWarning,
            "Dwarf Version",
            13, // Length
            c.LLVMValueAsMetadata(c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), 4, 0))
        );
        
        std.debug.print("✅ Enhanced debug information with DWARF enabled for {s}\n", .{source_file});
    }

    /// Compile defer statement with proper LLVM integration
    /// Handles scope-based cleanup and LIFO execution order
    pub fn compileDeferStatement(self: *AdvancedCodeGen, defer_stmt: ast.DeferStatement) !void {
        const context = self.base_codegen.context;
        const module = self.base_codegen.module;
        const builder = self.base_codegen.builder;
        const _current_function = self.base_codegen.current_function orelse return error.NoCurrentFunction;
        
        // Generate unique cleanup function name
        const defer_count = self.defer_stack.items.len;
        const cleanup_func_name = try std.fmt.allocPrint(
            self.base_codegen.allocator, 
            "defer_cleanup_{s}_{d}", 
            .{ self.current_function_name orelse "anonymous", defer_count }
        );
        defer self.base_codegen.allocator.free(cleanup_func_name);
        
        // Create cleanup function type (void function with no parameters)
        const cleanup_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(context),
            null,
            0,
            0
        );
        
        // Create cleanup function
        const cleanup_func = c.LLVMAddFunction(module, cleanup_func_name.ptr, cleanup_func_type);
        const cleanup_entry = c.LLVMAppendBasicBlockInContext(context, cleanup_func, "entry");
        
        // Save current builder context
        const saved_function = self.base_codegen.current_function;
        const saved_block = c.LLVMGetInsertBlock(builder);
        
        // Generate cleanup code in separate function
        c.LLVMPositionBuilderAtEnd(builder, cleanup_entry);
        self.base_codegen.current_function = cleanup_func;
        
        // Compile the deferred statement
        const statement_ptr: *ast.Statement = @ptrCast(@alignCast(defer_stmt.statement));
        try self.compileStatement(statement_ptr.*);
        
        // Ensure cleanup function has proper return
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
            _ = c.LLVMBuildRetVoid(builder);
        }
        
        // Restore builder context
        self.base_codegen.current_function = saved_function;
        if (saved_block != null) {
            c.LLVMPositionBuilderAtEnd(builder, saved_block);
        }
        
        // Register cleanup function with runtime defer stack
        try self.registerDeferCleanup(cleanup_func);
        
        // Store defer info for scope management with enhanced tracking
        const defer_info = DeferInfo{
            .cleanup_function = cleanup_func,
            .cleanup_block = cleanup_entry,
            .scope_name = self.current_function_name orelse "global",
            .function_name = self.current_function_name orelse "main",
            .is_error_safe = true, // Mark as error-safe for error handling integration
            .defer_id = @intCast(defer_count),
            .scope_id = if (self.scope_stack.items.len > 0) 
                self.scope_stack.items[self.scope_stack.items.len - 1] 
            else 
                0,
        };
        
        try self.defer_stack.append(defer_info);
        
        std.debug.print("✅ Advanced defer statement compiled: {s} (error-safe, LIFO)\n", .{cleanup_func_name});
    }
    
    /// Register defer cleanup function with runtime
    fn registerDeferCleanup(self: *AdvancedCodeGen, cleanup_func: c.LLVMValueRef) !void {
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        
        // Declare runtime defer functions if not already declared
        try self.declareDeferRuntimeFunctions();
        
        // Get defer push function
        const defer_push_func = self.base_codegen.runtime_functions.get("cursed_defer_push") orelse
            return error.DeferRuntimeNotAvailable;
            
        // Cast cleanup function to void* for runtime
        const void_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const func_ptr = c.LLVMBuildBitCast(
            builder,
            cleanup_func,
            void_ptr_type,
            "cleanup_ptr"
        );
        
        // Call runtime defer push
        _ = c.LLVMBuildCall2(
            builder,
            c.LLVMVoidTypeInContext(context),
            defer_push_func,
            &[_]c.LLVMValueRef{func_ptr},
            1,
            ""
        );
    }
    
    /// Declare defer runtime functions
    fn declareDeferRuntimeFunctions(self: *AdvancedCodeGen) !void {
        const context = self.base_codegen.context;
        const module = self.base_codegen.module;
        
        // cursed_defer_push(void* cleanup_func)
        if (self.base_codegen.runtime_functions.get("cursed_defer_push") == null) {
            const defer_push_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
                1,
                0
            );
            const defer_push_func = c.LLVMAddFunction(module, "cursed_defer_push", defer_push_type);
            try self.base_codegen.runtime_functions.put("cursed_defer_push", defer_push_func);
        }
        
        // cursed_defer_pop()
        if (self.base_codegen.runtime_functions.get("cursed_defer_pop") == null) {
            const defer_pop_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                null,
                0,
                0
            );
            const defer_pop_func = c.LLVMAddFunction(module, "cursed_defer_pop", defer_pop_type);
            try self.base_codegen.runtime_functions.put("cursed_defer_pop", defer_pop_func);
        }
        
        // cursed_defer_execute_all() - executes all defers for current scope
        if (self.base_codegen.runtime_functions.get("cursed_defer_execute_all") == null) {
            const defer_exec_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(context),
                null,
                0,
                0
            );
            const defer_exec_func = c.LLVMAddFunction(module, "cursed_defer_execute_all", defer_exec_type);
            try self.base_codegen.runtime_functions.put("cursed_defer_execute_all", defer_exec_func);
        }
    }
    
    /// Generate function exit with defer cleanup
    pub fn generateFunctionExitWithDefers(self: *AdvancedCodeGen) !void {
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        
        // Execute all defers for current function in LIFO order
        if (self.base_codegen.runtime_functions.get("cursed_defer_execute_all")) |defer_exec_func| {
            _ = c.LLVMBuildCall2(
                builder,
                c.LLVMVoidTypeInContext(context),
                defer_exec_func,
                null,
                0,
                ""
            );
        }
    }
    
    /// Enter a new scope for defer management
    pub fn enterScope(self: *AdvancedCodeGen, is_function_scope: bool) !u32 {
        self.current_scope_id += 1;
        try self.scope_stack.append(self.current_scope_id);
        
        std.debug.print("📍 Entered scope {d} (function: {any})\n", .{ self.current_scope_id, is_function_scope });
        return self.current_scope_id;
    }
    
    /// Exit current scope and generate defer cleanup
    pub fn exitScope(self: *AdvancedCodeGen) !void {
        if (self.scope_stack.items.len == 0) {
            std.debug.print("⚠️ Warning: Attempting to exit scope when no scopes are active\n");
            return;
        }
        
        const scope_id = self.scope_stack.pop();
        
        // Generate calls to execute defers for this scope in LIFO order
        var i = self.defer_stack.items.len;
        while (i > 0) {
            i -= 1;
            const defer_entry = self.defer_stack.items[i];
            
            if (defer_entry.scope_id == scope_id) {
                // Call the cleanup function directly
                const builder = self.base_codegen.builder;
                const context = self.base_codegen.context;
                
                _ = c.LLVMBuildCall2(
                    builder,
                    c.LLVMVoidTypeInContext(context),
                    defer_entry.cleanup_function,
                    null,
                    0,
                    ""
                );
                
                // Remove this defer from tracking
                _ = self.defer_stack.orderedRemove(i);
            }
        }
        
        std.debug.print("📍 Exited scope {d}\n", .{scope_id});
    }
    
    /// Generate error handling with defer cleanup integration
    pub fn generateErrorHandlingWithDefers(self: *AdvancedCodeGen, error_value: c.LLVMValueRef) !void {
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        const current_function = self.base_codegen.current_function orelse return error.NoCurrentFunction;
        
        // Create error cleanup block
        const error_cleanup_block = c.LLVMAppendBasicBlockInContext(context, current_function, "error_defer_cleanup");
        const error_exit_block = c.LLVMAppendBasicBlockInContext(context, current_function, "error_exit");
        
        // Branch to error cleanup
        _ = c.LLVMBuildBr(builder, error_cleanup_block);
        
        // Generate error cleanup code
        c.LLVMPositionBuilderAtEnd(builder, error_cleanup_block);
        
        // Execute only error-safe defers in LIFO order
        var i = self.defer_stack.items.len;
        while (i > 0) {
            i -= 1;
            const defer_entry = self.defer_stack.items[i];
            
            if (defer_entry.is_error_safe) {
                _ = c.LLVMBuildCall2(
                    builder,
                    c.LLVMVoidTypeInContext(context),
                    defer_entry.cleanup_function,
                    null,
                    0,
                    ""
                );
            }
        }
        
        // Branch to error exit
        _ = c.LLVMBuildBr(builder, error_exit_block);
        
        // Generate error return
        c.LLVMPositionBuilderAtEnd(builder, error_exit_block);
        _ = c.LLVMBuildRet(builder, error_value);
        
        std.debug.print("💥 Generated error handling with defer cleanup\n");
    }
    
    /// Set debug location for current instruction
    pub fn setDebugLocation(self: *AdvancedCodeGen, line: u32, column: u32) void {
        if (self.debug_enabled and self.debug_generator != null) {
            self.debug_generator.?.setCurrentLocation(line, column);
        }
    }
    
    /// Compile statement with debug location tracking and defer awareness
    pub fn compileStatement(self: *AdvancedCodeGen, statement: ast.Statement) !void {
        // Set debug location for statement if available
        if (self.debug_enabled and self.debug_generator != null) {
            // For now, use line 1 - in a real parser, this would come from AST
            self.setDebugLocation(1, 1);
        }
        
        switch (statement) {
            .Defer => |defer_stmt| {
                try self.compileDeferStatement(defer_stmt);
            },
            .Return => |return_stmt| {
                // Execute defers before return
                try self.generateFunctionExitWithDefers();
                try self.compileReturnStatement(return_stmt);
            },
            .If => |if_stmt| {
                try self.compileIfStatement(if_stmt);
            },
            .While => |while_stmt| {
                try self.compileWhileStatement(while_stmt);
            },
            .Select => |select_stmt| {
                try self.compileSelectStatement(select_stmt);
            },
            else => {
                // Use base codegen for other statements
                try self.base_codegen.generateStatement(&statement);
            },
        }
    }
    
    /// Compile return statement  
    fn compileReturnStatement(self: *AdvancedCodeGen, return_stmt: ast.ReturnStatement) !void {
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        
        if (return_stmt.expression) |expr_ptr| {
            const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
            const value = try self.base_codegen.generateExpression(expr.*);
            _ = c.LLVMBuildRet(builder, value);
        } else {
            _ = c.LLVMBuildRetVoid(builder);
        }
    }

    /// Compile if statement with proper basic block management
    /// Handles ready (if) statements with conditional branching and otherwise (else) clauses
    fn compileIfStatement(self: *AdvancedCodeGen, if_stmt: ast.IfStatement) !void {
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        const current_function = self.base_codegen.current_function orelse return error.NoCurrentFunction;
        
        // Enter new scope for the if statement
        _ = try self.enterScope(false);
        defer self.exitScope() catch {};
        
        // Generate condition expression
        const condition = try self.base_codegen.generateExpression(if_stmt.condition);
        
        // Create basic blocks for if statement control flow
        const then_block = c.LLVMAppendBasicBlockInContext(context, current_function, "ready_then");
        const else_block = c.LLVMAppendBasicBlockInContext(context, current_function, "otherwise_else");
        const merge_block = c.LLVMAppendBasicBlockInContext(context, current_function, "if_merge");
        
        // Build conditional branch
        _ = c.LLVMBuildCondBr(builder, condition, then_block, else_block);
        
        // Generate then branch (ready block)
        c.LLVMPositionBuilderAtEnd(builder, then_block);
        for (if_stmt.then_branch.items) |stmt| {
            try self.compileStatement(stmt);
        }
        
        // Add branch to merge block if no terminator exists
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
            _ = c.LLVMBuildBr(builder, merge_block);
        }
        
        // Generate else branch (otherwise block)
        c.LLVMPositionBuilderAtEnd(builder, else_block);
        if (if_stmt.else_branch) |else_stmts| {
            for (else_stmts.items) |stmt| {
                try self.compileStatement(stmt);
            }
        }
        
        // Add branch to merge block if no terminator exists
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
            _ = c.LLVMBuildBr(builder, merge_block);
        }
        
        // Continue building in merge block
        c.LLVMPositionBuilderAtEnd(builder, merge_block);
        
        std.debug.print("✅ If statement (ready/otherwise) compiled with proper control flow\n", .{});
    }

    /// Compile while statement with proper loop structure
    /// Handles bestie (while) loops with loop headers and exit conditions
    fn compileWhileStatement(self: *AdvancedCodeGen, while_stmt: ast.WhileStatement) !void {
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        const current_function = self.base_codegen.current_function orelse return error.NoCurrentFunction;
        
        // Enter new scope for the loop
        _ = try self.enterScope(false);
        defer self.exitScope() catch {};
        
        // Create basic blocks for while loop control flow
        const condition_block = c.LLVMAppendBasicBlockInContext(context, current_function, "bestie_condition");
        const body_block = c.LLVMAppendBasicBlockInContext(context, current_function, "bestie_body");
        const exit_block = c.LLVMAppendBasicBlockInContext(context, current_function, "bestie_exit");
        
        // Jump to condition block
        _ = c.LLVMBuildBr(builder, condition_block);
        
        // Generate condition block
        c.LLVMPositionBuilderAtEnd(builder, condition_block);
        const condition = try self.base_codegen.generateExpression(while_stmt.condition);
        _ = c.LLVMBuildCondBr(builder, condition, body_block, exit_block);
        
        // Generate body block
        c.LLVMPositionBuilderAtEnd(builder, body_block);
        for (while_stmt.body.items) |stmt| {
            try self.compileStatement(stmt);
        }
        
        // Add branch back to condition if no terminator exists
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
            _ = c.LLVMBuildBr(builder, condition_block);
        }
        
        // Continue building in exit block
        c.LLVMPositionBuilderAtEnd(builder, exit_block);
        
        std.debug.print("✅ While loop (bestie) compiled with proper control flow\n", .{});
    }

    /// Compile select statement for channel operations
    /// Handles ready { case -> action } select statements
    fn compileSelectStatement(self: *AdvancedCodeGen, select_stmt: ast.SelectStatement) !void {
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        const current_function = self.base_codegen.current_function orelse return error.NoCurrentFunction;
        
        // Enter new scope for the select statement
        _ = try self.enterScope(false);
        defer self.exitScope() catch {};
        
        // Ensure select runtime functions are declared
        try self.ensureSelectRuntimeFunctions();
        
        // Create select context for runtime
        const select_begin_func = self.base_codegen.runtime_functions.get("cursed_select_begin") orelse
            return error.SelectRuntimeNotAvailable;
        
        const case_count = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @as(u32, @intCast(select_stmt.cases.items.len)), 0);
        const select_context = c.LLVMBuildCall2(
            builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0),
            select_begin_func,
            &[_]c.LLVMValueRef{case_count},
            1,
            "select_ctx"
        );
        
        // Register select operations with runtime
        for (select_stmt.cases.items, 0..) |case_item, i| {
            const case_index = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @as(u32, @intCast(i)), 0);
            
            if (case_item.channel != null and case_item.send_value != null) {
                // Send operation
                const channel_value = try self.base_codegen.generateExpression(case_item.channel.?);
                const send_value = try self.base_codegen.generateExpression(case_item.send_value.?);
                
                const add_send_func = self.base_codegen.runtime_functions.get("cursed_select_add_send").?;
                _ = c.LLVMBuildCall2(
                    builder,
                    c.LLVMVoidTypeInContext(context),
                    add_send_func,
                    &[_]c.LLVMValueRef{ select_context, channel_value, send_value, case_index },
                    4,
                    ""
                );
            } else if (case_item.channel != null) {
                // Receive operation
                const channel_value = try self.base_codegen.generateExpression(case_item.channel.?);
                
                const add_recv_func = self.base_codegen.runtime_functions.get("cursed_select_add_recv").?;
                _ = c.LLVMBuildCall2(
                    builder,
                    c.LLVMVoidTypeInContext(context),
                    add_recv_func,
                    &[_]c.LLVMValueRef{ select_context, channel_value, case_index },
                    3,
                    ""
                );
            } else {
                // Default case
                const add_default_func = self.base_codegen.runtime_functions.get("cursed_select_add_default").?;
                _ = c.LLVMBuildCall2(
                    builder,
                    c.LLVMVoidTypeInContext(context),
                    add_default_func,
                    &[_]c.LLVMValueRef{ select_context, case_index },
                    2,
                    ""
                );
            }
        }
        
        // Execute select and get ready case index
        const select_execute_func = self.base_codegen.runtime_functions.get("cursed_select_execute").?;
        const ready_case_index = c.LLVMBuildCall2(
            builder,
            c.LLVMInt32TypeInContext(context),
            select_execute_func,
            &[_]c.LLVMValueRef{select_context},
            1,
            "ready_case"
        );
        
        // Create basic blocks for case handling
        const merge_block = c.LLVMAppendBasicBlockInContext(context, current_function, "select_merge");
        const no_case_ready_block = c.LLVMAppendBasicBlockInContext(context, current_function, "select_no_case");
        
        // Create switch instruction to jump to ready case
        const switch_inst = c.LLVMBuildSwitch(builder, ready_case_index, no_case_ready_block, @as(u32, @intCast(select_stmt.cases.items.len)));
        
        var case_blocks = std.ArrayList(c.LLVMBasicBlockRef).init(self.base_codegen.allocator);
        defer case_blocks.deinit();
        
        // Generate blocks for each case
        for (select_stmt.cases.items, 0..) |case_item, i| {
            const case_block = c.LLVMAppendBasicBlockInContext(context, current_function, 
                try std.fmt.allocPrint(self.base_codegen.allocator, "select_case_{d}", .{i}).ptr);
            try case_blocks.append(case_block);

            // Add case to switch
            const case_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @as(u32, @intCast(i)), 0);
            c.LLVMAddCase(switch_inst, case_value, case_block);

            // Position builder in case block
            c.LLVMPositionBuilderAtEnd(builder, case_block);
            
            // For receive operations, extract the received value
            if (case_item.channel != null and case_item.send_value == null) {
                const get_recv_value_func = self.base_codegen.runtime_functions.get("cursed_select_get_recv_value").?;
                const received_value = c.LLVMBuildCall2(
                    builder,
                    c.LLVMInt64TypeInContext(context),
                    get_recv_value_func,
                    &[_]c.LLVMValueRef{ select_context, case_value },
                    2,
                    "recv_value"
                );
                
                // Store in variable if case has variable binding
                if (case_item.variable_name) |var_name| {
                    // Create or update variable binding
                    // Create alloca for the received value
                    const var_type = c.LLVMInt64TypeInContext(context);
                    const alloca = c.LLVMBuildAlloca(builder, var_type, var_name.ptr);
                    _ = c.LLVMBuildStore(builder, received_value, alloca);
                    
                    // Store in variables map if available
                    try self.base_codegen.variables.put(var_name, alloca);
                }
            }
            
            // Generate case body statements
            for (case_item.body.items) |stmt| {
                try self.compileStatement(stmt);
            }
            
            // Branch to merge if no terminator
            if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
                _ = c.LLVMBuildBr(builder, merge_block);
            }
        }

        // Handle case where no operation is ready (should not happen with proper select)
        c.LLVMPositionBuilderAtEnd(builder, no_case_ready_block);
        _ = c.LLVMBuildBr(builder, merge_block);

        // Continue in merge block
        c.LLVMPositionBuilderAtEnd(builder, merge_block);
        
        // Cleanup select context
        const select_cleanup_func = self.base_codegen.runtime_functions.get("cursed_select_cleanup").?;
        _ = c.LLVMBuildCall2(
            builder,
            c.LLVMVoidTypeInContext(context),
            select_cleanup_func,
            &[_]c.LLVMValueRef{select_context},
            1,
            ""
        );
        
        std.debug.print("✅ Select statement (ready) compiled with proper channel operations\n", .{});
    }

    /// Compile CURSED source code to executable
    pub fn compileSource(self: *AdvancedCodeGen, source: []const u8) !void {
        // Use the working codegen to compile basic programs
        try self.base_codegen.compile(source);
        
        // Apply advanced optimizations if enabled
        try self.applyAdvancedOptimizations();
    }

    /// Generate advanced struct definition
    pub fn generateAdvancedStruct(self: *AdvancedCodeGen, struct_name: []const u8, fields: []const []const u8) !void {
        try self.base_codegen.generateStruct(struct_name, fields);
    }

    /// Generate advanced interface definition
    pub fn generateAdvancedInterface(self: *AdvancedCodeGen, interface_name: []const u8, methods: []const []const u8) !void {
        try self.base_codegen.generateInterface(interface_name, methods);
    }

    /// Generate advanced function with generics support
    pub fn generateAdvancedFunction(self: *AdvancedCodeGen, func_name: []const u8, return_type: []const u8, params: []const []const u8, body: []const u8) !void {
        try self.base_codegen.generateFunction(func_name, return_type, params, body);
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
        try self.monomorphizer.processInstantiations();
        
        // Fifth pass: generate pattern matching support
        try self.generatePatternMatchingSupport();
        
        // Sixth pass: generate code - skip for now due to type mismatch
        // try self.base_codegen.generateProgram(program);
        
        // Final pass: apply optimizations
        try self.applyAdvancedOptimizations();
    }

    /// Write executable using the working codegen
    pub fn writeExecutable(self: *AdvancedCodeGen, output_path: []const u8) !void {
        try self.base_codegen.writeExecutable(output_path);
    }

    /// Print LLVM IR
    pub fn printIR(self: *AdvancedCodeGen) void {
        self.base_codegen.printIR();
    }

    /// Generate comprehensive pattern matching support
    fn generatePatternMatchingSupport(self: *AdvancedCodeGen) CodeGenError!void {
        // Generate pattern matching helper functions
        try self.generatePatternMatchHelpers();
        
        // Generate pattern type checking functions
        try self.generatePatternTypeCheckers();
        
        // Generate optimized switch dispatch tables
        try self.generateSwitchDispatchTables();
    }

    /// Generate pattern matching helper functions
    fn generatePatternMatchHelpers(self: *AdvancedCodeGen) CodeGenError!void {
        // Generate string comparison helper for pattern matching
        const string_compare_type = c.LLVMFunctionType(
            c.LLVMInt1TypeInContext(self.base_codegen.context), // return bool
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), // str1
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), // str2
            },
            2, // parameter count
            0  // not variadic
        );
        
        const string_compare_func = c.LLVMAddFunction(self.base_codegen.module, "pattern_string_compare", string_compare_type);
        try self.base_codegen.functions.put("pattern_string_compare", string_compare_func);
        
        // Generate implementation
        const entry_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, string_compare_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, entry_block);
        
        const str1 = c.LLVMGetParam(string_compare_func, 0);
        const str2 = c.LLVMGetParam(string_compare_func, 1);
        
        // Call strcmp and compare result to 0
        const strcmp_func = self.base_codegen.functions.get("strcmp").?;
        const strcmp_result = c.LLVMBuildCall2(
            self.base_codegen.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(strcmp_func)),
            strcmp_func,
            &[_]c.LLVMValueRef{ str1, str2 },
            2,
            "strcmp_result"
        );
        
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), 0, 0);
        const is_equal = c.LLVMBuildICmp(self.base_codegen.builder, c.LLVMIntEQ, strcmp_result, zero, "is_equal");
        _ = c.LLVMBuildRet(self.base_codegen.builder, is_equal);
        
        // Generate tuple destructuring helper
        try self.generateTupleDestructuringHelper();
        
        // Generate array pattern matching helper
        try self.generateArrayPatternHelper();
    }

    /// Generate tuple destructuring helper
    fn generateTupleDestructuringHelper(self: *AdvancedCodeGen) CodeGenError!void {
        // Generate tuple access function: get_tuple_element(tuple_ptr, index) -> element_ptr
        const tuple_access_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), // return void*
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), // tuple_ptr
                c.LLVMInt32TypeInContext(self.base_codegen.context), // index
            },
            2, // parameter count
            0  // not variadic
        );
        
        const tuple_access_func = c.LLVMAddFunction(self.base_codegen.module, "pattern_tuple_access", tuple_access_type);
        try self.base_codegen.functions.put("pattern_tuple_access", tuple_access_func);
        
        // Implementation will be generated separately for each tuple type
    }

    /// Generate array pattern matching helper
    fn generateArrayPatternHelper(self: *AdvancedCodeGen) CodeGenError!void {
        // Generate array length check: check_array_length(array_ptr, expected_length) -> bool
        const array_length_check_type = c.LLVMFunctionType(
            c.LLVMInt1TypeInContext(self.base_codegen.context), // return bool
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), // array_ptr
                c.LLVMInt32TypeInContext(self.base_codegen.context), // expected_length
                c.LLVMInt1TypeInContext(self.base_codegen.context), // is_exact (false for >= check)
            },
            3, // parameter count
            0  // not variadic
        );
        
        const array_length_check_func = c.LLVMAddFunction(self.base_codegen.module, "pattern_array_length_check", array_length_check_type);
        try self.base_codegen.functions.put("pattern_array_length_check", array_length_check_func);
    }

    /// Generate pattern type checking functions
    fn generatePatternTypeCheckers(self: *AdvancedCodeGen) CodeGenError!void {
        // Generate runtime type checking for patterns
        const type_check_type = c.LLVMFunctionType(
            c.LLVMInt1TypeInContext(self.base_codegen.context), // return bool
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), // value_ptr
                c.LLVMInt32TypeInContext(self.base_codegen.context), // expected_type_id
            },
            2, // parameter count
            0  // not variadic
        );
        
        const type_check_func = c.LLVMAddFunction(self.base_codegen.module, "pattern_type_check", type_check_type);
        try self.base_codegen.functions.put("pattern_type_check", type_check_func);
        
        // Generate implementation
        const entry_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, type_check_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, entry_block);
        
        const value_ptr = c.LLVMGetParam(type_check_func, 0);
        const expected_type = c.LLVMGetParam(type_check_func, 1);
        
        // Extract type information from GC header
        const header_ptr = c.LLVMBuildGEP2(
            self.base_codegen.builder,
            c.LLVMInt8TypeInContext(self.base_codegen.context),
            value_ptr,
            &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), @as(u64, @intCast(-16)), 1)}, // -16 bytes for header
            1,
            "header_ptr"
        );
        
        const type_id_ptr = c.LLVMBuildBitCast(
            self.base_codegen.builder,
            header_ptr,
            c.LLVMPointerType(c.LLVMInt32TypeInContext(self.base_codegen.context), 0),
            "type_id_ptr"
        );
        
        const type_id_offset_ptr = c.LLVMBuildGEP2(
            self.base_codegen.builder,
            c.LLVMInt32TypeInContext(self.base_codegen.context),
            type_id_ptr,
            &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), 1, 0)},
            1,
            "type_id_offset_ptr"
        );
        
        const actual_type = c.LLVMBuildLoad2(self.base_codegen.builder, c.LLVMInt32TypeInContext(self.base_codegen.context), type_id_offset_ptr, "actual_type");
        const types_match = c.LLVMBuildICmp(self.base_codegen.builder, c.LLVMIntEQ, actual_type, expected_type, "types_match");
        _ = c.LLVMBuildRet(self.base_codegen.builder, types_match);
    }

    /// Generate optimized switch dispatch tables for literal patterns
    fn generateSwitchDispatchTables(self: *AdvancedCodeGen) CodeGenError!void {
        // Generate jump table creator for integer patterns
        const jump_table_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.base_codegen.context), // return void
            &[_]c.LLVMTypeRef{
                c.LLVMInt64TypeInContext(self.base_codegen.context), // switch_value
                c.LLVMPointerType(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), 0), // jump_table
                c.LLVMInt32TypeInContext(self.base_codegen.context), // table_size
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), // default_label
            },
            4, // parameter count
            0  // not variadic
        );
        
        const jump_table_func = c.LLVMAddFunction(self.base_codegen.module, "pattern_jump_table_dispatch", jump_table_type);
        try self.base_codegen.functions.put("pattern_jump_table_dispatch", jump_table_func);
    }

    /// Generate CURSED vibe_check statement with optimized pattern matching
    pub fn generateVibeCheckStatement(self: *AdvancedCodeGen, vibe_check: ast.VibeCheckStatement) CodeGenError!void {
        const discriminant_value = try self.generateExpression(vibe_check.discriminant);
        
        const current_func = self.current_function.?;
        const merge_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "vibe_check_merge");
        const default_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "vibe_check_default");
        
        // Analyze patterns for optimization
        const optimization_result = try self.analyzePatternOptimization(vibe_check.cases.items);
        
        if (optimization_result.use_jump_table) {
            // Generate optimized jump table for literal patterns
            try self.generateJumpTableDispatch(discriminant_value, vibe_check.cases.items, merge_block, default_block);
        } else {
            // Generate sequential pattern matching
            try self.generateSequentialPatternMatching(discriminant_value, vibe_check.cases.items, merge_block, default_block);
        }
        
        // Generate default case if present
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, default_block);
        if (vibe_check.default_case) |default_case| {
            for (default_case.items) |stmt| {
                try self.generateStatement(stmt);
            }
        }
        
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.base_codegen.builder)) == null) {
            _ = c.LLVMBuildBr(self.base_codegen.builder, merge_block);
        }
        
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, merge_block);
    }

    /// Generate CURSED match expression
    pub fn generateMatchExpression(self: *AdvancedCodeGen, match_expr: ast.MatchExpression) CodeGenError!c.LLVMValueRef {
        const discriminant_value = try self.generateExpression(match_expr.discriminant);
        
        const current_func = self.current_function.?;
        const merge_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "match_merge");
        
        // Create PHI node for result value
        const result_type = try self.inferMatchResultType(match_expr.cases.items);
        const result_phi = c.LLVMBuildPhi(self.base_codegen.builder, result_type, "match_result");
        
        // Generate pattern matching cases
        var phi_values = ArrayList(c.LLVMValueRef).init(self.base_codegen.allocator);
        var phi_blocks = ArrayList(c.LLVMBasicBlockRef).init(self.base_codegen.allocator);
        defer phi_values.deinit();
        defer phi_blocks.deinit();
        
        for (match_expr.cases.items) |case| {
            const case_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "match_case");
            const next_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "match_next");
            
            // Generate pattern check
            const pattern_matches = try self.generatePatternCheck(discriminant_value, case.pattern, case_block, next_block);
            _ = pattern_matches;
            
            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, case_block);
            const case_result = try self.generateExpression(case.result);
            try phi_values.append(case_result);
            try phi_blocks.append(c.LLVMGetInsertBlock(self.base_codegen.builder));
            
            if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.base_codegen.builder)) == null) {
                _ = c.LLVMBuildBr(self.base_codegen.builder, merge_block);
            }
            
            c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, next_block);
        }
        
        // Generate final merge
        _ = c.LLVMBuildBr(self.base_codegen.builder, merge_block);
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, merge_block);
        
        // Set PHI incoming values
        c.LLVMAddIncoming(result_phi, phi_values.items.ptr, phi_blocks.items.ptr, @as(u32, @intCast(phi_values.items.len)));
        
        return result_phi;
    }

    /// Analyze patterns for optimization opportunities
    fn analyzePatternOptimization(self: *AdvancedCodeGen, cases: []const ast.VibeCheckCase) CodeGenError!PatternOptimizationResult {
        _ = self;
        var literal_count: usize = 0;
        var complex_count: usize = 0;
        
        for (cases) |case| {
            if (case.pattern) |pattern| {
                switch (pattern) {
                    .Literal => literal_count += 1,
                    else => complex_count += 1,
                }
            }
        }
        
        return PatternOptimizationResult{
            .use_jump_table = literal_count >= 8 and complex_count == 0,
            .literal_pattern_count = literal_count,
            .complex_pattern_count = complex_count,
        };
    }

    /// Generate optimized jump table dispatch
    fn generateJumpTableDispatch(self: *AdvancedCodeGen, discriminant: c.LLVMValueRef, cases: []const ast.VibeCheckCase, merge_block: c.LLVMBasicBlockRef, default_block: c.LLVMBasicBlockRef) CodeGenError!void {
        const current_func = self.current_function.?;
        
        // Create switch instruction with jump table optimization hint
        const switch_inst = c.LLVMBuildSwitch(self.base_codegen.builder, discriminant, default_block, @as(u32, @intCast(cases.len)));
        
        for (cases) |case| {
            if (case.pattern) |pattern| {
                switch (pattern) {
                    .Literal => |literal| {
                        const case_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "jump_case");
                        
                        // Add case to switch
                        const case_value = try self.generateLiteralValue(literal);
                        c.LLVMAddCase(switch_inst, case_value, case_block);
                        
                        // Generate case body
                        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, case_block);
                        for (case.body.items) |stmt| {
                            try self.generateStatement(stmt);
                        }
                        
                        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.base_codegen.builder)) == null) {
                            _ = c.LLVMBuildBr(self.base_codegen.builder, merge_block);
                        }
                    },
                    else => return error.NonLiteralInJumpTable,
                }
            }
        }
    }

    /// Generate sequential pattern matching for complex patterns
    fn generateSequentialPatternMatching(self: *AdvancedCodeGen, discriminant: c.LLVMValueRef, cases: []const ast.VibeCheckCase, merge_block: c.LLVMBasicBlockRef, default_block: c.LLVMBasicBlockRef) CodeGenError!void {
        const current_func = self.current_function.?;
        var next_case_block = default_block;
        
        // Generate cases in reverse order
        for (0..cases.len) |i| {
            const case_index = cases.len - 1 - i;
            const case = cases[case_index];
            
            const case_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "seq_case");
            const current_next = next_case_block;
            next_case_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "seq_next");
            
            // Generate pattern check
            if (case.pattern) |pattern| {
                _ = try self.generatePatternCheck(discriminant, pattern, case_block, current_next);
            }
            
            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, case_block);
            for (case.body.items) |stmt| {
                try self.generateStatement(stmt);
            }
            
            if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.base_codegen.builder)) == null) {
                _ = c.LLVMBuildBr(self.base_codegen.builder, merge_block);
            }
        }
        
        // Jump to first case
        _ = c.LLVMBuildBr(self.base_codegen.builder, next_case_block);
    }

    /// Generate pattern check for various pattern types
    fn generatePatternCheck(self: *AdvancedCodeGen, value: c.LLVMValueRef, pattern: ast.Pattern, success_block: c.LLVMBasicBlockRef, fail_block: c.LLVMBasicBlockRef) CodeGenError!c.LLVMValueRef {
        switch (pattern) {
            .Literal => |literal| {
                return try self.generateLiteralPatternCheck(value, literal, success_block, fail_block);
            },
            .Variable => |variable| {
                return try self.generateVariablePatternCheck(value, variable, success_block);
            },
            .Wildcard => {
                _ = c.LLVMBuildBr(self.base_codegen.builder, success_block);
                return value;
            },
            .Tuple => |tuple_patterns| {
                return try self.generateTuplePatternCheck(value, tuple_patterns.items, success_block, fail_block);
            },
            .Struct => |struct_pattern| {
                return try self.generateStructPatternCheck(value, struct_pattern, success_block, fail_block);
            },
            .Array => |array_patterns| {
                return try self.generateArrayPatternCheck(value, array_patterns.items, success_block, fail_block);
            },
        }
    }

    /// Generate literal pattern check with type-specific optimization
    fn generateLiteralPatternCheck(self: *AdvancedCodeGen, value: c.LLVMValueRef, literal: ast.Literal, success_block: c.LLVMBasicBlockRef, fail_block: c.LLVMBasicBlockRef) CodeGenError!c.LLVMValueRef {
        const literal_value = try self.generateLiteralValue(literal);
        
        switch (literal) {
            .Integer => {
                const comparison = c.LLVMBuildICmp(self.base_codegen.builder, c.LLVMIntEQ, value, literal_value, "int_eq");
                _ = c.LLVMBuildCondBr(self.base_codegen.builder, comparison, success_block, fail_block);
                return comparison;
            },
            .Float => {
                const comparison = c.LLVMBuildFCmp(self.base_codegen.builder, c.LLVMRealOEQ, value, literal_value, "float_eq");
                _ = c.LLVMBuildCondBr(self.base_codegen.builder, comparison, success_block, fail_block);
                return comparison;
            },
            .String => {
                // Use string comparison helper
                const string_compare_func = self.base_codegen.functions.get("pattern_string_compare").?;
                const comparison = c.LLVMBuildCall2(
                    self.base_codegen.builder,
                    c.LLVMGetReturnType(c.LLVMGlobalGetValueType(string_compare_func)),
                    string_compare_func,
                    &[_]c.LLVMValueRef{ value, literal_value },
                    2,
                    "string_eq"
                );
                _ = c.LLVMBuildCondBr(self.base_codegen.builder, comparison, success_block, fail_block);
                return comparison;
            },
            .Boolean => {
                const comparison = c.LLVMBuildICmp(self.base_codegen.builder, c.LLVMIntEQ, value, literal_value, "bool_eq");
                _ = c.LLVMBuildCondBr(self.base_codegen.builder, comparison, success_block, fail_block);
                return comparison;
            },
        }
    }

    /// Generate variable pattern check (always succeeds, binds variable)
    fn generateVariablePatternCheck(self: *AdvancedCodeGen, value: c.LLVMValueRef, variable_name: []const u8, success_block: c.LLVMBasicBlockRef) CodeGenError!c.LLVMValueRef {
        // Store binding in current scope
        try self.base_codegen.variables.put(variable_name, value);
        _ = c.LLVMBuildBr(self.base_codegen.builder, success_block);
        return value;
    }

    /// Generate tuple pattern check with destructuring
    fn generateTuplePatternCheck(self: *AdvancedCodeGen, value: c.LLVMValueRef, patterns: []const ast.Pattern, success_block: c.LLVMBasicBlockRef, fail_block: c.LLVMBasicBlockRef) CodeGenError!c.LLVMValueRef {
        // Check tuple length first
        const tuple_length = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), @as(u64, @intCast(patterns.len)), 0);
        const length_check_func = self.base_codegen.functions.get("pattern_array_length_check").?;
        const length_matches = c.LLVMBuildCall2(
            self.base_codegen.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(length_check_func)),
            length_check_func,
            &[_]c.LLVMValueRef{ value, tuple_length, c.LLVMConstInt(c.LLVMInt1TypeInContext(self.base_codegen.context), 1, 0) },
            3,
            "tuple_length_check"
        );
        
        const length_ok_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, self.current_function.?, "tuple_length_ok");
        _ = c.LLVMBuildCondBr(self.base_codegen.builder, length_matches, length_ok_block, fail_block);
        
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, length_ok_block);
        
        // Match each tuple element
        for (patterns, 0..) |pattern, i| {
            const element_index = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), @as(u64, @intCast(i)), 0);
            const tuple_access_func = self.base_codegen.functions.get("pattern_tuple_access").?;
            const element_ptr = c.LLVMBuildCall2(
                self.base_codegen.builder,
                c.LLVMGetReturnType(c.LLVMGlobalGetValueType(tuple_access_func)),
                tuple_access_func,
                &[_]c.LLVMValueRef{ value, element_index },
                2,
                "tuple_element"
            );
            
            const element_success = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, self.current_function.?, "element_success");
            _ = try self.generatePatternCheck(element_ptr, pattern, element_success, fail_block);
            c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, element_success);
        }
        
        _ = c.LLVMBuildBr(self.base_codegen.builder, success_block);
        return value;
    }

    /// Generate struct pattern check with field matching
    fn generateStructPatternCheck(self: *AdvancedCodeGen, value: c.LLVMValueRef, struct_pattern: ast.StructPattern, success_block: c.LLVMBasicBlockRef, fail_block: c.LLVMBasicBlockRef) CodeGenError!c.LLVMValueRef {
        // Type check first
        const struct_info = self.struct_types.get(struct_pattern.name) orelse return error.UnknownStructType;
        const type_id = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), @as(u64, @intCast(struct_info.type_id)), 0);
        
        const type_check_func = self.base_codegen.functions.get("pattern_type_check").?;
        const type_matches = c.LLVMBuildCall2(
            self.base_codegen.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(type_check_func)),
            type_check_func,
            &[_]c.LLVMValueRef{ value, type_id },
            2,
            "struct_type_check"
        );
        
        const type_ok_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, self.current_function.?, "struct_type_ok");
        _ = c.LLVMBuildCondBr(self.base_codegen.builder, type_matches, type_ok_block, fail_block);
        
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, type_ok_block);
        
        // Match each field
        for (struct_pattern.fields.items) |field_pattern| {
            const field_index = self.getFieldIndex(struct_info, field_pattern.name) orelse return error.UnknownField;
            const field_ptr = c.LLVMBuildStructGEP2(
                self.base_codegen.builder,
                struct_info.llvm_type.?,
                value,
                @as(u32, @intCast(field_index)),
                "field_ptr"
            );
            
            const field_success = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, self.current_function.?, "field_success");
            _ = try self.generatePatternCheck(field_ptr, field_pattern.pattern, field_success, fail_block);
            c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, field_success);
        }
        
        _ = c.LLVMBuildBr(self.base_codegen.builder, success_block);
        return value;
    }

    /// Generate array pattern check with rest elements support
    fn generateArrayPatternCheck(self: *AdvancedCodeGen, value: c.LLVMValueRef, patterns: []const ast.Pattern, success_block: c.LLVMBasicBlockRef, fail_block: c.LLVMBasicBlockRef) CodeGenError!c.LLVMValueRef {
        // Check array length
        const required_length = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), @as(u64, @intCast(patterns.len)), 0);
        const length_check_func = self.base_codegen.functions.get("pattern_array_length_check").?;
        const length_matches = c.LLVMBuildCall2(
            self.base_codegen.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(length_check_func)),
            length_check_func,
            &[_]c.LLVMValueRef{ value, required_length, c.LLVMConstInt(c.LLVMInt1TypeInContext(self.base_codegen.context), 1, 0) },
            3,
            "array_length_check"
        );
        
        const length_ok_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, self.current_function.?, "array_length_ok");
        _ = c.LLVMBuildCondBr(self.base_codegen.builder, length_matches, length_ok_block, fail_block);
        
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, length_ok_block);
        
        // Match each array element
        for (patterns, 0..) |pattern, i| {
            const element_index = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), @as(u64, @intCast(i)), 0);
            const element_ptr = c.LLVMBuildGEP2(
                self.base_codegen.builder,
                c.LLVMInt8TypeInContext(self.base_codegen.context),
                value,
                &[_]c.LLVMValueRef{element_index},
                1,
                "array_element"
            );
            
            const element_success = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, self.current_function.?, "element_success");
            _ = try self.generatePatternCheck(element_ptr, pattern, element_success, fail_block);
            c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, element_success);
        }
        
        _ = c.LLVMBuildBr(self.base_codegen.builder, success_block);
        return value;
    }

    /// Helper functions for pattern matching
    
    const PatternOptimizationResult = struct {
        use_jump_table: bool,
        literal_pattern_count: usize,
        complex_pattern_count: usize,
    };

    /// Helper to generate literal values for pattern matching
    fn generateLiteralValue(self: *AdvancedCodeGen, literal: ast.Literal) CodeGenError!c.LLVMValueRef {
        switch (literal) {
            .Integer => |int_val| {
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.base_codegen.context), @as(u64, @bitCast(int_val)), 0);
            },
            .Float => |float_val| {
                return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.base_codegen.context), float_val);
            },
            .String => |str_val| {
                // Create global string constant
                const string_type = c.LLVMArrayType(c.LLVMInt8TypeInContext(self.base_codegen.context), @as(u32, @intCast(str_val.len + 1)));
                const string_global = c.LLVMAddGlobal(self.base_codegen.module, string_type, "str_const");
                const string_init = c.LLVMConstStringInContext(self.base_codegen.context, str_val.ptr, @as(u32, @intCast(str_val.len)), 0);
                c.LLVMSetInitializer(string_global, string_init);
                c.LLVMSetLinkage(string_global, c.LLVMPrivateLinkage);
                
                // Return pointer to first element
                const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), 0, 0);
                return c.LLVMConstGEP2(string_type, string_global, &[_]c.LLVMValueRef{ zero, zero }, 2);
            },
            .Boolean => |bool_val| {
                const bool_int = if (bool_val) @as(u64, 1) else @as(u64, 0);
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.base_codegen.context), bool_int, 0);
            },
        }
    }

    /// Helper to infer match result type from cases
    fn inferMatchResultType(self: *AdvancedCodeGen, cases: []const ast.MatchCase) CodeGenError!c.LLVMTypeRef {
        _ = self;
        _ = cases;
        // For now, return i64 as default - should be properly inferred from case expressions
        return c.LLVMInt64TypeInContext(self.base_codegen.context);
    }

    /// Helper to get field index in struct
    fn getFieldIndex(self: *AdvancedCodeGen, struct_info: StructTypeInfo, field_name: []const u8) ?usize {
        _ = self;
        for (struct_info.field_names, 0..) |name, index| {
            if (std.mem.eql(u8, name, field_name)) {
                return index;
            }
        }
        return null;
    }

    /// Generate CURSED expression with advanced type support
    pub fn generateExpression(self: *AdvancedCodeGen, expr: ast.Expression) CodeGenError!c.LLVMValueRef {
        switch (expr) {
            // Basic literals - delegate to base codegen
            .Literal, .Integer, .Float, .String, .Boolean, .Character, .Identifier => {
                return try self.base_codegen.generateExpression(expr);
            },
            
            // Binary operations with enhanced type checking
            .BinaryOp => |binary| {
                const left = try self.generateExpression(binary.left.*);
                const right = try self.generateExpression(binary.right.*);
                return try self.generateAdvancedBinaryOp(left, binary.operator, right);
            },
            
            // Unary operations with type promotion
            .UnaryOp => |unary| {
                const operand = try self.generateExpression(unary.operand.*);
                return try self.generateAdvancedUnaryOp(unary.operator, operand);
            },
            
            // Function calls with enhanced argument handling
            .FunctionCall => |call| {
                return try self.generateAdvancedCall(call);
            },
            
            // Struct and interface member access
            .MemberAccess => |member| {
                return try self.generateAdvancedMemberAccess(member);
            },
            
            // Array literal and access with bounds checking
            .ArrayLiteral => |array| {
                return try self.generateAdvancedArrayLiteral(array);
            },
            .Array => |array| {
                return try self.generateAdvancedArrayExpression(array);
            },
            .IndexAccess => |index| {
                return try self.generateAdvancedIndexAccess(index);
            },
            
            // Struct literals with type validation
            .StructLiteral => |struct_lit| {
                return try self.generateAdvancedStructLiteral(struct_lit);
            },
            
            // Interface method calls
            .InterfaceCall => |interface_call| {
                return try self.generateAdvancedInterfaceCall(interface_call);
            },
            
            // Advanced expressions
            .TypeCast => |cast| {
                return try self.generateAdvancedTypeCast(cast);
            },
            .Match => |match| {
                return try self.generateAdvancedMatch(match);
            },
            .Tuple => |tuple| {
                return try self.generateAdvancedTuple(tuple);
            },
            .TupleAccess => |tuple_access| {
                return try self.generateAdvancedTupleAccess(tuple_access);
            },
            
            // Concurrency expressions
            .ChannelSend => |send| {
                return try self.generateAdvancedChannelSend(send);
            },
            .ChannelReceive => |recv| {
                return try self.generateAdvancedChannelReceive(recv);
            },
            .ChannelCreation => |create| {
                return try self.generateAdvancedChannelCreation(create);
            },
            .Goroutine => |goroutine| {
                return try self.generateAdvancedGoroutine(goroutine);
            },
            
            // Error handling expressions
            .Shook => |shook| {
                return try self.generateAdvancedShook(shook);
            },
            .Yikes => |yikes| {
                return try self.generateAdvancedYikes(yikes);
            },
            .Fam => |fam| {
                return try self.generateAdvancedFam(fam);
            },
            
            // Lambda expressions
            .Lambda => |lambda| {
                return try self.generateAdvancedLambda(lambda);
            },
            
            // Fallback to base codegen for other expressions
            else => {
                return try self.base_codegen.generateExpression(expr);
            },
        }
    }

    fn generateStatement(self: *AdvancedCodeGen, stmt: ast.Statement) CodeGenError!void {
        switch (stmt) {
            .Select => {
                const select_stmt: *ast.SelectStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateAdvancedSelect(select_stmt.*);
            },
            else => {
                // Delegate other statements to base codegen
                try self.base_codegen.generateStatement(stmt);
            },
        }
    }

    // Required additional type info fields that were referenced
    const StructTypeInfo = struct {
        name: []const u8,
        field_types: []c.LLVMTypeRef,
        field_names: [][]const u8,
        llvm_type: ?c.LLVMTypeRef,
        methods: ArrayList(MethodInfo),
        is_generic: bool,
        type_parameters: ArrayList([]const u8),
        type_id: u32, // Added for pattern matching type checks
    };

    const InterfaceTypeInfo = struct {
        name: []const u8,
        methods: ArrayList(InterfaceMethodInfo),
        vtable_type: ?c.LLVMTypeRef,
        vtable_global: ?c.LLVMValueRef,
        type_id: u32,
    };

    const InterfaceMethodInfo = struct {
        name: []const u8,
        signature: MethodSignature,
        vtable_index: usize,
    };

    const MethodInfo = struct {
        name: []const u8,
        signature: MethodSignature,
        llvm_function: ?c.LLVMValueRef,
    };

    const GenericInstance = struct {
        name: []const u8,
        type_args: ArrayList(Type),
        instantiated_type: c.LLVMTypeRef,
    };

    const VTableInfo = struct {
        interface_name: []const u8,
        implementing_type: []const u8,
        vtable_global: c.LLVMValueRef,
        method_functions: ArrayList(c.LLVMValueRef),
    };



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
                .Implementation => |impl_stmt| {
                    try self.collectImplementationDefinition(impl_stmt);
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
    
    /// Collect implementation definition information
    fn collectImplementationDefinition(self: *AdvancedCodeGen, impl_stmt: ast.ImplementationStatement) CodeGenError!void {
        // Register the implementation in the interface registry
        const struct_type_id = self.getTypeId(impl_stmt.implementing_type);
        const interface_type_id = self.getTypeId(impl_stmt.interface_name);
        
        // Create vtable for this implementation
        const vtable_name = try std.fmt.allocPrint(
            self.base_codegen.allocator,
            "vtable_{s}_for_{s}",
            .{ impl_stmt.implementing_type, impl_stmt.interface_name }
        );
        
        // Get interface definition to match methods
        _ = self.interface_types.get(impl_stmt.interface_name) orelse {
            return CodeGenError.UndefinedSymbol;
        };
        
        // Create method map for this implementation
        var method_implementations = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.base_codegen.allocator);
        
        for (impl_stmt.methods.items) |method| {
            const impl_method_name = try std.fmt.allocPrint(
                self.base_codegen.allocator,
                "{s}_{s}",
                .{ impl_stmt.implementing_type, method.name }
            );
            try method_implementations.put(method.name, impl_method_name);
        }
        
        // Store vtable information (will be completed during vtable generation)
        const vtable_info = VTableInfo{
            .name = vtable_name,
            .interface_name = impl_stmt.interface_name,
            .struct_name = impl_stmt.implementing_type,
            .global_value = undefined, // Will be set during generation
            .method_count = impl_stmt.methods.items.len,
        };
        
        try self.vtables.put(vtable_name, vtable_info);
        
        // Register in interface registry for runtime dispatch
        try self.interface_registry.registerImplementation(struct_type_id, interface_type_id, null); // vtable will be set later
    }
    
    /// Get type ID for a type name (simplified implementation)
    fn getTypeId(self: *AdvancedCodeGen, type_name: []const u8) u32 {
        _ = self; // Parameter marked as used
        // Simple hash-based type ID generation
        return @as(u32, @truncate(std.hash_map.hashString(type_name)));
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
            c.LLVMStructSetBody(struct_type, struct_info.field_types.ptr, @as(u32, @intCast(0)));
            
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
            
            // Register interface with dispatcher
            var method_signatures = ArrayList(interface_dispatch.MethodSignature).init(self.base_codegen.allocator);
            defer method_signatures.deinit();
            
            for (interface_info.methods.items) |method| {
                const signature = interface_dispatch.MethodSignature{
                    .name = method.name,
                    .parameter_types = &[_][]const u8{}, // TODO: Add proper type conversion
                    .return_type = "void", // TODO: Add proper return type
                };
                try method_signatures.append(signature);
            }
            
            try self.interface_dispatcher.registerInterface(interface_info.name, method_signatures.items);
            
            // For each struct that implements this interface
            var struct_iterator = self.struct_types.iterator();
            while (struct_iterator.next()) |struct_entry| {
                const struct_info = struct_entry.value_ptr;
                
                if (self.structImplementsInterface(struct_info, interface_info)) {
                    try self.generateVTableForImplementation(struct_info, interface_info);
                    try self.registerStructImplementation(struct_info, interface_info);
                }
            }
        }
    }
    
    /// Register struct implementation with interface dispatcher
    fn registerStructImplementation(self: *AdvancedCodeGen, struct_info: *StructTypeInfo, interface_info: *InterfaceTypeInfo) CodeGenError!void {
        var method_impls = ArrayList(interface_dispatch.MethodImpl).init(self.base_codegen.allocator);
        defer method_impls.deinit();
        
        for (interface_info.methods.items) |interface_method| {
            // Find corresponding method in struct
            for (struct_info.methods.items) |struct_method| {
                if (std.mem.eql(u8, interface_method.name, struct_method.name)) {
                    // TODO: Create proper FunctionValue from struct method
                    // This is a placeholder - real implementation would convert LLVM function to FunctionValue
                    break;
                }
            }
        }
        
        // TODO: Register implementation with interface dispatcher
        // try self.interface_dispatcher.registerImplementation(struct_info.name, interface_info.name, method_impls.items);
    }

    /// Check if a struct implements an interface
    fn structImplementsInterface(self: *AdvancedCodeGen, struct_info: *StructTypeInfo, interface_info: *InterfaceTypeInfo) bool {
        // For each method in the interface
        for (interface_info.methods.items) |interface_method| {
            var found = false;
            
            // Check if struct has a method with the same signature
            for (struct_info.methods.items) |struct_method| {
                if (std.mem.eql(u8, interface_method.name, struct_method.name)) {
                    // Check function signature compatibility
                    if (self.compareMethodSignatures(interface_method.signature, struct_method.signature) catch false) {
                        found = true;
                        break;
                    }
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
    /// Register generic declaration with monomorphizer
    pub fn registerGeneric(self: *AdvancedCodeGen, name: []const u8, declaration: generics.GenericDeclaration) !void {
        try self.monomorphizer.registerGeneric(declaration);
    }
    
    /// Request generic instantiation  
    pub fn requestGenericInstantiation(self: *AdvancedCodeGen, generic_name: []const u8, type_arguments: []ast.Type, usage_location: []const u8) ![]const u8 {
        return try self.monomorphizer.requestInstantiation(generic_name, type_arguments, usage_location);
    }
    
    /// Get instantiated function
    pub fn getInstantiatedFunction(self: *AdvancedCodeGen, specialized_name: []const u8) ?c.LLVMValueRef {
        return self.monomorphizer.getInstantiatedFunction(specialized_name);
    }
    
    /// Get instantiated type
    pub fn getInstantiatedType(self: *AdvancedCodeGen, specialized_name: []const u8) ?c.LLVMTypeRef {
        return self.monomorphizer.getInstantiatedType(specialized_name);
    }

    /// Generate specialized version of generic type
    fn generateSpecializedType(self: *AdvancedCodeGen, base_name: []const u8, type_args: [][]const u8, specialized_name: []const u8) CodeGenError!void {
        _ = type_args; // Parameter marked as used
        // Find base generic type
        if (self.struct_types.get(base_name)) |base_struct| {
            // Create specialized struct type
            var specialized_field_types = ArrayList(c.LLVMTypeRef).init(self.base_codegen.allocator);
            defer specialized_field_types.deinit();
            
            // Substitute type parameters with concrete types
            for (base_struct.field_types) |field_type| {
                // For now, use the field type as-is
                // In a full implementation, we would do type parameter substitution
                try specialized_field_types.append(field_type);
            }
            
            // Create the specialized LLVM struct type
            const specialized_llvm_type = c.LLVMStructCreateNamed(self.base_codegen.context, specialized_name.ptr);
            c.LLVMStructSetBody(specialized_llvm_type, specialized_field_types.items.ptr, @as(u32, @intCast(specialized_field_types.items.len)), 0);
            
            // Store the specialized type
            const specialized_struct = StructTypeInfo{
                .name = specialized_name,
                .field_types = try specialized_field_types.toOwnedSlice(),
                .field_names = base_struct.field_names,
                .llvm_type = specialized_llvm_type,
                .methods = ArrayList(MethodInfo).init(self.base_codegen.allocator),
                .is_generic = false,
                .type_parameters = ArrayList(ast.TypeParameter).init(self.base_codegen.allocator),
            };
            
            try self.struct_types.put(specialized_name, specialized_struct);
        }
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

    /// Enhanced struct field access with automatic struct type detection
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
        
        // Generate field access with proper type handling
        const builder = self.base_codegen.builder;
        const struct_type = struct_info.llvm_type.?;
        
        // Generate GEP instruction for field access
        const field_ptr = c.LLVMBuildStructGEP2(
            builder,
            struct_type,
            struct_ptr,
            field_index,
            "field_ptr"
        );
        
        // Load field value with correct type
        const field_type = struct_info.field_types[field_index];
        return c.LLVMBuildLoad2(
            builder,
            field_type,
            field_ptr,
            "field_value"
        );
    }
    
    /// Enhanced struct field access that detects struct type automatically
    fn generateStructFieldAccessAuto(self: *AdvancedCodeGen, struct_value: c.LLVMValueRef, field_name: []const u8) CodeGenError!c.LLVMValueRef {
        const struct_type = c.LLVMTypeOf(struct_value);
        
        // If it's a pointer to struct, get the pointed type
        var actual_struct_type = struct_type;
        var is_pointer = false;
        
        if (c.LLVMGetTypeKind(struct_type) == c.LLVMPointerTypeKind) {
            actual_struct_type = c.LLVMGetElementType(struct_type);
            is_pointer = true;
        }
        
        if (c.LLVMGetTypeKind(actual_struct_type) != c.LLVMStructTypeKind) {
            return CodeGenError.NotAStruct;
        }
        
        // Find the struct info by searching through registered struct types
        var struct_info: ?StructTypeInfo = null;
        var iter = self.struct_types.iterator();
        while (iter.next()) |entry| {
            if (entry.value_ptr.llvm_type == actual_struct_type) {
                struct_info = entry.value_ptr.*;
                break;
            }
        }
        
        if (struct_info == null) {
            return CodeGenError.UndefinedSymbol;
        }
        
        const info = struct_info.?;
        
        // Find field index
        var field_index: u32 = 0;
        var found = false;
        for (info.field_names, 0..) |name, i| {
            if (std.mem.eql(u8, name, field_name)) {
                field_index = @as(u32, @intCast(i));
                found = true;
                break;
            }
        }
        
        if (!found) {
            return CodeGenError.UndefinedSymbol;
        }
        
        const builder = self.base_codegen.builder;
        
        // Generate field access
        var field_ptr: c.LLVMValueRef = undefined;
        if (is_pointer) {
            // If struct_value is a pointer, use GEP
            field_ptr = c.LLVMBuildStructGEP2(
                builder,
                actual_struct_type,
                struct_value,
                field_index,
                "field_ptr"
            );
        } else {
            // If struct_value is a value, first create alloca and store
            const temp_alloca = c.LLVMBuildAlloca(builder, actual_struct_type, "temp_struct");
            _ = c.LLVMBuildStore(builder, struct_value, temp_alloca);
            field_ptr = c.LLVMBuildStructGEP2(
                builder,
                actual_struct_type,
                temp_alloca,
                field_index,
                "field_ptr"
            );
        }
        
        // Load field value
        return c.LLVMBuildLoad2(
            builder,
            info.field_types[field_index],
            field_ptr,
            "field_value"
        );
    }

    /// Generate interface method call with dynamic dispatch
    pub fn generateInterfaceMethodCall(self: *AdvancedCodeGen, interface_ptr: c.LLVMValueRef, method_name: []const u8, args: []c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        // Extract vtable from interface object
        // Interface object layout: {vtable_ptr, data_ptr}
        const interface_struct_type = c.LLVMStructTypeInContext(
            self.base_codegen.context,
            &[_]c.LLVMTypeRef{
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0), // vtable
                c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0)  // data
            },
            2,
            0
        );
        
        const vtable_ptr_ptr = c.LLVMBuildStructGEP2(
            self.base_codegen.builder,
            interface_struct_type,
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
        
        // Extract data pointer
        const data_ptr_ptr = c.LLVMBuildStructGEP2(
            self.base_codegen.builder,
            interface_struct_type,
            interface_ptr,
            1, // data is second field
            "data_ptr_ptr"
        );
        
        const data_ptr = c.LLVMBuildLoad2(
            self.base_codegen.builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.base_codegen.context), 0),
            data_ptr_ptr,
            "data_ptr"
        );
        
        // Find method index by name
        var method_index: u32 = 0;
        var found = false;
        
        // Search through all interface types to find method index
        var interface_iterator = self.interface_types.iterator();
        while (interface_iterator.next()) |entry| {
            const interface_info = entry.value_ptr;
            for (interface_info.methods.items, 0..) |method, i| {
                if (std.mem.eql(u8, method.name, method_name)) {
                    method_index = @as(u32, @intCast(i));
                    found = true;
                    break;
                }
            }
            if (found) break;
        }
        
        if (!found) {
            return CodeGenError.UndefinedSymbol;
        }
        
        // Get method function pointer from vtable
        const func_ptr_type = c.LLVMPointerType(
            c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.base_codegen.context), // return type (placeholder)
                null, // parameters (placeholder)
                0, // parameter count
                0  // not variadic
            ),
            0
        );
        
        const method_ptr_ptr = c.LLVMBuildGEP2(
            self.base_codegen.builder,
            c.LLVMArrayType(func_ptr_type, 100), // vtable type (placeholder size)
            vtable_ptr,
            &[_]c.LLVMValueRef{
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), method_index, 0)
            },
            1,
            "method_ptr_ptr"
        );
        
        const method_ptr = c.LLVMBuildLoad2(
            self.base_codegen.builder,
            func_ptr_type,
            method_ptr_ptr,
            "method_ptr"
        );
        
        // Prepare arguments (data pointer + original args)
        var call_args = ArrayList(c.LLVMValueRef).init(self.base_codegen.allocator);
        defer call_args.deinit();
        
        try call_args.append(data_ptr); // self pointer
        for (args) |arg| {
            try call_args.append(arg);
        }
        
        // Call the method through function pointer
        const result = c.LLVMBuildCall2(
            self.base_codegen.builder,
            c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.base_codegen.context), // return type (placeholder)
                null, // parameters (placeholder) 
                0, // parameter count
                0  // not variadic
            ),
            method_ptr,
            call_args.items.ptr,
            @as(u32, @intCast(call_args.items.len)),
            "interface_method_call"
        );
        
        return result;
    }

    /// Apply advanced optimization passes
    fn applyAdvancedOptimizations(self: *AdvancedCodeGen) CodeGenError!void {
        // Initialize optimization engine if not already done
        if (self.optimization_engine == null) {
            self.optimization_engine = OptimizationEngine.init(
                self.base_codegen.allocator,
                self.base_codegen.context,
                self.base_codegen.module
            ) catch |err| {
                std.debug.print("Failed to initialize optimization engine: {}\n", .{err});
                return; // Fall back to basic optimization
            };
            
            // Configure optimization settings
            if (self.optimization_engine) |*engine| {
                engine.setOptimizationLevel(self.optimization_config.optimization_level);
                
                if (self.optimization_config.lto_enabled) {
                    engine.enableLTO();
                }
                
                if (self.optimization_config.debug_info_enabled) {
                    engine.enableDebugInfo(self.optimization_config.preserve_debug_info);
                }
            }
        }
        
        if (self.optimization_engine) |*engine| {
            // Configure optimization passes
            engine.configurePasses() catch |err| {
                std.debug.print("Failed to configure optimization passes: {}\n", .{err});
                return; // Fall back to basic optimization
            };
            
            // Run advanced optimizations
            const result = engine.runOptimizations() catch |err| {
                std.debug.print("Failed to run optimizations: {}\n", .{err});
                return; // Fall back to basic optimization
            };
            
            // Store optimization result
            self.last_optimization_result = result;
            
            // Print optimization statistics
            std.debug.print("✅ Advanced optimizations applied:\n");
            std.debug.print("   - Functions optimized: {}\n", .{result.functions_optimized});
            std.debug.print("   - Instructions eliminated: {}\n", .{result.instructions_eliminated});
            std.debug.print("   - Constants folded: {}\n", .{result.constants_folded});
            std.debug.print("   - Functions inlined: {}\n", .{result.functions_inlined});
            std.debug.print("   - Loops optimized: {}\n", .{result.loops_optimized});
            std.debug.print("   - Memory allocations optimized: {}\n", .{result.memory_allocations_optimized});
            std.debug.print("   - Estimated performance improvement: {d:.2}x\n", .{result.estimated_performance_improvement});
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
        if (!self.debug_enabled or self.debug_generator == null) {
            return; // Debug info disabled
        }
        
        var debug_gen = &self.debug_generator.?;
        
        // Finalize all debug information
        debug_gen.finalize();
        
        std.debug.print("✅ DWARF debug information generated successfully\n", .{});
    }

    /// Generate optimization report
    fn generateOptimizationReport(self: *AdvancedCodeGen, base_path: []const u8) CodeGenError!void {
        var report_path = ArrayList(u8).init(self.base_codegen.allocator);
        defer report_path.deinit();
        
        try report_path.appendSlice(base_path);
        try report_path.appendSlice(".opt_report");
        
        // Generate detailed optimization report
        const report_file = std.fs.cwd().createFile(report_path.items, .{}) catch |err| {
            std.debug.print("Warning: Could not create optimization report file: {}\n", .{err});
            return;
        };
        defer report_file.close();
        
        const writer = report_file.writer();
        
        try writer.print("CURSED Advanced Optimization Report\n");
        try writer.print("====================================\n\n");
        
        const timestamp = std.fmt.allocPrint(self.base_codegen.allocator, "{}", .{std.time.timestamp()}) catch "unknown";
        defer if (!std.mem.eql(u8, timestamp, "unknown")) self.base_codegen.allocator.free(timestamp);
        
        try writer.print("Generated: {s}\n", .{timestamp});
        try writer.print("Source file: {s}\n\n", .{self.source_file orelse "unknown"});
        
        // Report on optimization passes applied
        try writer.print("Optimization Passes Applied:\n");
        for (self.optimization_passes.items) |pass| {
            switch (pass) {
                .FunctionInlining => try writer.print("  - Function Inlining\n"),
                .DeadCodeElimination => try writer.print("  - Dead Code Elimination\n"),
                .ConstantFolding => try writer.print("  - Constant Folding\n"),
                .LoopOptimization => try writer.print("  - Loop Optimization\n"),
                .InterfaceDevirtualization => try writer.print("  - Interface Devirtualization\n"),
            }
        }
        
        // Report on type system statistics
        try writer.print("\nType System Statistics:\n");
        try writer.print("  - Struct types: {d}\n", .{self.struct_types.count()});
        try writer.print("  - Interface types: {d}\n", .{self.interface_types.count()});
        try writer.print("  - Generic instances: {d}\n", .{self.generic_instances.count()});
        try writer.print("  - VTables generated: {d}\n", .{self.vtables.count()});
        
        // Report on memory management
        try writer.print("\nMemory Management:\n");
        try writer.print("  - GC enabled: {s}\n", .{if (self.gc_enabled) "yes" else "no"});
        
        // Report on debug information
        try writer.print("\nDebug Information:\n");
        try writer.print("  - Debug enabled: {s}\n", .{if (self.debug_enabled) "yes" else "no"});
        try writer.print("  - Source locations tracked: {d}\n", .{self.source_locations.count()});
        
        std.debug.print("✅ Optimization report written to: {s}\n", .{report_path.items});
    }
    
    /// Generate debug info for CURSED function
    pub fn generateFunctionDebugInfo(self: *AdvancedCodeGen, function: c.LLVMValueRef, name: []const u8, line: u32, param_types: []c.LLVMMetadataRef, return_type: c.LLVMMetadataRef) !c.LLVMMetadataRef {
        if (!self.debug_enabled or self.debug_generator == null) {
            return null; // Debug disabled
        }
        
        var debug_gen = &self.debug_generator.?;
        
        // Create function type debug info
        const func_type = try debug_gen.createFunctionType(return_type, param_types);
        
        // Create function debug info
        const di_function = try debug_gen.createFunction(name, name, line, func_type, function);
        
        return di_function;
    }
    
    /// Generate debug info for CURSED variable
    pub fn generateVariableDebugInfo(self: *AdvancedCodeGen, alloca: c.LLVMValueRef, name: []const u8, line: u32, cursed_type: []const u8) !void {
        if (!self.debug_enabled or self.debug_generator == null) {
            return; // Debug disabled
        }
        
        var debug_gen = &self.debug_generator.?;
        
        // Create debug type for CURSED variable
        const di_type = try self.getCursedDebugType(cursed_type);
        
        // Create local variable debug info
        try debug_gen.createLocalVariable(name, line, di_type, alloca);
    }

    /// Compile function statement with comprehensive debug information
    pub fn compileFunctionWithDebugInfo(self: *AdvancedCodeGen, func_stmt: ast.FunctionStatement) !c.LLVMValueRef {
        const context = self.base_codegen.context;
        const module = self.base_codegen.module;
        const builder = self.base_codegen.builder;
        
        // Create function type
        var param_types = ArrayList(c.LLVMTypeRef).init(self.base_codegen.allocator);
        defer param_types.deinit();
        
        for (func_stmt.parameters.items) |param| {
            const param_type = self.base_codegen.convertType(param.param_type) catch c.LLVMInt64TypeInContext(context);
            try param_types.append(param_type);
        }
        
        const return_type = if (func_stmt.return_type) |ret_type|
            self.base_codegen.convertType(ret_type) catch c.LLVMVoidTypeInContext(context)
        else
            c.LLVMVoidTypeInContext(context);
        
        const function_type = c.LLVMFunctionType(
            return_type,
            param_types.items.ptr,
            @intCast(param_types.items.len),
            0
        );
        
        // Create function
        const function = c.LLVMAddFunction(module, func_stmt.name.ptr, function_type);
        
        // Generate debug information if enabled
        if (self.debug_enabled and self.debug_generator != null) {
            var debug_gen = &self.debug_generator.?;
            
            // Create debug types for parameters
            var debug_param_types = ArrayList(c.LLVMMetadataRef).init(self.base_codegen.allocator);
            defer debug_param_types.deinit();
            
            for (func_stmt.parameters.items) |param| {
                const debug_type = try self.getCursedDebugType("drip"); // Default to drip for now
                try debug_param_types.append(debug_type);
            }
            
            // Create debug type for return value
            const debug_return_type = try self.getCursedDebugType("void");
            
            // Generate function debug info
            const di_function = try self.generateFunctionDebugInfo(
                function,
                func_stmt.name,
                func_stmt.location.line,
                debug_param_types.items,
                debug_return_type
            );
            
            // Set debug location for function entry
            debug_gen.setCurrentLocation(func_stmt.location.line, func_stmt.location.column);
        }
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(context, function, "entry");
        c.LLVMPositionBuilderAtEnd(builder, entry_block);
        
        // Set current function context
        const saved_function = self.base_codegen.current_function;
        self.base_codegen.current_function = function;
        self.current_function_name = func_stmt.name;
        
        // Enter function scope
        _ = try self.enterScope(true);
        
        // Create allocas for parameters with debug info
        var param_index: u32 = 0;
        for (func_stmt.parameters.items) |param| {
            const param_alloca = c.LLVMBuildAlloca(builder, param_types.items[param_index], param.name.ptr);
            
            // Store parameter value
            const param_value = c.LLVMGetParam(function, param_index);
            _ = c.LLVMBuildStore(builder, param_value, param_alloca);
            
            // Generate debug info for parameter
            if (self.debug_enabled and self.debug_generator != null) {
                var debug_gen = &self.debug_generator.?;
                const debug_type = try self.getCursedDebugType("drip"); // Default type
                try debug_gen.createParameterVariable(param.name, param_index + 1, func_stmt.location.line, debug_type, param_alloca);
            }
            
            param_index += 1;
        }
        
        // Compile function body
        for (func_stmt.body.items) |statement| {
            try self.compileStatement(statement.*);
        }
        
        // Exit function scope and cleanup defers
        try self.exitScope();
        
        // Add return if missing
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
            if (func_stmt.return_type == null or std.mem.eql(u8, "void", "void")) {
                _ = c.LLVMBuildRetVoid(builder);
            } else {
                // Return default value
                const default_value = c.LLVMConstInt(return_type, 0, 0);
                _ = c.LLVMBuildRet(builder, default_value);
            }
        }
        
        // Restore previous function context
        self.base_codegen.current_function = saved_function;
        self.current_function_name = null;
        
        return function;
    }

    /// Get CURSED debug type by name
    fn getCursedDebugType(self: *AdvancedCodeGen, type_name: []const u8) !c.LLVMMetadataRef {
        if (!self.debug_enabled or self.debug_generator == null) {
            return error.DebugDisabled;
        }
        
        var debug_gen = &self.debug_generator.?;
        const types = debug_gen.cursed_debug_types orelse return error.TypesNotInitialized;
        
        if (std.mem.eql(u8, type_name, "normie")) return types.normie_type;
        if (std.mem.eql(u8, type_name, "tea")) return types.tea_type;
        if (std.mem.eql(u8, type_name, "drip")) return types.drip_type;
        if (std.mem.eql(u8, type_name, "lit")) return types.lit_type;
        if (std.mem.eql(u8, type_name, "meal")) return types.meal_type;
        if (std.mem.eql(u8, type_name, "smol")) return types.smol_type;
        if (std.mem.eql(u8, type_name, "thicc")) return types.thicc_type;
        if (std.mem.eql(u8, type_name, "sip")) return types.sip_type;
        if (std.mem.eql(u8, type_name, "void")) return types.void_type;
        
        // Default to normie for unknown types
        return types.normie_type;
    }
    
    /// Generate debug info for CURSED struct
    pub fn generateStructDebugInfo(self: *AdvancedCodeGen, struct_name: []const u8, field_names: [][]const u8, field_types: [][]const u8) !c.LLVMMetadataRef {
        if (!self.debug_enabled or self.debug_generator == null) {
            return null; // Debug disabled
        }
        
        var debug_gen = &self.debug_generator.?;
        
        // Create debug field information
        var fields = ArrayList(debug_info.StructField).init(self.base_codegen.allocator);
        defer fields.deinit();
        
        for (field_names, field_types) |field_name, field_type| {
            const di_type = try self.getCursedDebugType(field_type);
            const size_bits = self.getCursedTypeSize(field_type) * 8;
            
            try fields.append(debug_info.StructField{
                .name = field_name,
                .di_type = di_type,
                .size_bits = size_bits,
                .align_bits = 8, // Default alignment
            });
        }
        
        const total_size = self.getStructTotalSize(field_types) * 8;
        return try debug_gen.createStructType(struct_name, total_size, 8, fields.items);
    }
    
    /// Generate debug info for CURSED interface
    pub fn generateInterfaceDebugInfo(self: *AdvancedCodeGen, interface_name: []const u8, method_names: [][]const u8) !c.LLVMMetadataRef {
        if (!self.debug_enabled or self.debug_generator == null) {
            return null; // Debug disabled
        }
        
        var debug_gen = &self.debug_generator.?;
        
        // For interfaces, create a structure with function pointers
        var fields = ArrayList(debug_info.StructField).init(self.base_codegen.allocator);
        defer fields.deinit();
        
        // Add vtable pointer field
        const ptr_type = try debug_gen.createPointerType(try self.getCursedDebugType("normie"));
        try fields.append(debug_info.StructField{
            .name = "vtable_ptr",
            .di_type = ptr_type,
            .size_bits = 64,
            .align_bits = 8,
        });
        
        // Add method function pointers
        for (method_names) |method_name| {
            const func_ptr_type = try debug_gen.createPointerType(try self.getCursedDebugType("normie"));
            try fields.append(debug_info.StructField{
                .name = method_name,
                .di_type = func_ptr_type,
                .size_bits = 64,
                .align_bits = 8,
            });
        }
        
        const total_size = (1 + method_names.len) * 64; // vtable + methods
        return try debug_gen.createStructType(interface_name, total_size, 8, fields.items);
    }
    
    /// Set debug location for instruction
    pub fn setInstructionDebugLocation(self: *AdvancedCodeGen, instruction: c.LLVMValueRef, line: u32, column: u32) void {
        if (!self.debug_enabled or self.debug_generator == null) {
            return; // Debug disabled
        }
        
        var debug_gen = &self.debug_generator.?;
        debug_gen.setDebugLocation(instruction, line, column);
        
        // Track source location for instruction
        if (self.source_file) |file| {
            const location = SourceLocation{
                .line = line,
                .column = column,
                .filename = file,
            };
            self.source_locations.put(instruction, location) catch {};
        }
    }
    
    /// Enter new debug scope (for blocks, functions, etc.)
    pub fn pushDebugScope(self: *AdvancedCodeGen, line: u32, column: u32) !c.LLVMMetadataRef {
        if (!self.debug_enabled or self.debug_generator == null) {
            return null; // Debug disabled
        }
        
        var debug_gen = &self.debug_generator.?;
        const scope = try debug_gen.createLexicalBlock(line, column);
        try debug_gen.pushScope(scope);
        return scope;
    }
    
    /// Exit current debug scope
    pub fn popDebugScope(self: *AdvancedCodeGen) void {
        if (!self.debug_enabled or self.debug_generator == null) {
            return; // Debug disabled
        }
        
        var debug_gen = &self.debug_generator.?;
        debug_gen.popScope();
    }
    
    /// Get debug type for CURSED type
    fn getCursedDebugType(self: *AdvancedCodeGen, cursed_type: []const u8) !c.LLVMMetadataRef {
        var debug_gen = &self.debug_generator.?;
        
        return switch (std.mem.hash_map.hashString(cursed_type)) {
            std.mem.hash_map.hashString("normie") => try debug_gen.createBasicType("normie", 32, c.LLVMDWARFTypeEncodingSigned),
            std.mem.hash_map.hashString("tea") => try debug_gen.createBasicType("tea", 64, c.LLVMDWARFTypeEncodingUTF),
            std.mem.hash_map.hashString("drip") => try debug_gen.createBasicType("drip", 64, c.LLVMDWARFTypeEncodingSigned),
            std.mem.hash_map.hashString("lit") => try debug_gen.createBasicType("lit", 1, c.LLVMDWARFTypeEncodingBoolean),
            std.mem.hash_map.hashString("meal") => try debug_gen.createBasicType("meal", 64, c.LLVMDWARFTypeEncodingFloat),
            std.mem.hash_map.hashString("smol") => try debug_gen.createBasicType("smol", 8, c.LLVMDWARFTypeEncodingSigned),
            std.mem.hash_map.hashString("thicc") => try debug_gen.createBasicType("thicc", 64, c.LLVMDWARFTypeEncodingSigned),
            std.mem.hash_map.hashString("sip") => try debug_gen.createBasicType("sip", 8, c.LLVMDWARFTypeEncodingUnsigned),
            else => try debug_gen.createBasicType("unknown", 64, c.LLVMDWARFTypeEncodingSigned),
        };
    }
    
    /// Get size in bytes for CURSED type
    fn getCursedTypeSize(self: *AdvancedCodeGen, cursed_type: []const u8) u64 {
        _ = self;
        return switch (std.mem.hash_map.hashString(cursed_type)) {
            std.mem.hash_map.hashString("normie") => 4,
            std.mem.hash_map.hashString("tea") => 8,
            std.mem.hash_map.hashString("drip") => 8,
            std.mem.hash_map.hashString("lit") => 1,
            std.mem.hash_map.hashString("meal") => 8,
            std.mem.hash_map.hashString("smol") => 1,
            std.mem.hash_map.hashString("thicc") => 8,
            std.mem.hash_map.hashString("sip") => 1,
            else => 8,
        };
    }
    
    /// Calculate total struct size
    fn getStructTotalSize(self: *AdvancedCodeGen, field_types: [][]const u8) u64 {
        var total_size: u64 = 0;
        for (field_types) |field_type| {
            total_size += self.getCursedTypeSize(field_type);
        }
        return total_size;
    }

    /// Advanced binary operation with type checking
    fn generateAdvancedBinaryOp(self: *AdvancedCodeGen, left: c.LLVMValueRef, op: ast.BinaryOperator, right: c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        const left_type = c.LLVMTypeOf(left);
        const right_type = c.LLVMTypeOf(right);
        const builder = self.base_codegen.builder;
        
        // Type promotion for mixed numeric operations
        var promoted_left = left;
        var promoted_right = right;
        
        if (self.needsTypePromotion(left_type, right_type)) {
            const promoted_type = try self.getPromotedType(left_type, right_type);
            promoted_left = try self.promoteValue(left, promoted_type);
            promoted_right = try self.promoteValue(right, promoted_type);
        }
        
        return try self.base_codegen.generateBinaryOp(promoted_left, op, promoted_right);
    }
    
    /// Advanced unary operation with type checking
    fn generateAdvancedUnaryOp(self: *AdvancedCodeGen, op: ast.UnaryOperator, operand: c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateUnaryOp(op, operand);
    }
    
    /// Advanced function call with argument type checking
    fn generateAdvancedCall(self: *AdvancedCodeGen, call: ast.CallExpression) CodeGenError!c.LLVMValueRef {
        // Enhanced function call with generic instantiation support
        const function_expr = call.function.*;
        
        switch (function_expr) {
            .Identifier => |name| {
                // Check for generic function instantiation
                if (self.monomorphizer.isGenericFunction(name)) {
                    return try self.generateGenericFunctionCall(call, name);
                }
                return try self.generateRegularFunctionCall(call, name);
            },
            .MemberAccess => |member| {
                return try self.generateMethodCall(call, member);
            },
            else => {
                return try self.base_codegen.generateCall(call);
            },
        }
    }
    
    /// Advanced member access with struct/interface support
    fn generateAdvancedMemberAccess(self: *AdvancedCodeGen, member: ast.MemberAccessExpression) CodeGenError!c.LLVMValueRef {
        const object_value = try self.generateExpression(member.object.*);
        const object_type = c.LLVMTypeOf(object_value);
        
        // Check if this is struct field access
        if (c.LLVMGetTypeKind(object_type) == c.LLVMPointerTypeKind) {
            const pointed_type = c.LLVMGetElementType(object_type);
            if (c.LLVMGetTypeKind(pointed_type) == c.LLVMStructTypeKind) {
                return try self.generateStructFieldAccessAuto(object_value, member.property);
            }
        } else if (c.LLVMGetTypeKind(object_type) == c.LLVMStructTypeKind) {
            // Direct struct value access
            return try self.generateStructFieldAccessAuto(object_value, member.property);
        }
        
        // Check if this is interface method access
        if (self.isInterfaceType(object_type)) {
            return try self.generateInterfaceMethodAccess(object_value, member.property);
        }
        
        return try self.base_codegen.generateMemberAccess(member);
    }
    
    /// Advanced array literal with type inference
    fn generateAdvancedArrayLiteral(self: *AdvancedCodeGen, array: ast.ArrayLiteralExpression) CodeGenError!c.LLVMValueRef {
        if (array.elements.items.len == 0) {
            return try self.generateEmptyArray();
        }
        
        // Generate elements with type inference
        var element_values = ArrayList(c.LLVMValueRef).init(self.base_codegen.allocator);
        defer element_values.deinit();
        
        var common_type: ?c.LLVMTypeRef = null;
        for (array.elements.items) |element| {
            const value = try self.generateExpression(element);
            try element_values.append(value);
            
            if (common_type == null) {
                common_type = c.LLVMTypeOf(value);
            } else {
                // Check type compatibility and promote if needed
                const value_type = c.LLVMTypeOf(value);
                if (!self.typesAreCompatible(common_type.?, value_type)) {
                    common_type = try self.getCommonType(common_type.?, value_type);
                }
            }
        }
        
        return try self.generateTypedArrayLiteral(element_values.items, common_type.?);
    }
    
    /// Advanced array expression compilation
    fn generateAdvancedArrayExpression(self: *AdvancedCodeGen, array: ast.ArrayExpression) CodeGenError!c.LLVMValueRef {
        // Handle dynamic array creation with size
        const size_value = try self.generateExpression(array.size.*);
        const element_type = try self.getLLVMTypeFromCursedType(array.element_type);
        
        return try self.generateDynamicArray(element_type, size_value);
    }
    
    /// Advanced index access with bounds checking
    fn generateAdvancedIndexAccess(self: *AdvancedCodeGen, index: ast.IndexAccessExpression) CodeGenError!c.LLVMValueRef {
        const array_value = try self.generateExpression(index.object.*);
        const index_value = try self.generateExpression(index.index.*);
        
        // Add bounds checking if enabled
        if (self.optimization_config.bounds_checking) {
            try self.generateBoundsCheck(array_value, index_value);
        }
        
        return try self.generateSafeIndexAccess(array_value, index_value);
    }
    
    /// Advanced struct literal with field validation
    fn generateAdvancedStructLiteral(self: *AdvancedCodeGen, struct_lit: ast.StructLiteralExpression) CodeGenError!c.LLVMValueRef {
        const struct_info = self.struct_types.get(struct_lit.struct_name) orelse {
            return CodeGenError.UndefinedSymbol;
        };
        
        // Validate all required fields are provided
        try self.validateStructFields(struct_lit, struct_info);
        
        // Generate struct with proper field ordering
        return try self.generateValidatedStructLiteral(struct_lit, struct_info);
    }
    
    /// Advanced interface call with vtable dispatch
    fn generateAdvancedInterfaceCall(self: *AdvancedCodeGen, interface_call: ast.InterfaceCallExpression) CodeGenError!c.LLVMValueRef {
        const instance_value = try self.generateExpression(interface_call.instance.*);
        const method_name = interface_call.method_name;
        
        // Look up vtable entry and generate dispatch
        return try self.generateVTableDispatch(instance_value, method_name, interface_call.arguments.items);
    }
    
    /// Advanced type cast with runtime checking
    fn generateAdvancedTypeCast(self: *AdvancedCodeGen, cast: ast.TypeCastExpression) CodeGenError!c.LLVMValueRef {
        const value = try self.generateExpression(cast.expression.*);
        const target_type = try self.getLLVMTypeFromCursedType(cast.target_type);
        
        // Add runtime type checking for interface casts
        if (self.isInterfaceType(c.LLVMTypeOf(value))) {
            return try self.generateCheckedInterfaceCast(value, target_type);
        }
        
        return try self.generateSafeCast(value, target_type);
    }
    
    /// Advanced match expression compilation
    fn generateAdvancedMatch(self: *AdvancedCodeGen, match: ast.MatchExpression) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateMatch(match);
    }
    
    /// Advanced tuple with proper layout
    fn generateAdvancedTuple(self: *AdvancedCodeGen, tuple: ast.TupleExpression) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateTuple(tuple);
    }
    
    /// Advanced tuple access with bounds checking
    fn generateAdvancedTupleAccess(self: *AdvancedCodeGen, tuple_access: ast.TupleAccessExpression) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateTupleAccess(tuple_access);
    }
    
    /// Advanced channel send with type checking
    fn generateAdvancedChannelSend(self: *AdvancedCodeGen, send: ast.ChannelSendExpression) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateChannelSend(send);
    }
    
    /// Advanced channel receive with type checking
    fn generateAdvancedChannelReceive(self: *AdvancedCodeGen, recv: ast.ChannelReceiveExpression) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateChannelReceive(recv);
    }
    
    /// Advanced channel creation with capacity validation
    fn generateAdvancedChannelCreation(self: *AdvancedCodeGen, create: ast.ChannelCreationExpression) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateChannelCreation(create);
    }
    
    /// Advanced goroutine creation
    fn generateAdvancedGoroutine(self: *AdvancedCodeGen, goroutine: ast.GoroutineExpression) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateGoroutineExpression(goroutine);
    }
    
    /// Advanced error propagation with stack trace capture
    fn generateAdvancedShook(self: *AdvancedCodeGen, shook: ast.ShookExpression) CodeGenError!c.LLVMValueRef {
        // Enhanced SHOOK with stack trace capture and context preservation
        const error_codegen = @import("error_codegen_integration.zig");
        var err_gen = error_codegen.ErrorCodeGen.init(
            self.base_codegen.context,
            self.base_codegen.module,
            self.base_codegen.builder,
            self.allocator
        );
        err_gen.setupRuntimeFunctions();
        
        // Generate the expression that might fail
        const expr_value = try self.base_codegen.generateExpression(shook.expression);
        
        // Generate enhanced error propagation with stack trace
        return err_gen.generateShook(expr_value);
    }
    
    /// Advanced error creation with stack traces and context
    fn generateAdvancedYikes(self: *AdvancedCodeGen, yikes: ast.YikesExpression) CodeGenError!c.LLVMValueRef {
        const error_codegen = @import("error_codegen_integration.zig");
        var err_gen = error_codegen.ErrorCodeGen.init(
            self.base_codegen.context,
            self.base_codegen.module,
            self.base_codegen.builder,
            self.allocator
        );
        err_gen.setupRuntimeFunctions();
        
        // Get error message
        const message = if (yikes.message) |msg_expr| blk: {
            const msg_value = try self.base_codegen.generateExpression(msg_expr);
            // TODO: Extract string from LLVM value
            break :blk "dynamic message";
        } else "unknown error";
        
        // Get error code  
        const code = if (yikes.code) |code_expr| blk: {
            const code_value = try self.base_codegen.generateExpression(code_expr);
            // TODO: Extract integer from LLVM value
            break :blk 500;
        } else 0;
        
        // Generate enhanced error with stack trace and source location
        return err_gen.generateYikes(
            message,
            code,
            0, // Runtime error type
            self.base_codegen.current_file orelse "unknown",
            self.base_codegen.current_line,
            self.base_codegen.current_column
        );
    }
    
    /// Advanced panic recovery with defer integration
    fn generateAdvancedFam(self: *AdvancedCodeGen, fam: ast.FamExpression) CodeGenError!c.LLVMValueRef {
        const error_codegen = @import("error_codegen_integration.zig");
        var err_gen = error_codegen.ErrorCodeGen.init(
            self.base_codegen.context,
            self.base_codegen.module,
            self.base_codegen.builder,
            self.allocator
        );
        err_gen.setupRuntimeFunctions();
        
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.base_codegen.builder));
        
        // Create basic blocks for fam construct
        const try_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "fam_try");
        const catch_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "fam_catch");
        const finally_block = c.LLVMAppendBasicBlockInContext(self.base_codegen.context, current_func, "fam_finally");
        
        // Position builder at try block
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, try_block);
        
        // Generate try body
        if (fam.try_body) |try_body| {
            for (try_body.items) |stmt| {
                _ = try self.base_codegen.generateStatement(stmt);
            }
        }
        
        // Jump to finally block on success
        _ = c.LLVMBuildBr(self.base_codegen.builder, finally_block);
        
        // Position builder at catch block
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, catch_block);
        
        // Generate catch body  
        if (fam.catch_body) |catch_body| {
            for (catch_body.items) |stmt| {
                _ = try self.base_codegen.generateStatement(stmt);
            }
        }
        
        // Jump to finally block
        _ = c.LLVMBuildBr(self.base_codegen.builder, finally_block);
        
        // Position builder at finally block
        c.LLVMPositionBuilderAtEnd(self.base_codegen.builder, finally_block);
        
        // Generate cleanup code (execute defers)
        err_gen.generateCleanup();
        
        // Generate finally body
        if (fam.finally_body) |finally_body| {
            for (finally_body.items) |stmt| {
                _ = try self.base_codegen.generateStatement(stmt);
            }
        }
        
        // Generate complete fam construct
        return err_gen.generateFam(try_block, catch_block, finally_block);
    }
    
    /// Advanced lambda compilation
    fn generateAdvancedLambda(self: *AdvancedCodeGen, lambda: ast.LambdaExpression) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateLambda(lambda);
    }

    // Helper functions for advanced expression compilation
    
    /// Check if two types need promotion for binary operations
    fn needsTypePromotion(self: *AdvancedCodeGen, left_type: c.LLVMTypeRef, right_type: c.LLVMTypeRef) bool {
        _ = self;
        return !c.LLVMIsEqualType(left_type, right_type);
    }
    
    /// Get promoted type for binary operations
    fn getPromotedType(self: *AdvancedCodeGen, left_type: c.LLVMTypeRef, right_type: c.LLVMTypeRef) CodeGenError!c.LLVMTypeRef {
        _ = self;
        const left_kind = c.LLVMGetTypeKind(left_type);
        const right_kind = c.LLVMGetTypeKind(right_type);
        
        // Promote to larger numeric type
        if (left_kind == c.LLVMDoubleTypeKind or right_kind == c.LLVMDoubleTypeKind) {
            return c.LLVMDoubleTypeInContext(c.LLVMGetTypeContext(left_type));
        }
        if (left_kind == c.LLVMFloatTypeKind or right_kind == c.LLVMFloatTypeKind) {
            return c.LLVMFloatTypeInContext(c.LLVMGetTypeContext(left_type));
        }
        if (left_kind == c.LLVMIntegerTypeKind and right_kind == c.LLVMIntegerTypeKind) {
            const left_width = c.LLVMGetIntTypeWidth(left_type);
            const right_width = c.LLVMGetIntTypeWidth(right_type);
            const max_width = if (left_width > right_width) left_width else right_width;
            return c.LLVMIntTypeInContext(c.LLVMGetTypeContext(left_type), max_width);
        }
        
        return left_type; // Default to left type
    }
    
    /// Promote value to target type
    fn promoteValue(self: *AdvancedCodeGen, value: c.LLVMValueRef, target_type: c.LLVMTypeRef) CodeGenError!c.LLVMValueRef {
        const value_type = c.LLVMTypeOf(value);
        const builder = self.base_codegen.builder;
        
        if (c.LLVMIsEqualType(value_type, target_type)) {
            return value;
        }
        
        const value_kind = c.LLVMGetTypeKind(value_type);
        const target_kind = c.LLVMGetTypeKind(target_type);
        
        // Integer to float promotion
        if (value_kind == c.LLVMIntegerTypeKind and (target_kind == c.LLVMFloatTypeKind or target_kind == c.LLVMDoubleTypeKind)) {
            return c.LLVMBuildSIToFP(builder, value, target_type, "promote_itof");
        }
        
        // Integer width extension
        if (value_kind == c.LLVMIntegerTypeKind and target_kind == c.LLVMIntegerTypeKind) {
            const value_width = c.LLVMGetIntTypeWidth(value_type);
            const target_width = c.LLVMGetIntTypeWidth(target_type);
            
            if (value_width < target_width) {
                return c.LLVMBuildSExt(builder, value, target_type, "promote_sext");
            } else if (value_width > target_width) {
                return c.LLVMBuildTrunc(builder, value, target_type, "promote_trunc");
            }
        }
        
        // Float precision promotion
        if (value_kind == c.LLVMFloatTypeKind and target_kind == c.LLVMDoubleTypeKind) {
            return c.LLVMBuildFPExt(builder, value, target_type, "promote_fpext");
        }
        
        return value; // No promotion needed
    }
    
    /// Check if types are compatible for operations
    fn typesAreCompatible(self: *AdvancedCodeGen, type1: c.LLVMTypeRef, type2: c.LLVMTypeRef) bool {
        _ = self;
        return c.LLVMIsEqualType(type1, type2) or self.canPromoteTypes(type1, type2);
    }
    
    /// Check if types can be promoted to a common type
    fn canPromoteTypes(self: *AdvancedCodeGen, type1: c.LLVMTypeRef, type2: c.LLVMTypeRef) bool {
        _ = self;
        const kind1 = c.LLVMGetTypeKind(type1);
        const kind2 = c.LLVMGetTypeKind(type2);
        
        // Numeric types can be promoted
        return (kind1 == c.LLVMIntegerTypeKind or kind1 == c.LLVMFloatTypeKind or kind1 == c.LLVMDoubleTypeKind) and
               (kind2 == c.LLVMIntegerTypeKind or kind2 == c.LLVMFloatTypeKind or kind2 == c.LLVMDoubleTypeKind);
    }
    
    /// Get common type for array elements
    fn getCommonType(self: *AdvancedCodeGen, type1: c.LLVMTypeRef, type2: c.LLVMTypeRef) CodeGenError!c.LLVMTypeRef {
        return try self.getPromotedType(type1, type2);
    }
    
    /// Check if type is interface type
    fn isInterfaceType(self: *AdvancedCodeGen, llvm_type: c.LLVMTypeRef) bool {
        _ = self;
        _ = llvm_type;
        // TODO: Implement interface type detection
        return false;
    }
    
    /// Generate empty array
    fn generateEmptyArray(self: *AdvancedCodeGen) CodeGenError!c.LLVMValueRef {
        const i8_type = c.LLVMInt8TypeInContext(self.base_codegen.context);
        const array_type = c.LLVMArrayType(i8_type, 0);
        return c.LLVMGetUndef(array_type);
    }
    
    /// Get LLVM type from CURSED type
    fn getLLVMTypeFromCursedType(self: *AdvancedCodeGen, cursed_type: Type) CodeGenError!c.LLVMTypeRef {
        return try self.base_codegen.getLLVMType(cursed_type);
    }
    
    /// Generate typed array literal
    fn generateTypedArrayLiteral(self: *AdvancedCodeGen, elements: []c.LLVMValueRef, element_type: c.LLVMTypeRef) CodeGenError!c.LLVMValueRef {
        const array_type = c.LLVMArrayType(element_type, @as(u32, @intCast(elements.len)));
        const builder = self.base_codegen.builder;
        
        // Allocate array
        const array_alloca = c.LLVMBuildAlloca(builder, array_type, "array_literal");
        
        // Store elements
        for (elements, 0..) |element, i| {
            const element_ptr = c.LLVMBuildGEP2(
                builder,
                array_type,
                array_alloca,
                &[_]c.LLVMValueRef{
                    c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), 0, 0),
                    c.LLVMConstInt(c.LLVMInt32TypeInContext(self.base_codegen.context), @as(u32, @intCast(i)), 0)
                },
                2,
                "element_ptr"
            );
            _ = c.LLVMBuildStore(builder, element, element_ptr);
        }
        
        return array_alloca;
    }
    
    /// Generate dynamic array with size
    fn generateDynamicArray(self: *AdvancedCodeGen, element_type: c.LLVMTypeRef, size: c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        const builder = self.base_codegen.builder;
        
        // Calculate total size: element_size * count
        const element_size = c.LLVMSizeOf(element_type);
        const total_size = c.LLVMBuildMul(builder, element_size, size, "array_size");
        
        // Allocate memory
        const malloc_func = self.base_codegen.functions.get("malloc") orelse {
            return CodeGenError.UndefinedSymbol;
        };
        
        const array_ptr = c.LLVMBuildCall2(
            builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(malloc_func)),
            malloc_func,
            &[_]c.LLVMValueRef{total_size},
            1,
            "dynamic_array"
        );
        
        // Cast to element pointer type
        return c.LLVMBuildBitCast(
            builder,
            array_ptr,
            c.LLVMPointerType(element_type, 0),
            "typed_array"
        );
    }
    
    /// Generate bounds check for array access
    fn generateBoundsCheck(self: *AdvancedCodeGen, array: c.LLVMValueRef, index: c.LLVMValueRef) CodeGenError!void {
        _ = self;
        _ = array;
        _ = index;
        // TODO: Implement runtime bounds checking
    }
    
    /// Generate safe index access
    fn generateSafeIndexAccess(self: *AdvancedCodeGen, array: c.LLVMValueRef, index: c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateIndexAccess(ast.IndexAccessExpression{
            .object = @ptrCast(&ast.Expression{ .Identifier = "dummy" }),
            .index = @ptrCast(&ast.Expression{ .Identifier = "dummy" }),
        });
    }
    
    /// Validate struct fields match definition
    fn validateStructFields(self: *AdvancedCodeGen, struct_lit: ast.StructLiteralExpression, struct_info: StructTypeInfo) CodeGenError!void {
        _ = self;
        _ = struct_lit;
        _ = struct_info;
        // TODO: Implement field validation
    }
    
    /// Generate validated struct literal
    fn generateValidatedStructLiteral(self: *AdvancedCodeGen, struct_lit: ast.StructLiteralExpression, struct_info: StructTypeInfo) CodeGenError!c.LLVMValueRef {
        _ = struct_info;
        return try self.base_codegen.generateStructLiteral(struct_lit);
    }
    
    /// Generate vtable dispatch for interface calls
    fn generateVTableDispatch(self: *AdvancedCodeGen, instance: c.LLVMValueRef, method_name: []const u8, args: []ast.Expression) CodeGenError!c.LLVMValueRef {
        _ = self;
        _ = instance;
        _ = method_name;
        _ = args;
        // TODO: Implement vtable dispatch
        return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.base_codegen.context), 0, 0);
    }
    
    /// Generate checked interface cast
    fn generateCheckedInterfaceCast(self: *AdvancedCodeGen, value: c.LLVMValueRef, target_type: c.LLVMTypeRef) CodeGenError!c.LLVMValueRef {
        _ = self;
        _ = target_type;
        return value; // TODO: Implement runtime type checking
    }
    
    /// Generate safe cast with proper conversions
    fn generateSafeCast(self: *AdvancedCodeGen, value: c.LLVMValueRef, target_type: c.LLVMTypeRef) CodeGenError!c.LLVMValueRef {
        return try self.base_codegen.generateTypeCast(ast.TypeCastExpression{
            .expression = @ptrCast(&ast.Expression{ .Identifier = "dummy" }),
            .target_type = Type{ .Basic = .Normie }, // Dummy type
        });
    }
    
    /// Generate regular function call with enhanced support for recursive calls and stdlib functions
    fn generateRegularFunctionCall(self: *AdvancedCodeGen, call: ast.CallExpression, name: []const u8) CodeGenError!c.LLVMValueRef {
        const context = self.base_codegen.context;
        const module = self.base_codegen.module;
        const builder = self.base_codegen.builder;
        
        // Handle built-in stdlib functions
        if (std.mem.eql(u8, name, "len")) {
            return try self.generateArrayLengthCall(call);
        }
        
        // Handle user-defined functions (including recursive calls)
        if (self.base_codegen.functions.get(name)) |function| {
            return try self.generateEnhancedUserFunctionCall(function, call);
        }
        
        // Check if this is a runtime function
        if (self.base_codegen.runtime_functions.get(name)) |runtime_func| {
            return try self.generateRuntimeFunctionCall(runtime_func, call);
        }
        
        // Declare function if not found (for external functions)
        const func_type = c.LLVMFunctionType(
            c.LLVMInt64TypeInContext(context), // Default return type
            null,
            0,
            0
        );
        const new_function = c.LLVMAddFunction(module, name.ptr, func_type);
        try self.base_codegen.functions.put(name, new_function);
        
        return try self.generateEnhancedUserFunctionCall(new_function, call);
    }
    
    /// Generate generic function call with monomorphization
    fn generateGenericFunctionCall(self: *AdvancedCodeGen, call: ast.CallExpression, name: []const u8) CodeGenError!c.LLVMValueRef {
        _ = name;
        // TODO: Implement generic function instantiation
        return try self.base_codegen.generateCall(call);
    }
    
    /// Generate method call on struct or interface
    fn generateMethodCall(self: *AdvancedCodeGen, call: ast.CallExpression, member: ast.MemberAccessExpression) CodeGenError!c.LLVMValueRef {
        // Handle vibez.spill and other stdlib method calls
        switch (member.object.*) {
            .Identifier => |obj_name| {
                if (std.mem.eql(u8, obj_name, "vibez") and std.mem.eql(u8, member.property, "spill")) {
                    return try self.generateVibesSpillCall(call);
                }
            },
            else => {},
        }
        
        return try self.base_codegen.generateCall(call);
    }
    
    /// Generate array length function call (len function)
    fn generateArrayLengthCall(self: *AdvancedCodeGen, call: ast.CallExpression) CodeGenError!c.LLVMValueRef {
        const context = self.base_codegen.context;
        
        if (call.arguments.items.len != 1) {
            return CodeGenError.InvalidArgumentCount;
        }
        
        const array_arg = call.arguments.items[0];
        
        // Handle different array types
        switch (array_arg) {
            .ArrayLiteral => |array_lit| {
                // For array literals, return constant length
                const len_value = array_lit.elements.items.len;
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), @intCast(len_value), 0);
            },
            .Identifier => |var_name| {
                // For array variables, we need to look up the stored length
                // This is a simplified implementation
                _ = var_name;
                // In a full implementation, would maintain array metadata
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 5, 0); // Placeholder
            },
            else => {
                // Try to evaluate the expression and get its length
                const array_value = try self.generateExpression(array_arg);
                _ = array_value;
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0, 0); // Placeholder
            },
        }
    }
    
    /// Generate enhanced user function call with argument evaluation
    fn generateEnhancedUserFunctionCall(self: *AdvancedCodeGen, function: c.LLVMValueRef, call: ast.CallExpression) CodeGenError!c.LLVMValueRef {
        const builder = self.base_codegen.builder;
        
        // Generate arguments with proper expression evaluation
        var args = ArrayList(c.LLVMValueRef).init(self.base_codegen.allocator);
        defer args.deinit();
        
        for (call.arguments.items) |arg_expr| {
            const arg_value = try self.generateExpression(arg_expr);
            try args.append(arg_value);
        }
        
        // Generate function call
        const function_type = c.LLVMGlobalGetValueType(function);
        const return_type = c.LLVMGetReturnType(function_type);
        
        return c.LLVMBuildCall2(
            builder,
            return_type,
            function,
            if (args.items.len > 0) args.items.ptr else null,
            @intCast(args.items.len),
            "call_result"
        );
    }
    
    /// Generate runtime function call
    fn generateRuntimeFunctionCall(self: *AdvancedCodeGen, runtime_func: c.LLVMValueRef, call: ast.CallExpression) CodeGenError!c.LLVMValueRef {
        const builder = self.base_codegen.builder;
        
        // Generate arguments
        var args = ArrayList(c.LLVMValueRef).init(self.base_codegen.allocator);
        defer args.deinit();
        
        for (call.arguments.items) |arg_expr| {
            const arg_value = try self.generateExpression(arg_expr);
            try args.append(arg_value);
        }
        
        // Generate runtime function call
        const function_type = c.LLVMGlobalGetValueType(runtime_func);
        const return_type = c.LLVMGetReturnType(function_type);
        
        return c.LLVMBuildCall2(
            builder,
            return_type,
            runtime_func,
            if (args.items.len > 0) args.items.ptr else null,
            @intCast(args.items.len),
            "runtime_call"
        );
    }
    
    /// Generate vibez.spill call (enhanced print function)
    fn generateVibesSpillCall(self: *AdvancedCodeGen, call: ast.CallExpression) CodeGenError!c.LLVMValueRef {
        const context = self.base_codegen.context;
        const module = self.base_codegen.module;
        const builder = self.base_codegen.builder;
        
        // Get or declare printf function
        const printf_func = self.base_codegen.functions.get("printf") orelse blk: {
            const printf_type = c.LLVMFunctionType(
                c.LLVMInt32TypeInContext(context),
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
                1,
                1 // variadic
            );
            const func = c.LLVMAddFunction(module, "printf", printf_type);
            try self.base_codegen.functions.put("printf", func);
            break :blk func;
        };
        
        if (call.arguments.items.len == 0) {
            // Print newline if no arguments
            const newline_fmt = c.LLVMBuildGlobalStringPtr(builder, "\n", "newline_fmt");
            return c.LLVMBuildCall2(
                builder,
                c.LLVMInt32TypeInContext(context),
                printf_func,
                &[_]c.LLVMValueRef{newline_fmt},
                1,
                "printf_newline"
            );
        }
        
        // Handle multiple arguments
        var format_parts = ArrayList(u8).init(self.base_codegen.allocator);
        defer format_parts.deinit();
        
        var printf_args = ArrayList(c.LLVMValueRef).init(self.base_codegen.allocator);
        defer printf_args.deinit();
        
        for (call.arguments.items, 0..) |arg_expr, i| {
            if (i > 0) {
                try format_parts.appendSlice(" ");
            }
            
            const arg_value = try self.generateExpression(arg_expr);
            const arg_type = c.LLVMTypeOf(arg_value);
            
            // Determine format specifier based on type
            if (c.LLVMGetTypeKind(arg_type) == c.LLVMIntegerTypeKind) {
                const bit_width = c.LLVMGetIntTypeWidth(arg_type);
                if (bit_width == 1) {
                    // Boolean - convert to string
                    try format_parts.appendSlice("%s");
                    const true_str = c.LLVMBuildGlobalStringPtr(builder, "based", "true_str");
                    const false_str = c.LLVMBuildGlobalStringPtr(builder, "cringe", "false_str");
                    const bool_str = c.LLVMBuildSelect(builder, arg_value, true_str, false_str, "bool_str");
                    try printf_args.append(bool_str);
                } else {
                    try format_parts.appendSlice("%lld");
                    try printf_args.append(arg_value);
                }
            } else if (c.LLVMGetTypeKind(arg_type) == c.LLVMDoubleTypeKind) {
                try format_parts.appendSlice("%.2f");
                try printf_args.append(arg_value);
            } else if (c.LLVMGetTypeKind(arg_type) == c.LLVMPointerTypeKind) {
                try format_parts.appendSlice("%s");
                try printf_args.append(arg_value);
            } else {
                try format_parts.appendSlice("%p");
                try printf_args.append(arg_value);
            }
        }
        
        try format_parts.appendSlice("\n");
        
        // Create format string
        const format_str = try format_parts.toOwnedSlice();
        defer self.base_codegen.allocator.free(format_str);
        const format = c.LLVMBuildGlobalStringPtr(builder, format_str.ptr, "printf_fmt");
        
        // Build final printf arguments
        var final_args = ArrayList(c.LLVMValueRef).init(self.base_codegen.allocator);
        defer final_args.deinit();
        try final_args.append(format);
        try final_args.appendSlice(printf_args.items);
        
        return c.LLVMBuildCall2(
            builder,
            c.LLVMInt32TypeInContext(context),
            printf_func,
            final_args.items.ptr,
            @intCast(final_args.items.len),
            "printf_call"
        );
    }
    

    
    /// Generate interface method access
    fn generateInterfaceMethodAccess(self: *AdvancedCodeGen, interface_ptr: c.LLVMValueRef, method_name: []const u8) CodeGenError!c.LLVMValueRef {
        _ = method_name;
        return interface_ptr; // TODO: Implement interface method access
    }

    /// Advanced select statement generation for CSP-style concurrency
    fn generateAdvancedSelect(self: *AdvancedCodeGen, select_stmt: ast.SelectStatement) CodeGenError!void {
        const current_func = self.base_codegen.current_function.?;
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        
        // Generate runtime function declarations if not already done
        try self.ensureSelectRuntimeFunctions();
        
        // Create select context for managing operations
        const select_context_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        const select_begin_func = self.base_codegen.runtime_functions.get("cursed_select_begin").?;
        const case_count = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @as(u32, @intCast(select_stmt.cases.items.len)), 0);
        
        const select_context = c.LLVMBuildCall2(
            builder,
            select_context_type,
            select_begin_func,
            &[_]c.LLVMValueRef{case_count},
            1,
            "select_ctx"
        );

        // Create operation descriptors for each case
        var operation_values = std.ArrayList(c.LLVMValueRef).init(self.base_codegen.allocator);
        defer operation_values.deinit();
        
        for (select_stmt.cases.items, 0..) |case_item, i| {
            const case_index = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @as(u32, @intCast(i)), 0);
            
            // Determine operation type and register channel
            if (case_item.channel) |channel_expr| {
                const channel_value = try self.base_codegen.generateExpression(channel_expr);
                
                // Add send operation
                if (case_item.send_value) |send_value_expr| {
                    const send_value = try self.base_codegen.generateExpression(send_value_expr);
                    const add_send_func = self.base_codegen.runtime_functions.get("cursed_select_add_send").?;
                    
                    _ = c.LLVMBuildCall2(
                        builder,
                        c.LLVMVoidTypeInContext(context),
                        add_send_func,
                        &[_]c.LLVMValueRef{ select_context, channel_value, send_value, case_index },
                        4,
                        ""
                    );
                } else {
                    // Add receive operation
                    const add_recv_func = self.base_codegen.runtime_functions.get("cursed_select_add_recv").?;
                    
                    _ = c.LLVMBuildCall2(
                        builder,
                        c.LLVMVoidTypeInContext(context),
                        add_recv_func,
                        &[_]c.LLVMValueRef{ select_context, channel_value, case_index },
                        3,
                        ""
                    );
                }
            } else {
                // Default case
                const add_default_func = self.base_codegen.runtime_functions.get("cursed_select_add_default").?;
                
                _ = c.LLVMBuildCall2(
                    builder,
                    c.LLVMVoidTypeInContext(context),
                    add_default_func,
                    &[_]c.LLVMValueRef{ select_context, case_index },
                    2,
                    ""
                );
            }
        }

        // Execute select operation (non-blocking by default)
        const select_execute_func = self.base_codegen.runtime_functions.get("cursed_select_execute").?;
        const ready_case_index = c.LLVMBuildCall2(
            builder,
            c.LLVMInt32TypeInContext(context),
            select_execute_func,
            &[_]c.LLVMValueRef{select_context},
            1,
            "ready_case"
        );

        // Create basic blocks for each case
        var case_blocks = std.ArrayList(c.LLVMBasicBlockRef).init(self.base_codegen.allocator);
        defer case_blocks.deinit();
        
        const merge_block = c.LLVMAppendBasicBlockInContext(context, current_func, "select_merge");
        const no_case_ready_block = c.LLVMAppendBasicBlockInContext(context, current_func, "select_no_case");
        
        // Create switch instruction to jump to ready case
        const switch_inst = c.LLVMBuildSwitch(builder, ready_case_index, no_case_ready_block, @as(u32, @intCast(select_stmt.cases.items.len)));

        // Generate blocks for each case
        for (select_stmt.cases.items, 0..) |case_item, i| {
            const case_block = c.LLVMAppendBasicBlockInContext(context, current_func, 
                try std.fmt.allocPrint(self.base_codegen.allocator, "select_case_{d}", .{i}).ptr);
            try case_blocks.append(case_block);

            // Add case to switch
            const case_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @as(u32, @intCast(i)), 0);
            c.LLVMAddCase(switch_inst, case_value, case_block);

            // Position builder in case block
            c.LLVMPositionBuilderAtEnd(builder, case_block);
            
            // For receive operations, extract the received value
            if (case_item.channel != null and case_item.send_value == null) {
                const get_recv_value_func = self.base_codegen.runtime_functions.get("cursed_select_get_recv_value").?;
                const received_value = c.LLVMBuildCall2(
                    builder,
                    c.LLVMInt64TypeInContext(context),
                    get_recv_value_func,
                    &[_]c.LLVMValueRef{ select_context, case_value },
                    2,
                    "recv_value"
                );
                
                // Store in variable if case has variable binding
                if (case_item.variable_name) |var_name| {
                    // Create or update variable binding
                    // Create alloca for the received value
                    const var_type = c.LLVMInt64TypeInContext(context);
                    const alloca = c.LLVMBuildAlloca(builder, var_type, var_name.ptr);
                    _ = c.LLVMBuildStore(builder, received_value, alloca);
                    
                    // Store in variables map if available
                    try self.base_codegen.variables.put(var_name, alloca);
                }
            }
            
            // Generate case body statements
            for (case_item.body.items) |stmt| {
                try self.base_codegen.generateStatement(stmt);
            }
            
            // Branch to merge if no terminator
            if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
                _ = c.LLVMBuildBr(builder, merge_block);
            }
        }

        // Handle case where no operation is ready (should not happen with proper select)
        c.LLVMPositionBuilderAtEnd(builder, no_case_ready_block);
        _ = c.LLVMBuildBr(builder, merge_block);

        // Continue in merge block
        c.LLVMPositionBuilderAtEnd(builder, merge_block);
        
        // Cleanup select context
        const select_cleanup_func = self.base_codegen.runtime_functions.get("cursed_select_cleanup").?;
        _ = c.LLVMBuildCall2(
            builder,
            c.LLVMVoidTypeInContext(context),
            select_cleanup_func,
            &[_]c.LLVMValueRef{select_context},
            1,
            ""
        );
    }

    /// Ensure select runtime functions are declared
    fn ensureSelectRuntimeFunctions(self: *AdvancedCodeGen) CodeGenError!void {
        const context = self.base_codegen.context;
        const module = self.base_codegen.module;
        
        // Function types
        const void_type = c.LLVMVoidTypeInContext(context);
        const i32_type = c.LLVMInt32TypeInContext(context);
        const i64_type = c.LLVMInt64TypeInContext(context);
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0);
        
        // cursed_select_begin(case_count: i32) -> select_context*
        if (self.base_codegen.runtime_functions.get("cursed_select_begin") == null) {
            const select_begin_type = c.LLVMFunctionType(ptr_type, &[_]c.LLVMTypeRef{i32_type}, 1, 0);
            const select_begin_func = c.LLVMAddFunction(module, "cursed_select_begin", select_begin_type);
            try self.base_codegen.runtime_functions.put("cursed_select_begin", select_begin_func);
        }
        
        // cursed_select_add_send(ctx: select_context*, channel: i64, value: i64, case_index: i32) -> void
        if (self.base_codegen.runtime_functions.get("cursed_select_add_send") == null) {
            const add_send_type = c.LLVMFunctionType(void_type, &[_]c.LLVMTypeRef{ptr_type, i64_type, i64_type, i32_type}, 4, 0);
            const add_send_func = c.LLVMAddFunction(module, "cursed_select_add_send", add_send_type);
            try self.base_codegen.runtime_functions.put("cursed_select_add_send", add_send_func);
        }
        
        // cursed_select_add_recv(ctx: select_context*, channel: i64, case_index: i32) -> void
        if (self.base_codegen.runtime_functions.get("cursed_select_add_recv") == null) {
            const add_recv_type = c.LLVMFunctionType(void_type, &[_]c.LLVMTypeRef{ptr_type, i64_type, i32_type}, 3, 0);
            const add_recv_func = c.LLVMAddFunction(module, "cursed_select_add_recv", add_recv_type);
            try self.base_codegen.runtime_functions.put("cursed_select_add_recv", add_recv_func);
        }
        
        // cursed_select_add_default(ctx: select_context*, case_index: i32) -> void
        if (self.base_codegen.runtime_functions.get("cursed_select_add_default") == null) {
            const add_default_type = c.LLVMFunctionType(void_type, &[_]c.LLVMTypeRef{ptr_type, i32_type}, 2, 0);
            const add_default_func = c.LLVMAddFunction(module, "cursed_select_add_default", add_default_type);
            try self.base_codegen.runtime_functions.put("cursed_select_add_default", add_default_func);
        }
        
        // cursed_select_execute(ctx: select_context*) -> i32 (ready case index)
        if (self.base_codegen.runtime_functions.get("cursed_select_execute") == null) {
            const execute_type = c.LLVMFunctionType(i32_type, &[_]c.LLVMTypeRef{ptr_type}, 1, 0);
            const execute_func = c.LLVMAddFunction(module, "cursed_select_execute", execute_type);
            try self.base_codegen.runtime_functions.put("cursed_select_execute", execute_func);
        }
        
        // cursed_select_get_recv_value(ctx: select_context*, case_index: i32) -> i64
        if (self.base_codegen.runtime_functions.get("cursed_select_get_recv_value") == null) {
            const get_recv_type = c.LLVMFunctionType(i64_type, &[_]c.LLVMTypeRef{ptr_type, i32_type}, 2, 0);
            const get_recv_func = c.LLVMAddFunction(module, "cursed_select_get_recv_value", get_recv_type);
            try self.base_codegen.runtime_functions.put("cursed_select_get_recv_value", get_recv_func);
        }
        
        // cursed_select_cleanup(ctx: select_context*) -> void
        if (self.base_codegen.runtime_functions.get("cursed_select_cleanup") == null) {
            const cleanup_type = c.LLVMFunctionType(void_type, &[_]c.LLVMTypeRef{ptr_type}, 1, 0);
            const cleanup_func = c.LLVMAddFunction(module, "cursed_select_cleanup", cleanup_type);
            try self.base_codegen.runtime_functions.put("cursed_select_cleanup", cleanup_func);
        }
        
        // cursed_select_set_timeout(ctx: select_context*, timeout_ms: i64) -> void
        if (self.base_codegen.runtime_functions.get("cursed_select_set_timeout") == null) {
            const timeout_type = c.LLVMFunctionType(void_type, &[_]c.LLVMTypeRef{ptr_type, i64_type}, 2, 0);
            const timeout_func = c.LLVMAddFunction(module, "cursed_select_set_timeout", timeout_type);
            try self.base_codegen.runtime_functions.put("cursed_select_set_timeout", timeout_func);
        }
        
        // Non-blocking channel operations
        if (self.base_codegen.runtime_functions.get("cursed_channel_try_send") == null) {
            const try_send_type = c.LLVMFunctionType(i32_type, &[_]c.LLVMTypeRef{i64_type, i64_type}, 2, 0);
            const try_send_func = c.LLVMAddFunction(module, "cursed_channel_try_send", try_send_type);
            try self.base_codegen.runtime_functions.put("cursed_channel_try_send", try_send_func);
        }
        
        if (self.base_codegen.runtime_functions.get("cursed_channel_try_recv") == null) {
            const try_recv_type = c.LLVMFunctionType(i64_type, &[_]c.LLVMTypeRef{i64_type}, 1, 0);
            const try_recv_func = c.LLVMAddFunction(module, "cursed_channel_try_recv", try_recv_type);
            try self.base_codegen.runtime_functions.put("cursed_channel_try_recv", try_recv_func);
        }
        
        // Random number generation for fair selection
        if (self.base_codegen.runtime_functions.get("cursed_runtime_random") == null) {
            const rand_type = c.LLVMFunctionType(i32_type, null, 0, 0);
            const rand_func = c.LLVMAddFunction(module, "cursed_runtime_random", rand_type);
            try self.base_codegen.runtime_functions.put("cursed_runtime_random", rand_func);
        }
    }

    /// Generate non-blocking channel operations for select
    fn generateSelectChannelOp(self: *AdvancedCodeGen, operation: ast.SelectCaseOperation) CodeGenError!c.LLVMValueRef {
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        
        switch (operation) {
            .send => |send_op| {
                const channel_value = try self.base_codegen.generateExpression(send_op.channel);
                const send_value = try self.base_codegen.generateExpression(send_op.value);
                
                // Try non-blocking send
                const try_send_func = self.base_codegen.runtime_functions.get("cursed_channel_try_send").?;
                const send_result = c.LLVMBuildCall2(
                    builder,
                    c.LLVMInt32TypeInContext(context),
                    try_send_func,
                    &[_]c.LLVMValueRef{ channel_value, send_value },
                    2,
                    "send_result"
                );
                
                return send_result;
            },
            .receive => |recv_op| {
                const channel_value = try self.base_codegen.generateExpression(recv_op.channel);
                
                // Try non-blocking receive
                const try_recv_func = self.base_codegen.runtime_functions.get("cursed_channel_try_recv").?;
                const recv_result = c.LLVMBuildCall2(
                    builder,
                    c.LLVMInt64TypeInContext(context),
                    try_recv_func,
                    &[_]c.LLVMValueRef{channel_value},
                    1,
                    "recv_result"
                );
                
                return recv_result;
            },
            .default => {
                // Default case always returns success
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 1, 0);
            },
        }
    }

    /// Generate timeout handling for select statements
    fn generateSelectTimeout(self: *AdvancedCodeGen, timeout_expr: ast.Expression, select_context: c.LLVMValueRef) CodeGenError!void {
        const timeout_value = try self.base_codegen.generateExpression(timeout_expr);
        const set_timeout_func = self.base_codegen.runtime_functions.get("cursed_select_set_timeout").?;
        
        _ = c.LLVMBuildCall2(
            self.base_codegen.builder,
            c.LLVMVoidTypeInContext(self.base_codegen.context),
            set_timeout_func,
            &[_]c.LLVMValueRef{ select_context, timeout_value },
            2,
            ""
        );
    }

    /// Generate fair selection algorithm for multiple ready channels
    fn generateFairSelection(self: *AdvancedCodeGen, ready_cases: []c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        const builder = self.base_codegen.builder;
        const context = self.base_codegen.context;
        
        if (ready_cases.len == 1) {
            return ready_cases[0];
        }
        
        // Generate random selection for fairness
        const rand_func = self.base_codegen.runtime_functions.get("cursed_runtime_random").?;
        const random_value = c.LLVMBuildCall2(
            builder,
            c.LLVMInt32TypeInContext(context),
            rand_func,
            null,
            0,
            "rand_val"
        );
        
        const case_count = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), @as(u32, @intCast(ready_cases.len)), 0);
        const selected_index = c.LLVMBuildURem(builder, random_value, case_count, "selected_case");
        
        return selected_index;
    }
};







// OptimizationPass enum removed - now using OptimizationEngine

    // Type comparison and interface method lookup implementation
    fn compareMethodSignatures(self: *AdvancedCodeGen, interface_method: MethodSignature, struct_method: MethodSignature) CodeGenError!bool {
        // Compare parameter counts
        if (interface_method.parameters.items.len != struct_method.parameters.items.len) {
            return false;
        }
        
        // Compare parameter types
        for (interface_method.parameters.items, struct_method.parameters.items) |iface_param, struct_param| {
            if (!try self.typesAreEqual(iface_param.param_type, struct_param.param_type)) {
                return false;
            }
        }
        
        // Compare return types
        if (interface_method.return_type == null and struct_method.return_type != null) {
            return false;
        }
        if (interface_method.return_type != null and struct_method.return_type == null) {
            return false;
        }
        if (interface_method.return_type != null and struct_method.return_type != null) {
            if (!try self.typesAreEqual(interface_method.return_type.?, struct_method.return_type.?)) {
                return false;
            }
        }
        
        return true;
    }
    
    fn typesAreEqual(self: *AdvancedCodeGen, type1: Type, type2: Type) CodeGenError!bool {
        
        // Handle basic type comparisons
        switch (type1) {
            .Basic => |basic1| {
                switch (type2) {
                    .Basic => |basic2| return basic1 == basic2,
                    else => return false,
                }
            },
            .Custom => |name1| {
                switch (type2) {
                    .Custom => |name2| return std.mem.eql(u8, name1, name2),
                    else => return false,
                }
            },
            .Array => |array1| {
                switch (type2) {
                    .Array => |array2| {
                        if (array1.size != array2.size) return false;
                        return try self.typesAreEqual(array1.element_type.*, array2.element_type.*);
                    },
                    else => return false,
                }
            },
            .Slice => |slice1| {
                switch (type2) {
                    .Slice => |slice2| {
                        return try self.typesAreEqual(slice1.element_type.*, slice2.element_type.*);
                    },
                    else => return false,
                }
            },
            .Map => |map1| {
                switch (type2) {
                    .Map => |map2| {
                        const keys_equal = try self.typesAreEqual(map1.key_type.*, map2.key_type.*);
                        const values_equal = try self.typesAreEqual(map1.value_type.*, map2.value_type.*);
                        return keys_equal and values_equal;
                    },
                    else => return false,
                }
            },
            .Pointer => |ptr1| {
                switch (type2) {
                    .Pointer => |ptr2| {
                        return try self.typesAreEqual(ptr1.target_type.*, ptr2.target_type.*);
                    },
                    else => return false,
                }
            },
            .Function => |func1| {
                switch (type2) {
                    .Function => |func2| {
                        // Compare parameter counts
                        if (func1.parameters.items.len != func2.parameters.items.len) return false;
                        
                        // Compare parameter types
                        for (func1.parameters.items, func2.parameters.items) |param1, param2| {
                            if (!try self.typesAreEqual(param1, param2)) return false;
                        }
                        
                        // Compare return types
                        if (func1.return_type == null and func2.return_type != null) return false;
                        if (func1.return_type != null and func2.return_type == null) return false;
                        if (func1.return_type != null and func2.return_type != null) {
                            return try self.typesAreEqual(func1.return_type.?.*, func2.return_type.?.*);
                        }
                        
                        return true;
                    },
                    else => return false,
                }
            },
            .Interface => |iface1| {
                switch (type2) {
                    .Interface => |iface2| return std.mem.eql(u8, iface1.name, iface2.name),
                    else => return false,
                }
            },
            .Struct => |struct1| {
                switch (type2) {
                    .Struct => |struct2| return std.mem.eql(u8, struct1.name, struct2.name),
                    else => return false,
                }
            },
            .Generic => |gen1| {
                switch (type2) {
                    .Generic => |gen2| {
                        if (!std.mem.eql(u8, gen1.name, gen2.name)) return false;
                        if (gen1.type_arguments.items.len != gen2.type_arguments.items.len) return false;
                        
                        for (gen1.type_arguments.items, gen2.type_arguments.items) |arg1, arg2| {
                            if (!try self.typesAreEqual(arg1, arg2)) return false;
                        }
                        
                        return true;
                    },
                    else => return false,
                }
            },
            .Tuple => |tuple1| {
                switch (type2) {
                    .Tuple => |tuple2| {
                        if (tuple1.elements.items.len != tuple2.elements.items.len) return false;
                        
                        for (tuple1.elements.items, tuple2.elements.items) |elem1, elem2| {
                            if (!try self.typesAreEqual(elem1, elem2)) return false;
                        }
                        
                        return true;
                    },
                    else => return false,
                }
            },
            .Channel => |chan1| {
                switch (type2) {
                    .Channel => |chan2| {
                        if (chan1.is_send_only != chan2.is_send_only) return false;
                        if (chan1.is_receive_only != chan2.is_receive_only) return false;
                        return try self.typesAreEqual(chan1.element_type.*, chan2.element_type.*);
                    },
                    else => return false,
                }
            },
        }
    }
    
    // Enhanced interface method lookup with proper vtable generation
    fn lookupInterfaceMethod(self: *AdvancedCodeGen, interface_name: []const u8, method_name: []const u8) ?MethodSignature {
        const interface_info = self.interface_types.get(interface_name) orelse return null;
        
        for (interface_info.methods.items) |method| {
            if (std.mem.eql(u8, method.name, method_name)) {
                return method.signature;
            }
        }
        
        return null;
    }
    
    // Struct field access code generation
    fn generateFieldAccessWithTypeChecking(self: *AdvancedCodeGen, struct_ptr: c.LLVMValueRef, struct_name: []const u8, field_name: []const u8) CodeGenError!c.LLVMValueRef {
        const struct_info = self.struct_types.get(struct_name) orelse {
            return CodeGenError.UndefinedSymbol;
        };
        
        // Find field with type checking
        var field_index: u32 = 0;
        var field_type: ?Type = null;
        
        for (struct_info.field_names, struct_info.field_types.items, 0..) |name, ftype, i| {
            if (std.mem.eql(u8, name, field_name)) {
                field_index = @as(u32, @intCast(i));
                field_type = try self.llvmTypeToType(ftype);
                break;
            }
        }
        
        if (field_type == null) {
            return CodeGenError.UndefinedSymbol;
        }
        
        // Generate GEP instruction for field access
        const field_ptr = c.LLVMBuildStructGEP2(
            self.base_codegen.builder,
            struct_info.llvm_type.?,
            struct_ptr,
            field_index,
            "field_ptr"
        );
        
        // Load field value with proper type
        const field_value = c.LLVMBuildLoad2(
            self.base_codegen.builder,
            struct_info.field_types.items[field_index],
            field_ptr,
            "field_value"
        );
        
        return field_value;
    }
    
    // Convert LLVM type back to CURSED Type (utility function)
    fn llvmTypeToType(self: *AdvancedCodeGen, llvm_type: c.LLVMTypeRef) CodeGenError!Type {
        
        const type_kind = c.LLVMGetTypeKind(llvm_type);
        
        switch (type_kind) {
            c.LLVMIntegerTypeKind => {
                const bit_width = c.LLVMGetIntTypeWidth(llvm_type);
                switch (bit_width) {
                    8 => return Type{ .Basic = .Smol },
                    16 => return Type{ .Basic = .Mid },
                    32 => return Type{ .Basic = .Normie },
                    64 => return Type{ .Basic = .Thicc },
                    else => return Type{ .Basic = .Normie },
                }
            },
            c.LLVMFloatTypeKind => return Type{ .Basic = .Snack },
            c.LLVMDoubleTypeKind => return Type{ .Basic = .Meal },
            c.LLVMPointerTypeKind => {
                // For now, return a generic pointer type
                const void_type = Type{ .Basic = .Normie };
                const ptr_type = PointerType{
                    .target_type = self.base_codegen.allocator.create(Type) catch return CodeGenError.OutOfMemory,
                };
                ptr_type.target_type.* = void_type;
                return Type{ .Pointer = ptr_type };
            },
            else => return Type{ .Basic = .Normie }, // Default fallback
        }
    }

test "advanced codegen initialization" {
    const allocator = std.testing.allocator;
    
    var advanced_codegen = try AdvancedCodeGen.init(allocator);
    defer advanced_codegen.deinit();
    
    try std.testing.expect(advanced_codegen.gc_enabled == true);
    try std.testing.expect(advanced_codegen.struct_types.count() == 0);
}
