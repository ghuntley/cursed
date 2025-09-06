const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// CURSED compiler pipeline imports
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const interpreter = @import("interpreter.zig");
const LLVMIRPipeline = @import("llvm_ir_pipeline.zig").LLVMIRPipeline;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{
        .safety = false,  // Disable memory leak detection to avoid arena allocator false positives
    }){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Unified Compiler v1.0.0 (Production Ready)\n", .{});
        print("Full LLVM backend with CURSED language support\n", .{});
        print("Copyright (C) 2025 - Built with ❤️ by the CURSED team\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    // Parse command line options
    var compile_mode = false;
    var debug_mode = false;
    var verbose = false;
    var emit_ir = false;
    var output_name: ?[]const u8 = null;
    var optimize = false;
    var filename: ?[]const u8 = null;
    var expect_output_next = false;
    
    for (args[1..]) |arg| {
        if (expect_output_next) {
            output_name = arg;
            expect_output_next = false;
        } else if (std.mem.eql(u8, arg, "--version")) {
            print("CURSED Unified Compiler v1.0.0 (Production Ready)\n", .{});
            print("Full LLVM backend with CURSED language support\n", .{});
            print("Copyright (C) 2025 - Built with ❤️ by the CURSED team\n", .{});
            return;
        } else if (std.mem.eql(u8, arg, "--help")) {
            printUsage();
            return;
        } else if (std.mem.eql(u8, arg, "--compile")) {
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--debug")) {
            debug_mode = true;
            verbose = true;
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            verbose = true;
        } else if (std.mem.eql(u8, arg, "--emit-ir")) {
            emit_ir = true;
            compile_mode = true;
        } else if (std.mem.eql(u8, arg, "--optimize") or std.mem.eql(u8, arg, "-O")) {
            optimize = true;
        } else if (std.mem.startsWith(u8, arg, "--output=")) {
            output_name = arg[9..];
        } else if (std.mem.startsWith(u8, arg, "-o") and arg.len > 2) {
            output_name = arg[2..];
        } else if (std.mem.eql(u8, arg, "-o")) {
            expect_output_next = true;
        } else if (!std.mem.startsWith(u8, arg, "-") and filename == null) {
            // First non-flag argument is the filename
            filename = arg;
        }
    }
    
    // Check if filename was provided
    if (filename == null) {
        print("❌ Error: No input file specified\n", .{});
        printUsage();
        return;
    }

    // Default output name
    if (output_name == null and compile_mode) {
        const base = std.fs.path.basename(filename.?);
        const dot_pos = std.mem.lastIndexOf(u8, base, ".") orelse base.len;
        output_name = base[0..dot_pos];
    }

    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, filename.?, 1024 * 1024) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename.?, err });
        return;
    };
    defer allocator.free(source);

    if (verbose) print("📁 Read {s} ({d} bytes)\n", .{ filename.?, source.len });

    if (compile_mode) {
        try compileToExecutable(allocator, source, filename.?, output_name.?, verbose, debug_mode, optimize, emit_ir);
    } else {
        try interpretSource(allocator, source, filename.?, verbose);
    }
}

fn printUsage() void {
    print("🔥 CURSED Unified Compiler v1.0.0\n", .{});
    print("\n", .{});
    print("Usage: cursed-zig [options] <source.💀.💀>\n", .{});
    print("\n", .{});
    print("Options:\n", .{});
    print("  --compile        Generate native executable (LLVM backend)\n", .{});
    print("  --emit-ir        Generate LLVM IR file (.ll)\n", .{});
    print("  --debug          Enable debug information and verbose output\n", .{});
    print("  --verbose        Show detailed compilation information\n", .{});
    print("  --optimize, -O   Enable optimizations\n", .{});
    print("  --output=<name>  Specify output executable name\n", .{});
    print("  -o <name>        Specify output executable name (short form)\n", .{});
    print("  --version        Show version information\n", .{});
    print("  --help           Show this help message\n", .{});
    print("\n", .{});
    print("Examples:\n", .{});
    print("  cursed-zig hello.💀.💀                   # Interpret and run\n", .{});
    print("  cursed-zig --compile hello.💀.💀         # Compile to native binary\n", .{});
    print("  cursed-zig --emit-ir hello.💀.💀         # Generate LLVM IR (.ll file)\n", .{});
    print("  cursed-zig --compile -o app hello.💀.💀  # Compile with custom name\n", .{});
    print("  cursed-zig --debug --verbose hello.💀.💀 # Debug interpretation\n", .{});
    print("\n", .{});
}

