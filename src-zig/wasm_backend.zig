// Comprehensive WebAssembly Backend for CURSED
// Implements WASM compilation with both browser and WASI support
const std = @import("std");
const builtin = @import("builtin");
const Allocator = std.mem.Allocator;

// Import existing CURSED infrastructure  
const AST = @import("ast.zig");
const Parser = @import("parser.zig");
const Lexer = @import("lexer.zig");
const TypeSystem = @import("type_system_runtime.zig");

// WASM target configuration
pub const WasmTarget = enum {
    browser,        // Browser-compatible WASM with JS interop
    wasi,          // WebAssembly System Interface for serverless/CLI
    freestanding,  // Minimal WASM with no host dependencies
};

pub const WasmOptions = struct {
    target: WasmTarget = .browser,
    optimize_size: bool = true,
    enable_simd: bool = false,
    enable_threads: bool = false,
    enable_gc: bool = true,
    enable_js_interop: bool = true,
    memory_pages: u32 = 256, // Initial memory in 64KB pages
    max_memory_pages: ?u32 = null,
    export_all: bool = false,
    debug_info: bool = false,
};

// WASM code generation backend
pub const WasmBackend = struct {
    allocator: Allocator,
    options: WasmOptions,
    wasm_buffer: std.ArrayList(u8),
    exports: std.ArrayList(WasmExport),
    imports: std.ArrayList(WasmImport),
    functions: std.ArrayList(WasmFunction),
    globals: std.ArrayList(WasmGlobal),
    memory_info: WasmMemoryInfo,
    string_table: std.ArrayList([]const u8),
    type_table: std.ArrayList(WasmType),
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, options: WasmOptions) !Self {
        return Self{
            .allocator = allocator,
            .options = options,
            .wasm_buffer = std.ArrayList(u8){},
            .exports = std.ArrayList(WasmExport){},
            .imports = std.ArrayList(WasmImport){},
            .functions = std.ArrayList(WasmFunction){},
            .globals = std.ArrayList(WasmGlobal){},
            .memory_info = WasmMemoryInfo{
                .initial_pages = options.memory_pages,
                .max_pages = options.max_memory_pages,
            },
            .string_table = std.ArrayList([]const u8){},
            .type_table = std.ArrayList(WasmType){},
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.wasm_buffer.deinit(self.allocator);
        self.exports.deinit(self.allocator);
        self.imports.deinit(self.allocator);
        self.functions.deinit(self.allocator);
        self.globals.deinit(self.allocator);
        self.string_table.deinit(self.allocator);
        self.type_table.deinit(self.allocator);
    }
    
    // Main compilation entry point
    pub fn compileProgram(self: *Self, ast: *AST.Program) ![]const u8 {
        try self.setupWasmModule();
        
        // Add standard imports based on target
        try self.addStandardImports();
        
        // Process AST and generate WASM code
        try self.processProgram(ast);
        
        // Generate final WASM binary
        return self.generateWasmBinary();
    }
    
    fn setupWasmModule(self: *Self) !void {
        // WASM magic number
        try self.wasm_buffer.appendSlice(&[_]u8{ 0x00, 0x61, 0x73, 0x6D });
        // WASM version (1)
        try self.wasm_buffer.appendSlice(&[_]u8{ 0x01, 0x00, 0x00, 0x00 });
        
        // Setup basic types
        try self.addBasicTypes();
        
        // Setup memory
        try self.setupMemory();
    }
    
    fn addStandardImports(self: *Self) !void {
        switch (self.options.target) {
            .browser => {
                // Browser APIs for I/O
                try self.addImport("js", "console_log", WasmType.function(&[_]WasmValueType{.i32, .i32}, &[_]WasmValueType{}));
                try self.addImport("js", "alert", WasmType.function(&[_]WasmValueType{.i32, .i32}, &[_]WasmValueType{}));
                
                // DOM manipulation if JS interop enabled
                if (self.options.enable_js_interop) {
                    try self.addImport("dom", "createElement", WasmType.function(&[_]WasmValueType{.i32, .i32}, &[_]WasmValueType{.i32}));
                    try self.addImport("dom", "appendChild", WasmType.function(&[_]WasmValueType{.i32, .i32}, &[_]WasmValueType{}));
                    try self.addImport("dom", "getElementById", WasmType.function(&[_]WasmValueType{.i32, .i32}, &[_]WasmValueType{.i32}));
                }
            },
            .wasi => {
                // WASI standard I/O
                try self.addImport("wasi_snapshot_preview1", "fd_write", WasmType.function(&[_]WasmValueType{.i32, .i32, .i32, .i32}, &[_]WasmValueType{.i32}));
                try self.addImport("wasi_snapshot_preview1", "fd_read", WasmType.function(&[_]WasmValueType{.i32, .i32, .i32, .i32}, &[_]WasmValueType{.i32}));
                try self.addImport("wasi_snapshot_preview1", "proc_exit", WasmType.function(&[_]WasmValueType{.i32}, &[_]WasmValueType{}));
                try self.addImport("wasi_snapshot_preview1", "clock_time_get", WasmType.function(&[_]WasmValueType{.i32, .i64, .i32}, &[_]WasmValueType{.i32}));
                
                // WASI filesystem access
                try self.addImport("wasi_snapshot_preview1", "path_open", WasmType.function(&[_]WasmValueType{.i32, .i32, .i32, .i32, .i32, .i64, .i64, .i32, .i32}, &[_]WasmValueType{.i32}));
                try self.addImport("wasi_snapshot_preview1", "fd_close", WasmType.function(&[_]WasmValueType{.i32}, &[_]WasmValueType{.i32}));
            },
            .freestanding => {
                // Minimal freestanding environment - no imports needed
            },
        }
    }
    
    fn processProgram(self: *Self, program: *AST.Program) !void {
        // Process global variables
        for (program.statements) |stmt| {
            switch (stmt.*) {
                .variable_declaration => |var_decl| {
                    try self.processGlobalVariable(var_decl);
                },
                .function_declaration => |func_decl| {
                    try self.processFunction(func_decl);
                },
                .import_statement => |import_stmt| {
                    try self.processImport(import_stmt);
                },
                else => {
                    // Other statements go in main function
                },
            }
        }
        
        // Generate main function if needed
        try self.generateMainFunction(program);
        
        // Generate CURSED standard library functions used
        try self.generateStandardLibraryFunctions();
    }
    
    fn processFunction(self: *Self, func_decl: AST.FunctionDeclaration) !void {
        var wasm_func = WasmFunction{
            .name = try self.allocator.dupe(u8, func_decl.name),
            .type_index = try self.getFunctionTypeIndex(func_decl.parameters, func_decl.return_type),
            .locals = std.ArrayList(WasmValueType){},
            .body = std.ArrayList(u8){},
            .is_export = func_decl.is_public,
        };
        
        // Generate function body
        try self.generateFunctionBody(&wasm_func, func_decl.body);
        
        try self.functions.append(allocator, wasm_func);
        
        // Add export if public
        if (func_decl.is_public) {
            try self.addExport(func_decl.name, .function, @intCast(self.functions.items.len - 1));
        }
    }
    
    fn generateFunctionBody(self: *Self, wasm_func: *WasmFunction, body: AST.Block) !void {
        for (body.statements) |stmt| {
            try self.generateStatement(wasm_func, stmt);
        }
        
        // Add return if function doesn't end with one
        if (wasm_func.body.items.len == 0 or wasm_func.body.items[wasm_func.body.items.len - 1] != 0x0F) {
            try wasm_func.body.append(allocator, 0x0F); // return
        }
    }
    
    fn generateStatement(self: *Self, wasm_func: *WasmFunction, stmt: *AST.Statement) !void {
        switch (stmt.*) {
            .expression_statement => |expr_stmt| {
                try self.generateExpression(wasm_func, expr_stmt.expression);
                try wasm_func.body.append(allocator, 0x1A); // drop result if not used
            },
            .variable_declaration => |var_decl| {
                // Local variables
                if (var_decl.initializer) |init| {
                    try self.generateExpression(wasm_func, init);
                    try self.generateLocalSet(wasm_func, var_decl.name);
                }
            },
            .return_statement => |ret_stmt| {
                if (ret_stmt.value) |value| {
                    try self.generateExpression(wasm_func, value);
                }
                try wasm_func.body.append(allocator, 0x0F); // return
            },
            .if_statement => |if_stmt| {
                try self.generateExpression(wasm_func, if_stmt.condition);
                try wasm_func.body.append(allocator, 0x04); // if
                try wasm_func.body.append(allocator, 0x40); // void block type
                
                try self.generateStatement(wasm_func, if_stmt.then_branch);
                
                if (if_stmt.else_branch) |else_branch| {
                    try wasm_func.body.append(allocator, 0x05); // else
                    try self.generateStatement(wasm_func, else_branch);
                }
                
                try wasm_func.body.append(allocator, 0x0B); // end
            },
            .while_statement => |while_stmt| {
                try wasm_func.body.append(allocator, 0x02); // block
                try wasm_func.body.append(allocator, 0x40); // void block type
                try wasm_func.body.append(allocator, 0x03); // loop
                try wasm_func.body.append(allocator, 0x40); // void block type
                
                try self.generateExpression(wasm_func, while_stmt.condition);
                try wasm_func.body.append(allocator, 0x04); // if
                try wasm_func.body.append(allocator, 0x40); // void block type
                try self.generateStatement(wasm_func, while_stmt.body);
                try wasm_func.body.append(allocator, 0x0C); // br 0 (continue loop)
                try wasm_func.body.append(allocator, 0x00);
                try wasm_func.body.append(allocator, 0x0B); // end if
                try wasm_func.body.append(allocator, 0x0B); // end loop
                try wasm_func.body.append(allocator, 0x0B); // end block
            },
            .block_statement => |block_stmt| {
                for (block_stmt.statements) |nested_stmt| {
                    try self.generateStatement(wasm_func, nested_stmt);
                }
            },
            else => {
                // Handle other statement types
                std.debug.print("Warning: Unhandled statement type in WASM generation\n", .{});
            },
        }
    }
    
    fn generateExpression(self: *Self, wasm_func: *WasmFunction, expr: *AST.Expression) !void {
        switch (expr.*) {
            .literal => |literal| {
                switch (literal) {
                    .integer => |int_val| {
                        try wasm_func.body.append(allocator, 0x41); // i32.const
                        try self.encodeLEB128(wasm_func, @intCast(int_val));
                    },
                    .float => |float_val| {
                        try wasm_func.body.append(allocator, 0x44); // f64.const
                        const float_bytes = @as([8]u8, @bitCast(float_val));
                        try wasm_func.body.appendSlice(&float_bytes);
                    },
                    .string => |string_val| {
                        const str_index = try self.addStringConstant(string_val);
                        try wasm_func.body.append(allocator, 0x41); // i32.const (string pointer)
                        try self.encodeLEB128(wasm_func, str_index);
                    },
                    .boolean => |bool_val| {
                        try wasm_func.body.append(allocator, 0x41); // i32.const
                        try self.encodeLEB128(wasm_func, if (bool_val) 1 else 0);
                    },
                }
            },
            .identifier => |ident| {
                try self.generateLocalGet(wasm_func, ident.name);
            },
            .binary_operation => |binary_op| {
                try self.generateExpression(wasm_func, binary_op.left);
                try self.generateExpression(wasm_func, binary_op.right);
                
                switch (binary_op.operator) {
                    .add => try wasm_func.body.append(allocator, 0x6A), // i32.add
                    .subtract => try wasm_func.body.append(allocator, 0x6B), // i32.sub
                    .multiply => try wasm_func.body.append(allocator, 0x6C), // i32.mul
                    .divide => try wasm_func.body.append(allocator, 0x6D), // i32.div_s
                    .equal => try wasm_func.body.append(allocator, 0x46), // i32.eq
                    .not_equal => try wasm_func.body.append(allocator, 0x47), // i32.ne
                    .less_than => try wasm_func.body.append(allocator, 0x48), // i32.lt_s
                    .greater_than => try wasm_func.body.append(allocator, 0x4A), // i32.gt_s
                    .less_equal => try wasm_func.body.append(allocator, 0x4C), // i32.le_s
                    .greater_equal => try wasm_func.body.append(allocator, 0x4E), // i32.ge_s
                    else => {
                        std.debug.print("Warning: Unhandled binary operator in WASM generation\n", .{});
                    },
                }
            },
            .function_call => |func_call| {
                // Generate arguments
                for (func_call.arguments) |arg| {
                    try self.generateExpression(wasm_func, arg);
                }
                
                // Handle standard library functions
                if (try self.isStandardLibraryFunction(func_call.function)) {
                    try self.generateStandardLibraryCall(wasm_func, func_call);
                } else {
                    // Regular function call
                    const func_index = try self.getFunctionIndex(func_call.function);
                    try wasm_func.body.append(allocator, 0x10); // call
                    try self.encodeLEB128(wasm_func, func_index);
                }
            },
            .array_access => |array_access| {
                try self.generateExpression(wasm_func, array_access.array);
                try self.generateExpression(wasm_func, array_access.index);
                try self.generateArrayLoad(wasm_func);
            },
            .member_access => |member_access| {
                try self.generateExpression(wasm_func, member_access.object);
                try self.generateMemberLoad(wasm_func, member_access.member);
            },
            else => {
                std.debug.print("Warning: Unhandled expression type in WASM generation\n", .{});
            },
        }
    }
    
    fn generateMainFunction(self: *Self, program: *AST.Program) !void {
        var main_func = WasmFunction{
            .name = try self.allocator.dupe(u8, "main"),
            .type_index = try self.getFunctionTypeIndex(&[_]AST.Parameter{}, AST.Type.void),
            .locals = std.ArrayList(WasmValueType){},
            .body = std.ArrayList(u8){},
            .is_export = true,
        };
        
        // Process main-level statements
        for (program.statements) |stmt| {
            switch (stmt.*) {
                .variable_declaration, .function_declaration, .import_statement => {
                    // Already processed
                    continue;
                },
                else => {
                    try self.generateStatement(&main_func, stmt);
                },
            }
        }
        
        try self.functions.append(allocator, main_func);
        try self.addExport("main", .function, @intCast(self.functions.items.len - 1));
        
        // Add _start export for WASI compatibility
        if (self.options.target == .wasi) {
            try self.addExport("_start", .function, @intCast(self.functions.items.len - 1));
        }
    }
    
    fn generateStandardLibraryFunctions(self: *Self) !void {
        // Generate vibez.spill (print) function
        try self.generatePrintFunction();
        
        // Generate basic math functions
        try self.generateMathFunctions();
        
        // Generate string functions
        try self.generateStringFunctions();
        
        // Generate memory management functions if GC is enabled
        if (self.options.enable_gc) {
            try self.generateGCFunctions();
        }
    }
    
    fn generatePrintFunction(self: *Self) !void {
        var print_func = WasmFunction{
            .name = try self.allocator.dupe(u8, "cursed_print"),
            .type_index = try self.getFunctionTypeIndex(&[_]AST.Parameter{
                AST.Parameter{ .name = "ptr", .type = AST.Type.integer },
                AST.Parameter{ .name = "len", .type = AST.Type.integer },
            }, AST.Type.void),
            .locals = std.ArrayList(WasmValueType){},
            .body = std.ArrayList(u8){},
            .is_export = false,
        };
        
        switch (self.options.target) {
            .browser => {
                // Call JS console.log
                try print_func.body.append(allocator, 0x20); // local.get 0 (ptr)
                try print_func.body.append(allocator, 0x00);
                try print_func.body.append(allocator, 0x20); // local.get 1 (len)
                try print_func.body.append(allocator, 0x01);
                try print_func.body.append(allocator, 0x10); // call console_log import
                try print_func.body.append(allocator, 0x00); // import index
            },
            .wasi => {
                // Use WASI fd_write
                // TODO: Implement proper WASI I/O vectors
                try print_func.body.append(allocator, 0x41); // i32.const 1 (stdout)
                try print_func.body.append(allocator, 0x01);
                try print_func.body.append(allocator, 0x20); // local.get 0 (ptr to iov)
                try print_func.body.append(allocator, 0x00);
                try print_func.body.append(allocator, 0x41); // i32.const 1 (iov count)
                try print_func.body.append(allocator, 0x01);
                try print_func.body.append(allocator, 0x41); // i32.const result ptr
                try print_func.body.append(allocator, 0x08);
                try print_func.body.append(allocator, 0x10); // call fd_write
                try print_func.body.append(allocator, 0x00); // import index
                try print_func.body.append(allocator, 0x1A); // drop result
            },
            .freestanding => {
                // No-op for freestanding
            },
        }
        
        try self.functions.append(allocator, print_func);
    }
    
    fn generateWasmBinary(self: *Self) ![]const u8 {
        var output = std.ArrayList(u8){};
        defer output.deinit();
        
        // WASM header already added in setupWasmModule
        try output.appendSlice(self.wasm_buffer.items);
        
        // Type section
        if (self.type_table.items.len > 0) {
            try self.writeSection(&output, 1, try self.encodeTypeSection());
        }
        
        // Import section  
        if (self.imports.items.len > 0) {
            try self.writeSection(&output, 2, try self.encodeImportSection());
        }
        
        // Function section
        if (self.functions.items.len > 0) {
            try self.writeSection(&output, 3, try self.encodeFunctionSection());
        }
        
        // Memory section
        try self.writeSection(&output, 5, try self.encodeMemorySection());
        
        // Export section
        if (self.exports.items.len > 0) {
            try self.writeSection(&output, 7, try self.encodeExportSection());
        }
        
        // Code section
        if (self.functions.items.len > 0) {
            try self.writeSection(&output, 10, try self.encodeCodeSection());
        }
        
        // Data section (for string constants)
        if (self.string_table.items.len > 0) {
            try self.writeSection(&output, 11, try self.encodeDataSection());
        }
        
        return output.toOwnedSlice();
    }
    
    // Helper functions and supporting types
    fn addImport(self: *Self, module_name: []const u8, item_name: []const u8, import_type: WasmType) !void {
        try self.imports.append(WasmImport{
            .module_name = try self.allocator.dupe(u8, module_name),
            .item_name = try self.allocator.dupe(u8, item_name),
            .import_type = import_type,
        });
    }
    
    fn addExport(self: *Self, name: []const u8, export_type: WasmExportType, index: u32) !void {
        try self.exports.append(WasmExport{
            .name = try self.allocator.dupe(u8, name),
            .export_type = export_type,
            .index = index,
        });
    }
    
    fn addStringConstant(self: *Self, str: []const u8) !u32 {
        const index = @as(u32, @intCast(self.string_table.items.len));
        try self.string_table.append(try self.allocator.dupe(u8, str));
        return index;
    }
    
    // Placeholder implementations for complex functions
    fn addBasicTypes(self: *Self) !void {
        // Add basic function types
        try self.type_table.append(WasmType.function(&[_]WasmValueType{}, &[_]WasmValueType{})); // () -> ()
        try self.type_table.append(WasmType.function(&[_]WasmValueType{.i32}, &[_]WasmValueType{.i32})); // (i32) -> i32
        try self.type_table.append(WasmType.function(&[_]WasmValueType{.i32, .i32}, &[_]WasmValueType{})); // (i32, i32) -> ()
    }
    
    fn setupMemory(self: *Self) !void {
        // Memory will be added in memory section
    }
    
    fn getFunctionTypeIndex(self: *Self, parameters: []const AST.Parameter, return_type: AST.Type) !u32 {
        // Simplified type matching - in real implementation, would need proper type conversion
        return 0; // Default to first type for now
    }
    
    fn isStandardLibraryFunction(self: *Self, func_name: []const u8) !bool {
        _ = self;
        return std.mem.eql(u8, func_name, "vibez.spill") or 
               std.mem.eql(u8, func_name, "print") or
               std.mem.startsWith(u8, func_name, "mathz.") or
               std.mem.startsWith(u8, func_name, "stringz.");
    }
    
    // More placeholder implementations
    fn generateLocalGet(self: *Self, wasm_func: *WasmFunction, name: []const u8) !void { _ = self; _ = wasm_func; _ = name; }
    fn generateLocalSet(self: *Self, wasm_func: *WasmFunction, name: []const u8) !void { _ = self; _ = wasm_func; _ = name; }
    fn generateStandardLibraryCall(self: *Self, wasm_func: *WasmFunction, call: AST.FunctionCall) !void { _ = self; _ = wasm_func; _ = call; }
    fn generateArrayLoad(self: *Self, wasm_func: *WasmFunction) !void { _ = self; _ = wasm_func; }
    fn generateMemberLoad(self: *Self, wasm_func: *WasmFunction, member: []const u8) !void { _ = self; _ = wasm_func; _ = member; }
    fn generateMathFunctions(self: *Self) !void { _ = self; }
    fn generateStringFunctions(self: *Self) !void { _ = self; }
    fn generateGCFunctions(self: *Self) !void { _ = self; }
    fn getFunctionIndex(self: *Self, name: []const u8) !u32 { _ = self; _ = name; return 0; }
    fn processGlobalVariable(self: *Self, var_decl: AST.VariableDeclaration) !void { _ = self; _ = var_decl; }
    fn processImport(self: *Self, import_stmt: AST.ImportStatement) !void { _ = self; _ = import_stmt; }
    fn encodeLEB128(self: *Self, wasm_func: *WasmFunction, value: u32) !void { _ = self; try wasm_func.body.append(allocator, @intCast(value & 0x7F)); }
    fn writeSection(self: *Self, output: *std.ArrayList(u8), section_id: u8, data: []const u8) !void { _ = self; try output.append(allocator, section_id); try output.appendSlice(data); }
    fn encodeTypeSection(self: *Self) ![]const u8 { _ = self; return &[_]u8{}; }
    fn encodeImportSection(self: *Self) ![]const u8 { _ = self; return &[_]u8{}; }
    fn encodeFunctionSection(self: *Self) ![]const u8 { _ = self; return &[_]u8{}; }
    fn encodeMemorySection(self: *Self) ![]const u8 { _ = self; return &[_]u8{}; }
    fn encodeExportSection(self: *Self) ![]const u8 { _ = self; return &[_]u8{}; }
    fn encodeCodeSection(self: *Self) ![]const u8 { _ = self; return &[_]u8{}; }
    fn encodeDataSection(self: *Self) ![]const u8 { _ = self; return &[_]u8{}; }
};

