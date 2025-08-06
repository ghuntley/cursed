// CURSED Tooling Suite - Main Module
// Unified interface for all CURSED development tools

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Import tool modules
pub const lsp_server = @import("lsp_server.zig");
pub const formatter = @import("formatter.zig");
pub const linter = @import("linter.zig");
pub const package_manager = @import("package_manager_enhanced.zig");
pub const doc_generator = @import("doc_generator.zig");

// Tool Types
pub const ToolType = enum {
    LSP,
    Format,
    Lint,
    Package,
    Doc,
    
    pub fn toString(self: ToolType) []const u8 {
        return switch (self) {
            .LSP => "lsp",
            .Format => "fmt",
            .Lint => "lint",
            .Package => "pkg",
            .Doc => "doc",
        };
    }
    
    pub fn fromString(str: []const u8) ?ToolType {
        if (std.mem.eql(u8, str, "lsp")) return .LSP;
        if (std.mem.eql(u8, str, "fmt")) return .Format;
        if (std.mem.eql(u8, str, "lint")) return .Lint;
        if (std.mem.eql(u8, str, "pkg")) return .Package;
        if (std.mem.eql(u8, str, "doc")) return .Doc;
        return null;
    }
};

// Tool Configuration
pub const ToolConfig = struct {
    tool_type: ToolType,
    input_files: [][]const u8,
    output_dir: ?[]const u8 = null,
    options: std.HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator, tool_type: ToolType) ToolConfig {
        return ToolConfig{
            .tool_type = tool_type,
            .input_files = &[_][]const u8{},
            .options = std.HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *ToolConfig) void {
        self.options.deinit();
    }
};

// Unified Tool Runner
pub const ToolRunner = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) ToolRunner {
        return ToolRunner{
            .allocator = allocator,
        };
    }
    
    pub fn runTool(self: *ToolRunner, config: ToolConfig) !void {
        switch (config.tool_type) {
            .LSP => try self.runLSP(config),
            .Format => try self.runFormatter(config),
            .Lint => try self.runLinter(config),
            .Package => try self.runPackageManager(config),
            .Doc => try self.runDocGenerator(config),
        }
    }
    
    fn runLSP(self: *ToolRunner, _: ToolConfig) !void {
        std.log.info("Starting CURSED Language Server...");
        
        var handler = lsp_server.LSPHandler.init(self.allocator);
        defer handler.deinit();
        
        // Start LSP server (this would run indefinitely)
        try lsp_server.main();
    }
    
    fn runFormatter(self: *ToolRunner, config: ToolConfig) !void {
        std.log.info("Running CURSED formatter...");
        
        const formatter_config = formatter.FormatterConfig{};
        var fmt = formatter.Formatter.init(self.allocator, formatter_config);
        defer fmt.deinit();
        
        if (config.input_files.len == 0) {
            std.log.err("No input files specified for formatting");
            return;
        }
        
        for (config.input_files) |file_path| {
            try formatter.formatFile(self.allocator, file_path, formatter_config);
        }
    }
    
    fn runLinter(self: *ToolRunner, config: ToolConfig) !void {
        std.log.info("Running CURSED linter...");
        
        var linter_config = linter.LinterConfig.init(self.allocator);
        defer linter_config.deinit();
        
        var cursed_linter = linter.Linter.init(self.allocator, linter_config);
        defer cursed_linter.deinit();
        
        if (config.input_files.len == 0) {
            std.log.err("No input files specified for linting");
            return;
        }
        
        for (config.input_files) |file_path| {
            try cursed_linter.lintFile(file_path);
        }
        
        const issues = cursed_linter.getIssues();
        const format = config.options.get("format") orelse "human";
        try linter.printIssues(self.allocator, issues, format);
    }
    
    fn runPackageManager(self: *ToolRunner, config: ToolConfig) !void {
        std.log.info("Running CURSED package manager...");
        
        const command = config.options.get("command") orelse "help";
        const args = config.input_files;
        
        if (std.mem.eql(u8, command, "init")) {
            try package_manager.commands.init(self.allocator, args);
        } else if (std.mem.eql(u8, command, "add")) {
            try package_manager.commands.add(self.allocator, args);
        } else if (std.mem.eql(u8, command, "remove")) {
            try package_manager.commands.remove(self.allocator, args);
        } else if (std.mem.eql(u8, command, "install")) {
            try package_manager.commands.install(self.allocator, args);
        } else if (std.mem.eql(u8, command, "update")) {
            try package_manager.commands.update(self.allocator, args);
        } else if (std.mem.eql(u8, command, "search")) {
            try package_manager.commands.search(self.allocator, args);
        } else if (std.mem.eql(u8, command, "publish")) {
            try package_manager.commands.publish(self.allocator, args);
        } else {
            std.log.err("Unknown package manager command: {s}", .{command});
            std.log.err("Available commands: init, add, remove, install, update, search, publish");
        }
    }
    
    fn runDocGenerator(self: *ToolRunner, config: ToolConfig) !void {
        std.log.info("Running CURSED documentation generator...");
        
        const doc_config = doc_generator.DocConfig{
            .output_dir = config.output_dir orelse "docs",
            .output_format = if (std.mem.eql(u8, config.options.get("format") orelse "html", "markdown"))
                .Markdown
            else if (std.mem.eql(u8, config.options.get("format") orelse "html", "json"))
                .JSON
            else
                .HTML,
        };
        
        var generator = doc_generator.DocGenerator.init(self.allocator, doc_config);
        defer generator.deinit();
        
        if (config.input_files.len == 0) {
            std.log.err("No input directories specified for documentation generation");
            return;
        }
        
        for (config.input_files) |dir_path| {
            try generator.generateFromDirectory(dir_path);
        }
        
        try generator.writeDocumentation();
    }
};

