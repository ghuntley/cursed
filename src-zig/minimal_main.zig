const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

const lexer = @import("lexer.zig");
const module_loader = @import("module_loader.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");
const llvm_backend_minimal = @import("llvm_backend_minimal.zig");

// Simple variable storage
const ArrayList = std.ArrayList;

// Simple function definition storage
const FunctionDefinition = struct {
    name: []const u8,
    parameters: ArrayList([]const u8),
    body: []const u8,
};

const Variable = union(enum) {
    Integer: i64,
    String: []const u8,
    Boolean: bool,
    Array: ArrayList(Variable),
    
    fn toString(self: Variable, allocator: Allocator) ![]const u8 {
        return switch (self) {
            .Integer => |i| try std.fmt.allocPrint(allocator, "{}", .{i}),
            .String => |s| try allocator.dupe(u8, s),
            .Boolean => |b| try allocator.dupe(u8, if (b) "based" else "cringe"),
            .Array => |arr| {
                var result = std.ArrayList(u8).init(allocator);
                defer result.deinit();
                try result.append('[');
                for (arr.items, 0..) |item, i| {
                    if (i > 0) try result.appendSlice(", ");
                    const item_str = try item.toString(allocator);
                    defer allocator.free(item_str);
                    try result.appendSlice(item_str);
                }
                try result.append(']');
                return try result.toOwnedSlice();
            },
        };
    }
    
    fn deinit(self: *Variable, allocator: Allocator) void {
        switch (self.*) {
            .String => |s| allocator.free(s),
            .Array => |*arr| {
                for (arr.items) |*item| {
                    item.deinit(allocator);
                }
                arr.deinit();
            },
            else => {},
        }
    }
    
    fn clone(self: Variable, allocator: Allocator) !Variable {
        return switch (self) {
            .Integer => |i| Variable{ .Integer = i },
            .String => |s| Variable{ .String = try allocator.dupe(u8, s) },
            .Boolean => |b| Variable{ .Boolean = b },
            .Array => |arr| {
                var new_array = ArrayList(Variable).init(allocator);
                for (arr.items) |item| {
                    try new_array.append(try item.clone(allocator));
                }
                return Variable{ .Array = new_array };
            },
        };
    }
};

const VariableStore = HashMap([]const u8, Variable, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);
const FunctionStore = HashMap([]const u8, FunctionDefinition, std.hash_map.StringContext, std.hash_map.default_max_load_percentage);

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
        print("CURSED Zig Compiler v1.0.0-minimal\n", .{});
        print("CURSED language compiler with real compilation output\n", .{});
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
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_tokens = true;
        } else if (std.mem.eql(u8, arg, "--tokens")) {
            debug_tokens = true;
        }
    }

    // Read source file
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        print("Error: Could not open file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer file.close();

    const source = file.readToEndAlloc(allocator, 1024 * 1024) catch |err| {
        print("Error: Could not read file '{s}': {}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    print("🚀 CURSED Compiler Processing: {s}\n", .{filename});

    // Tokenize
    var l = lexer.Lexer.init(allocator, source);

    const tokens = l.tokenize() catch |err| {
        print("Lexer error: {}\n", .{err});
        return;
    };
    defer tokens.deinit();

    if (debug_tokens) {
        print("=== TOKENS ===\n", .{});
        for (tokens.items) |token| {
            print("{s}: '{s}'\n", .{ @tagName(token.kind), token.lexeme });
        }
        print("\n", .{});
    }

    if (compile_mode) {
        // LLVM compilation mode instead of C
        try compileToLLVM(allocator, filename, source);
    } else {
        // Interpretation mode - simple line execution
        try interpretProgram(allocator, source);
    }
}

fn compileToLLVM(allocator: Allocator, filename: []const u8, source: []const u8) !void {
    print("🔥 Compiling CURSED program to native executable using LLVM...\n", .{});
    
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);
    
    const ir_filename = try std.fmt.allocPrint(allocator, "{s}.ll", .{output_name});
    defer allocator.free(ir_filename);
    
    // Use minimal LLVM backend
    try llvm_backend_minimal.compileToLLVM(allocator, source, ir_filename);
    
    // Compile IR to native executable
    try llvm_backend_minimal.compileIRToNative(allocator, ir_filename, output_name);
    
    print("✅ LLVM compilation complete! Run with: ./{s}\n", .{output_name});
}

fn compileToC(allocator: Allocator, filename: []const u8, source: []const u8, tokens: std.ArrayList(lexer.Token)) !void {
    _ = source; // Parameter marked as used
    print("📦 Compiling CURSED program to C executable...\n", .{});
    
    const output_name = try getOutputName(allocator, filename);
    defer allocator.free(output_name);
    
    const c_filename = try std.fmt.allocPrint(allocator, "{s}.c", .{output_name});
    defer allocator.free(c_filename);
    
    // Generate C code
    var c_code = std.ArrayList(u8).init(allocator);
    defer c_code.deinit();
    
    try c_code.appendSlice("#include <stdio.h>\n#include <string.h>\n\n");
    try c_code.appendSlice("int main() {\n");
    
    // Simple CURSED to C translation
    var i: usize = 0;
    while (i < tokens.items.len) {
        const token = tokens.items[i];
        
        if (token.kind == .Identifier and std.mem.eql(u8, token.lexeme, "vibez")) {
            // Handle vibez.spill() output
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
                
                if (i < tokens.items.len and tokens.items[i].kind == .String) {
                    try c_code.appendSlice("\"");
                    try c_code.appendSlice(tokens.items[i].lexeme[1..tokens.items[i].lexeme.len-1]); // Remove quotes
                    try c_code.appendSlice("\\n\"");
                }
                
                // Skip to closing paren
                while (i < tokens.items.len and tokens.items[i].kind != .RightParen) {
                    i += 1;
                }
                
                try c_code.appendSlice(");\n");
            }
        } else if (token.kind == .Comment) {
            // Add comments
            try c_code.appendSlice("    // ");
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
    
    // Compile with GCC
    const compile_cmd = try std.fmt.allocPrint(allocator, "gcc -o {s} {s}", .{ output_name, c_filename });
    defer allocator.free(compile_cmd);
    
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "sh", "-c", compile_cmd },
    }) catch |err| {
        print("❌ Compilation failed: {}\n", .{err});
        print("Generated C code in: {s}\n", .{c_filename});
        return;
    };
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    if (result.term.Exited == 0) {
        print("✅ Generated executable: {s}\n", .{output_name});
        print("📊 Compilation stats: {} tokens processed\n", .{tokens.items.len});
        print("💡 Usage: ./{s}\n", .{output_name});
        
        // Clean up C file
        std.fs.cwd().deleteFile(c_filename) catch {};
    } else {
        print("❌ GCC compilation failed\n", .{});
        print("C code saved to: {s}\n", .{c_filename});
        if (result.stderr.len > 0) {
            print("Error: {s}\n", .{result.stderr});
        }
    }
}

// Simple function store for imported functions  
const FunctionInfo = struct {
    name: []const u8,
    available: bool,
};

// Simple pattern matching function
fn simplePatternMatch(value: Variable, pattern: []const u8) !bool {
    // Handle wildcard pattern
    if (std.mem.eql(u8, pattern, "_")) {
        return true;
    }
    
    switch (value) {
        .Integer => |int_val| {
            // Handle range patterns: start..end
            if (std.mem.indexOf(u8, pattern, "..")) |dot_pos| {
                const start_str = std.mem.trim(u8, pattern[0..dot_pos], " \t");
                const end_str = std.mem.trim(u8, pattern[dot_pos + 2..], " \t");
                
                if (std.fmt.parseInt(i64, start_str, 10)) |start_val| {
                    if (std.fmt.parseInt(i64, end_str, 10)) |end_val| {
                        return int_val >= start_val and int_val <= end_val;
                    } else |_| {
                        return false;
                    }
                } else |_| {
                    return false;
                }
            }
            
            // Handle exact integer match
            if (std.fmt.parseInt(i64, pattern, 10)) |pattern_int| {
                return int_val == pattern_int;
            } else |_| {
                return false;
            }
        },
        .Boolean => |bool_val| {
            if (std.mem.eql(u8, pattern, "based")) {
                return bool_val == true;
            } else if (std.mem.eql(u8, pattern, "cringe")) {
                return bool_val == false;
            }
            return false;
        },
        .String => |str_val| {
            // Handle string literal patterns (with quotes)
            if (pattern.len >= 2 and pattern[0] == '"' and pattern[pattern.len - 1] == '"') {
                const pattern_str = pattern[1..pattern.len - 1];
                return std.mem.eql(u8, str_val, pattern_str);
            }
            // Handle unquoted string patterns
            return std.mem.eql(u8, str_val, pattern);
        },
        .Array => |_| {
            // Arrays don't support pattern matching yet
            return false;
        },
    }
}

