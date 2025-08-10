//! CURSED Language Formatter
//! Automatic code formatting for consistent style

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");

/// Formatting configuration
const FormatConfig = struct {
    indent_size: u32 = 4,
    use_tabs: bool = false,
    max_line_length: u32 = 100,
    trailing_commas: bool = true,
    space_before_paren: bool = false,
    space_after_comma: bool = true,
    space_around_operators: bool = true,
    newline_before_brace: bool = false,
    align_assignments: bool = false,
    
    pub fn fromFile(allocator: Allocator, path: []const u8) !FormatConfig {
        _ = allocator;
        _ = path;
        // TODO: Parse configuration file
        return FormatConfig{};
    }
};

/// Token types from lexer
const TokenType = enum {
    // Literals
    identifier,
    integer,
    float,
    string,
    char,
    
    // Keywords
    sus, slay, damn, vibez, yeet, ready, otherwise, bestie, squad, collab,
    sick, when, stan, defer, yikes, fam, shook, based, cringe, facts,
    lit, tea, drip, normie, smol, mid, thicc, periodt, aight, spill,
    basic, mood, flex, dm, chan, be_like,
    
    // Operators
    plus, minus, multiply, divide, modulo,
    assign, plus_assign, minus_assign, multiply_assign, divide_assign,
    equal, not_equal, less_than, less_equal, greater_than, greater_equal,
    logical_and, logical_or, logical_not,
    bitwise_and, bitwise_or, bitwise_xor, bitwise_not,
    left_shift, right_shift,
    increment, decrement,
    walrus, arrow, channel_send,
    
    // Delimiters
    left_paren, right_paren,
    left_brace, right_brace,
    left_bracket, right_bracket,
    semicolon, comma, dot, colon, question,
    
    // Special
    newline, whitespace, comment,
    eof, invalid,
};

/// Position tracking for formatting
const Position = struct {
    line: u32,
    column: u32,
};

/// Formatted token with position
const FormattedToken = struct {
    type: TokenType,
    value: []const u8,
    position: Position,
    leading_whitespace: []const u8,
    trailing_whitespace: []const u8,
};

