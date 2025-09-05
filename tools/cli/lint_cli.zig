// CURSED Linter CLI Wrapper
// Provides command-line interface for the enhanced CURSED linter

const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

const VERSION = "1.0.0";
const CURSED_LINTER_PATH = "tools/enhanced_linter.💀";
const CURSED_INTERPRETER = "cursed-unified";

fn printHelp() void {
    print(
        \\CURSED Code Linter - Production Security & Quality Edition
        \\
        \\USAGE:
        \\    cursed-lint [OPTIONS] [FILES/DIRECTORIES...]
        \\
        \\OPTIONS:
        \\    -h, --help              Show this help message
        \\    -V, --version           Show version information
        \\    -c, --config <file>     Use custom linter configuration
        \\    -f, --format <format>   Output format: detailed, compact, json, sarif
        \\    -o, --output <file>     Write output to file instead of stdout
        \\    -q, --quiet             Show only errors and warnings
        \\    -v, --verbose           Show detailed analysis information
        \\    --fail-on <level>       Exit with error on: critical, error, warning, info
        \\    --fix                   Automatically fix auto-fixable issues
        \\    --test                  Run linter tests
        \\    --security-only         Run only security analysis
        \\    --performance-only      Run only performance analysis
        \\    --style-only            Run only style analysis
        \\    --no-security           Skip security analysis
        \\    --no-performance        Skip performance analysis
        \\    --no-style              Skip style analysis
        \\    --show-rules            List all available linting rules
        \\    --stdin                 Read from stdin instead of files
        \\
        \\SEVERITY LEVELS:
        \\    🔴 critical            Security vulnerabilities, critical bugs
        \\    🚨 error               Code errors, type mismatches
        \\    ⚠️  warning             Code quality issues, potential bugs
        \\    ℹ️  info                Performance suggestions, best practices
        \\    💡 hint                Style suggestions, minor improvements
        \\
        \\EXAMPLES:
        \\    cursed-lint src/                    Lint all files in src/
        \\    cursed-lint --security-only *.💀  Security analysis only
        \\    cursed-lint --fix src/main.💀.💀     Auto-fix issues in main.💀.💀
        \\    cursed-lint --format json > report.json  JSON output
        \\    echo "sus x drip=42" | cursed-lint --stdin  Lint stdin
        \\
        \\For more information, visit: https://cursed-lang.org/tools/linter
        \\
    );
}

fn printVersion() void {
    print("cursed-lint {s}\n", .{VERSION});
}

fn showRules() void {
    print(
        \\📋 Available Linting Rules
        \\=========================
        \\
        \\🔒 Security Rules:
        \\  hardcoded-secret         Detect hardcoded passwords and secrets
        \\  sql-injection-risk       SQL injection vulnerability patterns
        \\  command-injection-risk   Command injection vulnerabilities
        \\  weak-cryptography        Weak cryptographic algorithms
        \\  weak-random              Non-cryptographic random generation
        \\  unsafe-operation         Potentially unsafe operations
        \\
        \\⚡ Performance Rules:
        \\  string-concat-loop       String concatenation in loops
        \\  inefficient-array-length Array length computation in loops
        \\  nested-loops             Nested loop algorithmic complexity
        \\  allocation-in-loop       Memory allocation inside loops
        \\
        \\🧹 Quality Rules:
        \\  function-too-long        Function length limits
        \\  too-many-parameters      Function parameter limits
        \\  excessive-nesting        Nesting depth limits
        \\  magic-number             Magic number detection
        \\  missing-documentation    Documentation requirements
        \\
        \\🎨 Style Rules:
        \\  use-based                Use 'based' instead of 'true'
        \\  use-cringe               Use 'cringe' instead of 'false'
        \\  use-vibez                Use 'vibez.spill' for output
        \\  naming-convention        Enforce snake_case naming
        \\  missing-semicolon        Semicolon requirements
        \\
        \\For detailed rule documentation, visit:
        \\https://cursed-lang.org/tools/linter/rules
        \\
    );
}

fn runTest() !void {
    print("🧪 Running CURSED linter tests...\n");
    
    // Test the enhanced linter
    const result = std.ChildProcess.exec(.{
        .allocator = std.heap.page_allocator,
        .argv = &[_][]const u8{ CURSED_INTERPRETER, CURSED_LINTER_PATH },
    }) catch |err| {
        print("❌ Error running linter tests: {}\n", .{err});
        return;
    };
    
    defer std.heap.page_allocator.free(result.stdout);
    defer std.heap.page_allocator.free(result.stderr);
    
    if (result.term.Exited == 0) {
        print("✅ Linter tests passed!\n");
        print("{s}\n", .{result.stdout});
    } else {
        print("❌ Linter tests failed!\n");
        print("Error: {s}\n", .{result.stderr});
    }
}

