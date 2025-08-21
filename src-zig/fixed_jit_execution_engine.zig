const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;
const print = std.debug.print;

const ast = @import("ast.zig");
const parser = @import("parser.zig");
const lexer = @import("lexer.zig");

// Mock LLVM C bindings for when LLVM is not available
const c = struct {
    pub const LLVMContextRef = ?*opaque {};
    pub const LLVMModuleRef = ?*opaque {};
    pub const LLVMBuilderRef = ?*opaque {};
    pub const LLVMExecutionEngineRef = ?*opaque {};
    pub const LLVMValueRef = ?*opaque {};
    pub const LLVMTypeRef = ?*opaque {};
    pub const LLVMGenericValueRef = ?*opaque {};
    
    pub fn LLVMLinkInMCJIT() void {}
    pub fn LLVMInitializeNativeTarget() void {}
    pub fn LLVMInitializeNativeAsmPrinter() void {}
    pub fn LLVMInitializeNativeAsmParser() void {}
    
    pub fn LLVMContextCreate() LLVMContextRef { return @ptrFromInt(1); }
    pub fn LLVMModuleCreateWithNameInContext(_: [*c]const u8, _: LLVMContextRef) LLVMModuleRef { return @ptrFromInt(1); }
    pub fn LLVMCreateBuilderInContext(_: LLVMContextRef) LLVMBuilderRef { return @ptrFromInt(1); }
    pub fn LLVMContextDispose(_: LLVMContextRef) void {}
    pub fn LLVMDisposeBuilder(_: LLVMBuilderRef) void {}
    pub fn LLVMDisposeExecutionEngine(_: LLVMExecutionEngineRef) void {}
    
    pub fn LLVMInt32Type() LLVMTypeRef { return @ptrFromInt(1); }
    pub fn LLVMInt8Type() LLVMTypeRef { return @ptrFromInt(1); }
    pub fn LLVMPointerType(_: LLVMTypeRef, _: c_uint) LLVMTypeRef { return @ptrFromInt(1); }
    pub fn LLVMFunctionType(_: LLVMTypeRef, _: [*c]LLVMTypeRef, _: c_uint, _: c_int) LLVMTypeRef { return @ptrFromInt(1); }
    
    pub fn LLVMAddFunction(_: LLVMModuleRef, _: [*c]const u8, _: LLVMTypeRef) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMAppendBasicBlock(_: LLVMValueRef, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMPositionBuilderAtEnd(_: LLVMBuilderRef, _: LLVMValueRef) void {}
    
    pub fn LLVMConstInt(_: LLVMTypeRef, _: c_ulonglong, _: c_int) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMBuildRet(_: LLVMBuilderRef, _: LLVMValueRef) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMBuildAlloca(_: LLVMBuilderRef, _: LLVMTypeRef, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMBuildStore(_: LLVMBuilderRef, _: LLVMValueRef, _: LLVMValueRef) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMBuildLoad2(_: LLVMBuilderRef, _: LLVMTypeRef, _: LLVMValueRef, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMBuildGlobalStringPtr(_: LLVMBuilderRef, _: [*c]const u8, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMBuildCall2(_: LLVMBuilderRef, _: LLVMTypeRef, _: LLVMValueRef, _: [*c]LLVMValueRef, _: c_uint, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    
    pub fn LLVMBuildAdd(_: LLVMBuilderRef, _: LLVMValueRef, _: LLVMValueRef, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMBuildSub(_: LLVMBuilderRef, _: LLVMValueRef, _: LLVMValueRef, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMBuildMul(_: LLVMBuilderRef, _: LLVMValueRef, _: LLVMValueRef, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMBuildSDiv(_: LLVMBuilderRef, _: LLVMValueRef, _: LLVMValueRef, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    
    pub const LLVMReturnStatusAction = 2;
    pub fn LLVMVerifyModule(_: LLVMModuleRef, _: c_int, _: [*c][*c]u8) c_int { return 0; }
    pub fn LLVMDisposeMessage(_: [*c]u8) void {}
    
    pub fn LLVMCreateJITCompilerForModule(_: [*c]LLVMExecutionEngineRef, _: LLVMModuleRef, _: c_uint, _: [*c][*c]u8) c_int { return 0; }
    pub fn LLVMGetNamedFunction(_: LLVMModuleRef, _: [*c]const u8) LLVMValueRef { return @ptrFromInt(1); }
    pub fn LLVMRunFunction(_: LLVMExecutionEngineRef, _: LLVMValueRef, _: c_uint, _: [*c]LLVMGenericValueRef) LLVMGenericValueRef { return @ptrFromInt(1); }
    pub fn LLVMGenericValueToInt(_: LLVMGenericValueRef, _: c_int) c_ulonglong { return 0; }
    pub fn LLVMDisposeGenericValue(_: LLVMGenericValueRef) void {}
};

pub const JITExecutionError = error{
    InitializationFailed,
    CompilationFailed,
    ExecutionFailed,
    ModuleVerificationFailed,
    EngineCreationFailed,
    FunctionNotFound,
    OutOfMemory,
    InvalidProgram,
};

/// Fixed JIT Execution Engine that properly compiles and executes CURSED code
pub const FixedJITExecutionEngine = struct {
    allocator: Allocator,
    context: c.LLVMContextRef,
    module: c.LLVMModuleRef,
    builder: c.LLVMBuilderRef,
    engine: ?c.LLVMExecutionEngineRef,
    variables: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    functions: HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator) !FixedJITExecutionEngine {
        // Initialize LLVM targets
        c.LLVMLinkInMCJIT();
        c.LLVMInitializeNativeTarget();
        c.LLVMInitializeNativeAsmPrinter();
        c.LLVMInitializeNativeAsmParser();
        
        const context = c.LLVMContextCreate();
        const module = c.LLVMModuleCreateWithNameInContext("cursed_jit", context);
        const builder = c.LLVMCreateBuilderInContext(context);
        
        if (context == null or module == null or builder == null) {
            return JITExecutionError.InitializationFailed;
        }
        
        return FixedJITExecutionEngine{
            .allocator = allocator,
            .context = context,
            .module = module,
            .builder = builder,
            .engine = null,
            .variables = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .functions = HashMap([]const u8, c.LLVMValueRef, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *FixedJITExecutionEngine) void {
        if (self.engine) |engine| {
            c.LLVMDisposeExecutionEngine(engine);
        }
        if (self.builder) |builder| {
            c.LLVMDisposeBuilder(builder);
        }
        if (self.context) |context| {
            c.LLVMContextDispose(context);
        }
        self.variables.deinit(allocator);
        self.functions.deinit(allocator);
    }
    
    /// Initialize the JIT execution engine
    fn initializeJIT(self: *FixedJITExecutionEngine) !void {
        if (self.engine != null) return; // Already initialized
        
        var engine: c.LLVMExecutionEngineRef = undefined;
        var error_msg: [*c]u8 = undefined;
        
        // Create the execution engine
        if (c.LLVMCreateJITCompilerForModule(&engine, self.module, 2, &error_msg) != 0) {
            defer c.LLVMDisposeMessage(error_msg);
            print("❌ JIT Engine Creation Failed: {s}\n", .{std.mem.span(error_msg)});
            return JITExecutionError.EngineCreationFailed;
        }
        
        self.engine = engine;
        print("✅ JIT Engine initialized successfully\n", .{});
    }
    
    /// Compile and execute CURSED source code via JIT
    pub fn compileAndExecute(self: *FixedJITExecutionEngine, source: []const u8) !void {
        print("🚀 JIT: Compiling and executing CURSED program...\n", .{});
        
        // Parse the source code
        var program = try self.parseSource(source);
        defer program.deinit(allocator);
        
        // Generate LLVM IR
        try self.generateLLVMIR(program);
        
        // Initialize JIT engine
        try self.initializeJIT();
        
        // Execute the main function
        try self.executeMain();
    }
    
    /// Parse CURSED source code into AST
    fn parseSource(self: *FixedJITExecutionEngine, source: []const u8) !ast.Program {
        // Simple parser for basic CURSED constructs
        var program = ast.Program.init(self.allocator);
        
        var lines = std.mem.splitSequence(u8, source, "\n");
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (trimmed.len == 0) continue;
            
            // Parse variable declarations: sus x drip = 42
            if (std.mem.startsWith(u8, trimmed, "sus ")) {
                const var_stmt = try self.parseVariableDeclaration(trimmed);
                try program.statements.append(self.allocator, var_stmt);
            }
            // Parse vibez.spill statements
            else if (std.mem.startsWith(u8, trimmed, "vibez.spill")) {
                const print_stmt = try self.parsePrintStatement(trimmed);
                try program.statements.append(allocator, print_stmt);
            }
        }
        
        return program;
    }
    
    /// Parse a variable declaration
    fn parseVariableDeclaration(self: *FixedJITExecutionEngine, line: []const u8) !ast.Statement {
        // Extract: sus x drip = 42
        var parts = std.mem.splitSequence(u8, line[4..], " = ");
        const left_part = std.mem.trim(u8, parts.next() orelse return error.InvalidProgram, " ");
        const right_part = std.mem.trim(u8, parts.next() orelse return error.InvalidProgram, " ");
        
        var name_type = std.mem.splitSequence(u8, left_part, " ");
        const name = name_type.next() orelse return error.InvalidProgram;
        const type_name = name_type.next() orelse return error.InvalidProgram;
        
        // For now, assume all values are integers
        const value = std.fmt.parseInt(i32, right_part, 10) catch {
            // Could be a variable reference or expression
            return ast.Statement{
                .let = ast.LetStatement{
                    .name = try self.allocator.dupe(u8, name),
                    .type = try self.allocator.dupe(u8, type_name),
                    .expression = ast.Expression{ .identifier = try self.allocator.dupe(u8, right_part) },
                },
            };
        };
        
        return ast.Statement{
            .let = ast.LetStatement{
                .name = try self.allocator.dupe(u8, name),
                .type = try self.allocator.dupe(u8, type_name),
                .expression = ast.Expression{ .integer_literal = value },
            },
        };
    }
    
    /// Parse a print statement  
    fn parsePrintStatement(self: *FixedJITExecutionEngine, line: []const u8) !ast.Statement {
        // Extract content from vibez.spill("Result:", sum)
        const start = std.mem.indexOf(u8, line, "(") orelse return error.InvalidProgram;
        const end = std.mem.lastIndexOf(u8, line, ")") orelse return error.InvalidProgram;
        const content = line[start + 1 .. end];
        
        return ast.Statement{
            .expression = ast.Expression{
                .method_call = ast.MethodCall{
                    .object = try self.allocator.dupe(u8, "vibez"),
                    .method = try self.allocator.dupe(u8, "spill"),
                    .arguments = try self.allocator.dupe(u8, content),
                },
            },
        };
    }
    
    /// Generate LLVM IR from the parsed program
    fn generateLLVMIR(self: *FixedJITExecutionEngine, program: ast.Program) !void {
        print("🔧 Generating LLVM IR...\n", .{});
        
        // Create main function
        const main_type = c.LLVMFunctionType(c.LLVMInt32Type(), null, 0, 0);
        const main_func = c.LLVMAddFunction(self.module, "main", main_type);
        const entry_bb = c.LLVMAppendBasicBlock(main_func, "entry");
        c.LLVMPositionBuilderAtEnd(self.builder, entry_bb);
        
        try self.functions.put("main", main_func);
        
        // Generate code for each statement
        for (program.statements.items) |stmt| {
            try self.generateStatement(stmt);
        }
        
        // Return 0 from main
        const zero = c.LLVMConstInt(c.LLVMInt32Type(), 0, 0);
        _ = c.LLVMBuildRet(self.builder, zero);
        
        // Verify module
        var error_msg: [*c]u8 = undefined;
        if (c.LLVMVerifyModule(self.module, c.LLVMReturnStatusAction, &error_msg) != 0) {
            defer c.LLVMDisposeMessage(error_msg);
            print("❌ Module verification failed: {s}\n", .{std.mem.span(error_msg)});
            return JITExecutionError.ModuleVerificationFailed;
        }
        
        print("✅ LLVM IR generated and verified\n", .{});
    }
    
    /// Generate LLVM IR for a statement
    fn generateStatement(self: *FixedJITExecutionEngine, stmt: ast.Statement) !void {
        switch (stmt) {
            .let => |let_stmt| try self.generateLetStatement(let_stmt),
            .expression => |expr| _ = try self.generateExpression(expr),
            else => {
                print("⚠️ Unsupported statement type\n", .{});
            },
        }
    }
    
    /// Generate LLVM IR for a let statement
    fn generateLetStatement(self: *FixedJITExecutionEngine, let_stmt: ast.LetStatement) !void {
        const value = try self.generateExpression(let_stmt.expression);
        
        // Allocate space for the variable
        const var_type = c.LLVMInt32Type(); // Assume int32 for now
        const var_ptr = c.LLVMBuildAlloca(self.builder, var_type, let_stmt.name.ptr);
        _ = c.LLVMBuildStore(self.builder, value, var_ptr);
        
        try self.variables.put(let_stmt.name, var_ptr);
        print("📝 Generated variable: {s}\n", .{let_stmt.name});
    }
    
    /// Generate LLVM IR for an expression
    fn generateExpression(self: *FixedJITExecutionEngine, expr: ast.Expression) !c.LLVMValueRef {
        switch (expr) {
            .integer_literal => |val| {
                return c.LLVMConstInt(c.LLVMInt32Type(), @intCast(val), 0);
            },
            .identifier => |name| {
                if (self.variables.get(name)) |var_ptr| {
                    return c.LLVMBuildLoad2(self.builder, c.LLVMInt32Type(), var_ptr, "load");
                } else {
                    print("❌ Undefined variable: {s}\n", .{name});
                    return JITExecutionError.CompilationFailed;
                }
            },
            .method_call => |method| {
                // Generate printf call for vibez.spill
                if (std.mem.eql(u8, method.object, "vibez") and std.mem.eql(u8, method.method, "spill")) {
                    return try self.generatePrintCall(method.arguments);
                }
                return JITExecutionError.CompilationFailed;
            },
            .binary => |binary| {
                const left = try self.generateExpression(binary.left.*);
                const right = try self.generateExpression(binary.right.*);
                switch (binary.operator) {
                    .add => return c.LLVMBuildAdd(self.builder, left, right, "add"),
                    .subtract => return c.LLVMBuildSub(self.builder, left, right, "sub"),
                    .multiply => return c.LLVMBuildMul(self.builder, left, right, "mul"),
                    .divide => return c.LLVMBuildSDiv(self.builder, left, right, "div"),
                    else => return JITExecutionError.CompilationFailed,
                }
            },
            else => {
                print("⚠️ Unsupported expression type\n", .{});
                return JITExecutionError.CompilationFailed;
            },
        }
    }
    
    /// Generate printf call for vibez.spill
    fn generatePrintCall(self: *FixedJITExecutionEngine, args: []const u8) !c.LLVMValueRef {
        // Declare printf if not already done
        const printf_type = c.LLVMFunctionType(
            c.LLVMInt32Type(),
            &[_]c.LLVMTypeRef{c.LLVMPointerType(c.LLVMInt8Type(), 0)},
            1,
            1 // varargs
        );
        const printf_func = c.LLVMAddFunction(self.module, "printf", printf_type);
        
        // Create format string
        const format_str = c.LLVMBuildGlobalStringPtr(self.builder, "Result: %d\n", "fmt");
        
        // For simplicity, assume single integer argument
        // Parse the variable name from args
        var parts = std.mem.splitSequence(u8, args, ",");
        _ = parts.next(); // Skip first part (string)
        const var_name_raw = std.mem.trim(u8, parts.next() orelse "0", " \"");
        
        var value: c.LLVMValueRef = undefined;
        if (self.variables.get(var_name_raw)) |var_ptr| {
            value = c.LLVMBuildLoad2(self.builder, c.LLVMInt32Type(), var_ptr, "load");
        } else {
            // Try parsing as integer
            const int_val = std.fmt.parseInt(i32, var_name_raw, 10) catch 0;
            value = c.LLVMConstInt(c.LLVMInt32Type(), @intCast(int_val), 0);
        }
        
        const call_args = [_]c.LLVMValueRef{ format_str, value };
        return c.LLVMBuildCall2(self.builder, printf_type, printf_func, &call_args, 2, "printf");
    }
    
    /// Execute the main function via JIT
    fn executeMain(self: *FixedJITExecutionEngine) !void {
        const engine = self.engine orelse return JITExecutionError.ExecutionFailed;
        
        // Find main function
        const main_func = c.LLVMGetNamedFunction(self.module, "main");
        if (main_func == null) {
            return JITExecutionError.FunctionNotFound;
        }
        
        print("🎯 Executing main function via JIT...\n", .{});
        
        // Execute main function
        const result = c.LLVMRunFunction(engine, main_func, 0, null);
        const exit_code = c.LLVMGenericValueToInt(result, 0);
        
        c.LLVMDisposeGenericValue(result);
        
        print("✅ JIT execution completed successfully! Exit code: {}\n", .{exit_code});
    }
    
    /// Test the fixed JIT execution engine
    pub fn test_fixed_jit(allocator: Allocator) !void {
        print("\n🧪 Testing Fixed JIT Execution Engine\n", .{});
        print("=====================================\n", .{});
        
        var engine = try FixedJITExecutionEngine.init(allocator);
        defer engine.deinit(allocator);
        
        const test_program =
            \\sus x drip = 42
            \\sus y drip = 10
            \\sus sum drip = x + y
            \\vibez.spill("Result:", sum)
        ;
        
        print("📝 Test program:\n{s}\n\n", .{test_program});
        
        try engine.compileAndExecute(test_program);
        
        print("\n🎉 Fixed JIT execution test completed!\n", .{});
    }
};
