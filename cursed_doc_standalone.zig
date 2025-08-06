// CURSED Documentation Generator - Standalone Version
// Generates API documentation from CURSED source code

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Documentation Types
pub const DocItem = struct {
    name: []const u8,
    signature: []const u8,
    description: ?[]const u8,
    file: []const u8,
    line: u32,
    item_type: []const u8,
    
    pub fn init(allocator: Allocator, name: []const u8, signature: []const u8, file: []const u8, line: u32, item_type: []const u8) !DocItem {
        return DocItem{
            .name = try allocator.dupe(u8, name),
            .signature = try allocator.dupe(u8, signature),
            .description = null,
            .file = try allocator.dupe(u8, file),
            .line = line,
            .item_type = try allocator.dupe(u8, item_type),
        };
    }
};

pub const ModuleDoc = struct {
    name: []const u8,
    description: ?[]const u8,
    items: ArrayList(DocItem),
    
    pub fn init(allocator: Allocator, name: []const u8) ModuleDoc {
        return ModuleDoc{
            .name = name,
            .description = null,
            .items = ArrayList(DocItem).init(allocator),
        };
    }
    
    pub fn deinit(self: *ModuleDoc) void {
        self.items.deinit();
    }
};

pub const DocGenerator = struct {
    allocator: Allocator,
    output_dir: []const u8,
    modules: ArrayList(ModuleDoc),
    
    pub fn init(allocator: Allocator, output_dir: []const u8) DocGenerator {
        return DocGenerator{
            .allocator = allocator,
            .output_dir = output_dir,
            .modules = ArrayList(ModuleDoc).init(allocator),
        };
    }
    
    pub fn deinit(self: *DocGenerator) void {
        for (self.modules.items) |*module| {
            module.deinit();
        }
        self.modules.deinit();
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
        const file = try std.fs.cwd().openFile(file_path, .{});
        defer file.close();
        
        const source = try file.readToEndAlloc(self.allocator, 1024 * 1024);
        defer self.allocator.free(source);
        
        const module_name = std.fs.path.stem(file_path);
        var module = ModuleDoc.init(self.allocator, try self.allocator.dupe(u8, module_name));
        
        try self.extractDocumentation(file_path, source, &module);
        try self.modules.append(module);
    }
    
    pub fn extractDocumentation(self: *DocGenerator, file_path: []const u8, source: []const u8, module: *ModuleDoc) !void {
        var lines = std.mem.split(u8, source, "\n");
        var line_num: u32 = 0;
        var doc_comment: ?[]const u8 = null;
        
        while (lines.next()) |line| {
            line_num += 1;
            const trimmed = std.mem.trim(u8, line, " \t");
            
            // Extract documentation comments
            if (std.mem.startsWith(u8, trimmed, "fr fr/")) {
                doc_comment = std.mem.trim(u8, trimmed[6..], " \t");
                continue;
            }
            
            // Extract function definitions
            if (std.mem.startsWith(u8, trimmed, "slay ")) {
                const func_name = try self.extractFunctionName(trimmed);
                if (func_name) |name| {
                    var item = try DocItem.init(self.allocator, name, trimmed, file_path, line_num, "function");
                    if (doc_comment) |desc| {
                        item.description = try self.allocator.dupe(u8, desc);
                    }
                    try module.items.append(item);
                    doc_comment = null;
                }
            }
            
            // Extract struct definitions
            if (std.mem.startsWith(u8, trimmed, "squad ")) {
                const struct_name = try self.extractStructName(trimmed);
                if (struct_name) |name| {
                    var item = try DocItem.init(self.allocator, name, trimmed, file_path, line_num, "struct");
                    if (doc_comment) |desc| {
                        item.description = try self.allocator.dupe(u8, desc);
                    }
                    try module.items.append(item);
                    doc_comment = null;
                }
            }
            
            // Extract interface definitions
            if (std.mem.startsWith(u8, trimmed, "collab ")) {
                const interface_name = try self.extractInterfaceName(trimmed);
                if (interface_name) |name| {
                    var item = try DocItem.init(self.allocator, name, trimmed, file_path, line_num, "interface");
                    if (doc_comment) |desc| {
                        item.description = try self.allocator.dupe(u8, desc);
                    }
                    try module.items.append(item);
                    doc_comment = null;
                }
            }
            
            // Extract constants
            if (std.mem.startsWith(u8, trimmed, "facts ")) {
                const const_name = try self.extractConstantName(trimmed);
                if (const_name) |name| {
                    var item = try DocItem.init(self.allocator, name, trimmed, file_path, line_num, "constant");
                    if (doc_comment) |desc| {
                        item.description = try self.allocator.dupe(u8, desc);
                    }
                    try module.items.append(item);
                    doc_comment = null;
                }
            }
        }
    }
    
    fn extractFunctionName(self: *DocGenerator, line: []const u8) !?[]const u8 {
        _ = self;
        // Extract function name from "slay function_name(...)"
        const start = std.mem.indexOf(u8, line, "slay ") orelse return null;
        const name_start = start + 5;
        const paren_idx = std.mem.indexOf(u8, line[name_start..], "(") orelse return null;
        const name_end = name_start + paren_idx;
        
        return std.mem.trim(u8, line[name_start..name_end], " \t");
    }
    
    fn extractStructName(self: *DocGenerator, line: []const u8) !?[]const u8 {
        _ = self;
        // Extract struct name from "squad StructName {"
        const start = std.mem.indexOf(u8, line, "squad ") orelse return null;
        const name_start = start + 6;
        const brace_idx = std.mem.indexOf(u8, line[name_start..], " {") orelse
                         std.mem.indexOf(u8, line[name_start..], "{") orelse return null;
        const name_end = name_start + brace_idx;
        
        return std.mem.trim(u8, line[name_start..name_end], " \t");
    }
    
    fn extractInterfaceName(self: *DocGenerator, line: []const u8) !?[]const u8 {
        _ = self;
        // Extract interface name from "collab InterfaceName {"
        const start = std.mem.indexOf(u8, line, "collab ") orelse return null;
        const name_start = start + 7;
        const brace_idx = std.mem.indexOf(u8, line[name_start..], " {") orelse
                         std.mem.indexOf(u8, line[name_start..], "{") orelse return null;
        const name_end = name_start + brace_idx;
        
        return std.mem.trim(u8, line[name_start..name_end], " \t");
    }
    
    fn extractConstantName(self: *DocGenerator, line: []const u8) !?[]const u8 {
        _ = self;
        // Extract constant name from "facts CONST_NAME = value"
        const start = std.mem.indexOf(u8, line, "facts ") orelse return null;
        const name_start = start + 6;
        const eq_idx = std.mem.indexOf(u8, line[name_start..], "=") orelse
                      std.mem.indexOf(u8, line[name_start..], ":") orelse return null;
        const name_end = name_start + eq_idx;
        
        return std.mem.trim(u8, line[name_start..name_end], " \t");
    }
    
    pub fn writeHTMLDocumentation(self: *DocGenerator) !void {
        // Create output directory
        std.fs.cwd().makePath(self.output_dir) catch {};
        
        // Generate index.html
        try self.writeHTMLIndex();
        
        // Generate module documentation
        for (self.modules.items) |module| {
            try self.writeHTMLModule(module);
        }
        
        // Write CSS
        try self.writeHTMLAssets();
    }
    
    fn writeHTMLIndex(self: *DocGenerator) !void {
        const index_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.output_dir, "index.html" });
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
            \\    <title>CURSED Documentation</title>
            \\    <link rel="stylesheet" href="styles.css">
            \\</head>
            \\<body>
            \\    <header>
            \\        <h1>CURSED Language Documentation</h1>
            \\        <p>Generated documentation for the CURSED programming language stdlib</p>
            \\    </header>
            \\    <main>
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
            \\</body>
            \\</html>
        );
    }
    
    fn writeHTMLModule(self: *DocGenerator, module: ModuleDoc) !void {
        const module_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}.html", .{ self.output_dir, module.name });
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
            \\    <title>{s} - CURSED Documentation</title>
            \\    <link rel="stylesheet" href="styles.css">
            \\</head>
            \\<body>
            \\    <header>
            \\        <h1><a href="index.html">CURSED Documentation</a></h1>
            \\        <h2>{s}</h2>
            \\    </header>
            \\    <main>
            \\
        , .{ module.name, module.name });
        
        // Module description
        if (module.description) |desc| {
            try writer.print("        <section class=\"description\">\n            <p>{s}</p>\n        </section>\n", .{desc});
        }
        
        // Group items by type
        var functions = ArrayList(DocItem).init(self.allocator);
        var structs = ArrayList(DocItem).init(self.allocator);
        var interfaces = ArrayList(DocItem).init(self.allocator);
        var constants = ArrayList(DocItem).init(self.allocator);
        defer functions.deinit();
        defer structs.deinit();
        defer interfaces.deinit();
        defer constants.deinit();
        
        for (module.items.items) |item| {
            if (std.mem.eql(u8, item.item_type, "function")) {
                try functions.append(item);
            } else if (std.mem.eql(u8, item.item_type, "struct")) {
                try structs.append(item);
            } else if (std.mem.eql(u8, item.item_type, "interface")) {
                try interfaces.append(item);
            } else if (std.mem.eql(u8, item.item_type, "constant")) {
                try constants.append(item);
            }
        }
        
        // Write sections
        if (functions.items.len > 0) {
            try writer.writeAll("        <section id=\"functions\">\n            <h3>Functions</h3>\n");
            for (functions.items) |item| {
                try self.writeHTMLDocItem(writer, item);
            }
            try writer.writeAll("        </section>\n");
        }
        
        if (structs.items.len > 0) {
            try writer.writeAll("        <section id=\"structs\">\n            <h3>Structs</h3>\n");
            for (structs.items) |item| {
                try self.writeHTMLDocItem(writer, item);
            }
            try writer.writeAll("        </section>\n");
        }
        
        if (interfaces.items.len > 0) {
            try writer.writeAll("        <section id=\"interfaces\">\n            <h3>Interfaces</h3>\n");
            for (interfaces.items) |item| {
                try self.writeHTMLDocItem(writer, item);
            }
            try writer.writeAll("        </section>\n");
        }
        
        if (constants.items.len > 0) {
            try writer.writeAll("        <section id=\"constants\">\n            <h3>Constants</h3>\n");
            for (constants.items) |item| {
                try self.writeHTMLDocItem(writer, item);
            }
            try writer.writeAll("        </section>\n");
        }
        
        try writer.writeAll(
            \\    </main>
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
        
        if (item.description) |desc| {
            try writer.print("                <p class=\"description\">{s}</p>\n", .{desc});
        }
        
        try writer.print("                <p class=\"source\">Source: <a href=\"file://{s}#L{}\">{s}:{}</a></p>\n", .{ item.file, item.line, std.fs.path.basename(item.file), item.line });
        
        try writer.writeAll("            </div>\n");
    }
    
    fn writeHTMLAssets(self: *DocGenerator) !void {
        const css_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.output_dir, "styles.css" });
        defer self.allocator.free(css_path);
        
        const css_file = try std.fs.cwd().createFile(css_path, .{});
        defer css_file.close();
        
        try css_file.writeAll(
            \\body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; line-height: 1.6; margin: 0; background: #f8f9fa; }
            \\header { background: #2d3748; color: white; padding: 2rem; text-align: center; }
            \\header h1 { margin: 0; font-size: 2.5em; }
            \\header h1 a { color: white; text-decoration: none; }
            \\header p { margin: 0.5rem 0 0 0; opacity: 0.8; }
            \\main { max-width: 1200px; margin: 0 auto; padding: 2rem; background: white; min-height: 80vh; }
            \\.module-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem; margin-top: 2rem; }
            \\.module-card { border: 1px solid #e2e8f0; border-radius: 0.5rem; padding: 1.5rem; background: white; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
            \\.module-card h3 { margin-top: 0; color: #2d3748; }
            \\.module-card a { text-decoration: none; color: #3182ce; font-weight: 600; }
            \\.module-card a:hover { color: #2c5aa0; }
            \\.item-count { font-size: 0.9em; color: #718096; margin-top: 1rem; }
            \\.doc-item { border-bottom: 1px solid #e2e8f0; padding: 2rem 0; }
            \\.doc-item:last-child { border-bottom: none; }
            \\.doc-item h4 { margin-top: 0; color: #2d3748; font-size: 1.4em; }
            \\.signature { background: #f7fafc; padding: 1rem; border-radius: 0.25rem; overflow-x: auto; border-left: 4px solid #3182ce; margin: 1rem 0; }
            \\.signature code { font-family: 'SFMono-Regular', Consolas, monospace; color: #2d3748; }
            \\.description { color: #4a5568; font-size: 1.1em; margin: 1rem 0; }
            \\.source { font-size: 0.9em; color: #718096; margin-top: 1rem; }
            \\.source a { color: #3182ce; text-decoration: none; }
            \\.source a:hover { text-decoration: underline; }
            \\section { margin: 2rem 0; }
            \\section h3 { color: #2d3748; border-bottom: 2px solid #e2e8f0; padding-bottom: 0.5rem; }
        );
    }
};

// Main entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-doc <source-directory> [--output <output-dir>]", .{});
        return;
    }
    
    const source_dir = args[1];
    var output_dir: []const u8 = "docs";
    
    // Parse command line options
    var i: usize = 2;
    while (i < args.len) {
        if (std.mem.eql(u8, args[i], "--output") and i + 1 < args.len) {
            output_dir = args[i + 1];
            i += 2;
        } else {
            i += 1;
        }
    }
    
    var generator = DocGenerator.init(allocator, output_dir);
    defer generator.deinit();
    
    std.log.info("Generating documentation from {s} to {s}...", .{ source_dir, output_dir });
    
    try generator.generateFromDirectory(source_dir);
    try generator.writeHTMLDocumentation();
    
    std.log.info("Documentation generated successfully in {s}", .{output_dir});
    std.log.info("Found {} modules with {} total items", .{ generator.modules.items.len, blk: {
        var total: usize = 0;
        for (generator.modules.items) |module| {
            total += module.items.items.len;
        }
        break :blk total;
    }});
}
