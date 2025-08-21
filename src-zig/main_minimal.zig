const std = @import("std");
const ast = @import("ast.zig");
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const llvm_real = @import("llvm_real.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        std.debug.print("Usage: {s} [options] <file.csd>\n", .{args[0]});
        std.debug.print("Options:\n", .{});
        std.debug.print("  --compile    Compile to native binary using LLVM\n", .{});
        std.debug.print("  --interpret  Run in interpreter mode (default)\n", .{});
        return;
    }

    var compile_mode = false;
    var filename: ?[]const u8 = null;

    // Parse arguments
    for (args[1..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--interpret")) {
            compile_mode = false;
        } else if (std.mem.endsWith(u8, arg, ".csd")) {
            filename = arg;
        }
    }

    const file = filename orelse {
        std.debug.print("Error: No .csd file provided\n", .{});
        return;
    };

    // Read the file
    const source = std.fs.cwd().readFileAlloc(allocator, file, std.math.maxInt(usize)) catch |err| {
        std.debug.print("Error reading file {s}: {}\n", .{ file, err });
        return;
    };
    defer allocator.free(source);

    // Tokenize
    var lex = lexer.Lexer.init(allocator, source);
    var tokens = lex.tokenize() catch |err| {
        std.debug.print("Tokenization error: {}\n", .{err});
        return;
    };
    defer tokens.deinit(allocator);

    // Parse
    var p = parser.Parser.init(allocator, tokens.items);
    var program = p.parseProgram() catch |err| {
        std.debug.print("Parse error: {}\n", .{err});
        return;
    };
    defer program.deinit(allocator);

    if (compile_mode) {
        // Compile using LLVM
        std.debug.print("Compiling {s} with LLVM backend...\n", .{file});
        
        var codegen = llvm_real.RealLLVMCodeGen.init(allocator) catch |err| {
            std.debug.print("Failed to initialize LLVM: {}\n", .{err});
            std.debug.print("Note: This is expected - implementing LLVM function compilation now...\n", .{});
            return;
        };
        defer codegen.deinit();
        
        codegen.generateProgram(program) catch |err| {
            std.debug.print("Code generation error: {}\n", .{err});
            std.debug.print("This indicates issues with function compilation - let's fix them!\n", .{});
            return;
        };
        
        // Print the generated IR
        codegen.printModule();
        
        // Write to file
        const output_file = try std.fmt.allocPrint(allocator, "{s}.bc", .{file[0..file.len-4]});
        defer allocator.free(output_file);
        
        codegen.writeToFile(output_file) catch |err| {
            std.debug.print("Failed to write output file: {}\n", .{err});
            return;
        };
        
        std.debug.print("Compilation completed. Output: {s}\n", .{output_file});
    } else {
        std.debug.print("Interpreter mode not available in this minimal build.\n", .{});
        std.debug.print("Use --compile to test LLVM function compilation.\n", .{});
    }
}
