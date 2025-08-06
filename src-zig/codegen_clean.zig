const std = @import("std");
const ast = @import("ast.zig");
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
});

const CodeGenError = error{
    UndefinedSymbol,
    TypeMismatch,
    InvalidOperation,
    UnknownType,
    LLVMError,
    CompilationError,
    LinkerError,
    OutOfMemory,
};

pub const CodeGenerator = struct {
    allocator: std.mem.Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    variables: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn init(allocator: std.mem.Allocator) !*CodeGenerator {
        const self = try allocator.create(CodeGenerator);
        
        self.allocator = allocator;
        self.context = c.LLVMContextCreate();
        self.module = c.LLVMModuleCreateWithNameInContext("cursed_module", self.context);
        self.builder = c.LLVMCreateBuilderInContext(self.context);
        self.variables = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
        
        // Set up runtime functions needed for compilation
        try self.setupRuntimeFunctions();
        
        return self;
    }

    /// Set up essential runtime functions like printf, malloc, etc.
    fn setupRuntimeFunctions(self: *CodeGenerator) !void {
        // printf function declaration
        const i32_type = c.LLVMInt32TypeInContext(self.context);
        const i8_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        
        var printf_params = [_]c.LLVMTypeRef{i8_ptr_type};
        const printf_type = c.LLVMFunctionType(i32_type, &printf_params, 1, 1); // variadic
        _ = c.LLVMAddFunction(self.module, "printf", printf_type);
        
        // puts function declaration (simpler alternative)
        const puts_type = c.LLVMFunctionType(i32_type, &printf_params, 1, 0);
        _ = c.LLVMAddFunction(self.module, "puts", puts_type);
        
        // malloc function declaration
        const size_t_type = c.LLVMInt64TypeInContext(self.context);
        var malloc_params = [_]c.LLVMTypeRef{size_t_type};
        const malloc_type = c.LLVMFunctionType(i8_ptr_type, &malloc_params, 1, 0);
        _ = c.LLVMAddFunction(self.module, "malloc", malloc_type);
        
        // free function declaration
        var free_params = [_]c.LLVMTypeRef{i8_ptr_type};
        const free_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), &free_params, 1, 0);
        _ = c.LLVMAddFunction(self.module, "free", free_type);
    }

    pub fn deinit(self: *CodeGenerator) void {
        self.variables.deinit();
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
        self.allocator.destroy(self);
    }

    pub fn generateProgram(self: *CodeGenerator, program: ast.Program) !void {
        // Create main function that wraps the program
        const i32_type = c.LLVMInt32TypeInContext(self.context);
        const main_type = c.LLVMFunctionType(i32_type, null, 0, 0);
        const main_function = c.LLVMAddFunction(self.module, "main", main_type);
        
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Generate all statements inside main function
        for (program.statements) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Return 0 from main
        const return_value = c.LLVMConstInt(i32_type, 0, 0);
        _ = c.LLVMBuildRet(self.builder, return_value);
    }

    fn generateStatement(self: *CodeGenerator, stmt: ast.Statement) !void {
        switch (stmt) {
            .Function => |func| {
                try self.generateFunction(func);
            },
            .Variable => |var_decl| {
                try self.generateVariable(var_decl);
            },
            .Expression => |expr| {
                _ = try self.generateExpression(expr);
            },
            .If => |if_stmt| {
                try self.generateIfStatement(if_stmt);
            },
            .While => |while_stmt| {
                try self.generateWhileStatement(while_stmt);
            },
            .For => |for_stmt| {
                try self.generateForStatement(for_stmt);
            },
            .Return => |return_stmt| {
                try self.generateReturnStatement(return_stmt);
            },
            .Block => |block| {
                for (block.statements.items) |block_stmt| {
                    try self.generateStatement(block_stmt.*);
                }
            },
            .Struct => |struct_stmt| {
                try self.generateStructStatement(struct_stmt);
            },
            .Interface => |interface_stmt| {
                try self.generateInterfaceStatement(interface_stmt);
            },
            .Implementation => |impl_stmt| {
                try self.generateImplementationStatement(impl_stmt);
            },
            .Match => |match_stmt| {
                try self.generateMatchStatement(match_stmt);
            },
            .Defer => |defer_stmt| {
                try self.generateDeferStatement(defer_stmt);
            },
            .Try => |try_stmt| {
                try self.generateTryStatement(try_stmt);
            },
            .Goroutine => |goroutine_stmt| {
                try self.generateGoroutineStatement(goroutine_stmt);
            },
            .Select => |select_stmt| {
                try self.generateSelectStatement(select_stmt);
            },
            .Break => |break_stmt| {
                try self.generateBreakStatement(break_stmt);
            },
            .Continue => |continue_stmt| {
                try self.generateContinueStatement(continue_stmt);
            },
            .Constant => |const_stmt| {
                try self.generateConstantStatement(const_stmt);
            },
            .Type => |type_stmt| {
                try self.generateTypeStatement(type_stmt);
            },
            .Import => {
                // Import statements don't generate code
            },
            else => {
                std.debug.print("Warning: Unimplemented statement type in codegen\n", .{});
            },
        }
    }

    fn generateIfStatement(self: *CodeGenerator, if_stmt: ast.IfStatementData) !void {
        const condition_value = try self.generateExpression(if_stmt.condition.*);
        
        // Get current function
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Create basic blocks
        const then_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "if.then");
        const else_block = if (if_stmt.else_branch) |_| 
            c.LLVMAppendBasicBlockInContext(self.context, current_func, "if.else") 
        else 
            null;
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "if.merge");
        
        // Generate conditional branch
        if (else_block) |else_bb| {
            _ = c.LLVMBuildCondBr(self.builder, condition_value, then_block, else_bb);
        } else {
            _ = c.LLVMBuildCondBr(self.builder, condition_value, then_block, merge_block);
        }
        
        // Generate then block
        c.LLVMPositionBuilderAtEnd(self.builder, then_block);
        for (if_stmt.then_branch.items) |stmt| {
            try self.generateStatement(stmt.*);
        }
        // Only add branch if block doesn't end with return/break
        if (!c.LLVMGetBasicBlockTerminator(then_block)) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Generate else block if present
        if (if_stmt.else_branch) |else_stmts| {
            c.LLVMPositionBuilderAtEnd(self.builder, else_block.?);
            for (else_stmts.items) |stmt| {
                try self.generateStatement(stmt.*);
            }
            if (!c.LLVMGetBasicBlockTerminator(else_block.?)) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }
        
        // Continue with merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
    }

    fn generateWhileStatement(self: *CodeGenerator, while_stmt: ast.WhileStatementData) !void {
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        const loop_header = c.LLVMAppendBasicBlockInContext(self.context, current_func, "while.header");
        const loop_body = c.LLVMAppendBasicBlockInContext(self.context, current_func, "while.body");
        const loop_exit = c.LLVMAppendBasicBlockInContext(self.context, current_func, "while.exit");
        
        // Jump to header
        _ = c.LLVMBuildBr(self.builder, loop_header);
        
        // Generate header with condition
        c.LLVMPositionBuilderAtEnd(self.builder, loop_header);
        const condition_value = try self.generateExpression(while_stmt.condition.*);
        _ = c.LLVMBuildCondBr(self.builder, condition_value, loop_body, loop_exit);
        
        // Generate body
        c.LLVMPositionBuilderAtEnd(self.builder, loop_body);
        for (while_stmt.body.items) |stmt| {
            try self.generateStatement(stmt.*);
        }
        if (!c.LLVMGetBasicBlockTerminator(loop_body)) {
            _ = c.LLVMBuildBr(self.builder, loop_header);
        }
        
        // Continue with exit block
        c.LLVMPositionBuilderAtEnd(self.builder, loop_exit);
    }

    fn generateForStatement(self: *CodeGenerator, for_stmt: ast.ForStatementData) !void {
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Generate init statement
        if (for_stmt.init) |for_init| {
            try self.generateStatement(for_init.*);
        }
        
        const loop_header = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for.header");
        const loop_body = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for.body");
        const loop_update = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for.update");
        const loop_exit = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for.exit");
        
        // Jump to header
        _ = c.LLVMBuildBr(self.builder, loop_header);
        
        // Generate header with condition
        c.LLVMPositionBuilderAtEnd(self.builder, loop_header);
        if (for_stmt.condition) |condition| {
            const condition_value = try self.generateExpression(condition.*);
            _ = c.LLVMBuildCondBr(self.builder, condition_value, loop_body, loop_exit);
        } else {
            // Infinite loop if no condition
            _ = c.LLVMBuildBr(self.builder, loop_body);
        }
        
        // Generate body
        c.LLVMPositionBuilderAtEnd(self.builder, loop_body);
        for (for_stmt.body.items) |stmt| {
            try self.generateStatement(stmt.*);
        }
        if (!c.LLVMGetBasicBlockTerminator(loop_body)) {
            _ = c.LLVMBuildBr(self.builder, loop_update);
        }
        
        // Generate update
        c.LLVMPositionBuilderAtEnd(self.builder, loop_update);
        if (for_stmt.update) |update| {
            try self.generateStatement(update.*);
        }
        if (!c.LLVMGetBasicBlockTerminator(loop_update)) {
            _ = c.LLVMBuildBr(self.builder, loop_header);
        }
        
        // Continue with exit block
        c.LLVMPositionBuilderAtEnd(self.builder, loop_exit);
    }

    fn generateReturnStatement(self: *CodeGenerator, return_stmt: ast.ReturnStatementData) !void {
        if (return_stmt.value) |value| {
            const return_value = try self.generateExpression(value.*);
            _ = c.LLVMBuildRet(self.builder, return_value);
        } else {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
    }

    fn generateFunction(self: *CodeGenerator, func: ast.FunctionStatement) !void {
        // Create function type
        const return_type = self.getLLVMType("normie"); // Default to int for now
        const param_types = try self.allocator.alloc(c.LLVMTypeRef, func.parameters.len);
        defer self.allocator.free(param_types);
        
        for (func.parameters, 0..) |param, i| {
            param_types[i] = self.getLLVMType(param.type_name);
        }
        
        const func_type = c.LLVMFunctionType(return_type, param_types.ptr, @as(c_uint, @intCast(param_types.len)), 0);
        const llvm_func = c.LLVMAddFunction(self.module, func.name.ptr, func_type);
        
        // Create basic block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, llvm_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Generate function body
        for (func.body) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Add return if not present
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
    }

    fn generateVariable(self: *CodeGenerator, var_decl: ast.VariableStatement) !void {
        const llvm_type = self.getLLVMType(var_decl.type_name);
        const alloca = c.LLVMBuildAlloca(self.builder, llvm_type, var_decl.name.ptr);
        
        if (var_decl.value) |value| {
            const init_value = try self.generateExpression(value);
            _ = c.LLVMBuildStore(self.builder, init_value, alloca);
        }
        
        try self.variables.put(var_decl.name, alloca);
    }

    /// Generate LLVM IR for expressions
    fn generateExpression(self: *CodeGenerator, expr: ast.Expression) !c.LLVMValueRef {
        switch (expr) {
            .Literal => |literal| {
                return try self.generateLiteral(literal);
            },
            .Identifier => |identifier| {
                return try self.generateIdentifier(identifier);
            },
            .Binary => |binary| {
                return try self.generateBinaryExpression(binary);
            },
            .Unary => |unary| {
                return try self.generateUnaryExpression(unary);
            },
            .Call => |call| {
                return try self.generateCallExpression(call);
            },
            .Access => |access| {
                return try self.generateAccessExpression(access);
            },
            .Index => |index| {
                return try self.generateIndexExpression(index);
            },
            .Assignment => |assignment| {
                return try self.generateAssignmentExpression(assignment);
            },
            .Array => |array| {
                return try self.generateArrayExpression(array);
            },
            .Map => |map| {
                return try self.generateMapExpression(map);
            },
            .Tuple => |tuple| {
                return try self.generateTupleExpression(tuple);
            },
            .TypeAssertion => |type_assertion| {
                return try self.generateTypeAssertionExpression(type_assertion);
            },
            .Lambda => |lambda| {
                return try self.generateLambdaExpression(lambda);
            },
            else => {
                std.debug.print("Warning: Unimplemented expression type in codegen\n", .{});
                // Return a placeholder value for unimplemented expressions
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            },
        }
    }

    /// Generate literal values
    fn generateLiteral(self: *CodeGenerator, literal: ast.Literal) !c.LLVMValueRef {
        switch (literal) {
            .Integer => |int_val| {
                const int_type = c.LLVMInt32TypeInContext(self.context);
                return c.LLVMConstInt(int_type, @intCast(int_val), 0);
            },
            .Float => |float_val| {
                const float_type = c.LLVMDoubleTypeInContext(self.context);
                return c.LLVMConstReal(float_type, float_val);
            },
            .String => |string_val| {
                return try self.generateStringLiteral(string_val);
            },
            .Boolean => |bool_val| {
                const bool_type = c.LLVMInt1TypeInContext(self.context);
                return c.LLVMConstInt(bool_type, if (bool_val) 1 else 0, 0);
            },
            .Character => |char_val| {
                const char_type = c.LLVMInt8TypeInContext(self.context);
                return c.LLVMConstInt(char_type, char_val, 0);
            },
            .Null => {
                const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
                return c.LLVMConstNull(ptr_type);
            },
        }
    }

    /// Generate string literals with global string constants
    fn generateStringLiteral(self: *CodeGenerator, string_val: []const u8) !c.LLVMValueRef {
        const string_type = c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), @intCast(string_val.len + 1));
        
        // Create a global string constant
        const global_string = c.LLVMAddGlobal(self.module, string_type, "str_const");
        
        // Create string value with null terminator
        var string_with_null = try self.allocator.alloc(u8, string_val.len + 1);
        defer self.allocator.free(string_with_null);
        
        @memcpy(string_with_null[0..string_val.len], string_val);
        string_with_null[string_val.len] = 0;
        
        const string_constant = c.LLVMConstStringInContext(
            self.context,
            string_with_null.ptr,
            @intCast(string_with_null.len),
            0 // don't null terminate (we already did)
        );
        
        c.LLVMSetInitializer(global_string, string_constant);
        c.LLVMSetLinkage(global_string, c.LLVMPrivateLinkage);
        c.LLVMSetGlobalConstant(global_string, 1);
        
        // Return a pointer to the string
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        return c.LLVMConstGEP2(string_type, global_string, &[_]c.LLVMValueRef{ zero, zero }, 2);
    }

    /// Generate identifier access
    fn generateIdentifier(self: *CodeGenerator, identifier: ast.Identifier) !c.LLVMValueRef {
        if (self.variables.get(identifier.name)) |variable| {
            // Load the variable value
            const var_type = c.LLVMTypeOf(variable);
            const element_type = c.LLVMGetElementType(var_type);
            return c.LLVMBuildLoad2(self.builder, element_type, variable, identifier.name.ptr);
        } else {
            std.debug.print("Error: Undefined variable '{s}'\n", .{identifier.name});
            return error.UndefinedSymbol;
        }
    }

    /// Generate binary expressions
    fn generateBinaryExpression(self: *CodeGenerator, binary: ast.BinaryExpression) !c.LLVMValueRef {
        const left = try self.generateExpression(binary.left.*);
        const right = try self.generateExpression(binary.right.*);
        
        switch (binary.operator) {
            .Add => return c.LLVMBuildAdd(self.builder, left, right, "add_tmp"),
            .Subtract => return c.LLVMBuildSub(self.builder, left, right, "sub_tmp"),
            .Multiply => return c.LLVMBuildMul(self.builder, left, right, "mul_tmp"),
            .Divide => return c.LLVMBuildSDiv(self.builder, left, right, "div_tmp"),
            .Modulo => return c.LLVMBuildSRem(self.builder, left, right, "mod_tmp"),
            .Equal => return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq_tmp"),
            .NotEqual => return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "neq_tmp"),
            .LessThan => return c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt_tmp"),
            .LessThanOrEqual => return c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, left, right, "lte_tmp"),
            .GreaterThan => return c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt_tmp"),
            .GreaterThanOrEqual => return c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, left, right, "gte_tmp"),
            .LogicalAnd => {
                // Short-circuit AND
                const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
                const and_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "and.rhs");
                const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "and.end");
                
                _ = c.LLVMBuildCondBr(self.builder, left, and_block, merge_block);
                
                c.LLVMPositionBuilderAtEnd(self.builder, and_block);
                _ = c.LLVMBuildBr(self.builder, merge_block);
                
                c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
                const phi = c.LLVMBuildPhi(self.builder, c.LLVMInt1TypeInContext(self.context), "and_result");
                
                const false_val = c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 0, 0);
                const incoming_values = [_]c.LLVMValueRef{ false_val, right };
                const incoming_blocks = [_]c.LLVMBasicBlockRef{ c.LLVMGetPreviousBasicBlock(and_block), and_block };
                c.LLVMAddIncoming(phi, &incoming_values, &incoming_blocks, 2);
                
                return phi;
            },
            .LogicalOr => {
                // Short-circuit OR
                const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
                const or_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "or.rhs");
                const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "or.end");
                
                _ = c.LLVMBuildCondBr(self.builder, left, merge_block, or_block);
                
                c.LLVMPositionBuilderAtEnd(self.builder, or_block);
                _ = c.LLVMBuildBr(self.builder, merge_block);
                
                c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
                const phi = c.LLVMBuildPhi(self.builder, c.LLVMInt1TypeInContext(self.context), "or_result");
                
                const true_val = c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 1, 0);
                const incoming_values = [_]c.LLVMValueRef{ true_val, right };
                const incoming_blocks = [_]c.LLVMBasicBlockRef{ c.LLVMGetPreviousBasicBlock(or_block), or_block };
                c.LLVMAddIncoming(phi, &incoming_values, &incoming_blocks, 2);
                
                return phi;
            },
            else => {
                std.debug.print("Warning: Unimplemented binary operator\n", .{});
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            },
        }
    }

    /// Generate unary expressions
    fn generateUnaryExpression(self: *CodeGenerator, unary: ast.UnaryExpression) !c.LLVMValueRef {
        const operand = try self.generateExpression(unary.operand.*);
        
        switch (unary.operator) {
            .Negate => return c.LLVMBuildNeg(self.builder, operand, "neg_tmp"),
            .LogicalNot => return c.LLVMBuildNot(self.builder, operand, "not_tmp"),
            .BitwiseNot => return c.LLVMBuildNot(self.builder, operand, "bnot_tmp"),
            .AddressOf => {
                // Return the address of the operand (if it's an lvalue)
                std.debug.print("Warning: AddressOf operator not fully implemented\n", .{});
                return operand;
            },
            .Dereference => {
                // Dereference the pointer
                const element_type = c.LLVMGetElementType(c.LLVMTypeOf(operand));
                return c.LLVMBuildLoad2(self.builder, element_type, operand, "deref_tmp");
            },
            else => {
                std.debug.print("Warning: Unimplemented unary operator\n", .{});
                return operand;
            },
        }
    }

    /// Generate function call expressions
    fn generateCallExpression(self: *CodeGenerator, call: ast.CallExpression) !c.LLVMValueRef {
        // Special handling for built-in functions
        if (call.function.* == .Identifier) {
            const func_name = call.function.Identifier.name;
            
            if (std.mem.eql(u8, func_name, "printf") or std.mem.eql(u8, func_name, "vibez.spill")) {
                return try self.generatePrintfCall(call);
            }
        }
        
        // General function call handling
        const function = try self.generateExpression(call.function.*);
        
        // Generate arguments
        var args = try self.allocator.alloc(c.LLVMValueRef, call.arguments.len);
        defer self.allocator.free(args);
        
        for (call.arguments, 0..) |arg, i| {
            args[i] = try self.generateExpression(arg);
        }
        
        // Get function type for call
        const func_type = c.LLVMGetReturnType(c.LLVMGlobalGetValueType(function));
        
        return c.LLVMBuildCall2(
            self.builder,
            func_type,
            function,
            args.ptr,
            @intCast(args.len),
            "call_tmp"
        );
    }

    /// Generate printf/vibez.spill calls
    fn generatePrintfCall(self: *CodeGenerator, call: ast.CallExpression) !c.LLVMValueRef {
        if (call.arguments.len == 0) {
            return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        }
        
        const printf_func = c.LLVMGetNamedFunction(self.module, "printf");
        const puts_func = c.LLVMGetNamedFunction(self.module, "puts");
        
        // For simple string literals, use puts
        if (call.arguments.len == 1 and call.arguments[0] == .Literal and call.arguments[0].Literal == .String) {
            const string_arg = try self.generateExpression(call.arguments[0]);
            return c.LLVMBuildCall2(
                self.builder,
                c.LLVMGlobalGetValueType(puts_func),
                puts_func,
                &[_]c.LLVMValueRef{string_arg},
                1,
                "puts_call"
            );
        }
        
        // For format strings, use printf
        var args = try self.allocator.alloc(c.LLVMValueRef, call.arguments.len);
        defer self.allocator.free(args);
        
        for (call.arguments, 0..) |arg, i| {
            args[i] = try self.generateExpression(arg);
        }
        
        return c.LLVMBuildCall2(
            self.builder,
            c.LLVMGlobalGetValueType(printf_func),
            printf_func,
            args.ptr,
            @intCast(args.len),
            "printf_call"
        );
    }

    /// Generate member access expressions
    fn generateAccessExpression(self: *CodeGenerator, access: ast.AccessExpression) !c.LLVMValueRef {
        _ = access;
        std.debug.print("Warning: Member access not implemented\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    /// Generate array/slice index expressions
    fn generateIndexExpression(self: *CodeGenerator, index: ast.IndexExpression) !c.LLVMValueRef {
        _ = index;
        std.debug.print("Warning: Index expressions not implemented\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    /// Generate assignment expressions
    fn generateAssignmentExpression(self: *CodeGenerator, assignment: ast.AssignmentExpression) !c.LLVMValueRef {
        const value = try self.generateExpression(assignment.value.*);
        
        // Handle simple variable assignment
        if (assignment.target.* == .Identifier) {
            const var_name = assignment.target.Identifier.name;
            if (self.variables.get(var_name)) |variable| {
                _ = c.LLVMBuildStore(self.builder, value, variable);
                return value;
            } else {
                std.debug.print("Error: Undefined variable '{s}' in assignment\n", .{var_name});
                return error.UndefinedSymbol;
            }
        }
        
        std.debug.print("Warning: Complex assignment targets not implemented\n", .{});
        return value;
    }

    /// Generate array literal expressions
    fn generateArrayExpression(self: *CodeGenerator, array: ast.ArrayExpression) !c.LLVMValueRef {
        _ = array;
        std.debug.print("Warning: Array literals not implemented\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    /// Generate map literal expressions
    fn generateMapExpression(self: *CodeGenerator, map: ast.MapExpression) !c.LLVMValueRef {
        _ = map;
        std.debug.print("Warning: Map literals not implemented\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    /// Generate tuple expressions
    fn generateTupleExpression(self: *CodeGenerator, tuple: ast.TupleExpression) !c.LLVMValueRef {
        _ = tuple;
        std.debug.print("Warning: Tuple expressions not implemented\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    /// Generate type assertion expressions
    fn generateTypeAssertionExpression(self: *CodeGenerator, type_assertion: ast.TypeAssertionExpression) !c.LLVMValueRef {
        // For now, just return the expression being asserted
        return try self.generateExpression(type_assertion.expression.*);
    }

    /// Generate lambda expressions
    fn generateLambdaExpression(self: *CodeGenerator, lambda: ast.LambdaExpression) !c.LLVMValueRef {
        _ = lambda;
        std.debug.print("Warning: Lambda expressions not implemented\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }





    fn getLLVMType(self: *CodeGenerator, cursed_type: []const u8) c.LLVMTypeRef {
        if (std.mem.eql(u8, cursed_type, "normie")) {
            return c.LLVMInt32TypeInContext(self.context);
        } else if (std.mem.eql(u8, cursed_type, "meal")) {
            return c.LLVMDoubleTypeInContext(self.context);
        } else if (std.mem.eql(u8, cursed_type, "tea")) {
            return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        } else if (std.mem.eql(u8, cursed_type, "lit")) {
            return c.LLVMInt1TypeInContext(self.context);
        }
        return c.LLVMVoidTypeInContext(self.context);
    }

    /// Generate native executable from LLVM module
    pub fn writeExecutable(self: *CodeGenerator, output_path: []const u8) CodeGenError!void {
        // Initialize LLVM targets and execution engine
        c.LLVMInitializeAllTargetInfos();
        c.LLVMInitializeAllTargets();
        c.LLVMInitializeAllTargetMCs();
        c.LLVMInitializeAllAsmParsers();
        c.LLVMInitializeAllAsmPrinters();

        // Write LLVM IR to file for debugging
        var ir_filename = std.ArrayList(u8).init(self.allocator);
        defer ir_filename.deinit();
        
        try ir_filename.appendSlice(output_path);
        try ir_filename.appendSlice(".ll");
        
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMPrintModuleToFile(self.module, ir_filename.items.ptr, &error_msg) != 0) {
            std.debug.print("Failed to write LLVM IR: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }

        // Get native target triple for current platform
        const target_triple = c.LLVMGetDefaultTargetTriple();
        defer c.LLVMDisposeMessage(target_triple);

        // Get target from triple
        var llvm_target: c.LLVMTargetRef = undefined;
        if (c.LLVMGetTargetFromTriple(target_triple, &llvm_target, &error_msg) != 0) {
            std.debug.print("Failed to get target: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }

        // Create target machine
        const target_machine = c.LLVMCreateTargetMachine(
            llvm_target,
            target_triple,
            "generic", // CPU
            "", // Features
            c.LLVMCodeGenLevelDefault,
            c.LLVMRelocDefault,
            c.LLVMCodeModelDefault
        );
        defer c.LLVMDisposeTargetMachine(target_machine);

        if (target_machine == null) {
            std.debug.print("Failed to create target machine\n", .{});
            return CodeGenError.LLVMError;
        }

        // Generate object file
        var obj_filename = std.ArrayList(u8).init(self.allocator);
        defer obj_filename.deinit();
        try obj_filename.appendSlice(output_path);
        try obj_filename.appendSlice(".o");

        if (c.LLVMTargetMachineEmitToFile(target_machine, self.module, obj_filename.items.ptr, c.LLVMObjectFile, &error_msg) != 0) {
            std.debug.print("Failed to emit object file: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }

        // Link object file to executable using system linker
        try self.linkToExecutable(obj_filename.items, output_path);

        std.debug.print("✅ Generated executable: {s}\n", .{output_path});
        std.debug.print("🔧 LLVM IR written to: {s}\n", .{ir_filename.items});
        std.debug.print("🔗 Object file: {s}\n", .{obj_filename.items});
    }

    /// Link object file to executable using system linker
    fn linkToExecutable(self: *CodeGenerator, obj_path: []const u8, output_path: []const u8) CodeGenError!void {
        const allocator = self.allocator;
        
        // Detect platform and use appropriate linker
        const is_macos = std.builtin.os.tag == .macos;
        const is_windows = std.builtin.os.tag == .windows;
        
        var link_args = std.ArrayList([]const u8).init(allocator);
        defer link_args.deinit();
        
        if (is_windows) {
            // Windows: use link.exe or ld
            try link_args.append("ld");
            try link_args.append("-o");
            try link_args.append(output_path);
            try link_args.append(obj_path);
            try link_args.append("-lc");
        } else if (is_macos) {
            // macOS: use ld
            try link_args.append("ld");
            try link_args.append("-o");
            try link_args.append(output_path);
            try link_args.append(obj_path);
            try link_args.append("-lSystem");
            try link_args.append("-arch");
            try link_args.append("x86_64");
        } else {
            // Linux: use ld or gcc
            try link_args.append("gcc");
            try link_args.append("-o");
            try link_args.append(output_path);
            try link_args.append(obj_path);
            try link_args.append("-no-pie"); // Disable PIE for compatibility
        }

        // Execute linker
        var child = std.ChildProcess.init(link_args.items, allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;
        
        const result = child.spawnAndWait() catch |err| {
            std.debug.print("Failed to spawn linker: {}\n", .{err});
            return CodeGenError.LinkerError;
        };
        
        switch (result) {
            .Exited => |code| {
                if (code != 0) {
                    std.debug.print("Linker failed with exit code: {}\n", .{code});
                    return CodeGenError.LinkerError;
                }
            },
            else => {
                std.debug.print("Linker process terminated abnormally\n", .{});
                return CodeGenError.LinkerError;
            }
        }
    }

    pub fn generateBitcode(self: *CodeGenerator, output_path: []const u8) !void {
        if (c.LLVMWriteBitcodeToFile(self.module, output_path.ptr) != 0) {
            return error.BitcodeWriteFailed;
        }
    }

    /// Generate struct definitions
    fn generateStructStatement(self: *CodeGenerator, struct_stmt: ast.StructStatement) !void {
        _ = struct_stmt;
        std.debug.print("Warning: Struct statements not implemented\n", .{});
    }

    /// Generate interface definitions
    fn generateInterfaceStatement(self: *CodeGenerator, interface_stmt: ast.InterfaceStatement) !void {
        _ = interface_stmt;
        std.debug.print("Warning: Interface statements not implemented\n", .{});
    }

    /// Generate implementation blocks
    fn generateImplementationStatement(self: *CodeGenerator, impl_stmt: ast.ImplementationStatement) !void {
        _ = impl_stmt;
        std.debug.print("Warning: Implementation statements not implemented\n", .{});
    }

    /// Generate match/pattern matching statements
    fn generateMatchStatement(self: *CodeGenerator, match_stmt: ast.MatchStatement) !void {
        _ = match_stmt;
        std.debug.print("Warning: Match statements not implemented\n", .{});
    }

    /// Generate defer statements
    fn generateDeferStatement(self: *CodeGenerator, defer_stmt: ast.DeferStatement) !void {
        _ = defer_stmt;
        std.debug.print("Warning: Defer statements not implemented\n", .{});
    }

    /// Generate try statements
    fn generateTryStatement(self: *CodeGenerator, try_stmt: ast.TryStatement) !void {
        _ = try_stmt;
        std.debug.print("Warning: Try statements not implemented\n", .{});
    }

    /// Generate goroutine/async statements
    fn generateGoroutineStatement(self: *CodeGenerator, goroutine_stmt: ast.GoroutineStatement) !void {
        _ = goroutine_stmt;
        std.debug.print("Warning: Goroutine statements not implemented\n", .{});
    }

    /// Generate select statements for channel operations
    fn generateSelectStatement(self: *CodeGenerator, select_stmt: ast.SelectStatement) !void {
        _ = select_stmt;
        std.debug.print("Warning: Select statements not implemented\n", .{});
    }

    /// Generate break statements
    fn generateBreakStatement(self: *CodeGenerator, break_stmt: ast.BreakStatement) !void {
        _ = break_stmt;
        std.debug.print("Warning: Break statements not implemented\n", .{});
    }

    /// Generate continue statements
    fn generateContinueStatement(self: *CodeGenerator, continue_stmt: ast.ContinueStatement) !void {
        _ = continue_stmt;
        std.debug.print("Warning: Continue statements not implemented\n", .{});
    }

    /// Generate constant definitions
    fn generateConstantStatement(self: *CodeGenerator, const_stmt: ast.ConstantStatement) !void {
        _ = const_stmt;
        std.debug.print("Warning: Constant statements not implemented\n", .{});
    }

    /// Generate type alias statements
    fn generateTypeStatement(self: *CodeGenerator, type_stmt: ast.TypeStatement) !void {
        _ = type_stmt;
        std.debug.print("Warning: Type statements not implemented\n", .{});
    }
};
