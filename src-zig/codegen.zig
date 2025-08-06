const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
    @cInclude("llvm-c/Transforms/PassManagerBuilder.h");
    @cInclude("llvm-c/Transforms/IPO.h");
    @cInclude("llvm-c/Transforms/Scalar.h");
});

const ast = @import("ast_simple.zig");
const Program = ast.Program;
const Statement = ast.Statement;
const Expression = ast.Expression;
const FunctionStatement = ast.FunctionStatement;

pub const CodeGenError = error{
    LLVMError,
    OutOfMemory,
    InvalidType,
    UndefinedSymbol,
    TypeMismatch,
};

pub const InterfaceInfo = struct {
    name: []const u8,
    methods: ArrayList(InterfaceMethod),
    vtable_type: ?c.LLVMTypeRef,
};

pub const InterfaceMethod = struct {
    name: []const u8,
    index: usize,
    function_type: c.LLVMTypeRef,
};

pub const GoroutineInfo = struct {
    function: c.LLVMValueRef,
    stack_size: u32,
    id: u32,
};

pub const ChannelInfo = struct {
    element_type: c.LLVMTypeRef,
    channel_type: c.LLVMTypeRef,
    buffer_size: u32,
};

pub const LoopContext = struct {
    continue_block: c.LLVMBasicBlockRef,
    break_block: c.LLVMBasicBlockRef,
};

pub const DeferInfo = struct {
    cleanup_function: c.LLVMValueRef,
    cleanup_block: c.LLVMBasicBlockRef,
};

