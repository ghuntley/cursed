// CURSED Code Formatter
// Implements consistent formatting rules for CURSED syntax

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Import CURSED compiler components
const lexer = @import("../lexer.zig");
const ast = @import("../ast.zig");

// Formatting Configuration
pub const FormatterConfig = struct {
    indent_size: u32 = 4,
    max_line_length: u32 = 100,
    use_spaces: bool = true,
    newline_before_brace: bool = false,
    space_around_operators: bool = true,
    align_struct_fields: bool = true,
    sort_imports: bool = true,
    
    // CURSED-specific settings
    align_gen_z_keywords: bool = true,
    prefer_short_form_syntax: bool = true,
    max_chained_calls: u32 = 3,
};

// Formatting Context
const FormattingContext = struct {
    config: FormatterConfig,
    current_indent: u32 = 0,
    in_function_params: bool = false,
    in_struct_definition: bool = false,
    in_interface_definition: bool = false,
    line_length: u32 = 0,
    
    fn getIndent(self: *const FormattingContext, allocator: Allocator) ![]const u8 {
        const indent_size = if (self.config.use_spaces) self.config.indent_size else 1;
        const total_indent = self.current_indent * indent_size;
        
        const indent = try allocator.alloc(u8, total_indent);
        if (self.config.use_spaces) {
            for (indent) |*char| {
                char.* = ' ';
            }
        } else {
            for (indent) |*char| {
                char.* = '\t';
            }
        }
        return indent;
    }
};

