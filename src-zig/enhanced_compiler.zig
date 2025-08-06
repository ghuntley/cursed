const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");

/// Enhanced CURSED compiler that supports both C backend and LLVM backend compilation
pub const CompilationBackend = enum {
    C_Backend,
    LLVM_Backend,
};

pub const CompilerConfig = struct {
    backend: CompilationBackend = .C_Backend,
    optimization_level: u8 = 2,
    verbose: bool = false,
    output_path: ?[]const u8 = null,
};

const VariableInfo = struct {
    name: []const u8,
    var_type: []const u8, // "drip", "tea", "lit", "meal"
};

const LLVMVariableInfo = struct {
    llvm_type: []const u8,
    var_name: []const u8,
};

/// Compile CURSED source code to native executable using specified backend
pub fn compileProgram(allocator: Allocator, source: []const u8, filename: []const u8, config: CompilerConfig) !void {
    if (config.verbose) print("🔥 Compiling CURSED program to native executable...\n", .{});
    
    switch (config.backend) {
        .C_Backend => try compileToCBackend(allocator, source, filename, config),
        .LLVM_Backend => {
            const output_filename = config.output_path orelse blk: {
                if (std.mem.endsWith(u8, filename, ".csd"))
                    break :blk try std.fmt.allocPrint(allocator, "{s}.ll", .{filename[0..filename.len - 4]})
                else
                    break :blk try std.fmt.allocPrint(allocator, "{s}.ll", .{filename});
            };
            defer if (config.output_path == null) allocator.free(output_filename);
            try compileToLLVMBackend(allocator, source, filename, output_filename, config.verbose);
        },
    }
}