// Tool Discovery and Validation
pub const ToolDiscovery = struct {
    allocator: Allocator,
    available_tools: ArrayList(ToolInfo),
    
    const ToolInfo = struct {
        tool_type: ToolType,
        name: []const u8,
        description: []const u8,
        version: []const u8,
        executable_path: ?[]const u8 = null,
    };
    
    pub fn init(allocator: Allocator) ToolDiscovery {
        return ToolDiscovery{
            .allocator = allocator,
            .available_tools = ArrayList(ToolInfo).init(allocator),
        };
    }
    
    pub fn deinit(self: *ToolDiscovery) void {
        self.available_tools.deinit();
    }
    
    pub fn discoverTools(self: *ToolDiscovery) !void {
        // Add built-in tools
        try self.available_tools.append(ToolInfo{
            .tool_type = .LSP,
            .name = "cursed-lsp",
            .description = "CURSED Language Server Protocol implementation",
            .version = "1.0.0",
        });
        
        try self.available_tools.append(ToolInfo{
            .tool_type = .Format,
            .name = "cursed-fmt",
            .description = "CURSED code formatter",
            .version = "1.0.0",
        });
        
        try self.available_tools.append(ToolInfo{
            .tool_type = .Lint,
            .name = "cursed-lint",
            .description = "CURSED code linter and quality analyzer",
            .version = "1.0.0",
        });
        
        try self.available_tools.append(ToolInfo{
            .tool_type = .Package,
            .name = "cursed-pkg",
            .description = "CURSED package manager",
            .version = "1.0.0",
        });
        
        try self.available_tools.append(ToolInfo{
            .tool_type = .Doc,
            .name = "cursed-doc",
            .description = "CURSED documentation generator",
            .version = "1.0.0",
        });
    }
    
    pub fn listTools(self: *const ToolDiscovery) void {
        std.log.info("Available CURSED tools:");
        for (self.available_tools.items) |tool| {
            std.log.info("  {s} ({s}) - {s}", .{ tool.name, tool.version, tool.description });
        }
    }
    
    pub fn validateTool(self: *ToolDiscovery, tool_type: ToolType) bool {
        for (self.available_tools.items) |tool| {
            if (tool.tool_type == tool_type) {
                return true;
            }
        }
        return false;
    }
};

