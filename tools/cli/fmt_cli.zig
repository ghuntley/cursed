// CURSED Formatter CLI Wrapper
// Provides command-line interface for the enhanced CURSED formatter

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const VERSION = "1.0.0";
const CURSED_FORMATTER_PATH = "tools/enhanced_formatter.csd";
const CURSED_INTERPRETER = "cursed-unified";

fn printHelp() void {
    print(
        \\CURSED Code Formatter - Production Edition
        \\
        \\USAGE:
        \\    cursed-fmt [OPTIONS] [FILES...]
        \\
        \\OPTIONS:
        \\    -h, --help              Show this help message
        \\    -V, --version           Show version information
        \\    -c, --check             Check if files are formatted (don't write)
        \\    -w, --write             Write formatted output to files
        \\    -d, --diff              Show diff of formatting changes
        \\    -q, --quiet             Suppress output except errors
        \\    -v, --verbose           Verbose output with statistics
        \\    --config <file>         Use custom formatter configuration
        \\    --stdin                 Read from stdin instead of files
        \\    --test                  Run formatter tests
        \\
        \\EXAMPLES:
        \\    cursed-fmt src/main.csd              Format and display main.csd
        \\    cursed-fmt --write src/*.csd         Format all .csd files in src/
        \\    cursed-fmt --check src/              Check if src/ files are formatted
        \\    echo "sus x drip=42" | cursed-fmt --stdin  Format stdin input
        \\
        \\For more information, visit: https://cursed-lang.org/tools/formatter
        \\
    );
}

fn printVersion() void {
    print("cursed-fmt {s}\n", .{VERSION});
}

fn runTest() !void {
    print("🧪 Running CURSED formatter tests...\n");
    
    // Test the enhanced formatter
    const result = std.ChildProcess.exec(.{
        .allocator = std.heap.page_allocator,
        .argv = &[_][]const u8{ CURSED_INTERPRETER, CURSED_FORMATTER_PATH },
    }) catch |err| {
        print("❌ Error running formatter tests: {}\n", .{err});
        return;
    };
    
    defer std.heap.page_allocator.free(result.stdout);
    defer std.heap.page_allocator.free(result.stderr);
    
    if (result.term.Exited == 0) {
        print("✅ Formatter tests passed!\n");
        print("{s}\n", .{result.stdout});
    } else {
        print("❌ Formatter tests failed!\n");
        print("Error: {s}\n", .{result.stderr});
    }
}

fn formatFile(file_path: []const u8, options: FormatOptions) !void {
    if (options.verbose) {
        print("🎨 Formatting file: {s}\n", .{file_path});
    }
    
    // Read file content
    const file_content = std.fs.cwd().readFileAlloc(
        std.heap.page_allocator,
        file_path,
        std.math.maxInt(usize),
    ) catch |err| {
        print("❌ Error reading file {s}: {}\n", .{ file_path, err });
        return;
    };
    defer std.heap.page_allocator.free(file_content);
    
    // Create temporary file for input
    const temp_input = "temp_input.csd";
    const temp_file = std.fs.cwd().createFile(temp_input, .{}) catch |err| {
        print("❌ Error creating temporary file: {}\n", .{err});
        return;
    };
    defer temp_file.close();
    defer std.fs.cwd().deleteFile(temp_input) catch {};
    
    _ = temp_file.writeAll(file_content) catch |err| {
        print("❌ Error writing to temporary file: {}\n", .{err});
        return;
    };
    
    // Run the enhanced formatter via CURSED interpreter
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var argv = ArrayList([]const u8).init(allocator);
    try argv.append(CURSED_INTERPRETER);
    try argv.append(CURSED_FORMATTER_PATH);
    try argv.append(temp_input);
    
    const result = std.ChildProcess.exec(.{
        .allocator = allocator,
        .argv = argv.items,
    }) catch |err| {
        print("❌ Error running formatter: {}\n", .{err});
        return;
    };
    
    if (result.term.Exited != 0) {
        print("❌ Formatter failed for {s}\n", .{file_path});
        print("Error: {s}\n", .{result.stderr});
        return;
    }
    
    if (options.check) {
        // Check if file is already formatted
        if (std.mem.eql(u8, file_content, result.stdout)) {
            if (!options.quiet) {
                print("✅ {s} is already formatted\n", .{file_path});
            }
        } else {
            print("❌ {s} needs formatting\n", .{file_path});
            std.process.exit(1);
        }
    } else if (options.write) {
        // Write formatted content back to file
        const output_file = std.fs.cwd().createFile(file_path, .{}) catch |err| {
            print("❌ Error writing to {s}: {}\n", .{ file_path, err });
            return;
        };
        defer output_file.close();
        
        _ = output_file.writeAll(result.stdout) catch |err| {
            print("❌ Error writing formatted content to {s}: {}\n", .{ file_path, err });
            return;
        };
        
        if (!options.quiet) {
            print("✅ Formatted {s}\n", .{file_path});
        }
    } else if (options.diff) {
        // Show diff (simplified)
        if (!std.mem.eql(u8, file_content, result.stdout)) {
            print("📝 Diff for {s}:\n", .{file_path});
            print("--- Original\n+++ Formatted\n");
            // In a real implementation, would show proper diff
            print("Content changed.\n");
        } else {
            if (!options.quiet) {
                print("✅ {s} is already formatted\n", .{file_path});
            }
        }
    } else {
        // Display formatted content
        print("{s}", .{result.stdout});
    }
}

fn formatStdin(options: FormatOptions) !void {
    if (options.verbose) {
        print("🎨 Formatting stdin input...\n");
    }
    
    const stdin = std.io.getStdIn().reader();
    const input = stdin.readAllAlloc(
        std.heap.page_allocator,
        std.math.maxInt(usize),
    ) catch |err| {
        print("❌ Error reading from stdin: {}\n", .{err});
        return;
    };
    defer std.heap.page_allocator.free(input);
    
    // Create temporary file for input
    const temp_input = "temp_stdin.csd";
    const temp_file = std.fs.cwd().createFile(temp_input, .{}) catch |err| {
        print("❌ Error creating temporary file: {}\n", .{err});
        return;
    };
    defer temp_file.close();
    defer std.fs.cwd().deleteFile(temp_input) catch {};
    
    _ = temp_file.writeAll(input) catch |err| {
        print("❌ Error writing to temporary file: {}\n", .{err});
        return;
    };
    
    // Run formatter
    const result = std.ChildProcess.exec(.{
        .allocator = std.heap.page_allocator,
        .argv = &[_][]const u8{ CURSED_INTERPRETER, CURSED_FORMATTER_PATH, temp_input },
    }) catch |err| {
        print("❌ Error running formatter: {}\n", .{err});
        return;
    };
    
    defer std.heap.page_allocator.free(result.stdout);
    defer std.heap.page_allocator.free(result.stderr);
    
    if (result.term.Exited == 0) {
        print("{s}", .{result.stdout});
    } else {
        print("❌ Formatter failed\n");
        print("Error: {s}\n", .{result.stderr});
        std.process.exit(1);
    }
}

const FormatOptions = struct {
    check: bool = false,
    write: bool = false,
    diff: bool = false,
    quiet: bool = false,
    verbose: bool = false,
    stdin: bool = false,
    config_file: ?[]const u8 = null,
};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    var options = FormatOptions{};
    var files = ArrayList([]const u8).init(allocator);
    
    var i: usize = 1;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "-h") or std.mem.eql(u8, arg, "--help")) {
            printHelp();
            return;
        } else if (std.mem.eql(u8, arg, "-V") or std.mem.eql(u8, arg, "--version")) {
            printVersion();
            return;
        } else if (std.mem.eql(u8, arg, "--test")) {
            try runTest();
            return;
        } else if (std.mem.eql(u8, arg, "-c") or std.mem.eql(u8, arg, "--check")) {
            options.check = true;
        } else if (std.mem.eql(u8, arg, "-w") or std.mem.eql(u8, arg, "--write")) {
            options.write = true;
        } else if (std.mem.eql(u8, arg, "-d") or std.mem.eql(u8, arg, "--diff")) {
            options.diff = true;
        } else if (std.mem.eql(u8, arg, "-q") or std.mem.eql(u8, arg, "--quiet")) {
            options.quiet = true;
        } else if (std.mem.eql(u8, arg, "-v") or std.mem.eql(u8, arg, "--verbose")) {
            options.verbose = true;
        } else if (std.mem.eql(u8, arg, "--stdin")) {
            options.stdin = true;
        } else if (std.mem.eql(u8, arg, "--config")) {
            if (i + 1 < args.len) {
                i += 1;
                options.config_file = args[i];
            } else {
                print("❌ --config requires a file path\n");
                std.process.exit(1);
            }
        } else if (std.mem.startsWith(u8, arg, "-")) {
            print("❌ Unknown option: {s}\n", .{arg});
            std.process.exit(1);
        } else {
            try files.append(arg);
        }
    }
    
    if (options.stdin) {
        try formatStdin(options);
        return;
    }
    
    if (files.items.len == 0) {
        print("❌ No files specified. Use --stdin to read from stdin or specify files.\n");
        print("Run 'cursed-fmt --help' for usage information.\n");
        std.process.exit(1);
    }
    
    // Process files
    for (files.items) |file_path| {
        formatFile(file_path, options) catch |err| {
            print("❌ Error processing {s}: {}\n", .{ file_path, err });
            std.process.exit(1);
        };
    }
    
    if (options.verbose and !options.quiet) {
        print("🎉 Formatting complete!\n");
    }
}