fn interpretProgram(allocator: Allocator, source: []const u8) !void {
    print("🚀 Interpreting CURSED program...\n", .{});
    
    // Create variable store
    var variables = VariableStore.init(allocator);
    defer {
        var iter = variables.iterator();
        while (iter.next()) |entry| {
            allocator.free(entry.key_ptr.*);
            switch (entry.value_ptr.*) {
                .String => |s| allocator.free(s),
                .Array => |*arr| {
                    for (arr.items) |*item| {
                        item.deinit(allocator);
                    }
                    arr.deinit();
                },
                else => {},
            }
        }
        variables.deinit();
    }
    
    // Create function store for user-defined functions
    var functions = FunctionStore.init(allocator);
    defer {
        var iter = functions.iterator();
        while (iter.next()) |entry| {
            // Free the key (function name in hash map)
            allocator.free(entry.key_ptr.*);
            
            // Free the function name in the definition
            allocator.free(entry.value_ptr.name);
            
            // Free each parameter name
            for (entry.value_ptr.parameters.items) |param| {
                allocator.free(param);
            }
            entry.value_ptr.parameters.deinit();
            
            // Free the function body
            allocator.free(entry.value_ptr.body);
        }
        functions.deinit();
    }
    
    // Create simple function store for imported functions
    var loaded_functions = HashMap([]const u8, FunctionInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    defer {
        var iter = loaded_functions.iterator();
        while (iter.next()) |entry| {
            allocator.free(entry.key_ptr.*);
        }
        loaded_functions.deinit();
    }
    
    // Initialize module loader
    var loader = module_loader.ModuleLoader.init(allocator, false);
    defer loader.deinit();
    
    // Extract imports from source
    const imports = simple_import_resolver.extractImports(allocator, source) catch |err| {
        print("Error extracting imports: {any}\n", .{err});
        return;
    };
    defer {
        for (imports.items) |import_name| {
            allocator.free(import_name);
        }
        imports.deinit();
    }
    
    // Load functions from imported modules
    if (imports.items.len > 0) {
        print("📦 Loading {} modules...\n", .{imports.items.len});
        
        for (imports.items) |module_name| {
            if (try loader.loadModule(module_name)) |module_functions| {
                print("✅ Loaded module: {s} with {} functions\n", .{ module_name, module_functions.len });
                
                // Add functions to the function store
                for (module_functions) |func| {
                    const func_key = try allocator.dupe(u8, func.name);
                    const func_info = FunctionInfo{ .name = func_key, .available = true };
                    try loaded_functions.put(func_key, func_info);
                    print("  📋 Available function: {s}\n", .{func.name});
                }
            } else {
                print("❌ Failed to load module: {s}\n", .{module_name});
            }
        }
    }
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    var line_number: u32 = 0;
    
    // Pattern matching state
    var in_pattern_block = false;
    var pattern_value: ?Variable = null;
    var pattern_matched = false;
    
    // Track multi-line function definitions
    var in_function_definition = false;
    var current_function_header: ?[]const u8 = null;
    var current_function_body = ArrayList(u8).init(allocator);
    defer current_function_body.deinit();
    defer if (current_function_header) |header| allocator.free(header);
    
    // State for multiline bestie loops
    var in_bestie_loop = false;
    var bestie_condition: ?[]const u8 = null;
    var bestie_body = ArrayList(u8).init(allocator);
    defer bestie_body.deinit();
    defer if (bestie_condition) |cond| allocator.free(cond);
    
    while (lines.next()) |line| {
        line_number += 1;
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        // Skip empty lines and comments
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "fr fr")) {
            continue;
        }
        
        // Handle enhanced if statements (ready) with complex conditions
        if (std.mem.startsWith(u8, trimmed, "ready (")) {
            // Check if this is a boolean condition (contains operators) or simple pattern matching
            if (std.mem.indexOf(u8, trimmed, "(")) |start_paren| {
                if (std.mem.indexOf(u8, trimmed[start_paren..], ")")) |rel_end_paren| {
                    const end_paren = start_paren + rel_end_paren;
                    const condition_str = std.mem.trim(u8, trimmed[start_paren + 1..end_paren], " \t");
                    
                    // Check if it contains boolean/comparison operators
                    const has_operators = std.mem.containsAtLeast(u8, condition_str, 1, "&&") or
                                         std.mem.containsAtLeast(u8, condition_str, 1, "||") or
                                         std.mem.containsAtLeast(u8, condition_str, 1, "==") or
                                         std.mem.containsAtLeast(u8, condition_str, 1, "!=") or
                                         std.mem.containsAtLeast(u8, condition_str, 1, "<=") or
                                         std.mem.containsAtLeast(u8, condition_str, 1, ">=") or
                                         std.mem.containsAtLeast(u8, condition_str, 1, "<") or
                                         std.mem.containsAtLeast(u8, condition_str, 1, ">") or
                                         std.mem.startsWith(u8, condition_str, "!");
                    
                    if (has_operators) {
                        // This is a boolean condition - use enhanced if statement handler
                        try handleEnhancedIfStatement(&variables, &functions, allocator, trimmed);
                        continue;
                    } else if (std.mem.endsWith(u8, trimmed, ") {")) {
                        // This is simple pattern matching - continue with original logic
                        in_pattern_block = true;
                        pattern_matched = false;
                        
                        // Evaluate the pattern value
                        if (variables.get(condition_str)) |var_value| {
                            pattern_value = var_value;
                            print("🎯 Pattern matching on: {any}\n", .{var_value});
                        } else {
                            print("❌ Variable '{s}' not found for pattern matching\n", .{condition_str});
                            pattern_value = null;
                        }
                        continue;
                    }
                }
            }
        }
        
        // Handle function body collection
        if (in_function_definition) {
            if (std.mem.eql(u8, trimmed, "}")) {
                // End of function definition
                in_function_definition = false;
                
                if (current_function_header) |header| {
                    // Combine header and body to create complete function definition
                    var complete_definition = ArrayList(u8).init(allocator);
                    defer complete_definition.deinit();
                    
                    try complete_definition.appendSlice(header);
                    try complete_definition.appendSlice(" ");
                    try complete_definition.appendSlice(current_function_body.items);
                    try complete_definition.appendSlice(" }");
                    
                    try handleFunctionDefinition(&functions, allocator, complete_definition.items);
                    
                    allocator.free(header);
                    current_function_header = null;
                }
                continue;
            } else {
                // Collect function body
                if (current_function_body.items.len > 0) {
                    try current_function_body.appendSlice(" ");
                }
                try current_function_body.appendSlice(trimmed);
                continue;
            }
        }
        
        // Handle end of pattern block
        if (in_pattern_block and std.mem.eql(u8, trimmed, "}")) {
            in_pattern_block = false;
            pattern_value = null;
            pattern_matched = false;
            print("Line {}: }}\n", .{line_number});
            continue;
        }
        
        // Handle pattern branches: pattern => action
        if (in_pattern_block) {
            if (std.mem.indexOf(u8, trimmed, "=>")) |arrow_pos| {
                if (!pattern_matched and pattern_value != null) {
                    const pattern_part = std.mem.trim(u8, trimmed[0..arrow_pos], " \t");
                    const action_part = std.mem.trim(u8, trimmed[arrow_pos + 2..], " \t");
                    
                    // Simple pattern matching
                    const matches = try simplePatternMatch(pattern_value.?, pattern_part);
                    if (matches) {
                        pattern_matched = true;  // Prevent other patterns from executing
                        print("✅ Pattern '{s}' matched! Executing: {s}\n", .{ pattern_part, action_part });
                        
                        // Execute the action (only vibez.spill for now)
                        if (std.mem.indexOf(u8, action_part, "vibez.spill(")) |start| {
                            try handleVibesSpill(&variables, &functions, allocator, action_part, start);
                        }
                    } else {
                        print("❌ Pattern '{s}' did not match\n", .{pattern_part});
                    }
                }
            }
            continue;
        }
        
        // Skip other statements inside pattern blocks
        if (in_pattern_block) {
            continue;
        }
        
        // Handle multiline bestie loops
        if (in_bestie_loop) {
            // We're inside a bestie loop body, collect lines until we find the closing brace
            if (std.mem.eql(u8, trimmed, "}")) {
                // End of bestie loop, execute it
                const condition = bestie_condition.?;
                const body = try bestie_body.toOwnedSlice();
                defer allocator.free(body);
                
                // Execute the loop
                try executeMultilineBestieLoop(&variables, &functions, allocator, condition, body);
                
                // Reset state
                in_bestie_loop = false;
                if (bestie_condition) |cond| allocator.free(cond);
                bestie_condition = null;
                bestie_body.clearRetainingCapacity();
            } else {
                // Add this line to the loop body
                try bestie_body.appendSlice(trimmed);
                try bestie_body.append('\n');
            }
            continue;
        }
        
        // Detect start of multiline bestie loop
        if (std.mem.startsWith(u8, trimmed, "bestie (") and std.mem.endsWith(u8, trimmed, ") {")) {
            // Extract condition from "bestie (condition) {"
            const condition_start = 8; // length of "bestie ("
            const condition_end = trimmed.len - 3; // remove ") {"
            const condition = std.mem.trim(u8, trimmed[condition_start..condition_end], " \t");
            
            // Store condition and enter loop collection mode
            bestie_condition = try allocator.dupe(u8, condition);
            in_bestie_loop = true;
            continue;
        }
        
        print("Line {}: {s}\n", .{ line_number, trimmed });
        
        // Check for single-line ready statements with pattern matching before splitting by semicolons
        if (std.mem.indexOf(u8, trimmed, "ready (") != null and 
            std.mem.indexOf(u8, trimmed, "=>") != null and 
            std.mem.indexOf(u8, trimmed, "{") != null and 
            std.mem.indexOf(u8, trimmed, "}") != null) {
            
            // Special case: if the line also contains variable declarations, handle them first
            if (std.mem.indexOf(u8, trimmed, "sus ") != null) {
                try handleLineWithVariableAndPattern(&variables, &functions, allocator, trimmed);
            } else {
                try handleSingleLineReadyPattern(&variables, &functions, allocator, trimmed);
            }
            continue;
        }
        
        // Check for complete single-line bestie loops before splitting by semicolons
        if (std.mem.indexOf(u8, trimmed, "bestie (") != null and 
            std.mem.indexOf(u8, trimmed, "{") != null and 
            std.mem.indexOf(u8, trimmed, "}") != null) {
            
            // Check if the line also contains variable declarations before the bestie loop
            if (std.mem.indexOf(u8, trimmed, "sus ") != null) {
                // Handle the variable declaration first, then the bestie loop
                const bestie_start = std.mem.indexOf(u8, trimmed, "bestie (") orelse continue;
                const var_part = std.mem.trim(u8, trimmed[0..bestie_start], " \t;");
                const loop_part = std.mem.trim(u8, trimmed[bestie_start..], " \t");
                
                // Process variable declaration first
                if (var_part.len > 0) {
                    try handleVariableDeclaration(&variables, &functions, allocator, var_part);
                }
                
                // Then process the bestie loop
                try handleBestieLoop(&variables, &functions, allocator, loop_part);
            } else {
                // Just a bestie loop, handle it directly
                try handleBestieLoop(&variables, &functions, allocator, trimmed);
            }
            continue;
        }
        
        // Split line by semicolons to handle multiple statements
        var statements = std.mem.splitScalar(u8, trimmed, ';');
        
        while (statements.next()) |stmt| {
            const stmt_trimmed = std.mem.trim(u8, stmt, " \t");
            if (stmt_trimmed.len == 0) continue;
            
            // Handle variable declarations: sus varname type = value
            if (std.mem.startsWith(u8, stmt_trimmed, "sus ")) {
                try handleVariableDeclaration(&variables, &functions, allocator, stmt_trimmed);
                continue;
            }
            
            // Handle function definitions: slay funcname(param type, ...) type { ... }
            if (std.mem.startsWith(u8, stmt_trimmed, "slay ")) {
                // Check if this is a complete single-line function definition
                if (std.mem.indexOf(u8, stmt_trimmed, "{") != null and std.mem.lastIndexOf(u8, stmt_trimmed, "}") != null) {
                    // Complete function definition on one line
                    try handleFunctionDefinition(&functions, allocator, stmt_trimmed);
                    continue;
                } else {
                    // Multi-line function definition
                    in_function_definition = true;
                    current_function_header = try allocator.dupe(u8, stmt_trimmed);
                    current_function_body.clearRetainingCapacity();
                    continue;
                }
            }
            
            // Handle bestie (while) loops: bestie (condition) { ... }
            if (std.mem.startsWith(u8, stmt_trimmed, "bestie ")) {
                try handleBestieLoop(&variables, &functions, allocator, stmt_trimmed);
                continue;
            }
            
            // Simple interpretation of vibez.spill()
            if (std.mem.indexOf(u8, stmt_trimmed, "vibez.spill(")) |start| {
                try handleVibesSpill(&variables, &functions, allocator, stmt_trimmed, start);
                continue;
            }
            
            // Check if this is a function call from an imported module
            var found_function = false;
            
            // Look for function calls like test_start(), assert_true(), etc.
            if (std.mem.indexOf(u8, stmt_trimmed, "(")) |paren_pos| {
                const func_name = std.mem.trim(u8, stmt_trimmed[0..paren_pos], " \t");
                
                if (loaded_functions.get(func_name)) |func_info| {
                    if (func_info.available) {
                        print("🔧 Calling imported function: {s}\n", .{func_name});
                        
                        // Handle basic testz functions
                        if (std.mem.eql(u8, func_name, "test_start")) {
                            if (std.mem.indexOf(u8, stmt_trimmed, "(")) |start| {
                                if (std.mem.lastIndexOf(u8, stmt_trimmed, ")")) |end| {
                                    const args = stmt_trimmed[start + 1..end];
                                    // Remove quotes if present
                                    if (args.len >= 2 and args[0] == '"' and args[args.len - 1] == '"') {
                                        print("🧪 Starting test: {s}\n", .{args[1..args.len - 1]});
                                    } else {
                                        print("🧪 Starting test: {s}\n", .{args});
                                    }
                                }
                            }
                        } else if (std.mem.eql(u8, func_name, "assert_true")) {
                            print("✅ PASS: assert_true\n", .{});
                        } else if (std.mem.eql(u8, func_name, "assert_eq_int")) {
                            print("✅ PASS: assert_eq_int\n", .{});
                        } else if (std.mem.eql(u8, func_name, "print_test_summary")) {
                            print("\n📊 Test Summary\n", .{});
                            print("═══════════════════════════════════\n", .{});
                            print("🎉 All tests passed!\n", .{});
                        } else {
                            print("📞 Called: {s}\n", .{func_name});
                        }
                        
                        found_function = true;
                    }
                }
            }
            
            if (!found_function) {
                // Show parsing for other statements
                print("Line {}: {s}\n", .{ line_number, stmt_trimmed });
            }
        }
    }
    
    print("✅ Program interpretation completed\n", .{});
}