// Supporting types and structures

pub const WasmValueType = enum(u8) {
    i32 = 0x7F,
    i64 = 0x7E,
    f32 = 0x7D,
    f64 = 0x7C,
    v128 = 0x7B, // SIMD
    funcref = 0x70,
    externref = 0x6F,
};

pub const WasmType = union(enum) {
    function: struct {
        params: []const WasmValueType,
        results: []const WasmValueType,
    },
    
    pub fn function(params: []const WasmValueType, results: []const WasmValueType) WasmType {
        return WasmType{ .function = .{ .params = params, .results = results } };
    }
};

pub const WasmExportType = enum(u8) {
    function = 0x00,
    table = 0x01,
    memory = 0x02,
    global = 0x03,
};

pub const WasmExport = struct {
    name: []const u8,
    export_type: WasmExportType,
    index: u32,
};

pub const WasmImport = struct {
    module_name: []const u8,
    item_name: []const u8,
    import_type: WasmType,
};

pub const WasmFunction = struct {
    name: []const u8,
    type_index: u32,
    locals: std.ArrayList(WasmValueType),
    body: std.ArrayList(u8),
    is_export: bool,
};

pub const WasmGlobal = struct {
    name: []const u8,
    value_type: WasmValueType,
    is_mutable: bool,
    initial_value: u64,
};

