const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

// Minimal stable CURSED compiler without stdlib dependencies
// Focuses on core language features with zero memory leaks

const ParseError = error{
    UnknownIdentifier,
    InvalidSyntax,
    DivisionByZero,
    OutOfMemory,
    TypeError,
};

const Variable = union(enum) {
    Integer: i64,
    String: []const u8,
    Boolean: bool,
    Array: ArrayList(Variable),
    
    pub fn deinit(self: *Variable, allocator: Allocator) void {
        switch (self.*) {
            .String => |s| allocator.free(s),
            .Array => |*arr| {
                for (arr.items) |*item| {
                    item.deinit(allocator);
                }
                arr.deinit();
            },
            else => {},
        }
    }
    
    pub fn clone(self: Variable, allocator: Allocator) !Variable {
        return switch (self) {
            .Integer => |i| Variable{ .Integer = i },
            .String => |s| Variable{ .String = try allocator.dupe(u8, s) },
            .Boolean => |b| Variable{ .Boolean = b },
            .Array => |arr| {
                var new_array = ArrayList(Variable).init(allocator);
                for (arr.items) |item| {
                    try new_array.append(try item.clone(allocator));
                }
                return Variable{ .Array = new_array };
            },
        };
    }
    
    pub fn format(self: Variable, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        switch (self) {
            .Integer => |i| try writer.print("{}", .{i}),
            .String => |s| try writer.print("{s}", .{s}),
            .Boolean => |b| try writer.print("{s}", .{if (b) "based" else "cringe"}),
            .Array => |arr| {
                try writer.print("[", .{});
                for (arr.items, 0..) |item, i| {
                    if (i > 0) try writer.print(", ", .{});
                    try writer.print("{}", .{item});
                }
                try writer.print("]", .{});
            },
        }
    }
};

const FunctionDef = struct {
    name: []const u8,
    params: ArrayList([]const u8),
    body: []const u8,
    
    pub fn deinit(self: *FunctionDef, allocator: Allocator) void {
        allocator.free(self.name);
        for (self.params.items) |param| {
            allocator.free(param);
        }
        self.params.deinit();
        allocator.free(self.body);
    }
};