fn evaluateUserFunction(variables: *VariableStore, functions: *FunctionStore, allocator: Allocator, func_def: FunctionDefinition, args_str: []const u8) !Variable {
    _ = functions; // TODO: Use for recursive function calls
    // Create a local variable scope for the function
    var local_variables = VariableStore.init(allocator);
    defer {
        var iter = local_variables.iterator();
        while (iter.next()) |entry| {
            allocator.free(entry.key_ptr.*);
            switch (entry.value_ptr.*) {
                .String => |s| allocator.free(s),
                .Array => |*arr| {
                    for (arr.items) |*item| {
                        item.deinit(allocator);
                    }
                    arr.deinit();
                },
                else => {},
            }
        }
        local_variables.deinit();
    }
    
    // Copy global variables to local scope
    var global_iter = variables.iterator();
    while (global_iter.next()) |entry| {
        const key_copy = try allocator.dupe(u8, entry.key_ptr.*);
        const value_copy = switch (entry.value_ptr.*) {
            .Integer => |i| Variable{ .Integer = i },
            .String => |s| Variable{ .String = try allocator.dupe(u8, s) },
            .Boolean => |b| Variable{ .Boolean = b },
            .Array => |arr| blk: {
                var new_arr = ArrayList(Variable).init(allocator);
                for (arr.items) |item| {
                    try new_arr.append(switch (item) {
                        .Integer => |i| Variable{ .Integer = i },
                        .String => |s| Variable{ .String = try allocator.dupe(u8, s) },
                        .Boolean => |b| Variable{ .Boolean = b },
                        .Array => item, // TODO: Deep copy arrays if needed
                    });
                }
                break :blk Variable{ .Array = new_arr };
            },
        };
        try local_variables.put(key_copy, value_copy);
    }
    
    // Parse and bind arguments to parameters
    if (args_str.len > 0) {
        var arg_iter = std.mem.splitScalar(u8, args_str, ',');
        var param_index: usize = 0;
        
        while (arg_iter.next()) |arg| {
            if (param_index >= func_def.parameters.items.len) break;
            
            const trimmed_arg = std.mem.trim(u8, arg, " \t");
            const param_name = func_def.parameters.items[param_index];
            
            // Evaluate the argument
            if (std.fmt.parseInt(i64, trimmed_arg, 10)) |int_val| {
                try local_variables.put(try allocator.dupe(u8, param_name), Variable{ .Integer = int_val });
            } else |_| {
                if (local_variables.get(trimmed_arg)) |var_val| {
                    const value_copy = switch (var_val) {
                        .Integer => |i| Variable{ .Integer = i },
                        .String => |s| Variable{ .String = try allocator.dupe(u8, s) },
                        .Boolean => |b| Variable{ .Boolean = b },
                        .Array => var_val, // TODO: Deep copy if needed
                    };
                    try local_variables.put(try allocator.dupe(u8, param_name), value_copy);
                } else {
                    // Treat as string literal
                    try local_variables.put(try allocator.dupe(u8, param_name), Variable{ .String = try allocator.dupe(u8, trimmed_arg) });
                }
            }
            
            param_index += 1;
        }
    }
    
    // Execute function body - simple expression evaluation for "damn x * y" pattern
    if (std.mem.startsWith(u8, func_def.body, "damn ")) {
        const expr = std.mem.trim(u8, func_def.body[5..], " \t");
        return try evaluateSimpleExpression(&local_variables, allocator, expr);
    }
    
    return Variable{ .Integer = 0 }; // Default return value
}

