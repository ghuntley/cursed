//! Enhanced CURSED Language Server main entry point
//! Provides complete LSP Protocol implementation with rich IDE features

const std = @import("std");
const enhanced_lsp_server = @import("enhanced_lsp_server.zig");

pub fn main() !void {
    // Setup allocator with appropriate configuration
    var gpa = std.heap.GeneralPurposeAllocator(.{
        .safety = true,
        .stack_trace_frames = 12,
    }){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Set up logging
    const log_file = std.fs.cwd().createFile("cursed_lsp.log", .{}) catch null;
    defer if (log_file) |file| file.close();
    
    std.log.info("Enhanced CURSED Language Server v1.0.0 starting...", .{});
    std.log.info("Process ID: {}", .{std.os.linux.getpid()});
    std.log.info("Features: Code completion, diagnostics, hover info, go-to-definition, formatting", .{});
    
    // Run the enhanced LSP server
    try enhanced_lsp_server.runEnhancedLspServer(allocator);
}