/// C Backend compilation (existing approach)
fn compileToCBackend(allocator: Allocator, source: []const u8, filename: []const u8, config: CompilerConfig) !void {
    // Step 1: Lexical Analysis
    print("[1/5] Lexical Analysis...\n", .{});
    var l = lexer.Lexer.init(allocator, source);
    const tokens = l.tokenize() catch |err| {
        print("❌ Lexer error during compilation: {}\n", .{err});
        return;
    };
    defer tokens.deinit();
    
    if (config.verbose) print("📝 Lexed {} tokens for compilation\n", .{tokens.items.len});
    
    // Step 2: Generate C code
    print("[2/5] Generating C code...\n", .{});
    
    const c_filename = try std.fmt.allocPrint(allocator, "{s}.c", .{filename[0..filename.len - 4]});
    defer allocator.free(c_filename);
    
    const c_file = std.fs.cwd().createFile(c_filename, .{}) catch |err| {
        print("❌ Error creating C file: {}\n", .{err});
        return;
    };
    defer c_file.close();
    
    const writer = c_file.writer();
    try writer.writeAll("#include <stdio.h>\n");
    try writer.writeAll("#include <stdlib.h>\n");
    try writer.writeAll("#include <string.h>\n");
    try writer.writeAll("int main() {\n");
    
    // Step 3: Enhanced CURSED-to-C translation with better parsing
    print("[3/5] Translating CURSED to C...\n", .{});
    try translateCursedToC(allocator, source, writer, config.verbose);
    
    try writer.writeAll("    return 0;\n");
    try writer.writeAll("}\n");
    
    if (config.verbose) print("✅ Generated C code: {s}\n", .{c_filename});
    
    // Step 4: Compile C code with optimization
    print("[4/5] Compiling C to executable...\n", .{});
    
    const output_filename = config.output_path orelse blk: {
        if (std.mem.endsWith(u8, filename, ".csd"))
            break :blk try std.fmt.allocPrint(allocator, "{s}", .{filename[0..filename.len - 4]})
        else
            break :blk try std.fmt.allocPrint(allocator, "{s}_compiled", .{filename});
    };
    defer if (config.output_path == null) allocator.free(output_filename);
    
    const optimization_flag = try std.fmt.allocPrint(allocator, "-O{}", .{config.optimization_level});
    defer allocator.free(optimization_flag);
    
    var child = std.process.Child.init(&[_][]const u8{ 
        "gcc", optimization_flag, "-o", output_filename, c_filename 
    }, allocator);
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;
    
    child.spawn() catch |err| {
        print("❌ Error spawning GCC: {}\n", .{err});
        return;
    };
    
    const result = child.wait() catch |err| {
        print("❌ Error waiting for GCC: {}\n", .{err});
        return;
    };
    
    switch (result) {
        .Exited => |code| {
            if (code != 0) {
                print("❌ Compilation failed with exit code: {}\n", .{code});
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

/// LLVM Backend compilation (proper LLVM IR generation)
pub fn compileToLLVMBackend(allocator: Allocator, source: []const u8, filename: []const u8, output_filename: []const u8, verbose: bool) !void {
    _ = filename;
    if (verbose) print("[1/4] Generating LLVM IR...\n", .{});
    
    // Create LLVM IR file with the specified output filename
    const ir_file = std.fs.cwd().createFile(output_filename, .{}) catch |err| {
        print("❌ Error creating LLVM IR file: {}\n", .{err});
        return;
    };
    defer ir_file.close();
    
    const writer = ir_file.writer();
    
    if (verbose) print("[2/4] Translating CURSED to LLVM IR...\n", .{});
    try generateProperLLVMIR(allocator, source, writer, verbose);
    
    if (verbose) print("✅ Generated LLVM IR: {s}\n", .{output_filename});
}

/// Enhanced CURSED-to-C translation with better parsing
fn translateCursedToC(allocator: Allocator, source: []const u8, writer: anytype, verbose: bool) !void {
    var variables = std.ArrayList(VariableInfo).init(allocator);
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
        
        // Handle vibez.spill() statements with better variable resolution
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            try translateVibesSpillToC(trimmed, start, writer, &variables);
        }
        
        // Handle variable declarations with type checking
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            try translateVariableDeclarationToC(allocator, trimmed, writer, &variables, verbose);
        }
    }
}

/// Generate LLVM IR header with necessary declarations
fn generateLLVMHeader(writer: anytype) !void {
    try writer.writeAll("; Generated LLVM IR for CURSED program\n");
    try writer.writeAll("target triple = \"x86_64-pc-linux-gnu\"\n\n");
    
    // External function declarations
    try writer.writeAll("declare i32 @puts(i8*)\n");
    try writer.writeAll("declare i32 @printf(i8*, ...)\n\n");
    
    // String format constants
    try writer.writeAll("@.int_fmt = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\", align 1\n");
    try writer.writeAll("@.bool_true = private unnamed_addr constant [6 x i8] c\"based\\00\", align 1\n");
    try writer.writeAll("@.bool_false = private unnamed_addr constant [7 x i8] c\"cringe\\00\", align 1\n\n");
}

/// Enhanced CURSED-to-LLVM IR translation
fn translateCursedToLLVM(allocator: Allocator, source: []const u8, writer: anytype, verbose: bool) !void {
    var variables = std.ArrayList(VariableInfo).init(allocator);
    defer {
        for (variables.items) |var_info| {
            allocator.free(var_info.name);
            allocator.free(var_info.var_type);
        }
        variables.deinit();
    }
    
    var string_constants = std.ArrayList([]const u8).init(allocator);
    defer {
        for (string_constants.items) |str| {
            allocator.free(str);
        }
        string_constants.deinit();
    }
    
    var string_counter: u32 = 0;
    
    // First pass: collect all string literals
    var lines = std.mem.splitScalar(u8, source, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            if (std.mem.indexOf(u8, trimmed[start..], "(")) |paren_start| {
                if (std.mem.lastIndexOf(u8, trimmed, ")")) |paren_end| {
                    const content_start = start + paren_start + 1;
                    const content = trimmed[content_start..paren_end];
                    
                    if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                        const string_content = content[1..content.len - 1];
                        const string_copy = try allocator.dupe(u8, string_content);
                        try string_constants.append(string_copy);
                    }
                }
            }
        }
    }
    
    // Generate string constants
    for (string_constants.items, 0..) |str_content, i| {
        try writer.print("@.str{} = private unnamed_addr constant [{} x i8] c\"{s}\\00\", align 1\n", 
            .{ i, str_content.len + 1, str_content });
    }
    try writer.writeAll("\n");
    
    // Start main function
    try writer.writeAll("define i32 @main() {\n");
    try writer.writeAll("entry:\n");
    
    // Second pass: generate code
    lines = std.mem.splitScalar(u8, source, '\n');
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
            try translateVibesSpillToLLVMWithConstants(allocator, trimmed, start, writer, &variables, &string_counter, &string_constants);
        }
        
        // Handle variable declarations
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            try translateVariableDeclarationToLLVM(allocator, trimmed, writer, &variables, verbose);
        }
    }
    
    try writer.writeAll("  ret i32 0\n");
    try writer.writeAll("}\n");
}

/// Generate LLVM IR footer
fn generateLLVMFooter(writer: anytype) !void {
    _ = writer;
    // Footer is handled in translateCursedToLLVM
}

