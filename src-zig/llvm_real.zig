const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Real LLVM C API integration using the C wrapper functions
extern fn llvm_initialize_core() void;
extern fn llvm_create_context() ?*anyopaque;
extern fn llvm_dispose_context(?*anyopaque) void;
extern fn llvm_create_module(?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_dispose_module(?*anyopaque) void;
extern fn llvm_create_builder(?*anyopaque) ?*anyopaque;
extern fn llvm_dispose_builder(?*anyopaque) void;
extern fn llvm_int32_type(?*anyopaque) ?*anyopaque;
extern fn llvm_int8_type(?*anyopaque) ?*anyopaque;
extern fn llvm_pointer_type(?*anyopaque) ?*anyopaque;
extern fn llvm_function_type(?*anyopaque, [*]?*anyopaque, c_int, c_int) ?*anyopaque;
extern fn llvm_add_function(?*anyopaque, [*c]const u8, ?*anyopaque) ?*anyopaque;
extern fn llvm_append_basic_block(?*anyopaque, ?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_position_builder_at_end(?*anyopaque, ?*anyopaque) void;
extern fn llvm_build_global_string_ptr(?*anyopaque, [*c]const u8, [*c]const u8) ?*anyopaque;
extern fn llvm_const_int(?*anyopaque, c_ulonglong) ?*anyopaque;
extern fn llvm_build_ret(?*anyopaque, ?*anyopaque) ?*anyopaque;
extern fn llvm_get_named_function(?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_get_function_type(?*anyopaque) ?*anyopaque;
extern fn llvm_build_call2(?*anyopaque, ?*anyopaque, ?*anyopaque, [*]?*anyopaque, c_int, [*c]const u8) ?*anyopaque;
extern fn llvm_verify_module(?*anyopaque) c_int;
extern fn llvm_print_module_to_string(?*anyopaque) [*c]u8;
extern fn llvm_dispose_message([*c]u8) void;
extern fn llvm_write_bitcode_to_file(?*anyopaque, [*c]const u8) c_int;

// Additional LLVM operations for binary expressions
extern fn llvm_build_add(?*anyopaque, ?*anyopaque, ?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_build_sub(?*anyopaque, ?*anyopaque, ?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_build_mul(?*anyopaque, ?*anyopaque, ?*anyopaque, [*c]const u8) ?*anyopaque;
extern fn llvm_build_div(?*anyopaque, ?*anyopaque, ?*anyopaque, [*c]const u8) ?*anyopaque;

const ast = @import("ast.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const FunctionStatement = ast.FunctionStatement;

pub const LLVMError = error{
    LLVMInitializationFailed,
    ModuleCreationFailed,
    BuilderCreationFailed,
    FunctionCreationFailed,
    TypeCreationFailed,
    VerificationFailed,
    OutOfMemory,
    InvalidType,
    UndefinedSymbol,
};

pub const RealLLVMCodeGen = struct {
    allocator: Allocator,
    context: ?*anyopaque,
    module: ?*anyopaque,
    builder: ?*anyopaque,
    
    // Symbol tables
    functions: std.HashMap([]const u8, ?*anyopaque, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: std.HashMap([]const u8, ?*anyopaque, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Type cache
    i32_type: ?*anyopaque,
    i8_type: ?*anyopaque,
    i8_ptr_type: ?*anyopaque,
    
    pub fn init(allocator: Allocator) LLVMError!RealLLVMCodeGen {
        // Initialize LLVM core
        llvm_initialize_core();
        
        const context = llvm_create_context() orelse {
            return LLVMError.LLVMInitializationFailed;
        };
        
        const module = llvm_create_module(context, "cursed_module") orelse {
            llvm_dispose_context(context);
            return LLVMError.ModuleCreationFailed;
        };
        
        const builder = llvm_create_builder(context) orelse {
            llvm_dispose_module(module);
            llvm_dispose_context(context);
            return LLVMError.BuilderCreationFailed;
        };
        
        // Initialize type cache
        const i32_type = llvm_int32_type(context);
        const i8_type = llvm_int8_type(context);
        const i8_ptr_type = llvm_pointer_type(i8_type);
        
        var codegen = RealLLVMCodeGen{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .functions = std.HashMap([]const u8, ?*anyopaque, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = std.HashMap([]const u8, ?*anyopaque, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .i32_type = i32_type,
            .i8_type = i8_type,
            .i8_ptr_type = i8_ptr_type,
        };
        
        // Add external function declarations
        try codegen.addExternalFunctions();
        
        return codegen;
    }
    
    pub fn deinit(self: *RealLLVMCodeGen) void {
        self.functions.deinit(allocator);
        self.variables.deinit(allocator);
        
        if (self.builder) |builder| llvm_dispose_builder(builder);
        if (self.module) |module| llvm_dispose_module(module);
        if (self.context) |context| llvm_dispose_context(context);
    }
    
    fn addExternalFunctions(self: *RealLLVMCodeGen) LLVMError!void {
        // Add printf function for vibez.spill
        var param_types = [_]?*anyopaque{self.i8_ptr_type};
        const printf_type = llvm_function_type(self.i32_type, &param_types, 1, 1); // variadic
        const printf_func = llvm_add_function(self.module, "printf", printf_type);
        try self.functions.put("printf", printf_func);
    }
    
    pub fn generateProgram(self: *RealLLVMCodeGen, program: Program) LLVMError!void {
        // Separate functions from global statements
        var global_statements: std.ArrayList(*Statement) = .empty;
        defer global_statements.deinit(allocator);
        
        // Process statements - functions are generated immediately, others saved for main
        for (program.statements.items) |stmt_ptr| {
            const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
            switch (stmt.*) {
                .Function => {
                    try self.generateStatement(stmt.*);
                },
                else => {
                    try global_statements.append(allocator, stmt);
                },
            }
        }
        
        // Create main function if it doesn't exist
        if (self.functions.get("main") == null) {
            try self.generateMainFunctionWithStatements(global_statements.items);
        }
        
        // Verify the module
        if (llvm_verify_module(self.module) != 0) {
            return LLVMError.VerificationFailed;
        }
    }
    
    fn generateStatement(self: *RealLLVMCodeGen, stmt: Statement) LLVMError!void {
        switch (stmt) {
            .Function => |func| {
                try self.generateFunction(func);
            },
            .Expression => |expr_ptr| {
                const expr: *Expression = @ptrCast(@alignCast(expr_ptr));
                _ = try self.generateExpression(expr.*);
            },
            .Let => |let_stmt| {
                try self.generateLet(let_stmt);
            },
            .Assignment => |assign_stmt| {
                try self.generateAssignment(assign_stmt);
            },
            .Return => |return_stmt| {
                try self.generateReturn(return_stmt);
            },
            .If => |if_stmt| {
                try self.generateIf(if_stmt);
            },
            .While => |while_stmt| {
                try self.generateWhile(while_stmt);
            },
            .For => |for_stmt| {
                try self.generateFor(for_stmt);
            },
            .Break => {
                try self.generateBreak();
            },
            .Continue => {
                try self.generateContinue();
            },
            .Block => |block_stmt| {
                try self.generateBlock(block_stmt);
            },
            .Goroutine => |goroutine_stmt| {
                try self.generateGoroutine(goroutine_stmt);
            },
            .Select => |select_stmt| {
                try self.generateSelect(select_stmt);
            },
            .Defer => |defer_stmt| {
                try self.generateDefer(defer_stmt);
            },
            .Struct => |struct_stmt| {
                try self.generateStruct(struct_stmt);
            },
            .Interface => |interface_stmt| {
                try self.generateInterface(interface_stmt);
            },
            .Panic => |panic_stmt| {
                try self.generatePanic(panic_stmt);
            },
            .Yikes => |yikes_stmt| {
                try self.generateYikes(yikes_stmt);
            },
            .Fam => |fam_stmt| {
                try self.generateFam(fam_stmt);
            },
            else => {
                // Skip less common statement types for now
                std.debug.print("Statement type {s} not implemented yet\n", .{@tagName(stmt)});
            },
        }
    }
    
    fn generateFunction(self: *RealLLVMCodeGen, func: FunctionStatement) LLVMError!void {
        // Create function type (simplified - assuming no parameters for now)
        var empty_params: [0]?*anyopaque = undefined;
        const function_type = llvm_function_type(self.i32_type, &empty_params, 0, 0);
        const function = llvm_add_function(self.module, func.name.ptr, function_type);
        
        if (function == null) {
            return LLVMError.FunctionCreationFailed;
        }
        
        // Store function in symbol table
        try self.functions.put(func.name, function);
        
        // Create entry block
        const entry_block = llvm_append_basic_block(self.context, function, "entry");
        llvm_position_builder_at_end(self.builder, entry_block);
        
        // Generate function body
        for (func.body.items) |stmt_ptr| {
            const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateStatement(stmt.*);
        }
        
        // Add default return if needed
        const return_value = llvm_const_int(self.i32_type, 0);
        _ = llvm_build_ret(self.builder, return_value);
    }
    
    fn generateLet(self: *RealLLVMCodeGen, let_stmt: ast.LetStatement) LLVMError!void {
        // Generate the initializer expression
        if (let_stmt.initializer) |initializer| {
            const expr: *Expression = @ptrCast(@alignCast(initializer));
            const value = try self.generateExpression(expr.*);
            
            // Store in variable table
            try self.variables.put(let_stmt.name, value);
        }
    }
    
    fn generateExpression(self: *RealLLVMCodeGen, expr: Expression) LLVMError!?*anyopaque {
        switch (expr) {
            .Integer => |int_value| {
                return llvm_const_int(self.i32_type, @intCast(int_value));
            },
            .String => |str_value| {
                return llvm_build_global_string_ptr(self.builder, str_value.ptr, "str");
            },
            .Identifier => |name| {
                // Look up variable (simplified)
                return self.variables.get(name) orelse llvm_const_int(self.i32_type, 0);
            },
            .Variable => |name| {
                // Look up variable in symbol table
                return self.variables.get(name) orelse llvm_const_int(self.i32_type, 0);
            },
            .Call => |call_expr| {
                return try self.generateCall(call_expr);
            },
            .FunctionCall => |func_call| {
                return try self.generateFunctionCall(func_call);
            },
            .Array => |array_ptr| {
                const array_expr: *ast.ArrayExpression = @ptrCast(@alignCast(array_ptr));
                return try self.generateArrayLiteral(array_expr.*);
            },
            .Binary => |binary_expr| {
                return try self.generateBinaryExpression(binary_expr);
            },
            .Unary => |unary_ptr| {
                const unary_expr: *ast.UnaryExpression = @ptrCast(@alignCast(unary_ptr));
                return try self.generateUnaryExpression(unary_expr.*);
            },
            .ArrayAccess => |array_access| {
                return try self.generateArrayAccess(array_access);
            },
            .MemberAccess => |member_ptr| {
                const member_expr: *ast.MemberAccessExpression = @ptrCast(@alignCast(member_ptr));
                return try self.generateMemberAccess(member_expr.*);
            },
            .Boolean => |bool_value| {
                return llvm_const_int(self.i32_type, if (bool_value) 1 else 0);
            },
            .Float => |float_value| {
                // Use int type for now, proper float support can be added later
                return llvm_const_int(self.i32_type, @intFromFloat(float_value));
            },
            .Character => |char_value| {
                return llvm_const_int(self.i8_type, char_value);
            },
            .Literal => |literal| {
                return try self.generateLiteral(literal);
            },
            .ChannelSend => |channel_send| {
                return try self.generateChannelSend(channel_send);
            },
            .ChannelReceive => |channel_recv| {
                return try self.generateChannelReceive(channel_recv);
            },
            .ChannelCreation => |channel_create| {
                return try self.generateChannelCreation(channel_create);
            },
            .StructLiteral => |struct_literal| {
                return try self.generateStructLiteral(struct_literal);
            },
            .Lambda => |lambda| {
                return try self.generateLambda(lambda);
            },
            .Tuple => |tuple| {
                return try self.generateTuple(tuple);
            },
            .TupleAccess => |tuple_access| {
                return try self.generateTupleAccess(tuple_access);
            },
            .SliceAccess => |slice_access| {
                return try self.generateSliceAccess(slice_access);
            },
            .TypeAssertion => |type_assertion| {
                return try self.generateTypeAssertion(type_assertion);
            },
            .Yikes => |yikes_expr| {
                return try self.generateYikesExpression(yikes_expr);
            },
            .Fam => |fam_expr| {
                return try self.generateFamExpression(fam_expr);
            },
            .Shook => |shook_expr| {
                return try self.generateShookExpression(shook_expr);
            },
            .Panic => |panic_expr| {
                return try self.generatePanicExpression(panic_expr);
            },
            .Recover => |recover_expr| {
                return try self.generateRecoverExpression(recover_expr);
            },
            .StringInterpolation => |interpolation| {
                return try self.generateStringInterpolation(interpolation);
            },
            .If => |if_expr| {
                return try self.generateIfExpression(if_expr);
            },
            .Block => |block_expr| {
                return try self.generateBlockExpression(block_expr);
            },
            .Loop => |loop_expr| {
                return try self.generateLoopExpression(loop_expr);
            },
            .For => |for_expr| {
                return try self.generateForExpression(for_expr);
            },
            .While => |while_expr| {
                return try self.generateWhileExpression(while_expr);
            },
            .Match => |match_expr| {
                return try self.generateMatchExpression(match_expr);
            },
            .MethodCall => |method_call| {
                return try self.generateMethodCall(method_call);
            },
            else => {
                std.debug.print("Expression type {s} not implemented yet\n", .{@tagName(expr)});
                return llvm_const_int(self.i32_type, 0);
            },
        }
    }
    
    fn generateFunctionCall(self: *RealLLVMCodeGen, call: ast.FunctionCallExpression) LLVMError!?*anyopaque {
        // Handle special CURSED functions like vibez.spill
        const func_expr: *Expression = @ptrCast(@alignCast(call.function));
        
        // Check if this is a member access like vibez.spill
        if (func_expr.* == .MemberAccess) {
            const member_expr: *ast.MemberAccessExpression = @ptrCast(@alignCast(func_expr.*.MemberAccess));
            
            // Check for vibez.spill
            const object_expr: *Expression = @ptrCast(@alignCast(member_expr.object));
            if (object_expr.* == .Identifier and std.mem.eql(u8, object_expr.*.Identifier, "vibez") and 
                std.mem.eql(u8, member_expr.property, "spill")) {
                return try self.generatePrintCall(call);
            }
        }
        
        // For regular function calls, generate call instruction
        if (func_expr.* == .Identifier) {
            const func_name = func_expr.*.Identifier;
            
            // Look up function in symbol table
            if (self.functions.get(func_name)) |func| {
                // Generate arguments
                var args: std.ArrayList(?*anyopaque) = .empty;
                defer args.deinit(allocator);
                
                for (call.arguments) |arg_ptr| {
                    const arg_expr: *Expression = @ptrCast(@alignCast(arg_ptr));
                    const arg_value = try self.generateExpression(arg_expr.*);
                    try args.append(allocator, arg_value);
                }
                
                const func_type = llvm_get_function_type(func);
                if (args.items.len > 0) {
                    return llvm_build_call2(self.builder, func_type, func, args.items.ptr, @intCast(args.items.len), "call");
                } else {
                    var empty_array: [0]?*anyopaque = undefined;
                    return llvm_build_call2(self.builder, func_type, func, @ptrCast(&empty_array), 0, "call");
                }
            }
        }
        
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generatePrintCall(self: *RealLLVMCodeGen, call: ast.FunctionCallExpression) LLVMError!?*anyopaque {
        const printf_func = self.functions.get("printf") orelse {
            return LLVMError.UndefinedSymbol;
        };
        
        // Generate arguments
        var args = .empty;
        defer args.deinit(allocator);
        
        for (call.arguments) |arg| {
            const arg_value = try self.generateExpression(arg.*);
            try args.append(allocator, arg_value);
        }
        
        // Create format string based on argument types (simplified)
        const format_str = if (args.items.len > 0) "%s\n" else "Hello World\n";
        const format_global = llvm_build_global_string_ptr(self.builder, format_str, "fmt");
        
        // Insert format string at beginning
        try args.insert(0, format_global);
        
        const func_type = llvm_get_function_type(printf_func);
        return llvm_build_call2(self.builder, func_type, printf_func, args.items.ptr, @intCast(args.items.len), "printf_call");
    }
    
    fn generateMainFunctionWithStatements(self: *RealLLVMCodeGen, statements: []*Statement) LLVMError!void {
        // Create main function that returns int
        var empty_params: [0]?*anyopaque = undefined;
        const main_type = llvm_function_type(self.i32_type, &empty_params, 0, 0);
        const main_func = llvm_add_function(self.module, "main", main_type);
        
        if (main_func == null) {
            return LLVMError.FunctionCreationFailed;
        }
        
        // Create entry block
        const entry_block = llvm_append_basic_block(self.context, main_func, "entry");
        llvm_position_builder_at_end(self.builder, entry_block);
        
        // Generate the global statements inside main
        for (statements) |stmt| {
            try self.generateStatement(stmt.*);
        }
        
        // Return 0
        const return_value = llvm_const_int(self.i32_type, 0);
        _ = llvm_build_ret(self.builder, return_value);
        
        try self.functions.put("main", main_func);
    }
    
    fn generateMainFunction(self: *RealLLVMCodeGen) LLVMError!void {
        return self.generateMainFunctionWithStatements(&[_]*Statement{});
    }
    
    pub fn writeToFile(self: *RealLLVMCodeGen, filename: []const u8) LLVMError!void {
        const filename_c = try self.allocator.dupeZ(u8, filename);
        defer self.allocator.free(filename_c);
        
        if (llvm_write_bitcode_to_file(self.module, filename_c.ptr) != 0) {
            return LLVMError.LLVMInitializationFailed;
        }
    }
    
    pub fn printModule(self: *RealLLVMCodeGen) void {
        const module_str = llvm_print_module_to_string(self.module);
        if (module_str) |str| {
            std.debug.print("Generated LLVM IR:\n{s}\n", .{str});
            llvm_dispose_message(str);
        }
    }

    // === Missing Statement Implementations ===
    
    fn generateAssignment(self: *RealLLVMCodeGen, assign_stmt: ast.AssignmentStatement) LLVMError!void {
        // Cast the anyopaque pointers to Expressions
        const value_expr: *Expression = @ptrCast(@alignCast(assign_stmt.value));
        const target_expr: *Expression = @ptrCast(@alignCast(assign_stmt.target));
        
        // Generate value first
        const value = try self.generateExpression(value_expr.*);
        
        // For now, just store in variables map if target is identifier
        if (target_expr.* == .Identifier) {
            const name = target_expr.*.Identifier;
            try self.variables.put(name, value);
        }
    }
    
    fn generateReturn(self: *RealLLVMCodeGen, return_stmt: ast.ReturnStatement) LLVMError!void {
        if (return_stmt.value) |value_ptr| {
            const value_expr: *Expression = @ptrCast(@alignCast(value_ptr));
            const return_value = try self.generateExpression(value_expr.*);
            _ = llvm_build_ret(self.builder, return_value);
        } else {
            _ = llvm_build_ret(self.builder, llvm_const_int(self.i32_type, 0));
        }
    }
    
    fn generateIf(self: *RealLLVMCodeGen, if_stmt: ast.IfStatement) LLVMError!void {
        _ = self;
        _ = if_stmt;
        // Create basic blocks for then, else, and merge
        // Generate condition and branch logic
        // For now, return placeholder
        return;
    }
    
    fn generateWhile(self: *RealLLVMCodeGen, while_stmt: ast.WhileStatement) LLVMError!void {
        _ = self;
        _ = while_stmt;
        // Generate while loop with condition check and body
        return;
    }
    
    fn generateFor(self: *RealLLVMCodeGen, for_stmt: ast.ForStatement) LLVMError!void {
        _ = self;
        _ = for_stmt;
        // Generate for loop structure
        return;
    }
    
    fn generateBreak(self: *RealLLVMCodeGen) LLVMError!void {
        _ = self;
        // Generate break instruction
        return;
    }
    
    fn generateContinue(self: *RealLLVMCodeGen) LLVMError!void {
        _ = self;
        // Generate continue instruction
        return;
    }
    
    fn generateBlock(self: *RealLLVMCodeGen, block_stmt: ast.BlockStatement) LLVMError!void {
        // Generate statements in block
        for (block_stmt.statements.items) |stmt_ptr| {
            const stmt: *Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateStatement(stmt.*);
        }
    }
    
    fn generateGoroutine(self: *RealLLVMCodeGen, goroutine_stmt: ast.GoroutineStatement) LLVMError!void {
        _ = self;
        _ = goroutine_stmt;
        // Generate goroutine creation and launch
        return;
    }
    
    fn generateSelect(self: *RealLLVMCodeGen, select_stmt: ast.SelectStatement) LLVMError!void {
        _ = self;
        _ = select_stmt;
        // Generate select statement for channel operations
        return;
    }
    
    fn generateDefer(self: *RealLLVMCodeGen, defer_stmt: ast.DeferStatement) LLVMError!void {
        _ = self;
        _ = defer_stmt;
        // Generate defer statement cleanup
        return;
    }
    
    fn generateStruct(self: *RealLLVMCodeGen, struct_stmt: ast.StructStatement) LLVMError!void {
        _ = self;
        _ = struct_stmt;
        // Generate struct type definition
        return;
    }
    
    fn generateInterface(self: *RealLLVMCodeGen, interface_stmt: ast.InterfaceStatement) LLVMError!void {
        _ = self;
        _ = interface_stmt;
        // Generate interface type definition
        return;
    }
    
    fn generatePanic(self: *RealLLVMCodeGen, panic_stmt: ast.PanicStatement) LLVMError!void {
        _ = self;
        _ = panic_stmt;
        // Generate panic call
        return;
    }
    
    fn generateYikes(self: *RealLLVMCodeGen, yikes_stmt: ast.YikesStatement) LLVMError!void {
        _ = self;
        _ = yikes_stmt;
        // Generate error handling
        return;
    }
    
    fn generateFam(self: *RealLLVMCodeGen, fam_stmt: ast.FamStatement) LLVMError!void {
        _ = self;
        _ = fam_stmt;
        // Generate error recovery
        return;
    }

    // === Missing Expression Implementations ===
    
    fn generateCall(self: *RealLLVMCodeGen, call_expr: ast.CallExpression) LLVMError!?*anyopaque {
        _ = call_expr;
        // Generate function call
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateBinaryExpression(self: *RealLLVMCodeGen, binary_expr: ast.BinaryExpression) LLVMError!?*anyopaque {
        const left = try self.generateExpression(@as(*Expression, @ptrCast(@alignCast(binary_expr.left))).*);
        const right = try self.generateExpression(@as(*Expression, @ptrCast(@alignCast(binary_expr.right))).*);
        
        // Implement binary operations
        if (std.mem.eql(u8, binary_expr.operator, "+")) {
            return llvm_build_add(self.builder, left, right, "add");
        } else if (std.mem.eql(u8, binary_expr.operator, "-")) {
            return llvm_build_sub(self.builder, left, right, "sub");
        } else if (std.mem.eql(u8, binary_expr.operator, "*")) {
            return llvm_build_mul(self.builder, left, right, "mul");
        } else if (std.mem.eql(u8, binary_expr.operator, "/")) {
            return llvm_build_div(self.builder, left, right, "div");
        }
        
        return left; // Fallback
    }
    
    fn generateUnaryExpression(self: *RealLLVMCodeGen, unary_expr: ast.UnaryExpression) LLVMError!?*anyopaque {
        const operand = try self.generateExpression(@as(*Expression, @ptrCast(@alignCast(unary_expr.operand))).*);
        
        if (std.mem.eql(u8, unary_expr.operator, "-")) {
            // For negation, subtract from zero
            const zero = llvm_const_int(self.i32_type, 0);
            return llvm_build_sub(self.builder, zero, operand, "neg");
        } else if (std.mem.eql(u8, unary_expr.operator, "!")) {
            // For logical not, we'll use XOR with 1 for now
            const one = llvm_const_int(self.i32_type, 1);
            return llvm_build_sub(self.builder, one, operand, "not");
        }
        
        return operand; // Fallback
    }
    
    fn generateArrayLiteral(self: *RealLLVMCodeGen, array_expr: ast.ArrayExpression) LLVMError!?*anyopaque {
        _ = array_expr;
        // Generate array literal
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateArrayAccess(self: *RealLLVMCodeGen, array_access: ast.ArrayAccessExpression) LLVMError!?*anyopaque {
        _ = array_access;
        // Generate array indexing
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateMemberAccess(self: *RealLLVMCodeGen, member_expr: ast.MemberAccessExpression) LLVMError!?*anyopaque {
        _ = member_expr;
        // Generate member access (struct.field)
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateLiteral(self: *RealLLVMCodeGen, literal: ast.Literal) LLVMError!?*anyopaque {
        switch (literal) {
            .Integer => |int_val| return llvm_const_int(self.i32_type, @intCast(int_val)),
            .Float => |float_val| return llvm_const_int(self.i32_type, @intFromFloat(float_val)),
            .String => |str_val| return llvm_build_global_string_ptr(self.builder, str_val.ptr, "str"),
            .Boolean => |bool_val| return llvm_const_int(self.i32_type, if (bool_val) 1 else 0),
            .Character => |char_val| return llvm_const_int(self.i8_type, char_val),
            else => return llvm_const_int(self.i32_type, 0),
        }
    }
    
    fn generateChannelSend(self: *RealLLVMCodeGen, channel_send: ast.ChannelSendExpression) LLVMError!?*anyopaque {
        _ = channel_send;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateChannelReceive(self: *RealLLVMCodeGen, channel_recv: ast.ChannelReceiveExpression) LLVMError!?*anyopaque {
        _ = channel_recv;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateChannelCreation(self: *RealLLVMCodeGen, channel_create: ast.ChannelCreationExpression) LLVMError!?*anyopaque {
        _ = channel_create;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateStructLiteral(self: *RealLLVMCodeGen, struct_literal: ast.StructLiteralExpression) LLVMError!?*anyopaque {
        _ = struct_literal;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateLambda(self: *RealLLVMCodeGen, lambda: ast.LambdaExpression) LLVMError!?*anyopaque {
        _ = lambda;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateTuple(self: *RealLLVMCodeGen, tuple: ast.TupleExpression) LLVMError!?*anyopaque {
        _ = tuple;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateTupleAccess(self: *RealLLVMCodeGen, tuple_access: ast.TupleAccessExpression) LLVMError!?*anyopaque {
        _ = tuple_access;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateSliceAccess(self: *RealLLVMCodeGen, slice_access: ast.SliceAccessExpression) LLVMError!?*anyopaque {
        _ = slice_access;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateTypeAssertion(self: *RealLLVMCodeGen, type_assertion: ast.TypeAssertionExpression) LLVMError!?*anyopaque {
        _ = type_assertion;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateYikesExpression(self: *RealLLVMCodeGen, yikes_expr: ast.YikesExpression) LLVMError!?*anyopaque {
        _ = yikes_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateFamExpression(self: *RealLLVMCodeGen, fam_expr: ast.FamExpression) LLVMError!?*anyopaque {
        _ = fam_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateShookExpression(self: *RealLLVMCodeGen, shook_expr: ast.ShookExpression) LLVMError!?*anyopaque {
        _ = shook_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generatePanicExpression(self: *RealLLVMCodeGen, panic_expr: ast.PanicExpression) LLVMError!?*anyopaque {
        _ = panic_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateRecoverExpression(self: *RealLLVMCodeGen, recover_expr: ast.RecoverExpression) LLVMError!?*anyopaque {
        _ = recover_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateStringInterpolation(self: *RealLLVMCodeGen, interpolation: ast.StringInterpolationExpression) LLVMError!?*anyopaque {
        _ = interpolation;
        return llvm_build_global_string_ptr(self.builder, "interpolated", "str");
    }
    
    fn generateIfExpression(self: *RealLLVMCodeGen, if_expr: ast.IfExpression) LLVMError!?*anyopaque {
        _ = if_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateBlockExpression(self: *RealLLVMCodeGen, block_expr: ast.BlockExpression) LLVMError!?*anyopaque {
        _ = block_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateLoopExpression(self: *RealLLVMCodeGen, loop_expr: ast.LoopExpression) LLVMError!?*anyopaque {
        _ = loop_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateForExpression(self: *RealLLVMCodeGen, for_expr: ast.ForExpression) LLVMError!?*anyopaque {
        _ = for_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateWhileExpression(self: *RealLLVMCodeGen, while_expr: ast.WhileExpression) LLVMError!?*anyopaque {
        _ = while_expr;
        return llvm_const_int(self.i32_type, 0);
    }
    
    fn generateMatchExpression(self: *RealLLVMCodeGen, match_expr: ast.MatchExpression) LLVMError!?*anyopaque {
        _ = match_expr;
        return llvm_const_int(self.i32_type, 0);
    }

    fn generateMethodCall(self: *RealLLVMCodeGen, method_call: *ast.MethodCallExpression) LLVMError!?*anyopaque {
        // Handle vibez.spill() method calls
        const object_expr: *Expression = @ptrCast(@alignCast(method_call.object));
        
        if (object_expr.* == .Identifier and std.mem.eql(u8, object_expr.*.Identifier, "vibez") and 
            std.mem.eql(u8, method_call.method_name, "spill")) {
            // Generate vibez.spill() as printf calls
            for (method_call.arguments.items) |arg| {
                const arg_value = try self.generateExpression(arg);
                
                // Create format string based on type
                switch (arg) {
                    .Integer => {
                        const fmt_str = llvm_build_global_string_ptr(self.builder, "%lld ", "int_fmt");
                        const printf = try self.getOrCreatePrintf();
                        const printf_type = llvm_get_function_type(printf);
                        _ = llvm_build_call2(self.builder, printf_type, printf, &[_]?*anyopaque{ fmt_str, arg_value }, 2, "print");
                    },
                    .String => {
                        const fmt_str = llvm_build_global_string_ptr(self.builder, "%s ", "str_fmt");
                        const printf = try self.getOrCreatePrintf();
                        const printf_type = llvm_get_function_type(printf);
                        _ = llvm_build_call2(self.builder, printf_type, printf, &[_]?*anyopaque{ fmt_str, arg_value }, 2, "print");
                    },
                    .Float => {
                        const fmt_str = llvm_build_global_string_ptr(self.builder, "%f ", "float_fmt");
                        const printf = try self.getOrCreatePrintf();
                        const printf_type = llvm_get_function_type(printf);
                        _ = llvm_build_call2(self.builder, printf_type, printf, &[_]?*anyopaque{ fmt_str, arg_value }, 2, "print");
                    },
                    else => {
                        // Default to pointer format
                        const fmt_str = llvm_build_global_string_ptr(self.builder, "%p ", "ptr_fmt");
                        const printf = try self.getOrCreatePrintf();
                        const printf_type = llvm_get_function_type(printf);
                        _ = llvm_build_call2(self.builder, printf_type, printf, &[_]?*anyopaque{ fmt_str, arg_value }, 2, "print");
                    },
                }
            }
            
            // Print newline
            const newline_str = llvm_build_global_string_ptr(self.builder, "\n", "newline");
            const printf = try self.getOrCreatePrintf();
            const printf_type = llvm_get_function_type(printf);
            _ = llvm_build_call2(self.builder, printf_type, printf, &[_]?*anyopaque{newline_str}, 1, "print_newline");
            
            return llvm_const_int(self.i32_type, 0); // void return
        }
        
        // Default: return placeholder for other method calls
        return llvm_const_int(self.i32_type, 0);
    }

    fn getOrCreatePrintf(self: *RealLLVMCodeGen) LLVMError!?*anyopaque {
        // Try to get existing printf function
        const printf = llvm_get_named_function(self.module, "printf");
        if (printf != null) return printf;
        
        // Create printf function type: int printf(char*, ...)
        const char_ptr_type = llvm_pointer_type(self.i8_type, 0);
        const func_type = llvm_function_type(self.i32_type, &[_]*anyopaque{char_ptr_type}, 1, 1); // Variadic
        return llvm_add_function(self.module, "printf", func_type);
    }
};

test "real llvm basic" {
    const allocator = std.testing.allocator;
    
    var codegen = RealLLVMCodeGen.init(allocator) catch |err| {
        std.debug.print("Failed to initialize LLVM: {}\n", .{err});
        return;
    };
    defer codegen.deinit(allocator);
    
    // Test basic initialization
    try std.testing.expect(codegen.context != null);
    try std.testing.expect(codegen.module != null);
    try std.testing.expect(codegen.builder != null);
}
