const std = @import("std");
const testing = std.testing;
const ArrayList = std.ArrayList;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");

const Token = lexer.Token;
const TokenKind = lexer.TokenKind;
const Parser = parser.Parser;

// Test struct parsing
test "parse struct statement" {
    const allocator = testing.allocator;
    
    // Test tokens for: squad Person { name tea, age normie }
    const tokens = [_]Token{
        Token.init(.Squad, "squad", 1, 1),
        Token.init(.Identifier, "Person", 1, 7),
        Token.init(.LeftBrace, "{", 1, 14),
        Token.init(.Identifier, "name", 2, 5),
        Token.init(.Tea, "tea", 2, 10),
        Token.init(.Comma, ",", 2, 13),
        Token.init(.Identifier, "age", 3, 5),
        Token.init(.Normie, "normie", 3, 9),
        Token.init(.RightBrace, "}", 4, 1),
        Token.init(.Eof, "", 4, 2),
    };
    
    var parser_instance = Parser.init(allocator, &tokens);
    var program = try parser_instance.parseProgram();
    defer program.deinit(allocator);
    
    try testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .Struct => |struct_stmt| {
            try testing.expect(std.mem.eql(u8, struct_stmt.name, "Person"));
            try testing.expect(struct_stmt.fields.items.len == 2);
            
            // Check first field
            const first_field = struct_stmt.fields.items[0];
            try testing.expect(std.mem.eql(u8, first_field.name, "name"));
            switch (first_field.field_type) {
                .Basic => |basic| try testing.expect(basic == .Tea),
                else => try testing.expect(false),
            }
            
            // Check second field
            const second_field = struct_stmt.fields.items[1];
            try testing.expect(std.mem.eql(u8, second_field.name, "age"));
            switch (second_field.field_type) {
                .Basic => |basic| try testing.expect(basic == .Normie),
                else => try testing.expect(false),
            }
        },
        else => try testing.expect(false),
    }
}

// Test generic struct parsing
test "parse generic struct statement" {
    const allocator = testing.allocator;
    
    // Test tokens for: squad Container<T> { value T }
    const tokens = [_]Token{
        Token.init(.Squad, "squad", 1, 1),
        Token.init(.Identifier, "Container", 1, 7),
        Token.init(.Less, "<", 1, 16),
        Token.init(.Identifier, "T", 1, 17),
        Token.init(.Greater, ">", 1, 18),
        Token.init(.LeftBrace, "{", 1, 20),
        Token.init(.Identifier, "value", 2, 5),
        Token.init(.Identifier, "T", 2, 11),
        Token.init(.RightBrace, "}", 3, 1),
        Token.init(.Eof, "", 3, 2),
    };
    
    var parser_instance = Parser.init(allocator, &tokens);
    var program = try parser_instance.parseProgram();
    defer program.deinit(allocator);
    
    try testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .Struct => |struct_stmt| {
            try testing.expect(std.mem.eql(u8, struct_stmt.name, "Container"));
            try testing.expect(struct_stmt.type_parameters.items.len == 1);
            try testing.expect(std.mem.eql(u8, struct_stmt.type_parameters.items[0].name, "T"));
        },
        else => try testing.expect(false),
    }
}

// Test interface parsing
test "parse interface statement" {
    const allocator = testing.allocator;
    
    // Test tokens for: collab Drawable { slay draw() }
    const tokens = [_]Token{
        Token.init(.Collab, "collab", 1, 1),
        Token.init(.Identifier, "Drawable", 1, 8),
        Token.init(.LeftBrace, "{", 1, 17),
        Token.init(.Slay, "slay", 2, 5),
        Token.init(.Identifier, "draw", 2, 10),
        Token.init(.LeftParen, "(", 2, 14),
        Token.init(.RightParen, ")", 2, 15),
        Token.init(.RightBrace, "}", 3, 1),
        Token.init(.Eof, "", 3, 2),
    };
    
    var parser_instance = Parser.init(allocator, &tokens);
    var program = try parser_instance.parseProgram();
    defer program.deinit(allocator);
    
    try testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .Interface => |interface_stmt| {
            try testing.expect(std.mem.eql(u8, interface_stmt.name, "Drawable"));
            try testing.expect(interface_stmt.methods.items.len == 1);
            try testing.expect(std.mem.eql(u8, interface_stmt.methods.items[0].name, "draw"));
        },
        else => try testing.expect(false),
    }
}

// Test for loop parsing (C-style)
test "parse for loop statement" {
    const allocator = testing.allocator;
    
    // Test tokens for: bestie sus i normie = 0; i < 10; i++ { }
    const tokens = [_]Token{
        Token.init(.Bestie, "bestie", 1, 1),
        Token.init(.Sus, "sus", 1, 8),
        Token.init(.Identifier, "i", 1, 12),
        Token.init(.Normie, "normie", 1, 14),
        Token.init(.Equal, "=", 1, 21),
        Token.init(.Number, "0", 1, 23),
        Token.init(.Semicolon, ";", 1, 24),
        Token.init(.Identifier, "i", 1, 26),
        Token.init(.Less, "<", 1, 28),
        Token.init(.Number, "10", 1, 30),
        Token.init(.Semicolon, ";", 1, 32),
        Token.init(.Identifier, "i", 1, 34),
        Token.init(.PlusPlus, "++", 1, 35),
        Token.init(.LeftBrace, "{", 1, 38),
        Token.init(.RightBrace, "}", 2, 1),
        Token.init(.Eof, "", 2, 2),
    };
    
    var parser_instance = Parser.init(allocator, &tokens);
    var program = try parser_instance.parseProgram();
    defer program.deinit(allocator);
    
    try testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .For => |for_stmt| {
            try testing.expect(for_stmt.init != null);
            try testing.expect(for_stmt.condition != null);
            try testing.expect(for_stmt.update != null);
            try testing.expect(for_stmt.body.items.len == 0);
        },
        else => try testing.expect(false),
    }
}

