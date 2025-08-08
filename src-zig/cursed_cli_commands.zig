// CURSED CLI Commands for Build System Integration
// Provides command-line interface for CURSED project management

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

const cursed_build_system = @import("cursed_build_system.zig");
const cursed_project_templates = @import("cursed_project_templates.zig");

pub const CliCommand = struct {
    name: []const u8,
    description: []const u8,
    handler: *const fn(allocator: Allocator, args: [][]const u8) anyerror!void,
};

// Available CLI commands
pub const CLI_COMMANDS = [_]CliCommand{
    .{
        .name = "init",
        .description = "Initialize a new CURSED project",
        .handler = handleInit,
    },
    .{
        .name = "compile",
        .description = "Compile CURSED project",
        .handler = handleCompile,
    },
    .{
        .name = "run",
        .description = "Run CURSED project",
        .handler = handleRun,
    },
    .{
        .name = "test",
        .description = "Run tests for CURSED project",
        .handler = handleTest,
    },
    .{
        .name = "clean",
        .description = "Clean build artifacts",
        .handler = handleClean,
    },
    .{
        .name = "check",
        .description = "Check project for errors without building",
        .handler = handleCheck,
    },
    .{
        .name = "format",
        .description = "Format CURSED source files",
        .handler = handleFormat,
    },
    .{
        .name = "doc",
        .description = "Generate documentation",
        .handler = handleDoc,
    },
    .{
        .name = "install",
        .description = "Install project dependencies",
        .handler = handleInstall,
    },
    .{
        .name = "build",
        .description = "Build project using Zig build system",
        .handler = handleBuild,
    },
};

// Main CLI dispatcher
pub fn handleCliCommand(allocator: Allocator, args: [][]const u8) !void {
    if (args.len < 2) {
        showHelp();
        return;
    }
    
    const command = args[1];
    
    // Special commands
    if (std.mem.eql(u8, command, "help") or std.mem.eql(u8, command, "--help") or std.mem.eql(u8, command, "-h")) {
        showHelp();
        return;
    }
    
    if (std.mem.eql(u8, command, "version") or std.mem.eql(u8, command, "--version") or std.mem.eql(u8, command, "-v")) {
        showVersion();
        return;
    }
    
    // Find and execute command
    for (CLI_COMMANDS) |cmd| {
        if (std.mem.eql(u8, cmd.name, command)) {
            try cmd.handler(allocator, args);
            return;
        }
    }
    
    print("Unknown command: {s}\n", .{command});
    print("Run 'cursed help' for available commands.\n", .{});
}

fn showHelp() void {
    print("CURSED - The Programming Language for the Chronically Online\n", .{});
    print("Version: 1.0.0\n", .{});
    print("\n", .{});
    print("Usage: cursed <command> [options]\n", .{});
    print("\n", .{});
    print("Commands:\n", .{});
    
    for (CLI_COMMANDS) |cmd| {
        print("  {s:<12} {s}\n", .{ cmd.name, cmd.description });
    }
    
    print("\n", .{});
    print("Global Options:\n", .{});
    print("  -h, --help       Show this help message\n", .{});
    print("  -v, --version    Show version information\n", .{});
    print("  --verbose        Enable verbose output\n", .{});
    print("\n", .{});
    print("Examples:\n", .{});
    print("  cursed init my-project          Create new project\n", .{});
    print("  cursed compile --release        Compile with optimizations\n", .{});
    print("  cursed run                      Run the current project\n", .{});
    print("  cursed test                     Run all tests\n", .{});
}

fn showVersion() void {
    print("CURSED v1.0.0\n", .{});
    print("Build: zig-integration\n", .{});
    print("Target: native\n", .{});
}

// Command handlers

fn handleInit(allocator: Allocator, args: [][]const u8) !void {
    var project_name: []const u8 = "cursed-project";
    var template_name: []const u8 = "executable";
    var target_dir: ?[]const u8 = null;
    
    // Parse arguments
    var i: usize = 2;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--name")) {
            if (i + 1 >= args.len) {
                print("Error: --name requires a value\n", .{});
                return;
            }
            i += 1;
            project_name = args[i];
        } else if (std.mem.eql(u8, arg, "--template")) {
            if (i + 1 >= args.len) {
                print("Error: --template requires a value\n", .{});
                return;
            }
            i += 1;
            template_name = args[i];
        } else if (std.mem.eql(u8, arg, "--dir")) {
            if (i + 1 >= args.len) {
                print("Error: --dir requires a value\n", .{});
                return;
            }
            i += 1;
            target_dir = args[i];
        } else if (!std.mem.startsWith(u8, arg, "-")) {
            // Positional argument - project name
            project_name = arg;
        }
    }
    
    const final_target_dir = target_dir orelse project_name;
    
    var template_manager = cursed_project_templates.TemplateManager.init(allocator);
    
    template_manager.createProject(template_name, project_name, final_target_dir) catch |err| switch (err) {
        error.UnknownTemplate => {
            print("Error: Unknown template '{s}'\n", .{template_name});
            print("Available templates:\n", .{});
            template_manager.listTemplates();
        },
        else => return err,
    };
}