// CURSED Code Formatter
pub const Formatter = struct {
    allocator: Allocator,
    config: FormatterConfig,
    output: ArrayList(u8),
    
    pub fn init(allocator: Allocator, config: FormatterConfig) Formatter {
        return Formatter{
            .allocator = allocator,
            .config = config,
            .output = ArrayList(u8).init(allocator),
        };
    }
    
    pub fn deinit(self: *Formatter) void {
        self.output.deinit();
    }
    
    pub fn format(self: *Formatter, source: []const u8) ![]const u8 {
        // Clear previous output
        self.output.clearRetainingCapacity();
        
        // Tokenize source with error recovery
        var token_lexer = lexer.Lexer.init(self.allocator, source);
        
        const tokens = token_lexer.tokenize() catch |err| {
            // Attempt error recovery for common syntax errors
            switch (err) {
                error.UnexpectedCharacter, 
                error.UnterminatedString, 
                error.UnterminatedChar, 
                error.UnterminatedBlockComment => {
                    return self.formatWithErrorRecovery(source, err);
                },
                else => return err,
            }
        };
        defer tokens.deinit();
        
        // Format tokens
        var context = FormattingContext{ .config = self.config };
        try self.formatTokens(tokens.items, &context);
        
        return try self.output.toOwnedSlice();
    }
    
    fn formatTokens(self: *Formatter, tokens: []const lexer.Token, context: *FormattingContext) !void {
        var i: usize = 0;
        var at_line_start = true;
        
        while (i < tokens.len) {
            const token = tokens[i];
            
            // Add indentation at line start
            if (at_line_start and token.kind != .Newline) {
                const indent = try context.getIndent(self.allocator);
                defer self.allocator.free(indent);
                try self.output.appendSlice(indent);
                context.line_length = @as(u32, @intCast(indent.len));
                at_line_start = false;
            }
            
            switch (token.kind) {
                .Slay, .Sus, .Facts, .Lowkey, .Highkey, .Periodt, .Stan, .Bestie, .Squad, .Collab, .Yeet, .Later, .Normie, .Drip, .Tea, .Lit, .Smol, .Thicc, .Meal, .Spill => {
                    try self.formatKeyword(token, context);
                    at_line_start = false;
                },
                .Identifier => {
                    try self.formatIdentifier(token, context);
                    at_line_start = false;
                },
                .LeftBrace => {
                    try self.formatLeftBrace(token, context);
                    at_line_start = true; // Next line starts after brace
                },
                .RightBrace => {
                    try self.formatRightBrace(token, context);
                    at_line_start = true;
                },
                .LeftParen => {
                    try self.formatLeftParen(token, context);
                    at_line_start = false;
                },
                .RightParen => {
                    try self.formatRightParen(token, context);
                    at_line_start = false;
                },
                .Semicolon => {
                    try self.formatSemicolon(token, context);
                    at_line_start = true; // Semicolon forces new line
                },
                .Comma => {
                    try self.formatComma(token, context);
                    at_line_start = !context.in_function_params; // Function params stay on same line
                },
                .Equal, .Plus, .Minus, .Star, .Slash, .Percent, .Dot, .BeLike => {
                    try self.formatOperator(token, context);
                    at_line_start = false;
                },
                .StringLiteral, .String => {
                    try self.formatString(token, context);
                    at_line_start = false;
                },
                .Number, .Integer => {
                    try self.formatNumber(token, context);
                    at_line_start = false;
                },
                .Comment => {
                    try self.formatComment(token, context);
                    at_line_start = true; // Comments force new line
                },
                .Newline => {
                    // Skip extra newlines, we handle formatting ourselves
                    at_line_start = true;
                },
                else => {
                    try self.formatDefault(token, context);
                    at_line_start = false;
                },
            }
            
            i += 1;
        }
        
        // Ensure file ends with newline
        if (self.output.items.len > 0 and self.output.items[self.output.items.len - 1] != '\n') {
            try self.output.append('\n');
        }
    }
    
    fn formatKeyword(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        const keyword = token.lexeme;
        
        // Handle function declarations
        if (std.mem.eql(u8, keyword, "slay")) {
            try self.output.appendSlice("slay ");
            context.line_length += 5;
        }
        // Handle variable declarations
        else if (std.mem.eql(u8, keyword, "sus")) {
            try self.output.appendSlice("sus ");
            context.line_length += 4;
        }
        // Handle returns
        else if (std.mem.eql(u8, keyword, "yolo")) {
            try self.output.appendSlice("yolo ");
            context.line_length += 5;
        }
        // Handle loops
        else if (std.mem.eql(u8, keyword, "bestie")) {
            try self.output.appendSlice("bestie ");
            context.line_length += 7;
        }
        // Handle conditionals
        else if (std.mem.eql(u8, keyword, "lowkey")) {
            try self.output.appendSlice("lowkey ");
            context.line_length += 7;
        }
        // Handle goroutines
        else if (std.mem.eql(u8, keyword, "stan")) {
            try self.output.appendSlice("stan ");
            context.line_length += 5;
        }
        // Handle interfaces
        else if (std.mem.eql(u8, keyword, "collab")) {
            try self.output.appendSlice("collab ");
            context.line_length += 7;
            context.in_interface_definition = true;
        }
        // Handle structs
        else if (std.mem.eql(u8, keyword, "squad")) {
            try self.output.appendSlice("squad ");
            context.line_length += 6;
            context.in_struct_definition = true;
        }
        // Handle implementations
        else if (std.mem.eql(u8, keyword, "flex")) {
            try self.output.appendSlice("flex ");
            context.line_length += 5;
        }
        // Default keyword formatting
        else {
            try self.output.appendSlice(keyword);
            try self.output.append(' ');
            context.line_length += @as(u32, @intCast(keyword.len + 1));
        }
    }
    
    fn formatIdentifier(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        // Add space before identifier if needed
        if (self.output.items.len > 0) {
            const last_char = self.output.items[self.output.items.len - 1];
            if (last_char != ' ' and last_char != '\n' and last_char != '(' and last_char != '{') {
                try self.output.append(' ');
                context.line_length += 1;
            }
        }
        
        try self.output.appendSlice(token.lexeme);
        context.line_length += @as(u32, @intCast(token.lexeme.len));
    }
    
    fn formatLeftBrace(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        _ = token;
        if (context.config.newline_before_brace) {
            try self.output.appendSlice("\n");
            const indent = try context.getIndent(self.allocator);
            defer self.allocator.free(indent);
            try self.output.appendSlice(indent);
            context.line_length = @as(u32, @intCast(indent.len));
        } else {
            if (context.line_length > 0 and self.output.items[self.output.items.len - 1] != ' ') {
                try self.output.append(' ');
                context.line_length += 1;
            }
        }
        
        try self.output.append('{');
        context.line_length += 1;
        context.current_indent += 1;
        
        // Add newline after opening brace
        try self.output.append('\n');
        context.line_length = 0;
    }
    
    fn formatRightBrace(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        _ = token;
        context.current_indent -= 1;
        
        // Ensure we're on a new line
        if (context.line_length > 0) {
            try self.output.append('\n');
        }
        
        const indent = try context.getIndent(self.allocator);
        defer self.allocator.free(indent);
        try self.output.appendSlice(indent);
        try self.output.append('}');
        
        context.line_length = @as(u32, @intCast(indent.len + 1));
        context.in_struct_definition = false;
        context.in_interface_definition = false;
    }
    
    fn formatLeftParen(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        _ = token;
        try self.output.append('(');
        context.line_length += 1;
        context.in_function_params = true;
    }
    
    fn formatRightParen(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        _ = token;
        try self.output.append(')');
        context.line_length += 1;
        context.in_function_params = false;
    }
    
    fn formatSemicolon(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        _ = token;
        // CURSED doesn't typically use semicolons, but handle them if present
        // Replace semicolon with newline for better formatting
        try self.output.append('\n');
        context.line_length = 0;
    }
    
    fn formatComma(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        _ = token;
        try self.output.append(',');
        
        if (context.in_function_params) {
            try self.output.append(' ');
            context.line_length += 2;
        } else {
            try self.output.append('\n');
            const indent = try context.getIndent(self.allocator);
            defer self.allocator.free(indent);
            try self.output.appendSlice(indent);
            context.line_length = @as(u32, @intCast(indent.len));
        }
    }
    
    fn formatOperator(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        const operator = token.lexeme;
        
        if (context.config.space_around_operators) {
            // Add space before operator
            if (self.output.items.len > 0 and self.output.items[self.output.items.len - 1] != ' ') {
                try self.output.append(' ');
                context.line_length += 1;
            }
            
            try self.output.appendSlice(operator);
            try self.output.append(' ');
            context.line_length += @as(u32, @intCast(operator.len + 1));
        } else {
            try self.output.appendSlice(operator);
            context.line_length += @as(u32, @intCast(operator.len));
        }
    }
    
    fn formatString(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        try self.output.appendSlice(token.lexeme);
        context.line_length += @as(u32, @intCast(token.lexeme.len));
    }
    
    fn formatNumber(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        try self.output.appendSlice(token.lexeme);
        context.line_length += @as(u32, @intCast(token.lexeme.len));
    }
    
    fn formatComment(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        // Handle both comment styles
        if (std.mem.startsWith(u8, token.lexeme, "fr fr")) {
            try self.output.appendSlice("fr fr ");
            try self.output.appendSlice(token.lexeme[5..]);
        } else if (std.mem.startsWith(u8, token.lexeme, "#")) {
            try self.output.appendSlice("# ");
            try self.output.appendSlice(token.lexeme[1..]);
        } else {
            try self.output.appendSlice(token.lexeme);
        }
        
        try self.output.append('\n');
        context.line_length = 0;
    }
    
    fn formatNewline(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        _ = token;
        try self.output.append('\n');
        context.line_length = 0;
    }
    
    fn formatDefault(self: *Formatter, token: lexer.Token, context: *FormattingContext) !void {
        try self.output.appendSlice(token.lexeme);
        context.line_length += @as(u32, @intCast(token.lexeme.len));
    }
    
    // Check if line needs breaking
    fn needsLineBreak(self: *Formatter, context: *FormattingContext) bool {
        _ = self;
        return context.line_length > context.config.max_line_length;
    }
    
    // Error recovery formatting - attempt to format malformed code
    fn formatWithErrorRecovery(self: *Formatter, source: []const u8, original_error: anyerror) ![]const u8 {
        // Clear output for recovery attempt
        self.output.clearRetainingCapacity();
        
        // Add error comment at the top
        try self.output.appendSlice("# CURSED Formatter: Partial formatting due to syntax errors\n");
        try self.output.appendSlice("# Original error: ");
        try self.output.appendSlice(@errorName(original_error));
        try self.output.appendSlice("\n\n");
        
        // Attempt line-by-line formatting for recoverable content
        var lines = std.mem.splitScalar(u8, source, '\n');
        var line_num: usize = 0;
        
        while (lines.next()) |line| {
            line_num += 1;
            const trimmed = std.mem.trim(u8, line, " \t\r");
            
            if (trimmed.len == 0) {
                // Preserve empty lines
                try self.output.append('\n');
                continue;
            }
            
            // Try to format individual lines that look like valid CURSED syntax
            if (self.isLineFormattable(trimmed)) {
                try self.formatSingleLine(trimmed);
            } else {
                // Preserve problematic lines with a comment
                try self.output.appendSlice("# Line ");
                const line_str = try std.fmt.allocPrint(self.allocator, "{d}", .{line_num});
                defer self.allocator.free(line_str);
                try self.output.appendSlice(line_str);
                try self.output.appendSlice(" - formatting skipped due to syntax error\n");
                try self.output.appendSlice(line);
            }
            try self.output.append('\n');
        }
        
        // Add footer comment
        try self.output.appendSlice("\n# End of partial formatting\n");
        try self.output.appendSlice("# Please fix syntax errors and re-run formatter\n");
        
        return try self.output.toOwnedSlice();
    }
    
    // Check if a line can be safely formatted
    fn isLineFormattable(self: *Formatter, line: []const u8) bool {
        _ = self;
        
        // Skip lines that are likely to cause parsing errors
        if (std.mem.indexOf(u8, line, "\"") != null and std.mem.count(u8, line, "\"") % 2 != 0) {
            return false; // Unterminated string
        }
        
        if (std.mem.indexOf(u8, line, "'") != null and std.mem.count(u8, line, "'") % 2 != 0) {
            return false; // Unterminated char
        }
        
        // Allow comments and basic statements
        return std.mem.startsWith(u8, line, "#") or 
               std.mem.startsWith(u8, line, "//") or
               std.mem.startsWith(u8, line, "fr fr") or
               std.mem.startsWith(u8, line, "sus") or
               std.mem.startsWith(u8, line, "slay") or
               std.mem.startsWith(u8, line, "yeet") or
               std.mem.startsWith(u8, line, "squad") or
               std.mem.startsWith(u8, line, "collab");
    }
    
    // Format a single line with basic indentation
    fn formatSingleLine(self: *Formatter, line: []const u8) !void {
        // Basic formatting: trim whitespace and add proper spacing
        const trimmed = std.mem.trim(u8, line, " \t");
        
        // Add basic indentation for certain keywords
        if (std.mem.indexOf(u8, trimmed, "{") != null) {
            try self.output.appendSlice("    "); // 4-space indent
        }
        
        try self.output.appendSlice(trimmed);
    }
};

