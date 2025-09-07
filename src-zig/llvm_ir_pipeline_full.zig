const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Import AST and components
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

// Use Zig's built-in LLVM IR builder (cross-platform, no C dependencies)
const llvm = std.zig.llvm;

/// Full Cross-platform LLVM IR Generation Pipeline using Zig's native LLVM builder
/// Works on Linux, Windows, macOS without external LLVM library dependencies
pub const LLVMIRPipeline = struct {
    allocator: Allocator,
    
    // Zig LLVM IR Builder (cross-platform)
    builder: llvm.Builder,
    
    // Symbol tables using Zig native types
    functions: HashMap([]const u8, llvm.Builder.Function.Index, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variables: HashMap([]const u8, llvm.Builder.Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    variable_types: HashMap([]const u8, llvm.Builder.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    // Current compilation state
    current_function: ?llvm.Builder.WipFunction,
    
    // Optimization settings
    optimization_level: u8,
    debug_info: bool,
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, module_name: []const u8) !Self {
        // Initialize Zig's LLVM IR builder (no external dependencies)
        var builder = try llvm.Builder.init(.{
            .allocator = allocator,
            .strip = false,
        });
        
        // Set module metadata
        const source_filename = try builder.string(module_name);
        builder.source_filename = source_filename;
        
        return Self{
            .allocator = allocator,
            .builder = builder,
            .functions = HashMap([]const u8, llvm.Builder.Function.Index, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variables = HashMap([]const u8, llvm.Builder.Value, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .variable_types = HashMap([]const u8, llvm.Builder.Type, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .current_function = null,
            .optimization_level = 0,
            .debug_info = false,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Clean up current function if still active
        if (self.current_function) |*wip_func| {
            wip_func.deinit();
        }
        
        self.functions.deinit();
        self.variables.deinit();
        self.variable_types.deinit();
        self.builder.deinit();
    }
    
    /// Main compilation entry point
    pub fn compileSource(self: *Self, source: []const u8, output_file: []const u8, verbose: bool) !void {
        if (verbose) print("🔥 Starting full Zig-native LLVM compilation...\n", .{});
        
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
        
        if (verbose) print("✅ Full Zig-native LLVM compilation complete!\n", .{});
    }
    
    /// Compile AST program to LLVM IR
    fn compileProgram(self: *Self, program: *const ast.Program) !void {
        // First pass: declare all functions
        for (program.statements.items) |stmt| {
            switch (stmt.*) {
                .Function => |func_stmt| try self.declareFunctionStmt(&func_stmt),
                else => {},
            }
        }
        
        // Second pass: implement all functions
        for (program.statements.items) |stmt| {
            try self.compileStatement(stmt);
        }
    }
    
    /// Declare function signature without implementation
    fn declareFunctionStmt(self: *Self, func_stmt: *const ast.FunctionStatement) !void {
        const func_name = func_stmt.name;
        
        // Create function type - for now, simple void() functions
        var param_types: [0]llvm.Builder.Type = .{};
        const func_type = try self.builder.fnType(llvm.Builder.Type.void, &param_types, .normal);
        
        // Create function name string
        const func_name_str = try self.builder.strtabString(func_name);
        
        // Add function to module
        const function = try self.builder.addFunction(func_type, func_name_str, .default);
        
        // Store function for later reference
        try self.functions.put(func_name, function);
    }
    
    /// Compile a statement to LLVM IR
    fn compileStatement(self: *Self, stmt: *const ast.Statement) !void {
        switch (stmt.*) {
            .Function => |func_stmt| try self.compileFunctionStmt(&func_stmt),
            .Let => |let_stmt| try self.compileLetStmt(&let_stmt),
            .Assignment => |assign_stmt| try self.compileAssignmentStmt(&assign_stmt),
            .Expression => |expr| _ = try self.compileExpression(expr),
            .Return => |ret| try self.compileReturn(&ret),
            else => {
                if (@import("builtin").mode == .Debug) {
                    print("⚠️ Unsupported statement type in LLVM generation: {}\n", .{stmt.*});
                }
            },
        }
    }
    
    /// Compile function statement implementation
    fn compileFunctionStmt(self: *Self, func_stmt: *const ast.FunctionStatement) !void {
        const func_name = func_stmt.name;
        
        // Get the pre-declared function
        const function = self.functions.get(func_name) orelse {
            print("❌ Function {s} not found in declarations\n", .{func_name});
            return;
        };
        
        // Start implementing the function
        var wip_function = try llvm.Builder.WipFunction.init(&self.builder, .{
            .function = function,
            .strip = false,
        });
        self.current_function = wip_function;
        
        // Create entry basic block
        _ = try wip_function.block(0, "entry");
        
        // Compile function body
        for (func_stmt.body.items) |stmt| {
            try self.compileStatement(stmt);
        }
        
        // Add return if function doesn't end with one
        _ = try wip_function.retVoid();
        
        // Finish function implementation
        try wip_function.finish();
        wip_function.deinit();
        self.current_function = null;
    }
    
    /// Compile let statement (variable declaration)
    fn compileLetStmt(self: *Self, let_stmt: *const ast.LetStatement) !void {
        const var_name = let_stmt.name;
        const var_type = llvm.Builder.Type.i64; // Default to i64 for now
        
        _ = &(self.current_function orelse {
            print("❌ Let statement outside function context\n", .{});
            return;
        });
        
        // TODO: Implement let statement compilation
        _ = var_name;
        _ = var_type;
        print("⚠️ Let statements not yet fully implemented in LLVM backend\n", .{});
    }
    
    /// Compile assignment statement
    fn compileAssignmentStmt(self: *Self, assign_stmt: *const ast.AssignmentStatement) !void {
        _ = assign_stmt; // TODO: Implement assignment statements
        _ = self; // TODO: Remove when implementation is added
        print("⚠️ Assignment statements not yet implemented in LLVM backend\n", .{});
    }
    
    /// Compile expression to LLVM value
    fn compileExpression(self: *Self, expr: *const ast.Expression) !llvm.Builder.Value {
        _ = &(self.current_function orelse {
            print("❌ Expression outside function context\n", .{});
            return (try self.builder.intConst(llvm.Builder.Type.i64, 0)).toValue();
        });
        
        return switch (expr.*) {
            .IntegerLiteral => |int_lit| {
                const val = std.fmt.parseInt(i64, int_lit.token.lexeme, 10) catch 0;
                return (try self.builder.intConst(llvm.Builder.Type.i64, val)).toValue();
            },
            .FloatLiteral => |float_lit| {
                const val = std.fmt.parseFloat(f64, float_lit.token.lexeme) catch 0.0;
                return (try self.builder.floatConst(llvm.Builder.Type.double, val)).toValue();
            },
            .Identifier => |ident| {
                return try self.compileIdentifier(ident.token.lexeme);
            },
            .Binary => |binary| {
                return try self.compileBinaryExpression(binary);
            },
            else => {
                if (@import("builtin").mode == .Debug) {
                    print("⚠️ Unsupported expression type in LLVM generation: {}\n", .{expr.*});
                }
                return (try self.builder.intConst(llvm.Builder.Type.i64, 0)).toValue();
            },
        };
    }
    
    /// Compile identifier reference
    fn compileIdentifier(self: *Self, name: []const u8) !llvm.Builder.Value {
        const wip_func = &(self.current_function orelse {
            return (try self.builder.intConst(llvm.Builder.Type.i64, 0)).toValue();
        });
        
        if (self.variables.get(name)) |var_ref| {
            const var_type = self.variable_types.get(name) orelse llvm.Builder.Type.i64;
            return try wip_func.load(.normal, var_type, var_ref, .default, "");
        }
        
        // Return default value if not found
        return (try self.builder.intConst(llvm.Builder.Type.i64, 0)).toValue();
    }
    
    /// Compile binary expression
    fn compileBinaryExpression(self: *Self, binary: *const ast.BinaryExpression) !llvm.Builder.Value {
        const wip_func = &(self.current_function orelse {
            return (try self.builder.intConst(llvm.Builder.Type.i64, 0)).toValue();
        });
        
        const left = try self.compileExpression(binary.left);
        const right = try self.compileExpression(binary.right);
        
        return switch (binary.operator.kind) {
            .Plus => try wip_func.bin(.add, left, right, ""),
            .Minus => try wip_func.bin(.sub, left, right, ""),
            .Star => try wip_func.bin(.mul, left, right, ""),
            .Slash => try wip_func.bin(.sdiv, left, right, ""),
            else => {
                if (@import("builtin").mode == .Debug) {
                    print("⚠️ Unsupported binary operator in LLVM generation\n", .{});
                }
                return left;
            },
        };
    }
    
    /// Compile return statement
    fn compileReturn(self: *Self, ret: *const ast.ReturnStatement) !void {
        const wip_func = &(self.current_function orelse {
            print("❌ Return statement outside function context\n", .{});
            return;
        });
        
        if (ret.value) |val| {
            const return_val = try self.compileExpression(val);
            _ = try wip_func.ret(return_val);
        } else {
            _ = try wip_func.retVoid();
        }
    }
    
    /// Write LLVM bitcode to file
    fn writeBitcode(self: *Self, output_file: []const u8) !void {
        const bitcode_file = try std.fmt.allocPrint(self.allocator, "{s}.bc", .{output_file});
        defer self.allocator.free(bitcode_file);
        
        var file = try std.fs.cwd().createFile(bitcode_file, .{});
        defer file.close();
        
        try llvm.bitcode_writer.writeBitcode(file.writer(), &self.builder);
        print("🎉 Generated LLVM bitcode: {s}\n", .{bitcode_file});
        print("💡 To execute: lli {s}\n", .{bitcode_file});
    }
};