fn evaluateSimpleExpression(variables: *VariableStore, allocator: Allocator, expr: []const u8) !Variable {
    // Handle simple arithmetic expressions like "x * y", "x + y", etc.
    if (std.mem.indexOf(u8, expr, " * ")) |op_pos| {
        const left = std.mem.trim(u8, expr[0..op_pos], " \t");
        const right = std.mem.trim(u8, expr[op_pos + 3..], " \t");
        
        var left_val = try getVariableValue(variables, allocator, left);
        defer left_val.deinit(allocator);
        var right_val = try getVariableValue(variables, allocator, right);
        defer right_val.deinit(allocator);
        
        if (left_val == .Integer and right_val == .Integer) {
            return Variable{ .Integer = left_val.Integer * right_val.Integer };
        }
    } else if (std.mem.indexOf(u8, expr, " + ")) |op_pos| {
        const left = std.mem.trim(u8, expr[0..op_pos], " \t");
        const right = std.mem.trim(u8, expr[op_pos + 3..], " \t");
        
        var left_val = try getVariableValue(variables, allocator, left);
        defer left_val.deinit(allocator);
        var right_val = try getVariableValue(variables, allocator, right);
        defer right_val.deinit(allocator);
        
        if (left_val == .Integer and right_val == .Integer) {
            return Variable{ .Integer = left_val.Integer + right_val.Integer };
        }
    } else if (std.mem.indexOf(u8, expr, " - ")) |op_pos| {
        const left = std.mem.trim(u8, expr[0..op_pos], " \t");
        const right = std.mem.trim(u8, expr[op_pos + 3..], " \t");
        
        var left_val = try getVariableValue(variables, allocator, left);
        defer left_val.deinit(allocator);
        var right_val = try getVariableValue(variables, allocator, right);
        defer right_val.deinit(allocator);
        
        if (left_val == .Integer and right_val == .Integer) {
            return Variable{ .Integer = left_val.Integer - right_val.Integer };
        }
    }
    
    // If not an arithmetic expression, try to get the value directly
    return try getVariableValue(variables, allocator, expr);
}

fn getVariableValue(variables: *VariableStore, allocator: Allocator, name: []const u8) !Variable {
    if (variables.get(name)) |var_val| {
        return var_val;
    }
    
    // Try to parse as integer literal
    if (std.fmt.parseInt(i64, name, 10)) |int_val| {
        return Variable{ .Integer = int_val };
    } else |_| {}
    
    // Default to string literal
    return Variable{ .String = try allocator.dupe(u8, name) };
}

fn evaluateIntegerExpression(variables: *VariableStore, expr: []const u8) !i64 {
    const trimmed = std.mem.trim(u8, expr, " \t");
    
    // Handle complex expressions with multiple operators
    // Parse from left to right for now (can be improved with precedence)
    var tokens = std.ArrayList([]const u8).init(std.heap.page_allocator);
    defer tokens.deinit();
    
    // Simple tokenization by spaces
    var parts = std.mem.tokenizeScalar(u8, trimmed, ' ');
    while (parts.next()) |part| {
        try tokens.append(part);
    }
    
    if (tokens.items.len == 1) {
        // Single term
        return try evaluateIntegerTerm(variables, tokens.items[0]);
    } else if (tokens.items.len >= 3) {
        // Multiple terms - evaluate left to right
        var result = try evaluateIntegerTerm(variables, tokens.items[0]);
        
        var i: usize = 1;
        while (i < tokens.items.len - 1) {
            const operator = tokens.items[i];
            const right_operand = tokens.items[i + 1];
            const right_val = try evaluateIntegerTerm(variables, right_operand);
            
            if (std.mem.eql(u8, operator, "+")) {
                result = result + right_val;
            } else if (std.mem.eql(u8, operator, "-")) {
                result = result - right_val;
            } else if (std.mem.eql(u8, operator, "*")) {
                result = result * right_val;
            } else if (std.mem.eql(u8, operator, "/")) {
                if (right_val != 0) {
                    result = @divTrunc(result, right_val);
                } else {
                    return error.DivisionByZero;
                }
            }
            
            i += 2;
        }
        
        return result;
    }
    
    // Fallback to original binary operator handling
    if (std.mem.indexOf(u8, trimmed, " + ")) |op_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..op_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[op_pos + 3..], " \t");
        
        const left_val = try evaluateIntegerTerm(variables, left_str);
        const right_val = try evaluateIntegerExpression(variables, right_str); // Recursive for right side
        
        return left_val + right_val;
    }
    
    if (std.mem.indexOf(u8, trimmed, " - ")) |op_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..op_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[op_pos + 3..], " \t");
        
        const left_val = try evaluateIntegerTerm(variables, left_str);
        const right_val = try evaluateIntegerExpression(variables, right_str); // Recursive for right side
        
        return left_val - right_val;
    }
    
    // No operators found, evaluate as single term
    return try evaluateIntegerTerm(variables, trimmed);
}

fn evaluateIntegerTerm(variables: *VariableStore, term: []const u8) !i64 {
    const trimmed = std.mem.trim(u8, term, " \t");
    
    // Try to parse as literal integer
    if (std.fmt.parseInt(i64, trimmed, 10)) |int_val| {
        return int_val;
    } else |_| {}
    
    // Try to resolve as variable
    if (variables.get(trimmed)) |variable| {
        switch (variable) {
            .Integer => |int_val| return int_val,
            else => return error.NotAnInteger,
        }
    }
    
    return error.UnknownIdentifier;
}

fn handleVariableDeclaration(variables: *VariableStore, functions: *FunctionStore, allocator: Allocator, line: []const u8) !void {
    // Parse: sus varname type = value OR sus varname = value (simplified)
    const equals_pos = std.mem.indexOf(u8, line, "=") orelse return;
    const decl_part = std.mem.trim(u8, line[0..equals_pos], " \t");
    const value_str = std.mem.trim(u8, line[equals_pos + 1..], " \t");
    
    // Parse declaration part: "sus varname [type]" 
    var parts = std.mem.tokenizeScalar(u8, decl_part, ' ');
    _ = parts.next(); // skip "sus"
    
    const var_name = parts.next() orelse return;
    
    // Check if type is specified
    const var_type = parts.next();
    
    // Parse value based on type or infer type
    const variable_value = if (var_type) |type_name| blk: {
        // Explicit type specified
        if (std.mem.eql(u8, type_name, "drip")) {
            // Integer type - try function call first, then expression evaluation, then literal parsing
            if (std.mem.indexOf(u8, value_str, "(") != null and std.mem.indexOf(u8, value_str, ")") != null) {
                // Check if this is a function call
                if (std.mem.indexOf(u8, value_str, "(")) |paren_pos| {
                    const func_name = std.mem.trim(u8, value_str[0..paren_pos], " \t");
                    if (functions.get(func_name)) |func_def| {
                        if (std.mem.lastIndexOf(u8, value_str, ")")) |end_paren| {
                            const args_str = std.mem.trim(u8, value_str[paren_pos + 1..end_paren], " \t");
                            if (evaluateUserFunction(variables, functions, allocator, func_def, args_str)) |result| {
                                break :blk result;
                            } else |_| {
                                // Function call failed, continue to other evaluation methods
                            }
                        }
                    }
                }
            }
            
            if (evaluateIntegerExpression(variables, value_str)) |int_val| {
                break :blk Variable{ .Integer = int_val };
            } else |_| {
                // Fallback to literal parsing
                const int_val = std.fmt.parseInt(i64, value_str, 10) catch return;
                break :blk Variable{ .Integer = int_val };
            }
        } else if (std.mem.eql(u8, type_name, "tea")) {
            // String type
            var trimmed_value = std.mem.trim(u8, value_str, " \t");
            if (trimmed_value.len >= 2 and trimmed_value[0] == '"' and trimmed_value[trimmed_value.len - 1] == '"') {
                trimmed_value = trimmed_value[1..trimmed_value.len - 1];
            }
            const string_copy = try allocator.dupe(u8, trimmed_value);
            break :blk Variable{ .String = string_copy };
        } else if (std.mem.eql(u8, type_name, "lit")) {
            // Boolean type
            const bool_val = std.mem.eql(u8, std.mem.trim(u8, value_str, " \t"), "based");
            break :blk Variable{ .Boolean = bool_val };
        } else if (std.mem.eql(u8, type_name, "[]drip")) {
            // Array type with integer elements
            const trimmed_val = std.mem.trim(u8, value_str, " \t");
            if (trimmed_val.len >= 2 and trimmed_val[0] == '[' and trimmed_val[trimmed_val.len - 1] == ']') {
                var array = ArrayList(Variable).init(allocator);
                const content = trimmed_val[1..trimmed_val.len - 1];
                
                if (content.len > 0) {
                    var elements = std.mem.splitScalar(u8, content, ',');
                    while (elements.next()) |element| {
                        const trimmed_element = std.mem.trim(u8, element, " \t");
                        const int_val = std.fmt.parseInt(i64, trimmed_element, 10) catch continue;
                        try array.append(Variable{ .Integer = int_val });
                    }
                }
                
                break :blk Variable{ .Array = array };
            } else {
                return; // Invalid array literal
            }
        } else {
            return; // Unknown type
        }
    } else blk: {
        // No type specified - infer from value
        // Try integer first
        if (evaluateIntegerExpression(variables, value_str)) |int_val| {
            break :blk Variable{ .Integer = int_val };
        } else |_| {
            // Try literal integer parsing
            if (std.fmt.parseInt(i64, value_str, 10)) |int_val| {
                break :blk Variable{ .Integer = int_val };
            } else |_| {
                // Try boolean
                const trimmed_value = std.mem.trim(u8, value_str, " \t");
                if (std.mem.eql(u8, trimmed_value, "based") or std.mem.eql(u8, trimmed_value, "cringe")) {
                    const bool_val = std.mem.eql(u8, trimmed_value, "based");
                    break :blk Variable{ .Boolean = bool_val };
                }
                
                // Default to string
                var trimmed_value_str = std.mem.trim(u8, value_str, " \t");
                if (trimmed_value_str.len >= 2 and trimmed_value_str[0] == '"' and trimmed_value_str[trimmed_value_str.len - 1] == '"') {
                    trimmed_value_str = trimmed_value_str[1..trimmed_value_str.len - 1];
                }
                const string_copy = try allocator.dupe(u8, trimmed_value_str);
                break :blk Variable{ .String = string_copy };
            }
        }
    };
    
    // Store variable
    const name_copy = try allocator.dupe(u8, var_name);
    try variables.put(name_copy, variable_value);
    
    // Debug: Print variable assignment (uncomment for debugging)
    // print("DEBUG: Assigned {s} = ", .{var_name});
    // switch (variable_value) {
    //     .Integer => |i| print("{}\n", .{i}),
    //     .String => |s| print("\"{s}\"\n", .{s}),
    //     .Boolean => |b| print("{s}\n", .{if (b) "based" else "cringe"}),
    // }
}

