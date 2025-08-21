// LSP Server - Disabled due to API compatibility issues
const std = @import("std");
const Allocator = std.mem.Allocator;

pub const LSPServer = struct {
    allocator: Allocator,
    symbols: std.HashMap([]const u8, SymbolInfo, std.hash_map.StringContext, 80),
    diagnostics: std.ArrayList(Diagnostic),
    
    const Self = @This();
    
    pub fn init(allocator: Allocator) !Self {
        const symbols = std.HashMap([]const u8, SymbolInfo, std.hash_map.StringContext, 80).init(allocator);
        const diagnostics = std.ArrayList(Diagnostic){};
        return Self{
            .allocator = allocator,
            .symbols = symbols,
            .diagnostics = diagnostics,
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.symbols.deinit();
        self.diagnostics.deinit(self.allocator);
    }
    
    /// Main LSP message processing loop - DISABLED
    pub fn run(self: *Self) !void {
        _ = self;
        std.debug.print("LSP Server disabled due to API compatibility issues\n", .{});
    }
};

const SymbolInfo = struct {
    name: []const u8,
    kind: u8,
    location: Location,
};

const Location = struct {
    uri: []const u8,
    line: u32,
    character: u32,
};

const Diagnostic = struct {
    message: []const u8,
    severity: u8,
    location: Location,
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var server = try LSPServer.init(allocator);
    defer server.deinit();
    
    try server.run();
}