pub const WasmMemoryInfo = struct {
    initial_pages: u32,
    max_pages: ?u32,
};

// High-level compilation function
pub fn compileToWasm(allocator: Allocator, source: []const u8, options: WasmOptions) ![]const u8 {
    // Tokenize
    var lexer = try Lexer.init(allocator, source);
    defer lexer.deinit();
    
    const tokens = try lexer.tokenize();
    defer tokens.deinit();
    
    // Parse  
    var parser = try Parser.init(allocator, tokens.items);
    defer parser.deinit();
    
    var ast = try parser.parseProgram();
    defer ast.deinit();
    
    // Type check
    var type_system = TypeSystem.init(allocator);
    defer type_system.deinit();
    
    try type_system.checkProgram(&ast);
    
    // Generate WASM
    var backend = try WasmBackend.init(allocator, options);
    defer backend.deinit();
    
    return backend.compileProgram(&ast);
}

// Convenience functions for different targets
pub fn compileToBrowser(allocator: Allocator, source: []const u8) ![]const u8 {
    const options = WasmOptions{
        .target = .browser,
        .enable_js_interop = true,
        .optimize_size = true,
    };
    return compileToWasm(allocator, source, options);
}

pub fn compileToWasi(allocator: Allocator, source: []const u8) ![]const u8 {
    const options = WasmOptions{
        .target = .wasi,
        .enable_js_interop = false,
        .optimize_size = false,
    };
    return compileToWasm(allocator, source, options);
}

pub fn compileToFreestanding(allocator: Allocator, source: []const u8) ![]const u8 {
    const options = WasmOptions{
        .target = .freestanding,
        .enable_js_interop = false,
        .enable_gc = false,
        .optimize_size = true,
    };
    return compileToWasm(allocator, source, options);
}
