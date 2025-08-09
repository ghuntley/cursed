const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Simple module function extractor that just extracts function names
// without complex AST parsing to avoid memory corruption issues

pub const SimpleFunctionInfo = struct {
    name: []const u8,
    available: bool,
    
    pub fn deinit(self: *SimpleFunctionInfo, allocator: Allocator) void {
        allocator.free(self.name);
    }
};

pub fn extractFunctionNames(allocator: Allocator, source: []const u8) !ArrayList(SimpleFunctionInfo) {
    var functions = ArrayList(SimpleFunctionInfo).init(allocator);
    
    // Simple regex-like parsing to find function declarations
    var lines = std.mem.splitScalar(u8, source, '\n');
    
    while (lines.next()) |line| {
        var trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        // Look for function declarations: "slay function_name"
        if (std.mem.startsWith(u8, trimmed, "slay ")) {
            const after_slay = trimmed[5..]; // Skip "slay "
            
            // Find the function name (up to the first '(' or space)
            var end_idx: usize = 0;
            for (after_slay, 0..) |char, i| {
                if (char == '(' or char == ' ' or char == '\t') {
                    end_idx = i;
                    break;
                }
            }
            
            if (end_idx > 0) {
                const func_name = after_slay[0..end_idx];
                
                // Validate function name
                if (func_name.len > 0 and func_name.len <= 64 and isValidIdentifier(func_name)) {
                    const name_copy = try allocator.dupe(u8, func_name);
                    try functions.append(SimpleFunctionInfo{
                        .name = name_copy,
                        .available = true,
                    });
                }
            }
        }
    }
    
    return functions;
}

fn isValidIdentifier(name: []const u8) bool {
    if (name.len == 0) return false;
    
    for (name) |char| {
        if (!std.ascii.isAlphanumeric(char) and char != '_') {
            return false;
        }
    }
    
    return true;
}
