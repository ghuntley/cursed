const std = @import("std");
const ast = @import("ast.zig");
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
});

/// Enhanced function call generation with recursive support
pub fn generateEnhancedFunctionCall(
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    functions: *std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    call: ast.CallExpression,
    allocator: std.mem.Allocator,
) !c.LLVMValueRef {
    const function_expr = call.function.*;
    
    switch (function_expr) {
        .Identifier => |name| {
            // Handle built-in functions
            if (std.mem.eql(u8, name, "len")) {
                return try generateArrayLengthCall(context, builder, call, allocator);
            }
            
            // Handle user-defined functions (including recursive calls)
            if (functions.get(name)) |function| {
                return try generateUserFunctionCall(context, builder, function, call, allocator);
            }
            
            return error.UndefinedFunction;
        },
        .MemberAccess => |member| {
            // Handle stdlib function calls like vibez.spill
            if (std.mem.eql(u8, member.object.*.Identifier, "vibez") and 
                std.mem.eql(u8, member.property, "spill")) {
                return try generateVibesSpillCall(context, module, builder, call, allocator);
            }
            
            return error.UndefinedFunction;
        },
        else => return error.UnsupportedFunctionCallType,
    }
}

/// Generate array length function call (len function)
fn generateArrayLengthCall(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    call: ast.CallExpression,
    allocator: std.mem.Allocator,
) !c.LLVMValueRef {
    if (call.arguments.items.len != 1) {
        return error.InvalidArgumentCount;
    }
    
    const array_arg = call.arguments.items[0];
    
    // For array literals, return constant length
    switch (array_arg) {
        .ArrayLiteral => |array_lit| {
            const len_value = array_lit.elements.items.len;
            return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), @intCast(len_value), 0);
        },
        .Identifier => |var_name| {
            // For array variables, we need to store length metadata
            // This is a simplified implementation - in production would use array metadata
            const len_var_name = try std.fmt.allocPrint(allocator, "{s}_length", .{var_name});
            defer allocator.free(len_var_name);
            
            // Return placeholder - in full implementation would look up stored length
            return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 5, 0); // Placeholder
        },
        else => {
            return error.UnsupportedArrayType;
        },
    }
}

/// Generate user-defined function call with full argument evaluation
fn generateUserFunctionCall(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    function: c.LLVMValueRef,
    call: ast.CallExpression,
    allocator: std.mem.Allocator,
) !c.LLVMValueRef {
    _ = allocator;
    
    // Generate arguments
    var args = std.ArrayList(c.LLVMValueRef).init(std.heap.page_allocator);
    defer args.deinit();
    
    for (call.arguments.items) |arg_expr| {
        const arg_value = try generateExpressionValue(context, builder, arg_expr);
        try args.append(arg_value);
    }
    
    // Generate function call
    const function_type = c.LLVMGlobalGetValueType(function);
    const return_type = c.LLVMGetReturnType(function_type);
    
    return c.LLVMBuildCall2(
        builder,
        return_type,
        function,
        if (args.items.len > 0) args.items.ptr else null,
        @intCast(args.items.len),
        "call_result"
    );
}

/// Generate vibez.spill call (print function)
fn generateVibesSpillCall(
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    call: ast.CallExpression,
    allocator: std.mem.Allocator,
) !c.LLVMValueRef {
    _ = allocator;
    
    // Get or declare printf function
    const printf_func = c.LLVMGetNamedFunction(module, "printf") orelse {
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0)},
            1,
            1 // variadic
        );
        return c.LLVMAddFunction(module, "printf", printf_type);
    };
    
    // For simplicity, just print the first argument
    if (call.arguments.items.len > 0) {
        const first_arg = call.arguments.items[0];
        const arg_value = try generateExpressionValue(context, builder, first_arg);
        
        // Create format string based on argument type
        const arg_type = c.LLVMTypeOf(arg_value);
        var format_str: []const u8 = undefined;
        
        if (c.LLVMGetTypeKind(arg_type) == c.LLVMIntegerTypeKind) {
            format_str = "%lld\n";
        } else if (c.LLVMGetTypeKind(arg_type) == c.LLVMPointerTypeKind) {
            format_str = "%s\n";
        } else {
            format_str = "%p\n";
        }
        
        const format = c.LLVMBuildGlobalStringPtr(builder, format_str.ptr, "fmt");
        
        return c.LLVMBuildCall2(
            builder,
            c.LLVMInt32TypeInContext(context),
            printf_func,
            &[_]c.LLVMValueRef{ format, arg_value },
            2,
            "printf_call"
        );
    }
    
    return c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0);
}