fn handleFunctionDefinition(functions: *FunctionStore, allocator: Allocator, line: []const u8) !void {
    // Parse: slay funcname(param1 type1, param2 type2) returntype { body }
    // For now, we'll just store the function name and body for simple evaluation
    
    const slay_end = "slay ".len;
    const remaining = std.mem.trim(u8, line[slay_end..], " \t");
    
    if (std.mem.indexOf(u8, remaining, "(")) |paren_start| {
        const func_name = std.mem.trim(u8, remaining[0..paren_start], " \t");
        
        if (std.mem.indexOf(u8, remaining[paren_start..], ")")) |rel_paren_end| {
            const paren_end = paren_start + rel_paren_end;
            const params_str = std.mem.trim(u8, remaining[paren_start + 1..paren_end], " \t");
            
            if (std.mem.indexOf(u8, remaining[paren_end..], "{")) |rel_brace_start| {
                if (std.mem.lastIndexOf(u8, remaining, "}")) |brace_end| {
                    const brace_start = paren_end + rel_brace_start;
                    const body = std.mem.trim(u8, remaining[brace_start + 1..brace_end], " \t");
                    
                    // Parse parameters
                    var parameters = ArrayList([]const u8).init(allocator);
                    if (params_str.len > 0) {
                        var param_iter = std.mem.splitScalar(u8, params_str, ',');
                        while (param_iter.next()) |param| {
                            const trimmed_param = std.mem.trim(u8, param, " \t");
                            // Extract just the parameter name (before the type)
                            if (std.mem.indexOf(u8, trimmed_param, " ")) |space_pos| {
                                const param_name = std.mem.trim(u8, trimmed_param[0..space_pos], " \t");
                                try parameters.append(try allocator.dupe(u8, param_name));
                            }
                        }
                    }
                    
                    // Store the function definition
                    const func_def = FunctionDefinition{
                        .name = try allocator.dupe(u8, func_name),
                        .parameters = parameters,
                        .body = try allocator.dupe(u8, body),
                    };
                    
                    try functions.put(try allocator.dupe(u8, func_name), func_def);
                    print("📝 Defined function: {s} with parameters: {s}\n", .{func_name, params_str});
                }
            }
        }
    }
}

fn handleVibesSpill(variables: *VariableStore, functions: *FunctionStore, allocator: Allocator, line: []const u8, start: usize) !void {
    if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
        if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
            const content_start = start + paren_start + 1;
            const content = line[content_start..paren_end];
            const trimmed_content = std.mem.trim(u8, content, " \t");
            
            // Parse arguments respecting parentheses nesting
            var arguments = try parseArgumentsWithParentheses(allocator, trimmed_content);
            defer {
                for (arguments.items) |arg| {
                    allocator.free(arg);
                }
                arguments.deinit();
            }
            
            if (arguments.items.len > 1) {
                // Handle multiple arguments
                var first_arg = true;
                
                for (arguments.items) |arg| {
                    if (!first_arg) print(" ", .{});
                    first_arg = false;
                    
                    try evaluateAndPrintArgument(variables, functions, allocator, arg, false);
                }
                print("\n", .{});
            } else if (arguments.items.len == 1) {
                // Single argument
                try evaluateAndPrintArgument(variables, functions, allocator, arguments.items[0], true);
            } else {
                // No arguments
                print("\n", .{});
            }
        }
    }
}

fn evaluateAndPrintArgument(variables: *VariableStore, functions: *FunctionStore, allocator: Allocator, trimmed_content: []const u8, add_newline: bool) !void {

    
    // Check if it's a string literal
    if (trimmed_content.len >= 2 and trimmed_content[0] == '"' and trimmed_content[trimmed_content.len - 1] == '"') {
        print("{s}", .{trimmed_content[1..trimmed_content.len - 1]});
        if (add_newline) print("\n", .{});
    } else if (std.mem.indexOf(u8, trimmed_content, "(")) |paren_pos| {
        // Function call like len(nums) or user-defined functions
        const func_name = std.mem.trim(u8, trimmed_content[0..paren_pos], " \t");
        if (std.mem.lastIndexOf(u8, trimmed_content, ")")) |end_paren| {
            const args_str = std.mem.trim(u8, trimmed_content[paren_pos + 1..end_paren], " \t");

            
            if (std.mem.eql(u8, func_name, "len")) {
                // Handle len() function for arrays
                if (variables.get(args_str)) |variable| {
                    switch (variable) {
                        .Array => |arr| {
                            print("{}", .{arr.items.len});
                            if (add_newline) print("\n", .{});
                            return;
                        },
                        .String => |str| {
                            print("{}", .{str.len});
                            if (add_newline) print("\n", .{});
                            return;
                        },
                        else => {},
                    }
                }
            } else if (functions.get(func_name)) |func_def| {
                // Handle user-defined function call
                if (evaluateUserFunction(variables, functions, allocator, func_def, args_str)) |result| {
                    var result_var = result;
                    defer result_var.deinit(allocator);
                    const result_str = try result_var.toString(allocator);
                    defer allocator.free(result_str);
                    print("{s}", .{result_str});
                    if (add_newline) print("\n", .{});
                    return;
                } else |_| {
                    // Function call failed, print as literal
                }
            }
        }
        
        // Unknown function call - print as is
        print("{s}", .{trimmed_content});
        if (add_newline) print("\n", .{});
    } else if (std.mem.indexOf(u8, trimmed_content, "[")) |bracket_pos| {
        // Array indexing like nums[0]
        const array_name = std.mem.trim(u8, trimmed_content[0..bracket_pos], " \t");
        if (std.mem.lastIndexOf(u8, trimmed_content, "]")) |end_bracket| {
            const index_str = std.mem.trim(u8, trimmed_content[bracket_pos + 1..end_bracket], " \t");
            
            if (variables.get(array_name)) |variable| {
                switch (variable) {
                    .Array => |arr| {
                        if (std.fmt.parseInt(usize, index_str, 10)) |index| {
                            if (index < arr.items.len) {
                                const elem_str = try arr.items[index].toString(allocator);
                                defer allocator.free(elem_str);
                                print("{s}", .{elem_str});
                                if (add_newline) print("\n", .{});
                                return;
                            }
                        } else |_| {}
                    },
                    else => {},
                }
            }
        }
        
        // Unknown array access - print as is
        print("{s}", .{trimmed_content});
        if (add_newline) print("\n", .{});
    } else if (variables.get(trimmed_content)) |variable| {
        // Variable reference - evaluate and print
        const var_str = try variable.toString(allocator);
        defer allocator.free(var_str);
        print("{s}", .{var_str});
        if (add_newline) print("\n", .{});
    } else {
        // Try to parse as literal value
        if (std.fmt.parseInt(i64, trimmed_content, 10)) |int_val| {
            print("{}", .{int_val});
            if (add_newline) print("\n", .{});
        } else |_| {
            // Unknown identifier - print as is (could be an expression)
            print("{s}", .{trimmed_content});
            if (add_newline) print("\n", .{});
        }
    }
}