/// CURSED code formatter
const CursedFormatter = struct {
    allocator: Allocator,
    config: FormatConfig,
    source_code: []const u8,
    output: ArrayList(u8),
    current_line: u32,
    current_column: u32,
    indent_level: u32,
    in_function: bool,
    in_struct: bool,
    in_expression: bool,
    
    pub fn init(allocator: Allocator, config: FormatConfig) CursedFormatter {
        return CursedFormatter{
            .allocator = allocator,
            .config = config,
            .source_code = "",
            .output = ArrayList(u8).init(allocator),
            .current_line = 1,
            .current_column = 1,
            .indent_level = 0,
            .in_function = false,
            .in_struct = false,
            .in_expression = false,
        };
    }
    
    pub fn deinit(self: *CursedFormatter) void {
        self.output.deinit();
    }
    
    /// Format a source file
    pub fn formatFile(self: *CursedFormatter, input_path: []const u8, output_path: ?[]const u8) !void {
        // Read input file
        const file = try std.fs.cwd().openFile(input_path, .{});
        defer file.close();
        
        const file_size = try file.getEndPos();
        self.source_code = try self.allocator.alloc(u8, file_size);
        _ = try file.readAll(self.source_code);
        
        // Format the code
        try self.format();
        
        // Write output
        if (output_path) |path| {
            const output_file = try std.fs.cwd().createFile(path, .{});
            defer output_file.close();
            try output_file.writeAll(self.output.items);
        } else {
            // Overwrite input file
            const output_file = try std.fs.cwd().createFile(input_path, .{});
            defer output_file.close();
            try output_file.writeAll(self.output.items);
        }
    }
    
    /// Format code from string
    pub fn formatString(self: *CursedFormatter, source: []const u8) ![]const u8 {
        self.source_code = source;
        try self.format();
        return self.output.toOwnedSlice();
    }
    
    /// Main formatting logic
    fn format(self: *CursedFormatter) !void {
        // Tokenize the source code
        var lex = lexer.Lexer.init(self.allocator, self.source_code);
        defer lex.deinit();
        
        const tokens = try lex.tokenize();
        defer tokens.deinit();
        
        // Format tokens
        try self.formatTokens(tokens.items);
    }
    
    /// Format a list of tokens
    fn formatTokens(self: *CursedFormatter, tokens: []const lexer.Token) !void {
        var i: usize = 0;
        while (i < tokens.len) : (i += 1) {
            const token = tokens[i];
            const next_token = if (i + 1 < tokens.len) tokens[i + 1] else null;
            const prev_token = if (i > 0) tokens[i - 1] else null;
            
            try self.formatToken(token, prev_token, next_token);
        }
        
        // Ensure file ends with newline
        if (self.output.items.len > 0 and self.output.items[self.output.items.len - 1] != '\n') {
            try self.output.append('\n');
        }
    }
    
    /// Format a single token
    fn formatToken(self: *CursedFormatter, token: lexer.Token, prev_token: ?lexer.Token, next_token: ?lexer.Token) !void {
        // Skip whitespace and comments in original source (we'll add our own)
        if (token.type == .whitespace or token.type == .newline) return;
        
        // Handle different token types
        switch (token.type) {
            .comment => try self.formatComment(token),
            .left_brace => try self.formatLeftBrace(token, prev_token),
            .right_brace => try self.formatRightBrace(token, next_token),
            .semicolon => try self.formatSemicolon(token, next_token),
            .comma => try self.formatComma(token, next_token),
            .left_paren => try self.formatLeftParen(token, prev_token),
            .right_paren => try self.formatRightParen(token, next_token),
            .assign, .plus_assign, .minus_assign, .multiply_assign, .divide_assign => {
                try self.formatAssignmentOperator(token);
            },
            .equal, .not_equal, .less_than, .less_equal, .greater_than, .greater_equal => {
                try self.formatComparisonOperator(token);
            },
            .plus, .minus, .multiply, .divide, .modulo => {
                try self.formatArithmeticOperator(token, prev_token, next_token);
            },
            .sus => try self.formatVariableDeclaration(token),
            .slay => try self.formatFunctionDeclaration(token),
            .ready => try self.formatIfStatement(token),
            .otherwise => try self.formatElseStatement(token),
            .bestie => try self.formatWhileLoop(token),
            .squad => try self.formatStructDeclaration(token),
            .sick => try self.formatPatternMatch(token),
            .when => try self.formatPatternCase(token),
            .yeet => try self.formatImport(token),
            else => try self.formatGenericToken(token),
        }
    }
    
    /// Format comments
    fn formatComment(self: *CursedFormatter, token: lexer.Token) !void {
        // Preserve comment indentation relative to surrounding code
        try self.writeIndent();
        try self.writeToken(token.value);
        try self.writeNewline();
    }
    
    /// Format opening braces
    fn formatLeftBrace(self: *CursedFormatter, token: lexer.Token, prev_token: ?lexer.Token) !void {
        _ = prev_token;
        
        if (self.config.newline_before_brace) {
            try self.writeNewline();
            try self.writeIndent();
        } else {
            try self.writeSpace();
        }
        
        try self.writeToken(token.value);
        try self.writeNewline();
        self.indent_level += 1;
    }
    
    /// Format closing braces
    fn formatRightBrace(self: *CursedFormatter, token: lexer.Token, next_token: ?lexer.Token) !void {
        self.indent_level = if (self.indent_level > 0) self.indent_level - 1 else 0;
        try self.writeIndent();
        try self.writeToken(token.value);
        
        // Add newline unless next token is else-like
        if (next_token) |next| {
            if (next.type != .otherwise) {
                try self.writeNewline();
            }
        } else {
            try self.writeNewline();
        }
    }
    
    /// Format semicolons
    fn formatSemicolon(self: *CursedFormatter, token: lexer.Token, next_token: ?lexer.Token) !void {
        try self.writeToken(token.value);
        
        // Add newline unless it's in a for loop
        if (next_token) |next| {
            if (next.type != .semicolon) { // Not part of for loop
                try self.writeNewline();
            } else {
                try self.writeSpace();
            }
        } else {
            try self.writeNewline();
        }
    }
    
    /// Format commas
    fn formatComma(self: *CursedFormatter, token: lexer.Token, next_token: ?lexer.Token) !void {
        try self.writeToken(token.value);
        
        if (self.config.space_after_comma and next_token != null) {
            try self.writeSpace();
        }
    }
    
    /// Format left parentheses
    fn formatLeftParen(self: *CursedFormatter, token: lexer.Token, prev_token: ?lexer.Token) !void {
        // Add space before paren for function calls if configured
        if (self.config.space_before_paren and prev_token != null) {
            const prev = prev_token.?;
            if (prev.type == .identifier) {
                try self.writeSpace();
            }
        }
        
        try self.writeToken(token.value);
    }
    
    /// Format right parentheses
    fn formatRightParen(self: *CursedFormatter, token: lexer.Token, next_token: ?lexer.Token) !void {
        try self.writeToken(token.value);
        
        // Add space after paren in certain contexts
        if (next_token) |next| {
            if (next.type == .left_brace) {
                if (!self.config.newline_before_brace) {
                    try self.writeSpace();
                }
            }
        }
    }
    
    /// Format assignment operators
    fn formatAssignmentOperator(self: *CursedFormatter, token: lexer.Token) !void {
        if (self.config.space_around_operators) {
            try self.writeSpace();
        }
        try self.writeToken(token.value);
        if (self.config.space_around_operators) {
            try self.writeSpace();
        }
    }
    
    /// Format comparison operators
    fn formatComparisonOperator(self: *CursedFormatter, token: lexer.Token) !void {
        if (self.config.space_around_operators) {
            try self.writeSpace();
        }
        try self.writeToken(token.value);
        if (self.config.space_around_operators) {
            try self.writeSpace();
        }
    }
    
    /// Format arithmetic operators
    fn formatArithmeticOperator(self: *CursedFormatter, token: lexer.Token, prev_token: ?lexer.Token, next_token: ?lexer.Token) !void {
        // Handle unary operators differently
        const is_unary = prev_token == null or 
            (prev_token.?.type == .left_paren or 
             prev_token.?.type == .comma or 
             prev_token.?.type == .assign);
        
        if (!is_unary and self.config.space_around_operators) {
            try self.writeSpace();
        }
        
        try self.writeToken(token.value);
        
        if (next_token != null and self.config.space_around_operators) {
            try self.writeSpace();
        }
    }
    
    /// Format variable declarations
    fn formatVariableDeclaration(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeIndent();
        try self.writeToken(token.value);
        try self.writeSpace();
    }
    
    /// Format function declarations
    fn formatFunctionDeclaration(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeIndent();
        try self.writeToken(token.value);
        try self.writeSpace();
        self.in_function = true;
    }
    
    /// Format if statements
    fn formatIfStatement(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeIndent();
        try self.writeToken(token.value);
        try self.writeSpace();
    }
    
    /// Format else statements
    fn formatElseStatement(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeSpace();
        try self.writeToken(token.value);
        try self.writeSpace();
    }
    
    /// Format while loops
    fn formatWhileLoop(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeIndent();
        try self.writeToken(token.value);
        try self.writeSpace();
    }
    
    /// Format struct declarations
    fn formatStructDeclaration(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeIndent();
        try self.writeToken(token.value);
        try self.writeSpace();
        self.in_struct = true;
    }
    
    /// Format pattern matching
    fn formatPatternMatch(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeIndent();
        try self.writeToken(token.value);
        try self.writeSpace();
    }
    
    /// Format pattern cases
    fn formatPatternCase(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeIndent();
        try self.writeToken(token.value);
        try self.writeSpace();
    }
    
    /// Format import statements
    fn formatImport(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeIndent();
        try self.writeToken(token.value);
        try self.writeSpace();
    }
    
    /// Format generic tokens
    fn formatGenericToken(self: *CursedFormatter, token: lexer.Token) !void {
        try self.writeToken(token.value);
    }
    
    /// Write indentation
    fn writeIndent(self: *CursedFormatter) !void {
        const indent_chars = if (self.config.use_tabs) "\t" else " ";
        const indent_size = if (self.config.use_tabs) 1 else self.config.indent_size;
        
        var i: u32 = 0;
        while (i < self.indent_level * indent_size) : (i += 1) {
            try self.output.appendSlice(indent_chars);
        }
        
        self.current_column += self.indent_level * indent_size;
    }
    
    /// Write a token value
    fn writeToken(self: *CursedFormatter, value: []const u8) !void {
        try self.output.appendSlice(value);
        self.current_column += @as(u32, @intCast(value.len));
    }
    
    /// Write a space
    fn writeSpace(self: *CursedFormatter) !void {
        try self.output.append(' ');
        self.current_column += 1;
    }
    
    /// Write a newline
    fn writeNewline(self: *CursedFormatter) !void {
        try self.output.append('\n');
        self.current_line += 1;
        self.current_column = 1;
    }
    
    /// Check if line is too long and needs wrapping
    fn needsLineWrap(self: *CursedFormatter) bool {
        return self.current_column > self.config.max_line_length;
    }
    
    /// Advanced formatting for complex structures
    fn formatComplexStructure(self: *CursedFormatter, tokens: []const lexer.Token, start: usize, end: usize) !void {
        // Handle function parameters, array literals, etc.
        var i = start;
        var depth: u32 = 0;
        var needs_multiline = false;
        
        // First pass: determine if multiline formatting is needed
        while (i <= end) : (i += 1) {
            const token = tokens[i];
            switch (token.type) {
                .left_paren, .left_bracket, .left_brace => depth += 1,
                .right_paren, .right_bracket, .right_brace => depth -= 1,
                .comma => if (depth == 1) needs_multiline = true,
                else => {},
            }
        }
        
        // Check line length
        if (self.current_column > self.config.max_line_length / 2) {
            needs_multiline = true;
        }
        
        // Format accordingly
        i = start;
        while (i <= end) : (i += 1) {
            const token = tokens[i];
            const next_token = if (i + 1 < tokens.len) tokens[i + 1] else null;
            
            switch (token.type) {
                .comma => {
                    try self.writeToken(token.value);
                    if (needs_multiline) {
                        try self.writeNewline();
                        try self.writeIndent();
                    } else if (self.config.space_after_comma) {
                        try self.writeSpace();
                    }
                },
                else => try self.formatToken(token, null, next_token),
            }
        }
    }
    
    /// Align assignments in variable declarations
    fn alignAssignments(self: *CursedFormatter, tokens: []const lexer.Token) !void {
        if (!self.config.align_assignments) return;
        
        // Find all assignment positions in consecutive variable declarations
        var assignments = ArrayList(u32).init(self.allocator);
        defer assignments.deinit();
        
        var i: usize = 0;
        while (i < tokens.len) : (i += 1) {
            const token = tokens[i];
            if (token.type == .sus) {
                // Look for assignment in this declaration
                var j = i + 1;
                while (j < tokens.len and tokens[j].type != .semicolon and tokens[j].type != .newline) : (j += 1) {
                    if (tokens[j].type == .assign) {
                        try assignments.append(@as(u32, @intCast(j)));
                        break;
                    }
                }
            }
        }
        
        // Calculate alignment position
        if (assignments.items.len > 1) {
            // TODO: Implement assignment alignment logic
        }
    }
};

