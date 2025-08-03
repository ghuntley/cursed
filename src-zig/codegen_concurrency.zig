//! CURSED Compiler Concurrency Code Generation
//!
//! This module provides LLVM code generation for CURSED concurrency features:
//! - stan keyword compilation to goroutine spawn calls
//! - dm<T> type compilation to channel creation
//! - ready keyword compilation to select statements
//! - Channel send/receive operation compilation
//!
//! Integrates with the LLVM backend for native code generation.

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const ast = @import("ast_simple.zig");
const concurrency = @import("concurrency.zig");

/// LLVM code generation context for concurrency
pub const ConcurrencyCodeGen = struct {
    allocator: Allocator,
    output: ArrayList(u8),
    function_counter: u32,
    variable_counter: u32,
    label_counter: u32,
    runtime_functions_declared: bool,

    pub fn init(allocator: Allocator) ConcurrencyCodeGen {
        return ConcurrencyCodeGen{
            .allocator = allocator,
            .output = ArrayList(u8).init(allocator),
            .function_counter = 0,
            .variable_counter = 0,
            .label_counter = 0,
            .runtime_functions_declared = false,
        };
    }

    pub fn deinit(self: *ConcurrencyCodeGen) void {
        self.output.deinit();
    }

    /// Generate LLVM IR for entire program with concurrency support
    pub fn generateProgram(self: *ConcurrencyCodeGen, program: *ast.Program) !void {
        // Generate runtime function declarations
        try self.generateRuntimeDeclarations();

        // Generate main function with runtime initialization
        try self.generateMainFunction(program);

        // Generate concurrency helper functions
        try self.generateConcurrencyHelpers();
    }

    /// Generate runtime function declarations
    fn generateRuntimeDeclarations(self: *ConcurrencyCodeGen) !void {
        if (self.runtime_functions_declared) return;

        const declarations =
            \\; CURSED Concurrency Runtime Function Declarations
            \\declare i64 @cursed_runtime_spawn_goroutine(i8*, i8*)
            \\declare i64 @cursed_runtime_create_channel(i32, i64)
            \\declare i32 @cursed_runtime_send_channel(i64, i8*, i64)
            \\declare i8* @cursed_runtime_receive_channel(i64, i64*)
            \\declare i32 @cursed_runtime_select(i8*, i64)
            \\declare void @cursed_runtime_yield()
            \\declare void @cursed_runtime_init()
            \\declare void @cursed_runtime_shutdown()
            \\
            \\; Standard C functions
            \\declare i32 @printf(i8*, ...)
            \\declare i8* @malloc(i64)
            \\declare void @free(i8*)
            \\declare void @exit(i32)
            \\
            \\; CURSED concurrency types
            \\%goroutine_id = type i64
            \\%channel_id = type i64
            \\%select_result = type i32
            \\%concurrency_value = type { i32, i8* }
            \\
            \\
        ;

        try self.output.appendSlice(declarations);
        self.runtime_functions_declared = true;
    }

    /// Generate main function with runtime initialization
    fn generateMainFunction(self: *ConcurrencyCodeGen, program: *ast.Program) !void {
        try self.output.appendSlice("define i32 @main() {\n");
        try self.output.appendSlice("entry:\n");
        
        // Initialize concurrency runtime
        try self.output.appendSlice("  call void @cursed_runtime_init()\n");

        // Generate statements
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }

        // Shutdown runtime and exit
        try self.output.appendSlice("  call void @cursed_runtime_shutdown()\n");
        try self.output.appendSlice("  ret i32 0\n");
        try self.output.appendSlice("}\n\n");
    }

    /// Generate LLVM IR for statement
    fn generateStatement(self: *ConcurrencyCodeGen, stmt: ast.Statement) !void {
        switch (stmt) {
            .expression_statement => |expr_stmt| {
                _ = try self.generateExpression(expr_stmt.expression);
            },
            .let_statement => |let_stmt| {
                try self.generateLetStatement(let_stmt);
            },
            .return_statement => |ret_stmt| {
                if (ret_stmt.return_value) |ret_val| {
                    const value_reg = try self.generateExpression(ret_val);
                    try self.output.writer().print("  ret i64 {s}\n", .{value_reg});
                } else {
                    try self.output.appendSlice("  ret void\n");
                }
            },
            .block_statement => |block_stmt| {
                for (block_stmt.statements.items) |block_stmt_item| {
                    try self.generateStatement(block_stmt_item);
                }
            },
            else => {},
        }
    }

    /// Generate let statement
    fn generateLetStatement(self: *ConcurrencyCodeGen, let_stmt: *ast.LetStatement) !void {
        const value_reg = try self.generateExpression(let_stmt.value);
        const var_reg = try self.nextRegister();
        
        try self.output.writer().print("  %{s} = alloca i64\n", .{var_reg});
        try self.output.writer().print("  store i64 {s}, i64* %{s}\n", .{ value_reg, var_reg });
        
        // Store variable name for later reference (in real implementation)
        _ = let_stmt.name.value;
    }

    /// Generate LLVM IR for expression and return register name
    fn generateExpression(self: *ConcurrencyCodeGen, expr: ast.Expression) ![]const u8 {
        return switch (expr) {
            .integer_literal => |int_lit| try self.generateIntegerLiteral(int_lit),
            .string_literal => |str_lit| try self.generateStringLiteral(str_lit),
            .boolean_literal => |bool_lit| try self.generateBooleanLiteral(bool_lit),
            .identifier => |ident| try self.generateIdentifier(ident),
            .function_literal => |func_lit| try self.generateFunctionLiteral(func_lit),
            
            // Concurrency expressions
            .call_expression => |call_expr| try self.generateCallExpression(call_expr),
            .channel_literal => |chan_lit| try self.generateChannelLiteral(chan_lit),
            .goroutine_spawn => |spawn_expr| try self.generateGoroutineSpawn(spawn_expr),
            .channel_send => |send_expr| try self.generateChannelSend(send_expr),
            .channel_receive => |recv_expr| try self.generateChannelReceive(recv_expr),
            .select_expression => |select_expr| try self.generateSelectExpression(select_expr),
            
            else => try self.nextRegister(),
        };
    }

    /// Generate integer literal
    fn generateIntegerLiteral(self: *ConcurrencyCodeGen, int_lit: *ast.IntegerLiteral) ![]const u8 {
        return try std.fmt.allocPrint(self.allocator, "{}", .{int_lit.value});
    }

    /// Generate string literal
    fn generateStringLiteral(self: *ConcurrencyCodeGen, str_lit: *ast.StringLiteral) ![]const u8 {
        const reg = try self.nextRegister();
        const str_len = str_lit.value.len;
        
        try self.output.writer().print("  %{s} = alloca [{} x i8]\n", .{ reg, str_len + 1 });
        try self.output.writer().print("  store [{} x i8] c\"{s}\\00\", [{} x i8]* %{s}\n", .{ str_len + 1, str_lit.value, str_len + 1, reg });
        
        const ptr_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = getelementptr [{} x i8], [{} x i8]* %{s}, i32 0, i32 0\n", .{ ptr_reg, str_len + 1, str_len + 1, reg });
        
        return ptr_reg;
    }

    /// Generate boolean literal
    fn generateBooleanLiteral(self: *ConcurrencyCodeGen, bool_lit: *ast.BooleanLiteral) ![]const u8 {
        return try std.fmt.allocPrint(self.allocator, "{}", .{if (bool_lit.value) 1 else 0});
    }

    /// Generate identifier (variable access)
    fn generateIdentifier(self: *ConcurrencyCodeGen, ident: *ast.Identifier) ![]const u8 {
        // In real implementation, would look up variable in symbol table
        const reg = try self.nextRegister();
        try self.output.writer().print("  ; Identifier access: {s}\n", .{ident.value});
        try self.output.writer().print("  %{s} = load i64, i64* %%{s}_var\n", .{ reg, ident.value });
        return reg;
    }

    /// Generate function literal
    fn generateFunctionLiteral(self: *ConcurrencyCodeGen, func_lit: *ast.FunctionLiteral) ![]const u8 {
        const func_name = try self.nextFunction();
        
        // Generate function definition
        try self.output.writer().print("define i64 @{s}(", .{func_name});
        
        // Parameters
        for (func_lit.parameters.items, 0..) |param, i| {
            if (i > 0) try self.output.appendSlice(", ");
            try self.output.writer().print("i64 %{s}", .{param.value});
        }
        
        try self.output.appendSlice(") {\n");
        try self.output.appendSlice("entry:\n");
        
        // Function body
        try self.generateStatement(ast.Statement{ .block_statement = func_lit.body });
        
        try self.output.appendSlice("  ret i64 0\n");
        try self.output.appendSlice("}\n\n");
        
        // Return function pointer
        const reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = bitcast i64 (i64)* @{s} to i8*\n", .{ reg, func_name });
        return reg;
    }

    /// Generate call expression
    fn generateCallExpression(self: *ConcurrencyCodeGen, call_expr: *ast.CallExpression) ![]const u8 {
        const function_reg = try self.generateExpression(call_expr.function.*);
        
        var arg_regs = ArrayList([]const u8).init(self.allocator);
        defer arg_regs.deinit();
        
        for (call_expr.arguments.items) |arg| {
            const arg_reg = try self.generateExpression(arg);
            try arg_regs.append(arg_reg);
        }
        
        const result_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = call i64 {s}(", .{ result_reg, function_reg });
        
        for (arg_regs.items, 0..) |arg_reg, i| {
            if (i > 0) try self.output.appendSlice(", ");
            try self.output.writer().print("i64 {s}", .{arg_reg});
        }
        
        try self.output.appendSlice(")\n");
        return result_reg;
    }

    /// Generate channel literal (dm<T> type)
    fn generateChannelLiteral(self: *ConcurrencyCodeGen, chan_lit: *ast.ChannelLiteral) ![]const u8 {
        const capacity_reg = if (chan_lit.capacity) |cap_expr| blk: {
            break :blk try self.generateExpression(cap_expr.*);
        } else "0";
        
        const channel_type = switch (chan_lit.element_type) {
            .normie => "0", // Integer channel
            .tea => "1", // String channel
            .lit => "2", // Boolean channel
            else => "0",
        };
        
        const channel_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = call i64 @cursed_runtime_create_channel(i32 {s}, i64 {s})\n", .{ channel_reg, channel_type, capacity_reg });
        
        return channel_reg;
    }

    /// Generate goroutine spawn (stan keyword)
    fn generateGoroutineSpawn(self: *ConcurrencyCodeGen, spawn_expr: *ast.GoroutineSpawn) ![]const u8 {
        const function_reg = try self.generateExpression(spawn_expr.function.*);
        
        const goroutine_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = call i64 @cursed_runtime_spawn_goroutine(i8* {s}, i8* null)\n", .{ goroutine_reg, function_reg });
        
        return goroutine_reg;
    }

    /// Generate channel send operation
    fn generateChannelSend(self: *ConcurrencyCodeGen, send_expr: *ast.ChannelSend) ![]const u8 {
        const channel_reg = try self.generateExpression(send_expr.channel.*);
        const value_reg = try self.generateExpression(send_expr.value.*);
        
        // Create concurrency value structure
        const value_struct_reg = try self.nextRegister();
        const value_ptr_reg = try self.nextRegister();
        
        try self.output.writer().print("  %{s} = alloca %concurrency_value\n", .{value_struct_reg});
        try self.output.writer().print("  %{s} = alloca i64\n", .{value_ptr_reg});
        try self.output.writer().print("  store i64 {s}, i64* %{s}\n", .{ value_reg, value_ptr_reg });
        
        const cast_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = bitcast i64* %{s} to i8*\n", .{ cast_reg, value_ptr_reg });
        
        const result_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = call i32 @cursed_runtime_send_channel(i64 {s}, i8* {s}, i64 8)\n", .{ result_reg, channel_reg, cast_reg });
        
        return result_reg;
    }

    /// Generate channel receive operation
    fn generateChannelReceive(self: *ConcurrencyCodeGen, recv_expr: *ast.ChannelReceive) ![]const u8 {
        const channel_reg = try self.generateExpression(recv_expr.channel.*);
        
        const size_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = alloca i64\n", .{size_reg});
        
        const result_ptr_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = call i8* @cursed_runtime_receive_channel(i64 {s}, i64* %{s})\n", .{ result_ptr_reg, channel_reg, size_reg });
        
        const result_reg = try self.nextRegister();
        const cast_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = bitcast i8* {s} to i64*\n", .{ cast_reg, result_ptr_reg });
        try self.output.writer().print("  %{s} = load i64, i64* {s}\n", .{ result_reg, cast_reg });
        
        return result_reg;
    }

    /// Generate select expression (ready keyword)
    fn generateSelectExpression(self: *ConcurrencyCodeGen, select_expr: *ast.SelectExpression) ![]const u8 {
        // Create select operations array
        const ops_array_reg = try self.nextRegister();
        const num_cases = select_expr.cases.items.len;
        
        try self.output.writer().print("  %{s} = alloca [{}] i8*\n", .{ ops_array_reg, num_cases });
        
        // Generate each case
        for (select_expr.cases.items, 0..) |case_item, i| {
            const case_reg = try self.nextRegister();
            
            switch (case_item.operation) {
                .send => |send_op| {
                    const channel_reg = try self.generateExpression(send_op.channel.*);
                    const value_reg = try self.generateExpression(send_op.value.*);
                    
                    try self.output.writer().print("  ; Send operation case {}\n", .{i});
                    try self.output.writer().print("  %{s} = call i8* @malloc(i64 24)\n", .{case_reg});
                    
                    _ = channel_reg;
                    _ = value_reg;
                },
                .receive => |recv_op| {
                    const channel_reg = try self.generateExpression(recv_op.channel.*);
                    
                    try self.output.writer().print("  ; Receive operation case {}\n", .{i});
                    try self.output.writer().print("  %{s} = call i8* @malloc(i64 16)\n", .{case_reg});
                    
                    _ = channel_reg;
                },
                .default => {
                    try self.output.writer().print("  ; Default case {}\n", .{i});
                    try self.output.writer().print("  %{s} = call i8* @malloc(i64 8)\n", .{case_reg});
                },
            }
            
            const array_ptr_reg = try self.nextRegister();
            try self.output.writer().print("  %{s} = getelementptr [{}] i8*, [{}] i8** %{s}, i32 0, i32 {}\n", .{ array_ptr_reg, num_cases, num_cases, ops_array_reg, i });
            try self.output.writer().print("  store i8* %{s}, i8** %{s}\n", .{ case_reg, array_ptr_reg });
        }
        
        // Execute select
        const select_result_reg = try self.nextRegister();
        const array_cast_reg = try self.nextRegister();
        try self.output.writer().print("  %{s} = bitcast [{}] i8** %{s} to i8*\n", .{ array_cast_reg, num_cases, ops_array_reg });
        try self.output.writer().print("  %{s} = call i32 @cursed_runtime_select(i8* {s}, i64 {})\n", .{ select_result_reg, array_cast_reg, num_cases });
        
        return select_result_reg;
    }

    /// Generate concurrency helper functions
    fn generateConcurrencyHelpers(self: *ConcurrencyCodeGen) !void {
        const helpers =
            \\; CURSED Concurrency Helper Functions
            \\define void @cursed_yield() {
            \\entry:
            \\  call void @cursed_runtime_yield()
            \\  ret void
            \\}
            \\
            \\define i64 @cursed_make_channel(i32 %type, i64 %capacity) {
            \\entry:
            \\  %result = call i64 @cursed_runtime_create_channel(i32 %type, i64 %capacity)
            \\  ret i64 %result
            \\}
            \\
            \\define i32 @cursed_send(i64 %channel, i8* %value, i64 %size) {
            \\entry:
            \\  %result = call i32 @cursed_runtime_send_channel(i64 %channel, i8* %value, i64 %size)
            \\  ret i32 %result
            \\}
            \\
            \\define i8* @cursed_receive(i64 %channel, i64* %size) {
            \\entry:
            \\  %result = call i8* @cursed_runtime_receive_channel(i64 %channel, i64* %size)
            \\  ret i8* %result
            \\}
            \\
            \\
        ;
        
        try self.output.appendSlice(helpers);
    }

    /// Generate next register name
    fn nextRegister(self: *ConcurrencyCodeGen) ![]const u8 {
        self.variable_counter += 1;
        return try std.fmt.allocPrint(self.allocator, "v{}", .{self.variable_counter});
    }

    /// Generate next function name
    fn nextFunction(self: *ConcurrencyCodeGen) ![]const u8 {
        self.function_counter += 1;
        return try std.fmt.allocPrint(self.allocator, "func_{}", .{self.function_counter});
    }

    /// Generate next label name
    fn nextLabel(self: *ConcurrencyCodeGen) ![]const u8 {
        self.label_counter += 1;
        return try std.fmt.allocPrint(self.allocator, "label_{}", .{self.label_counter});
    }

    /// Get generated LLVM IR
    pub fn getOutput(self: *ConcurrencyCodeGen) []const u8 {
        return self.output.items;
    }

    /// Write output to file
    pub fn writeToFile(self: *ConcurrencyCodeGen, filename: []const u8) !void {
        const file = try std.fs.cwd().createFile(filename, .{});
        defer file.close();
        
        try file.writeAll(self.output.items);
    }
};

