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
    struct_types: std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    interface_types: std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    struct_fields: std.HashMap([]const u8, std.ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn init(allocator: std.mem.Allocator) !*CodeGenerator {
        const self = try allocator.create(CodeGenerator);
        
        self.allocator = allocator;
        self.context = c.LLVMContextCreate();
        self.module = c.LLVMModuleCreateWithNameInContext("cursed_module", self.context);
        self.builder = c.LLVMCreateBuilderInContext(self.context);
        self.variables = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
        self.struct_types = std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
        self.interface_types = std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
        self.struct_fields = std.HashMap([]const u8, std.ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
        
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
        self.struct_types.deinit();
        self.interface_types.deinit();
        
        // Clean up struct fields
        var field_iter = self.struct_fields.iterator();
        while (field_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.struct_fields.deinit();
        
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
            .Let => |let_stmt| {
                try self.generateLetStatement(let_stmt);
            },
            .Assignment => |assign_stmt| {
                try self.generateAssignmentStatement(assign_stmt);
            },
            .Expression => |expr| {
                const expr_ptr: *ast.Expression = @ptrCast(@alignCast(expr));
                _ = try self.generateExpression(expr_ptr.*);
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
            .ForIn => |for_in_stmt| {
                try self.generateForInStatement(for_in_stmt);
            },
            .Return => |return_stmt| {
                try self.generateReturnStatement(return_stmt);
            },
            .Switch => |switch_stmt| {
                try self.generateSwitchStatement(switch_stmt);
            },
            .PatternSwitch => |pattern_stmt| {
                try self.generatePatternSwitchStatement(pattern_stmt);
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
            .TypeAlias => |type_alias| {
                try self.generateTypeAliasStatement(type_alias);
            },
            .Defer => |defer_stmt| {
                try self.generateDeferStatement(defer_stmt);
            },
            .Goroutine => |goroutine_stmt| {
                try self.generateGoroutineStatement(goroutine_stmt);
            },
            .Channel => |channel_stmt| {
                try self.generateChannelStatement(channel_stmt);
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
            .Increment => |inc_stmt| {
                try self.generateIncrementStatement(inc_stmt);
            },
            .Decrement => |dec_stmt| {
                try self.generateDecrementStatement(dec_stmt);
            },
            .ShortDeclaration => |short_decl| {
                try self.generateShortDeclarationStatement(short_decl);
            },
            .Panic => |panic_stmt| {
                try self.generatePanicStatement(panic_stmt);
            },
            .Catch => |catch_stmt| {
                try self.generateCatchStatement(catch_stmt);
            },
            .Yikes => |yikes_stmt| {
                try self.generateYikesStatement(yikes_stmt);
            },
            .Fam => |fam_stmt| {
                try self.generateFamStatement(fam_stmt);
            },
            .Const => |const_decl| {
                try self.generateConstStatement(const_decl);
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
                return try self.generateIdentifierExpression(identifier);
            },
            .Variable => |variable| {
                return try self.generateVariableExpression(variable);
            },
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
            .Binary => |binary| {
                return try self.generateBinaryExpression(binary);
            },
            .Unary => |unary| {
                return try self.generateUnaryExpression(unary.*);
            },
            .Call => |call| {
                return try self.generateCallExpression(call);
            },
            .MemberAccess => |access| {
                return try self.generateMemberAccessExpression(access.*);
            },
            .ArrayAccess => |index| {
                return try self.generateArrayAccessExpression(index);
            },
            .SliceAccess => |slice| {
                return try self.generateSliceAccessExpression(slice);
            },
            .Array => |array| {
                return try self.generateArrayExpression(array.*);
            },
            .Map => |map| {
                return try self.generateMapExpression(map.*);
            },
            .Tuple => |tuple| {
                return try self.generateTupleExpression(tuple);
            },
            .TupleAccess => |tuple_access| {
                return try self.generateTupleAccessExpression(tuple_access);
            },
            .TypeAssertion => |type_assertion| {
                return try self.generateTypeAssertionExpression(type_assertion);
            },
            .Lambda => |lambda| {
                return try self.generateLambdaExpression(lambda);
            },
            .CompositeLiteral => |composite| {
                return try self.generateCompositeLiteralExpression(composite);
            },
            .ChannelSend => |channel_send| {
                return try self.generateChannelSendExpression(channel_send);
            },
            .ChannelReceive => |channel_recv| {
                return try self.generateChannelReceiveExpression(channel_recv);
            },
            .ChannelCreation => |channel_create| {
                return try self.generateChannelCreationExpression(channel_create);
            },
            .StructLiteral => |struct_lit| {
                return try self.generateStructLiteralExpression(struct_lit);
            },
            .Increment => |increment| {
                return try self.generateIncrementExpression(increment);
            },
            .Decrement => |decrement| {
                return try self.generateDecrementExpression(decrement);
            },
            .Yikes => |yikes| {
                return try self.generateYikesExpression(yikes);
            },
            .Shook => |shook| {
                return try self.generateShookExpression(shook);
            },
            .Fam => |fam| {
                return try self.generateFamExpression(fam);
            },
            .ErrorValue => |error_val| {
                return try self.generateErrorValueExpression(error_val);
            },
            .StructuredError => |struct_err| {
                return try self.generateStructuredErrorExpression(struct_err);
            },
            .Panic => |panic| {
                return try self.generatePanicExpression(panic);
            },
            .Recover => |recover| {
                return try self.generateRecoverExpression(recover);
            },
            .TestResult => |test_result| {
                return try self.generateTestResultExpression(test_result);
            },
            .TestResultCheck => |test_check| {
                return try self.generateTestResultCheckExpression(test_check);
            },
            .RangeFor => |range_for| {
                return try self.generateRangeForExpression(range_for);
            },
            .Match => |match| {
                return try self.generateMatchExpression(match);
            },
            .TypeSwitch => |type_switch| {
                return try self.generateTypeSwitchExpression(type_switch);
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
    fn generateIdentifierExpression(self: *CodeGenerator, identifier: []const u8) !c.LLVMValueRef {
        if (self.variables.get(identifier)) |variable| {
            // Load the variable value
            const var_type = c.LLVMTypeOf(variable);
            const element_type = c.LLVMGetElementType(var_type);
            return c.LLVMBuildLoad2(self.builder, element_type, variable, identifier.ptr);
        } else {
            std.debug.print("Error: Undefined variable '{s}'\n", .{identifier});
            return error.UndefinedSymbol;
        }
    }
    
    /// Generate variable access
    fn generateVariableExpression(self: *CodeGenerator, variable: []const u8) !c.LLVMValueRef {
        return try self.generateIdentifierExpression(variable);
    }

    /// Generate binary expressions
    fn generateBinaryExpression(self: *CodeGenerator, binary: ast.BinaryExpression) !c.LLVMValueRef {
        const left = try self.generateExpression(binary.left.*);
        const right = try self.generateExpression(binary.right.*);
        
        // Handle string-based operators
        if (std.mem.eql(u8, binary.operator, "+")) {
            return c.LLVMBuildAdd(self.builder, left, right, "add_tmp");
        } else if (std.mem.eql(u8, binary.operator, "-")) {
            return c.LLVMBuildSub(self.builder, left, right, "sub_tmp");
        } else if (std.mem.eql(u8, binary.operator, "*")) {
            return c.LLVMBuildMul(self.builder, left, right, "mul_tmp");
        } else if (std.mem.eql(u8, binary.operator, "/")) {
            return c.LLVMBuildSDiv(self.builder, left, right, "div_tmp");
        } else if (std.mem.eql(u8, binary.operator, "%")) {
            return c.LLVMBuildSRem(self.builder, left, right, "mod_tmp");
        } else if (std.mem.eql(u8, binary.operator, "==")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq_tmp");
        } else if (std.mem.eql(u8, binary.operator, "!=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "neq_tmp");
        } else if (std.mem.eql(u8, binary.operator, "<")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt_tmp");
        } else if (std.mem.eql(u8, binary.operator, "<=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, left, right, "lte_tmp");
        } else if (std.mem.eql(u8, binary.operator, ">")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt_tmp");
        } else if (std.mem.eql(u8, binary.operator, ">=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, left, right, "gte_tmp");
        } else if (std.mem.eql(u8, binary.operator, "&")) {
            return c.LLVMBuildAnd(self.builder, left, right, "and_tmp");
        } else if (std.mem.eql(u8, binary.operator, "|")) {
            return c.LLVMBuildOr(self.builder, left, right, "or_tmp");
        } else if (std.mem.eql(u8, binary.operator, "^")) {
            return c.LLVMBuildXor(self.builder, left, right, "xor_tmp");
        } else if (std.mem.eql(u8, binary.operator, "<<")) {
            return c.LLVMBuildShl(self.builder, left, right, "shl_tmp");
        } else if (std.mem.eql(u8, binary.operator, ">>")) {
            return c.LLVMBuildAShr(self.builder, left, right, "shr_tmp");
        } else if (std.mem.eql(u8, binary.operator, "&&")) {
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
        } else if (std.mem.eql(u8, binary.operator, "||")) {
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
        } else {
            std.debug.print("Warning: Unimplemented binary operator: {s}\n", .{binary.operator});
            return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        }
    }

    /// Generate unary expressions
    fn generateUnaryExpression(self: *CodeGenerator, unary: ast.UnaryExpression) !c.LLVMValueRef {
        const operand = try self.generateExpression(unary.operand.*);
        
        // Handle string-based operators
        if (std.mem.eql(u8, unary.operator, "-")) {
            return c.LLVMBuildNeg(self.builder, operand, "neg_tmp");
        } else if (std.mem.eql(u8, unary.operator, "!")) {
            return c.LLVMBuildNot(self.builder, operand, "not_tmp");
        } else if (std.mem.eql(u8, unary.operator, "~")) {
            return c.LLVMBuildNot(self.builder, operand, "bnot_tmp");
        } else if (std.mem.eql(u8, unary.operator, "&")) {
            // Return the address of the operand (if it's an lvalue)
            // For now, we'll assume operand is already an address
            std.debug.print("Warning: AddressOf operator simplified implementation\n", .{});
            return operand;
        } else if (std.mem.eql(u8, unary.operator, "*")) {
            // Dereference the pointer
            const element_type = c.LLVMGetElementType(c.LLVMTypeOf(operand));
            return c.LLVMBuildLoad2(self.builder, element_type, operand, "deref_tmp");
        } else if (std.mem.eql(u8, unary.operator, "++")) {
            // Pre-increment
            const one = c.LLVMConstInt(c.LLVMTypeOf(operand), 1, 0);
            return c.LLVMBuildAdd(self.builder, operand, one, "preinc_tmp");
        } else if (std.mem.eql(u8, unary.operator, "--")) {
            // Pre-decrement
            const one = c.LLVMConstInt(c.LLVMTypeOf(operand), 1, 0);
            return c.LLVMBuildSub(self.builder, operand, one, "predec_tmp");
        } else {
            std.debug.print("Warning: Unimplemented unary operator: {s}\n", .{unary.operator});
            return operand;
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
    fn generateMemberAccessExpression(self: *CodeGenerator, access: *ast.MemberAccessExpression) !c.LLVMValueRef {
        const object_value = try self.generateExpression(access.object.*);
        
        // For struct member access, we need to generate GEP instructions
        const object_type = c.LLVMTypeOf(object_value);
        
        // Check if this is a pointer to struct
        if (c.LLVMGetTypeKind(object_type) == c.LLVMPointerTypeKind) {
            const element_type = c.LLVMGetElementType(object_type);
            
            if (c.LLVMGetTypeKind(element_type) == c.LLVMStructTypeKind) {
                // Get field index for the member name
                const field_index = self.getStructFieldIndex(element_type, access.property) orelse {
                    std.debug.print("Error: Unknown field '{s}'\n", .{access.property});
                    return error.UndefinedSymbol;
                };
                
                // Generate GEP for struct member access
                const indices = [_]c.LLVMValueRef{
                    c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                    c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), field_index, 0),
                };
                
                const gep = c.LLVMBuildGEP2(
                    self.builder,
                    element_type,
                    object_value,
                    &indices,
                    2,
                    "member_ptr"
                );
                
                // Load the field value
                const field_type = c.LLVMStructGetTypeAtIndex(element_type, @intCast(field_index));
                return c.LLVMBuildLoad2(self.builder, field_type, gep, "member_val");
            }
        }
        
        std.debug.print("Warning: Non-struct member access not fully implemented\n", .{});
        return object_value;
    }

    /// Generate array/slice index expressions
    fn generateArrayAccessExpression(self: *CodeGenerator, index: ast.ArrayAccessExpression) !c.LLVMValueRef {
        const array_value = try self.generateExpression(index.array.*);
        const index_value = try self.generateExpression(index.index.*);
        
        const array_type = c.LLVMTypeOf(array_value);
        
        // Handle array indexing
        if (c.LLVMGetTypeKind(array_type) == c.LLVMPointerTypeKind) {
            const element_type = c.LLVMGetElementType(array_type);
            
            if (c.LLVMGetTypeKind(element_type) == c.LLVMArrayTypeKind) {
                // Array access using GEP
                const indices = [_]c.LLVMValueRef{
                    c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                    index_value,
                };
                
                const gep = c.LLVMBuildGEP2(
                    self.builder,
                    element_type,
                    array_value,
                    &indices,
                    2,
                    "array_ptr"
                );
                
                const element_element_type = c.LLVMGetElementType(element_type);
                return c.LLVMBuildLoad2(self.builder, element_element_type, gep, "array_elem");
            }
        }
        
        // Handle pointer arithmetic for slice-like access
        if (c.LLVMGetTypeKind(array_type) == c.LLVMPointerTypeKind) {
            const ptr_gep = c.LLVMBuildGEP2(
                self.builder,
                c.LLVMGetElementType(array_type),
                array_value,
                &[_]c.LLVMValueRef{index_value},
                1,
                "ptr_offset"
            );
            
            const element_type = c.LLVMGetElementType(array_type);
            return c.LLVMBuildLoad2(self.builder, element_type, ptr_gep, "elem_val");
        }
        
        std.debug.print("Warning: Unsupported array indexing type\n", .{});
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
        
        // Handle member access assignment (e.g., obj.field = value)
        if (assignment.target.* == .MemberAccess) {
            const access = assignment.target.MemberAccess;
            const object_value = try self.generateExpression(access.object.*);
            
            const object_type = c.LLVMTypeOf(object_value);
            if (c.LLVMGetTypeKind(object_type) == c.LLVMPointerTypeKind) {
                const element_type = c.LLVMGetElementType(object_type);
                
                if (c.LLVMGetTypeKind(element_type) == c.LLVMStructTypeKind) {
                    const field_index = self.getStructFieldIndex(element_type, access.property) orelse {
                        std.debug.print("Error: Unknown field '{s}'\n", .{access.property});
                        return error.UndefinedSymbol;
                    };
                    
                    const indices = [_]c.LLVMValueRef{
                        c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                        c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), field_index, 0),
                    };
                    
                    const gep = c.LLVMBuildGEP2(
                        self.builder,
                        element_type,
                        object_value,
                        &indices,
                        2,
                        "field_ptr"
                    );
                    
                    _ = c.LLVMBuildStore(self.builder, value, gep);
                    return value;
                }
            }
        }
        
        // Handle array index assignment (e.g., arr[i] = value)
        if (assignment.target.* == .ArrayAccess) {
            const index_expr = assignment.target.ArrayAccess;
            const array_value = try self.generateExpression(index_expr.array.*);
            const index_value = try self.generateExpression(index_expr.index.*);
            
            const array_type = c.LLVMTypeOf(array_value);
            if (c.LLVMGetTypeKind(array_type) == c.LLVMPointerTypeKind) {
                const element_type = c.LLVMGetElementType(array_type);
                
                if (c.LLVMGetTypeKind(element_type) == c.LLVMArrayTypeKind) {
                    const indices = [_]c.LLVMValueRef{
                        c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                        index_value,
                    };
                    
                    const gep = c.LLVMBuildGEP2(
                        self.builder,
                        element_type,
                        array_value,
                        &indices,
                        2,
                        "array_elem_ptr"
                    );
                    
                    _ = c.LLVMBuildStore(self.builder, value, gep);
                    return value;
                }
            }
        }
        
        std.debug.print("Warning: Complex assignment targets not fully implemented\n", .{});
        return value;
    }

    /// Generate array literal expressions
    fn generateArrayExpression(self: *CodeGenerator, array: ast.ArrayExpression) !c.LLVMValueRef {
        if (array.elements.items.len == 0) {
            // Empty array
            const i32_type = c.LLVMInt32TypeInContext(self.context);
            return c.LLVMConstArray(i32_type, null, 0);
        }
        
        // Generate all element values
        var element_values = try self.allocator.alloc(c.LLVMValueRef, array.elements.items.len);
        defer self.allocator.free(element_values);
        
        var element_type: c.LLVMTypeRef = undefined;
        
        for (array.elements.items, 0..) |elem, i| {
            const elem_expr: *ast.Expression = @ptrCast(@alignCast(elem));
            element_values[i] = try self.generateExpression(elem_expr.*);
            
            if (i == 0) {
                element_type = c.LLVMTypeOf(element_values[i]);
            }
        }
        
        // Create array constant
        const array_const = c.LLVMConstArray(
            element_type,
            element_values.ptr,
            @intCast(element_values.len)
        );
        
        // Allocate space for the array and store it
        const array_type = c.LLVMArrayType(element_type, @intCast(element_values.len));
        const array_alloca = c.LLVMBuildAlloca(self.builder, array_type, "array_alloca");
        _ = c.LLVMBuildStore(self.builder, array_const, array_alloca);
        
        return array_alloca;
    }

    /// Generate map literal expressions
    fn generateMapExpression(self: *CodeGenerator, map: ast.MapExpression) !c.LLVMValueRef {
        // For now, create a simple struct-based map representation
        // In a full implementation, this would use hash table runtime functions
        
        if (map.entries.items.len == 0) {
            // Empty map - return null pointer for now
            const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            return c.LLVMConstNull(ptr_type);
        }
        
        // Generate a simple array of key-value pairs
        var key_values = try self.allocator.alloc(c.LLVMValueRef, map.entries.items.len * 2);
        defer self.allocator.free(key_values);
        
        var key_type: c.LLVMTypeRef = undefined;
        var value_type: c.LLVMTypeRef = undefined;
        
        for (map.entries.items, 0..) |entry, i| {
            const key_expr: *ast.Expression = @ptrCast(@alignCast(entry.key));
            const value_expr: *ast.Expression = @ptrCast(@alignCast(entry.value));
            
            const key_val = try self.generateExpression(key_expr.*);
            const value_val = try self.generateExpression(value_expr.*);
            
            key_values[i * 2] = key_val;
            key_values[i * 2 + 1] = value_val;
            
            if (i == 0) {
                key_type = c.LLVMTypeOf(key_val);
                value_type = c.LLVMTypeOf(value_val);
            }
        }
        
        // Create a struct type for key-value pairs
        const kv_pair_types = [_]c.LLVMTypeRef{ key_type, value_type };
        const kv_pair_type = c.LLVMStructTypeInContext(self.context, &kv_pair_types, 2, 0);
        
        // Create array of key-value pairs
        const map_array_type = c.LLVMArrayType(kv_pair_type, @intCast(map.entries.items.len));
        const map_alloca = c.LLVMBuildAlloca(self.builder, map_array_type, "map_alloca");
        
        // Store each key-value pair
        for (0..map.entries.items.len) |i| {
            const indices = [_]c.LLVMValueRef{
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(i), 0),
            };
            
            const pair_ptr = c.LLVMBuildGEP2(
                self.builder,
                map_array_type,
                map_alloca,
                &indices,
                2,
                "pair_ptr"
            );
            
            // Store key
            const key_indices = [_]c.LLVMValueRef{
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
            };
            const key_ptr = c.LLVMBuildGEP2(
                self.builder,
                kv_pair_type,
                pair_ptr,
                &key_indices,
                1,
                "key_ptr"
            );
            _ = c.LLVMBuildStore(self.builder, key_values[i * 2], key_ptr);
            
            // Store value
            const value_indices = [_]c.LLVMValueRef{
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 1, 0),
            };
            const value_ptr = c.LLVMBuildGEP2(
                self.builder,
                kv_pair_type,
                pair_ptr,
                &value_indices,
                1,
                "value_ptr"
            );
            _ = c.LLVMBuildStore(self.builder, key_values[i * 2 + 1], value_ptr);
        }
        
        return map_alloca;
    }

    /// Generate tuple expressions
    fn generateTupleExpression(self: *CodeGenerator, tuple: ast.TupleExpression) !c.LLVMValueRef {
        if (tuple.elements.items.len == 0) {
            // Empty tuple - unit type
            const void_type = c.LLVMVoidTypeInContext(self.context);
            return c.LLVMGetUndef(void_type);
        }
        
        // Generate all tuple element values
        var element_values = try self.allocator.alloc(c.LLVMValueRef, tuple.elements.items.len);
        defer self.allocator.free(element_values);
        
        var element_types = try self.allocator.alloc(c.LLVMTypeRef, tuple.elements.items.len);
        defer self.allocator.free(element_types);
        
        for (tuple.elements.items, 0..) |elem, i| {
            const elem_expr: *ast.Expression = @ptrCast(@alignCast(elem));
            element_values[i] = try self.generateExpression(elem_expr.*);
            element_types[i] = c.LLVMTypeOf(element_values[i]);
        }
        
        // Create tuple struct type
        const tuple_type = c.LLVMStructTypeInContext(
            self.context,
            element_types.ptr,
            @intCast(element_types.len),
            0
        );
        
        // Allocate tuple and store elements
        const tuple_alloca = c.LLVMBuildAlloca(self.builder, tuple_type, "tuple_alloca");
        
        for (element_values, 0..) |elem_val, i| {
            const indices = [_]c.LLVMValueRef{
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(i), 0),
            };
            
            const elem_ptr = c.LLVMBuildGEP2(
                self.builder,
                tuple_type,
                tuple_alloca,
                &indices,
                2,
                "tuple_elem_ptr"
            );
            
            _ = c.LLVMBuildStore(self.builder, elem_val, elem_ptr);
        }
        
        return tuple_alloca;
    }

    /// Generate type assertion expressions
    fn generateTypeAssertionExpression(self: *CodeGenerator, type_assertion: ast.TypeAssertionExpression) !c.LLVMValueRef {
        // For now, just return the expression being asserted
        return try self.generateExpression(type_assertion.expression.*);
    }

    /// Generate lambda expressions
    fn generateLambdaExpression(self: *CodeGenerator, lambda: ast.LambdaExpression) !c.LLVMValueRef {
        // Create function type for lambda
        const return_type = c.LLVMInt32TypeInContext(self.context); // Default to int for now
        
        var param_types = try self.allocator.alloc(c.LLVMTypeRef, lambda.parameters.items.len);
        defer self.allocator.free(param_types);
        
        for (lambda.parameters.items, 0..) |_, i| {
            param_types[i] = c.LLVMInt32TypeInContext(self.context); // Default to int
        }
        
        const func_type = c.LLVMFunctionType(
            return_type,
            param_types.ptr,
            @intCast(param_types.len),
            0
        );
        
        // Create lambda function
        var lambda_name_buf: [64]u8 = undefined;
        const lambda_name = try std.fmt.bufPrint(&lambda_name_buf, "lambda_{}", .{self.variables.count()});
        
        const lambda_func = c.LLVMAddFunction(self.module, lambda_name.ptr, func_type);
        
        // Save current builder state
        const saved_block = c.LLVMGetInsertBlock(self.builder);
        
        // Create lambda body
        const lambda_entry = c.LLVMAppendBasicBlockInContext(self.context, lambda_func, "lambda_entry");
        c.LLVMPositionBuilderAtEnd(self.builder, lambda_entry);
        
        // Generate lambda body
        const body_value = try self.generateExpression(lambda.body.*);
        _ = c.LLVMBuildRet(self.builder, body_value);
        
        // Restore builder state
        c.LLVMPositionBuilderAtEnd(self.builder, saved_block);
        
        // Return function pointer
        return lambda_func;
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

    fn getLLVMTypeFromCursedType(self: *CodeGenerator, cursed_type: ast.Type) c.LLVMTypeRef {
        switch (cursed_type) {
            .Basic => |basic| {
                switch (basic) {
                    .Normie => return c.LLVMInt32TypeInContext(self.context),
                    .Tea, .Txt => return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
                    .Sip, .Smol, .Byte => return c.LLVMInt8TypeInContext(self.context),
                    .Mid => return c.LLVMInt16TypeInContext(self.context),
                    .Thicc => return c.LLVMInt64TypeInContext(self.context),
                    .Snack => return c.LLVMFloatTypeInContext(self.context),
                    .Meal => return c.LLVMDoubleTypeInContext(self.context),
                    .Lit => return c.LLVMInt1TypeInContext(self.context),
                    .Cap => return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
                    else => return c.LLVMVoidTypeInContext(self.context),
                }
            },
            .Array => |array| {
                const element_type = self.getLLVMTypeFromCursedType(array.element_type.*);
                const size = array.size orelse 0;
                return c.LLVMArrayType(element_type, @intCast(size));
            },
            .Pointer => |pointer| {
                const target_type = self.getLLVMTypeFromCursedType(pointer.target_type.*);
                return c.LLVMPointerType(target_type, 0);
            },
            .Struct => |struct_type| {
                if (self.struct_types.get(struct_type.name)) |llvm_type| {
                    return llvm_type;
                }
                return c.LLVMVoidTypeInContext(self.context);
            },
            else => return c.LLVMVoidTypeInContext(self.context),
        }
    }

    fn getStructFieldIndex(self: *CodeGenerator, struct_type: c.LLVMTypeRef, field_name: []const u8) ?u32 {
        _ = self;
        _ = struct_type;
        // This is a simplified implementation
        // In a full compiler, we'd maintain a mapping of struct names to field layouts
        const field_names = [_][]const u8{ "x", "y", "z", "name", "age", "value" };
        for (field_names, 0..) |name, i| {
            if (std.mem.eql(u8, name, field_name)) {
                return @intCast(i);
            }
        }
        return null;
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
        // Create struct type
        var field_types = try self.allocator.alloc(c.LLVMTypeRef, struct_stmt.fields.items.len);
        defer self.allocator.free(field_types);
        
        for (struct_stmt.fields.items, 0..) |field, i| {
            field_types[i] = self.getLLVMTypeFromCursedType(field.field_type);
        }
        
        const struct_type = c.LLVMStructTypeInContext(
            self.context,
            field_types.ptr,
            @intCast(field_types.len),
            0
        );
        
        // Store struct type for later reference
        try self.struct_types.put(struct_stmt.name, struct_type);
        
        std.debug.print("✅ Struct '{s}' defined with {} fields\n", .{ struct_stmt.name, struct_stmt.fields.items.len });
    }

    /// Generate interface definitions
    fn generateInterfaceStatement(self: *CodeGenerator, interface_stmt: ast.InterfaceStatement) !void {
        // Interfaces in LLVM are typically implemented as vtables
        // For now, we'll create a struct containing function pointers
        
        var method_types = try self.allocator.alloc(c.LLVMTypeRef, interface_stmt.methods.items.len);
        defer self.allocator.free(method_types);
        
        for (interface_stmt.methods.items, 0..) |method, i| {
            // Create function pointer type for each method
            var param_types = try self.allocator.alloc(c.LLVMTypeRef, method.parameters.items.len + 1); // +1 for self parameter
            defer self.allocator.free(param_types);
            
            param_types[0] = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0); // self pointer
            
            for (method.parameters.items, 0..) |param, j| {
                param_types[j + 1] = self.getLLVMTypeFromCursedType(param.param_type);
            }
            
            const return_type = if (method.return_type) |ret_type|
                self.getLLVMTypeFromCursedType(ret_type)
            else
                c.LLVMVoidTypeInContext(self.context);
            
            const method_func_type = c.LLVMFunctionType(
                return_type,
                param_types.ptr,
                @intCast(param_types.len),
                0
            );
            
            method_types[i] = c.LLVMPointerType(method_func_type, 0);
        }
        
        // Create vtable struct type
        const vtable_type = c.LLVMStructTypeInContext(
            self.context,
            method_types.ptr,
            @intCast(method_types.len),
            0
        );
        
        // Store interface type for later reference
        try self.interface_types.put(interface_stmt.name, vtable_type);
        
        std.debug.print("✅ Interface '{s}' defined with {} methods\n", .{ interface_stmt.name, interface_stmt.methods.items.len });
    }

    /// Generate implementation blocks
    fn generateImplementationStatement(self: *CodeGenerator, impl_stmt: ast.ImplementationStatement) !void {
        // Implementation blocks generate the actual methods for interfaces
        // Each method becomes a regular function with name mangling
        
        for (impl_stmt.methods.items) |*method| {
            // Generate mangled name: Type_Interface_MethodName
            var mangled_name_buf: [256]u8 = undefined;
            const mangled_name = try std.fmt.bufPrint(
                &mangled_name_buf,
                "{s}_{s}_{s}",
                .{ impl_stmt.implementing_type, impl_stmt.interface_name, method.name }
            );
            
            // Create function with mangled name
            var method_copy = method.*;
            method_copy.name = mangled_name;
            
            try self.generateFunction(method_copy);
        }
        
        std.debug.print("✅ Implementation of {} for {} with {} methods\n", .{
            impl_stmt.interface_name,
            impl_stmt.implementing_type,
            impl_stmt.methods.items.len,
        });
    }

    /// Generate match/pattern matching statements
    fn generateMatchStatement(self: *CodeGenerator, match_stmt: ast.MatchStatement) !void {
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Generate the expression to match against
        const match_value = try self.generateExpression(match_stmt.expression.*);
        
        // Create basic blocks for each case and default
        var case_blocks = try self.allocator.alloc(c.LLVMBasicBlockRef, match_stmt.cases.items.len);
        defer self.allocator.free(case_blocks);
        
        const default_block = if (match_stmt.default_case) |_|
            c.LLVMAppendBasicBlockInContext(self.context, current_func, "match.default")
        else
            null;
        
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "match.end");
        
        // Create blocks for each case
        for (match_stmt.cases.items, 0..) |_, i| {
            var case_name_buf: [32]u8 = undefined;
            const case_name = try std.fmt.bufPrint(&case_name_buf, "match.case.{}", .{i});
            case_blocks[i] = c.LLVMAppendBasicBlockInContext(self.context, current_func, case_name.ptr);
        }
        
        // Generate switch instruction
        const switch_inst = c.LLVMBuildSwitch(
            self.builder,
            match_value,
            default_block orelse merge_block,
            @intCast(match_stmt.cases.items.len)
        );
        
        // Add cases to switch
        for (match_stmt.cases.items, 0..) |match_case, i| {
            // Generate pattern value (simplified for literals)
            switch (match_case.pattern) {
                .Literal => |literal| {
                    const case_value = try self.generateLiteral(literal);
                    c.LLVMAddCase(switch_inst, case_value, case_blocks[i]);
                },
                else => {
                    std.debug.print("Warning: Non-literal patterns not fully implemented\n", .{});
                }
            }
            
            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.builder, case_blocks[i]);
            const result_expr: *ast.Expression = @ptrCast(@alignCast(match_case.result));
            _ = try self.generateExpression(result_expr.*);
            
            if (!c.LLVMGetBasicBlockTerminator(case_blocks[i])) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }
        
        // Generate default case if present
        if (match_stmt.default_case) |default_expr| {
            c.LLVMPositionBuilderAtEnd(self.builder, default_block.?);
            const default_result: *ast.Expression = @ptrCast(@alignCast(default_expr));
            _ = try self.generateExpression(default_result.*);
            
            if (!c.LLVMGetBasicBlockTerminator(default_block.?)) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }
        
        // Continue with merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
    }

    /// Generate defer statements
    fn generateDeferStatement(self: *CodeGenerator, defer_stmt: ast.DeferStatement) !void {
        // Defer statements need to be executed at function exit
        // For simplicity, we'll implement basic defer by storing the statement
        // and generating it before each return in the function
        
        // In a full implementation, we'd maintain a defer stack per function
        std.debug.print("⚠️  Defer statement registered (simplified implementation)\n", .{});
        
        // For now, just generate the deferred statement immediately as a placeholder
        const deferred_stmt: *ast.Statement = @ptrCast(@alignCast(defer_stmt.statement));
        try self.generateStatement(deferred_stmt.*);
    }

    /// Generate try statements
    fn generateTryStatement(self: *CodeGenerator, try_stmt: ast.TryStatement) !void {
        // Try statements in CURSED are similar to try-catch blocks
        // They need exception handling support
        _ = self;
        _ = try_stmt;
        
        std.debug.print("⚠️  Try statement (exception handling placeholder)\n", .{});
        
        // For now, just generate the try block without error handling
        // In a full implementation, this would set up exception handling frames
    }

    /// Generate goroutine/async statements
    fn generateGoroutineStatement(self: *CodeGenerator, goroutine_stmt: ast.GoroutineStatement) !void {
        // Goroutines require runtime support for coroutines/green threads
        // For now, we'll generate a simple function call
        
        std.debug.print("⚠️  Goroutine started (simplified as function call)\n", .{});
        
        // Generate the function call that should run as a goroutine
        _ = try self.generateCallExpression(goroutine_stmt.call);
        
        // In a full implementation, this would:
        // 1. Create a new goroutine/fiber
        // 2. Set up its stack and context
        // 3. Schedule it for execution
        // 4. Return immediately to the caller
    }

    /// Generate select statements for channel operations
    fn generateSelectStatement(self: *CodeGenerator, select_stmt: ast.SelectStatement) !void {
        // Select statements are complex - they involve non-blocking channel operations
        // and require runtime support for channel multiplexing
        
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Create blocks for each case and default
        var case_blocks = try self.allocator.alloc(c.LLVMBasicBlockRef, select_stmt.cases.items.len);
        defer self.allocator.free(case_blocks);
        
        const default_block = if (select_stmt.default_case) |_|
            c.LLVMAppendBasicBlockInContext(self.context, current_func, "select.default")
        else
            null;
        
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "select.end");
        
        // Create blocks for each case
        for (select_stmt.cases.items, 0..) |_, i| {
            var case_name_buf: [32]u8 = undefined;
            const case_name = try std.fmt.bufPrint(&case_name_buf, "select.case.{}", .{i});
            case_blocks[i] = c.LLVMAppendBasicBlockInContext(self.context, current_func, case_name.ptr);
        }
        
        std.debug.print("⚠️  Select statement (simplified implementation)\n", .{});
        
        // For now, just execute the first available case
        // In a full implementation, this would use runtime functions to:
        // 1. Check which channels are ready
        // 2. Select one randomly if multiple are ready
        // 3. Block until at least one is ready
        
        if (select_stmt.cases.items.len > 0) {
            _ = c.LLVMBuildBr(self.builder, case_blocks[0]);
            
            // Generate first case as example
            c.LLVMPositionBuilderAtEnd(self.builder, case_blocks[0]);
            const first_case = select_stmt.cases.items[0];
            for (first_case.body.items) |stmt| {
                const case_stmt: *ast.Statement = @ptrCast(@alignCast(stmt));
                try self.generateStatement(case_stmt.*);
            }
            
            if (!c.LLVMGetBasicBlockTerminator(case_blocks[0])) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        } else if (default_block) |default_bb| {
            _ = c.LLVMBuildBr(self.builder, default_bb);
            c.LLVMPositionBuilderAtEnd(self.builder, default_bb);
            
            if (select_stmt.default_case) |default_stmts| {
                for (default_stmts.items) |stmt| {
                    const default_stmt: *ast.Statement = @ptrCast(@alignCast(stmt));
                    try self.generateStatement(default_stmt.*);
                }
            }
            
            if (!c.LLVMGetBasicBlockTerminator(default_bb)) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        } else {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Continue with merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
    }

    /// Generate break statements
    fn generateBreakStatement(self: *CodeGenerator, break_stmt: ast.BreakStatement) !void {
        _ = self;
        _ = break_stmt;
        // Break statements need to jump to the loop exit block
        // This requires maintaining a stack of loop exit blocks
        std.debug.print("⚠️  Break statement (needs loop context)\n", .{});
    }

    /// Generate continue statements
    fn generateContinueStatement(self: *CodeGenerator, continue_stmt: ast.ContinueStatement) !void {
        _ = self;
        _ = continue_stmt;
        // Continue statements need to jump to the loop header/update block
        // This requires maintaining a stack of loop continue blocks
        std.debug.print("⚠️  Continue statement (needs loop context)\n", .{});
    }

    /// Generate constant definitions
    fn generateConstantStatement(self: *CodeGenerator, const_stmt: ast.ConstantStatement) !void {
        const value = try self.generateExpression(@ptrCast(@alignCast(const_stmt.value)));
        
        // Create a global constant
        const const_type = c.LLVMTypeOf(value);
        const global_const = c.LLVMAddGlobal(self.module, const_type, const_stmt.name.ptr);
        c.LLVMSetInitializer(global_const, value);
        c.LLVMSetGlobalConstant(global_const, 1);
        c.LLVMSetLinkage(global_const, c.LLVMPrivateLinkage);
        
        // Store in variables map for later reference
        try self.variables.put(const_stmt.name, global_const);
        
        std.debug.print("✅ Constant '{s}' defined\n", .{const_stmt.name});
    }

    /// Generate type alias statements
    fn generateTypeStatement(self: *CodeGenerator, type_stmt: ast.TypeStatement) !void {
        // Type aliases don't generate runtime code, but we store them for type checking
        _ = self;
        std.debug.print("✅ Type alias '{s}' registered\n", .{type_stmt.name});
    }

    // Additional statement generation functions
    fn generateLetStatement(self: *CodeGenerator, let_stmt: ast.LetStatement) !void {
        const llvm_type = self.getLLVMType("normie"); // Default type for now
        const alloca = c.LLVMBuildAlloca(self.builder, llvm_type, let_stmt.name.ptr);
        
        if (let_stmt.initializer) |initializer| {
            const init_expr: *ast.Expression = @ptrCast(@alignCast(initializer));
            const init_value = try self.generateExpression(init_expr.*);
            _ = c.LLVMBuildStore(self.builder, init_value, alloca);
        }
        
        try self.variables.put(let_stmt.name, alloca);
        std.debug.print("✅ Let variable '{s}' declared\n", .{let_stmt.name});
    }

    fn generateAssignmentStatement(self: *CodeGenerator, assign_stmt: ast.AssignmentStatement) !void {
        const target_expr: *ast.Expression = @ptrCast(@alignCast(assign_stmt.target));
        const value_expr: *ast.Expression = @ptrCast(@alignCast(assign_stmt.value));
        
        if (target_expr.* == .Identifier) {
            const var_name = target_expr.Identifier;
            if (self.variables.get(var_name)) |variable| {
                const value = try self.generateExpression(value_expr.*);
                _ = c.LLVMBuildStore(self.builder, value, variable);
            } else {
                std.debug.print("Error: Undefined variable '{s}' in assignment\n", .{var_name});
                return error.UndefinedSymbol;
            }
        }
    }

    fn generateForInStatement(self: *CodeGenerator, for_in_stmt: ast.ForInStatement) !void {
        _ = self;
        _ = for_in_stmt;
        std.debug.print("⚠️ ForIn statement implementation placeholder\n", .{});
    }

    fn generateSwitchStatement(self: *CodeGenerator, switch_stmt: ast.SwitchStatement) !void {
        _ = self;
        _ = switch_stmt;
        std.debug.print("⚠️ Switch statement implementation placeholder\n", .{});
    }

    fn generatePatternSwitchStatement(self: *CodeGenerator, pattern_stmt: ast.PatternSwitchStatement) !void {
        _ = self;
        _ = pattern_stmt;
        std.debug.print("⚠️ Pattern switch statement implementation placeholder\n", .{});
    }

    fn generateTypeAliasStatement(self: *CodeGenerator, type_alias: ast.TypeAliasStatement) !void {
        _ = self;
        std.debug.print("✅ Type alias '{s}' defined\n", .{type_alias.name});
    }

    fn generateChannelStatement(self: *CodeGenerator, channel_stmt: ast.ChannelStatement) !void {
        _ = self;
        _ = channel_stmt;
        std.debug.print("⚠️ Channel statement implementation placeholder\n", .{});
    }

    fn generateIncrementStatement(self: *CodeGenerator, inc_stmt: ast.IncrementStatement) !void {
        _ = self;
        _ = inc_stmt;
        std.debug.print("⚠️ Increment statement implementation placeholder\n", .{});
    }

    fn generateDecrementStatement(self: *CodeGenerator, dec_stmt: ast.DecrementStatement) !void {
        _ = self;
        _ = dec_stmt;
        std.debug.print("⚠️ Decrement statement implementation placeholder\n", .{});
    }

    fn generateShortDeclarationStatement(self: *CodeGenerator, short_decl: ast.ShortDeclarationStatement) !void {
        _ = self;
        _ = short_decl;
        std.debug.print("⚠️ Short declaration statement implementation placeholder\n", .{});
    }

    fn generatePanicStatement(self: *CodeGenerator, panic_stmt: ast.PanicStatement) !void {
        _ = self;
        _ = panic_stmt;
        std.debug.print("⚠️ Panic statement implementation placeholder\n", .{});
    }

    fn generateCatchStatement(self: *CodeGenerator, catch_stmt: ast.CatchStatement) !void {
        _ = self;
        _ = catch_stmt;
        std.debug.print("⚠️ Catch statement implementation placeholder\n", .{});
    }

    fn generateYikesStatement(self: *CodeGenerator, yikes_stmt: ast.YikesStatement) !void {
        _ = self;
        _ = yikes_stmt;
        std.debug.print("⚠️ Yikes statement implementation placeholder\n", .{});
    }

    fn generateFamStatement(self: *CodeGenerator, fam_stmt: ast.FamStatement) !void {
        _ = self;
        _ = fam_stmt;
        std.debug.print("⚠️ Fam statement implementation placeholder\n", .{});
    }

    fn generateConstStatement(self: *CodeGenerator, const_decl: ast.ConstDecl) !void {
        const value_expr: *ast.Expression = @ptrCast(@alignCast(const_decl.value));
        const value = try self.generateExpression(value_expr.*);
        
        // Create a global constant
        const const_type = c.LLVMTypeOf(value);
        const global_const = c.LLVMAddGlobal(self.module, const_type, const_decl.name.ptr);
        c.LLVMSetInitializer(global_const, value);
        c.LLVMSetGlobalConstant(global_const, 1);
        c.LLVMSetLinkage(global_const, c.LLVMPrivateLinkage);
        
        try self.variables.put(const_decl.name, global_const);
        std.debug.print("✅ Constant '{s}' defined\n", .{const_decl.name});
    }

    // Expression generation functions for missing types
    fn generateSliceAccessExpression(self: *CodeGenerator, slice: ast.SliceAccessExpression) !c.LLVMValueRef {
        _ = slice;
        std.debug.print("⚠️ Slice access expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateTupleAccessExpression(self: *CodeGenerator, tuple_access: ast.TupleAccessExpression) !c.LLVMValueRef {
        _ = tuple_access;
        std.debug.print("⚠️ Tuple access expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateCompositeLiteralExpression(self: *CodeGenerator, composite: ast.CompositeLiteralExpression) !c.LLVMValueRef {
        _ = composite;
        std.debug.print("⚠️ Composite literal expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateChannelSendExpression(self: *CodeGenerator, channel_send: ast.ChannelSendExpression) !c.LLVMValueRef {
        _ = channel_send;
        std.debug.print("⚠️ Channel send expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateChannelReceiveExpression(self: *CodeGenerator, channel_recv: ast.ChannelReceiveExpression) !c.LLVMValueRef {
        _ = channel_recv;
        std.debug.print("⚠️ Channel receive expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateChannelCreationExpression(self: *CodeGenerator, channel_create: ast.ChannelCreationExpression) !c.LLVMValueRef {
        _ = channel_create;
        std.debug.print("⚠️ Channel creation expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateStructLiteralExpression(self: *CodeGenerator, struct_lit: ast.StructLiteralExpression) !c.LLVMValueRef {
        _ = struct_lit;
        std.debug.print("⚠️ Struct literal expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateIncrementExpression(self: *CodeGenerator, increment: ast.IncrementExpression) !c.LLVMValueRef {
        _ = increment;
        std.debug.print("⚠️ Increment expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateDecrementExpression(self: *CodeGenerator, decrement: ast.DecrementExpression) !c.LLVMValueRef {
        _ = decrement;
        std.debug.print("⚠️ Decrement expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateYikesExpression(self: *CodeGenerator, yikes: ast.YikesExpression) !c.LLVMValueRef {
        _ = yikes;
        std.debug.print("⚠️ Yikes expression placeholder (error propagation)\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateShookExpression(self: *CodeGenerator, shook: ast.ShookExpression) !c.LLVMValueRef {
        _ = shook;
        std.debug.print("⚠️ Shook expression placeholder (error handling)\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateFamExpression(self: *CodeGenerator, fam: ast.FamExpression) !c.LLVMValueRef {
        _ = fam;
        std.debug.print("⚠️ Fam expression placeholder (error recovery)\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateErrorValueExpression(self: *CodeGenerator, error_val: ast.ErrorValueExpression) !c.LLVMValueRef {
        _ = error_val;
        std.debug.print("⚠️ Error value expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateStructuredErrorExpression(self: *CodeGenerator, struct_err: ast.StructuredErrorExpression) !c.LLVMValueRef {
        _ = struct_err;
        std.debug.print("⚠️ Structured error expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generatePanicExpression(self: *CodeGenerator, panic: ast.PanicExpression) !c.LLVMValueRef {
        _ = panic;
        std.debug.print("⚠️ Panic expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateRecoverExpression(self: *CodeGenerator, recover: ast.RecoverExpression) !c.LLVMValueRef {
        _ = recover;
        std.debug.print("⚠️ Recover expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateTestResultExpression(self: *CodeGenerator, test_result: ast.TestResultExpression) !c.LLVMValueRef {
        _ = test_result;
        std.debug.print("⚠️ Test result expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateTestResultCheckExpression(self: *CodeGenerator, test_check: ast.TestResultCheckExpression) !c.LLVMValueRef {
        _ = test_check;
        std.debug.print("⚠️ Test result check expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateRangeForExpression(self: *CodeGenerator, range_for: ast.RangeForExpression) !c.LLVMValueRef {
        _ = range_for;
        std.debug.print("⚠️ Range for expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateMatchExpression(self: *CodeGenerator, match: ast.MatchExpression) !c.LLVMValueRef {
        _ = match;
        std.debug.print("⚠️ Match expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    fn generateTypeSwitchExpression(self: *CodeGenerator, type_switch: ast.TypeSwitchExpression) !c.LLVMValueRef {
        _ = type_switch;
        std.debug.print("⚠️ Type switch expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }
};