/// Command line interface
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("CURSED Formatter v1.0.0\n");
        print("Usage: cursed-fmt [options] <file.csd>\n");
        print("\nOptions:\n");
        print("  --config <file>       Use configuration file\n");
        print("  --output <file>       Output to specific file (default: overwrite input)\n");
        print("  --check               Check if file is formatted (exit 1 if not)\n");
        print("  --diff                Show diff instead of writing\n");
        print("  --indent-size <n>     Indentation size (default: 4)\n");
        print("  --use-tabs            Use tabs instead of spaces\n");
        print("  --max-line-length <n> Maximum line length (default: 100)\n");
        print("  --help                Show this help\n");
        print("\nExamples:\n");
        print("  cursed-fmt main.csd                    # Format in place\n");
        print("  cursed-fmt --output formatted.csd main.csd  # Format to new file\n");
        print("  cursed-fmt --check main.csd           # Check if formatted\n");
        return;
    }
    
    var config = FormatConfig{};
    var input_file: ?[]const u8 = null;
    var output_file: ?[]const u8 = null;
    var check_mode = false;
    var diff_mode = false;
    
    var i: usize = 1;
    while (i < args.len) : (i += 1) {
        if (std.mem.eql(u8, args[i], "--help")) {
            // Help already printed above
            return;
        } else if (std.mem.eql(u8, args[i], "--config")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --config requires a file path\n");
                return;
            }
            config = FormatConfig.fromFile(allocator, args[i]) catch |err| {
                print("Error reading config file: {}\n", .{err});
                return;
            };
        } else if (std.mem.eql(u8, args[i], "--output")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --output requires a file path\n");
                return;
            }
            output_file = args[i];
        } else if (std.mem.eql(u8, args[i], "--check")) {
            check_mode = true;
        } else if (std.mem.eql(u8, args[i], "--diff")) {
            diff_mode = true;
        } else if (std.mem.eql(u8, args[i], "--indent-size")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --indent-size requires a number\n");
                return;
            }
            config.indent_size = std.fmt.parseInt(u32, args[i], 10) catch |err| {
                print("Error parsing indent size: {}\n", .{err});
                return;
            };
        } else if (std.mem.eql(u8, args[i], "--use-tabs")) {
            config.use_tabs = true;
        } else if (std.mem.eql(u8, args[i], "--max-line-length")) {
            i += 1;
            if (i >= args.len) {
                print("Error: --max-line-length requires a number\n");
                return;
            }
            config.max_line_length = std.fmt.parseInt(u32, args[i], 10) catch |err| {
                print("Error parsing max line length: {}\n", .{err});
                return;
            };
        } else if (!std.mem.startsWith(u8, args[i], "--")) {
            input_file = args[i];
        } else {
            print("Error: Unknown option '{s}'\n", .{args[i]});
            return;
        }
    }
    
    if (input_file == null) {
        print("Error: No input file specified\n");
        return;
    }
    
    var formatter = CursedFormatter.init(allocator, config);
    defer formatter.deinit();
    
    if (check_mode) {
        // Check if file is already formatted
        const original = try std.fs.cwd().readFileAlloc(allocator, input_file.?, 1024 * 1024);
        defer allocator.free(original);
        
        const formatted = try formatter.formatString(original);
        defer allocator.free(formatted);
        
        if (std.mem.eql(u8, original, formatted)) {
            print("File is already formatted.\n");
        } else {
            print("File is not formatted.\n");
            std.process.exit(1);
        }
    } else if (diff_mode) {
        // Show diff between original and formatted
        const original = try std.fs.cwd().readFileAlloc(allocator, input_file.?, 1024 * 1024);
        defer allocator.free(original);
        
        const formatted = try formatter.formatString(original);
        defer allocator.free(formatted);
        
        if (std.mem.eql(u8, original, formatted)) {
            print("No changes needed.\n");
        } else {
            print("--- {s}\n", .{input_file.?});
            print("+++ {s} (formatted)\n", .{input_file.?});
            // TODO: Implement actual diff output
            print("Formatted version would differ from original.\n");
        }
    } else {
        // Format the file
        try formatter.formatFile(input_file.?, output_file);
        
        if (output_file) |out| {
            print("Formatted {s} -> {s}\n", .{ input_file.?, out });
        } else {
            print("Formatted {s}\n", .{input_file.?});
        }
    }
}