fn lintFile(file_path: []const u8, options: LintOptions) !u8 {
    if (options.verbose) {
        print("🔍 Analyzing file: {s}\n", .{file_path});
    }
    
    // Read file content
    const file_content = std.fs.cwd().readFileAlloc(
        std.heap.page_allocator,
        file_path,
        std.math.maxInt(usize),
    ) catch |err| {
        print("❌ Error reading file {s}: {}\n", .{ file_path, err });
        return 1;
    };
    defer std.heap.page_allocator.free(file_content);
    
    // Create temporary file for input
    const temp_input = "temp_lint_input.💀";
    const temp_file = std.fs.cwd().createFile(temp_input, .{}) catch |err| {
        print("❌ Error creating temporary file: {}\n", .{err});
        return 1;
    };
    defer temp_file.close();
    defer std.fs.cwd().deleteFile(temp_input) catch {};
    
    _ = temp_file.writeAll(file_content) catch |err| {
        print("❌ Error writing to temporary file: {}\n", .{err});
        return 1;
    };
    
    // Run the enhanced linter via CURSED interpreter
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    var argv = ArrayList([]const u8).init(allocator);
    try argv.append(CURSED_INTERPRETER);
    try argv.append(CURSED_LINTER_PATH);
    try argv.append(temp_input);
    
    // Add options for specialized analysis
    if (options.security_only) {
        try argv.append("--security-only");
    }
    if (options.performance_only) {
        try argv.append("--performance-only");
    }
    if (options.style_only) {
        try argv.append("--style-only");
    }
    
    const result = std.ChildProcess.exec(.{
        .allocator = allocator,
        .argv = argv.items,
    }) catch |err| {
        print("❌ Error running linter: {}\n", .{err});
        return 1;
    };
    
    // Write output
    if (options.output_file) |output_path| {
        const output_file = std.fs.cwd().createFile(output_path, .{}) catch |err| {
            print("❌ Error creating output file {s}: {}\n", .{ output_path, err });
            return 1;
        };
        defer output_file.close();
        
        _ = output_file.writeAll(result.stdout) catch |err| {
            print("❌ Error writing to output file: {}\n", .{err});
            return 1;
        };
        
        if (!options.quiet) {
            print("📄 Linting results written to {s}\n", .{output_path});
        }
    } else {
        print("{s}", .{result.stdout});
    }
    
    if (result.stderr.len > 0 and options.verbose) {
        print("Debug info: {s}\n", .{result.stderr});
    }
    
    // Determine exit code based on fail-on level and issues found
    if (result.term.Exited != 0) {
        return 1; // Linter itself failed
    }
    
    // Parse output to determine severity of issues found
    // This is a simplified check - in practice would parse JSON/structured output
    if (std.mem.indexOf(u8, result.stdout, "🔴") != null and 
        (std.mem.eql(u8, options.fail_on, "critical") or std.mem.eql(u8, options.fail_on, "error") or 
         std.mem.eql(u8, options.fail_on, "warning") or std.mem.eql(u8, options.fail_on, "info"))) {
        return 2; // Critical issues found
    }
    
    if (std.mem.indexOf(u8, result.stdout, "🚨") != null and 
        (std.mem.eql(u8, options.fail_on, "error") or std.mem.eql(u8, options.fail_on, "warning") or 
         std.mem.eql(u8, options.fail_on, "info"))) {
        return 1; // Errors found
    }
    
    if (std.mem.indexOf(u8, result.stdout, "⚠️") != null and 
        (std.mem.eql(u8, options.fail_on, "warning") or std.mem.eql(u8, options.fail_on, "info"))) {
        return 1; // Warnings found
    }
    
    if (std.mem.indexOf(u8, result.stdout, "ℹ️") != null and std.mem.eql(u8, options.fail_on, "info")) {
        return 1; // Info issues found
    }
    
    return 0; // No issues or acceptable level
}

fn lintStdin(options: LintOptions) !u8 {
    if (options.verbose) {
        print("🔍 Analyzing stdin input...\n");
    }
    
    const stdin = std.io.getStdIn().reader();
    const input = stdin.readAllAlloc(
        std.heap.page_allocator,
        std.math.maxInt(usize),
    ) catch |err| {
        print("❌ Error reading from stdin: {}\n", .{err});
        return 1;
    };
    defer std.heap.page_allocator.free(input);
    
    // Create temporary file for input
    const temp_input = "temp_lint_stdin.💀";
    const temp_file = std.fs.cwd().createFile(temp_input, .{}) catch |err| {
        print("❌ Error creating temporary file: {}\n", .{err});
        return 1;
    };
    defer temp_file.close();
    defer std.fs.cwd().deleteFile(temp_input) catch {};
    
    _ = temp_file.writeAll(input) catch |err| {
        print("❌ Error writing to temporary file: {}\n", .{err});
        return 1;
    };
    
    // Use the same linting logic as file
    return lintFile(temp_input, options);
}

