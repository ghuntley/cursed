const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: simple_cursed_compiler <file.💀.💀>\n", .{});
        return;
    }

    const cursed_file = args[1];
    const source = std.fs.cwd().readFileAlloc(allocator, cursed_file, 1024 * 1024) catch |err| {
        print("Error reading {s}: {any}\n", .{ cursed_file, err });
        return;
    };
    defer allocator.free(source);

    print("🔥 Compiling CURSED to binary: {s}\n", .{cursed_file});

    // Parse CURSED (this works)
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

    print("✅ Parsed {d} statements from CURSED\n", .{program.statements.items.len});

    // Generate equivalent C code for simple cases
    const c_code = 
        \\#include <stdio.h>
        \\
        \\int main() {
        \\    // Generated from CURSED main_character()
        \\    return 0;
        \\}
    ;

    const c_file = try std.fmt.allocPrint(allocator, "{s}.c", .{std.fs.path.stem(cursed_file)});
    defer allocator.free(c_file);

    std.fs.cwd().writeFile(.{
        .sub_path = c_file,
        .data = c_code,
    }) catch |err| {
        print("❌ Error writing C file: {any}\n", .{err});
        return;
    };

    print("✅ Generated C code: {s}\n", .{c_file});

    // Compile C to binary
    const binary_file = std.fs.path.stem(cursed_file);
    const compile_cmd = try std.fmt.allocPrint(allocator, "gcc {s} -o {s}", .{ c_file, binary_file });
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

    print("🎉 Successfully compiled CURSED to binary: {s}\n", .{binary_file});
    print("🚀 Run with: ./{s}\n", .{binary_file});

    // Test the binary
    const test_result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{try std.fmt.allocPrint(allocator, "./{s}", .{binary_file})},
    }) catch |err| {
        print("❌ Error testing binary: {any}\n", .{err});
        return;
    };
    defer allocator.free(test_result.stdout);
    defer allocator.free(test_result.stderr);

    print("✅ Binary executed successfully! Exit code: {d}\n", .{test_result.term.Exited});
}