/// Translate vibez.spill() to C printf
fn translateVibesSpillToC(line: []const u8, start: usize, writer: anytype, variables: *std.ArrayList(VariableInfo)) !void {
    if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
        if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
            const content_start = start + paren_start + 1;
            const content = line[content_start..paren_end];
            
            if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                // String literal
                try writer.print("    printf({s}\"\\n\");\n", .{content});
            } else {
                // Variable reference or literal value
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

/// Translate variable declaration to C
fn translateVariableDeclarationToC(allocator: Allocator, line: []const u8, writer: anytype, variables: *std.ArrayList(VariableInfo), verbose: bool) !void {
    var parts = std.mem.tokenizeScalar(u8, line, ' ');
    _ = parts.next(); // skip "sus"
    
    const var_name = parts.next() orelse return;
    const var_type = parts.next() orelse return;
    const equals = parts.next() orelse return;
    
    if (!std.mem.eql(u8, equals, "=")) return;
    
    const value_str = parts.rest();
    
    // Add variable to tracking list
    const var_name_copy = try allocator.dupe(u8, var_name);
    const var_type_copy = try allocator.dupe(u8, var_type);
    try variables.append(VariableInfo{ .name = var_name_copy, .var_type = var_type_copy });
    
    if (verbose) {
        try writer.print("    // Variable: {s} {s} = {s}\n", .{ var_name, var_type, value_str });
    }
    
    if (std.mem.eql(u8, var_type, "drip")) {
        try writer.print("    long {s} = {s};\n", .{ var_name, value_str });
    } else if (std.mem.eql(u8, var_type, "tea")) {
        if (value_str.len >= 2 and value_str[0] == '"' and value_str[value_str.len - 1] == '"') {
            try writer.print("    char* {s} = {s};\n", .{ var_name, value_str });
        } else {
            try writer.print("    char* {s} = \"{s}\";\n", .{ var_name, value_str });
        }
    } else if (std.mem.eql(u8, var_type, "lit")) {
        const c_bool = if (std.mem.eql(u8, std.mem.trim(u8, value_str, " \t"), "based")) "1" else "0";
        try writer.print("    int {s} = {s};\n", .{ var_name, c_bool });
    } else if (std.mem.eql(u8, var_type, "meal")) {
        try writer.print("    double {s} = {s};\n", .{ var_name, value_str });
    }
}

/// Translate vibez.spill() to LLVM IR with pre-declared constants
fn translateVibesSpillToLLVMWithConstants(allocator: Allocator, line: []const u8, start: usize, writer: anytype, variables: *std.ArrayList(VariableInfo), string_counter: *u32, string_constants: *std.ArrayList([]const u8)) !void {
    _ = allocator;
    if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
        if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
            const content_start = start + paren_start + 1;
            const content = line[content_start..paren_end];
            
            if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                // String literal - find it in our pre-declared constants
                const string_content = content[1..content.len - 1];
                var const_index: ?usize = null;
                for (string_constants.items, 0..) |str_const, i| {
                    if (std.mem.eql(u8, str_const, string_content)) {
                        const_index = i;
                        break;
                    }
                }
                
                if (const_index) |index| {
                    try writer.print("  ; String literal: {s}\n", .{string_content});
                    try writer.print("  %str{} = getelementptr [{} x i8], [{} x i8]* @.str{}, i32 0, i32 0\n", 
                        .{ string_counter.*, string_content.len + 1, string_content.len + 1, index });
                    try writer.print("  %call{} = call i32 @puts(i8* %str{})\n", .{ string_counter.*, string_counter.* });
                    string_counter.* += 1;
                }
            } else {
                // Variable reference or literal value
                if (std.fmt.parseInt(i64, content, 10)) |num| {
                    try writer.print("  %fmt{} = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0\n", .{string_counter.*});
                    try writer.print("  %call{} = call i32 (i8*, ...) @printf(i8* %fmt{}, i64 {})\n", .{ string_counter.*, string_counter.*, num });
                    string_counter.* += 1;
                } else |_| {
                    // Find variable and generate load + print
                    var var_type: ?[]const u8 = null;
                    for (variables.items) |var_info| {
                        if (std.mem.eql(u8, var_info.name, content)) {
                            var_type = var_info.var_type;
                            break;
                        }
                    }
                    
                    if (var_type) |vtype| {
                        if (std.mem.eql(u8, vtype, "drip")) {
                            try writer.print("  %{s}_load = load i64, i64* %{s}, align 8\n", .{ content, content });
                            try writer.print("  %fmt{} = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0\n", .{string_counter.*});
                            try writer.print("  %call{} = call i32 (i8*, ...) @printf(i8* %fmt{}, i64 %{s}_load)\n", .{ string_counter.*, string_counter.*, content });
                            string_counter.* += 1;
                        } else if (std.mem.eql(u8, vtype, "lit")) {
                            try writer.print("  %{s}_load = load i1, i1* %{s}, align 1\n", .{ content, content });
                            try writer.print("  %{s}_select = select i1 %{s}_load, i8* getelementptr ([6 x i8], [6 x i8]* @.bool_true, i32 0, i32 0), i8* getelementptr ([7 x i8], [7 x i8]* @.bool_false, i32 0, i32 0)\n", .{ content, content });
                            try writer.print("  %call{} = call i32 @puts(i8* %{s}_select)\n", .{ string_counter.*, content });
                            string_counter.* += 1;
                        }
                        // TODO: Add support for other types
                    } else {
                        // Unknown variable, treat as string literal
                        try writer.print("  ; Unknown variable: {s}\n", .{content});
                    }
                }
            }
        }
    }
}

