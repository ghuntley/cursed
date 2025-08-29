const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Import the fixed LLVM backend with proper CPU detection
const llvm_backend_fixed = @import("llvm_backend_fixed.zig");
const LLVMBackendFixed = llvm_backend_fixed.LLVMBackendFixed;

const ast = @import("ast.zig");
const Variable = @import("Variable.zig");

// Advanced LLVM code generator using full LLVM C API (athlon-xp issue fixed)
pub const AdvancedCodeGenFull = struct {
    allocator: Allocator,
    llvm_backend: LLVMBackendFixed,
    output_file: []const u8,
    
    pub fn init(allocator: Allocator, module_name: []const u8, output_file: []const u8) !AdvancedCodeGenFull {
        const llvm_backend = try LLVMBackendFixed.init(allocator, module_name);
        
        return AdvancedCodeGenFull{
            .allocator = allocator,
            .llvm_backend = llvm_backend,
            .output_file = output_file,
        };
    }
    
    pub fn deinit(self: *AdvancedCodeGenFull) void {
        self.llvm_backend.deinit(self.allocator);
    }
    
    pub fn compileProgram(self: *AdvancedCodeGenFull, statements: []const ast.Statement, variables: std.StringHashMap(Variable)) !void {
        // Create main function
        const int32_type = self.llvm_backend.getInt32Type();
        const main_func = try self.llvm_backend.createFunction("main", int32_type, &[_]anyopaque{}, false);
        
        const entry_block = try self.llvm_backend.createBasicBlock(main_func, "entry");
        self.llvm_backend.positionBuilder(entry_block);
        
        // Generate code for each statement
        for (statements) |statement| {
            try self.compileStatement(statement, variables);
        }
        
        // Return 0 from main
        const return_value = self.llvm_backend.buildConstInt(int32_type, 0);
        _ = self.llvm_backend.buildRet(return_value);
        
        // Verify and save module
        try self.llvm_backend.verifyModule();
        try self.llvm_backend.printModuleToFile(self.output_file);
        
        // Also write bitcode
        const bc_file = try std.mem.concat(self.allocator, u8, &[_][]const u8{ self.output_file[0..self.output_file.len-3], ".bc" });
        defer self.allocator.free(bc_file);
        try self.llvm_backend.writeBitcodeToFile(bc_file);
    }
    
    fn compileStatement(self: *AdvancedCodeGenFull, statement: ast.Statement, variables: std.StringHashMap(Variable)) !void {
        switch (statement) {
            .Expression => |expr| {
                _ = try self.compileExpression(expr, variables);
            },
            .Declaration => |decl| {
                try self.compileDeclaration(decl, variables);
            },
            .Assignment => |assign| {
                try self.compileAssignment(assign, variables);
            },
            .Print => |print_stmt| {
                try self.compilePrint(print_stmt, variables);
            },
            else => {
                // Handle other statement types as needed
                std.debug.print("Statement type not yet implemented in full LLVM codegen\n", .{});
            },
        }
    }
    
    fn compileExpression(self: *AdvancedCodeGenFull, expr: ast.Expression, variables: std.StringHashMap(Variable)) !anyopaque {
        switch (expr) {
            .Literal => |lit| {
                return self.compileLiteral(lit);
            },
            .Identifier => |ident| {
                return self.compileIdentifier(ident, variables);
            },
            .BinaryOperation => |binop| {
                return self.compileBinaryOperation(binop, variables);
            },
            else => {
                std.debug.print("Expression type not yet implemented in full LLVM codegen\n", .{});
                return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), 0);
            },
        }
    }
    
    fn compileLiteral(self: *AdvancedCodeGenFull, literal: ast.Literal) anyopaque {
        switch (literal) {
            .Integer => |val| {
                return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), @intCast(val));
            },
            .String => |str| {
                return self.llvm_backend.buildConstString(str, "str_const") catch {
                    return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), 0);
                };
            },
            .Boolean => |val| {
                return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), if (val) 1 else 0);
            },
            else => {
                return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), 0);
            },
        }
    }
    
    fn compileIdentifier(self: *AdvancedCodeGenFull, identifier: []const u8, variables: std.StringHashMap(Variable)) anyopaque {
        if (variables.get(identifier)) |variable| {
            // Convert variable value to LLVM value
            switch (variable) {
                .Integer => |val| {
                    return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), @intCast(val));
                },
                .String => |str| {
                    return self.llvm_backend.buildConstString(str, "var_str") catch {
                        return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), 0);
                    };
                },
                .Boolean => |val| {
                    return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), if (val) 1 else 0);
                },
                else => {
                    return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), 0);
                },
            }
        } else {
            std.debug.print("Unknown variable: {s}\n", .{identifier});
            return self.llvm_backend.buildConstInt(self.llvm_backend.getInt32Type(), 0);
        }
    }
    
    fn compileBinaryOperation(self: *AdvancedCodeGenFull, binop: ast.BinaryOperation, variables: std.StringHashMap(Variable)) !anyopaque {
        const lhs = try self.compileExpression(binop.left.*, variables);
        const rhs = try self.compileExpression(binop.right.*, variables);
        
        // For now, only handle addition of integers
        if (std.mem.eql(u8, binop.operator, "+")) {
            return try self.llvm_backend.buildAdd(lhs, rhs, "add_result");
        }
        
        // Return lhs for other operations (placeholder)
        return lhs;
    }
    
    fn compileDeclaration(self: *AdvancedCodeGenFull, decl: ast.Declaration, variables: std.StringHashMap(Variable)) !void {
        _ = decl;
        _ = variables;
        // Declaration compilation not yet implemented
        std.debug.print("Declaration compilation not yet implemented\n", .{});
    }
    
    fn compileAssignment(self: *AdvancedCodeGenFull, assign: ast.Assignment, variables: std.StringHashMap(Variable)) !void {
        _ = assign;
        _ = variables;
        // Assignment compilation not yet implemented  
        std.debug.print("Assignment compilation not yet implemented\n", .{});
    }
    
    fn compilePrint(self: *AdvancedCodeGenFull, print_stmt: ast.Print, variables: std.StringHashMap(Variable)) !void {
        // Generate printf call for print statement
        const int8_ptr_type = self.llvm_backend.getInt8PtrType();
        const int32_type = self.llvm_backend.getInt32Type();
        
        // Create printf function declaration if not exists
        const printf_func_type = [_]anyopaque{int8_ptr_type}; // printf(const char*, ...)
        const printf_func = try self.llvm_backend.createFunction("printf", int32_type, &printf_func_type, true);
        
        // Compile print arguments
        var args = std.ArrayList(u8){};
        defer args.deinit();
        
        // Format string - for now just use "%s\n" or "%d\n" 
        const format_str = try self.llvm_backend.buildConstString("%d\n", "fmt_str");
        try args.append(allocator, format_str);
        
        // Add the value to print
        for (print_stmt.arguments) |arg| {
            const arg_value = try self.compileExpression(arg, variables);
            try args.append(allocator, arg_value);
        }
        
        // Generate printf call
        _ = try self.llvm_backend.buildCall(printf_func, args.items, "printf_call");
    }
};

// Test function to verify full LLVM codegen works
pub fn testAdvancedCodeGenFull(allocator: Allocator) !void {
        _ = allocator;
    std.debug.print("Testing advanced codegen with full LLVM backend...\n", .{});
    
    var codegen = try AdvancedCodeGenFull.init(allocator, "test_full_codegen", "test_full_codegen.ll");
    defer codegen.deinit();
    
    // Create a simple test program
    var variables = std.StringHashMap(Variable){};
    defer variables.deinit();
    
    try variables.put("x", Variable{ .Integer = 42 });
    
    // Create simple statements (this would normally come from the parser)
    const statements = [_]ast.Statement{
        // This is a placeholder - real statements would be generated by the parser
    };
    
    try codegen.compileProgram(&statements, variables);
    
    std.debug.print("✅ Full LLVM codegen test completed!\n", .{});
}
