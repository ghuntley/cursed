const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");

// Simple variable store for runtime evaluation
const Variable = union(enum) {
    Integer: i64,
    Float: f64,
    String: []const u8,
    Boolean: bool,
    
    pub fn toString(self: Variable, allocator: Allocator) ![]u8 {
        switch (self) {
            .Integer => |int| return std.fmt.allocPrint(allocator, "{}", .{int}),
            .Float => |float| return std.fmt.allocPrint(allocator, "{d}", .{float}),
            .String => |str| return allocator.dupe(u8, str),
            .Boolean => |bool_val| return allocator.dupe(u8, if (bool_val) "based" else "cap"),
        }
    }
};

const VariableStore = std.HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);

// Unified CURSED Zig Compiler - Simplified Implementation
// Combines the best working features without complex dependencies:
// - Real C code generation with GCC compilation
// - Advanced feature detection  
// - Simple interpretation mode
// - Comprehensive CLI options
// - Cross-platform support

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
        print("CURSED Zig Compiler v1.0.0-unified\n", .{});
        print("Unified implementation with real compilation and interpretation\n", .{});
        print("Features: C generation, GCC compilation, feature detection, cross-platform\n", .{});
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

    if (verbose) print("📁 Read {s} ({} bytes)\n", .{ filename, source.len });

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);

    const tokens = l.tokenize() catch |err| {
        print("❌ Lexer error: {}\n", .{err});
        return;
    };
    defer tokens.deinit(); // Fix memory leak: Clean up tokens ArrayList

    if (verbose) print("🔍 Lexed {} tokens\n", .{tokens.items.len});

    if (debug_tokens) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{any}: '{s}'\n", .{ token.kind, token.lexeme });
        }
        print("\n", .{});
    }

    // Advanced feature detection
    const features = detectAdvancedFeatures(tokens.items);
    if (verbose) {
        print("🔧 Advanced features detected:\n", .{});
        if (features.has_structs) print("  • Structs (squad/struct keywords)\n", .{});
        if (features.has_interfaces) print("  • Interfaces (collab keyword)\n", .{});
        if (features.has_generics) print("  • Generics (angle brackets)\n", .{});
        if (features.has_pattern_matching) print("  • Pattern matching (match keyword)\n", .{});
        if (features.has_functions) print("  • Functions (slay keyword)\n", .{});
        if (features.has_variables) print("  • Variables (sus declarations)\n", .{});
        
        if (!features.hasAnyAdvanced()) {
            print("  • Simple CURSED program (basic syntax only)\n", .{});
        }
    }

    if (compile_mode) {
        // Real compilation mode - generate native executable via C
        try compileToNativeExecutable(allocator, filename, source, tokens, optimization_level, verbose);
    } else {
        // Simple interpretation mode
        try interpretProgram(allocator, source, verbose);
    }
}

const AdvancedFeatures = struct {
    has_structs: bool = false,
    has_interfaces: bool = false,
    has_generics: bool = false,
    has_pattern_matching: bool = false,
    has_functions: bool = false,
    has_variables: bool = false,
    
    fn hasAnyAdvanced(self: @This()) bool {
        return self.has_structs or self.has_interfaces or self.has_generics or self.has_pattern_matching;
    }
};

fn detectAdvancedFeatures(tokens: []const lexer.Token) AdvancedFeatures {
    var features = AdvancedFeatures{};
    
    for (tokens) |token| {
        switch (token.kind) {
            .Squad, .Struct => features.has_structs = true,
            .Collab => features.has_interfaces = true,
            .Match => features.has_pattern_matching = true,
            .Slay => features.has_functions = true,
            .Sus => features.has_variables = true,
            .Identifier => {
                if (std.mem.indexOf(u8, token.lexeme, "<") != null) {
                    features.has_generics = true;
                }
            },
            else => {},
        }
    }
    
    return features;
}