/// Translate variable declaration to LLVM IR
fn translateVariableDeclarationToLLVM(allocator: Allocator, line: []const u8, writer: anytype, variables: *std.ArrayList(VariableInfo), verbose: bool) !void {
    var parts = std.mem.tokenizeScalar(u8, line, ' ');
    _ = parts.next(); // skip "sus"
    
    const var_name = parts.next() orelse return;
    const var_type = parts.next() orelse return;
    const equals = parts.next() orelse return;
    
    if (!std.mem.eql(u8, equals, "=")) return;
    
    const value_str = parts.rest();
    
    // Add variable to tracking list
    const var_name_copy = try allocator.dupe(u8, var_name);
    const var_type_copy = try allocator.dupe(u8, var_type);
    try variables.append(VariableInfo{ .name = var_name_copy, .var_type = var_type_copy });
    
    if (verbose) {
        try writer.print("  ; Variable: {s} {s} = {s}\n", .{ var_name, var_type, value_str });
    }
    
    if (std.mem.eql(u8, var_type, "drip")) {
        try writer.print("  %{s} = alloca i64, align 8\n", .{var_name});
        try writer.print("  store i64 {s}, i64* %{s}, align 8\n", .{ value_str, var_name });
    } else if (std.mem.eql(u8, var_type, "lit")) {
        const llvm_bool = if (std.mem.eql(u8, std.mem.trim(u8, value_str, " \t"), "based")) "true" else "false";
        try writer.print("  %{s} = alloca i1, align 1\n", .{var_name});
        try writer.print("  store i1 {s}, i1* %{s}, align 1\n", .{ llvm_bool, var_name });
    } else if (std.mem.eql(u8, var_type, "meal")) {
        try writer.print("  %{s} = alloca double, align 8\n", .{var_name});
        try writer.print("  store double {s}, double* %{s}, align 8\n", .{ value_str, var_name });
    }
    // TODO: Add support for tea (string) type
}

/// Optimize LLVM IR using opt
fn optimizeLLVMIR(allocator: Allocator, ir_filename: []const u8, optimization_level: u8, verbose: bool) !void {
    const opt_level = try std.fmt.allocPrint(allocator, "-O{}", .{optimization_level});
    defer allocator.free(opt_level);
    
    const optimized_filename = try std.fmt.allocPrint(allocator, "{s}.opt", .{ir_filename});
    defer allocator.free(optimized_filename);
    
    var child = std.process.Child.init(&[_][]const u8{ 
        "opt", opt_level, "-o", optimized_filename, ir_filename 
    }, allocator);
    child.stdout_behavior = .Pipe;
    child.stderr_behavior = .Pipe;
    
    child.spawn() catch |err| {
        if (verbose) print("⚠️  opt not available, skipping optimization: {}\n", .{err});
        return;
    };
    
    const result = child.wait() catch |err| {
        if (verbose) print("⚠️  opt failed, skipping optimization: {}\n", .{err});
        return;
    };
    
    switch (result) {
        .Exited => |code| {
            if (code == 0) {
                // Replace original with optimized version
                std.fs.cwd().deleteFile(ir_filename) catch {};
                std.fs.cwd().rename(optimized_filename, ir_filename) catch {};
                if (verbose) print("✅ LLVM IR optimized successfully\n", .{});
            } else {
                if (verbose) print("⚠️  opt failed with exit code: {}, skipping optimization\n", .{code});
                std.fs.cwd().deleteFile(optimized_filename) catch {};
            }
        },
        else => {
            if (verbose) print("⚠️  opt terminated abnormally, skipping optimization\n", .{});
            std.fs.cwd().deleteFile(optimized_filename) catch {};
        },
    }
}

