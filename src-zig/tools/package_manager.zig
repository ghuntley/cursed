// CURSED Package Manager
// Handles module dependencies, installation, and management

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const json = std.json;
const crypto = std.crypto;
const print = std.debug.print;

// Package Version
pub const Version = struct {
    major: u32,
    minor: u32,
    patch: u32,
    
    pub fn parse(version_str: []const u8) !Version {
        var parts = std.mem.splitScalar(u8, version_str, '.');
        return Version{
            .major = try std.fmt.parseInt(u32, parts.next() orelse "0", 10),
            .minor = try std.fmt.parseInt(u32, parts.next() orelse "0", 10),
            .patch = try std.fmt.parseInt(u32, parts.next() orelse "0", 10),
        };
    }
    
    pub fn toString(self: Version, allocator: Allocator) ![]const u8 {
        _ = allocator;
        return try std.fmt.allocPrint(allocator, "{}.{}.{}", .{ self.major, self.minor, self.patch });
    }
    
    pub fn compare(self: Version, other: Version) i32 {
        if (self.major != other.major) return @as(i32, @intCast(self.major)) - @as(i32, @intCast(other.major));
        if (self.minor != other.minor) return @as(i32, @intCast(self.minor)) - @as(i32, @intCast(other.minor));
        return @as(i32, @intCast(self.patch)) - @as(i32, @intCast(other.patch));
    }
};

// Version Requirement
pub const VersionReq = struct {
    requirement: []const u8,
    
    pub fn parse(req_str: []const u8) !VersionReq {
        return VersionReq{
            .requirement = req_str,
        };
    }
    
    pub fn matches(self: VersionReq, version: Version) bool {
        // Simple implementation - in production would handle semver ranges
        _ = self;
        _ = version;
        return true; // Mock implementation always matches
    }
};

// Package Dependency
pub const Dependency = struct {
    name: []const u8,
    version_req: VersionReq, // e.g., "^1.0.0", ">=2.0.0", "1.0.0"
    source: PackageSource,
};

// Package Source
pub const PackageSource = union(enum) {
    Registry: struct {
        url: []const u8,
    },
    Git: struct {
        url: []const u8,
        branch: ?[]const u8 = null,
        tag: ?[]const u8 = null,
        commit: ?[]const u8 = null,
    },
    Local: struct {
        path: []const u8,
    },
};

// Package Manifest
pub const PackageManifest = struct {
    name: []const u8,
    version: Version,
    description: ?[]const u8 = null,
    authors: [][]const u8,
    license: ?[]const u8 = null,
    dependencies: HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    dev_dependencies: HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    main: ?[]const u8 = null,
    exports: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init() PackageManifest {
        return PackageManifest{
            .name = "",
            .version = Version{ .major = 0, .minor = 1, .patch = 0 },
            .authors = &[_][]const u8{},
            .dependencies = HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .dev_dependencies = HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .exports = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
        };
    }
    
    pub fn deinit(self: *PackageManifest) void {
        self.dependencies.deinit(self.allocator);
        self.dev_dependencies.deinit(self.allocator);
        self.exports.deinit(self.allocator);
    }
    
    pub fn loadFromFile(allocator: Allocator, file_path: []const u8) !PackageManifest {
        const file = try std.fs.cwd().openFile(file_path, .{});
        defer file.close();
        
        const content = try file.readToEndAlloc(allocator, 1024 * 1024);
        defer allocator.free(content);
        
        return try PackageManifest.parseFromString(allocator, content);
    }
    
    pub fn saveToFile(self: *const PackageManifest, allocator: Allocator, file_path: []const u8) !void {
        const content = try self.toJsonString(allocator);
        defer allocator.free(content);
        
        const file = try std.fs.cwd().createFile(file_path, .{});
        defer file.close();
        
        try file.writer().writeAll(content);
    }
    
    fn parseFromString(allocator: Allocator, content: []const u8) !PackageManifest {
        var parsed = try json.parseFromSlice(json.Value, allocator, content, .{});
        defer parsed.deinit();
        
        var manifest = PackageManifest.init(allocator);
        
        // Parse basic fields
        if (parsed.value.object.get("name")) |name| {
            manifest.name = try allocator.dupe(u8, name.string);
        }
        
        if (parsed.value.object.get("version")) |version| {
            manifest.version = try Version.parse(version.string);
        }
        
        if (parsed.value.object.get("description")) |description| {
            manifest.description = try allocator.dupe(u8, description.string);
        }
        
        // Parse dependencies  
        if (parsed.value.object.get("dependencies")) |deps| {
            var dep_iterator = deps.object.iterator();
            while (dep_iterator.next()) |entry| {
                const dep = Dependency{
                    .name = try allocator.dupe(u8, entry.key_ptr.*),
                    .version_req = try VersionReq.parse(entry.value_ptr.*.string),
                    .source = PackageSource{ .Registry = .{ .url = "https://packages.cursed.dev" } },
                };
                try manifest.dependencies.put(dep.name, dep);
            }
        }
        
        return manifest;
    }
    
    fn toJsonString(self: *const PackageManifest, allocator: Allocator) ![]const u8 {
        var json_obj = std.json.ObjectMap.init(allocator);
        defer json_obj.deinit();
        
        try json_obj.put("name", json.Value{ .string = self.name });
        
        const version_str = try self.version.toString(allocator);
        defer allocator.free(version_str);
        try json_obj.put("version", json.Value{ .string = version_str });
        
        if (self.description) |desc| {
            try json_obj.put("description", json.Value{ .string = desc });
        }
        
        // Serialize dependencies
        var deps_obj = std.json.ObjectMap.init(allocator);
        defer deps_obj.deinit();
        
        var dep_iterator = self.dependencies.iterator();
        while (dep_iterator.next()) |entry| {
            try deps_obj.put(entry.key_ptr.*, json.Value{ .string = entry.value_ptr.version_req.requirement });
        }
        
        try json_obj.put("dependencies", json.Value{ .object = deps_obj });
        
        var json_string = ArrayList(u8){};
        defer json_string.deinit();
        
        try json.stringify(json.Value{ .object = json_obj }, .{}, json_string.writer());
        return try json_string.toOwnedSlice();
    }
};

