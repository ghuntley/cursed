const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Import AST and components
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const type_system = @import("type_system_runtime.zig");

// Use Zig's built-in LLVM IR builder (cross-platform, no C dependencies)
const llvm = std.zig.llvm;

/// Error types for LLVM IR Pipeline
const LLVMIRError = error{
    LLVMContextCreationFailed,
    LLVMModuleCreationFailed,
    LLVMBuilderCreationFailed,
    TargetCreationFailed,
    TargetMachineCreationFailed,
    UndefinedVariable,
    UndefinedFunction,
    ModuleVerificationFailed,
    IRWriteFailed,
    CompilationFailed,
    OutOfMemory,
    UnsupportedOperator,
    // Lexer errors
    UnexpectedCharacter,
    InvalidEscapeSequence,
    InvalidHexEscape,
    InvalidUnicodeEscape,
    UnterminatedString,
    UnterminatedChar,
    UnterminatedBlockComment,
    // Parser errors
    ParseError,
    SyntaxError,
    InvalidExpression,
    UnexpectedToken,
    UnexpectedEof,
    InvalidSyntax,
    MissingToken,
    InvalidStatement,
    InvalidType,
    InvalidFunction,
    InvalidParameter,
    InvalidBlock,
    InvalidAssignment,
    InvalidPattern,
    InvalidGeneric,
    AlignmentError,
};

