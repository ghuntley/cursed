const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Lexer = lexer.Lexer;

const ast = @import("ast.zig");
const Expression = ast.Expression;
const SourceLocation = ast.SourceLocation;

const attribute_system = @import("attribute_system.zig");
const Attribute = attribute_system.Attribute;
const AttributeType = attribute_system.AttributeType;
const AttributeValue = attribute_system.AttributeValue;
const AttributeList = attribute_system.AttributeList;
const AttributeError = attribute_system.AttributeError;
const parseAttributeType = attribute_system.parseAttributeType;

/// Attribute parser that extends the CURSED parser with attribute support
/// Parses attribute syntax: @attribute_name(param1=value1, param2=value2)
pub const AttributeParser = struct {
    lexer: *Lexer,
    allocator: Allocator,
    current_token: Token,
    peek_token: Token,
    
    pub fn init(allocator: Allocator, lex: *Lexer) AttributeParser {
        var parser = AttributeParser{
            .lexer = lex,
            .allocator = allocator,
            .current_token = undefined,
            .peek_token = undefined,
        };
        
        // Initialize tokens
        parser.nextToken() catch unreachable;
        parser.nextToken() catch unreachable;
        
        return parser;
    }
    
    fn nextToken(self: *AttributeParser) !void {
        self.current_token = self.peek_token;
        self.peek_token = try self.lexer.nextToken();
    }
    
    fn currentTokenIs(self: *AttributeParser, token_kind: TokenKind) bool {
        return self.current_token.kind == token_kind;
    }
    
    fn peekTokenIs(self: *AttributeParser, token_kind: TokenKind) bool {
        return self.peek_token.kind == token_kind;
    }
    
    fn expectToken(self: *AttributeParser, token_kind: TokenKind) !void {
        if (!self.currentTokenIs(token_kind)) {
            return error.UnexpectedToken;
        }
        try self.nextToken();
    }
    
    /// Parse a list of attributes starting with @
    /// Syntax: @attr1 @attr2(param=value) @attr3(p1=v1, p2=v2)
    pub fn parseAttributeList(self: *AttributeParser) !AttributeList {
        var attrs = AttributeList.init(self.allocator);
        
        while (self.currentTokenIs(.At)) {
            const attr = try self.parseAttribute();
            try attrs.addAttribute(attr);
        }
        
        return attrs;
    }
    
    /// Parse a single attribute
    /// Syntax: @attribute_name or @attribute_name(parameters)
    pub fn parseAttribute(self: *AttributeParser) !Attribute {
        try self.expectToken(.At); // Consume @
        
        if (!self.currentTokenIs(.Identifier)) {
            return error.ExpectedAttributeName;
        }
        
        const attr_name = self.current_token.lexeme;
        const location = SourceLocation.init(
            "<unknown>",
            @intCast(self.current_token.line),
            @intCast(self.current_token.column),
            @intCast(self.current_token.offset)
        );
        
        try self.nextToken(); // Consume attribute name
        
        // Determine attribute type - fail on unknown attributes
        const attr_type = parseAttributeType(attr_name) orelse {
            std.log.err("Unknown attribute '@{s}' at line {d}, column {d}. Valid attributes are: performance, inline, optimize, unroll, vectorize, memory_layout, align, pack, cache, debug, no_debug, profile_guided, export, import, extern, link_section, unsafe, bounds, overflow, atomic, thread_safe, lock, test, benchmark, fuzz, doc, deprecated, since", .{ attr_name, location.line, location.column });
            return AttributeError.UnknownAttribute;
        };
        
        var attr = Attribute.init(self.allocator, attr_type, attr_name, location);
        
        // Check for parameters
        if (self.currentTokenIs(.LeftParen)) {
            try self.parseAttributeParameters(&attr);
        }
        
        return attr;
    }
    
    /// Parse attribute parameters
    /// Syntax: (param1=value1, param2=value2, ...)
    fn parseAttributeParameters(self: *AttributeParser, attr: *Attribute) !void {
        try self.expectToken(.LeftParen); // Consume (
        
        // Handle empty parameter list
        if (self.currentTokenIs(.RightParen)) {
            try self.nextToken();
            return;
        }
        
        while (true) {
            // Parse parameter name
            if (!self.currentTokenIs(.Identifier)) {
                return error.ExpectedParameterName;
            }
            
            const param_name = self.current_token.lexeme;
            try self.nextToken();
            
            // Expect =
            try self.expectToken(.Assign);
            
            // Parse parameter value
            const param_value = try self.parseAttributeValue();
            try attr.addParameter(self.allocator, param_name, param_value);
            
            // Check for more parameters
            if (self.currentTokenIs(.Comma)) {
                try self.nextToken(); // Consume comma
                continue;
            } else if (self.currentTokenIs(.RightParen)) {
                break;
            } else {
                return error.ExpectedCommaOrRightParen;
            }
        }
        
        try self.expectToken(.RightParen); // Consume )
    }
    
    /// Parse an attribute parameter value
    fn parseAttributeValue(self: *AttributeParser) !AttributeValue {
        switch (self.current_token.kind) {
            .StringLiteral => {
                const value = AttributeValue{ .String = self.current_token.lexeme };
                try self.nextToken();
                return value;
            },
            .Integer => {
                // Parse integer from lexeme
                const int_val = std.fmt.parseInt(i64, self.current_token.lexeme, 10) catch {
                    return error.InvalidIntegerValue;
                };
                const value = AttributeValue{ .Integer = int_val };
                try self.nextToken();
                return value;
            },
            .Number => {
                // Parse float from lexeme  
                const float_val = std.fmt.parseFloat(f64, self.current_token.lexeme) catch {
                    return error.InvalidFloatValue;
                };
                const value = AttributeValue{ .Float = float_val };
                try self.nextToken();
                return value;
            },
            .Based => {
                const value = AttributeValue{ .Boolean = true };
                try self.nextToken();
                return value;
            },
            .Cringe => {
                const value = AttributeValue{ .Boolean = false };
                try self.nextToken();
                return value;
            },
            .Identifier => {
                const value = AttributeValue{ .Identifier = self.current_token.lexeme };
                try self.nextToken();
                return value;
            },
            else => {
                // For complex expressions, we would need to integrate with the main parser
                // For now, return an error
                return error.UnsupportedAttributeValueType;
            }
        }
    }
    
    /// Check if current position has attributes
    pub fn hasAttributes(self: *AttributeParser) bool {
        return self.currentTokenIs(.At);
    }
    
    /// Skip all attributes at current position (useful for error recovery)
    pub fn skipAttributes(self: *AttributeParser) void {
        while (self.currentTokenIs(.At)) {
            self.nextToken() catch break; // Skip @
            
            if (self.currentTokenIs(.Identifier)) {
                self.nextToken() catch break; // Skip attribute name
            }
            
            // Skip parameters if present
            if (self.currentTokenIs(.LeftParen)) {
                var paren_count: usize = 1;
                self.nextToken() catch break;
                
                while (paren_count > 0 and !self.currentTokenIs(.Eof)) {
                    if (self.currentTokenIs(.LeftParen)) {
                        paren_count += 1;
                    } else if (self.currentTokenIs(.RightParen)) {
                        paren_count -= 1;
                    }
                    self.nextToken() catch break;
                }
            }
        }
    }
};