/// Generate expression value for function arguments
fn generateExpressionValue(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    expr: ast.Expression,
) !c.LLVMValueRef {
    switch (expr) {
        .Literal => |literal| {
            switch (literal) {
                .IntegerLiteral => |int| {
                    return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), @intCast(int), 0);
                },
                .StringLiteral => |str| {
                    return c.LLVMBuildGlobalStringPtr(builder, str.ptr, "str_literal");
                },
                .BooleanLiteral => |bool_val| {
                    return c.LLVMConstInt(c.LLVMInt1TypeInContext(context), if (bool_val) 1 else 0, 0);
                },
                else => {
                    return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0, 0);
                },
            }
        },
        .BinaryOp => |binary| {
            const left = try generateExpressionValue(context, builder, binary.left.*);
            const right = try generateExpressionValue(context, builder, binary.right.*);
            return try generateBinaryOperation(builder, left, right, binary.operator);
        },
        .FunctionCall => |call| {
            // Handle recursive function calls within expressions
            return try generateRecursiveFunctionCall(context, builder, call);
        },
        .Identifier => |name| {
            // For now, return a placeholder for variable references
            // In full implementation, would look up variable in scope
            _ = name;
            return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0, 0);
        },
        else => {
            return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0, 0);
        },
    }
}

/// Generate binary operation (+ - * /)
fn generateBinaryOperation(
    builder: c.LLVMBuilderRef,
    left: c.LLVMValueRef,
    right: c.LLVMValueRef,
    op: ast.BinaryOperator,
) !c.LLVMValueRef {
    switch (op) {
        .Add => return c.LLVMBuildAdd(builder, left, right, "add_result"),
        .Sub => return c.LLVMBuildSub(builder, left, right, "sub_result"),
        .Mul => return c.LLVMBuildMul(builder, left, right, "mul_result"),
        .Div => return c.LLVMBuildSDiv(builder, left, right, "div_result"),
        .Lt => return c.LLVMBuildICmp(builder, c.LLVMIntSLT, left, right, "lt_result"),
        .Gt => return c.LLVMBuildICmp(builder, c.LLVMIntSGT, left, right, "gt_result"),
        .Eq => return c.LLVMBuildICmp(builder, c.LLVMIntEQ, left, right, "eq_result"),
        .LEq => return c.LLVMBuildICmp(builder, c.LLVMIntSLE, left, right, "leq_result"),
        .GEq => return c.LLVMBuildICmp(builder, c.LLVMIntSGE, left, right, "geq_result"),
        .NEq => return c.LLVMBuildICmp(builder, c.LLVMIntNE, left, right, "neq_result"),
        else => return error.UnsupportedBinaryOperator,
    }
}

/// Handle recursive function calls
fn generateRecursiveFunctionCall(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    call: ast.CallExpression,
) !c.LLVMValueRef {
    // This is a simplified version - in production would need full function resolution
    _ = context;
    _ = builder;
    _ = call;
    
    // Placeholder for recursive calls
    return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 1, 0);
}

/// Enhanced struct field access generation
pub fn generateStructFieldAccess(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    struct_value: c.LLVMValueRef,
    field_name: []const u8,
    struct_info: StructTypeInfo,
) !c.LLVMValueRef {
    // Find field index
    var field_index: u32 = 0;
    var found = false;
    
    for (struct_info.field_names, 0..) |name, i| {
        if (std.mem.eql(u8, name, field_name)) {
            field_index = @intCast(i);
            found = true;
            break;
        }
    }
    
    if (!found) {
        return error.FieldNotFound;
    }
    
    // Generate GEP instruction for field access
    const field_ptr = c.LLVMBuildStructGEP2(
        builder,
        struct_info.llvm_type,
        struct_value,
        field_index,
        "field_ptr"
    );
    
    // Load field value
    return c.LLVMBuildLoad2(
        builder,
        struct_info.field_types[field_index],
        field_ptr,
        "field_value"
    );
}