/// Cross-platform LLVM IR Generation Pipeline using Zig's native LLVM builder
/// Works on Linux, Windows, macOS without external LLVM library dependencies
pub const LLVMIRPipeline = struct {
    allocator: Allocator,
    arena: std.heap.ArenaAllocator,
    
    // Zig LLVM IR Builder (cross-platform)
    builder: llvm.Builder,
    
    // Type checking integration
    type_checker: type_system.TypeChecker,
    
    // Symbol tables using Zig native types
    functions: HashMap([]const u8, llvm.Builder.Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: HashMap([]const u8, llvm.Builder.Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variable_types: HashMap([]const u8, llvm.Builder.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    array_lengths: HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    global_strings: HashMap([]const u8, llvm.Builder.Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Function signature registry for forward declarations
    function_signatures: HashMap([]const u8, llvm.Builder.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // CURSED module compilation tracking
    compiled_modules: HashMap([]const u8, void, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Type mapping
    type_cache: HashMap([]const u8, llvm.Builder.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Current compilation state
    current_function: ?llvm.Builder.Value,
    current_module_name: ?[]const u8,
    program_package: ?[]const u8,
    
    // Optimization settings
    optimization_level: u8,
    debug_info: bool,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, module_name: []const u8) !Self {
        const arena = std.heap.ArenaAllocator.init(allocator);
        
        // Initialize Zig's LLVM IR builder (no external dependencies)
        var builder = try llvm.Builder.init(.{
            .allocator = allocator,
            .strip = false,
        });
        
        // Set module metadata
        const source_filename = try builder.string(module_name);
        builder.source_filename = source_filename;
        
        // Initialize type checker with dummy registries for now
        var gc_registry = type_system.GCTypeRegistry.init(allocator);
        var interface_registry = type_system.InterfaceRegistry.init(allocator);
        const type_checker = type_system.TypeChecker.init(&gc_registry, &interface_registry);
        
        return Self{
            .allocator = allocator,
            .arena = arena,
            .builder = builder,
            .type_checker = type_checker,
            .functions = HashMap([]const u8, llvm.Builder.Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = HashMap([]const u8, llvm.Builder.Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variable_types = HashMap([]const u8, llvm.Builder.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .array_lengths = HashMap([]const u8, u32, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .global_strings = HashMap([]const u8, llvm.Builder.Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .function_signatures = HashMap([]const u8, llvm.Builder.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .compiled_modules = HashMap([]const u8, void, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .type_cache = HashMap([]const u8, llvm.Builder.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
            .current_module_name = null,
            .program_package = null,
            .optimization_level = 0,
            .debug_info = false,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.functions.deinit();
        self.variables.deinit();
        self.variable_types.deinit();
        self.array_lengths.deinit();
        self.global_strings.deinit();
        self.function_signatures.deinit();
        self.compiled_modules.deinit();
        self.type_cache.deinit();
        // TypeChecker doesn't have deinit method, skip it
        self.builder.deinit();
        self.arena.deinit();
    }
    
    /// Main compilation entry point
    pub fn compileSource(self: *Self, source: []const u8, output_file: []const u8, verbose: bool) !void {
        if (verbose) print("🔥 Starting Zig-native LLVM compilation...\n", .{});
        
        // Step 1: Parse CURSED source
        if (verbose) print("🔍 Parsing CURSED source...\n", .{});
        var lex = lexer.Lexer.init(self.allocator, source);
        var tokens_list = try lex.tokenize();
        defer tokens_list.deinit(self.allocator);
        
        var cursed_parser = parser.Parser.initWithFile(self.allocator, tokens_list.items, "source.💀");
        defer cursed_parser.deinit();
        
        const program = try cursed_parser.parseProgram();
        
        // Step 2: Generate LLVM IR using Zig builder
        if (verbose) print("⚡ Generating LLVM IR...\n", .{});
        try self.compileProgram(&program);
        
        // Step 3: Write LLVM bitcode to file
        if (verbose) print("📝 Writing LLVM bitcode...\n", .{});
        try self.writeBitcode(output_file);
        
        // Step 4: Generate native executable
        if (verbose) print("🏗️ Compiling to native binary...\n", .{});
        try self.compileToNative(output_file);
        
        if (verbose) print("✅ Zig-native LLVM compilation complete!\n", .{});
    }
    
    /// Compile AST program to LLVM IR
    fn compileProgram(self: *Self, program: *const ast.Program) !void {
        // Initialize basic types
        try self.initializeTypes();
        
        // Process each statement
        for (program.statements.items) |stmt| {
            try self.compileStatement(stmt);
        }
    }
    
    /// Initialize LLVM basic types
    fn initializeTypes(self: *Self) !void {
        // Cache common types for efficient access using Zig's predefined type constants
        const i32_type = llvm.Builder.Type.i32;
        const i64_type = llvm.Builder.Type.i64;
        const f64_type = llvm.Builder.Type.double;
        const i8_type = llvm.Builder.Type.i8;
        const void_type = llvm.Builder.Type.void;
        
        try self.type_cache.put("i32", i32_type);
        try self.type_cache.put("i64", i64_type);
        try self.type_cache.put("f64", f64_type);
        try self.type_cache.put("i8", i8_type);
        try self.type_cache.put("void", void_type);
    }
    
    /// Compile a statement to LLVM IR
    fn compileStatement(self: *Self, stmt: *const ast.Statement) !void {
        switch (stmt.*) {
            .Function => |func_stmt| try self.compileFunctionStmt(func_stmt),
            .Let => |let_stmt| try self.compileLetStmt(let_stmt),
            .Assignment => |assign_stmt| try self.compileAssignmentStmt(assign_stmt),
            .Expression => |expr| _ = try self.compileExpression(expr),
            .Return => |ret| try self.compileReturn(ret),
            .If => |if_stmt| try self.compileIf(if_stmt),
            .While => |while_stmt| try self.compileWhile(while_stmt),
            else => {
                print("⚠️ Unsupported statement type in LLVM generation\n", .{});
            },
        }
    }
    
    /// Compile function statement
    fn compileFunctionStmt(self: *Self, func_stmt: *const ast.FunctionStatement) !void {
        const func_name = func_stmt.name.lexeme;
        
        // Create function type
        const return_type = self.type_cache.get("void") orelse llvm.Builder.Type.void;
        
        // For now, create simple functions with no parameters
        var param_types: [0]llvm.Builder.Type = .{};
        const func_type = try self.builder.functionType(return_type, &param_types, false);
        
        // Create function name string
        const func_name_str = try self.builder.string(func_name);
        
        // Create function
        const function = try self.builder.addFunction(func_type, func_name_str, .default);
        
        try self.functions.put(func_name, function.toValue());
        self.current_function = function.toValue();
        
        // Create entry basic block
        _ = try self.builder.appendBasicBlock(function, "entry");
        
        // Compile function body
        if (func_stmt.body) |body| {
            for (body.items) |stmt| {
                try self.compileStatement(stmt);
            }
        }
        
        // Add return if not present
        try self.builder.ret(null);
        
        self.current_function = null;
    }
    
    /// Compile let statement (variable declaration)
    fn compileLetStmt(self: *Self, let_stmt: *const ast.LetStatement) !void {
        const var_name = let_stmt.name.lexeme;
        const var_type = self.type_cache.get("i64") orelse llvm.Builder.Type.i64;
        
        // Create alloca for local variable
        const alloca = try self.builder.alloca(var_type, "");
        try self.variables.put(var_name, alloca);
        try self.variable_types.put(var_name, var_type);
        
        // Initialize if there's an initializer
        if (let_stmt.initializer) |initializer| {
            const value = try self.compileExpression(initializer);
            _ = try self.builder.store(value, alloca);
        }
    }
    
    /// Compile assignment statement
    fn compileAssignmentStmt(self: *Self, assign_stmt: *const ast.AssignmentStatement) !void {
        const var_name = switch (assign_stmt.name.*) {
            .Identifier => |ident| ident.token.lexeme,
            else => return, // Skip non-identifier assignments for now
        };
        
        if (self.variables.get(var_name)) |var_ref| {
            const value = try self.compileExpression(assign_stmt.value);
            _ = try self.builder.store(value, var_ref);
        }
    }
    
    /// Compile expression to LLVM value
    fn compileExpression(self: *Self, expr: *const ast.Expression) !llvm.Builder.Value {
        return switch (expr.*) {
            .IntegerLiteral => |int_lit| {
                const val = std.fmt.parseInt(i64, int_lit.token.lexeme, 10) catch 0;
                return self.builder.intConst(self.builder.intType(64), val);
            },
            .FloatLiteral => |float_lit| {
                const val = std.fmt.parseFloat(f64, float_lit.token.lexeme) catch 0.0;
                return self.builder.doubleConst(val);
            },
            .StringLiteral => |string_lit| {
                return try self.compileStringLiteral(string_lit.token.lexeme);
            },
            .Identifier => |ident| {
                return try self.compileIdentifier(ident.token.lexeme);
            },
            .Binary => |binary| {
                return try self.compileBinaryExpression(binary);
            },
            .Call => |call| {
                return try self.compileCall(call);
            },
            else => {
                print("⚠️ Unsupported expression type in LLVM generation\n", .{});
                return self.builder.intConst(self.builder.intType(64), 0);
            },
        };
    }
    
    /// Compile string literal
    fn compileStringLiteral(self: *Self, str: []const u8) !llvm.Builder.Value {
        // Create global string constant
        const str_type = try self.builder.arrayType(str.len, self.builder.intType(8));
        const str_value = try self.builder.stringConst(str);
        
        const global = try self.builder.addGlobal(str_type, "");
        _ = try self.builder.setInitializer(global, str_value);
        
        return global;
    }
    
    /// Compile identifier reference
    fn compileIdentifier(self: *Self, name: []const u8) !llvm.Builder.Value {
        if (self.variables.get(name)) |var_ref| {
            const var_type = self.variable_types.get(name) orelse self.builder.intType(64);
            return try self.builder.load(var_type, var_ref, "");
        }
        
        if (self.functions.get(name)) |func| {
            return func;
        }
        
        // Return default value if not found
        return self.builder.intConst(self.builder.intType(64), 0);
    }
    
    /// Compile binary expression
    fn compileBinaryExpression(self: *Self, binary: *const ast.BinaryExpression) !llvm.Builder.Value {
        const left = try self.compileExpression(binary.left);
        const right = try self.compileExpression(binary.right);
        
        return switch (binary.operator.kind) {
            .Plus => try self.builder.add(left, right, ""),
            .Minus => try self.builder.sub(left, right, ""),
            .Star => try self.builder.mul(left, right, ""),
            .Slash => try self.builder.sdiv(left, right, ""),
            else => {
                print("⚠️ Unsupported binary operator in LLVM generation\n", .{});
                return left;
            },
        };
    }
    
    /// Compile function call
    fn compileCall(self: *Self, call: *const ast.CallExpression) !llvm.Builder.Value {
        const func_name = switch (call.callee.*) {
            .Identifier => |ident| ident.token.lexeme,
            else => return self.builder.intConst(self.builder.intType(64), 0),
        };
        
        if (self.functions.get(func_name)) |function| {
            // For now, compile calls with no arguments
            var args: [0]llvm.Builder.Value = .{};
            return try self.builder.call(function, &args, "");
        }
        
        return self.builder.intConst(self.builder.intType(64), 0);
    }
    
    /// Compile return statement
    fn compileReturn(self: *Self, ret: *const ast.ReturnStatement) !void {
        if (ret.value) |val| {
            const return_val = try self.compileExpression(val);
            try self.builder.ret(return_val);
        } else {
            try self.builder.ret(null);
        }
    }
    
    /// Compile if statement
    fn compileIf(self: *Self, if_stmt: *const ast.IfStatement) !void {
        const condition = try self.compileExpression(if_stmt.condition);
        const current_func = self.current_function orelse return;
        
        const then_block = try self.builder.appendBasicBlock(current_func, "if.then");
        const merge_block = try self.builder.appendBasicBlock(current_func, "if.end");
        
        const else_block = if (if_stmt.else_branch != null) 
            try self.builder.appendBasicBlock(current_func, "if.else")
        else 
            merge_block;
            
        try self.builder.condBr(condition, then_block, else_block);
        
        // Compile then branch
        try self.builder.positionAtEnd(then_block);
        try self.compileStatement(if_stmt.then_branch);
        try self.builder.br(merge_block);
        
        // Compile else branch if present
        if (if_stmt.else_branch) |else_branch| {
            try self.builder.positionAtEnd(else_block);
            try self.compileStatement(else_branch);
            try self.builder.br(merge_block);
        }
        
        try self.builder.positionAtEnd(merge_block);
    }
    
    /// Compile while statement
    fn compileWhile(self: *Self, while_stmt: *const ast.WhileStatement) !void {
        const current_func = self.current_function orelse return;
        
        const loop_header = try self.builder.appendBasicBlock(current_func, "while.cond");
        const loop_body = try self.builder.appendBasicBlock(current_func, "while.body");
        const loop_end = try self.builder.appendBasicBlock(current_func, "while.end");
        
        try self.builder.br(loop_header);
        
        // Loop condition
        try self.builder.positionAtEnd(loop_header);
        const condition = try self.compileExpression(while_stmt.condition);
        try self.builder.condBr(condition, loop_body, loop_end);
        
        // Loop body
        try self.builder.positionAtEnd(loop_body);
        try self.compileStatement(while_stmt.body);
        try self.builder.br(loop_header);
        
        // Continue after loop
        try self.builder.positionAtEnd(loop_end);
    }
    
    /// Compile block statement
    fn compileBlock(self: *Self, block: *const ast.BlockStatement) !void {
        for (block.statements.items) |stmt| {
            try self.compileStatement(stmt);
        }
    }
    
    /// Write LLVM bitcode to file
    fn writeBitcode(self: *Self, output_file: []const u8) !void {
        const bitcode_file = try std.fmt.allocPrint(self.allocator, "{s}.bc", .{output_file});
        defer self.allocator.free(bitcode_file);
        
        var file = try std.fs.cwd().createFile(bitcode_file, .{});
        defer file.close();
        
        try llvm.bitcode_writer.writeBitcode(file.writer(), &self.builder);
    }
    
    /// Compile bitcode to native executable (stub - needs external toolchain)
    fn compileToNative(_: *Self, output_file: []const u8) !void {
        // For now, just output bitcode
        // In a full implementation, this would invoke the system linker
        print("🎉 Generated LLVM bitcode for {s}\n", .{output_file});
        print("💡 To compile to native: lli {s}.bc\n", .{output_file});
    }
};
