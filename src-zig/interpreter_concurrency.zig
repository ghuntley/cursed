//! CURSED Interpreter Concurrency Integration
//!
//! This module provides interpreter support for CURSED concurrency features:
//! - stan keyword for goroutine spawning
//! - dm<T> type for channels
//! - ready keyword for select statements
//! - Channel send/receive operations
//!
//! Integrates with the concurrency runtime for execution.

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const ast = @import("ast_simple.zig");
const concurrency_runtime = @import("concurrency_runtime.zig");
const concurrency = @import("concurrency.zig");

/// Interpreter value extended with concurrency types
pub const InterpreterValue = union(enum) {
    integer: i64,
    string: []const u8,
    boolean: bool,
    array: ArrayList(InterpreterValue),
    function: *ast.FunctionLiteral,
    
    // Concurrency types
    goroutine_id: concurrency.GoroutineId,
    channel_id: concurrency.ChannelId,
    channel_value: concurrency_runtime.ConcurrencyValue,
    select_result: concurrency.SelectResult,
    
    null_value: void,

    pub fn deinit(self: *InterpreterValue, allocator: Allocator) void {
        _ = allocator;
        switch (self.*) {
            .string => |s| allocator.free(s),
            .array => |*arr| {
                for (arr.items) |*item| {
                    item.deinit();
                }
                arr.deinit();
            },
            .channel_value => |*val| val.deinit(),
            else => {},
        }
    }

    pub fn toString(self: InterpreterValue, allocator: Allocator) ![]const u8 {
        _ = allocator;
        return switch (self) {
            .integer => |i| try std.fmt.allocPrint(allocator, "{}", .{i}),
            .string => |s| try allocator.dupe(u8, s),
            .boolean => |b| try allocator.dupe(u8, if (b) "based" else "cringe"),
            .goroutine_id => |id| try std.fmt.allocPrint(allocator, "goroutine#{}", .{id}),
            .channel_id => |id| try std.fmt.allocPrint(allocator, "channel#{}", .{id}),
            .select_result => |result| try std.fmt.allocPrint(allocator, "select:{}", .{result}),
            .null_value => try allocator.dupe(u8, "null"),
            else => try allocator.dupe(u8, "unknown"),
        };
    }
};