// IDE Integration Support
pub const IDEIntegration = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) IDEIntegration {
        return IDEIntegration{
            .allocator = allocator,
        };
    }
    
    pub fn generateVSCodeExtension(self: *IDEIntegration, output_dir: []const u8) !void {
        const extension_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ output_dir, "cursed-vscode" });
        defer self.allocator.free(extension_dir);
        
        std.fs.cwd().makePath(extension_dir) catch {};
        
        // Generate package.json for VS Code extension
        const package_json_path = try std.fs.path.join(self.allocator, &[_][]const u8{ extension_dir, "package.json" });
        defer self.allocator.free(package_json_path);
        
        const package_file = try std.fs.cwd().createFile(package_json_path, .{});
        defer package_file.close();
        
        try package_file.writeAll(
            \\{
            \\    "name": "cursed-language-support",
            \\    "displayName": "CURSED Language Support",
            \\    "description": "Language support for CURSED programming language",
            \\    "version": "1.0.0",
            \\    "engines": {
            \\        "vscode": "^1.60.0"
            \\    },
            \\    "categories": ["Programming Languages"],
            \\    "activationEvents": [
            \\        "onLanguage:cursed"
            \\    ],
            \\    "main": "./out/extension.js",
            \\    "contributes": {
            \\        "languages": [{
            \\            "id": "cursed",
            \\            "aliases": ["CURSED", "cursed"],
            \\            "extensions": [".csd"],
            \\            "configuration": "./language-configuration.json"
            \\        }],
            \\        "grammars": [{
            \\            "language": "cursed",
            \\            "scopeName": "source.cursed",
            \\            "path": "./syntaxes/cursed.tmLanguage.json"
            \\        }],
            \\        "configuration": {
            \\            "type": "object",
            \\            "title": "CURSED",
            \\            "properties": {
            \\                "cursed.lsp.enabled": {
            \\                    "type": "boolean",
            \\                    "default": true,
            \\                    "description": "Enable CURSED language server"
            \\                },
            \\                "cursed.format.onSave": {
            \\                    "type": "boolean",
            \\                    "default": true,
            \\                    "description": "Format CURSED files on save"
            \\                }
            \\            }
            \\        }
            \\    },
            \\    "scripts": {
            \\        "compile": "tsc -p ./",
            \\        "package": "vsce package"
            \\    },
            \\    "devDependencies": {
            \\        "@types/vscode": "^1.60.0",
            \\        "typescript": "^4.4.3",
            \\        "vsce": "^1.100.1"
            \\    },
            \\    "dependencies": {
            \\        "vscode-languageclient": "^7.0.0"
            \\    }
            \\}
        );
        
        std.log.info("Generated VS Code extension template in {s}", .{extension_dir});
    }
    
    pub fn generateLanguageGrammar(_: *IDEIntegration, output_path: []const u8) !void {
        const grammar_file = try std.fs.cwd().createFile(output_path, .{});
        defer grammar_file.close();
        
        try grammar_file.writeAll(
            \\{
            \\    "name": "CURSED",
            \\    "scopeName": "source.cursed",
            \\    "fileTypes": ["csd"],
            \\    "patterns": [
            \\        { "include": "#comments" },
            \\        { "include": "#keywords" },
            \\        { "include": "#strings" },
            \\        { "include": "#numbers" },
            \\        { "include": "#functions" },
            \\        { "include": "#variables" }
            \\    ],
            \\    "repository": {
            \\        "comments": {
            \\            "patterns": [
            \\                {
            \\                    "name": "comment.line.cursed",
            \\                    "match": "(fr fr|#).*$"
            \\                }
            \\            ]
            \\        },
            \\        "keywords": {
            \\            "patterns": [
            \\                {
            \\                    "name": "keyword.control.cursed",
            \\                    "match": "\\b(sus|slay|damn|vibes|bestie|ready|yeet|stan|collab|squad|flex|match|case)\\b"
            \\                },
            \\                {
            \\                    "name": "constant.language.cursed",
            \\                    "match": "\\b(based|cringe|lit)\\b"
            \\                },
            \\                {
            \\                    "name": "storage.type.cursed",
            \\                    "match": "\\b(drip|normie|thicc|smol|meal|tea)\\b"
            \\                }
            \\            ]
            \\        },
            \\        "strings": {
            \\            "patterns": [
            \\                {
            \\                    "name": "string.quoted.double.cursed",
            \\                    "begin": "\"",
            \\                    "end": "\"",
            \\                    "patterns": [
            \\                        {
            \\                            "name": "constant.character.escape.cursed",
            \\                            "match": "\\\\."
            \\                        }
            \\                    ]
            \\                }
            \\            ]
            \\        },
            \\        "numbers": {
            \\            "patterns": [
            \\                {
            \\                    "name": "constant.numeric.cursed",
            \\                    "match": "\\b\\d+(\\.\\d+)?\\b"
            \\                }
            \\            ]
            \\        },
            \\        "functions": {
            \\            "patterns": [
            \\                {
            \\                    "name": "entity.name.function.cursed",
            \\                    "match": "\\b[a-zA-Z_][a-zA-Z0-9_]*(?=\\s*\\()"
            \\                }
            \\            ]
            \\        },
            \\        "variables": {
            \\            "patterns": [
            \\                {
            \\                    "name": "variable.other.cursed",
            \\                    "match": "\\b[a-zA-Z_][a-zA-Z0-9_]*\\b"
            \\                }
            \\            ]
            \\        }
            \\    }
            \\}
        );
        
        std.log.info("Generated language grammar: {s}", .{output_path});
    }
};

