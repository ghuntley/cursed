const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const c = @import("llvm_c_bindings.zig");

pub const CodeGenError = error{
    UndefinedSymbol,
    InvalidType,
    OutOfMemory,
    CompilationFailed,
    LLVMInitializationFailed,
    ModuleCreationFailed,
    BuilderCreationFailed,
    UnsupportedOperation,
    InvalidFunction,
    InvalidExpression,
    InvalidStatement,
    MemoryAllocationFailed,
    TypeConversionFailed,
    UnreachableCodePath,
};

pub const CodeGen = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    functions: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    runtime_functions: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Control flow for loops and conditionals
    current_function: ?c.LLVMValueRef,
    loop_exit_blocks: ArrayList(c.LLVMBasicBlockRef),
    loop_continue_blocks: ArrayList(c.LLVMBasicBlockRef),

    pub fn init(allocator: Allocator) CodeGen {
        return CodeGen{
            .allocator = allocator,
            .context = c.LLVMContextCreate(),
            .module = null,
            .builder = null,
            .functions = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .runtime_functions = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
            .loop_exit_blocks = ArrayList(c.LLVMBasicBlockRef){},
            .loop_continue_blocks = ArrayList(c.LLVMBasicBlockRef){},
        };
    }

    pub fn deinit(self: *CodeGen) void {
        self.functions.deinit();
        self.variables.deinit();
        self.runtime_functions.deinit();
        self.loop_exit_blocks.deinit(self.allocator);
        self.loop_continue_blocks.deinit(self.allocator);
        
        if (self.builder) |builder| {
            c.LLVMDisposeBuilder(builder);
        }
        if (self.module) |module| {
            c.LLVMDisposeModule(module);
        }
        if (self.context) |context| {
            c.LLVMContextDispose(context);
        }
    }

    pub fn compile(self: *CodeGen, program: ast.Program) CodeGenError!void {
        // Initialize module and builder
        self.module = c.LLVMModuleCreateWithNameInContext("cursed_module", self.context);
        if (self.module == null) return CodeGenError.ModuleCreationFailed;
        
        self.builder = c.LLVMCreateBuilderInContext(self.context);
        if (self.builder == null) return CodeGenError.BuilderCreationFailed;
        
        // First pass: collect function definitions
        try self.collectFunctionDefinitions(program);
        
        // Create main function
        const main_type = c.LLVMFunctionType(c.LLVMInt32TypeInContext(self.context), null, 0, 0);
        const main_func = c.LLVMAddFunction(self.module, "main", main_type);
        self.current_function = main_func;
        
        const entry_bb = c.LLVMAppendBasicBlockInContext(self.context, main_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_bb);
        
        // Generate code for each statement in main
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Return 0 from main
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
    }
    
    fn collectFunctionDefinitions(self: *CodeGen, program: ast.Program) CodeGenError!void {
        for (program.statements.items) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            if (stmt.* == .Function) {
                try self.registerFunction(stmt.*.Function);
            }
        }
    }
    
    fn registerFunction(self: *CodeGen, func_stmt: ast.FunctionStatement) CodeGenError!void {
        // Create parameter types
        var param_types: []c.LLVMTypeRef = undefined;
        if (func_stmt.parameters.items.len > 0) {
            param_types = try self.allocator.alloc(c.LLVMTypeRef, func_stmt.parameters.items.len);
            for (param_types) |*param_type| {
                // Default to i64 for simplicity, could be enhanced with proper type inference
                param_type.* = c.LLVMInt64TypeInContext(self.context);
            }
        } else {
            param_types = &[_]c.LLVMTypeRef{};
        }
        defer if (func_stmt.parameters.items.len > 0) self.allocator.free(param_types);
        
        // Create function type (default return type: i64)
        const return_type = c.LLVMInt64TypeInContext(self.context);
        const func_type = c.LLVMFunctionType(return_type, param_types.ptr, @intCast(param_types.len), 0);
        
        // Add function to module
        const func = c.LLVMAddFunction(self.module, func_stmt.name.ptr, func_type);
        try self.functions.put(func_stmt.name, func);
    }

    fn generateStatement(self: *CodeGen, stmt_ptr: *anyopaque) CodeGenError!void {
        const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
        switch (stmt.*) {
            .Let => |let| try self.generateLet(let),
            .Assignment => |assign| try self.generateAssignment(assign),
            .Expression => |expr| {
                _ = try self.generateExpression(expr);
            },
            .If => |if_stmt| try self.generateIf(if_stmt),
            .While => |while_stmt| try self.generateWhile(while_stmt),
            .For => |for_stmt| try self.generateFor(for_stmt),
            .Function => |func_stmt| try self.generateFunction(func_stmt),
            .Return => |ret_stmt| try self.generateReturn(ret_stmt),
            .Break => |break_stmt| try self.generateBreak(break_stmt),
            .Continue => |continue_stmt| try self.generateContinue(continue_stmt),
            .Block => |block_stmt| try self.generateBlock(block_stmt),
            else => {
                // Placeholder for other statement types
                std.debug.print("Warning: Unsupported statement type in codegen\n", .{});
            },
        }
    }

    fn generateLet(self: *CodeGen, let_stmt: ast.LetStatement) CodeGenError!void {
        // Determine type from the type annotation or default to i64
        const var_type = if (let_stmt.type_annotation) |type_ann|
            try self.generateType(type_ann)
        else
            c.LLVMInt64TypeInContext(self.context);
            
        const alloca = c.LLVMBuildAlloca(self.builder, var_type, let_stmt.name.ptr);
        
        if (let_stmt.initializer) |value_expr| {
            const value = try self.generateExpression(value_expr);
            _ = c.LLVMBuildStore(self.builder, value, alloca);
        }
        
        try self.variables.put(let_stmt.name, alloca);
    }
    
    fn generateAssignment(self: *CodeGen, assign_stmt: ast.AssignmentStatement) CodeGenError!void {
        const variable = self.variables.get(assign_stmt.target) orelse
            return CodeGenError.UndefinedSymbol;
            
        const value = try self.generateExpression(assign_stmt.expression);
        _ = c.LLVMBuildStore(self.builder, value, variable);
    }
    
    fn generateIf(self: *CodeGen, if_stmt: ast.IfStatement) CodeGenError!void {
        // Generate condition
        const condition = try self.generateExpression(if_stmt.condition);
        
        // Create basic blocks
        const then_bb = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "if.then");
        const else_bb = if (if_stmt.else_branch.items.len > 0)
            c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "if.else")
        else
            null;
        const merge_bb = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "if.end");
        
        // Create conditional branch
        _ = c.LLVMBuildCondBr(self.builder, condition, then_bb, else_bb orelse merge_bb);
        
        // Generate then block
        c.LLVMPositionBuilderAtEnd(self.builder, then_bb);
        for (if_stmt.then_branch.items) |stmt| {
            try self.generateStatement(stmt);
        }
        _ = c.LLVMBuildBr(self.builder, merge_bb);
        
        // Generate else block if it exists
        if (else_bb) |ebb| {
            c.LLVMPositionBuilderAtEnd(self.builder, ebb);
            for (if_stmt.else_branch.items) |stmt| {
                try self.generateStatement(stmt);
            }
            _ = c.LLVMBuildBr(self.builder, merge_bb);
        }
        
        // Position builder at merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_bb);
    }
    
    fn generateWhile(self: *CodeGen, while_stmt: ast.WhileStatement) CodeGenError!void {
        // Create basic blocks
        const cond_bb = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "while.cond");
        const body_bb = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "while.body");
        const exit_bb = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "while.exit");
        
        // Push loop blocks for break/continue
        try self.loop_exit_blocks.append(self.allocator, exit_bb);
        try self.loop_continue_blocks.append(self.allocator, cond_bb);
        defer _ = self.loop_exit_blocks.pop();
        defer _ = self.loop_continue_blocks.pop();
        
        // Jump to condition
        _ = c.LLVMBuildBr(self.builder, cond_bb);
        
        // Generate condition block
        c.LLVMPositionBuilderAtEnd(self.builder, cond_bb);
        const condition = try self.generateExpression(while_stmt.condition);
        _ = c.LLVMBuildCondBr(self.builder, condition, body_bb, exit_bb);
        
        // Generate body block
        c.LLVMPositionBuilderAtEnd(self.builder, body_bb);
        for (while_stmt.body.items) |stmt| {
            try self.generateStatement(stmt);
        }
        _ = c.LLVMBuildBr(self.builder, cond_bb);
        
        // Position builder at exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_bb);
    }
    
    fn generateFor(self: *CodeGen, for_stmt: ast.ForStatement) CodeGenError!void {
        // For now, implement a simple for loop similar to C-style
        // Support both C-style and iterator-based for loops
        if (for_stmt.iterable != null) {
            // Iterator-based for loop
            try self.generateIteratorLoop(for_stmt);
        } else {
            // C-style for loop
            try self.generateCStyleLoop(for_stmt);
        }
        
        // Generate initialization
        if (for_stmt.init) |init_stmt| {
            try self.generateStatement(init_stmt);
        }
        
        // Create basic blocks
        const cond_bb = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "for.cond");
        const body_bb = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "for.body");
        const incr_bb = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "for.incr");
        const exit_bb = c.LLVMAppendBasicBlockInContext(self.context, self.current_function.?, "for.exit");
        
        // Push loop blocks
        try self.loop_exit_blocks.append(self.allocator, exit_bb);
        try self.loop_continue_blocks.append(self.allocator, incr_bb);
        defer _ = self.loop_exit_blocks.pop();
        defer _ = self.loop_continue_blocks.pop();
        
        // Jump to condition
        _ = c.LLVMBuildBr(self.builder, cond_bb);
        
        // Generate condition block
        c.LLVMPositionBuilderAtEnd(self.builder, cond_bb);
        if (for_stmt.condition) |cond| {
            const condition = try self.generateExpression(cond);
            _ = c.LLVMBuildCondBr(self.builder, condition, body_bb, exit_bb);
        } else {
            _ = c.LLVMBuildBr(self.builder, body_bb); // Infinite loop
        }
        
        // Generate body block
        c.LLVMPositionBuilderAtEnd(self.builder, body_bb);
        for (for_stmt.body.items) |stmt| {
            try self.generateStatement(stmt);
        }
        _ = c.LLVMBuildBr(self.builder, incr_bb);
        
        // Generate increment block
        c.LLVMPositionBuilderAtEnd(self.builder, incr_bb);
        if (for_stmt.increment) |incr| {
            try self.generateStatement(incr);
        }
        _ = c.LLVMBuildBr(self.builder, cond_bb);
        
        // Position builder at exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_bb);
    }
    
    fn generateFunction(self: *CodeGen, func_stmt: ast.FunctionStatement) CodeGenError!void {
        const func = self.functions.get(func_stmt.name) orelse
            return CodeGenError.UndefinedSymbol;
            
        // Create entry block
        const entry_bb = c.LLVMAppendBasicBlockInContext(self.context, func, "entry");
        
        // Save current state
        const old_function = self.current_function;
        const old_variables = self.variables;
        
        // Set up function context
        self.current_function = func;
        self.variables = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        c.LLVMPositionBuilderAtEnd(self.builder, entry_bb);
        
        // Set up parameters
        for (func_stmt.parameters.items, 0..) |param, i| {
            const param_value = c.LLVMGetParam(func, @intCast(i));
            const param_alloca = c.LLVMBuildAlloca(self.builder, c.LLVMTypeOf(param_value), param.name.ptr);
            _ = c.LLVMBuildStore(self.builder, param_value, param_alloca);
            try self.variables.put(param.name, param_alloca);
        }
        
        // Generate function body
        for (func_stmt.body.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // If no explicit return, add default return
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            const default_return = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
            _ = c.LLVMBuildRet(self.builder, default_return);
        }
        
        // Restore state
        self.variables.deinit();
        self.variables = old_variables;
        self.current_function = old_function;
    }
    
    fn generateReturn(self: *CodeGen, ret_stmt: ast.ReturnStatement) CodeGenError!void {
        if (ret_stmt.value) |value_expr| {
            const value = try self.generateExpression(value_expr);
            _ = c.LLVMBuildRet(self.builder, value);
        } else {
            const void_return = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
            _ = c.LLVMBuildRet(self.builder, void_return);
        }
    }
    
    fn generateBreak(self: *CodeGen, break_stmt: ast.BreakStatement) CodeGenError!void {
        _ = break_stmt; // Label support can be added later
        if (self.loop_exit_blocks.items.len > 0) {
            const exit_bb = self.loop_exit_blocks.items[self.loop_exit_blocks.items.len - 1];
            _ = c.LLVMBuildBr(self.builder, exit_bb);
        } else {
            return CodeGenError.InvalidStatement; // Break outside loop
        }
    }
    
    fn generateContinue(self: *CodeGen, continue_stmt: ast.ContinueStatement) CodeGenError!void {
        _ = continue_stmt; // Label support can be added later
        if (self.loop_continue_blocks.items.len > 0) {
            const continue_bb = self.loop_continue_blocks.items[self.loop_continue_blocks.items.len - 1];
            _ = c.LLVMBuildBr(self.builder, continue_bb);
        } else {
            return CodeGenError.InvalidStatement; // Continue outside loop
        }
    }
    
    fn generateBlock(self: *CodeGen, block_stmt: ast.BlockStatement) CodeGenError!void {
        for (block_stmt.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
    }
    
    fn generateType(self: *CodeGen, type_info: ast.Type) CodeGenError!c.LLVMTypeRef {
        return switch (type_info) {
            .Basic => |basic_type| switch (basic_type) {
                .Normie => c.LLVMInt32TypeInContext(self.context), // i32
                .Drip => c.LLVMInt64TypeInContext(self.context),    // f64 -> i64 for now
                .Tea, .Txt => c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // string (i8*)
                .Sip => c.LLVMInt8TypeInContext(self.context),     // char (i8)
                .Smol => c.LLVMInt8TypeInContext(self.context),    // i8
                .Mid => c.LLVMInt16TypeInContext(self.context),    // i16
                .Thicc => c.LLVMInt64TypeInContext(self.context),  // i64
                .Snack => c.LLVMInt32TypeInContext(self.context),  // f32 -> i32 for now
                .Meal => c.LLVMInt64TypeInContext(self.context),   // f64 -> i64 for now
                .Byte => c.LLVMInt8TypeInContext(self.context),    // u8
                .Rune => c.LLVMInt32TypeInContext(self.context),   // i32
                .Lit => c.LLVMInt1TypeInContext(self.context),     // bool (i1)
                .Cap => c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // null pointer
                else => c.LLVMInt64TypeInContext(self.context), // default to i64
            },
            else => c.LLVMInt64TypeInContext(self.context), // default for complex types
        };
    }

    fn generateExpression(self: *CodeGen, expr_ptr: *anyopaque) CodeGenError!c.LLVMValueRef {
        const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
        switch (expr.*) {
            .Literal => |literal| return try self.generateLiteral(literal),
            .Integer => |int| return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @bitCast(int), 0),
            .Float => |float| return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @bitCast(float), 0),
            .String => |str| return c.LLVMBuildGlobalStringPtr(self.builder, str.ptr, "str"),
            .Boolean => |bool_val| return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0),
            .Character => |char| return c.LLVMConstInt(c.LLVMInt8TypeInContext(self.context), char, 0),
            .Identifier => |ident| return try self.generateIdentifier(ident),
            .Variable => |var_name| return try self.generateIdentifier(var_name),
            .Binary => |binary| return try self.generateBinaryExpression(binary),
            .Unary => |unary| return try self.generateUnaryExpression(unary.*),
            .Call => |call| return try self.generateCallExpression(call),
            .FunctionCall => |func_call| return try self.generateFunctionCallExpression(func_call),
            .ArrayAccess => |array_access| return try self.generateArrayAccess(array_access),
            .MemberAccess => |member_access| return try self.generateMemberAccess(member_access.*),
            .Array => |array_expr| return try self.generateArrayExpression(array_expr.*),
            else => {
                std.debug.print("Warning: Unsupported expression type in codegen: {s}\n", .{@tagName(expr.*)});
                return CodeGenError.UnsupportedOperation;
            },
        }
    }
    
    fn generateLiteral(self: *CodeGen, literal: ast.Literal) CodeGenError!c.LLVMValueRef {
        switch (literal) {
            .Integer => |int| return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @bitCast(int), 0),
            .Float => |float| return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @bitCast(float), 0),
            .String => |str| return c.LLVMBuildGlobalStringPtr(self.builder, str.ptr, "str"),
            .Boolean => |bool_val| return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0),
            .Character => |char| return c.LLVMConstInt(c.LLVMInt8TypeInContext(self.context), char, 0),
            else => return CodeGenError.UnsupportedOperation,
        }
    }
    
    fn generateIdentifier(self: *CodeGen, ident: []const u8) CodeGenError!c.LLVMValueRef {
        if (self.variables.get(ident)) |var_ref| {
            const var_type = c.LLVMTypeOf(var_ref);
            const elem_type = c.LLVMGetElementType(var_type);
            return c.LLVMBuildLoad2(self.builder, elem_type, var_ref, "load");
        }
        return CodeGenError.UndefinedSymbol;
    }
    
    fn generateBinaryExpression(self: *CodeGen, binary: ast.BinaryExpression) CodeGenError!c.LLVMValueRef {
        const left = try self.generateExpression(binary.left);
        const right = try self.generateExpression(binary.right);
        const op = binary.operator;
        
        // Arithmetic operations
        if (std.mem.eql(u8, op, "+")) {
            return c.LLVMBuildAdd(self.builder, left, right, "add");
        } else if (std.mem.eql(u8, op, "-")) {
            return c.LLVMBuildSub(self.builder, left, right, "sub");
        } else if (std.mem.eql(u8, op, "*")) {
            return c.LLVMBuildMul(self.builder, left, right, "mul");
        } else if (std.mem.eql(u8, op, "/")) {
            // For now, assume signed division. Could be enhanced with type checking
            return c.LLVMBuildSDiv(self.builder, left, right, "div");
        } else if (std.mem.eql(u8, op, "%")) {
            return c.LLVMBuildSRem(self.builder, left, right, "mod");
        }
        
        // Comparison operations
        else if (std.mem.eql(u8, op, "==")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq");
        } else if (std.mem.eql(u8, op, "!=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "ne");
        } else if (std.mem.eql(u8, op, "<")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt");
        } else if (std.mem.eql(u8, op, "<=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, left, right, "le");
        } else if (std.mem.eql(u8, op, ">")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt");
        } else if (std.mem.eql(u8, op, ">=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, left, right, "ge");
        }
        
        // Logical operations
        else if (std.mem.eql(u8, op, "&&")) {
            return c.LLVMBuildAnd(self.builder, left, right, "and");
        } else if (std.mem.eql(u8, op, "||")) {
            return c.LLVMBuildOr(self.builder, left, right, "or");
        }
        
        // Bitwise operations
        else if (std.mem.eql(u8, op, "&")) {
            return c.LLVMBuildAnd(self.builder, left, right, "bitand");
        } else if (std.mem.eql(u8, op, "|")) {
            return c.LLVMBuildOr(self.builder, left, right, "bitor");
        } else if (std.mem.eql(u8, op, "^")) {
            return c.LLVMBuildXor(self.builder, left, right, "xor");
        } else if (std.mem.eql(u8, op, "<<")) {
            return c.LLVMBuildShl(self.builder, left, right, "shl");
        } else if (std.mem.eql(u8, op, ">>")) {
            return c.LLVMBuildAShr(self.builder, left, right, "shr");
        }
        
        return CodeGenError.UnsupportedOperation;
    }
    
    fn generateUnaryExpression(self: *CodeGen, unary: ast.UnaryExpression) CodeGenError!c.LLVMValueRef {
        const operand = try self.generateExpression(unary.operand);
        const op = unary.operator;
        
        if (std.mem.eql(u8, op, "-")) {
            return c.LLVMBuildNeg(self.builder, operand, "neg");
        } else if (std.mem.eql(u8, op, "!")) {
            return c.LLVMBuildNot(self.builder, operand, "not");
        } else if (std.mem.eql(u8, op, "~")) {
            return c.LLVMBuildNot(self.builder, operand, "bitnot");
        }
        
        return CodeGenError.UnsupportedOperation;
    }
    
    fn generateCallExpression(self: *CodeGen, call: ast.CallExpression) CodeGenError!c.LLVMValueRef {
        const func_expr: *ast.Expression = @ptrCast(@alignCast(call.function));
        
        // Handle direct function calls
        if (func_expr.* == .Identifier) {
            const func_name = func_expr.*.Identifier;
            
            // Handle built-in functions first
            if (try self.generateBuiltinCall(func_name, call.arguments)) |result| {
                return result;
            }
            
            // Handle user-defined functions
            if (self.functions.get(func_name)) |func| {
                var args = try self.allocator.alloc(c.LLVMValueRef, call.arguments.items.len);
                defer self.allocator.free(args);
                
                for (call.arguments.items, 0..) |arg_ptr, i| {
                    args[i] = try self.generateExpression(arg_ptr);
                }
                
                return c.LLVMBuildCall2(self.builder, c.LLVMTypeOf(func), func, args.ptr, @intCast(args.len), "call");
            }
            
            return CodeGenError.UndefinedSymbol;
        }
        
        return CodeGenError.InvalidFunction;
    }
    
    fn generateFunctionCallExpression(self: *CodeGen, func_call: ast.FunctionCallExpression) CodeGenError!c.LLVMValueRef {
        const func_expr = try self.generateExpression(func_call.function);
        
        var args = try self.allocator.alloc(c.LLVMValueRef, func_call.arguments.len);
        defer self.allocator.free(args);
        
        for (func_call.arguments, 0..) |arg_ptr, i| {
            args[i] = try self.generateExpression(arg_ptr);
        }
        
        // For indirect calls, we need the function type
        const func_type = c.LLVMTypeOf(func_expr);
        return c.LLVMBuildCall2(self.builder, func_type, func_expr, args.ptr, @intCast(args.len), "indirect_call");
    }
    
    fn generateBuiltinCall(self: *CodeGen, func_name: []const u8, args: ArrayList(*ast.Expression)) CodeGenError!?c.LLVMValueRef {
        // Handle CURSED built-in functions
        if (std.mem.eql(u8, func_name, "vibez.spill") or std.mem.eql(u8, func_name, "print")) {
            // For now, just return void. Full printf implementation would require more setup
            return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
        }
        
        if (std.mem.eql(u8, func_name, "len")) {
            if (args.items.len != 1) return CodeGenError.UnsupportedOperation;
            
            const arg_expr = try self.generateExpression(args.items[0]);
            // For arrays, we'd need to store length metadata. For now, return a placeholder
            _ = arg_expr;
            return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
        }
        
        return null; // Not a built-in function
    }
    
    fn generateArrayAccess(self: *CodeGen, array_access: ast.ArrayAccessExpression) CodeGenError!c.LLVMValueRef {
        const array = try self.generateExpression(array_access.array);
        const index = try self.generateExpression(array_access.index);
        
        // For now, treat arrays as pointers and use GEP
        const indices = [_]c.LLVMValueRef{index};
        const gep = c.LLVMBuildGEP2(self.builder, c.LLVMInt64TypeInContext(self.context), array, &indices, indices.len, "arrayaccess");
        return c.LLVMBuildLoad2(self.builder, c.LLVMInt64TypeInContext(self.context), gep, "load");
    }
    
    fn generateMemberAccess(self: *CodeGen, member_access: ast.MemberAccessExpression) CodeGenError!c.LLVMValueRef {
        const object = try self.generateExpression(member_access.object);
        const property = member_access.property;
        
        // For now, this is a placeholder. Proper struct member access would require type information
        _ = object;
        _ = property;
        return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 0);
    }
    
    fn generateArrayExpression(self: *CodeGen, array_expr: ast.ArrayExpression) CodeGenError!c.LLVMValueRef {
        // For now, create a simple array allocation
        const array_size = array_expr.elements.items.len;
        const element_type = c.LLVMInt64TypeInContext(self.context);
        
        var values = try self.allocator.alloc(c.LLVMValueRef, array_size);
        defer self.allocator.free(values);
        
        for (array_expr.elements.items, 0..) |elem_ptr, i| {
            values[i] = try self.generateExpression(elem_ptr);
        }
        
        return c.LLVMConstArray(element_type, values.ptr, @intCast(values.len));
    }

    pub fn generateIR(self: *CodeGen) ![]const u8 {
        if (self.module == null) return CodeGenError.CompilationFailed;
        
        const ir_str = c.LLVMPrintModuleToString(self.module);
        defer c.LLVMDisposeMessage(ir_str);
        
        return self.allocator.dupe(u8, std.mem.span(ir_str));
    }

    pub fn writeToFile(self: *CodeGen, filename: []const u8) !void {
        if (self.module == null) return CodeGenError.CompilationFailed;
        
        var error_msg: [*c]u8 = undefined;
        const result = c.LLVMPrintModuleToFile(self.module, filename.ptr, &error_msg);
        if (result != 0) {
            defer c.LLVMDisposeMessage(error_msg);
            return CodeGenError.CompilationFailed;
        }
    }
};

test "codegen basic" {
    const allocator = std.testing.allocator;
    
    var codegen = CodeGen.init(allocator);
    defer codegen.deinit();
    
    // Test basic initialization
    try std.testing.expect(codegen.context != null);
}

test "codegen simple program" {
    const allocator = std.testing.allocator;
    
    var codegen = CodeGen.init(allocator);
    defer codegen.deinit();
    
    // Create a simple program with a variable declaration
    var program = ast.Program.init(allocator);
    defer program.deinit(allocator);
    
    // Test that we can initialize without errors
    try std.testing.expect(program.statements.items.len == 0);
}
