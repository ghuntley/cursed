const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

/// Simple LLVM IR Generator that outputs text IR
/// This bypasses LLVM C API issues and generates text IR directly
pub const SimpleLLVMIRGenerator = struct {
    allocator: Allocator,
    ir_buffer: ArrayList(u8),
    string_counter: u32,
    verbose: bool,
    
    pub fn init(allocator: Allocator) SimpleLLVMIRGenerator {
        return SimpleLLVMIRGenerator{
            .allocator = allocator,
            .ir_buffer = ArrayList(u8).init(allocator),
            .string_counter = 0,
            .verbose = false,
        };
    }
    
    pub fn deinit(self: *SimpleLLVMIRGenerator) void {
        self.ir_buffer.deinit();
    }
    
    pub fn setVerbose(self: *SimpleLLVMIRGenerator, verbose: bool) void {
        self.verbose = verbose;
    }
    
    /// Generate LLVM IR from CURSED source
    pub fn generateFromSource(self: *SimpleLLVMIRGenerator, source: []const u8) !void {
        if (self.verbose) print("🔧 Parsing CURSED source...\n", .{});
        
        // Parse the source
        var lex = lexer.Lexer.init(self.allocator, source);
        
        const tokens = try lex.tokenize();
        defer tokens.deinit();
        
        var parse = parser.Parser.init(self.allocator, tokens.items);
        defer parse.deinit();
        
        const program = try parse.parseProgram();
        
        // Generate IR
        try self.generateProgram(program);
    }
    
    /// Generate IR for a program
    fn generateProgram(self: *SimpleLLVMIRGenerator, program: ast.Program) !void {
        // Clear buffer
        self.ir_buffer.clearRetainingCapacity();
        
        // Generate header
        try self.generateHeader();
        
        // Generate each statement  
        for (program.statements.items) |stmt_ptr| {
            const stmt: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            try self.generateStatement(stmt.*);
        }
        
        // Ensure main function exists
        try self.ensureMainFunction();
    }
    
    /// Generate LLVM IR header
    fn generateHeader(self: *SimpleLLVMIRGenerator) !void {
        const writer = self.ir_buffer.writer();
        
        try writer.writeAll("; Generated LLVM IR for CURSED program\n");
        try writer.writeAll("target triple = \"x86_64-unknown-linux-gnu\"\n\n");
        
        // External function declarations
        try writer.writeAll("declare i32 @puts(i8*)\n");
        try writer.writeAll("declare i32 @printf(i8*, ...)\n\n");
        
        // String format constants
        try writer.writeAll("@.int_fmt = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\", align 1\n\n");
    }
    
    /// Generate statement
    fn generateStatement(self: *SimpleLLVMIRGenerator, stmt: ast.Statement) !void {
        switch (stmt) {
            .Function => |func_stmt| {
                try self.generateFunction(func_stmt);
            },
            else => {
                // Other statements handled in function body
                if (self.verbose) print("⚠️  Unsupported statement type: {s}\n", .{@tagName(stmt)});
            },
        }
    }

    /// Generate function
    fn generateFunction(self: *SimpleLLVMIRGenerator, func_stmt: ast.FunctionStatement) !void {
        const writer = self.ir_buffer.writer();
        
        // Determine return type
        const return_type = if (func_stmt.return_type) |ret_type|
            try self.cursedTypeToLLVM(ret_type)
        else
            "void";
        
        // Generate function signature
        try writer.print("define {s} @{s}(", .{ return_type, func_stmt.name });
        
        // Parameters
        for (func_stmt.parameters.items, 0..) |param, i| {
            if (i > 0) try writer.writeAll(", ");
            const param_type = try self.cursedTypeToLLVM(param.param_type);
            try writer.print("{s} %{s}", .{ param_type, param.name });
        }
        
        try writer.writeAll(") {\n");
        try writer.writeAll("entry:\n");
        
        // Generate function body
        for (func_stmt.body.items) |stmt| {
            try self.generateFunctionBodyStatement(stmt.*);
        }
        
        // Add return if not present
        if (func_stmt.return_type == null) {
            try writer.writeAll("  ret void\n");
        }
        
        try writer.writeAll("}\n\n");
    }
    
    /// Generate statement in function body
    fn generateFunctionBodyStatement(self: *SimpleLLVMIRGenerator, stmt: ast.Statement) !void {
        switch (stmt) {
            .Let => |let_stmt| {
                try self.generateLetStatement(let_stmt);
            },
            .Expression => |expr_ptr| {
                const expr: *ast.Expression = @ptrCast(@alignCast(expr_ptr));
                try self.generateExpressionStatement(expr.*);
            },
            else => {
                if (self.verbose) print("⚠️ Unhandled statement type in function body\n", .{});
            },
        }
    }
    
    /// Generate let statement (variable declaration)
    fn generateLetStatement(self: *SimpleLLVMIRGenerator, let_stmt: ast.LetStatement) !void {
        const writer = self.ir_buffer.writer();
        
        // Determine type - use annotation if available, otherwise infer from initializer
        const llvm_type = if (let_stmt.type_annotation) |type_annotation|
            try self.cursedTypeToLLVM(type_annotation)
        else if (let_stmt.var_type) |var_type|
            try self.cursedTypeToLLVM(var_type)
        else
            "i64"; // Default to i64
        
        // Create alloca
        try writer.print("  %{s} = alloca {s}, align 8\n", .{ let_stmt.name, llvm_type });
        
        // Generate initializer if present
        if (let_stmt.initializer) |initializer| {
            const init_result = try self.generateExpression(initializer.*);
            try writer.print("  store {s} {s}, {s}* %{s}, align 8\n", 
                .{ llvm_type, init_result, llvm_type, let_stmt.name });
        }
    }
    
    /// Generate expression statement
    fn generateExpressionStatement(self: *SimpleLLVMIRGenerator, expr: ast.Expression) !void {
        _ = try self.generateExpression(expr);
    }
    
    /// Generate expression and return the result
    fn generateExpression(self: *SimpleLLVMIRGenerator, expr: ast.Expression) anyerror![]const u8 {
        switch (expr) {
            .Literal => |lit| {
                return try self.generateLiteral(lit);
            },
            .Call => |call| {
                return try self.generateFunctionCall(call);
            },
            .Binary => |bin_op| {
                return try self.generateBinaryOperation(bin_op);
            },
            .Identifier => |ident| {
                return try self.generateIdentifier(ident);
            },
            .Integer => |int_val| {
                return try std.fmt.allocPrint(self.allocator, "{d}", .{int_val});
            },
            .String => |str_val| {
                return try self.generateStringLiteral(str_val);
            },
            .Boolean => |bool_val| {
                return if (bool_val) "1" else "0";
            },
            .Float => |float_val| {
                return try std.fmt.allocPrint(self.allocator, "{d}", .{float_val});
            },
            else => {
                if (self.verbose) print("⚠️ Unhandled expression type\n", .{});
                return "0";
            },
        }
    }
    
    /// Generate literal
    fn generateLiteral(self: *SimpleLLVMIRGenerator, literal: ast.Literal) ![]const u8 {
        switch (literal) {
            .Integer => |int_val| {
                return try std.fmt.allocPrint(self.allocator, "{d}", .{int_val});
            },
            .String => |str_val| {
                return try self.generateStringLiteral(str_val);
            },
            .Boolean => |bool_val| {
                return if (bool_val) "1" else "0";
            },
            .Float => |float_val| {
                return try std.fmt.allocPrint(self.allocator, "{d}", .{float_val});
            },
            .Character => |char_val| {
                return try std.fmt.allocPrint(self.allocator, "{d}", .{char_val});
            },
            .Null, .Nil => {
                return "null";
            },
        }
    }
    
    /// Generate string literal
    fn generateStringLiteral(self: *SimpleLLVMIRGenerator, str_val: []const u8) ![]const u8 {
        // Generate global string constant
        const str_name = try std.fmt.allocPrint(self.allocator, ".str.{d}", .{self.string_counter});
        self.string_counter += 1;
        
        // Add the global string to the top of the buffer (before current position)
        const global_def = try std.fmt.allocPrint(self.allocator, 
            "@{s} = private unnamed_addr constant [{d} x i8] c\"{s}\\00\", align 1\n",
            .{ str_name, str_val.len + 1, str_val });
        
        // Insert at end of globals section (before first function)
        try self.ir_buffer.insertSlice(self.findGlobalsInsertPoint(), global_def);
        
        // Return GEP instruction
        const gep_result = try std.fmt.allocPrint(self.allocator, 
            "getelementptr [{d} x i8], [{d} x i8]* @{s}, i32 0, i32 0",
            .{ str_val.len + 1, str_val.len + 1, str_name });
        
        return gep_result;
    }
    
    /// Find insertion point for global declarations
    fn findGlobalsInsertPoint(self: *SimpleLLVMIRGenerator) usize {
        const content = self.ir_buffer.items;
        
        // Look for "define" which marks the start of functions
        if (std.mem.indexOf(u8, content, "define ")) |pos| {
            return pos;
        }
        
        // If no functions yet, append at the end
        return content.len;
    }
    
    /// Generate function call
    fn generateFunctionCall(self: *SimpleLLVMIRGenerator, call: ast.CallExpression) ![]const u8 {
        const writer = self.ir_buffer.writer();
        
        // Extract function name - for now assume it's an identifier
        const func_name = switch (call.function.*) {
            .Identifier => |name| name,
            else => "unknown_function",
        };
        
        // Handle special functions
        if (std.mem.eql(u8, func_name, "vibez.spill")) {
            var args = ArrayList(ast.Expression).init(self.allocator);
            defer args.deinit();
            
            for (call.arguments.items) |arg_ptr| {
                try args.append(arg_ptr.*);
            }
            
            return try self.generatePrintCall(args.items);
        }
        
        // Regular function call
        var args = ArrayList([]const u8).init(self.allocator);
        defer args.deinit();
        
        for (call.arguments.items) |arg_ptr| {
            const arg_result = try self.generateExpression(arg_ptr.*);
            try args.append(arg_result);
        }
        
        const call_result = try std.fmt.allocPrint(self.allocator, "call_result_{d}", .{self.string_counter});
        self.string_counter += 1;
        
        try writer.print("  %{s} = call i64 @{s}(", .{ call_result, func_name });
        for (args.items, 0..) |arg, i| {
            if (i > 0) try writer.writeAll(", ");
            try writer.print("i64 {s}", .{arg});
        }
        try writer.writeAll(")\n");
        
        return call_result;
    }
    
    /// Generate print call (vibez.spill)
    fn generatePrintCall(self: *SimpleLLVMIRGenerator, args: []ast.Expression) anyerror![]const u8 {
        const writer = self.ir_buffer.writer();
        
        if (args.len == 0) return "0";
        
        const arg_result = self.generateExpression(args[0]) catch |err| switch (err) {
            else => return err,
        };
        
        // Check if it's a string (contains "getelementptr") or integer
        if (std.mem.indexOf(u8, arg_result, "getelementptr") != null) {
            // String print using puts
            const str_ptr = try std.fmt.allocPrint(self.allocator, "str_ptr_{d}", .{self.string_counter});
            self.string_counter += 1;
            
            try writer.print("  %{s} = {s}\n", .{ str_ptr, arg_result });
            try writer.print("  %call_{d} = call i32 @puts(i8* %{s})\n", .{ self.string_counter, str_ptr });
            self.string_counter += 1;
        } else {
            // Integer print using printf
            try writer.print("  %fmt_ptr_{d} = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0\n", .{self.string_counter});
            try writer.print("  %call_{d} = call i32 (i8*, ...) @printf(i8* %fmt_ptr_{d}, i64 {s})\n", .{ self.string_counter, self.string_counter, arg_result });
            self.string_counter += 1;
        }
        
        return "0";
    }
    
    /// Generate binary operation
    fn generateBinaryOperation(self: *SimpleLLVMIRGenerator, bin_op: ast.BinaryExpression) ![]const u8 {
        const writer = self.ir_buffer.writer();
        
        const left = try self.generateExpression(bin_op.left.*);
        const right = try self.generateExpression(bin_op.right.*);
        
        const result_name = try std.fmt.allocPrint(self.allocator, "op_result_{d}", .{self.string_counter});
        self.string_counter += 1;
        
        // Determine operation based on operator string
        const op_name = if (std.mem.eql(u8, bin_op.operator, "+"))
            "add"
        else if (std.mem.eql(u8, bin_op.operator, "-"))
            "sub"
        else if (std.mem.eql(u8, bin_op.operator, "*"))
            "mul"
        else if (std.mem.eql(u8, bin_op.operator, "/"))
            "sdiv"
        else
            "add"; // fallback
        
        try writer.print("  %{s} = {s} i64 {s}, {s}\n", .{ result_name, op_name, left, right });
        
        return result_name;
    }
    
    /// Generate identifier (variable load)
    fn generateIdentifier(self: *SimpleLLVMIRGenerator, ident_name: []const u8) ![]const u8 {
        const writer = self.ir_buffer.writer();
        
        const load_result = try std.fmt.allocPrint(self.allocator, "load_{s}_{d}", .{ ident_name, self.string_counter });
        self.string_counter += 1;
        
        try writer.print("  %{s} = load i64, i64* %{s}, align 8\n", .{ load_result, ident_name });
        
        return load_result;
    }
    
    /// Convert CURSED type to LLVM type string
    fn cursedTypeToLLVM(self: *SimpleLLVMIRGenerator, cursed_type: ast.Type) ![]const u8 {
        _ = self;
        return switch (cursed_type) {
            .Basic => |basic| switch (basic) {
                .Drip => "i64",
                .Normie => "i32",
                .Thicc => "i64", 
                .Smol => "i8",
                .Mid => "i16",
                .Tea => "i8*",
                .Txt => "i8*",
                .Lit => "i1",
                .Snack => "float",
                .Meal => "double",
                .Sip => "i8",
                .Byte => "i8",
                .Rune => "i32",
                .Cap => "i8*",
                else => "i64",
            },
            else => "i64", // fallback
        };
    }
    
    /// Ensure main function exists
    fn ensureMainFunction(self: *SimpleLLVMIRGenerator) !void {
        const content = self.ir_buffer.items;
        
        // Check if main already exists
        if (std.mem.indexOf(u8, content, "define i32 @main") != null) {
            return;
        }
        
        const writer = self.ir_buffer.writer();
        
        // Create main function
        try writer.writeAll("define i32 @main() {\n");
        try writer.writeAll("entry:\n");
        
        // Call main_character if it exists
        if (std.mem.indexOf(u8, content, "define void @main_character") != null) {
            try writer.writeAll("  call void @main_character()\n");
        }
        
        try writer.writeAll("  ret i32 0\n");
        try writer.writeAll("}\n");
    }
    
    /// Write IR to file
    pub fn writeToFile(self: *SimpleLLVMIRGenerator, filename: []const u8) !void {
        const file = try std.fs.cwd().createFile(filename, .{});
        defer file.close();
        
        try file.writeAll(self.ir_buffer.items);
        
        if (self.verbose) {
            print("✅ LLVM IR written to: {s}\n", .{filename});
        }
    }
    
    /// Compile IR to executable using clang
    pub fn compileToExecutable(self: *SimpleLLVMIRGenerator, ir_file: []const u8, output_file: []const u8) !void {
        // Try clang-18 first, then fallback to clang
        var clang_cmd: []const u8 = "clang-18";
        const result = std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{
                clang_cmd,
                "-O2",
                "-o", output_file,
                ir_file,
            },
        }) catch blk: {
            // Fallback to clang
            clang_cmd = "clang";
            const fallback = std.process.Child.run(.{
                .allocator = self.allocator,
                .argv = &[_][]const u8{
                    clang_cmd,
                    "-O2",
                    "-o", output_file,
                    ir_file,
                },
            }) catch |err| {
                print("❌ Neither clang-18 nor clang found\n", .{});
                return err;
            };
            break :blk fallback;
        };
        
        defer self.allocator.free(result.stdout);
        defer self.allocator.free(result.stderr);
        
        if (result.term != .Exited or result.term.Exited != 0) {
            print("❌ Compilation failed:\n{s}\n", .{result.stderr});
            return error.CompilationFailed;
        }
        
        if (self.verbose) {
            print("✅ Successfully compiled to: {s}\n", .{output_file});
        }
    }
    
    /// Get generated IR as string
    pub fn getIR(self: *SimpleLLVMIRGenerator) []const u8 {
        return self.ir_buffer.items;
    }
};