/// Generate proper LLVM IR using text-based approach (avoids LLVM C API linking issues)
fn generateProperLLVMIR(allocator: Allocator, source: []const u8, writer: anytype, verbose: bool) !void {
    // Target and basic module setup
    try writer.writeAll("; Generated LLVM IR for CURSED program\n");
    try writer.writeAll("target triple = \"x86_64-pc-linux-gnu\"\n");
    try writer.writeAll("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n\n");
    
    // External function declarations
    try writer.writeAll("declare i32 @printf(i8*, ...)\n");
    try writer.writeAll("declare i32 @puts(i8*)\n\n");
    
    // Collect string literals for global constants
    var string_literals = std.ArrayList([]const u8).init(allocator);
    defer {
        for (string_literals.items) |str| {
            allocator.free(str);
        }
        string_literals.deinit();
    }
    
    try collectStringLiteralsForLLVM(source, &string_literals, allocator);
    
    // Generate global string constants
    for (string_literals.items, 0..) |str_content, i| {
        try writer.print("@.str.{} = private unnamed_addr constant [{} x i8] c\"{s}\\00\", align 1\n", 
            .{ i, str_content.len + 1, str_content });
    }
    
    // Format strings for various types
    try writer.writeAll("@.int_fmt = private unnamed_addr constant [6 x i8] c\"%lld\\0A\\00\", align 1\n");
    try writer.writeAll("@.float_fmt = private unnamed_addr constant [4 x i8] c\"%f\\0A\\00\", align 1\n");
    try writer.writeAll("@.bool_true = private unnamed_addr constant [6 x i8] c\"based\\00\", align 1\n");
    try writer.writeAll("@.bool_false = private unnamed_addr constant [7 x i8] c\"cringe\\00\", align 1\n\n");
    
    // Main function
    try writer.writeAll("define i32 @main() {\n");
    try writer.writeAll("entry:\n");
    
    // Parse and generate code from source
    try generateLLVMStatementsFromSource(allocator, source, writer, &string_literals, verbose);
    
    // Return 0 from main
    try writer.writeAll("  ret i32 0\n");
    try writer.writeAll("}\n");
}

fn collectStringLiteralsForLLVM(source: []const u8, string_literals: *std.ArrayList([]const u8), allocator: Allocator) !void {
    var lines = std.mem.splitScalar(u8, source, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        
        // Look for vibez.spill() with string literals
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |start| {
            if (std.mem.indexOf(u8, trimmed[start..], "(")) |paren_start| {
                if (std.mem.lastIndexOf(u8, trimmed, ")")) |paren_end| {
                    const content_start = start + paren_start + 1;
                    const content = trimmed[content_start..paren_end];
                    
                    if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                        const string_content = content[1..content.len - 1];
                        const string_copy = try allocator.dupe(u8, string_content);
                        try string_literals.append(string_copy);
                    }
                }
            }
        }
    }
}

fn generateLLVMStatementsFromSource(allocator: Allocator, source: []const u8, writer: anytype, string_literals: *std.ArrayList([]const u8), verbose: bool) !void {
    var variable_counter: u32 = 0;
    var variables = std.HashMap([]const u8, LLVMVariableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    defer variables.deinit();
    
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
        if (std.mem.indexOf(u8, trimmed, "vibez.spill(")) |_| {
            try generateLLVMVibesSpill(trimmed, writer, string_literals, &variables, &variable_counter, verbose);
        }
        
        // Handle variable declarations
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            try generateLLVMVariableDeclaration(trimmed, writer, &variables, &variable_counter, verbose);
        }
    }
}