// Tests
test "concurrency codegen initialization" {
    const allocator = std.testing.allocator;
    
    var codegen = ConcurrencyCodeGen.init(allocator);
    defer codegen.deinit();

    try std.testing.expect(codegen.function_counter == 0);
    try std.testing.expect(codegen.variable_counter == 0);
}

test "runtime declarations generation" {
    const allocator = std.testing.allocator;
    
    var codegen = ConcurrencyCodeGen.init(allocator);
    defer codegen.deinit();

    try codegen.generateRuntimeDeclarations();
    
    const output = codegen.getOutput();
    try std.testing.expect(std.mem.indexOf(u8, output, "cursed_runtime_spawn_goroutine") != null);
    try std.testing.expect(std.mem.indexOf(u8, output, "cursed_runtime_create_channel") != null);
}

test "channel literal generation" {
    const allocator = std.testing.allocator;
    
    var codegen = ConcurrencyCodeGen.init(allocator);
    defer codegen.deinit();

    try codegen.generateRuntimeDeclarations();

    var channel_literal = ast.ChannelLiteral{
        .element_type = .normie,
        .capacity = null,
    };

    const result_reg = try codegen.generateChannelLiteral(&channel_literal);
    try std.testing.expect(std.mem.startsWith(u8, result_reg, "v"));

    const output = codegen.getOutput();
    try std.testing.expect(std.mem.indexOf(u8, output, "cursed_runtime_create_channel") != null);
}

test "goroutine spawn generation" {
    const allocator = std.testing.allocator;
    
    var codegen = ConcurrencyCodeGen.init(allocator);
    defer codegen.deinit();

    try codegen.generateRuntimeDeclarations();

    var function_literal = ast.FunctionLiteral{
        .parameters = ArrayList(*ast.Identifier).init(allocator),
        .body = ast.BlockStatement{ .statements = ArrayList(ast.Statement).init(allocator) },
    };
    defer function_literal.parameters.deinit();
    defer function_literal.body.statements.deinit();

    var spawn_expr = ast.GoroutineSpawn{
        .function = &ast.Expression{ .function_literal = &function_literal },
    };

    const result_reg = try codegen.generateGoroutineSpawn(&spawn_expr);
    try std.testing.expect(std.mem.startsWith(u8, result_reg, "v"));

    const output = codegen.getOutput();
    try std.testing.expect(std.mem.indexOf(u8, output, "cursed_runtime_spawn_goroutine") != null);
}
