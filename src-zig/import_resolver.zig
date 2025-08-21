const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Module Import Resolution System for CURSED Zig Compiler
// Handles "yeet" import statements and stdlib module loading

pub const ImportResolver = struct {
    allocator: Allocator,
    stdlib_path: []const u8,
    search_paths: ArrayList([]const u8),
    loaded_modules: std.StringHashMap([]const u8),

    pub fn init(allocator: Allocator) !ImportResolver {
        const cwd = std.fs.cwd();
        var buf: [1024]u8 = undefined;
        const current_dir = try cwd.realpath(".", &buf);
        
        const stdlib_path = try std.fmt.allocPrint(allocator, "{s}/stdlib", .{current_dir});
        
        var search_paths = .empty;
        try search_paths.append(try allocator.dupe(u8, current_dir));
        
        return ImportResolver{
            .allocator = allocator,
            .stdlib_path = stdlib_path,
            .search_paths = search_paths,
            .loaded_modules = std.StringHashMap([]const u8).init(allocator),
        };
    }

    pub fn deinit(self: *ImportResolver) void {
        self.allocator.free(self.stdlib_path);
        for (self.search_paths.items) |path| {
            self.allocator.free(path);
        }
        self.search_paths.deinit();
        
        var iterator = self.loaded_modules.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.loaded_modules.deinit();
    }

    pub fn resolveImport(self: *ImportResolver, module_name: []const u8) ![]const u8 {
        // Check if module is already loaded
        if (self.loaded_modules.get(module_name)) |content| {
            return content;
        }

        // Try to resolve as stdlib module first
        if (try self.resolveStdlibModule(module_name)) |path| {
            const content = try self.loadModuleContent(path);
            try self.loaded_modules.put(try self.allocator.dupe(u8, module_name), content);
            return content;
        }

        // Try to resolve as local module
        if (try self.resolveLocalModule(module_name)) |path| {
            const content = try self.loadModuleContent(path);
            try self.loaded_modules.put(try self.allocator.dupe(u8, module_name), content);
            return content;
        }

        return error.ModuleNotFound;
    }

    fn resolveStdlibModule(self: *ImportResolver, module_name: []const u8) !?[]const u8 {
        // Remove stdlib/ prefix if present
        const clean_name = if (std.mem.startsWith(u8, module_name, "stdlib/"))
            module_name[7..]
        else
            module_name;

        // Enhanced module name mapping for common aliases
        const mapped_name = self.mapStdlibModuleName(clean_name);

        // Try different file patterns with both original and mapped names
        const patterns = [_][]const u8{
            "{s}/{s}/mod.csd",
            "{s}/{s}.csd",
            "{s}/{s}/lib.csd",
        };

        // Try with mapped name first
        for (patterns) |pattern| {
            const path = try std.fmt.allocPrint(self.allocator, pattern, .{ self.stdlib_path, mapped_name });
            defer self.allocator.free(path);

            if (self.fileExists(path)) {
                return try self.allocator.dupe(u8, path);
            }
        }

        // Try with original name if mapping failed
        if (!std.mem.eql(u8, clean_name, mapped_name)) {
            for (patterns) |pattern| {
                const path = try std.fmt.allocPrint(self.allocator, pattern, .{ self.stdlib_path, clean_name });
                defer self.allocator.free(path);

                if (self.fileExists(path)) {
                    return try self.allocator.dupe(u8, path);
                }
            }
        }

        return null;
    }

    fn mapStdlibModuleName(self: *ImportResolver, module_name: []const u8) []const u8 {
        _ = self;
        
        // Map legacy module names to current names
        if (std.mem.eql(u8, module_name, "mathz")) return "math";
        if (std.mem.eql(u8, module_name, "stringz")) return "string_simple";
        if (std.mem.eql(u8, module_name, "ioz")) return "io";
        if (std.mem.eql(u8, module_name, "timez")) return "time";
        if (std.mem.eql(u8, module_name, "dropz")) return "collections";
        
        // Return original name if no mapping exists
        return module_name;
    }

    fn resolveLocalModule(self: *ImportResolver, module_name: []const u8) !?[]const u8 {
        for (self.search_paths.items) |search_path| {
            const patterns = [_][]const u8{
                "{s}/{s}.csd",
                "{s}/{s}/mod.csd",
                "{s}/{s}/lib.csd",
            };

            for (patterns) |pattern| {
                const path = try std.fmt.allocPrint(self.allocator, pattern, .{ search_path, module_name });
                defer self.allocator.free(path);

                if (self.fileExists(path)) {
                    return try self.allocator.dupe(u8, path);
                }
            }
        }

        return null;
    }

    fn loadModuleContent(self: *ImportResolver, path: []const u8) ![]const u8 {
        const file = std.fs.cwd().openFile(path, .{}) catch |err| {
            print("Error: Cannot open module file '{s}': {any}\n", .{ path, err });
            return error.ModuleLoadFailed;
        };
        defer file.close();

        const file_size = try file.getEndPos();
        const content = try self.allocator.alloc(u8, file_size);
        _ = try file.readAll(content);
        
        return content;
    }

    fn fileExists(self: *ImportResolver, path: []const u8) bool {
        _ = self;
        std.fs.cwd().access(path, .{}) catch return false;
        return true;
    }
};

// Parse "yeet" import statements from source code
pub fn extractImports(allocator: Allocator, source: []const u8) !ArrayList([]const u8) {
    var imports = .empty;
    
    var lines = std.mem.splitScalar(u8, source, '\n');
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r");
        
        // Look for "yeet" statements
        if (std.mem.startsWith(u8, trimmed, "yeet ")) {
            const import_part = trimmed[5..]; // Skip "yeet "
            
            // Extract module name from quotes
            if (std.mem.indexOf(u8, import_part, "\"")) |start_quote| {
                const after_start = import_part[start_quote + 1..];
                if (std.mem.indexOf(u8, after_start, "\"")) |end_quote| {
                    const module_name = after_start[0..end_quote];
                    try imports.append(try allocator.dupe(u8, module_name));
                }
            }
        }
    }
    
    return imports;
}

// Test function to validate import resolution
pub fn testImportResolution(allocator: Allocator) !void {
    print("🧪 Testing Import Resolution System...\n", .{});
    
    var resolver = try ImportResolver.init(allocator);
    defer resolver.deinit();
    
    // Test stdlib module resolution
    const test_modules = [_][]const u8{ "testz", "math", "io", "string_simple" };
    
    for (test_modules) |module| {
        print("Testing module: {s}... ", .{module});
        
        if (resolver.resolveImport(module)) |content| {
            print("✅ Found ({} bytes)\n", .{content.len});
        } else |err| {
            print("❌ Error: {any}\n", .{err});
        }
    }
    
    print("✅ Import resolution test complete\n", .{});
}