fn generateLLVMVibesSpill(line: []const u8, writer: anytype, string_literals: *std.ArrayList([]const u8), variables: *std.HashMap([]const u8, LLVMVariableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), variable_counter: *u32, verbose: bool) !void {
    if (std.mem.indexOf(u8, line, "vibez.spill(")) |start| {
        if (std.mem.indexOf(u8, line[start..], "(")) |paren_start| {
            if (std.mem.lastIndexOf(u8, line, ")")) |paren_end| {
                const content_start = start + paren_start + 1;
                const content = line[content_start..paren_end];
                
                if (content.len >= 2 and content[0] == '"' and content[content.len - 1] == '"') {
                    // String literal
                    const string_content = content[1..content.len - 1];
                    
                    // Find the string in our global constants
                    var string_index: ?usize = null;
                    for (string_literals.items, 0..) |str_const, i| {
                        if (std.mem.eql(u8, str_const, string_content)) {
                            string_index = i;
                            break;
                        }
                    }
                    
                    if (string_index) |index| {
                        if (verbose) try writer.print("  ; String literal: {s}\n", .{string_content});
                        try writer.print("  %str_ptr.{} = getelementptr [{} x i8], [{} x i8]* @.str.{}, i32 0, i32 0\n", 
                            .{ variable_counter.*, string_content.len + 1, string_content.len + 1, index });
                        try writer.print("  call i32 @puts(i8* %str_ptr.{})\n", .{variable_counter.*});
                        variable_counter.* += 1;
                    }
                } else {
                    // Variable or numeric literal
                    if (std.fmt.parseInt(i64, content, 10) catch null) |num| {
                        // Integer literal
                        if (verbose) try writer.print("  ; Integer literal: {}\n", .{num});
                        try writer.print("  %fmt_ptr.{} = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0\n", .{variable_counter.*});
                        try writer.print("  call i32 (i8*, ...) @printf(i8* %fmt_ptr.{}, i64 {})\n", .{ variable_counter.*, num });
                        variable_counter.* += 1;
                    } else if (std.fmt.parseFloat(f64, content) catch null) |num| {
                        // Float literal  
                        if (verbose) try writer.print("  ; Float literal: {}\n", .{num});
                        try writer.print("  %fmt_ptr.{} = getelementptr [4 x i8], [4 x i8]* @.float_fmt, i32 0, i32 0\n", .{variable_counter.*});
                        try writer.print("  call i32 (i8*, ...) @printf(i8* %fmt_ptr.{}, double {})\n", .{ variable_counter.*, num });
                        variable_counter.* += 1;
                    } else {
                        // Variable reference
                        if (variables.get(content)) |var_info| {
                            if (verbose) try writer.print("  ; Variable: {s}\n", .{content});
                            
                            if (std.mem.eql(u8, var_info.llvm_type, "i64")) {
                                try writer.print("  %loaded.{} = load i64, i64* %{s}, align 8\n", .{ variable_counter.*, var_info.var_name });
                                try writer.print("  %fmt_ptr.{} = getelementptr [6 x i8], [6 x i8]* @.int_fmt, i32 0, i32 0\n", .{variable_counter.*});
                                try writer.print("  call i32 (i8*, ...) @printf(i8* %fmt_ptr.{}, i64 %loaded.{})\n", .{ variable_counter.*, variable_counter.* });
                            } else if (std.mem.eql(u8, var_info.llvm_type, "i1")) {
                                try writer.print("  %loaded.{} = load i1, i1* %{s}, align 1\n", .{ variable_counter.*, var_info.var_name });
                                try writer.print("  %select.{} = select i1 %loaded.{}, i8* getelementptr ([6 x i8], [6 x i8]* @.bool_true, i32 0, i32 0), i8* getelementptr ([7 x i8], [7 x i8]* @.bool_false, i32 0, i32 0)\n", .{ variable_counter.*, variable_counter.* });
                                try writer.print("  call i32 @puts(i8* %select.{})\n", .{variable_counter.*});
                            } else if (std.mem.eql(u8, var_info.llvm_type, "double")) {
                                try writer.print("  %loaded.{} = load double, double* %{s}, align 8\n", .{ variable_counter.*, var_info.var_name });
                                try writer.print("  %fmt_ptr.{} = getelementptr [4 x i8], [4 x i8]* @.float_fmt, i32 0, i32 0\n", .{variable_counter.*});
                                try writer.print("  call i32 (i8*, ...) @printf(i8* %fmt_ptr.{}, double %loaded.{})\n", .{ variable_counter.*, variable_counter.* });
                            }
                            variable_counter.* += 1;
                        } else {
                            if (verbose) try writer.print("  ; Unknown variable: {s}\n", .{content});
                        }
                    }
                }
            }
        }
    }
}

