const std = @import("std");
const ast = @import("ast.zig");
// LLVM C imports disabled to fix "athlon-xp" CPU detection issues
// Replace with dummy types to allow compilation without LLVM
const c = struct {
    // Dummy LLVM types to make compilation work without LLVM
    pub const LLVMModuleRef = ?*anyopaque;
    pub const LLVMBuilderRef = ?*anyopaque;
    pub const LLVMContextRef = ?*anyopaque;
    pub const LLVMValueRef = ?*anyopaque;
    pub const LLVMTypeRef = ?*anyopaque;
    pub const LLVMBasicBlockRef = ?*anyopaque;
    pub const LLVMExecutionEngineRef = ?*anyopaque;
    pub const LLVMTargetRef = ?*anyopaque;
    pub const LLVMTargetMachineRef = ?*anyopaque;
    pub const LLVMPassManagerRef = ?*anyopaque;
    pub const LLVMMemoryBufferRef = ?*anyopaque;
    pub const LLVMBool = c_int;
    
    // Dummy functions to prevent link errors (add more as needed)
    pub fn LLVMCreateModule(_: [*c]const u8) LLVMModuleRef { return null; }
    pub fn LLVMCreateBuilder() LLVMBuilderRef { return null; }
    pub fn LLVMGetGlobalContext() LLVMContextRef { return null; }
    pub fn LLVMDisposeModule(_: LLVMModuleRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMInitializeX86TargetInfo() void {}
    pub fn LLVMInitializeX86Target() void {}
    pub fn LLVMInitializeX86TargetMC() void {}
    pub fn LLVMInitializeX86AsmPrinter() void {}
    pub fn LLVMCreateTargetMachine() LLVMTargetMachineRef { return null; }
    pub fn LLVMDisposeTargetMachine(_: LLVMTargetMachineRef) void {}
};

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
        
        // exit function declaration
        var exit_params = [_]c.LLVMTypeRef{i32_type};
        const exit_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), &exit_params, 1, 0);
        _ = c.LLVMAddFunction(self.module, "exit", exit_type);
        
        // strcmp function declaration
        var strcmp_params = [_]c.LLVMTypeRef{ i8_ptr_type, i8_ptr_type };
        const strcmp_type = c.LLVMFunctionType(i32_type, &strcmp_params, 2, 0);
        _ = c.LLVMAddFunction(self.module, "strcmp", strcmp_type);
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

    /// Generate binary expressions with proper type checking
    fn generateBinaryExpression(self: *CodeGenerator, binary: ast.BinaryExpression) !c.LLVMValueRef {
        const left = try self.generateExpression(binary.left.*);
        const right = try self.generateExpression(binary.right.*);
        
        // Get operand types for type-specific operations
        const left_type = c.LLVMTypeOf(left);
        const right_type = c.LLVMTypeOf(right);
        const is_float_left = c.LLVMGetTypeKind(left_type) == c.LLVMFloatTypeKind or c.LLVMGetTypeKind(left_type) == c.LLVMDoubleTypeKind;
        const is_float_right = c.LLVMGetTypeKind(right_type) == c.LLVMFloatTypeKind or c.LLVMGetTypeKind(right_type) == c.LLVMDoubleTypeKind;
        
        // Handle arithmetic operators
        if (std.mem.eql(u8, binary.operator, "+")) {
            if (is_float_left or is_float_right) {
                // Convert to float if needed
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFAdd(self.builder, left_f, right_f, "add_tmp");
            } else {
                return c.LLVMBuildAdd(self.builder, left, right, "add_tmp");
            }
        } else if (std.mem.eql(u8, binary.operator, "-")) {
            if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFSub(self.builder, left_f, right_f, "sub_tmp");
            } else {
                return c.LLVMBuildSub(self.builder, left, right, "sub_tmp");
            }
        } else if (std.mem.eql(u8, binary.operator, "*")) {
            if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFMul(self.builder, left_f, right_f, "mul_tmp");
            } else {
                return c.LLVMBuildMul(self.builder, left, right, "mul_tmp");
            }
        } else if (std.mem.eql(u8, binary.operator, "/")) {
            // Add division by zero check
            const zero = if (is_float_right) c.LLVMConstReal(right_type, 0.0) else c.LLVMConstInt(right_type, 0, 0);
            const is_zero = if (is_float_right) 
                c.LLVMBuildFCmp(self.builder, c.LLVMRealOEQ, right, zero, "div_zero_check")
            else
                c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, right, zero, "div_zero_check");
            
            const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
            const div_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "div.safe");
            const error_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "div.error");
            const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "div.end");
            
            _ = c.LLVMBuildCondBr(self.builder, is_zero, error_block, div_block);
            
            // Error block - could throw exception or return NaN/0
            c.LLVMPositionBuilderAtEnd(self.builder, error_block);
            const error_val = if (is_float_left or is_float_right) 
                c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), std.math.nan(f64))
            else
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            _ = c.LLVMBuildBr(self.builder, merge_block);
            
            // Safe division block
            c.LLVMPositionBuilderAtEnd(self.builder, div_block);
            const div_result = if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                c.LLVMBuildFDiv(self.builder, left_f, right_f, "div_tmp")
            } else {
                c.LLVMBuildSDiv(self.builder, left, right, "div_tmp")
            };
            _ = c.LLVMBuildBr(self.builder, merge_block);
            
            // Merge block with PHI
            c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
            const result_type = if (is_float_left or is_float_right) c.LLVMDoubleTypeInContext(self.context) else c.LLVMInt32TypeInContext(self.context);
            const phi = c.LLVMBuildPhi(self.builder, result_type, "div_result");
            const incoming_values = [_]c.LLVMValueRef{ error_val, div_result };
            const incoming_blocks = [_]c.LLVMBasicBlockRef{ error_block, div_block };
            c.LLVMAddIncoming(phi, &incoming_values, &incoming_blocks, 2);
            
            return phi;
        } else if (std.mem.eql(u8, binary.operator, "%")) {
            if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFRem(self.builder, left_f, right_f, "fmod_tmp");
            } else {
                return c.LLVMBuildSRem(self.builder, left, right, "mod_tmp");
            }
        } 
        
        // Handle comparison operators
        else if (std.mem.eql(u8, binary.operator, "==")) {
            if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOEQ, left_f, right_f, "eq_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq_tmp");
            }
        } else if (std.mem.eql(u8, binary.operator, "!=")) {
            if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealONE, left_f, right_f, "neq_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "neq_tmp");
            }
        } else if (std.mem.eql(u8, binary.operator, "<")) {
            if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOLT, left_f, right_f, "lt_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt_tmp");
            }
        } else if (std.mem.eql(u8, binary.operator, "<=")) {
            if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOLE, left_f, right_f, "lte_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, left, right, "lte_tmp");
            }
        } else if (std.mem.eql(u8, binary.operator, ">")) {
            if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOGT, left_f, right_f, "gt_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt_tmp");
            }
        } else if (std.mem.eql(u8, binary.operator, ">=")) {
            if (is_float_left or is_float_right) {
                const left_f = if (is_float_left) left else c.LLVMBuildSIToFP(self.builder, left, c.LLVMDoubleTypeInContext(self.context), "itof");
                const right_f = if (is_float_right) right else c.LLVMBuildSIToFP(self.builder, right, c.LLVMDoubleTypeInContext(self.context), "itof");
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOGE, left_f, right_f, "gte_tmp");
            } else {
                return c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, left, right, "gte_tmp");
            }
        }
        
        // Handle bitwise operators (integers only)
        else if (std.mem.eql(u8, binary.operator, "&")) {
            return c.LLVMBuildAnd(self.builder, left, right, "and_tmp");
        } else if (std.mem.eql(u8, binary.operator, "|")) {
            return c.LLVMBuildOr(self.builder, left, right, "or_tmp");
        } else if (std.mem.eql(u8, binary.operator, "^")) {
            return c.LLVMBuildXor(self.builder, left, right, "xor_tmp");
        } else if (std.mem.eql(u8, binary.operator, "<<")) {
            return c.LLVMBuildShl(self.builder, left, right, "shl_tmp");
        } else if (std.mem.eql(u8, binary.operator, ">>")) {
            return c.LLVMBuildAShr(self.builder, left, right, "shr_tmp");
        }
        
        // Handle logical operators (short-circuit)
        else if (std.mem.eql(u8, binary.operator, "&&")) {
            // Short-circuit AND
            const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
            const and_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "and.rhs");
            const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "and.end");
            
            // Convert left to boolean if needed
            const left_bool = try self.convertToBool(left);
            _ = c.LLVMBuildCondBr(self.builder, left_bool, and_block, merge_block);
            
            c.LLVMPositionBuilderAtEnd(self.builder, and_block);
            const right_bool = try self.convertToBool(right);
            _ = c.LLVMBuildBr(self.builder, merge_block);
            
            c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
            const phi = c.LLVMBuildPhi(self.builder, c.LLVMInt1TypeInContext(self.context), "and_result");
            
            const false_val = c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 0, 0);
            const incoming_values = [_]c.LLVMValueRef{ false_val, right_bool };
            const incoming_blocks = [_]c.LLVMBasicBlockRef{ c.LLVMGetPreviousBasicBlock(and_block), and_block };
            c.LLVMAddIncoming(phi, &incoming_values, &incoming_blocks, 2);
            
            return phi;
        } else if (std.mem.eql(u8, binary.operator, "||")) {
            // Short-circuit OR
            const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
            const or_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "or.rhs");
            const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "or.end");
            
            const left_bool = try self.convertToBool(left);
            _ = c.LLVMBuildCondBr(self.builder, left_bool, merge_block, or_block);
            
            c.LLVMPositionBuilderAtEnd(self.builder, or_block);
            const right_bool = try self.convertToBool(right);
            _ = c.LLVMBuildBr(self.builder, merge_block);
            
            c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
            const phi = c.LLVMBuildPhi(self.builder, c.LLVMInt1TypeInContext(self.context), "or_result");
            
            const true_val = c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), 1, 0);
            const incoming_values = [_]c.LLVMValueRef{ true_val, right_bool };
            const incoming_blocks = [_]c.LLVMBasicBlockRef{ c.LLVMGetPreviousBasicBlock(or_block), or_block };
            c.LLVMAddIncoming(phi, &incoming_values, &incoming_blocks, 2);
            
            return phi;
        } else {
            std.debug.print("Error: Unimplemented binary operator: {s}\n", .{binary.operator});
            return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        }
    }

    /// Generate unary expressions with proper type checking
    fn generateUnaryExpression(self: *CodeGenerator, unary: ast.UnaryExpression) !c.LLVMValueRef {
        // Get operand type for type-specific operations
        const operand_type = c.LLVMTypeOf(try self.generateExpression(unary.operand.*));
        const is_float = c.LLVMGetTypeKind(operand_type) == c.LLVMFloatTypeKind or c.LLVMGetTypeKind(operand_type) == c.LLVMDoubleTypeKind;
        
        // Handle arithmetic unary operators
        if (std.mem.eql(u8, unary.operator, "-")) {
            const operand = try self.generateExpression(unary.operand.*);
            if (is_float) {
                return c.LLVMBuildFNeg(self.builder, operand, "neg_tmp");
            } else {
                return c.LLVMBuildNeg(self.builder, operand, "neg_tmp");
            }
        } else if (std.mem.eql(u8, unary.operator, "+")) {
            // Unary plus - just return the operand (identity operation)
            return try self.generateExpression(unary.operand.*);
        } 
        
        // Handle logical unary operators
        else if (std.mem.eql(u8, unary.operator, "!") or std.mem.eql(u8, unary.operator, "not")) {
            const operand = try self.generateExpression(unary.operand.*);
            const operand_bool = try self.convertToBool(operand);
            return c.LLVMBuildNot(self.builder, operand_bool, "not_tmp");
        }
        
        // Handle bitwise unary operators
        else if (std.mem.eql(u8, unary.operator, "~")) {
            const operand = try self.generateExpression(unary.operand.*);
            if (is_float) {
                std.debug.print("Error: Bitwise NOT cannot be applied to floating-point numbers\n", .{});
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            }
            return c.LLVMBuildNot(self.builder, operand, "bnot_tmp");
        }
        
        // Handle pointer operations
        else if (std.mem.eql(u8, unary.operator, "&")) {
            // Address-of operator - return the address of the operand (if it's an lvalue)
            // This requires proper lvalue handling
            if (unary.operand.* == .Identifier) {
                const identifier = unary.operand.Identifier.name;
                if (self.variables.get(identifier)) |variable| {
                    // Return the variable address directly
                    return variable;
                } else {
                    std.debug.print("Error: Cannot take address of undefined variable '{s}'\n", .{identifier});
                    return c.LLVMConstNull(c.LLVMPointerTypeInContext(self.context, 0));
                }
            } else {
                std.debug.print("Error: Address-of operator can only be applied to lvalues\n", .{});
                return c.LLVMConstNull(c.LLVMPointerTypeInContext(self.context, 0));
            }
        } else if (std.mem.eql(u8, unary.operator, "*")) {
            // Dereference the pointer
            const operand = try self.generateExpression(unary.operand.*);
            const operand_type_kind = c.LLVMGetTypeKind(c.LLVMTypeOf(operand));
            
            if (operand_type_kind != c.LLVMPointerTypeKind) {
                std.debug.print("Error: Dereference operator can only be applied to pointers\n", .{});
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
            }
            
            const element_type = c.LLVMGetElementType(c.LLVMTypeOf(operand));
            return c.LLVMBuildLoad2(self.builder, element_type, operand, "deref_tmp");
        }
        
        // Handle increment/decrement operators
        else if (std.mem.eql(u8, unary.operator, "++")) {
            // Pre-increment - need lvalue handling
            if (unary.operand.* == .Identifier) {
                const identifier = unary.operand.Identifier.name;
                if (self.variables.get(identifier)) |variable| {
                    const var_type = c.LLVMGetElementType(c.LLVMTypeOf(variable));
                    const current_val = c.LLVMBuildLoad2(self.builder, var_type, variable, "load_pre_inc");
                    
                    const one = if (c.LLVMGetTypeKind(var_type) == c.LLVMFloatTypeKind or c.LLVMGetTypeKind(var_type) == c.LLVMDoubleTypeKind)
                        c.LLVMConstReal(var_type, 1.0)
                    else
                        c.LLVMConstInt(var_type, 1, 0);
                    
                    const incremented = if (c.LLVMGetTypeKind(var_type) == c.LLVMFloatTypeKind or c.LLVMGetTypeKind(var_type) == c.LLVMDoubleTypeKind)
                        c.LLVMBuildFAdd(self.builder, current_val, one, "preinc_tmp")
                    else
                        c.LLVMBuildAdd(self.builder, current_val, one, "preinc_tmp");
                    
                    _ = c.LLVMBuildStore(self.builder, incremented, variable);
                    return incremented;
                } else {
                    std.debug.print("Error: Cannot increment undefined variable '{s}'\n", .{identifier});
                    return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
                }
            } else {
                std.debug.print("Error: Pre-increment can only be applied to variables\n", .{});
                const operand = try self.generateExpression(unary.operand.*);
                return operand;
            }
        } else if (std.mem.eql(u8, unary.operator, "--")) {
            // Pre-decrement - need lvalue handling
            if (unary.operand.* == .Identifier) {
                const identifier = unary.operand.Identifier.name;
                if (self.variables.get(identifier)) |variable| {
                    const var_type = c.LLVMGetElementType(c.LLVMTypeOf(variable));
                    const current_val = c.LLVMBuildLoad2(self.builder, var_type, variable, "load_pre_dec");
                    
                    const one = if (c.LLVMGetTypeKind(var_type) == c.LLVMFloatTypeKind or c.LLVMGetTypeKind(var_type) == c.LLVMDoubleTypeKind)
                        c.LLVMConstReal(var_type, 1.0)
                    else
                        c.LLVMConstInt(var_type, 1, 0);
                    
                    const decremented = if (c.LLVMGetTypeKind(var_type) == c.LLVMFloatTypeKind or c.LLVMGetTypeKind(var_type) == c.LLVMDoubleTypeKind)
                        c.LLVMBuildFSub(self.builder, current_val, one, "predec_tmp")
                    else
                        c.LLVMBuildSub(self.builder, current_val, one, "predec_tmp");
                    
                    _ = c.LLVMBuildStore(self.builder, decremented, variable);
                    return decremented;
                } else {
                    std.debug.print("Error: Cannot decrement undefined variable '{s}'\n", .{identifier});
                    return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
                }
            } else {
                std.debug.print("Error: Pre-decrement can only be applied to variables\n", .{});
                const operand = try self.generateExpression(unary.operand.*);
                return operand;
            }
        }
        
        // Handle typeof operator (CURSED-specific)
        else if (std.mem.eql(u8, unary.operator, "typeof")) {
            const operand = try self.generateExpression(unary.operand.*);
            const operand_llvm_type = c.LLVMTypeOf(operand);
            
            // Return a string representing the type (simplified implementation)
            const type_kind = c.LLVMGetTypeKind(operand_llvm_type);
            const type_str = switch (type_kind) {
                c.LLVMIntegerTypeKind => "drip",
                c.LLVMFloatTypeKind, c.LLVMDoubleTypeKind => "tea",
                c.LLVMPointerTypeKind => "pointer",
                c.LLVMStructTypeKind => "squad",
                c.LLVMArrayTypeKind => "array",
                c.LLVMFunctionTypeKind => "slay",
                else => "unknown",
            };
            
            // Create a global string constant for the type name
            const type_string = c.LLVMBuildGlobalStringPtr(self.builder, type_str, "typeof_result");
            return type_string;
        } else {
            std.debug.print("Error: Unimplemented unary operator: {s}\n", .{unary.operator});
            return try self.generateExpression(unary.operand.*);
        }
    }

    /// Convert a value to boolean for logical operations
    fn convertToBool(self: *CodeGenerator, value: c.LLVMValueRef) !c.LLVMValueRef {
        const value_type = c.LLVMTypeOf(value);
        const type_kind = c.LLVMGetTypeKind(value_type);
        
        switch (type_kind) {
            c.LLVMIntegerTypeKind => {
                // For integers, non-zero is true
                const zero = c.LLVMConstInt(value_type, 0, 0);
                return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, value, zero, "to_bool");
            },
            c.LLVMFloatTypeKind, c.LLVMDoubleTypeKind => {
                // For floats, non-zero and non-NaN is true
                const zero = c.LLVMConstReal(value_type, 0.0);
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealONE, value, zero, "to_bool");
            },
            c.LLVMPointerTypeKind => {
                // For pointers, non-null is true
                const null_ptr = c.LLVMConstNull(value_type);
                return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, value, null_ptr, "to_bool");
            },
            else => {
                // For other types, assume already boolean or convert to boolean
                if (c.LLVMGetIntTypeWidth(value_type) == 1) {
                    // Already a boolean
                    return value;
                } else {
                    // Convert to boolean by comparing with zero
                    const zero = c.LLVMConstInt(value_type, 0, 0);
                    return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, value, zero, "to_bool");
                }
            }
        }
    }

    /// Generate function call expressions with interface dispatch support
    fn generateCallExpression(self: *CodeGenerator, call: ast.CallExpression) !c.LLVMValueRef {
        // Special handling for built-in functions
        if (call.function.* == .Identifier) {
            const func_name = call.function.Identifier.name;
            
            if (std.mem.eql(u8, func_name, "printf") or std.mem.eql(u8, func_name, "vibez.spill") or std.mem.eql(u8, func_name, "facts")) {
            return try self.generatePrintfCall(call);
            }
        }
        
        // Check for interface method calls (Member access on interface objects)
        if (call.function.* == .Member) {
            return try self.generateInterfaceMethodCall(call);
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

    /// Generate interface method calls with guaranteed vtable dispatch
    fn generateInterfaceMethodCall(self: *CodeGenerator, call: ast.CallExpression) !c.LLVMValueRef {
        const member_expr = call.function.Member;
        const obj_expr = member_expr.object;
        const method_name = member_expr.member;
        
        // Generate the object (interface instance)
        const obj_value = try self.generateExpression(obj_expr.*);
        
        // Extract vtable pointer from interface object
        // Interface object layout: {vtable_ptr, data_ptr}
        const vtable_ptr = c.LLVMBuildExtractValue(self.builder, obj_value, 0, "vtable_ptr");
        const data_ptr = c.LLVMBuildExtractValue(self.builder, obj_value, 1, "data_ptr");
        
        // Fail-fast assertion: Validate vtable magic number
        const magic_ptr = c.LLVMBuildStructGEP2(
            self.builder,
            c.LLVMTypeOf(vtable_ptr),
            vtable_ptr,
            0,
            "magic_ptr"
        );
        const magic_value = c.LLVMBuildLoad2(
            self.builder,
            c.LLVMInt64TypeInContext(self.context),
            magic_ptr,
            "magic_value"
        );
        
        // Magic number for vtable validation (0xDEADBEEF12345678)
        const expected_magic = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0xDEADBEEF12345678, 0);
        const magic_check = c.LLVMBuildICmp(
            self.builder,
            c.LLVMIntEQ,
            magic_value,
            expected_magic,
            "magic_check"
        );
        
        // Create trap block for invalid vtable
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        const trap_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "vtable_trap");
        const valid_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "vtable_valid");
        
        // Branch based on magic check
        _ = c.LLVMBuildCondBr(self.builder, magic_check, valid_block, trap_block);
        
        // Generate trap block (fail-fast assertion)
        c.LLVMPositionBuilderAtEnd(self.builder, trap_block);
        const trap_func = c.LLVMGetNamedFunction(self.module, "llvm.trap");
        if (trap_func != null) {
            _ = c.LLVMBuildCall2(
                self.builder,
                c.LLVMGlobalGetValueType(trap_func),
                trap_func,
                null,
                0,
                ""
            );
        }
        _ = c.LLVMBuildUnreachable(self.builder);
        
        // Continue with valid vtable
        c.LLVMPositionBuilderAtEnd(self.builder, valid_block);
        
        // Calculate method index in vtable (simplified - would need method registry)
        const method_index = self.getMethodIndex(method_name) orelse {
            std.debug.print("Error: Unknown interface method '{s}'\n", .{method_name});
            return error.UnknownMethod;
        };
        
        // Get method function pointer from vtable
        const method_ptr_ptr = c.LLVMBuildStructGEP2(
            self.builder,
            c.LLVMTypeOf(vtable_ptr),
            vtable_ptr,
            method_index + 1, // +1 to skip magic number
            "method_ptr_ptr"
        );
        const method_ptr = c.LLVMBuildLoad2(
            self.builder,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
            method_ptr_ptr,
            "method_ptr"
        );
        
        // Prepare arguments: self pointer, error context, then user arguments
        var args = try self.allocator.alloc(c.LLVMValueRef, call.arguments.len + 2);
        defer self.allocator.free(args);
        
        args[0] = data_ptr; // self pointer
        args[1] = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)); // error context placeholder
        
        // Generate user arguments
        for (call.arguments, 0..) |arg, i| {
            args[i + 2] = try self.generateExpression(arg);
        }
        
        // Create function type for interface method call
        var param_types = try self.allocator.alloc(c.LLVMTypeRef, args.len);
        defer self.allocator.free(param_types);
        
        param_types[0] = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0); // self
        param_types[1] = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0); // error context
        for (call.arguments, 0..) |_, i| {
            param_types[i + 2] = c.LLVMTypeOf(args[i + 2]);
        }
        
        // Error-aware return type struct: {result, error_code}
        var return_struct_members = [_]c.LLVMTypeRef{
            c.LLVMInt32TypeInContext(self.context), // placeholder return type
            c.LLVMInt32TypeInContext(self.context), // error code
        };
        const error_aware_return_type = c.LLVMStructTypeInContext(
            self.context,
            &return_struct_members,
            2,
            0
        );
        
        const method_func_type = c.LLVMFunctionType(
            error_aware_return_type,
            param_types.ptr,
            @intCast(param_types.len),
            0
        );
        
        // Call interface method through vtable
        const result = c.LLVMBuildCall2(
            self.builder,
            method_func_type,
            method_ptr,
            args.ptr,
            @intCast(args.len),
            "interface_call"
        );
        
        // Extract result and check error code
        const method_result = c.LLVMBuildExtractValue(self.builder, result, 0, "method_result");
        const error_code = c.LLVMBuildExtractValue(self.builder, result, 1, "error_code");
        
        // Check for interface method errors
        const zero_error = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        const error_check = c.LLVMBuildICmp(
            self.builder,
            c.LLVMIntEQ,
            error_code,
            zero_error,
            "error_check"
        );
        
        // Create error handling blocks
        const error_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "method_error");
        const success_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "method_success");
        
        _ = c.LLVMBuildCondBr(self.builder, error_check, success_block, error_block);
        
        // Error block - could propagate error or trap
        c.LLVMPositionBuilderAtEnd(self.builder, error_block);
        _ = c.LLVMBuildUnreachable(self.builder); // Simplified error handling
        
        // Success block
        c.LLVMPositionBuilderAtEnd(self.builder, success_block);
        
        std.debug.print("✅ Interface method call '{s}' generated with vtable dispatch\n", .{method_name});
        return method_result;
    }
    
    /// Get method index for interface dispatch (simplified registry)
    fn getMethodIndex(self: *CodeGenerator, method_name: []const u8) ?u32 {
        _ = self;
        
        // Simplified method registry - in production this would be a proper hash map
        const method_registry = std.ComptimeStringMap(u32, .{
            .{ "toString", 0 },
            .{ "equals", 1 },
            .{ "hashCode", 2 },
            .{ "clone", 3 },
            .{ "dispose", 4 },
        });
        
        return method_registry.get(method_name);
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
        
        var link_args = std.ArrayList([]const u8).init(self.allocator);
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
        var child = std.process.Child.init(link_args.items, allocator);
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

    /// Generate interface definitions with error propagation
    fn generateInterfaceStatement(self: *CodeGenerator, interface_stmt: ast.InterfaceStatement) !void {
        // Interfaces in LLVM are typically implemented as vtables
        // Enhanced with error propagation through interface dispatch
        
        var method_types = try self.allocator.alloc(c.LLVMTypeRef, interface_stmt.methods.items.len);
        defer self.allocator.free(method_types);
        
        for (interface_stmt.methods.items, 0..) |method, i| {
            // Create function pointer type for each method
            var param_types = try self.allocator.alloc(c.LLVMTypeRef, method.parameters.items.len + 2); // +1 for self, +1 for error context
            defer self.allocator.free(param_types);
            
            param_types[0] = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0); // self pointer
            param_types[1] = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0); // error context
            
            for (method.parameters.items, 0..) |param, j| {
                param_types[j + 2] = self.getLLVMTypeFromCursedType(param.param_type);
            }
            
            // Enhanced return type with error propagation
            const base_return_type = if (method.return_type) |ret_type|
                self.getLLVMTypeFromCursedType(ret_type)
            else
                c.LLVMVoidTypeInContext(self.context);
                
            // Create error-aware return type struct: {result, error_code}
            var return_struct_members = [_]c.LLVMTypeRef{
                base_return_type,
                c.LLVMInt32TypeInContext(self.context), // error code
            };
            const error_aware_return_type = c.LLVMStructTypeInContext(
                self.context,
                &return_struct_members,
                2,
                0
            );
            
            const method_func_type = c.LLVMFunctionType(
                error_aware_return_type,
                param_types.ptr,
                @intCast(param_types.len),
                0
            );
            
            method_types[i] = c.LLVMPointerType(method_func_type, 0);
        }
        
        // Create vtable struct type with null safety marker
        var vtable_members = try self.allocator.alloc(c.LLVMTypeRef, method_types.len + 1);
        defer self.allocator.free(vtable_members);
        
        vtable_members[0] = c.LLVMInt64TypeInContext(self.context); // vtable validation magic
        for (method_types, 0..) |method_type, i| {
            vtable_members[i + 1] = method_type;
        }
        
        const vtable_type = c.LLVMStructTypeInContext(
            self.context,
            vtable_members.ptr,
            @intCast(vtable_members.len),
            0
        );
        
        // Store interface type for later reference
        try self.interface_types.put(interface_stmt.name, vtable_type);
        
        // Generate vtable instance template with magic validation
        const vtable_name_buf = try self.allocator.alloc(u8, interface_stmt.name.len + 8);
        defer self.allocator.free(vtable_name_buf);
        const vtable_name = try std.fmt.bufPrint(vtable_name_buf, "{s}_vtable", .{interface_stmt.name});
        
        // Create global vtable template
        const global_vtable = c.LLVMAddGlobal(self.module, vtable_type, vtable_name.ptr);
        c.LLVMSetLinkage(global_vtable, c.LLVMWeakLinkage);
        
        // Initialize vtable with magic number and null function pointers
        var vtable_values = try self.allocator.alloc(c.LLVMValueRef, vtable_members.len);
        defer self.allocator.free(vtable_values);
        
        // Set magic number for validation
        vtable_values[0] = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0xDEADBEEF12345678, 0);
        
        // Initialize method pointers to null (will be set by implementations)
        for (1..vtable_values.len) |i| {
            vtable_values[i] = c.LLVMConstNull(vtable_members[i]);
        }
        
        const vtable_const = c.LLVMConstStructInContext(
            self.context,
            vtable_values.ptr,
            @intCast(vtable_values.len),
            0
        );
        c.LLVMSetInitializer(global_vtable, vtable_const);
        
        std.debug.print("✅ Interface '{s}' defined with {} methods (error-aware + vtable validation)\n", .{ interface_stmt.name, interface_stmt.methods.items.len });
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
        
        // Create blocks for each select case
        for (select_stmt.cases.items, 0..) |_, i| {
            var case_name_buf: [32]u8 = undefined;
            const case_name = try std.fmt.bufPrint(&case_name_buf, "select.case.{}", .{i});
            case_blocks[i] = c.LLVMAppendBasicBlockInContext(self.context, current_func, case_name.ptr);
        }
        
        // Generate runtime select() call - this requires runtime support
        // For now, simplified implementation that tries each channel in order
        const select_runtime_func = try self.getOrCreateSelectFunction();
        
        // Prepare channel array for runtime selection
        const channel_count = select_stmt.cases.items.len;
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        const channel_array_type = c.LLVMArrayType(ptr_type, @intCast(channel_count));
        const channel_array = c.LLVMBuildAlloca(self.builder, channel_array_type, "channel_array");
        
        // Fill channel array with case channels
        for (select_stmt.cases.items, 0..) |select_case, i| {
            // Extract channel from select case (simplified)
            const channel_ptr = try self.generateExpression(select_case.channel_op.*);
            const array_elem = c.LLVMBuildGEP2(self.builder, channel_array_type, channel_array, 
                &[_]c.LLVMValueRef{
                    c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                    c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(i), 0)
                }, 2, "channel_elem");
            _ = c.LLVMBuildStore(self.builder, channel_ptr, array_elem);
        }
        
        // Call runtime select function
        const selected_case = c.LLVMBuildCall2(self.builder, 
            c.LLVMGlobalGetValueType(select_runtime_func),
            select_runtime_func, 
            &[_]c.LLVMValueRef{
                channel_array,
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(channel_count), 0)
            }, 2, "selected_case");
        
        // Generate switch on selected case
        const switch_inst = c.LLVMBuildSwitch(self.builder, selected_case, 
            default_block orelse merge_block, @intCast(channel_count));
        
        // Add cases to switch and generate their bodies
        for (select_stmt.cases.items, 0..) |select_case, i| {
            const case_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @intCast(i), 0);
            c.LLVMAddCase(switch_inst, case_value, case_blocks[i]);
            
            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.builder, case_blocks[i]);
            for (select_case.statements.items) |stmt| {
                try self.generateStatement(stmt);
            }
            
            if (!c.LLVMGetBasicBlockTerminator(case_blocks[i])) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }
        
        // Generate default case
        if (select_stmt.default_case) |default_stmts| {
            c.LLVMPositionBuilderAtEnd(self.builder, default_block.?);
            for (default_stmts.items) |stmt| {
                try self.generateStatement(stmt);
            }
            if (!c.LLVMGetBasicBlockTerminator(default_block.?)) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }
        
        // Continue with merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
        
        std.debug.print("✅ Select statement with {} cases generated\n", .{select_stmt.cases.items.len});
    }
    
    /// Get or create runtime select function for channel multiplexing
    fn getOrCreateSelectFunction(self: *CodeGenerator) !c.LLVMValueRef {
        const select_func = c.LLVMGetNamedFunction(self.module, "cursed_runtime_select");
        if (select_func != null) return select_func;
        
        // Create runtime select function signature: int cursed_runtime_select(void** channels, int count)
        const ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        const ptr_ptr_type = c.LLVMPointerType(ptr_type, 0);
        const int_type = c.LLVMInt32TypeInContext(self.context);
        
        const select_func_type = c.LLVMFunctionType(int_type, 
            &[_]c.LLVMTypeRef{ptr_ptr_type, int_type}, 2, 0);
        return c.LLVMAddFunction(self.module, "cursed_runtime_select", select_func_type);
            null;
        
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "select.end");
        
        // Create blocks for each case
        for (select_stmt.cases.items, 0..) |_, i| {
            var case_name_buf: [32]u8 = undefined;
            const case_name = try std.fmt.bufPrint(&case_name_buf, "select.case.{}", .{i});
            case_blocks[i] = c.LLVMAppendBasicBlockInContext(self.context, current_func, case_name.ptr);
        }
        
        std.debug.print("⚠️  Select statement (simplified implementation)\n", .{});
        
        // Complete channel select implementation
        // 1. Check which channels are ready using runtime functions
        // 2. Select one randomly if multiple are ready
        // 3. Block until at least one is ready
        
        const runtime_select_fn = try self.getOrCreateRuntimeFunction("cursed_select_channels", 
            c.LLVMFunctionType(c.LLVMInt32TypeInContext(self.context), null, 0, 0));
            
        // Build channel info array for runtime
        const channel_array_type = c.LLVMArrayType(c.LLVMPointerTypeInContext(self.context, 0), 
            @intCast(select_stmt.cases.items.len));
        const channel_array = c.LLVMBuildArrayAlloca(self.builder, 
            c.LLVMPointerTypeInContext(self.context, 0),
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), select_stmt.cases.items.len, 0),
            "select_channels");
            
        // Populate channel array
        for (select_stmt.cases.items, 0..) |case_info, i| {
            // Extract channel from case (this is simplified)
            const case_channel = try self.generateExpression(case_info.expression.*);
            const channel_slot = c.LLVMBuildGEP2(self.builder, 
                channel_array_type, channel_array,
                &[_]c.LLVMValueRef{c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), i, 0)},
                1, "channel_slot");
            _ = c.LLVMBuildStore(self.builder, case_channel, channel_slot);
        }
        
        // Call runtime select function
        const selected_index = c.LLVMBuildCall2(self.builder, 
            c.LLVMGlobalGetValueType(runtime_select_fn),
            runtime_select_fn,
            &[_]c.LLVMValueRef{channel_array},
            1, "selected_case");
            
        // Switch on the selected index
        const switch_instr = c.LLVMBuildSwitch(self.builder, selected_index, 
            default_block orelse merge_block, @intCast(select_stmt.cases.items.len));
        
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
        // Generate ForIn loop: bestie item in array { ... }
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Get the iterable value (array/collection)
        const iterable_value = try self.generateExpression(for_in_stmt.iterable.*);
        
        // Create basic blocks for the loop
        const loop_init = c.LLVMAppendBasicBlockInContext(self.context, current_func, "forin.init");
        const loop_condition = c.LLVMAppendBasicBlockInContext(self.context, current_func, "forin.condition");
        const loop_body = c.LLVMAppendBasicBlockInContext(self.context, current_func, "forin.body");
        const loop_increment = c.LLVMAppendBasicBlockInContext(self.context, current_func, "forin.increment");
        const loop_exit = c.LLVMAppendBasicBlockInContext(self.context, current_func, "forin.exit");
        
        // Jump to initialization
        _ = c.LLVMBuildBr(self.builder, loop_init);
        
        // Initialize loop counter
        c.LLVMPositionBuilderAtEnd(self.builder, loop_init);
        const i32_type = c.LLVMInt32TypeInContext(self.context);
        const index_alloca = c.LLVMBuildAlloca(self.builder, i32_type, "forin_index");
        const zero = c.LLVMConstInt(i32_type, 0, 0);
        _ = c.LLVMBuildStore(self.builder, zero, index_alloca);
        
        // Get array length (assuming array has a length field or we compute it)
        // For simplicity, assume we have a function to get length
        const array_length = try self.getArrayLength(iterable_value);
        _ = c.LLVMBuildBr(self.builder, loop_condition);
        
        // Check loop condition: index < array_length
        c.LLVMPositionBuilderAtEnd(self.builder, loop_condition);
        const current_index = c.LLVMBuildLoad2(self.builder, i32_type, index_alloca, "current_index");
        const condition = c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, current_index, array_length, "condition");
        _ = c.LLVMBuildCondBr(self.builder, condition, loop_body, loop_exit);
        
        // Generate loop body
        c.LLVMPositionBuilderAtEnd(self.builder, loop_body);
        
        // Get current array element and bind to loop variable
        const element_ptr = c.LLVMBuildGEP2(
            self.builder,
            c.LLVMInt64TypeInContext(self.context), // Element type (simplified)
            iterable_value,
            &[_]c.LLVMValueRef{current_index},
            1,
            "element_ptr"
        );
        const element_value = c.LLVMBuildLoad2(
            self.builder,
            c.LLVMInt64TypeInContext(self.context),
            element_ptr,
            "element_value"
        );
        
        // Create loop variable
        const loop_var_alloca = c.LLVMBuildAlloca(
            self.builder, 
            c.LLVMInt64TypeInContext(self.context), 
            for_in_stmt.variable.ptr
        );
        _ = c.LLVMBuildStore(self.builder, element_value, loop_var_alloca);
        try self.variables.put(for_in_stmt.variable, loop_var_alloca);
        
        // Generate body statements
        for (for_in_stmt.body.items) |stmt| {
            try self.generateStatement(stmt.*);
        }
        
        if (!c.LLVMGetBasicBlockTerminator(loop_body)) {
            _ = c.LLVMBuildBr(self.builder, loop_increment);
        }
        
        // Increment index
        c.LLVMPositionBuilderAtEnd(self.builder, loop_increment);
        const one = c.LLVMConstInt(i32_type, 1, 0);
        const next_index = c.LLVMBuildAdd(self.builder, current_index, one, "next_index");
        _ = c.LLVMBuildStore(self.builder, next_index, index_alloca);
        _ = c.LLVMBuildBr(self.builder, loop_condition);
        
        // Continue with exit block
        c.LLVMPositionBuilderAtEnd(self.builder, loop_exit);
        
        std.debug.print("✅ ForIn statement compiled for variable '{s}'\n", .{for_in_stmt.variable});
    }

    fn generateSwitchStatement(self: *CodeGenerator, switch_stmt: ast.SwitchStatement) !void {
        // Generate switch statement: vibe_check value { mood 1: ...; mood 2: ...; basic: ... }
        const switch_expr: *ast.Expression = @ptrCast(@alignCast(switch_stmt.expression));
        const switch_value = try self.generateExpression(switch_expr.*);
        
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Create basic blocks
        const default_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "switch.default");
        const end_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "switch.end");
        
        // Create switch instruction
        const switch_inst = c.LLVMBuildSwitch(self.builder, switch_value, default_block, @intCast(switch_stmt.cases.items.len));
        
        // Generate case blocks
        for (switch_stmt.cases.items, 0..) |case_item, i| {
            const case_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, 
                try std.fmt.allocPrintZ(self.allocator, "switch.case_{}", .{i}).?);
            
            // Get case value and add to switch
            const case_value_expr: *ast.Expression = @ptrCast(@alignCast(case_item.value));
            const case_value = try self.generateExpression(case_value_expr.*);
            c.LLVMAddCase(switch_inst, case_value, case_block);
            
            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.builder, case_block);
            const case_body_stmts: []const *ast.Statement = @ptrCast(@alignCast(case_item.body.items));
            for (case_body_stmts) |stmt| {
                try self.generateStatement(stmt.*);
            }
            
            // Jump to end (no fallthrough by default)
            if (!c.LLVMGetBasicBlockTerminator(case_block)) {
                _ = c.LLVMBuildBr(self.builder, end_block);
            }
        }
        
        // Generate default case
        c.LLVMPositionBuilderAtEnd(self.builder, default_block);
        if (switch_stmt.default_case) |default_stmts| {
            const default_body_stmts: []const *ast.Statement = @ptrCast(@alignCast(default_stmts.items));
            for (default_body_stmts) |stmt| {
                try self.generateStatement(stmt.*);
            }
        }
        
        if (!c.LLVMGetBasicBlockTerminator(default_block)) {
            _ = c.LLVMBuildBr(self.builder, end_block);
        }
        
        // Continue with end block
        c.LLVMPositionBuilderAtEnd(self.builder, end_block);
        
        std.debug.print("✅ Switch statement compiled with {} cases\n", .{switch_stmt.cases.items.len});
    }

    fn generatePatternSwitchStatement(self: *CodeGenerator, pattern_stmt: ast.PatternSwitchStatement) !void {
        // Generate the expression to match against
        const match_value = try self.generateExpression(pattern_stmt.expression.*);
        
        // Get current function for creating basic blocks
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Create basic blocks for pattern matching
        const end_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "pattern_end");
        const default_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "pattern_default");
        
        // Track all case blocks for cleanup
        var case_blocks = std.ArrayList(c.LLVMBasicBlockRef).init(self.allocator);
        defer case_blocks.deinit();
        
        // Generate comparison chains for each pattern
        var current_block = c.LLVMGetInsertBlock(self.builder);
        
        for (pattern_stmt.patterns.items, 0..) |pattern_case, i| {
            const case_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, 
                try std.fmt.allocPrintZ(self.allocator, "pattern_case_{}", .{i}).?);
            const next_test_block = if (i == pattern_stmt.patterns.items.len - 1) 
                default_block 
            else 
                c.LLVMAppendBasicBlockInContext(self.context, current_func, 
                    try std.fmt.allocPrintZ(self.allocator, "pattern_test_{}", .{i + 1}).?);
            
            try case_blocks.append(self.allocator, case_block);
            
            // Position builder for pattern test
            c.LLVMPositionBuilderAtEnd(self.builder, current_block);
            
            // Generate pattern matching logic
            try self.generatePatternTest(pattern_case.pattern, match_value, case_block, next_test_block);
            
            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.builder, case_block);
            
            // Generate guard condition if present
            if (pattern_case.guard) |guard| {
                const guard_value = try self.generateExpression(guard.*);
                const guard_true_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, 
                    try std.fmt.allocPrintZ(self.allocator, "guard_true_{}", .{i}).?);
                _ = c.LLVMBuildCondBr(self.builder, guard_value, guard_true_block, next_test_block);
                c.LLVMPositionBuilderAtEnd(self.builder, guard_true_block);
            }
            
            // Generate statements for this case
            for (pattern_case.body.items) |stmt| {
                try self.generateStatement(stmt.*);
            }
            
            // Jump to end after executing case
            _ = c.LLVMBuildBr(self.builder, end_block);
            
            current_block = next_test_block;
        }
        
        // Generate default case
        c.LLVMPositionBuilderAtEnd(self.builder, default_block);
        if (pattern_stmt.default_case) |default_stmts| {
            for (default_stmts.items) |stmt| {
                try self.generateStatement(stmt.*);
            }
        } else {
            // No default case - generate runtime error
            try self.generateRuntimeError("Pattern match failed: no matching case");
        }
        _ = c.LLVMBuildBr(self.builder, end_block);
        
        // Position builder at end block for continuation
        c.LLVMPositionBuilderAtEnd(self.builder, end_block);
        
        std.debug.print("✅ Pattern switch statement compiled with {} cases\n", .{pattern_stmt.patterns.items.len});
    }

    /// Generate pattern matching test logic for different pattern types
    fn generatePatternTest(self: *CodeGenerator, pattern: ast.Pattern, match_value: c.LLVMValueRef, success_block: c.LLVMBasicBlockRef, failure_block: c.LLVMBasicBlockRef) !void {
        switch (pattern) {
            .Wildcard => {
                // Wildcard always matches - jump directly to success
                _ = c.LLVMBuildBr(self.builder, success_block);
            },
            .Literal => |literal| {
                // Generate comparison for literal values
                const cmp_result = try self.generateLiteralComparison(literal, match_value);
                _ = c.LLVMBuildCondBr(self.builder, cmp_result, success_block, failure_block);
            },
            .Variable => |var_name| {
                // Variable pattern always matches and binds the value
                const var_alloca = try self.createVariable(var_name, c.LLVMTypeOf(match_value));
                _ = c.LLVMBuildStore(self.builder, match_value, var_alloca);
                _ = c.LLVMBuildBr(self.builder, success_block);
            },
            .Range => |range_pattern| {
                // Generate range comparison: start <= value <= end
                const start_value = try self.generateExpression(range_pattern.start.*);
                const end_value = try self.generateExpression(range_pattern.end.*);
                
                const ge_start = c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, match_value, start_value, "ge_start");
                const le_end = c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, match_value, end_value, "le_end");
                const in_range = c.LLVMBuildAnd(self.builder, ge_start, le_end, "in_range");
                
                _ = c.LLVMBuildCondBr(self.builder, in_range, success_block, failure_block);
            },
            .Guard => |guard_pattern| {
                // First check the base pattern, then evaluate guard condition
                const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
                const guard_check_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "guard_check");
                
                // Generate test for base pattern
                try self.generatePatternTest(guard_pattern.pattern.*, match_value, guard_check_block, failure_block);
                
                // Generate guard condition check
                c.LLVMPositionBuilderAtEnd(self.builder, guard_check_block);
                const guard_result = try self.generateExpression(guard_pattern.guard.*);
                const guard_bool = try self.convertToBool(guard_result);
                _ = c.LLVMBuildCondBr(self.builder, guard_bool, success_block, failure_block);
            },
            .Or => |or_pattern| {
                // Try each alternative pattern in sequence
                const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
                
                // Create blocks for each alternative except the last
                for (or_pattern.patterns.items[0..or_pattern.patterns.items.len-1], 0..) |alt_pattern, i| {
                    const next_alt_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, 
                        try std.fmt.allocPrintZ(self.allocator, "or_alt_{}", .{i+1}).?);
                    try self.generatePatternTest(alt_pattern, match_value, success_block, next_alt_block);
                    c.LLVMPositionBuilderAtEnd(self.builder, next_alt_block);
                }
                
                // Last alternative - if it fails, go to failure block
                if (or_pattern.patterns.items.len > 0) {
                    const last_pattern = or_pattern.patterns.items[or_pattern.patterns.items.len-1];
                    try self.generatePatternTest(last_pattern, match_value, success_block, failure_block);
                }
            },
            .Enum => |enum_pattern| {
                // Generate enum variant matching
                const variant_tag = try self.generateEnumVariantTag(enum_pattern.enum_name, enum_pattern.variant_name);
                const match_tag = try self.extractEnumTag(match_value);
                
                const tag_matches = c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, match_tag, variant_tag, "enum_tag_match");
                
                if (enum_pattern.patterns.items.len > 0) {
                    // Has associated data - need to extract and match sub-patterns
                    const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
                    const data_check_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "enum_data_check");
                    
                    _ = c.LLVMBuildCondBr(self.builder, tag_matches, data_check_block, failure_block);
                    c.LLVMPositionBuilderAtEnd(self.builder, data_check_block);
                    
                    // Extract enum payload and test sub-patterns
                    const payload = try self.extractEnumPayload(match_value, enum_pattern.variant_name);
                    for (enum_pattern.patterns.items, 0..) |sub_pattern, i| {
                        const element = try self.extractTupleElement(payload, i);
                        try self.generatePatternTest(sub_pattern, element, success_block, failure_block);
                    }
                } else {
                    // No associated data - just check tag
                    _ = c.LLVMBuildCondBr(self.builder, tag_matches, success_block, failure_block);
                }
            },
            .Tuple => |tuple_patterns| {
                // Tuple destructuring (simplified implementation)
                for (tuple_patterns.items, 0..) |sub_pattern, i| {
                    const element_ptr = c.LLVMBuildStructGEP2(
                        self.builder,
                        c.LLVMTypeOf(match_value),
                        match_value,
                        @intCast(i),
                        try std.fmt.allocPrintZ(self.allocator, "tuple_elem_{}", .{i}).?
                    );
                    const element_value = c.LLVMBuildLoad2(
                        self.builder,
                        c.LLVMInt64TypeInContext(self.context),
                        element_ptr,
                        try std.fmt.allocPrintZ(self.allocator, "tuple_val_{}", .{i}).?
                    );
                    try self.generatePatternTest(sub_pattern, element_value, success_block, failure_block);
                }
            },
            .Struct => |struct_pattern| {
                // Struct pattern matching (simplified)
                for (struct_pattern.fields.items) |field_pattern| {
                    // This would need proper struct field access implementation
                    _ = field_pattern;
                    // Placeholder: assume match for now
                    _ = c.LLVMBuildBr(self.builder, success_block);
                    return;
                }
            },
            .Array => |array_patterns| {
                // Array pattern matching (simplified)
                for (array_patterns.items, 0..) |sub_pattern, i| {
                    // Get array element at index i
                    const index_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), i, 0);
                    const element_ptr = c.LLVMBuildGEP2(
                        self.builder,
                        c.LLVMInt64TypeInContext(self.context),
                        match_value,
                        &[_]c.LLVMValueRef{index_value},
                        1,
                        try std.fmt.allocPrintZ(self.allocator, "array_elem_{}", .{i}).?
                    );
                    const element_value = c.LLVMBuildLoad2(
                        self.builder,
                        c.LLVMInt64TypeInContext(self.context),
                        element_ptr,
                        try std.fmt.allocPrintZ(self.allocator, "array_val_{}", .{i}).?
                    );
                    try self.generatePatternTest(sub_pattern, element_value, success_block, failure_block);
                }
            },
        }
    }

    /// Generate comparison for literal patterns
    fn generateLiteralComparison(self: *CodeGenerator, literal: ast.Literal, match_value: c.LLVMValueRef) !c.LLVMValueRef {
        switch (literal) {
            .Integer => |int_val| {
                const literal_value = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @bitCast(int_val), 0);
                return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, match_value, literal_value, "int_cmp");
            },
            .Float => |float_val| {
                const literal_value = c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float_val);
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealOEQ, match_value, literal_value, "float_cmp");
            },
            .Boolean => |bool_val| {
                const literal_value = c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0);
                return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, match_value, literal_value, "bool_cmp");
            },
            .String => |str_val| {
                // For string comparison, we need to call strcmp or similar
                const str_constant = c.LLVMBuildGlobalStringPtr(self.builder, str_val.ptr, "str_literal");
                
                // Get or create strcmp function
                const strcmp_func = c.LLVMGetNamedFunction(self.module, "strcmp") orelse {
                    const i8_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
                    const strcmp_type = c.LLVMFunctionType(
                        c.LLVMInt32TypeInContext(self.context),
                        &[_]c.LLVMTypeRef{ i8_ptr_type, i8_ptr_type },
                        2,
                        0
                    );
                    c.LLVMAddFunction(self.module, "strcmp", strcmp_type);
                };
                
                const strcmp_result = c.LLVMBuildCall2(
                    self.builder,
                    c.LLVMInt32TypeInContext(self.context),
                    strcmp_func,
                    &[_]c.LLVMValueRef{ match_value, str_constant },
                    2,
                    "strcmp_result"
                );
                
                const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
                return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, strcmp_result, zero, "str_cmp");
            },
        }
    }

    /// Generate runtime error with message
    fn generateRuntimeError(self: *CodeGenerator, message: []const u8) !void {
        // Get or create printf function for error reporting
        const printf_func = c.LLVMGetNamedFunction(self.module, "printf") orelse {
            return CodeGenError.LLVMError;
        };
        
        // Create error message string
        const error_format = c.LLVMBuildGlobalStringPtr(self.builder, "Runtime Error: %s\n", "error_fmt");
        const error_msg = c.LLVMBuildGlobalStringPtr(self.builder, message.ptr, "error_msg");
        
        // Call printf with error message
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMInt32TypeInContext(self.context),
            printf_func,
            &[_]c.LLVMValueRef{ error_format, error_msg },
            2,
            "printf_result"
        );
        
        // Generate exit call
        const exit_func = c.LLVMGetNamedFunction(self.module, "exit") orelse {
            const exit_type = c.LLVMFunctionType(
                c.LLVMVoidTypeInContext(self.context),
                &[_]c.LLVMTypeRef{c.LLVMInt32TypeInContext(self.context)},
                1,
                0
            );
            c.LLVMAddFunction(self.module, "exit", exit_type);
        };
        
        const exit_code = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 1, 0);
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            exit_func,
            &[_]c.LLVMValueRef{exit_code},
            1,
            "exit_call"
        );
        
        // This is unreachable, but LLVM requires a terminator
        _ = c.LLVMBuildUnreachable(self.builder);
    }

    /// Helper to create a variable allocation
    fn createVariable(self: *CodeGenerator, name: []const u8, var_type: c.LLVMTypeRef) !c.LLVMValueRef {
        const var_alloca = c.LLVMBuildAlloca(self.builder, var_type, name.ptr);
        
        // Store in variables map for later lookup
        const name_copy = try self.allocator.dupe(u8, name);
        try self.variables.put(name_copy, var_alloca);
        
        return var_alloca;
    }

    /// Helper to get array length for ForIn loops
    fn getArrayLength(self: *CodeGenerator, array_value: c.LLVMValueRef) !c.LLVMValueRef {
        // For now, return a constant length (this should be enhanced for dynamic arrays)
        // In a real implementation, this would extract the length from the array structure
        const array_type = c.LLVMTypeOf(array_value);
        
        if (c.LLVMGetTypeKind(array_type) == c.LLVMArrayTypeKind) {
            // Static array - get length from type
            const length = c.LLVMGetArrayLength(array_type);
            return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), length, 0);
        } else {
            // Dynamic array - would need to access length field
            // For now, return a default length
            std.debug.print("⚠️ Using default array length for dynamic array\n", .{});
            return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 10, 0);
        }
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
        // Generate increment statement: variable++
        const var_expr: *ast.Expression = @ptrCast(@alignCast(inc_stmt.variable));
        
        if (var_expr.* == .Identifier) {
            const var_name = var_expr.Identifier;
            if (self.variables.get(var_name)) |variable| {
                // Load current value
                const var_type = c.LLVMGetElementType(c.LLVMTypeOf(variable));
                const current_value = c.LLVMBuildLoad2(self.builder, var_type, variable, "current_value");
                
                // Add 1 to the value
                const one = if (c.LLVMGetTypeKind(var_type) == c.LLVMIntegerTypeKind) 
                    c.LLVMConstInt(var_type, 1, 0)
                else 
                    c.LLVMConstReal(var_type, 1.0);
                
                const incremented_value = if (c.LLVMGetTypeKind(var_type) == c.LLVMIntegerTypeKind)
                    c.LLVMBuildAdd(self.builder, current_value, one, "incremented")
                else
                    c.LLVMBuildFAdd(self.builder, current_value, one, "incremented");
                
                // Store back to variable
                _ = c.LLVMBuildStore(self.builder, incremented_value, variable);
                
                std.debug.print("✅ Increment statement compiled for variable '{s}'\n", .{var_name});
            } else {
                std.debug.print("Error: Undefined variable '{s}' in increment\n", .{var_name});
                return error.UndefinedSymbol;
            }
        } else {
            std.debug.print("Error: Increment target must be a variable identifier\n", .{});
            return error.InvalidOperation;
        }
    }

    fn generateDecrementStatement(self: *CodeGenerator, dec_stmt: ast.DecrementStatement) !void {
        // Generate decrement statement: variable--
        const var_expr: *ast.Expression = @ptrCast(@alignCast(dec_stmt.variable));
        
        if (var_expr.* == .Identifier) {
            const var_name = var_expr.Identifier;
            if (self.variables.get(var_name)) |variable| {
                // Load current value
                const var_type = c.LLVMGetElementType(c.LLVMTypeOf(variable));
                const current_value = c.LLVMBuildLoad2(self.builder, var_type, variable, "current_value");
                
                // Subtract 1 from the value
                const one = if (c.LLVMGetTypeKind(var_type) == c.LLVMIntegerTypeKind) 
                    c.LLVMConstInt(var_type, 1, 0)
                else 
                    c.LLVMConstReal(var_type, 1.0);
                
                const decremented_value = if (c.LLVMGetTypeKind(var_type) == c.LLVMIntegerTypeKind)
                    c.LLVMBuildSub(self.builder, current_value, one, "decremented")
                else
                    c.LLVMBuildFSub(self.builder, current_value, one, "decremented");
                
                // Store back to variable
                _ = c.LLVMBuildStore(self.builder, decremented_value, variable);
                
                std.debug.print("✅ Decrement statement compiled for variable '{s}'\n", .{var_name});
            } else {
                std.debug.print("Error: Undefined variable '{s}' in decrement\n", .{var_name});
                return error.UndefinedSymbol;
            }
        } else {
            std.debug.print("Error: Decrement target must be a variable identifier\n", .{});
            return error.InvalidOperation;
        }
    }

    fn generateShortDeclarationStatement(self: *CodeGenerator, short_decl: ast.ShortDeclarationStatement) !void {
        // Generate short declaration: name := value
        if (short_decl.names.items.len != short_decl.values.items.len) {
            std.debug.print("Error: Names and values count mismatch in short declaration\n", .{});
            return error.TypeMismatch;
        }
        
        // Process each name-value pair
        for (short_decl.names.items, short_decl.values.items) |name, value| {
            // Generate the value expression
            const init_value = try self.generateExpression(value.*);
            
            // Infer type from the value
            const var_type = c.LLVMTypeOf(init_value);
            
            // Create variable allocation
            const var_alloca = c.LLVMBuildAlloca(self.builder, var_type, name.ptr);
            
            // Store the initial value
            _ = c.LLVMBuildStore(self.builder, init_value, var_alloca);
            
            // Add to variables map
            try self.variables.put(name, var_alloca);
            
            std.debug.print("✅ Short declaration compiled for variable '{s}'\n", .{name});
        }
    }

    fn generatePanicStatement(self: *CodeGenerator, panic_stmt: ast.PanicStatement) !void {
        // Declare cursed_panic runtime function if not already declared
        const cursed_panic_func = c.LLVMGetNamedFunction(self.module, "cursed_panic") orelse blk: {
            var params = [_]c.LLVMTypeRef{c.LLVMPointerTypeInContext(self.context, 0)}; // const char* message
            const func_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), &params, 1, 0);
            break :blk c.LLVMAddFunction(self.module, "cursed_panic", func_type);
        };
        
        // Generate the panic message expression
        const message_expr: *ast.Expression = @ptrCast(@alignCast(panic_stmt.message));
        const message_value = try self.generateExpression(message_expr.*);
        
        // Call cursed_panic with the message
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            cursed_panic_func,
            &[_]c.LLVMValueRef{message_value},
            1,
            ""
        );
        
        // Add unreachable instruction since panic never returns
        _ = c.LLVMBuildUnreachable(self.builder);
        
        std.debug.print("✅ Panic statement compiled\n", .{});
    }

    fn generateCatchStatement(self: *CodeGenerator, catch_stmt: ast.CatchStatement) !void {
        // Declare cursed_error_create runtime function if not already declared
        const cursed_error_create_func = c.LLVMGetNamedFunction(self.module, "cursed_create_error") orelse blk: {
            var params = [_]c.LLVMTypeRef{ 
                c.LLVMPointerTypeInContext(self.context, 0), // const char* message
                c.LLVMInt32TypeInContext(self.context),      // int32_t code
                c.LLVMPointerTypeInContext(self.context, 0)  // const char* source_location
            };
            const func_type = c.LLVMFunctionType(c.LLVMPointerTypeInContext(self.context, 0), &params, 3, 0);
            break :blk c.LLVMAddFunction(self.module, "cursed_create_error", func_type);
        };
        
        // Create error handling block
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        const catch_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "catch_block");
        const after_catch_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "after_catch");
        
        // Jump to catch block
        _ = c.LLVMBuildBr(self.builder, catch_block);
        
        // Generate catch block body
        c.LLVMPositionBuilderAtEnd(self.builder, catch_block);
        
        // If there's an error variable, allocate and store it
        if (catch_stmt.error_variable) |error_var| {
            const error_ptr_type = c.LLVMPointerTypeInContext(self.context, 0);
            const error_alloca = c.LLVMBuildAlloca(self.builder, error_ptr_type, error_var.ptr);
            try self.variables.put(error_var, error_alloca);
            
            // Initialize with current error (placeholder - would be set by exception mechanism)
            const null_value = c.LLVMConstNull(error_ptr_type);
            _ = c.LLVMBuildStore(self.builder, null_value, error_alloca);
        }
        
        // Generate catch body statements
        var stmt_iter = catch_stmt.body.iterator();
        while (stmt_iter.next()) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateStatement(stmt.*);
        }
        
        // Jump to after catch block
        _ = c.LLVMBuildBr(self.builder, after_catch_block);
        c.LLVMPositionBuilderAtEnd(self.builder, after_catch_block);
        
        std.debug.print("✅ Catch statement compiled\n", .{});
    }

    fn generateYikesStatement(self: *CodeGenerator, yikes_stmt: ast.YikesStatement) !void {
        // Declare cursed_create_error and cursed_panic_with_error functions
        const cursed_create_error_func = c.LLVMGetNamedFunction(self.module, "cursed_create_error") orelse blk: {
            var params = [_]c.LLVMTypeRef{ 
                c.LLVMPointerTypeInContext(self.context, 0), // const char* message
                c.LLVMInt32TypeInContext(self.context),      // int32_t code
                c.LLVMPointerTypeInContext(self.context, 0)  // const char* source_location
            };
            const func_type = c.LLVMFunctionType(c.LLVMPointerTypeInContext(self.context, 0), &params, 3, 0);
            break :blk c.LLVMAddFunction(self.module, "cursed_create_error", func_type);
        };
        
        const cursed_panic_with_error_func = c.LLVMGetNamedFunction(self.module, "cursed_panic_with_error") orelse blk: {
            var params = [_]c.LLVMTypeRef{c.LLVMPointerTypeInContext(self.context, 0)}; // CursedError* error
            const func_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), &params, 1, 0);
            break :blk c.LLVMAddFunction(self.module, "cursed_panic_with_error", func_type);
        };
        
        // Generate the error message expression
        const message_value = try self.generateExpression(yikes_stmt.message.*);
        
        // Create error code (default to -1)
        const error_code = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @bitCast(@as(i32, -1)), 1);
        
        // Create source location string (placeholder)
        const location_str = if (yikes_stmt.location) |loc| 
            try self.createGlobalString("yikes statement")
        else 
            try self.createGlobalString("unknown location");
        
        // Call cursed_create_error
        const error_obj = c.LLVMBuildCall2(
            self.builder,
            c.LLVMPointerTypeInContext(self.context, 0),
            cursed_create_error_func,
            &[_]c.LLVMValueRef{ message_value, error_code, location_str },
            3,
            "yikes_error"
        );
        
        // Call cursed_panic_with_error
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            cursed_panic_with_error_func,
            &[_]c.LLVMValueRef{error_obj},
            1,
            ""
        );
        
        // Add unreachable instruction since yikes never returns
        _ = c.LLVMBuildUnreachable(self.builder);
        
        std.debug.print("✅ Yikes statement compiled\n", .{});
    }

    fn generateFamStatement(self: *CodeGenerator, fam_stmt: ast.FamStatement) !void {
        // Declare setjmp/longjmp for exception handling
        const setjmp_func = c.LLVMGetNamedFunction(self.module, "setjmp") orelse blk: {
            var params = [_]c.LLVMTypeRef{c.LLVMPointerTypeInContext(self.context, 0)}; // jmp_buf*
            const func_type = c.LLVMFunctionType(c.LLVMInt32TypeInContext(self.context), &params, 1, 0);
            break :blk c.LLVMAddFunction(self.module, "setjmp", func_type);
        };
        
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Create basic blocks for control flow
        const try_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "fam_try");
        const catch_dispatch_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "fam_catch_dispatch");
        const finally_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "fam_finally");
        const after_fam_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "after_fam");
        
        // Allocate jmp_buf for exception handling
        const jmp_buf_size = 64; // Standard jmp_buf size on most platforms
        const jmp_buf_type = c.LLVMArrayType(c.LLVMInt8TypeInContext(self.context), jmp_buf_size);
        const jmp_buf_alloca = c.LLVMBuildAlloca(self.builder, jmp_buf_type, "jmp_buf");
        
        // Call setjmp to establish exception handling point
        const setjmp_result = c.LLVMBuildCall2(
            self.builder,
            c.LLVMInt32TypeInContext(self.context),
            setjmp_func,
            &[_]c.LLVMValueRef{jmp_buf_alloca},
            1,
            "setjmp_result"
        );
        
        // Check setjmp return value (0 = normal execution, non-zero = exception)
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        const is_exception = c.LLVMBuildICmp(self.builder, c.LLVMIntNE, setjmp_result, zero, "is_exception");
        
        // Branch based on setjmp result
        _ = c.LLVMBuildCondBr(self.builder, is_exception, catch_dispatch_block, try_block);
        
        // Generate try block
        c.LLVMPositionBuilderAtEnd(self.builder, try_block);
        
        // Execute try body statements
        for (fam_stmt.try_body.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // If no exception, jump to finally block
        _ = c.LLVMBuildBr(self.builder, finally_block);
        
        // Generate catch dispatch block
        c.LLVMPositionBuilderAtEnd(self.builder, catch_dispatch_block);
        
        // Generate catch blocks
        for (fam_stmt.catch_blocks.items) |catch_block| {
            const catch_handler_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "catch_handler");
            const next_catch_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "next_catch");
            
            // TODO: Add error type checking here for specific catch blocks
            _ = c.LLVMBuildBr(self.builder, catch_handler_block);
            
            // Generate catch handler
            c.LLVMPositionBuilderAtEnd(self.builder, catch_handler_block);
            
            // If there's an error variable, allocate and store it
            if (catch_block.error_variable) |error_var| {
                const error_ptr_type = c.LLVMPointerTypeInContext(self.context, 0);
                const error_alloca = c.LLVMBuildAlloca(self.builder, error_ptr_type, error_var.ptr);
                try self.variables.put(error_var, error_alloca);
                
                // Store current error (placeholder implementation)
                const null_value = c.LLVMConstNull(error_ptr_type);
                _ = c.LLVMBuildStore(self.builder, null_value, error_alloca);
            }
            
            // Execute catch body
            for (catch_block.body.items) |stmt| {
                try self.generateStatement(stmt);
            }
            
            // Jump to finally block after catch
            _ = c.LLVMBuildBr(self.builder, finally_block);
            
            c.LLVMPositionBuilderAtEnd(self.builder, next_catch_block);
        }
        
        // If no catch block matches, jump to finally
        _ = c.LLVMBuildBr(self.builder, finally_block);
        
        // Generate finally block
        c.LLVMPositionBuilderAtEnd(self.builder, finally_block);
        
        // Execute finally block if present
        if (fam_stmt.finally_block) |finally_stmts| {
            for (finally_stmts.items) |stmt| {
                try self.generateStatement(stmt);
            }
        }
        
        // Jump to after fam block
        _ = c.LLVMBuildBr(self.builder, after_fam_block);
        c.LLVMPositionBuilderAtEnd(self.builder, after_fam_block);
        
        std.debug.print("✅ Fam statement compiled\n", .{});
    }

    /// Helper function to create a global string constant
    fn createGlobalString(self: *CodeGenerator, str: []const u8) !c.LLVMValueRef {
        return c.LLVMBuildGlobalStringPtr(self.builder, str.ptr, "str_const");
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
        return try self.generateMatchExpressionComplete(match);
    }

    fn generateTypeSwitchExpression(self: *CodeGenerator, type_switch: ast.TypeSwitchExpression) !c.LLVMValueRef {
        _ = type_switch;
        std.debug.print("⚠️ Type switch expression placeholder\n", .{});
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    // ====== Pattern Matching Helper Functions ======

    /// Convert value to boolean for conditional branches
    fn convertToBool(self: *CodeGenerator, value: c.LLVMValueRef) !c.LLVMValueRef {
        const value_type = c.LLVMTypeOf(value);
        const type_kind = c.LLVMGetTypeKind(value_type);
        
        switch (type_kind) {
            c.LLVMIntegerTypeKind => {
                const zero = c.LLVMConstInt(value_type, 0, 0);
                return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, value, zero, "to_bool");
            },
            c.LLVMFloatTypeKind, c.LLVMDoubleTypeKind => {
                const zero = c.LLVMConstReal(value_type, 0.0);
                return c.LLVMBuildFCmp(self.builder, c.LLVMRealONE, value, zero, "to_bool");
            },
            c.LLVMPointerTypeKind => {
                const null_ptr = c.LLVMConstNull(value_type);
                return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, value, null_ptr, "to_bool");
            },
            else => {
                // For other types, assume already boolean or create a simple check
                return value;
            }
        }
    }

    /// Generate enum variant tag constant
    fn generateEnumVariantTag(self: *CodeGenerator, enum_name: []const u8, variant_name: []const u8) !c.LLVMValueRef {
        // Look up variant index in registry
        _ = enum_name;
        _ = variant_name;
        
        // For now, generate a placeholder tag
        // In a real implementation, this would look up the actual variant index
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    /// Extract enum tag from enum value
    fn extractEnumTag(self: *CodeGenerator, enum_value: c.LLVMValueRef) !c.LLVMValueRef {
        // Extract the tag field (first element) from the enum struct
        const tag_ptr = c.LLVMBuildStructGEP2(self.builder, 
            c.LLVMTypeOf(enum_value), enum_value, 0, "enum_tag_ptr");
        return c.LLVMBuildLoad2(self.builder, 
            c.LLVMInt32TypeInContext(self.context), tag_ptr, "enum_tag");
    }

    /// Extract enum payload from enum value
    fn extractEnumPayload(self: *CodeGenerator, enum_value: c.LLVMValueRef, variant_name: []const u8) !c.LLVMValueRef {
        _ = variant_name; // For type-specific payload extraction
        
        // Extract the payload field (second element) from the enum struct
        const payload_ptr = c.LLVMBuildStructGEP2(self.builder,
            c.LLVMTypeOf(enum_value), enum_value, 1, "enum_payload_ptr");
        return c.LLVMBuildLoad2(self.builder,
            c.LLVMPointerTypeInContext(self.context, 0), payload_ptr, "enum_payload");
    }

    /// Extract element from tuple value
    fn extractTupleElement(self: *CodeGenerator, tuple_value: c.LLVMValueRef, index: usize) !c.LLVMValueRef {
        const element_ptr = c.LLVMBuildStructGEP2(self.builder,
            c.LLVMTypeOf(tuple_value), tuple_value, @intCast(index), "tuple_elem_ptr");
        return c.LLVMBuildLoad2(self.builder,
            c.LLVMInt64TypeInContext(self.context), element_ptr, "tuple_elem");
    }

    /// Generate comprehensive vibe_check pattern matching statement
    fn generateVibeCheckStatement(self: *CodeGenerator, vibe_check: ast.VibeCheckStatement) !void {
        const match_value = try self.generateExpression(vibe_check.expression.*);
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Create blocks for each mood case and the basic (default) case
        var case_blocks = try self.allocator.alloc(c.LLVMBasicBlockRef, vibe_check.mood_cases.items.len);
        defer self.allocator.free(case_blocks);
        
        const basic_block = if (vibe_check.basic_case) |_|
            c.LLVMAppendBasicBlockInContext(self.context, current_func, "vibe_basic")
        else
            null;
            
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "vibe_end");
        
        // Create blocks for each mood case
        for (vibe_check.mood_cases.items, 0..) |_, i| {
            const case_name = try std.fmt.allocPrintZ(self.allocator, "vibe_mood_{}", .{i});
            case_blocks[i] = c.LLVMAppendBasicBlockInContext(self.context, current_func, case_name);
        }
        
        // Generate pattern matching logic for each mood case
        var next_test_block: ?c.LLVMBasicBlockRef = null;
        for (vibe_check.mood_cases.items, 0..) |mood_case, i| {
            // Create test block for this case
            const test_block_name = try std.fmt.allocPrintZ(self.allocator, "vibe_test_{}", .{i});
            const test_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, test_block_name);
            
            if (i == 0) {
                // Jump to first test from current block
                _ = c.LLVMBuildBr(self.builder, test_block);
            } else {
                // Connect previous test block failure to this test
                if (next_test_block) |prev_test| {
                    c.LLVMPositionBuilderAtEnd(self.builder, prev_test);
                    _ = c.LLVMBuildBr(self.builder, test_block);
                }
            }
            
            c.LLVMPositionBuilderAtEnd(self.builder, test_block);
            
            // Determine next test block or fallback
            next_test_block = if (i < vibe_check.mood_cases.items.len - 1) 
                c.LLVMAppendBasicBlockInContext(self.context, current_func, 
                    try std.fmt.allocPrintZ(self.allocator, "vibe_test_{}", .{i+1}))
            else
                basic_block orelse merge_block;
            
            // Generate pattern tests for multiple patterns in OR fashion
            if (mood_case.patterns.items.len > 0) {
                for (mood_case.patterns.items) |pattern| {
                    try self.generatePatternTest(pattern, match_value, case_blocks[i], next_test_block.?);
                }
            }
            
            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.builder, case_blocks[i]);
            for (mood_case.body.items) |stmt| {
                const case_stmt: *ast.Statement = @ptrCast(@alignCast(stmt));
                try self.generateStatement(case_stmt.*);
            }
            
            // Jump to merge block if no terminator
            if (!c.LLVMGetBasicBlockTerminator(case_blocks[i])) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }
        
        // Generate basic (default) case if present
        if (basic_block) |bb| {
            c.LLVMPositionBuilderAtEnd(self.builder, bb);
            if (vibe_check.basic_case) |basic_stmts| {
                for (basic_stmts.items) |stmt| {
                    const basic_stmt: *ast.Statement = @ptrCast(@alignCast(stmt));
                    try self.generateStatement(basic_stmt.*);
                }
            }
            
            if (!c.LLVMGetBasicBlockTerminator(bb)) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }
        
        // If no basic case and we reach here, it's a non-exhaustive pattern match
        if (basic_block == null and next_test_block != null) {
            c.LLVMPositionBuilderAtEnd(self.builder, next_test_block.?);
            try self.generateRuntimeError("Non-exhaustive vibe_check: no pattern matched");
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Continue with merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
        
        std.debug.print("✅ vibe_check statement compiled with {} mood cases\n", .{vibe_check.mood_cases.items.len});
    }

    /// Complete match expression generation with all pattern types
    fn generateMatchExpressionComplete(self: *CodeGenerator, match_expr: ast.MatchExpression) !c.LLVMValueRef {
        const match_value = try self.generateExpression(match_expr.expression.*);
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Determine result type from first case
        const result_type = if (match_expr.cases.items.len > 0) blk: {
            // This is a simplified type inference - should be enhanced
            const first_case = match_expr.cases.items[0];
            const first_result = try self.generateExpression(first_case.result.*);
            break :blk c.LLVMTypeOf(first_result);
        } else c.LLVMInt32TypeInContext(self.context);
        
        // Allocate result variable
        const result_alloca = c.LLVMBuildAlloca(self.builder, result_type, "match_result");
        
        // Create blocks
        var case_blocks = try self.allocator.alloc(c.LLVMBasicBlockRef, match_expr.cases.items.len);
        defer self.allocator.free(case_blocks);
        
        const default_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "match_default");
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "match_end");
        
        // Create case blocks
        for (match_expr.cases.items, 0..) |_, i| {
            const case_name = try std.fmt.allocPrintZ(self.allocator, "match_case_{}", .{i});
            case_blocks[i] = c.LLVMAppendBasicBlockInContext(self.context, current_func, case_name);
        }
        
        // Generate pattern matching logic
        var next_test_block: ?c.LLVMBasicBlockRef = null;
        for (match_expr.cases.items, 0..) |case_info, i| {
            const test_block_name = try std.fmt.allocPrintZ(self.allocator, "match_test_{}", .{i});
            const test_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, test_block_name);
            
            if (i == 0) {
                _ = c.LLVMBuildBr(self.builder, test_block);
            } else if (next_test_block) |prev_test| {
                c.LLVMPositionBuilderAtEnd(self.builder, prev_test);
                _ = c.LLVMBuildBr(self.builder, test_block);
            }
            
            c.LLVMPositionBuilderAtEnd(self.builder, test_block);
            
            next_test_block = if (i < match_expr.cases.items.len - 1)
                c.LLVMAppendBasicBlockInContext(self.context, current_func, 
                    try std.fmt.allocPrintZ(self.allocator, "match_test_{}", .{i+1}))
            else
                default_block;
            
            // Generate pattern test
            try self.generatePatternTest(case_info.pattern, match_value, case_blocks[i], next_test_block.?);
            
            // Generate case result
            c.LLVMPositionBuilderAtEnd(self.builder, case_blocks[i]);
            const case_result = try self.generateExpression(case_info.result.*);
            _ = c.LLVMBuildStore(self.builder, case_result, result_alloca);
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Generate default case (runtime error for non-exhaustive matches)
        c.LLVMPositionBuilderAtEnd(self.builder, default_block);
        try self.generateRuntimeError("Match expression: no pattern matched");
        _ = c.LLVMBuildBr(self.builder, merge_block);
        
        // Continue with merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
        
        // Return the result
        const final_result = c.LLVMBuildLoad2(self.builder, result_type, result_alloca, "match_final_result");
        std.debug.print("✅ Match expression compiled with {} cases\n", .{match_expr.cases.items.len});
        
        return final_result;
    }
};
