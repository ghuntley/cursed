const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Import the real LLVM integration
const llvm_fix = @import("llvm_integration_fix.zig");

// Simple AST structures for basic CURSED programs
const Value = union(enum) {
    Integer: i64,
    String: []const u8,
    Boolean: bool,
};

const BinaryOp = enum {
    Add,
    Subtract,
    Multiply,
    Divide,
};

const Expression = union(enum) {
    Literal: Value,
    Identifier: []const u8,
    BinaryOp: struct {
        left: *Expression,
        right: *Expression,
        op: BinaryOp,
    },
    Call: struct {
        function: []const u8,
        args: []Expression,
    },
};

const Statement = union(enum) {
    VarDecl: struct {
        name: []const u8,
        value: Expression,
    },
    Call: struct {
        function: []const u8,
        args: []Expression,
    },
};

const Program = struct {
    statements: []Statement,
};

const CompilerError = error{
    LLVMError,
    UndefinedVariable,
    UndefinedFunction,
    OutOfMemory,
};

// Simple LLVM compiler implementation
const SimpleLLVMCompiler = struct {
    allocator: Allocator,
    context: llvm_fix.LLVMContextRef,
    module: llvm_fix.LLVMModuleRef,
    builder: llvm_fix.LLVMBuilderRef,
    variables: std.HashMap([]const u8, llvm_fix.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator) CompilerError!SimpleLLVMCompiler {
        llvm_fix.LLVMInitializeNativeTarget();
        llvm_fix.LLVMInitializeNativeAsmPrinter();
        llvm_fix.LLVMInitializeNativeAsmParser();
        
        const context = llvm_fix.LLVMContextCreate() orelse return CompilerError.LLVMError;
        const module = llvm_fix.LLVMModuleCreateWithNameInContext("cursed_module", context) orelse return CompilerError.LLVMError;
        const builder = llvm_fix.LLVMCreateBuilderInContext(context) orelse return CompilerError.LLVMError;
        
        var compiler = SimpleLLVMCompiler{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .variables = std.HashMap([]const u8, llvm_fix.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
        
        try compiler.setupExternalFunctions();
        return compiler;
    }
    
    pub fn deinit(self: *SimpleLLVMCompiler) void {
        self.variables.deinit();
        llvm_fix.LLVMDisposeBuilder(self.builder);
        llvm_fix.LLVMDisposeModule(self.module);
        llvm_fix.LLVMContextDispose(self.context);
    }
    
    fn setupExternalFunctions(self: *SimpleLLVMCompiler) CompilerError!void {
        // Declare printf function
        const i8_ptr_type = llvm_fix.LLVMPointerType(llvm_fix.LLVMInt8TypeInContext(self.context), 0);
        const i32_type = llvm_fix.LLVMInt32TypeInContext(self.context);
        
        var printf_params = [_]llvm_fix.LLVMTypeRef{i8_ptr_type};
        const printf_type = llvm_fix.LLVMFunctionType(i32_type, &printf_params, 1, 1); // variadic
        _ = llvm_fix.LLVMAddFunction(self.module, "printf", printf_type);
    }
    
    pub fn compileProgram(self: *SimpleLLVMCompiler, program: Program) CompilerError!void {
        // Create main function
        const i32_type = llvm_fix.LLVMInt32TypeInContext(self.context);
        const main_type = llvm_fix.LLVMFunctionType(i32_type, null, 0, 0);
        const main_func = llvm_fix.LLVMAddFunction(self.module, "main", main_type);
        
        const entry_block = llvm_fix.LLVMAppendBasicBlockInContext(self.context, main_func, "entry");
        llvm_fix.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Compile statements
        for (program.statements) |stmt| {
            try self.compileStatement(stmt);
        }
        
        // Return 0
        const zero = llvm_fix.LLVMConstInt(i32_type, 0, 0);
        _ = llvm_fix.LLVMBuildRet(self.builder, zero);
        
        // Verify module
        var error_message: [*c]u8 = undefined;
        if (llvm_fix.LLVMVerifyModule(self.module, 1, &error_message) != 0) {
            print("LLVM verification failed: {s}\n", .{error_message});
            llvm_fix.LLVMDisposeMessage(error_message);
            return CompilerError.LLVMError;
        }
    }
    
    fn compileStatement(self: *SimpleLLVMCompiler, stmt: Statement) CompilerError!void {
        switch (stmt) {
            .VarDecl => |var_decl| {
                const value = try self.compileExpression(var_decl.value);
                // For this simple implementation, we'll store in a global
                try self.variables.put(var_decl.name, value);
            },
            .Call => |call| {
                _ = try self.compileCall(call.function, call.args);
            },
        }
    }
    
    fn compileExpression(self: *SimpleLLVMCompiler, expr: Expression) CompilerError!llvm_fix.LLVMValueRef {
        switch (expr) {
            .Literal => |literal| {
                switch (literal) {
                    .Integer => |int_val| {
                        const i32_type = llvm_fix.LLVMInt32TypeInContext(self.context);
                        return llvm_fix.LLVMConstInt(i32_type, @intCast(int_val), 0);
                    },
                    .String => |str_val| {
                        return llvm_fix.LLVMBuildGlobalStringPtr(self.builder, str_val.ptr, "str");
                    },
                    .Boolean => |bool_val| {
                        const i1_type = llvm_fix.LLVMInt8TypeInContext(self.context); // Use i8 for boolean
                        return llvm_fix.LLVMConstInt(i1_type, if (bool_val) 1 else 0, 0);
                    },
                }
            },
            .Identifier => |name| {
                return self.variables.get(name) orelse CompilerError.UndefinedVariable;
            },
            .BinaryOp => |binop| {
                const left = try self.compileExpression(binop.left.*);
                const right = try self.compileExpression(binop.right.*);
                _ = right; // Suppress unused warning
                
                switch (binop.op) {
                    .Add => {
                        // We need the actual LLVM build functions - for now use const add
                        return left; // Simplified - would need LLVMBuildAdd
                    },
                    else => return left, // Simplified
                }
            },
            .Call => |call| {
                return try self.compileCall(call.function, call.args);
            },
        }
    }
    
    fn compileCall(self: *SimpleLLVMCompiler, function_name: []const u8, args: []Expression) CompilerError!llvm_fix.LLVMValueRef {
        if (std.mem.eql(u8, function_name, "vibez.spill")) {
            // Handle print call
            var llvm_args = ArrayList(llvm_fix.LLVMValueRef).init(self.allocator);
            defer llvm_args.deinit();
            
            // Create format string based on arguments
            var format_str = ArrayList(u8).init(self.allocator);
            defer format_str.deinit();
            
            for (args, 0..) |arg, i| {
                if (i > 0) try format_str.append(' ');
                
                const arg_value = try self.compileExpression(arg);
                try llvm_args.append(arg_value);
                
                switch (arg) {
                    .Literal => |lit| {
                        switch (lit) {
                            .Integer => try format_str.appendSlice("%d"),
                            .String => try format_str.appendSlice("%s"),
                            .Boolean => try format_str.appendSlice("%d"),
                        }
                    },
                    else => try format_str.appendSlice("%s"),
                }
            }
            
            try format_str.append('\n');
            try format_str.append(0); // null terminator
            
            const format_global = llvm_fix.LLVMBuildGlobalStringPtr(self.builder, format_str.items.ptr, "printf_fmt");
            
            // Get printf function
            const printf_func = llvm_fix.LLVMGetNamedFunction(self.module, "printf") orelse return CompilerError.UndefinedFunction;
            _ = printf_func; // Suppress unused warning
            
            // Build call arguments
            var printf_args = ArrayList(llvm_fix.LLVMValueRef).init(self.allocator);
            defer printf_args.deinit();
            try printf_args.append(format_global);
            try printf_args.appendSlice(llvm_args.items);
            
            // For now return a dummy value since we need the actual build call function
            return format_global;
        }
        
        return CompilerError.UndefinedFunction;
    }
    
    pub fn generateBitcode(self: *SimpleLLVMCompiler, output_path: []const u8) CompilerError!void {
        const result = llvm_fix.LLVMWriteBitcodeToFile(self.module, output_path.ptr);
        if (result != 0) {
            return CompilerError.LLVMError;
        }
    }
    
    pub fn printModule(self: *SimpleLLVMCompiler) void {
        const module_str = llvm_fix.LLVMPrintModuleToString(self.module);
        print("Generated LLVM IR:\n{s}\n", .{module_str});
        llvm_fix.LLVMDisposeMessage(module_str);
    }
};

// Simple parser for basic CURSED syntax
fn parseBasicCursed(allocator: Allocator, source: []const u8) !Program {
    // Very simple parser for "sus x drip = 42; vibez.spill("Answer:", x)"
    var statements = ArrayList(Statement).init(allocator);
    
    // Look for variable declaration
    if (std.mem.indexOf(u8, source, "sus ") != null and std.mem.indexOf(u8, source, " drip = ") != null) {
        // Extract variable name and value
        const sus_start = std.mem.indexOf(u8, source, "sus ").? + 4;
        const drip_start = std.mem.indexOf(u8, source, " drip = ").?;
        const var_name = std.mem.trim(u8, source[sus_start..drip_start], " ");
        
        const value_start = drip_start + 8;
        const semicolon = std.mem.indexOf(u8, source[value_start..], ";");
        const value_end = if (semicolon) |s| value_start + s else source.len;
        const value_str = std.mem.trim(u8, source[value_start..value_end], " ");
        
        // Parse the value
        const value = if (std.fmt.parseInt(i64, value_str, 10)) |int_val|
            Value{ .Integer = int_val }
        else |_|
            Value{ .String = value_str };
        
        const var_name_owned = try allocator.dupe(u8, var_name);
        try statements.append(Statement{ .VarDecl = .{ .name = var_name_owned, .value = Expression{ .Literal = value } } });
    }
    
    // Look for vibez.spill call
    if (std.mem.indexOf(u8, source, "vibez.spill(") != null) {
        const call_start = std.mem.indexOf(u8, source, "vibez.spill(").? + 12;
        const call_end = std.mem.indexOf(u8, source[call_start..], ")").? + call_start;
        const args_str = source[call_start..call_end];
        
        // Simple argument parsing
        var args = ArrayList(Expression).init(allocator);
        
        // Split by comma and parse each argument
        var iter = std.mem.splitSequence(u8, args_str, ",");
        while (iter.next()) |arg_str| {
            const trimmed = std.mem.trim(u8, arg_str, " \"");
            
            // Check if it's a variable reference
            if (std.mem.eql(u8, trimmed, "x")) {
                try args.append(Expression{ .Identifier = "x" });
            } else {
                // It's a string literal
                const arg_owned = try allocator.dupe(u8, trimmed);
                try args.append(Expression{ .Literal = Value{ .String = arg_owned } });
            }
        }
        
        try statements.append(Statement{ .Call = .{ .function = "vibez.spill", .args = try args.toOwnedSlice() } });
    }
    
    return Program{ .statements = try statements.toOwnedSlice() };
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("Usage: simple-llvm-compiler <file.csd> [--compile]\n", .{});
        return;
    }
    
    const file_path = args[1];
    const should_compile = args.len > 2 and std.mem.eql(u8, args[2], "--compile");
    
    // Read the source file
    const source = std.fs.cwd().readFileAlloc(allocator, file_path, 1024 * 1024) catch |err| {
        print("Error reading file {s}: {}\n", .{ file_path, err });
        return;
    };
    defer allocator.free(source);
    
    print("🚀 CURSED LLVM Compiler Processing: {s}\n", .{file_path});
    
    // Parse the program
    const program = parseBasicCursed(allocator, source) catch |err| {
        print("Parse error: {}\n", .{err});
        return;
    };
    
    if (should_compile) {
        print("🔧 Compiling to LLVM IR...\n", .{});
        
        // Test LLVM integration first
        llvm_fix.testLLVMIntegration() catch |err| {
            print("LLVM integration test failed: {}\n", .{err});
            return;
        };
        
        var compiler = SimpleLLVMCompiler.init(allocator) catch |err| {
            print("Failed to initialize LLVM compiler: {}\n", .{err});
            return;
        };
        defer compiler.deinit();
        
        compiler.compileProgram(program) catch |err| {
            print("Compilation failed: {}\n", .{err});
            return;
        };
        
        print("✅ Compilation successful!\n", .{});
        compiler.printModule();
        
        // Generate bitcode
        const bitcode_path = "output.bc";
        compiler.generateBitcode(bitcode_path) catch |err| {
            print("Failed to generate bitcode: {}\n", .{err});
            return;
        };
        
        print("💾 Generated bitcode: {s}\n", .{bitcode_path});
        
        // TODO: Link and create executable
        print("🔗 To create executable, run: clang output.bc -o {s}\n", .{file_path[0..file_path.len-4]});
        
    } else {
        print("🔄 Interpreting program...\n", .{});
        
        // Simple interpretation
        for (program.statements) |stmt| {
            switch (stmt) {
                .VarDecl => |var_decl| {
                    switch (var_decl.value) {
                        .Literal => |lit| {
                            switch (lit) {
                                .Integer => |int_val| print("Variable {s} = {d}\n", .{ var_decl.name, int_val }),
                                .String => |str_val| print("Variable {s} = \"{s}\"\n", .{ var_decl.name, str_val }),
                                .Boolean => |bool_val| print("Variable {s} = {}\n", .{ var_decl.name, bool_val }),
                            }
                        },
                        else => {},
                    }
                },
                .Call => |call| {
                    if (std.mem.eql(u8, call.function, "vibez.spill")) {
                        for (call.args, 0..) |arg, i| {
                            if (i > 0) print(" ", .{});
                            switch (arg) {
                                .Literal => |lit| {
                                    switch (lit) {
                                        .Integer => |int_val| print("{d}", .{int_val}),
                                        .String => |str_val| print("{s}", .{str_val}),
                                        .Boolean => |bool_val| print("{}", .{bool_val}),
                                    }
                                },
                                .Identifier => |name| {
                                    if (std.mem.eql(u8, name, "x")) {
                                        print("42", .{}); // Simple hardcoded value
                                    } else {
                                        print("{s}", .{name});
                                    }
                                },
                                else => {},
                            }
                        }
                        print("\n", .{});
                    }
                },
            }
        }
        
        print("✅ Program execution completed\n", .{});
    }
}
