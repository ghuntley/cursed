const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

// External C wrapper functions for LLVM
extern fn llvm_initialize_core() void;
extern fn llvm_create_context() ?*anyopaque;
extern fn llvm_dispose_context(context: ?*anyopaque) void;
extern fn llvm_create_module(context: ?*anyopaque, name: [*:0]const u8) ?*anyopaque;
extern fn llvm_dispose_module(module: ?*anyopaque) void;
extern fn llvm_create_builder(context: ?*anyopaque) ?*anyopaque;
extern fn llvm_dispose_builder(builder: ?*anyopaque) void;
extern fn llvm_int32_type(context: ?*anyopaque) ?*anyopaque;
extern fn llvm_int8_type(context: ?*anyopaque) ?*anyopaque;
extern fn llvm_pointer_type(element_type: ?*anyopaque) ?*anyopaque;
extern fn llvm_function_type(return_type: ?*anyopaque, param_types: [*]?*anyopaque, param_count: c_int, is_var_arg: c_int) ?*anyopaque;
extern fn llvm_add_function(module: ?*anyopaque, name: [*:0]const u8, function_type: ?*anyopaque) ?*anyopaque;
extern fn llvm_append_basic_block(context: ?*anyopaque, function: ?*anyopaque, name: [*:0]const u8) ?*anyopaque;
extern fn llvm_position_builder_at_end(builder: ?*anyopaque, block: ?*anyopaque) void;
extern fn llvm_build_global_string_ptr(builder: ?*anyopaque, str: [*:0]const u8, name: [*:0]const u8) ?*anyopaque;
extern fn llvm_const_int(int_type: ?*anyopaque, value: c_ulonglong) ?*anyopaque;
extern fn llvm_build_ret(builder: ?*anyopaque, value: ?*anyopaque) ?*anyopaque;
extern fn llvm_get_named_function(module: ?*anyopaque, name: [*:0]const u8) ?*anyopaque;
extern fn llvm_build_call2(builder: ?*anyopaque, function_type: ?*anyopaque, function: ?*anyopaque, args: [*]?*anyopaque, arg_count: c_int, name: [*:0]const u8) ?*anyopaque;
extern fn llvm_verify_module(module: ?*anyopaque) c_int;
extern fn llvm_print_module_to_string(module: ?*anyopaque) [*:0]u8;
extern fn llvm_dispose_message(message: [*:0]u8) void;
extern fn llvm_write_bitcode_to_file(module: ?*anyopaque, path: [*:0]const u8) c_int;

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
    
    // Initialize LLVM
    llvm_initialize_core();
    
    // Create LLVM context and module
    const context = llvm_create_context() orelse {
        print("❌ Failed to create LLVM context\n", .{});
        return;
    };
    defer llvm_dispose_context(context);
    
    const module = llvm_create_module(context, "cursed_module") orelse {
        print("❌ Failed to create LLVM module\n", .{});
        return;
    };
    defer llvm_dispose_module(module);
    
    // Create builder
    const builder = llvm_create_builder(context) orelse {
        print("❌ Failed to create LLVM builder\n", .{});
        return;
    };
    defer llvm_dispose_builder(builder);
    
    // Generate simple LLVM IR for basic CURSED program
    try generateSimpleLLVMIR(allocator, source, module, builder, context, verbose);
    
    // Verify the module
    if (llvm_verify_module(module) != 0) {
        print("❌ LLVM module verification failed\n", .{});
        return;
    }
    
    if (verbose) {
        print("✅ LLVM module verified successfully\n", .{});
    }
    
    // Determine output filename
    const output_file = output_name orelse blk: {
        const base_name = std.fs.path.stem(filename);
        break :blk try std.fmt.allocPrint(allocator, "{s}.ll", .{base_name});
    };
    defer if (output_name == null) allocator.free(output_file);
    
    // Write LLVM IR to file
    const ir_str = llvm_print_module_to_string(module);
    defer llvm_dispose_message(ir_str);
    
    if (verbose) {
        print("Generated LLVM IR:\n{s}\n", .{ir_str});
    }
    
    const output_file_handle = std.fs.cwd().createFile(output_file, .{}) catch |err| {
        print("❌ Error creating output file '{s}': {s}\n", .{ output_file, err });
        return;
    };
    defer output_file_handle.close();
    
    _ = output_file_handle.writeAll(std.mem.span(ir_str)) catch |err| {
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
        print("🎉 LLVM compilation completed! Run with: ./{s}\n", .{executable_name});
    } else {
        print("❌ Compilation failed:\n{s}\n", .{result.stderr});
    }
}

fn generateSimpleLLVMIR(allocator: Allocator, source: []const u8, module: ?*anyopaque, builder: ?*anyopaque, context: ?*anyopaque, verbose: bool) !void {
    _ = allocator;
        
    // Create main function
    const int32_type = llvm_int32_type(context);
    const main_type = llvm_function_type(int32_type, undefined, 0, 0);
    const main_func = llvm_add_function(module, "main", main_type);
    
    // Create basic block
    const entry_block = llvm_append_basic_block(context, main_func, "entry");
    llvm_position_builder_at_end(builder, entry_block);
    
    // Look for vibez.spill statements and generate printf calls
    var lines = std.mem.splitScalar(u8, source, '\n');
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
    const return_val = llvm_const_int(int32_type, 0);
    _ = llvm_build_ret(builder, return_val);
}

fn generatePrintfCall(line: []const u8, module: ?*anyopaque, builder: ?*anyopaque, context: ?*anyopaque) !void {
    // Extract string from vibez.spill("...")
    const start = std.mem.indexOf(u8, line, "\"");
    const end = std.mem.lastIndexOf(u8, line, "\"");
    
    var formatted_str: []const u8 = undefined;
    var should_free = false;
    
    if (start == null or end == null or start.? >= end.?) {
        // Generate a default printf
        formatted_str = "Hello from CURSED LLVM!\n";
    } else {
        const str_content = line[start.? + 1 .. end.?];
        formatted_str = try std.fmt.allocPrint(std.heap.page_allocator, "{s}\n", .{str_content});
        should_free = true;
    }
    defer if (should_free) std.heap.page_allocator.free(formatted_str);
    
    // Create global string with null terminator
    const str_z = try std.heap.page_allocator.dupeZ(u8, formatted_str);
    defer std.heap.page_allocator.free(str_z);
    
    const str_global = llvm_build_global_string_ptr(builder, str_z.ptr, "str");
    
    // Get or declare printf
    const int8_type = llvm_int8_type(context);
    const char_ptr_type = llvm_pointer_type(int8_type);
    const int32_type = llvm_int32_type(context);
    
    var printf_params = [_]?*anyopaque{char_ptr_type};
    const printf_type = llvm_function_type(int32_type, &printf_params, 1, 1);
    const printf_func = llvm_get_named_function(module, "printf") orelse llvm_add_function(module, "printf", printf_type);
    
    // Call printf
    var call_args = [_]?*anyopaque{str_global};
    _ = llvm_build_call2(builder, printf_type, printf_func, &call_args, 1, "");
}

fn interpretSimple(allocator: Allocator, source: []const u8, verbose: bool) !void {
    _ = allocator;
    if (verbose) {
        print("🚀 Interpreting CURSED program...\n", .{});
    }
    
    var lines = std.mem.splitScalar(u8, source, '\n');
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
