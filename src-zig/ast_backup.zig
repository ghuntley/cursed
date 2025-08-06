const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Index-based AST to break circular dependencies
pub const NodeIndex = u32;
pub const INVALID_NODE: NodeIndex = std.math.maxInt(NodeIndex);

// Forward declaration for AST container
pub const AST = struct {
    expressions: ArrayList(Expression),
    statements: ArrayList(Statement),
    allocator: Allocator,

    pub fn init(allocator: Allocator) AST {
        return AST{
            .expressions = ArrayList(Expression).init(allocator),
            .statements = ArrayList(Statement).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *AST) void {
        // Clean up all expressions in reverse order to handle dependencies
        var i = self.expressions.items.len;
        while (i > 0) {
            i -= 1;
            self.expressions.items[i].deinit(self.allocator, self);
        }
        self.expressions.deinit();

        // Clean up all statements
        i = self.statements.items.len;
        while (i > 0) {
            i -= 1;
            self.statements.items[i].deinit(self.allocator, self);
        }
        self.statements.deinit();
    }

    pub fn addExpression(self: *AST, expr: Expression) !NodeIndex {
        const index = @as(NodeIndex, @intCast(self.expressions.items.len));
        try self.expressions.append(expr);
        return index;
    }

    pub fn getExpression(self: *AST, index: NodeIndex) ?*Expression {
        if (index == INVALID_NODE or index >= self.expressions.items.len) return null;
        return &self.expressions.items[index];
    }
    
    pub fn addStatement(self: *AST, stmt: Statement) !NodeIndex {
        const index = @as(NodeIndex, @intCast(self.statements.items.len));
        try self.statements.append(stmt);
        return index;
    }
};

// Basic types
pub const BasicType = enum { Normie, Tea, Lit, Cap };

pub const Literal = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,
};

// Expression structures using indices to break circular dependency
pub const BinaryExpression = struct {
    left: NodeIndex,
    operator: []const u8,
    right: NodeIndex,

    pub fn deinit(self: *BinaryExpression, allocator: Allocator, ast: *AST) void {
        _ = allocator;
        _ = ast;
        _ = self;
        // Individual expressions cleaned up by AST container
    }
};

pub const CallExpression = struct {
    function: NodeIndex,
    arguments: ArrayList(NodeIndex),

    pub fn deinit(self: *CallExpression, allocator: Allocator, ast: *AST) void {
        _ = allocator;
        _ = ast;
        self.arguments.deinit();
    }
};

// Minimal statement stubs
pub const LetStatement = struct {
    identifier: []const u8,
    value: NodeIndex,
    pub fn deinit(self: *LetStatement, allocator: Allocator, ast: *AST) void { 
        _ = self; _ = allocator; _ = ast; 
    }
};

// Expression type uses indices instead of pointers to break circular dependency
pub const Expression = union(enum) {
    Identifier: []const u8,
    Variable: []const u8,
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Binary: BinaryExpression,
    Call: CallExpression,
    Literal: Literal,

    pub fn deinit(self: *Expression, allocator: Allocator, ast: *AST) void {
        switch (self.*) {
            .Binary => |*bin| bin.deinit(allocator, ast),
            .Call => |*call| call.deinit(allocator, ast),
            else => {}, // Simple types don't need cleanup
        }
    }

    pub fn print(self: Expression, indent: usize, ast: *AST) !void {
        const print_fn = std.debug.print;
        _ = indent;
        
        switch (self) {
            .Identifier => |id| print_fn("{s}", .{id}),
            .Variable => |var_name| print_fn("{s}", .{var_name}),
            .Integer => |int| print_fn("{}", .{int}),
            .Float => |float| print_fn("{d}", .{float}),
            .String => |str| print_fn("\"{s}\"", .{str}),
            .Boolean => |bool_val| print_fn("{}", .{bool_val}),
            .Character => |char| print_fn("'{c}'", .{char}),
            .Binary => |bin| {
                if (bin.left != INVALID_NODE) {
                    try ast.expressions.items[bin.left].print(0, ast);
                }
                print_fn(" {s} ", .{bin.operator});
                if (bin.right != INVALID_NODE) {
                    try ast.expressions.items[bin.right].print(0, ast);
                }
            },
            .Call => |call| {
                if (call.function != INVALID_NODE) {
                    try ast.expressions.items[call.function].print(0, ast);
                }
                print_fn("(", .{});
                for (call.arguments.items, 0..) |arg, i| {
                    if (i > 0) print_fn(", ", .{});
                    if (arg != INVALID_NODE) {
                        try ast.expressions.items[arg].print(0, ast);
                    }
                }
                print_fn(")", .{});
            },
            else => print_fn("{s}", .{@tagName(self)}),
        }
    }
};

// Statement structure also uses indices for consistency
pub const Statement = union(enum) {
    Expression: NodeIndex,
    Let: LetStatement,

    pub fn deinit(self: *Statement, allocator: Allocator, ast: *AST) void {
        switch (self.*) {
            .Let => |*let| let.deinit(allocator, ast),
            else => {}, // Simple statements don't need cleanup
        }
    }
};

// Program structure
pub const Program = struct {
    statements: ArrayList(Statement),
    
    pub fn init(allocator: Allocator) Program {
        return Program{
            .statements = ArrayList(Statement).init(allocator),
        };
    }

    pub fn deinit(self: *Program) void {
        self.statements.deinit();
    }
};
