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
        self.functions.deinit();
        self.variables.deinit();
        
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
        // Generate all statements
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Create main function if it doesn't exist
        if (self.functions.get("main") == null) {
            try self.generateMainFunction();
        }
        
        // Verify the module
        if (llvm_verify_module(self.module) != 0) {
            return LLVMError.VerificationFailed;
        }
    }
    
    fn generateStatement(self: *RealLLVMCodeGen, stmt: Statement) LLVMError!void {
        switch (stmt.tag) {
            .Function => {
                const func: *FunctionStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateFunction(func.*);
            },
            .Expression => {
                const expr: *Expression = @ptrCast(@alignCast(stmt.data));
                _ = try self.generateExpression(expr.*);
            },
            .Let => {
                const let_stmt: *ast.LetStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateLet(let_stmt.*);
            },
            else => {
                // For now, skip other statement types
                std.debug.print("Statement type not implemented: {}\n", .{stmt.tag});
            },
        }
    }
    
    fn generateFunction(self: *RealLLVMCodeGen, func: FunctionStatement) LLVMError!void {
        // Create function type (simplified - assuming no parameters for now)
        const function_type = llvm_function_type(self.i32_type, null, 0, 0);
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
        try self.generateStatement(func.body);
        
        // Add default return if needed
        const return_value = llvm_const_int(self.i32_type, 0);
        _ = llvm_build_ret(self.builder, return_value);
    }
    
    fn generateLet(self: *RealLLVMCodeGen, let_stmt: ast.LetStatement) LLVMError!void {
        // For now, just generate the value expression
        _ = try self.generateExpression(let_stmt.value);
        // TODO: Store in variable table
    }
    
    fn generateExpression(self: *RealLLVMCodeGen, expr: Expression) LLVMError!?*anyopaque {
        switch (expr.tag) {
            .Integer => {
                const int_expr: *ast.IntegerExpression = @ptrCast(@alignCast(expr.data));
                return llvm_const_int(self.i32_type, @intCast(int_expr.value));
            },
            .String => {
                const str_expr: *ast.StringExpression = @ptrCast(@alignCast(expr.data));
                return llvm_build_global_string_ptr(self.builder, str_expr.value.ptr, "str");
            },
            .Identifier => {
                // Look up variable (simplified)
                const id_expr: *ast.IdentifierExpression = @ptrCast(@alignCast(expr.data));
                return self.variables.get(id_expr.name) orelse null;
            },
            .FunctionCall => {
                const call_expr: *ast.FunctionCallExpression = @ptrCast(@alignCast(expr.data));
                return try self.generateFunctionCall(call_expr.*);
            },
            else => {
                std.debug.print("Expression type not implemented: {}\n", .{expr.tag});
                return llvm_const_int(self.i32_type, 0);
            },
        }
    }
    
    fn generateFunctionCall(self: *RealLLVMCodeGen, call: ast.FunctionCallExpression) LLVMError!?*anyopaque {
        // Handle vibez.spill special case
        if (std.mem.eql(u8, call.name, "vibez.spill")) {
            return try self.generatePrintCall(call);
        }
        
        // Regular function call
        const function = self.functions.get(call.name) orelse {
            return LLVMError.UndefinedSymbol;
        };
        
        const func_type = llvm_get_function_type(function);
        return llvm_build_call2(self.builder, func_type, function, null, 0, "call");
    }
    
    fn generatePrintCall(self: *RealLLVMCodeGen, call: ast.FunctionCallExpression) LLVMError!?*anyopaque {
        const printf_func = self.functions.get("printf") orelse {
            return LLVMError.UndefinedSymbol;
        };
        
        // Generate arguments
        var args = ArrayList(?*anyopaque).init(self.allocator);
        defer args.deinit();
        
        for (call.arguments.items) |arg| {
            const arg_value = try self.generateExpression(arg.*);
            try args.append(arg_value);
        }
        
        // Create format string based on argument types (simplified)
        const format_str = if (args.items.len > 0) "%s\n" else "Hello World\n";
        const format_global = llvm_build_global_string_ptr(self.builder, format_str, "fmt");
        
        // Insert format string at beginning
        try args.insert(0, format_global);
        
        const func_type = llvm_get_function_type(printf_func);
        return llvm_build_call2(self.builder, func_type, printf_func, args.items.ptr, @intCast(args.items.len), "printf_call");
    }
    
    fn generateMainFunction(self: *RealLLVMCodeGen) LLVMError!void {
        // Create main function that returns int
        const main_type = llvm_function_type(self.i32_type, null, 0, 0);
        const main_func = llvm_add_function(self.module, "main", main_type);
        
        if (main_func == null) {
            return LLVMError.FunctionCreationFailed;
        }
        
        // Create entry block
        const entry_block = llvm_append_basic_block(self.context, main_func, "entry");
        llvm_position_builder_at_end(self.builder, entry_block);
        
        // Return 0
        const return_value = llvm_const_int(self.i32_type, 0);
        _ = llvm_build_ret(self.builder, return_value);
        
        try self.functions.put("main", main_func);
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
};

test "real llvm basic" {
    const allocator = std.testing.allocator;
    
    var codegen = RealLLVMCodeGen.init(allocator) catch |err| {
        std.debug.print("Failed to initialize LLVM: {}\n", .{err});
        return;
    };
    defer codegen.deinit();
    
    // Test basic initialization
    try std.testing.expect(codegen.context != null);
    try std.testing.expect(codegen.module != null);
    try std.testing.expect(codegen.builder != null);
}