pub const CodeGen = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    
    // Symbol tables
    functions: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    struct_types: std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    interface_types: std.HashMap([]const u8, InterfaceInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // CURSED-specific runtime support
    goroutines: std.HashMap(u32, GoroutineInfo, std.hash_map.AutoContext, std.hash_map.default_max_load_percentage),
    channels: std.HashMap([]const u8, ChannelInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    loop_stack: ArrayList(LoopContext),
    defer_stack: ArrayList(DeferInfo),
    
    // Current execution context
    current_function: ?c.LLVMValueRef,
    goroutine_counter: u32,
    
    // Runtime function declarations
    runtime_functions: std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn init(allocator: Allocator) CodeGen {
        _ = c.LLVMInitializeNativeTarget();
        _ = c.LLVMInitializeNativeAsmPrinter();
        _ = c.LLVMInitializeNativeAsmParser();
        
        const context = c.LLVMContextCreate();
        const module = c.LLVMModuleCreateWithNameInContext("cursed_module", context);
        const builder = c.LLVMCreateBuilderInContext(context);
        
        return CodeGen{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .functions = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .struct_types = std.HashMap([]const u8, c.LLVMTypeRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .interface_types = std.HashMap([]const u8, InterfaceInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .goroutines = std.HashMap(u32, GoroutineInfo, std.hash_map.AutoContext, std.hash_map.default_max_load_percentage).init(allocator),
            .channels = std.HashMap([]const u8, ChannelInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .loop_stack = ArrayList(LoopContext).init(allocator),
            .defer_stack = ArrayList(DeferInfo).init(allocator),
            .current_function = null,
            .goroutine_counter = 0,
            .runtime_functions = std.HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }

    pub fn deinit(self: *CodeGen) void {
        self.functions.deinit();
        self.variables.deinit();
        self.struct_types.deinit();
        
        // Clean up interface types
        var interface_iter = self.interface_types.iterator();
        while (interface_iter.next()) |entry| {
            entry.value_ptr.methods.deinit();
        }
        self.interface_types.deinit();
        
        // Clean up CURSED runtime structures
        self.goroutines.deinit();
        self.channels.deinit();
        self.loop_stack.deinit();
        self.defer_stack.deinit();
        self.runtime_functions.deinit();
        
        c.LLVMDisposeBuilder(self.builder);
        c.LLVMDisposeModule(self.module);
        c.LLVMContextDispose(self.context);
    }

    pub fn generateProgram(self: *CodeGen, program: Program) CodeGenError!void {
        // Generate external declarations
        try self.generateExternalDeclarations();
        
        // Generate statements
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Add main function if it doesn't exist
        if (self.functions.get("main_character") == null) {
            try self.generateMainWrapper();
        }
        
        // Verify module
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module, c.LLVMPrintMessageAction, &error_msg) != 0) {
            std.debug.print("LLVM module verification failed: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
        
        // Run optimization passes
        try self.optimizeModule();
    }

    fn generateExternalDeclarations(self: *CodeGen) CodeGenError!void {
        // Declare printf for vibez.spill
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return type
            @as([*c]c.LLVMTypeRef, @ptrCast(@constCast(&[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}))), // char* parameter
            1, // parameter count
            1  // is variadic
        );
        const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        try self.functions.put("printf", printf_func);
        
        // Declare malloc and free for memory management
        const malloc_type = c.LLVMFunctionType(
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // return void*
            @as([*c]c.LLVMTypeRef, @ptrCast(@constCast(&[_]c.LLVMTypeRef{c.LLVMInt64TypeInContext(self.context)}))), // size_t parameter
            1, // parameter count
            0  // not variadic
        );
        const malloc_func = c.LLVMAddFunction(self.module, "malloc", malloc_type);
        try self.functions.put("malloc", malloc_func);
        
        const free_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context), // return void
            @as([*c]c.LLVMTypeRef, @ptrCast(@constCast(&[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}))), // void* parameter
            1, // parameter count
            0  // not variadic
        );
        const free_func = c.LLVMAddFunction(self.module, "free", free_type);
        try self.functions.put("free", free_func);
    }

    fn generateStatement(self: *CodeGen, stmt: Statement) CodeGenError!void {
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
            .Return => {
                const ret_stmt: *ast.ReturnStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateReturn(ret_stmt.*);
            },
            .If => {
                const if_stmt: *ast.IfStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateIf(if_stmt.*);
            },
            .While => {
                const while_stmt: *ast.WhileStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateWhile(while_stmt.*);
            },
            .Struct => {
                const struct_stmt: *ast.StructStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateStruct(struct_stmt.*);
            },
            .Interface => {
                const interface_stmt: *ast.InterfaceStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateInterface(interface_stmt.*);
            },
            .Implementation => {
                const impl_stmt: *ast.ImplementationStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateImplementation(impl_stmt.*);
            },
            .Yikes => {
                const yikes_stmt: *ast.YikesStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateYikes(yikes_stmt.*);
            },
            .Fam => {
                const fam_stmt: *ast.FamStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateFam(fam_stmt.*);
            },
            .Block => {
                const block_stmt: *ast.BlockStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateBlock(block_stmt.*);
            },
            .Assignment => {
                const assign_stmt: *ast.AssignmentStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateAssignment(assign_stmt.*);
            },
            .Stan => {
                const stan_stmt: *ast.StanStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateStan(stan_stmt.*);
            },
            .Select => {
                const select_stmt: *ast.SelectStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateSelect(select_stmt.*);
            },
            .Defer => {
                const defer_stmt: *ast.DeferStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateDefer(defer_stmt.*);
            },
            .Bestie => {
                const bestie_stmt: *ast.BestieStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateBestie(bestie_stmt.*);
            },
            .Vibes => {
                const vibes_stmt: *ast.VibesStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateVibes(vibes_stmt.*);
            },
            .Match => {
                const match_stmt: *ast.MatchStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateMatchStatement(match_stmt.*);
            },
            .For => {
                const for_stmt: *ast.ForStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateFor(for_stmt.*);
            },
            .ForIn => {
                const for_in_stmt: *ast.ForInStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateForIn(for_in_stmt.*);
            },
            .Switch => {
                const switch_stmt: *ast.SwitchStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateSwitch(switch_stmt.*);
            },
            .PatternSwitch => {
                const pattern_switch_stmt: *ast.PatternSwitchStatement = @ptrCast(@alignCast(stmt.data));
                try self.generatePatternSwitch(pattern_switch_stmt.*);
            },
            .Goroutine => {
                const goroutine_stmt: *ast.GoroutineStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateGoroutineStatement(goroutine_stmt.*);
            },
            .Channel => {
                const channel_stmt: *ast.ChannelStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateChannelStatement(channel_stmt.*);
            },
            .TypeAlias => {
                const type_alias_stmt: *ast.TypeAliasStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateTypeAlias(type_alias_stmt.*);
            },
            .Panic => {
                const panic_stmt: *ast.PanicStatement = @ptrCast(@alignCast(stmt.data));
                try self.generatePanicStatement(panic_stmt.*);
            },
            .Catch => {
                const catch_stmt: *ast.CatchStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateCatch(catch_stmt.*);
            },
            .Break => {
                const break_stmt: *ast.BreakStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateBreak(break_stmt.*);
            },
            .Continue => {
                const continue_stmt: *ast.ContinueStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateContinue(continue_stmt.*);
            },
            .Increment => {
                const inc_stmt: *ast.IncrementStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateIncrementStatement(inc_stmt.*);
            },
            .Decrement => {
                const dec_stmt: *ast.DecrementStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateDecrementStatement(dec_stmt.*);
            },
            .ShortDeclaration => {
                const short_decl_stmt: *ast.ShortDeclarationStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateShortDeclaration(short_decl_stmt.*);
            },
            .Const => {
                const const_stmt: *ast.ConstStatement = @ptrCast(@alignCast(stmt.data));
                try self.generateConst(const_stmt.*);
            },
            else => {
                std.debug.print("Unknown statement type in codegen: {}\n", .{stmt.tag});
                return CodeGenError.LLVMError;
            },
        }
    }

    fn generateFunction(self: *CodeGen, func: FunctionStatement) CodeGenError!void {
        // Create function type
        var param_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer param_types.deinit();
        
        for (func.parameters.items) |param| {
            const param_type = try self.getLLVMType(param.param_type);
            try param_types.append(param_type);
        }
        
        const return_type = if (func.return_type) |ret_type|
            try self.getLLVMType(ret_type)
        else
            c.LLVMVoidTypeInContext(self.context);
        
        const function_type = c.LLVMFunctionType(
            return_type,
            if (param_types.items.len > 0) param_types.items.ptr else null,
            @as(u32, @intCast(param_types.items.len)),
            0 // not variadic
        );
        
        // Create function
        const function = c.LLVMAddFunction(self.module, func.name.ptr, function_type);
        try self.functions.put(func.name, function);
        
        // Create entry block
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Set current function context
        const old_function = self.current_function;
        self.current_function = function;
        
        // Create parameter allocas
        for (func.parameters.items, 0..) |param, i| {
            const param_value = c.LLVMGetParam(function, @as(u32, @intCast(i)));
            const param_type = try self.getLLVMType(param.param_type);
            const alloca = c.LLVMBuildAlloca(self.builder, param_type, param.name.ptr);
            _ = c.LLVMBuildStore(self.builder, param_value, alloca);
            try self.variables.put(param.name, alloca);
        }
        
        // Generate function body
        try self.generateStatement(func.body);
        
        // Add return void if no explicit return
        const last_block = c.LLVMGetInsertBlock(self.builder);
        if (c.LLVMGetBasicBlockTerminator(last_block) == null) {
            if (func.return_type == null) {
                _ = c.LLVMBuildRetVoid(self.builder);
            } else {
                // Return default value for type
                const default_value = try self.getDefaultValue(func.return_type.?);
                _ = c.LLVMBuildRet(self.builder, default_value);
            }
        }
        
        // Restore previous function context
        self.current_function = old_function;
        
        // Clear local variables
        self.variables.clearRetainingCapacity();
    }

    fn generateMainWrapper(self: *CodeGen) CodeGenError!void {
        // Create main function that calls main_character
        const main_type = c.LLVMFunctionType(
            c.LLVMInt32TypeInContext(self.context), // return int
            null, // no parameters
            0, // parameter count
            0  // not variadic
        );
        
        const main_function = c.LLVMAddFunction(self.module, "main", main_type);
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, main_function, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        
        // Call main_character if it exists
        if (self.functions.get("main_character")) |main_char_func| {
            _ = c.LLVMBuildCall2(self.builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(main_char_func)), main_char_func, null, 0, "");
        }
        
        // Return 0
        const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
    }



    /// Generate struct type declaration
    fn generateStruct(self: *CodeGen, struct_stmt: ast.StructStatement) CodeGenError!void {
        // Create field types array
        var field_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer field_types.deinit();
        
        for (struct_stmt.fields.items) |field| {
            const field_type = try self.getLLVMType(field.field_type);
            try field_types.append(field_type);
        }
        
        // Create struct type
        const struct_type = c.LLVMStructTypeInContext(
            self.context,
            if (field_types.items.len > 0) field_types.items.ptr else null,
            @as(u32, @intCast(field_types.items.len)),
            0 // not packed
        );
        
        // Store struct type for later use
        try self.struct_types.put(struct_stmt.name, struct_type);
    }

    /// Generate interface type declaration
    fn generateInterface(self: *CodeGen, interface_stmt: ast.InterfaceStatement) CodeGenError!void {
        var interface_info = InterfaceInfo{
            .name = interface_stmt.name,
            .methods = ArrayList(InterfaceMethod).init(self.allocator),
            .vtable_type = null,
        };
        
        // Create method info for each interface method
        for (interface_stmt.methods.items, 0..) |method, i| {
            var param_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
            defer param_types.deinit();
            
            // Add 'self' parameter as first parameter
            try param_types.append(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));
            
            for (method.parameters.items) |param| {
                const param_type = try self.getLLVMType(param.param_type);
                try param_types.append(param_type);
            }
            
            const return_type = if (method.return_type) |ret_type|
                try self.getLLVMType(ret_type)
            else
                c.LLVMVoidTypeInContext(self.context);
            
            const method_type = c.LLVMFunctionType(
                return_type,
                if (param_types.items.len > 0) param_types.items.ptr else null,
                @as(u32, @intCast(param_types.items.len)),
                0
            );
            
            const interface_method = InterfaceMethod{
                .name = method.name,
                .index = i,
                .function_type = method_type,
            };
            
            try interface_info.methods.append(interface_method);
        }
        
        // Create vtable type
        const method_count = interface_info.methods.items.len;
        if (method_count > 0) {
            const func_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            interface_info.vtable_type = c.LLVMArrayType(func_ptr_type, @as(u32, @intCast(method_count)));
        }
        
        try self.interface_types.put(interface_stmt.name, interface_info);
    }

    /// Generate yikes statement (error handling/panic)
    fn generateYikes(self: *CodeGen, yikes: ast.YikesStatement) CodeGenError!void {
        // Evaluate panic condition if present
        if (yikes.condition) |condition| {
            const condition_value = try self.generateExpression(condition);
            
            // Create conditional panic
            const current_func = self.current_function.?;
            const panic_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "panic");
            const continue_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "continue");
            
            _ = c.LLVMBuildCondBr(self.builder, condition_value, panic_block, continue_block);
            
            // Generate panic block
            c.LLVMPositionBuilderAtEnd(self.builder, panic_block);
            const panic_msg = if (yikes.message) |msg| 
                c.LLVMBuildGlobalStringPtr(self.builder, msg.ptr, "panic_msg")
            else
                c.LLVMBuildGlobalStringPtr(self.builder, "panic", "panic_msg");
            
            const panic_func = self.runtime_functions.get("cursed_panic").?;
            _ = c.LLVMBuildCall2(
                self.builder,
                c.LLVMVoidTypeInContext(self.context),
                panic_func,
                &[_]c.LLVMValueRef{panic_msg},
                1,
                ""
            );
            _ = c.LLVMBuildUnreachable(self.builder);
            
            // Continue normal execution
            c.LLVMPositionBuilderAtEnd(self.builder, continue_block);
        } else {
            // Unconditional panic
            const panic_msg = if (yikes.message) |msg| 
                c.LLVMBuildGlobalStringPtr(self.builder, msg.ptr, "panic_msg")
            else
                c.LLVMBuildGlobalStringPtr(self.builder, "panic", "panic_msg");
            
            const panic_func = self.runtime_functions.get("cursed_panic").?;
            _ = c.LLVMBuildCall2(
                self.builder,
                c.LLVMVoidTypeInContext(self.context),
                panic_func,
                &[_]c.LLVMValueRef{panic_msg},
                1,
                ""
            );
            _ = c.LLVMBuildUnreachable(self.builder);
        }
    }

    /// Generate fam statement (error recovery)
    fn generateFam(self: *CodeGen, fam: ast.FamStatement) CodeGenError!void {
        const current_func = self.current_function.?;
        
        // Create blocks for error handling
        const try_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "try");
        const catch_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "catch");
        const continue_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "continue");
        
        // Generate try block
        _ = c.LLVMBuildBr(self.builder, try_block);
        c.LLVMPositionBuilderAtEnd(self.builder, try_block);
        
        // Execute statements that might fail
        for (fam.try_statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Jump to continue if no error occurred
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, continue_block);
        }
        
        // Generate catch block
        c.LLVMPositionBuilderAtEnd(self.builder, catch_block);
        
        // If error variable specified, create it
        if (fam.error_variable) |error_var| {
            // Create error value (simplified)
            const error_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            const error_alloca = c.LLVMBuildAlloca(self.builder, error_type, error_var.ptr);
            const error_msg = c.LLVMBuildGlobalStringPtr(self.builder, "error", "error_msg");
            _ = c.LLVMBuildStore(self.builder, error_msg, error_alloca);
            try self.variables.put(error_var, error_alloca);
        }
        
        // Execute recovery code
        for (fam.catch_statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        _ = c.LLVMBuildBr(self.builder, continue_block);
        
        // Continue execution after fam block
        c.LLVMPositionBuilderAtEnd(self.builder, continue_block);
    }

    fn getLLVMTypeFromString(self: *CodeGen, cursed_type: []const u8) !c.LLVMTypeRef {
        if (std.mem.eql(u8, cursed_type, "normie")) {
            return c.LLVMInt32TypeInContext(self.context);
        } else if (std.mem.eql(u8, cursed_type, "meal")) {
            return c.LLVMDoubleTypeInContext(self.context);
        } else if (std.mem.eql(u8, cursed_type, "tea")) {
            return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
        }
        return error.UnknownType;
    }

    // Generate expressions based on type
    fn generateExpression(self: *CodeGen, expr: ast.Expression) CodeGenError!c.LLVMValueRef {
        switch (expr) {
            .Literal => |literal| {
                switch (literal) {
                    .IntegerLiteral => |int| {
                        return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @intCast(int), 0);
                    },
                    .FloatLiteral => |float| {
                        return c.LLVMConstReal(c.LLVMDoubleTypeInContext(self.context), float);
                    },
                    .StringLiteral => |str| {
                        return c.LLVMBuildGlobalStringPtr(self.builder, str.ptr, "str");
                    },
                    .BooleanLiteral => |bool_val| {
                        return c.LLVMConstInt(c.LLVMInt1TypeInContext(self.context), if (bool_val) 1 else 0, 0);
                    },
                    .CharLiteral => |char| {
                        return c.LLVMConstInt(c.LLVMInt8TypeInContext(self.context), @as(u8, @intCast(char)), 0);
                    },
                }
            },
            .Identifier => |ident| {
                // Look up variable in symbol table
                if (self.variables.get(ident)) |alloca| {
                    const var_type = c.LLVMGetAllocatedType(alloca);
                    return c.LLVMBuildLoad2(self.builder, var_type, alloca, ident.ptr);
                } else {
                    return CodeGenError.UndefinedSymbol;
                }
            },
            .BinaryOp => |binary| {
                const left = try self.generateExpression(binary.left.*);
                const right = try self.generateExpression(binary.right.*);
                return try self.generateBinaryOp(left, binary.operator, right);
            },
            .UnaryOp => |unary| {
                const operand = try self.generateExpression(unary.operand.*);
                return try self.generateUnaryOp(unary.operator, operand);
            },
            .FunctionCall => |call| {
                return try self.generateCall(call);
            },
            .MemberAccess => |member| {
                return try self.generateMemberAccess(member);
            },
            .StructLiteral => |struct_lit| {
                return try self.generateStructLiteral(struct_lit);
            },
            .Tuple => |tuple| {
                return try self.generateTuple(tuple);
            },
            .TupleAccess => |tuple_access| {
                return try self.generateTupleAccess(tuple_access);
            },
            .ArrayLiteral => |array| {
                return try self.generateArrayLiteral(array);
            },
            .IndexAccess => |index| {
                return try self.generateIndexAccess(index);
            },
            .TypeCast => |cast| {
                return try self.generateTypeCast(cast);
            },
            .Shook => |shook| {
                return try self.generateShook(shook);
            },
            .Match => |match| {
                return try self.generateMatch(match);
            },
            .ChannelSend => |send| {
                return try self.generateChannelSend(send);
            },
            .ChannelReceive => |recv| {
                return try self.generateChannelReceive(recv);
            },
            .ChannelCreation => |create| {
                return try self.generateChannelCreation(create);
            },
            .InterfaceCall => |interface_call| {
                return try self.generateInterfaceCall(interface_call);
            },
            .Goroutine => |goroutine| {
                return try self.generateGoroutineExpression(goroutine);
            },
            .Range => |range| {
                return try self.generateRange(range);
            },
            .Variable => |variable| {
                if (self.variables.get(variable)) |alloca| {
                    const var_type = c.LLVMGetAllocatedType(alloca);
                    return c.LLVMBuildLoad2(self.builder, var_type, alloca, variable.ptr);
                } else {
                    return CodeGenError.UndefinedSymbol;
                }
            },
            .Map => |map| {
                return try self.generateMapLiteral(map);
            },
            .CompositeLiteral => |composite| {
                return try self.generateCompositeLiteral(composite);
            },
            .Lambda => |lambda| {
                return try self.generateLambda(lambda);
            },
            .SliceAccess => |slice| {
                return try self.generateSliceAccess(slice);
            },
            .TypeAssertion => |type_assert| {
                return try self.generateTypeAssertion(type_assert);
            },
            .ErrorValue => |error_val| {
                return try self.generateErrorValue(error_val);
            },
            .StructuredError => |structured_error| {
                return try self.generateStructuredError(structured_error);
            },
            .Panic => |panic| {
                return try self.generatePanicExpression(panic);
            },
            .Recover => |recover| {
                return try self.generateRecover(recover);
            },
            .TestResult => |test_result| {
                return try self.generateTestResult(test_result);
            },
            .TestResultCheck => |test_check| {
                return try self.generateTestResultCheck(test_check);
            },
            .RangeFor => |range_for| {
                return try self.generateRangeFor(range_for);
            },
            .TypeSwitch => |type_switch| {
                return try self.generateTypeSwitch(type_switch);
            },
            .Block => |block| {
                return try self.generateBlockExpression(block);
            },
            else => {
                std.debug.print("Unknown expression type in codegen: {}\n", .{expr});
                return c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 0, 1);
            },
        }
    }

    fn generateBinaryOp(self: *CodeGen, left: c.LLVMValueRef, operator: []const u8, right: c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        if (std.mem.eql(u8, operator, "+")) {
            return c.LLVMBuildAdd(self.builder, left, right, "add");
        } else if (std.mem.eql(u8, operator, "-")) {
            return c.LLVMBuildSub(self.builder, left, right, "sub");
        } else if (std.mem.eql(u8, operator, "*")) {
            return c.LLVMBuildMul(self.builder, left, right, "mul");
        } else if (std.mem.eql(u8, operator, "/")) {
            return c.LLVMBuildSDiv(self.builder, left, right, "div");
        } else if (std.mem.eql(u8, operator, "%")) {
            return c.LLVMBuildSRem(self.builder, left, right, "rem");
        } else if (std.mem.eql(u8, operator, "==")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntEQ, left, right, "eq");
        } else if (std.mem.eql(u8, operator, "!=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, left, right, "ne");
        } else if (std.mem.eql(u8, operator, "<")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, left, right, "lt");
        } else if (std.mem.eql(u8, operator, "<=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSLE, left, right, "le");
        } else if (std.mem.eql(u8, operator, ">")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSGT, left, right, "gt");
        } else if (std.mem.eql(u8, operator, ">=")) {
            return c.LLVMBuildICmp(self.builder, c.LLVMIntSGE, left, right, "ge");
        } else if (std.mem.eql(u8, operator, "&&")) {
            return c.LLVMBuildAnd(self.builder, left, right, "and");
        } else if (std.mem.eql(u8, operator, "||")) {
            return c.LLVMBuildOr(self.builder, left, right, "or");
        } else if (std.mem.eql(u8, operator, "&")) {
            return c.LLVMBuildAnd(self.builder, left, right, "bitand");
        } else if (std.mem.eql(u8, operator, "|")) {
            return c.LLVMBuildOr(self.builder, left, right, "bitor");
        } else if (std.mem.eql(u8, operator, "^")) {
            return c.LLVMBuildXor(self.builder, left, right, "xor");
        } else if (std.mem.eql(u8, operator, "<<")) {
            return c.LLVMBuildShl(self.builder, left, right, "shl");
        } else if (std.mem.eql(u8, operator, ">>")) {
            return c.LLVMBuildAShr(self.builder, left, right, "shr");
        } else {
            std.debug.print("Unsupported binary operator: {s}\n", .{operator});
            return CodeGenError.LLVMError;
        }
    }

    fn generateCall(self: *CodeGen, call: ast.CallExpression) CodeGenError!c.LLVMValueRef {
        // Handle built-in functions
        switch (call.function.*) {
            .MemberAccess => |member| {
                if (std.mem.eql(u8, member.property, "spill")) {
                    // vibez.spill - print function (supports multiple arguments)
                    const printf_func = self.functions.get("printf").?;
                    
                    if (call.arguments.items.len == 0) {
                        // No arguments, just print newline
                        const format = c.LLVMBuildGlobalStringPtr(self.builder, "\n", "fmt");
                        return c.LLVMBuildCall2(self.builder, c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)), printf_func, &[_]c.LLVMValueRef{format}, 1, "print_call");
                    }
                    
                    // Build format string and arguments dynamically
                    var format_parts = ArrayList(u8).init(self.allocator);
                    defer format_parts.deinit();
                    
                    var printf_args = ArrayList(c.LLVMValueRef).init(self.allocator);
                    defer printf_args.deinit();
                    
                    for (call.arguments.items, 0..) |arg_expr, i| {
                        if (i > 0) {
                            try format_parts.appendSlice(" ");
                        }
                        
                        const arg = try self.generateExpression(arg_expr);
                        const arg_type = c.LLVMTypeOf(arg);
                        
                        if (c.LLVMGetTypeKind(arg_type) == c.LLVMIntegerTypeKind) {
                            const bit_width = c.LLVMGetIntTypeWidth(arg_type);
                            if (bit_width == 1) {
                                try format_parts.appendSlice("%s");
                                // Convert bool to string
                                const true_str = c.LLVMBuildGlobalStringPtr(self.builder, "true", "true_str");
                                const false_str = c.LLVMBuildGlobalStringPtr(self.builder, "false", "false_str");
                                const cond_arg = c.LLVMBuildSelect(self.builder, arg, true_str, false_str, "bool_str");
                                try printf_args.append(cond_arg);
                            } else {
                                try format_parts.appendSlice("%d");
                                try printf_args.append(arg);
                            }
                        } else if (c.LLVMGetTypeKind(arg_type) == c.LLVMDoubleTypeKind) {
                            try format_parts.appendSlice("%.2f");
                            try printf_args.append(arg);
                        } else if (c.LLVMGetTypeKind(arg_type) == c.LLVMPointerTypeKind) {
                            try format_parts.appendSlice("%s");
                            try printf_args.append(arg);
                        } else {
                            try format_parts.appendSlice("%p");
                            try printf_args.append(arg);
                        }
                    }
                    
                    try format_parts.appendSlice("\n");
                    
                    // Create final format string
                    const format_str = try format_parts.toOwnedSlice();
                    defer self.allocator.free(format_str);
                    const format = c.LLVMBuildGlobalStringPtr(self.builder, format_str.ptr, "fmt");
                    
                    // Build final printf arguments (format string + actual arguments)
                    var final_args = ArrayList(c.LLVMValueRef).init(self.allocator);
                    defer final_args.deinit();
                    try final_args.append(format);
                    try final_args.appendSlice(printf_args.items);
                    
                    return c.LLVMBuildCall2(
                        self.builder, 
                        c.LLVMGetReturnType(c.LLVMGlobalGetValueType(printf_func)), 
                        printf_func, 
                        final_args.items.ptr, 
                        @as(u32, @intCast(final_args.items.len)), 
                        "print_call"
                    );
                }
            },
            .Identifier => |name| {
                if (self.functions.get(name)) |function| {
                    // Generate arguments
                    var args = ArrayList(c.LLVMValueRef).init(self.allocator);
                    defer args.deinit();
                    
                    for (call.arguments.items) |arg_expr| {
                        const arg = try self.generateExpression(arg_expr);
                        try args.append(arg);
                    }
                    
                    return c.LLVMBuildCall2(
                        self.builder,
                        c.LLVMGetReturnType(c.LLVMGlobalGetValueType(function)),
                        function,
                        if (args.items.len > 0) args.items.ptr else null,
                        @as(u32, @intCast(args.items.len)),
                        "call"
                    );
                }
            },
            else => {},
        }
        
        return CodeGenError.UndefinedSymbol;
    }

    fn generateMemberAccess(self: *CodeGen, member: ast.MemberAccessExpression) CodeGenError!c.LLVMValueRef {
        // For now, just return the object (simplified implementation)
        return try self.generateExpression(member.object.*);
    }

    fn generateLet(self: *CodeGen, let: ast.LetStatement) CodeGenError!void {
        const var_type = if (let.var_type) |vt| 
            try self.getLLVMType(vt) 
        else 
            c.LLVMInt64TypeInContext(self.context); // default to i64
        
        const alloca = c.LLVMBuildAlloca(self.builder, var_type, let.name.ptr);
        
        if (let.initializer) |initializer_expr| {
            const value = try self.generateExpression(initializer_expr);
            _ = c.LLVMBuildStore(self.builder, value, alloca);
        }
        
        try self.variables.put(let.name, alloca);
    }

    fn generateReturn(self: *CodeGen, ret: ast.ReturnStatement) CodeGenError!void {
        if (ret.value) |value| {
            const return_value = try self.generateExpression(value);
            _ = c.LLVMBuildRet(self.builder, return_value);
        } else {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
    }

    fn generateIf(self: *CodeGen, if_stmt: ast.IfStatement) CodeGenError!void {
        const condition = try self.generateExpression(if_stmt.condition);
        
        const function = self.current_function.?;
        const then_block = c.LLVMAppendBasicBlockInContext(self.context, function, "then");
        const else_block = c.LLVMAppendBasicBlockInContext(self.context, function, "else");
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, function, "merge");
        
        _ = c.LLVMBuildCondBr(self.builder, condition, then_block, else_block);
        
        // Generate then branch
        c.LLVMPositionBuilderAtEnd(self.builder, then_block);
        for (if_stmt.then_branch.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Only add branch if block doesn't already have a terminator
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Generate else branch
        c.LLVMPositionBuilderAtEnd(self.builder, else_block);
        if (if_stmt.else_branch) |else_stmts| {
            for (else_stmts.items) |stmt| {
                try self.generateStatement(stmt);
            }
        }
        
        // Only add branch if block doesn't already have a terminator
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }
        
        // Continue building in merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
    }

    fn generateWhile(self: *CodeGen, while_stmt: ast.WhileStatement) CodeGenError!void {
        const function = self.current_function.?;
        const condition_block = c.LLVMAppendBasicBlockInContext(self.context, function, "while_cond");
        const body_block = c.LLVMAppendBasicBlockInContext(self.context, function, "while_body");
        const exit_block = c.LLVMAppendBasicBlockInContext(self.context, function, "while_exit");
        
        _ = c.LLVMBuildBr(self.builder, condition_block);
        
        // Generate condition
        c.LLVMPositionBuilderAtEnd(self.builder, condition_block);
        const condition = try self.generateExpression(while_stmt.condition);
        _ = c.LLVMBuildCondBr(self.builder, condition, body_block, exit_block);
        
        // Generate body
        c.LLVMPositionBuilderAtEnd(self.builder, body_block);
        for (while_stmt.body.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Only add branch if block doesn't already have a terminator
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, condition_block);
        }
        
        // Continue building in exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
    }

    fn getLLVMType(self: *CodeGen, cursed_type: ast.Type) CodeGenError!c.LLVMTypeRef {
        switch (cursed_type) {
            .Basic => |basic| {
                switch (basic) {
                    .Normie => return c.LLVMInt32TypeInContext(self.context),
                    .Tea, .Txt => return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
                    .Sip => return c.LLVMInt8TypeInContext(self.context),
                    .Smol => return c.LLVMInt8TypeInContext(self.context),
                    .Mid => return c.LLVMInt16TypeInContext(self.context),
                    .Thicc => return c.LLVMInt64TypeInContext(self.context),
                    .Snack => return c.LLVMFloatTypeInContext(self.context),
                    .Meal => return c.LLVMDoubleTypeInContext(self.context),
                    .Byte => return c.LLVMInt8TypeInContext(self.context),
                    .Rune => return c.LLVMInt32TypeInContext(self.context),
                    .Lit => return c.LLVMInt1TypeInContext(self.context),
                    .Cap => return c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
                    else => return CodeGenError.InvalidType,
                }
            },
            else => return CodeGenError.InvalidType,
        }
    }

    fn getDefaultValue(self: *CodeGen, cursed_type: ast.Type) CodeGenError!c.LLVMValueRef {
        const llvm_type = try self.getLLVMType(cursed_type);
        
        switch (cursed_type) {
            .Basic => |basic| {
                switch (basic) {
                    .Normie, .Smol, .Mid, .Thicc, .Byte, .Rune => {
                        return c.LLVMConstInt(llvm_type, 0, 0);
                    },
                    .Snack, .Meal => {
                        return c.LLVMConstReal(llvm_type, 0.0);
                    },
                    .Lit => {
                        return c.LLVMConstInt(llvm_type, 0, 0); // false
                    },
                    .Tea, .Txt, .Cap => {
                        return c.LLVMConstNull(llvm_type);
                    },
                    else => return CodeGenError.InvalidType,
                }
            },
            else => return CodeGenError.InvalidType,
        }
    }



    pub fn writeExecutable(self: *CodeGen, output_path: []const u8) CodeGenError!void {
        // Write LLVM IR to file for debugging
        var ir_filename = ArrayList(u8).init(self.allocator);
        defer ir_filename.deinit();
        
        try ir_filename.appendSlice(output_path);
        try ir_filename.appendSlice(".ll");
        
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMPrintModuleToFile(self.module, ir_filename.items.ptr, &error_msg) != 0) {
            std.debug.print("Failed to write LLVM IR: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
        
        // Create execution engine for native compilation
        var execution_engine: c.LLVMExecutionEngineRef = undefined;
        if (c.LLVMCreateExecutionEngineForModule(&execution_engine, self.module, &error_msg) != 0) {
            std.debug.print("Failed to create execution engine: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
        defer c.LLVMDisposeExecutionEngine(execution_engine);
        
        // For now, just write the IR file - native compilation would require more setup
        std.debug.print("Generated LLVM IR: {s}\n", .{ir_filename.items});
        std.debug.print("Note: Native compilation not yet implemented. Use llc to compile IR to object file.\n", .{});
    }



    /// Generate struct literal expression
    fn generateStructLiteral(self: *CodeGen, struct_lit: ast.StructLiteralExpression) CodeGenError!c.LLVMValueRef {
        const struct_type = self.struct_types.get(struct_lit.struct_name) orelse {
            return CodeGenError.UndefinedSymbol;
        };
        
        // Allocate memory for struct
        const struct_size = c.LLVMSizeOf(struct_type);
        const malloc_func = self.functions.get("malloc").?;
        const struct_ptr = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(malloc_func)),
            malloc_func,
            &[_]c.LLVMValueRef{struct_size},
            1,
            "struct_alloc"
        );
        
        // Cast to proper struct pointer type
        const typed_ptr = c.LLVMBuildBitCast(
            self.builder,
            struct_ptr,
            c.LLVMPointerType(struct_type, 0),
            "struct_ptr"
        );
        
        // Initialize fields
        for (struct_lit.fields.items, 0..) |field_assignment, i| {
            const field_value = try self.generateExpression(field_assignment.value);
            const field_ptr = c.LLVMBuildStructGEP2(
                self.builder,
                struct_type,
                typed_ptr,
                @as(u32, @intCast(i)),
                "field_ptr"
            );
            _ = c.LLVMBuildStore(self.builder, field_value, field_ptr);
        }
        
        return typed_ptr;
    }

    /// Generate tuple expression
    fn generateTuple(self: *CodeGen, tuple: ast.TupleExpression) CodeGenError!c.LLVMValueRef {
        // Create tuple type
        var element_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer element_types.deinit();
        
        var element_values = ArrayList(c.LLVMValueRef).init(self.allocator);
        defer element_values.deinit();
        
        for (tuple.elements.items) |element| {
            const value = try self.generateExpression(element);
            const value_type = c.LLVMTypeOf(value);
            try element_types.append(value_type);
            try element_values.append(value);
        }
        
        // Create tuple struct type
        const tuple_type = c.LLVMStructTypeInContext(
            self.context,
            element_types.items.ptr,
            @as(u32, @intCast(element_types.items.len)),
            0
        );
        
        // Create tuple value
        var tuple_value = c.LLVMGetUndef(tuple_type);
        for (element_values.items, 0..) |value, i| {
            tuple_value = c.LLVMBuildInsertValue(
                self.builder,
                tuple_value,
                value,
                @as(u32, @intCast(i)),
                "tuple_elem"
            );
        }
        
        return tuple_value;
    }

    /// Generate tuple access expression
    fn generateTupleAccess(self: *CodeGen, tuple_access: ast.TupleAccessExpression) CodeGenError!c.LLVMValueRef {
        const tuple_value = try self.generateExpression(tuple_access.tuple.*);
        
        return c.LLVMBuildExtractValue(
            self.builder,
            tuple_value,
            @as(u32, @intCast(tuple_access.index)),
            "tuple_access"
        );
    }



    fn generateShook(self: *CodeGen, shook: ast.ShookExpression) CodeGenError!c.LLVMValueRef {
        // Generate the wrapped expression that might fail
        const result = try self.generateExpression(shook.expression.*);
        
        // Check if result indicates an error (simplified error propagation)
        // In full implementation, would check error union type and propagate accordingly
        
        // For now, create a simple error check pattern:
        // if (is_error(result)) return error;
        // return result;
        
        const is_error_func = c.LLVMGetNamedFunction(self.module, "cursed_is_error");
        if (is_error_func == null) {
            // Create error checking function if it doesn't exist
            const error_check_type = c.LLVMFunctionType(
                c.LLVMInt1TypeInContext(self.context), // returns bool
                &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}, // takes pointer
                1,
                0
            );
            _ = c.LLVMAddFunction(self.module, "cursed_is_error", error_check_type);
        }
        
        // Generate error propagation logic
        const current_func = self.current_function orelse return CodeGenError.LLVMError;
        const error_check_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "shook_check");
        const return_error_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "shook_error");
        const continue_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "shook_continue");
        
        // Jump to error check
        _ = c.LLVMBuildBr(self.builder, error_check_block);
        
        c.LLVMPositionBuilderAtEnd(self.builder, error_check_block);
        
        // Create error condition check (simplified)
        const null_check = c.LLVMBuildIsNull(self.builder, result, "error_check");
        _ = c.LLVMBuildCondBr(self.builder, null_check, return_error_block, continue_block);
        
        // Return error block
        c.LLVMPositionBuilderAtEnd(self.builder, return_error_block);
        _ = c.LLVMBuildRet(self.builder, result); // Return the error value
        
        // Continue with normal execution
        c.LLVMPositionBuilderAtEnd(self.builder, continue_block);
        
        return result;
    }

    /// Generate unary operation
    fn generateUnaryOp(self: *CodeGen, operator: []const u8, operand: c.LLVMValueRef) CodeGenError!c.LLVMValueRef {
        if (std.mem.eql(u8, operator, "-")) {
            // Unary minus
            const operand_type = c.LLVMTypeOf(operand);
            if (c.LLVMGetTypeKind(operand_type) == c.LLVMIntegerTypeKind) {
                const zero = c.LLVMConstInt(operand_type, 0, 0);
                return c.LLVMBuildSub(self.builder, zero, operand, "neg");
            } else if (c.LLVMGetTypeKind(operand_type) == c.LLVMDoubleTypeKind or c.LLVMGetTypeKind(operand_type) == c.LLVMFloatTypeKind) {
                return c.LLVMBuildFNeg(self.builder, operand, "fneg");
            }
        } else if (std.mem.eql(u8, operator, "!")) {
            // Logical not
            return c.LLVMBuildNot(self.builder, operand, "not");
        } else if (std.mem.eql(u8, operator, "~")) {
            // Bitwise not
            return c.LLVMBuildNot(self.builder, operand, "bitnot");
        } else if (std.mem.eql(u8, operator, "+")) {
            // Unary plus (no-op)
            return operand;
        }
        
        std.debug.print("Unsupported unary operator: {s}\n", .{operator});
        return CodeGenError.LLVMError;
    }

    /// Generate array literal
    fn generateArrayLiteral(self: *CodeGen, array: ast.ArrayLiteralExpression) CodeGenError!c.LLVMValueRef {
        if (array.elements.items.len == 0) {
            // Empty array
            const i8_type = c.LLVMInt8TypeInContext(self.context);
            const array_type = c.LLVMArrayType(i8_type, 0);
            return c.LLVMGetUndef(array_type);
        }

        // Generate all elements first to determine array type
        var element_values = ArrayList(c.LLVMValueRef).init(self.allocator);
        defer element_values.deinit();

        var element_type: ?c.LLVMTypeRef = null;
        for (array.elements.items) |element| {
            const value = try self.generateExpression(element);
            try element_values.append(value);
            
            if (element_type == null) {
                element_type = c.LLVMTypeOf(value);
            }
        }

        // Create array type
        const array_type = c.LLVMArrayType(element_type.?, @as(u32, @intCast(element_values.items.len)));
        
        // Allocate array on heap
        const array_size = c.LLVMSizeOf(array_type);
        const malloc_func = self.functions.get("malloc").?;
        const array_ptr = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(malloc_func)),
            malloc_func,
            &[_]c.LLVMValueRef{array_size},
            1,
            "array_alloc"
        );

        // Cast to proper array pointer type
        const typed_array_ptr = c.LLVMBuildBitCast(
            self.builder,
            array_ptr,
            c.LLVMPointerType(array_type, 0),
            "array_ptr"
        );

        // Initialize array elements
        for (element_values.items, 0..) |value, i| {
            const element_ptr = c.LLVMBuildGEP2(
                self.builder,
                array_type,
                typed_array_ptr,
                &[_]c.LLVMValueRef{
                    c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                    c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @as(u32, @intCast(i)), 0)
                },
                2,
                "element_ptr"
            );
            _ = c.LLVMBuildStore(self.builder, value, element_ptr);
        }

        return typed_array_ptr;
    }

    /// Generate array index access
    fn generateIndexAccess(self: *CodeGen, index: ast.IndexAccessExpression) CodeGenError!c.LLVMValueRef {
        const array_value = try self.generateExpression(index.object.*);
        const index_value = try self.generateExpression(index.index.*);

        // Get array type from pointer
        const array_ptr_type = c.LLVMTypeOf(array_value);
        const array_type = c.LLVMGetElementType(array_ptr_type);
        const element_type = c.LLVMGetElementType(array_type);

        // Generate element pointer
        const element_ptr = c.LLVMBuildGEP2(
            self.builder,
            array_type,
            array_value,
            &[_]c.LLVMValueRef{
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                index_value
            },
            2,
            "element_ptr"
        );

        // Load element value
        return c.LLVMBuildLoad2(self.builder, element_type, element_ptr, "element_value");
    }

    /// Generate type cast
    fn generateTypeCast(self: *CodeGen, cast: ast.TypeCastExpression) CodeGenError!c.LLVMValueRef {
        const value = try self.generateExpression(cast.expression.*);
        const target_type = try self.getLLVMType(cast.target_type);

        const source_type = c.LLVMTypeOf(value);
        const source_kind = c.LLVMGetTypeKind(source_type);
        const target_kind = c.LLVMGetTypeKind(target_type);

        // Handle various cast combinations
        if (source_kind == c.LLVMIntegerTypeKind and target_kind == c.LLVMIntegerTypeKind) {
            // Integer to integer cast
            const source_width = c.LLVMGetIntTypeWidth(source_type);
            const target_width = c.LLVMGetIntTypeWidth(target_type);
            
            if (source_width < target_width) {
                return c.LLVMBuildSExt(self.builder, value, target_type, "sext");
            } else if (source_width > target_width) {
                return c.LLVMBuildTrunc(self.builder, value, target_type, "trunc");
            } else {
                return value; // Same width, no cast needed
            }
        } else if (source_kind == c.LLVMIntegerTypeKind and (target_kind == c.LLVMFloatTypeKind or target_kind == c.LLVMDoubleTypeKind)) {
            // Integer to float cast
            return c.LLVMBuildSIToFP(self.builder, value, target_type, "itof");
        } else if ((source_kind == c.LLVMFloatTypeKind or source_kind == c.LLVMDoubleTypeKind) and target_kind == c.LLVMIntegerTypeKind) {
            // Float to integer cast
            return c.LLVMBuildFPToSI(self.builder, value, target_type, "ftoi");
        } else if (source_kind == c.LLVMPointerTypeKind and target_kind == c.LLVMPointerTypeKind) {
            // Pointer to pointer cast
            return c.LLVMBuildBitCast(self.builder, value, target_type, "bitcast");
        } else {
            // Default bitcast for other types
            return c.LLVMBuildBitCast(self.builder, value, target_type, "cast");
        }
    }

    /// Generate pattern matching expression
    fn generateMatch(self: *CodeGen, match: ast.MatchExpression) CodeGenError!c.LLVMValueRef {
        const match_value = try self.generateExpression(match.expression.*);
        const current_func = self.current_function.?;

        // Create basic blocks for each case and merge
        var case_blocks = ArrayList(c.LLVMBasicBlockRef).init(self.allocator);
        defer case_blocks.deinit();

        var case_values = ArrayList(c.LLVMValueRef).init(self.allocator);
        defer case_values.deinit();

        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "match_merge");
        const match_value_type = c.LLVMTypeOf(match_value);

        // Generate blocks and conditions for each case
        for (match.cases.items, 0..) |case_item, i| {
            const case_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "match_case");
            try case_blocks.append(case_block);

            // Generate condition check
            if (i == 0) {
                // First case, generate the switch logic
                const default_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "match_default");
                const switch_inst = c.LLVMBuildSwitch(self.builder, match_value, default_block, @as(u32, @intCast(match.cases.items.len)));

                // Add cases to switch
                for (match.cases.items, 0..) |case_check, j| {
                    if (case_check.pattern) |pattern| {
                        switch (pattern) {
                            .Literal => |literal| {
                                const case_value = try self.generateExpression(ast.Expression{ .Literal = literal });
                                c.LLVMAddCase(switch_inst, case_value, case_blocks.items[j]);
                            },
                            else => {
                                // For complex patterns, add to default for now
                                c.LLVMAddCase(switch_inst, c.LLVMConstInt(match_value_type, @as(u64, @intCast(j)), 0), case_blocks.items[j]);
                            },
                        }
                    }
                }
            }

            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.builder, case_block);
            const case_result = if (case_item.body) |body|
                try self.generateExpression(body)
            else
                c.LLVMGetUndef(match_value_type);

            try case_values.append(case_result);

            // Branch to merge block
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }

        // Create PHI node in merge block to collect results
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
        const phi = c.LLVMBuildPhi(self.builder, match_value_type, "match_result");

        // Add incoming values to PHI
        for (case_values.items, 0..) |value, i| {
            c.LLVMAddIncoming(phi, &[_]c.LLVMValueRef{value}, &[_]c.LLVMBasicBlockRef{case_blocks.items[i]}, 1);
        }

        return phi;
    }

    /// Generate block statement
    fn generateBlock(self: *CodeGen, block: ast.BlockStatement) CodeGenError!void {
        for (block.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
    }

    /// Generate assignment statement
    fn generateAssignment(self: *CodeGen, assignment: ast.AssignmentStatement) CodeGenError!void {
        const value = try self.generateExpression(assignment.value);
        
        if (self.variables.get(assignment.target)) |alloca| {
            _ = c.LLVMBuildStore(self.builder, value, alloca);
        } else {
            return CodeGenError.UndefinedSymbol;
        }
    }

    /// Generate implementation statement (struct implementing interface)
    fn generateImplementation(self: *CodeGen, impl: ast.ImplementationStatement) CodeGenError!void {
        // Store implementation info for vtable generation
        const struct_type_name = impl.implementing_type;
        const interface_name = impl.interface_name;

        // Generate methods with mangled names
        for (impl.methods.items) |method| {
            var mangled_name = ArrayList(u8).init(self.allocator);
            defer mangled_name.deinit();

            try mangled_name.appendSlice(struct_type_name);
            try mangled_name.appendSlice("_");
            try mangled_name.appendSlice(method.name);

            // Convert method to function statement and generate
            const func_stmt = ast.FunctionStatement{
                .name = try mangled_name.toOwnedSlice(),
                .parameters = method.parameters,
                .return_type = method.return_type,
                .body = method.body,
                .type_parameters = ArrayList(ast.TypeParameter).init(self.allocator),
                .attributes = ArrayList(ast.Attribute).init(self.allocator),
            };

            try self.generateFunction(func_stmt);
        }

        // Register this implementation for interface dispatch
        const vtable_name = try std.fmt.allocPrint(
            self.allocator,
            "vtable_{s}_{s}",
            .{ struct_type_name, interface_name }
        );

        // Create and store vtable global
        if (self.interface_types.get(interface_name)) |interface_info| {
            const method_count = interface_info.methods.items.len;
            const func_ptr_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0);
            const vtable_type = c.LLVMArrayType(func_ptr_type, @as(u32, @intCast(method_count)));

            const vtable_global = c.LLVMAddGlobal(self.module, vtable_type, vtable_name.ptr);
            c.LLVMSetLinkage(vtable_global, c.LLVMInternalLinkage);

            // Initialize vtable with method pointers
            var method_pointers = ArrayList(c.LLVMValueRef).init(self.allocator);
            defer method_pointers.deinit();

            for (interface_info.methods.items) |interface_method| {
                const method_name = try std.fmt.allocPrint(
                    self.allocator,
                    "{s}_{s}",
                    .{ struct_type_name, interface_method.name }
                );
                
                if (self.functions.get(method_name)) |method_func| {
                    const casted_func = c.LLVMBuildBitCast(self.builder, method_func, func_ptr_type, "method_cast");
                    try method_pointers.append(casted_func);
                } else {
                    // Use null for missing methods
                    try method_pointers.append(c.LLVMConstNull(func_ptr_type));
                }
            }

            if (method_pointers.items.len > 0) {
                const vtable_init = c.LLVMConstArray(func_ptr_type, method_pointers.items.ptr, @as(u32, @intCast(method_pointers.items.len)));
                c.LLVMSetInitializer(vtable_global, vtable_init);
            }
        }
    }

    // ===== CURSED LANGUAGE FEATURE IMPLEMENTATIONS =====

    /// Generate runtime function declarations for CURSED
    fn generateRuntimeDeclarations(self: *CodeGen) CodeGenError!void {
        // Goroutine runtime functions
        try self.declareRuntimeFunction("cursed_spawn_goroutine", &[_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), null, 0, 0), 0), // function pointer
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // context data
            c.LLVMInt32TypeInContext(self.context), // stack size
        }, c.LLVMInt32TypeInContext(self.context)); // returns goroutine ID

        // Channel runtime functions
        try self.declareRuntimeFunction("cursed_channel_create", &[_]c.LLVMTypeRef{
            c.LLVMInt32TypeInContext(self.context), // element size
            c.LLVMInt32TypeInContext(self.context), // buffer size
        }, c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)); // returns channel pointer

        try self.declareRuntimeFunction("cursed_channel_send", &[_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // channel
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // data
        }, c.LLVMInt1TypeInContext(self.context)); // returns success

        try self.declareRuntimeFunction("cursed_channel_receive", &[_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // channel
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // output buffer
        }, c.LLVMInt1TypeInContext(self.context)); // returns success

        try self.declareRuntimeFunction("cursed_channel_close", &[_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // channel
        }, c.LLVMVoidTypeInContext(self.context));

        // Select runtime functions
        try self.declareRuntimeFunction("cursed_select_begin", &[_]c.LLVMTypeRef{
            c.LLVMInt32TypeInContext(self.context), // number of cases
        }, c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)); // returns select context

        try self.declareRuntimeFunction("cursed_select_add_channel", &[_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // select context
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // channel
            c.LLVMInt32TypeInContext(self.context), // case index
        }, c.LLVMVoidTypeInContext(self.context));

        try self.declareRuntimeFunction("cursed_select_wait", &[_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // select context
        }, c.LLVMInt32TypeInContext(self.context)); // returns ready case index

        // Interface dispatch functions
        try self.declareRuntimeFunction("cursed_interface_call", &[_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // object
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // vtable
            c.LLVMInt32TypeInContext(self.context), // method index
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // args
        }, c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)); // returns result

        // Defer/panic functions
        try self.declareRuntimeFunction("cursed_defer_push", &[_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), null, 0, 0), 0), // cleanup function
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // context
        }, c.LLVMVoidTypeInContext(self.context));

        try self.declareRuntimeFunction("cursed_defer_pop", &[_]c.LLVMTypeRef{}, c.LLVMVoidTypeInContext(self.context));

        try self.declareRuntimeFunction("cursed_panic", &[_]c.LLVMTypeRef{
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0), // error message
        }, c.LLVMVoidTypeInContext(self.context));
    }

    fn declareRuntimeFunction(self: *CodeGen, name: []const u8, param_types: []const c.LLVMTypeRef, return_type: c.LLVMTypeRef) CodeGenError!void {
        const func_type = c.LLVMFunctionType(
            return_type,
            if (param_types.len > 0) param_types.ptr else null,
            @as(u32, @intCast(param_types.len)),
            0
        );
        const func = c.LLVMAddFunction(self.module, name.ptr, func_type);
        try self.runtime_functions.put(name, func);
    }

    /// Generate stan (goroutine) statement
    fn generateStan(self: *CodeGen, stan: ast.StanStatement) CodeGenError!void {
        // Create wrapper function for the goroutine body
        const goroutine_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)}, // context parameter
            1,
            0
        );

        const goroutine_id = self.goroutine_counter;
        self.goroutine_counter += 1;

        const func_name = try std.fmt.allocPrint(self.allocator, "goroutine_{d}", .{goroutine_id});
        const goroutine_func = c.LLVMAddFunction(self.module, func_name.ptr, goroutine_func_type);

        // Create entry block for goroutine
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, goroutine_func, "entry");
        
        // Save current builder position
        const saved_function = self.current_function;
        const saved_block = c.LLVMGetInsertBlock(self.builder);
        
        // Generate goroutine body
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        self.current_function = goroutine_func;
        
        // Generate the goroutine body statements
        for (stan.body.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Add return if missing
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
        
        // Restore builder position
        self.current_function = saved_function;
        if (saved_block != null) {
            c.LLVMPositionBuilderAtEnd(self.builder, saved_block);
        }

        // Spawn the goroutine
        const spawn_func = self.runtime_functions.get("cursed_spawn_goroutine").?;
        const func_ptr = c.LLVMBuildBitCast(
            self.builder,
            goroutine_func,
            c.LLVMPointerType(c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), null, 0, 0), 0),
            "func_ptr"
        );
        const null_context = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));
        const default_stack_size = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 65536, 0); // 64KB

        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(spawn_func)),
            spawn_func,
            &[_]c.LLVMValueRef{ func_ptr, null_context, default_stack_size },
            3,
            "spawn_result"
        );

        // Store goroutine info
        const goroutine_info = GoroutineInfo{
            .function = goroutine_func,
            .stack_size = 65536,
            .id = goroutine_id,
        };
        try self.goroutines.put(goroutine_id, goroutine_info);
    }

    /// Generate select statement
    fn generateSelect(self: *CodeGen, select_stmt: ast.SelectStatement) CodeGenError!void {
        const current_func = self.current_function.?;
        
        // Begin select operation
        const select_begin_func = self.runtime_functions.get("cursed_select_begin").?;
        const case_count = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @as(u32, @intCast(select_stmt.cases.items.len)), 0);
        const select_context = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(select_begin_func)),
            select_begin_func,
            &[_]c.LLVMValueRef{case_count},
            1,
            "select_ctx"
        );

        // Add channels to select
        const add_channel_func = self.runtime_functions.get("cursed_select_add_channel").?;
        for (select_stmt.cases.items, 0..) |case_item, i| {
            if (case_item.channel) |channel_expr| {
                const channel_value = try self.generateExpression(channel_expr);
                const case_index = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @as(u32, @intCast(i)), 0);
                
                _ = c.LLVMBuildCall2(
                    self.builder,
                    c.LLVMVoidTypeInContext(self.context),
                    add_channel_func,
                    &[_]c.LLVMValueRef{ select_context, channel_value, case_index },
                    3,
                    ""
                );
            }
        }

        // Wait for a channel to be ready
        const select_wait_func = self.runtime_functions.get("cursed_select_wait").?;
        const ready_case = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(select_wait_func)),
            select_wait_func,
            &[_]c.LLVMValueRef{select_context},
            1,
            "ready_case"
        );

        // Create blocks for each case and merge
        var case_blocks = ArrayList(c.LLVMBasicBlockRef).init(self.allocator);
        defer case_blocks.deinit();

        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "select_merge");
        const default_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "select_default");

        // Create switch instruction
        const switch_inst = c.LLVMBuildSwitch(self.builder, ready_case, default_block, @as(u32, @intCast(select_stmt.cases.items.len)));

        // Generate case blocks
        for (select_stmt.cases.items, 0..) |case_item, i| {
            const case_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "select_case");
            try case_blocks.append(case_block);

            const case_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @as(u32, @intCast(i)), 0);
            c.LLVMAddCase(switch_inst, case_value, case_block);

            c.LLVMPositionBuilderAtEnd(self.builder, case_block);
            
            // Generate case body
            for (case_item.body.items) |stmt| {
                try self.generateStatement(stmt);
            }
            
            if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }

        // Generate default case if present
        c.LLVMPositionBuilderAtEnd(self.builder, default_block);
        if (select_stmt.default_case) |default_case| {
            for (default_case.items) |stmt| {
                try self.generateStatement(stmt);
            }
        }
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }

        // Continue building in merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
    }

    /// Generate defer statement
    fn generateDefer(self: *CodeGen, defer_stmt: ast.DeferStatement) CodeGenError!void {
        // Create cleanup function for the deferred code
        const cleanup_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)},
            1,
            0
        );

        const cleanup_func_name = try std.fmt.allocPrint(self.allocator, "defer_cleanup_{d}", .{self.defer_stack.items.len});
        const cleanup_func = c.LLVMAddFunction(self.module, cleanup_func_name.ptr, cleanup_func_type);

        // Generate cleanup function body
        const cleanup_entry = c.LLVMAppendBasicBlockInContext(self.context, cleanup_func, "entry");
        
        // Save current context
        const saved_function = self.current_function;
        const saved_block = c.LLVMGetInsertBlock(self.builder);
        
        // Generate cleanup code
        c.LLVMPositionBuilderAtEnd(self.builder, cleanup_entry);
        self.current_function = cleanup_func;
        
        try self.generateStatement(defer_stmt.statement);
        
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
        
        // Restore context
        self.current_function = saved_function;
        if (saved_block != null) {
            c.LLVMPositionBuilderAtEnd(self.builder, saved_block);
        }

        // Push cleanup function onto defer stack
        const defer_push_func = self.runtime_functions.get("cursed_defer_push").?;
        const func_ptr = c.LLVMBuildBitCast(
            self.builder,
            cleanup_func,
            c.LLVMPointerType(c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), null, 0, 0), 0),
            "cleanup_ptr"
        );
        const null_context = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));

        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            defer_push_func,
            &[_]c.LLVMValueRef{ func_ptr, null_context },
            2,
            ""
        );

        // Store defer info
        const defer_info = DeferInfo{
            .cleanup_function = cleanup_func,
            .cleanup_block = cleanup_entry,
        };
        try self.defer_stack.append(defer_info);
    }

    /// Generate bestie (for) loop statement
    fn generateBestie(self: *CodeGen, bestie: ast.BestieStatement) CodeGenError!void {
        const current_func = self.current_function.?;

        // Create loop blocks
        const condition_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "bestie_cond");
        const body_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "bestie_body");
        const exit_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "bestie_exit");

        // Save current loop context
        const loop_context = LoopContext{
            .continue_block = condition_block,
            .break_block = exit_block,
        };
        try self.loop_stack.append(loop_context);
        defer _ = self.loop_stack.pop();

        // Generate initializer if present
        if (bestie.initializer) |init_stmt| {
            try self.generateStatement(init_stmt);
        }

        // Jump to condition
        _ = c.LLVMBuildBr(self.builder, condition_block);

        // Generate condition block
        c.LLVMPositionBuilderAtEnd(self.builder, condition_block);
        if (bestie.condition) |cond| {
            const condition_value = try self.generateExpression(cond);
            _ = c.LLVMBuildCondBr(self.builder, condition_value, body_block, exit_block);
        } else {
            // Infinite loop
            _ = c.LLVMBuildBr(self.builder, body_block);
        }

        // Generate body block
        c.LLVMPositionBuilderAtEnd(self.builder, body_block);
        for (bestie.body.items) |stmt| {
            try self.generateStatement(stmt);
        }

        // Generate increment if present
        if (bestie.increment) |inc_stmt| {
            try self.generateStatement(inc_stmt);
        }

        // Jump back to condition (if no terminator was added)
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, condition_block);
        }

        // Continue building in exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
    }

    /// Generate vibes (break) statement
    fn generateVibes(self: *CodeGen, _: ast.VibesStatement) CodeGenError!void {
        if (self.loop_stack.items.len == 0) {
            return CodeGenError.LLVMError; // No loop to break from
        }

        const current_loop = self.loop_stack.items[self.loop_stack.items.len - 1];
        _ = c.LLVMBuildBr(self.builder, current_loop.break_block);
    }

    /// Generate match statement (pattern matching)
    fn generateMatchStatement(self: *CodeGen, match_stmt: ast.MatchStatement) CodeGenError!void {
        const match_expr = try self.generateExpression(match_stmt.expression);
        _ = try self.generateMatchPattern(match_expr, match_stmt.cases.items);
    }

    fn generateMatchPattern(self: *CodeGen, value: c.LLVMValueRef, cases: []const ast.MatchCase) CodeGenError!c.LLVMValueRef {
        const current_func = self.current_function.?;
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "match_merge");
        const default_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "match_default");

        // Create switch instruction
        const switch_inst = c.LLVMBuildSwitch(self.builder, value, default_block, @as(u32, @intCast(cases.len)));

        var case_blocks = ArrayList(c.LLVMBasicBlockRef).init(self.allocator);
        defer case_blocks.deinit();

        // Generate case blocks
        for (cases, 0..) |case_item, i| {
            const case_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "match_case");
            try case_blocks.append(case_block);

            // Add case to switch (simplified - assumes literal patterns)
            if (case_item.pattern) |pattern| {
                switch (pattern) {
                    .Literal => |literal| {
                        const case_value = try self.generateExpression(ast.Expression{ .Literal = literal });
                        c.LLVMAddCase(switch_inst, case_value, case_block);
                    },
                    else => {
                        // For complex patterns, add a sequential check
                        const case_index = c.LLVMConstInt(c.LLVMTypeOf(value), @as(u64, @intCast(i)), 0);
                        c.LLVMAddCase(switch_inst, case_index, case_block);
                    },
                }
            }

            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.builder, case_block);
            for (case_item.body.items) |stmt| {
                try self.generateStatement(stmt);
            }

            if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }

        // Generate default case
        c.LLVMPositionBuilderAtEnd(self.builder, default_block);
        _ = c.LLVMBuildBr(self.builder, merge_block);

        // Continue in merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
        return value; // Return original value for now
    }

    /// Generate channel send expression
    fn generateChannelSend(self: *CodeGen, send: ast.ChannelSendExpression) CodeGenError!c.LLVMValueRef {
        const channel_value = try self.generateExpression(send.channel.*);
        const data_value = try self.generateExpression(send.value.*);

        // Allocate temporary storage for data
        const data_type = c.LLVMTypeOf(data_value);
        const data_alloca = c.LLVMBuildAlloca(self.builder, data_type, "send_data");
        _ = c.LLVMBuildStore(self.builder, data_value, data_alloca);

        // Cast to void pointer
        const data_ptr = c.LLVMBuildBitCast(
            self.builder,
            data_alloca,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
            "data_ptr"
        );

        // Call channel send
        const send_func = self.runtime_functions.get("cursed_channel_send").?;
        return c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(send_func)),
            send_func,
            &[_]c.LLVMValueRef{ channel_value, data_ptr },
            2,
            "send_result"
        );
    }

    /// Generate channel receive expression
    fn generateChannelReceive(self: *CodeGen, recv: ast.ChannelReceiveExpression) CodeGenError!c.LLVMValueRef {
        const channel_value = try self.generateExpression(recv.channel.*);

        // Determine element type from channel type (simplified)
        const element_type = c.LLVMInt64TypeInContext(self.context); // Default to i64 for now
        
        // Allocate storage for received data
        const recv_alloca = c.LLVMBuildAlloca(self.builder, element_type, "recv_data");
        const recv_ptr = c.LLVMBuildBitCast(
            self.builder,
            recv_alloca,
            c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
            "recv_ptr"
        );

        // Call channel receive
        const recv_func = self.runtime_functions.get("cursed_channel_receive").?;
        const success = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(recv_func)),
            recv_func,
            &[_]c.LLVMValueRef{ channel_value, recv_ptr },
            2,
            "recv_success"
        );

        // Load the received value
        const received_value = c.LLVMBuildLoad2(self.builder, element_type, recv_alloca, "received_value");

        // For now, return the received value (in practice, would return a tuple with success flag)
        _ = success;
        return received_value;
    }

    /// Generate channel creation expression
    fn generateChannelCreation(self: *CodeGen, create: ast.ChannelCreationExpression) CodeGenError!c.LLVMValueRef {
        // Get element type
        const element_type = try self.getLLVMType(create.element_type);
        const element_size = c.LLVMSizeOf(element_type);

        // Get buffer size (default to 0 for unbuffered)
        const buffer_size = if (create.buffer_size) |size|
            try self.generateExpression(size)
        else
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);

        // Call channel creation function
        const create_func = self.runtime_functions.get("cursed_channel_create").?;
        const channel_ptr = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(create_func)),
            create_func,
            &[_]c.LLVMValueRef{ element_size, buffer_size },
            2,
            "channel_ptr"
        );

        // Store channel info
        const channel_info = ChannelInfo{
            .element_type = element_type,
            .channel_type = c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0),
            .buffer_size = 0, // Will be determined at runtime
        };
        const channel_name = try std.fmt.allocPrint(self.allocator, "channel_{d}", .{self.channels.count()});
        try self.channels.put(channel_name, channel_info);

        return channel_ptr;
    }

    /// Generate interface method call
    fn generateInterfaceCall(self: *CodeGen, interface_call: ast.InterfaceCallExpression) CodeGenError!c.LLVMValueRef {
        const object_value = try self.generateExpression(interface_call.object.*);

        // Generate arguments
        var args = ArrayList(c.LLVMValueRef).init(self.allocator);
        defer args.deinit();

        for (interface_call.arguments.items) |arg_expr| {
            const arg = try self.generateExpression(arg_expr);
            try args.append(arg);
        }

        // For now, create a simplified interface call
        // In a full implementation, this would involve vtable lookup
        const interface_call_func = self.runtime_functions.get("cursed_interface_call").?;
        
        // Pack arguments into a buffer (simplified)
        const args_size = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @as(u32, @intCast(args.items.len * 8)), 0);
        const malloc_func = self.functions.get("malloc").?;
        const args_buffer = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(malloc_func)),
            malloc_func,
            &[_]c.LLVMValueRef{args_size},
            1,
            "args_buffer"
        );

        // Pack arguments into buffer for interface calls
        var arg_offset: u32 = 0;
        for (args.items) |arg| {
            const arg_type = c.LLVMTypeOf(arg);
            const arg_size = c.LLVMSizeOf(arg_type);
            
            // Calculate offset within buffer
            const offset_value = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), arg_offset, 0);
            const arg_ptr = c.LLVMBuildGEP2(
                self.builder,
                c.LLVMInt8TypeInContext(self.context),
                args_buffer,
                &[_]c.LLVMValueRef{offset_value},
                1,
                "arg_ptr"
            );
            
            // Cast to proper type and store
            const typed_ptr = c.LLVMBuildBitCast(
                self.builder,
                arg_ptr,
                c.LLVMPointerType(arg_type, 0),
                "typed_arg_ptr"
            );
            _ = c.LLVMBuildStore(self.builder, arg, typed_ptr);
            
            // Update offset for next argument (align to 8 bytes)
            const size_value = c.LLVMConstTruncOrBitCast(arg_size, c.LLVMInt32TypeInContext(self.context));
            arg_offset += @intCast(c.LLVMConstIntGetZExtValue(size_value));
            arg_offset = (arg_offset + 7) & ~@as(u32, 7); // 8-byte alignment
        }

        // Get method index (simplified)
        const method_index = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0); // First method for now

        // Get vtable (simplified - would be extracted from object)
        const null_vtable = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));

        return c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(interface_call_func)),
            interface_call_func,
            &[_]c.LLVMValueRef{ object_value, null_vtable, method_index, args_buffer },
            4,
            "interface_result"
        );
    }

    /// Generate goroutine expression (for inline goroutine creation)
    fn generateGoroutineExpression(self: *CodeGen, goroutine: ast.GoroutineExpression) CodeGenError!c.LLVMValueRef {
        // Similar to generateStan but returns the goroutine ID
        const goroutine_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)},
            1,
            0
        );

        const goroutine_id = self.goroutine_counter;
        self.goroutine_counter += 1;

        const func_name = try std.fmt.allocPrint(self.allocator, "inline_goroutine_{d}", .{goroutine_id});
        const goroutine_func = c.LLVMAddFunction(self.module, func_name.ptr, goroutine_func_type);

        // Create and generate function body (similar to generateStan)
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, goroutine_func, "entry");
        
        const saved_function = self.current_function;
        const saved_block = c.LLVMGetInsertBlock(self.builder);
        
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        self.current_function = goroutine_func;
        
        _ = try self.generateExpression(goroutine.body.*);
        
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
        
        self.current_function = saved_function;
        if (saved_block != null) {
            c.LLVMPositionBuilderAtEnd(self.builder, saved_block);
        }

        // Spawn and return ID
        const spawn_func = self.runtime_functions.get("cursed_spawn_goroutine").?;
        const func_ptr = c.LLVMBuildBitCast(
            self.builder,
            goroutine_func,
            c.LLVMPointerType(c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), null, 0, 0), 0),
            "func_ptr"
        );
        const null_context = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));
        const default_stack_size = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 65536, 0);

        return c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(spawn_func)),
            spawn_func,
            &[_]c.LLVMValueRef{ func_ptr, null_context, default_stack_size },
            3,
            "goroutine_id"
        );
    }

    /// Generate range expression (for iteration)
    fn generateRange(self: *CodeGen, range: ast.RangeExpression) CodeGenError!c.LLVMValueRef {
        const start_value = try self.generateExpression(range.start.*);
        const end_value = try self.generateExpression(range.end.*);

        // Create a simple range iterator structure
        // For now, return start value (full implementation would create iterator)
        _ = end_value;
        return start_value;
    }

    /// Enhanced optimization passes for CURSED
    fn optimizeModule(self: *CodeGen) CodeGenError!void {
        // Create function pass manager
        const fpm = c.LLVMCreateFunctionPassManagerForModule(self.module);
        defer c.LLVMDisposeFunctionPassManager(fpm);

        // Add aggressive optimization passes
        c.LLVMAddInstructionCombiningPass(fpm);
        c.LLVMAddReassociatePass(fpm);
        c.LLVMAddGVNPass(fpm);
        c.LLVMAddCFGSimplificationPass(fpm);
        c.LLVMAddPromoteMemoryToRegisterPass(fpm);
        c.LLVMAddTailCallEliminationPass(fpm);
        c.LLVMAddJumpThreadingPass(fpm);
        c.LLVMAddCorrelatedValuePropagationPass(fpm);
        c.LLVMAddDeadStoreEliminationPass(fpm);
        c.LLVMAddLoopUnrollPass(fpm);

        // Initialize and run passes
        _ = c.LLVMInitializeFunctionPassManager(fpm);

        // Run passes on all functions
        var func = c.LLVMGetFirstFunction(self.module);
        while (func != null) {
            _ = c.LLVMRunFunctionPassManager(fpm, func);
            func = c.LLVMGetNextFunction(func);
        }

        _ = c.LLVMFinalizeFunctionPassManager(fpm);

        // Module-level optimization passes
        const mpm = c.LLVMCreatePassManager();
        defer c.LLVMDisposePassManager(mpm);

        c.LLVMAddGlobalOptimizerPass(mpm);
        c.LLVMAddIPSCCPPass(mpm);
        c.LLVMAddDeadArgEliminationPass(mpm);
        c.LLVMAddInstructionCombiningPass(mpm);
        c.LLVMAddCFGSimplificationPass(mpm);
        c.LLVMAddPruneEHPass(mpm);
        c.LLVMAddGlobalDCEPass(mpm);
        c.LLVMAddConstantMergePass(mpm);

        _ = c.LLVMRunPassManager(mpm, self.module);
    }

    /// Enhanced program generation with runtime setup
    pub fn generateProgramAdvanced(self: *CodeGen, program: Program) CodeGenError!void {
        // Generate runtime function declarations first
        try self.generateRuntimeDeclarations();
        
        // Generate external declarations
        try self.generateExternalDeclarations();
        
        // Generate statements
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Add main function if it doesn't exist
        if (self.functions.get("main_character") == null) {
            try self.generateMainWrapper();
        }
        
        // Verify module
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module, c.LLVMPrintMessageAction, &error_msg) != 0) {
            std.debug.print("LLVM module verification failed: {s}\n", .{error_msg});
            c.LLVMDisposeMessage(error_msg);
            return CodeGenError.LLVMError;
        }
        
        // Run enhanced optimization passes
        try self.optimizeModule();
    }

    // ===== MISSING STATEMENT IMPLEMENTATIONS =====

    /// Generate for loop statement
    fn generateFor(self: *CodeGen, for_stmt: ast.ForStatement) CodeGenError!void {
        const current_func = self.current_function.?;

        // Create loop blocks
        const condition_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for_cond");
        const body_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for_body");
        const increment_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for_inc");
        const exit_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for_exit");

        // Save current loop context
        const loop_context = LoopContext{
            .continue_block = increment_block,
            .break_block = exit_block,
        };
        try self.loop_stack.append(loop_context);
        defer _ = self.loop_stack.pop();

        // Generate initializer if present
        if (for_stmt.initializer) |init_stmt| {
            try self.generateStatement(init_stmt);
        }

        // Jump to condition
        _ = c.LLVMBuildBr(self.builder, condition_block);

        // Generate condition block
        c.LLVMPositionBuilderAtEnd(self.builder, condition_block);
        if (for_stmt.condition) |cond| {
            const condition_value = try self.generateExpression(cond);
            _ = c.LLVMBuildCondBr(self.builder, condition_value, body_block, exit_block);
        } else {
            // Infinite loop
            _ = c.LLVMBuildBr(self.builder, body_block);
        }

        // Generate body block
        c.LLVMPositionBuilderAtEnd(self.builder, body_block);
        try self.generateStatement(for_stmt.body);

        // Jump to increment
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, increment_block);
        }

        // Generate increment block
        c.LLVMPositionBuilderAtEnd(self.builder, increment_block);
        if (for_stmt.increment) |inc_stmt| {
            try self.generateStatement(inc_stmt);
        }

        // Jump back to condition
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, condition_block);
        }

        // Continue building in exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
    }

    /// Generate for-in loop statement
    fn generateForIn(self: *CodeGen, for_in_stmt: ast.ForInStatement) CodeGenError!void {
        const current_func = self.current_function.?;
        
        // Generate the iterable expression
        _ = try self.generateExpression(for_in_stmt.iterable);
        
        // Create blocks for the loop
        const condition_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for_in_cond");
        const body_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for_in_body");
        const exit_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "for_in_exit");

        // Create iterator variable
        const iterator_type = c.LLVMInt64TypeInContext(self.context);
        const iterator_alloca = c.LLVMBuildAlloca(self.builder, iterator_type, "iterator");
        const zero = c.LLVMConstInt(iterator_type, 0, 0);
        _ = c.LLVMBuildStore(self.builder, zero, iterator_alloca);

        // Jump to condition
        _ = c.LLVMBuildBr(self.builder, condition_block);

        // Generate condition block
        c.LLVMPositionBuilderAtEnd(self.builder, condition_block);
        const iterator_value = c.LLVMBuildLoad2(self.builder, iterator_type, iterator_alloca, "iter_val");
        
        // For simplicity, assume we know the collection size (should be determined from iterable type)
        const collection_size = c.LLVMConstInt(iterator_type, 10, 0); // placeholder
        const condition = c.LLVMBuildICmp(self.builder, c.LLVMIntSLT, iterator_value, collection_size, "for_in_cond");
        _ = c.LLVMBuildCondBr(self.builder, condition, body_block, exit_block);

        // Generate body block
        c.LLVMPositionBuilderAtEnd(self.builder, body_block);
        
        // Create loop variable and assign current element
        const loop_var_type = c.LLVMInt64TypeInContext(self.context); // simplified
        const loop_var_alloca = c.LLVMBuildAlloca(self.builder, loop_var_type, for_in_stmt.variable.ptr);
        _ = c.LLVMBuildStore(self.builder, iterator_value, loop_var_alloca);
        try self.variables.put(for_in_stmt.variable, loop_var_alloca);

        // Generate body
        try self.generateStatement(for_in_stmt.body);

        // Increment iterator
        const one = c.LLVMConstInt(iterator_type, 1, 0);
        const next_iter = c.LLVMBuildAdd(self.builder, iterator_value, one, "next_iter");
        _ = c.LLVMBuildStore(self.builder, next_iter, iterator_alloca);

        // Jump back to condition
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, condition_block);
        }

        // Continue building in exit block
        c.LLVMPositionBuilderAtEnd(self.builder, exit_block);
    }

    /// Generate switch statement
    fn generateSwitch(self: *CodeGen, switch_stmt: ast.SwitchStatement) CodeGenError!void {
        const switch_expr = try self.generateExpression(switch_stmt.expression);
        const current_func = self.current_function.?;
        
        const default_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "switch_default");
        const merge_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "switch_merge");
        
        // Create switch instruction
        const switch_inst = c.LLVMBuildSwitch(self.builder, switch_expr, default_block, @as(u32, @intCast(switch_stmt.cases.items.len)));

        // Generate case blocks
        for (switch_stmt.cases.items) |case_item| {
            const case_block = c.LLVMAppendBasicBlockInContext(self.context, current_func, "switch_case");
            
            // Add case value to switch
            const case_value = try self.generateExpression(case_item.value);
            c.LLVMAddCase(switch_inst, case_value, case_block);
            
            // Generate case body
            c.LLVMPositionBuilderAtEnd(self.builder, case_block);
            for (case_item.body.items) |stmt| {
                try self.generateStatement(stmt);
            }
            
            if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
                _ = c.LLVMBuildBr(self.builder, merge_block);
            }
        }

        // Generate default case
        c.LLVMPositionBuilderAtEnd(self.builder, default_block);
        if (switch_stmt.default_case) |default_case| {
            for (default_case.items) |stmt| {
                try self.generateStatement(stmt);
            }
        }
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildBr(self.builder, merge_block);
        }

        // Continue building in merge block
        c.LLVMPositionBuilderAtEnd(self.builder, merge_block);
    }

    /// Generate pattern switch statement
    fn generatePatternSwitch(self: *CodeGen, pattern_switch: ast.PatternSwitchStatement) CodeGenError!void {
        // For now, treat pattern switch like regular switch
        // In a full implementation, this would handle pattern matching
        const switch_expr = try self.generateExpression(pattern_switch.expression);
        _ = switch_expr;
        
        // Simplified pattern matching - just call first case for now
        if (pattern_switch.cases.items.len > 0) {
            const first_case = pattern_switch.cases.items[0];
            for (first_case.body.items) |stmt| {
                try self.generateStatement(stmt);
            }
        }
    }

    /// Generate goroutine statement (duplicate of generateStan for compatibility)
    fn generateGoroutineStatement(self: *CodeGen, goroutine_stmt: ast.GoroutineStatement) CodeGenError!void {
        // Same implementation as generateStan but adapted for GoroutineStatement type
        const goroutine_func_type = c.LLVMFunctionType(
            c.LLVMVoidTypeInContext(self.context),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0)},
            1,
            0
        );

        const goroutine_id = self.goroutine_counter;
        self.goroutine_counter += 1;

        const func_name = try std.fmt.allocPrint(self.allocator, "goroutine_stmt_{d}", .{goroutine_id});
        const goroutine_func = c.LLVMAddFunction(self.module, func_name.ptr, goroutine_func_type);

        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, goroutine_func, "entry");
        
        const saved_function = self.current_function;
        const saved_block = c.LLVMGetInsertBlock(self.builder);
        
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        self.current_function = goroutine_func;
        
        try self.generateStatement(goroutine_stmt.body);
        
        if (c.LLVMGetBasicBlockTerminator(c.LLVMGetInsertBlock(self.builder)) == null) {
            _ = c.LLVMBuildRetVoid(self.builder);
        }
        
        self.current_function = saved_function;
        if (saved_block != null) {
            c.LLVMPositionBuilderAtEnd(self.builder, saved_block);
        }

        // Spawn the goroutine
        const spawn_func = self.runtime_functions.get("cursed_spawn_goroutine").?;
        const func_ptr = c.LLVMBuildBitCast(
            self.builder,
            goroutine_func,
            c.LLVMPointerType(c.LLVMFunctionType(c.LLVMVoidTypeInContext(self.context), null, 0, 0), 0),
            "func_ptr"
        );
        const null_context = c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));
        const default_stack_size = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 65536, 0);

        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(spawn_func)),
            spawn_func,
            &[_]c.LLVMValueRef{ func_ptr, null_context, default_stack_size },
            3,
            "spawn_result"
        );
    }

    /// Generate channel statement
    fn generateChannelStatement(self: *CodeGen, channel_stmt: ast.ChannelStatement) CodeGenError!void {
        // Create a channel and store it in a variable
        const element_type = try self.getLLVMType(channel_stmt.element_type);
        const element_size = c.LLVMSizeOf(element_type);
        
        const buffer_size = if (channel_stmt.buffer_size) |size|
            try self.generateExpression(size)
        else
            c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);

        const create_func = self.runtime_functions.get("cursed_channel_create").?;
        const channel_ptr = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(create_func)),
            create_func,
            &[_]c.LLVMValueRef{ element_size, buffer_size },
            2,
            "channel_ptr"
        );

        // Store in variable if name is provided
        if (channel_stmt.name) |name| {
            const channel_alloca = c.LLVMBuildAlloca(self.builder, c.LLVMTypeOf(channel_ptr), name.ptr);
            _ = c.LLVMBuildStore(self.builder, channel_ptr, channel_alloca);
            try self.variables.put(name, channel_alloca);
        }
    }

    /// Generate type alias statement
    fn generateTypeAlias(self: *CodeGen, type_alias: ast.TypeAliasStatement) CodeGenError!void {
        // Store type alias for later use in type resolution
        const target_type = try self.getLLVMType(type_alias.target_type);
        try self.struct_types.put(type_alias.name, target_type);
    }

    /// Generate panic statement
    fn generatePanicStatement(self: *CodeGen, panic_stmt: ast.PanicStatement) CodeGenError!void {
        // Generate panic with message
        const panic_msg = if (panic_stmt.message) |msg|
            c.LLVMBuildGlobalStringPtr(self.builder, msg.ptr, "panic_msg")
        else
            c.LLVMBuildGlobalStringPtr(self.builder, "panic", "panic_msg");

        const panic_func = self.runtime_functions.get("cursed_panic").?;
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            panic_func,
            &[_]c.LLVMValueRef{panic_msg},
            1,
            ""
        );
        _ = c.LLVMBuildUnreachable(self.builder);
    }

    /// Generate catch statement
    fn generateCatch(self: *CodeGen, catch_stmt: ast.CatchStatement) CodeGenError!void {
        // Simplified catch implementation
        // In full implementation, would set up exception handling
        try self.generateStatement(catch_stmt.try_body);
        
        // If there's an error, execute catch body
        if (catch_stmt.catch_body) |catch_body| {
            try self.generateStatement(catch_body);
        }
    }

    /// Generate break statement
    fn generateBreak(self: *CodeGen, _: ast.BreakStatement) CodeGenError!void {
        if (self.loop_stack.items.len == 0) {
            return CodeGenError.LLVMError; // No loop to break from
        }

        const current_loop = self.loop_stack.items[self.loop_stack.items.len - 1];
        _ = c.LLVMBuildBr(self.builder, current_loop.break_block);
    }

    /// Generate continue statement
    fn generateContinue(self: *CodeGen, _: ast.ContinueStatement) CodeGenError!void {
        if (self.loop_stack.items.len == 0) {
            return CodeGenError.LLVMError; // No loop to continue
        }

        const current_loop = self.loop_stack.items[self.loop_stack.items.len - 1];
        _ = c.LLVMBuildBr(self.builder, current_loop.continue_block);
    }

    /// Generate increment statement
    fn generateIncrementStatement(self: *CodeGen, inc_stmt: ast.IncrementStatement) CodeGenError!void {
        if (self.variables.get(inc_stmt.variable)) |alloca| {
            const var_type = c.LLVMGetAllocatedType(alloca);
            const current_value = c.LLVMBuildLoad2(self.builder, var_type, alloca, "current_val");
            const one = c.LLVMConstInt(var_type, 1, 0);
            const incremented = c.LLVMBuildAdd(self.builder, current_value, one, "incremented");
            _ = c.LLVMBuildStore(self.builder, incremented, alloca);
        } else {
            return CodeGenError.UndefinedSymbol;
        }
    }

    /// Generate decrement statement
    fn generateDecrementStatement(self: *CodeGen, dec_stmt: ast.DecrementStatement) CodeGenError!void {
        if (self.variables.get(dec_stmt.variable)) |alloca| {
            const var_type = c.LLVMGetAllocatedType(alloca);
            const current_value = c.LLVMBuildLoad2(self.builder, var_type, alloca, "current_val");
            const one = c.LLVMConstInt(var_type, 1, 0);
            const decremented = c.LLVMBuildSub(self.builder, current_value, one, "decremented");
            _ = c.LLVMBuildStore(self.builder, decremented, alloca);
        } else {
            return CodeGenError.UndefinedSymbol;
        }
    }

    /// Generate short declaration statement (e.g., x := 5)
    fn generateShortDeclaration(self: *CodeGen, short_decl: ast.ShortDeclarationStatement) CodeGenError!void {
        const value = try self.generateExpression(short_decl.value);
        const value_type = c.LLVMTypeOf(value);
        const alloca = c.LLVMBuildAlloca(self.builder, value_type, short_decl.name.ptr);
        _ = c.LLVMBuildStore(self.builder, value, alloca);
        try self.variables.put(short_decl.name, alloca);
    }

    /// Generate const statement
    fn generateConst(self: *CodeGen, const_stmt: ast.ConstStatement) CodeGenError!void {
        // Create global constant
        const value = try self.generateExpression(const_stmt.value);
        const value_type = c.LLVMTypeOf(value);
        const global_const = c.LLVMAddGlobal(self.module, value_type, const_stmt.name.ptr);
        c.LLVMSetInitializer(global_const, value);
        c.LLVMSetGlobalConstant(global_const, 1);
        try self.variables.put(const_stmt.name, global_const);
    }

    // ===== MISSING EXPRESSION IMPLEMENTATIONS =====

    /// Generate map literal expression
    fn generateMapLiteral(self: *CodeGen, map: ast.MapLiteralExpression) CodeGenError!c.LLVMValueRef {
        // Create a simple map structure (hash table)
        const map_size = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), @as(u32, @intCast(map.entries.items.len)), 0);
        const malloc_func = self.functions.get("malloc").?;
        
        // Allocate memory for map entries (simplified)
        const entry_size = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), 16, 0); // key + value pointers
        const total_size = c.LLVMBuildMul(self.builder, c.LLVMConstZExt(map_size, c.LLVMInt64TypeInContext(self.context)), entry_size, "total_size");
        
        const map_ptr = c.LLVMBuildCall2(
            self.builder,
            c.LLVMGetReturnType(c.LLVMGlobalGetValueType(malloc_func)),
            malloc_func,
            &[_]c.LLVMValueRef{total_size},
            1,
            "map_alloc"
        );

        // Initialize map entries
        for (map.entries.items, 0..) |entry, i| {
            const key_value = try self.generateExpression(entry.key);
            const value_value = try self.generateExpression(entry.value);
            
            // Store key and value at appropriate offsets
            const entry_offset = c.LLVMConstInt(c.LLVMInt64TypeInContext(self.context), @as(u64, @intCast(i * 16)), 0);
            const entry_ptr = c.LLVMBuildGEP2(
                self.builder,
                c.LLVMInt8TypeInContext(self.context),
                map_ptr,
                &[_]c.LLVMValueRef{entry_offset},
                1,
                "entry_ptr"
            );
            
            // Store key (simplified)
            _ = c.LLVMBuildStore(self.builder, key_value, entry_ptr);
            _ = value_value; // Would store value at offset + 8
        }

        return map_ptr;
    }

    /// Generate composite literal expression
    fn generateCompositeLiteral(self: *CodeGen, composite: ast.CompositeLiteralExpression) CodeGenError!c.LLVMValueRef {
        // Similar to struct literal but more general
        return try self.generateStructLiteral(composite);
    }

    /// Generate lambda expression
    fn generateLambda(self: *CodeGen, lambda: ast.LambdaExpression) CodeGenError!c.LLVMValueRef {
        // Create anonymous function
        var param_types = ArrayList(c.LLVMTypeRef).init(self.allocator);
        defer param_types.deinit();
        
        for (lambda.parameters.items) |param| {
            const param_type = try self.getLLVMType(param.param_type);
            try param_types.append(param_type);
        }
        
        const return_type = if (lambda.return_type) |ret_type|
            try self.getLLVMType(ret_type)
        else
            c.LLVMVoidTypeInContext(self.context);
        
        const lambda_type = c.LLVMFunctionType(
            return_type,
            if (param_types.items.len > 0) param_types.items.ptr else null,
            @as(u32, @intCast(param_types.items.len)),
            0
        );
        
        const lambda_name = try std.fmt.allocPrint(self.allocator, "lambda_{d}", .{self.goroutine_counter});
        self.goroutine_counter += 1;
        
        const lambda_func = c.LLVMAddFunction(self.module, lambda_name.ptr, lambda_type);
        
        // Generate lambda body (simplified)
        const entry_block = c.LLVMAppendBasicBlockInContext(self.context, lambda_func, "entry");
        const saved_function = self.current_function;
        const saved_block = c.LLVMGetInsertBlock(self.builder);
        
        c.LLVMPositionBuilderAtEnd(self.builder, entry_block);
        self.current_function = lambda_func;
        
        const result = try self.generateExpression(lambda.body.*);
        _ = c.LLVMBuildRet(self.builder, result);
        
        self.current_function = saved_function;
        if (saved_block != null) {
            c.LLVMPositionBuilderAtEnd(self.builder, saved_block);
        }
        
        return lambda_func;
    }

    /// Generate slice access expression
    fn generateSliceAccess(self: *CodeGen, slice: ast.SliceAccessExpression) CodeGenError!c.LLVMValueRef {
        const array_value = try self.generateExpression(slice.object.*);
        const start_value = try self.generateExpression(slice.start.*);
        const end_value = if (slice.end) |end| try self.generateExpression(end) else null;
        
        // Create a new array/slice with elements from start to end
        // For simplicity, just return the start element
        _ = end_value;
        
        const array_ptr_type = c.LLVMTypeOf(array_value);
        const array_type = c.LLVMGetElementType(array_ptr_type);
        const element_type = c.LLVMGetElementType(array_type);
        
        const element_ptr = c.LLVMBuildGEP2(
            self.builder,
            array_type,
            array_value,
            &[_]c.LLVMValueRef{
                c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0),
                start_value
            },
            2,
            "slice_element_ptr"
        );
        
        return c.LLVMBuildLoad2(self.builder, element_type, element_ptr, "slice_element");
    }

    /// Generate type assertion expression
    fn generateTypeAssertion(self: *CodeGen, type_assert: ast.TypeAssertionExpression) CodeGenError!c.LLVMValueRef {
        const value = try self.generateExpression(type_assert.expression.*);
        const target_type = try self.getLLVMType(type_assert.target_type);
        
        // Simplified type assertion - just cast
        return c.LLVMBuildBitCast(self.builder, value, target_type, "type_assert");
    }

    /// Generate error value expression
    fn generateErrorValue(self: *CodeGen, error_val: ast.ErrorValueExpression) CodeGenError!c.LLVMValueRef {
        // Create error value (simplified - just return error code)
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), error_val.error_code, 0);
    }

    /// Generate structured error expression
    fn generateStructuredError(self: *CodeGen, structured_error: ast.StructuredErrorExpression) CodeGenError!c.LLVMValueRef {
        // Create structured error with message and code
        const error_msg = c.LLVMBuildGlobalStringPtr(self.builder, structured_error.message.ptr, "error_msg");
        const error_code = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), structured_error.code, 0);
        
        // For now, just return the error code
        _ = error_msg;
        return error_code;
    }

    /// Generate panic expression
    fn generatePanicExpression(self: *CodeGen, panic: ast.PanicExpression) CodeGenError!c.LLVMValueRef {
        const panic_msg = c.LLVMBuildGlobalStringPtr(self.builder, panic.message.ptr, "panic_msg");
        const panic_func = self.runtime_functions.get("cursed_panic").?;
        _ = c.LLVMBuildCall2(
            self.builder,
            c.LLVMVoidTypeInContext(self.context),
            panic_func,
            &[_]c.LLVMValueRef{panic_msg},
            1,
            ""
        );
        _ = c.LLVMBuildUnreachable(self.builder);
        
        // Never reached, but return something to satisfy type system
        return c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
    }

    /// Generate recover expression
    fn generateRecover(self: *CodeGen, _: ast.RecoverExpression) CodeGenError!c.LLVMValueRef {
        // Simplified recover - just return null for now
        return c.LLVMConstNull(c.LLVMPointerType(c.LLVMInt8TypeInContext(self.context), 0));
    }

    /// Generate test result expression
    fn generateTestResult(self: *CodeGen, test_result: ast.TestResultExpression) CodeGenError!c.LLVMValueRef {
        // Generate test execution and return result
        const test_expr = try self.generateExpression(test_result.expression.*);
        
        // Simplified - just return the expression result
        return test_expr;
    }

    /// Generate test result check expression
    fn generateTestResultCheck(self: *CodeGen, test_check: ast.TestResultCheckExpression) CodeGenError!c.LLVMValueRef {
        const test_result = try self.generateExpression(test_check.test_expression.*);
        
        // Check if test passed (simplified)
        const zero = c.LLVMConstInt(c.LLVMTypeOf(test_result), 0, 0);
        return c.LLVMBuildICmp(self.builder, c.LLVMIntNE, test_result, zero, "test_passed");
    }

    /// Generate range-for expression
    fn generateRangeFor(self: *CodeGen, range_for: ast.RangeForExpression) CodeGenError!c.LLVMValueRef {
        // Similar to generateRange but for expressions
        const start_value = try self.generateExpression(range_for.start.*);
        const end_value = try self.generateExpression(range_for.end.*);
        
        // Create range iterator (simplified)
        _ = end_value;
        return start_value;
    }

    /// Generate type switch expression
    fn generateTypeSwitch(self: *CodeGen, type_switch: ast.TypeSwitchExpression) CodeGenError!c.LLVMValueRef {
        const switch_expr = try self.generateExpression(type_switch.expression.*);
        
        // Simplified type switch - just return the original expression
        return switch_expr;
    }

    /// Generate block expression
    fn generateBlockExpression(self: *CodeGen, block: ast.BlockExpression) CodeGenError!c.LLVMValueRef {
        // Execute all statements in block and return last expression
        var last_value: c.LLVMValueRef = c.LLVMConstInt(c.LLVMInt32TypeInContext(self.context), 0, 0);
        
        for (block.statements.items) |stmt| {
            if (stmt.is_expression) {
                last_value = try self.generateExpression(stmt.expression);
            } else {
                try self.generateStatement(stmt.statement);
            }
        }
        
        return last_value;
    }
};

test "codegen basic" {
    const allocator = std.testing.allocator;
    
    var codegen = CodeGen.init(allocator);
    defer codegen.deinit();
    
    // Test basic initialization
    try std.testing.expect(codegen.context != null);
    try std.testing.expect(codegen.module != null);
    try std.testing.expect(codegen.builder != null);
}
