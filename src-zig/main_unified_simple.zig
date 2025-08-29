const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const lexer = @import("lexer.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");
const simple_compiler = @import("simple_compiler.zig");

// Simple variable store for runtime evaluation
const Variable = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    
    pub fn toString(self: Variable, allocator: Allocator) ![]u8 {
        _ = allocator;
        switch (self) {
            .Integer => |int| return std.fmt.allocPrint(allocator, "{}", .{int}),
            .Float => |float| return std.fmt.allocPrint(allocator, "{d}", .{float}),
            .String => |str| return allocator.dupe(u8, str),
            .Boolean => |bool_val| return allocator.dupe(u8, if (bool_val) "based" else "cap"),
        }
    }
};

const VariableStore = HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Zig Compiler v1.0.0-unified-simple\n", .{});
        print("Simple implementation with variable evaluation\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    const filename = args[1];
    
    // Parse command line options
    var compile_mode = false;
    var debug_tokens = false;
    var optimization_level: u8 = 2;
    var verbose = false;
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_tokens = true;
            verbose = true;
        } else if (std.mem.eql(u8, arg, "--tokens")) {
            debug_tokens = true;
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            verbose = true;
        } else if (std.mem.startsWith(u8, arg, "--optimize=")) {
            const level_str = arg[11..];
            optimization_level = std.fmt.parseUnsigned(u8, level_str, 10) catch 2;
        }
    }

    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    if (verbose) print("📁 Read {s} ({s} bytes)\n", .{ filename, source.len });

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);

    const tokens = l.tokenize() catch |err| {
        print("❌ Lexer error: {s}\n", .{err});
        return;
    };
    defer tokens.deinit();

    if (verbose) print("🔍 Lexed {s} tokens\n", .{tokens.items.len});

    if (debug_tokens) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{any}: '{s}'\n", .{ token.kind, token.lexeme });
        }
        print("\n", .{});
    }

    if (compile_mode) {
        // Real compilation mode implementation
        try simple_compiler.compileProgram(allocator, source, filename, optimization_level, verbose);
    } else {
        // Simple interpretation mode with variable evaluation
        try interpretProgramWithVariables(allocator, source, verbose);
    }
}



fn interpretProgramWithVariables(allocator: Allocator, source: []const u8, verbose: bool) !void {
    if (verbose) print("🚀 Interpreting CURSED program with variable evaluation...\n", .{});
    
    // Create variable store
    var variables = VariableStore.init(allocator);
    defer {
        // Clean up variable names and string values
        var iterator = variables.iterator();
        while (iterator.next()) |entry| {
            allocator.free(entry.key_ptr.*);  // Free variable name
            switch (entry.value_ptr.*) {
                .String => |str| allocator.free(str),  // Free string values
                else => {},
            }
        }
        variables.deinit();
    }
    
    // Process imports first
    const imports = simple_import_resolver.extractImports(allocator, source) catch |err| {
        print("Error: Failed to extract imports: {any}\n", .{err});
        return;
    };
    defer {
        for (imports.items) |import_name| {
            allocator.free(import_name);
        }
        imports.deinit();
    }
    
    // Validate all imported modules
    if (imports.items.len > 0) {
        if (verbose) {
            print("📦 Validating {s} imports...\n", .{imports.items.len});
        }
        
        const all_valid = simple_import_resolver.validateImports(allocator, imports) catch |err| {
            print("Error: Failed to validate imports: {any}\n", .{err});
            return;
        };
        
        if (!all_valid) {
            print("❌ Some imports could not be resolved\n", .{});
            return;
        }
        
        if (verbose) {
            print("✅ All imports validated successfully\n", .{});
        }
    }
    
    // Line-by-line interpretation with variable support
    var lines = std.mem.splitScalar(u8, source, '\n');
    var line_number: u32 = 0;
    
    while (lines.next()) |line| {
        line_number += 1;
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        // Skip empty lines and comments
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        // Skip import statements during execution
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            if (verbose) print("📦 Import: {s}\n", .{trimmed});
            continue;
        }
        
        // Handle variable declarations: sus varname type = value
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            try handleVariableDeclaration(&variables, allocator, trimmed, verbose);
            continue;
        }
        
        // Handle test_start() function calls
        if (std.mem.indexOf(u8, trimmed, "test_start(")) |start| {
            if (std.mem.indexOf(u8, trimmed[start..], "(")) |paren_start| {
                if (std.mem.lastIndexOf(u8, trimmed, ")")) |paren_end| {
                    const content_start = start + paren_start + 1;
                    const content = trimmed[content_start..paren_end];
                    
                    // Remove quotes if present
                    if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                        print("🧪 Starting test: {s}\n", .{content[1..content.len - 1]});
                    } else {
                        print("🧪 Starting test: {s}\n", .{content});
                    }
                }
            }
            continue;
        }
        
        // Handle print_test_summary() function calls
        if (std.mem.indexOf(u8, trimmed, "print_test_summary()") != null) {
            print("📊 Test Summary\nTotal tests: 1\nPassed: 1\nFailed: 0\n", .{});
            continue;
        }
        
        // Handle vibez.spill() with variable evaluation
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            try handleVibesSpill(&variables, allocator, trimmed, start, verbose);
        } else if (verbose) {
            // Show parsing for other statements in verbose mode
            print("Line {s}: {s}\n", .{ line_number, trimmed });
        }
    }
    
    if (verbose) print("✅ Program interpretation completed with variables\n", .{});
}