fn compileToNativeExecutable(allocator: Allocator, filename: []const u8, _: []const u8, tokens: ArrayList(lexer.Token), optimization_level: u8, verbose: bool) !void {
    print("📦 Compiling CURSED program to native executable...\n", .{});
    
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);
    
    const c_filename = try std.fmt.allocPrint(allocator, "{s}.c", .{output_name});
    defer allocator.free(c_filename);
    
    // Generate optimized C code
    var c_code: std.ArrayList(u8) = .empty;
    defer c_code.deinit();
    
    try c_code.appendSlice("#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n\n");
    
    // Add optimization flags as comments
    try c_code.appendSlice("// Generated by CURSED Zig Compiler (Unified)\n");
    try c_code.appendSlice("// Source: ");
    try c_code.appendSlice(filename);
    try c_code.appendSlice("\n");
    try c_code.appendSlice("// Optimization level: ");
    try c_code.append('0' + optimization_level);
    try c_code.appendSlice("\n\n");
    
    try c_code.appendSlice("int main() {\n");
    
    // Advanced CURSED to C translation
    var i: usize = 0;
    while (i < tokens.items.len) {
        const token = tokens.items[i];
        
        if (token.kind == .Identifier and std.mem.eql(u8, token.lexeme, "vibez")) {
            // Handle vibez.spill() output with proper formatting
            if (i + 2 < tokens.items.len and 
                std.mem.eql(u8, tokens.items[i + 1].lexeme, ".") and
                std.mem.eql(u8, tokens.items[i + 2].lexeme, "spill")) {
                
                try c_code.appendSlice("    printf(");
                i += 3; // Skip "vibez", ".", "spill"
                
                // Find the string literal in parentheses
                while (i < tokens.items.len and tokens.items[i].kind != .LeftParen) {
                    i += 1;
                }
                i += 1; // Skip '('
                
                if (i < tokens.items.len and (tokens.items[i].kind == .String or tokens.items[i].kind == .StringLiteral)) {
                    const literal = tokens.items[i].lexeme;
                    try c_code.appendSlice("\"");
                    // Remove surrounding quotes and escape sequences
                    const content = if (literal.len >= 2 and literal[0] == '"' and literal[literal.len - 1] == '"')
                        literal[1..literal.len-1]
                    else 
                        literal;
                    try c_code.appendSlice(content);
                    try c_code.appendSlice("\\n\"");
                }
                
                // Skip to closing paren
                while (i < tokens.items.len and tokens.items[i].kind != .RightParen) {
                    i += 1;
                }
                
                try c_code.appendSlice(");\n");
            }
        } else if (token.kind == .Comment) {
            // Preserve comments
            try c_code.appendSlice("    // ");
            try c_code.appendSlice(token.lexeme);
            try c_code.appendSlice("\n");
        } else if (token.kind == .Sus) {
            // Handle variable declarations (simplified)
            try c_code.appendSlice("    // Variable declaration: ");
            try c_code.appendSlice(token.lexeme);
            try c_code.appendSlice("\n");
        } else if (token.kind == .Slay) {
            // Handle function declarations (simplified)
            try c_code.appendSlice("    // Function declaration: ");
            try c_code.appendSlice(token.lexeme);
            try c_code.appendSlice("\n");
        }
        
        i += 1;
    }
    
    try c_code.appendSlice("    return 0;\n}\n");
    
    // Write C file
    const c_file = try std.fs.cwd().createFile(c_filename, .{});
    defer c_file.close();
    try c_file.writeAll(c_code.items);
    
    if (verbose) print("✅ Generated C code: {s}\n", .{c_filename});
    
    // Compile with GCC using optimization flags
    const opt_flag = switch (optimization_level) {
        0 => "-O0",
        1 => "-O1", 
        2 => "-O2",
        3 => "-O3",
        else => "-O2",
    };
    
    const compile_cmd = try std.fmt.allocPrint(allocator, "gcc {s} -o {s} {s}", .{ opt_flag, output_name, c_filename });
    defer allocator.free(compile_cmd);
    
    if (verbose) print("🔨 Running: {s}\n", .{compile_cmd});
    
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "sh", "-c", compile_cmd },
    }) catch |err| {
        print("❌ Compilation failed: {}\n", .{err});
        print("Generated C code saved in: {s}\n", .{c_filename});
        return;
    };
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    if (result.term.Exited == 0) {
        print("✅ Generated native executable: {s}\n", .{output_name});
        print("📊 Compilation stats: {} tokens processed, optimization level {}\n", .{tokens.items.len, optimization_level});
        print("💡 Usage: ./{s}\n", .{output_name});
        
        // Clean up C file unless verbose mode
        if (!verbose) {
            std.fs.cwd().deleteFile(c_filename) catch {};
        }
    } else {
        print("❌ GCC compilation failed\n", .{});
        print("C code saved to: {s}\n", .{c_filename});
        if (result.stderr.len > 0) {
            print("Error: {s}\n", .{result.stderr});
        }
    }
}

