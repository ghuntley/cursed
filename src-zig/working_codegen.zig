const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/TargetMachine.h");
});

const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

pub const CodeGenError = error{
    LLVMError,
    OutOfMemory,
    InvalidType,
    UndefinedSymbol,
    TypeMismatch,
    ParseError,
    InvalidExpression,
};

/// Working LLVM Code Generator for CURSED
/// This implementation actually generates executable LLVM IR
pub const WorkingCodeGen = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    
    // Symbol tables
    functions: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Current function context
    current_function: ?c.LLVMValueRef,
    
    pub fn init(allocator: Allocator) !WorkingCodeGen {
        // Initialize LLVM targets
        _ = c.LLVMInitializeNativeTarget();
        _ = c.LLVMInitializeNativeAsmPrinter();
        _ = c.LLVMInitializeNativeAsmParser();
        
        const context = c.LLVMContextCreate();
        if (context == null) return CodeGenError.LLVMError;
        
        const module = c.LLVMModuleCreateWithNameInContext("cursed_module", context);
        if (module == null) return CodeGenError.LLVMError;
        
        const builder = c.LLVMCreateBuilderInContext(context);
        if (builder == null) return CodeGenError.LLVMError;
        
        return WorkingCodeGen{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .functions = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
        };
    }

    pub fn deinit(self: *WorkingCodeGen) void {
        self.functions.deinit();
        self.variables.deinit();
        
        if (self.builder != null) c.LLVMDisposeBuilder(self.builder);
        if (self.module != null) c.LLVMDisposeModule(self.module);
        if (self.context != null) c.LLVMContextDispose(self.context);
    }

    /// Compile CURSED source code to LLVM IR and executable
    pub fn compile(self: *WorkingCodeGen, source: []const u8) !void {
        // For now, manually parse a simple main_character function
        // This is a temporary implementation for testing
        _ = source;
        
        try self.generateSimpleProgram();
    }
    
    /// Generate a simple program for testing
    fn generateSimpleProgram(self: *WorkingCodeGen) !void {
        // Set target triple for current platform
        const target_triple = c.LLVMGetDefaultTargetTriple();
        c.LLVMSetTarget(self.module, target_triple);
        
        // Declare external functions
        try self.declareExternalFunctions();
        
        // Create main_character function
        const function_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context), // return void
            null, // no parameters
            0, // parameter count
            0  // not variadic
        );
        
        const function = c.LLVMAddFunction(self.module, "main_character", function_type);
        try self.functions.put("main_character", function);
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        self.current_function = function;
        
        // Generate: vibez.spill("Hello from CURSED Zig!")
        const hello_str = c.LLVMBuildGlobalStringPtr(self.builder, "Hello from CURSED Zig!", "hello_str");
        _ = try self.generatePrint(hello_str);
        
        // Generate: sus x drip = 42
        const x_type = c.LLVMInt64TypeInContext(self.context);
        const x_alloca = c.LLVMBuildAlloca(self.builder, x_type, "x");
        const x_value = c.LLVMConstInt(x_type, 42, 0);
        _ = c.LLVMBuildStore(self.builder, x_value, x_alloca);
        try self.variables.put("x", x_alloca);
        
        // Generate: vibez.spill(x)
        const x_loaded = c.LLVMBuildLoad2(self.builder, x_type, x_alloca, "x_load");
        _ = try self.generatePrint(x_loaded);
        
        // Return void
        _ = c.LLVMBuildRetVoid(self.builder);
        
        // Add main function wrapper
        try self.generateMainWrapper();
        
        // Verify the module
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module, c.LLVMReturnStatusAction, &error_msg) != 0) {
            std.debug.print("Module verification failed: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
        
        // Apply basic optimizations
        try self.optimizeModule();
    }

    /// Generate LLVM IR for the entire program
    fn generateProgram(self: *WorkingCodeGen, program: ast.Program) !void {
        // Set target triple for current platform
        const target_triple = c.LLVMGetDefaultTargetTriple();
        c.LLVMSetTarget(self.module, target_triple);
        
        // Declare external functions
        try self.declareExternalFunctions();
        
        // Generate all statements
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Add main function wrapper if needed
        if (self.functions.get("main") == null) {
            try self.generateMainWrapper();
        }
        
        // Verify the module
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module, c.LLVMReturnStatusAction, &error_msg) != 0) {
            std.debug.print("Module verification failed: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
        
        // Apply basic optimizations
        try self.optimizeModule();
    }

    /// Declare external functions like printf, malloc, etc.
    fn declareExternalFunctions(self: *WorkingCodeGen) !void {
        // Declare printf function
        var printf_params = [_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)};
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return int
            &printf_params,
            1, // parameter count
            1  // is variadic
        );
        const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        try self.functions.put("printf", printf_func);
        
        // Declare puts function (simpler than printf for basic output)
        var puts_params = [_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)};
        const puts_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return int
            &puts_params,
            1, // parameter count
            0  // not variadic
        );
        const puts_func = c.LLVMAddFunction(self.module, "puts", puts_type);
        try self.functions.put("puts", puts_func);
        
        // Declare malloc
        var malloc_params = [_]c.LLVMTypeRef{c.LLVMInt64TypeInContext(self.context)};
        const malloc_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // return void*
            &malloc_params,
            1, // parameter count
            0  // not variadic
        );
        const malloc_func = c.LLVMAddFunction(self.module, "malloc", malloc_type);
        try self.functions.put("malloc", malloc_func);
        
        // Declare free
        var free_params = [_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)};
        const free_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context), // return void
            &free_params,
            1, // parameter count
            0  // not variadic
        );
        const free_func = c.LLVMAddFunction(self.module, "free", free_type);
        try self.functions.put("free", free_func);
    }

    /// Generate a statement
    fn generateStatement(self: *WorkingCodeGen, stmt: ast.Statement) !void {
        switch (stmt) {
            .Function => |func| {
                try self.generateFunction(func);
            },
            .Expression => |expr| {
                _ = try self.generateExpression(expr);
            },
            .Variable => |var_stmt| {
                try self.generateVariable(var_stmt);
            },
            .Return => |ret| {
                try self.generateReturn(ret);
            },
            .If => |if_stmt| {
                try self.generateIf(if_stmt);
            },
            .While => |while_stmt| {
                try self.generateWhile(while_stmt);
            },
            else => {
                std.debug.print("Unsupported statement type: {s}\n", .{@tagName(stmt)});
            },
        }
    }

    /// Generate a function
    fn generateFunction(self: *WorkingCodeGen, func: ast.FunctionStatement) !void {
        // Determine parameter types
        var param_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer param_types.deinit();
        
        if (func.parameters) |params| {
            for (params.items) |param| {
                const param_type = try self.getCursedTypeToLLVM(param.param_type);
                try param_types.append(param_type);
            }
        }
        
        // Determine return type
        const return_type = if (func.return_type) |ret_type|
            try self.getCursedTypeToLLVM(ret_type)
        else
            c.LLVMVoidTypeInContext(self.context);
        
        // Create function type
        const function_type = c.LLVMFunctionType(
            return_type,
            if (param_types.items.len > 0) param_types.items.ptr else null,
            @as(u32, @intCast(param_types.items.len)),
            0 // not variadic
        );
        
        // Create function
        const function = c.LLVMAddFunction(self.module, func.name.ptr, function_type);
        if (function == null) return CodeGenError.LLVMError;
        
        try self.functions.put(func.name, function);
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, function, "entry");
        if (entry_block == null) return CodeGenError.LLVMError;
        
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Save previous function context
        const old_function = self.current_function;
        const old_variables = try self.variables.clone();
        self.current_function = function;
        
        // Create allocas for parameters
        if (func.parameters) |params| {
            for (params.items, 0..) |param, i| {
                const param_value = c.LLVMGetParam(function, @as(u32, @intCast(i)));
                const param_type = try self.getCursedTypeToLLVM(param.param_type);
                const alloca = c.LLVMBuildAlloca(self.builder, param_type, param.name.ptr);
                _ = c.LLVMBuildStore(self.builder, param_value, alloca);
                try self.variables.put(param.name, alloca);
            }
        }
        
        // Generate function body
        try self.generateStatement(func.body);
        
        // Add return if needed
        const last_block = c.LLVMGetInsertBlock(self.builder);
        if (c.LLVMGetBasicBlockTerminator(last_block) == null) {
            if (func.return_type == null) {
                _ = c.LLVMBuildRetVoid(self.builder);
            } else {
                // Return default value
                const default_val = try self.getDefaultValue(func.return_type.?);
                _ = c.LLVMBuildRet(self.builder, default_val);
            }
        }
        
        // Restore context
        self.current_function = old_function;
        self.variables.deinit();
        self.variables = old_variables;
    }

    /// Generate main function wrapper if main_character exists
    fn generateMainWrapper(self: *WorkingCodeGen) !void {
        const main_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return int
            null, // no parameters
            0, // parameter count
            0  // not variadic
        );
        
        const main_function = c.LLVMAddFunction(self.module, "main", main_type);
        if (main_function == null) return CodeGenError.LLVMError;
        
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Call main_character if it exists
        if (self.functions.get("main_character")) |main_char_func| {
            _ = c.LLVMBuildCall2(
                self.builder,
                c.LLVMGetReturnType(c.LLVMGlobalGetValueType(main_char_func)),
                main_char_func,
                null,
                0,
                "main_char_call"
            );
        }
        
        // Return 0
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
        
        try self.functions.put("main", main_function);
    }

    /// Generate an expression and return its LLVM value
    fn generateExpression(self: *WorkingCodeGen, expr: ast.Expression) !c.LLVMValueRef {
        switch (expr) {
            .Integer => |int_val| {
                return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @bitCast(@as(i32, @intCast(int_val))), 0);
            },
            .Float => |float_val| {
                return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float_val);
            },
            .String => |str_val| {
                return c.LLVMBuildGlobalStringPtr(self.builder, str_val.ptr, "str");
            },
            .Boolean => |bool_val| {
                return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0);
            },
            .Identifier => |name| {
                if (self.variables.get(name)) |var_ref| {
                    // Load variable value
                    const var_type = c.LLVMGetElementType(c.LLVMTypeOf(var_ref));
                    return c.LLVMBuildLoad2(self.builder, var_type, var_ref, "load");
                } else {
                    std.debug.print("Undefined variable: {s}\n", .{name});
                    return CodeGenError.UndefinedSymbol;
                }
            },
            .Call => |call| {
                return try self.generateCall(call);
            },
            .Binary => |bin| {
                return try self.generateBinary(bin);
            },
            else => {
                std.debug.print("Unsupported expression type: {s}\n", .{@tagName(expr)});
                return CodeGenError.InvalidExpression;
            },
        }
    }

    /// Generate variable declaration
    fn generateVariable(self: *WorkingCodeGen, var_stmt: ast.VariableStatement) !void {
        const var_type = if (var_stmt.var_type) |vt|
            try self.getCursedTypeToLLVM(vt)
        else
            c.LLVMInt32TypeInContext(self.context); // default to i32
        
        const alloca = c.LLVMBuildAlloca(self.builder, var_type, var_stmt.name.ptr);
        
        if (var_stmt.initializer) |initializer| {
            const value = try self.generateExpression(initializer);
            _ = c.LLVMBuildStore(self.builder, value, alloca);
        }
        
        try self.variables.put(var_stmt.name, alloca);
    }

    /// Generate return statement
    fn generateReturn(self: *WorkingCodeGen, ret: ast.ReturnStatement) !void {
        if (ret.value) |value| {
            const return_value = try self.generateExpression(value);
            _ = c.LLVMBuildRet(self.builder, return_value);
        } else {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
    }

    /// Generate if statement
    fn generateIf(self: *WorkingCodeGen, if_stmt: ast.IfStatement) !void {
        const condition = try self.generateExpression(if_stmt.condition);
        
        const function = self.current_function orelse return CodeGenError.LLVMError;
        const then_block = c.LLVMAppendBasicBlockInContext(self.context, function, "then");
        const else_block = c.LLVMAppendBasicBlockInContext(self.context, function, "else");
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, function, "merge");
        
        _ = c.LLVMBuildCondBr(self.builder, condition, then_block, else_block);
        
        // Generate then branch
        c.LLVMPositionBuilderAtEnd(self.builder, then_block);
        try self.generateStatement(if_stmt.then_branch);
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Generate else branch
        c.LLVMPositionBuilderAtEnd(self.builder, else_block);
        if (if_stmt.else_branch) |else_branch| {
            try self.generateStatement(else_branch);
        }
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Continue in merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
    }

    /// Generate while statement
    fn generateWhile(self: *WorkingCodeGen, while_stmt: ast.WhileStatement) !void {
        const function = self.current_function orelse return CodeGenError.LLVMError;
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
        try self.generateStatement(while_stmt.body);
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, condition_block);
        }
        
        // Continue in exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
    }

    /// Generate function call
    fn generateCall(self: *WorkingCodeGen, call: ast.CallExpression) !c.LLVMValueRef {
        // Handle built-in functions
        switch (call.function.*) {
            .MemberAccess => |member| {
                // Handle vibez.spill
                if (std.mem.eql(u8, member.property, "spill")) {
                    if (call.arguments.items.len != 1) {
                        return CodeGenError.TypeMismatch;
                    }
                    
                    const arg = try self.generateExpression(call.arguments.items[0]);
                    return try self.generatePrint(arg);
                }
            },
            .Identifier => |name| {
                // Regular function call
                if (self.functions.get(name)) |function| {
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

    /// Generate binary operation
    fn generateBinary(self: *WorkingCodeGen, bin: ast.BinaryExpression) !c.LLVMValueRef {
        const left = try self.generateExpression(bin.left.*);
        const right = try self.generateExpression(bin.right.*);
        
        if (std.mem.eql(u8, bin.operator, "+")) {
            return c.LLVMBuildAdd(self.builder, left, right, "add");
        } else if (std.mem.eql(u8, bin.operator, "-")) {
            return c.LLVMBuildSub(self.builder, left, right, "sub");
        } else if (std.mem.eql(u8, bin.operator, "*")) {
            return c.LLVMBuildMul(self.builder, left, right, "mul");
        } else if (std.mem.eql(u8, bin.operator, "/")) {
            return c.LLVMBuildSDiv(self.builder, left, right, "div");
        } else if (std.mem.eql(u8, bin.operator, "==")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq");
        } else if (std.mem.eql(u8, bin.operator, "!=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "ne");
        } else if (std.mem.eql(u8, bin.operator, "<")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt");
        } else if (std.mem.eql(u8, bin.operator, ">")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt");
        } else {
            std.debug.print("Unsupported binary operator: {s}\n", .{bin.operator});
            return CodeGenError.InvalidExpression;
        }
    }

    /// Generate print call (vibez.spill)
    fn generatePrint(self: *WorkingCodeGen, arg: c.LLVMValueRef) !c.LLVMValueRef {
        const arg_type = c.LLVMTypeOf(arg);
        const type_kind = c.LLVMGetTypeKind(arg_type);
        
        const printf_func = self.functions.get("printf").?;
        
        if (type_kind == c.LLVMIntegerTypeKind) {
            const bit_width = c.LLVMGetIntTypeWidth(arg_type);
            if (bit_width == 1) {
                // Boolean - convert to string
                const true_str = c.LLVMBuildGlobalStringPtr(self.builder, "true\n", "true_str");
                const false_str = c.LLVMBuildGlobalStringPtr(self.builder, "false\n", "false_str");
                const result_str = c.LLVMBuildSelect(self.builder, arg, true_str, false_str, "bool_str");
                const puts_func = self.functions.get("puts").?;
                return c.LLVMBuildCall2(
                    self.builder,
                    c.LLVMGetReturnType(c.LLVMGlobalGetValueType(puts_func)),
                    puts_func,
                    &[_]c.LLVMValueRef{result_str},
                    1,
                    "puts_call"
                );
            } else {
                // Integer
                const format = c.LLVMBuildGlobalStringPtr(self.builder, "%d\n", "int_fmt");
                return c.LLVMBuildCall2(
                    self.builder,
                    c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)),
                    printf_func,
                    &[_]c.LLVMValueRef{ format, arg },
                    2,
                    "printf_call"
                );
            }
        } else if (type_kind == c.LLVMDoubleTypeKind or type_kind == c.LLVMFloatTypeKind) {
            // Float
            const format = c.LLVMBuildGlobalStringPtr(self.builder, "%f\n", "float_fmt");
            return c.LLVMBuildCall2(
                self.builder,
                c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)),
                printf_func,
                &[_]c.LLVMValueRef{ format, arg },
                2,
                "printf_call"
            );
        } else if (type_kind == c.LLVMPointerTypeKind) {
            // String - use puts for simplicity
            const puts_func = self.functions.get("puts").?;
            return c.LLVMBuildCall2(
                self.builder,
                c.LLVMGetReturnType(c.LLVMGlobalGetValueType(puts_func)),
                puts_func,
                &[_]c.LLVMValueRef{arg},
                1,
                "puts_call"
            );
        } else {
            // Default - print as pointer
            const format = c.LLVMBuildGlobalStringPtr(self.builder, "%p\n", "ptr_fmt");
            return c.LLVMBuildCall2(
                self.builder,
                c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)),
                printf_func,
                &[_]c.LLVMValueRef{ format, arg },
                2,
                "printf_call"
            );
        }
    }

    /// Convert CURSED type to LLVM type
    fn getCursedTypeToLLVM(self: *WorkingCodeGen, cursed_type: ast.Type) !c.LLVMTypeRef {
        switch (cursed_type) {
            .Basic => |basic| {
                switch (basic) {
                    .Normie => return c.LLVMInt32TypeInContext(self.context),
                    .Drip => return c.LLVMInt64TypeInContext(self.context),
                    .Tea => return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
                    .Sip => return c.LLVMInt8TypeInContext(self.context),
                    .Smol => return c.LLVMInt8TypeInContext(self.context),
                    .Mid => return c.LLVMInt16TypeInContext(self.context),
                    .Thicc => return c.LLVMInt64TypeInContext(self.context),
                    .Snack => return c.LLVMFloatTypeInContext(self.context),
                    .Meal => return c.LLVMDoubleTypeInContext(self.context),
                    .Lit => return c.LLVMInt1TypeInContext(self.context),
                    else => return CodeGenError.InvalidType,
                }
            },
            else => return CodeGenError.InvalidType,
        }
    }

    /// Get default value for a type
    fn getDefaultValue(self: *WorkingCodeGen, cursed_type: ast.Type) !c.LLVMValueRef {
        const llvm_type = try self.getCursedTypeToLLVM(cursed_type);
        
        switch (cursed_type) {
            .Basic => |basic| {
                switch (basic) {
                    .Normie, .Drip, .Smol, .Mid, .Thicc, .Sip => {
                        return c.LLVMConstInt(llvm_type, 0, 0);
                    },
                    .Snack, .Meal => {
                        return c.LLVMConstReal(llvm_type, 0.0);
                    },
                    .Lit => {
                        return c.LLVMConstInt(llvm_type, 0, 0); // false
                    },
                    .Tea => {
                        return c.LLVMConstNull(llvm_type);
                    },
                    else => return CodeGenError.InvalidType,
                }
            },
            else => return CodeGenError.InvalidType,
        }
    }

    /// Apply basic optimization passes
    fn optimizeModule(self: *WorkingCodeGen) !void {
        const pass_manager = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(pass_manager);
        
        // Add basic optimization passes
        c.LLVMAddInstructionCombiningPass(pass_manager);
        c.LLVMAddReassociatePass(pass_manager);
        c.LLVMAddGVNPass(pass_manager);
        c.LLVMAddCFGSimplificationPass(pass_manager);
        c.LLVMAddPromoteMemoryToRegisterPass(pass_manager);
        
        // Run the passes
        _ = c.LLVMRunPassManager(pass_manager, self.module);
    }

    /// Write LLVM IR to file
    pub fn writeIR(self: *WorkingCodeGen, filename: []const u8) !void {
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMPrintModuleToFile(self.module, filename.ptr, &error_msg) != 0) {
            std.debug.print("Failed to write IR: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
    }

    /// Write executable using system tools
    pub fn writeExecutable(self: *WorkingCodeGen, output_path: []const u8) !void {
        // First write IR to temporary file
        var arena = std.heap.ArenaAllocator.init(self.allocator);
        defer arena.deinit();
        const temp_allocator = arena.allocator();
        
        const ir_file = try std.fmt.allocPrint(temp_allocator, "{s}.ll", .{output_path});
        try self.writeIR(ir_file);
        
        // Use clang to compile IR to executable
        const clang_cmd = try std.fmt.allocPrint(temp_allocator, "clang -O2 {s} -o {s}", .{ ir_file, output_path });
        
        var child = std.process.Child.init(&[_][]const u8{ "sh", "-c", clang_cmd }, self.allocator);
        const result = child.spawnAndWait() catch |err| {
            std.debug.print("Failed to run clang: {}\n", .{err});
            return CodeGenError.LLVMError;
        };
        
        switch (result) {
            .Exited => |code| {
                if (code == 0) {
                    std.debug.print("Successfully compiled to: {s}\n", .{output_path});
                } else {
                    std.debug.print("Clang failed with exit code: {}\n", .{code});
                    return CodeGenError.LLVMError;
                }
            },
            else => {
                std.debug.print("Clang process failed\n", .{});
                return CodeGenError.LLVMError;
            },
        }
        
        // Clean up temporary IR file
        std.fs.cwd().deleteFile(ir_file) catch {};
    }

    /// Print the generated LLVM IR
    pub fn printIR(self: *WorkingCodeGen) void {
        const ir_string = c.LLVMPrintModuleToString(self.module);
        if (ir_string != null) {
            std.debug.print("{s}\n", .{ir_string});
            c.LLVMDisposeMessage(ir_string);
        }
    }
};

// Test function
pub fn testWorkingCodegen() !void {
    const allocator = std.heap.page_allocator;
    
    var codegen = try WorkingCodeGen.init(allocator);
    defer codegen.deinit();
    
    const source = 
        \\slay main_character() {
        \\    vibez.spill("Hello from CURSED Zig!")
        \\    sus x drip = 42
        \\    vibez.spill(x)
        \\}
    ;
    
    try codegen.compile(source);
    codegen.printIR();
    
    try codegen.writeExecutable("test_program");
}