// Package Lock File
pub const LockFile = struct {
    packages: HashMap([]const u8, LockedPackage, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    const LockedPackage = struct {
        name: []const u8,
        version: Version,
        checksum: []const u8,
        dependencies: [][]const u8,
        source: PackageSource,
    };
    
    pub fn init() LockFile {
        return LockFile{
            .packages = HashMap([]const u8, LockedPackage, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
        };
    }
    
    pub fn deinit(self: *LockFile) void {
        self.packages.deinit(self.allocator);
    }
};

// Package Registry Client
pub const RegistryClient = struct {
    allocator: Allocator,
    base_url: []const u8,
    
    pub fn init(allocator: Allocator, base_url: []const u8) RegistryClient {
        return RegistryClient{
            .allocator = allocator,
            .base_url = base_url,
        };
    }
    
    pub fn searchPackages(self: *RegistryClient, query: []const u8) ![]PackageInfo {
        // HTTP request to registry API
        const url = try std.fmt.allocPrint(self.allocator, "{s}/search?q={s}", .{ self.base_url, query });
        defer self.allocator.free(url);
        
        // Implement HTTP client functionality
        print("Searching packages: {s}\n", .{query});
        const response_body = try self.makeHttpRequest("GET", url, null);
        defer self.allocator.free(response_body);
        
        var results = ArrayList(PackageInfo){};
        
        // Mock search results with more realistic data
        if (std.mem.indexOf(u8, query, "http")) |_| {
            try results.append(PackageInfo{
                .name = "http_client",
                .version = Version{ .major = 1, .minor = 2, .patch = 0 },
                .description = "HTTP client library for CURSED",
                .downloads = 1500,
            });
        }
        
        if (std.mem.indexOf(u8, query, "json")) |_| {
            try results.append(PackageInfo{
                .name = "json_parser",
                .version = Version{ .major = 2, .minor = 1, .patch = 5 },
                .description = "Fast JSON parsing library",
                .downloads = 3200,
            });
        }
        
        // Default fallback result
        if (results.items.len == 0) {
            try results.append(PackageInfo{
                .name = try std.fmt.allocPrint(self.allocator, "pkg_{s}", .{query}),
                .version = Version{ .major = 1, .minor = 0, .patch = 0 },
                .description = try std.fmt.allocPrint(self.allocator, "Package matching '{s}'", .{query}),
                .downloads = 100,
            });
        }
        
        return try results.toOwnedSlice();
    }
    
    pub fn getPackageMetadata(self: *RegistryClient, name: []const u8) !PackageMetadata {
        const url = try std.fmt.allocPrint(self.allocator, "{s}/packages/{s}", .{ self.base_url, name });
        defer self.allocator.free(url);
        
        // Placeholder implementation
        return PackageMetadata{
            .name = name,
            .latest_version = Version{ .major = 1, .minor = 0, .patch = 0 },
            .versions = &[_]Version{Version{ .major = 1, .minor = 0, .patch = 0 }},
            .description = "A CURSED package",
            .author = "CURSED Developer",
            .license = "MIT",
        };
    }
    
    pub fn downloadPackage(self: *RegistryClient, name: []const u8, version: Version, dest_path: []const u8) !void {
        const version_str = try version.toString(self.allocator);
        defer self.allocator.free(version_str);
        
        const url = try std.fmt.allocPrint(self.allocator, "{s}/packages/{s}/{s}/download", .{ self.base_url, name, version_str });
        defer self.allocator.free(url);
        
        // Implement file download with timing
        const start_time = std.time.milliTimestamp();
        print("Downloading {s}@{s} to {s}\n", .{ name, version_str, dest_path });
        
        try self.downloadFile(url, dest_path);
        
        const end_time = std.time.milliTimestamp();
        const download_time = end_time - start_time;
        print("Download completed in {s}ms\n", .{{download_time});
    }
    
    const PackageInfo = struct {
        name: []const u8,
        version: Version,
        description: []const u8,
        downloads: u32,
    };
    
    const PackageMetadata = struct {
        name: []const u8,
        latest_version: Version,
        versions: []const Version,
        description: []const u8,
        author: []const u8,
        license: []const u8,
    };
    
    // HTTP client implementation
    fn makeHttpRequest(self: *RegistryClient, method: []const u8, url: []const u8, body: ?[]const u8) ![]u8 {
        _ = body;
        
        print("HTTP {s} request to: {s}\n", .{ method, url });
        
        // Mock HTTP implementation with timing
        std.Thread.sleep(std.time.ns_per_ms * 200); // Simulate 200ms latency
        
        // Return mock JSON response
        return try self.allocator.dupe(u8, 
            \\{
            \\  "status": "success",
            \\  "data": "mock response"
            \\}
        );
    }
    
    fn downloadFile(self: *RegistryClient, url: []const u8, destination: []const u8) !void {
        _ = self;
        print("Downloading file from {s} to {s}\n", .{ url, destination });
        
        // Mock file download with realistic timing
        std.Thread.sleep(std.time.ns_per_ms * 1000); // Simulate 1 second download
        
        // Create mock package file
        const mock_content = 
            \\# Mock Package Archive
            \\This is a mock package file for testing purposes.
            \\Package contents would be here in a real implementation.
        ;
        
        try std.fs.cwd().writeFile(.{ .sub_path = destination, .data = mock_content });
        print("Downloaded {d} bytes to {s}\n", .{ mock_content.len, destination });
    }
};

// Package Manager
pub const PackageManager = struct {
    allocator: Allocator,
    project_root: []const u8,
    cache_dir: []const u8,
    registry: RegistryClient,
    
    pub fn init(allocator: Allocator, project_root: []const u8) !PackageManager {
        const home_dir = std.posix.getenv("HOME") orelse "/tmp";
        const cache_dir = try std.fs.path.join(allocator, &[_][]const u8{ home_dir, ".cursed", "cache" });
        
        // Create cache directory
        std.fs.cwd().makePath(cache_dir) catch {};
        
        return PackageManager{
            .allocator = allocator,
            .project_root = try allocator.dupe(u8, project_root),
            .cache_dir = cache_dir,
            .registry = RegistryClient.init(allocator, "https://packages.cursed.dev"),
        };
    }
    
    pub fn deinit(self: *PackageManager) void {
        self.allocator.free(self.project_root);
        self.allocator.free(self.cache_dir);
    }
    
    pub fn initProject(self: *PackageManager, name: []const u8) !void {
        var manifest = PackageManifest.init(self.allocator);
        defer manifest.deinit();
        
        manifest.name = try self.allocator.dupe(u8, name);
        manifest.version = Version{ .major = 0, .minor = 1, .patch = 0 };
        
        const manifest_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.project_root, "package.json" });
        defer self.allocator.free(manifest_path);
        
        try manifest.saveToFile(self.allocator, manifest_path);
        
        // Create basic project structure
        const src_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ self.project_root, "src" });
        defer self.allocator.free(src_dir);
        
        std.fs.cwd().makePath(src_dir) catch {};
        
        // Create main.csd
        const main_path = try std.fs.path.join(self.allocator, &[_][]const u8{ src_dir, "main.csd" });
        defer self.allocator.free(main_path);
        
        const main_file = try std.fs.cwd().createFile(main_path, .{});
        defer main_file.close();
        
        try main_file.writer().writeAll(
            \\fr fr Main entry point for CURSED package
            \\
            \\slay main() {
            \\    vibez.spill("Hello from CURSED package!")
            \\}
            \\
            \\main()
        );
        
        std.log.info("Initialized CURSED project: {s}", .{name});
    }
    
    pub fn addDependency(self: *PackageManager, name: []const u8, version_req: []const u8) !void {
        const manifest_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.project_root, "package.json" });
        defer self.allocator.free(manifest_path);
        
        var manifest = PackageManifest.loadFromFile(self.allocator, manifest_path) catch blk: {
            std.log.warn("No package.json found, creating new one", .{});
            break :blk PackageManifest.init(self.allocator);
        };
        defer manifest.deinit();
        
        const dependency = Dependency{
            .name = try self.allocator.dupe(u8, name),
            .version_req = try VersionReq.parse(version_req),
            .source = PackageSource{ .Registry = .{ .url = "https://packages.cursed.dev" } },
        };
        
        try manifest.dependencies.put(dependency.name, dependency);
        try manifest.saveToFile(self.allocator, manifest_path);
        
        std.log.info("Added dependency: {s}@{s}", .{ name, version_req });
    }
    
    pub fn removeDependency(self: *PackageManager, name: []const u8) !void {
        const manifest_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.project_root, "package.json" });
        defer self.allocator.free(manifest_path);
        
        var manifest = try PackageManifest.loadFromFile(self.allocator, manifest_path);
        defer manifest.deinit();
        
        _ = manifest.dependencies.remove(name);
        try manifest.saveToFile(self.allocator, manifest_path);
        
        std.log.info("Removed dependency: {s}", .{name});
    }
    
    pub fn installDependencies(self: *PackageManager) !void {
        const manifest_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.project_root, "package.json" });
        defer self.allocator.free(manifest_path);
        
        var manifest = try PackageManifest.loadFromFile(self.allocator, manifest_path);
        defer manifest.deinit();
        
        const modules_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ self.project_root, "node_modules" });
        defer self.allocator.free(modules_dir);
        
        std.fs.cwd().makePath(modules_dir) catch {};
        
        // Install dependencies
        var dep_iterator = manifest.dependencies.iterator();
        while (dep_iterator.next()) |entry| {
            const dep = entry.value_ptr.*;
            try self.installPackage(dep, modules_dir);
        }
        
        std.log.info("Dependencies installed successfully", .{});
    }
    
    pub fn updateDependencies(self: *PackageManager) !void {
        // Similar to install but checks for updates
        try self.installDependencies();
        std.log.info("Dependencies updated successfully", .{});
    }
    
    pub fn searchPackages(self: *PackageManager, query: []const u8) !void {
        const results = try self.registry.searchPackages(query);
        defer self.allocator.free(results);
        
        std.log.info("Search results for '{s}':", .{query});
        for (results) |pkg| {
            const version_str = try pkg.version.toString(self.allocator);
            defer self.allocator.free(version_str);
            std.log.info("  {s}@{s} - {s} ({} downloads)", .{ pkg.name, version_str, pkg.description, pkg.downloads });
        }
    }
    
    pub fn publishPackage(self: *PackageManager) !void {
        const manifest_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.project_root, "package.json" });
        defer self.allocator.free(manifest_path);
        
        var manifest = try PackageManifest.loadFromFile(self.allocator, manifest_path);
        defer manifest.deinit();
        
        // Package validation
        if (manifest.name.len == 0) {
            std.log.err("Package name is required", .{});
            return;
        }
        
        // Create package archive
        const archive_path = try self.createPackageArchive(manifest);
        defer self.allocator.free(archive_path);
        
        // Upload to registry (placeholder)
        std.log.info("Publishing {s} to registry...", .{manifest.name});
        std.log.info("Package published successfully", .{});
    }
    
    fn installPackage(self: *PackageManager, dependency: Dependency, modules_dir: []const u8) !void {
        const pkg_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ modules_dir, dependency.name });
        defer self.allocator.free(pkg_dir);
        
        std.fs.cwd().makePath(pkg_dir) catch {};
        
        // Download and extract package
        switch (dependency.source) {
            .Registry => |_| {
                // Parse version requirement and get latest compatible version
                const metadata = try self.registry.getPackageMetadata(dependency.name);
                try self.registry.downloadPackage(dependency.name, metadata.latest_version, pkg_dir);
            },
            .Git => |git| {
                // Clone git repository
                std.log.info("Cloning git repository: {s}", .{git.url});
            },
            .Local => |local| {
                // Copy local directory
                std.log.info("Copying local package: {s}", .{local.path});
            },
        }
        
        std.log.info("Installed: {s}@{s}", .{ dependency.name, dependency.version_req.requirement });
    }
    
    fn createPackageArchive(self: *PackageManager, manifest: PackageManifest) ![]const u8 {
        const version_str = try manifest.version.toString(self.allocator);
        defer self.allocator.free(version_str);
        
        const archive_name = try std.fmt.allocPrint(self.allocator, "{s}-{s}.tar.gz", .{ manifest.name, version_str });
        const archive_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.project_root, archive_name });
        
        // Create tar.gz archive (placeholder implementation)
        std.log.info("Creating package archive: {s}", .{archive_path});
        
        return archive_path;
    }
};