/// Integration helpers for the main parser

/// Parse attributes and attach them to function declarations
pub fn parseAttributesForFunction(allocator: Allocator, lex: *Lexer) !?AttributeList {
    var attr_parser = AttributeParser.init(allocator, lex);
    
    if (!attr_parser.hasAttributes()) {
        return null;
    }
    
    return try attr_parser.parseAttributeList();
}

/// Parse attributes and attach them to struct declarations
pub fn parseAttributesForStruct(allocator: Allocator, lex: *Lexer) !?AttributeList {
    var attr_parser = AttributeParser.init(allocator, lex);
    
    if (!attr_parser.hasAttributes()) {
        return null;
    }
    
    return try attr_parser.parseAttributeList();
}

/// Parse attributes and attach them to variable declarations
pub fn parseAttributesForVariable(allocator: Allocator, lex: *Lexer) !?AttributeList {
    var attr_parser = AttributeParser.init(allocator, lex);
    
    if (!attr_parser.hasAttributes()) {
        return null;
    }
    
    return try attr_parser.parseAttributeList();
}

/// Error types for attribute parsing
pub const AttributeParseError = error{
    UnexpectedToken,
    ExpectedAttributeName,
    ExpectedParameterName,
    ExpectedCommaOrRightParen,
    UnsupportedAttributeValueType,
    InvalidAttributeSyntax,
    InvalidIntegerValue,
    InvalidFloatValue,
};

/// Testing utilities
fn createTestLexer(allocator: Allocator, source: []const u8) !Lexer {
    return Lexer.init(allocator, source);
}

test "parse simple attribute" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const source = "@inline";
    var lex = try createTestLexer(allocator, source);
    
    var parser = AttributeParser.init(allocator, &lex);
    var attrs = try parser.parseAttributeList();
    defer attrs.deinit(allocator);
    
    try std.testing.expect(attrs.attributes.items.len == 1);
    const attr = &attrs.attributes.items[0];
    try std.testing.expect(attr.type == .Inline);
    try std.testing.expect(std.mem.eql(u8, attr.name, "inline"));
}

test "parse attribute with parameters" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const source = "@performance(level=high)";
    var lex = try createTestLexer(allocator, source);
    
    var parser = AttributeParser.init(allocator, &lex);
    var attrs = try parser.parseAttributeList();
    defer attrs.deinit(allocator);
    
    try std.testing.expect(attrs.attributes.items.len == 1);
    const attr = &attrs.attributes.items[0];
    try std.testing.expect(attr.type == .Performance);
    
    const level = attr.getStringParameter("level");
    try std.testing.expect(level != null);
    try std.testing.expect(std.mem.eql(u8, level.?, "high"));
}

test "parse multiple attributes" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const source = "@inline @performance(level=medium) @debug(enable=true)";
    var lex = try createTestLexer(allocator, source);
    
    var parser = AttributeParser.init(allocator, &lex);
    var attrs = try parser.parseAttributeList();
    defer attrs.deinit(allocator);
    
    try std.testing.expect(attrs.attributes.items.len == 3);
    try std.testing.expect(attrs.hasAttribute(.Inline));
    try std.testing.expect(attrs.hasAttribute(.Performance));
    try std.testing.expect(attrs.hasAttribute(.Debug));
}

test "reject unknown custom attribute" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const source = "@my_custom_attr(value=42, name=\"test\")";
    var lex = try createTestLexer(allocator, source);
    
    var parser = AttributeParser.init(allocator, &lex);
    const result = parser.parseAttributeList();
    
    // This should now fail with UnknownAttribute error since custom attributes are not allowed
    try std.testing.expectError(AttributeError.UnknownAttribute, result);
}
