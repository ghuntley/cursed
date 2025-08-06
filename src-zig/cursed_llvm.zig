const std = @import("std");
const print = std.debug.print;

const enhanced_compiler = @import("enhanced_compiler.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("Usage: cursed-llvm [options] <file.csd>\n", .{});
        print("Options:\n", .{});
        print("  --backend c|llvm     Compilation backend (default: llvm)\n", .{});
        print("  -O<level>           Optimization level (0-3, default: 2)\n", .{});
        print("  --verbose           Verbose output\n", .{});
        print("  -o <file>           Output file\n", .{});
        return;
    }
    
    var config = enhanced_compiler.CompilerConfig{
        .backend = .LLVM_Backend,
        .optimization_level = 2,
        .verbose = false,
        .output_path = null,
    };
    
    var filename: ?[]const u8 = null;
    var i: usize = 1;
    
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--backend")) {
            i += 1;
            if (i >= args.len) {
                print("❌ Error: --backend requires an argument (c|llvm)\n", .{});
                return;
            }
            if (std.mem.eql(u8, args[i], "c")) {
                config.backend = .C_Backend;
            } else if (std.mem.eql(u8, args[i], "llvm")) {
                config.backend = .LLVM_Backend;
            } else {
                print("❌ Error: Invalid backend '{s}'. Use 'c' or 'llvm'\n", .{args[i]});
                return;
            }
        } else if (std.mem.startsWith(u8, arg, "-O")) {
            if (arg.len > 2) {
                config.optimization_level = std.fmt.parseInt(u8, arg[2..], 10) catch 2;
            }
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            config.verbose = true;
        } else if (std.mem.eql(u8, arg, "-o")) {
            i += 1;
            if (i >= args.len) {
                print("❌ Error: -o requires an argument\n", .{});
                return;
            }
            config.output_path = args[i];
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            filename = arg;
        }
    }
    
    if (filename == null) {
        print("❌ Error: No input file specified\n", .{});
        return;
    }
    
    // Read the source file
    const source = std.fs.cwd().readFileAlloc(allocator, filename.?, 1024 * 1024) catch |err| {
        print("❌ Error reading file: {}\n", .{err});
        return;
    };
    defer allocator.free(source);
    
    // Compile the program
    enhanced_compiler.compileProgram(allocator, source, filename.?, config) catch |err| {
        print("❌ Compilation failed: {}\n", .{err});
        return;
    };
}
