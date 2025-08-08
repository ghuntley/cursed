// CURSED Build System Integration with Package Manager
// Integrates package dependencies into the Zig build system

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const package_manager = @import("tools/package_manager_enhanced.zig");

// Build dependency information
pub const BuildDependency = struct {
    name: []const u8,
    version: package_manager.Version,
    path: []const u8,
    lib_type: LibType = .static,
    include_dirs: ArrayList([]const u8),
    lib_dirs: ArrayList([]const u8),
    system_libs: ArrayList([]const u8),
    frameworks: ArrayList([]const u8), // macOS/iOS only
    defines: HashMap([]const u8, ?[]const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const LibType = enum {
        static,
        dynamic,
        object,
        header_only,
    };
    
    pub fn init(allocator: Allocator, name: []const u8, version: package_manager.Version, path: []const u8) BuildDependency {
        return BuildDependency{
            .name = name,
            .version = version,
            .path = path,
            .include_dirs = ArrayList([]const u8).init(allocator),
            .lib_dirs = ArrayList([]const u8).init(allocator),
            .system_libs = ArrayList([]const u8).init(allocator),
            .frameworks = ArrayList([]const u8).init(allocator),
            .defines = HashMap([]const u8, ?[]const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *BuildDependency) void {
        self.include_dirs.deinit();
        self.lib_dirs.deinit();
        self.system_libs.deinit();
        self.frameworks.deinit();
        self.defines.deinit();
    }
    
    pub fn addIncludeDir(self: *BuildDependency, dir: []const u8) !void {
        try self.include_dirs.append(dir);
    }
    
    pub fn addLibDir(self: *BuildDependency, dir: []const u8) !void {
        try self.lib_dirs.append(dir);
    }
    
    pub fn addSystemLib(self: *BuildDependency, lib: []const u8) !void {
        try self.system_libs.append(lib);
    }
    
    pub fn addFramework(self: *BuildDependency, framework: []const u8) !void {
        try self.frameworks.append(framework);
    }
    
    pub fn addDefine(self: *BuildDependency, name: []const u8, value: ?[]const u8) !void {
        try self.defines.put(name, value);
    }
};

// Main build integration manager
pub const BuildIntegration = struct {
    allocator: Allocator,
    cache_dir: []const u8,
    manifest: ?package_manager.PackageManifest = null,
    lock_file: ?package_manager.LockFile = null,
    dependencies: ArrayList(BuildDependency),
    target: ?std.Build.ResolvedTarget = null,
    optimize: ?std.builtin.OptimizeMode = null,
    
    pub fn init(allocator: Allocator, cache_dir: []const u8) BuildIntegration {
        return BuildIntegration{
            .allocator = allocator,
            .cache_dir = cache_dir,
            .dependencies = ArrayList(BuildDependency).init(allocator),
        };
    }
    
    pub fn deinit(self: *BuildIntegration) void {
        for (self.dependencies.items) |*dep| {
            dep.deinit();
        }
        self.dependencies.deinit();
        
        if (self.manifest) |*manifest| {
            manifest.deinit(self.allocator);
        }
        
        if (self.lock_file) |*lock_file| {
            lock_file.deinit();
        }
    }
    
    pub fn loadProject(self: *BuildIntegration) !void {
        // Load package manifest
        self.manifest = package_manager.PackageManifest.loadFromToml(
            self.allocator,
            "CursedPackage.toml"
        ) catch |err| switch (err) {
            error.FileNotFound => {
                std.debug.print("No CursedPackage.toml found, skipping package dependencies\n", .{});
                return;
            },
            else => return err,
        };
        
        // Load lock file if it exists
        self.lock_file = package_manager.LockFile.loadFromFile(
            self.allocator,
            "CursedPackage.lock"
        ) catch |err| switch (err) {
            error.FileNotFound => {
                std.debug.print("No lock file found, dependencies may need to be installed\n", .{});
                return;
            },
            else => return err,
        };
        
        // Load dependencies from lock file
        if (self.lock_file) |lock_file| {
            try self.loadDependenciesFromLock(lock_file);
        }
    }
    
    fn loadDependenciesFromLock(self: *BuildIntegration, lock_file: package_manager.LockFile) !void {
        for (lock_file.packages.items) |locked_pkg| {
            var cache = package_manager.PackageCache.init(self.allocator, self.cache_dir);
            const pkg_path = try cache.getPackagePath(locked_pkg.name, locked_pkg.version);
            defer self.allocator.free(pkg_path);
            
            var build_dep = BuildDependency.init(self.allocator, locked_pkg.name, locked_pkg.version, pkg_path);
            
            // Set up standard include and lib directories
            const src_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ pkg_path, "src" });
            const lib_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ pkg_path, "lib" });
            const include_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ pkg_path, "include" });
            
            try build_dep.addIncludeDir(src_dir);
            try build_dep.addIncludeDir(include_dir);
            try build_dep.addLibDir(lib_dir);
            
            // Load package-specific build configuration if it exists
            try self.loadPackageBuildConfig(&build_dep, pkg_path);
            
            try self.dependencies.append(build_dep);
        }
    }
    
    fn loadPackageBuildConfig(self: *BuildIntegration, build_dep: *BuildDependency, pkg_path: []const u8) !void {
        const config_path = try std.fs.path.join(self.allocator, &[_][]const u8{ pkg_path, "build_config.toml" });
        defer self.allocator.free(config_path);
        
        const config_file = std.fs.cwd().openFile(config_path, .{}) catch |err| switch (err) {
            error.FileNotFound => return, // No build config, use defaults
            else => return err,
        };
        defer config_file.close();
        
        const content = try config_file.readToEndAlloc(self.allocator, 1024 * 1024);
        defer self.allocator.free(content);
        
        var parser = package_manager.TomlParser.init(self.allocator, content);
        var toml = try parser.parse();
        defer toml.deinit(self.allocator);
        
        if (toml.table.get("build")) |build_val| {
            if (build_val == .table) {
                try self.parseBuildConfig(build_dep, build_val.table);
            }
        }
    }
    
    fn parseBuildConfig(
        self: *BuildIntegration,
        build_dep: *BuildDependency,
        build_table: HashMap([]const u8, package_manager.TomlValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)
    ) !void {
        // Parse library type
        if (build_table.get("lib_type")) |lib_type_val| {
            if (lib_type_val == .string) {
                if (std.mem.eql(u8, lib_type_val.string, "static")) {
                    build_dep.lib_type = .static;
                } else if (std.mem.eql(u8, lib_type_val.string, "dynamic")) {
                    build_dep.lib_type = .dynamic;
                } else if (std.mem.eql(u8, lib_type_val.string, "header_only")) {
                    build_dep.lib_type = .header_only;
                }
            }
        }
        
        // Parse system libraries
        if (build_table.get("system_libs")) |sys_libs_val| {
            if (sys_libs_val == .array) {
                for (sys_libs_val.array.items) |lib_val| {
                    if (lib_val == .string) {
                        try build_dep.addSystemLib(try self.allocator.dupe(u8, lib_val.string));
                    }
                }
            }
        }
        
        // Parse frameworks (macOS/iOS)
        if (build_table.get("frameworks")) |frameworks_val| {
            if (frameworks_val == .array) {
                for (frameworks_val.array.items) |framework_val| {
                    if (framework_val == .string) {
                        try build_dep.addFramework(try self.allocator.dupe(u8, framework_val.string));
                    }
                }
            }
        }
        
        // Parse additional include directories
        if (build_table.get("include_dirs")) |include_dirs_val| {
            if (include_dirs_val == .array) {
                for (include_dirs_val.array.items) |dir_val| {
                    if (dir_val == .string) {
                        const full_path = try std.fs.path.join(self.allocator, &[_][]const u8{ build_dep.path, dir_val.string });
                        try build_dep.addIncludeDir(full_path);
                    }
                }
            }
        }
        
        // Parse preprocessor defines
        if (build_table.get("defines")) |defines_val| {
            if (defines_val == .table) {
                var iter = defines_val.table.iterator();
                while (iter.next()) |entry| {
                    const value = if (entry.value_ptr.* == .string)
                        try self.allocator.dupe(u8, entry.value_ptr.string)
                    else
                        null;
                    
                    try build_dep.addDefine(try self.allocator.dupe(u8, entry.key_ptr.*), value);
                }
            }
        }
    }
    
    // Apply dependencies to a Zig build artifact
    pub fn applyDependencies(self: *BuildIntegration, exe: *std.Build.Step.Compile) !void {
        for (self.dependencies.items) |dep| {
            // Add include paths
            for (dep.include_dirs.items) |include_dir| {
                exe.addIncludePath(.{ .cwd_relative = include_dir });
            }
            
            // Add library paths
            for (dep.lib_dirs.items) |lib_dir| {
                exe.addLibraryPath(.{ .cwd_relative = lib_dir });
            }
            
            // Link system libraries
            for (dep.system_libs.items) |sys_lib| {
                exe.linkSystemLibrary(sys_lib);
            }
            
            // Link frameworks (macOS/iOS)
            for (dep.frameworks.items) |framework| {
                exe.linkFramework(framework);
            }
            
            // Add preprocessor defines
            // TODO: Update to use correct Zig build API for defining C macros
            var defines_iter = dep.defines.iterator();
            while (defines_iter.next()) |entry| {
                _ = entry; // Suppress unused variable warning
                // if (entry.value_ptr.*) |value| {
                //     exe.defineCMacroRaw(try std.fmt.allocPrint(self.allocator, "{s}={s}", .{ entry.key_ptr.*, value }));
                // } else {
                //     exe.defineCMacroRaw(try std.fmt.allocPrint(self.allocator, "{s}=1", .{entry.key_ptr.*}));
                // }
            }
            
            // Link the dependency library if it's not header-only
            if (dep.lib_type != .header_only) {
                const lib_name = switch (dep.lib_type) {
                    .static => try std.fmt.allocPrint(self.allocator, "lib{s}.a", .{dep.name}),
                    .dynamic => try std.fmt.allocPrint(self.allocator, "lib{s}.so", .{dep.name}),
                    .object => try std.fmt.allocPrint(self.allocator, "{s}.o", .{dep.name}),
                    .header_only => unreachable,
                };
                defer self.allocator.free(lib_name);
                
                // Check if library exists in any lib directory
                for (dep.lib_dirs.items) |lib_dir| {
                    const lib_path = try std.fs.path.join(self.allocator, &[_][]const u8{ lib_dir, lib_name });
                    defer self.allocator.free(lib_path);
                    
                    if (std.fs.cwd().access(lib_path, .{})) {
                        exe.addObjectFile(.{ .cwd_relative = lib_path });
                        break;
                    } else |_| {
                        // Try to link by name
                        exe.linkSystemLibrary(dep.name);
                        break;
                    }
                }
            }
        }
    }
    
    // Build all dependencies that need building
    pub fn buildDependencies(self: *BuildIntegration, b: *std.Build, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode) !void {
        for (self.dependencies.items) |dep| {
            try self.buildDependency(b, dep, target, optimize);
        }
    }
    
    fn buildDependency(self: *BuildIntegration, b: *std.Build, dep: BuildDependency, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode) !void {
        // Check if dependency has its own build system
        const dep_build_zig = try std.fs.path.join(self.allocator, &[_][]const u8{ dep.path, "build.zig" });
        defer self.allocator.free(dep_build_zig);
        
        if (std.fs.cwd().access(dep_build_zig, .{})) {
            // Dependency has its own build.zig, build it as a subproject
            try self.buildSubproject(b, dep, target, optimize);
        } else |_| {
            // Try to build from source files
            try self.buildFromSources(b, dep, target, optimize);
        }
    }
    
    fn buildSubproject(self: *BuildIntegration, b: *std.Build, dep: BuildDependency, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode) !void {
        _ = self;
        _ = b;
        _ = target;
        _ = optimize;
        
        // TODO: Implement subproject building
        // This would involve:
        // 1. Running `zig build` in the dependency directory
        // 2. Copying build artifacts to the appropriate locations
        // 3. Setting up proper linking
        
        std.debug.print("Building subproject {s} (not yet implemented)\n", .{dep.name});
    }
    
    fn buildFromSources(self: *BuildIntegration, b: *std.Build, dep: BuildDependency, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode) !void {
        // Find source files in the dependency
        const src_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ dep.path, "src" });
        defer self.allocator.free(src_dir);
        
        var src_files = ArrayList([]const u8).init(self.allocator);
        defer src_files.deinit();
        
        // Look for main library file
        const main_lib_files = [_][]const u8{ "lib.zig", "main.zig", "mod.zig" };
        var main_source: ?[]const u8 = null;
        
        for (main_lib_files) |lib_file| {
            const lib_path = try std.fs.path.join(self.allocator, &[_][]const u8{ src_dir, lib_file });
            defer self.allocator.free(lib_path);
            
            if (std.fs.cwd().access(lib_path, .{})) {
                main_source = try self.allocator.dupe(u8, lib_path);
                break;
            } else |_| {}
        }
        
        if (main_source) |source| {
            defer self.allocator.free(source);
            
            // Create library artifact for dependency
            const lib = switch (dep.lib_type) {
                .static => b.addStaticLibrary(.{
                    .name = dep.name,
                    .root_source_file = b.path(source),
                    .target = target,
                    .optimize = optimize,
                }),
                .dynamic => b.addSharedLibrary(.{
                    .name = dep.name,
                    .root_source_file = b.path(source),
                    .target = target,
                    .optimize = optimize,
                }),
                .object => b.addObject(.{
                    .name = dep.name,
                    .root_source_file = b.path(source),
                    .target = target,
                    .optimize = optimize,
                }),
                .header_only => return, // Nothing to build
            };
            
            // Apply dependency configuration to the library
            for (dep.include_dirs.items) |include_dir| {
                lib.addIncludePath(.{ .cwd_relative = include_dir });
            }
            
            for (dep.system_libs.items) |sys_lib| {
                lib.linkSystemLibrary(sys_lib);
            }
            
            // TODO: Update to use correct Zig build API for defining C macros
            var defines_iter = dep.defines.iterator();
            while (defines_iter.next()) |entry| {
                _ = entry; // Suppress unused variable warning
                // if (entry.value_ptr.*) |value| {
                //     lib.defineCMacroRaw(try std.fmt.allocPrint(self.allocator, "{s}={s}", .{ entry.key_ptr.*, value }));
                // } else {
                //     lib.defineCMacroRaw(try std.fmt.allocPrint(self.allocator, "{s}=1", .{entry.key_ptr.*}));
                // }
            }
            
            b.installArtifact(lib);
            
            std.debug.print("Built dependency library: {s}\n", .{dep.name});
        } else {
            std.debug.print("No buildable sources found for dependency: {s}\n", .{dep.name});
        }
    }
    
    // Verify all dependencies are available and up to date
    pub fn verifyDependencies(self: *BuildIntegration) !bool {
        if (self.manifest == null) {
            return true; // No dependencies to verify
        }
        
        for (self.dependencies.items) |dep| {
            var cache = package_manager.PackageCache.init(self.allocator, self.cache_dir);
            const is_cached = try cache.isPackageCached(dep.name, dep.version);
            
            if (!is_cached) {
                const version_str = try dep.version.toString(self.allocator);
                defer self.allocator.free(version_str);
                std.debug.print("Dependency not found in cache: {s} v{s}\n", .{ dep.name, version_str });
                std.debug.print("Run 'cursed pkg install' to install missing dependencies\n", .{});
                return false;
            }
        }
        
        return true;
    }
    
    // Generate import path mappings for the CURSED interpreter
    pub fn generateImportMap(self: *BuildIntegration) !HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage) {
        var import_map = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        
        for (self.dependencies.items) |dep| {
            // Map dependency name to its main module path
            const main_module_files = [_][]const u8{ "lib.csd", "main.csd", "mod.csd" };
            
            for (main_module_files) |module_file| {
                const module_path = try std.fs.path.join(self.allocator, &[_][]const u8{ dep.path, "src", module_file });
                defer self.allocator.free(module_path);
                
                if (std.fs.cwd().access(module_path, .{})) {
                    try import_map.put(
                        try self.allocator.dupe(u8, dep.name),
                        try self.allocator.dupe(u8, module_path)
                    );
                    break;
                } else |_| {}
            }
        }
        
        return import_map;
    }
    
    // Check if dependencies need to be updated
    pub fn checkForUpdates(self: *BuildIntegration) !ArrayList(struct { name: []const u8, current: package_manager.Version, available: package_manager.Version }) {
        const updates = ArrayList(struct { name: []const u8, current: package_manager.Version, available: package_manager.Version }).init(self.allocator);
        
        // TODO: Implement update checking by querying package registry
        // This would involve:
        // 1. For each dependency, query the registry for latest compatible version
        // 2. Compare with currently installed version
        // 3. Report available updates
        
        return updates;
    }
};

