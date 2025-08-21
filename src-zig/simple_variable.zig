// Simple Variable type for concurrency bridge compatibility
const std = @import("std");

pub const Variable = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    Null,
};