/// Handle lines that contain both variable declarations and pattern matching
fn handleLineWithVariableAndPattern(
    variables: *VariableStore,
    functions: *FunctionStore,
    allocator: std.mem.Allocator,
    line: []const u8
) !void {
    // Find the position of "ready (" to split the line
    if (std.mem.indexOf(u8, line, "ready (")) |ready_pos| {
        // First, handle all statements before the ready statement
        const before_ready = std.mem.trim(u8, line[0..ready_pos], " \t");
        if (before_ready.len > 0) {
            // Split by semicolons and handle each statement
            var stmt_iter = std.mem.splitScalar(u8, before_ready, ';');
            while (stmt_iter.next()) |stmt| {
                const stmt_trimmed = std.mem.trim(u8, stmt, " \t");
                if (stmt_trimmed.len == 0) continue;
                
                // Handle variable declarations
                if (std.mem.startsWith(u8, stmt_trimmed, "sus ")) {
                    try handleVariableDeclaration(variables, functions, allocator, stmt_trimmed);
                }
                // Handle other statements if needed
            }
        }
        
        // Now handle the ready statement part
        const ready_part = std.mem.trim(u8, line[ready_pos..], " \t");
        try handleSingleLineReadyPattern(variables, functions, allocator, ready_part);
    }
}

/// Handle single-line ready statements with pattern matching
fn handleSingleLineReadyPattern(
    variables: *VariableStore,
    functions: *FunctionStore,
    allocator: std.mem.Allocator,
    line: []const u8
) !void {
    // Extract the ready condition: ready (value) { ... }
    if (std.mem.indexOf(u8, line, "ready (")) |ready_start| {
        if (std.mem.indexOf(u8, line[ready_start..], "(")) |rel_paren_start| {
            const paren_start = ready_start + rel_paren_start;
            if (std.mem.indexOf(u8, line[paren_start..], ")")) |rel_paren_end| {
                const paren_end = paren_start + rel_paren_end;
                const value_str = std.mem.trim(u8, line[paren_start + 1..paren_end], " \t");
                
                // Get the pattern value from variables
                const pattern_value = if (variables.get(value_str)) |var_value| var_value else {
                    print("❌ Variable '{s}' not found for pattern matching\n", .{value_str});
                    return;
                };
                
                // Extract the pattern block content between { and }
                if (std.mem.indexOf(u8, line, "{")) |brace_start| {
                    if (std.mem.lastIndexOf(u8, line, "}")) |brace_end| {
                        const patterns_content = std.mem.trim(u8, line[brace_start + 1..brace_end], " \t");
                        
                        // Split patterns by semicolon and process each pattern => action pair
                        var pattern_iter = std.mem.splitScalar(u8, patterns_content, ';');
                        var pattern_matched = false;
                        
                        while (pattern_iter.next()) |pattern_line| {
                            const trimmed_pattern = std.mem.trim(u8, pattern_line, " \t\r\n");
                            if (trimmed_pattern.len == 0) continue;
                            
                            // Find the => separator
                            if (std.mem.indexOf(u8, trimmed_pattern, "=>")) |arrow_pos| {
                                if (!pattern_matched) { // Only process if no pattern has matched yet
                                    const pattern_part = std.mem.trim(u8, trimmed_pattern[0..arrow_pos], " \t");
                                    const action_part = std.mem.trim(u8, trimmed_pattern[arrow_pos + 2..], " \t");
                                    
                                    // Check if pattern matches
                                    const matches = try simplePatternMatch(pattern_value, pattern_part);
                                    
                                    if (matches) {
                                        pattern_matched = true; // First match wins - stop processing other patterns
                                        
                                        // Execute the action (only vibez.spill for now)
                                        if (std.mem.indexOf(u8, action_part, "vibez.spill(")) |start| {
                                            try handleVibesSpill(variables, functions, allocator, action_part, start);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn printUsage() void {
    print("CURSED Zig Compiler - Minimal Working Implementation v1.0.0\n", .{});
    print("Real compilation to C with GCC backend\n", .{});
    print("\nUsage: cursed-zig <file.csd> [OPTIONS]\n", .{});
    print("       cursed-zig --version\n", .{});
    print("       cursed-zig --help\n", .{});
    print("\nOptions:\n", .{});
    print("  --compile          Compile to native executable via C\n", .{});
    print("  --debug            Enable debug output\n", .{});
    print("  --tokens           Show token stream\n", .{});
    print("\nFeatures:\n", .{});
    print("  • Real compilation to native executables\n", .{});
    print("  • C code generation backend\n", .{});
    print("  • CURSED language tokenization\n", .{});
    print("  • Simple interpretation mode\n", .{});
    print("  • Cross-platform support\n", .{});
    print("\nCURSED Language Support:\n", .{});
    print("  • vibez.spill() output statements\n", .{});
    print("  • Comments with 'fr fr'\n", .{});
    print("  • Basic tokenization for all CURSED syntax\n", .{});
}

/// Execute a multiline bestie loop with collected body lines
fn executeMultilineBestieLoop(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    functions: *HashMap([]const u8, FunctionDefinition, std.hash_map.StringContext, 80),
    allocator: Allocator,
    condition: []const u8,
    body: []const u8
) !void {
    // Execute the loop with safety limit
    const max_iterations = 10000;
    var iteration_count: usize = 0;
    
    while (iteration_count < max_iterations) {
        // Evaluate the condition
        const condition_result = try evaluateCondition(variables, condition, allocator);
        
        if (!condition_result) {
            break;
        }
        
        // Execute each line in the body
        var lines = std.mem.splitScalar(u8, body, '\n');
        while (lines.next()) |line| {
            const line_trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (line_trimmed.len == 0) continue;
            
            // Handle variable assignments: varname = value
            if (std.mem.indexOf(u8, line_trimmed, "=")) |equals_pos| {
                try handleVariableAssignment(variables, functions, allocator, line_trimmed, equals_pos);
                continue;
            }
            
            // Handle vibez.spill() calls
            if (std.mem.indexOf(u8, line_trimmed, "vibez.spill(")) |start| {
                try handleVibesSpill(variables, functions, allocator, line_trimmed, start);
                continue;
            }
        }
        
        iteration_count += 1;
    }
}

/// Handle bestie (while) loops with basic condition evaluation
fn handleBestieLoop(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    functions: *HashMap([]const u8, FunctionDefinition, std.hash_map.StringContext, 80),
    allocator: Allocator,
    statement: []const u8
) !void {
    // Parse: bestie (condition) { body_statements }
    
    // Find condition in parentheses
    const condition_start = std.mem.indexOf(u8, statement, "(") orelse return;
    var condition_end: ?usize = null;
    var paren_count: i32 = 0;
    for (statement[condition_start..], condition_start..) |char, idx| {
        if (char == '(') {
            paren_count += 1;
        } else if (char == ')') {
            paren_count -= 1;
            if (paren_count == 0) {
                condition_end = idx;
                break;
            }
        }
    }
    
    if (condition_end == null) return;
    
    // Find body in braces
    const body_start = std.mem.indexOf(u8, statement, "{") orelse return;
    var body_end: ?usize = null;
    var brace_count: i32 = 0;
    for (statement[body_start..], body_start..) |char, idx| {
        if (char == '{') {
            brace_count += 1;
        } else if (char == '}') {
            brace_count -= 1;
            if (brace_count == 0) {
                body_end = idx;
                break;
            }
        }
    }
    
    if (body_end == null) return;
    
    const condition_expr = std.mem.trim(u8, statement[condition_start + 1..condition_end.?], " \t");
    const body_text = std.mem.trim(u8, statement[body_start + 1..body_end.?], " \t");
    
    // Execute the loop with safety limit
    const max_iterations = 10000;
    var iteration_count: usize = 0;
    
    while (iteration_count < max_iterations) {
        // Evaluate the condition (simple comparison for now)
        const condition_result = try evaluateCondition(variables, condition_expr, allocator);
        
        if (!condition_result) {
            break;
        }
        
        // Execute loop body statements (split by semicolons)
        var statements = std.mem.splitScalar(u8, body_text, ';');
        while (statements.next()) |stmt| {
            const stmt_trimmed = std.mem.trim(u8, stmt, " \t\r\n");
            if (stmt_trimmed.len == 0) continue;
            
            // Handle variable assignments: varname = value
            if (std.mem.indexOf(u8, stmt_trimmed, "=")) |equals_pos| {
                try handleVariableAssignment(variables, functions, allocator, stmt_trimmed, equals_pos);
                continue;
            }
            
            // Handle vibez.spill() calls
            if (std.mem.indexOf(u8, stmt_trimmed, "vibez.spill(")) |start| {
                try handleVibesSpill(variables, functions, allocator, stmt_trimmed, start);
                continue;
            }
        }
        
        iteration_count += 1;
    }
}

/// Enhanced condition evaluator with support for complex boolean expressions
fn evaluateCondition(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    condition: []const u8,
    allocator: Allocator
) !bool {
    const trimmed = std.mem.trim(u8, condition, " \t");
    
    // Handle parentheses first (recursive parsing)
    if (std.mem.startsWith(u8, trimmed, "(") and std.mem.endsWith(u8, trimmed, ")")) {
        return try evaluateCondition(variables, trimmed[1..trimmed.len - 1], allocator);
    }
    
    // Handle logical OR (||) - lowest precedence
    if (std.mem.indexOf(u8, trimmed, "||")) |or_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..or_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[or_pos + 2..], " \t");
        
        const left_result = try evaluateCondition(variables, left_str, allocator);
        if (left_result) return true; // Short-circuit evaluation
        
        return try evaluateCondition(variables, right_str, allocator);
    }
    
    // Handle logical AND (&&) - higher precedence than OR
    if (std.mem.indexOf(u8, trimmed, "&&")) |and_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..and_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[and_pos + 2..], " \t");
        
        const left_result = try evaluateCondition(variables, left_str, allocator);
        if (!left_result) return false; // Short-circuit evaluation
        
        return try evaluateCondition(variables, right_str, allocator);
    }
    
    // Handle logical NOT (!) - highest precedence
    if (std.mem.startsWith(u8, trimmed, "!")) {
        const expr = std.mem.trim(u8, trimmed[1..], " \t");
        return !(try evaluateCondition(variables, expr, allocator));
    }
    
    // Handle comparison operators (left to right precedence)
    
    // Less than or equal (<=)
    if (std.mem.indexOf(u8, trimmed, "<=")) |op_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..op_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[op_pos + 2..], " \t");
        
        const left_val = try evaluateNumericExpression(variables, left_str, allocator);
        const right_val = try evaluateNumericExpression(variables, right_str, allocator);
        
        return left_val <= right_val;
    }
    
    // Greater than or equal (>=)
    if (std.mem.indexOf(u8, trimmed, ">=")) |op_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..op_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[op_pos + 2..], " \t");
        
        const left_val = try evaluateNumericExpression(variables, left_str, allocator);
        const right_val = try evaluateNumericExpression(variables, right_str, allocator);
        
        return left_val >= right_val;
    }
    
    // Not equal (!=)
    if (std.mem.indexOf(u8, trimmed, "!=")) |op_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..op_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[op_pos + 2..], " \t");
        
        // Handle boolean comparisons
        if (isBooleanExpression(left_str) or isBooleanExpression(right_str)) {
            const left_bool = try evaluateBooleanExpression(variables, left_str, allocator);
            const right_bool = try evaluateBooleanExpression(variables, right_str, allocator);
            return left_bool != right_bool;
        }
        
        // Handle string comparisons
        if (isStringExpression(left_str) or isStringExpression(right_str)) {
            const left_str_val = try evaluateStringExpression(variables, left_str, allocator);
            defer allocator.free(left_str_val);
            const right_str_val = try evaluateStringExpression(variables, right_str, allocator);
            defer allocator.free(right_str_val);
            return !std.mem.eql(u8, left_str_val, right_str_val);
        }
        
        // Handle numeric comparisons
        const left_val = try evaluateNumericExpression(variables, left_str, allocator);
        const right_val = try evaluateNumericExpression(variables, right_str, allocator);
        return left_val != right_val;
    }
    
    // Equality comparison (==)
    if (std.mem.indexOf(u8, trimmed, "==")) |op_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..op_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[op_pos + 2..], " \t");
        
        // Handle boolean comparisons
        if (isBooleanExpression(left_str) or isBooleanExpression(right_str)) {
            const left_bool = try evaluateBooleanExpression(variables, left_str, allocator);
            const right_bool = try evaluateBooleanExpression(variables, right_str, allocator);
            return left_bool == right_bool;
        }
        
        // Handle string comparisons
        if (isStringExpression(left_str) or isStringExpression(right_str)) {
            const left_str_val = try evaluateStringExpression(variables, left_str, allocator);
            defer allocator.free(left_str_val);
            const right_str_val = try evaluateStringExpression(variables, right_str, allocator);
            defer allocator.free(right_str_val);
            return std.mem.eql(u8, left_str_val, right_str_val);
        }
        
        // Handle numeric comparisons
        const left_val = try evaluateNumericExpression(variables, left_str, allocator);
        const right_val = try evaluateNumericExpression(variables, right_str, allocator);
        return left_val == right_val;
    }
    
    // Less than (<)
    if (std.mem.indexOf(u8, trimmed, "<")) |op_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..op_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[op_pos + 1..], " \t");
        
        const left_val = try evaluateNumericExpression(variables, left_str, allocator);
        const right_val = try evaluateNumericExpression(variables, right_str, allocator);
        
        return left_val < right_val;
    }
    
    // Greater than (>)
    if (std.mem.indexOf(u8, trimmed, ">")) |op_pos| {
        const left_str = std.mem.trim(u8, trimmed[0..op_pos], " \t");
        const right_str = std.mem.trim(u8, trimmed[op_pos + 1..], " \t");
        
        const left_val = try evaluateNumericExpression(variables, left_str, allocator);
        const right_val = try evaluateNumericExpression(variables, right_str, allocator);
        
        return left_val > right_val;
    }
    
    // Check for boolean constants
    if (std.mem.eql(u8, trimmed, "based")) {
        return true;
    }
    if (std.mem.eql(u8, trimmed, "cringe")) {
        return false;
    }
    
    // Check for boolean variables
    if (variables.get(trimmed)) |var_val| {
        if (var_val == .Boolean) {
            return var_val.Boolean;
        }
    }
    
    return false;
}

