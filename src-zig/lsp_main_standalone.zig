//! CURSED LSP Server Main Entry Point - Standalone Version

const std = @import("std");
const lsp_server = @import("lsp_standalone.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // Handle command line arguments
    if (args.len > 1) {
        if (std.mem.eql(u8, args[1], "--version")) {
            try std.fs.File.stdout().writer(&[_]u8{}).print("CURSED LSP Server v1.0.0 - Language Server Protocol Support\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("Features: Basic completion, diagnostics, and hover support\n", .{});
            return;
        }
        
        if (std.mem.eql(u8, args[1], "--help")) {
            try std.fs.File.stdout().writer(&[_]u8{}).print("CURSED Language Server Protocol Server\n\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("USAGE:\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    cursed-lsp [OPTIONS]\n\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("OPTIONS:\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    --version    Show version information\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    --help       Show this help message\n\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("FEATURES:\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    • Basic syntax highlighting\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    • Error diagnostics\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    • Code completion for keywords\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    • Basic hover information\n\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("EDITOR SETUP:\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    VS Code: Configure in settings.json\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    Vim/Neovim: Use with nvim-lspconfig\n", .{});
            try std.fs.File.stdout().writer(&[_]u8{}).print("    Emacs: Use with lsp-mode\n", .{});
            return;
        }
    }

    // Start the LSP server
    try lsp_server.runLspServer(allocator);
}
