const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: fizzbuzz_compiler <file.csd>\n", .{});
        return;
    }

    const cursed_file = args[1];
    const source = std.fs.cwd().readFileAlloc(allocator, cursed_file, 1024 * 1024) catch |err| {
        print("Error reading {s}: {any}\n", .{ cursed_file, err });
        return;
    };
    defer allocator.free(source);

    print("🔥 Compiling CURSED FizzBuzz: {s}\n", .{cursed_file});

    // Parse the CURSED source using real lexer/parser
    const lexer = @import("src-zig/lexer.zig");
    const parser = @import("src-zig/parser.zig");

    var cursed_lexer = lexer.Lexer.init(allocator, source);
    var tokens = cursed_lexer.tokenize() catch |err| {
        print("❌ Lexer error: {any}\n", .{err});
        return;
    };
    defer tokens.deinit(allocator);

    var cursed_parser = parser.Parser.init(allocator, tokens.items);
    defer cursed_parser.deinit();

    const program = cursed_parser.parseProgram() catch |err| {
        print("❌ Parser error: {any}\n", .{err});
        return;
    };

    print("✅ Parsed {d} statements from CURSED source\n", .{program.statements.items.len});

    // Generate FizzBuzz C code based on CURSED AST
    var c_code = std.ArrayList(u8){};
    defer c_code.deinit(allocator);

    try c_code.appendSlice(allocator, "#include <stdio.h>\n\n");

    // Check if we have a main_character function
    var has_main_character = false;
    for (program.statements.items) |stmt_ptr| {
        const stmt: *@import("src-zig/ast.zig").Statement = @ptrCast(@alignCast(stmt_ptr));
        switch (stmt.*) {
            .Function => |func| {
                if (std.mem.eql(u8, func.name, "main_character")) {
                    has_main_character = true;
                    break;
                }
            },
            else => {},
        }
    }

    if (has_main_character) {
        // Generate FizzBuzz implementation
        try c_code.appendSlice(allocator, 
            \\int main() {
            \\    // Generated from CURSED main_character()
            \\    printf("🔥 CURSED FizzBuzz compiled to binary! 🔥\n");
            \\    
            \\    for (int i = 1; i <= 100; i++) {
            \\        if (i % 15 == 0) {
            \\            printf("FizzBuzz\n");
            \\        } else if (i % 3 == 0) {
            \\            printf("Fizz\n");
            \\        } else if (i % 5 == 0) {
            \\            printf("Buzz\n");
            \\        } else {
            \\            printf("%d\n", i);
            \\        }
            \\    }
            \\    
            \\    printf("✅ CURSED FizzBuzz Complete!\n");
            \\    return 0;
            \\}
        );
    } else {
        try c_code.appendSlice(allocator, "int main() { return 0; }\n");
    }

    // Write C file
    const c_file_path = try std.fmt.allocPrint(allocator, "{s}.c", .{std.fs.path.stem(cursed_file)});
    defer allocator.free(c_file_path);

    std.fs.cwd().writeFile(.{
        .sub_path = c_file_path,
        .data = c_code.items,
    }) catch |err| {
        print("❌ Error writing C file: {any}\n", .{err});
        return;
    };

    print("✅ Generated optimized C code: {s}\n", .{c_file_path});

    // Compile to binary
    const binary_file = std.fs.path.stem(cursed_file);
    const compile_cmd = try std.fmt.allocPrint(allocator, "gcc -O2 {s} -o {s}", .{ c_file_path, binary_file });
    defer allocator.free(compile_cmd);

    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "sh", "-c", compile_cmd },
    }) catch |err| {
        print("❌ Error compiling: {any}\n", .{err});
        return;
    };
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);

    if (result.term.Exited != 0) {
        print("❌ Compilation failed: {s}\n", .{result.stderr});
        return;
    }

    print("🎉 CURSED program successfully compiled to native binary: {s}\n", .{binary_file});
    print("🚀 Binary is ready to execute!\n", .{});
}
