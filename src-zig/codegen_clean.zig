const std = @import("std");
const ast = @import("ast.zig");
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
    LLVMError,
    CompilationError,
    LinkerError,
    OutOfMemory,
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
        
        // Set up runtime functions needed for compilation
        try self.setupRuntimeFunctions();
        
        return self;
    }

    /// Set up essential runtime functions like printf, malloc, etc.
    fn setupRuntimeFunctions(self: *CodeGenerator) !void {
        // printf function declaration
        const i32_type = c.LLVMInt32TypeInContext(self.context);
        const i8_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        
        var printf_params = [_]c.LLVMTypeRef{i8_ptr_type};
        const printf_type = c.LLVMFunctionType(i32_type, &printf_params, 1, 1); // variadic
        _ = c.LLVMAddFunction(self.module, "printf", printf_type);
        
        // puts function declaration (simpler alternative)
        const puts_type = c.LLVMFunctionType(i32_type, &printf_params, 1, 0);
        _ = c.LLVMAddFunction(self.module, "puts", puts_type);
        
        // malloc function declaration
        const size_t_type = c.LLVMInt64TypeInContext(self.context);
        var malloc_params = [_]c.LLVMTypeRef{size_t_type};
        const malloc_type = c.LLVMFunctionType(i8_ptr_type, &malloc_params, 1, 0);
        _ = c.LLVMAddFunction(self.module, "malloc", malloc_type);
        
        // free function declaration
        var free_params = [_]c.LLVMTypeRef{i8_ptr_type};
        const free_type = c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), &free_params, 1, 0);
        _ = c.LLVMAddFunction(self.module, "free", free_type);
    }

    pub fn deinit(self: *CodeGenerator) void {
        self.variables.deinit();
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
        self.allocator.destroy(self);
    }

    pub fn generateProgram(self: *CodeGenerator, program: ast.Program) !void {
        // Create main function that wraps the program
        const i32_type = c.LLVMInt32TypeInContext(self.context);
        const main_type = c.LLVMFunctionType(i32_type, null, 0, 0);
        const main_function = c.LLVMAddFunction(self.module, "main", main_type);
        
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Generate all statements inside main function
        for (program.statements) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Return 0 from main
        const return_value = c.LLVMConstInt(i32_type, 0, 0);
        _ = c.LLVMBuildRet(self.builder, return_value);
    }

    fn generateStatement(self: *CodeGenerator, stmt: ast.Statement) !void {
        switch (stmt) {
            .Function => |func| {
                try self.generateFunction(func);
            },
            .Variable => |var_decl| {
                try self.generateVariable(var_decl);
            },
            .Expression => |expr| {
                _ = try self.generateExpression(expr);
            },
            .If => |if_stmt| {
                try self.generateIfStatement(if_stmt);
            },
            .While => |while_stmt| {
                try self.generateWhileStatement(while_stmt);
            },
            .For => |for_stmt| {
                try self.generateForStatement(for_stmt);
            },
            .Return => |return_stmt| {
                try self.generateReturnStatement(return_stmt);
            },
            .Block => |block| {
                for (block.statements.items) |block_stmt| {
                    try self.generateStatement(block_stmt.*);
                }
            },
            .Import => {
                // Import statements don't generate code
            },
            else => {
                std.debug.print("Warning: Unimplemented statement type in codegen\n", .{});
            },
        }
    }

    fn generateIfStatement(self: *CodeGenerator, if_stmt: ast.IfStatementData) !void {
        const condition_value = try self.generateExpression(if_stmt.condition.*);
        
        // Get current function
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Create basic blocks
        const then_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "if.then");
        const else_block = if (if_stmt.else_branch) |_| 
            c.LLVMAppendBasicBlockInContext(self.context, current_func, "if.else") 
        else 
            null;
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "if.merge");
        
        // Generate conditional branch
        if (else_block) |else_bb| {
            _ = c.LLVMBuildCondBr(self.builder, condition_value, then_block, else_bb);
        } else {
            _ = c.LLVMBuildCondBr(self.builder, condition_value, then_block, merge_block);
        }
        
        // Generate then block
        c.LLVMPositionBuilderAtEnd(self.builder, then_block);
        for (if_stmt.then_branch.items) |stmt| {
            try self.generateStatement(stmt.*);
        }
        // Only add branch if block doesn't end with return/break
        if (!c.LLVMGetBasicBlockTerminator(then_block)) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Generate else block if present
        if (if_stmt.else_branch) |else_stmts| {
            c.LLVMPositionBuilderAtEnd(self.builder, else_block.?);
            for (else_stmts.items) |stmt| {
                try self.generateStatement(stmt.*);
            }
            if (!c.LLVMGetBasicBlockTerminator(else_block.?)) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }
        
        // Continue with merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
    }

    fn generateWhileStatement(self: *CodeGenerator, while_stmt: ast.WhileStatementData) !void {
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        const loop_header = c.LLVMAppendBasicBlockInContext(self.context, current_func, "while.header");
        const loop_body = c.LLVMAppendBasicBlockInContext(self.context, current_func, "while.body");
        const loop_exit = c.LLVMAppendBasicBlockInContext(self.context, current_func, "while.exit");
        
        // Jump to header
        _ = c.LLVMBuildBr(self.builder, loop_header);
        
        // Generate header with condition
        c.LLVMPositionBuilderAtEnd(self.builder, loop_header);
        const condition_value = try self.generateExpression(while_stmt.condition.*);
        _ = c.LLVMBuildCondBr(self.builder, condition_value, loop_body, loop_exit);
        
        // Generate body
        c.LLVMPositionBuilderAtEnd(self.builder, loop_body);
        for (while_stmt.body.items) |stmt| {
            try self.generateStatement(stmt.*);
        }
        if (!c.LLVMGetBasicBlockTerminator(loop_body)) {
            _ = c.LLVMBuildBr(self.builder, loop_header);
        }
        
        // Continue with exit block
        c.LLVMPositionBuilderAtEnd(self.builder, loop_exit);
    }

    fn generateForStatement(self: *CodeGenerator, for_stmt: ast.ForStatementData) !void {
        const current_func = c.LLVMGetBasicBlockParent(c.LLVMGetInsertBlock(self.builder));
        
        // Generate init statement
        if (for_stmt.init) |for_init| {
            try self.generateStatement(for_init.*);
        }
        
        const loop_header = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for.header");
        const loop_body = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for.body");
        const loop_update = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for.update");
        const loop_exit = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for.exit");
        
        // Jump to header
        _ = c.LLVMBuildBr(self.builder, loop_header);
        
        // Generate header with condition
        c.LLVMPositionBuilderAtEnd(self.builder, loop_header);
        if (for_stmt.condition) |condition| {
            const condition_value = try self.generateExpression(condition.*);
            _ = c.LLVMBuildCondBr(self.builder, condition_value, loop_body, loop_exit);
        } else {
            // Infinite loop if no condition
            _ = c.LLVMBuildBr(self.builder, loop_body);
        }
        
        // Generate body
        c.LLVMPositionBuilderAtEnd(self.builder, loop_body);
        for (for_stmt.body.items) |stmt| {
            try self.generateStatement(stmt.*);
        }
        if (!c.LLVMGetBasicBlockTerminator(loop_body)) {
            _ = c.LLVMBuildBr(self.builder, loop_update);
        }
        
        // Generate update
        c.LLVMPositionBuilderAtEnd(self.builder, loop_update);
        if (for_stmt.update) |update| {
            try self.generateStatement(update.*);
        }
        if (!c.LLVMGetBasicBlockTerminator(loop_update)) {
            _ = c.LLVMBuildBr(self.builder, loop_header);
        }
        
        // Continue with exit block
        c.LLVMPositionBuilderAtEnd(self.builder, loop_exit);
    }

    fn generateReturnStatement(self: *CodeGenerator, return_stmt: ast.ReturnStatementData) !void {
        if (return_stmt.value) |value| {
            const return_value = try self.generateExpression(value.*);
            _ = c.LLVMBuildRet(self.builder, return_value);
        } else {
            _ = c.LLVMBuildRetVoid(self.builder);
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

    /// Generate native executable from LLVM module
    pub fn writeExecutable(self: *CodeGenerator, output_path: []const u8) CodeGenError!void {
        // Initialize LLVM targets and execution engine
        c.LLVMInitializeAllTargetInfos();
        c.LLVMInitializeAllTargets();
        c.LLVMInitializeAllTargetMCs();
        c.LLVMInitializeAllAsmParsers();
        c.LLVMInitializeAllAsmPrinters();

        // Write LLVM IR to file for debugging
        var ir_filename = std.ArrayList(u8).init(self.allocator);
        defer ir_filename.deinit();
        
        try ir_filename.appendSlice(output_path);
        try ir_filename.appendSlice(".ll");
        
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMPrintModuleToFile(self.module, ir_filename.items.ptr, &error_msg) != 0) {
            std.debug.print("Failed to write LLVM IR: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }

        // Get native target triple for current platform
        const target_triple = c.LLVMGetDefaultTargetTriple();
        defer c.LLVMDisposeMessage(target_triple);

        // Get target from triple
        var llvm_target: c.LLVMTargetRef = undefined;
        if (c.LLVMGetTargetFromTriple(target_triple, &llvm_target, &error_msg) != 0) {
            std.debug.print("Failed to get target: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }

        // Create target machine
        const target_machine = c.LLVMCreateTargetMachine(
            llvm_target,
            target_triple,
            "generic", // CPU
            "", // Features
            c.LLVMCodeGenLevelDefault,
            c.LLVMRelocDefault,
            c.LLVMCodeModelDefault
        );
        defer c.LLVMDisposeTargetMachine(target_machine);

        if (target_machine == null) {
            std.debug.print("Failed to create target machine\n", .{});
            return CodeGenError.LLVMError;
        }

        // Generate object file
        var obj_filename = std.ArrayList(u8).init(self.allocator);
        defer obj_filename.deinit();
        try obj_filename.appendSlice(output_path);
        try obj_filename.appendSlice(".o");

        if (c.LLVMTargetMachineEmitToFile(target_machine, self.module, obj_filename.items.ptr, c.LLVMObjectFile, &error_msg) != 0) {
            std.debug.print("Failed to emit object file: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }

        // Link object file to executable using system linker
        try self.linkToExecutable(obj_filename.items, output_path);

        std.debug.print("✅ Generated executable: {s}\n", .{output_path});
        std.debug.print("🔧 LLVM IR written to: {s}\n", .{ir_filename.items});
        std.debug.print("🔗 Object file: {s}\n", .{obj_filename.items});
    }

    /// Link object file to executable using system linker
    fn linkToExecutable(self: *CodeGenerator, obj_path: []const u8, output_path: []const u8) CodeGenError!void {
        const allocator = self.allocator;
        
        // Detect platform and use appropriate linker
        const is_macos = std.builtin.os.tag == .macos;
        const is_windows = std.builtin.os.tag == .windows;
        
        var link_args = std.ArrayList([]const u8).init(allocator);
        defer link_args.deinit();
        
        if (is_windows) {
            // Windows: use link.exe or ld
            try link_args.append("ld");
            try link_args.append("-o");
            try link_args.append(output_path);
            try link_args.append(obj_path);
            try link_args.append("-lc");
        } else if (is_macos) {
            // macOS: use ld
            try link_args.append("ld");
            try link_args.append("-o");
            try link_args.append(output_path);
            try link_args.append(obj_path);
            try link_args.append("-lSystem");
            try link_args.append("-arch");
            try link_args.append("x86_64");
        } else {
            // Linux: use ld or gcc
            try link_args.append("gcc");
            try link_args.append("-o");
            try link_args.append(output_path);
            try link_args.append(obj_path);
            try link_args.append("-no-pie"); // Disable PIE for compatibility
        }

        // Execute linker
        var child = std.ChildProcess.init(link_args.items, allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;
        
        const result = child.spawnAndWait() catch |err| {
            std.debug.print("Failed to spawn linker: {}\n", .{err});
            return CodeGenError.LinkerError;
        };
        
        switch (result) {
            .Exited => |code| {
                if (code != 0) {
                    std.debug.print("Linker failed with exit code: {}\n", .{code});
                    return CodeGenError.LinkerError;
                }
            },
            else => {
                std.debug.print("Linker process terminated abnormally\n", .{});
                return CodeGenError.LinkerError;
            }
        }
    }

    pub fn generateBitcode(self: *CodeGenerator, output_path: []const u8) !void {
        if (c.LLVMWriteBitcodeToFile(self.module, output_path.ptr) != 0) {
            return error.BitcodeWriteFailed;
        }
    }
};
