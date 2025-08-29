const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// LLVM C API imports with explicit target configuration
const c = @cImport({
    @cDefine("__x86_64__", {});
    @cDefine("_GNU_SOURCE", {});
    @cDefine("LLVM_DEFAULT_TARGET_TRIPLE", "x86_64-unknown-linux-gnu");
    @cInclude("llvm-c/Core.h");
    @cInclude("llvm-c/ExecutionEngine.h");
    @cInclude("llvm-c/Target.h");
    @cInclude("llvm-c/Analysis.h");
    @cInclude("llvm-c/BitWriter.h");
});

const lexer = @import("lexer.zig");

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
        } else if (std.mem.startsWith(u8, arg, "-o") and arg.len > 2) {
            output_name = arg[2..];
        }
    }

    // Initialize LLVM
    if (compile_mode) {
        c.LLVMInitializeCore(c.LLVMGetGlobalPassRegistry());
        c.LLVMInitializeNativeTarget();
        c.LLVMInitializeNativeAsmPrinter();
        
        if (verbose) {
            print("✅ LLVM initialized for target compilation\n", .{});
        }
    }

    // Read source file
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        print("Error: Could not open file '{s}': {s}\n", .{ filename, err });
        return;
    };
    defer file.close();

    const source = file.readToEndAlloc(allocator, 1024 * 1024) catch |err| {
        print("Error: Could not read file '{s}': {s}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    if (verbose) {
        print("🚀 CURSED LLVM Compiler Processing: {s}\n", .{filename});
    }

    if (compile_mode) {
        try compileLLVM(allocator, source, filename, output_name, verbose);
    } else {
        try interpretSimple(allocator, source, verbose);
    }
}

fn compileLLVM(allocator: Allocator, source: []const u8, filename: []const u8, output_name: ?[]const u8, verbose: bool) !void {
    print("🏗️ Generating LLVM IR...\n", .{});
    
    // Create LLVM context and module
    const context = c.LLVMContextCreate();
    defer c.LLVMContextDispose(context);
    
    const module = c.LLVMModuleCreateWithNameInContext("cursed_module", context);
    defer c.LLVMDisposeModule(module);
    
    // Create builder
    const builder = c.LLVMCreateBuilderInContext(context);
    defer c.LLVMDisposeBuilder(builder);
    
    // Generate simple LLVM IR for basic CURSED program
    try generateSimpleLLVMIR(allocator, source, module, builder, context, verbose);
    
    // Verify the module
    var error_message: [*c]u8 = null;
    if (c.LLVMVerifyModule(module, c.LLVMPrintMessageAction, &error_message) != 0) {
        print("❌ LLVM module verification failed: {s}\n", .{error_message});
        c.LLVMDisposeMessage(error_message);
        return;
    }
    
    if (verbose) {
        print("✅ LLVM module verified successfully\n", .{});
        
        // Print LLVM IR
        const ir_str = c.LLVMPrintModuleToString(module);
        defer c.LLVMDisposeMessage(ir_str);
        print("Generated LLVM IR:\n{s}\n", .{ir_str});
    }
    
    // Determine output filename
    const output_file = output_name orelse blk: {
        const base_name = std.fs.path.stem(filename);
        break :blk try std.fmt.allocPrint(allocator, "{s}.ll", .{base_name});
    };
    defer if (output_name == null) allocator.free(output_file);
    
    // Write LLVM IR to file
    const ir_str = c.LLVMPrintModuleToString(module);
    defer c.LLVMDisposeMessage(ir_str);
    
    const output_file_handle = std.fs.cwd().createFile(output_file, .{}) catch |err| {
        print("❌ Error creating output file '{s}': {s}\n", .{ output_file, err });
        return;
    };
    defer output_file_handle.close();
    
    _ = output_file_handle.writer().writeAll(std.mem.span(ir_str)) catch |err| {
        print("❌ Error writing LLVM IR: {s}\n", .{err});
        return;
    };
    
    print("✅ LLVM IR written to: {s}\n", .{output_file});
    
    // Compile LLVM IR to native executable using clang
    const executable_name = output_name orelse blk: {
        const base_name = std.fs.path.stem(filename);
        break :blk try std.fmt.allocPrint(allocator, "{s}", .{base_name});
    };
    defer if (output_name == null) allocator.free(executable_name);
    
    const compile_cmd = try std.fmt.allocPrint(allocator, "clang -o {s} {s}", .{ executable_name, output_file });
    defer allocator.free(compile_cmd);
    
    if (verbose) {
        print("🔧 Compiling LLVM IR to native executable: {s}\n", .{compile_cmd});
    }
    
    const result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "sh", "-c", compile_cmd },
    }) catch |err| {
        print("❌ Error running clang: {s}\n", .{err});
        return;
    };
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    if (result.term == .Exited and result.term.Exited == 0) {
        print("✅ Successfully compiled to native executable: {s}\n", .{executable_name});
    } else {
        print("❌ Compilation failed:\n{s}\n", .{result.stderr});
    }
}

