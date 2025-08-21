//! CURSED Language Server main entry point

const std = @import("std");
const lsp_server = @import("lsp_server.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.log.info("CURSED Language Server starting...", .{});
    
    try lsp_server.runLspServer(allocator);
}
