const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simple AST without circular dependencies for initial Zig compilation

pub const Program = struct {
    statements: ArrayList(*Statement),
    allocator: Allocator,

    pub fn init(allocator: Allocator) Program {
        return Program{
            .statements = ArrayList(*Statement).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Program) void {
        for (self.statements.items) |stmt| {
            stmt.deinit(self.allocator);
            self.allocator.destroy(stmt);
        }
        self.statements.deinit();
    }

    pub fn print(self: Program, indent: usize) !void {
        const print_fn = std.debug.print;
        const spaces = "  " ** 10;
        
        print_fn("{s}Program:\n", .{spaces[0..indent]});
        
        for (self.statements.items) |stmt| {
            try stmt.print(indent + 2);
        }
    }
};

pub const Statement = union(enum) {
    Function: FunctionStatement,
    Expression: ExpressionStatement,

    pub fn deinit(self: *Statement, allocator: Allocator) void {
        switch (self.*) {
            .Function => |*func| func.deinit(allocator),
            .Expression => |*expr| expr.deinit(allocator),
        }
    }

    pub fn print(self: Statement, indent: usize) !void {
        const print_fn = std.debug.print;
        const spaces = "  " ** 10;
        
        switch (self) {
            .Function => |func| {
                print_fn("{s}Function: {s}\n", .{ spaces[0..indent], func.name });
            },
            .Expression => {
                print_fn("{s}Expression\n", .{spaces[0..indent]});
            },
        }
    }
};

pub const FunctionStatement = struct {
    name: []const u8,
    body: ArrayList(*Statement),
    allocator: Allocator,

    pub fn init(allocator: Allocator, name: []const u8) FunctionStatement {
        return FunctionStatement{
            .name = name,
            .body = ArrayList(*Statement).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *FunctionStatement, allocator: Allocator) void {
        for (self.body.items) |stmt| {
            stmt.deinit(allocator);
            allocator.destroy(stmt);
        }
        self.body.deinit();
    }
};

pub const ExpressionStatement = struct {
    // Simplified expression for now
    
    pub fn deinit(self: *ExpressionStatement, allocator: Allocator) void {
        _ = self;
        _ = allocator;
    }
};

test "simple ast creation" {
    const allocator = std.testing.allocator;
    
    var program = Program.init(allocator);
    defer program.deinit();
    
    try std.testing.expect(program.statements.items.len == 0);
}
