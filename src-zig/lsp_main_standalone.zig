//! CURSED LSP Server Main Entry Point - Standalone Version

const std = @import("std");
const lsp_server = @import("lsp_standalone.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // Handle command line arguments
    if (args.len > 1) {
        if (std.mem.eql(u8, args[1], "--version")) {
            try std.io.getStdOut().writer().print("CURSED LSP Server v1.0.0 - Language Server Protocol Support\n", .{});
            try std.io.getStdOut().writer().print("Features: Basic completion, diagnostics, and hover support\n", .{});
            return;
        }
        
        if (std.mem.eql(u8, args[1], "--help")) {
            try std.io.getStdOut().writer().print("CURSED Language Server Protocol Server\n\n", .{});
            try std.io.getStdOut().writer().print("USAGE:\n", .{});
            try std.io.getStdOut().writer().print("    cursed-lsp [OPTIONS]\n\n", .{});
            try std.io.getStdOut().writer().print("OPTIONS:\n", .{});
            try std.io.getStdOut().writer().print("    --version    Show version information\n", .{});
            try std.io.getStdOut().writer().print("    --help       Show this help message\n\n", .{});
            try std.io.getStdOut().writer().print("FEATURES:\n", .{});
            try std.io.getStdOut().writer().print("    • Basic syntax highlighting\n", .{});
            try std.io.getStdOut().writer().print("    • Error diagnostics\n", .{});
            try std.io.getStdOut().writer().print("    • Code completion for keywords\n", .{});
            try std.io.getStdOut().writer().print("    • Basic hover information\n\n", .{});
            try std.io.getStdOut().writer().print("EDITOR SETUP:\n", .{});
            try std.io.getStdOut().writer().print("    VS Code: Configure in settings.json\n", .{});
            try std.io.getStdOut().writer().print("    Vim/Neovim: Use with nvim-lspconfig\n", .{});
            try std.io.getStdOut().writer().print("    Emacs: Use with lsp-mode\n", .{});
            return;
        }
    }

    // Start the LSP server
    try lsp_server.runLspServer(allocator);
}