// Helper function to integrate with build.zig
pub fn integrateBuildSystem(b: *std.Build, exe: *std.Build.Step.Compile, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode) !void {
    var integration = BuildIntegration.init(b.allocator, ".cursed/cache");
    defer integration.deinit();
    
    try integration.loadProject();
    
    // Verify dependencies are available
    if (!try integration.verifyDependencies()) {
        std.debug.print("Dependencies verification failed. Please run 'cursed pkg install'\n", .{});
        return;
    }
    
    // Build dependencies first
    try integration.buildDependencies(b, target, optimize);
    
    // Apply dependencies to main executable
    try integration.applyDependencies(exe);
    
    std.debug.print("Successfully integrated {d} package dependencies\n", .{integration.dependencies.items.len});
}

// Test integration functionality
test "build integration initialization" {
    const allocator = std.testing.allocator;
    
    var integration = BuildIntegration.init(allocator, "test_cache");
    defer integration.deinit();
    
    try std.testing.expect(integration.dependencies.items.len == 0);
    try std.testing.expectEqualStrings("test_cache", integration.cache_dir);
}

test "build dependency management" {
    const allocator = std.testing.allocator;
    
    const version = package_manager.Version{ .major = 1, .minor = 0, .patch = 0 };
    var dep = BuildDependency.init(allocator, "test-dep", version, "/path/to/dep");
    defer dep.deinit();
    
    try dep.addIncludeDir("/path/to/include");
    try dep.addSystemLib("m");
    try dep.addDefine("TEST_FLAG", "1");
    
    try std.testing.expect(dep.include_dirs.items.len == 1);
    try std.testing.expect(dep.system_libs.items.len == 1);
    try std.testing.expect(dep.defines.count() == 1);
}