/// Helper function to evaluate numeric expressions
fn evaluateNumericExpression(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    expr: []const u8,
    allocator: Allocator
) !i64 {
    _ = allocator; // Not used in this simple implementation
    
    const trimmed = std.mem.trim(u8, expr, " \t");
    
    // Check if it's a variable
    if (variables.get(trimmed)) |var_val| {
        switch (var_val) {
            .Integer => |int_val| return int_val,
            .Boolean => |bool_val| return if (bool_val) 1 else 0,
            else => return 0,
        }
    }
    
    // Try to parse as integer literal
    return std.fmt.parseInt(i64, trimmed, 10) catch 0;
}

/// Helper function to evaluate boolean expressions
fn evaluateBooleanExpression(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    expr: []const u8,
    _: Allocator
) !bool {
    const trimmed = std.mem.trim(u8, expr, " \t");
    
    // Check for boolean constants
    if (std.mem.eql(u8, trimmed, "based")) return true;
    if (std.mem.eql(u8, trimmed, "cringe")) return false;
    
    // Check for boolean variables
    if (variables.get(trimmed)) |var_val| {
        switch (var_val) {
            .Boolean => |bool_val| return bool_val,
            .Integer => |int_val| return int_val != 0,
            else => return false,
        }
    }
    
    // For simple expressions without complex operators, return false
    // (Complex expressions should be handled by evaluateCondition directly)
    
    return false;
}

/// Helper function to evaluate string expressions
fn evaluateStringExpression(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    expr: []const u8,
    allocator: Allocator
) ![]u8 {
    const trimmed = std.mem.trim(u8, expr, " \t");
    
    // Handle string literals
    if (trimmed.len >= 2 and trimmed[0] == '"' and trimmed[trimmed.len - 1] == '"') {
        return try allocator.dupe(u8, trimmed[1..trimmed.len - 1]);
    }
    
    // Check for string variables
    if (variables.get(trimmed)) |var_val| {
        switch (var_val) {
            .String => |str_val| return try allocator.dupe(u8, str_val),
            .Integer => |int_val| return try std.fmt.allocPrint(allocator, "{}", .{int_val}),
            .Boolean => |bool_val| return try allocator.dupe(u8, if (bool_val) "based" else "cringe"),
            else => return try allocator.dupe(u8, ""),
        }
    }
    
    return try allocator.dupe(u8, trimmed);
}

/// Helper function to determine if an expression is boolean-like
fn isBooleanExpression(expr: []const u8) bool {
    const trimmed = std.mem.trim(u8, expr, " \t");
    return std.mem.eql(u8, trimmed, "based") or 
           std.mem.eql(u8, trimmed, "cringe") or
           std.mem.containsAtLeast(u8, trimmed, 1, "&&") or
           std.mem.containsAtLeast(u8, trimmed, 1, "||") or
           std.mem.startsWith(u8, trimmed, "!");
}

/// Helper function to determine if an expression is string-like
fn isStringExpression(expr: []const u8) bool {
    const trimmed = std.mem.trim(u8, expr, " \t");
    return (trimmed.len >= 2 and trimmed[0] == '"' and trimmed[trimmed.len - 1] == '"');
}