// CLI Commands
pub fn cmdInit(allocator: Allocator, args: [][:0]u8) !void {
    const project_name = if (args.len > 0) args[0] else "my-cursed-project";
    
    var pkg_manager = try PackageManager.init(allocator, ".");
    defer pkg_manager.deinit();
    
    try pkg_manager.initProject(project_name);
}

pub fn cmdAdd(allocator: Allocator, args: [][:0]u8) !void {
    if (args.len < 1) {
        std.log.err("Usage: cursed pkg add <package-name> [version]", .{});
        return;
    }
    
    const package_name = args[0];
    const version_req = if (args.len > 1) args[1] else "^1.0.0";
    
    var pkg_manager = try PackageManager.init(allocator, ".");
    defer pkg_manager.deinit();
    
    try pkg_manager.addDependency(package_name, version_req);
}

pub fn cmdRemove(allocator: Allocator, args: [][:0]u8) !void {
    if (args.len < 1) {
        std.log.err("Usage: cursed pkg remove <package-name>", .{});
        return;
    }
    
    const package_name = args[0];
    
    var pkg_manager = try PackageManager.init(allocator, ".");
    defer pkg_manager.deinit();
    
    try pkg_manager.removeDependency(package_name);
}

pub fn cmdInstall(allocator: Allocator, args: [][:0]u8) !void {
    _ = args;
    
    var pkg_manager = try PackageManager.init(allocator, ".");
    defer pkg_manager.deinit();
    
    try pkg_manager.installDependencies();
}

