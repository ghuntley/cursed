const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Separate types file to break circular dependencies
// This contains only the type definitions that don't reference each other circularly

pub const Visibility = enum {
    Public,    // spill
    Private,   // priv  
    Package,   // crew
};

pub const Comment = struct {
    text: []const u8,
    is_doc_comment: bool,
    line: usize,
    column: usize,
};

pub const BasicType = enum {
    Normie,    // i32
    Tea,       // string  
    Txt,       // string alias
    Sip,       // char
    Smol,      // i8
    Mid,       // i16
    Thicc,     // i64
    Snack,     // f32
    Meal,      // f64
    Byte,      // u8
    Rune,      // i32 alias
    Extra,     // complex
    Lit,       // bool
    Cap,       // null/nil
};

pub const Literal = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Character: u8,
    Null,
};

// Forward declarations for types that will be defined in ast_fixed.zig
pub const ExpressionRef = *opaque{};
pub const StatementRef = *opaque{};
pub const TypeRef = *opaque{};

// Helper functions for casting - these avoid circular import by using function parameters
pub fn expressionCast(ref: ExpressionRef) *anyopaque {
    return @ptrCast(ref);
}

pub fn statementCast(ref: StatementRef) *anyopaque {
    return @ptrCast(ref);
}

pub fn typeCast(ref: TypeRef) *anyopaque {
    return @ptrCast(ref);
}

pub fn expressionRef(expr: *anyopaque) ExpressionRef {
    return @ptrCast(expr);
}

pub fn statementRef(stmt: *anyopaque) StatementRef {
    return @ptrCast(stmt);
}

pub fn typeRef(type_val: *anyopaque) TypeRef {
    return @ptrCast(type_val);
}