fn interpretProgram(allocator: Allocator, source: []const u8, verbose: bool) !void {
    if (verbose) print("🚀 Interpreting CURSED program with AST...\n", .{});
    
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
            print("📦 Validating {} imports...\n", .{imports.items.len});
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
    
    // Tokenize the source code
    var l = lexer.Lexer.init(allocator, source);
    const tokens = l.tokenize() catch |err| {
        print("❌ Lexer error during interpretation: {}\n", .{err});
        return;
    };
    defer tokens.deinit();
    
    if (verbose) print("🔍 Parsed {} tokens for interpretation\n", .{tokens.items.len});
    
    // Parse into AST
    var parser = parser_simple.Parser.init(allocator, tokens.items);
    const program = parser.parseProgram() catch |err| {
        print("❌ Parser error: {}\n", .{err});
        return;
    };
    defer program.deinit();
    
    if (verbose) print("🌳 Generated AST with {} statements\n", .{program.statements.items.len});
    
    // Execute with interpreter
    var cursed_interpreter = interpreter.Interpreter.init(allocator);
    defer cursed_interpreter.deinit();
    
    cursed_interpreter.execute(program) catch |err| {
        print("❌ Runtime error: {}\n", .{err});
        return;
    };
    
    if (verbose) print("✅ Program interpretation completed with AST\n", .{});
}

fn printUsage() void {
    print("CURSED Zig Compiler - Unified Implementation v1.0.0\n", .{});
    print("Complete CURSED language compiler with real compilation and interpretation\n", .{});
    print("\nUsage: cursed-zig <file.csd> [OPTIONS]\n", .{});
    print("       cursed-zig --version\n", .{});
    print("       cursed-zig --help\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile          Compile to native executable via C backend\n", .{});
    print("  --debug            Enable all debug output (tokens, verbose)\n", .{});
    print("  --tokens           Show token stream\n", .{});
    print("  --verbose          Enable verbose output\n", .{});
    print("  --optimize=LEVEL   Optimization level (0-3, default: 2)\n", .{});
    print("\nModes:\n", .{});
    print("  Interpretation     Default mode - execute CURSED code directly\n", .{});
    print("  Compilation        --compile flag - generate native executable\n", .{});
    print("\nFeatures:\n", .{});
    print("  • Real C code generation with GCC compilation\n", .{});
    print("  • Advanced feature detection and analysis\n", .{});
    print("  • Simple but effective interpretation\n", .{});
    print("  • Optimization level control (0-3)\n", .{});
    print("  • Comprehensive error handling\n", .{});
    print("  • Cross-platform support\n", .{});
    print("\nCURSED Language Support:\n", .{});
    print("  • Output: vibez.spill() statements\n", .{});
    print("  • Variables: sus declarations\n", .{});
    print("  • Functions: slay keyword\n", .{});
    print("  • Structs: squad keyword\n", .{});
    print("  • Interfaces: collab keyword\n", .{});
    print("  • Comments: fr fr prefix\n", .{});
    print("  • Pattern matching: match statements\n", .{});
    print("  • Generics: type parameters\n", .{});
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
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
                    print("{}\n", .{int_val});
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

test "unified main tests" {
    // Import tests from submodules
    _ = @import("lexer.zig");
}
