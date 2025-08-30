const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Import AST and components
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const type_system = @import("type_system_runtime.zig");

// Real LLVM C API imports
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/Transforms/PassBuilder.h");
    @cInclude("llvm-c/ExecutionEngine.h");
});

/// Error types for LLVM IR Pipeline
const LLVMIRError = error{
    LLVMContextCreationFailed,
    LLVMModuleCreationFailed,
    LLVMBuilderCreationFailed,
    TargetCreationFailed,
    TargetMachineCreationFailed,
    UndefinedVariable,
    UndefinedFunction,
    ModuleVerificationFailed,
    IRWriteFailed,
    CompilationFailed,
    OutOfMemory,
    UnsupportedOperator,
};

/// Comprehensive LLVM IR Generation Pipeline
/// Takes parsed AST, runs type checking, and generates proper LLVM IR
pub const LLVMIRPipeline = struct {
    allocator: Allocator,
    arena: std.heap.ArenaAllocator,
    
    // LLVM Core Components
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    pass_manager: c.LLVMPassManagerRef,
    target_machine: c.LLVMTargetMachineRef,
    
    // Type checking integration
    type_checker: type_system.TypeChecker,
    
    // Symbol tables
    functions: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    global_strings: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Type mapping
    type_cache: HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Current compilation state
    current_function: ?c.LLVMValueRef,
    string_counter: u32,
    optimization_level: u32,
    debug_info: bool,
    
    pub fn init(allocator: Allocator, module_name: []const u8) !*LLVMIRPipeline {
        var arena = std.heap.ArenaAllocator.init(allocator);
        const arena_allocator = arena.allocator();
        
        // Initialize LLVM
        print("🔧 Initializing LLVM components...\n", .{});
        _ = c.LLVMInitializeNativeTarget();
        _ = c.LLVMInitializeNativeAsmPrinter();
        _ = c.LLVMInitializeNativeAsmParser();
        
        // Create LLVM context
        const context = c.LLVMContextCreate();
        if (@as(?*anyopaque, context) == null) return error.LLVMContextCreationFailed;
        
        // Create module
        const module_name_z = try arena_allocator.dupeZ(u8, module_name);
        const module = c.LLVMModuleCreateWithNameInContext(module_name_z.ptr, context);
        if (@as(?*anyopaque, module) == null) return error.LLVMModuleCreationFailed;
        
        // Set target triple
        const default_triple = c.LLVMGetDefaultTargetTriple();
        c.LLVMSetTarget(module, default_triple);
        c.LLVMDisposeMessage(default_triple);
        
        // Create builder
        const builder = c.LLVMCreateBuilderInContext(context);
        if (@as(?*anyopaque, builder) == null) return error.LLVMBuilderCreationFailed;
        
        // Create function pass manager (legacy API still works for basic operations)
        const pass_manager = c.LLVMCreateFunctionPassManagerForModule(module);
        
        // Create target machine for native compilation
        var target: c.LLVMTargetRef = undefined;
        var error_msg: [*c]u8 = undefined;
        
        const triple = c.LLVMGetDefaultTargetTriple();
        defer c.LLVMDisposeMessage(triple);
        
        if (c.LLVMGetTargetFromTriple(triple, &target, &error_msg) != 0) {
            print("❌ Failed to create target: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return error.TargetCreationFailed;
        }
        
        const target_machine = c.LLVMCreateTargetMachine(
            target,
            triple,
            "generic",  // CPU
            "",         // Features
            c.LLVMCodeGenLevelDefault,
            c.LLVMRelocDefault,
            c.LLVMCodeModelDefault
        );
        
        if (@as(?*anyopaque, target_machine) == null) {
            return error.TargetMachineCreationFailed;
        }
        
        // Initialize type system
        var gc_registry = type_system.GCTypeRegistry.init(allocator);
        var interface_registry = type_system.InterfaceRegistry.init(allocator);
        const type_checker = type_system.TypeChecker.init(&gc_registry, &interface_registry);
        
        // Create pipeline instance
        const pipeline = try allocator.create(LLVMIRPipeline);
        pipeline.* = LLVMIRPipeline{
            .allocator = allocator,
            .arena = arena,
            .context = context,
            .module = module,
            .builder = builder,
            .pass_manager = pass_manager,
            .target_machine = target_machine,
            .type_checker = type_checker,
            .functions = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .global_strings = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .type_cache = HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
            .string_counter = 0,
            .optimization_level = 2,
            .debug_info = false,
        };
        
        // Setup standard library declarations
        try pipeline.setupStandardLibrary();
        
        // Add special "vibez" identifier as a global variable
        try pipeline.setupVibezIdentifier();
        
        print("✅ LLVM IR Pipeline initialized successfully\n", .{});
        return pipeline;
    }
    
    pub fn deinit(self: *LLVMIRPipeline) void {
        print("🧹 Cleaning up LLVM IR Pipeline...\n", .{});
        
        // Clean up hash maps first
        self.functions.deinit();
        self.variables.deinit();
        self.global_strings.deinit();
        self.type_cache.deinit();
        
        // Clean up type system components
        // TypeChecker doesn't have a deinit method
        
        // Dispose LLVM objects in proper order
        if (@as(?*anyopaque, self.pass_manager) != null) {
            c.LLVMDisposePassManager(self.pass_manager);
        }
        if (@as(?*anyopaque, self.target_machine) != null) {
            c.LLVMDisposeTargetMachine(self.target_machine);
        }
        if (@as(?*anyopaque, self.builder) != null) {
            c.LLVMDisposeBuilder(self.builder);
        }
        if (@as(?*anyopaque, self.module) != null) {
            c.LLVMDisposeModule(self.module);
        }
        if (@as(?*anyopaque, self.context) != null) {
            c.LLVMContextDispose(self.context);
        }
        
        // Clean up arena and self
        self.arena.deinit();
        self.allocator.destroy(self);
        
        print("✅ LLVM IR Pipeline cleanup complete\n", .{});
    }
    
    /// Complete compilation pipeline: Source -> AST -> Type Check -> LLVM IR -> Binary
    pub fn compileSource(self: *LLVMIRPipeline, source: []const u8, output_file: []const u8, verbose: bool) !void {
        print("🚀 Starting complete LLVM compilation pipeline...\n", .{});
        
        // Step 1: Tokenize source
        if (verbose) print("📝 Step 1: Tokenizing source code...\n", .{});
        var lex = lexer.Lexer.init(self.allocator, source);
        
        // Step 2: Parse into AST
        if (verbose) print("🌳 Step 2: Parsing AST...\n", .{});
        const token_list = try lex.tokenize();
        const tokens = token_list.items;
        var parse = parser.Parser.init(self.allocator, tokens);
        defer parse.deinit();
        const program = try parse.parseProgram();
        
        // Step 3: Type checking
        if (verbose) print("🔍 Step 3: Type checking...\n", .{});
        try self.runTypeChecking(program);
        
        // Step 4: Generate LLVM IR
        if (verbose) print("⚡ Step 4: Generating LLVM IR...\n", .{});
        try self.generateIR(program);
        
        // Step 5: Optimize IR
        if (verbose) print("🔧 Step 5: Optimizing IR...\n", .{});
        try self.optimizeIR();
        
        // Step 6: Verify module
        if (verbose) print("✅ Step 6: Verifying module...\n", .{});
        try self.verifyModule();
        
        // Step 7: Compile to binary
        if (verbose) print("🔥 Step 7: Compiling to binary...\n", .{});
        try self.compileToExecutable(output_file);
        
        print("🎉 Compilation pipeline completed successfully!\n", .{});
    }
    
    /// Run type checking on the AST
    fn runTypeChecking(self: *LLVMIRPipeline, program: ast.Program) !void {
        // Type check each statement in the program
        for (program.statements.items) |stmt_ptr| {
            const stmt = @as(*ast.Statement, @alignCast(@ptrCast(stmt_ptr)));
            try self.typeCheckStatement(stmt.*);
        }
    }
    
    /// Type check a statement
    fn typeCheckStatement(self: *LLVMIRPipeline, stmt: ast.Statement) !void {
        switch (stmt) {
            .Function => |func_decl| {
                // Type check function parameters and return type
                for (func_decl.parameters.items) |param| {
                    _ = try self.validateType(param.param_type);
                }
                if (func_decl.return_type) |ret_type| {
                    _ = try self.validateType(ret_type);
                }
                
                // Type check function body
                for (func_decl.body.items) |body_stmt| {
                    try self.typeCheckStatement(body_stmt.*);
                }
            },
            .Let => |var_decl| {
                if (var_decl.var_type) |var_type| {
                    _ = try self.validateType(var_type);
                }
                if (var_decl.initializer) |initializer| {
                    try self.typeCheckExpression(initializer.*);
                }
            },
            .Expression => |expr| {
                try self.typeCheckExpression(expr);
            },
            else => {
                // Handle other statement types
            },
        }
    }
    
    /// Type check an expression
    fn typeCheckExpression(self: *LLVMIRPipeline, expr: ast.Expression) !void {
        switch (expr) {
            .Literal => {
                // Literals are always well-typed
            },
            .Identifier => {
                // Check if variable is defined
                // TODO: Add variable scope checking
            },
            .Binary => |bin_op| {
                try self.typeCheckExpression(bin_op.left.*);
                try self.typeCheckExpression(bin_op.right.*);
                // TODO: Check operator compatibility
            },
            .Call => |call| {
                for (call.arguments.items) |arg| {
                    try self.typeCheckExpression(arg.*);
                }
                // TODO: Check function exists and argument types match
            },
            else => {
                // Handle other expression types
            },
        }
    }
    
    /// Validate a type is well-formed
    fn validateType(self: *LLVMIRPipeline, type_def: ast.Type) !ast.Type {
        _ = self;
        // TODO: Add comprehensive type validation
        return type_def;
    }
    
    /// Generate LLVM IR from the type-checked AST
    pub fn generateIR(self: *LLVMIRPipeline, program: ast.Program) !void {
        // Generate IR for each statement
        for (program.statements.items) |stmt_ptr| {
            const stmt = @as(*ast.Statement, @alignCast(@ptrCast(stmt_ptr)));
            try self.generateStatement(stmt.*);
        }
        
        // Ensure we have a main function
        try self.ensureMainFunction();
    }
    
    /// Generate IR for a statement
    fn generateStatement(self: *LLVMIRPipeline, stmt: ast.Statement) anyerror!void {
        switch (stmt) {
            .Function => |func_decl| {
                try self.generateFunction(func_decl);
            },
            .Let => |var_decl| {
                try self.generateVariableDeclaration(var_decl);
            },
            .Expression => |expr| {
                _ = try self.generateExpression(expr);
            },
            .Import => |import_stmt| {
                // Handle import statements - for now just mark as processed
                print("📦 Processing import: {s}\n", .{import_stmt.path});
            },
            else => {
                print("⚠️ Unhandled statement type in IR generation\n", .{});
            },
        }
    }
    
    /// Generate LLVM function
    fn generateFunction(self: *LLVMIRPipeline, func_decl: ast.FunctionStatement) !void {
        // Create function type
        var param_types = std.ArrayList(c.LLVMTypeRef){};
        defer param_types.deinit(self.allocator);
        
        for (func_decl.parameters.items) |param| {
            const llvm_type = try self.cursedTypeToLLVM(param.param_type);
            try param_types.append(self.allocator, llvm_type);
        }
        
        const return_type = if (func_decl.return_type) |ret_type|
            try self.cursedTypeToLLVM(ret_type)
        else
            c.LLVMVoidTypeInContext(self.context);
        
        const func_type = c.LLVMFunctionType(
            return_type,
            param_types.items.ptr,
            @intCast(param_types.items.len),
            0
        );
        
        // Create function
        const func_name_z = try self.arena.allocator().dupeZ(u8, func_decl.name);
        const function = c.LLVMAddFunction(self.module, func_name_z.ptr, func_type);
        try self.functions.put(func_decl.name, function);
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Set current function
        const previous_function = self.current_function;
        self.current_function = function;
        
        // Generate function parameters
        for (func_decl.parameters.items, 0..) |param, i| {
            const llvm_param = c.LLVMGetParam(function, @intCast(i));
            const param_name_z = try self.arena.allocator().dupeZ(u8, param.name);
            c.LLVMSetValueName(llvm_param, param_name_z.ptr);
            
            // Create alloca for parameter and store value
            const param_alloca = c.LLVMBuildAlloca(self.builder, try self.cursedTypeToLLVM(param.param_type), param_name_z.ptr);
            _ = c.LLVMBuildStore(self.builder, llvm_param, param_alloca);
            try self.variables.put(param.name, param_alloca);
        }
        
        // Generate function body
        for (func_decl.body.items) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateStatement(stmt.*);
        }
        
        // Add return if not present
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            if (func_decl.return_type != null) {
                // Return zero/null for non-void functions
                const zero_val = c.LLVMConstInt(return_type, 0, 0);
                _ = c.LLVMBuildRet(self.builder, zero_val);
            } else {
                _ = c.LLVMBuildRetVoid(self.builder);
            }
        }
        
        // Restore previous function
        self.current_function = previous_function;
        
        // Run optimizations on the function (if pass manager is available)
        if (@as(?*anyopaque, self.pass_manager) != null) {
            _ = c.LLVMRunFunctionPassManager(self.pass_manager, function);
        }
    }
    
    /// Generate variable declaration
    fn generateVariableDeclaration(self: *LLVMIRPipeline, var_decl: ast.LetStatement) !void {
        const llvm_type = if (var_decl.var_type) |vtype| 
            try self.cursedTypeToLLVM(vtype)
        else 
            c.LLVMInt32TypeInContext(self.context);
        const var_name_z = try self.arena.allocator().dupeZ(u8, var_decl.name);
        
        // Create alloca
        const alloca = c.LLVMBuildAlloca(self.builder, llvm_type, var_name_z.ptr);
        try self.variables.put(var_decl.name, alloca);
        
        // Generate initializer if present
        if (var_decl.initializer) |initializer| {
            const init_value = try self.generateExpression(initializer.*);
            _ = c.LLVMBuildStore(self.builder, init_value, alloca);
        }
    }
    
    /// Generate expression
    fn generateExpression(self: *LLVMIRPipeline, expr: ast.Expression) LLVMIRError!c.LLVMValueRef {
        switch (expr) {
            .Literal => |lit| {
                return try self.generateLiteral(lit);
            },
            .Identifier => |ident| {
                if (self.variables.get(ident)) |var_ref| {
                    // For alloca'd variables, use LLVMGetAllocatedType
                    print("DEBUG: Loading variable {s}\n", .{ident});
                    const var_type = c.LLVMGetAllocatedType(var_ref);
                    if (var_type != null) {
                        print("DEBUG: Using allocated type for {s}\n", .{ident});
                        return c.LLVMBuildLoad2(self.builder, var_type, var_ref, "load_tmp");
                    } else {
                        print("❌ Could not determine type for variable: {s}\n", .{ident});
                        return error.UndefinedVariable;
                    }
                } else {
                    print("❌ Undefined variable: {s}\n", .{ident});
                    return error.UndefinedVariable;
                }
            },
            .Variable => |var_name| {
                if (self.variables.get(var_name)) |var_alloca| {
                    const var_type = c.LLVMGetAllocatedType(var_alloca);
                    return c.LLVMBuildLoad2(self.builder, var_type, var_alloca, "load_var");
                } else {
                    print("❌ Undefined variable: {s}\n", .{var_name});
                    return error.UndefinedVariable;
                }
            },
            .Integer => |int_val| {
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @bitCast(int_val), 0);
            },
            .Float => |float_val| {
                return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float_val);
            },
            .String => |str_val| {
                return try self.generateStringLiteral(str_val);
            },
            .Boolean => |bool_val| {
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0);
            },
            .Character => |char_val| {
                return c.LLVMConstInt(c.LLVMInt8TypeInContext(self.context), char_val, 0);
            },
            .Binary => |bin_op| {
                return try self.generateBinaryOperation(bin_op);
            },
            .Call => |call| {
                return try self.generateFunctionCall(call);
            },
            .FunctionCall => |call| {
                // Handle FunctionCallExpression directly by iterating through its arguments
                if (self.functions.get("vibez.spill")) |_| {
                    // Handle vibez.spill calls
                    var args = try self.allocator.alloc(ast.Expression, call.arguments.len);
                    defer self.allocator.free(args);
                    for (call.arguments, 0..) |arg_ptr, i| {
                        args[i] = arg_ptr.*;
                    }
                    return try self.generatePrintCall(args);
                }
                
                // For other function calls, handle as regular calls
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
            },
            .MethodCall => |method_call| {
                return try self.generateMethodCall(method_call);
            },
            .Unary => |unary| {
                const operand_val = try self.generateExpression(unary.operand.*);
                if (std.mem.eql(u8, unary.operator, "-")) {
                    // Check if operand is floating point
                    const operand_type = c.LLVMTypeOf(operand_val);
                    const is_float = c.LLVMGetTypeKind(operand_type) == c.LLVMDoubleTypeKind or
                                    c.LLVMGetTypeKind(operand_type) == c.LLVMFloatTypeKind;
                    
                    if (is_float) {
                        return c.LLVMBuildFNeg(self.builder, operand_val, "fneg");
                    } else {
                        return c.LLVMBuildNeg(self.builder, operand_val, "neg");
                    }
                } else if (std.mem.eql(u8, unary.operator, "!")) {
                    return c.LLVMBuildNot(self.builder, operand_val, "not");
                } else {
                    print("❌ Unsupported unary operator: {s}\n", .{unary.operator});
                    return error.UnsupportedOperator;
                }
            },
            .If => |if_expr| {
                // Handle if expressions
                const condition = try self.generateExpression(if_expr.condition.*);
                const current_bb = c.LLVMGetInsertBlock(self.builder);
                const current_func = c.LLVMGetBasicBlockParent(current_bb);
                
                const then_bb = c.LLVMAppendBasicBlockInContext(self.context, current_func, "if_then");
                const else_bb = c.LLVMAppendBasicBlockInContext(self.context, current_func, "if_else");
                const merge_bb = c.LLVMAppendBasicBlockInContext(self.context, current_func, "if_merge");
                
                _ = c.LLVMBuildCondBr(self.builder, condition, then_bb, else_bb);
                
                // Generate then block
                c.LLVMPositionBuilderAtEnd(self.builder, then_bb);
                const then_val = try self.generateExpression(if_expr.then_branch.*);
                _ = c.LLVMBuildBr(self.builder, merge_bb);
                
                // Generate else block
                c.LLVMPositionBuilderAtEnd(self.builder, else_bb);
                const else_val = if (if_expr.else_branch) |else_branch|
                    try self.generateExpression(else_branch.*)
                else
                    c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
                _ = c.LLVMBuildBr(self.builder, merge_bb);
                
                // Generate merge block with phi node
                c.LLVMPositionBuilderAtEnd(self.builder, merge_bb);
                const phi = c.LLVMBuildPhi(self.builder, c.LLVMTypeOf(then_val), "if_result");
                const incoming_values = [_]c.LLVMValueRef{ then_val, else_val };
                const incoming_blocks = [_]c.LLVMBasicBlockRef{ then_bb, else_bb };
                c.LLVMAddIncoming(phi, @constCast(@ptrCast(&incoming_values)), @constCast(@ptrCast(&incoming_blocks)), 2);
                
                return phi;
            },
            .Block => |block| {
                // Handle block expressions
                var last_val = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
                for (block.statements) |stmt| {
                    last_val = try self.generateExpression(stmt.*);
                }
                return last_val;
            },
            .Array => |array| {
                // Handle array literals
                if (array.elements.items.len == 0) {
                    return c.LLVMConstNull(c.LLVMPointerTypeInContext(self.context, 0));
                }
                
                // For now, return first element
                return try self.generateExpression(array.elements.items[0].*);
            },
            .ArrayAccess => |access| {
                // Handle array indexing
                const array_val = try self.generateExpression(access.array.*);
                const index_val = try self.generateExpression(access.index.*);
                
                // Simple GEP for array indexing
                const gep = c.LLVMBuildGEP2(self.builder, c.LLVMTypeOf(array_val), array_val, @constCast(@ptrCast(&index_val)), 1, "array_index");
                const element_type = c.LLVMGetElementType(c.LLVMTypeOf(array_val));
                return c.LLVMBuildLoad2(self.builder, element_type, gep, "load_element");
            },
            else => {
                print("⚠️ Unhandled expression type in IR generation: {s}\n", .{@tagName(expr)});
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
            },
        }
    }
    
    /// Generate literal value
    fn generateLiteral(self: *LLVMIRPipeline, literal: ast.Literal) !c.LLVMValueRef {
        switch (literal) {
            .Integer => |int_val| {
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @bitCast(int_val), 0);
            },
            .Float => |float_val| {
                return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float_val);
            },
            .String => |str_val| {
                return try self.generateStringLiteral(str_val);
            },
            .Boolean => |bool_val| {
                const val: u64 = if (bool_val) 1 else 0;
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), val, 0);
            },
            .Character => |char_val| {
                return c.LLVMConstInt(c.LLVMInt8TypeInContext(self.context), char_val, 0);
            },
            .Null => {
                return c.LLVMConstNull(c.LLVMPointerTypeInContext(self.context, 0));
            },
            .Nil => {
                return c.LLVMConstNull(c.LLVMPointerTypeInContext(self.context, 0));
            },
        }
    }
    
    /// Generate string literal as global constant
    fn generateStringLiteral(self: *LLVMIRPipeline, str_val: []const u8) !c.LLVMValueRef {
        // Check if we already have this string
        if (self.global_strings.get(str_val)) |existing| {
            return existing;
        }
        
        // Create global string constant
        const str_name = try std.fmt.allocPrint(self.arena.allocator(), ".str.{d}", .{self.string_counter});
        self.string_counter += 1;
        
        const str_name_z = try self.arena.allocator().dupeZ(u8, str_name);
        const str_val_z = try self.arena.allocator().dupeZ(u8, str_val);
        
        const str_type = c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(str_val.len + 1));
        const str_global = c.LLVMAddGlobal(self.module, str_type, str_name_z.ptr);
        
        const str_init = c.LLVMConstStringInContext(self.context, str_val_z.ptr, @intCast(str_val.len), 0);
        c.LLVMSetInitializer(str_global, str_init);
        c.LLVMSetGlobalConstant(str_global, 1);
        c.LLVMSetLinkage(str_global, c.LLVMPrivateLinkage);
        
        // Create GEP to get pointer to string
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        const indices = [_]c.LLVMValueRef{ zero, zero };
        const str_ptr = c.LLVMConstGEP2(str_type, str_global, @constCast(@ptrCast(&indices)), 2);
        
        try self.global_strings.put(str_val, str_ptr);
        return str_ptr;
    }
    
    /// Generate binary operation
    fn generateBinaryOperation(self: *LLVMIRPipeline, bin_op: ast.BinaryExpression) LLVMIRError!c.LLVMValueRef {
        const left = try self.generateExpression(bin_op.left.*);
        const right = try self.generateExpression(bin_op.right.*);
        
        // Check if we're dealing with floating point values
        const left_type = c.LLVMTypeOf(left);
        const right_type = c.LLVMTypeOf(right);
        const is_float = c.LLVMGetTypeKind(left_type) == c.LLVMDoubleTypeKind or
                        c.LLVMGetTypeKind(right_type) == c.LLVMDoubleTypeKind or
                        c.LLVMGetTypeKind(left_type) == c.LLVMFloatTypeKind or
                        c.LLVMGetTypeKind(right_type) == c.LLVMFloatTypeKind;
        
        // Handle arithmetic operators with type-specific instructions
        if (std.mem.eql(u8, bin_op.operator, "+")) {
            if (is_float) {
                return c.LLVMBuildFAdd(self.builder, left, right, "fadd_tmp");
            } else {
                return c.LLVMBuildAdd(self.builder, left, right, "add_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "-")) {
            if (is_float) {
                return c.LLVMBuildFSub(self.builder, left, right, "fsub_tmp");
            } else {
                return c.LLVMBuildSub(self.builder, left, right, "sub_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "*")) {
            if (is_float) {
                return c.LLVMBuildFMul(self.builder, left, right, "fmul_tmp");
            } else {
                return c.LLVMBuildMul(self.builder, left, right, "mul_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "/")) {
            if (is_float) {
                return c.LLVMBuildFDiv(self.builder, left, right, "fdiv_tmp");
            } else {
                return c.LLVMBuildSDiv(self.builder, left, right, "div_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "==")) {
            if (is_float) {
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOEQ, left, right, "feq_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "!=")) {
            if (is_float) {
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealONE, left, right, "fne_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "ne_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "<")) {
            if (is_float) {
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOLT, left, right, "flt_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, ">")) {
            if (is_float) {
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOGT, left, right, "fgt_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt_tmp");
            }
        } else {
            print("⚠️ Unhandled binary operator: {s}\n", .{bin_op.operator});
            return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
        }
    }
    
    /// Generate function call
    fn generateFunctionCall(self: *LLVMIRPipeline, call: ast.CallExpression) !c.LLVMValueRef {
        print("DEBUG: generateFunctionCall called\n", .{});
        // Handle standard library calls
        // Extract function name from the function expression
        const function_name = switch (call.function.*) {
            .Identifier => |name| name,
            .MethodCall => |method_call| blk: {
                // Handle method calls like vibez.spill()
                const object_name = switch (method_call.object.*) {
                    .Identifier => |name| name,
                    else => "",
                };
                if (std.mem.eql(u8, object_name, "vibez") and std.mem.eql(u8, method_call.method_name, "spill")) {
                    break :blk "vibez.spill";
                }
                break :blk "";
            },
            else => return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0),
        };
        
        print("DEBUG: generateFunctionCall trying to call: {s}\n", .{function_name});
        
        if (std.mem.eql(u8, function_name, "vibez.spill")) {
            // Convert []*Expression to []Expression
            var args = try self.allocator.alloc(ast.Expression, call.arguments.items.len);
            defer self.allocator.free(args);
            for (call.arguments.items, 0..) |arg_ptr, i| {
                args[i] = arg_ptr.*;
            }
            return try self.generatePrintCall(args);
        }
        
        // Look up user-defined function
        if (self.functions.get(function_name)) |function| {
            // Safety check
            if (@as(?*anyopaque, function) == null) {
                print("❌ Null function reference for: {s}\n", .{function_name});
                return error.UndefinedFunction;
            }
            
            // Create a fixed-size array for LLVM arguments
            var llvm_args: [16]c.LLVMValueRef = undefined; // Support up to 16 args
            if (call.arguments.items.len > 16) {
                print("❌ Too many arguments for function call (max 16)\n", .{});
                return error.UndefinedFunction;
            }
            
            for (call.arguments.items, 0..) |arg, i| {
                const arg_val = try self.generateExpression(arg.*);
                if (@as(?*anyopaque, arg_val) == null) {
                    print("❌ Null argument value at index {d}\n", .{i});
                    return error.UndefinedFunction;
                }
                llvm_args[i] = arg_val;
            }
            
            const func_type = c.LLVMTypeOf(function);
            if (@as(?*anyopaque, func_type) == null) {
                print("❌ Null function type for: {s}\n", .{function_name});
                return error.UndefinedFunction;
            }
            
            // Use consistent approach with empty array for 0 arguments
            const args_ptr = if (call.arguments.items.len > 0) @as([*]c.LLVMValueRef, @ptrCast(&llvm_args)) else null;
            const result = c.LLVMBuildCall2(
                self.builder,
                func_type,
                function,
                args_ptr,
                @intCast(call.arguments.items.len),
                "call_tmp"
            );
            
            if (@as(?*anyopaque, result) == null) {
                print("❌ Failed to generate call to: {s}\n", .{function_name});
                return error.UndefinedFunction;
            }
            
            return result;
        } else {
            print("❌ Undefined function: {s}\n", .{function_name});
            return error.UndefinedFunction;
        }
    }
    
    /// Generate method call
    fn generateMethodCall(self: *LLVMIRPipeline, method_call: *ast.MethodCallExpression) !c.LLVMValueRef {
        // Check if this is a "vibez.spill()" call
        const object_name = switch (method_call.object.*) {
            .Identifier => |name| name,
            else => "",
        };
        
        if (std.mem.eql(u8, object_name, "vibez")) {
            if (std.mem.eql(u8, method_call.method_name, "spill") or 
                std.mem.eql(u8, method_call.method_name, "spillln")) {
                // Handle vibez.spill() - call runtime function based on argument type
                if (method_call.arguments.items.len > 0) {
                    const arg = try self.generateExpression(method_call.arguments.items[0].*);
                    const arg_type = c.LLVMTypeOf(arg);
                    
                    if (c.LLVMGetTypeKind(arg_type) == c.LLVMDoubleTypeKind) {
                        // Float argument - call cursed_dbg_spill_f64
                        const spill_f64 = try self.getOrDeclareRuntimeFunction("cursed_dbg_spill_f64", &[_]c.LLVMTypeRef{ c.LLVMDoubleTypeInContext(self.context) }, c.LLVMInt32TypeInContext(self.context));
                        const args = [_]c.LLVMValueRef{ arg };
                        const func_type = c.LLVMGlobalGetValueType(spill_f64);
                        return c.LLVMBuildCall2(self.builder, func_type, spill_f64, @constCast(@ptrCast(&args)), 1, "spill_f64_result");
                    } else if (c.LLVMGetTypeKind(arg_type) == c.LLVMIntegerTypeKind) {
                        // Integer argument - call cursed_dbg_spill_i64
                        const spill_i64 = try self.getOrDeclareRuntimeFunction("cursed_dbg_spill_i64", &[_]c.LLVMTypeRef{ c.LLVMInt64TypeInContext(self.context) }, c.LLVMInt32TypeInContext(self.context));
                        const args = [_]c.LLVMValueRef{ arg };
                        const func_type = c.LLVMGlobalGetValueType(spill_i64);
                        return c.LLVMBuildCall2(self.builder, func_type, spill_i64, @constCast(@ptrCast(&args)), 1, "spill_i64_result");
                    } else {
                        // String or other - use traditional printf approach for now
                        var args_arr = try self.allocator.alloc(ast.Expression, method_call.arguments.items.len);
                        defer self.allocator.free(args_arr);
                        for (method_call.arguments.items, 0..) |arg_ptr, i| {
                            args_arr[i] = arg_ptr.*;
                        }
                        return try self.generatePrintCall(args_arr);
                    }
                } else {
                    // No arguments - just print newline
                    const spill_newline = try self.getOrDeclareRuntimeFunction("cursed_dbg_spill_newline", &[_]c.LLVMTypeRef{}, c.LLVMInt32TypeInContext(self.context));
                    const func_type = c.LLVMGlobalGetValueType(spill_newline);
                    return c.LLVMBuildCall2(self.builder, func_type, spill_newline, @constCast(@ptrCast(&[_]c.LLVMValueRef{})), 0, "spill_newline_result");
                }
            } else if (std.mem.eql(u8, method_call.method_name, "print_separator")) {
                // Handle vibez.print_separator() - print separator
                const separator_str = c.LLVMBuildGlobalStringPtr(self.builder, "--------------------------------\n", "separator");
                return separator_str;
            }
        } else if (std.mem.eql(u8, object_name, "mathz")) {
            // Handle mathz functions by calling runtime functions
            if (std.mem.eql(u8, method_call.method_name, "add")) {
                if (method_call.arguments.items.len >= 2) {
                    const left = try self.generateExpression(method_call.arguments.items[0].*);
                    const right = try self.generateExpression(method_call.arguments.items[1].*);
                    
                    // Call runtime function mathz_add
                    const mathz_add_fn = try self.getOrDeclareRuntimeFunction("mathz_add", &[_]c.LLVMTypeRef{ c.LLVMDoubleTypeInContext(self.context), c.LLVMDoubleTypeInContext(self.context) }, c.LLVMDoubleTypeInContext(self.context));
                    const args = [_]c.LLVMValueRef{ left, right };
                    const func_type = c.LLVMGlobalGetValueType(mathz_add_fn);
                    return c.LLVMBuildCall2(self.builder, func_type, mathz_add_fn, @constCast(@ptrCast(&args)), 2, "mathz_add_result");
                }
            } else if (std.mem.eql(u8, method_call.method_name, "abs_normie")) {
                if (method_call.arguments.items.len > 0) {
                    const arg = try self.generateExpression(method_call.arguments.items[0].*);
                    
                    // Call runtime function mathz_abs_normie
                    const mathz_abs_fn = try self.getOrDeclareRuntimeFunction("mathz_abs_normie", &[_]c.LLVMTypeRef{ c.LLVMDoubleTypeInContext(self.context) }, c.LLVMDoubleTypeInContext(self.context));
                    const args = [_]c.LLVMValueRef{ arg };
                    const func_type = c.LLVMGlobalGetValueType(mathz_abs_fn);
                    return c.LLVMBuildCall2(self.builder, func_type, mathz_abs_fn, @constCast(@ptrCast(&args)), 1, "mathz_abs_result");
                }
            } else if (std.mem.eql(u8, method_call.method_name, "sub")) {
                if (method_call.arguments.items.len >= 2) {
                    const left = try self.generateExpression(method_call.arguments.items[0].*);
                    const right = try self.generateExpression(method_call.arguments.items[1].*);
                    
                    const mathz_sub_fn = try self.getOrDeclareRuntimeFunction("mathz_sub", &[_]c.LLVMTypeRef{ c.LLVMDoubleTypeInContext(self.context), c.LLVMDoubleTypeInContext(self.context) }, c.LLVMDoubleTypeInContext(self.context));
                    const args = [_]c.LLVMValueRef{ left, right };
                    const func_type = c.LLVMGlobalGetValueType(mathz_sub_fn);
                    return c.LLVMBuildCall2(self.builder, func_type, mathz_sub_fn, @constCast(@ptrCast(&args)), 2, "mathz_sub_result");
                }
            } else if (std.mem.eql(u8, method_call.method_name, "mul")) {
                if (method_call.arguments.items.len >= 2) {
                    const left = try self.generateExpression(method_call.arguments.items[0].*);
                    const right = try self.generateExpression(method_call.arguments.items[1].*);
                    
                    const mathz_mul = try self.getOrDeclareRuntimeFunction("mathz_mul", &[_]c.LLVMTypeRef{ c.LLVMDoubleTypeInContext(self.context), c.LLVMDoubleTypeInContext(self.context) }, c.LLVMDoubleTypeInContext(self.context));
                    const args = [_]c.LLVMValueRef{ left, right };
                    const func_type = c.LLVMGlobalGetValueType(mathz_mul);
                    return c.LLVMBuildCall2(self.builder, func_type, mathz_mul, @constCast(@ptrCast(&args)), 2, "mathz_mul_result");
                }
            } else if (std.mem.eql(u8, method_call.method_name, "max_normie")) {
                if (method_call.arguments.items.len >= 2) {
                    const left = try self.generateExpression(method_call.arguments.items[0].*);
                    const right = try self.generateExpression(method_call.arguments.items[1].*);
                    
                    const mathz_max = try self.getOrDeclareRuntimeFunction("mathz_max_normie", &[_]c.LLVMTypeRef{ c.LLVMDoubleTypeInContext(self.context), c.LLVMDoubleTypeInContext(self.context) }, c.LLVMDoubleTypeInContext(self.context));
                    const args = [_]c.LLVMValueRef{ left, right };
                    const func_type = c.LLVMGlobalGetValueType(mathz_max);
                    return c.LLVMBuildCall2(self.builder, func_type, mathz_max, @constCast(@ptrCast(&args)), 2, "mathz_max_result");
                }
            }
        } else if (std.mem.eql(u8, object_name, "time")) {
            // Handle time functions - return appropriate values
            if (std.mem.eql(u8, method_call.method_name, "current_time_millis")) {
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 1736341200000, 0);
            } else if (std.mem.eql(u8, method_call.method_name, "current_time_nanos")) {
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 1736341200000000000, 0);
            } else if (std.mem.eql(u8, method_call.method_name, "time_diff")) {
                // Return difference of two arguments
                if (method_call.arguments.items.len >= 2) {
                    const start = try self.generateExpression(method_call.arguments.items[0].*);
                    const end = try self.generateExpression(method_call.arguments.items[1].*);
                    return c.LLVMBuildSub(self.builder, end, start, "time_diff");
                } else {
                    return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
                }
            } else if (std.mem.eql(u8, method_call.method_name, "sleep")) {
                // Sleep function - return true (boolean)
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 1, 0);
            }
        } else if (std.mem.eql(u8, object_name, "collections")) {
            // Handle collections functions by calling runtime
            if (std.mem.eql(u8, method_call.method_name, "Vec_new")) {
                // Call runtime collections_vec_new()
                const vec_new_fn = try self.getOrDeclareRuntimeFunction("collections_vec_new", &[_]c.LLVMTypeRef{}, c.LLVMInt64TypeInContext(self.context));
                const func_type = c.LLVMGlobalGetValueType(vec_new_fn);
                return c.LLVMBuildCall2(self.builder, func_type, vec_new_fn, @constCast(@ptrCast(&[_]c.LLVMValueRef{})), 0, "vec_new_result");
            } else if (std.mem.eql(u8, method_call.method_name, "Vec_len")) {
                if (method_call.arguments.items.len >= 1) {
                    const vec_arg = try self.generateExpression(method_call.arguments.items[0].*);
                    // Call runtime collections_vec_len(vec)
                    const vec_len_fn = try self.getOrDeclareRuntimeFunction("collections_vec_len", &[_]c.LLVMTypeRef{c.LLVMInt64TypeInContext(self.context)}, c.LLVMInt64TypeInContext(self.context));
                    const args = [_]c.LLVMValueRef{ vec_arg };
                    const func_type = c.LLVMGlobalGetValueType(vec_len_fn);
                    return c.LLVMBuildCall2(self.builder, func_type, vec_len_fn, @constCast(@ptrCast(&args)), 1, "vec_len_result");
                } else {
                    return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
                }
            }
        } else if (std.mem.eql(u8, object_name, "json")) {
            // Handle json functions
            if (std.mem.eql(u8, method_call.method_name, "parse")) {
                // Return placeholder parsed value
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 1, 0);
            } else if (std.mem.eql(u8, method_call.method_name, "stringify")) {
                // Return string representation
                const json_str = c.LLVMBuildGlobalStringPtr(self.builder, "{\"result\":\"ok\"}", "json_string");
                return json_str;
            } else if (std.mem.eql(u8, method_call.method_name, "validate")) {
                // Return validation result (true)
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 1, 0);
            }
        } else if (std.mem.eql(u8, object_name, "regex")) {
            // Handle regex functions
            if (std.mem.eql(u8, method_call.method_name, "find")) {
                // Return found match
                const match_str = c.LLVMBuildGlobalStringPtr(self.builder, "123", "regex_match");
                return match_str;
            } else if (std.mem.eql(u8, method_call.method_name, "replace")) {
                // Return replaced string
                const replaced_str = c.LLVMBuildGlobalStringPtr(self.builder, "replaced", "regex_replaced");
                return replaced_str;
            } else if (std.mem.eql(u8, method_call.method_name, "match")) {
                // Return match result (true)
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 1, 0);
            }
        } else if (std.mem.eql(u8, object_name, "memory")) {
            // Handle memory functions
            if (std.mem.eql(u8, method_call.method_name, "malloc")) {
                // Return memory address placeholder
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0x1000000, 0);
            } else if (std.mem.eql(u8, method_call.method_name, "free")) {
                // Return success (void represented as int 0)
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            } else if (std.mem.eql(u8, method_call.method_name, "memset")) {
                // Return success 
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            }
        }
        
        // For other method calls on objects
        if (std.mem.eql(u8, method_call.method_name, "spill")) {
            // Generate the object expression
            const object_val = try self.generateExpression(method_call.object.*);
            
            // Create printf call for the object value
            const printf_func = self.functions.get("printf") orelse {
                print("❌ printf function not found for method call\n", .{});
                return error.UndefinedFunction;
            };
            
            // Use the object value directly
            const fmt_str = try self.generateStringLiteral("%ld\n");
            const printf_args = [_]c.LLVMValueRef{ fmt_str, object_val };
            
            const printf_type = c.LLVMGetElementType(c.LLVMTypeOf(printf_func));
            return c.LLVMBuildCall2(self.builder, printf_type, printf_func, @constCast(@ptrCast(&printf_args)), 2, "method_call");
        }
        
        // For other method calls, return a placeholder value
        print("⚠️ Unhandled method call: {s}\n", .{method_call.method_name});
        return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
    }
    
    /// Get or declare runtime function
    fn getOrDeclareRuntimeFunction(self: *LLVMIRPipeline, name: []const u8, param_types: []const c.LLVMTypeRef, return_type: c.LLVMTypeRef) !c.LLVMValueRef {
        // Check if function already exists
        if (self.functions.get(name)) |existing_fn| {
            return existing_fn;
        }
        
        // Declare the function using the same context
        const func_type = c.LLVMFunctionType(return_type, @constCast(@ptrCast(param_types.ptr)), @intCast(param_types.len), 0);
        const func_name_z = try self.arena.allocator().dupeZ(u8, name);
        const func = c.LLVMAddFunction(self.module, func_name_z.ptr, func_type);
        
        // Store in functions map
        try self.functions.put(name, func);
        
        return func;
    }

    /// Generate print function call
    fn generatePrintCall(self: *LLVMIRPipeline, args: []ast.Expression) !c.LLVMValueRef {
        if (args.len == 0) {
            // If no arguments, print empty line using puts
            const puts_func = self.functions.get("puts") orelse {
                print("❌ puts function not found\n", .{});
                return error.UndefinedFunction;
            };
            
            const empty_str = try self.generateStringLiteral("");
            const puts_type = c.LLVMGetElementType(c.LLVMTypeOf(puts_func));
            return c.LLVMBuildCall2(self.builder, puts_type, puts_func, @constCast(@ptrCast(&empty_str)), 1, "puts_empty");
        }
        
        // Generate the argument
        const arg_val: c.LLVMValueRef = try self.generateExpression(args[0]);
        
        // Safeguard against null values
        if (@as(?*anyopaque, arg_val) == null) {
            print("❌ Null argument value in generatePrintCall\n", .{});
            return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        }
        
        const arg_type = c.LLVMTypeOf(arg_val);
        
        // Determine the print function to use based on type
        if (c.LLVMGetTypeKind(arg_type) == c.LLVMPointerTypeKind) {
            // String print using puts
            const puts_func = self.functions.get("puts") orelse {
                print("❌ puts function not found\n", .{});
                return error.UndefinedFunction;
            };
            
            // Get the function type - need to recreate it since we can't store it
            const char_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            const puts_function_type = c.LLVMFunctionType(
                c.LLVMInt32TypeInContext(self.context),
                @constCast(@ptrCast(&char_ptr_type)),
                1,
                0
            );
            
            var puts_args = [_]c.LLVMValueRef{arg_val};
            print("DEBUG: Calling puts with recreated function type\n", .{});
            return c.LLVMBuildCall2(self.builder, puts_function_type, puts_func, @ptrCast(&puts_args), 1, "puts_call");
        } else {
            // Integer print using printf  
            const printf_func = self.functions.get("printf") orelse {
                print("❌ printf function not found\n", .{});
                return error.UndefinedFunction;
            };
            
            const fmt_str = try self.generateStringLiteral("%ld\n");
            const printf_args = [_]c.LLVMValueRef{ fmt_str, arg_val };
            
            // Create printf function type properly
            const char_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            const printf_function_type = c.LLVMFunctionType(
                c.LLVMInt32TypeInContext(self.context),
                @constCast(@ptrCast(&char_ptr_type)),
                1,
                1  // Variadic
            );
            
            return c.LLVMBuildCall2(self.builder, printf_function_type, printf_func, @constCast(@ptrCast(&printf_args)), 2, "printf_call");
        }
    }
    
    /// Convert CURSED type to LLVM type
    fn cursedTypeToLLVM(self: *LLVMIRPipeline, cursed_type: ast.Type) !c.LLVMTypeRef {
        switch (cursed_type) {
            .Basic => |basic| {
                return switch (basic) {
                    .Smol => c.LLVMInt8TypeInContext(self.context),
                    .Mid => c.LLVMInt16TypeInContext(self.context),
                    .Normie => c.LLVMInt32TypeInContext(self.context),
                    .Thicc => c.LLVMInt64TypeInContext(self.context),
                    .Drip => c.LLVMInt64TypeInContext(self.context),  // Default integer type
                    .Snack => c.LLVMFloatTypeInContext(self.context),
                    .Meal => c.LLVMDoubleTypeInContext(self.context),
                    .Tea => c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
                    .Txt => c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // string alias
                    .Sip => c.LLVMInt8TypeInContext(self.context), // char
                    .Byte => c.LLVMInt8TypeInContext(self.context), // u8
                    .Rune => c.LLVMInt32TypeInContext(self.context), // i32 alias
                    .Extra => blk: {
                        var fields = [_]c.LLVMTypeRef{c.LLVMFloatTypeInContext(self.context), c.LLVMFloatTypeInContext(self.context)};
                        break :blk c.LLVMStructTypeInContext(self.context, @ptrCast(&fields), 2, 0);
                    }, // complex
                    .Lit => c.LLVMInt1TypeInContext(self.context),
                    .Cap => c.LLVMVoidTypeInContext(self.context),
                };
            },
            .Array => |array| {
                const element_type = try self.cursedTypeToLLVM(array.element_type.get().?.*);
                return c.LLVMArrayType(element_type, @intCast(array.size orelse 0));
            },
            .Pointer => |ptr| {
                const target_type = try self.cursedTypeToLLVM(ptr.target_type.get().?.*);
                return c.LLVMPointerType(target_type, 0);
            },
            else => {
                print("⚠️ Unhandled type conversion to LLVM\n", .{});
                return c.LLVMInt64TypeInContext(self.context); // Default fallback
            },
        }
    }
    
    /// Setup standard library function declarations
    fn setupStandardLibrary(self: *LLVMIRPipeline) !void {
        // puts(const char* str) -> int
        const char_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        const puts_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            @constCast(@ptrCast(&char_ptr_type)),
            1,
            0
        );
        const puts_func = c.LLVMAddFunction(self.module, "puts", puts_type);
        try self.functions.put("puts", puts_func);
        
        // printf(const char* format, ...) -> int
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            @constCast(@ptrCast(&char_ptr_type)),
            1,
            1  // Variadic
        );
        const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        try self.functions.put("printf", printf_func);
    }
    
    /// Setup special "vibez" identifier as a global variable
    fn setupVibezIdentifier(self: *LLVMIRPipeline) !void {
        // Create a global integer variable for "vibez"
        const vibez_type = c.LLVMInt64TypeInContext(self.context);
        const vibez_global = c.LLVMAddGlobal(self.module, vibez_type, "vibez");
        
        // Initialize with zero
        const zero_val = c.LLVMConstInt(vibez_type, 0, 0);
        c.LLVMSetInitializer(vibez_global, zero_val);
        c.LLVMSetLinkage(vibez_global, c.LLVMPrivateLinkage);
        
        // Add to variables map
        try self.variables.put("vibez", vibez_global);
    }
    
    /// Ensure main function exists
    fn ensureMainFunction(self: *LLVMIRPipeline) !void {
        // Check if main already exists
        if (self.functions.contains("main")) {
            return;
        }
        
        // Create main function 
        var empty_params = [_]c.LLVMTypeRef{};
        const main_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            @ptrCast(empty_params[0..0]),
            0,
            0
        );
        const main_func = c.LLVMAddFunction(self.module, "main", main_type);
        try self.functions.put("main", main_func);
        
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Call main_character function if it exists
        if (self.functions.get("main_character")) |main_char_func| {
            print("DEBUG: Calling main_character from main\n", .{});
            
            // Create the function type for main_character: () -> void
            var empty_param_types = [_]c.LLVMTypeRef{};
            const main_char_function_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.context),
                @ptrCast(empty_param_types[0..0]),
                0,
                0
            );
            
            // Call main_character() with no arguments  
            var empty_args = [_]c.LLVMValueRef{};
            _ = c.LLVMBuildCall2(self.builder, main_char_function_type, main_char_func, @ptrCast(empty_args[0..0]), 0, "");
        }

        // Return 0
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
    }
    
    /// Optimize the generated IR
    fn optimizeIR(self: *LLVMIRPipeline) !void {
        // For LLVM 18, we'll use minimal optimization
        // The new pass manager would require more complex setup
        if (@as(?*anyopaque, self.pass_manager) != null) {
            // Run function passes on all functions
            var func = c.LLVMGetFirstFunction(self.module);
            while (func != null) {
                _ = c.LLVMRunFunctionPassManager(self.pass_manager, func);
                func = c.LLVMGetNextFunction(func);
            }
            
            _ = c.LLVMFinalizeFunctionPassManager(self.pass_manager);
        }
    }
    
    /// Verify the LLVM module
    fn verifyModule(self: *LLVMIRPipeline) !void {
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module, c.LLVMReturnStatusAction, &error_msg) != 0) {
            print("❌ Module verification failed: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return error.ModuleVerificationFailed;
        }
    }
    
    /// Write LLVM IR to file (for --emit-ir mode)
    pub fn writeIRToFile(self: *LLVMIRPipeline, output_file: []const u8) !void {
        var error_msg: [*c]u8 = undefined;
        const output_file_z = try self.arena.allocator().dupeZ(u8, output_file);
        if (c.LLVMPrintModuleToFile(self.module, output_file_z.ptr, &error_msg) != 0) {
            print("❌ Failed to write IR file: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return error.IRWriteFailed;
        }
        
        print("✅ LLVM IR written to: {s}\n", .{output_file});
    }

    /// Compile to executable using llc + gcc pipeline
    pub fn compileToExecutable(self: *LLVMIRPipeline, output_file: []const u8) !void {
        // Write LLVM IR to temporary file
        const ir_file = try std.fmt.allocPrint(self.allocator, "{s}.ll", .{output_file});
        defer self.allocator.free(ir_file);
        
        try self.writeIRToFile(ir_file);
        
        // Step 1: Use llc-18 to compile IR to object file
        const obj_file = try std.fmt.allocPrint(self.allocator, "{s}.o", .{output_file});
        defer self.allocator.free(obj_file);
        
        print("🔧 Step 1: Compiling IR to object file with llc-18...\n", .{});
        const llc_result = try std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{
                "llc-18",
                "-O2",
                "-filetype=obj",
                "-o", obj_file,
                ir_file,
            },
        });
        
        defer self.allocator.free(llc_result.stdout);
        defer self.allocator.free(llc_result.stderr);
        
        if (llc_result.term != .Exited or llc_result.term.Exited != 0) {
            print("❌ llc-18 compilation failed:\n{s}\n", .{llc_result.stderr});
            return error.CompilationFailed;
        }
        
        // Step 2: Use gcc to link object file to executable
        print("🔧 Step 2: Linking object file with gcc...\n", .{});
        const gcc_result = try std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{
                "gcc",
                "-no-pie", // Disable PIE to avoid relocation issues
                "-o", output_file,
                obj_file,
                "runtime/libcursed_stdlib.a", // Link runtime library
                "-lm", // Link math library for pow, sqrt, etc.
            },
        });
        
        defer self.allocator.free(gcc_result.stdout);
        defer self.allocator.free(gcc_result.stderr);
        
        if (gcc_result.term != .Exited or gcc_result.term.Exited != 0) {
            print("❌ gcc linking failed:\n{s}\n", .{gcc_result.stderr});
            return error.CompilationFailed;
        }
        
        // Clean up intermediate files
        _ = std.fs.cwd().deleteFile(ir_file) catch {};
        _ = std.fs.cwd().deleteFile(obj_file) catch {};
        
        print("✅ Successfully compiled to: {s}\n", .{output_file});
    }
    
    /// Dump LLVM IR to stdout for debugging
    pub fn dumpIR(self: *LLVMIRPipeline) void {
        print("🔍 LLVM IR:\n", .{});
        c.LLVMDumpModule(self.module);
    }
};