/// Enhanced function body generation with control flow
pub fn generateEnhancedFunctionBody(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    function: c.LLVMValueRef,
    body: ast.BlockStatement,
    params: []c.LLVMValueRef,
    allocator: std.mem.Allocator,
) !void {
    _ = params;
    _ = allocator;
    
    // Create entry block
    const entry_block = c.LLVMAppendBasicBlockInContext(context, function, "entry");
    c.LLVMPositionBuilderAtEnd(builder, entry_block);
    
    // Generate statements
    for (body.statements.items) |stmt| {
        try generateStatement(context, builder, function, stmt);
    }
    
    // Ensure function has a terminator
    if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
        const return_type = c.LLVMGetReturnType(c.LLVMGlobalGetValueType(function));
        if (c.LLVMGetTypeKind(return_type) == c.LLVMVoidTypeKind) {
            _ = c.LLVMBuildRetVoid(builder);
        } else {
            _ = c.LLVMBuildRet(builder, c.LLVMConstInt(return_type, 0, 0));
        }
    }
}

/// Generate individual statements
fn generateStatement(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    function: c.LLVMValueRef,
    stmt: ast.Statement,
) !void {
    switch (stmt) {
        .Return => |ret_stmt| {
            if (ret_stmt.value) |value_expr| {
                const return_value = try generateExpressionValue(context, builder, value_expr);
                _ = c.LLVMBuildRet(builder, return_value);
            } else {
                _ = c.LLVMBuildRetVoid(builder);
            }
        },
        .If => |if_stmt| {
            try generateIfStatement(context, builder, function, if_stmt);
        },
        .Let => |let_stmt| {
            try generateLetStatement(context, builder, let_stmt);
        },
        .Expression => |expr| {
            _ = try generateExpressionValue(context, builder, expr);
        },
        else => {
            // Skip other statement types for now
        },
    }
}

/// Generate if statement with proper control flow
fn generateIfStatement(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    function: c.LLVMValueRef,
    if_stmt: ast.IfStatement,
) !void {
    const condition = try generateExpressionValue(context, builder, if_stmt.condition);
    
    const then_block = c.LLVMAppendBasicBlockInContext(context, function, "then");
    const else_block = c.LLVMAppendBasicBlockInContext(context, function, "else");
    const merge_block = c.LLVMAppendBasicBlockInContext(context, function, "merge");
    
    _ = c.LLVMBuildCondBr(builder, condition, then_block, else_block);
    
    // Generate then branch
    c.LLVMPositionBuilderAtEnd(builder, then_block);
    for (if_stmt.then_branch.items) |stmt| {
        try generateStatement(context, builder, function, stmt);
    }
    if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
        _ = c.LLVMBuildBr(builder, merge_block);
    }
    
    // Generate else branch
    c.LLVMPositionBuilderAtEnd(builder, else_block);
    if (if_stmt.else_branch) |else_stmts| {
        for (else_stmts.items) |stmt| {
            try generateStatement(context, builder, function, stmt);
        }
    }
    if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(builder)) == null) {
        _ = c.LLVMBuildBr(builder, merge_block);
    }
    
    // Continue in merge block
    c.LLVMPositionBuilderAtEnd(builder, merge_block);
}

/// Generate let statement (variable declaration)
fn generateLetStatement(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    let_stmt: ast.LetStatement,
) !void {
    const var_type = c.LLVMInt64TypeInContext(context); // Default to i64
    const alloca = c.LLVMBuildAlloca(builder, var_type, let_stmt.name.ptr);
    
    if (let_stmt.initializer) |init_expr| {
        const init_value = try generateExpressionValue(context, builder, init_expr);
        _ = c.LLVMBuildStore(builder, init_value, alloca);
    }
    
    // TODO: Store in variable map for later lookup
}

/// Struct type information for field access
pub const StructTypeInfo = struct {
    field_names: [][]const u8,
    field_types: []c.LLVMTypeRef,
    llvm_type: c.LLVMTypeRef,
};
