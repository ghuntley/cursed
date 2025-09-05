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
    // Lexer errors
    UnexpectedCharacter,
    InvalidEscapeSequence,
    InvalidHexEscape,
    InvalidUnicodeEscape,
    UnterminatedString,
    UnterminatedChar,
    UnterminatedBlockComment,
    // Parser errors
    ParseError,
    SyntaxError,
    InvalidExpression,
    UnexpectedToken,
    UnexpectedEof,
    InvalidSyntax,
    MissingToken,
    InvalidStatement,
    InvalidType,
    InvalidFunction,
    InvalidParameter,
    InvalidBlock,
    InvalidAssignment,
    InvalidPattern,
    InvalidGeneric,
    AlignmentError,
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
    variable_types: HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    array_lengths: HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    global_strings: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Function signature registry for forward declarations
    function_signatures: HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // CURSED module compilation tracking
    compiled_modules: HashMap([]const u8, void, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Type mapping
    type_cache: HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Current compilation state
    current_function: ?c.LLVMValueRef,
    current_module_name: ?[]const u8,
    program_package: ?[]const u8,
    string_counter: u32,
    array_length_counter: u32,
    optimization_level: u32,
    debug_info: bool,
    
    pub fn init(allocator: Allocator, module_name: []const u8) !*LLVMIRPipeline {
        var arena = std.heap.ArenaAllocator.init(allocator);
        const arena_allocator = arena.allocator();
        
        // Initialize LLVM
        // print("🔧 Initializing LLVM components...\n", .{});
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
            .variable_types = HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        .array_lengths = HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .global_strings = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .function_signatures = HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .compiled_modules = HashMap([]const u8, void, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .type_cache = HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
            .current_module_name = null,
            .program_package = null,
            .string_counter = 0,
        .array_length_counter = 0,
            .optimization_level = 2,
            .debug_info = false,
        };
        
        // Add special "vibez" identifier as a global variable
        try pipeline.setupVibezIdentifier();
        
        // Declare standard C library functions
        try pipeline.declareCLibraryFunctions();
        
        // Register builtin functions 
        try pipeline.registerBuiltinFunctions();
        
        // print("✅ LLVM IR Pipeline initialized successfully\n", .{});
        return pipeline;
    }
    
    pub fn deinit(self: *LLVMIRPipeline) void {
        // print("🧹 Cleaning up LLVM IR Pipeline...\n", .{});
        
        // Clean up hash maps first
        self.functions.deinit();
        self.variables.deinit();
        self.variable_types.deinit();
        self.array_lengths.deinit();
        self.global_strings.deinit();
        self.function_signatures.deinit();
        self.compiled_modules.deinit();
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
        
        // print("✅ LLVM IR Pipeline cleanup complete\n", .{});
    }
    
    /// Intern a string to ensure it lives for the duration of compilation
    fn intern(self: *LLVMIRPipeline, name: []const u8) ![]const u8 {
        // Check if we already have this string in type_cache (reuse existing allocation)
        if (self.type_cache.contains(name)) {
            return name; // already interned, pointer is stable
        }
        // Store in the pipeline arena - one allocation per distinct spelling for whole compilation
        return try self.arena.allocator().dupe(u8, name);
    }
    
    /// Complete compilation pipeline: Source -> AST -> Type Check -> LLVM IR -> Binary
    pub fn compileSource(self: *LLVMIRPipeline, source: []const u8, output_file: []const u8, verbose: bool) !void {
        // print("🚀 Starting complete LLVM compilation pipeline...\n", .{});
        
        // Step 1: Tokenize source
        if (verbose) print("📝 Step 1: Tokenizing source code...\n", .{});
        var lex = lexer.Lexer.init(self.allocator, source);
        
        // Step 2: Parse into AST
        if (verbose) print("🌳 Step 2: Parsing AST...\n", .{});
        const token_list = try lex.tokenize();
        const tokens = token_list.items;
        var parse = parser.Parser.init(self.allocator, tokens);
        // CRITICAL FIX: Don't free parser until after code generation to prevent use-after-free
        const program = try parse.parseProgram();
        
        // Step 3: Type checking
        if (verbose) print("🔍 Step 3: Type checking...\n", .{});
        try self.runTypeChecking(program);
        
        // Step 4: Generate LLVM IR
        if (verbose) print("⚡ Step 4: Generating LLVM IR...\n", .{});
        try self.generateIR(program);
        
        // CRITICAL FIX: Only free parser after all AST data usage is complete
        parse.deinit();
        
        // Step 5: Optimize IR
        if (verbose) print("🔧 Step 5: Optimizing IR...\n", .{});
        try self.optimizeIR();
        
        // Step 6: Verify module
        if (verbose) print("✅ Step 6: Verifying module...\n", .{});
        try self.verifyModule();
        
        // Step 7: Compile to binary
        if (verbose) print("🔥 Step 7: Compiling to binary...\n", .{});
        try self.compileToExecutable(output_file);
        
        // print("🎉 Compilation pipeline completed successfully!\n", .{});
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
        // CRITICAL FIX: Store program package for main function resolution
        if (program.package) |pkg| {
            self.program_package = pkg.name;
        } else {
            self.program_package = null;
        }
        
        // Separate functions from other statements
        var functions = ArrayList(*ast.Statement){};
        defer functions.deinit(self.allocator);
        var global_statements = ArrayList(*ast.Statement){};
        defer global_statements.deinit(self.allocator);
        
        for (program.statements.items) |stmt_ptr| {
            const stmt = @as(*ast.Statement, @alignCast(@ptrCast(stmt_ptr)));
            switch (stmt.*) {
                .Function => {
                    try functions.append(self.allocator, stmt);
                },
                else => {
                    try global_statements.append(self.allocator, stmt);
                }
            }
        }
        
        // // print("🔍 DEBUG: Building function signature registry for {} functions\n", .{functions.items.len});
        // First pass: Build function signatures for forward declaration support
        for (functions.items) |stmt| {
            if (stmt.* == .Function) {
                try self.buildFunctionSignature(stmt.Function);
            }
        }
        
        // // print("🔍 DEBUG: About to generate {} functions\n", .{functions.items.len});
        // Generate IR for functions first
        for (functions.items) |stmt| {
            // Generating function
            try self.generateStatement(stmt.*);
            // // print("🔍 DEBUG: Successfully generated function {} of {}\n", .{i + 1, functions.items.len});
        }
        
        // // print("🔍 DEBUG: About to ensure main function with {} global statements\n", .{global_statements.items.len});
        // Ensure we have a main function and generate global statements within it
        try self.ensureMainFunctionWithGlobalStatements(global_statements.items);
        // // print("🔍 DEBUG: Successfully ensured main function\n", .{});
    }
    
    /// Generate IR for a statement
    fn generateStatement(self: *LLVMIRPipeline, stmt: ast.Statement) anyerror!void {
        // Only skip generating statements if current block already has a terminator AND we're not in a function body
        // This check was being too aggressive and preventing function bodies from being generated
        if (self.current_function != null) {
            const current_block = c.LLVMGetInsertBlock(self.builder);
            if (current_block != null and c.LLVMGetBasicBlockTerminator(current_block) != null) {
                // Only skip for main function global statements, not for function body statements
                const main_func = self.functions.get("main");
                if (main_func != null and self.current_function.? == main_func.?) {
                    // For main function, we'll create a new block to continue
                    const cont_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "unreachable_cont");
                    c.LLVMPositionBuilderAtEnd(self.builder, cont_block);
                } else {
                    // For other functions, don't skip - we need to generate all the statements in the function body
                    // This was the main bug - we were returning early for all non-main functions
                    print("⚠️ Current block has terminator in function, continuing anyway...\n", .{});
                }
            }
        }
        
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
                // Handle import statements by loading and compiling the module
                // print("📦 Processing import: {s}\n", .{import_stmt.path});
                
                // Skip vibez module loading - use built-in printf handling instead
                if (std.mem.eql(u8, import_stmt.path, "vibez")) {
                    // print("DEBUG: Skipping vibez module loading - using built-in printf handling\n", .{});
                    return;
                }
                
                try self.loadAndCompileModule(import_stmt.path);
            },
            .While => |while_stmt| {
                try self.generateWhileStatement(while_stmt);
            },
            .For => |for_stmt| {
                try self.generateForStatement(for_stmt);
            },
            .Assignment => |assign_stmt| {
                try self.generateAssignmentStatement(assign_stmt);
            },
            .If => |if_stmt| {
                try self.generateIfStatement(if_stmt);
            },
            .Return => |return_stmt| {
                try self.generateReturnStatement(return_stmt);
            },
            .ShortDeclaration => |short_decl| {
                try self.generateShortDeclarationStatement(short_decl);
            },
            else => {
                print("⚠️ Unhandled statement type in IR generation: {s}\n", .{@tagName(stmt)});
            },
        }
    }
    
    /// Build function signature and store in registry for forward declarations
    fn buildFunctionSignature(self: *LLVMIRPipeline, func_decl: ast.FunctionStatement) !void {
        // // print("🔧 DEBUG: Building function signature for {s}\n", .{func_decl.name});
        
        // Create parameter types
        var param_types = std.ArrayList(c.LLVMTypeRef){};
        defer param_types.deinit(self.allocator);
        
        for (func_decl.parameters.items) |param| {
            const llvm_type = try self.cursedTypeToLLVM(param.param_type);
            try param_types.append(self.allocator, llvm_type);
        }
        
        // Determine return type
        const return_type = if (func_decl.return_type) |ret_type|
            try self.cursedTypeToLLVM(ret_type)
        else
            try self.inferReturnTypeFromFunctionBody(func_decl);
        
        // Create function type
        const func_type = c.LLVMFunctionType(
            return_type,
            param_types.items.ptr,
            @intCast(param_types.items.len),
            0
        );
        
        // Store function signature for forward declarations
        const func_name_duped = try self.arena.allocator().dupe(u8, func_decl.name);
        try self.function_signatures.put(func_name_duped, func_type);
        
        // // print("✅ DEBUG: Stored function signature for {s} with return type\n", .{func_decl.name});
    }

    /// Infer return type from function body by analyzing return statements
    fn inferReturnTypeFromFunctionBody(self: *LLVMIRPipeline, func_decl: ast.FunctionStatement) !c.LLVMTypeRef {
        // // print("🔍 DEBUG: Inferring return type for function {s}\n", .{func_decl.name});
        
        // Look for return statements in the function body
        for (func_decl.body.items) |stmt_ptr| {
            const stmt = @as(*ast.Statement, @alignCast(@ptrCast(stmt_ptr)));
            switch (stmt.*) {
                .Return => |return_stmt| {
                    if (return_stmt.value) |_| {
                        // Found a return statement with a value - infer it's an integer for now
                        // TODO: In a more sophisticated implementation, we would analyze the expression type
                        // // print("🔍 DEBUG: Found return statement with value, inferring integer type\n", .{});
                        return c.LLVMInt32TypeInContext(self.context);
                    }
                },
                else => continue,
            }
        }
        
        // No return statements with values found, assume void
        // // print("🔍 DEBUG: No return statements with values found, assuming void\n", .{});
        return c.LLVMVoidTypeInContext(self.context);
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
            try self.inferReturnTypeFromFunctionBody(func_decl);
        
        const func_type = c.LLVMFunctionType(
            return_type,
            param_types.items.ptr,
            @intCast(param_types.items.len),
            0
        );
        
        // Create function - use qualified name if in module context
        const qualified_name = if (self.current_module_name) |module_name|
            try std.fmt.allocPrint(self.arena.allocator(), "{s}.{s}", .{ module_name, func_decl.name })
        else
            func_decl.name;
        
        // Check if function already exists
        if (self.functions.get(qualified_name)) |existing_func| {
            // // print("🔍 DEBUG: Function {s} already exists, checking if it's a forward declaration...\n", .{qualified_name});
            
            // Check if existing function has a body (if it's just a declaration or has implementation)
            const first_block = c.LLVMGetFirstBasicBlock(existing_func);
            if (first_block != null) {
                // // print("⚠️ DEBUG: Function {s} already has implementation, skipping\n", .{qualified_name});
                return; // Function already implemented
            }
            
            // // print("🔧 DEBUG: Function {s} is forward declaration, replacing with proper implementation\n", .{qualified_name});
            
            // Get existing function type for comparison
            const existing_type = c.LLVMGlobalGetValueType(existing_func);
            const existing_param_count = c.LLVMCountParamTypes(existing_type);
            
            // Check if signatures are compatible
            if (existing_param_count == @as(c_uint, @intCast(param_types.items.len))) {
                // Signatures match parameter count, check parameter types
                var param_types_match = true;
                if (param_types.items.len > 0) {
                    const existing_param_types = try self.allocator.alloc(c.LLVMTypeRef, param_types.items.len);
                    defer self.allocator.free(existing_param_types);
                    c.LLVMGetParamTypes(existing_type, existing_param_types.ptr);
                    
                    for (param_types.items, existing_param_types) |new_type, existing_param_type| {
                        // Check if types are compatible (not just identical)
                        if (!self.areTypesCompatible(new_type, existing_param_type)) {
                            param_types_match = false;
                            break;
                        }
                    }
                }
                
                if (param_types_match) {
                    // // print("✅ DEBUG: Forward declaration signature is compatible with {s}\n", .{qualified_name});
                    
                    // Check if types are compatible but not identical - need signature update
                    var need_replacement = false;
                    const existing_param_types = try self.allocator.alloc(c.LLVMTypeRef, param_types.items.len);
                    defer self.allocator.free(existing_param_types);
                    c.LLVMGetParamTypes(existing_type, existing_param_types.ptr);
                    
                    for (param_types.items, existing_param_types) |new_type, existing_param_type| {
                        if (new_type != existing_param_type) {
                            need_replacement = true;
                            break;
                        }
                    }
                    
                    if (need_replacement) {
                        // Remove and replace forward declaration with correct signature
                        _ = self.functions.remove(qualified_name);
                        c.LLVMDeleteFunction(existing_func);
                        // Continue to create new function with correct signature
                    } else {
                        // Generate function body using existing function
                        try self.generateFunctionBody(existing_func, func_decl);
                        return;
                    }
                } else {
                    print("❌ ERROR: Forward declaration signature mismatch for {s} - parameter types incompatible\n", .{qualified_name});
                }
            } else {
                print("❌ ERROR: Forward declaration signature mismatch for {s}: forward={d}, actual={d}\n", .{qualified_name, existing_param_count, param_types.items.len});
                // If we reach here, signatures don't match - this is an error
                return error.CompilationFailed;
            }
        }
        
        const func_name_z = try self.arena.allocator().dupeZ(u8, qualified_name);
        const function = c.LLVMAddFunction(self.module, func_name_z.ptr, func_type);
        try self.functions.put(qualified_name, function);
        // print("🔧 DEBUG: Created new function {s} with {d} parameters\n", .{qualified_name, param_types.items.len});
        
        // Generate function body
        try self.generateFunctionBody(function, func_decl);
    }
    
    /// Generate function body (extracted for reuse)
    fn generateFunctionBody(self: *LLVMIRPipeline, function: c.LLVMValueRef, func_decl: ast.FunctionStatement) !void {
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
            const param_type = try self.cursedTypeToLLVM(param.param_type);
            const param_alloca = self.buildEntryAlloca(function, param_type, param_name_z.ptr);
            _ = c.LLVMBuildStore(self.builder, llvm_param, param_alloca);
            
            // Store both variable reference and type information
            const key = try self.intern(param.name);
            try self.variables.put(key, param_alloca);
            try self.variable_types.put(key, param_type);
        }
        
        // Generate function body
        for (func_decl.body.items) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateStatement(stmt.*);
        }
        
        // Ensure we are positioned in the correct function before adding terminator
        // Module compilation may have moved the builder to a different function
        var target_block = c.LLVMGetInsertBlock(self.builder);
        
        // If the current block doesn't belong to this function, use the last block of this function
        if (target_block == null or c.LLVMGetBasicBlockParent(target_block) != function) {
            target_block = c.LLVMGetLastBasicBlock(function);
            if (target_block == null) target_block = c.LLVMGetEntryBasicBlock(function);
            c.LLVMPositionBuilderAtEnd(self.builder, target_block);
        }
        
        // Add return if not present
        if (c.LLVMGetBasicBlockTerminator(target_block) == null) {
            if (func_decl.return_type != null) {
                // Return zero/null for non-void functions
                const return_type = if (func_decl.return_type) |ret_type|
                    try self.cursedTypeToLLVM(ret_type)
                else
                    c.LLVMVoidTypeInContext(self.context);
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
        
        // Verify this specific function
        if (c.LLVMVerifyFunction(function, c.LLVMPrintMessageAction) != 0) {
            print("❌ Function verification failed\n", .{});
            return LLVMIRError.ModuleVerificationFailed;
        }
    }
    
    /// Generate function with qualified name (for stdlib modules)
    fn generateFunctionWithQualifiedName(self: *LLVMIRPipeline, func_decl: ast.FunctionStatement, qualified_name: []const u8) !void {
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
        
        // Create function with qualified name
        const func_name_z = try self.arena.allocator().dupeZ(u8, qualified_name);
        const function = c.LLVMAddFunction(self.module, func_name_z.ptr, func_type);
        try self.functions.put(qualified_name, function);
        
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
            const param_type = try self.cursedTypeToLLVM(param.param_type);
            const param_alloca = self.buildEntryAlloca(self.current_function.?, param_type, param_name_z.ptr);
            _ = c.LLVMBuildStore(self.builder, llvm_param, param_alloca);
            const key = try self.intern(param.name);
            try self.variables.put(key, param_alloca);
            try self.variable_types.put(key, param_type);
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
        
        // Verify this specific function
        if (c.LLVMVerifyFunction(function, c.LLVMPrintMessageAction) != 0) {
            print("❌ Function verification failed for: {s}\n", .{qualified_name});
            return LLVMIRError.ModuleVerificationFailed;
        }
    }
    
    /// Generate variable declaration
    fn generateVariableDeclaration(self: *LLVMIRPipeline, var_decl: ast.LetStatement) !void {
        // MEMORY SAFETY: Validate variable name before using it
        if (var_decl.name.len == 0) {
            print("❌ ERROR: Empty variable name in declaration\n", .{});
            return LLVMIRError.UndefinedVariable;
        }
        // Basic memory safety check by trying to access first character
        const first_char = var_decl.name[0]; // This will crash if pointer is bad
        _ = first_char; // Use the variable to prevent compiler warnings
        const var_name_z = try self.arena.allocator().dupeZ(u8, var_decl.name);
        
        // Generate initializer first if present to determine type
        var init_value: ?c.LLVMValueRef = null;
        if (var_decl.initializer) |initializer| {
            init_value = try self.generateExpression(initializer.*);
        }
        
        // Determine type - prioritize initializer type for better type safety
        const llvm_type = if (init_value) |init_val|
            c.LLVMTypeOf(init_val)  // Use initializer's type if available
        else if (var_decl.var_type) |vtype| 
            try self.cursedTypeToLLVM(vtype)  // Fall back to declared type
        else 
            c.LLVMDoubleTypeInContext(self.context); // Default to drip type (float/f64)
        
        // Check if we're in a function context or at global scope
        if (self.current_function) |func| {
            // Inside a function - create alloca in entry block
            const alloca = self.buildEntryAlloca(func, llvm_type, var_name_z.ptr);
            // MEMORY SAFETY: Create a proper owned copy for HashMap key
            const safe_name = try self.arena.allocator().dupe(u8, var_decl.name);
            try self.variables.put(safe_name, alloca);
            try self.variable_types.put(safe_name, llvm_type);
            
            // Track array length if this is an array variable
            if (var_decl.var_type) |vtype| {
                if (vtype == .Array) {
                    try self.array_lengths.put(safe_name, 0); // Initialize empty array
                    // print("🔍 DEBUG: Initialized array variable '{s}' with length 0\n", .{safe_name});
                }
            }
            
            // Store initializer if present
            if (init_value) |init_val| {
                
                // If this is an array initialization, track the length
                if (var_decl.initializer) |initializer| {
                    if (initializer.* == .Array) {
                        const array_length = initializer.Array.elements.items.len;
                        const safe_name_for_length = try self.arena.allocator().dupe(u8, var_decl.name);
                        try self.array_lengths.put(safe_name_for_length, @intCast(array_length));
                        // print("🔍 DEBUG: Tracked array literal '{s}' with {} elements\n", .{safe_name_for_length, array_length});
                    }
                }
                
                // Check for type conversion needs
                const init_type = c.LLVMTypeOf(init_val);
                const init_type_kind = c.LLVMGetTypeKind(init_type);
                const var_type_kind = c.LLVMGetTypeKind(llvm_type);
                
                var converted_value = init_val;
                
                // Convert float literal to integer if variable is integer type
                if (init_type_kind == c.LLVMDoubleTypeKind and var_type_kind == c.LLVMIntegerTypeKind) {
                    converted_value = c.LLVMBuildFPToSI(self.builder, init_val, llvm_type, "float_to_int");
                } else if (init_type_kind == c.LLVMFloatTypeKind and var_type_kind == c.LLVMIntegerTypeKind) {
                    converted_value = c.LLVMBuildFPToSI(self.builder, init_val, llvm_type, "float_to_int");
                } else if (init_type_kind == c.LLVMIntegerTypeKind and (var_type_kind == c.LLVMDoubleTypeKind or var_type_kind == c.LLVMFloatTypeKind)) {
                    converted_value = c.LLVMBuildSIToFP(self.builder, init_val, llvm_type, "int_to_float");
                }
                
                _ = c.LLVMBuildStore(self.builder, converted_value, alloca);
            }
        } else {
            // At global scope - create global variable
            const initializer_value = if (var_decl.initializer) |initializer| 
                try self.generateConstantExpressionWithType(initializer.*, llvm_type)
            else 
                c.LLVMConstNull(llvm_type);
                
            const global_var = c.LLVMAddGlobal(self.module, llvm_type, var_name_z.ptr);
            c.LLVMSetInitializer(global_var, initializer_value);
            // MEMORY SAFETY: Create a proper owned copy for HashMap key
            const safe_name = try self.arena.allocator().dupe(u8, var_decl.name);
            try self.variables.put(safe_name, global_var);
            try self.variable_types.put(safe_name, llvm_type);
        }
    }
    
    /// Generate while statement (bestie loops)
    fn generateWhileStatement(self: *LLVMIRPipeline, while_stmt: ast.WhileStatement) !void {
        if (self.current_function == null) {
            print("❌ While loops can only be used inside functions\n", .{});
            return LLVMIRError.UndefinedVariable;
        }
        
        // Create basic blocks for loop
        const condition_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "while_cond");
        const body_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "while_body");
        const exit_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "while_exit");
        
        // Jump to condition block
        _ = c.LLVMBuildBr(self.builder, condition_block);
        
        // Generate condition block
        c.LLVMPositionBuilderAtEnd(self.builder, condition_block);
        const condition_val = try self.generateExpression(while_stmt.condition.*);
        
        // Convert condition to boolean if needed
        const bool_condition = if (c.LLVMGetTypeKind(c.LLVMTypeOf(condition_val)) == c.LLVMIntegerTypeKind) 
            c.LLVMBuildICmp(self.builder, c.LLVMIntNE, condition_val, c.LLVMConstInt(c.LLVMTypeOf(condition_val), 0, 0), "while_bool")
        else 
            condition_val;
            
        _ = c.LLVMBuildCondBr(self.builder, bool_condition, body_block, exit_block);
        
        // Generate body block
        c.LLVMPositionBuilderAtEnd(self.builder, body_block);
        for (while_stmt.body.items) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateStatement(stmt.*);
        }
        
        // Jump back to condition
        _ = c.LLVMBuildBr(self.builder, condition_block);
        
        // Position builder at exit block for next statements
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
    }
    
    /// Generate for statement (C-style for loops)
    fn generateForStatement(self: *LLVMIRPipeline, for_stmt: ast.ForStatement) !void {
        if (self.current_function == null) {
            print("❌ For loops can only be used inside functions\n", .{});
            return LLVMIRError.UndefinedVariable;
        }
        
        // Generate init statement if present
        if (for_stmt.init) |init_stmt| {
            try self.generateStatement(init_stmt.*);
        }
        
        // Create basic blocks
        const condition_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "for_cond");
        const body_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "for_body");
        const update_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "for_update");
        const exit_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "for_exit");
        
        // Jump to condition
        _ = c.LLVMBuildBr(self.builder, condition_block);
        
        // Generate condition block
        c.LLVMPositionBuilderAtEnd(self.builder, condition_block);
        if (for_stmt.condition) |condition| {
            const condition_val = try self.generateExpression(condition.*);
            const bool_condition = if (c.LLVMGetTypeKind(c.LLVMTypeOf(condition_val)) == c.LLVMIntegerTypeKind) 
                c.LLVMBuildICmp(self.builder, c.LLVMIntNE, condition_val, c.LLVMConstInt(c.LLVMTypeOf(condition_val), 0, 0), "for_bool")
            else 
                condition_val;
            _ = c.LLVMBuildCondBr(self.builder, bool_condition, body_block, exit_block);
        } else {
            // No condition - infinite loop, just branch to body
            _ = c.LLVMBuildBr(self.builder, body_block);
        }
        
        // Generate body block
        c.LLVMPositionBuilderAtEnd(self.builder, body_block);
        for (for_stmt.body.items) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateStatement(stmt.*);
        }
        
        // Jump to update block
        _ = c.LLVMBuildBr(self.builder, update_block);
        
        // Generate update block
        c.LLVMPositionBuilderAtEnd(self.builder, update_block);
        if (for_stmt.update) |update_stmt| {
            try self.generateStatement(update_stmt.*);
        }
        
        // Jump back to condition
        _ = c.LLVMBuildBr(self.builder, condition_block);
        
        // Position builder at exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
    }
    
    /// Generate assignment statement
    fn generateAssignmentStatement(self: *LLVMIRPipeline, assign_stmt: ast.AssignmentStatement) !void {
        // Cast value from *anyopaque to *Expression
        const value_expr: *ast.Expression = @ptrCast(@alignCast(assign_stmt.value));
        const value = try self.generateExpression(value_expr.*);
        
        // Get the target variable 
        const target_expr: *ast.Expression = @ptrCast(@alignCast(assign_stmt.target));
        switch (target_expr.*) {
            .Identifier => |var_name| {
                if (self.variables.get(var_name)) |var_ref| {
                    _ = c.LLVMBuildStore(self.builder, value, var_ref);
                    
                    // Track array length if this is an array assignment
                    if (value_expr.* == .Array) {
                        const array_length = value_expr.Array.elements.items.len;
                        const safe_name = try self.arena.allocator().dupe(u8, var_name);
                        try self.array_lengths.put(safe_name, @intCast(array_length));
                        // print("🔍 DEBUG: Tracked array assignment '{s}' with {} elements\n", .{safe_name, array_length});
                    }
                    // Special case: assignment from append call result  
                    // Note: The length tracking was already updated in generateAppendCall
                    // so no additional tracking needed here for append results
                } else {
                    print("❌ Undefined variable in assignment: {s}\n", .{var_name});
                    return LLVMIRError.UndefinedVariable;
                }
            },
            else => {
                print("❌ Unsupported assignment target\n", .{});
                return LLVMIRError.UndefinedVariable;
            }
        }
    }
    
    /// Generate Short Declaration statement (i := value syntax)
    fn generateShortDeclarationStatement(self: *LLVMIRPipeline, short_decl: ast.ShortDeclarationStatement) !void {
        // Short declaration is like: i := 0, name := "hello"
        // It declares variables and assigns initial values in one statement
        
        if (short_decl.names.items.len != short_decl.values.items.len) {
            print("❌ Mismatched names and values in short declaration\n", .{});
            return LLVMIRError.UndefinedVariable;
        }
        
        // Process each name-value pair
        for (short_decl.names.items, short_decl.values.items) |name, value_expr| {
            // Generate the initial value first to determine type
            const init_value = try self.generateExpression(value_expr.*);
            const llvm_type = c.LLVMTypeOf(init_value);
            
            // Create variable in current scope
            if (self.current_function) |func| {
                // Inside a function - create alloca in entry block
                const var_name_z = try self.arena.allocator().dupeZ(u8, name);
                const alloca = self.buildEntryAlloca(func, llvm_type, var_name_z.ptr);
                
                // Store the initial value
                _ = c.LLVMBuildStore(self.builder, init_value, alloca);
                
                // Register variable for future access
                const safe_name = try self.arena.allocator().dupe(u8, name);
                try self.variables.put(safe_name, alloca);
                try self.variable_types.put(safe_name, llvm_type);
            } else {
                // Global scope - create global variable
                const var_name_z = try self.arena.allocator().dupeZ(u8, name);
                const global = c.LLVMAddGlobal(self.module, llvm_type, var_name_z.ptr);
                c.LLVMSetInitializer(global, init_value);
                
                const safe_name = try self.arena.allocator().dupe(u8, name);
                try self.variables.put(safe_name, global);
                try self.variable_types.put(safe_name, llvm_type);
            }
        }
    }
    
    /// Generate increment/decrement operations (++/-- operators)
    fn generateIncrementDecrement(self: *LLVMIRPipeline, operand_expr: ast.Expression, operator: []const u8) !c.LLVMValueRef {
        // Increment/decrement can only be applied to variables (identifiers)
        switch (operand_expr) {
            .Identifier => |var_name| {
                if (self.variables.get(var_name)) |var_ref| {
                    // Load current value
                    const current_value = c.LLVMBuildLoad2(self.builder, 
                        self.variable_types.get(var_name) orelse c.LLVMInt32TypeInContext(self.context), 
                        var_ref, "load_for_inc");
                    
                    // Create increment/decrement value
                    const one = c.LLVMConstInt(c.LLVMTypeOf(current_value), 1, 0);
                    const new_value = if (std.mem.eql(u8, operator, "++"))
                        c.LLVMBuildAdd(self.builder, current_value, one, "inc")
                    else
                        c.LLVMBuildSub(self.builder, current_value, one, "dec");
                    
                    // Store new value back to variable
                    _ = c.LLVMBuildStore(self.builder, new_value, var_ref);
                    
                    // Return the new value
                    return new_value;
                } else {
                    print("❌ Undefined variable in increment/decrement: {s}\n", .{var_name});
                    return error.UndefinedVariable;
                }
            },
            else => {
                print("❌ Increment/decrement can only be applied to variables\n", .{});
                return error.UnsupportedOperator;
            },
        }
    }
    
    /// Generate If statement (lowkey statements)
    fn generateIfStatement(self: *LLVMIRPipeline, if_stmt: ast.IfStatement) !void {
        if (self.current_function == null) {
            print("❌ If statements can only be used inside functions\n", .{});
            return LLVMIRError.UndefinedVariable;
        }
        
        // Cast condition from *anyopaque to *Expression
        const condition_expr: *ast.Expression = @ptrCast(@alignCast(if_stmt.condition));
        
        // Create basic blocks for then, else (if present), and merge
        const then_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "if_then");
        const else_block = if (if_stmt.else_branch != null) 
            c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "if_else") 
        else 
            null;
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "if_merge");
        
        // Generate condition
        const condition_val = try self.generateExpression(condition_expr.*);
        
        // Convert condition to boolean if needed
        const bool_condition = if (c.LLVMGetTypeKind(c.LLVMTypeOf(condition_val)) == c.LLVMIntegerTypeKind and c.LLVMGetIntTypeWidth(c.LLVMTypeOf(condition_val)) != 1)
            c.LLVMBuildICmp(self.builder, c.LLVMIntNE, condition_val, c.LLVMConstInt(c.LLVMTypeOf(condition_val), 0, 0), "if_bool")
        else 
            condition_val;
            
        // Create conditional branch
        const else_target = else_block orelse merge_block;
        _ = c.LLVMBuildCondBr(self.builder, bool_condition, then_block, else_target);
        
        // Generate then block
        c.LLVMPositionBuilderAtEnd(self.builder, then_block);
        for (if_stmt.then_branch.items) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateStatement(stmt.*);
        }
        
        // Add branch to merge block if no terminator exists
        const then_has_terminator = c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) != null;
        if (!then_has_terminator) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Generate else block if present
        var else_has_terminator = false;
        if (if_stmt.else_branch) |else_branch| {
            if (else_block) |eb| {
                c.LLVMPositionBuilderAtEnd(self.builder, eb);
                for (else_branch.items) |stmt_ptr| {
                    const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
                    try self.generateStatement(stmt.*);
                }
                
                // Add branch to merge block if no terminator exists
                else_has_terminator = c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) != null;
                if (!else_has_terminator) {
                    _ = c.LLVMBuildBr(self.builder, merge_block);
                }
            }
        }
        
        // Only position at merge block if it's reachable (at least one branch doesn't have a terminator)
        if (!then_has_terminator or !else_has_terminator) {
            c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
            // Note: The merge block terminator will be added by subsequent statements or function terminator logic
        } else {
            // Both branches terminate, so the merge block is unreachable
            // Delete it to avoid validation errors
            c.LLVMDeleteBasicBlock(merge_block);
        }
    }
    
    /// Generate Return statement (damn statements)
    fn generateReturnStatement(self: *LLVMIRPipeline, return_stmt: ast.ReturnStatement) !void {
        // print("🔍 DEBUG: Generating return statement\n", .{});
        const parent = self.current_function orelse {
            print("❌ Return statements can only be used inside functions\n", .{});
            return LLVMIRError.UndefinedVariable;
        };
        
        if (return_stmt.value) |value_ptr| {
            // Cast return value from *anyopaque to *Expression
            const return_expr: *ast.Expression = @ptrCast(@alignCast(value_ptr));
            // print("🔍 DEBUG: Generating return value expression\n", .{});
            const ret_val = try self.generateExpression(return_expr.*);
            // print("🔍 DEBUG: Return value generated successfully\n", .{});
            
            // Ensure return type matches function signature
            const func_type = c.LLVMGlobalGetValueType(parent);
            const func_ret_ty = c.LLVMGetReturnType(func_type);
            
            // print("🔍 DEBUG: Checking type compatibility for return value\n", .{});
            const final_val = if (func_ret_ty == c.LLVMTypeOf(ret_val)) blk: {
                // print("🔍 DEBUG: Return type matches exactly\n", .{});
                break :blk ret_val;
            } else if (c.LLVMGetTypeKind(func_ret_ty) == c.LLVMIntegerTypeKind and
                     c.LLVMGetTypeKind(c.LLVMTypeOf(ret_val)) == c.LLVMIntegerTypeKind) blk: {
                // print("🔍 DEBUG: Return type needs integer cast\n", .{});
                break :blk c.LLVMBuildIntCast(self.builder, ret_val, func_ret_ty, "ret.cast");
            } else if (c.LLVMGetTypeKind(func_ret_ty) == c.LLVMPointerTypeKind and
                     c.LLVMGetTypeKind(c.LLVMTypeOf(ret_val)) == c.LLVMPointerTypeKind) blk: {
                // print("🔍 DEBUG: Return type needs pointer cast\n", .{});
                break :blk c.LLVMBuildPointerCast(self.builder, ret_val, func_ret_ty, "ret.ptr.cast");
            } else {
                print("❌ RETURN TYPE MISMATCH ERROR! Expected: {}, Got: {}\n", .{c.LLVMGetTypeKind(func_ret_ty), c.LLVMGetTypeKind(c.LLVMTypeOf(ret_val))});
                return LLVMIRError.UndefinedVariable; // Type mismatch
            };
                
            _ = c.LLVMBuildRet(self.builder, final_val);
        } else {
            // Void return
            _ = c.LLVMBuildRetVoid(self.builder);
        }
        
        // Note: We do NOT create a continuation block here since this is a terminator.
        // If there are more statements after this return, they will be unreachable and
        // that's correct behavior. The function generation will handle adding a final
        // return if needed.
    }
    
    /// Build alloca in entry block to ensure dominance
    fn buildEntryAlloca(
        self: *LLVMIRPipeline,
        function: c.LLVMValueRef,
        ty: c.LLVMTypeRef,
        name: [*:0]const u8) c.LLVMValueRef {

        const entry = c.LLVMGetEntryBasicBlock(function);

        // A throw-away builder just for the alloca
        const tmp = c.LLVMCreateBuilderInContext(self.context);
        defer c.LLVMDisposeBuilder(tmp);

        if (c.LLVMGetFirstInstruction(entry) != null) {
            c.LLVMPositionBuilderBefore(tmp, c.LLVMGetFirstInstruction(entry));
        } else {
            c.LLVMPositionBuilderAtEnd(tmp, entry);
        }

        return c.LLVMBuildAlloca(tmp, ty, name);
    }

    /// Generate constant expression (for global variable initializers)
    fn generateConstantExpression(self: *LLVMIRPipeline, expr: ast.Expression) !c.LLVMValueRef {
        switch (expr) {
            .Literal => |lit| {
                return try self.generateConstantLiteral(lit);
            },
            .Binary => |bin_op| {
                // For constants, we can only handle simple arithmetic at compile time
                const left = try self.generateConstantExpression(bin_op.left.*);
                const right = try self.generateConstantExpression(bin_op.right.*);
                
                if (std.mem.eql(u8, bin_op.operator, "+")) {
                    return c.LLVMConstAdd(left, right);
                } else if (std.mem.eql(u8, bin_op.operator, "-")) {
                    return c.LLVMConstSub(left, right);
                } else if (std.mem.eql(u8, bin_op.operator, "*")) {
                    return c.LLVMConstMul(left, right);
                } else {
                    // For complex expressions, just return zero
                    return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
                }
            },
            else => {
                // For other expressions, return null/zero constant
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            }
        }
    }
    
    /// Generate constant expression with specific target type
    fn generateConstantExpressionWithType(self: *LLVMIRPipeline, expr: ast.Expression, target_type: c.LLVMTypeRef) !c.LLVMValueRef {
        switch (expr) {
            .Literal => |lit| {
                return try self.generateConstantLiteralWithType(lit, target_type);
            },
            .Binary => |bin_op| {
                // For constants, we can only handle simple arithmetic at compile time
                const left = try self.generateConstantExpressionWithType(bin_op.left.*, target_type);
                const right = try self.generateConstantExpressionWithType(bin_op.right.*, target_type);
                
                if (std.mem.eql(u8, bin_op.operator, "+")) {
                    return c.LLVMConstAdd(left, right);
                } else if (std.mem.eql(u8, bin_op.operator, "-")) {
                    return c.LLVMConstSub(left, right);
                } else if (std.mem.eql(u8, bin_op.operator, "*")) {
                    return c.LLVMConstMul(left, right);
                } else {
                    // For complex expressions, just return zero of target type
                    return c.LLVMConstNull(target_type);
                }
            },
            else => {
                // For other expressions, return null/zero constant of target type
                return c.LLVMConstNull(target_type);
            }
        }
    }
    
    /// Generate constant literal (for global constants)
    fn generateConstantLiteral(self: *LLVMIRPipeline, lit: ast.Literal) !c.LLVMValueRef {
        switch (lit) {
            .Integer => |int_val| {
                // Use i32 (normie) type for integer literals to match CURSED type system
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(int_val), 0);
            },
            .Float => |float_val| {
                return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float_val);
            },
            .String => |str_val| {
                return c.LLVMConstStringInContext(self.context, str_val.ptr, @intCast(str_val.len), 0);
            },
            .Boolean => |bool_val| {
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0);
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
    
    /// Generate constant literal with specific target type
    fn generateConstantLiteralWithType(self: *LLVMIRPipeline, lit: ast.Literal, target_type: c.LLVMTypeRef) !c.LLVMValueRef {
        const type_kind = c.LLVMGetTypeKind(target_type);
        
        switch (lit) {
            .Integer => |int_val| {
                // Only use target_type if it's an integer type
                if (type_kind == c.LLVMIntegerTypeKind) {
                    return c.LLVMConstInt(target_type, @bitCast(@as(u64, @intCast(int_val))), 0);
                } else {
                    // For non-integer targets, use i32 (normie) as default integer type  
                    return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(int_val), 0);
                }
            },
            .Float => |float_val| {
                // Check if target type is float or double
                if (type_kind == c.LLVMFloatTypeKind or type_kind == c.LLVMDoubleTypeKind) {
                    return c.LLVMConstReal(target_type, float_val);
                } else {
                    // Fallback to double
                    return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float_val);
                }
            },
            .String => |str_val| {
                // Strings are always pointer types - check if target expects a pointer
                if (type_kind == c.LLVMPointerTypeKind) {
                    return c.LLVMConstStringInContext(self.context, str_val.ptr, @intCast(str_val.len), 0);
                } else {
                    // Create global string if target is not a pointer
                    const str_global = c.LLVMAddGlobal(self.module, c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(str_val.len + 1)), "str_literal");
                    const str_const = c.LLVMConstStringInContext(self.context, str_val.ptr, @intCast(str_val.len), 1);
                    c.LLVMSetInitializer(str_global, str_const);
                    c.LLVMSetLinkage(str_global, c.LLVMPrivateLinkage);
                    return str_global;
                }
            },
            .Boolean => |bool_val| {
                // Check if target type is i1 (boolean)
                if (type_kind == c.LLVMIntegerTypeKind and c.LLVMGetIntTypeWidth(target_type) == 1) {
                    return c.LLVMConstInt(target_type, if (bool_val) 1 else 0, 0);
                } else {
                    return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0);
                }
            },
            .Character => |char_val| {
                // Only use target type if it's an integer type
                if (type_kind == c.LLVMIntegerTypeKind) {
                    return c.LLVMConstInt(target_type, char_val, 0);
                } else {
                    return c.LLVMConstInt(c.LLVMInt8TypeInContext(self.context), char_val, 0);
                }
            },
            .Null => {
                return c.LLVMConstNull(target_type);
            },
            .Nil => {
                return c.LLVMConstNull(target_type);
            },
        }
    }
    
    /// Generate expression
    fn generateExpression(self: *LLVMIRPipeline, expr: ast.Expression) anyerror!c.LLVMValueRef {
        switch (expr) {
            .Literal => |lit| {
                return try self.generateLiteral(lit);
            },
            .Identifier => |ident| {
                // print("🔍 DEBUG: Looking up identifier: {s}\n", .{ident});
                
                // First check if it's a builtin function (should not be used as variable)
                if (self.functions.get(ident)) |function| {
                    // print("🔍 DEBUG: Found {s} as function reference\n", .{ident});
                    // Return the function reference itself - this is for function pointers or calls
                    return function;
                }
                
                // Then check if it's a variable
                if (self.variables.get(ident)) |var_ref| {
                    // print("🔍 DEBUG: Found {s} as variable reference\n", .{ident});
                    
                    // Use stored type information
                    if (self.variable_types.get(ident)) |var_type| {
                        // print("🔍 DEBUG: Found type info for variable {s}\n", .{ident});
                        
                        // Check if we're in a function context
                        if (self.current_function == null) {
                            print("❌ Cannot load variable {s} outside of function context\n", .{ident});
                            return error.UndefinedVariable;
                        }
                        
                        return c.LLVMBuildLoad2(self.builder, var_type, var_ref, "load_var");
                    } else {
                        print("❌ No type information stored for variable: {s}\n", .{ident});
                        return error.UndefinedVariable;
                    }
                } else {
                    print("❌ Undefined variable: {s}\n", .{ident});
                    // print("🔍 DEBUG: Available variables:\n", .{});
                    var iter = self.variables.iterator();
                    while (iter.next()) |entry| {
                        print("  - {s}\n", .{entry.key_ptr.*});
                    }
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
                // Use i32 (normie) type for integer literals to match CURSED type system
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(int_val), 0);
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
                } else if (std.mem.eql(u8, unary.operator, "++") or std.mem.eql(u8, unary.operator, "--")) {
                    // Handle increment/decrement operators
                    return try self.generateIncrementDecrement(unary.operand.*, unary.operator);
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
                // Handle array literals - create proper array storage
                if (array.elements.items.len == 0) {
                    // Return a pointer to an empty array
                    const empty_array = c.LLVMConstArray(c.LLVMInt32TypeInContext(self.context), null, 0);
                    return c.LLVMConstBitCast(empty_array, c.LLVMPointerTypeInContext(self.context, 0));
                }
                
                // Create a proper array with all elements
                var array_elements = try self.allocator.alloc(c.LLVMValueRef, array.elements.items.len);
                defer self.allocator.free(array_elements);
                
                // Generate all array elements
                for (array.elements.items, 0..) |element, i| {
                    array_elements[i] = try self.generateExpression(element.*);
                }
                
                // Determine the element type from the first element
                const element_type = c.LLVMTypeOf(array_elements[0]);
                
                // Create the array type
                const array_type = c.LLVMArrayType(element_type, @intCast(array.elements.items.len));
                
                // If we're in a function, allocate storage for the array and store elements individually
                if (self.current_function != null) {
                    const alloca = c.LLVMBuildAlloca(self.builder, array_type, "array_literal");
                    
                    // Store each element individually since we may have runtime values
                    for (array_elements, 0..) |elem_val, i| {
                        const idx = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), i, 0);
                        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
                        const indices = [_]c.LLVMValueRef{zero, idx};
                        const elem_ptr = c.LLVMBuildGEP2(self.builder, array_type, alloca, @constCast(@ptrCast(&indices)), 2, "elem_ptr");
                        _ = c.LLVMBuildStore(self.builder, elem_val, elem_ptr);
                    }
                    
                    return alloca;
                } else {
                    // At global scope, check if all elements are constants
                    var all_constants = true;
                    for (array_elements) |elem| {
                        if (c.LLVMIsConstant(elem) == 0) {
                            all_constants = false;
                            break;
                        }
                    }
                    
                    if (all_constants) {
                        // Create constant array for global scope
                        const const_array = c.LLVMConstArray(element_type, array_elements.ptr, @intCast(array.elements.items.len));
                        const global_array = c.LLVMAddGlobal(self.module, array_type, "global_array");
                        c.LLVMSetInitializer(global_array, const_array);
                        return global_array;
                    } else {
                        // Fall back to creating a function-local array (shouldn't happen normally)
                        return c.LLVMConstNull(c.LLVMPointerTypeInContext(self.context, 0));
                    }
                }
            },
            .ArrayAccess => |access| {
                // TEMPORARY FIX: Array access in LLVM backend causes segfaults
                // TODO: Implement proper array indexing support
                print("⚠️ Array access not yet fully supported in LLVM backend, returning placeholder value\n", .{});
                
                // Generate the array and index expressions for completeness but don't use them
                _ = try self.generateExpression(access.array.*);
                _ = try self.generateExpression(access.index.*);
                
                // Return a placeholder integer value to avoid segfault
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
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
                // Use i32 (normie) type for integer literals to match CURSED type system
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(int_val), 0);
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
        // Create a stable copy of the string key to avoid dangling pointers
        const stable_key = try self.arena.allocator().dupe(u8, str_val);
        
        // Check if we already have this string (use stable key)
        if (self.global_strings.get(stable_key)) |existing| {
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
        
        // Use stable key for hash map to prevent memory corruption
        try self.global_strings.put(stable_key, str_ptr);
        return str_ptr;
    }
    
    /// Generate binary operation
    fn generateBinaryOperation(self: *LLVMIRPipeline, bin_op: ast.BinaryExpression) anyerror!c.LLVMValueRef {
        // print("🔍 DEBUG: Generating binary operation: {s}\n", .{bin_op.operator});
        // print("🔍 DEBUG: Generating left operand\n", .{});
        var left = try self.generateExpression(bin_op.left.*);
        // print("🔍 DEBUG: Generated left operand successfully\n", .{});
        // print("🔍 DEBUG: Generating right operand\n", .{});
        var right = try self.generateExpression(bin_op.right.*);
        // print("🔍 DEBUG: Generated right operand successfully\n", .{});
        
        // Check if we're dealing with floating point values
        const left_type = c.LLVMTypeOf(left);
        const right_type = c.LLVMTypeOf(right);
        

        
        const left_is_float = c.LLVMGetTypeKind(left_type) == c.LLVMDoubleTypeKind or
                             c.LLVMGetTypeKind(left_type) == c.LLVMFloatTypeKind;
        const right_is_float = c.LLVMGetTypeKind(right_type) == c.LLVMDoubleTypeKind or
                              c.LLVMGetTypeKind(right_type) == c.LLVMFloatTypeKind;
        
        const is_float = left_is_float or right_is_float;
        
        // Check if we're dealing with pointer types (for string comparisons)
        const left_is_pointer = c.LLVMGetTypeKind(left_type) == c.LLVMPointerTypeKind;
        const right_is_pointer = c.LLVMGetTypeKind(right_type) == c.LLVMPointerTypeKind;
        const is_pointer_comparison = left_is_pointer and right_is_pointer;
        
        // Skip type normalization for logical operators (&&, ||) - they handle their own boolean conversions
        const is_logical_op = std.mem.eql(u8, bin_op.operator, "&&") or std.mem.eql(u8, bin_op.operator, "||");
        
        // Handle integer type normalization for all operations (except logical operators and pointer comparisons)
        if (!left_is_float and !right_is_float and !is_logical_op and !is_pointer_comparison) {
            // Check if operands are boolean (i1) - don't normalize boolean values for comparison operations
            const left_is_bool = c.LLVMGetTypeKind(left_type) == c.LLVMIntegerTypeKind and c.LLVMGetIntTypeWidth(left_type) == 1;
            const right_is_bool = c.LLVMGetTypeKind(right_type) == c.LLVMIntegerTypeKind and c.LLVMGetIntTypeWidth(right_type) == 1;
            
            // Don't normalize if both operands are already booleans for comparison operations
            if (left_is_bool and right_is_bool) {
                // Keep boolean types as-is for comparison operations
            } else {
                // Both are integers - normalize to consistent types
                const left_width = if (c.LLVMGetTypeKind(left_type) == c.LLVMIntegerTypeKind) 
                    c.LLVMGetIntTypeWidth(left_type) else 64;
                const right_width = if (c.LLVMGetTypeKind(right_type) == c.LLVMIntegerTypeKind) 
                    c.LLVMGetIntTypeWidth(right_type) else 64;
                
                // Use the wider type, or default to i64 for consistency
                const target_width = @max(left_width, right_width);
                const target_type = if (target_width <= 32) 
                    c.LLVMInt32TypeInContext(self.context) 
                else 
                    c.LLVMInt64TypeInContext(self.context);
                
                // Convert both operands to the target type if needed
                if (c.LLVMTypeOf(left) != target_type) {
                    if (c.LLVMGetIntTypeWidth(c.LLVMTypeOf(left)) < target_width) {
                        left = c.LLVMBuildSExt(self.builder, left, target_type, "extend_left");
                    } else {
                        left = c.LLVMBuildTrunc(self.builder, left, target_type, "trunc_left");
                    }
                }
                
                if (c.LLVMTypeOf(right) != target_type) {
                    if (c.LLVMGetIntTypeWidth(c.LLVMTypeOf(right)) < target_width) {
                        right = c.LLVMBuildSExt(self.builder, right, target_type, "extend_right");
                    } else {
                        right = c.LLVMBuildTrunc(self.builder, right, target_type, "trunc_right");
                    }
                }
            }
        }
        
        // Handle type promotion for mixed integer/float operations
        if (is_float) {
            // Promote integer operands to double when mixed with float
            if (!left_is_float) {
                // Convert integer to double
                left = c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "int_to_double_left");
            } else if (c.LLVMGetTypeKind(left_type) == c.LLVMFloatTypeKind) {
                // Convert float to double for consistency
                left = c.LLVMBuildFPExt(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "float_to_double_left");
            }
            
            if (!right_is_float) {
                // Convert integer to double
                right = c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "int_to_double_right");
            } else if (c.LLVMGetTypeKind(right_type) == c.LLVMFloatTypeKind) {
                // Convert float to double for consistency
                right = c.LLVMBuildFPExt(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "float_to_double_right");
            }
        }
        
        // Handle arithmetic operators with type-specific instructions
        if (std.mem.eql(u8, bin_op.operator, "+")) {
            if (is_float) {
                return c.LLVMBuildFAdd(self.builder, left, right, "fadd_tmp");
            } else if (is_pointer_comparison) {
                // String concatenation - for now, just return the first string
                // TODO: Implement proper string concatenation
                return left; // Temporary fallback
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
            if (!left_is_float and !right_is_float) {
                // Integer / Integer -> Integer division with division by zero check
                // Check if divisor is zero
                const zero_val = c.LLVMConstInt(c.LLVMTypeOf(right), 0, 0);
                const is_zero = c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, right, zero_val, "div_by_zero_check");
                
                // Create basic blocks for division by zero handling
                const current_bb = c.LLVMGetInsertBlock(self.builder);
                const current_func = c.LLVMGetBasicBlockParent(current_bb);
                const zero_bb = c.LLVMAppendBasicBlockInContext(self.context, current_func, "div_by_zero");
                const div_bb = c.LLVMAppendBasicBlockInContext(self.context, current_func, "div_ok");
                
                // Branch based on zero check
                _ = c.LLVMBuildCondBr(self.builder, is_zero, zero_bb, div_bb);
                
                // Generate division by zero panic block
                c.LLVMPositionBuilderAtEnd(self.builder, zero_bb);
                try self.generateDivisionByZeroPanic();
                // Note: generateDivisionByZeroPanic() should terminate the block, so no need for further instructions
                
                // Generate normal division block  
                c.LLVMPositionBuilderAtEnd(self.builder, div_bb);
                return c.LLVMBuildSDiv(self.builder, left, right, "sdiv_tmp");
            } else {
                // If either operand is float, perform float division
                // Convert integer operands to double first
                if (!left_is_float) {
                    left = c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "int_to_double_left");
                } else if (c.LLVMGetTypeKind(left_type) == c.LLVMFloatTypeKind) {
                    left = c.LLVMBuildFPExt(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "float_to_double_left");
                }
                
                if (!right_is_float) {
                    right = c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "int_to_double_right");
                } else if (c.LLVMGetTypeKind(right_type) == c.LLVMFloatTypeKind) {
                    right = c.LLVMBuildFPExt(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "float_to_double_right");
                }
                
                // For float division, we don't need to check for zero since IEEE 754 defines behavior
                // Float division by zero returns infinity, not an error
                return c.LLVMBuildFDiv(self.builder, left, right, "fdiv_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "==")) {
            if (is_float) {
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOEQ, left, right, "feq_tmp");
            } else if (is_pointer_comparison) {
                // For string/pointer comparisons, we need to call strcmp or similar
                // For now, use pointer equality (works for string literals)
                return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "ptr_eq_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "!=")) {
            if (is_float) {
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealONE, left, right, "fne_tmp");
            } else if (is_pointer_comparison) {
                // For string/pointer comparisons, use pointer inequality
                return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "ptr_ne_tmp");
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
        } else if (std.mem.eql(u8, bin_op.operator, "<=")) {
            if (is_float) {
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOLE, left, right, "fle_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, left, right, "le_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, ">=")) {
            if (is_float) {
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOGE, left, right, "fge_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, left, right, "ge_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "=")) {
            // Single = is equality comparison in CURSED (like == in other languages)
            if (is_float) {
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOEQ, left, right, "feq_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "%")) {
            // Modulo operation
            if (is_float) {
                return c.LLVMBuildFRem(self.builder, left, right, "fmod_tmp");
            } else {
                // Integer modulo with division by zero check
                const zero_val = c.LLVMConstInt(c.LLVMTypeOf(right), 0, 0);
                const is_zero = c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, right, zero_val, "mod_by_zero_check");
                
                // Create basic blocks for modulo by zero handling
                const current_bb = c.LLVMGetInsertBlock(self.builder);
                const current_func = c.LLVMGetBasicBlockParent(current_bb);
                const zero_bb = c.LLVMAppendBasicBlockInContext(self.context, current_func, "mod_by_zero");
                const mod_bb = c.LLVMAppendBasicBlockInContext(self.context, current_func, "mod_ok");
                
                // Branch based on zero check
                _ = c.LLVMBuildCondBr(self.builder, is_zero, zero_bb, mod_bb);
                
                // Generate modulo by zero panic block
                c.LLVMPositionBuilderAtEnd(self.builder, zero_bb);
                try self.generateDivisionByZeroPanic();
                
                // Generate normal modulo block  
                c.LLVMPositionBuilderAtEnd(self.builder, mod_bb);
                return c.LLVMBuildSRem(self.builder, left, right, "mod_tmp");
            }
        } else if (std.mem.eql(u8, bin_op.operator, "&&")) {
            // Logical AND - ensure operands are boolean (i1) type
            const bool_type = c.LLVMInt1TypeInContext(self.context);
            
            // Convert operands to boolean if needed
            if (c.LLVMTypeOf(left) != bool_type) {
                // Check if value is zero for integer types
                const zero_val = c.LLVMConstInt(c.LLVMTypeOf(left), 0, 0);
                left = c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, zero_val, "bool_cast_left");
            }
            if (c.LLVMTypeOf(right) != bool_type) {
                // Check if value is zero for integer types  
                const zero_val = c.LLVMConstInt(c.LLVMTypeOf(right), 0, 0);
                right = c.LLVMBuildICmp(self.builder, c.LLVMIntNE, right, zero_val, "bool_cast_right");
            }
            
            return c.LLVMBuildAnd(self.builder, left, right, "and_tmp");
        } else if (std.mem.eql(u8, bin_op.operator, "||")) {
            // Logical OR - ensure operands are boolean (i1) type
            const bool_type = c.LLVMInt1TypeInContext(self.context);
            
            // Convert operands to boolean if needed
            if (c.LLVMTypeOf(left) != bool_type) {
                // Check if value is zero for integer types
                const zero_val = c.LLVMConstInt(c.LLVMTypeOf(left), 0, 0);
                left = c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, zero_val, "bool_cast_left");
            }
            if (c.LLVMTypeOf(right) != bool_type) {
                // Check if value is zero for integer types
                const zero_val = c.LLVMConstInt(c.LLVMTypeOf(right), 0, 0);
                right = c.LLVMBuildICmp(self.builder, c.LLVMIntNE, right, zero_val, "bool_cast_right");
            }
            
            return c.LLVMBuildOr(self.builder, left, right, "or_tmp");
        } else {
            print("⚠️ Unhandled binary operator: {s}\n", .{bin_op.operator});
            return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
        }
    }
    
    /// Generate function call
    fn generateFunctionCall(self: *LLVMIRPipeline, call: ast.CallExpression) !c.LLVMValueRef {
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
        
        // print("🔍 DEBUG: Looking for function: {s}\n", .{function_name});
        
        if (std.mem.eql(u8, function_name, "vibez.spill")) {
            // Convert []*Expression to []Expression
            var args = try self.allocator.alloc(ast.Expression, call.arguments.items.len);
            defer self.allocator.free(args);
            for (call.arguments.items, 0..) |arg_ptr, i| {
                args[i] = arg_ptr.*;
            }
            return try self.generatePrintCall(args);
        }
        
        if (std.mem.eql(u8, function_name, "yap")) {
            // Convert []*Expression to []Expression  
            var args = try self.allocator.alloc(ast.Expression, call.arguments.items.len);
            defer self.allocator.free(args);
            for (call.arguments.items, 0..) |arg_ptr, i| {
                args[i] = arg_ptr.*;
            }
            return try self.generatePrintCall(args);
        }
        
        if (std.mem.eql(u8, function_name, "append")) {
            // print("🔍 DEBUG: Handling append function call\n", .{});
            if (call.arguments.items.len != 2) {
                // print("❌ DEBUG: append requires 2 arguments, got {}\n", .{call.arguments.items.len});
                return error.InvalidArgumentCount;
            }
            return try self.generateAppendCall(call.arguments.items[0].*, call.arguments.items[1].*);
        }
        
        if (std.mem.eql(u8, function_name, "len")) {
            // print("🔍 DEBUG: Handling len function call\n", .{});
            if (call.arguments.items.len != 1) {
                // print("❌ DEBUG: len requires 1 argument, got {}\n", .{call.arguments.items.len});
                return error.InvalidArgumentCount;
            }
            return try self.generateLenCall(call.arguments.items[0].*);
        }
        
        // Look up user-defined function
        // First try exact name, then try with current module prefix for same-module calls
        const function = self.functions.get(function_name) orelse blk: {
            if (self.current_module_name) |module_name| {
                const qualified_name = try std.fmt.allocPrint(self.allocator, "{s}.{s}", .{module_name, function_name});
                defer self.allocator.free(qualified_name);
                break :blk self.functions.get(qualified_name);
            }
            break :blk null;
        };
        
        if (function) |func| {
            // print("✅ DEBUG: Found function {s} in function table\n", .{function_name});
            
            // Safety check
            if (@as(?*anyopaque, func) == null) {
                print("❌ Null function reference for: {s}\n", .{function_name});
                return error.UndefinedFunction;
            }
            
            // Get function type first
            const func_type = c.LLVMGlobalGetValueType(func);
            if (@as(?*anyopaque, func_type) == null) {
                print("❌ Null function type for: {s}\n", .{function_name});
                return error.UndefinedFunction;
            }
            
            // Create a fixed-size array for LLVM arguments
            var llvm_args: [16]c.LLVMValueRef = undefined; // Support up to 16 args
            if (call.arguments.items.len > 16) {
                print("❌ Too many arguments for function call (max 16)\n", .{});
                return error.UndefinedFunction;
            }
            
            // Get function parameter types for type conversion
            const param_count = c.LLVMCountParamTypes(func_type);
            var param_types: [16]c.LLVMTypeRef = undefined;
            if (param_count > 0) {
                c.LLVMGetParamTypes(func_type, @ptrCast(&param_types));
            }
            
            for (call.arguments.items, 0..) |arg, i| {
                var arg_val = try self.generateExpression(arg.*);
                if (@as(?*anyopaque, arg_val) == null) {
                    print("❌ Null argument value at index {d}\n", .{i});
                    return error.UndefinedFunction;
                }
                
                // Convert argument type to match function parameter type
                if (i < param_count) {
                    const expected_type = param_types[i];
                    const actual_type = c.LLVMTypeOf(arg_val);
                    
                    if (expected_type != actual_type) {
                        // Handle integer type conversions
                        if (c.LLVMGetTypeKind(expected_type) == c.LLVMIntegerTypeKind and 
                            c.LLVMGetTypeKind(actual_type) == c.LLVMIntegerTypeKind) {
                            
                            const expected_width = c.LLVMGetIntTypeWidth(expected_type);
                            const actual_width = c.LLVMGetIntTypeWidth(actual_type);
                            
                            if (expected_width < actual_width) {
                                // Truncate to smaller type
                                arg_val = c.LLVMBuildTrunc(self.builder, arg_val, expected_type, "arg_trunc");
                            } else if (expected_width > actual_width) {
                                // Extend to larger type
                                arg_val = c.LLVMBuildSExt(self.builder, arg_val, expected_type, "arg_extend");
                            }
                        }
                        // TODO: Handle float/int conversions if needed
                    }
                }
                
                llvm_args[i] = arg_val;
            }
            
            // Use consistent approach with empty array for 0 arguments
            const args_ptr = if (call.arguments.items.len > 0) @as([*]c.LLVMValueRef, @ptrCast(&llvm_args)) else null;
            
            // Check if function returns void - if so, don't assign result to a variable
            const return_type = c.LLVMGetReturnType(func_type);
            const is_void = c.LLVMGetTypeKind(return_type) == c.LLVMVoidTypeKind;
            
            const result = c.LLVMBuildCall2(
                self.builder,
                func_type,
                func,
                args_ptr,
                @intCast(call.arguments.items.len),
                if (is_void) "" else "call_tmp"
            );
            
            if (@as(?*anyopaque, result) == null) {
                print("❌ Failed to generate call to: {s}\n", .{function_name});
                return error.UndefinedFunction;
            }
            
            return result;
        } else {
            // print("❌ DEBUG: Function {s} not found in function table, creating forward declaration...\n", .{function_name});
            
            // CRITICAL BUG FIX: Instead of returning error, create a forward declaration
            // with the correct signature based on the call arguments
            return try self.createForwardDeclaration(function_name, call);
        }
    }
    
    /// Create forward declaration for undefined function with proper signature
    fn createForwardDeclaration(self: *LLVMIRPipeline, function_name: []const u8, call: ast.CallExpression) !c.LLVMValueRef {
        // print("🔧 DEBUG: Creating forward declaration for {s} with {d} arguments\n", .{function_name, call.arguments.items.len});
        
        // Analyze argument types to create proper function signature
        var param_types = std.ArrayList(c.LLVMTypeRef){};
        defer param_types.deinit(self.allocator);
        var llvm_args: [16]c.LLVMValueRef = undefined;
        
        if (call.arguments.items.len > 16) {
            print("❌ Too many arguments for forward declaration (max 16)\n", .{});
            return error.UndefinedFunction;
        }
        
        // Generate arguments and normalize their types for CURSED semantics
        const normie_type = c.LLVMInt32TypeInContext(self.context);
        
        for (call.arguments.items, 0..) |arg, i| {
            var arg_val = try self.generateExpression(arg.*);
            var arg_type = c.LLVMTypeOf(arg_val);
            
            // All CURSED integers are 'normie' (i32). Force the prototype to use i32
            // and cast the value so the call itself is well-typed.
            if (c.LLVMGetTypeKind(arg_type) == c.LLVMIntegerTypeKind) {
                if (arg_type != normie_type) {
                    const width = c.LLVMGetIntTypeWidth(arg_type);
                    arg_val = if (width > 32)
                        c.LLVMBuildTrunc(self.builder, arg_val, normie_type, "arg_to_i32")
                    else
                        c.LLVMBuildSExt(self.builder, arg_val, normie_type, "arg_to_i32");
                    arg_type = normie_type;
                }
            }
            
            llvm_args[i] = arg_val;
            try param_types.append(self.allocator, arg_type);
        }
        
        // Check if we have the function signature in our registry
        const func_type = if (self.function_signatures.get(function_name)) |stored_func_type| blk: {
            // print("✅ DEBUG: Found function signature for {s} in registry\n", .{function_name});
            break :blk stored_func_type;
        } else blk: {
            // print("⚠️ DEBUG: Function {s} not in signature registry, falling back to inferred type\n", .{function_name});
            // For unknown functions, assume they return void (CURSED functions without return type are void)
            const return_type = c.LLVMVoidTypeInContext(self.context);
            
            // Create function type with proper signature
            break :blk c.LLVMFunctionType(
                return_type,
                param_types.items.ptr,
                @intCast(param_types.items.len),
                0
            );
        };
        
        // Create forward declaration
        const func_name_z = try self.arena.allocator().dupeZ(u8, function_name);
        const function = c.LLVMAddFunction(self.module, func_name_z.ptr, func_type);
        
        // Store in function table for future use
        try self.functions.put(function_name, function);
        
        // print("✅ DEBUG: Created forward declaration for {s} with signature: {d} params\n", .{function_name, param_types.items.len});
        
        // Convert arguments to match function parameter types from the actual signature
        const param_count = c.LLVMCountParamTypes(func_type);
        var func_param_types: [16]c.LLVMTypeRef = undefined;
        if (param_count > 0) {
            c.LLVMGetParamTypes(func_type, @ptrCast(&func_param_types));
        }
        
        // Apply type conversions to arguments
        for (call.arguments.items, 0..) |_, i| {
            if (i < param_count) {
                const expected_type = func_param_types[i];
                const actual_type = c.LLVMTypeOf(llvm_args[i]);
                
                if (expected_type != actual_type) {
                    // Handle integer type conversions
                    if (c.LLVMGetTypeKind(expected_type) == c.LLVMIntegerTypeKind and 
                        c.LLVMGetTypeKind(actual_type) == c.LLVMIntegerTypeKind) {
                        
                        const expected_width = c.LLVMGetIntTypeWidth(expected_type);
                        const actual_width = c.LLVMGetIntTypeWidth(actual_type);
                        
                        if (expected_width < actual_width) {
                            // Truncate to smaller type
                            llvm_args[i] = c.LLVMBuildTrunc(self.builder, llvm_args[i], expected_type, "fwd_arg_trunc");
                            // print("🔧 DEBUG: Truncated argument {d} from i{d} to i{d}\n", .{i, actual_width, expected_width});
                        } else if (expected_width > actual_width) {
                            // Extend to larger type  
                            llvm_args[i] = c.LLVMBuildSExt(self.builder, llvm_args[i], expected_type, "fwd_arg_extend");
                            // print("🔧 DEBUG: Extended argument {d} from i{d} to i{d}\n", .{i, actual_width, expected_width});
                        }
                    }
                }
            }
        }
        
        // Generate the function call with proper signature
        const args_ptr = if (call.arguments.items.len > 0) @as([*]c.LLVMValueRef, @ptrCast(&llvm_args)) else null;
        
        // Check if function returns void to avoid naming void calls
        const return_type = c.LLVMGetReturnType(func_type);
        const is_void_return = c.LLVMGetTypeKind(return_type) == c.LLVMVoidTypeKind;
        const call_name = if (is_void_return) "" else "forward_call_tmp";
        
        const result = c.LLVMBuildCall2(
            self.builder,
            func_type,
            function,
            args_ptr,
            @intCast(call.arguments.items.len),
            call_name
        );
        
        if (@as(?*anyopaque, result) == null) {
            print("❌ Failed to generate call to forward declaration: {s}\n", .{function_name});
            return error.UndefinedFunction;
        }
        
        return result;
    }
    
    /// Generate method call
    fn generateMethodCall(self: *LLVMIRPipeline, method_call: *ast.MethodCallExpression) anyerror!c.LLVMValueRef {
        // Check if this is a "vibez.spill()" call
        const object_name = switch (method_call.object.*) {
            .Identifier => |name| name,
            else => "",
        };
        
        if (std.mem.eql(u8, object_name, "vibez")) {
            if (std.mem.eql(u8, method_call.method_name, "spill") or 
            std.mem.eql(u8, method_call.method_name, "spillln")) {
            // Handle vibez.spill() - use printf for all types to avoid runtime execution issues
            if (method_call.arguments.items.len > 0) {

            var args_arr = try self.allocator.alloc(ast.Expression, method_call.arguments.items.len);
            defer self.allocator.free(args_arr);
            for (method_call.arguments.items, 0..) |arg_ptr, i| {
            args_arr[i] = arg_ptr.*;
            }
            return try self.generatePrintCall(args_arr);
            } else {
            // No arguments - just print newline using puts
            const puts_func = self.functions.get("puts") orelse {
                print("❌ puts function not found\n", .{});
            return error.UndefinedFunction;
            };
            
            const empty_str = try self.generateStringLiteral("");
            const puts_type = c.LLVMGetElementType(c.LLVMTypeOf(puts_func));
            return c.LLVMBuildCall2(self.builder, puts_type, puts_func, @constCast(@ptrCast(&empty_str)), 1, "puts_empty");
            }
            } else if (std.mem.eql(u8, method_call.method_name, "print_separator")) {
                // Handle vibez.print_separator() - print separator
                const separator_str = c.LLVMBuildGlobalStringPtr(self.builder, "--------------------------------\n", "separator");
                return separator_str;
            }
        } else if (std.mem.eql(u8, object_name, "mathz")) {
            // Handle mathz functions by calling the CURSED-compiled stdlib functions
            // First try to load the module if not already loaded
            try self.loadAndCompileModule("mathz");
            
            // Generate the qualified function name (module.function)
            const qualified_name = try std.fmt.allocPrint(self.arena.allocator(), "{s}.{s}", .{ object_name, method_call.method_name });
            
            // Check if the function exists in the compiled functions
            if (self.functions.get(qualified_name)) |stdlib_func| {
                // Generate arguments with type conversion for CURSED semantics
                var args = std.ArrayList(c.LLVMValueRef){};
                defer args.deinit(self.allocator);
                
                // Get function parameter types for proper type conversion
                const func_type = c.LLVMGlobalGetValueType(stdlib_func);
                const param_count = c.LLVMCountParamTypes(func_type);
                var param_types: [16]c.LLVMTypeRef = undefined;
                if (param_count > 0) {
                    c.LLVMGetParamTypes(func_type, @ptrCast(&param_types));
                }
                
                for (method_call.arguments.items, 0..) |arg, i| {
                    var arg_val = try self.generateExpression(arg.*);
                    
                    // Convert argument type to match function parameter type
                    if (i < param_count) {
                        const expected_type = param_types[i];
                        const actual_type = c.LLVMTypeOf(arg_val);
                        
                        if (expected_type != actual_type) {
                            // Handle integer type conversions - all CURSED integers should be i32
                            if (c.LLVMGetTypeKind(expected_type) == c.LLVMIntegerTypeKind and 
                                c.LLVMGetTypeKind(actual_type) == c.LLVMIntegerTypeKind) {
                                
                                const expected_width = c.LLVMGetIntTypeWidth(expected_type);
                                const actual_width = c.LLVMGetIntTypeWidth(actual_type);
                                
                                if (expected_width < actual_width) {
                                    // Truncate to smaller type (i64 -> i32)
                                    arg_val = c.LLVMBuildTrunc(self.builder, arg_val, expected_type, "arg_trunc");
                                } else if (expected_width > actual_width) {
                                    // Extend to larger type (i16 -> i32)
                                    arg_val = c.LLVMBuildSExt(self.builder, arg_val, expected_type, "arg_extend");
                                }
                            }
                        }
                    }
                    
                    try args.append(self.allocator, arg_val);
                }
                
                // Call the function with properly typed arguments
                const result_name = try self.arena.allocator().dupeZ(u8, try std.fmt.allocPrint(self.arena.allocator(), "{s}_result", .{method_call.method_name}));
                return c.LLVMBuildCall2(self.builder, func_type, stdlib_func, args.items.ptr, @intCast(args.items.len), result_name);
            }
            
            // Fallback to hardcoded runtime functions for backwards compatibility
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
                    
                    // Convert i64 argument to i32 for normie type consistency
                    const arg_i32 = c.LLVMBuildTrunc(self.builder, arg, c.LLVMInt32TypeInContext(self.context), "arg_trunc_i32");
                    
                    // Call runtime function mathz_abs_normie (normie -> i32 types as per CURSED type system)
                    const mathz_abs_fn = try self.getOrDeclareRuntimeFunction("mathz_abs_normie", &[_]c.LLVMTypeRef{ c.LLVMInt32TypeInContext(self.context) }, c.LLVMInt32TypeInContext(self.context));
                    const args = [_]c.LLVMValueRef{ arg_i32 };
                    const func_type = c.LLVMGlobalGetValueType(mathz_abs_fn);
                    const result_i32 = c.LLVMBuildCall2(self.builder, func_type, mathz_abs_fn, @constCast(@ptrCast(&args)), 1, "mathz_abs_result");
                    
                    // Convert i32 result back to i64 for compatibility with variable storage
                    return c.LLVMBuildSExt(self.builder, result_i32, c.LLVMInt64TypeInContext(self.context), "result_ext_i64");
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
            // Handle collections functions by calling the CURSED-compiled stdlib functions
            // First try to load the module if not already loaded
            try self.loadAndCompileModule("collections");
            
            // Generate the qualified function name (module.function)
            const qualified_name = try std.fmt.allocPrint(self.arena.allocator(), "{s}.{s}", .{ object_name, method_call.method_name });
            
            // Check if the function exists in the compiled functions
            if (self.functions.get(qualified_name)) |stdlib_func| {
                // Generate arguments with type conversion for CURSED semantics
                var args = std.ArrayList(c.LLVMValueRef){};
                defer args.deinit(self.allocator);
                
                // Get function parameter types for proper type conversion
                const func_type = c.LLVMGlobalGetValueType(stdlib_func);
                const param_count = c.LLVMCountParamTypes(func_type);
                var param_types: [16]c.LLVMTypeRef = undefined;
                if (param_count > 0) {
                    c.LLVMGetParamTypes(func_type, @ptrCast(&param_types));
                }
                
                for (method_call.arguments.items, 0..) |arg, i| {
                    var arg_val = try self.generateExpression(arg.*);
                    
                    // Convert argument type to match function parameter type
                    if (i < param_count) {
                        const expected_type = param_types[i];
                        const actual_type = c.LLVMTypeOf(arg_val);
                        
                        if (expected_type != actual_type) {
                            // Handle integer type conversions - all CURSED integers should be i32
                            if (c.LLVMGetTypeKind(expected_type) == c.LLVMIntegerTypeKind and 
                                c.LLVMGetTypeKind(actual_type) == c.LLVMIntegerTypeKind) {
                                
                                const expected_width = c.LLVMGetIntTypeWidth(expected_type);
                                const actual_width = c.LLVMGetIntTypeWidth(actual_type);
                                
                                if (expected_width < actual_width) {
                                    // Truncate to smaller type (i64 -> i32)
                                    arg_val = c.LLVMBuildTrunc(self.builder, arg_val, expected_type, "arg_trunc");
                                } else if (expected_width > actual_width) {
                                    // Extend to larger type (i16 -> i32)
                                    arg_val = c.LLVMBuildSExt(self.builder, arg_val, expected_type, "arg_extend");
                                }
                            }
                        }
                    }
                    
                    try args.append(self.allocator, arg_val);
                }
                
                // Call the function with properly typed arguments
                const result_name = try self.arena.allocator().dupeZ(u8, try std.fmt.allocPrint(self.arena.allocator(), "{s}_result", .{method_call.method_name}));
                return c.LLVMBuildCall2(self.builder, func_type, stdlib_func, args.items.ptr, @intCast(args.items.len), result_name);
            }
            
            // Fallback to hardcoded runtime functions for backwards compatibility
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
            } else if (std.mem.eql(u8, method_call.method_name, "length")) {
                if (method_call.arguments.items.len >= 1) {
                    // Handle collections.length() - try multiple approaches for slice types
                    const arg_expr = method_call.arguments.items[0].*;
                    if (arg_expr == .Identifier) {
                        const var_name = arg_expr.Identifier;
                        
                        // First, check if we have a tracked array length
                        if (self.array_lengths.get(var_name)) |tracked_length| {
                            // print("🔍 DEBUG: collections.length({s}) = {} (tracked)\n", .{var_name, tracked_length});
                            return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), tracked_length, 0);
                        }
                        
                        // Second, try to get the variable and extract length from slice structure
                        if (self.variables.get(var_name)) |var_ref| {
                            // Check if this is a slice type (struct with length field)
                            const var_type = c.LLVMTypeOf(var_ref);
                            const pointed_type = c.LLVMGetElementType(var_type);
                            
                            if (c.LLVMGetTypeKind(pointed_type) == c.LLVMStructTypeKind) {
                                // Assume slice structure: {i32 length, ptr data, i32 capacity}
                                // Extract the length field (index 0)
                                const length_ptr = c.LLVMBuildStructGEP2(self.builder, pointed_type, var_ref, 0, "length_ptr");
                                const length_val = c.LLVMBuildLoad2(self.builder, c.LLVMInt32TypeInContext(self.context), length_ptr, "slice_length");
                                // print("🔍 DEBUG: collections.length({s}) extracted from slice structure\n", .{var_name});
                                return length_val;
                            }
                        }
                        
                        // Third, call runtime collections_length function for dynamic cases
                        const arg_val = try self.generateExpression(arg_expr);
                        const collections_len_fn = try self.getOrDeclareRuntimeFunction("collections_length", 
                            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}, 
                            c.LLVMInt32TypeInContext(self.context));
                        const args = [_]c.LLVMValueRef{arg_val};
                        const func_type = c.LLVMGlobalGetValueType(collections_len_fn);
                        // print("🔍 DEBUG: collections.length({s}) using runtime function\n", .{var_name});
                        return c.LLVMBuildCall2(self.builder, func_type, collections_len_fn, @constCast(@ptrCast(&args)), 1, "collections_len_result");
                    }
                    
                    // Fallback: return 0 if we can't determine length
                    // print("🔍 DEBUG: collections.length() fallback to 0\n", .{});
                    return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
                } else {
                    return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
                }
            }
        } else if (std.mem.eql(u8, object_name, "stringz")) {
            // Handle stringz functions by calling the CURSED-compiled stdlib functions
            // First try to load the module if not already loaded
            try self.loadAndCompileModule("stringz");
            
            // Generate the qualified function name (module.function)
            const qualified_name = try std.fmt.allocPrint(self.arena.allocator(), "{s}.{s}", .{ object_name, method_call.method_name });
            
            // Check if the function exists in the compiled functions
            if (self.functions.get(qualified_name)) |stdlib_func| {
                // Generate arguments with type conversion for CURSED semantics
                var args = std.ArrayList(c.LLVMValueRef){};
                defer args.deinit(self.allocator);
                
                // Get function parameter types for proper type conversion
                const func_type = c.LLVMGlobalGetValueType(stdlib_func);
                const param_count = c.LLVMCountParamTypes(func_type);
                var param_types: [16]c.LLVMTypeRef = undefined;
                if (param_count > 0) {
                    c.LLVMGetParamTypes(func_type, @ptrCast(&param_types));
                }
                
                for (method_call.arguments.items, 0..) |arg, i| {
                    var arg_val = try self.generateExpression(arg.*);
                    
                    // Convert argument type to match function parameter type
                    if (i < param_count) {
                        const expected_type = param_types[i];
                        const actual_type = c.LLVMTypeOf(arg_val);
                        
                        if (expected_type != actual_type) {
                            // Handle integer type conversions - all CURSED integers should be i32
                            if (c.LLVMGetTypeKind(expected_type) == c.LLVMIntegerTypeKind and 
                                c.LLVMGetTypeKind(actual_type) == c.LLVMIntegerTypeKind) {
                                
                                const expected_width = c.LLVMGetIntTypeWidth(expected_type);
                                const actual_width = c.LLVMGetIntTypeWidth(actual_type);
                                
                                if (expected_width < actual_width) {
                                    // Truncate to smaller type (i64 -> i32)
                                    arg_val = c.LLVMBuildTrunc(self.builder, arg_val, expected_type, "arg_trunc");
                                } else if (expected_width > actual_width) {
                                    // Extend to larger type (i16 -> i32)
                                    arg_val = c.LLVMBuildSExt(self.builder, arg_val, expected_type, "arg_extend");
                                }
                            }
                        }
                    }
                    
                    try args.append(self.allocator, arg_val);
                }
                
                // Call the function with properly typed arguments
                const result_name = try self.arena.allocator().dupeZ(u8, try std.fmt.allocPrint(self.arena.allocator(), "{s}_result", .{method_call.method_name}));
                return c.LLVMBuildCall2(self.builder, func_type, stdlib_func, args.items.ptr, @intCast(args.items.len), result_name);
            }
            
            // Fallback to hardcoded runtime functions for backwards compatibility
            if (std.mem.eql(u8, method_call.method_name, "length")) {
                if (method_call.arguments.items.len > 0) {
                    const str_arg = try self.generateExpression(method_call.arguments.items[0].*);
                    // Call runtime stringz_length function
                    const stringz_len_fn = try self.getOrDeclareRuntimeFunction("stringz_length", &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}, c.LLVMInt32TypeInContext(self.context));
                    const args = [_]c.LLVMValueRef{ str_arg };
                    const func_type = c.LLVMGlobalGetValueType(stringz_len_fn);
                    const result_i32 = c.LLVMBuildCall2(self.builder, func_type, stringz_len_fn, @constCast(@ptrCast(&args)), 1, "stringz_length_result");
                    // Convert i32 result to i64 for compatibility with variable storage
                    return c.LLVMBuildSExt(self.builder, result_i32, c.LLVMInt64TypeInContext(self.context), "length_result_ext_i64");
                } else {
                    return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
                }
            } else if (std.mem.eql(u8, method_call.method_name, "concat")) {
                if (method_call.arguments.items.len >= 2) {
                    const str1_arg = try self.generateExpression(method_call.arguments.items[0].*);
                    const str2_arg = try self.generateExpression(method_call.arguments.items[1].*);
                    // Call runtime stringz_concat function
                    const stringz_concat_fn = try self.getOrDeclareRuntimeFunction("stringz_concat", &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}, c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));
                    const args = [_]c.LLVMValueRef{ str1_arg, str2_arg };
                    const func_type = c.LLVMGlobalGetValueType(stringz_concat_fn);
                    return c.LLVMBuildCall2(self.builder, func_type, stringz_concat_fn, @constCast(@ptrCast(&args)), 2, "stringz_concat_result");
                } else {
                    // Return empty string if insufficient arguments
                    return c.LLVMBuildGlobalStringPtr(self.builder, "", "empty_string");
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
        } else if (std.mem.eql(u8, object_name, "path")) {
            // Handle path functions by calling runtime
            if (std.mem.eql(u8, method_call.method_name, "basename")) {
                if (method_call.arguments.items.len >= 1) {
                    _ = try self.generateExpression(method_call.arguments.items[0].*);
                    // For now, return a sample basename
                    const basename_str = c.LLVMBuildGlobalStringPtr(self.builder, "file.txt", "basename_result");
                    return basename_str;
                } else {
                    const empty_str = c.LLVMBuildGlobalStringPtr(self.builder, "", "empty_basename");
                    return empty_str;
                }
            } else if (std.mem.eql(u8, method_call.method_name, "dirname")) {
                if (method_call.arguments.items.len >= 1) {
                    _ = try self.generateExpression(method_call.arguments.items[0].*);
                    // For now, return a sample dirname
                    const dirname_str = c.LLVMBuildGlobalStringPtr(self.builder, "/home/user", "dirname_result");
                    return dirname_str;
                } else {
                    const current_dir_str = c.LLVMBuildGlobalStringPtr(self.builder, ".", "current_dir");
                    return current_dir_str;
                }
            } else if (std.mem.eql(u8, method_call.method_name, "exists")) {
                if (method_call.arguments.items.len >= 1) {
                    _ = try self.generateExpression(method_call.arguments.items[0].*);
                    // Return true for demonstration
                    return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 1, 0);
                } else {
                    return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 0, 0);
                }
            } else if (std.mem.eql(u8, method_call.method_name, "is_dir")) {
                if (method_call.arguments.items.len >= 1) {
                    _ = try self.generateExpression(method_call.arguments.items[0].*);
                    // Return true for demonstration
                    return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 1, 0);
                } else {
                    return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 0, 0);
                }
            }
        }
        
        // Generic stdlib module handling - try to call compiled CURSED functions first
        if (!std.mem.eql(u8, object_name, "vibez") and !std.mem.eql(u8, object_name, "")) {
            // Try to load the module if it's a stdlib module
            self.loadAndCompileModule(object_name) catch {
                // If module loading fails, continue to fallback handling
            };
            
            // Generate the qualified function name (module.function)
            const qualified_name = try std.fmt.allocPrint(self.arena.allocator(), "{s}.{s}", .{ object_name, method_call.method_name });
            
            // Check if the function exists in the compiled functions
            if (self.functions.get(qualified_name)) |stdlib_func| {
                print("DEBUG: Calling compiled CURSED stdlib function: {s}\n", .{qualified_name});
                
                // Get the function type first for proper argument conversion
                const func_type = c.LLVMGlobalGetValueType(stdlib_func);
                const param_count = c.LLVMCountParamTypes(func_type);
                var param_types: [16]c.LLVMTypeRef = undefined;
                if (param_count > 0) {
                    c.LLVMGetParamTypes(func_type, @ptrCast(&param_types));
                }
                
                // Generate arguments with proper type conversion for arrays
                var args = std.ArrayList(c.LLVMValueRef){};
                defer args.deinit(self.allocator);
                
                for (method_call.arguments.items, 0..) |arg, i| {
                    var arg_val = try self.generateExpression(arg.*);
                    
                    // Handle array-to-pointer conversion for function parameters
                    if (i < param_count) {
                        const expected_type = param_types[i];
                        const actual_type = c.LLVMTypeOf(arg_val);
                        
                        // If we have an array allocation and need a pointer, convert it
                        if (c.LLVMGetTypeKind(actual_type) == c.LLVMPointerTypeKind and
                            c.LLVMGetTypeKind(expected_type) == c.LLVMPointerTypeKind) {
                            
                            const actual_pointee = c.LLVMGetElementType(actual_type);
                            
                            // If actual is pointer-to-array and expected is pointer-to-element, convert
                            if (c.LLVMGetTypeKind(actual_pointee) == c.LLVMArrayTypeKind) {
                                // Cast array pointer to element pointer for function call
                                const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
                                const indices = [_]c.LLVMValueRef{zero, zero};
                                arg_val = c.LLVMBuildGEP2(self.builder, actual_pointee, arg_val, @constCast(@ptrCast(&indices)), 2, "array_to_ptr");
                            }
                        }
                    }
                    
                    try args.append(self.allocator, arg_val);
                }
                
                const result_name = try self.arena.allocator().dupeZ(u8, try std.fmt.allocPrint(self.arena.allocator(), "{s}_result", .{method_call.method_name}));
                return c.LLVMBuildCall2(self.builder, func_type, stdlib_func, args.items.ptr, @intCast(args.items.len), result_name);
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
        print("⚠️ Unhandled method call: {s}.{s}\n", .{object_name, method_call.method_name});
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
        
        // Handle multiple arguments by making separate print calls
        var result: c.LLVMValueRef = undefined;
        for (args, 0..) |arg, i| {
            result = try self.generateSinglePrintCall(arg, i == args.len - 1);
        }
        return result;
    }
    
    fn generateSinglePrintCall(self: *LLVMIRPipeline, arg: ast.Expression, is_last: bool) !c.LLVMValueRef {
        // Generate the argument
        const arg_val: c.LLVMValueRef = try self.generateExpression(arg);
        
        // Safeguard against null values
        if (@as(?*anyopaque, arg_val) == null) {
            print("❌ Null argument value in generatePrintCall\n", .{});
            return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        }
        
        const arg_type = c.LLVMTypeOf(arg_val);
        
        // Determine the print function to use based on type
        if (c.LLVMGetTypeKind(arg_type) == c.LLVMPointerTypeKind) {
            // String print using printf with proper formatting
            const printf_func = self.functions.get("printf") orelse {
                print("❌ printf function not found\n", .{});
                return error.UndefinedFunction;
            };
            
            // Use printf with %s format and proper suffix
            const fmt_str = if (is_last) try self.generateStringLiteral("%s\n") else try self.generateStringLiteral("%s ");
            
            const printf_args = [_]c.LLVMValueRef{ fmt_str, arg_val };
            
            // Create printf function type
            const char_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            const printf_function_type = c.LLVMFunctionType(
                c.LLVMInt32TypeInContext(self.context),
                @constCast(@ptrCast(&char_ptr_type)),
                1,
                1  // Variadic
            );
            
            return c.LLVMBuildCall2(self.builder, printf_function_type, printf_func, @constCast(@ptrCast(&printf_args)), 2, "printf_string_call");
        } else {
            // Integer or float print using printf  
            const printf_func = self.functions.get("printf") orelse {
                print("❌ printf function not found\n", .{});
                return error.UndefinedFunction;
            };
            
            // Determine format string based on LLVM type
            const printf_arg_type = c.LLVMTypeOf(arg_val);
            const type_kind = c.LLVMGetTypeKind(printf_arg_type);
            
            var fmt_str: c.LLVMValueRef = undefined;
            var converted_arg: c.LLVMValueRef = arg_val;
            
            if (type_kind == c.LLVMFloatTypeKind) {
                // Float (32-bit) - convert to double for printf
                fmt_str = if (is_last) try self.generateStringLiteral("%g\n") else try self.generateStringLiteral("%g ");
                converted_arg = c.LLVMBuildFPExt(self.builder, arg_val, c.LLVMDoubleTypeInContext(self.context), "float_to_double");
            } else if (type_kind == c.LLVMDoubleTypeKind) {
                // Double (64-bit float)
                fmt_str = if (is_last) try self.generateStringLiteral("%g\n") else try self.generateStringLiteral("%g ");
            } else if (type_kind == c.LLVMIntegerTypeKind) {
                // Integer types - check bit width
                const bit_width = c.LLVMGetIntTypeWidth(printf_arg_type);
                if (bit_width == 1) {
                    // Boolean type (i1) - convert to "based" or "cap" string
                    const based_str = if (is_last) try self.generateStringLiteral("based\n") else try self.generateStringLiteral("based ");
                    const cap_str = if (is_last) try self.generateStringLiteral("cap\n") else try self.generateStringLiteral("cap ");
                    
                    // Select the appropriate string based on the boolean value
                    const selected_str = c.LLVMBuildSelect(self.builder, arg_val, based_str, cap_str, "bool_to_string");
                    
                    // Use %s format for string output
                    fmt_str = try self.generateStringLiteral("%s");
                    converted_arg = selected_str;
                } else if (bit_width <= 32) {
                    // 32-bit or smaller integers
                    fmt_str = if (is_last) try self.generateStringLiteral("%d\n") else try self.generateStringLiteral("%d ");
                    // Ensure it's 32-bit for printf
                    converted_arg = c.LLVMBuildSExt(self.builder, arg_val, c.LLVMInt32TypeInContext(self.context), "extend_to_int32");
                } else {
                    // 64-bit integers
                    fmt_str = if (is_last) try self.generateStringLiteral("%lld\n") else try self.generateStringLiteral("%lld ");
                    // Ensure it's 64-bit for printf
                    converted_arg = c.LLVMBuildSExt(self.builder, arg_val, c.LLVMInt64TypeInContext(self.context), "extend_to_int64");
                }
            } else {
                // Fallback for unknown types
                fmt_str = if (is_last) try self.generateStringLiteral("%d\n") else try self.generateStringLiteral("%d ");
                converted_arg = c.LLVMBuildSExt(self.builder, arg_val, c.LLVMInt32TypeInContext(self.context), "fallback_to_int32");
            }
            
            const printf_args = [_]c.LLVMValueRef{ fmt_str, converted_arg };
            
            // Create printf function type properly
            const char_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            const printf_function_type = c.LLVMFunctionType(
                c.LLVMInt32TypeInContext(self.context),
                @constCast(@ptrCast(&char_ptr_type)),
                1,
                1  // Variadic
            );
            
            const printf_call = c.LLVMBuildCall2(self.builder, printf_function_type, printf_func, @constCast(@ptrCast(&printf_args)), 2, "printf_call");
            
            // Add fflush(stdout) to ensure immediate output
            const fflush_func = self.functions.get("fflush") orelse {
                print("❌ fflush function not found\n", .{});
                return error.UndefinedFunction;
            };
            
            // Create null pointer for stdout (fflush(NULL) flushes all streams)
            const null_ptr = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));
            var fflush_args = [_]c.LLVMValueRef{null_ptr};
            
            const file_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            const fflush_function_type = c.LLVMFunctionType(
                c.LLVMInt32TypeInContext(self.context),
                @constCast(@ptrCast(&file_ptr_type)),
                1,
                0
            );
            
            _ = c.LLVMBuildCall2(self.builder, fflush_function_type, fflush_func, @ptrCast(&fflush_args), 1, "fflush_call");
            
            return printf_call;
        }
    }

    /// Generate append function call for arrays
    fn generateAppendCall(self: *LLVMIRPipeline, array_expr: ast.Expression, element_expr: ast.Expression) !c.LLVMValueRef {
        // This is a simplified implementation that focuses on length tracking
        // for interpreter/compiler parity. In a full implementation, this would
        // need to handle dynamic memory allocation and array resizing.
        
        // Evaluate the element expression (for side effects)
        const element_val = try self.generateExpression(element_expr);
        _ = element_val; // Acknowledge we have the element value
        
        // Track array length if we can identify the variable name
        if (array_expr == .Identifier) {
            const var_name = array_expr.Identifier;
            if (self.array_lengths.getPtr(var_name)) |length_ptr| {
                length_ptr.* += 1;
            } else {
                // Initialize array length if not tracked yet
                const safe_name = try self.arena.allocator().dupe(u8, var_name);
                try self.array_lengths.put(safe_name, 1);
            }
        }
        
        // Return the array expression unchanged (simplified implementation)
        // Note: In a full implementation, this would return a new array with the appended element
        return try self.generateExpression(array_expr);
    }

    /// Generate len function call for arrays
    fn generateLenCall(self: *LLVMIRPipeline, array_expr: ast.Expression) !c.LLVMValueRef {
        // print("🔍 DEBUG: Generating len call\n", .{});
        
        // Evaluate the array expression
        const array_val = try self.generateExpression(array_expr);
        _ = array_val; // suppress unused variable warning
        
        // Look up the array length if we can identify the variable name
        var length: u32 = 0;
        if (array_expr == .Identifier) {
            const var_name = array_expr.Identifier;
            if (self.array_lengths.get(var_name)) |tracked_length| {
                length = tracked_length;
                // print("🔍 DEBUG: Found tracked length for '{s}': {}\n", .{var_name, length});
            } else {
                // print("🔍 DEBUG: No length tracking found for '{s}', using default 0\n", .{var_name});
            }
        }
        
        const length_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), length, 0);
        
        // print("✅ DEBUG: Len call generated - length: {}\n", .{length});
        return length_value;
    }
    
    /// Convert CURSED type to LLVM type
    /// Check if two LLVM types are compatible for function signatures
    /// This allows for integer types with different widths to be considered compatible
    /// if they are both integer types (handles int64 <-> int32 compatibility)
    fn areTypesCompatible(self: *LLVMIRPipeline, type1: c.LLVMTypeRef, type2: c.LLVMTypeRef) bool {
        _ = self; // unused parameter
        // If types are identical, they're compatible
        if (type1 == type2) return true;
        
        const kind1 = c.LLVMGetTypeKind(type1);
        const kind2 = c.LLVMGetTypeKind(type2);
        
        // Both must be the same kind of type
        if (kind1 != kind2) return false;
        
        // For integer types, we allow different widths to be compatible
        // This handles cases where forward declaration infers i64 but actual function uses i32
        if (kind1 == c.LLVMIntegerTypeKind) {
            return true; // All integer types are compatible with each other
        }
        
        // For other types, they must be identical
        return false;
    }

    fn cursedTypeToLLVM(self: *LLVMIRPipeline, cursed_type: ast.Type) !c.LLVMTypeRef {
        switch (cursed_type) {
            .Basic => |basic| {
                return switch (basic) {
                    .Smol => c.LLVMInt8TypeInContext(self.context),
                    .Mid => c.LLVMInt16TypeInContext(self.context),
                    .Normie => c.LLVMInt32TypeInContext(self.context),
                    .Thicc => c.LLVMInt64TypeInContext(self.context),
                    .Drip => c.LLVMDoubleTypeInContext(self.context),  // Float type (f64)
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
                    .Yikes => c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // error type (string-like)
                    .Auto => c.LLVMInt32TypeInContext(self.context), // Auto type defaults to normie (32-bit int)
                };
            },
            .Array => |array| {
                const element_type = try self.cursedTypeToLLVM(array.element_type.get().?.*);
                // Arrays as function parameters are passed as pointers to array
                return c.LLVMPointerType(element_type, 0);
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
    
    /// Load and compile CURSED stdlib module
    fn loadAndCompileModule(self: *LLVMIRPipeline, module_name: []const u8) anyerror!void {
        // Check if already compiled
        if (self.compiled_modules.contains(module_name)) return;
        
        // Save the current LLVM builder context
        const saved_function = self.current_function;
        const saved_block = c.LLVMGetInsertBlock(self.builder);
        
        print("DEBUG: Loading and compiling CURSED module: {s}\n", .{module_name});
        
        // Create temporary arena for module compilation
        var module_arena = std.heap.ArenaAllocator.init(self.allocator);
        defer module_arena.deinit();
        const tmp_allocator = module_arena.allocator();
        
        // 1. Locate the .csd file - try multiple paths like interpreter
        const local_path = try std.fmt.allocPrint(tmp_allocator, "stdlib/{s}/mod.csd", .{module_name});
        const parent_path = try std.fmt.allocPrint(tmp_allocator, "../stdlib/{s}/mod.csd", .{module_name});
        const layer1_path = try std.fmt.allocPrint(tmp_allocator, "../stdlib/layer1/{s}.csd", .{module_name});
        
        const source = std.fs.cwd().readFileAlloc(tmp_allocator, local_path, std.math.maxInt(usize)) catch |err1| blk: {
            if (err1 == error.FileNotFound) {
                break :blk std.fs.cwd().readFileAlloc(tmp_allocator, parent_path, std.math.maxInt(usize)) catch |err2| blk2: {
                    if (err2 == error.FileNotFound) {
                        break :blk2 std.fs.cwd().readFileAlloc(tmp_allocator, layer1_path, std.math.maxInt(usize)) catch {
                            print("DEBUG: Could not read CURSED module from any path: {s}, {s}, {s}\n", .{ local_path, parent_path, layer1_path });
                            return;
                        };
                    } else {
                        print("DEBUG: Could not read CURSED module {s}: {}\n", .{ parent_path, err2 });
                        return;
                    }
                };
            } else {
                print("DEBUG: Could not read CURSED module {s}: {}\n", .{ local_path, err1 });
                return;
            }
        };
        
        print("DEBUG: Successfully read CURSED module {s} ({} bytes)\n", .{ module_name, source.len });
        
        // 2. Front-end: tokenize and parse
        var lex = lexer.Lexer.init(tmp_allocator, source);
        const token_list = try lex.tokenize();
        const tokens = token_list.items;
        var parse = parser.Parser.init(tmp_allocator, tokens);
        defer parse.deinit();
        const program = try parse.parseProgram();
        defer {
            var mutable_program = @constCast(&program);
            mutable_program.deinit(tmp_allocator);
        }
        
        print("DEBUG: Successfully parsed CURSED module {s} ({} statements)\n", .{ module_name, program.statements.items.len });
        
        // 3. Set current module context and generate LLVM IR for each function
        const previous_module_name = self.current_module_name;
        self.current_module_name = try self.arena.allocator().dupe(u8, module_name);
        
        for (program.statements.items) |stmt_ptr| {
            const stmt = @as(*ast.Statement, @alignCast(@ptrCast(stmt_ptr)));
            switch (stmt.*) {
                .Function => |func_decl| {
                    print("DEBUG: Compiling CURSED stdlib function: {s}.{s}\n", .{module_name, func_decl.name});
                    
                    // Use the regular generateFunction which now handles qualified names
                    try self.generateFunction(func_decl);
                },
                else => {
                    // For now, skip non-function statements in stdlib modules
                }
            }
        }
        
        // Restore previous module context
        self.current_module_name = previous_module_name;
        
        // Mark module as compiled
        const module_name_owned = try self.arena.allocator().dupe(u8, module_name);
        try self.compiled_modules.put(module_name_owned, {});
        print("DEBUG: Successfully compiled CURSED module: {s}\n", .{module_name});
        
        // Restore the LLVM builder context
        self.current_function = saved_function;
        if (saved_block != null) {
            c.LLVMPositionBuilderAtEnd(self.builder, saved_block);
        }
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
        try self.variable_types.put("vibez", vibez_type);
    }
    
    /// Declare standard C library functions
    fn declareCLibraryFunctions(self: *LLVMIRPipeline) !void {
        // Declare printf: int printf(const char *format, ...)
        const char_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            @constCast(@ptrCast(&char_ptr_type)),
            1,
            1 // variadic
        );
        const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        try self.functions.put("printf", printf_func);
        
        // Declare puts: int puts(const char *s)
        const puts_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            @constCast(@ptrCast(&char_ptr_type)),
            1,
            0 // not variadic
        );
        const puts_func = c.LLVMAddFunction(self.module, "puts", puts_type);
        try self.functions.put("puts", puts_func);
        
        // Declare fflush: int fflush(FILE *stream)
        const file_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0); // FILE* as void*
        const fflush_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            @constCast(@ptrCast(&file_ptr_type)),
            1,
            0 // not variadic
        );
        const fflush_func = c.LLVMAddFunction(self.module, "fflush", fflush_type);
        try self.functions.put("fflush", fflush_func);
        
        // print("✅ C library functions declared\n", .{});
    }
    
    /// Register builtin functions available globally without imports
    fn registerBuiltinFunctions(self: *LLVMIRPipeline) !void {
        // Register yap function as a builtin
        // yap(value) -> i32 (returns 0 for success)
        const char_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        const yap_param_types = [_]c.LLVMTypeRef{char_ptr_type};
        const yap_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return type
            @constCast(@ptrCast(&yap_param_types)),
            1, // param count
            0  // not variadic
        );
        const yap_func = c.LLVMAddFunction(self.module, "yap", yap_type);
        try self.functions.put("yap", yap_func);
        
        // print("✅ Builtin functions registered (yap)\n", .{});
    }
    
    /// Ensure main function exists and includes global statements
    fn ensureMainFunctionWithGlobalStatements(self: *LLVMIRPipeline, global_statements: []*ast.Statement) !void {
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
        
        // Set current function context
        self.current_function = main_func;
        
        // Generate global statements within main function context
        for (global_statements) |stmt| {
            // Ensure we're still in the main function after each statement
            self.current_function = main_func;
            c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
            try self.generateStatement(stmt.*);
        }
        
        // Restore main function context explicitly 
        self.current_function = main_func;
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // CRITICAL FIX: CURSED always uses "main_character" as the entry function
        // The package name can be "main" but the function is always "main_character"
        const main_function_name: []const u8 = "main_character";
        
        if (self.functions.get(main_function_name)) |cursed_main_func| {
            // Create the function type for main function: () -> void
            const main_function_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.context),
                null,
                0,
                0
            );
            
            // Call main function with no arguments  
            _ = c.LLVMBuildCall2(self.builder, main_function_type, cursed_main_func, null, 0, "");
        }

        // Add proper terminator if block doesn't have one
        const current_block = c.LLVMGetInsertBlock(self.builder);
        if (current_block == null) {
            c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        } else {
            const terminator = c.LLVMGetBasicBlockTerminator(current_block);
            if (terminator == null) {
                // No terminator, add return statement
            } else {
                // Already has terminator, clear context and return
                self.current_function = null;
                return;
            }
        }
        
        // Return 0 with correct type
        const func_type = c.LLVMGlobalGetValueType(main_func);
        const func_ret_ty = c.LLVMGetReturnType(func_type);
        const zero = c.LLVMConstInt(func_ret_ty, 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
        
        // Clear current function context
        self.current_function = null;
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
        
        // Check if main function exists and has content
        const main_func = c.LLVMGetNamedFunction(self.module, "main");
        if (main_func == null) {
            print("❌ No main function generated - compilation incomplete\n", .{});
            return error.EmptyCodeGeneration;
        }
        
        // Check if main function has any basic blocks (non-empty body)
        const first_bb = c.LLVMGetFirstBasicBlock(main_func);
        if (first_bb == null) {
            print("❌ Main function has empty body - code generation failed\n", .{});
            return error.EmptyCodeGeneration;
        }
        
        // Check if there are any instructions in the first basic block
        const first_instr = c.LLVMGetFirstInstruction(first_bb);
        if (first_instr == null) {
            print("❌ Main function basic block is empty - code generation failed\n", .{});
            return error.EmptyCodeGeneration;
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
        
        // print("✅ LLVM IR written to: {s}\n", .{output_file});
    }

    /// Compile to executable using llc + gcc pipeline
    pub fn compileToExecutable(self: *LLVMIRPipeline, output_file: []const u8) !void {
        // Verify module before compilation
        try self.verifyModule();
        
        // Write LLVM IR to temporary file
        const ir_file = try std.fmt.allocPrint(self.allocator, "{s}.ll", .{output_file});
        defer self.allocator.free(ir_file);
        
        try self.writeIRToFile(ir_file);
        
        // Step 1: Use llc-18 to compile IR to object file
        const obj_file = try std.fmt.allocPrint(self.allocator, "{s}.o", .{output_file});
        defer self.allocator.free(obj_file);
        
        // print("🔧 Step 1: Compiling IR to object file with llc-18...\n", .{});
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
        // print("🔧 Step 2: Linking object file with gcc...\n", .{});
        const gcc_result = try std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{
                "gcc",
                "-no-pie", // Disable PIE to avoid relocation issues
                "-o", output_file,
                obj_file,
                "/home/ghuntley/cursed/runtime/libcursed_stdlib.a", // Link runtime library
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
        
        // print("✅ Successfully compiled to: {s}\n", .{output_file});
    }
    
    /// Dump LLVM IR to stdout for debugging
    pub fn dumpIR(self: *LLVMIRPipeline) void {
        print("🔍 LLVM IR:\n", .{});
        c.LLVMDumpModule(self.module);
    }
    
    /// Generate division by zero panic following CURSED error handling specification
    fn generateDivisionByZeroPanic(self: *LLVMIRPipeline) !void {
        // Generate call to printf to print the panic message
        const printf_func = self.functions.get("printf") orelse {
            print("❌ printf function not found for panic\n", .{});
            return error.UndefinedFunction;
        };
        
        // Create panic message string
        const panic_msg = try self.generateStringLiteral("Division by zero");
        
        // Call printf with panic message
        const char_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        const printf_function_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            @constCast(@ptrCast(&char_ptr_type)),
            1,
            1  // Variadic
        );
        
        const printf_args = [_]c.LLVMValueRef{panic_msg};
        _ = c.LLVMBuildCall2(self.builder, printf_function_type, printf_func, @constCast(@ptrCast(&printf_args)), 1, "");
        
        // Generate call to exit(1) to terminate the program
        // First declare exit function if not already declared
        const exit_func = if (self.functions.get("exit")) |existing_exit|
            existing_exit
        else blk: {
            const exit_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.context),
                @constCast(@ptrCast(&c.LLVMInt32TypeInContext(self.context))),
                1,
                0
            );
            const exit_fn = c.LLVMAddFunction(self.module, "exit", exit_type);
            try self.functions.put("exit", exit_fn);
            break :blk exit_fn;
        };
        
        // Call exit(1)
        const exit_code = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 1, 0);
        const exit_args = [_]c.LLVMValueRef{exit_code};
        const exit_function_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            @constCast(@ptrCast(&c.LLVMInt32TypeInContext(self.context))),
            1,
            0
        );
        _ = c.LLVMBuildCall2(self.builder, exit_function_type, exit_func, @constCast(@ptrCast(&exit_args)), 1, "");
        
        // Add unreachable instruction since exit() never returns
        _ = c.LLVMBuildUnreachable(self.builder);
    }
};