// Test while-style for loop
test "parse while-style for loop" {
    const allocator = testing.allocator;
    
    // Test tokens for: bestie i < 10 { }
    const tokens = [_]Token{
        Token.init(.Bestie, "bestie", 1, 1),
        Token.init(.Identifier, "i", 1, 8),
        Token.init(.Less, "<", 1, 10),
        Token.init(.Number, "10", 1, 12),
        Token.init(.LeftBrace, "{", 1, 15),
        Token.init(.RightBrace, "}", 2, 1),
        Token.init(.Eof, "", 2, 2),
    };
    
    var parser_instance = Parser.init(allocator, &tokens);
    var program = try parser_instance.parseProgram();
    defer program.deinit(allocator);
    
    try testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .For => |for_stmt| {
            try testing.expect(for_stmt.init == null);
            try testing.expect(for_stmt.condition != null);
            try testing.expect(for_stmt.update == null);
        },
        else => try testing.expect(false),
    }
}

// Test range-for loop
test "parse range for loop" {
    const allocator = testing.allocator;
    
    // Test tokens for: bestie item := flex collection { }
    const tokens = [_]Token{
        Token.init(.Bestie, "bestie", 1, 1),
        Token.init(.Identifier, "item", 1, 8),
        Token.init(.ColonEqual, ":=", 1, 13),
        Token.init(.Flex, "flex", 1, 16),
        Token.init(.Identifier, "collection", 1, 21),
        Token.init(.LeftBrace, "{", 1, 32),
        Token.init(.RightBrace, "}", 2, 1),
        Token.init(.Eof, "", 2, 2),
    };
    
    var parser_instance = Parser.init(allocator, &tokens);
    var program = try parser_instance.parseProgram();
    defer program.deinit(allocator);
    
    try testing.expect(program.statements.items.len == 1);
    
    switch (program.statements.items[0]) {
        .For => |for_stmt| {
            // Range-for is simplified to a condition-only for loop for now
            try testing.expect(for_stmt.init == null);
            try testing.expect(for_stmt.condition != null);
            try testing.expect(for_stmt.update == null);
        },
        else => try testing.expect(false),
    }
}

// Test complex program with multiple constructs
test "parse complex program" {
    const allocator = testing.allocator;
    
    // Test tokens for a program with struct, interface, and function
    const tokens = [_]Token{
        // squad Person { name tea }
        Token.init(.Squad, "squad", 1, 1),
        Token.init(.Identifier, "Person", 1, 7),
        Token.init(.LeftBrace, "{", 1, 14),
        Token.init(.Identifier, "name", 2, 5),
        Token.init(.Tea, "tea", 2, 10),
        Token.init(.RightBrace, "}", 3, 1),
        
        // collab Named { slay getName() tea }
        Token.init(.Collab, "collab", 5, 1),
        Token.init(.Identifier, "Named", 5, 8),
        Token.init(.LeftBrace, "{", 5, 14),
        Token.init(.Slay, "slay", 6, 5),
        Token.init(.Identifier, "getName", 6, 10),
        Token.init(.LeftParen, "(", 6, 17),
        Token.init(.RightParen, ")", 6, 18),
        Token.init(.Tea, "tea", 6, 20),
        Token.init(.RightBrace, "}", 7, 1),
        
        // slay main_character() { }
        Token.init(.Slay, "slay", 9, 1),
        Token.init(.MainCharacter, "main_character", 9, 6),
        Token.init(.LeftParen, "(", 9, 20),
        Token.init(.RightParen, ")", 9, 21),
        Token.init(.LeftBrace, "{", 9, 23),
        Token.init(.RightBrace, "}", 10, 1),
        
        Token.init(.Eof, "", 11, 1),
    };
    
    var parser_instance = Parser.init(allocator, &tokens);
    var program = try parser_instance.parseProgram();
    defer program.deinit(allocator);
    
    try testing.expect(program.statements.items.len == 3);
    
    // Check struct
    switch (program.statements.items[0]) {
        .Struct => |struct_stmt| {
            try testing.expect(std.mem.eql(u8, struct_stmt.name, "Person"));
        },
        else => try testing.expect(false),
    }
    
    // Check interface
    switch (program.statements.items[1]) {
        .Interface => |interface_stmt| {
            try testing.expect(std.mem.eql(u8, interface_stmt.name, "Named"));
        },
        else => try testing.expect(false),
    }
    
    // Check function
    switch (program.statements.items[2]) {
        .Function => |func_stmt| {
            try testing.expect(std.mem.eql(u8, func_stmt.name, "main_character"));
        },
        else => try testing.expect(false),
    }
}
