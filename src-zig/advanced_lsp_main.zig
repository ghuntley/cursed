//! Advanced CURSED Language Server Protocol Main Entry Point
//! Provides enhanced IDE features with comprehensive analysis

const std = @import("std");
const advanced_lsp = @import("advanced_lsp_server.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    
    const allocator = gpa.allocator();
    
    try advanced_lsp.runAdvancedLspServer(allocator);
}