fn generateLLVMVariableDeclaration(line: []const u8, writer: anytype, variables: *std.HashMap([]const u8, LLVMVariableInfo, std.hash_map.StringContext, std.hash_map.default_max_load_percentage), variable_counter: *u32, verbose: bool) !void {
    var parts = std.mem.tokenizeScalar(u8, line, ' ');
    _ = parts.next(); // skip "sus"
    
    const var_name = parts.next() orelse return;
    const var_type = parts.next() orelse return;
    const equals = parts.next() orelse return;
    
    if (!std.mem.eql(u8, equals, "=")) return;
    
    const value_str = parts.rest();
    
    if (verbose) try writer.print("  ; Variable: {s} {s} = {s}\n", .{ var_name, var_type, value_str });
    
    if (std.mem.eql(u8, var_type, "drip")) {
        // Integer type
        try writer.print("  %{s} = alloca i64, align 8\n", .{var_name});
        if (std.fmt.parseInt(i64, value_str, 10) catch null) |num| {
            try writer.print("  store i64 {}, i64* %{s}, align 8\n", .{ num, var_name });
        } else {
            try writer.print("  store i64 0, i64* %{s}, align 8\n", .{var_name});
        }
        try variables.put(var_name, .{ .llvm_type = "i64", .var_name = var_name });
    } else if (std.mem.eql(u8, var_type, "lit")) {
        // Boolean type
        try writer.print("  %{s} = alloca i1, align 1\n", .{var_name});
        const bool_value = if (std.mem.eql(u8, std.mem.trim(u8, value_str, " \t"), "based")) "true" else "false";
        try writer.print("  store i1 {s}, i1* %{s}, align 1\n", .{ bool_value, var_name });
        try variables.put(var_name, .{ .llvm_type = "i1", .var_name = var_name });
    } else if (std.mem.eql(u8, var_type, "meal")) {
        // Float type
        try writer.print("  %{s} = alloca double, align 8\n", .{var_name});
        if (std.fmt.parseFloat(f64, value_str) catch null) |num| {
            try writer.print("  store double {}, double* %{s}, align 8\n", .{ num, var_name });
        } else {
            try writer.print("  store double 0.0, double* %{s}, align 8\n", .{var_name});
        }
        try variables.put(var_name, .{ .llvm_type = "double", .var_name = var_name });
    }
    variable_counter.* += 1;
}

