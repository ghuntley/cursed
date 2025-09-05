const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Import AST and components
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const llvm_ir = @import("llvm_ir_pipeline.zig");

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

/// CURSED→LLVM Compiler for native stdlib compilation
/// Extends LLVM IR Pipeline to compile CURSED functions directly to IR
pub const CursedLLVMCompiler = struct {
    allocator: Allocator,
    arena: std.heap.ArenaAllocator,
    
    // Base LLVM pipeline
    pipeline: *llvm_ir.LLVMIRPipeline,
    
    // CURSED module management
    cursed_modules: HashMap([]const u8, CursedModule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Function registry for CURSED functions
    cursed_functions: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub const CursedModule = struct {
        name: []const u8,
        ast_program: ast.Program,
        functions: HashMap([]const u8, ast.FunctionStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        loaded: bool,
    };

    const CursedCompilerError = error{
        ModuleNotFound,
        ModuleParseFailed,
        FunctionNotFound,
        CompilationFailed,
        OutOfMemory,
    };

    pub fn init(allocator: Allocator, base_pipeline: *llvm_ir.LLVMIRPipeline) !*CursedLLVMCompiler {
        var arena = std.heap.ArenaAllocator.init(allocator);
        
        const compiler = try allocator.create(CursedLLVMCompiler);
        compiler.* = CursedLLVMCompiler{
            .allocator = allocator,
            .arena = arena,
            .pipeline = base_pipeline,
            .cursed_modules = HashMap([]const u8, CursedModule, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .cursed_functions = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
        
        print("🚀 CURSED→LLVM Compiler initialized\n", .{});
        return compiler;
    }
    
    pub fn deinit(self: *CursedLLVMCompiler) void {
        print("🧹 Cleaning up CURSED→LLVM Compiler...\n", .{});
        
        // Clean up modules
        var module_iter = self.cursed_modules.iterator();
        while (module_iter.next()) |entry| {
            var module = entry.value_ptr;
            module.functions.deinit();
        }
        self.cursed_modules.deinit();
        self.cursed_functions.deinit();
        
        self.arena.deinit();
        self.allocator.destroy(self);
        
        print("✅ CURSED→LLVM Compiler cleanup complete\n", .{});
    }

    /// Load and compile a CURSED stdlib module 
    pub fn loadCursedModule(self: *CursedLLVMCompiler, module_name: []const u8) !void {
        print("📦 Loading CURSED module: {s}\n", .{module_name});
        
        // Check if already loaded
        if (self.cursed_modules.contains(module_name)) {
            print("✅ Module {s} already loaded\n", .{module_name});
            return;
        }
        
        // Build module path
        const module_path = try std.fmt.allocPrint(self.allocator, "stdlib/{s}/mod.💀", .{module_name});
        defer self.allocator.free(module_path);
        
        // Read CURSED source
        const source = std.fs.cwd().readFileAlloc(self.allocator, module_path, 1024 * 1024) catch |err| {
            print("❌ Failed to read {s}: {}\n", .{module_path, err});
            return CursedCompilerError.ModuleNotFound;
        };
        defer self.allocator.free(source);
        
        // Parse CURSED source to AST
        print("🌳 Parsing CURSED module: {s}\n", .{module_name});
        var lex = lexer.Lexer.init(self.allocator, source);
        const token_list = lex.tokenize() catch |err| {
            print("❌ Tokenization failed: {}\n", .{err});
            return CursedCompilerError.ModuleParseFailed;
        };
        
        var parse = parser.Parser.init(self.allocator, token_list.items);
        defer parse.deinit();
        const program = parse.parseProgram() catch |err| {
            print("❌ Parsing failed: {}\n", .{err});
            return CursedCompilerError.ModuleParseFailed;
        };
        
        // Create module structure
        var module = CursedModule{
            .name = try self.arena.allocator().dupe(u8, module_name),
            .ast_program = program,
            .functions = HashMap([]const u8, ast.FunctionStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator),
            .loaded = false,
        };
        
        // Extract function definitions
        print("🔍 Extracting functions from {s}\n", .{module_name});
        for (program.statements.items) |stmt_ptr| {
            const stmt = @as(*ast.Statement, @alignCast(@ptrCast(stmt_ptr)));
            switch (stmt.*) {
                .Function => |func_decl| {
                    const func_name = try self.arena.allocator().dupe(u8, func_decl.name);
                    try module.functions.put(func_name, func_decl);
                    print("  📋 Found function: {s}\n", .{func_name});
                },
                else => {
                    // Skip non-function statements
                }
            }
        }
        
        module.loaded = true;
        try self.cursed_modules.put(try self.arena.allocator().dupe(u8, module_name), module);
        
        print("✅ Successfully loaded CURSED module: {s} with {} functions\n", .{module_name, module.functions.count()});
    }

    /// Compile a CURSED function to LLVM IR
    pub fn compileCursedFunction(self: *CursedLLVMCompiler, module_name: []const u8, func_name: []const u8) !c.LLVMValueRef {
        print("⚡ Compiling CURSED function: {s}.{s}\n", .{module_name, func_name});
        
        // Build qualified function name
        const qualified_name = try std.fmt.allocPrint(self.arena.allocator(), "{s}_{s}", .{module_name, func_name});
        
        // Check if already compiled
        if (self.cursed_functions.get(qualified_name)) |existing_func| {
            print("✅ Function {s} already compiled\n", .{qualified_name});
            return existing_func;
        }
        
        // Load module if needed
        if (!self.cursed_modules.contains(module_name)) {
            try self.loadCursedModule(module_name);
        }
        
        // Get module and function
        const module = self.cursed_modules.get(module_name).?;
        const func_decl = module.functions.get(func_name) orelse {
            print("❌ Function {s} not found in module {s}\n", .{func_name, module_name});
            return CursedCompilerError.FunctionNotFound;
        };
        
        // Generate LLVM function
        const llvm_func = try self.generateLLVMFunction(func_decl, qualified_name);
        
        // Register compiled function
        try self.cursed_functions.put(qualified_name, llvm_func);
        
        print("✅ Successfully compiled: {s}\n", .{qualified_name});
        return llvm_func;
    }
    
    /// Generate LLVM IR for a CURSED function
    fn generateLLVMFunction(self: *CursedLLVMCompiler, func_decl: ast.FunctionStatement, qualified_name: []const u8) !c.LLVMValueRef {
        print("🔧 Generating LLVM IR for: {s}\n", .{qualified_name});
        
        // Create function type
        var param_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer param_types.deinit();
        
        for (func_decl.parameters.items) |param| {
            const llvm_type = try self.cursedTypeToLLVM(param.param_type);
            try param_types.append(llvm_type);
        }
        
        const return_type = if (func_decl.return_type) |ret_type|
            try self.cursedTypeToLLVM(ret_type)
        else
            c.LLVMVoidTypeInContext(self.pipeline.context);
        
        const func_type = c.LLVMFunctionType(
            return_type,
            param_types.items.ptr,
            @intCast(param_types.items.len),
            0
        );
        
        // Create function
        const func_name_z = try self.arena.allocator().dupeZ(u8, qualified_name);
        const function = c.LLVMAddFunction(self.pipeline.module, func_name_z.ptr, func_type);
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.pipeline.context, function, "entry");
        c.LLVMPositionBuilderAtEnd(self.pipeline.builder, entry_block);
        
        // Save current pipeline state
        const previous_function = self.pipeline.current_function;
        const previous_variables = try self.cloneVariableMap();
        defer self.restoreVariableMap(previous_variables);
        
        self.pipeline.current_function = function;
        
        // Generate function parameters
        for (func_decl.parameters.items, 0..) |param, i| {
            const llvm_param = c.LLVMGetParam(function, @intCast(i));
            const param_name_z = try self.arena.allocator().dupeZ(u8, param.name);
            c.LLVMSetValueName(llvm_param, param_name_z.ptr);
            
            // Create alloca for parameter and store value
            const param_alloca = c.LLVMBuildAlloca(self.pipeline.builder, try self.cursedTypeToLLVM(param.param_type), param_name_z.ptr);
            _ = c.LLVMBuildStore(self.pipeline.builder, llvm_param, param_alloca);
            try self.pipeline.variables.put(param.name, param_alloca);
        }
        
        // Generate function body
        for (func_decl.body.items) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateCursedStatement(stmt.*);
        }
        
        // Add return if not present
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.pipeline.builder)) == null) {
            if (func_decl.return_type != null) {
                // Return zero/null for non-void functions
                const zero_val = c.LLVMConstInt(return_type, 0, 0);
                _ = c.LLVMBuildRet(self.pipeline.builder, zero_val);
            } else {
                _ = c.LLVMBuildRetVoid(self.pipeline.builder);
            }
        }
        
        // Restore pipeline state
        self.pipeline.current_function = previous_function;
        
        print("✅ LLVM function generated: {s}\n", .{qualified_name});
        return function;
    }
    
    /// Generate LLVM IR for CURSED statements
    fn generateCursedStatement(self: *CursedLLVMCompiler, stmt: ast.Statement) !void {
        switch (stmt) {
            .Let => |var_decl| {
                try self.generateCursedVariableDeclaration(var_decl);
            },
            .Expression => |expr| {
                _ = try self.generateCursedExpression(expr);
            },
            .Return => |ret_stmt| {
                if (ret_stmt.expression) |ret_expr| {
                    const ret_val = try self.generateCursedExpression(ret_expr.*);
                    _ = c.LLVMBuildRet(self.pipeline.builder, ret_val);
                } else {
                    _ = c.LLVMBuildRetVoid(self.pipeline.builder);
                }
            },
            .If => |if_stmt| {
                try self.generateCursedIfStatement(if_stmt);
            },
            .While => |while_stmt| {
                try self.generateCursedWhileStatement(while_stmt);
            },
            .Assignment => |assign_stmt| {
                try self.generateCursedAssignment(assign_stmt);
            },
            else => {
                print("⚠️ Unhandled CURSED statement type in IR generation\n", .{});
            },
        }
    }
    
    /// Generate LLVM IR for CURSED expressions  
    fn generateCursedExpression(self: *CursedLLVMCompiler, expr: ast.Expression) !c.LLVMValueRef {
        switch (expr) {
            .Literal => |lit| {
                return try self.generateCursedLiteral(lit);
            },
            .Identifier => |ident| {
                if (self.pipeline.variables.get(ident)) |var_ref| {
                    const var_type = c.LLVMGetAllocatedType(var_ref);
                    if (var_type != null) {
                        return c.LLVMBuildLoad2(self.pipeline.builder, var_type, var_ref, "load_tmp");
                    } else {
                        return llvm_ir.LLVMIRError.UndefinedVariable;
                    }
                } else {
                    print("❌ Undefined variable in CURSED function: {s}\n", .{ident});
                    return llvm_ir.LLVMIRError.UndefinedVariable;
                }
            },
            .Integer => |int_val| {
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.pipeline.context), @bitCast(int_val), 0);
            },
            .Float => |float_val| {
                return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.pipeline.context), float_val);
            },
            .Boolean => |bool_val| {
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.pipeline.context), if (bool_val) 1 else 0, 0);
            },
            .Binary => |bin_op| {
                return try self.generateCursedBinaryOperation(bin_op);
            },
            .Call => |call| {
                return try self.generateCursedFunctionCall(call);
            },
            else => {
                print("⚠️ Unhandled CURSED expression type in IR generation\n", .{});
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.pipeline.context), 0, 0);
            },
        }
    }
    
    /// Generate binary operations for CURSED code
    fn generateCursedBinaryOperation(self: *CursedLLVMCompiler, bin_op: ast.BinaryOperationExpression) !c.LLVMValueRef {
        const left = try self.generateCursedExpression(bin_op.left.*);
        const right = try self.generateCursedExpression(bin_op.right.*);
        
        switch (bin_op.operator) {
            .Plus => return c.LLVMBuildAdd(self.pipeline.builder, left, right, "add_tmp"),
            .Minus => return c.LLVMBuildSub(self.pipeline.builder, left, right, "sub_tmp"),
            .Star => return c.LLVMBuildMul(self.pipeline.builder, left, right, "mul_tmp"),
            .Slash => return c.LLVMBuildSDiv(self.pipeline.builder, left, right, "div_tmp"),
            .Percent => return c.LLVMBuildSRem(self.pipeline.builder, left, right, "mod_tmp"),
            .EqualEqual => return c.LLVMBuildICmp(self.pipeline.builder, c.LLVMIntEQ, left, right, "eq_tmp"),
            .BangEqual => return c.LLVMBuildICmp(self.pipeline.builder, c.LLVMIntNE, left, right, "ne_tmp"),
            .Less => return c.LLVMBuildICmp(self.pipeline.builder, c.LLVMIntSLT, left, right, "lt_tmp"),
            .LessEqual => return c.LLVMBuildICmp(self.pipeline.builder, c.LLVMIntSLE, left, right, "le_tmp"),
            .Greater => return c.LLVMBuildICmp(self.pipeline.builder, c.LLVMIntSGT, left, right, "gt_tmp"),
            .GreaterEqual => return c.LLVMBuildICmp(self.pipeline.builder, c.LLVMIntSGE, left, right, "ge_tmp"),
            else => {
                print("⚠️ Unhandled binary operator in CURSED function\n", .{});
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.pipeline.context), 0, 0);
            }
        }
    }
    
    /// Generate variable declarations for CURSED code
    fn generateCursedVariableDeclaration(self: *CursedLLVMCompiler, var_decl: ast.LetStatement) !void {
        const llvm_type = if (var_decl.var_type) |vtype| 
            try self.cursedTypeToLLVM(vtype)
        else 
            c.LLVMInt64TypeInContext(self.pipeline.context);
        const var_name_z = try self.arena.allocator().dupeZ(u8, var_decl.name);
        
        // Create alloca
        const alloca = c.LLVMBuildAlloca(self.pipeline.builder, llvm_type, var_name_z.ptr);
        try self.pipeline.variables.put(var_decl.name, alloca);
        
        // Generate initializer if present
        if (var_decl.initializer) |initializer| {
            const init_value = try self.generateCursedExpression(initializer.*);
            _ = c.LLVMBuildStore(self.pipeline.builder, init_value, alloca);
        }
    }
    
    /// Generate if statements for CURSED code  
    fn generateCursedIfStatement(self: *CursedLLVMCompiler, if_stmt: ast.IfStatement) !void {
        const condition = try self.generateCursedExpression(if_stmt.condition.*);
        
        // Create blocks
        const then_block = c.LLVMAppendBasicBlockInContext(self.pipeline.context, self.pipeline.current_function.?, "if_then");
        const else_block = c.LLVMAppendBasicBlockInContext(self.pipeline.context, self.pipeline.current_function.?, "if_else");
        const cont_block = c.LLVMAppendBasicBlockInContext(self.pipeline.context, self.pipeline.current_function.?, "if_cont");
        
        // Branch on condition
        _ = c.LLVMBuildCondBr(self.pipeline.builder, condition, then_block, else_block);
        
        // Generate then block
        c.LLVMPositionBuilderAtEnd(self.pipeline.builder, then_block);
        try self.generateCursedStatement(if_stmt.then_branch.*);
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.pipeline.builder)) == null) {
            _ = c.LLVMBuildBr(self.pipeline.builder, cont_block);
        }
        
        // Generate else block
        c.LLVMPositionBuilderAtEnd(self.pipeline.builder, else_block);
        if (if_stmt.else_branch) |else_branch| {
            try self.generateCursedStatement(else_branch.*);
        }
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.pipeline.builder)) == null) {
            _ = c.LLVMBuildBr(self.pipeline.builder, cont_block);
        }
        
        // Continue block
        c.LLVMPositionBuilderAtEnd(self.pipeline.builder, cont_block);
    }
    
    /// Generate while loops for CURSED code
    fn generateCursedWhileStatement(self: *CursedLLVMCompiler, while_stmt: ast.WhileStatement) !void {
        const loop_block = c.LLVMAppendBasicBlockInContext(self.pipeline.context, self.pipeline.current_function.?, "while_loop");
        const body_block = c.LLVMAppendBasicBlockInContext(self.pipeline.context, self.pipeline.current_function.?, "while_body");
        const cont_block = c.LLVMAppendBasicBlockInContext(self.pipeline.context, self.pipeline.current_function.?, "while_cont");
        
        // Jump to loop block
        _ = c.LLVMBuildBr(self.pipeline.builder, loop_block);
        
        // Generate loop condition
        c.LLVMPositionBuilderAtEnd(self.pipeline.builder, loop_block);
        const condition = try self.generateCursedExpression(while_stmt.condition.*);
        _ = c.LLVMBuildCondBr(self.pipeline.builder, condition, body_block, cont_block);
        
        // Generate body
        c.LLVMPositionBuilderAtEnd(self.pipeline.builder, body_block);
        try self.generateCursedStatement(while_stmt.body.*);
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.pipeline.builder)) == null) {
            _ = c.LLVMBuildBr(self.pipeline.builder, loop_block);
        }
        
        // Continue block
        c.LLVMPositionBuilderAtEnd(self.pipeline.builder, cont_block);
    }
    
    /// Generate assignment statements for CURSED code
    fn generateCursedAssignment(self: *CursedLLVMCompiler, assign_stmt: ast.AssignmentStatement) !void {
        const value = try self.generateCursedExpression(assign_stmt.value.*);
        
        if (self.pipeline.variables.get(assign_stmt.variable)) |var_alloca| {
            _ = c.LLVMBuildStore(self.pipeline.builder, value, var_alloca);
        } else {
            print("❌ Undefined variable in assignment: {s}\n", .{assign_stmt.variable});
            return llvm_ir.LLVMIRError.UndefinedVariable;
        }
    }
    
    /// Generate function calls for CURSED code
    fn generateCursedFunctionCall(self: *CursedLLVMCompiler, call: ast.CallExpression) !c.LLVMValueRef {
        // For now, handle basic function calls
        // This would need to be extended to handle module function calls
        if (self.pipeline.functions.get(call.function_name)) |func| {
            var args = ArrayList(c.LLVMValueRef).init(self.allocator);
            defer args.deinit();
            
            for (call.arguments.items) |arg| {
                const arg_val = try self.generateCursedExpression(arg.*);
                try args.append(arg_val);
            }
            
            const func_type = c.LLVMGlobalGetValueType(func);
            return c.LLVMBuildCall2(self.pipeline.builder, func_type, func, args.items.ptr, @intCast(args.items.len), "call_result");
        }
        
        // Default fallback
        return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.pipeline.context), 0, 0);
    }
    
    /// Generate CURSED literals
    fn generateCursedLiteral(self: *CursedLLVMCompiler, lit: ast.LiteralExpression) !c.LLVMValueRef {
        switch (lit) {
            .Integer => |int_val| {
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.pipeline.context), @bitCast(int_val), 0);
            },
            .Float => |float_val| {
                return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.pipeline.context), float_val);
            },
            .String => |str_val| {
                return c.LLVMBuildGlobalStringPtr(self.pipeline.builder, try self.arena.allocator().dupeZ(u8, str_val), "str_literal");
            },
            .Boolean => |bool_val| {
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.pipeline.context), if (bool_val) 1 else 0, 0);
            },
            .Character => |char_val| {
                return c.LLVMConstInt(c.LLVMInt8TypeInContext(self.pipeline.context), char_val, 0);
            },
        }
    }
    
    /// Convert CURSED type to LLVM type (delegate to pipeline)
    fn cursedTypeToLLVM(self: *CursedLLVMCompiler, cursed_type: ast.Type) !c.LLVMTypeRef {
        return self.pipeline.cursedTypeToLLVM(cursed_type);
    }
    
    /// Get a CURSED function for LLVM compilation (instead of runtime call)
    pub fn getCursedFunction(self: *CursedLLVMCompiler, module_name: []const u8, func_name: []const u8) !c.LLVMValueRef {
        return self.compileCursedFunction(module_name, func_name);
    }
    
    /// Utility functions for variable map management
    fn cloneVariableMap(self: *CursedLLVMCompiler) !HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage) {
        var cloned = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        var iter = self.pipeline.variables.iterator();
        while (iter.next()) |entry| {
            try cloned.put(entry.key_ptr.*, entry.value_ptr.*);
        }
        return cloned;
    }
    
    fn restoreVariableMap(self: *CursedLLVMCompiler, previous_map: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) void {
        self.pipeline.variables.deinit();
        self.pipeline.variables = previous_map;
    }
};
