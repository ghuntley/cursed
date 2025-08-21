const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const enhanced_compiler = @import("enhanced_compiler.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    if (std.mem.eql(u8, args[1], "--version")) {
        print("CURSED Compiler v1.0.0-enhanced\n", .{});
        print("Enhanced compilation with C and LLVM backends\n", .{});
        return;
    }

    if (std.mem.eql(u8, args[1], "--help")) {
        printUsage();
        return;
    }

    // Parse command line options
    var backend = enhanced_compiler.CompilationBackend.C_Backend;
    var optimization_level: u8 = 2;
    var verbose = false;
    var output_path: ?[]const u8 = null;
    var filename: ?[]const u8 = null;
    
    var i: usize = 1;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "compile")) {
            // compile subcommand
            continue;
        } else if (std.mem.eql(u8, arg, "--backend")) {
            i += 1;
            if (i >= args.len) {
                print("❌ Error: --backend requires an argument (c|llvm)\n", .{});
                return;
            }
            if (std.mem.eql(u8, args[i], "llvm")) {
                backend = .LLVM_Backend;
            } else if (std.mem.eql(u8, args[i], "c")) {
                backend = .C_Backend;
            } else {
                print("❌ Error: Invalid backend '{s}'. Use 'c' or 'llvm'\n", .{args[i]});
                return;
            }
        } else if (std.mem.eql(u8, arg, "-o")) {
            i += 1;
            if (i >= args.len) {
                print("❌ Error: -o requires an output filename\n", .{});
                return;
            }
            output_path = args[i];
        } else if (std.mem.startsWith(u8, arg, "--optimize=")) {
            const level_str = arg[11..];
            optimization_level = std.fmt.parseUnsigned(u8, level_str, 10) catch {
                print("❌ Error: Invalid optimization level '{s}'\n", .{level_str});
                return;
            };
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            verbose = true;
        } else if (!std.mem.startsWith(u8, arg, "--")) {
            // This looks like a filename
            filename = arg;
        }
    }
    
    if (filename == null) {
        print("❌ Error: No CURSED source file specified\n", .{});
        printUsage();
        return;
    }

    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, filename.?, 1024 * 1024) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename.?, err });
        return;
    };
    defer allocator.free(source);

    if (verbose) print("📁 Read {s} ({} bytes)\n", .{ filename.?, source.len });

    // Configure compiler
    const config = enhanced_compiler.CompilerConfig{
        .backend = backend,
        .optimization_level = optimization_level,
        .verbose = verbose,
        .output_path = output_path,
    };

    // Compile the program
    enhanced_compiler.compileProgram(allocator, source, filename.?, config) catch |err| {
        print("❌ Compilation failed: {any}\n", .{err});
        return;
    };
}

fn printUsage() void {
    print("CURSED Compiler - Enhanced Implementation v1.0.0\n", .{});
    print("The Gen Z Programming Language with native compilation\n", .{});
    print("\nUsage: cursed-compile <file.csd> [OPTIONS]\n", .{});
    print("       cursed-compile compile <file.csd> [OPTIONS]\n", .{});
    print("       cursed-compile --version\n", .{});
    print("       cursed-compile --help\n", .{});
    print("\nOptions:\n", .{});
    print("  --backend c|llvm     Compilation backend (default: c)\n", .{});
    print("  -o <output>          Output executable name\n", .{});
    print("  --optimize=LEVEL     Optimization level (0-3, default: 2)\n", .{});
    print("  --verbose            Enable verbose output\n", .{});
    print("\nBackends:\n", .{});
    print("  c        Transpile to C, then compile with GCC\n", .{});
    print("  llvm     Generate LLVM IR, then compile with clang\n", .{});
    print("\nExamples:\n", .{});
    print("  cursed-compile hello.csd\n", .{});
    print("  cursed-compile hello.csd --backend llvm -o hello_native\n", .{});
    print("  cursed-compile hello.csd --optimize=3 --verbose\n", .{});
    print("\nSupported CURSED Features:\n", .{});
    print("  • Variable declarations: sus varname type = value\n", .{});
    print("  • Types: drip (int), meal (float), tea (string), lit (bool)\n", .{});
    print("  • Output: vibez.spill() statements\n", .{});
    print("  • Comments: fr fr prefix\n", .{});
    print("  • Imports: yeet statements (skipped during compilation)\n", .{});
    print("  • Gen Z slang: based (true), cringe (false)\n", .{});
}
