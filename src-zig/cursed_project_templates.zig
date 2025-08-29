// CURSED Project Template System
// Provides project scaffolding and templates for different project types

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

pub const ProjectTemplate = struct {
    name: []const u8,
    description: []const u8,
    files: ArrayList(TemplateFile),
    
    pub const TemplateFile = struct {
        path: []const u8,
        content: []const u8,
        executable: bool = false,
    };
    
    pub fn init(allocator: Allocator, name: []const u8, description: []const u8) ProjectTemplate {
        return ProjectTemplate{
            .name = name,
            .description = description,
            .files = .empty,
        };
    }
    
    pub fn deinit(self: *ProjectTemplate) void {
        self.files.deinit(self.allocator);
    }
    
    pub fn addFile(self: *ProjectTemplate, path: []const u8, content: []const u8, executable: bool) !void {
        try self.files.append(TemplateFile{
            .path = path,
            .content = content,
            .executable = executable,
        });
    }
};

// Standard templates for different project types
pub const ProjectTemplates = struct {
    
    // Basic executable project template
    pub fn createExecutableTemplate(allocator: Allocator, project_name: []const u8) !ProjectTemplate {
        var template = ProjectTemplate.init(allocator, "executable", "Basic CURSED executable project");
        
        // CursedPackage.toml
        const package_toml = 
            \\name = "{s}"
            \\version = "0.1.0"
            \\description = "A CURSED executable project"
            \\authors = ["Your Name <your.email@example.com>"]
            \\keywords = []
            \\categories = []
            \\
            \\[dependencies]
            \\
            \\[dev_dependencies]
            \\
            \\[build]
            \\target_type = "executable"
            \\optimization = "debug"
            \\use_llvm = true
            \\
        ;
        
        const package_content = try std.fmt.allocPrint(allocator, package_toml, .{project_name});
        try template.addFile("CursedPackage.toml", package_content, false);
        
        // main.csd
        const main_csd = 
            \\yeet "vibez"
            \\
            \\slay main() {
            \\    vibez.spill("Hello, CURSED World!")
            \\    vibez.spill("Welcome to your new {s} project!")
            \\}
            \\
        ;
        
        const main_content = try std.fmt.allocPrint(allocator, main_csd, .{project_name});
        try template.addFile("src/main.csd", main_content, false);
        
        // test/main_test.csd
        const test_csd = 
            \\yeet "testz"
            \\yeet "../src/main"
            \\
            \\test_start("main function test")
            \\
            \\// Add your tests here
            \\assert_true(based)
            \\
            \\print_test_summary()
            \\
        ;
        
        try template.addFile("test/main_test.csd", test_csd, false);
        
        // README.md
        const readme_md = 
            \\# {s}
            \\
            \\A CURSED project.
            \\
            \\## Building
            \\
            \\```bash
            \\zig build cursed-compile
            \\```
            \\
            \\## Running
            \\
            \\```bash
            \\zig build cursed-run
            \\```
            \\
            \\## Testing
            \\
            \\```bash
            \\zig build cursed-test
            \\```
            \\
            \\## Dependencies
            \\
            \\- Zig (for build system)
            \\- CURSED compiler
            \\
        ;
        
        const readme_content = try std.fmt.allocPrint(allocator, readme_md, .{project_name});
        try template.addFile("README.md", readme_content, false);
        
        // .gitignore
        const gitignore = 
            \\# CURSED build artifacts
            \\target/
            \\build/
            \\zig-cache/
            \\zig-out/
            \\
            \\# IDE files
            \\.vscode/
            \\.idea/
            \\*.swp
            \\*.swo
            \\*~
            \\
            \\# OS files
            \\.DS_Store
            \\Thumbs.db
            \\
        ;
        
        try template.addFile(".gitignore", gitignore, false);
        
        // build.zig for Zig build system integration
        const build_zig = 
            \\const std = @import("std");
            \\
            \\pub fn build(b: *std.Build) void {
            \\    const target = b.standardTargetOptions(.{});
            \\    const optimize = b.standardOptimizeOption(.{});
            \\
            \\    // Use CURSED build integration
            \\    const cursed_build = @import("cursed_build_system.zig");
            \\    cursed_build.createCursedBuildStep(b, target, optimize, "cursed") catch |err| {
            \\        std.debug.print("CURSED build integration failed: {s}\n", .{err});
            \\    };
            \\}
            \\
        ;
        
        try template.addFile("build.zig", build_zig, false);
        
        return template;
    }
    
    // Library project template
    pub fn createLibraryTemplate(allocator: Allocator, project_name: []const u8) !ProjectTemplate {
        var template = ProjectTemplate.init(allocator, "library", "CURSED library project");
        
        // CursedPackage.toml for library
        const package_toml = 
            \\name = "{s}"
            \\version = "0.1.0"
            \\description = "A CURSED library"
            \\authors = ["Your Name <your.email@example.com>"]
            \\keywords = []
            \\categories = []
            \\
            \\[dependencies]
            \\
            \\[dev_dependencies]
            \\testz = "1.0.0"
            \\
            \\[build]
            \\target_type = "static_library"
            \\optimization = "release_safe"
            \\use_llvm = true
            \\
        ;
        
        const package_content = try std.fmt.allocPrint(allocator, package_toml, .{project_name});
        try template.addFile("CursedPackage.toml", package_content, false);
        
        // lib.csd
        const lib_csd = 
            \\// {s} - A CURSED Library
            \\
            \\// Public API functions
            \\slay hello(name tea) tea {
            \\    damn "Hello, " + name + "!"
            \\}
            \\
            \\slay add(a drip, b drip) drip {
            \\    damn a + b
            \\}
            \\
            \\// Library version
            \\sus LIB_VERSION tea = "0.1.0"
            \\
            \\// Library metadata
            \\squad LibInfo {
            \\    spill name tea
            \\    spill version tea
            \\    spill description tea
            \\}
            \\
            \\slay get_lib_info() LibInfo {
            \\    damn LibInfo{
            \\        name: "{s}",
            \\        version: LIB_VERSION,
            \\        description: "A CURSED library"
            \\    }
            \\}
            \\
        ;
        
        const lib_content = try std.fmt.allocPrint(allocator, lib_csd, .{ project_name, project_name });
        try template.addFile("src/lib.csd", lib_content, false);
        
        // test/lib_test.csd
        const test_csd = 
            \\yeet "testz"
            \\yeet "../src/lib"
            \\
            \\test_start("{s} library tests")
            \\
            \\// Test hello function
            \\sus greeting tea = hello("CURSED")
            \\assert_eq_string(greeting, "Hello, CURSED!")
            \\
            \\// Test add function
            \\sus result drip = add(2, 3)
            \\assert_eq_int(result, 5)
            \\
            \\// Test library info
            \\sus info LibInfo = get_lib_info()
            \\assert_eq_string(info.name, "{s}")
            \\assert_eq_string(info.version, "0.1.0")
            \\
            \\print_test_summary()
            \\
        ;
        
        const test_content = try std.fmt.allocPrint(allocator, test_csd, .{ project_name, project_name });
        try template.addFile("test/lib_test.csd", test_content, false);
        
        // examples/usage.csd
        const example_csd = 
            \\yeet "vibez"
            \\yeet "../src/lib"
            \\
            \\slay main() {
            \\    vibez.spill("Example usage of {s}")
            \\    
            \\    sus greeting tea = hello("World")
            \\    vibez.spill(greeting)
            \\    
            \\    sus sum drip = add(10, 20)
            \\    vibez.spillf("10 + 20 = %d", sum)
            \\    
            \\    sus info LibInfo = get_lib_info()
            \\    vibez.spillf("Library: %s v%s", info.name, info.version)
            \\}
            \\
        ;
        
        const example_content = try std.fmt.allocPrint(allocator, example_csd, .{project_name});
        try template.addFile("examples/usage.csd", example_content, false);
        
        // README.md for library
        const readme_md = 
            \\# {s}
            \\
            \\A CURSED library.
            \\
            \\## Installation
            \\
            \\Add to your `CursedPackage.toml`:
            \\
            \\```toml
            \\[dependencies]
            \\{s} = "0.1.0"
            \\```
            \\
            \\## Usage
            \\
            \\```cursed
            \\yeet "{s}"
            \\
            \\slay main() {
            \\    sus greeting tea = hello("World")
            \\    vibez.spill(greeting)
            \\}
            \\```
            \\
            \\## API Documentation
            \\
            \\### Functions
            \\
            \\- `hello(name: tea) -> tea` - Returns a greeting message
            \\- `add(a: drip, b: drip) -> drip` - Adds two numbers
            \\- `get_lib_info() -> LibInfo` - Returns library information
            \\
            \\## Building
            \\
            \\```bash
            \\zig build cursed-compile
            \\```
            \\
            \\## Testing
            \\
            \\```bash
            \\zig build cursed-test
            \\```
            \\
        ;
        
        const readme_content = try std.fmt.allocPrint(allocator, readme_md, .{ project_name, project_name, project_name });
        try template.addFile("README.md", readme_content, false);
        
        // .gitignore
        const gitignore = 
            \\# CURSED build artifacts
            \\target/
            \\build/
            \\zig-cache/
            \\zig-out/
            \\
            \\# IDE files
            \\.vscode/
            \\.idea/
            \\*.swp
            \\*.swo
            \\*~
            \\
            \\# OS files
            \\.DS_Store
            \\Thumbs.db
            \\
        ;
        
        try template.addFile(".gitignore", gitignore, false);
        
        return template;
    }
    
    // Web project template with WASM support
    pub fn createWebTemplate(allocator: Allocator, project_name: []const u8) !ProjectTemplate {
        var template = ProjectTemplate.init(allocator, "web", "CURSED web project with WASM");
        
        // CursedPackage.toml for web
        const package_toml = 
            \\name = "{s}"
            \\version = "0.1.0"
            \\description = "A CURSED web application"
            \\authors = ["Your Name <your.email@example.com>"]
            \\keywords = ["web", "wasm"]
            \\categories = ["web"]
            \\
            \\[dependencies]
            \\
            \\[dev_dependencies]
            \\
            \\[build]
            \\target_type = "executable"
            \\optimization = "release_small"
            \\use_llvm = false
            \\
            \\[targets.wasm]
            \\enabled = true
            \\optimization = "release_small"
            \\
        ;
        
        const package_content = try std.fmt.allocPrint(allocator, package_toml, .{project_name});
        try template.addFile("CursedPackage.toml", package_content, false);
        
        // main.csd for web
        const main_csd = 
            \\// {s} - CURSED Web Application
            \\
            \\// Web-specific stdlib imports would go here
            \\
            \\slay main() {
            \\    // Initialize web application
            \\    init_web_app()
            \\    
            \\    // Set up event handlers
            \\    setup_handlers()
            \\    
            \\    // Main application loop
            \\    run_app()
            \\}
            \\
            \\slay init_web_app() {
            \\    // Web app initialization
            \\}
            \\
            \\slay setup_handlers() {
            \\    // Event handler setup
            \\}
            \\
            \\slay run_app() {
            \\    // Main application logic
            \\}
            \\
        ;
        
        const main_content = try std.fmt.allocPrint(allocator, main_csd, .{project_name});
        try template.addFile("src/main.csd", main_content, false);
        
        // index.html
        const index_html = 
            \\<!DOCTYPE html>
            \\<html lang="en">
            \\<head>
            \\    <meta charset="UTF-8">
            \\    <meta name="viewport" content="width=device-width, initial-scale=1.0">
            \\    <title>{s}</title>
            \\    <style>
            \\        body {
            \\            font-family: Arial, sans-serif;
            \\            margin: 0;
            \\            padding: 20px;
            \\            background-color: #f0f0f0;
            \\        }
            \\        .container {
            \\            max-width: 800px;
            \\            margin: 0 auto;
            \\            background: white;
            \\            padding: 20px;
            \\            border-radius: 8px;
            \\            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            \\        }
            \\        h1 {
            \\            color: #333;
            \\            text-align: center;
            \\        }
            \\        #app {
            \\            margin-top: 20px;
            \\        }
            \\    </style>
            \\</head>
            \\<body>
            \\    <div class="container">
            \\        <h1>{s}</h1>
            \\        <div id="app">
            \\            <p>Loading CURSED WASM application...</p>
            \\        </div>
            \\    </div>
            \\    
            \\    <script>
            \\        // Load WASM module
            \\        WebAssembly.instantiateStreaming(fetch('target/wasm32-wasi/{s}.wasm'))
            \\            .then(result => {
            \\                // Initialize CURSED app
            \\                result.instance.exports.main();
            \\            })
            \\            .catch(error => {
            \\                console.error('Error loading WASM:', error);
            \\                document.getElementById('app').innerHTML = 
            \\                    '<p style="color: red;">Failed to load WASM application</p>';
            \\            });
            \\    </script>
            \\</body>
            \\</html>
            \\
        ;
        
        const html_content = try std.fmt.allocPrint(allocator, index_html, .{ project_name, project_name, project_name });
        try template.addFile("web/index.html", html_content, false);
        
        // Build script for web
        const build_sh = 
            \\#!/bin/bash
            \\set -e
            \\
            \\echo "Building {s} for web..."
            \\
            \\# Build WASM target
            \\zig build -Dtarget=wasm32-wasi cursed-compile
            \\
            \\# Copy WASM file to web directory
            \\mkdir -p web/target/wasm32-wasi/
            \\cp target/wasm32-wasi/{s}.wasm web/target/wasm32-wasi/
            \\
            \\echo "Build complete! Open web/index.html in a browser"
            \\
        ;
        
        const build_content = try std.fmt.allocPrint(allocator, build_sh, .{ project_name, project_name });
        try template.addFile("build-web.sh", build_content, true);
        
        return template;
    }
    
    // CLI tool template
    pub fn createCliTemplate(allocator: Allocator, project_name: []const u8) !ProjectTemplate {
        var template = ProjectTemplate.init(allocator, "cli", "CURSED command-line tool");
        
        // CursedPackage.toml for CLI
        const package_toml = 
            \\name = "{s}"
            \\version = "0.1.0"
            \\description = "A CURSED command-line tool"
            \\authors = ["Your Name <your.email@example.com>"]
            \\keywords = ["cli", "tool"]
            \\categories = ["command-line-utilities"]
            \\
            \\[dependencies]
            \\
            \\[dev_dependencies]
            \\testz = "1.0.0"
            \\
            \\[build]
            \\target_type = "executable"
            \\optimization = "release_fast"
            \\use_llvm = true
            \\static_linking = true
            \\
        ;
        
        const package_content = try std.fmt.allocPrint(allocator, package_toml, .{project_name});
        try template.addFile("CursedPackage.toml", package_content, false);
        
        // main.csd for CLI
        const main_csd = 
            \\yeet "vibez"
            \\yeet "args"
            \\
            \\slay main() {
            \\    sus args []tea = args.get_args()
            \\    
            \\    if (args.length <= 1) {
            \\        show_help()
            \\        damn
            \\    }
            \\    
            \\    sus command tea = args[1]
            \\    
            \\    match (command) {
            \\        "help" => show_help(),
            \\        "version" => show_version(),
            \\        "greet" => handle_greet(args),
            \\        default => {
            \\            vibez.spillf("Unknown command: %s", command)
            \\            show_help()
            \\        }
            \\    }
            \\}
            \\
            \\slay show_help() {
            \\    vibez.spill("{s} - A CURSED CLI tool")
            \\    vibez.spill("")
            \\    vibez.spill("Usage:")
            \\    vibez.spill("  {s} <command> [options]")
            \\    vibez.spill("")
            \\    vibez.spill("Commands:")
            \\    vibez.spill("  help        Show this help message")
            \\    vibez.spill("  version     Show version information")
            \\    vibez.spill("  greet <name> Greet someone")
            \\}
            \\
            \\slay show_version() {
            \\    vibez.spill("{s} v0.1.0")
            \\}
            \\
            \\slay handle_greet(args []tea) {
            \\    if (args.length < 3) {
            \\        vibez.spill("Error: greet command requires a name")
            \\        damn
            \\    }
            \\    
            \\    sus name tea = args[2]
            \\    vibez.spillf("Hello, %s! Welcome to CURSED CLI tools.", name)
            \\}
            \\
        ;
        
        const main_content = try std.fmt.allocPrint(allocator, main_csd, .{ project_name, project_name, project_name });
        try template.addFile("src/main.csd", main_content, false);
        
        return template;
    }
};

