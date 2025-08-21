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

// LLVM C imports with proper configuration
const c = @cImport({
    @cDefine("__x86_64__", "1");
    @cDefine("__i386__", "0");
    @cDefine("TARGET_CPU", "\"x86-64\"");
    @cDefine("LLVM_HOST_TRIPLE", "\"x86_64-unknown-linux-gnu\"");
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Transforms/Scalar.h");
    @cInclude("llvm-c/Transforms/Utils.h");
});

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
        print("🔧 Initializing LLVM components...\n");
        c.LLVMInitializeCore(c.LLVMGetGlobalPassRegistry());
        c.LLVMInitializeNativeTarget();
        c.LLVMInitializeNativeAsmPrinter();
        c.LLVMInitializeNativeAsmParser();
        
        // Create LLVM context
        const context = c.LLVMContextCreate();
        if (context == null) return error.LLVMContextCreationFailed;
        
        // Create module
        const module_name_z = try arena_allocator.dupeZ(u8, module_name);
        const module = c.LLVMModuleCreateWithNameInContext(module_name_z.ptr, context);
        if (module == null) return error.LLVMModuleCreationFailed;
        
        // Set target triple
        const default_triple = c.LLVMGetDefaultTargetTriple();
        c.LLVMSetTarget(module, default_triple);
        c.LLVMDisposeMessage(default_triple);
        
        // Create builder
        const builder = c.LLVMCreateBuilderInContext(context);
        if (builder == null) return error.LLVMBuilderCreationFailed;
        
        // Create pass manager with optimizations
        const pass_manager = c.LLVMCreateFunctionPassManagerForModule(module);
        
        // Add common optimization passes
        c.LLVMAddInstructionCombiningPass(pass_manager);
        c.LLVMAddReassociatePass(pass_manager);
        c.LLVMAddGVNPass(pass_manager);
        c.LLVMAddCFGSimplificationPass(pass_manager);
        c.LLVMAddPromoteMemoryToRegisterPass(pass_manager);
        c.LLVMAddDeadStoreEliminationPass(pass_manager);
        
        c.LLVMInitializeFunctionPassManager(pass_manager);
        
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
        
        if (target_machine == null) {
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
        
        print("✅ LLVM IR Pipeline initialized successfully\n");
        return pipeline;
    }
    
    pub fn deinit(self: *LLVMIRPipeline) void {
        print("🧹 Cleaning up LLVM IR Pipeline...\n");
        
        // Clean up hash maps first
        self.functions.deinit(allocator);
        self.variables.deinit(allocator);
        self.global_strings.deinit(allocator);
        self.type_cache.deinit(allocator);
        
        // Clean up type system components
        self.type_checker.deinit(allocator);
        
        // Dispose LLVM objects in proper order
        if (self.pass_manager) |pm| {
            c.LLVMDisposePassManager(pm);
        }
        if (self.target_machine) |tm| {
            c.LLVMDisposeTargetMachine(tm);
        }
        if (self.builder) |builder| {
            c.LLVMDisposeBuilder(builder);
        }
        if (self.module) |module| {
            c.LLVMDisposeModule(module);
        }
        if (self.context) |context| {
            c.LLVMContextDispose(context);
        }
        
        // Clean up arena and self
        self.arena.deinit(allocator);
        self.allocator.destroy(self);
        
        print("✅ LLVM IR Pipeline cleanup complete\n");
    }
    
    /// Complete compilation pipeline: Source -> AST -> Type Check -> LLVM IR -> Binary
    pub fn compileSource(self: *LLVMIRPipeline, source: []const u8, output_file: []const u8, verbose: bool) !void {
        print("🚀 Starting complete LLVM compilation pipeline...\n");
        
        // Step 1: Tokenize source
        if (verbose) print("📝 Step 1: Tokenizing source code...\n");
        var lex = try lexer.Lexer.init(self.allocator, source);
        defer lex.deinit(allocator);
        
        // Step 2: Parse into AST
        if (verbose) print("🌳 Step 2: Parsing AST...\n");
        var parse = try parser.Parser.init(self.allocator, &lex);
        defer parse.deinit(allocator);
        const program = try parse.parseProgram();
        
        // Step 3: Type checking
        if (verbose) print("🔍 Step 3: Type checking...\n");
        try self.runTypeChecking(program);
        
        // Step 4: Generate LLVM IR
        if (verbose) print("⚡ Step 4: Generating LLVM IR...\n");
        try self.generateIR(program);
        
        // Step 5: Optimize IR
        if (verbose) print("🔧 Step 5: Optimizing IR...\n");
        try self.optimizeIR();
        
        // Step 6: Verify module
        if (verbose) print("✅ Step 6: Verifying module...\n");
        try self.verifyModule();
        
        // Step 7: Compile to binary
        if (verbose) print("🔥 Step 7: Compiling to binary...\n");
        try self.compileToExecutable(output_file);
        
        print("🎉 Compilation pipeline completed successfully!\n");
    }
    
    /// Run type checking on the AST
    fn runTypeChecking(self: *LLVMIRPipeline, program: ast.Program) !void {
        // Type check each statement in the program
        for (program.statements.items) |stmt| {
            try self.typeCheckStatement(stmt);
        }
    }
    
    /// Type check a statement
    fn typeCheckStatement(self: *LLVMIRPipeline, stmt: ast.Statement) !void {
        switch (stmt) {
            .FunctionDeclaration => |func_decl| {
                // Type check function parameters and return type
                for (func_decl.parameters.items) |param| {
                    _ = try self.validateType(param.param_type);
                }
                if (func_decl.return_type) |ret_type| {
                    _ = try self.validateType(ret_type);
                }
                
                // Type check function body
                for (func_decl.body.items) |body_stmt| {
                    try self.typeCheckStatement(body_stmt);
                }
            },
            .VariableDeclaration => |var_decl| {
                _ = try self.validateType(var_decl.variable_type);
                if (var_decl.initializer) |initializer| {
                    try self.typeCheckExpression(initializer);
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
            .BinaryOperation => |bin_op| {
                try self.typeCheckExpression(bin_op.left.*);
                try self.typeCheckExpression(bin_op.right.*);
                // TODO: Check operator compatibility
            },
            .FunctionCall => |call| {
                for (call.arguments.items) |arg| {
                    try self.typeCheckExpression(arg);
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
    fn generateIR(self: *LLVMIRPipeline, program: ast.Program) !void {
        // Generate IR for each statement
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Ensure we have a main function
        try self.ensureMainFunction();
    }
    
    /// Generate IR for a statement
    fn generateStatement(self: *LLVMIRPipeline, stmt: ast.Statement) !void {
        switch (stmt) {
            .FunctionDeclaration => |func_decl| {
                try self.generateFunction(func_decl);
            },
            .VariableDeclaration => |var_decl| {
                try self.generateVariableDeclaration(var_decl);
            },
            .Expression => |expr| {
                _ = try self.generateExpression(expr);
            },
            else => {
                print("⚠️ Unhandled statement type in IR generation\n");
            },
        }
    }
    
    /// Generate LLVM function
    fn generateFunction(self: *LLVMIRPipeline, func_decl: ast.FunctionDeclaration) !void {
        // Create function type
        var param_types = .empty;
        defer param_types.deinit(allocator);
        
        for (func_decl.parameters.items) |param| {
            const llvm_type = try self.cursedTypeToLLVM(param.param_type);
            try param_types.append(allocator, llvm_type);
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
        for (func_decl.body.items) |stmt| {
            try self.generateStatement(stmt);
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
        
        // Run optimizations on the function
        _ = c.LLVMRunFunctionPassManager(self.pass_manager, function);
    }
    
    /// Generate variable declaration
    fn generateVariableDeclaration(self: *LLVMIRPipeline, var_decl: ast.VariableDeclaration) !void {
        const llvm_type = try self.cursedTypeToLLVM(var_decl.variable_type);
        const var_name_z = try self.arena.allocator().dupeZ(u8, var_decl.name);
        
        // Create alloca
        const alloca = c.LLVMBuildAlloca(self.builder, llvm_type, var_name_z.ptr);
        try self.variables.put(var_decl.name, alloca);
        
        // Generate initializer if present
        if (var_decl.initializer) |initializer| {
            const init_value = try self.generateExpression(initializer);
            _ = c.LLVMBuildStore(self.builder, init_value, alloca);
        }
    }
    
    /// Generate expression
    fn generateExpression(self: *LLVMIRPipeline, expr: ast.Expression) !c.LLVMValueRef {
        switch (expr) {
            .Literal => |lit| {
                return try self.generateLiteral(lit);
            },
            .Identifier => |ident| {
                if (self.variables.get(ident.name)) |var_alloca| {
                    const var_type = c.LLVMGetAllocatedType(var_alloca);
                    return c.LLVMBuildLoad2(self.builder, var_type, var_alloca, "load_tmp");
                } else {
                    print("❌ Undefined variable: {s}\n", .{ident.name});
                    return error.UndefinedVariable;
                }
            },
            .BinaryOperation => |bin_op| {
                return try self.generateBinaryOperation(bin_op);
            },
            .FunctionCall => |call| {
                return try self.generateFunctionCall(call);
            },
            else => {
                print("⚠️ Unhandled expression type in IR generation\n");
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
        const str_ptr = c.LLVMConstGEP2(str_type, str_global, @ptrCast(&indices), 2);
        
        try self.global_strings.put(str_val, str_ptr);
        return str_ptr;
    }
    
    /// Generate binary operation
    fn generateBinaryOperation(self: *LLVMIRPipeline, bin_op: ast.BinaryOperation) !c.LLVMValueRef {
        const left = try self.generateExpression(bin_op.left.*);
        const right = try self.generateExpression(bin_op.right.*);
        
        return switch (bin_op.operator) {
            .Plus => c.LLVMBuildAdd(self.builder, left, right, "add_tmp"),
            .Minus => c.LLVMBuildSub(self.builder, left, right, "sub_tmp"),
            .Multiply => c.LLVMBuildMul(self.builder, left, right, "mul_tmp"),
            .Divide => c.LLVMBuildSDiv(self.builder, left, right, "div_tmp"),
            .Equal => c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq_tmp"),
            .NotEqual => c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "ne_tmp"),
            .LessThan => c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt_tmp"),
            .GreaterThan => c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt_tmp"),
            else => {
                print("⚠️ Unhandled binary operator\n");
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
            },
        };
    }
    
    /// Generate function call
    fn generateFunctionCall(self: *LLVMIRPipeline, call: ast.FunctionCall) !c.LLVMValueRef {
        // Handle standard library calls
        if (std.mem.eql(u8, call.name, "vibez.spill")) {
            return try self.generatePrintCall(call.arguments.items);
        }
        
        // Look up user-defined function
        if (self.functions.get(call.name)) |function| {
            var args = .empty;
            defer args.deinit(allocator);
            
            for (call.arguments.items) |arg| {
                const arg_val = try self.generateExpression(arg);
                try args.append(allocator, arg_val);
            }
            
            const func_type = c.LLVMGetElementType(c.LLVMTypeOf(function));
            return c.LLVMBuildCall2(
                self.builder,
                func_type,
                function,
                args.items.ptr,
                @intCast(args.items.len),
                "call_tmp"
            );
        } else {
            print("❌ Undefined function: {s}\n", .{call.name});
            return error.UndefinedFunction;
        }
    }
    
    /// Generate print function call
    fn generatePrintCall(self: *LLVMIRPipeline, args: []ast.Expression) !c.LLVMValueRef {
        if (args.len == 0) return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        
        // Generate the argument
        const arg_val = try self.generateExpression(args[0]);
        const arg_type = c.LLVMTypeOf(arg_val);
        
        // Determine the print function to use based on type
        if (c.LLVMGetTypeKind(arg_type) == c.LLVMPointerTypeKind) {
            // String print using puts
            const puts_func = self.functions.get("puts") orelse {
                print("❌ puts function not found\n");
                return error.UndefinedFunction;
            };
            
            const puts_type = c.LLVMGetElementType(c.LLVMTypeOf(puts_func));
            return c.LLVMBuildCall2(self.builder, puts_type, puts_func, @ptrCast(&arg_val), 1, "puts_call");
        } else {
            // Integer print using printf
            const printf_func = self.functions.get("printf") orelse {
                print("❌ printf function not found\n");
                return error.UndefinedFunction;
            };
            
            const fmt_str = try self.generateStringLiteral("%ld\n");
            const printf_args = [_]c.LLVMValueRef{ fmt_str, arg_val };
            
            const printf_type = c.LLVMGetElementType(c.LLVMTypeOf(printf_func));
            return c.LLVMBuildCall2(self.builder, printf_type, printf_func, @ptrCast(&printf_args), 2, "printf_call");
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
                    .Lit => c.LLVMInt1TypeInContext(self.context),
                    .Sus => c.LLVMVoidTypeInContext(self.context),
                };
            },
            .Array => |array| {
                const element_type = try self.cursedTypeToLLVM(array.element_type.*);
                return c.LLVMArrayType(element_type, @intCast(array.size));
            },
            .Pointer => |ptr| {
                const target_type = try self.cursedTypeToLLVM(ptr.target_type.*);
                return c.LLVMPointerType(target_type, 0);
            },
            else => {
                print("⚠️ Unhandled type conversion to LLVM\n");
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
            @ptrCast(&char_ptr_type),
            1,
            0
        );
        const puts_func = c.LLVMAddFunction(self.module, "puts", puts_type);
        try self.functions.put("puts", puts_func);
        
        // printf(const char* format, ...) -> int
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            @ptrCast(&char_ptr_type),
            1,
            1  // Variadic
        );
        const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        try self.functions.put("printf", printf_func);
    }
    
    /// Ensure main function exists
    fn ensureMainFunction(self: *LLVMIRPipeline) !void {
        // Check if main already exists
        if (self.functions.contains("main")) {
            return;
        }
        
        // Create main function that calls main_character if it exists
        const main_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            null,
            0,
            0
        );
        const main_func = c.LLVMAddFunction(self.module, "main", main_type);
        try self.functions.put("main", main_func);
        
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Call main_character if it exists
        if (self.functions.get("main_character")) |main_char_func| {
            const main_char_type = c.LLVMGetElementType(c.LLVMTypeOf(main_char_func));
            _ = c.LLVMBuildCall2(self.builder, main_char_type, main_char_func, null, 0, "");
        }
        
        // Return 0
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
    }
    
    /// Optimize the generated IR
    fn optimizeIR(self: *LLVMIRPipeline) !void {
        // Run function passes on all functions
        var func = c.LLVMGetFirstFunction(self.module);
        while (func != null) {
            _ = c.LLVMRunFunctionPassManager(self.pass_manager, func);
            func = c.LLVMGetNextFunction(func);
        }
        
        c.LLVMFinalizeFunctionPassManager(self.pass_manager);
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
    
    /// Compile to executable using LLVM
    pub fn compileToExecutable(self: *LLVMIRPipeline, output_file: []const u8) !void {
        // Write LLVM IR to temporary file
        const ir_file = try std.fmt.allocPrint(self.allocator, "{s}.ll", .{output_file});
        defer self.allocator.free(ir_file);
        
        var error_msg: [*c]u8 = undefined;
        const ir_file_z = try self.arena.allocator().dupeZ(u8, ir_file);
        if (c.LLVMPrintModuleToFile(self.module, ir_file_z.ptr, &error_msg) != 0) {
            print("❌ Failed to write IR file: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return error.IRWriteFailed;
        }
        
        // Compile using clang
        const result = try std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{
                "clang",
                "-O2",
                "-o", output_file,
                ir_file,
            },
        });
        
        defer self.allocator.free(result.stdout);
        defer self.allocator.free(result.stderr);
        
        if (result.term != .Exited or result.term.Exited != 0) {
            print("❌ Compilation failed:\n{s}\n", .{result.stderr});
            return error.CompilationFailed;
        }
        
        print("✅ Successfully compiled to: {s}\n", .{output_file});
    }
    
    /// Dump LLVM IR to stdout for debugging
    pub fn dumpIR(self: *LLVMIRPipeline) void {
        print("🔍 LLVM IR:\n");
        c.LLVMDumpModule(self.module);
    }
};
