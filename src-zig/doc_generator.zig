// CURSED Documentation Generator
// Generates API documentation from source code and inline comments

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;

// Import CURSED compiler components using direct file imports
const lexer = @import("lexer.zig");
const parser = @import("parser.zig");
const ast = @import("ast.zig");

// Documentation Types
pub const DocType = enum {
    Function,
    Struct,
    Interface,
    Variable,
    Constant,
    Module,
    
    pub fn toString(self: DocType) []const u8 {
        return switch (self) {
            .Function => "function",
            .Struct => "struct",
            .Interface => "interface",
            .Variable => "variable",
            .Constant => "constant",
            .Module => "module",
        };
    }
};

// Documentation Comment
pub const DocComment = struct {
    brief: ?[]const u8 = null,
    description: ?[]const u8 = null,
    params: ArrayList(ParamDoc),
    returns: ?[]const u8 = null,
    examples: ArrayList([]const u8),
    see_also: ArrayList([]const u8),
    since: ?[]const u8 = null,
    deprecated: ?[]const u8 = null,
    
    const ParamDoc = struct {
        name: []const u8,
        type: []const u8,
        description: []const u8,
    };
    
    pub fn init(allocator: Allocator) DocComment {
        return DocComment{
            .params = .empty,
            .examples = .empty,
            .see_also = .empty,
        };
    }
    
    pub fn deinit(self: *DocComment) void {
        self.params.deinit(allocator);
        self.examples.deinit(allocator);
        self.see_also.deinit(allocator);
    }
};

// Documentation Item
pub const DocItem = struct {
    name: []const u8,
    type: DocType,
    signature: []const u8,
    file: []const u8,
    line: u32,
    visibility: Visibility,
    comment: DocComment,
    
    const Visibility = enum {
        Public,
        Private,
        Internal,
    };
};

// Module Documentation
pub const ModuleDoc = struct {
    name: []const u8,
    description: ?[]const u8,
    items: ArrayList(DocItem),
    submodules: ArrayList(ModuleDoc),
    
    pub fn init(allocator: Allocator, name: []const u8) ModuleDoc {
        return ModuleDoc{
            .name = name,
            .description = null,
            .items = .empty,
            .submodules = .empty,
        };
    }
    
    pub fn deinit(self: *ModuleDoc) void {
        self.items.deinit(allocator);
        for (self.submodules.items) |*submodule| {
            submodule.deinit(allocator);
        }
        self.submodules.deinit(allocator);
    }
};

// Documentation Configuration
pub const DocConfig = struct {
    output_format: OutputFormat = .HTML,
    output_dir: []const u8 = "docs",
    include_private: bool = false,
    include_source_links: bool = true,
    theme: []const u8 = "default",
    title: []const u8 = "CURSED Documentation",
    
    const OutputFormat = enum {
        HTML,
        Markdown,
        JSON,
    };
};