/// Environment for storing variables and channels
pub const Environment = struct {
    values: std.HashMap([]const u8, InterpreterValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    parent: ?*Environment,
    allocator: Allocator,

    pub fn init(allocator: Allocator, parent: ?*Environment) Environment {
        return Environment{
            .values = std.HashMap([]const u8, InterpreterValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .parent = parent,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Environment) void {
        var iter = self.values.iterator();
        while (iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.values.deinit(self.allocator);
    }

    pub fn define(self: *Environment, name: []const u8, value: InterpreterValue) !void {
        const key = try self.allocator.dupe(u8, name);
        try self.values.put(key, value);
    }

    pub fn get(self: *Environment, name: []const u8) ?InterpreterValue {
        if (self.values.get(name)) |value| {
            return value;
        }
        if (self.parent) |parent| {
            return parent.get(name);
        }
        return null;
    }

    pub fn set(self: *Environment, name: []const u8, value: InterpreterValue) !void {
        if (self.values.contains(name)) {
            try self.values.put(name, value);
            return;
        }
        if (self.parent) |parent| {
            try parent.set(name, value);
            return;
        }
        return error.UndefinedVariable;
    }
};

/// Concurrency-aware interpreter
pub const ConcurrencyInterpreter = struct {
    allocator: Allocator,
    environment: *Environment,
    runtime_initialized: bool,

    pub fn init(allocator: Allocator) !ConcurrencyInterpreter {
        _ = allocator;
        // Initialize concurrency runtime
        try concurrency_runtime.initializeRuntime(allocator);

        const env = try allocator.create(Environment);
        env.* = Environment.init(allocator, null);

        return ConcurrencyInterpreter{
            .allocator = allocator,
            .environment = env,
            .runtime_initialized = true,
        };
    }

    pub fn deinit(self: *ConcurrencyInterpreter) void {
        self.environment.deinit(self.allocator);
        self.allocator.destroy(self.environment);
        
        if (self.runtime_initialized) {
            concurrency_runtime.shutdownRuntime(self.allocator);
        }
    }

    /// Evaluate AST node with concurrency support
    pub fn eval(self: *ConcurrencyInterpreter, node: ast.Node) !InterpreterValue {
        return switch (node) {
            .statement => |stmt| try self.evalStatement(stmt),
            .expression => |expr| try self.evalExpression(expr),
        };
    }

    fn evalStatement(self: *ConcurrencyInterpreter, stmt: ast.Statement) !InterpreterValue {
        return switch (stmt) {
            .expression_statement => |expr_stmt| try self.evalExpression(expr_stmt.expression),
            .let_statement => |let_stmt| {
                const value = try self.evalExpression(let_stmt.value);
                try self.environment.define(let_stmt.name.value, value);
                return InterpreterValue{ .null_value = {} };
            },
            .return_statement => |ret_stmt| {
                if (ret_stmt.return_value) |ret_val| {
                    return try self.evalExpression(ret_val);
                }
                return InterpreterValue{ .null_value = {} };
            },
            .block_statement => |block_stmt| {
                var result = InterpreterValue{ .null_value = {} };
                for (block_stmt.statements.items) |block_stmt_item| {
                    result = try self.evalStatement(block_stmt_item);
                }
                return result;
            },
            else => InterpreterValue{ .null_value = {} },
        };
    }

    fn evalExpression(self: *ConcurrencyInterpreter, expr: ast.Expression) !InterpreterValue {
        return switch (expr) {
            .integer_literal => |int_lit| InterpreterValue{ .integer = int_lit.value },
            .string_literal => |str_lit| InterpreterValue{ .string = try self.allocator.dupe(u8, str_lit.value) },
            .boolean_literal => |bool_lit| InterpreterValue{ .boolean = bool_lit.value },
            .identifier => |ident| self.environment.get(ident.value) orelse InterpreterValue{ .null_value = {} },
            .function_literal => |func_lit| InterpreterValue{ .function = func_lit },
            
            // Concurrency expressions
            .call_expression => |call_expr| try self.evalCallExpression(call_expr),
            .channel_literal => |chan_lit| try self.evalChannelLiteral(chan_lit),
            .goroutine_spawn => |spawn_expr| try self.evalGoroutineSpawn(spawn_expr),
            .channel_send => |send_expr| try self.evalChannelSend(send_expr),
            .channel_receive => |recv_expr| try self.evalChannelReceive(recv_expr),
            .select_expression => |select_expr| try self.evalSelectExpression(select_expr),
            
            else => InterpreterValue{ .null_value = {} },
        };
    }

    /// Evaluate function call (including built-in concurrency functions)
    fn evalCallExpression(self: *ConcurrencyInterpreter, call_expr: *ast.CallExpression) !InterpreterValue {
        const function = try self.evalExpression(call_expr.function.*);
        
        // Check for built-in concurrency functions
        if (function == .identifier) {
            // This would be handled by identifier resolution
        }

        if (function == .function) {
            // Execute user-defined function
            var args = std.ArrayList(u8){};
            defer args.deinit();

            for (call_expr.arguments.items) |arg| {
                const arg_value = try self.evalExpression(arg);
                try args.append(allocator, arg_value);
            }

            return try self.executeFunction(function.function, args.items);
        }

        return InterpreterValue{ .null_value = {} };
    }

    /// Evaluate channel literal (dm<type> with capacity)
    fn evalChannelLiteral(self: *ConcurrencyInterpreter, chan_lit: *ast.ChannelLiteral) !InterpreterValue {
        const capacity = if (chan_lit.capacity) |cap_expr| blk: {
            const cap_value = try self.evalExpression(cap_expr.*);
            if (cap_value == .integer) {
                break :blk @as(usize, @intCast(cap_value.integer));
            }
            break :blk @as(usize, 0);
        } else 0;

        const channel_type = switch (chan_lit.element_type) {
            .normie => concurrency_runtime.ChannelType.integer,
            .tea => concurrency_runtime.ChannelType.string,
            .lit => concurrency_runtime.ChannelType.boolean,
            else => concurrency_runtime.ChannelType.integer,
        };

        const channel_id = try concurrency_runtime.executeDmCreate(channel_type, capacity);
        return InterpreterValue{ .channel_id = channel_id };
    }

    /// Evaluate goroutine spawn (stan keyword)
    fn evalGoroutineSpawn(self: *ConcurrencyInterpreter, spawn_expr: *ast.GoroutineSpawn) !InterpreterValue {
        const function = try self.evalExpression(spawn_expr.function.*);
        
        if (function == .function) {
            const goroutine_id = try concurrency_runtime.executeStan(function.function, null);
            return InterpreterValue{ .goroutine_id = goroutine_id };
        }

        return InterpreterValue{ .null_value = {} };
    }

    /// Evaluate channel send operation
    fn evalChannelSend(self: *ConcurrencyInterpreter, send_expr: *ast.ChannelSend) !InterpreterValue {
        const channel_val = try self.evalExpression(send_expr.channel.*);
        const value_val = try self.evalExpression(send_expr.value.*);

        if (channel_val == .channel_id) {
            const concurrency_value = switch (value_val) {
                .integer => |i| concurrency_runtime.ConcurrencyValue{ .goroutine_id = @intCast(i) },
                .goroutine_id => |id| concurrency_runtime.ConcurrencyValue{ .goroutine_id = id },
                else => return InterpreterValue{ .null_value = {} },
            };

            const result = try concurrency_runtime.executeDmSend(channel_val.channel_id, concurrency_value);
            return InterpreterValue{ .boolean = result == .sent };
        }

        return InterpreterValue{ .null_value = {} };
    }

    /// Evaluate channel receive operation
    fn evalChannelReceive(self: *ConcurrencyInterpreter, recv_expr: *ast.ChannelReceive) !InterpreterValue {
        const channel_val = try self.evalExpression(recv_expr.channel.*);

        if (channel_val == .channel_id) {
            const result = try concurrency_runtime.executeDmReceive(channel_val.channel_id);
            if (result) |value| {
                return switch (value) {
                    .goroutine_id => |id| InterpreterValue{ .goroutine_id = id },
                    .void_value => InterpreterValue{ .null_value = {} },
                    else => InterpreterValue{ .channel_value = value },
                };
            }
        }

        return InterpreterValue{ .null_value = {} };
    }

    /// Evaluate select expression (ready keyword)
    fn evalSelectExpression(self: *ConcurrencyInterpreter, select_expr: *ast.SelectExpression) !InterpreterValue {
        var operations = std.ArrayList(u8){};
        defer operations.deinit();

        for (select_expr.cases.items) |case_item| {
            switch (case_item.operation) {
                .send => |send_op| {
                    const channel_val = try self.evalExpression(send_op.channel.*);
                    if (channel_val == .channel_id) {
                        const value_val = try self.evalExpression(send_op.value.*);
                        const concurrency_value = switch (value_val) {
                            .integer => |i| concurrency_runtime.ConcurrencyValue{ .goroutine_id = @intCast(i) },
                            else => concurrency_runtime.ConcurrencyValue{ .void_value = {} },
                        };

                        try operations.append(concurrency_runtime.SelectOperation{
                            .type = .send,
                            .channel_id = channel_val.channel_id,
                            .value = concurrency_value,
                        });
                    }
                },
                .receive => |recv_op| {
                    const channel_val = try self.evalExpression(recv_op.channel.*);
                    if (channel_val == .channel_id) {
                        try operations.append(concurrency_runtime.SelectOperation{
                            .type = .receive,
                            .channel_id = channel_val.channel_id,
                            .value = null,
                        });
                    }
                },
                .default => {
                    try operations.append(concurrency_runtime.SelectOperation{
                        .type = .default,
                        .channel_id = 0, // Not used for default
                        .value = null,
                    });
                },
            }
        }

        const result = try concurrency_runtime.executeReady(operations.items);
        return InterpreterValue{ .select_result = result };
    }

    /// Execute function with arguments
    fn executeFunction(self: *ConcurrencyInterpreter, function: *ast.FunctionLiteral, args: []const InterpreterValue) !InterpreterValue {
        // Create new environment for function execution
        var func_env = Environment.init(self.allocator, self.environment);
        defer func_env.deinit();

        // Bind parameters to arguments
        for (function.parameters.items, 0..) |param, i| {
            if (i < args.len) {
                try func_env.define(param.value, args[i]);
            }
        }

        // Execute function body
        const old_env = self.environment;
        self.environment = &func_env;
        defer self.environment = old_env;

        return try self.evalStatement(ast.Statement{ .block_statement = function.body });
    }

    /// Yield current goroutine
    pub fn yieldGoroutine(self: *ConcurrencyInterpreter) !void {
        _ = self;
        try concurrency_runtime.executeYolo();
    }

    /// Get runtime statistics
    pub fn getRuntimeStats(self: *ConcurrencyInterpreter) concurrency_runtime.RuntimeStats {
        _ = self;
        if (concurrency_runtime.getRuntime()) |runtime| {
            return runtime.getStats();
        }
        return concurrency_runtime.RuntimeStats.init();
    }
};

// Tests
test "concurrency interpreter initialization" {
    const allocator = std.testing.allocator;
    
    var interpreter = try ConcurrencyInterpreter.init(allocator);
    defer interpreter.deinit();

    try std.testing.expect(interpreter.runtime_initialized);
}

test "channel creation in interpreter" {
    const allocator = std.testing.allocator;
    
    var interpreter = try ConcurrencyInterpreter.init(allocator);
    defer interpreter.deinit();

    // Create mock channel literal
    var channel_literal = ast.ChannelLiteral{
        .element_type = .normie,
        .capacity = null,
    };

    const result = try interpreter.evalChannelLiteral(&channel_literal);
    try std.testing.expect(result == .channel_id);
    try std.testing.expect(result.channel_id > 0);
}

test "goroutine spawn in interpreter" {
    const allocator = std.testing.allocator;
    
    var interpreter = try ConcurrencyInterpreter.init(allocator);
    defer interpreter.deinit();

    // Create mock function literal
    var function_literal = ast.FunctionLiteral{
        .parameters = .empty,
        .body = ast.BlockStatement{ .statements = .empty },
    };
    defer function_literal.parameters.deinit();
    defer function_literal.body.statements.deinit();

    var spawn_expr = ast.GoroutineSpawn{
        .function = &ast.Expression{ .function_literal = &function_literal },
    };

    const result = try interpreter.evalGoroutineSpawn(&spawn_expr);
    try std.testing.expect(result == .goroutine_id);
    try std.testing.expect(result.goroutine_id > 0);
}

test "interpreter environment operations" {
    const allocator = std.testing.allocator;
    
    var env = Environment.init(allocator, null);
    defer env.deinit();

    try env.define("test_var", InterpreterValue{ .integer = 42 });
    
    const value = env.get("test_var");
    try std.testing.expect(value != null);
    try std.testing.expect(value.?.integer == 42);
}