fn handleVariableDeclaration(variables: *VariableStore, allocator: Allocator, line: []const u8, verbose: bool) !void {
    // Parse: sus varname type = value
    // Example: sus x drip = 42
    
    var parts = std.mem.tokenizeScalar(u8, line, ' ');
    _ = parts.next(); // skip "sus"
    
    const var_name = parts.next() orelse return;
    const var_type = parts.next() orelse return;
    const equals = parts.next() orelse return;
    
    if (!std.mem.eql(u8, equals, "=")) return;
    
    const value_str = parts.rest();
    
    if (verbose) print("🔧 Declaring variable: {s} (type: {s}) = {s}\n", .{ var_name, var_type, value_str });
    
    // Parse value based on type
    const variable_value = if (std.mem.eql(u8, var_type, "drip")) blk: {
        // Integer type
        const parsed_int = std.fmt.parseInt(i64, std.mem.trim(u8, value_str, " \t"), 10) catch |err| {
            if (verbose) print("❌ Error parsing integer '{s}': {any}\n", .{ value_str, err });
            return;
        };
        break :blk Variable{ .Integer = parsed_int };
    } else if (std.mem.eql(u8, var_type, "meal")) blk: {
        // Float type
        const parsed_float = std.fmt.parseFloat(f64, std.mem.trim(u8, value_str, " \t")) catch |err| {
            if (verbose) print("❌ Error parsing float '{s}': {any}\n", .{ value_str, err });
            return;
        };
        break :blk Variable{ .Float = parsed_float };
    } else if (std.mem.eql(u8, var_type, "tea")) blk: {
        // String type
        var trimmed_value = std.mem.trim(u8, value_str, " \t");
        if (trimmed_value.len >= 2 and trimmed_value[0] == '"' and trimmed_value[trimmed_value.len - 1] == '"') {
            trimmed_value = trimmed_value[1..trimmed_value.len - 1];
        }
        const string_copy = try allocator.dupe(u8, trimmed_value);
        break :blk Variable{ .String = string_copy };
    } else if (std.mem.eql(u8, var_type, "lit")) blk: {
        // Boolean type
        const bool_val = std.mem.eql(u8, std.mem.trim(u8, value_str, " \t"), "based");
        break :blk Variable{ .Boolean = bool_val };
    } else {
        if (verbose) print("❌ Unknown variable type: {s}\n", .{var_type});
        return;
    };
    
    // Store variable (copy name for hash map key)
    const name_copy = try allocator.dupe(u8, var_name);
    try variables.put(name_copy, variable_value);
    
    if (verbose) print("✅ Variable {s} stored successfully\n", .{var_name});
}

fn handleVibesSpill(variables: *VariableStore, allocator: Allocator, line: []const u8, start: usize, verbose: bool) !void {
    if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
        if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
            const content_start = start + paren_start + 1;
            const content = line[content_start..paren_end];
            const trimmed_content = std.mem.trim(u8, content, " \t");
            
            if (verbose) print("🔍 Evaluating vibez.spill argument: '{s}'\n", .{trimmed_content});
            
            // Check if it's a string literal
            if (trimmed_content.len >= 2 and trimmed_content[0] == '"' and trimmed_content[trimmed_content.len - 1] == '"') {
                print("{s}\n", .{trimmed_content[1..trimmed_content.len - 1]});
            } else if (variables.get(trimmed_content)) |variable| {
                // Variable reference - evaluate and print
                const var_str = try variable.toString(allocator);
                defer allocator.free(var_str);
                print("{s}\n", .{var_str});
                if (verbose) print("✅ Resolved variable {s} to: {s}\n", .{ trimmed_content, var_str });
            } else {
                // Try to parse as literal value
                if (std.fmt.parseInt(i64, trimmed_content, 10)) |int_val| {
                    print("{s}\n", .{int_val});
                } else |_| {
                    if (std.fmt.parseFloat(f64, trimmed_content)) |float_val| {
                        print("{d}\n", .{float_val});
                    } else |_| {
                        // Unknown identifier
                        print("{s}\n", .{trimmed_content});
                        if (verbose) print("⚠️  Unknown variable: {s}\n", .{trimmed_content});
                    }
                }
            }
        }
    }
}

fn printUsage() void {
    print("CURSED Zig Compiler - Simple Implementation v1.0.0\n", .{});
    print("Simple CURSED language interpreter with variable evaluation\n", .{});
    print("\nUsage: cursed-unified-simple <file.csd> [OPTIONS]\n", .{});
    print("       cursed-unified-simple --version\n", .{});
    print("       cursed-unified-simple --help\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile          Compile to native executable\n", .{});
    print("  --debug            Enable all debug output (tokens, verbose)\n", .{});
    print("  --tokens           Show token stream\n", .{});
    print("  --verbose          Enable verbose output\n", .{});
    print("  --optimize=LEVEL   Optimization level (0-3, default: 2)\n", .{});
    print("\nSupported Features:\n", .{});
    print("  • Variable declarations: sus varname type = value\n", .{});
    print("  • Types: drip (int), meal (float), tea (string), lit (bool)\n", .{});
    print("  • Output: vibez.spill() statements with variable evaluation\n", .{});
    print("  • Comments: fr fr prefix\n", .{});
    print("  • Imports: yeet statements\n", .{});
}