// Formatter CLI Interface
pub fn formatFile(allocator: Allocator, file_path: []const u8, config: FormatterConfig) !void {
    // Read file
    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();
    
    const source = try file.readToEndAlloc(allocator, 1024 * 1024); // 1MB max
    defer allocator.free(source);
    
    // Format code
    var formatter = Formatter.init(allocator, config);
    defer formatter.deinit();
    
    const formatted = try formatter.format(source);
    defer allocator.free(formatted);
    

    // Write back to file
    const output_file = try std.fs.cwd().createFile(file_path, .{});
    defer output_file.close();
    try output_file.writeAll(formatted);
    
    std.log.info("Formatted: {s}", .{file_path});
}

pub fn formatDirectory(allocator: Allocator, dir_path: []const u8, config: FormatterConfig) !void {
    var dir = try std.fs.cwd().openDir(dir_path, .{ .iterate = true });
    defer dir.close();
    
    var iterator = dir.iterate();
    while (try iterator.next()) |entry| {
        if (entry.kind == .file and std.mem.endsWith(u8, entry.name, ".csd")) {
            const full_path = try std.fs.path.join(allocator, &[_][]const u8{ dir_path, entry.name });
            defer allocator.free(full_path);
            
            try formatFile(allocator, full_path, config);
        } else if (entry.kind == .directory) {
            const sub_dir = try std.fs.path.join(allocator, &[_][]const u8{ dir_path, entry.name });
            defer allocator.free(sub_dir);
            
            try formatDirectory(allocator, sub_dir, config);
        }
    }
}

// Main formatter entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-fmt <file or directory>", .{});
        return;
    }
    
    const config = FormatterConfig{};
    const target = args[1];
    
    // Check if target is file or directory
    const stat = std.fs.cwd().statFile(target) catch |err| {
        std.log.err("Error accessing {s}: {}", .{ target, err });
        return;
    };
    
    if (stat.kind == .file) {
        try formatFile(allocator, target, config);
    } else if (stat.kind == .directory) {
        try formatDirectory(allocator, target, config);
    } else {
        std.log.err("{s} is not a file or directory", .{target});
    }
}
