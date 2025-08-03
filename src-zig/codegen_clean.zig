const std = @import("std");
const ast = @import("ast_simple.zig");
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/TargetMachine.h");
});

const CodeGenError = error{
    UndefinedSymbol,
    TypeMismatch,
    InvalidOperation,
    UnknownType,
};

pub const CodeGenerator = struct {
    allocator: std.mem.Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    variables: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn init(allocator: std.mem.Allocator) !*CodeGenerator {
        const self = try allocator.create(CodeGenerator);
        
        self.allocator = allocator;
        self.context = c.LLVMContextCreate();
        self.module = c.LLVMModuleCreateWithNameInContext("cursed_module", self.context);
        self.builder = c.LLVMCreateBuilderInContext(self.context);
        self.variables = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
        
        return self;
    }

    pub fn deinit(self: *CodeGenerator) void {
        self.variables.deinit();
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
        self.allocator.destroy(self);
    }

    pub fn generateProgram(self: *CodeGenerator, program: ast.Program) !void {
        for (program.statements) |stmt| {
            try self.generateStatement(stmt);
        }
    }

    fn generateStatement(self: *CodeGenerator, stmt: ast.Statement) !void {
        switch (stmt) {
            .Function => |func| {
                try self.generateFunction(func);
            },
            .Variable => |var_decl| {
                try self.generateVariable(var_decl);
            },
            else => {
                // TODO: Implement other statement types
            },
        }
    }

    fn generateFunction(self: *CodeGenerator, func: ast.FunctionStatement) !void {
        // Create function type
        const return_type = self.getLLVMType("normie"); // Default to int for now
        const param_types = try self.allocator.alloc(c.LLVMTypeRef, func.parameters.len);
        defer self.allocator.free(param_types);
        
        for (func.parameters, 0..) |param, i| {
            param_types[i] = self.getLLVMType(param.type_name);
        }
        
        const func_type = c.LLVMFunctionType(return_type, param_types.ptr, @as(c_uint, @intCast(param_types.len)), 0);
        const llvm_func = c.LLVMAddFunction(self.module, func.name.ptr, func_type);
        
        // Create basic block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, llvm_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Generate function body
        for (func.body) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Add return if not present
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
    }

    fn generateVariable(self: *CodeGenerator, var_decl: ast.VariableStatement) !void {
        const llvm_type = self.getLLVMType(var_decl.type_name);
        const alloca = c.LLVMBuildAlloca(self.builder, llvm_type, var_decl.name.ptr);
        
        if (var_decl.value) |value| {
            const init_value = try self.generateExpression(value);
            _ = c.LLVMBuildStore(self.builder, init_value, alloca);
        }
        
        try self.variables.put(var_decl.name, alloca);
    }

    fn generateExpression(self: *CodeGenerator, expr: ast.Expression) !c.LLVMValueRef {
        return switch (expr) {
            .Integer => |int| c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @as(c_ulonglong, @intCast(int)), 1),
            .Float => |float| c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float),
            .String => |str| c.LLVMBuildGlobalStringPtr(self.builder, str.ptr, "str"),
            .Boolean => |bool_val| c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0),
            .Identifier => |name| {
                if (self.variables.get(name)) |variable| {
                    return c.LLVMBuildLoad2(
                        self.builder,
                        c.LLVMGetElementType(c.LLVMTypeOf(variable)),
                        variable,
                        name.ptr
                    );
                }
                return error.UndefinedSymbol;
            },
            .Call => |call| self.generateCall(call),
            else => c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 1), // Placeholder
        };
    }

    fn generateCall(self: *CodeGenerator, call: ast.CallExpression) !c.LLVMValueRef {
        // Handle built-in functions
        if (std.mem.eql(u8, call.function, "vibez.spill")) {
            // Simple print implementation
            const args = try self.allocator.alloc(c.LLVMValueRef, call.arguments.len);
            defer self.allocator.free(args);
            
            for (call.arguments, 0..) |arg, i| {
                args[i] = try self.generateExpression(arg);
            }
            
            // Create printf call
            const printf_type = c.LLVMFunctionType(
                c.LLVMInt32TypeInContext(self.context),
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)},
                1,
                1
            );
            const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
            
            return c.LLVMBuildCall2(self.builder, printf_type, printf_func, args.ptr, @as(c_uint, @intCast(args.len)), "printf_call");
        }
        
        return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 1);
    }

    fn getLLVMType(self: *CodeGenerator, cursed_type: []const u8) c.LLVMTypeRef {
        if (std.mem.eql(u8, cursed_type, "normie")) {
            return c.LLVMInt32TypeInContext(self.context);
        } else if (std.mem.eql(u8, cursed_type, "meal")) {
            return c.LLVMDoubleTypeInContext(self.context);
        } else if (std.mem.eql(u8, cursed_type, "tea")) {
            return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        } else if (std.mem.eql(u8, cursed_type, "lit")) {
            return c.LLVMInt1TypeInContext(self.context);
        }
        return c.LLVMVoidTypeInContext(self.context);
    }

    pub fn generateBitcode(self: *CodeGenerator, output_path: []const u8) !void {
        if (c.LLVMWriteBitcodeToFile(self.module, output_path.ptr) != 0) {
            return error.BitcodeWriteFailed;
        }
    }
};