fn handleCompile(allocator: Allocator, args: [][]const u8) !void {
    var release_mode = false;
    var target: ?[]const u8 = null;
    var output: ?[]const u8 = null;
    var backend: []const u8 = "c";
    var project_dir: []const u8 = ".";
    
    // Parse arguments
    var i: usize = 2;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--release")) {
            release_mode = true;
        } else if (std.mem.eql(u8, arg, "--target")) {
            if (i + 1 >= args.len) {
                print("Error: --target requires a value\n", .{});
                return;
            }
            i += 1;
            target = args[i];
        } else if (std.mem.eql(u8, arg, "--output") or std.mem.eql(u8, arg, "-o")) {
            if (i + 1 >= args.len) {
                print("Error: --output requires a value\n", .{});
                return;
            }
            i += 1;
            output = args[i];
        } else if (std.mem.eql(u8, arg, "--backend") or std.mem.eql(u8, arg, "-b")) {
            if (i + 1 >= args.len) {
                print("Error: --backend requires a value\n", .{});
                return;
            }
            i += 1;
            backend = args[i];
        } else if (std.mem.eql(u8, arg, "--project")) {
            if (i + 1 >= args.len) {
                print("Error: --project requires a value\n", .{});
                return;
            }
            i += 1;
            project_dir = args[i];
        }
    }
    
    print("Compiling CURSED project in {s}...\n", .{project_dir});
    
    // Load project configuration
    const project_file = try std.fs.path.join(allocator, &[_][]const u8{ project_dir, "CursedPackage.toml" });
    defer allocator.free(project_file);
    
    var project = cursed_build_system.loadCursedProject(allocator, project_file) catch |err| switch (err) {
        error.FileNotFound => {
            print("Error: No CursedPackage.toml found in {s}\n", .{project_dir});
            print("Run 'cursed init' to create a new project.\n", .{});
            return;
        },
        else => return err,
    };
    defer project.deinit();
    
    // Apply command line overrides
    if (release_mode) {
        project.build_config.optimization = .release_fast;
    }
    
    // Create compilation command
    var compile_args = ArrayList([]const u8).init(allocator);
    defer compile_args.deinit();
    
    try compile_args.append("zig");
    try compile_args.append("build");
    
    if (target) |t| {
        try compile_args.append(try std.fmt.allocPrint(allocator, "-Dtarget={s}", .{t}));
    }
    
    if (release_mode) {
        try compile_args.append("-Doptimize=ReleaseFast");
    }
    
    try compile_args.append("cursed-compile");
    
    // Execute compilation
    const result = try std.process.Child.run(.{
        .allocator = allocator,
        .argv = compile_args.items,
        .cwd = project_dir,
    });
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    if (result.term.Exited == 0) {
        print("Compilation successful!\n", .{});
        if (output) |o| {
            print("Output: {s}\n", .{o});
        }
    } else {
        print("Compilation failed:\n", .{});
        print("{s}\n", .{result.stderr});
    }
}

fn handleRun(allocator: Allocator, args: [][]const u8) !void {
    var project_dir: []const u8 = ".";
    var run_args = ArrayList([]const u8).init(allocator);
    defer run_args.deinit();
    
    // Parse arguments
    var i: usize = 2;
    var collecting_args = false;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--project")) {
            if (i + 1 >= args.len) {
                print("Error: --project requires a value\n", .{});
                return;
            }
            i += 1;
            project_dir = args[i];
        } else if (std.mem.eql(u8, arg, "--")) {
            collecting_args = true;
        } else if (collecting_args) {
            try run_args.append(arg);
        }
    }
    
    print("Running CURSED project in {s}...\n", .{project_dir});
    
    // First compile the project
    try handleCompile(allocator, &[_][]const u8{ "cursed", "compile", "--project", project_dir });
    
    // Then run it using Zig build system
    var exec_args = ArrayList([]const u8).init(allocator);
    defer exec_args.deinit();
    
    try exec_args.append("zig");
    try exec_args.append("build");
    try exec_args.append("cursed-run");
    
    for (run_args.items) |arg| {
        try exec_args.append(arg);
    }
    
    const result = try std.process.Child.run(.{
        .allocator = allocator,
        .argv = exec_args.items,
        .cwd = project_dir,
    });
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    print("{s}", .{result.stdout});
    if (result.stderr.len > 0) {
        print("{s}", .{result.stderr});
    }
}

fn handleTest(allocator: Allocator, args: [][]const u8) !void {
    var project_dir: []const u8 = ".";
    var test_pattern: ?[]const u8 = null;
    
    // Parse arguments
    var i: usize = 2;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--project")) {
            if (i + 1 >= args.len) {
                print("Error: --project requires a value\n", .{});
                return;
            }
            i += 1;
            project_dir = args[i];
        } else if (std.mem.eql(u8, arg, "--pattern")) {
            if (i + 1 >= args.len) {
                print("Error: --pattern requires a value\n", .{});
                return;
            }
            i += 1;
            test_pattern = args[i];
        }
    }
    
    print("Running tests for CURSED project in {s}...\n", .{project_dir});
    
    const result = try std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "zig", "build", "cursed-test" },
        .cwd = project_dir,
    });
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    print("{s}", .{result.stdout});
    if (result.stderr.len > 0) {
        print("{s}", .{result.stderr});
    }
    
    if (result.term.Exited == 0) {
        print("All tests passed!\n", .{});
    } else {
        print("Some tests failed.\n", .{});
    }
}