/// Convert simple LLVM IR back to C code (fallback for when clang is not available)
fn convertLLVMIRToC(allocator: Allocator, ir_filename: []const u8, c_filename: []const u8) !void {
    const ir_content = std.fs.cwd().readFileAlloc(allocator, ir_filename, 1024 * 1024) catch |err| {
        print("❌ Error reading LLVM IR file: {}\n", .{err});
        return;
    };
    defer allocator.free(ir_content);
    
    const c_file = std.fs.cwd().createFile(c_filename, .{}) catch |err| {
        print("❌ Error creating C file: {}\n", .{err});
        return;
    };
    defer c_file.close();
    
    const writer = c_file.writer();
    
    // Write C headers
    try writer.writeAll("#include <stdio.h>\n");
    try writer.writeAll("#include <stdlib.h>\n");
    try writer.writeAll("#include <string.h>\n\n");
    
    // Extract string constants and convert to C
    var lines = std.mem.splitScalar(u8, ir_content, '\n');
    var in_main = false;
    
    // First pass: extract string constants
    lines = std.mem.splitScalar(u8, ir_content, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t");
        if (std.mem.startsWith(u8, trimmed, "@.str")) {
            // Convert: @.str0 = private unnamed_addr constant [31 x i8] c"Hello from CURSED compilation!\00", align 1
            // To: const char* str0 = "Hello from CURSED compilation!";
            if (std.mem.indexOf(u8, trimmed, "c\"")) |start| {
                if (std.mem.lastIndexOf(u8, trimmed, "\\00\"")) |end| {
                    const string_content = trimmed[start + 2..end];
                    if (std.mem.indexOf(u8, trimmed, "@.str")) |str_start| {
                        if (std.mem.indexOf(u8, trimmed[str_start..], " ")) |space_pos| {
                            const str_name_raw = trimmed[str_start + 1..str_start + space_pos];
                            // Convert .str0 to str0 (remove the dot)
                            const str_name = if (str_name_raw[0] == '.') str_name_raw[1..] else str_name_raw;
                            try writer.print("const char* {s} = \"{s}\";\n", .{ str_name, string_content });
                        }
                    }
                }
            }
        }
    }
    
    try writer.writeAll("\nint main() {\n");
    
    // Second pass: convert main function
    lines = std.mem.splitScalar(u8, ir_content, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t");
        
        if (std.mem.eql(u8, trimmed, "define i32 @main() {")) {
            in_main = true;
            continue;
        }
        
        if (!in_main) continue;
        
        if (std.mem.eql(u8, trimmed, "ret i32 0")) {
            break;
        }
        
        // Convert LLVM IR instructions to C
        if (std.mem.indexOf(u8, trimmed, "call i32 @puts")) |_| {
            // Extract string variable and convert to puts call
            if (std.mem.indexOf(u8, trimmed, "%str")) |str_start| {
                const rest = trimmed[str_start..];
                if (std.mem.indexOf(u8, rest, ")")) |end| {
                    const str_var_with_percent = rest[0..end];
                    const str_var = str_var_with_percent[1..]; // Remove %
                    try writer.print("    puts({s});\n", .{str_var});
                }
            }
        } else if (std.mem.indexOf(u8, trimmed, "alloca i64")) |_| {
            // Variable declaration: %x = alloca i64, align 8
            if (std.mem.indexOf(u8, trimmed, "%")) |start| {
                if (std.mem.indexOf(u8, trimmed[start..], " ")) |end| {
                    const var_name = trimmed[start + 1..start + end];
                    try writer.print("    long {s};\n", .{var_name});
                }
            }
        } else if (std.mem.indexOf(u8, trimmed, "store i64")) |_| {
            // Store instruction: store i64 42, i64* %x, align 8
            var parts = std.mem.tokenizeScalar(u8, trimmed, ' ');
            _ = parts.next(); // skip "store"
            _ = parts.next(); // skip "i64"
            const value = parts.next() orelse continue;
            _ = parts.next(); // skip "i64*"
            const var_ref = parts.next() orelse continue;
            if (var_ref.len > 1 and var_ref[0] == '%') {
                const var_name = var_ref[1..var_ref.len - 1]; // Remove % and ,
                try writer.print("    {s} = {s};\n", .{ var_name, value[0..value.len - 1] }); // Remove comma
            }
        } else if (std.mem.indexOf(u8, trimmed, "call i32 (i8*, ...) @printf")) |_| {
            // Printf call with integer
            if (std.mem.indexOf(u8, trimmed, "%fmt")) |_| {
                if (std.mem.lastIndexOf(u8, trimmed, ")")) |end| {
                    // Extract variable name from the end
                    const before_end = trimmed[0..end];
                    if (std.mem.lastIndexOf(u8, before_end, "%")) |var_start| {
                        const var_part = before_end[var_start + 1..];
                        if (std.mem.indexOf(u8, var_part, "_load")) |load_pos| {
                            const var_name = var_part[0..load_pos];
                            try writer.print("    printf(\"%lld\\n\", {s});\n", .{var_name});
                        }
                    }
                }
            }
        } else if (std.mem.indexOf(u8, trimmed, "alloca i1")) |_| {
            // Boolean variable declaration
            if (std.mem.indexOf(u8, trimmed, "%")) |start| {
                if (std.mem.indexOf(u8, trimmed[start..], " ")) |end| {
                    const var_name = trimmed[start + 1..start + end];
                    try writer.print("    int {s};\n", .{var_name});
                }
            }
        } else if (std.mem.indexOf(u8, trimmed, "store i1")) |_| {
            // Store boolean: store i1 true, i1* %isReady, align 1
            var parts = std.mem.tokenizeScalar(u8, trimmed, ' ');
            _ = parts.next(); // skip "store"
            _ = parts.next(); // skip "i1"
            const value = parts.next() orelse continue;
            _ = parts.next(); // skip "i1*"
            const var_ref = parts.next() orelse continue;
            if (var_ref.len > 1 and var_ref[0] == '%') {
                const var_name = var_ref[1..var_ref.len - 1]; // Remove % and ,
                const c_value = if (std.mem.eql(u8, value[0..value.len - 1], "true")) "1" else "0";
                try writer.print("    {s} = {s};\n", .{ var_name, c_value });
            }
        }
        // Handle other puts calls for boolean values
        else if (std.mem.indexOf(u8, trimmed, "call i32 @puts") != null and std.mem.indexOf(u8, trimmed, "_select") != null) {
            // Boolean output - simplified to just print "based" or "cringe"
            if (std.mem.indexOf(u8, trimmed, "%")) |start| {
                const rest = trimmed[start..];
                if (std.mem.indexOf(u8, rest, "_select")) |select_pos| {
                    const var_name = rest[1..select_pos]; // Remove %
                    try writer.print("    printf(\"%s\\n\", {s} ? \"based\" : \"cringe\");\n", .{var_name});
                }
            }
        }
    }
    
    try writer.writeAll("    return 0;\n");
    try writer.writeAll("}\n");
}