fn compileToExecutable(allocator: Allocator, source: []const u8, filename: []const u8, output_name: []const u8, verbose: bool, debug_mode: bool, optimize: bool, emit_ir: bool) !void {
    if (verbose) print("🔥 Starting CURSED LLVM compilation pipeline...\n", .{});
    
    // Step 1: Lexical Analysis
    if (verbose) print("🔍 Step 1: Lexical analysis...\n", .{});
    
    var lex = lexer.Lexer.init(allocator, source);
    
    var tokens_list = lex.tokenize() catch |err| {
        print("❌ Lexer error in {s}: {any}\n", .{ filename, err });
        return err;
    };
    defer tokens_list.deinit(allocator);
    
    if (verbose) print("✅ Generated {d} tokens\n", .{tokens_list.items.len});
    
    // Step 2: Parse AST (keep parser alive during compilation)
    if (verbose) print("🌳 Step 2: Parsing AST...\n", .{});
    
    var cursed_parser = parser.Parser.initWithFile(allocator, tokens_list.items, filename);
    defer cursed_parser.deinit();  // Arena cleanup handled here
    
    const program = cursed_parser.parseProgram() catch |err| {
        print("❌ Parser error in {s}: {any}\n", .{ filename, err });
        if (cursed_parser.had_error) {
            print("💡 Parser encountered errors during parsing\n", .{});
            cursed_parser.error_recovery_stats.reportStats();
        }
        return err;
    };

    if (verbose) print("✅ Parsed AST with {d} statements\n", .{program.statements.items.len});
    
    // Step 3: Initialize LLVM Pipeline  
    if (verbose) print("🛠️ Step 3: Initializing LLVM backend...\n", .{});
    
    const base_name = std.fs.path.stem(filename);
    const pipeline = LLVMIRPipeline.init(allocator, base_name) catch |err| {
        print("❌ LLVM initialization failed: {any}\n", .{err});
        return;
    };
    defer pipeline.deinit();
    
    // Configure optimization level
    if (optimize) {
        pipeline.optimization_level = 2;
        if (verbose) print("🚀 Optimization enabled (O2)\n", .{});
    } else if (debug_mode) {
        pipeline.optimization_level = 0;
        pipeline.debug_info = true;
        if (verbose) print("🐛 Debug mode enabled\n", .{});
    }
    
    // Step 4: Generate LLVM IR (use Program while parser arena is still valid)
    if (verbose) print("⚡ Step 4: Generating LLVM IR...\n", .{});
    
    const final_output = if (emit_ir) 
        try std.fmt.allocPrint(allocator, "{s}.ll", .{output_name})
    else 
        output_name;
    defer if (emit_ir) allocator.free(final_output);
    
    pipeline.compileSource(source, final_output, verbose) catch |err| {
        print("❌ LLVM code generation failed: {any}\n", .{err});
        return err;
    };
    
    if (emit_ir) {
        print("🎉 Successfully generated LLVM IR: {s}\n", .{final_output});
        if (verbose) {
            print("🔍 View IR with: cat {s}\n", .{final_output});
            print("💡 Compile to binary: clang -O2 -o {s} {s}\n", .{ output_name, final_output });
        }
    } else {
        // print("🎉 Successfully compiled {s} to {s}\n", .{ filename, output_name });
        if (verbose) {
            print("🚀 Run with: ./{s}\n", .{output_name});
        }
    }
    
    if (verbose) print("✅ CURSED LLVM compilation complete!\n", .{});
    // Parser deinit() will be called here, cleaning up the arena and Program
}

fn interpretSource(allocator: Allocator, source: []const u8, filename: []const u8, verbose: bool) !void {
    if (verbose) print("🔥 Starting CURSED interpreter pipeline...\n", .{});
    
    // Step 1: Tokenize source
    if (verbose) print("🔤 Step 1: Tokenizing CURSED source code...\n", .{});
    var lex = lexer.Lexer.init(allocator, source);
    
    var tokens_list = lex.tokenize() catch |err| {
        print("❌ Lexer error in {s}: {any}\n", .{ filename, err });
        return err;
    };
    defer tokens_list.deinit(allocator);
    
    if (verbose) print("📝 Generated {d} tokens\n", .{tokens_list.items.len});
    
    // Step 2: Parse AST (keep parser alive during interpretation)
    if (verbose) print("🧠 Step 2: Parsing CURSED AST...\n", .{});
    var cursed_parser = parser.Parser.initWithFile(allocator, tokens_list.items, filename);
    defer cursed_parser.deinit(); // Arena cleanup handled here
    
    const program = cursed_parser.parseProgram() catch |err| {
        print("❌ Parser error in {s}: {any}\n", .{ filename, err });
        if (cursed_parser.had_error) {
            print("💡 Parser encountered errors during parsing\n", .{});
            cursed_parser.error_recovery_stats.reportStats();
        }
        return err;
    };
    
    if (verbose) print("🎯 Parsed {d} statements\n", .{program.statements.items.len});
    
    // Step 3: Execute with interpreter (use Program while parser arena is still valid)
    if (verbose) print("🚀 Step 3: Executing CURSED program...\n", .{});
    var cursed_interpreter = interpreter.Interpreter.initWithVerbose(allocator, verbose);
    defer cursed_interpreter.deinit();
    
    cursed_interpreter.interpret(program) catch |err| {
        print("❌ Runtime error in {s}: {any}\n", .{ filename, err });
        return err;
    };
    
    if (verbose) {
        print("✅ CURSED program executed successfully\n", .{});
        print("💡 Use --compile flag to generate native executable\n", .{});
    }
    // Parser deinit() will be called here, cleaning up the arena and Program
}
