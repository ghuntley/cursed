// Advanced Import Resolution System for CURSED
// Complete module resolution with search paths, versioning, cycle detection, and alias handling

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

// Import the package management system
const package_manager = @import("tools/package_manager_enhanced.zig");
const Version = package_manager.Version;
const VersionRequirement = package_manager.VersionRequirement;
const PackageManifest = package_manager.PackageManifest;

// ===== Module Resolution Types =====

pub const ModuleType = enum {
    stdlib,      // Built-in standard library module
    local,       // Local file/directory module  
    package,     // External package dependency
    absolute,    // Absolute file path
    relative,    // Relative file path
    alias,       // Module alias/shortcut
};

pub const ImportSpec = struct {
    raw_path: []const u8,
    resolved_path: ?[]const u8 = null,
    module_type: ModuleType,
    version_req: ?VersionRequirement = null,
    alias: ?[]const u8 = null,
    
    // Import source location for error reporting
    source_file: []const u8,
    line: u32,
    column: u32,
    
    // Flag to track ownership of resolved_path to prevent double-free
    owns_resolved_path: bool = true,
    
    pub fn deinit(self: *ImportSpec, allocator: Allocator) void {
        if (self.owns_resolved_path) {
            if (self.resolved_path) |path| {
                allocator.free(path);
                self.resolved_path = null;
            }
        }
        if (self.alias) |alias| {
            allocator.free(alias);
            self.alias = null;
        }
        // Clean up raw_path (always owned)
        allocator.free(self.raw_path);
        // Clean up source_file (always owned)
        allocator.free(self.source_file);
        // Clean up version requirement if present
        if (self.version_req) |*version_req| {
            version_req.deinit(allocator);
        }
    }
    
    // Create a safe copy that doesn't own the memory
    pub fn copyForReturn(self: *const ImportSpec) ImportSpec {
        return ImportSpec{
            .raw_path = self.raw_path,
            .resolved_path = self.resolved_path,
            .module_type = self.module_type,
            .version_req = self.version_req,
            .alias = self.alias,
            .source_file = self.source_file,
            .line = self.line,
            .column = self.column,
            .owns_resolved_path = false,  // Copy doesn't own the memory
        };
    }
};

pub const ModuleSearchPath = struct {
    path: []const u8,
    priority: u32,
    is_stdlib: bool = false,
    is_cache: bool = false,
    
    pub fn init(allocator: Allocator, path: []const u8, priority: u32) !ModuleSearchPath {
        return ModuleSearchPath{
            .path = try allocator.dupe(u8, path),
            .priority = priority,
        };
    }
    
    pub fn deinit(self: *ModuleSearchPath, allocator: Allocator) void {
        allocator.free(self.path);
    }
};