const Context = struct {
    variables: std.HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    functions: std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, 80),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) Context {
        return Context{
            .variables = std.HashMap([]const u8, Variable, std.hash_map.StringContext, 80).init(allocator),
            .functions = std.HashMap([]const u8, FunctionDef, std.hash_map.StringContext, 80).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *Context) void {
        // Clean up variables
        var var_iter = self.variables.iterator();
        while (var_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit(self.allocator);
        }
        self.variables.deinit();
        
        // Clean up functions
        var func_iter = self.functions.iterator();
        while (func_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit(self.allocator);
        }
        self.functions.deinit();
    }
    
    pub fn setVariable(self: *Context, name: []const u8, value: Variable) !void {
        const owned_name = try self.allocator.dupe(u8, name);
        errdefer self.allocator.free(owned_name);
        
        // If variable already exists, clean up old value
        if (self.variables.getPtr(name)) |old_var| {
            old_var.deinit(self.allocator);
        }
        
        try self.variables.put(owned_name, value);
    }
    
    pub fn getVariable(self: *Context, name: []const u8) ?Variable {
        return self.variables.get(name);
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Stable Minimal Compiler v1.0.0\n", .{});
        print("Core language features only - no stdlib dependencies\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    const filename = args[1];

    // Read source file
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        print("Error: Could not open file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer file.close();

    const source = file.readToEndAlloc(allocator, 1024 * 1024) catch |err| {
        print("Error: Could not read file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    print("🚀 CURSED Stable Compiler Processing: {s}\n", .{filename});

    // Initialize context
    var ctx = Context.init(allocator);
    defer ctx.deinit();

    // Interpret the program
    interpretProgram(&ctx, source) catch |err| {
        print("Runtime error: {}\n", .{err});
        return;
    };
}

fn printUsage() void {
    print("CURSED Stable Minimal Compiler\n", .{});
    print("Usage: cursed-stable <file.csd> [options]\n", .{});
    print("\n", .{});
    print("Options:\n", .{});
    print("  --help      Show this help message\n", .{});
    print("  --version   Show version information\n", .{});
    print("\n", .{});
    print("Supported Features:\n", .{});
    print("  • Variables: sus x drip = 42\n", .{});
    print("  • Functions: slay add(a drip, b drip) drip {{ damn a + b }}\n", .{});
    print("  • Control flow: ready (x > 5) {{ vibez.spill(\"yes\") }}\n", .{});
    print("  • Loops: bestie (i < 3) {{ vibez.spill(i); i = i + 1 }}\n", .{});
    print("  • Arrays: sus arr []drip = [1, 2, 3]\n", .{});
    print("  • Basic I/O: vibez.spill(\"Hello\")\n", .{});
    print("\n", .{});
    print("No stdlib imports - built-in functions only for maximum stability\n", .{});
}

fn interpretProgram(ctx: *Context, source: []const u8) !void {
    var lines = std.mem.splitScalar(u8, source, '\n');
    
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        // Skip empty lines and comments
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        // Skip module imports (yeet statements) - make them no-ops
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            print("Info: Skipping module import (not supported in stable mode): {s}\n", .{trimmed});
            continue;
        }
        
        try executeStatement(ctx, trimmed);
    }
}

fn executeStatement(ctx: *Context, statement: []const u8) ParseError!void {
    const trimmed = std.mem.trim(u8, statement, " \t\r\n");
    if (trimmed.len == 0) return;
    
    // Variable declarations: sus name drip = value
    if (std.mem.startsWith(u8, trimmed, "sus ")) {
        try handleVariableDeclaration(ctx, trimmed);
        return;
    }
    
    // Function definitions: slay name(params) type { body }
    if (std.mem.startsWith(u8, trimmed, "slay ")) {
        try handleFunctionDeclaration(ctx, trimmed);
        return;
    }
    
    // Variable assignments: name = value
    if (std.mem.indexOf(u8, trimmed, "=")) |equals_pos| {
        // Make sure it's not a comparison operator
        if (equals_pos > 0 and equals_pos < trimmed.len - 1) {
            const prev_char = trimmed[equals_pos - 1];
            const next_char = trimmed[equals_pos + 1];
            if (prev_char != '!' and prev_char != '=' and prev_char != '<' and prev_char != '>' and 
                next_char != '=') {
                try handleAssignment(ctx, trimmed, equals_pos);
                return;
            }
        }
    }
    
    // Control flow: ready (condition) { body }
    if (std.mem.startsWith(u8, trimmed, "ready (")) {
        try handleIfStatement(ctx, trimmed);
        return;
    }
    
    // Loops: bestie (condition) { body }
    if (std.mem.startsWith(u8, trimmed, "bestie (")) {
        try handleWhileLoop(ctx, trimmed);
        return;
    }
    
    // I/O: vibez.spill(value)
    if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |_| {
        try handlePrint(ctx, trimmed);
        return;
    }
    
    // Function calls
    if (std.mem.indexOf(u8, trimmed, "(")) |_| {
        _ = try evaluateExpression(ctx, trimmed);
        return;
    }
}

fn handleVariableDeclaration(ctx: *Context, statement: []const u8) ParseError!void {
    // Parse: sus name type = value
    const parts_iter = std.mem.tokenizeAny(u8, statement[4..], " \t"); // Skip "sus "
    var parts = ArrayList([]const u8).init(ctx.allocator);
    defer parts.deinit();
    
    var iter = parts_iter;
    while (iter.next()) |part| {
        try parts.append(part);
    }
    
    if (parts.items.len < 4) { // name, type, "=", value
        print("Error: Invalid variable declaration: {s}\n", .{statement});
        return;
    }
    
    const name = parts.items[0];
    // Skip type for now - parts.items[1]
    if (!std.mem.eql(u8, parts.items[2], "=")) {
        print("Error: Expected '=' in variable declaration: {s}\n", .{statement});
        return;
    }
    
    // Join remaining parts as value expression
    var value_expr = ArrayList(u8).init(ctx.allocator);
    defer value_expr.deinit();
    
    for (parts.items[3..], 0..) |part, i| {
        if (i > 0) try value_expr.append(' ');
        try value_expr.appendSlice(part);
    }
    
    const value = try evaluateExpression(ctx, value_expr.items);
    try ctx.setVariable(name, value);
}

fn handleFunctionDeclaration(ctx: *Context, statement: []const u8) ParseError!void {
    // Parse: slay name(params) type { body }
    const content = std.mem.trim(u8, statement[5..], " \t"); // Skip "slay "
    
    const paren_start = std.mem.indexOf(u8, content, "(") orelse {
        print("Error: Missing opening parenthesis in function declaration\n", .{});
        return;
    };
    
    const name = std.mem.trim(u8, content[0..paren_start], " \t");
    
    const paren_end = std.mem.indexOf(u8, content, ")") orelse {
        print("Error: Missing closing parenthesis in function declaration\n", .{});
        return;
    };
    
    const params_str = std.mem.trim(u8, content[paren_start + 1..paren_end], " \t");
    
    const brace_start = std.mem.indexOf(u8, content, "{") orelse {
        print("Error: Missing opening brace in function declaration\n", .{});
        return;
    };
    
    const brace_end = std.mem.lastIndexOf(u8, content, "}") orelse {
        print("Error: Missing closing brace in function declaration\n", .{});
        return;
    };
    
    const body = std.mem.trim(u8, content[brace_start + 1..brace_end], " \t\r\n");
    
    // Parse parameters
    var params = ArrayList([]const u8).init(ctx.allocator);
    if (params_str.len > 0) {
        var param_iter = std.mem.splitSequence(u8, params_str, ",");
        while (param_iter.next()) |param| {
            const trimmed_param = std.mem.trim(u8, param, " \t");
            // Extract parameter name (before type)
            const space_pos = std.mem.indexOf(u8, trimmed_param, " ") orelse trimmed_param.len;
            const param_name = trimmed_param[0..space_pos];
            try params.append(try ctx.allocator.dupe(u8, param_name));
        }
    }
    
    const func_def = FunctionDef{
        .name = try ctx.allocator.dupe(u8, name),
        .params = params,
        .body = try ctx.allocator.dupe(u8, body),
    };
    
    const func_key = try ctx.allocator.dupe(u8, name);
    try ctx.functions.put(func_key, func_def);
}

fn handleAssignment(ctx: *Context, statement: []const u8, equals_pos: usize) ParseError!void {
    const var_name = std.mem.trim(u8, statement[0..equals_pos], " \t");
    const value_expr = std.mem.trim(u8, statement[equals_pos + 1..], " \t");
    
    const value = try evaluateExpression(ctx, value_expr);
    try ctx.setVariable(var_name, value);
}

fn handleIfStatement(ctx: *Context, statement: []const u8) ParseError!void {
    // Parse: ready (condition) { body }
    const content = statement[6..]; // Skip "ready "
    
    const paren_start = std.mem.indexOf(u8, content, "(") orelse return;
    const paren_end = std.mem.indexOf(u8, content, ")") orelse return;
    const brace_start = std.mem.indexOf(u8, content, "{") orelse return;
    const brace_end = std.mem.lastIndexOf(u8, content, "}") orelse return;
    
    const condition = std.mem.trim(u8, content[paren_start + 1..paren_end], " \t");
    const body = std.mem.trim(u8, content[brace_start + 1..brace_end], " \t\r\n");
    
    const condition_result = try evaluateCondition(ctx, condition);
    
    if (condition_result) {
        try executeStatementBlock(ctx, body);
    }
}

fn handleWhileLoop(ctx: *Context, statement: []const u8) ParseError!void {
    // Parse: bestie (condition) { body }
    const content = statement[7..]; // Skip "bestie "
    
    const paren_start = std.mem.indexOf(u8, content, "(") orelse return;
    const paren_end = std.mem.indexOf(u8, content, ")") orelse return;
    const brace_start = std.mem.indexOf(u8, content, "{") orelse return;
    const brace_end = std.mem.lastIndexOf(u8, content, "}") orelse return;
    
    const condition = std.mem.trim(u8, content[paren_start + 1..paren_end], " \t");
    const body = std.mem.trim(u8, content[brace_start + 1..brace_end], " \t\r\n");
    
    // Safety limit to prevent infinite loops in case of bugs
    var iterations: u32 = 0;
    const max_iterations = 10000;
    
    while (try evaluateCondition(ctx, condition) and iterations < max_iterations) {
        try executeStatementBlock(ctx, body);
        iterations += 1;
    }
    
    if (iterations >= max_iterations) {
        print("Warning: Loop iteration limit reached ({})\n", .{max_iterations});
    }
}

fn handlePrint(ctx: *Context, statement: []const u8) ParseError!void {
    // Parse: vibez.spill(value)
    const start = std.mem.indexOf(u8, statement, "vibez.spill(") orelse return;
    const args_start = start + 12; // Length of "vibez.spill("
    const paren_end = std.mem.lastIndexOf(u8, statement, ")") orelse return;
    
    const args_str = std.mem.trim(u8, statement[args_start..paren_end], " \t");
    
    if (args_str.len == 0) {
        print("\n", .{});
        return;
    }
    
    // Parse multiple arguments separated by commas
    var arg_iter = std.mem.splitSequence(u8, args_str, ",");
    var first = true;
    
    while (arg_iter.next()) |arg| {
        const trimmed_arg = std.mem.trim(u8, arg, " \t");
        if (trimmed_arg.len == 0) continue;
        
        if (!first) {
            print(" ", .{});
        }
        first = false;
        
        const value = try evaluateExpression(ctx, trimmed_arg);
        print("{}", .{value});
        
        // Clean up temporary values
        var temp_value = value;
        temp_value.deinit(ctx.allocator);
    }
    
    print("\n", .{});
}

fn executeStatementBlock(ctx: *Context, block: []const u8) ParseError!void {
    var statements = std.mem.splitAny(u8, block, ";\n");
    
    while (statements.next()) |stmt| {
        const trimmed = std.mem.trim(u8, stmt, " \t\r\n");
        if (trimmed.len == 0) continue;
        
        try executeStatement(ctx, trimmed);
    }
}

fn evaluateExpression(ctx: *Context, expr: []const u8) ParseError!Variable {
    const trimmed = std.mem.trim(u8, expr, " \t");
    
    // Boolean literals
    if (std.mem.eql(u8, trimmed, "based")) {
        return Variable{ .Boolean = true };
    }
    if (std.mem.eql(u8, trimmed, "cringe")) {
        return Variable{ .Boolean = false };
    }
    
    // String literals
    if (trimmed.len >= 2 and trimmed[0] == '"' and trimmed[trimmed.len - 1] == '"') {
        const str_content = trimmed[1..trimmed.len - 1];
        return Variable{ .String = try ctx.allocator.dupe(u8, str_content) };
    }
    
    // Array literals
    if (trimmed.len >= 2 and trimmed[0] == '[' and trimmed[trimmed.len - 1] == ']') {
        return try evaluateArrayLiteral(ctx, trimmed);
    }
    
    // Function calls
    if (std.mem.indexOf(u8, trimmed, "(")) |paren_pos| {
        return try evaluateFunctionCall(ctx, trimmed, paren_pos);
    }
    
    // Arithmetic expressions
    if (std.mem.containsAtLeast(u8, trimmed, 1, "+") or 
        std.mem.containsAtLeast(u8, trimmed, 1, "-") or 
        std.mem.containsAtLeast(u8, trimmed, 1, "*") or 
        std.mem.containsAtLeast(u8, trimmed, 1, "/")) {
        const result = try evaluateArithmetic(ctx, trimmed);
        return Variable{ .Integer = result };
    }
    
    // Variables
    if (ctx.getVariable(trimmed)) |value| {
        return try value.clone(ctx.allocator);
    }
    
    // Integer literals
    if (std.fmt.parseInt(i64, trimmed, 10)) |int_val| {
        return Variable{ .Integer = int_val };
    } else |_| {}
    
    // Default to string if nothing else matches
    return Variable{ .String = try ctx.allocator.dupe(u8, trimmed) };
}

fn evaluateArrayLiteral(ctx: *Context, expr: []const u8) ParseError!Variable {
    var array = ArrayList(Variable).init(ctx.allocator);
    
    const content = std.mem.trim(u8, expr[1..expr.len - 1], " \t");
    if (content.len == 0) {
        return Variable{ .Array = array };
    }
    
    var elem_iter = std.mem.splitSequence(u8, content, ",");
    while (elem_iter.next()) |elem| {
        const trimmed_elem = std.mem.trim(u8, elem, " \t");
        if (trimmed_elem.len == 0) continue;
        
        const value = try evaluateExpression(ctx, trimmed_elem);
        try array.append(value);
    }
    
    return Variable{ .Array = array };
}

fn evaluateFunctionCall(ctx: *Context, expr: []const u8, paren_pos: usize) ParseError!Variable {
    const func_name = std.mem.trim(u8, expr[0..paren_pos], " \t");
    const paren_end = std.mem.lastIndexOf(u8, expr, ")") orelse return error.InvalidSyntax;
    const args_str = std.mem.trim(u8, expr[paren_pos + 1..paren_end], " \t");
    
    // Built-in functions
    if (std.mem.eql(u8, func_name, "len")) {
        return try evaluateBuiltinLen(ctx, args_str);
    }
    
    // User-defined functions
    if (ctx.functions.get(func_name)) |func_def| {
        return try evaluateUserFunction(ctx, func_def, args_str);
    }
    
    print("Error: Unknown function: {s}\n", .{func_name});
    return error.UnknownIdentifier;
}

fn evaluateBuiltinLen(ctx: *Context, args_str: []const u8) ParseError!Variable {
    const value = try evaluateExpression(ctx, args_str);
    defer {
        var temp_value = value;
        temp_value.deinit(ctx.allocator);
    }
    
    switch (value) {
        .Array => |arr| return Variable{ .Integer = @intCast(arr.items.len) },
        .String => |s| return Variable{ .Integer = @intCast(s.len) },
        else => return Variable{ .Integer = 0 },
    }
}

fn evaluateUserFunction(ctx: *Context, func_def: FunctionDef, args_str: []const u8) ParseError!Variable {
    // Parse arguments
    var args = ArrayList(Variable).init(ctx.allocator);
    defer {
        for (args.items) |*arg| {
            arg.deinit(ctx.allocator);
        }
        args.deinit();
    }
    
    if (args_str.len > 0) {
        var arg_iter = std.mem.splitSequence(u8, args_str, ",");
        while (arg_iter.next()) |arg| {
            const trimmed_arg = std.mem.trim(u8, arg, " \t");
            if (trimmed_arg.len == 0) continue;
            
            const value = try evaluateExpression(ctx, trimmed_arg);
            try args.append(value);
        }
    }
    
    // Create new scope for function execution
    var func_ctx = Context.init(ctx.allocator);
    defer func_ctx.deinit();
    
    // Bind parameters to arguments
    for (func_def.params.items, 0..) |param, i| {
        if (i < args.items.len) {
            const cloned_arg = try args.items[i].clone(ctx.allocator);
            try func_ctx.setVariable(param, cloned_arg);
        }
    }
    
    // Execute function body
    try executeStatementBlock(&func_ctx, func_def.body);
    
    // Look for return value (damn statement)
    return try findReturnValue(&func_ctx, func_def.body);
}

fn findReturnValue(ctx: *Context, body: []const u8) ParseError!Variable {
    // Simple implementation: look for "damn expression" and evaluate it
    var lines = std.mem.splitAny(u8, body, "\n;");
    
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        if (std.mem.startsWith(u8, trimmed, "damn ")) {
            const return_expr = std.mem.trim(u8, trimmed[5..], " \t");
            return try evaluateExpression(ctx, return_expr);
        }
    }
    
    // Default return value
    return Variable{ .Integer = 0 };
}

fn evaluateCondition(ctx: *Context, condition: []const u8) ParseError!bool {
    const trimmed = std.mem.trim(u8, condition, " \t");
    
    // Boolean literals
    if (std.mem.eql(u8, trimmed, "based")) return true;
    if (std.mem.eql(u8, trimmed, "cringe")) return false;
    
    // Comparison operators
    if (std.mem.indexOf(u8, trimmed, "==")) |pos| {
        const left = try evaluateExpression(ctx, trimmed[0..pos]);
        defer { var temp = left; temp.deinit(ctx.allocator); }
        const right = try evaluateExpression(ctx, trimmed[pos + 2..]);
        defer { var temp = right; temp.deinit(ctx.allocator); }
        return compareVariables(left, right);
    }
    
    if (std.mem.indexOf(u8, trimmed, "!=")) |pos| {
        const left = try evaluateExpression(ctx, trimmed[0..pos]);
        defer { var temp = left; temp.deinit(ctx.allocator); }
        const right = try evaluateExpression(ctx, trimmed[pos + 2..]);
        defer { var temp = right; temp.deinit(ctx.allocator); }
        return !compareVariables(left, right);
    }
    
    if (std.mem.indexOf(u8, trimmed, "<=")) |pos| {
        const left = try evaluateArithmetic(ctx, trimmed[0..pos]);
        const right = try evaluateArithmetic(ctx, trimmed[pos + 2..]);
        return left <= right;
    }
    
    if (std.mem.indexOf(u8, trimmed, ">=")) |pos| {
        const left = try evaluateArithmetic(ctx, trimmed[0..pos]);
        const right = try evaluateArithmetic(ctx, trimmed[pos + 2..]);
        return left >= right;
    }
    
    if (std.mem.indexOf(u8, trimmed, "<")) |pos| {
        const left = try evaluateArithmetic(ctx, trimmed[0..pos]);
        const right = try evaluateArithmetic(ctx, trimmed[pos + 1..]);
        return left < right;
    }
    
    if (std.mem.indexOf(u8, trimmed, ">")) |pos| {
        const left = try evaluateArithmetic(ctx, trimmed[0..pos]);
        const right = try evaluateArithmetic(ctx, trimmed[pos + 1..]);
        return left > right;
    }
    
    // Variable or expression that evaluates to boolean
    const value = try evaluateExpression(ctx, trimmed);
    defer { var temp = value; temp.deinit(ctx.allocator); }
    
    return switch (value) {
        .Boolean => |b| b,
        .Integer => |i| i != 0,
        else => false,
    };
}

fn compareVariables(left: Variable, right: Variable) bool {
    switch (left) {
        .Integer => |l| switch (right) {
            .Integer => |r| return l == r,
            else => return false,
        },
        .Boolean => |l| switch (right) {
            .Boolean => |r| return l == r,
            else => return false,
        },
        .String => |l| switch (right) {
            .String => |r| return std.mem.eql(u8, l, r),
            else => return false,
        },
        else => return false,
    }
}

fn evaluateArithmetic(ctx: *Context, expr: []const u8) ParseError!i64 {
    const trimmed = std.mem.trim(u8, expr, " \t");
    
    // Addition
    if (std.mem.lastIndexOf(u8, trimmed, "+")) |pos| {
        const left = try evaluateArithmetic(ctx, trimmed[0..pos]);
        const right = try evaluateArithmetic(ctx, trimmed[pos + 1..]);
        return left + right;
    }
    
    // Subtraction
    if (std.mem.lastIndexOf(u8, trimmed, "-")) |pos| {
        if (pos > 0) { // Make sure it's not a negative sign at the beginning
            const left = try evaluateArithmetic(ctx, trimmed[0..pos]);
            const right = try evaluateArithmetic(ctx, trimmed[pos + 1..]);
            return left - right;
        }
    }
    
    // Multiplication
    if (std.mem.lastIndexOf(u8, trimmed, "*")) |pos| {
        const left = try evaluateArithmetic(ctx, trimmed[0..pos]);
        const right = try evaluateArithmetic(ctx, trimmed[pos + 1..]);
        return left * right;
    }
    
    // Division
    if (std.mem.lastIndexOf(u8, trimmed, "/")) |pos| {
        const left = try evaluateArithmetic(ctx, trimmed[0..pos]);
        const right = try evaluateArithmetic(ctx, trimmed[pos + 1..]);
        if (right == 0) return error.DivisionByZero;
        return @divTrunc(left, right);
    }
    
    // Variables
    if (ctx.getVariable(trimmed)) |value| {
        switch (value) {
            .Integer => |i| return i,
            .Boolean => |b| return if (b) 1 else 0,
            else => return 0,
        }
    }
    
    // Integer literals
    return std.fmt.parseInt(i64, trimmed, 10) catch 0;
}
