// WASM-compatible CURSED compiler implementation
// Excludes Thread and filesystem dependencies that aren't supported in WASM

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

// Import only WASM-compatible modules
const lexer = @import("lexer.zig");
const ast = @import("ast.zig");
const parser = @import("parser.zig");

// Minimal Variable type for WASM
const Variable = union(enum) {
    String: []const u8,
    Integer: i64,
    Float: f64,
    Boolean: bool,
    Null,
    
    pub fn deinit(self: *Variable, allocator: Allocator) void {
        _ = allocator;
        switch (self.*) {
            .String => |str| allocator.free(str),
            else => {},
        }
    }
    
    pub fn clone(self: Variable, allocator: Allocator) !Variable {
        _ = allocator;
        return switch (self) {
            .String => |str| Variable{ .String = try allocator.dupe(u8, str) },
            .Integer => |val| Variable{ .Integer = val },
            .Float => |val| Variable{ .Float = val },
            .Boolean => |val| Variable{ .Boolean = val },
            .Null => Variable.Null,
        };
    }
};

// WASM-compatible stores (no Thread dependencies)
const VariableStore = HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);
const FunctionStore = HashMap([]const u8, ast.FunctionStatement, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);

// WASM-compatible runtime environment
const WasmRuntime = struct {
    allocator: Allocator,
    variables: VariableStore,
    functions: FunctionStore,
    output_buffer: ArrayList(u8),
    
    const Self = @This();
    
    pub fn init() Self {
        return Self{
            .allocator = allocator,
            .variables = VariableStore.init(allocator),
            .functions = FunctionStore.init(allocator),
            .output_buffer = .empty,
        };
    }
    
    pub fn deinit(self: *Self) void {
        // Clean up variables
        var var_iter = self.variables.iterator();
        while (var_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.variables.deinit(self.allocator);
        
        // Clean up functions
        var func_iter = self.functions.iterator();
        while (func_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            // Functions are managed by AST cleanup
        }
        self.functions.deinit(self.allocator);
        
        self.output_buffer.deinit(self.allocator);
    }
    
    // WASM print implementation - writes to buffer instead of stdout
    pub fn wasmPrint(self: *Self, text: []const u8) !void {
        try self.output_buffer.appendSlice(text);
        try self.output_buffer.append(allocator, '\n');
    }
    
    // Get output buffer contents
    pub fn getOutput(self: *Self) []const u8 {
        return self.output_buffer.items;
    }
    
    // Clear output buffer
    pub fn clearOutput(self: *Self) void {
        self.output_buffer.clearRetainingCapacity();
    }
};

// WASM-compatible interpreter (no filesystem or concurrency)
const WasmInterpreter = struct {
    runtime: *WasmRuntime,
    
    const Self = @This();
    
    pub fn init(runtime: *WasmRuntime) Self {
        return Self{ .runtime = runtime };
    }
    
    pub fn execute(self: *Self, ast_nodes: []ast.Statement) !void {
        for (ast_nodes) |statement| {
            try self.executeStatement(statement);
        }
    }
    
    fn executeStatement(self: *Self, statement: ast.Statement) !void {
        switch (statement) {
            .Let => |let_stmt| {
                const value = try self.evaluateExpression(let_stmt.value);
                const name = try self.runtime.allocator.dupe(u8, let_stmt.name);
                try self.runtime.variables.put(name, value);
            },
            .Expression => |expr| {
                _ = try self.evaluateExpression(expr);
            },
            else => {
                // Other statement types - simplified for WASM
            },
        }
    }
    
    fn evaluateExpression(self: *Self, expr: ast.Expression) !Variable {
        return switch (expr) {
            .String => |str| Variable{ .String = try self.runtime.allocator.dupe(u8, str) },
            .Integer => |val| Variable{ .Integer = val },
            .Float => |val| Variable{ .Float = val },
            .Boolean => |val| Variable{ .Boolean = val },
            .Identifier => |name| {
                if (self.runtime.variables.get(name)) |variable| {
                    return try variable.clone(self.runtime.allocator);
                } else {
                    return error.UndefinedVariable;
                }
            },
            .Call => |call| {
                // Handle function calls like vibez.spill
                const func_expr = call.function.*;
                if (func_expr == .Identifier) {
                    const func_name = func_expr.Identifier;
                    if (std.mem.eql(u8, func_name, "vibez.spill")) {
                        // WASM print implementation
                        if (call.arguments.items.len > 0) {
                            const arg_expr = call.arguments.items[0].*;
                            const value = try self.evaluateExpression(arg_expr);
                            const text = switch (value) {
                                .String => |str| str,
                                .Integer => |val| try std.fmt.allocPrint(self.runtime.allocator, "{d}", .{val}),
                                .Float => |val| try std.fmt.allocPrint(self.runtime.allocator, "{d}", .{val}),
                                .Boolean => |val| if (val) "true" else "false",
                                .Null => "null",
                            };
                            try self.runtime.wasmPrint(text);
                            if (value != .String) {
                                self.runtime.allocator.free(text);
                            }
                        }
                        return Variable.Null;
                    }
                }
                return Variable.Null;
            },
            else => Variable.Null,
        };
    }
};

// WASM exports for JavaScript integration
var global_runtime: ?WasmRuntime = null;
var global_arena: ?std.heap.ArenaAllocator = null;

export fn wasm_init() i32 {
    const allocator = std.heap.page_allocator;
    
    var arena = std.heap.ArenaAllocator.init(allocator);
    global_arena = arena;
    
    const runtime = WasmRuntime.init(arena.allocator());
    global_runtime = runtime;
    
    return 0; // Success
}

export fn wasm_deinit() void {
    if (global_runtime) |*runtime| {
        runtime.deinit();
        global_runtime = null;
    }
    if (global_arena) |*arena| {
        arena.deinit();
        global_arena = null;
    }
}

export fn wasm_execute_source(source_ptr: [*]const u8, source_len: usize) i32 {
    if (global_runtime == null or global_arena == null) {
        return -1; // Not initialized
    }
    
    const runtime = &global_runtime.?;
    const allocator = global_arena.?.allocator();
    const source = source_ptr[0..source_len];
    
    // Parse source code
    var token_lexer = lexer.Lexer.init(allocator, source);
    defer token_lexer.deinit();
    
    const tokens = token_lexer.tokenize() catch return -3;
    defer allocator.free(tokens);
    
    var ast_parser = parser.Parser.init(allocator, tokens);
    defer ast_parser.deinit();
    
    const ast_nodes = ast_parser.parseProgram() catch return -5;
    defer allocator.free(ast_nodes);
    
    // Execute AST
    var interpreter = WasmInterpreter.init(runtime);
    interpreter.execute(ast_nodes) catch return -6;
    
    return 0; // Success
}

export fn wasm_get_output(buffer_ptr: [*]u8, buffer_len: usize) i32 {
    if (global_runtime == null) {
        return -1;
    }
    
    const runtime = &global_runtime.?;
    const output = runtime.getOutput();
    
    if (output.len > buffer_len) {
        return @as(i32, @intCast(output.len)); // Return required size
    }
    
    @memcpy(buffer_ptr[0..output.len], output);
    return @as(i32, @intCast(output.len));
}

export fn wasm_clear_output() void {
    if (global_runtime) |*runtime| {
        runtime.clearOutput();
    }
}

export fn wasm_tokenize(source_ptr: [*]const u8, source_len: usize) i32 {
    if (global_arena == null) {
        return -1;
    }
    
    const allocator = global_arena.?.allocator();
    const source = source_ptr[0..source_len];
    
    var token_lexer = lexer.Lexer.init(allocator, source) catch return -2;
    defer token_lexer.deinit();
    
    const tokens = token_lexer.tokenize() catch return -3;
    defer allocator.free(tokens);
    
    return @as(i32, @intCast(tokens.len));
}

export fn wasm_check_syntax(source_ptr: [*]const u8, source_len: usize) i32 {
    if (global_arena == null) {
        return -1;
    }
    
    const allocator = global_arena.?.allocator();
    const source = source_ptr[0..source_len];
    
    // Tokenize
    var token_lexer = lexer.Lexer.init(allocator, source);
    defer token_lexer.deinit();
    
    const tokens = token_lexer.tokenize() catch return -3;
    defer allocator.free(tokens);
    
    // Parse
    var ast_parser = parser.Parser.init(allocator, tokens);
    defer ast_parser.deinit();
    
    const ast_nodes = ast_parser.parseProgram() catch return -5;
    defer allocator.free(ast_nodes);
    
    return 0; // Success - syntax is valid
}

// Version info
export fn wasm_version() [*:0]const u8 {
    return "CURSED v1.0.0-wasm-compatible";
}

// WASM memory management
export fn wasm_alloc(size: usize) ?[*]u8 {
    const allocator = std.heap.page_allocator;
    const memory = allocator.alloc(u8, size) catch return null;
    return memory.ptr;
}

export fn wasm_free(ptr: [*]u8, size: usize) void {
    const allocator = std.heap.page_allocator;
    const memory = ptr[0..size];
    allocator.free(memory);
}

// Main function for executable compatibility
pub fn main() !void {
    // Initialize for testing
    _ = wasm_init();
    defer wasm_deinit();
    
    // Test basic functionality
    const test_source = "vibez.spill(\"Hello from WASM CURSED!\")";
    const result = wasm_execute_source(test_source.ptr, test_source.len);
    
    if (result == 0) {
        print("WASM CURSED test: SUCCESS\n", .{});
        
        // Get and print output
        var output_buffer: [1024]u8 = undefined;
        const output_len = wasm_get_output(output_buffer.ptr, output_buffer.len);
        if (output_len > 0) {
            print("Output: {s}\n", .{output_buffer[0..@as(usize, @intCast(output_len))]});
        }
    } else {
        print("WASM CURSED test: FAILED with code {d}\n", .{result});
    }
}
