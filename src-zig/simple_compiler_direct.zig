const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// Simple Direct Compiler that generates LLVM IR without complex AST parsing
/// This is designed to get basic compilation working quickly
pub const SimpleDirectCompiler = struct {
    allocator: Allocator,
    ir_buffer: ArrayList(u8),
    string_counter: u32,
    
    pub fn init(allocator: Allocator) SimpleDirectCompiler {
        return SimpleDirectCompiler{
            .allocator = allocator,
            .ir_buffer = ArrayList(u8){},
            .string_counter = 0,
        };
    }
    
    pub fn deinit(self: *SimpleDirectCompiler) void {
        self.ir_buffer.deinit(self.allocator);
    }
    
    /// Compile CURSED source to LLVM IR using simple line-by-line approach
    pub fn compileToLLVMIR(self: *SimpleDirectCompiler, source: []const u8, verbose: bool) !void {
        const writer = self.ir_buffer.writer(self.allocator);
        
        if (verbose) print("🔧 Generating LLVM IR header...\n", .{});
        
        // LLVM IR header
        try writer.writeAll("; ModuleID = 'cursed_program'\n");
        try writer.writeAll("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        try writer.writeAll("target triple = \"x86_64-pc-linux-gnu\"\n\n");
        
        // String constants section
        try writer.writeAll("; String constants\n");
        
        // Main function header
        try writer.writeAll("\n; Main function\n");
        try writer.writeAll("define i32 @main() {\n");
        try writer.writeAll("entry:\n");
        
        if (verbose) print("🔧 Processing CURSED source line by line...\n", .{});
        
        // First pass: collect all variable declarations
        var lines_first_pass = std.mem.splitScalar(u8, source, '\n');
        while (lines_first_pass.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (trimmed.len == 0) continue;
            
            // Handle variable declarations first
            if (std.mem.startsWith(u8, trimmed, "sus ")) {
                try self.compileSusStatement(writer, trimmed, verbose);
            }
        }
        
        // Second pass: process all other statements
        var lines = std.mem.splitScalar(u8, source, '\n');
        var line_num: u32 = 0;
        
        while (lines.next()) |line| {
            line_num += 1;
            const trimmed = std.mem.trim(u8, line, " \t\r\n");
            
            // Skip empty lines and comments
            if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
                continue;
            }
            
            if (verbose) print("  Processing line {}: {s}\n", .{ line_num, trimmed });
            
            // Skip imports and variable declarations (already processed)
            if (std.mem.startsWith(u8, trimmed, "yeet ") or std.mem.startsWith(u8, trimmed, "sus ")) {
                continue;
            }
            
            // Handle print statements: vibez.spill("Hello")
            if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |_| {
                try self.compileVibesSpill(writer, trimmed, verbose);
                continue;
            }
        }
        
        // Function footer
        try writer.writeAll("  ret i32 0\n");
        try writer.writeAll("}\n\n");
        
        // Add printf declaration
        try writer.writeAll("; External function declarations\n");
        try writer.writeAll("declare i32 @printf(i8*, ...)\n");
        
        if (verbose) print("✅ LLVM IR generation completed\n", .{});
    }
    
    fn compileSusStatement(self: *SimpleDirectCompiler, writer: anytype, line: []const u8, verbose: bool) !void {
        _ = self;
        // Parse: sus x drip = 42
        const equals_pos = std.mem.indexOf(u8, line, "=") orelse return;
        const decl_part = std.mem.trim(u8, line[0..equals_pos], " \t");
        const value_str = std.mem.trim(u8, line[equals_pos + 1..], " \t");
        
        // Extract variable name - skip "sus" and type
        var parts = std.mem.tokenizeScalar(u8, decl_part[4..], ' '); // Skip "sus "
        const var_name = parts.next() orelse return;
        
        if (verbose) print("    Variable: {s} = {s}\n", .{ var_name, value_str });
        
        // Simple integer allocation and assignment
        try writer.print("  %{s} = alloca i64, align 8\n", .{var_name});
        
        // Parse value - simple integer for now
        if (std.fmt.parseInt(i64, value_str, 10)) |int_val| {
            try writer.print("  store i64 {}, i64* %{s}, align 8\n", .{ int_val, var_name });
        } else |_| {
            // Default to 0 for complex expressions
            try writer.print("  store i64 0, i64* %{s}, align 8\n", .{var_name});
        }
    }
    
    fn compileVibesSpill(self: *SimpleDirectCompiler, writer: anytype, line: []const u8, verbose: bool) !void {
        // Extract content inside parentheses
        const start = std.mem.indexOf(u8, line, "(") orelse return;
        const end = std.mem.lastIndexOf(u8, line, ")") orelse return;
        const content = line[start + 1..end];
        const trimmed_content = std.mem.trim(u8, content, " \t");
        
        if (verbose) print("    Print: {s}\n", .{trimmed_content});
        
        // Parse arguments - handle multiple comma-separated arguments
        var arguments = ArrayList([]const u8){};
        defer arguments.deinit(self.allocator);
        
        try self.parseArguments(trimmed_content, &arguments);
        
        if (arguments.items.len == 0) {
            return; // No arguments to process
        }
        
        // Handle single string literal case
        if (arguments.items.len == 1) {
            const arg = std.mem.trim(u8, arguments.items[0], " \t");
            if (arg.len >= 2 and arg[0] == '"' and arg[arg.len - 1] == '"') {
                try self.generateSingleStringPrint(writer, arg[1..arg.len - 1], verbose);
                return;
            }
        }
        
        // Handle multi-argument case - generate format string and arguments
        try self.generateMultiArgumentPrint(writer, arguments.items, verbose);
    }
    
    fn parseArguments(self: *SimpleDirectCompiler, content: []const u8, arguments: *ArrayList([]const u8)) !void {
        var in_quotes = false;
        var paren_depth: i32 = 0;
        var start: usize = 0;
        
        for (content, 0..) |char, i| {
            switch (char) {
                '"' => in_quotes = !in_quotes,
                '(' => {
                    if (!in_quotes) paren_depth += 1;
                },
                ')' => {
                    if (!in_quotes) paren_depth -= 1;
                },
                ',' => {
                    if (!in_quotes and paren_depth == 0) {
                        const arg = std.mem.trim(u8, content[start..i], " \t");
                        if (arg.len > 0) {
                            try arguments.append(self.allocator, arg);
                        }
                        start = i + 1;
                    }
                },
                else => {},
            }
        }
        
        // Add the last argument
        const last_arg = std.mem.trim(u8, content[start..], " \t");
        if (last_arg.len > 0) {
            try arguments.append(self.allocator, last_arg);
        }
    }
    
    fn generateSingleStringPrint(self: *SimpleDirectCompiler, writer: anytype, str_content: []const u8, verbose: bool) !void {
        _ = verbose;
        
        // Create string constant
        const str_name = try std.fmt.allocPrint(self.allocator, "str.{}", .{self.string_counter});
        defer self.allocator.free(str_name);
        self.string_counter += 1;
        
        // Generate printf call
        try writer.writeAll("  ; String constant for printf\n");
        try writer.print("  %call{} = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @{s}, i32 0, i32 0))\n",
            .{ self.string_counter, str_content.len + 1, str_content.len + 1, str_name }
        );
        
        // Add the global string constant
        try self.addGlobalString(str_name, str_content);
    }
    
    fn generateMultiArgumentPrint(self: *SimpleDirectCompiler, writer: anytype, arguments: []const []const u8, verbose: bool) !void {
        _ = verbose;
        
        // Build format string by processing each argument
        var format_parts = ArrayList(u8){};
        defer format_parts.deinit(self.allocator);
        
        var printf_args = ArrayList([]const u8){};
        defer printf_args.deinit(self.allocator);
        
        for (arguments) |arg| {
            const trimmed_arg = std.mem.trim(u8, arg, " \t");
            
            if (trimmed_arg.len >= 2 and trimmed_arg[0] == '"' and trimmed_arg[trimmed_arg.len - 1] == '"') {
                // String literal - add directly to format string
                const str_content = trimmed_arg[1..trimmed_arg.len - 1];
                try format_parts.appendSlice(self.allocator, str_content);
            } else {
                // Variable - add %s placeholder and the variable to arguments
                try format_parts.appendSlice(self.allocator, "%s");
                try printf_args.append(self.allocator, trimmed_arg);
            }
        }
        
        const format_string = format_parts.items;
        
        // Create string constant for format string
        const str_name = try std.fmt.allocPrint(self.allocator, "str.{}", .{self.string_counter});
        defer self.allocator.free(str_name);
        self.string_counter += 1;
        
        // Add the global string constant
        try self.addGlobalString(str_name, format_string);
        
        // Generate printf call
        try writer.writeAll("  ; Multi-argument printf call\n");
        
        if (printf_args.items.len == 0) {
            // No variable arguments, just format string
            try writer.print("  %call{} = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @{s}, i32 0, i32 0))\n",
                .{ self.string_counter, format_string.len + 1, format_string.len + 1, str_name }
            );
        } else {
            // Has variable arguments - for now, treat all variables as integers (simplified)
            var printf_call = ArrayList(u8){};
            defer printf_call.deinit(self.allocator);
            
            const call_writer = printf_call.writer(self.allocator);
            try call_writer.print("  %call{} = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @{s}, i32 0, i32 0)",
                .{ self.string_counter, format_string.len + 1, format_string.len + 1, str_name }
            );
            
            // Add variable arguments - simplified: treat all as i64 for now
            for (printf_args.items) |var_name| {
                try call_writer.print(", i64 %{s}", .{var_name});
            }
            
            try call_writer.writeAll(")\n");
            try writer.writeAll(printf_call.items);
        }
    }
    
    fn addGlobalString(self: *SimpleDirectCompiler, name: []const u8, content: []const u8) !void {
        // This is a simplified approach - in a real compiler you'd insert at the right place
        const global_def = try std.fmt.allocPrint(self.allocator, 
            "@{s} = private unnamed_addr constant [{} x i8] c\"{s}\\00\", align 1\n",
            .{ name, content.len + 1, content }
        );
        defer self.allocator.free(global_def);
        
        // Insert near the beginning of the IR (after target info)
        const insert_point = self.findTargetEndPoint();
        try self.ir_buffer.insertSlice(self.allocator, insert_point, global_def);
    }
    
    fn findTargetEndPoint(self: *SimpleDirectCompiler) usize {
        const content = self.ir_buffer.items;
        // Find end of target triple line
        if (std.mem.indexOf(u8, content, "target triple")) |pos| {
            if (std.mem.indexOf(u8, content[pos..], "\n")) |newline| {
                return pos + newline + 1;
            }
        }
        return 0;
    }
    
    /// Write generated IR to file
    pub fn writeToFile(self: *SimpleDirectCompiler, filename: []const u8) !void {
        var file = try std.fs.cwd().createFile(filename, .{});
        defer file.close();
        try file.writeAll(self.ir_buffer.items);
    }
    
    /// Get the generated IR as string
    pub fn getIR(self: *SimpleDirectCompiler) []const u8 {
        return self.ir_buffer.items;
    }
};
