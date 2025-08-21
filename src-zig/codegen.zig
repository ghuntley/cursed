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

// ... [keeping just the essential parts of the CodeGen struct and removing duplicates]

pub const CodeGen = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    functions: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    runtime_functions: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn init() CodeGen {
        return CodeGen{
            .allocator = allocator,
            .context = c.LLVMContextCreate(),
            .module = null,
            .builder = null,
            .functions = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .runtime_functions = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }

    pub fn deinit(self: *CodeGen) void {
        self.functions.deinit();
        self.variables.deinit();
        self.runtime_functions.deinit();
        
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
        // Basic implementation - just create a simple main function
        self.module = c.LLVMModuleCreateWithNameInContext("cursed_module", self.context);
        if (self.module == null) return CodeGenError.ModuleCreationFailed;
        
        self.builder = c.LLVMCreateBuilderInContext(self.context);
        if (self.builder == null) return CodeGenError.BuilderCreationFailed;
        
        // Create main function
        const main_type = c.LLVMFunctionType(c.LLVMInt32TypeInContext(self.context), null, 0, 0);
        const main_func = c.LLVMAddFunction(self.module, "main", main_type);
        const entry_bb = c.LLVMAppendBasicBlockInContext(self.context, main_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_bb);
        
        // Generate code for each statement
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Return 0 from main
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
    }

    fn generateStatement(self: *CodeGen, stmt: ast.Statement) CodeGenError!void {
        switch (stmt) {
            .Let => |let| try self.generateLet(let),
            .Expression => |expr| {
                _ = try self.generateExpression(expr.*);
            },
            else => {
                // Basic implementations for other statements
            },
        }
    }

    fn generateLet(self: *CodeGen, let_stmt: ast.LetStatement) CodeGenError!void {
        const var_type = c.LLVMInt64TypeInContext(self.context); // default to i64
        const alloca = c.LLVMBuildAlloca(self.builder, var_type, let_stmt.name.ptr);
        
        if (let_stmt.value) |value_expr| {
            const value = try self.generateExpression(value_expr.*);
            _ = c.LLVMBuildStore(self.builder, value, alloca);
        }
        
        try self.variables.put(let_stmt.name, alloca);
    }

    fn generateExpression(self: *CodeGen, expr: ast.Expression) CodeGenError!c.LLVMValueRef {
        switch (expr) {
            .Literal => |literal| {
                switch (literal) {
                    .IntegerLiteral => |int| {
                        return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @intCast(int), 0);
                    },
                    .StringLiteral => |str| {
                        return c.LLVMBuildGlobalStringPtr(self.builder, str.ptr, "str");
                    },
                    else => return CodeGenError.UnsupportedOperation,
                }
            },
            .Identifier => |ident| {
                if (self.variables.get(ident)) |var_ref| {
                    return c.LLVMBuildLoad2(self.builder, c.LLVMInt64TypeInContext(self.context), var_ref, "load");
                }
                return CodeGenError.UndefinedSymbol;
            },
            else => return CodeGenError.UnsupportedOperation,
        }
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
