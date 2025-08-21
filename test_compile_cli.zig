const std = @import("std");
const print = std.debug.print;

const Command = enum {
    interpret,
    compile,
    help,
};

const Config = struct {
    command: Command = .interpret,
    source_file: ?[]const u8 = null,
    verbose: bool = false,
};

fn parseArgs(args: [][:0]u8) Config {
    var config = Config{};
    
    if (args.len < 2) {
        config.command = .help;
        return config;
    }
    
    var i: usize = 1;
    
    // Check for global flags first
    if (std.mem.eql(u8, args[i], "--help") or std.mem.eql(u8, args[i], "-h")) {
        config.command = .help;
        return config;
    }
    
    // Parse subcommand or flags
    while (i < args.len) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "compile")) {
            config.command = .compile;
        } else if (std.mem.eql(u8, arg, "interpret")) {
            config.command = .interpret;
        } else if (std.mem.eql(u8, arg, "--compile")) {
            config.command = .compile;
        } else if (std.mem.eql(u8, arg, "--verbose")) {
            config.verbose = true;
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            // Assume it's a source file if no source file is set yet
            if (config.source_file == null) {
                config.source_file = arg;
            }
        }
        
        i += 1;
    }
    
    return config;
}

fn printHelp() void {
    print("CURSED Compiler CLI Test v1.0.0\n", .{});
    print("A modern programming language for the next generation\n\n", .{});
    
    print("USAGE:\n", .{});
    print("    cursed [COMMAND] [OPTIONS] [FILE]\n\n", .{});
    
    print("COMMANDS:\n", .{});
    print("    interpret       Interpret CURSED source code (default)\n", .{});
    print("    compile         Compile CURSED source to native executable\n", .{});
    print("\n", .{});
    print("    --help, -h      Show this help message\n\n", .{});
    
    print("OPTIONS:\n", .{});
    print("    --compile       Compile source to native executable (same as compile command)\n", .{});
    print("    --verbose       Enable verbose output\n\n", .{});
    
    print("EXAMPLES:\n", .{});
    print("    cursed hello.csd                          # Interpret hello.csd\n", .{});
    print("    cursed hello.csd --compile                 # Compile hello.csd to native executable\n", .{});
    print("    cursed compile hello.csd                   # Compile with subcommand\n", .{});
    print("    cursed hello.csd --compile --verbose       # Compile with verbose output\n\n", .{});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    const config = parseArgs(args);
    
    switch (config.command) {
        .help => {
            printHelp();
        },
        .interpret => {
            if (config.source_file) |file| {
                print("🔧 Interpreting CURSED file: {s}\n", .{file});
                if (config.verbose) print("🐛 Verbose mode enabled\n", .{});
                print("✅ --compile flag integration successful! Interpreter mode works.\n", .{});
            } else {
                print("Error: No source file specified\n", .{});
            }
        },
        .compile => {
            if (config.source_file) |file| {
                print("🔨 Compiling CURSED file: {s}\n", .{file});
                if (config.verbose) print("🐛 Verbose mode enabled\n", .{});
                print("✅ --compile flag integration successful! Compilation mode triggered.\n", .{});
                print("🎯 The CLI now supports both 'cursed file.csd --compile' and 'cursed compile file.csd'\n", .{});
            } else {
                print("Error: No source file specified for compilation\n", .{});
            }
        },
    }
}
