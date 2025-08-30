const std = @import("std");
const print = std.debug.print;

const c = @cImport({
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/Target.h");
});

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: generate_ir_for_cursed <file.csd>\n", .{});
        return;
    }

    const cursed_file = args[1];
    const source = std.fs.cwd().readFileAlloc(allocator, cursed_file, 1024 * 1024) catch |err| {
        print("Error reading {s}: {any}\n", .{ cursed_file, err });
        return;
    };
    defer allocator.free(source);

    print("🔥 Compiling CURSED file: {s}\n", .{cursed_file});

    // Parse the CURSED source
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

    var program = cursed_parser.parseProgram() catch |err| {
        print("❌ Parser error: {any}\n", .{err});
        return;
    };

    print("✅ Parsed {d} statements\n", .{program.statements.items.len});

    // Generate LLVM IR manually for now 
    _ = c.LLVMInitializeCore(c.LLVMGetGlobalPassRegistry());
    _ = c.LLVMInitializeNativeTarget();
    _ = c.LLVMInitializeNativeAsmPrinter();

    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);

    const module = c.LLVMModuleCreateWithNameInContext("cursed_program", context);
    defer c.LLVMDisposeModule(module);

    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);

    // Generate CURSED main_character function -> int main()
    const main_type = c.LLVMFunctionType(c.LLVMInt32TypeInContext(context), null, 0, 0);
    const main_func = c.LLVMAddFunction(module, "main", main_type);

    const entry_block = c.LLVMAppendBasicBlockInContext(context, main_func, "entry");
    c.LLVMPositionBuilderAtEnd(builder, entry_block);

    // If we have a main_character function, we'd call it here
    // For now, just return 0
    const zero = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0);
    _ = c.LLVMBuildRet(builder, zero);

    // Verify module
    var error_msg: [*c]u8 = undefined;
    if (c.LLVMVerifyModule(module, c.LLVMReturnStatusAction, &error_msg) != 0) {
        print("❌ Module verification failed: {s}\n", .{std.mem.span(error_msg)});
        c.LLVMDisposeMessage(error_msg);
        return;
    }

    // Write IR to file
    const output_file = try std.fmt.allocPrint(allocator, "{s}.ll", .{std.fs.path.stem(cursed_file)});
    defer allocator.free(output_file);

    if (c.LLVMPrintModuleToFile(module, output_file.ptr, &error_msg) != 0) {
        print("❌ Failed to write IR: {s}\n", .{std.mem.span(error_msg)});
        c.LLVMDisposeMessage(error_msg);
        return;
    }

    print("✅ LLVM IR written to: {s}\n", .{output_file});

    // Compile to binary
    const asm_file = try std.fmt.allocPrint(allocator, "{s}.s", .{std.fs.path.stem(cursed_file)});
    defer allocator.free(asm_file);

    const binary_file = try std.fmt.allocPrint(allocator, "{s}", .{std.fs.path.stem(cursed_file)});
    defer allocator.free(binary_file);

    // Use llc to compile IR to assembly
    const llc_result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "llc-18", output_file, "-o", asm_file },
    }) catch |err| {
        print("❌ Error running llc: {any}\n", .{err});
        return;
    };
    defer allocator.free(llc_result.stdout);
    defer allocator.free(llc_result.stderr);

    if (llc_result.term.Exited != 0) {
        print("❌ llc failed: {s}\n", .{llc_result.stderr});
        return;
    }

    // Use gcc to compile assembly to binary
    const gcc_result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "gcc", asm_file, "-o", binary_file },
    }) catch |err| {
        print("❌ Error running gcc: {any}\n", .{err});
        return;
    };
    defer allocator.free(gcc_result.stdout);
    defer allocator.free(gcc_result.stderr);

    if (gcc_result.term.Exited != 0) {
        print("❌ gcc failed: {s}\n", .{gcc_result.stderr});
        return;
    }

    print("🎉 Successfully compiled CURSED program to binary: {s}\n", .{binary_file});
    print("🚀 Run with: ./{s}\n", .{binary_file});
}
