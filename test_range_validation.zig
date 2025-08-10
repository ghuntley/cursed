const std = @import("std");
const lsp = @import("src-zig/lsp_server.zig");

// Test the range validation fix
test "LSP range validation prevents negative ranges" {
    const allocator = std.testing.allocator;
    
    // Test tokenPositionToLSP function
    const pos1 = lsp.tokenPositionToLSP(0, 0); // Should work
    try std.testing.expect(pos1.line == 0);
    try std.testing.expect(pos1.character == 0);
    
    // Test with very large values (would overflow u32)
    const large_val: usize = std.math.maxInt(u32) + 1000;
    const pos2 = lsp.tokenPositionToLSP(large_val, large_val);
    try std.testing.expect(pos2.line == std.math.maxInt(u32));
    try std.testing.expect(pos2.character == std.math.maxInt(u32));
    
    // Test createSafeRange function
    const range1 = lsp.createSafeRange(0, 0, 5, 10);
    try std.testing.expect(range1.start.line == 0);
    try std.testing.expect(range1.start.character == 0);
    try std.testing.expect(range1.end.line == 5);
    try std.testing.expect(range1.end.character == 10);
    
    // Test invalid range (end before start) - should be corrected
    const range2 = lsp.createSafeRange(5, 10, 2, 5);
    try std.testing.expect(range2.start.line == 5);
    try std.testing.expect(range2.start.character == 10);
    try std.testing.expect(range2.end.line == 5);
    try std.testing.expect(range2.end.character == 11); // Should be start + 1
}