fn lintDirectory(dir_path: []const u8, options: LintOptions) !u8 {
    if (options.verbose) {
        print("📂 Scanning directory: {s}\n", .{dir_path});
    }
    
    var dir = std.fs.cwd().openIterableDir(dir_path, .{}) catch |err| {
        print("❌ Error opening directory {s}: {}\n", .{ dir_path, err });
        return 1;
    };
    defer dir.close();
    
    var iterator = dir.iterate();
    var exit_code: u8 = 0;
    var files_processed: u32 = 0;
    
    while (iterator.next() catch null) |entry| {
        if (entry.kind == .File and std.mem.endsWith(u8, entry.name, ".💀")) {
            var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
            defer arena.deinit();
            const allocator = arena.allocator();
            
            const full_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ dir_path, entry.name });
            const file_exit = try lintFile(full_path, options);
            
            if (file_exit > exit_code) {
                exit_code = file_exit;
            }
            
            files_processed += 1;
        }
    }
    
    if (options.verbose and !options.quiet) {
        print("📊 Processed {} CURSED files in {s}\n", .{ files_processed, dir_path });
    }
    
    return exit_code;
}

const LintOptions = struct {
    config_file: ?[]const u8 = null,
    format: []const u8 = "detailed",
    output_file: ?[]const u8 = null,
    quiet: bool = false,
    verbose: bool = false,
    fail_on: []const u8 = "error",
    fix: bool = false,
    security_only: bool = false,
    performance_only: bool = false,
    style_only: bool = false,
    no_security: bool = false,
    no_performance: bool = false,
    no_style: bool = false,
    stdin: bool = false,
};

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    var options = LintOptions{};
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
        } else if (std.mem.eql(u8, arg, "--show-rules")) {
            showRules();
            return;
        } else if (std.mem.eql(u8, arg, "--test")) {
            try runTest();
            return;
        } else if (std.mem.eql(u8, arg, "-c") or std.mem.eql(u8, arg, "--config")) {
            if (i + 1 < args.len) {
                i += 1;
                options.config_file = args[i];
            } else {
                print("❌ --config requires a file path\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "-f") or std.mem.eql(u8, arg, "--format")) {
            if (i + 1 < args.len) {
                i += 1;
                options.format = args[i];
            } else {
                print("❌ --format requires a format type\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "-o") or std.mem.eql(u8, arg, "--output")) {
            if (i + 1 < args.len) {
                i += 1;
                options.output_file = args[i];
            } else {
                print("❌ --output requires a file path\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "--fail-on")) {
            if (i + 1 < args.len) {
                i += 1;
                options.fail_on = args[i];
            } else {
                print("❌ --fail-on requires a severity level\n");
                std.process.exit(1);
            }
        } else if (std.mem.eql(u8, arg, "-q") or std.mem.eql(u8, arg, "--quiet")) {
            options.quiet = true;
        } else if (std.mem.eql(u8, arg, "-v") or std.mem.eql(u8, arg, "--verbose")) {
            options.verbose = true;
        } else if (std.mem.eql(u8, arg, "--fix")) {
            options.fix = true;
        } else if (std.mem.eql(u8, arg, "--security-only")) {
            options.security_only = true;
        } else if (std.mem.eql(u8, arg, "--performance-only")) {
            options.performance_only = true;
        } else if (std.mem.eql(u8, arg, "--style-only")) {
            options.style_only = true;
        } else if (std.mem.eql(u8, arg, "--no-security")) {
            options.no_security = true;
        } else if (std.mem.eql(u8, arg, "--no-performance")) {
            options.no_performance = true;
        } else if (std.mem.eql(u8, arg, "--no-style")) {
            options.no_style = true;
        } else if (std.mem.eql(u8, arg, "--stdin")) {
            options.stdin = true;
        } else if (std.mem.startsWith(u8, arg, "-")) {
            print("❌ Unknown option: {s}\n", .{arg});
            std.process.exit(1);
        } else {
            try files.append(arg);
        }
    }
    
    if (options.stdin) {
        const exit_code = try lintStdin(options);
        std.process.exit(exit_code);
    }
    
    if (files.items.len == 0) {
        print("❌ No files or directories specified. Use --stdin to read from stdin.\n");
        print("Run 'cursed-lint --help' for usage information.\n");
        std.process.exit(1);
    }
    
    // Process files and directories
    var max_exit_code: u8 = 0;
    
    for (files.items) |path| {
        const stat = std.fs.cwd().statFile(path) catch |err| {
            print("❌ Error accessing {s}: {}\n", .{ path, err });
            max_exit_code = 1;
            continue;
        };
        
        const exit_code = if (stat.kind == .Directory) 
            try lintDirectory(path, options)
        else 
            try lintFile(path, options);
        
        if (exit_code > max_exit_code) {
            max_exit_code = exit_code;
        }
    }
    
    if (options.verbose and !options.quiet) {
        switch (max_exit_code) {
            0 => print("✅ Linting completed successfully!\n"),
            1 => print("⚠️ Linting completed with warnings/errors\n"),
            2 => print("🔴 Linting found critical issues\n"),
            else => print("❌ Linting failed\n"),
        }
    }
    
    std.process.exit(max_exit_code);
}
