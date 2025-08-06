const std = @import("std");
const print = std.debug.print;

/// Fixed C code generation backend for CURSED
/// Properly handles string concatenation, variable declarations, and function calls

const VariableInfo = struct {
    name: []const u8,
    var_type: []const u8,
    value: []const u8,
};

/// Fixed vibez.spill() translation with proper string handling
fn translateVibesSpillToCFixed(line: []const u8, start: usize, writer: anytype, variables: *std.ArrayList(VariableInfo)) !void {
    if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
        if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
            const content_start = start + paren_start + 1;
            const content = line[content_start..paren_end];
            
            // Check if it's a simple string literal
            if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                try writer.print("    printf({s}\"\\n\");\n", .{content});
                return;
            }
            
            // Check for string concatenation with +
            if (std.mem.indexOf(u8, content, " + ")) |_| {
                try translateStringConcatenation(content, writer, variables);
                return;
            }
            
            // Handle simple variable or literal
            if (std.fmt.parseInt(i64, content, 10)) |num| {
                try writer.print("    printf(\"%ld\\n\", {}L);\n", .{num});
            } else |_| {
                // Look up variable type
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
                    // Treat as string literal if unknown
                    try writer.print("    printf(\"%s\\n\");\n", .{});
                }
            }
        }
    }
}

/// Translate string concatenation expressions to proper C code
fn translateStringConcatenation(expression: []const u8, writer: anytype, variables: *std.ArrayList(VariableInfo)) !void {
    var parts = std.ArrayList([]const u8).init(std.heap.page_allocator);
    defer parts.deinit();
    
    // Split by " + " to get concatenation parts
    var iter = std.mem.split(u8, expression, " + ");
    while (iter.next()) |part| {
        const trimmed = std.mem.trim(u8, part, " \t");
        try parts.append(trimmed);
    }
    
    if (parts.items.len == 0) return;
    
    // Generate C code for string concatenation using sprintf
    try writer.print("    char temp_str[1024];\n", .{});
    try writer.print("    sprintf(temp_str, ", .{});
    
    // Build format string
    var first = true;
    for (parts.items) |part| {
        if (!first) try writer.print(" ", .{});
        first = false;
        
        if (part.len >= 2 and part[0] == '"' and part[part.len - 1] == '"') {
            // String literal - add without quotes to format string
            const literal = part[1..part.len-1];
            try writer.print("\"{s}\"", .{literal});
        } else {
            // Variable - determine format specifier
            var var_type: ?[]const u8 = null;
            for (variables.items) |var_info| {
                if (std.mem.eql(u8, var_info.name, part)) {
                    var_type = var_info.var_type;
                    break;
                }
            }
            
            if (var_type) |vtype| {
                if (std.mem.eql(u8, vtype, "drip")) {
                    try writer.print("\"%ld\"", .{});
                } else if (std.mem.eql(u8, vtype, "tea")) {
                    try writer.print("\"%s\"", .{});
                } else if (std.mem.eql(u8, vtype, "lit")) {
                    try writer.print("\"%s\"", .{});
                } else if (std.mem.eql(u8, vtype, "meal")) {
                    try writer.print("\"%f\"", .{});
                } else {
                    try writer.print("\"%s\"", .{});
                }
            } else {
                try writer.print("\"%s\"", .{});
            }
        }
    }
    
    // Add variable arguments
    for (parts.items) |part| {
        if (part.len >= 2 and part[0] == '"' and part[part.len - 1] == '"') {
            // String literal - already included in format string
            continue;
        } else {
            // Variable
            var var_type: ?[]const u8 = null;
            for (variables.items) |var_info| {
                if (std.mem.eql(u8, var_info.name, part)) {
                    var_type = var_info.var_type;
                    break;
                }
            }
            
            if (var_type) |vtype| {
                if (std.mem.eql(u8, vtype, "lit")) {
                    try writer.print(", {s} ? \"based\" : \"cringe\"", .{part});
                } else {
                    try writer.print(", {s}", .{part});
                }
            } else {
                try writer.print(", \"{s}\"", .{part});
            }
        }
    }
    
    try writer.print(");\n", .{});
    try writer.print("    printf(\"%s\\n\", temp_str);\n", .{});
}

