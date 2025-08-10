const std = @import("std");
const testing = std.testing;

const attribute_parser = @import("src-zig/attribute_parser.zig");
const attribute_system = @import("src-zig/attribute_system.zig");
const lexer = @import("src-zig/lexer.zig");

test "AttributeParser rejects unknown attributes with proper error" {
    var arena = std.heap.ArenaAllocator.init(testing.allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    // Test parsing a known attribute (should succeed)
    const input_valid = "@inline(hint=\"always\")";
    var lex_valid = lexer.Lexer.init(allocator, input_valid);
    var parser_valid = attribute_parser.AttributeParser.init(allocator, &lex_valid);
    
    const valid_result = parser_valid.parseAttribute();
    try testing.expect(valid_result != attribute_system.AttributeError.UnknownAttribute);
    std.debug.print("✓ Valid attribute parsed successfully\n", .{});
    
    // Test parsing an unknown attribute (should fail with UnknownAttribute error)
    const input_invalid = "@unknown_attr";
    var lex_invalid = lexer.Lexer.init(allocator, input_invalid);
    var parser_invalid = attribute_parser.AttributeParser.init(allocator, &lex_invalid);
    
    const invalid_result = parser_invalid.parseAttribute();
    try testing.expectError(attribute_system.AttributeError.UnknownAttribute, invalid_result);
    std.debug.print("✓ Unknown attribute properly rejected with UnknownAttribute error\n", .{});
}
