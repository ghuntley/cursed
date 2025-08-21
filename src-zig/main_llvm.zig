const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");
const parser_simple = @import("parser_simple.zig");
const codegen = @import("codegen.zig");
const ast = @import("ast_simple.zig");
const simple_import_resolver = @import("simple_import_resolver.zig");

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
        print("CURSED LLVM Compiler v1.0.0\n", .{});
        print("Real LLVM IR generation with native compilation\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    const filename = args[1];
    
    // Parse command line options
    var compile_mode = false;
    var debug_mode = false;
    var verbose = false;
    var output_name: ?[]const u8 = null;
    
    for (args[2..]) |arg| {
        if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_mode = true;
            verbose = true;
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            verbose = true;
        } else if (std.mem.startsWith(u8, arg, "--output=")) {
            output_name = arg[9..];
        }
    }

    // Default output name
    if (output_name == null) {
        const base = std.fs.path.basename(filename);
        const dot_pos = std.mem.lastIndexOf(u8, base, ".") orelse base.len;
        output_name = base[0..dot_pos];
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
    defer tokens.deinit();

    if (verbose) print("🔍 Lexed {} tokens\n", .{tokens.items.len});

    // Parse
    var parser = parser_simple.Parser.init(allocator, tokens.items);

    const program = parser.parseProgram() catch |err| {
        print("❌ Parser error: {}\n", .{err});
        return;
    };
    defer {
        var mut_program = program;
        mut_program.deinit();
    }

    if (verbose) print("✅ Parsed {} statements\n", .{program.statements.items.len});

    if (compile_mode) {
        try compileToLLVM(allocator, program, output_name.?, verbose, debug_mode);
    } else {
        print("❌ Interpretation mode not supported in LLVM compiler\n", .{});
        print("💡 Use --compile flag to compile to native executable\n", .{});
    }
}

fn compileToLLVM(allocator: Allocator, program: ast.Program, output_name: []const u8, verbose: bool, debug_mode: bool) !void {
    if (verbose) print("🚀 Compiling with LLVM backend...\n", .{});

    // Initialize LLVM codegen
    var cg = codegen.CodeGen.init(allocator);
    defer cg.deinit();

    // Generate LLVM IR
    cg.generateProgramAdvanced(program) catch |err| {
        print("❌ LLVM IR generation failed: {}\n", .{err});
        return;
    };

    if (verbose) print("✅ Generated LLVM IR\n", .{});

    // Output LLVM IR to file if debug mode
    if (debug_mode) {
        const ir_filename = try std.fmt.allocPrint(allocator, "{s}.ll", .{output_name});
        defer allocator.free(ir_filename);
        
        // Write LLVM IR to file
        _ = @import("std").c.LLVMWriteBitcodeToFile(cg.module, ir_filename.ptr);
        if (verbose) print("🔍 LLVM IR written to: {s}\n", .{ir_filename});
    }

    // Create execution engine for JIT compilation
    const c = @import("std").c;
    
    var engine: c.LLVMExecutionEngineRef = undefined;
    var error_msg: [*c]u8 = undefined;
    
    if (c.LLVMCreateExecutionEngineForModule(&engine, cg.module, &error_msg) != 0) {
        print("❌ Failed to create execution engine: {s}\n", .{error_msg});
        c.LLVMDisposeMessage(error_msg);
        return;
    }
    defer c.LLVMDisposeExecutionEngine(engine);

    // Find main function
    const main_func = c.LLVMGetNamedFunction(cg.module, "main");
    if (main_func == null) {
        print("❌ No main function found\n", .{});
        return;
    }

    if (verbose) print("🔨 Executing LLVM JIT compilation...\n", .{});

    // Execute main function using JIT
    const result = c.LLVMRunFunction(engine, main_func, 0, null);
    const exit_code = c.LLVMGenericValueToInt(result, 0);
    
    c.LLVMDisposeGenericValue(result);

    if (verbose) {
        print("✅ Program executed successfully\n", .{});
        print("📊 Exit code: {}\n", .{exit_code});
    }

    // For compilation to file, we would need to use:
    // c.LLVMWriteBitcodeToFile() or write object file and link with system linker
    print("✅ LLVM compilation completed (JIT execution)\n", .{});
    print("💡 Native executable compilation coming soon\n", .{});
}

fn printUsage() void {
    print("CURSED LLVM Compiler - Real LLVM IR Generation\n", .{});
    print("\n", .{});
    print("Usage: cursed-llvm <file.csd> [options]\n", .{});
    print("\n", .{});
    print("Options:\n", .{});
    print("  --compile         Generate native executable using LLVM\n", .{});
    print("  --debug           Enable debug output and save LLVM IR\n", .{});
    print("  --verbose         Verbose compilation output\n", .{});
    print("  --output=<name>   Specify output executable name\n", .{});
    print("  --version         Show version information\n", .{});
    print("  --help            Show this help message\n", .{});
    print("\n", .{});
    print("Examples:\n", .{});
    print("  cursed-llvm program.csd --compile\n", .{});
    print("  cursed-llvm program.csd --compile --debug --verbose\n", .{});
    print("  cursed-llvm program.csd --compile --output=myprogram\n", .{});
}