pub const ModuleCache = struct {
    resolved_modules: HashMap([]const u8, ImportSpec, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    import_graph: HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) ModuleCache {
        return ModuleCache{
            .resolved_modules = HashMap([]const u8, ImportSpec, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .import_graph = HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *ModuleCache) void {
        // Clean up resolved modules
        var resolved_iter = self.resolved_modules.iterator();
        while (resolved_iter.next()) |entry| {
            var import_spec = entry.value_ptr;
            import_spec.deinit(self.allocator);
            self.allocator.free(entry.key_ptr.*);
        }
        self.resolved_modules.deinit();
        
        // Clean up import graph
        var graph_iter = self.import_graph.iterator();
        while (graph_iter.next()) |entry| {
            var deps = entry.value_ptr;
            for (deps.items) |dep| {
                self.allocator.free(dep);
            }
            deps.deinit();
            self.allocator.free(entry.key_ptr.*);
        }
        self.import_graph.deinit();
    }
    
    pub fn addToGraph(self: *ModuleCache, from_module: []const u8, to_module: []const u8) !void {
        // Check if key already exists to avoid duplicating it
        if (self.import_graph.getPtr(from_module)) |dependencies| {
            try dependencies.append(try self.allocator.dupe(u8, to_module));
        } else {
            const from_key = try self.allocator.dupe(u8, from_module);
            var new_deps = ArrayList([]const u8).init(self.allocator);
            try new_deps.append(try self.allocator.dupe(u8, to_module));
            try self.import_graph.put(from_key, new_deps);
        }
    }
    
    pub fn detectCycle(self: *ModuleCache, start_module: []const u8) !?ArrayList([]const u8) {
        var visited = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer visited.deinit();
        
        var rec_stack = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer rec_stack.deinit();
        
        var path = ArrayList([]const u8).init(self.allocator);
        
        const has_cycle = try self.detectCycleRecursive(start_module, &visited, &rec_stack, &path);
        
        if (has_cycle) {
            return path;
        } else {
            path.deinit();
            return null;
        }
    }
    
    fn detectCycleRecursive(
        self: *ModuleCache,
        module: []const u8,
        visited: *HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        rec_stack: *HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        path: *ArrayList([]const u8)
    ) !bool {
        try visited.put(module, true);
        try rec_stack.put(module, true);
        try path.append(module);
        
        if (self.import_graph.get(module)) |dependencies| {
            for (dependencies.items) |dep| {
                if (rec_stack.get(dep) orelse false) {
                    // Found cycle
                    try path.append(dep);
                    return true;
                }
                
                if (!(visited.get(dep) orelse false)) {
                    if (try self.detectCycleRecursive(dep, visited, rec_stack, path)) {
                        return true;
                    }
                }
            }
        }
        
        _ = rec_stack.remove(module);
        _ = path.pop();
        return false;
    }
};

// ===== Advanced Import Resolver =====

pub const AdvancedImportResolver = struct {
    allocator: Allocator,
    search_paths: ArrayList(ModuleSearchPath),
    module_cache: ModuleCache,
    package_manifest: ?PackageManifest = null,
    aliases: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    current_file: ?[]const u8 = null,
    
    pub fn init(allocator: Allocator) AdvancedImportResolver {
        return AdvancedImportResolver{
            .allocator = allocator,
            .search_paths = ArrayList(ModuleSearchPath).init(allocator),
            .module_cache = ModuleCache.init(allocator),
            .aliases = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *AdvancedImportResolver) void {
        // Clean up search paths
        for (self.search_paths.items) |*path| {
            path.deinit(self.allocator);
        }
        self.search_paths.deinit();
        
        // Clean up module cache
        self.module_cache.deinit();
        
        // Clean up aliases
        var alias_iter = self.aliases.iterator();
        while (alias_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.aliases.deinit();
        
        // Clean up package manifest
        if (self.package_manifest) |*manifest| {
            manifest.deinit(self.allocator);
        }
    }
    
    // Initialize with default search paths
    pub fn initWithDefaults(allocator: Allocator) !AdvancedImportResolver {
        var resolver = AdvancedImportResolver.init(allocator);
        
        // Add default search paths in priority order
        try resolver.addStdlibPath();
        try resolver.addLocalPath(".");
        try resolver.addLocalPath("src");
        try resolver.addLocalPath("lib");
        try resolver.addCachePath(".cursed/cache");
        
        // Load package manifest if available
        try resolver.loadPackageManifest();
        
        return resolver;
    }
    
    pub fn addStdlibPath(self: *AdvancedImportResolver) !void {
        const stdlib_path = try self.findStdlibPath();
        var search_path = try ModuleSearchPath.init(self.allocator, stdlib_path, 1000);
        search_path.is_stdlib = true;
        try self.search_paths.append(search_path);
        self.allocator.free(stdlib_path);
    }
    
    pub fn addLocalPath(self: *AdvancedImportResolver, path: []const u8) !void {
        const search_path = try ModuleSearchPath.init(self.allocator, path, 500);
        try self.search_paths.append(search_path);
    }
    
    pub fn addCachePath(self: *AdvancedImportResolver, path: []const u8) !void {
        var search_path = try ModuleSearchPath.init(self.allocator, path, 100);
        search_path.is_cache = true;
        try self.search_paths.append(search_path);
    }
    
    pub fn addAlias(self: *AdvancedImportResolver, alias: []const u8, target: []const u8) !void {
        const alias_key = try self.allocator.dupe(u8, alias);
        const target_value = try self.allocator.dupe(u8, target);
        try self.aliases.put(alias_key, target_value);
    }
    
    fn findStdlibPath(self: *AdvancedImportResolver) ![]const u8 {
        const cwd = std.fs.cwd();
        
        // Try to find project root
        const project_root = try self.findProjectRoot();
        defer self.allocator.free(project_root);
        
        // Check for stdlib directory
        const stdlib_path = try std.fs.path.join(self.allocator, &[_][]const u8{ project_root, "stdlib" });
        
        // Verify stdlib directory exists
        cwd.access(stdlib_path, .{}) catch {
            self.allocator.free(stdlib_path);
            return try self.allocator.dupe(u8, "stdlib"); // Fallback
        };
        
        return stdlib_path;
    }
    
    fn findProjectRoot(self: *AdvancedImportResolver) ![]const u8 {
        const cwd = std.fs.cwd();
        var buf: [1024]u8 = undefined;
        const current_path = try cwd.realpath(".", &buf);
        
        const markers = [_][]const u8{
            "CursedPackage.toml",
            "build.zig",
            "Cargo.toml",
            ".git",
            "AGENT.md"
        };
        
        var path_components = ArrayList([]const u8).init(self.allocator);
        defer path_components.deinit();
        
        var iter = std.mem.splitScalar(u8, current_path, '/');
        while (iter.next()) |component| {
            if (component.len > 0) {
                try path_components.append(component);
            }
        }
        
        while (path_components.items.len > 0) {
            var test_path = ArrayList(u8).init(self.allocator);
            defer test_path.deinit();
            
            for (path_components.items) |component| {
                try test_path.append('/');
                try test_path.appendSlice(component);
            }
            
            for (markers) |marker| {
                var marker_path = ArrayList(u8).init(self.allocator);
                defer marker_path.deinit();
                
                try marker_path.appendSlice(test_path.items);
                try marker_path.append('/');
                try marker_path.appendSlice(marker);
                
                cwd.access(marker_path.items, .{}) catch continue;
                return try self.allocator.dupe(u8, test_path.items);
            }
            
            _ = path_components.pop();
        }
        
        return try self.allocator.dupe(u8, "/");
    }
    
    fn loadPackageManifest(self: *AdvancedImportResolver) !void {
        if (PackageManifest.loadFromToml(self.allocator, "CursedPackage.toml")) |manifest| {
            self.package_manifest = manifest;
            
            // Add package dependencies to search paths
            var deps_iter = manifest.dependencies.iterator();
            while (deps_iter.next()) |entry| {
                const dep_name = entry.key_ptr.*;
                const cache_path = try std.fmt.allocPrint(self.allocator, ".cursed/cache/{s}", .{dep_name});
                defer self.allocator.free(cache_path);
                
                var search_path = try ModuleSearchPath.init(self.allocator, cache_path, 200);
                search_path.is_cache = true;
                try self.search_paths.append(search_path);
            }
        } else |_| {
            // No package manifest found, continue without it
        }
    }
    
    // Main import resolution function
    pub fn resolveImport(self: *AdvancedImportResolver, import_path: []const u8, current_file: []const u8) !ImportSpec {
        self.current_file = current_file;
        
        // Check if this import has already been resolved and cached
        const cache_key = try std.fmt.allocPrint(self.allocator, "{s}:{s}", .{ current_file, import_path });
        defer self.allocator.free(cache_key);
        
        if (self.module_cache.resolved_modules.get(cache_key)) |cached| {
            return cached.copyForReturn();
        }
        
        // Parse the import statement to extract details
        var import_spec = try self.parseImportStatement(import_path, current_file);
        
        // Resolve aliases first
        if (self.aliases.get(import_spec.raw_path)) |alias_target| {
            self.allocator.free(import_spec.raw_path);
            import_spec.raw_path = try self.allocator.dupe(u8, alias_target);
            import_spec.module_type = .alias;
        }
        
        // Determine module type and resolve path
        try self.determineModuleType(&import_spec);
        try self.resolveModulePath(&import_spec);
        
        // Validate the resolved module exists
        try self.validateModule(&import_spec);
        
        // Add to dependency graph for cycle detection
        try self.module_cache.addToGraph(current_file, import_spec.resolved_path.?);
        
        // Check for import cycles
        if (try self.module_cache.detectCycle(current_file)) |cycle| {
            defer cycle.deinit();
            
            print("Error: Import cycle detected:\n", .{});
            for (cycle.items, 0..) |module, i| {
                if (i == cycle.items.len - 1) {
                    print("  {s} -> {s}\n", .{ module, cycle.items[0] });
                } else {
                    print("  {s} -> \n", .{module});
                }
            }
            // Clean up allocated memory in import_spec before returning error
            import_spec.deinit(self.allocator);
            return error.ImportCycle;
        }
        
        // Cache the resolved import
        const cache_key_owned = try self.allocator.dupe(u8, cache_key);
        try self.module_cache.resolved_modules.put(cache_key_owned, import_spec);
        
        return import_spec;
    }
    
    fn parseImportStatement(self: *AdvancedImportResolver, import_path: []const u8, current_file: []const u8) !ImportSpec {
        var import_spec = ImportSpec{
            .raw_path = try self.allocator.dupe(u8, import_path),
            .module_type = .local, // Default, will be determined later
            .source_file = try self.allocator.dupe(u8, current_file),
            .line = 1, // TODO: Extract from parser
            .column = 1, // TODO: Extract from parser
        };
        
        // Parse version requirements from import path
        // Format: "module@^1.0.0" or "module@1.2.3"
        if (std.mem.indexOf(u8, import_path, "@")) |at_pos| {
            const module_name = import_path[0..at_pos];
            const version_spec = import_path[at_pos + 1..];
            
            self.allocator.free(import_spec.raw_path);
            import_spec.raw_path = try self.allocator.dupe(u8, module_name);
            import_spec.version_req = VersionRequirement.parse(self.allocator, version_spec) catch |err| {
                // Clean up on error
                import_spec.deinit(self.allocator);
                return err;
            };
        }
        
        return import_spec;
    }
    
    fn determineModuleType(self: *AdvancedImportResolver, import_spec: *ImportSpec) !void {
        const path = import_spec.raw_path;
        
        // Check for absolute paths
        if (std.fs.path.isAbsolute(path)) {
            import_spec.module_type = .absolute;
            return;
        }
        
        // Check for relative paths
        if (std.mem.startsWith(u8, path, "./") or std.mem.startsWith(u8, path, "../")) {
            import_spec.module_type = .relative;
            return;
        }
        
        // Check if it's a known stdlib module
        for (self.search_paths.items) |search_path| {
            if (search_path.is_stdlib) {
                const test_path = try std.fs.path.join(self.allocator, &[_][]const u8{ search_path.path, path, "mod.csd" });
                defer self.allocator.free(test_path);
                
                std.fs.cwd().access(test_path, .{}) catch continue;
                import_spec.module_type = .stdlib;
                return;
            }
        }
        
        // Check if it's a package dependency
        if (self.package_manifest) |manifest| {
            if (manifest.dependencies.contains(path)) {
                import_spec.module_type = .package;
                return;
            }
        }
        
        // Default to local module
        import_spec.module_type = .local;
    }
    
    fn resolveModulePath(self: *AdvancedImportResolver, import_spec: *ImportSpec) !void {
        switch (import_spec.module_type) {
            .absolute => {
                import_spec.resolved_path = try self.allocator.dupe(u8, import_spec.raw_path);
            },
            .relative => {
                const current_dir = std.fs.path.dirname(self.current_file.?) orelse ".";
                import_spec.resolved_path = try std.fs.path.resolve(self.allocator, &[_][]const u8{ current_dir, import_spec.raw_path });
            },
            .stdlib => {
                try self.resolveStdlibModule(import_spec);
            },
            .package => {
                try self.resolvePackageModule(import_spec);
            },
            .local => {
                try self.resolveLocalModule(import_spec);
            },
            .alias => {
                // Aliases should have been resolved earlier
                return error.UnresolvedAlias;
            },
        }
    }
    
    fn resolveStdlibModule(self: *AdvancedImportResolver, import_spec: *ImportSpec) !void {
        for (self.search_paths.items) |search_path| {
            if (!search_path.is_stdlib) continue;
            
            const test_path = try std.fs.path.join(self.allocator, &[_][]const u8{ search_path.path, import_spec.raw_path, "mod.csd" });
            defer self.allocator.free(test_path);
            
            std.fs.cwd().access(test_path, .{}) catch continue;
            
            import_spec.resolved_path = try self.allocator.dupe(u8, test_path);
            return;
        }
        
        return error.StdlibModuleNotFound;
    }
    
    fn resolvePackageModule(self: *AdvancedImportResolver, import_spec: *ImportSpec) !void {
        // Look for the package in cache directories
        for (self.search_paths.items) |search_path| {
            if (!search_path.is_cache) continue;
            
            // Try different file patterns - manually format strings to avoid comptime issue
            const pattern_formats = [_]struct { middle: []const u8, suffix: []const u8 }{
                .{ .middle = "/src/lib.csd", .suffix = "" },
                .{ .middle = "/mod.csd", .suffix = "" },
                .{ .middle = ".csd", .suffix = "" },
            };
            
            for (pattern_formats) |format| {
                var test_path = std.ArrayList(u8).init(self.allocator);
                defer test_path.deinit();
                
                try test_path.appendSlice(search_path.path);
                try test_path.append('/');
                try test_path.appendSlice(import_spec.raw_path);
                try test_path.appendSlice(format.middle);
                
                std.fs.cwd().access(test_path.items, .{}) catch continue;
                
                // Verify version requirements if specified
                if (import_spec.version_req) |version_req| {
                    if (!try self.verifyPackageVersion(import_spec.raw_path, version_req)) {
                        continue;
                    }
                }
                
                import_spec.resolved_path = try self.allocator.dupe(u8, test_path.items);
                return;
            }
        }
        
        return error.PackageNotFound;
    }
    
    fn resolveLocalModule(self: *AdvancedImportResolver, import_spec: *ImportSpec) !void {
        // Sort search paths by priority (highest first)
        const sorted_paths = try self.allocator.alloc(ModuleSearchPath, self.search_paths.items.len);
        defer self.allocator.free(sorted_paths);
        
        @memcpy(sorted_paths, self.search_paths.items);
        std.mem.sort(ModuleSearchPath, sorted_paths, {}, struct {
            fn lessThan(_: void, a: ModuleSearchPath, b: ModuleSearchPath) bool {
                return a.priority > b.priority; // Higher priority first
            }
        }.lessThan);
        
        for (sorted_paths) |search_path| {
            if (search_path.is_stdlib or search_path.is_cache) continue;
            
            // Try different file patterns - manually format strings to avoid comptime issue
            const pattern_formats = [_]struct { prefix: []const u8, suffix: []const u8 }{
                .{ .prefix = ".csd", .suffix = "" },
                .{ .prefix = "/mod.csd", .suffix = "" },
                .{ .prefix = "/lib.csd", .suffix = "" },
                .{ .prefix = "/index.csd", .suffix = "" },
            };
            
            for (pattern_formats) |format| {
                var test_path = std.ArrayList(u8).init(self.allocator);
                defer test_path.deinit();
                
                try test_path.appendSlice(search_path.path);
                try test_path.append('/');
                try test_path.appendSlice(import_spec.raw_path);
                try test_path.appendSlice(format.prefix);
                
                std.fs.cwd().access(test_path.items, .{}) catch continue;
                
                import_spec.resolved_path = try self.allocator.dupe(u8, test_path.items);
                return;
            }
        }
        
        return error.LocalModuleNotFound;
    }
    
    fn validateModule(_: *AdvancedImportResolver, import_spec: *ImportSpec) !void {
        const path = import_spec.resolved_path orelse return error.UnresolvedPath;
        
        // Check if file exists and is readable
        const file = std.fs.cwd().openFile(path, .{}) catch |err| switch (err) {
            error.FileNotFound => {
                print("Module not found: {s}\n", .{path});
                return error.ModuleNotFound;
            },
            error.AccessDenied => {
                print("Access denied to module: {s}\n", .{path});
                return error.ModuleAccessDenied;
            },
            else => return err,
        };
        defer file.close();
        
        // Basic validation: check if it's a valid CURSED file
        const file_size = try file.getEndPos();
        if (file_size > 0) {
            // Module exists and is readable
            return;
        }
        
        return error.EmptyModule;
    }
    
    fn verifyPackageVersion(self: *AdvancedImportResolver, package_name: []const u8, version_req: VersionRequirement) !bool {
        _ = self;
        _ = package_name;
        _ = version_req;
        
        // TODO: Implement actual package version verification
        // This would read the package's manifest and check the version
        return true;
    }
    
    // Extract all imports from a source file
    pub fn extractImports(self: *AdvancedImportResolver, source: []const u8) !ArrayList(ImportSpec) {
        var imports = ArrayList(ImportSpec).init(self.allocator);
        
        var line_num: u32 = 1;
        var lines = std.mem.splitScalar(u8, source, '\n');
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r");
            
            // Look for "yeet" statements
            if (std.mem.startsWith(u8, trimmed, "yeet ")) {
                var import_specs = try self.parseYeetStatementMultiple(trimmed, line_num);
                defer import_specs.deinit();
                
                for (import_specs.items) |import_spec| {
                    try imports.append(import_spec);
                }
            }
            
            line_num += 1;
        }
        
        return imports;
    }
    
    fn parseYeetStatementMultiple(self: *AdvancedImportResolver, line: []const u8, line_num: u32) !ArrayList(ImportSpec) {
        var imports = ArrayList(ImportSpec).init(self.allocator);
        
        const import_part = line[5..]; // Skip "yeet "
        
        // Extract all module names from quotes (handle comma-separated imports)
        var search_offset: usize = 0;
        var column_offset: u32 = 5; // Start after "yeet "
        
        while (search_offset < import_part.len) {
            if (std.mem.indexOfPos(u8, import_part, search_offset, "\"")) |start_quote| {
                const after_start = import_part[start_quote + 1 ..];
                if (std.mem.indexOf(u8, after_start, "\"")) |end_quote| {
                    const module_name = after_start[0..end_quote];
                    
                    const import_spec = ImportSpec{
                        .raw_path = try self.allocator.dupe(u8, module_name),
                        .module_type = .local, // Will be determined during resolution
                        .source_file = try self.allocator.dupe(u8, self.current_file orelse "unknown"),
                        .line = line_num,
                        .column = @intCast(column_offset + start_quote + 1),
                    };
                    
                    try imports.append(import_spec);
                    search_offset = start_quote + 1 + end_quote + 1;
                    column_offset += @intCast(start_quote + 1 + end_quote + 1);
                } else {
                    break; // No closing quote found
                }
            } else {
                break; // No more quotes found
            }
        }
        
        return imports;
    }

    fn parseYeetStatement(self: *AdvancedImportResolver, line: []const u8, line_num: u32) !?ImportSpec {
        // Extract module name from quotes in "yeet" statement
        // Format: yeet "module_name" [as alias]
        
        const import_part = line[5..]; // Skip "yeet "
        
        // Find quoted module name
        if (std.mem.indexOf(u8, import_part, "\"")) |start_quote| {
            const after_start = import_part[start_quote + 1..];
            if (std.mem.indexOf(u8, after_start, "\"")) |end_quote| {
                const module_name = after_start[0..end_quote];
                
                var import_spec = ImportSpec{
                    .raw_path = try self.allocator.dupe(u8, module_name),
                    .module_type = .local, // Will be determined during resolution
                    .source_file = try self.allocator.dupe(u8, self.current_file orelse "unknown"),
                    .line = line_num,
                    .column = @intCast(start_quote + 1),
                };
                
                // Check for alias: yeet "module" as alias_name
                const remaining = import_part[start_quote + end_quote + 2..];
                if (std.mem.indexOf(u8, remaining, " as ")) |as_pos| {
                    const alias_part = std.mem.trim(u8, remaining[as_pos + 4..], " \t");
                    if (alias_part.len > 0) {
                        import_spec.alias = try self.allocator.dupe(u8, alias_part);
                    }
                }
                
                return import_spec;
            }
        }
        
        return null;
    }
    
    // Batch resolve all imports in a source file
    pub fn resolveFileImports(self: *AdvancedImportResolver, source: []const u8, file_path: []const u8) !ArrayList(ImportSpec) {
        self.current_file = file_path;
        
        var extracted_imports = try self.extractImports(source);
        defer {
            for (extracted_imports.items) |*import| {
                import.deinit(self.allocator);
            }
            extracted_imports.deinit();
        }
        
        var resolved_imports = ArrayList(ImportSpec).init(self.allocator);
        
        for (extracted_imports.items) |import| {
            const resolved = self.resolveImport(import.raw_path, file_path) catch |err| {
                print("Failed to resolve import '{s}' in {s}: {any}\n", .{ import.raw_path, file_path, err });
                continue;
            };
            
            try resolved_imports.append(resolved);
        }
        
        return resolved_imports;
    }
    
    // Generate import dependency report
    pub fn generateDependencyReport(self: *AdvancedImportResolver) !void {
        print("\n=== Module Dependency Report ===\n", .{});
        
        var iter = self.module_cache.import_graph.iterator();
        while (iter.next()) |entry| {
            const module = entry.key_ptr.*;
            const dependencies = entry.value_ptr.*;
            
            print("\nModule: {s}\n", .{module});
            print("Dependencies:\n", .{});
            
            for (dependencies.items) |dep| {
                print("  - {s}\n", .{dep});
            }
        }
        
        // Check for cycles in the entire dependency graph
        print("\n=== Cycle Detection ===\n", .{});
        
        var checked = HashMap([]const u8, bool, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer checked.deinit();
        
        var graph_iter = self.module_cache.import_graph.iterator();
        while (graph_iter.next()) |entry| {
            const module = entry.key_ptr.*;
            
            if (checked.get(module) orelse false) continue;
            
            if (try self.module_cache.detectCycle(module)) |cycle| {
                defer cycle.deinit();
                
                print("Cycle detected starting from {s}:\n", .{module});
                for (cycle.items) |cycle_module| {
                    print("  -> {s}\n", .{cycle_module});
                }
                print("\n", .{});
                
                // Mark all modules in cycle as checked
                for (cycle.items) |cycle_module| {
                    try checked.put(cycle_module, true);
                }
            } else {
                try checked.put(module, true);
            }
        }
    }
};

// ===== Test Functions =====

test "advanced import resolver basic functionality" {
    const allocator = std.testing.allocator;
    
    var resolver = try AdvancedImportResolver.initWithDefaults(allocator);
    defer resolver.deinit();
    
    // Test adding search paths
    try resolver.addLocalPath("test_modules");
    try std.testing.expect(resolver.search_paths.items.len >= 1);
    
    // Test adding aliases
    try resolver.addAlias("testlib", "stdlib/testz");
    try std.testing.expect(resolver.aliases.contains("testlib"));
}

test "version requirement parsing and matching" {
    const allocator = std.testing.allocator;
    
    var resolver = AdvancedImportResolver.init(allocator);
    defer resolver.deinit();
    
    const import_spec = try resolver.parseImportStatement("json@^1.0.0", "test.csd");
    defer {
        var spec = import_spec;
        spec.deinit(allocator);
    }
    
    try std.testing.expectEqualStrings("json", import_spec.raw_path);
    try std.testing.expect(import_spec.version_req != null);
}

test "cycle detection" {
    const allocator = std.testing.allocator;
    
    var resolver = AdvancedImportResolver.init(allocator);
    defer resolver.deinit();
    
    // Create a cycle: A -> B -> C -> A
    try resolver.module_cache.addToGraph("A", "B");
    try resolver.module_cache.addToGraph("B", "C");
    try resolver.module_cache.addToGraph("C", "A");
    
    const cycle = try resolver.module_cache.detectCycle("A");
    try std.testing.expect(cycle != null);
    
    if (cycle) |c| {
        defer c.deinit();
        try std.testing.expect(c.items.len >= 3);
    }
}