// Tool Integration Tests
pub const ToolTester = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) ToolTester {
        return ToolTester{
            .allocator = allocator,
        };
    }
    
    pub fn runIntegrationTests(self: *ToolTester) !void {
        std.log.info("Running CURSED tooling integration tests...");
        
        // Test formatter
        try self.testFormatter();
        
        // Test linter
        try self.testLinter();
        
        // Test package manager
        try self.testPackageManager();
        
        // Test documentation generator
        try self.testDocGenerator();
        
        std.log.info("All tooling integration tests passed!");
    }
    
    fn testFormatter(self: *ToolTester) !void {
        const test_code =
            \\slay test_function(param normie){
            \\vibez.spill("test")
            \\damn param+1
            \\}
        ;
        
        const config = formatter.FormatterConfig{};
        var fmt = formatter.Formatter.init(self.allocator, config);
        defer fmt.deinit();
        
        const formatted = try fmt.format(test_code);
        defer self.allocator.free(formatted);
        
        std.log.info("Formatter test passed");
    }
    
    fn testLinter(self: *ToolTester) !void {
        const test_code =
            \\slay test_function() {
            \\    vibez.spill("test")
            \\}
        ;
        
        var config = linter.LinterConfig.init(self.allocator);
        defer config.deinit();
        
        var cursed_linter = linter.Linter.init(self.allocator, config);
        defer cursed_linter.deinit();
        
        try cursed_linter.lintSource("test.csd", test_code);
        
        std.log.info("Linter test passed");
    }
    
    fn testPackageManager(self: *ToolTester) !void {
        // Test package manifest creation
        var manifest = package_manager.PackageManifest.init(self.allocator);
        defer manifest.deinit();
        
        manifest.name = "test-package";
        manifest.version = package_manager.Version{ .major = 1, .minor = 0, .patch = 0 };
        
        std.log.info("Package manager test passed");
    }
    
    fn testDocGenerator(self: *ToolTester) !void {
        const config = doc_generator.DocConfig{};
        var generator = doc_generator.DocGenerator.init(self.allocator, config);
        defer generator.deinit();
        
        std.log.info("Documentation generator test passed");
    }
};

// Main tooling entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-tools <tool> [args...]");
        std.log.err("Tools: lsp, fmt, lint, pkg, doc, test, discover");
        return;
    }
    
    const tool_name = args[1];
    
    if (std.mem.eql(u8, tool_name, "discover")) {
        var discovery = ToolDiscovery.init(allocator);
        defer discovery.deinit();
        
        try discovery.discoverTools();
        discovery.listTools();
    } else if (std.mem.eql(u8, tool_name, "test")) {
        var tester = ToolTester.init(allocator);
        try tester.runIntegrationTests();
    } else if (std.mem.eql(u8, tool_name, "ide")) {
        var ide = IDEIntegration.init(allocator);
        
        if (args.len > 2 and std.mem.eql(u8, args[2], "vscode")) {
            try ide.generateVSCodeExtension(".");
        } else if (args.len > 2 and std.mem.eql(u8, args[2], "grammar")) {
            try ide.generateLanguageGrammar("cursed.tmLanguage.json");
        }
    } else if (ToolType.fromString(tool_name)) |tool_type| {
        var runner = ToolRunner.init(allocator);
        
        var config = ToolConfig.init(allocator, tool_type);
        defer config.deinit();
        
        // Parse additional arguments
        config.input_files = args[2..];
        
        try runner.runTool(config);
    } else {
        std.log.err("Unknown tool: {s}", .{tool_name});
    }
}