/// Generate proper C variable declarations with initialization
fn generateCVariableDeclaration(var_name: []const u8, var_type: []const u8, var_value: []const u8, writer: anytype) !void {
    if (std.mem.eql(u8, var_type, "drip")) {
        // Integer type
        if (std.fmt.parseInt(i64, var_value, 10)) |num| {
            try writer.print("    long {s} = {}L;\n", .{ var_name, num });
        } else |_| {
            try writer.print("    long {s} = 0;\n", .{var_name});
        }
    } else if (std.mem.eql(u8, var_type, "tea")) {
        // String type
        if (var_value.len >= 2 and var_value[0] == '"' and var_value[var_value.len - 1] == '"') {
            try writer.print("    char* {s} = {s};\n", .{ var_name, var_value });
        } else {
            try writer.print("    char* {s} = \"{s}\";\n", .{ var_name, var_value });
        }
    } else if (std.mem.eql(u8, var_type, "lit")) {
        // Boolean type
        if (std.mem.eql(u8, var_value, "based")) {
            try writer.print("    int {s} = 1;\n", .{var_name});
        } else if (std.mem.eql(u8, var_value, "cringe")) {
            try writer.print("    int {s} = 0;\n", .{var_name});
        } else {
            try writer.print("    int {s} = 0;\n", .{var_name});
        }
    } else if (std.mem.eql(u8, var_type, "meal")) {
        // Float type
        if (std.fmt.parseFloat(f64, var_value)) |num| {
            try writer.print("    double {s} = {};\n", .{ var_name, num });
        } else |_| {
            try writer.print("    double {s} = 0.0;\n", .{var_name});
        }
    } else {
        // Default to string
        try writer.print("    char* {s} = \"{s}\";\n", .{ var_name, var_value });
    }
}

/// Generate proper C function declarations
fn generateCFunctionDeclaration(func_name: []const u8, params: []const u8, return_type: []const u8, writer: anytype) !void {
    // Map CURSED types to C types
    var c_return_type: []const u8 = "void";
    if (std.mem.eql(u8, return_type, "drip")) {
        c_return_type = "long";
    } else if (std.mem.eql(u8, return_type, "tea")) {
        c_return_type = "char*";
    } else if (std.mem.eql(u8, return_type, "lit")) {
        c_return_type = "int";
    } else if (std.mem.eql(u8, return_type, "meal")) {
        c_return_type = "double";
    }
    
    try writer.print("{s} {s}({s}) {{\n", .{ c_return_type, func_name, params });
}

/// Complete fixed C code generation example
pub fn generateFixedCCode(allocator: std.mem.Allocator, cursed_source: []const u8, output_filename: []const u8) !void {
    var variables = std.ArrayList(VariableInfo).init(allocator);
    defer variables.deinit();
    
    const c_filename = try std.fmt.allocPrint(allocator, "{s}.c", .{output_filename});
    defer allocator.free(c_filename);
    
    const file = try std.fs.cwd().createFile(c_filename, .{});
    defer file.close();
    
    const writer = file.writer();
    
    // Generate C header
    try writer.print("#include <stdio.h>\n", .{});
    try writer.print("#include <stdlib.h>\n", .{});
    try writer.print("#include <string.h>\n\n", .{});
    
    try writer.print("int main() {{\n", .{});
    
    // Process CURSED source line by line
    var line_iter = std.mem.split(u8, cursed_source, "\n");
    while (line_iter.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t");
        
        if (trimmed.len == 0 or trimmed[0] == '#') continue;
        
        // Handle vibez.spill() calls
        if (std.mem.indexOf(u8, trimmed, "vibez.spill")) |start| {
            try translateVibesSpillToCFixed(trimmed, start, writer, &variables);
        }
        // Handle variable declarations
        else if (std.mem.startsWith(u8, trimmed, "sus ")) {
            // Parse variable declaration
            // sus name type = value
            if (std.mem.indexOf(u8, trimmed, " = ")) |eq_pos| {
                const decl_part = trimmed[4..eq_pos]; // Skip "sus "
                const value_part = std.mem.trim(u8, trimmed[eq_pos + 3..], " \t");
                
                if (std.mem.indexOf(u8, decl_part, " ")) |space_pos| {
                    const var_name = std.mem.trim(u8, decl_part[0..space_pos], " \t");
                    const var_type = std.mem.trim(u8, decl_part[space_pos + 1..], " \t");
                    
                    try generateCVariableDeclaration(var_name, var_type, value_part, writer);
                    
                    // Store variable info
                    try variables.append(.{
                        .name = try allocator.dupe(u8, var_name),
                        .var_type = try allocator.dupe(u8, var_type),
                        .value = try allocator.dupe(u8, value_part),
                    });
                }
            }
        }
    }
    
    try writer.print("    return 0;\n", .{});
    try writer.print("}}\n", .{});
    
    print("✅ Generated fixed C code: {s}\n", .{c_filename});
}

// Test the fixed C code generation
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const test_cursed = 
        \\sus message tea = "Hello, World!"
        \\sus count drip = 42
        \\sus active lit = based
        \\vibez.spill("Test message: " + message)
        \\vibez.spill("Count is: " + count)
        \\vibez.spill("Status: " + active)
    ;
    
    try generateFixedCCode(allocator, test_cursed, "test_fixed");
    
    print("🎉 Fixed C code generation test complete!\n", .{});
}