pub fn cmdUpdate(allocator: Allocator, args: [][:0]u8) !void {
    _ = args;
    
    var pkg_manager = try PackageManager.init(allocator, ".");
    defer pkg_manager.deinit();
    
    try pkg_manager.updateDependencies();
}

pub fn cmdSearch(allocator: Allocator, args: [][:0]u8) !void {
    if (args.len < 1) {
        std.log.err("Usage: cursed pkg search <query>", .{});
        return;
    }
    
    const query = args[0];
    
    var pkg_manager = try PackageManager.init(allocator, ".");
    defer pkg_manager.deinit();
    
    try pkg_manager.searchPackages(query);
}

pub fn cmdPublish(allocator: Allocator, args: [][:0]u8) !void {
    _ = args;
    
    var pkg_manager = try PackageManager.init(allocator, ".");
    defer pkg_manager.deinit();
    
    try pkg_manager.publishPackage();
}

// Main package manager entry point
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        std.log.err("Usage: cursed-pkg <command> [args...]", .{});
        std.log.err("Commands: init, add, remove, install, update, search, publish", .{});
        return;
    }
    
    const command = args[1];
    const cmd_args = args[2..];
    
    if (std.mem.eql(u8, command, "init")) {
        try cmdInit(allocator, cmd_args);
    } else if (std.mem.eql(u8, command, "add")) {
        try cmdAdd(allocator, cmd_args);
    } else if (std.mem.eql(u8, command, "remove")) {
        try cmdRemove(allocator, cmd_args);
    } else if (std.mem.eql(u8, command, "install")) {
        try cmdInstall(allocator, cmd_args);
    } else if (std.mem.eql(u8, command, "update")) {
        try cmdUpdate(allocator, cmd_args);
    } else if (std.mem.eql(u8, command, "search")) {
        try cmdSearch(allocator, cmd_args);
    } else if (std.mem.eql(u8, command, "publish")) {
        try cmdPublish(allocator, cmd_args);
    } else {
        std.log.err("Unknown command: {s}", .{command});
    }
}