// Documentation Generator
pub const DocGenerator = struct {
    allocator: Allocator,
    config: DocConfig,
    modules: ArrayList(ModuleDoc),
    
    pub fn init(allocator: Allocator, config: DocConfig) DocGenerator {
        return DocGenerator{
            .allocator = allocator,
            .config = config,
            .modules = .empty,
        };
    }
    
    pub fn deinit(self: *DocGenerator) void {
        for (self.modules.items) |*module| {
            module.deinit(allocator);
        }
        self.modules.deinit(allocator);
    }
    
    pub fn generateFromDirectory(self: *DocGenerator, dir_path: []const u8) !void {
        var dir = try std.fs.cwd().openDir(dir_path, .{ .iterate = true });
        defer dir.close();
        
        var iterator = dir.iterate();
        while (try iterator.next()) |entry| {
            if (entry.kind == .file and std.mem.endsWith(u8, entry.name, ".csd")) {
                const file_path = try std.fs.path.join(self.allocator, &[_][]const u8{ dir_path, entry.name });
                defer self.allocator.free(file_path);
                
                try self.generateFromFile(file_path);
            } else if (entry.kind == .directory) {
                const sub_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ dir_path, entry.name });
                defer self.allocator.free(sub_dir);
                
                try self.generateFromDirectory(sub_dir);
            }
        }
    }
    
    pub fn generateFromFile(self: *DocGenerator, file_path: []const u8) !void {
        // Read file
        const file = try std.fs.cwd().openFile(file_path, .{});
        defer file.close();
        
        const source = try file.readToEndAlloc(self.allocator, 1024 * 1024);
        defer self.allocator.free(source);
        
        // Extract module name from file path
        const module_name = std.fs.path.stem(file_path);
        
        // Parse documentation
        var module = ModuleDoc.init(self.allocator, try self.allocator.dupe(u8, module_name));
        try self.extractDocumentation(file_path, source, &module);
        try self.modules.append(self.allocator, module);
    }
    
    pub fn writeDocumentation(self: *DocGenerator) !void {
        // Create output directory
        std.fs.cwd().makePath(self.config.output_dir) catch {};
        
        switch (self.config.output_format) {
            .HTML => try self.writeHTMLDocumentation(),
            .Markdown => try self.writeMarkdownDocumentation(),
            .JSON => try self.writeJSONDocumentation(),
        }
    }
    
    fn extractDocumentation(self: *DocGenerator, file_path: []const u8, source: []const u8, module: *ModuleDoc) !void {
        // Tokenize source
        var token_lexer = lexer.Lexer.init(self.allocator, source);
        
        const tokens = try token_lexer.tokenize();
        defer tokens.deinit(allocator);
        
        // Parse tokens
        var cursed_parser = parser.Parser.init(self.allocator, tokens.items);
        
        const ast_tree = cursed_parser.parseProgram() catch |err| {
            std.log.warn("Failed to parse {s}: {}", .{ file_path, err });
            return;
        };
        
        // Extract documentation from AST
        try self.extractFromAST(file_path, ast_tree, module);
    }
    
    fn extractFromAST(self: *DocGenerator, file_path: []const u8, ast_tree: ast.AST, module: *ModuleDoc) !void {
        // Walk AST and extract documentation
        // This would be a full AST traversal to find functions, structs, etc.
        
        // Example: Extract function documentation
        for (ast_tree.statements.items) |stmt_ptr| {
            const statement: *ast.Statement = @ptrCast(@alignCast(stmt_ptr));
            switch (statement.*) {
                .Function => |func| {
                    const doc_item = DocItem{
                        .name = try self.allocator.dupe(u8, func.name),
                        .type = .Function,
                        .signature = try self.buildFunctionSignature(func),
                        .file = try self.allocator.dupe(u8, file_path),
                        .line = 0, // TODO: Add source location tracking
                        .visibility = .Public,
                        .comment = DocComment.init(self.allocator),
                    };
                    
                    // Extract documentation comment
                    // TODO: Extract from func.comments if available
                    
                    try module.items.append(self.allocator, doc_item);
                },
                .Struct => |structure| {
                    const doc_item = DocItem{
                        .name = try self.allocator.dupe(u8, structure.name),
                        .type = .Struct,
                        .signature = try self.buildStructSignature(structure),
                        .file = try self.allocator.dupe(u8, file_path),
                        .line = 0, // TODO: Add source location tracking
                        .visibility = .Public,
                        .comment = DocComment.init(self.allocator),
                    };
                    
                    // TODO: Extract from structure.comments if available
                    try module.items.append(self.allocator, doc_item);
                },
                .Interface => |interface| {
                    const doc_item = DocItem{
                        .name = try self.allocator.dupe(u8, interface.name),
                        .type = .Interface,
                        .signature = try self.buildInterfaceSignature(interface),
                        .file = try self.allocator.dupe(u8, file_path),
                        .line = 0, // TODO: Add source location tracking
                        .visibility = .Public,
                        .comment = DocComment.init(self.allocator),
                    };
                    
                    // TODO: Extract from interface.comments if available
                    try module.items.append(self.allocator, doc_item);
                },
                else => {},
            }
        }
    }
    
    fn extractDocComment(self: *DocGenerator, doc_comment: *DocComment, comment_text: ?[]const u8) !void {
        if (comment_text == null) return;
        
        const text = comment_text.?;
        var lines = std.mem.splitScalar(u8, text, '\n');
        
        var current_section: ?[]const u8 = null;
        var description_lines = .empty;
        defer description_lines.deinit(allocator);
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t");
            
            // Skip comment markers
            const clean_line = if (std.mem.startsWith(u8, trimmed, "fr fr"))
                std.mem.trim(u8, trimmed[5..], " \t")
            else if (std.mem.startsWith(u8, trimmed, "#"))
                std.mem.trim(u8, trimmed[1..], " \t")
            else
                trimmed;
            
            // Parse documentation tags
            if (std.mem.startsWith(u8, clean_line, "@param")) {
                try self.parseParamTag(doc_comment, clean_line[6..]);
            } else if (std.mem.startsWith(u8, clean_line, "@returns")) {
                doc_comment.returns = try self.allocator.dupe(u8, std.mem.trim(u8, clean_line[8..], " \t"));
            } else if (std.mem.startsWith(u8, clean_line, "@example")) {
                current_section = "example";
            } else if (std.mem.startsWith(u8, clean_line, "@see")) {
                try doc_comment.see_also.append(self.allocator, try self.allocator.dupe(u8, std.mem.trim(u8, clean_line[4..], " \t")));
            } else if (std.mem.startsWith(u8, clean_line, "@since")) {
                doc_comment.since = try self.allocator.dupe(u8, std.mem.trim(u8, clean_line[6..], " \t"));
            } else if (std.mem.startsWith(u8, clean_line, "@deprecated")) {
                doc_comment.deprecated = try self.allocator.dupe(u8, std.mem.trim(u8, clean_line[11..], " \t"));
            } else {
                // Regular description text
                if (clean_line.len > 0) {
                    if (description_lines.items.len > 0) {
                        try description_lines.append(self.allocator, '\n');
                    }
                    try description_lines.appendSlice(clean_line);
                }
            }
        }
        
        if (description_lines.items.len > 0) {
            doc_comment.description = try description_lines.toOwnedSlice(allocator);
        }
    }
    
    fn parseParamTag(self: *DocGenerator, doc_comment: *DocComment, param_text: []const u8) !void {
        // Parse "@param name type description" format
        var parts = std.mem.splitScalar(u8, std.mem.trim(u8, param_text, " \t"), ' ');
        
        const name = parts.next() orelse return;
        const param_type = parts.next() orelse "";
        
        var description = .empty;
        defer description.deinit(allocator);
        
        while (parts.next()) |part| {
            if (description.items.len > 0) try description.append(self.allocator, ' ');
            try description.appendSlice(part);
        }
        
        try doc_comment.params.append(self.allocator, DocComment.ParamDoc{
            .name = try self.allocator.dupe(u8, name),
            .type = try self.allocator.dupe(u8, param_type),
            .description = try description.toOwnedSlice(self.allocator),
        });
    }
    
    fn buildFunctionSignature(self: *DocGenerator, func: ast.FunctionStatement) ![]const u8 {
        var signature = .empty;
        defer signature.deinit(allocator);
        
        try signature.appendSlice("slay ");
        try signature.appendSlice(func.name);
        try signature.append(allocator, '(');
        
        for (func.parameters.items, 0..) |param, i| {
            if (i > 0) try signature.appendSlice(", ");
            try signature.appendSlice(param.name);
            try signature.append(allocator, ' ');
            // Note: param_type is a Type union, need to convert to string
            try signature.appendSlice("Type"); // Placeholder for now
        }
        
        try signature.append(allocator, ')');
        
        if (func.return_type) |_| {
            try signature.append(allocator, ' ');
            try signature.appendSlice("ReturnType"); // Placeholder for now
        }
        
        return try signature.toOwnedSlice(allocator);
    }
    
    fn buildStructSignature(self: *DocGenerator, structure: ast.StructStatement) ![]const u8 {
        var signature = .empty;
        defer signature.deinit(allocator);
        
        try signature.appendSlice("squad ");
        try signature.appendSlice(structure.name);
        
        return try signature.toOwnedSlice(allocator);
    }
    
    fn buildInterfaceSignature(self: *DocGenerator, interface: ast.InterfaceStatement) ![]const u8 {
        var signature = .empty;
        defer signature.deinit(allocator);
        
        try signature.appendSlice("collab ");
        try signature.appendSlice(interface.name);
        
        return try signature.toOwnedSlice(allocator);
    }
    
    // HTML Documentation Generation
    fn writeHTMLDocumentation(self: *DocGenerator) !void {
        // Generate index.html
        try self.writeHTMLIndex();
        
        // Generate module documentation
        for (self.modules.items) |module| {
            try self.writeHTMLModule(module);
        }
        
        // Copy CSS and JavaScript assets
        try self.writeHTMLAssets();
    }
    
    fn writeHTMLIndex(self: *DocGenerator) !void {
        const index_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.config.output_dir, "index.html" });
        defer self.allocator.free(index_path);
        
        const file = try std.fs.cwd().createFile(index_path, .{});
        defer file.close();
        
        const writer = file.writer();
        
        try writer.writeAll(
            \\<!DOCTYPE html>
            \\<html lang="en">
            \\<head>
            \\    <meta charset="UTF-8">
            \\    <meta name="viewport" content="width=device-width, initial-scale=1.0">
            \\    <title>
        );
        try writer.writeAll(self.config.title);
        try writer.writeAll(
            \\</title>
            \\    <link rel="stylesheet" href="styles.css">
            \\</head>
            \\<body>
            \\    <header>
            \\        <h1>
        );
        try writer.writeAll(self.config.title);
        try writer.writeAll(
            \\</h1>
            \\        <nav>
            \\            <ul>
            \\
        );
        
        // Generate navigation links
        for (self.modules.items) |module| {
            try writer.print("                <li><a href=\"{s}.html\">{s}</a></li>\n", .{ module.name, module.name });
        }
        
        try writer.writeAll(
            \\            </ul>
            \\        </nav>
            \\    </header>
            \\    <main>
            \\        <section id="overview">
            \\            <h2>Overview</h2>
            \\            <p>Welcome to the CURSED language documentation.</p>
            \\        </section>
            \\        <section id="modules">
            \\            <h2>Modules</h2>
            \\            <div class="module-grid">
            \\
        );
        
        // Generate module cards
        for (self.modules.items) |module| {
            try writer.print(
                \\                <div class="module-card">
                \\                    <h3><a href="{s}.html">{s}</a></h3>
                \\                    <p>{s}</p>
                \\                    <div class="item-count">{} items</div>
                \\                </div>
                \\
            , .{ module.name, module.name, module.description orelse "No description", module.items.items.len });
        }
        
        try writer.writeAll(
            \\            </div>
            \\        </section>
            \\    </main>
            \\    <script src="script.js"></script>
            \\</body>
            \\</html>
        );
    }
    
    fn writeHTMLModule(self: *DocGenerator, module: ModuleDoc) !void {
        const module_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}.html", .{ self.config.output_dir, module.name });
        defer self.allocator.free(module_path);
        
        const file = try std.fs.cwd().createFile(module_path, .{});
        defer file.close();
        
        const writer = file.writer();
        
        // HTML header
        try writer.print(
            \\<!DOCTYPE html>
            \\<html lang="en">
            \\<head>
            \\    <meta charset="UTF-8">
            \\    <meta name="viewport" content="width=device-width, initial-scale=1.0">
            \\    <title>{s} - {s}</title>
            \\    <link rel="stylesheet" href="styles.css">
            \\</head>
            \\<body>
            \\    <header>
            \\        <h1><a href="index.html">{s}</a></h1>
            \\        <h2>{s}</h2>
            \\    </header>
            \\    <main>
            \\
        , .{ module.name, self.config.title, self.config.title, module.name });
        
        // Module description
        if (module.description) |desc| {
            try writer.print("        <section class=\"description\">\n            <p>{s}</p>\n        </section>\n", .{desc});
        }
        
        // Group items by type
        var functions = .empty;
        var structs = .empty;
        var interfaces = .empty;
        defer functions.deinit(allocator);
        defer structs.deinit(allocator);
        defer interfaces.deinit(allocator);
        
        for (module.items.items) |item| {
            switch (item.type) {
                .Function => try functions.append(allocator, item),
                .Struct => try structs.append(allocator, item),
                .Interface => try interfaces.append(allocator, item),
                else => {},
            }
        }
        
        // Write functions
        if (functions.items.len > 0) {
            try writer.writeAll("        <section id=\"functions\">\n            <h3>Functions</h3>\n");
            for (functions.items) |item| {
                try self.writeHTMLDocItem(writer, item);
            }
            try writer.writeAll("        </section>\n");
        }
        
        // Write structs
        if (structs.items.len > 0) {
            try writer.writeAll("        <section id=\"structs\">\n            <h3>Structs</h3>\n");
            for (structs.items) |item| {
                try self.writeHTMLDocItem(writer, item);
            }
            try writer.writeAll("        </section>\n");
        }
        
        // Write interfaces
        if (interfaces.items.len > 0) {
            try writer.writeAll("        <section id=\"interfaces\">\n            <h3>Interfaces</h3>\n");
            for (interfaces.items) |item| {
                try self.writeHTMLDocItem(writer, item);
            }
            try writer.writeAll("        </section>\n");
        }
        
        try writer.writeAll(
            \\    </main>
            \\    <script src="script.js"></script>
            \\</body>
            \\</html>
        );
    }
    
    fn writeHTMLDocItem(self: *DocGenerator, writer: anytype, item: DocItem) !void {
        _ = self;
        try writer.print(
            \\            <div class="doc-item" id="{s}">
            \\                <h4>{s}</h4>
            \\                <pre class="signature"><code>{s}</code></pre>
            \\
        , .{ item.name, item.name, item.signature });
        
        if (item.comment.description) |desc| {
            try writer.print("                <p class=\"description\">{s}</p>\n", .{desc});
        }
        
        // Parameters
        if (item.comment.params.items.len > 0) {
            try writer.writeAll("                <h5>Parameters</h5>\n                <dl class=\"parameters\">\n");
            for (item.comment.params.items) |param| {
                try writer.print("                    <dt><code>{s}</code> <span class=\"type\">{s}</span></dt>\n", .{ param.name, param.type });
                try writer.print("                    <dd>{s}</dd>\n", .{param.description});
            }
            try writer.writeAll("                </dl>\n");
        }
        
        // Returns
        if (item.comment.returns) |returns| {
            try writer.print("                <h5>Returns</h5>\n                <p>{s}</p>\n", .{returns});
        }
        
        // Examples
        if (item.comment.examples.items.len > 0) {
            try writer.writeAll("                <h5>Examples</h5>\n");
            for (item.comment.examples.items) |example| {
                try writer.print("                <pre class=\"example\"><code>{s}</code></pre>\n", .{example});
            }
        }
        
        try writer.writeAll("            </div>\n");
    }
    
    fn writeHTMLAssets(self: *DocGenerator) !void {
        // Write CSS
        const css_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.config.output_dir, "styles.css" });
        defer self.allocator.free(css_path);
        
        const css_file = try std.fs.cwd().createFile(css_path, .{});
        defer css_file.close();
        
        try css_file.writeAll(
            \\body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; line-height: 1.6; margin: 0; }
            \\header { background: #2d3748; color: white; padding: 1rem; }
            \\header h1 { margin: 0; }
            \\header h1 a { color: white; text-decoration: none; }
            \\nav ul { list-style: none; padding: 0; margin: 1rem 0 0 0; }
            \\nav li { display: inline; margin-right: 1rem; }
            \\nav a { color: #a0aec0; text-decoration: none; }
            \\nav a:hover { color: white; }
            \\main { max-width: 1200px; margin: 0 auto; padding: 2rem; }
            \\.module-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1rem; }
            \\.module-card { border: 1px solid #e2e8f0; border-radius: 0.5rem; padding: 1rem; }
            \\.module-card h3 { margin-top: 0; }
            \\.module-card a { text-decoration: none; color: #3182ce; }
            \\.doc-item { border-bottom: 1px solid #e2e8f0; padding: 1rem 0; }
            \\.signature { background: #f7fafc; padding: 1rem; border-radius: 0.25rem; overflow-x: auto; }
            \\.description { color: #4a5568; }
            \\.parameters dt { font-weight: bold; margin-top: 0.5rem; }
            \\.parameters dd { margin-left: 1rem; color: #4a5568; }
            \\.type { color: #805ad5; font-weight: normal; }
            \\.example { background: #f7fafc; padding: 1rem; border-radius: 0.25rem; border-left: 4px solid #3182ce; }
        );
        
        // Write JavaScript
        const js_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.config.output_dir, "script.js" });
        defer self.allocator.free(js_path);
        
        const js_file = try std.fs.cwd().createFile(js_path, .{});
        defer js_file.close();
        
        try js_file.writeAll(
            \\// Search functionality
            \\document.addEventListener('DOMContentLoaded', function() {
            \\    // Add search functionality if needed
            \\    console.log('CURSED Documentation loaded');
            \\});
        );
    }
    
    // Markdown Documentation Generation
    fn writeMarkdownDocumentation(self: *DocGenerator) !void {
        for (self.modules.items) |module| {
            const module_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}.md", .{ self.config.output_dir, module.name });
            defer self.allocator.free(module_path);
            
            const file = try std.fs.cwd().createFile(module_path, .{});
            defer file.close();
            
            const writer = file.writer();
            
            try writer.print("# {s}\n\n", .{module.name});
            
            if (module.description) |desc| {
                try writer.print("{s}\n\n", .{desc});
            }
            
            for (module.items.items) |item| {
                try writer.print("## {s}\n\n", .{item.name});
                try writer.print("```cursed\n{s}\n```\n\n", .{item.signature});
                
                if (item.comment.description) |desc| {
                    try writer.print("{s}\n\n", .{desc});
                }
            }
        }
    }
    
    // JSON Documentation Generation
    fn writeJSONDocumentation(self: *DocGenerator) !void {
        const json_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.config.output_dir, "api.json" });
        defer self.allocator.free(json_path);
        
        const file = try std.fs.cwd().createFile(json_path, .{});
        defer file.close();
        
        // Generate JSON representation of documentation
        // This would be a full JSON serialization of the documentation structure
        try file.writeAll("{\n  \"modules\": []\n}\n");
    }
};

// Main documentation generator entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-doc <source-directory> [--output <output-dir>] [--format html|markdown|json]", .{});
        return;
    }
    
    const source_dir = args[1];
    
    var config = DocConfig{};
    
    // Parse command line options
    var i: usize = 2;
    while (i < args.len) {
        if (std.mem.eql(u8, args[i], "--output") and i + 1 < args.len) {
            config.output_dir = args[i + 1];
            i += 2;
        } else if (std.mem.eql(u8, args[i], "--format") and i + 1 < args.len) {
            const format = args[i + 1];
            if (std.mem.eql(u8, format, "html")) {
                config.output_format = .HTML;
            } else if (std.mem.eql(u8, format, "markdown")) {
                config.output_format = .Markdown;
            } else if (std.mem.eql(u8, format, "json")) {
                config.output_format = .JSON;
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    
    var generator = DocGenerator.init(allocator, config);
    defer generator.deinit(allocator);
    
    try generator.generateFromDirectory(source_dir);
    try generator.writeDocumentation();
    
    std.log.info("Documentation generated in {s}", .{config.output_dir});
}