fn handleClean(allocator: Allocator, args: [][]const u8) !void {
    var project_dir: []const u8 = ".";
    
    // Parse arguments
    var i: usize = 2;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--project")) {
            if (i + 1 >= args.len) {
                print("Error: --project requires a value\n", .{});
                return;
            }
            i += 1;
            project_dir = args[i];
        }
    }
    
    print("Cleaning build artifacts in {s}...\n", .{project_dir});
    
    const result = try std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "zig", "build", "cursed-clean" },
        .cwd = project_dir,
    });
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    print("Clean completed.\n", .{});
}

fn handleCheck(allocator: Allocator, args: [][]const u8) !void {
    _ = allocator;
    _ = args;
    print("Checking CURSED project...\n", .{});
    print("Type checking and syntax validation (not yet implemented)\n", .{});
}

fn handleFormat(allocator: Allocator, args: [][]const u8) !void {
    var project_dir: []const u8 = ".";
    
    // Parse arguments
    var i: usize = 2;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--project")) {
            if (i + 1 >= args.len) {
                print("Error: --project requires a value\n", .{});
                return;
            }
            i += 1;
            project_dir = args[i];
        }
    }
    
    print("Formatting CURSED source files in {s}...\n", .{project_dir});
    
    // Find and format all .csd files
    var dir = std.fs.cwd().openDir(project_dir, .{ .iterate = true }) catch |err| {
        print("Error opening directory: {}\n", .{err});
        return;
    };
    defer dir.close();
    
    var walker = try dir.walk(allocator);
    defer walker.deinit();
    
    while (try walker.next()) |entry| {
        if (entry.kind == .file and std.mem.endsWith(u8, entry.path, ".csd")) {
            print("Formatting: {s}\n", .{entry.path});
            // Format logic would go here
        }
    }
    
    print("Formatting completed.\n", .{});
}

fn handleDoc(allocator: Allocator, args: [][]const u8) !void {
    _ = allocator;
    var project_dir: []const u8 = ".";
    var output_dir: []const u8 = "docs";
    
    // Parse arguments
    var i: usize = 2;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--project")) {
            if (i + 1 >= args.len) {
                print("Error: --project requires a value\n", .{});
                return;
            }
            i += 1;
            project_dir = args[i];
        } else if (std.mem.eql(u8, arg, "--output")) {
            if (i + 1 >= args.len) {
                print("Error: --output requires a value\n", .{});
                return;
            }
            i += 1;
            output_dir = args[i];
        }
    }
    
    print("Generating documentation for {s} -> {s}...\n", .{ project_dir, output_dir });
    print("Documentation generation (not yet implemented)\n", .{});
}

fn handleInstall(allocator: Allocator, args: [][]const u8) !void {
    _ = allocator;
    _ = args;
    print("Installing dependencies...\n", .{});
    print("Package management (not yet implemented)\n", .{});
}

fn handleBuild(allocator: Allocator, args: [][]const u8) !void {
    var project_dir: []const u8 = ".";
    var build_args = ArrayList([]const u8).init(allocator);
    defer build_args.deinit();
    
    // Parse arguments
    var i: usize = 2;
    while (i < args.len) : (i += 1) {
        const arg = args[i];
        
        if (std.mem.eql(u8, arg, "--project")) {
            if (i + 1 >= args.len) {
                print("Error: --project requires a value\n", .{});
                return;
            }
            i += 1;
            project_dir = args[i];
        } else {
            try build_args.append(arg);
        }
    }
    
    print("Building CURSED project using Zig build system...\n", .{});
    
    var exec_args = ArrayList([]const u8).init(allocator);
    defer exec_args.deinit();
    
    try exec_args.append("zig");
    try exec_args.append("build");
    
    for (build_args.items) |arg| {
        try exec_args.append(arg);
    }
    
    const result = try std.process.Child.run(.{
        .allocator = allocator,
        .argv = exec_args.items,
        .cwd = project_dir,
    });
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    print("{s}", .{result.stdout});
    if (result.stderr.len > 0) {
        print("{s}", .{result.stderr});
    }
    
    if (result.term.Exited == 0) {
        print("Build successful!\n", .{});
    } else {
        print("Build failed.\n", .{});
    }
}

// Test CLI commands
test "cli command parsing" {
    const allocator = std.testing.allocator;
    
    // Test help command
    const help_args = [_][]const u8{ "cursed", "help" };
    try handleCliCommand(allocator, &help_args);
    
    // Test version command
    const version_args = [_][]const u8{ "cursed", "version" };
    try handleCliCommand(allocator, &version_args);
}