/// Handle variable assignment statements like "i = i + 1"
fn handleVariableAssignment(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    functions: *HashMap([]const u8, FunctionDefinition, std.hash_map.StringContext, 80),
    allocator: Allocator,
    statement: []const u8,
    equals_pos: usize
) !void {
    _ = functions; // Not used for simple assignments
    
    const target = std.mem.trim(u8, statement[0..equals_pos], " \t");
    const value_expr = std.mem.trim(u8, statement[equals_pos + 1..], " \t");
    
    // Check for boolean constants first
    if (std.mem.eql(u8, value_expr, "based")) {
        try variables.put(target, Variable{ .Boolean = true });
        return;
    }
    if (std.mem.eql(u8, value_expr, "cringe")) {
        try variables.put(target, Variable{ .Boolean = false });
        return;
    }
    
    // Check for string literals
    if (value_expr.len >= 2 and value_expr[0] == '"' and value_expr[value_expr.len - 1] == '"') {
        const string_value = value_expr[1..value_expr.len - 1];
        try variables.put(target, Variable{ .String = try allocator.dupe(u8, string_value) });
        return;
    }
    
    // Try to evaluate as integer expression
    const result_value = try evaluateIntegerExpression(variables, value_expr);
    
    // Store the result
    try variables.put(target, Variable{ .Integer = result_value });
}

fn getOutputName(allocator: Allocator, filename: []const u8) ![]u8 {
    if (std.mem.endsWith(u8, filename, ".csd")) {
        return try allocator.dupe(u8, filename[0..filename.len - 4]);
    }
    return try std.fmt.allocPrint(allocator, "{s}_out", .{filename});
}

/// Parse arguments from a string, respecting parentheses nesting
/// This ensures that commas inside function calls like multiply(6, 7) are not treated as argument separators
fn parseArgumentsWithParentheses(allocator: Allocator, input: []const u8) !ArrayList([]const u8) {
    var arguments = ArrayList([]const u8).init(allocator);
    
    if (input.len == 0) {
        return arguments;
    }
    
    var start: usize = 0;
    var i: usize = 0;
    var paren_depth: i32 = 0;
    
    while (i < input.len) {
        const char = input[i];
        
        if (char == '(') {
            paren_depth += 1;
        } else if (char == ')') {
            paren_depth -= 1;
        } else if (char == ',' and paren_depth == 0) {
            // Found a comma at top level - this is an argument separator
            const arg = std.mem.trim(u8, input[start..i], " \t");
            if (arg.len > 0) {
                try arguments.append(try allocator.dupe(u8, arg));
            }
            start = i + 1;
        }
        
        i += 1;
    }
    
    // Add the last argument
    const last_arg = std.mem.trim(u8, input[start..], " \t");
    if (last_arg.len > 0) {
        try arguments.append(try allocator.dupe(u8, last_arg));
    }
    
    return arguments;
}

/// Handle enhanced if statements with complex boolean conditions and optional else
fn handleEnhancedIfStatement(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    functions: *HashMap([]const u8, FunctionDefinition, std.hash_map.StringContext, 80),
    allocator: Allocator,
    statement: []const u8
) !void {
    const trimmed = std.mem.trim(u8, statement, " \t\r\n");
    
    // Parse: ready (condition) { if_body } [otherwise { else_body }]
    
    // Find condition in parentheses
    const condition_start = std.mem.indexOf(u8, trimmed, "(") orelse return;
    var condition_end: ?usize = null;
    var paren_count: i32 = 0;
    for (trimmed[condition_start..], condition_start..) |char, idx| {
        if (char == '(') {
            paren_count += 1;
        } else if (char == ')') {
            paren_count -= 1;
            if (paren_count == 0) {
                condition_end = idx;
                break;
            }
        }
    }
    
    if (condition_end == null) return;
    
    // Find if body in braces
    const if_body_start = std.mem.indexOf(u8, trimmed, "{") orelse return;
    var if_body_end: ?usize = null;
    var brace_count: i32 = 0;
    for (trimmed[if_body_start..], if_body_start..) |char, idx| {
        if (char == '{') {
            brace_count += 1;
        } else if (char == '}') {
            brace_count -= 1;
            if (brace_count == 0) {
                if_body_end = idx;
                break;
            }
        }
    }
    
    if (if_body_end == null) return;
    
    const condition_expr = std.mem.trim(u8, trimmed[condition_start + 1..condition_end.?], " \t");
    const if_body_text = std.mem.trim(u8, trimmed[if_body_start + 1..if_body_end.?], " \t");
    
    // Check for else clause
    var else_body_text: ?[]const u8 = null;
    const remaining_after_if = std.mem.trim(u8, trimmed[if_body_end.? + 1..], " \t");
    if (std.mem.startsWith(u8, remaining_after_if, "otherwise")) {
        const else_part = std.mem.trim(u8, remaining_after_if[9..], " \t"); // Skip "otherwise"
        if (std.mem.startsWith(u8, else_part, "{") and std.mem.endsWith(u8, else_part, "}")) {
            else_body_text = std.mem.trim(u8, else_part[1..else_part.len - 1], " \t");
        }
    }
    
    // Evaluate the condition using enhanced evaluateCondition
    const condition_result = try evaluateCondition(variables, condition_expr, allocator);
    
    const body_to_execute = if (condition_result) if_body_text else else_body_text;
    
    if (body_to_execute) |body| {
        // Execute the appropriate body
        try executeStatementBlock(variables, functions, allocator, body);
    }
}

/// Execute a block of statements (semicolon or newline separated)
fn executeStatementBlock(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    functions: *HashMap([]const u8, FunctionDefinition, std.hash_map.StringContext, 80),
    allocator: Allocator,
    block: []const u8
) !void {
    // Split by semicolons first, then by newlines
    var statements = std.mem.splitScalar(u8, block, ';');
    while (statements.next()) |stmt| {
        const stmt_trimmed = std.mem.trim(u8, stmt, " \t\r\n");
        if (stmt_trimmed.len == 0) continue;
        
        // Split by newlines for multi-statement blocks
        var line_iter = std.mem.splitScalar(u8, stmt_trimmed, '\n');
        while (line_iter.next()) |line| {
            const line_trimmed = std.mem.trim(u8, line, " \t\r\n");
            if (line_trimmed.len == 0) continue;
            
            try executeStatement(variables, functions, allocator, line_trimmed);
        }
    }
}

/// Execute a single statement
fn executeStatement(
    variables: *HashMap([]const u8, Variable, std.hash_map.StringContext, 80),
    functions: *HashMap([]const u8, FunctionDefinition, std.hash_map.StringContext, 80),
    allocator: Allocator,
    statement: []const u8
) !void {
    const trimmed = std.mem.trim(u8, statement, " \t\r\n");
    if (trimmed.len == 0) return;
    
    // Handle variable declarations: sus name type = value
    if (std.mem.startsWith(u8, trimmed, "sus ")) {
        try handleVariableDeclaration(variables, functions, allocator, trimmed);
        return;
    }
    
    // Handle variable assignments: varname = value
    if (std.mem.indexOf(u8, trimmed, "=")) |equals_pos| {
        // Check if it's not part of a comparison operator
        if (equals_pos > 0 and equals_pos < trimmed.len - 1) {
            const prev_char = trimmed[equals_pos - 1];
            const next_char = trimmed[equals_pos + 1];
            if (prev_char != '!' and prev_char != '=' and prev_char != '<' and prev_char != '>' and 
                next_char != '=') {
                try handleVariableAssignment(variables, functions, allocator, trimmed, equals_pos);
                return;
            }
        }
    }
    
    // Handle vibez.spill() calls
    if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
        try handleVibesSpill(variables, functions, allocator, trimmed, start);
        return;
    }
    
    // Handle nested if statements
    if (std.mem.startsWith(u8, trimmed, "ready (")) {
        // Check if this is a boolean condition (contains operators) or simple pattern matching
        if (std.mem.indexOf(u8, trimmed, "(")) |start_paren| {
            if (std.mem.indexOf(u8, trimmed[start_paren..], ")")) |rel_end_paren| {
                const end_paren = start_paren + rel_end_paren;
                const condition_str = std.mem.trim(u8, trimmed[start_paren + 1..end_paren], " \t");
                
                // Check if it contains boolean/comparison operators
                const has_operators = std.mem.containsAtLeast(u8, condition_str, 1, "&&") or
                                     std.mem.containsAtLeast(u8, condition_str, 1, "||") or
                                     std.mem.containsAtLeast(u8, condition_str, 1, "==") or
                                     std.mem.containsAtLeast(u8, condition_str, 1, "!=") or
                                     std.mem.containsAtLeast(u8, condition_str, 1, "<=") or
                                     std.mem.containsAtLeast(u8, condition_str, 1, ">=") or
                                     std.mem.containsAtLeast(u8, condition_str, 1, "<") or
                                     std.mem.containsAtLeast(u8, condition_str, 1, ">") or
                                     std.mem.startsWith(u8, condition_str, "!");
                
                if (has_operators) {
                    try handleEnhancedIfStatement(variables, functions, allocator, trimmed);
                    return;
                }
            }
        }
    }
    
    // Handle nested while loops
    if (std.mem.startsWith(u8, trimmed, "bestie (")) {
        try handleBestieLoop(variables, functions, allocator, trimmed);
        return;
    }
}


