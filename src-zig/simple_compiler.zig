const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");

const VariableInfo = struct {
    name: []const u8,
    var_type: []const u8, // "drip", "tea", "lit", "meal"
};

/// Simple CURSED-to-C compiler that generates and compiles C code
pub fn compileProgram(allocator: Allocator, source: []const u8, filename: []const u8, optimization_level: u8, verbose: bool) !void {
        _ = allocator;
    return compileProgramWithOutput(allocator, source, filename, null, optimization_level, verbose);
}

pub fn compileProgramWithOutput(allocator: Allocator, source: []const u8, filename: []const u8, output_file: ?[]const u8, optimization_level: u8, verbose: bool) !void {
        _ = allocator;
    _ = optimization_level;
    
    if (verbose) print("🔥 Compiling CURSED program to native executable...\n", .{});
    
    // Step 1: Lexical Analysis
    print("[1/5] Lexical Analysis...\n", .{});
    var l = lexer.Lexer.init(allocator, source);
    const tokens = l.tokenize() catch {
        print("❌ Lexer error during compilation: {s}\n", .{err});
        return;
    };
    defer tokens.deinit();
    
    if (verbose) print("📝 Lexed {s} tokens for compilation\n", .{tokens.items.len});
    
    // Step 2: Generate a simple C program
    print("[2/5] Generating C code...\n", .{});
    
    const c_filename = try std.fmt.allocPrint(allocator, "{s}.c", .{filename[0..filename.len - 4]});
    defer allocator.free(c_filename);
    
    const c_file = std.fs.cwd().createFile(c_filename, .{}) catch {
        print("❌ Error creating C file: {s}\n", .{err});
        return;
    };
    defer c_file.close();
    
    var writer = c_file.writer();
    try writer.writer().writeAll("#include <stdio.h>\n");
    try writer.writer().writeAll("#include <stdlib.h>\n");
    try writer.writer().writeAll("#include <string.h>\n");
    try writer.writer().writeAll("int main() {\n");
    
    // Step 3: Simple CURSED-to-C translation
    print("[3/5] Translating CURSED to C...\n", .{});
    
    var variables = std.ArrayList(VariableInfo){};
    defer {
        for (variables.items) |var_info| {
            allocator.free(var_info.name);
            allocator.free(var_info.var_type);
        }
        variables.deinit();
    }
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        // Skip empty lines and comments
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        // Skip imports
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            continue;
        }
        
        // Handle vibez.spill() statements
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            if (std.mem.indexOf(u8, trimmed[start..], "(")) |paren_start| {
                if (std.mem.lastIndexOf(u8, trimmed, ")")) |paren_end| {
                    const content_start = start + paren_start + 1;
                    const content = trimmed[content_start..paren_end];
                    
                    if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                        // String literal
                        try writer.print("    printf({s}\"\\n\");\n", .{content});
                    } else {
                        // Variable reference or literal value
                        
                        // Check if it's a number
                        if (std.fmt.parseInt(i64, content, 10)) |num| {
                            try writer.print("    printf(\"%ld\\n\", {}L);\n", .{num});
                        } else |_| {
                            // Find variable type
                            var var_type: ?[]const u8 = null;
                            for (variables.items) |var_info| {
                                if (std.mem.eql(u8, var_info.name, content)) {
                                    var_type = var_info.var_type;
                                    break;
                                }
                            }
                            
                            if (var_type) |vtype| {
                                if (std.mem.eql(u8, vtype, "drip")) {
                                    try writer.print("    printf(\"%ld\\n\", {s});\n", .{content});
                                } else if (std.mem.eql(u8, vtype, "tea")) {
                                    try writer.print("    printf(\"%s\\n\", {s});\n", .{content});
                                } else if (std.mem.eql(u8, vtype, "lit")) {
                                    try writer.print("    printf(\"%s\\n\", {s} ? \"based\" : \"cringe\");\n", .{content});
                                } else if (std.mem.eql(u8, vtype, "meal")) {
                                    try writer.print("    printf(\"%f\\n\", {s});\n", .{content});
                                }
                            } else {
                                // Unknown variable, treat as string
                                try writer.print("    printf(\"%s\\n\", \"{s}\");\n", .{content});
                            }
                        }
                    }
                }
            }
        }
        
        // Handle simple variable declarations
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            var parts = std.mem.tokenizeScalar(u8, trimmed, ' ');
            _ = parts.next(); // skip "sus"
            
            const var_name = parts.next() orelse continue;
            const var_type = parts.next() orelse continue;
            const equals = parts.next() orelse continue;
            
            if (!std.mem.eql(u8, equals, "=")) continue;
            
            const value_str = parts.rest();
            
            // Add variable to our tracking list
            const var_name_copy = try allocator.dupe(u8, var_name);
            const var_type_copy = try allocator.dupe(u8, var_type);
            try variables.append(VariableInfo{ .name = var_name_copy, .var_type = var_type_copy });
            
            if (std.mem.eql(u8, var_type, "drip")) {
                // Integer type
                try writer.print("    long {s} = {s};\n", .{ var_name, value_str });
            } else if (std.mem.eql(u8, var_type, "tea")) {
                // String type
                if (value_str.len >= 2 and value_str[0] == '"' and value_str[value_str.len - 1] == '"') {
                    try writer.print("    char* {s} = {s};\n", .{ var_name, value_str });
                } else {
                    try writer.print("    char* {s} = \"{s}\";\n", .{ var_name, value_str });
                }
            } else if (std.mem.eql(u8, var_type, "lit")) {
                // Boolean type
                const c_bool = if (std.mem.eql(u8, std.mem.trim(u8, value_str, " \t"), "based")) "1" else "0";
                try writer.print("    int {s} = {s};\n", .{ var_name, c_bool });
            } else if (std.mem.eql(u8, var_type, "meal")) {
                // Float type
                try writer.print("    double {s} = {s};\n", .{ var_name, value_str });
            }
        }
    }
    
    try writer.writer().writeAll("    return 0;\n");
    try writer.writer().writeAll("}\n");
    
    if (verbose) print("✅ Generated C code: {s}\n", .{c_filename});
    
    // Step 4: Compile C code
    print("[4/5] Compiling C to executable...\n", .{});
    
    const output_filename = if (output_file) |custom_output| 
        try allocator.dupe(u8, custom_output)
    else if (std.mem.endsWith(u8, filename, ".💀"))
        try std.fmt.allocPrint(allocator, "{s}", .{filename[0..filename.len - 4]})
    else
        try std.fmt.allocPrint(allocator, "{s}_compiled", .{filename});
    defer allocator.free(output_filename);
    
    var child = std.process.Child.init(&[_][]const u8{ "gcc", "-o", output_filename, c_filename }, allocator);
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;
    
    child.spawn() catch {
        print("❌ Error spawning GCC: {s}\n", .{err});
        return;
    };
    
    const result = child.wait() catch {
        print("❌ Error waiting for GCC: {s}\n", .{err});
        return;
    };
    
    switch (result) {
        .Exited => |code| {
            if (code != 0) {
                print("❌ Compilation failed with exit code: {s}\n", .{code});
                return;
            }
        },
        else => {
            print("❌ Compilation process terminated abnormally\n", .{});
            return;
        },
    }
    
    // Step 5: Cleanup
    print("[5/5] Cleanup...\n", .{});
    std.fs.cwd().deleteFile(c_filename) catch {};
    
    print("✅ Compilation successful!\n", .{});
    print("📦 Output executable: {s}\n", .{output_filename});
    print("🚀 Run with: ./{s}\n", .{output_filename});
}
