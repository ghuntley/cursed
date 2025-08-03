const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
});

const ast = @import("ast_simple.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const FunctionStatement = ast.FunctionStatement;

pub const CodeGenError = error{
    LLVMError,
    OutOfMemory,
    InvalidType,
    UndefinedSymbol,
    TypeMismatch,
};

pub const InterfaceInfo = struct {
    name: []const u8,
    methods: ArrayList(InterfaceMethod),
    vtable_type: ?c.LLVMTypeRef,
};

pub const InterfaceMethod = struct {
    name: []const u8,
    index: usize,
    function_type: c.LLVMTypeRef,
};

pub const CodeGen = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    
    // Symbol tables
    functions: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    struct_types: std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    interface_types: std.HashMap([]const u8, InterfaceInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Current function context
    current_function: ?c.LLVMValueRef,

    pub fn init(allocator: Allocator) CodeGen {
        _ = c.LLVMInitializeNativeTarget();
        _ = c.LLVMInitializeNativeAsmPrinter();
        _ = c.LLVMInitializeNativeAsmParser();
        
        const context = c.LLVMContextCreate();
        const module = c.LLVMModuleCreateWithNameInContext("cursed_module", context);
        const builder = c.LLVMCreateBuilderInContext(context);
        
        return CodeGen{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .functions = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .struct_types = std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .interface_types = std.HashMap([]const u8, InterfaceInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
        };
    }

    pub fn deinit(self: *CodeGen) void {
        self.functions.deinit();
        self.variables.deinit();
        self.struct_types.deinit();
        
        // Clean up interface types
        var interface_iter = self.interface_types.iterator();
        while (interface_iter.next()) |entry| {
            entry.value_ptr.methods.deinit();
        }
        self.interface_types.deinit();
        
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
    }

    pub fn generateProgram(self: *CodeGen, program: Program) CodeGenError!void {
        // Generate external declarations
        try self.generateExternalDeclarations();
        
        // Generate statements
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Add main function if it doesn't exist
        if (self.functions.get("main_character") == null) {
            try self.generateMainWrapper();
        }
        
        // Verify module
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module, c.LLVMPrintMessageAction, &error_msg) != 0) {
            std.debug.print("LLVM module verification failed: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
        
        // Run optimization passes
        try self.optimizeModule();
    }

    fn generateExternalDeclarations(self: *CodeGen) CodeGenError!void {
        // Declare printf for vibez.spill
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return type
            @as([*c]c.LLVMTypeRef, @ptrCast(@constCast(&[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}))), // char* parameter
            1, // parameter count
            1  // is variadic
        );
        const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        try self.functions.put("printf", printf_func);
        
        // Declare malloc and free for memory management
        const malloc_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // return void*
            @as([*c]c.LLVMTypeRef, @ptrCast(@constCast(&[_]c.LLVMTypeRef{c.LLVMInt64TypeInContext(self.context)}))), // size_t parameter
            1, // parameter count
            0  // not variadic
        );
        const malloc_func = c.LLVMAddFunction(self.module, "malloc", malloc_type);
        try self.functions.put("malloc", malloc_func);
        
        const free_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context), // return void
            @as([*c]c.LLVMTypeRef, @ptrCast(@constCast(&[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}))), // void* parameter
            1, // parameter count
            0  // not variadic
        );
        const free_func = c.LLVMAddFunction(self.module, "free", free_type);
        try self.functions.put("free", free_func);
    }

    fn generateStatement(self: *CodeGen, stmt: Statement) CodeGenError!void {
        switch (stmt.tag) {
            .Function => {
                const func: *FunctionStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateFunction(func.*);
            },
            .Expression => {
                const expr: *Expression = @ptrCast(@alignCast(stmt.data));
                _ = try self.generateExpression(expr.*);
            },
            else => {
                // TODO: Implement other statement types
            },
        }
    }

    fn generateFunction(self: *CodeGen, func: FunctionStatement) CodeGenError!void {
        // Create function type
        var param_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer param_types.deinit();
        
        for (func.parameters.items) |param| {
            const param_type = try self.getLLVMType(param.param_type);
            try param_types.append(param_type);
        }
        
        const return_type = if (func.return_type) |ret_type|
            try self.getLLVMType(ret_type)
        else
            c.LLVMVoidTypeInContext(self.context);
        
        const function_type = c.LLVMFunctionType(
            return_type,
            if (param_types.items.len > 0) param_types.items.ptr else null,
            @as(u32, @intCast(param_types.items.len)),
            0 // not variadic
        );
        
        // Create function
        const function = c.LLVMAddFunction(self.module, func.name.ptr, function_type);
        try self.functions.put(func.name, function);
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Set current function context
        const old_function = self.current_function;
        self.current_function = function;
        
        // Create parameter allocas
        for (func.parameters.items, 0..) |param, i| {
            const param_value = c.LLVMGetParam(function, @as(u32, @intCast(i)));
            const param_type = try self.getLLVMType(param.param_type);
            const alloca = c.LLVMBuildAlloca(self.builder, param_type, param.name.ptr);
            _ = c.LLVMBuildStore(self.builder, param_value, alloca);
            try self.variables.put(param.name, alloca);
        }
        
        // Generate function body
        try self.generateStatement(func.body);
        
        // Add return void if no explicit return
        const last_block = c.LLVMGetInsertBlock(self.builder);
        if (c.LLVMGetBasicBlockTerminator(last_block) == null) {
            if (func.return_type == null) {
                _ = c.LLVMBuildRetVoid(self.builder);
            } else {
                // Return default value for type
                const default_value = try self.getDefaultValue(func.return_type.?);
                _ = c.LLVMBuildRet(self.builder, default_value);
            }
        }
        
        // Restore previous function context
        self.current_function = old_function;
        
        // Clear local variables
        self.variables.clearRetainingCapacity();
    }

    fn generateMainWrapper(self: *CodeGen) CodeGenError!void {
        // Create main function that calls main_character
        const main_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return int
            null, // no parameters
            0, // parameter count
            0  // not variadic
        );
        
        const main_function = c.LLVMAddFunction(self.module, "main", main_type);
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Call main_character if it exists
        if (self.functions.get("main_character")) |main_char_func| {
            _ = c.LLVMBuildCall2(self.builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(main_char_func)), main_char_func, null, 0, "");
        }
        
        // Return 0
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
    }



    fn generateBinaryOp_old(self: *CodeGen, _: c.LLVMValueRef, _: []const u8, _: c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        // TODO: Implement binary operations
        return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 1);
    }

    fn getLLVMTypeFromString(self: *CodeGen, cursed_type: []const u8) !c.LLVMTypeRef {
        if (std.mem.eql(u8, cursed_type, "normie")) {
            return c.LLVMInt32TypeInContext(self.context);
        } else if (std.mem.eql(u8, cursed_type, "meal")) {
            return c.LLVMDoubleTypeInContext(self.context);
        } else if (std.mem.eql(u8, cursed_type, "tea")) {
            return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        }
        return error.UnknownType;
    }

    // Generate expressions based on type
    fn generateExpression(self: *CodeGen, expr: ast.Expression) CodeGenError!c.LLVMValueRef {
        switch (expr) {
            .Literal => |literal| {
                switch (literal) {
                    .IntegerLiteral => |int| {
                        return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @intCast(int), 0);
                    },
                    .FloatLiteral => |float| {
                        return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float);
                    },
                    .StringLiteral => |str| {
                        return c.LLVMBuildGlobalStringPtr(self.builder, str.ptr, "str");
                    },
                    .BooleanLiteral => |bool_val| {
                        return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0);
                    },
                }
            },
            .Identifier => |ident| {
                // Look up variable in symbol table
                if (self.symbols.get(ident)) |value| {
                    return value;
                } else {
                    return error.UndefinedVariable;
                }
            },
            .BinaryOp => |binary| {
                const left = try self.generateExpression(binary.left.*);
                const right = try self.generateExpression(binary.right.*);
                return try self.generateBinaryOp(left, binary.operator, right);
            },
            .UnaryOp => |unary| {
                const operand = try self.generateExpression(unary.operand.*);
                return try self.generateUnaryOp(unary.operator, operand);
            },
            .FunctionCall => |call| {
                return try self.generateFunctionCall(call);
            },
            else => {
                // Default fallback for unimplemented expressions
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 1);
            },
        }
    }

    fn generateBinaryOp(self: *CodeGen, left: c.LLVMValueRef, operator: []const u8, right: c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        if (std.mem.eql(u8, operator, "+")) {
            return c.LLVMBuildAdd(self.builder, left, right, "add");
        } else if (std.mem.eql(u8, operator, "-")) {
            return c.LLVMBuildSub(self.builder, left, right, "sub");
        } else if (std.mem.eql(u8, operator, "*")) {
            return c.LLVMBuildMul(self.builder, left, right, "mul");
        } else if (std.mem.eql(u8, operator, "/")) {
            return c.LLVMBuildSDiv(self.builder, left, right, "div");
        } else if (std.mem.eql(u8, operator, "%")) {
            return c.LLVMBuildSRem(self.builder, left, right, "rem");
        } else if (std.mem.eql(u8, operator, "==")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq");
        } else if (std.mem.eql(u8, operator, "!=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "ne");
        } else if (std.mem.eql(u8, operator, "<")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt");
        } else if (std.mem.eql(u8, operator, "<=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, left, right, "le");
        } else if (std.mem.eql(u8, operator, ">")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt");
        } else if (std.mem.eql(u8, operator, ">=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, left, right, "ge");
        } else if (std.mem.eql(u8, operator, "&&")) {
            return c.LLVMBuildAnd(self.builder, left, right, "and");
        } else if (std.mem.eql(u8, operator, "||")) {
            return c.LLVMBuildOr(self.builder, left, right, "or");
        } else if (std.mem.eql(u8, operator, "&")) {
            return c.LLVMBuildAnd(self.builder, left, right, "bitand");
        } else if (std.mem.eql(u8, operator, "|")) {
            return c.LLVMBuildOr(self.builder, left, right, "bitor");
        } else if (std.mem.eql(u8, operator, "^")) {
            return c.LLVMBuildXor(self.builder, left, right, "xor");
        } else if (std.mem.eql(u8, operator, "<<")) {
            return c.LLVMBuildShl(self.builder, left, right, "shl");
        } else if (std.mem.eql(u8, operator, ">>")) {
            return c.LLVMBuildAShr(self.builder, left, right, "shr");
        } else {
            std.debug.print("Unsupported binary operator: {s}\n", .{operator});
            return CodeGenError.LLVMError;
        }
    }

    fn generateCall(self: *CodeGen, call: ast.CallExpression) CodeGenError!c.LLVMValueRef {
        // Handle built-in functions
        switch (call.function.*) {
            .MemberAccess => |member| {
                if (std.mem.eql(u8, member.property, "spill")) {
                    // vibez.spill - print function
                    if (call.arguments.items.len != 1) {
                        return CodeGenError.TypeMismatch;
                    }
                    
                    const arg = try self.generateExpression(call.arguments.items[0]);
                    const printf_func = self.functions.get("printf").?;
                    
                    // Create format string based on argument type
                    const arg_type = c.LLVMTypeOf(arg);
                    var format_str: []const u8 = undefined;
                    
                    if (c.LLVMGetTypeKind(arg_type) == c.LLVMIntegerTypeKind) {
                        const bit_width = c.LLVMGetIntTypeWidth(arg_type);
                        if (bit_width == 1) {
                            format_str = "%s\n"; // boolean
                            // Convert bool to string
                            const true_str = c.LLVMBuildGlobalStringPtr(self.builder, "true", "true_str");
                            const false_str = c.LLVMBuildGlobalStringPtr(self.builder, "false", "false_str");
                            const cond_arg = c.LLVMBuildSelect(self.builder, arg, true_str, false_str, "bool_str");
                            const format = c.LLVMBuildGlobalStringPtr(self.builder, format_str.ptr, "fmt");
                            return c.LLVMBuildCall2(self.builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)), printf_func, &[_]c.LLVMValueRef{format, cond_arg}, 2, "print_call");
                        } else {
                            format_str = "%lld\n"; // integer
                        }
                    } else if (c.LLVMGetTypeKind(arg_type) == c.LLVMDoubleTypeKind) {
                        format_str = "%f\n"; // float
                    } else if (c.LLVMGetTypeKind(arg_type) == c.LLVMPointerTypeKind) {
                        format_str = "%s\n"; // string
                    } else {
                        format_str = "%p\n"; // pointer/other
                    }
                    
                    const format = c.LLVMBuildGlobalStringPtr(self.builder, format_str.ptr, "fmt");
                    return c.LLVMBuildCall2(self.builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)), printf_func, &[_]c.LLVMValueRef{format, arg}, 2, "print_call");
                }
            },
            .Identifier => |name| {
                if (self.functions.get(name)) |function| {
                    // Generate arguments
                    var args = ArrayList(c.LLVMValueRef).init(self.allocator);
                    defer args.deinit();
                    
                    for (call.arguments.items) |arg_expr| {
                        const arg = try self.generateExpression(arg_expr);
                        try args.append(arg);
                    }
                    
                    return c.LLVMBuildCall2(
                        self.builder,
                        c.LLVMGetReturnType(c.LLVMGlobalGetValueType(function)),
                        function,
                        if (args.items.len > 0) args.items.ptr else null,
                        @as(u32, @intCast(args.items.len)),
                        "call"
                    );
                }
            },
            else => {},
        }
        
        return CodeGenError.UndefinedSymbol;
    }

    fn generateMemberAccess(self: *CodeGen, member: ast.MemberAccessExpression) CodeGenError!c.LLVMValueRef {
        // For now, just return the object (simplified implementation)
        return try self.generateExpression(member.object.*);
    }

    fn generateLet(self: *CodeGen, let: ast.LetStatement) CodeGenError!void {
        const var_type = if (let.var_type) |vt| 
            try self.getLLVMType(vt) 
        else 
            c.LLVMInt64TypeInContext(self.context); // default to i64
        
        const alloca = c.LLVMBuildAlloca(self.builder, var_type, let.name.ptr);
        
        if (let.initializer) |initializer_expr| {
            const value = try self.generateExpression(initializer_expr);
            _ = c.LLVMBuildStore(self.builder, value, alloca);
        }
        
        try self.variables.put(let.name, alloca);
    }

    fn generateReturn(self: *CodeGen, ret: ast.ReturnStatement) CodeGenError!void {
        if (ret.value) |value| {
            const return_value = try self.generateExpression(value);
            _ = c.LLVMBuildRet(self.builder, return_value);
        } else {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
    }

    fn generateIf(self: *CodeGen, if_stmt: ast.IfStatement) CodeGenError!void {
        const condition = try self.generateExpression(if_stmt.condition);
        
        const function = self.current_function.?;
        const then_block = c.LLVMAppendBasicBlockInContext(self.context, function, "then");
        const else_block = c.LLVMAppendBasicBlockInContext(self.context, function, "else");
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, function, "merge");
        
        _ = c.LLVMBuildCondBr(self.builder, condition, then_block, else_block);
        
        // Generate then branch
        c.LLVMPositionBuilderAtEnd(self.builder, then_block);
        for (if_stmt.then_branch.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Only add branch if block doesn't already have a terminator
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Generate else branch
        c.LLVMPositionBuilderAtEnd(self.builder, else_block);
        if (if_stmt.else_branch) |else_stmts| {
            for (else_stmts.items) |stmt| {
                try self.generateStatement(stmt);
            }
        }
        
        // Only add branch if block doesn't already have a terminator
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Continue building in merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
    }

    fn generateWhile(self: *CodeGen, while_stmt: ast.WhileStatement) CodeGenError!void {
        const function = self.current_function.?;
        const condition_block = c.LLVMAppendBasicBlockInContext(self.context, function, "while_cond");
        const body_block = c.LLVMAppendBasicBlockInContext(self.context, function, "while_body");
        const exit_block = c.LLVMAppendBasicBlockInContext(self.context, function, "while_exit");
        
        _ = c.LLVMBuildBr(self.builder, condition_block);
        
        // Generate condition
        c.LLVMPositionBuilderAtEnd(self.builder, condition_block);
        const condition = try self.generateExpression(while_stmt.condition);
        _ = c.LLVMBuildCondBr(self.builder, condition, body_block, exit_block);
        
        // Generate body
        c.LLVMPositionBuilderAtEnd(self.builder, body_block);
        for (while_stmt.body.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Only add branch if block doesn't already have a terminator
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, condition_block);
        }
        
        // Continue building in exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
    }

    fn getLLVMType(self: *CodeGen, cursed_type: ast.Type) CodeGenError!c.LLVMTypeRef {
        switch (cursed_type) {
            .Basic => |basic| {
                switch (basic) {
                    .Normie => return c.LLVMInt32TypeInContext(self.context),
                    .Tea, .Txt => return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
                    .Sip => return c.LLVMInt8TypeInContext(self.context),
                    .Smol => return c.LLVMInt8TypeInContext(self.context),
                    .Mid => return c.LLVMInt16TypeInContext(self.context),
                    .Thicc => return c.LLVMInt64TypeInContext(self.context),
                    .Snack => return c.LLVMFloatTypeInContext(self.context),
                    .Meal => return c.LLVMDoubleTypeInContext(self.context),
                    .Byte => return c.LLVMInt8TypeInContext(self.context),
                    .Rune => return c.LLVMInt32TypeInContext(self.context),
                    .Lit => return c.LLVMInt1TypeInContext(self.context),
                    .Cap => return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
                    else => return CodeGenError.InvalidType,
                }
            },
            else => return CodeGenError.InvalidType,
        }
    }

    fn getDefaultValue(self: *CodeGen, cursed_type: ast.Type) CodeGenError!c.LLVMValueRef {
        const llvm_type = try self.getLLVMType(cursed_type);
        
        switch (cursed_type) {
            .Basic => |basic| {
                switch (basic) {
                    .Normie, .Smol, .Mid, .Thicc, .Byte, .Rune => {
                        return c.LLVMConstInt(llvm_type, 0, 0);
                    },
                    .Snack, .Meal => {
                        return c.LLVMConstReal(llvm_type, 0.0);
                    },
                    .Lit => {
                        return c.LLVMConstInt(llvm_type, 0, 0); // false
                    },
                    .Tea, .Txt, .Cap => {
                        return c.LLVMConstNull(llvm_type);
                    },
                    else => return CodeGenError.InvalidType,
                }
            },
            else => return CodeGenError.InvalidType,
        }
    }

    fn optimizeModule(self: *CodeGen) CodeGenError!void {
        // Create pass manager
        const pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(pass_manager);
        
        // Add basic optimization passes
        c.LLVMAddInstructionCombiningPass(pass_manager);
        c.LLVMAddReassociatePass(pass_manager);
        c.LLVMAddGVNPass(pass_manager);
        c.LLVMAddCFGSimplificationPass(pass_manager);
        c.LLVMAddPromoteMemoryToRegisterPass(pass_manager);
        
        // Run optimization passes
        _ = c.LLVMRunPassManager(pass_manager, self.module);
    }

    pub fn writeExecutable(self: *CodeGen, output_path: []const u8) CodeGenError!void {
        // Write LLVM IR to file for debugging
        var ir_filename = ArrayList(u8).init(self.allocator);
        defer ir_filename.deinit();
        
        try ir_filename.appendSlice(output_path);
        try ir_filename.appendSlice(".ll");
        
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMPrintModuleToFile(self.module, ir_filename.items.ptr, &error_msg) != 0) {
            std.debug.print("Failed to write LLVM IR: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
        
        // Create execution engine for native compilation
        var execution_engine: c.LLVMExecutionEngineRef = undefined;
        if (c.LLVMCreateExecutionEngineForModule(&execution_engine, self.module, &error_msg) != 0) {
            std.debug.print("Failed to create execution engine: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
        defer c.LLVMDisposeExecutionEngine(execution_engine);
        
        // For now, just write the IR file - native compilation would require more setup
        std.debug.print("Generated LLVM IR: {s}\n", .{ir_filename.items});
        std.debug.print("Note: Native compilation not yet implemented. Use llc to compile IR to object file.\n", .{});
    }

    /// Generate struct definition
    fn generateStruct(self: *CodeGen, struct_stmt: ast.StructStatement) CodeGenError!void {
        // Create field types array
        var field_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer field_types.deinit();
        
        for (struct_stmt.fields.items) |field| {
            const field_type = try self.getLLVMType(field.field_type);
            try field_types.append(field_type);
        }
        
        // Create LLVM struct type
        const struct_type = c.LLVMStructCreateNamed(self.context, struct_stmt.name.ptr);
        c.LLVMStructSetBody(struct_type, field_types.items.ptr, @as(u32, @intCast(field_types.items.len)), 0);
        
        // Store struct type for later use
        try self.struct_types.put(struct_stmt.name, struct_type);
    }

    /// Generate interface definition
    fn generateInterface(self: *CodeGen, interface_stmt: ast.InterfaceStatement) CodeGenError!void {
        var methods = ArrayList(InterfaceMethod).init(self.allocator);
        
        for (interface_stmt.methods.items, 0..) |method, index| {
            // Create function type for method
            var param_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
            defer param_types.deinit();
            
            // Add self parameter (pointer to implementing type)
            try param_types.append(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));
            
            // Add method parameters
            for (method.parameters.items) |param| {
                const param_type = try self.getLLVMType(param.param_type);
                try param_types.append(param_type);
            }
            
            const return_type = if (method.return_type) |ret_type|
                try self.getLLVMType(ret_type)
            else
                c.LLVMVoidTypeInContext(self.context);
            
            const function_type = c.LLVMFunctionType(
                return_type,
                param_types.items.ptr,
                @as(u32, @intCast(param_types.items.len)),
                0 // not variadic
            );
            
            const interface_method = InterfaceMethod{
                .name = method.name,
                .index = index,
                .function_type = function_type,
            };
            
            try methods.append(interface_method);
        }
        
        // Create vtable type (array of function pointers)
        const func_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        const vtable_type = c.LLVMArrayType(func_ptr_type, @as(u32, @intCast(methods.items.len)));
        
        const interface_info = InterfaceInfo{
            .name = interface_stmt.name,
            .methods = methods,
            .vtable_type = vtable_type,
        };
        
        try self.interface_types.put(interface_stmt.name, interface_info);
    }

    /// Generate struct literal expression
    fn generateStructLiteral(self: *CodeGen, struct_lit: ast.StructLiteralExpression) CodeGenError!c.LLVMValueRef {
        const struct_type = self.struct_types.get(struct_lit.struct_name) orelse {
            return CodeGenError.UndefinedSymbol;
        };
        
        // Allocate memory for struct
        const struct_size = c.LLVMSizeOf(struct_type);
        const malloc_func = self.functions.get("malloc").?;
        const struct_ptr = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(malloc_func)),
            malloc_func,
            &[_]c.LLVMValueRef{struct_size},
            1,
            "struct_alloc"
        );
        
        // Cast to proper struct pointer type
        const typed_ptr = c.LLVMBuildBitCast(
            self.builder,
            struct_ptr,
            c.LLVMPointerType(struct_type, 0),
            "struct_ptr"
        );
        
        // Initialize fields
        for (struct_lit.fields.items, 0..) |field_assignment, i| {
            const field_value = try self.generateExpression(field_assignment.value);
            const field_ptr = c.LLVMBuildStructGEP2(
                self.builder,
                struct_type,
                typed_ptr,
                @as(u32, @intCast(i)),
                "field_ptr"
            );
            _ = c.LLVMBuildStore(self.builder, field_value, field_ptr);
        }
        
        return typed_ptr;
    }

    /// Generate tuple expression
    fn generateTuple(self: *CodeGen, tuple: ast.TupleExpression) CodeGenError!c.LLVMValueRef {
        // Create tuple type
        var element_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer element_types.deinit();
        
        var element_values = ArrayList(c.LLVMValueRef).init(self.allocator);
        defer element_values.deinit();
        
        for (tuple.elements.items) |element| {
            const value = try self.generateExpression(element);
            const value_type = c.LLVMTypeOf(value);
            try element_types.append(value_type);
            try element_values.append(value);
        }
        
        // Create tuple struct type
        const tuple_type = c.LLVMStructTypeInContext(
            self.context,
            element_types.items.ptr,
            @as(u32, @intCast(element_types.items.len)),
            0
        );
        
        // Create tuple value
        var tuple_value = c.LLVMGetUndef(tuple_type);
        for (element_values.items, 0..) |value, i| {
            tuple_value = c.LLVMBuildInsertValue(
                self.builder,
                tuple_value,
                value,
                @as(u32, @intCast(i)),
                "tuple_elem"
            );
        }
        
        return tuple_value;
    }

    /// Generate tuple access expression
    fn generateTupleAccess(self: *CodeGen, tuple_access: ast.TupleAccessExpression) CodeGenError!c.LLVMValueRef {
        const tuple_value = try self.generateExpression(tuple_access.tuple.*);
        
        return c.LLVMBuildExtractValue(
            self.builder,
            tuple_value,
            @as(u32, @intCast(tuple_access.index)),
            "tuple_access"
        );
    }

    // CURSED Error Handling System Code Generation
    
    fn generateYikes(self: *CodeGen, yikes: ast.YikesStatement) CodeGenError!void {
        // Create error type structure 
        // For CURSED error types: {i8*, i64, i8*} = {message, code, context}
        const error_struct_types = [_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // message
            c.LLVMInt64TypeInContext(self.context),                      // code
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // context
        };
        
        const error_type = c.LLVMStructTypeInContext(
            self.context,
            &error_struct_types,
            error_struct_types.len,
            0 // not packed
        );
        
        // Register the error type in symbol table
        try self.struct_types.put(yikes.name, error_type);
        
        // If there's an initial value, create a global constant
        if (yikes.value) |value| {
            const error_value = try self.generateExpression(value);
            const global_error = c.LLVMAddGlobal(self.module, error_type, yikes.name.ptr);
            c.LLVMSetInitializer(global_error, error_value);
            try self.variables.put(yikes.name, global_error);
        }
    }

    fn generateFam(self: *CodeGen, fam: ast.FamStatement) CodeGenError!void {
        // Implement panic recovery blocks using LLVM exception handling
        const current_func = self.current_function orelse return CodeGenError.LLVMError;
        
        // Create basic blocks for try/catch pattern
        const try_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "fam_try");
        const catch_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "fam_catch");
        const continue_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "fam_continue");
        
        // Generate setjmp/longjmp style error handling for simplified implementation
        // In production, would use LLVM's proper exception handling intrinsics
        
        // Jump to try block
        _ = c.LLVMBuildBr(self.builder, try_block);
        
        // Generate try block
        c.LLVMPositionBuilderAtEnd(self.builder, try_block);
        
        // Execute main body with error propagation context
        for (fam.body.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // If no exception occurred, jump to continue
        _ = c.LLVMBuildBr(self.builder, continue_block);
        
        // Generate catch block (if recovery body exists)
        if (fam.recovery_body) |recovery| {
            c.LLVMPositionBuilderAtEnd(self.builder, catch_block);
            
            // If error variable specified, bind the caught error
            if (fam.error_variable) |error_var| {
                // Create error value binding (simplified)
                const error_alloca = c.LLVMBuildAlloca(
                    self.builder,
                    c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
                    error_var.ptr
                );
                try self.variables.put(error_var, error_alloca);
            }
            
            // Execute recovery code
            for (recovery.items) |stmt| {
                try self.generateStatement(stmt);
            }
            
            _ = c.LLVMBuildBr(self.builder, continue_block);
        }
        
        // Continue execution after fam block
        c.LLVMPositionBuilderAtEnd(self.builder, continue_block);
    }

    fn generateShook(self: *CodeGen, shook: ast.ShookExpression) CodeGenError!c.LLVMValueRef {
        // Generate the wrapped expression that might fail
        const result = try self.generateExpression(shook.expression.*);
        
        // Check if result indicates an error (simplified error propagation)
        // In full implementation, would check error union type and propagate accordingly
        
        // For now, create a simple error check pattern:
        // if (is_error(result)) return error;
        // return result;
        
        const is_error_func = c.LLVMGetNamedFunction(self.module, "cursed_is_error");
        if (is_error_func == null) {
            // Create error checking function if it doesn't exist
            const error_check_type = c.LLVMFunctionType(
                c.LLVMInt1TypeInContext(self.context), // returns bool
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}, // takes pointer
                1,
                0
            );
            _ = c.LLVMAddFunction(self.module, "cursed_is_error", error_check_type);
        }
        
        // Generate error propagation logic
        const current_func = self.current_function orelse return CodeGenError.LLVMError;
        const error_check_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "shook_check");
        const return_error_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "shook_error");
        const continue_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "shook_continue");
        
        // Jump to error check
        _ = c.LLVMBuildBr(self.builder, error_check_block);
        
        c.LLVMPositionBuilderAtEnd(self.builder, error_check_block);
        
        // Create error condition check (simplified)
        const null_check = c.LLVMBuildIsNull(self.builder, result, "error_check");
        _ = c.LLVMBuildCondBr(self.builder, null_check, return_error_block, continue_block);
        
        // Return error block
        c.LLVMPositionBuilderAtEnd(self.builder, return_error_block);
        _ = c.LLVMBuildRet(self.builder, result); // Return the error value
        
        // Continue with normal execution
        c.LLVMPositionBuilderAtEnd(self.builder, continue_block);
        
        return result;
    }
};

test "codegen basic" {
    const allocator = std.testing.allocator;
    
    var codegen = CodeGen.init(allocator);
    defer codegen.deinit();
    
    // Test basic initialization
    try std.testing.expect(codegen.context != null);
    try std.testing.expect(codegen.module != null);
    try std.testing.expect(codegen.builder != null);
}