// Template manager for creating projects
pub const TemplateManager = struct {
    allocator: Allocator,
    
    pub fn init() TemplateManager {
        return TemplateManager{
            .allocator = allocator,
        };
    }
    
    pub fn createProject(
        self: *TemplateManager,
        template_name: []const u8,
        project_name: []const u8,
        target_dir: []const u8
    ) !void {
        const template = try self.getTemplate(template_name, project_name);
        defer {
            var mut_template = template;
            mut_template.deinit();
        }
        
        print("Creating {s} project '{s}' in {s}\n", .{ template_name, project_name, target_dir });
        
        // Create target directory
        std.fs.cwd().makeDir(target_dir) catch |err| switch (err) {
            error.PathAlreadyExists => {},
            else => return err,
        };
        
        // Create all template files
        for (template.files.items) |file| {
            const full_path = try std.fs.path.join(self.allocator, &[_][]const u8{ target_dir, file.path });
            defer self.allocator.free(full_path);
            
            // Create parent directory if needed
            if (std.fs.path.dirname(full_path)) |dir| {
                std.fs.cwd().makePath(dir) catch |err| switch (err) {
                    error.PathAlreadyExists => {},
                    else => return err,
                };
            }
            
            // Write file content
            const output_file = try std.fs.cwd().createFile(full_path, .{});
            defer output_file.close();
            
            try output_file.writer().writeAll(file.content);
            
            // Set executable permission if needed
            if (file.executable) {
                // On Unix systems, make file executable
                if (@import("builtin").os.tag != .windows) {
                    _ = std.os.system(try std.fmt.allocPrintZ(
                        self.allocator, "chmod +x {s}", .{full_path}
                    )) catch {};
                }
            }
            
            print("Created: {s}\n", .{file.path});
        }
        
        print("Project '{s}' created successfully!\n", .{project_name});
        print("Next steps:\n", .{});
        print("  cd {s}\n", .{target_dir});
        print("  zig build cursed-compile\n", .{});
    }
    
    fn getTemplate(self: *TemplateManager, template_name: []const u8, project_name: []const u8) !ProjectTemplate {
        if (std.mem.eql(u8, template_name, "executable")) {
            return ProjectTemplates.createExecutableTemplate(self.allocator, project_name);
        } else if (std.mem.eql(u8, template_name, "library")) {
            return ProjectTemplates.createLibraryTemplate(self.allocator, project_name);
        } else if (std.mem.eql(u8, template_name, "web")) {
            return ProjectTemplates.createWebTemplate(self.allocator, project_name);
        } else if (std.mem.eql(u8, template_name, "cli")) {
            return ProjectTemplates.createCliTemplate(self.allocator, project_name);
        } else {
            return error.UnknownTemplate;
        }
    }
    
    pub fn listTemplates(self: *TemplateManager) void {
        _ = self;
        print("Available CURSED project templates:\n", .{});
        print("  executable  - Basic executable project\n", .{});
        print("  library     - Library project\n", .{});
        print("  web         - Web application with WASM support\n", .{});
        print("  cli         - Command-line tool\n", .{});
    }
};

// Test template system
test "template creation" {
    const allocator = std.testing.allocator;
    
    var manager = TemplateManager.init(allocator);
    
    const template = try manager.getTemplate("executable", "test-project");
    defer {
        var mut_template = template;
        mut_template.deinit();
    }
    
    try std.testing.expect(template.files.items.len > 0);
    try std.testing.expectEqualStrings("executable", template.name);
}
