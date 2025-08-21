//! CURSED LSP Server Main Entry Point
//! Standard LSP implementation entry point for IDE integration

const std = @import("std");
const lsp_server = @import("lsp_server.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    try lsp_server.runLspServer(allocator);
}
