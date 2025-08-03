const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simplified AST to break circular dependency
pub const Expression = struct {
    tag: ExpressionTag,
    data: *anyopaque,

    pub const ExpressionTag = enum {
        Identifier,
        Variable,
        Integer,
        Float,
        String,
        Boolean,
        Character,
        Binary,
        Call,
        Array,
        Literal,
    };

    pub fn deinit(self: *Expression, allocator: Allocator) void {
        _ = self;
        _ = allocator;
        // TODO: Implement cleanup
    }

    pub fn print(self: Expression, indent: usize) !void {
        _ = self;
        _ = indent;
        // TODO: Implement print
    }
};

pub const Program = struct {
    statements: ArrayList(Statement),
    imports: ArrayList(ImportStatement),
    package: ?PackageDeclaration,

    pub fn init(allocator: Allocator) Program {
        return Program{
            .statements = ArrayList(Statement).init(allocator),
            .imports = ArrayList(ImportStatement).init(allocator),
            .package = null,
        };
    }

    pub fn deinit(self: *Program, allocator: Allocator) void {
        _ = self;
        _ = allocator;
        // TODO: Implement cleanup
    }

    pub fn print(self: Program, indent: usize) !void {
        _ = self;
        _ = indent;
        // TODO: Implement print
    }
};

pub const Statement = struct {
    tag: StatementTag,
    data: *anyopaque,

    pub const StatementTag = enum {
        Expression,
        Variable,
        Function,
        Return,
        If,
        While,
        For,
        Block,
        Break,
        Continue,
        Package,
        Import,
        Defer,
        Assignment,
    };

    pub fn deinit(self: *Statement, allocator: Allocator) void {
        _ = self;
        _ = allocator;
        // TODO: Implement cleanup
    }

    pub fn print(self: Statement, indent: usize) !void {
        _ = self;
        _ = indent;
        // TODO: Implement print
    }
};

pub const FunctionStatement = struct {
    name: []const u8,
    parameters: ArrayList(Parameter),
    return_type: ?Type,
    body: Statement,
    visibility: Visibility,

    pub fn deinit(self: *FunctionStatement, allocator: Allocator) void {
        _ = self;
        _ = allocator;
        // TODO: Implement cleanup
    }
};

pub const Parameter = struct {
    name: []const u8,
    param_type: Type,
};

pub const Type = struct {
    name: []const u8,
};

pub const Visibility = enum {
    Public,
    Private,
};

pub const ImportStatement = struct {
    path: []const u8,
    alias: ?[]const u8,

    pub fn deinit(self: *ImportStatement, allocator: Allocator) void {
        _ = self;
        _ = allocator;
        // TODO: Implement cleanup
    }
};

pub const PackageDeclaration = struct {
    name: []const u8,

    pub fn deinit(self: *PackageDeclaration, allocator: Allocator) void {
        _ = self;
        _ = allocator;
        // TODO: Implement cleanup
    }
};
