const std = @import("std");

// Real LLVM C API bindings for LLVM 18
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
    @cInclude("llvm-c/Transforms/Scalar.h");
    @cInclude("llvm-c/Transforms/IPO.h");
    @cInclude("llvm-c/Transforms/PassManagerBuilder.h");
});

const ast = @import("ast.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// Real LLVM backend implementation for CURSED
/// Replaces dummy implementations with actual LLVM IR generation
pub const RealLLVMBackend = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    
    // Function and variable management
    functions: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Type cache
    types: std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Current state
    current_function: ?c.LLVMValueRef,
    current_function_name: ?[]const u8,
    
    // Basic blocks for control flow
    current_block: ?c.LLVMBasicBlockRef,
    break_blocks: ArrayList(c.LLVMBasicBlockRef),
    continue_blocks: ArrayList(c.LLVMBasicBlockRef),
    
    pub fn init(allocator: Allocator, module_name: []const u8) !*RealLLVMBackend {
        // Initialize LLVM targets
        c.LLVMInitializeAllTargetInfos();
        c.LLVMInitializeAllTargets();
        c.LLVMInitializeAllTargetMCs();
        c.LLVMInitializeAllAsmParsers();
        c.LLVMInitializeAllAsmPrinters();
        
        const context = c.LLVMContextCreate();
        const module = c.LLVMModuleCreateWithNameInContext(module_name.ptr, context);
        const builder = c.LLVMCreateBuilderInContext(context);
        
        var self = try allocator.create(RealLLVMBackend);
        self.* = RealLLVMBackend{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .functions = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .types = std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
            .current_function_name = null,
            .current_block = null,
            .break_blocks = ArrayList(c.LLVMBasicBlockRef).init(allocator),
            .continue_blocks = ArrayList(c.LLVMBasicBlockRef).init(allocator),
        };
        
        try self.initializeBuiltinTypes();
        try self.declareRuntimeFunctions();
        
        return self;
    }
    
    pub fn deinit(self: *RealLLVMBackend) void {
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
        
        self.functions.deinit();
        self.variables.deinit();
        self.types.deinit();
        self.break_blocks.deinit();
        self.continue_blocks.deinit();
        
        self.allocator.destroy(self);
    }
    
    fn initializeBuiltinTypes(self: *RealLLVMBackend) !void {
        // CURSED basic types
        try self.types.put("drip", c.LLVMInt64TypeInContext(self.context)); // Integer
        try self.types.put("tea", c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)); // String
        try self.types.put("lit", c.LLVMInt1TypeInContext(self.context)); // Boolean
        try self.types.put("sus", c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)); // Generic pointer
        try self.types.put("void", c.LLVMVoidTypeInContext(self.context));
        
        // Array types (will be specialized later)
        try self.types.put("[]drip", c.LLVMPointerType(c.LLVMInt64TypeInContext(self.context), 0));
        try self.types.put("[]tea", c.LLVMPointerType(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), 0));
        try self.types.put("[]lit", c.LLVMPointerType(c.LLVMInt1TypeInContext(self.context), 0));
    }
    
    fn declareRuntimeFunctions(self: *RealLLVMBackend) !void {
        // vibez.spill function - printf-like
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)},
            1,
            1 // variadic
        );
        const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        try self.functions.put("printf", printf_func);
        
        // CURSED spill function
        const spill_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            null,
            0,
            1 // variadic
        );
        const spill_func = c.LLVMAddFunction(self.module, "cursed_spill", spill_type);
        try self.functions.put("cursed_spill", spill_func);
        
        // Memory management functions
        const malloc_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
            &[_]c.LLVMTypeRef{c.LLVMInt64TypeInContext(self.context)},
            1,
            0
        );
        const malloc_func = c.LLVMAddFunction(self.module, "malloc", malloc_type);
        try self.functions.put("malloc", malloc_func);
        
        const free_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)},
            1,
            0
        );
        const free_func = c.LLVMAddFunction(self.module, "free", free_type);
        try self.functions.put("free", free_func);
    }
    
    /// Compile a complete CURSED program
    pub fn compileProgram(self: *RealLLVMBackend, program: ast.Program) !void {
        std.debug.print("🔨 Compiling CURSED program to LLVM IR...\n", .{});
        
        // Compile all statements in the program
        for (program.statements) |statement| {
            try self.compileStatement(statement);
        }
        
        // Create main function if none exists
        if (self.functions.get("main") == null) {
            try self.createMainFunction();
        }
        
        std.debug.print("✅ Program compilation completed\n", .{});
    }
    
    fn createMainFunction(self: *RealLLVMBackend) !void {
        const main_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context),
            null,
            0,
            0
        );
        const main_func = c.LLVMAddFunction(self.module, "main", main_type);
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_func, "entry");
        
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Return 0 from main
        const return_val = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        _ = c.LLVMBuildRet(self.builder, return_val);
        
        try self.functions.put("main", main_func);
    }
    
    /// Compile a statement
    pub fn compileStatement(self: *RealLLVMBackend, statement: ast.Statement) !void {
        switch (statement) {
            .variable_declaration => |var_decl| try self.compileVariableDeclaration(var_decl),
            .function_declaration => |func_decl| try self.compileFunctionDeclaration(func_decl),
            .expression_statement => |expr_stmt| _ = try self.compileExpression(expr_stmt.expression),
            .if_statement => |if_stmt| try self.compileIfStatement(if_stmt),
            .while_statement => |while_stmt| try self.compileWhileStatement(while_stmt),
            .for_statement => |for_stmt| try self.compileForStatement(for_stmt),
            .return_statement => |ret_stmt| try self.compileReturnStatement(ret_stmt),
            .block_statement => |block_stmt| try self.compileBlockStatement(block_stmt),
            .break_statement => try self.compileBreakStatement(),
            .continue_statement => try self.compileContinueStatement(),
            else => {
                std.debug.print("⚠️ Unsupported statement type: {any}\n", .{statement});
            }
        }
    }
    
    /// Compile variable declaration: sus x drip = 42
    fn compileVariableDeclaration(self: *RealLLVMBackend, var_decl: ast.VariableDeclaration) !void {
        std.debug.print("🔧 Compiling variable declaration: {s}\n", .{var_decl.name});
        
        // Get the LLVM type
        const llvm_type = self.types.get(var_decl.type_name) orelse {
            std.debug.print("❌ Unknown type: {s}\n", .{var_decl.type_name});
            return error.UnknownType;
        };
        
        // Allocate space for the variable
        const alloca = c.LLVMBuildAlloca(self.builder, llvm_type, var_decl.name.ptr);
        
        // Compile initial value if provided
        if (var_decl.initializer) |init_expr| {
            const init_value = try self.compileExpression(init_expr);
            _ = c.LLVMBuildStore(self.builder, init_value, alloca);
        }
        
        // Store variable reference
        try self.variables.put(var_decl.name, alloca);
        std.debug.print("✅ Variable {s} compiled\n", .{var_decl.name});
    }
    
    /// Compile function declaration
    fn compileFunctionDeclaration(self: *RealLLVMBackend, func_decl: ast.FunctionDeclaration) !void {
        std.debug.print("🔧 Compiling function: {s}\n", .{func_decl.name});
        
        // Build parameter types
        var param_types = try self.allocator.alloc(c.LLVMTypeRef, func_decl.parameters.len);
        defer self.allocator.free(param_types);
        
        for (func_decl.parameters, 0..) |param, i| {
            param_types[i] = self.types.get(param.type_name) orelse {
                std.debug.print("❌ Unknown parameter type: {s}\n", .{param.type_name});
                return error.UnknownType;
            };
        }
        
        // Get return type
        const return_type = if (func_decl.return_type) |ret_type|
            self.types.get(ret_type) orelse c.LLVMVoidTypeInContext(self.context)
        else
            c.LLVMVoidTypeInContext(self.context);
        
        // Create function type
        const func_type = c.LLVMFunctionType(
            return_type,
            param_types.ptr,
            @intCast(param_types.len),
            0
        );
        
        // Create function
        const llvm_func = c.LLVMAddFunction(self.module, func_decl.name.ptr, func_type);
        try self.functions.put(func_decl.name, llvm_func);
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, llvm_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Store function context
        const previous_function = self.current_function;
        const previous_function_name = self.current_function_name;
        self.current_function = llvm_func;
        self.current_function_name = func_decl.name;
        self.current_block = entry_block;
        
        // Create allocas for parameters
        for (func_decl.parameters, 0..) |param, i| {
            const param_value = c.LLVMGetParam(llvm_func, @intCast(i));
            const param_alloca = c.LLVMBuildAlloca(self.builder, param_types[i], param.name.ptr);
            _ = c.LLVMBuildStore(self.builder, param_value, param_alloca);
            try self.variables.put(param.name, param_alloca);
        }
        
        // Compile function body
        try self.compileStatement(func_decl.body);
        
        // Add return void if no explicit return
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            if (func_decl.return_type == null or std.mem.eql(u8, func_decl.return_type.?, "void")) {
                _ = c.LLVMBuildRetVoid(self.builder);
            } else {
                // Return zero/null for other types
                const zero_value = c.LLVMConstNull(return_type);
                _ = c.LLVMBuildRet(self.builder, zero_value);
            }
        }
        
        // Restore previous function context
        self.current_function = previous_function;
        self.current_function_name = previous_function_name;
        
        std.debug.print("✅ Function {s} compiled\n", .{func_decl.name});
    }
    
    /// Compile expression and return LLVM value
    pub fn compileExpression(self: *RealLLVMBackend, expression: ast.Expression) !c.LLVMValueRef {
        switch (expression) {
            .integer_literal => |int_lit| {
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @intCast(int_lit.value), 0);
            },
            .string_literal => |str_lit| {
                return c.LLVMBuildGlobalStringPtr(self.builder, str_lit.value.ptr, "str");
            },
            .boolean_literal => |bool_lit| {
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_lit.value) 1 else 0, 0);
            },
            .identifier => |ident| {
                const var_ptr = self.variables.get(ident.name) orelse {
                    std.debug.print("❌ Undefined variable: {s}\n", .{ident.name});
                    return error.UndefinedVariable;
                };
                return c.LLVMBuildLoad2(self.builder, c.LLVMInt64TypeInContext(self.context), var_ptr, ident.name.ptr);
            },
            .function_call => |func_call| {
                return try self.compileFunctionCall(func_call);
            },
            .binary_operation => |bin_op| {
                return try self.compileBinaryOperation(bin_op);
            },
            .unary_operation => |un_op| {
                return try self.compileUnaryOperation(un_op);
            },
            else => {
                std.debug.print("⚠️ Unsupported expression type: {any}\n", .{expression});
                return error.UnsupportedExpression;
            }
        }
    }
    
    /// Compile function call
    fn compileFunctionCall(self: *RealLLVMBackend, func_call: ast.FunctionCall) !c.LLVMValueRef {
        std.debug.print("🔧 Compiling function call: {s}\n", .{func_call.name});
        
        // Special handling for vibez.spill
        if (std.mem.eql(u8, func_call.name, "vibez.spill")) {
            return try self.compileSpillCall(func_call.arguments);
        }
        
        const llvm_func = self.functions.get(func_call.name) orelse {
            std.debug.print("❌ Undefined function: {s}\n", .{func_call.name});
            return error.UndefinedFunction;
        };
        
        // Compile arguments
        var args = try self.allocator.alloc(c.LLVMValueRef, func_call.arguments.len);
        defer self.allocator.free(args);
        
        for (func_call.arguments, 0..) |arg, i| {
            args[i] = try self.compileExpression(arg);
        }
        
        // Get function type for call
        const func_type = c.LLVMGetElementType(c.LLVMTypeOf(llvm_func));
        
        return c.LLVMBuildCall2(
            self.builder,
            func_type,
            llvm_func,
            args.ptr,
            @intCast(args.len),
            ""
        );
    }
    
    /// Compile vibez.spill call - CURSED's print function
    fn compileSpillCall(self: *RealLLVMBackend, arguments: []ast.Expression) !c.LLVMValueRef {
        if (arguments.len == 0) return error.SpillRequiresArguments;
        
        const printf_func = self.functions.get("printf") orelse return error.PrintfNotDeclared;
        
        // For simple case, just print the first argument
        const first_arg = try self.compileExpression(arguments[0]);
        
        // Create format string based on argument type
        const format_str = switch (arguments[0]) {
            .integer_literal => c.LLVMBuildGlobalStringPtr(self.builder, "%lld\n", "fmt_int"),
            .string_literal => c.LLVMBuildGlobalStringPtr(self.builder, "%s\n", "fmt_str"),
            .boolean_literal => c.LLVMBuildGlobalStringPtr(self.builder, "%s\n", "fmt_bool"),
            else => c.LLVMBuildGlobalStringPtr(self.builder, "%p\n", "fmt_ptr"),
        };
        
        const args = [_]c.LLVMValueRef{ format_str, first_arg };
        const func_type = c.LLVMGetElementType(c.LLVMTypeOf(printf_func));
        
        return c.LLVMBuildCall2(
            self.builder,
            func_type,
            printf_func,
            &args,
            args.len,
            ""
        );
    }
    
    /// Compile binary operation (+, -, *, /, ==, !=, etc.)
    fn compileBinaryOperation(self: *RealLLVMBackend, bin_op: ast.BinaryOperation) !c.LLVMValueRef {
        const left = try self.compileExpression(bin_op.left.*);
        const right = try self.compileExpression(bin_op.right.*);
        
        return switch (bin_op.operator) {
            .add => c.LLVMBuildAdd(self.builder, left, right, "add"),
            .subtract => c.LLVMBuildSub(self.builder, left, right, "sub"),
            .multiply => c.LLVMBuildMul(self.builder, left, right, "mul"),
            .divide => c.LLVMBuildSDiv(self.builder, left, right, "div"),
            .modulo => c.LLVMBuildSRem(self.builder, left, right, "mod"),
            .equal => c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq"),
            .not_equal => c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "ne"),
            .less_than => c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt"),
            .less_equal => c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, left, right, "le"),
            .greater_than => c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt"),
            .greater_equal => c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, left, right, "ge"),
            .logical_and => c.LLVMBuildAnd(self.builder, left, right, "and"),
            .logical_or => c.LLVMBuildOr(self.builder, left, right, "or"),
            else => {
                std.debug.print("❌ Unsupported binary operator: {any}\n", .{bin_op.operator});
                return error.UnsupportedOperator;
            }
        };
    }
    
    /// Compile unary operation (-, !, etc.)
    fn compileUnaryOperation(self: *RealLLVMBackend, un_op: ast.UnaryOperation) !c.LLVMValueRef {
        const operand = try self.compileExpression(un_op.operand.*);
        
        return switch (un_op.operator) {
            .minus => c.LLVMBuildNeg(self.builder, operand, "neg"),
            .logical_not => c.LLVMBuildNot(self.builder, operand, "not"),
            else => {
                std.debug.print("❌ Unsupported unary operator: {any}\n", .{un_op.operator});
                return error.UnsupportedOperator;
            }
        };
    }
    
    /// Compile if statement
    fn compileIfStatement(self: *RealLLVMBackend, if_stmt: ast.IfStatement) !void {
        const condition = try self.compileExpression(if_stmt.condition);
        
        const current_function = self.current_function orelse return error.NoCurrentFunction;
        const then_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "if.then");
        const else_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "if.else");
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "if.merge");
        
        _ = c.LLVMBuildCondBr(self.builder, condition, then_block, else_block);
        
        // Compile then branch
        c.LLVMPositionBuilderAtEnd(self.builder, then_block);
        try self.compileStatement(if_stmt.then_branch);
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Compile else branch
        c.LLVMPositionBuilderAtEnd(self.builder, else_block);
        if (if_stmt.else_branch) |else_branch| {
            try self.compileStatement(else_branch);
        }
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Continue with merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
        self.current_block = merge_block;
    }
    
    /// Compile while loop
    fn compileWhileStatement(self: *RealLLVMBackend, while_stmt: ast.WhileStatement) !void {
        const current_function = self.current_function orelse return error.NoCurrentFunction;
        const cond_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "while.cond");
        const body_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "while.body");
        const exit_block = c.LLVMAppendBasicBlockInContext(self.context, current_function, "while.exit");
        
        // Jump to condition block
        _ = c.LLVMBuildBr(self.builder, cond_block);
        
        // Condition block
        c.LLVMPositionBuilderAtEnd(self.builder, cond_block);
        const condition = try self.compileExpression(while_stmt.condition);
        _ = c.LLVMBuildCondBr(self.builder, condition, body_block, exit_block);
        
        // Body block
        c.LLVMPositionBuilderAtEnd(self.builder, body_block);
        
        // Set up break/continue targets
        try self.break_blocks.append(exit_block);
        try self.continue_blocks.append(cond_block);
        
        try self.compileStatement(while_stmt.body);
        
        // Pop break/continue targets
        _ = self.break_blocks.pop();
        _ = self.continue_blocks.pop();
        
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, cond_block);
        }
        
        // Continue with exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
        self.current_block = exit_block;
    }
    
    /// Compile for statement (placeholder)
    fn compileForStatement(self: *RealLLVMBackend, for_stmt: ast.ForStatement) !void {
        _ = for_stmt;
        std.debug.print("⚠️ For statement compilation not yet implemented\n", .{});
    }
    
    /// Compile return statement
    fn compileReturnStatement(self: *RealLLVMBackend, ret_stmt: ast.ReturnStatement) !void {
        if (ret_stmt.expression) |expr| {
            const return_value = try self.compileExpression(expr);
            _ = c.LLVMBuildRet(self.builder, return_value);
        } else {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
    }
    
    /// Compile block statement
    fn compileBlockStatement(self: *RealLLVMBackend, block_stmt: ast.BlockStatement) !void {
        for (block_stmt.statements) |statement| {
            try self.compileStatement(statement);
        }
    }
    
    /// Compile break statement
    fn compileBreakStatement(self: *RealLLVMBackend) !void {
        if (self.break_blocks.items.len == 0) {
            return error.BreakOutsideLoop;
        }
        const break_block = self.break_blocks.items[self.break_blocks.items.len - 1];
        _ = c.LLVMBuildBr(self.builder, break_block);
    }
    
    /// Compile continue statement
    fn compileContinueStatement(self: *RealLLVMBackend) !void {
        if (self.continue_blocks.items.len == 0) {
            return error.ContinueOutsideLoop;
        }
        const continue_block = self.continue_blocks.items[self.continue_blocks.items.len - 1];
        _ = c.LLVMBuildBr(self.builder, continue_block);
    }
    
    /// Generate LLVM IR as string
    pub fn generateIR(self: *RealLLVMBackend) ![]const u8 {
        const ir_string = c.LLVMPrintModuleToString(self.module);
        defer c.LLVMDisposeMessage(ir_string);
        
        const ir_len = std.mem.len(ir_string);
        var result = try self.allocator.alloc(u8, ir_len);
        @memcpy(result, ir_string[0..ir_len]);
        return result;
    }
    
    /// Compile to native binary
    pub fn compileToNative(self: *RealLLVMBackend, output_path: []const u8) !void {
        std.debug.print("🔧 Compiling to native binary: {s}\n", .{output_path});
        
        // Get default target
        var target_triple = c.LLVMGetDefaultTargetTriple();
        defer c.LLVMDisposeMessage(target_triple);
        
        // Initialize target
        var target: c.LLVMTargetRef = undefined;
        var error_message: [*c]u8 = undefined;
        
        if (c.LLVMGetTargetFromTriple(target_triple, &target, &error_message) != 0) {
            defer c.LLVMDisposeMessage(error_message);
            std.debug.print("❌ Failed to get target: {s}\n", .{error_message});
            return error.TargetError;
        }
        
        // Create target machine
        const target_machine = c.LLVMCreateTargetMachine(
            target,
            target_triple,
            c.LLVMGetHostCPUName(),
            c.LLVMGetHostCPUFeatures(),
            c.LLVMCodeGenLevelDefault,
            c.LLVMRelocDefault,
            c.LLVMCodeModelDefault
        );
        defer c.LLVMDisposeTargetMachine(target_machine);
        
        // Emit object file
        const obj_path = try std.fmt.allocPrint(self.allocator, "{s}.o", .{output_path});
        defer self.allocator.free(obj_path);
        
        if (c.LLVMTargetMachineEmitToFile(
            target_machine,
            self.module,
            obj_path.ptr,
            c.LLVMObjectFile,
            &error_message
        ) != 0) {
            defer c.LLVMDisposeMessage(error_message);
            std.debug.print("❌ Failed to emit object file: {s}\n", .{error_message});
            return error.EmitError;
        }
        
        // Link with clang
        const link_cmd = try std.fmt.allocPrint(
            self.allocator,
            "clang {s} -o {s}",
            .{ obj_path, output_path }
        );
        defer self.allocator.free(link_cmd);
        
        const result = std.ChildProcess.exec(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{ "sh", "-c", link_cmd },
        }) catch |err| {
            std.debug.print("❌ Failed to execute linker: {any}\n", .{err});
            return error.LinkError;
        };
        
        if (result.term != .Exited or result.term.Exited != 0) {
            std.debug.print("❌ Linker failed: {s}\n", .{result.stderr});
            return error.LinkError;
        }
        
        std.debug.print("✅ Native binary compiled successfully: {s}\n", .{output_path});
    }
    
    /// Optimize the module
    pub fn optimize(self: *RealLLVMBackend, optimization_level: u32) !void {
        const pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(pass_manager);
        
        // Add optimization passes based on level
        if (optimization_level >= 1) {
            c.LLVMAddInstructionCombiningPass(pass_manager);
            c.LLVMAddReassociatePass(pass_manager);
            c.LLVMAddGVNPass(pass_manager);
            c.LLVMAddCFGSimplificationPass(pass_manager);
        }
        
        if (optimization_level >= 2) {
            c.LLVMAddFunctionInliningPass(pass_manager);
            c.LLVMAddDeadCodeEliminationPass(pass_manager);
        }
        
        if (optimization_level >= 3) {
            c.LLVMAddAggressiveDCEPass(pass_manager);
        }
        
        // Run the passes
        _ = c.LLVMRunPassManager(pass_manager, self.module);
        
        std.debug.print("✅ Optimization level {d} applied\n", .{optimization_level});
    }
};

// Test function for real LLVM backend
pub fn testRealLLVMBackend() !void {
    std.debug.print("🧪 Testing Real LLVM Backend...\n", .{});
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Create backend
    var backend = try RealLLVMBackend.init(allocator, "test_module");
    defer backend.deinit();
    
    // Test IR generation
    const ir = try backend.generateIR();
    defer allocator.free(ir);
    
    std.debug.print("Generated IR:\n{s}\n", .{ir});
    std.debug.print("✅ Real LLVM Backend test completed\n", .{});
}