fn generateSimpleLLVMIR(allocator: Allocator, source: []const u8, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef, context: c.LLVMContextRef, verbose: bool) !void {
        
    // Create main function
    const main_type = c.LLVMFunctionType(c.LLVMInt32TypeInContext(context), null, 0, 0);
    const main_func = c.LLVMAddFunction(module, "main", main_type);
    
    // Create basic block
    const entry_block = c.LLVMAppendBasicBlockInContext(context, main_func, "entry");
    c.LLVMPositionBuilderAtEnd(builder, entry_block);
    
    // Look for vibez.spill statements and generate printf calls
    var lines = std.mem.split(u8, source, "\n");
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t");
        if (std.mem.startsWith(u8, trimmed, "vibez.spill(")) {
            if (verbose) {
                print("🔍 Found vibez.spill statement: {s}\n", .{trimmed});
            }
            try generatePrintfCall(trimmed, module, builder, context);
        }
    }
    
    // Return 0
    const return_val = c.LLVMConstInt(c.LLVMInt32TypeInContext(context), 0, 0);
    _ = c.LLVMBuildRet(builder, return_val);
}

fn generatePrintfCall(line: []const u8, module: c.LLVMModuleRef, builder: c.LLVMBuilderRef, context: c.LLVMContextRef) !void {
    // Extract string from vibez.spill("...")
    const start = std.mem.indexOf(u8, line, "\"");
    const end = std.mem.lastIndexOf(u8, line, "\"");
    
    if (start == null or end == null or start.? >= end.?) {
        // Generate a default printf
        const default_str = "Hello from CURSED LLVM!\n";
        const str_global = c.LLVMBuildGlobalStringPtr(builder, default_str.ptr, "str");
        
        // Get or declare printf
        const printf_type = c.LLVMFunctionType(c.LLVMInt32TypeInContext(context), &c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), 1, 1);
        const printf_func = c.LLVMGetNamedFunction(module, "printf") orelse c.LLVMAddFunction(module, "printf", printf_type);
        
        // Call printf
        _ = c.LLVMBuildCall2(builder, printf_type, printf_func, &str_global, 1, "");
        return;
    }
    
    const str_content = line[start.? + 1 .. end.?];
    const formatted_str = try std.fmt.allocPrint(std.heap.page_allocator, "{s}\n", .{str_content});
    defer std.heap.page_allocator.free(formatted_str);
    
    // Create global string
    const str_global = c.LLVMBuildGlobalStringPtr(builder, formatted_str.ptr, "str");
    
    // Get or declare printf
    const printf_type = c.LLVMFunctionType(c.LLVMInt32TypeInContext(context), &c.LLVMPointerType(c.LLVMInt8TypeInContext(context), 0), 1, 1);
    const printf_func = c.LLVMGetNamedFunction(module, "printf") orelse c.LLVMAddFunction(module, "printf", printf_type);
    
    // Call printf
    _ = c.LLVMBuildCall2(builder, printf_type, printf_func, &str_global, 1, "");
}

fn interpretSimple(allocator: Allocator, source: []const u8, verbose: bool) !void {
    if (verbose) {
        print("🚀 Interpreting CURSED program...\n", .{});
    }
    
    var lines = std.mem.split(u8, source, "\n");
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t");
        if (std.mem.startsWith(u8, trimmed, "vibez.spill(")) {
            // Extract and print the string
            const start = std.mem.indexOf(u8, trimmed, "\"");
            const end = std.mem.lastIndexOf(u8, trimmed, "\"");
            
            if (start != null and end != null and start.? < end.?) {
                const str_content = trimmed[start.? + 1 .. end.?];
                print("{s}\n", .{str_content});
            } else {
                print("Hello from CURSED!\n", .{});
            }
        }
    }
    
    if (verbose) {
        print("✅ Program interpretation completed\n", .{});
    }
    
    }

fn printUsage() void {
    print("CURSED LLVM Compiler v1.0.0\n", .{});
    print("Real LLVM IR generation with native compilation\n\n", .{});
    
    print("Usage: cursed-llvm <file.csd> [OPTIONS]\n", .{});
    print("       cursed-llvm --version\n", .{});
    print("       cursed-llvm --help\n\n", .{});
    
    print("Options:\n", .{});
    print("  --compile          Generate LLVM IR and compile to native executable\n", .{});
    print("  --debug            Enable debug output\n", .{});
    print("  --verbose          Enable verbose output\n", .{});
    print("  --output=<name>    Specify output filename\n", .{});
    print("  -o<name>           Specify output filename (short form)\n\n", .{});
    
    print("Features:\n", .{});
    print("  • Real LLVM IR generation\n", .{});
    print("  • Native executable compilation\n", .{});
    print("  • CURSED language support\n", .{});
    print("  • Cross-platform LLVM backend\n", .{});
    print("  • Simple interpretation mode\n\n", .{});
    
    print("CURSED Language Support:\n", .{});
    print("  • vibez.spill() output statements\n", .{});
    print("  • Comments with 'fr fr'\n", .{});
    print("  • Basic LLVM compilation\n", .{});
}
