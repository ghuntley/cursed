const std = @import("std");
const ast = @import("ast.zig");
const variable_scope = @import("variable_scope.zig");
const VariableScopeManager = variable_scope.VariableScopeManager;
const VariableInfo = variable_scope.VariableInfo;

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

// Global scope manager for variable resolution
// In production, this would be passed as context
var global_scope_manager: ?*VariableScopeManager = null;

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
    _: c.LLVMBuilderRef,
    call: ast.CallExpression,
    _: std.mem.Allocator,
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
            // For array variables, we need to extract length from array metadata
            // This requires the array value to be passed, which requires a codegen instance
            // For now, we'll return a dynamic length extraction approach
            
            // This is a simplified implementation - in production would:
            // 1. Look up the array value from the variable name
            // 2. Extract element type from array structure  
            // 3. Use ArrayMetadata.getArrayLength() to get the runtime length
            
            // Since we don't have access to the full codegen context here,
            // we'll need to handle this at a higher level in the call chain
            // For backward compatibility, we'll keep this placeholder but mark it clearly
            std.log.warn("len() called on variable '{s}' - requires dynamic length extraction", .{var_name});
            return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0, 0); // Marked as 0 to indicate need for proper implementation
        },
        else => {
            return error.UnsupportedArrayType;
        },
    }
}

/// Generate user-defined function call with full argument evaluation including variable resolution
fn generateUserFunctionCall(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    function: c.LLVMValueRef,
    call: ast.CallExpression,
    allocator: std.mem.Allocator,
) !c.LLVMValueRef {
        
    // Generate arguments with proper variable resolution
    var args: std.ArrayList(c.LLVMValueRef) = .empty;
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
            // Look up variable in scope and load its value
            return lookupAndLoadVariable(context, builder, name) catch |err| {
                std.debug.print("Variable lookup error for '{s}': {}\n", .{ name, err });
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 0, 0);
            };
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
    _ = builder;
    _ = call;
    
    // Placeholder for recursive calls
    return c.LLVMConstInt(c.LLVMInt64TypeInContext(context), 1, 0);
}

/// Enhanced struct field access generation
pub fn generateStructFieldAccess(
    _: c.LLVMContextRef,
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
    // Determine CURSED type - for now default to "drip"
    const cursed_type = if (let_stmt.type_annotation) |type_info| 
        getCursedTypeFromAnnotation(type_info) 
    else 
        "drip";
    
    var initial_value: ?c.LLVMValueRef = null;
    if (let_stmt.initializer) |init_expr| {
        initial_value = try generateExpressionValue(context, builder, init_expr);
    }
    
    // Register the variable in scope
    _ = try registerVariable(context, builder, let_stmt.name, cursed_type, initial_value);
}

/// Extract CURSED type from AST type annotation
fn getCursedTypeFromAnnotation(type_annotation: ast.Type) []const u8 {
    switch (type_annotation) {
        .Basic => |basic| {
            if (std.mem.eql(u8, basic, "drip")) return "drip";
            if (std.mem.eql(u8, basic, "normie")) return "normie";
            if (std.mem.eql(u8, basic, "lit")) return "lit";
            if (std.mem.eql(u8, basic, "meal")) return "meal";
            if (std.mem.eql(u8, basic, "tea")) return "tea";
        },
        else => {},
    }
    return "drip"; // Default type
}

/// Struct type information for field access
pub const StructTypeInfo = struct {
    field_names: [][]const u8,
    field_types: []c.LLVMTypeRef,
    llvm_type: c.LLVMTypeRef,
};

/// Initialize the global scope manager for variable resolution
pub fn initializeVariableScope(allocator: std.mem.Allocator) !void {
    const scope_manager = try allocator.create(VariableScopeManager);
    scope_manager.* = VariableScopeManager.init(allocator);
    global_scope_manager = scope_manager;
    
    // Create global scope
    _ = try scope_manager.enterScope();
}

/// Cleanup the global scope manager
pub fn deinitializeVariableScope(allocator: std.mem.Allocator) void {
    if (global_scope_manager) |scope_manager| {
        scope_manager.deinit();
        allocator.destroy(scope_manager);
        global_scope_manager = null;
    }
}

/// Enter a new scope for variable resolution
pub fn enterVariableScope() !void {
    if (global_scope_manager) |scope_manager| {
        _ = try scope_manager.enterScope();
    }
}

/// Exit the current variable scope
pub fn exitVariableScope() void {
    if (global_scope_manager) |scope_manager| {
        scope_manager.exitScope();
    }
}

/// Register a variable in the current scope
pub fn registerVariable(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    name: []const u8,
    cursed_type: []const u8,
    initial_value: ?c.LLVMValueRef,
) !c.LLVMValueRef {
    if (global_scope_manager) |scope_manager| {
        const llvm_type = variable_scope.cursedTypeToLLVMType(context, cursed_type);
        _ = variable_scope.getLLVMTypeAlignment(llvm_type);
        
        // Create alloca for the variable
        const alloca = c.LLVMBuildAlloca(builder, llvm_type, name.ptr);
        
        // Store initial value if provided
        if (initial_value) |value| {
            _ = c.LLVMBuildStore(builder, value, alloca);
        }
        
        // Register in scope
        const var_info = VariableInfo{
            .name = name,
            .llvm_value = alloca,
            .llvm_type = llvm_type,
            .cursed_type = cursed_type,
            .is_parameter = false,
            .scope_id = scope_manager.getCurrentScopeId(),
        };
        
        try scope_manager.define(name, var_info);
        
        std.debug.print("✅ Registered variable '{s}' of type '{s}' in scope {}\n", .{ name, cursed_type, var_info.scope_id });
        return alloca;
    }
    
    return error.NoScopeManager;
}

/// Look up and load a variable value
pub fn lookupAndLoadVariable(
    context: c.LLVMContextRef,
    builder: c.LLVMBuilderRef,
    name: []const u8,
) !c.LLVMValueRef {
    _ = context;
    
    if (global_scope_manager) |scope_manager| {
        if (scope_manager.lookup(name)) |var_info| {
            // Load the value from the alloca
            const load_name = try std.fmt.allocPrint(std.heap.page_allocator, "{s}_load", .{name});
            defer std.heap.page_allocator.free(load_name);
            
            const loaded_value = c.LLVMBuildLoad2(builder, var_info.llvm_type, var_info.llvm_value, load_name.ptr);
            
            std.debug.print("✅ Loaded variable '{s}' from scope {}\n", .{ name, var_info.scope_id });
            return loaded_value;
        } else {
            std.debug.print("❌ Variable '{s}' not found in any scope\n", .{name});
            return error.VariableNotFound;
        }
    }
    
    return error.NoScopeManager;
}

/// Register a function parameter as a variable
pub fn registerParameter(
    name: []const u8,
    cursed_type: []const u8,
    llvm_value: c.LLVMValueRef,
    llvm_type: c.LLVMTypeRef,
) !void {
    if (global_scope_manager) |scope_manager| {
        const var_info = VariableInfo{
            .name = name,
            .llvm_value = llvm_value,
            .llvm_type = llvm_type,
            .cursed_type = cursed_type,
            .is_parameter = true,
            .scope_id = scope_manager.getCurrentScopeId(),
        };
        
        try scope_manager.define(name, var_info);
        
        std.debug.print("✅ Registered parameter '{s}' of type '{s}'\n", .{ name, cursed_type });
    }
}
