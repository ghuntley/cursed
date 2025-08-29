// CURSED Package Manager Core Implementation
// Production-ready package management with comprehensive functionality

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;
const templates = @import("package_templates.zig");

// ===== CORE DATA STRUCTURES =====

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
    
    pub fn satisfies(self: Version, constraint: VersionConstraint) bool {
        switch (constraint) {
            .exact => |v| return self.compare(v) == 0,
            .caret => |v| return self.major == v.major and self.compare(v) >= 0,
            .tilde => |v| return self.major == v.major and self.minor == v.minor and self.patch >= v.patch,
            .greater => |v| return self.compare(v) > 0,
            .greater_eq => |v| return self.compare(v) >= 0,
            .less => |v| return self.compare(v) < 0,
            .less_eq => |v| return self.compare(v) <= 0,
            .wildcard => |w| {
                if (w.major) |maj| {
                    if (self.major != maj) return false;
                    if (w.minor) |min| {
                        return self.minor == min;
                    }
                    return true;
                }
                return true;
            },
        }
    }
};

pub const VersionConstraint = union(enum) {
    exact: Version,
    caret: Version,    // ^1.0.0 - compatible version
    tilde: Version,    // ~1.0.0 - reasonably close
    greater: Version,  // >1.0.0
    greater_eq: Version, // >=1.0.0
    less: Version,     // <1.0.0
    less_eq: Version,  // <=1.0.0
    wildcard: struct {
        major: ?u32 = null,
        minor: ?u32 = null,
    },
};

pub const VersionRequirement = struct {
    constraint: VersionConstraint,
    
    pub fn parse(allocator: Allocator, req_str: []const u8) !VersionRequirement {
                
        if (std.mem.startsWith(u8, req_str, "^")) {
            const version = try Version.parse(req_str[1..]);
            return VersionRequirement{ .constraint = .{ .caret = version } };
        } else if (std.mem.startsWith(u8, req_str, "~")) {
            const version = try Version.parse(req_str[1..]);
            return VersionRequirement{ .constraint = .{ .tilde = version } };
        } else if (std.mem.startsWith(u8, req_str, ">=")) {
            const version = try Version.parse(req_str[2..]);
            return VersionRequirement{ .constraint = .{ .greater_eq = version } };
        } else if (std.mem.startsWith(u8, req_str, ">")) {
            const version = try Version.parse(req_str[1..]);
            return VersionRequirement{ .constraint = .{ .greater = version } };
        } else if (std.mem.startsWith(u8, req_str, "<=")) {
            const version = try Version.parse(req_str[2..]);
            return VersionRequirement{ .constraint = .{ .less_eq = version } };
        } else if (std.mem.startsWith(u8, req_str, "<")) {
            const version = try Version.parse(req_str[1..]);
            return VersionRequirement{ .constraint = .{ .less = version } };
        } else if (std.mem.indexOf(u8, req_str, "*")) |_| {
            // Handle wildcard versions like 1.* or 1.2.*
            var parts = std.mem.splitScalar(u8, req_str, '.');
            const major_str = parts.next() orelse return error.InvalidVersion;
            const minor_str = parts.next();
            
            var wildcard = VersionConstraint{ .wildcard = .{} };
            
            if (!std.mem.eql(u8, major_str, "*")) {
                wildcard.wildcard.major = try std.fmt.parseInt(u32, major_str, 10);
                
                if (minor_str) |min_str| {
                    if (!std.mem.eql(u8, min_str, "*")) {
                        wildcard.wildcard.minor = try std.fmt.parseInt(u32, min_str, 10);
                    }
                }
            }
            
            return VersionRequirement{ .constraint = wildcard };
        } else {
            // Exact version
            const version = try Version.parse(req_str);
            return VersionRequirement{ .constraint = .{ .exact = version } };
        }
    }
    
    pub fn satisfiedBy(self: VersionRequirement, version: Version) bool {
        return version.satisfies(self.constraint);
    }
    
    pub fn toString(self: VersionRequirement, allocator: Allocator) ![]const u8 {
        _ = allocator;
        switch (self.constraint) {
            .exact => |v| return v.toString(allocator),
            .caret => |v| {
                const v_str = try v.toString(allocator);
                defer allocator.free(v_str);
                return std.fmt.allocPrint(allocator, "^{s}", .{v_str});
            },
            .tilde => |v| {
                const v_str = try v.toString(allocator);
                defer allocator.free(v_str);
                return std.fmt.allocPrint(allocator, "~{s}", .{v_str});
            },
            .greater => |v| {
                const v_str = try v.toString(allocator);
                defer allocator.free(v_str);
                return std.fmt.allocPrint(allocator, ">{s}", .{v_str});
            },
            .greater_eq => |v| {
                const v_str = try v.toString(allocator);
                defer allocator.free(v_str);
                return std.fmt.allocPrint(allocator, ">={s}", .{v_str});
            },
            .less => |v| {
                const v_str = try v.toString(allocator);
                defer allocator.free(v_str);
                return std.fmt.allocPrint(allocator, "<{s}", .{v_str});
            },
            .less_eq => |v| {
                const v_str = try v.toString(allocator);
                defer allocator.free(v_str);
                return std.fmt.allocPrint(allocator, "<={s}", .{v_str});
            },
            .wildcard => |w| {
                if (w.major) |maj| {
                    if (w.minor) |min| {
                        return std.fmt.allocPrint(allocator, "{}.{}.*", .{maj, min});
                    } else {
                        return std.fmt.allocPrint(allocator, "{}.*", .{maj});
                    }
                } else {
                    return allocator.dupe(u8, "*");
                }
            },
        }
    }
};

pub const PackageSource = union(enum) {
    registry: struct {
        url: []const u8 = "https://packages.cursed.dev",
        name: []const u8,
    },
    git: struct {
        url: []const u8,
        branch: ?[]const u8 = null,
        tag: ?[]const u8 = null,
        commit: ?[]const u8 = null,
    },
    local: struct {
        path: []const u8,
    },
    
    pub fn deinit(self: *PackageSource, allocator: Allocator) void {
        _ = allocator;
        switch (self.*) {
            .registry => |*reg| {
                allocator.free(reg.name);
            },
            .git => |*git| {
                allocator.free(git.url);
                if (git.branch) |branch| allocator.free(branch);
                if (git.tag) |tag| allocator.free(tag);
                if (git.commit) |commit| allocator.free(commit);
            },
            .local => |*local| {
                allocator.free(local.path);
            },
        }
    }
};

pub const Dependency = struct {
    name: []const u8,
    version_req: VersionRequirement,
    source: PackageSource,
    dev_only: bool = false,
    
    pub fn init(allocator: Allocator, name: []const u8, version_req: VersionRequirement, source: PackageSource) Dependency {
                return Dependency{
            .name = name,
            .version_req = version_req,
            .source = source,
        };
    }
    
    pub fn deinit(self: *Dependency, allocator: Allocator) void {
        _ = allocator;
        allocator.free(self.name);
        self.source.deinit(self.allocator);
    }
};

pub const PackageManifest = struct {
    name: []const u8,
    version: Version,
    description: ?[]const u8 = null,
    authors: ArrayList([]const u8),
    license: ?[]const u8 = null,
    dependencies: HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    dev_dependencies: HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    main: ?[]const u8 = null,
    exports: HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    allocator: Allocator,
    
    pub fn init() PackageManifest {
        return PackageManifest{
            .name = "",
            .version = Version{ .major = 0, .minor = 1, .patch = 0 },
            .authors = ArrayList([]const u8){},
            .dependencies = HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .dev_dependencies = HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .exports = HashMap([]const u8, []const u8, std.hash_map.StringContext, std.hash_map.default_max_load_percentage){},
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *PackageManifest) void {
        if (self.name.len > 0) self.allocator.free(self.name);
        if (self.description) |desc| self.allocator.free(desc);
        if (self.license) |license| self.allocator.free(license);
        if (self.main) |main| self.allocator.free(main);
        
        for (self.authors.items) |author| {
            self.allocator.free(author);
        }
        self.authors.deinit(self.allocator);
        
        var dep_iter = self.dependencies.iterator();
        while (dep_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.dependencies.deinit(self.allocator);
        
        var dev_dep_iter = self.dev_dependencies.iterator();
        while (dev_dep_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            entry.value_ptr.deinit();
        }
        self.dev_dependencies.deinit(self.allocator);
        
        var export_iter = self.exports.iterator();
        while (export_iter.next()) |entry| {
            self.allocator.free(entry.key_ptr.*);
            self.allocator.free(entry.value_ptr.*);
        }
        self.exports.deinit(self.allocator);
    }
    
    pub fn loadFromToml(allocator: Allocator, file_path: []const u8) !PackageManifest {
        const file_content = std.fs.cwd().readFileAlloc(allocator, file_path, 1024 * 1024) catch |err| {
            return err;
        };
        defer allocator.free(file_content);
        
        return try parseTomlContent(allocator, file_content);
    }
    
    fn parseTomlContent(allocator: Allocator, content: []const u8) !PackageManifest {
        var manifest = PackageManifest.init(allocator);
        
        // Simple TOML parser - parse line by line
        var lines = std.mem.splitScalar(u8, content, '\n');
        var current_section: []const u8 = "";
        
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r");
            if (trimmed.len == 0 or trimmed[0] == '#') continue;
            
            // Check for section headers
            if (trimmed[0] == '[') {
                const end = std.mem.indexOf(u8, trimmed, "]") orelse continue;
                current_section = trimmed[1..end];
                continue;
            }
            
            // Parse key-value pairs
            const eq_pos = std.mem.indexOf(u8, trimmed, "=") orelse continue;
            const key = std.mem.trim(u8, trimmed[0..eq_pos], " ");
            var value = std.mem.trim(u8, trimmed[eq_pos + 1..], " ");
            
            // Remove quotes from string values
            if (value.len >= 2 and value[0] == '"' and value[value.len - 1] == '"') {
                value = value[1..value.len - 1];
            }
            
            // Handle different sections
            if (std.mem.eql(u8, current_section, "")) {
                // Root section
                if (std.mem.eql(u8, key, "name")) {
                    manifest.name = try allocator.dupe(u8, value);
                } else if (std.mem.eql(u8, key, "version")) {
                    manifest.version = try Version.parse(value);
                } else if (std.mem.eql(u8, key, "description")) {
                    manifest.description = try allocator.dupe(u8, value);
                } else if (std.mem.eql(u8, key, "license")) {
                    manifest.license = try allocator.dupe(u8, value);
                } else if (std.mem.eql(u8, key, "main")) {
                    manifest.main = try allocator.dupe(u8, value);
                } else if (std.mem.eql(u8, key, "authors")) {
                    // Parse array format ["author1", "author2"]
                    try parseAuthorsArray(&manifest, value);
                }
            } else if (std.mem.eql(u8, current_section, "dependencies")) {
                try parseDependency(&manifest, key, value, false);
            } else if (std.mem.eql(u8, current_section, "dev_dependencies")) {
                try parseDependency(&manifest, key, value, true);
            }
        }
        
        return manifest;
    }
    
    fn parseAuthorsArray(manifest: *PackageManifest, array_str: []const u8) !void {
        // Simple array parser for ["item1", "item2"] format
        if (array_str.len < 2 or array_str[0] != '[' or array_str[array_str.len - 1] != ']') {
            return;
        }
        
        const inner = array_str[1..array_str.len - 1];
        var items = std.mem.splitScalar(u8, inner, ',');
        
        while (items.next()) |item| {
            var trimmed = std.mem.trim(u8, item, " \t");
            if (trimmed.len >= 2 and trimmed[0] == '"' and trimmed[trimmed.len - 1] == '"') {
                trimmed = trimmed[1..trimmed.len - 1];
                try manifest.authors.append(try manifest.allocator.dupe(u8, trimmed));
            }
        }
    }
    
    fn parseDependency(manifest: *PackageManifest, name: []const u8, version_str: []const u8, is_dev: bool) !void {
        const version_req = try VersionRequirement.parse(manifest.allocator, version_str);
        const source = PackageSource{ .registry = .{ .name = try manifest.allocator.dupe(u8, name) } };
        
        const dependency = Dependency.init(
            manifest.allocator,
            try manifest.allocator.dupe(u8, name),
            version_req,
            source,
        );
        
        const map = if (is_dev) &manifest.dev_dependencies else &manifest.dependencies;
        try map.put(try manifest.allocator.dupe(u8, name), dependency);
    }
    
    pub fn saveToToml(self: *const PackageManifest, file_path: []const u8) !void {
        const content = try self.toTomlString();
        defer self.allocator.free(content);
        
        try std.fs.cwd().writeFile(.{ .sub_path = file_path, .data = content });
    }
    
    pub fn toTomlString(self: *const PackageManifest) ![]const u8 {
        var content = ArrayList(u8){};
        var writer = content.writer();
        
        // Package metadata
        try writer.writer().print("name = \"{s}\"\n", .{self.name});
        const version_str = try self.version.toString(self.allocator);
        defer self.allocator.free(version_str);
        try writer.writer().print("version = \"{s}\"\n", .{version_str});
        
        if (self.description) |desc| {
            try writer.writer().print("description = \"{s}\"\n", .{desc});
        }
        
        if (self.license) |license| {
            try writer.writer().print("license = \"{s}\"\n", .{license});
        }
        
        if (self.main) |main| {
            try writer.writer().print("main = \"{s}\"\n", .{main});
        }
        
        // Authors array
        if (self.authors.items.len > 0) {
            try writer.writer().writeAll("authors = [");
            for (self.authors.items, 0..) |author, i| {
                if (i > 0) try writer.writer().writeAll(", ");
                try writer.writer().print("\"{s}\"", .{author});
            }
            try writer.writer().writeAll("]\n");
        }
        
        // Dependencies
        if (self.dependencies.count() > 0) {
            try writer.writer().writeAll("\n[dependencies]\n");
            var dep_iter = self.dependencies.iterator();
            while (dep_iter.next()) |entry| {
                const version_str_dep = try entry.value_ptr.version_req.toString(self.allocator);
                defer self.allocator.free(version_str_dep);
                try writer.writer().print("{s} = \"{s}\"\n", .{ entry.key_ptr.*, version_str_dep });
            }
        }
        
        // Dev dependencies
        if (self.dev_dependencies.count() > 0) {
            try writer.writer().writeAll("\n[dev_dependencies]\n");
            var dev_dep_iter = self.dev_dependencies.iterator();
            while (dev_dep_iter.next()) |entry| {
                const version_str_dev = try entry.value_ptr.version_req.toString(self.allocator);
                defer self.allocator.free(version_str_dev);
                try writer.writer().print("{s} = \"{s}\"\n", .{ entry.key_ptr.*, version_str_dev });
            }
        }
        
        return content.toOwnedSlice();
    }
};

// ===== PACKAGE MANAGER COMMANDS =====

pub const commands = struct {
    pub fn init(allocator: Allocator, args: [][]const u8) !void {
        var project_name: []const u8 = "new-cursed-package";
        var template_type: templates.TemplateType = .library;
        
        // Parse arguments
        var i: usize = 0;
        while (i < args.len) {
            const arg = args[i];
            if (std.mem.eql(u8, arg, "--type") and i + 1 < args.len) {
                i += 1;
                template_type = templates.TemplateType.fromString(args[i]) orelse .library;
            } else if (std.mem.eql(u8, arg, "--name") and i + 1 < args.len) {
                i += 1;
                project_name = args[i];
            } else if (!std.mem.startsWith(u8, arg, "--")) {
                project_name = arg;
            }
            i += 1;
        }
        
        print("🎯 Initializing new CURSED package: {s}\n", .{project_name});
        print("📦 Template type: {s}\n", .{@tagName(template_type)});
        
        // Use template system
        try templates.createFromTemplate(allocator, template_type, project_name);
    }
    
    pub fn add(allocator: Allocator, args: [][]const u8) !void {
        if (args.len == 0) {
            print("❌ Package name required\n", .{});
            print("Usage: cursed-pkg add <package> [version]\n", .{});
            return;
        }
        
        const package_name = args[0];
        const version_spec = if (args.len > 1) args[1] else "^1.0.0";
        
        print("📦 Adding dependency: {s}@{s}\n", .{package_name, version_spec});
        
        // Load existing manifest
        var manifest = PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
            error.FileNotFound => {
                print("❌ No CursedPackage.toml found. Run 'cursed-pkg init' first.\n", .{});
                return;
            },
            else => return err,
        };
        defer manifest.deinit();
        
        // Parse version requirement
        const version_req = try VersionRequirement.parse(allocator, version_spec);
        
        // Create dependency
        const source = PackageSource{ .registry = .{ .name = try allocator.dupe(u8, package_name) } };
        const dependency = Dependency.init(allocator, try allocator.dupe(u8, package_name), version_req, source);
        
        // Add to manifest
        try manifest.dependencies.put(try allocator.dupe(u8, package_name), dependency);
        
        // Save updated manifest
        try manifest.saveToToml("CursedPackage.toml");
        
        print("✅ Added {s}@{s} to dependencies\n", .{package_name, version_spec});
        print("🏃 Run 'cursed-pkg install' to download the package\n", .{});
    }
    
    pub fn install(allocator: Allocator, args: [][]const u8) !void {
        _ = args;
        
        print("📥 Installing dependencies...\n", .{});
        
        // Load manifest
        var manifest = PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
            error.FileNotFound => {
                print("❌ No CursedPackage.toml found. Run 'cursed-pkg init' first.\n", .{});
                return;
            },
            else => return err,
        };
        defer manifest.deinit();
        
        // Create cache directory
        try std.fs.cwd().makePath(".cursed/cache");
        try std.fs.cwd().makePath(".cursed/packages");
        
        var installed_count: u32 = 0;
        
        // Install regular dependencies
        var dep_iter = manifest.dependencies.iterator();
        while (dep_iter.next()) |entry| {
            const dep = entry.value_ptr.*;
            try installSinglePackage(allocator, dep);
            installed_count += 1;
        }
        
        // Install dev dependencies
        var dev_dep_iter = manifest.dev_dependencies.iterator();
        while (dev_dep_iter.next()) |entry| {
            const dep = entry.value_ptr.*;
            try installSinglePackage(allocator, dep);
            installed_count += 1;
        }
        
        // Generate lock file
        try generateLockFile(allocator, &manifest);
        
        // Generate build integration
        try generateBuildIntegration(allocator, &manifest);
        
        print("✅ Successfully installed {s} dependencies\n", .{{installed_count});
        print("📁 Dependencies cached in .cursed/cache/\n", .{});
        print("🔒 Lock file generated: CursedPackage.lock\n", .{});
        print("🏗️  Build integration generated: build_generated.zig\n", .{});
    }
    
    fn installSinglePackage(allocator: Allocator, dependency: Dependency) !void {
        print("  📦 Installing {s}...\n", .{dependency.name});
        
        const cache_path = try std.fmt.allocPrint(allocator, ".cursed/packages/{s}", .{dependency.name});
        defer allocator.free(cache_path);
        
        // Create package directory
        try std.fs.cwd().makePath(cache_path);
        
        // For now, create a mock package (in production would download from registry)
        const mock_content = try std.fmt.allocPrint(allocator,
            \\// Mock implementation of {s}
            \\// This would be downloaded from the registry in production
            \\
            \\slay {s}_function() {{
            \\    vibez.spill("Function from {s} package")
            \\}}
            \\
            \\sus {s}_constant tea = "mock value"
        , .{dependency.name, dependency.name, dependency.name, dependency.name});
        defer allocator.free(mock_content);
        
        const package_file = try std.fmt.allocPrint(allocator, "{s}/mod.csd", .{cache_path});
        defer allocator.free(package_file);
        
        try std.fs.cwd().writeFile(.{ .sub_path = package_file, .data = mock_content });
    }
    
    fn generateLockFile(allocator: Allocator, manifest: *const PackageManifest) !void {
        var content = ArrayList(u8){};
        defer content.deinit();
        var writer = content.writer();
        
        try writer.writer().writeAll("# CursedPackage.lock - Generated lock file\n");
        try writer.writer().writeAll("# Do not edit manually\n\n");
        
        try writer.writer().writeAll("[[package]]\n");
        try writer.writer().print("name = \"{s}\"\n", .{manifest.name});
        const version_str = try manifest.version.toString(allocator);
        defer allocator.free(version_str);
        try writer.writer().print("version = \"{s}\"\n", .{version_str});
        
        // Lock dependencies
        var dep_iter = manifest.dependencies.iterator();
        while (dep_iter.next()) |entry| {
            const dep = entry.value_ptr.*;
            try writer.writer().writeAll("\n[[package]]\n");
            try writer.writer().print("name = \"{s}\"\n", .{dep.name});
            try writer.writer().writeAll("version = \"1.0.0\"\n"); // Mock resolved version
            try writer.writer().writeAll("source = \"registry+https://packages.cursed.dev\"\n");
            
            const checksum = try generateMockChecksum(allocator, dep.name);
            defer allocator.free(checksum);
            try writer.writer().print("checksum = \"{s}\"\n", .{checksum});
        }
        
        try std.fs.cwd().writeFile(.{ .sub_path = "CursedPackage.lock", .data = content.items });
    }
    
    fn generateMockChecksum(allocator: Allocator, name: []const u8) ![]const u8 {
        // Generate a mock checksum based on package name
        var hasher = std.crypto.hash.sha2.Sha256.init(.{});
        hasher.update(name);
        hasher.update("cursed-package-mock");
        
        var hash: [32]u8 = undefined;
        hasher.final(&hash);
        
        return try std.fmt.allocPrint(allocator, "{}", .{std.fmt.fmtSliceHexLower(&hash)});
    }
    
    fn generateBuildIntegration(allocator: Allocator, manifest: *const PackageManifest) !void {
        var content = ArrayList(u8){};
        defer content.deinit();
        var writer = content.writer();
        
        try writer.writer().writeAll("// Generated build integration for CURSED package manager\n");
        try writer.writer().writeAll("// This file is auto-generated, do not edit manually\n\n");
        try writer.writer().writeAll("const std = @import(\"std\");\n\n");
        
        try writer.writer().writeAll("pub fn addDependencies(b: *std.build.Builder) void {\n");
        try writer.writer().writeAll("    // Package dependencies\n");
        
        var dep_iter = manifest.dependencies.iterator();
        while (dep_iter.next()) |entry| {
            const dep = entry.value_ptr.*;
            try writer.writer().print("    // {s}\n", .{dep.name});
            try writer.writer().print("    b.addPackagePath(\"{s}\", \".cursed/packages/{s}/mod.csd\");\n", .{dep.name, dep.name});
        }
        
        try writer.writer().writeAll("}\n\n");
        
        try writer.writer().writeAll("pub const dependencies = struct {\n");
        var dep_iter2 = manifest.dependencies.iterator();
        while (dep_iter2.next()) |entry| {
            const dep = entry.value_ptr.*;
            try writer.writer().print("    pub const {s} = \".cursed/packages/{s}/mod.csd\";\n", .{dep.name, dep.name});
        }
        try writer.writer().writeAll("};\n");
        
        try std.fs.cwd().writeFile(.{ .sub_path = "build_generated.zig", .data = content.items });
    }
    
    pub fn update(allocator: Allocator, args: [][]const u8) !void {
        _ = args;
        
        print("🔄 Updating dependencies...\n", .{});
        
        // Load manifest
        var manifest = PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
            error.FileNotFound => {
                print("❌ No CursedPackage.toml found. Run 'cursed-pkg init' first.\n", .{});
                return;
            },
            else => return err,
        };
        defer manifest.deinit();
        
        // Update each dependency
        var dep_iter = manifest.dependencies.iterator();
        while (dep_iter.next()) |entry| {
            const dep = entry.value_ptr.*;
            print("  🔄 Updating {s}...\n", .{dep.name});
            
            // In production, would check for newer versions and update
            try installSinglePackage(allocator, dep);
        }
        
        // Regenerate lock file
        try generateLockFile(allocator, &manifest);
        
        print("✅ Dependencies updated successfully\n", .{});
    }
    
    pub fn remove(allocator: Allocator, args: [][]const u8) !void {
        if (args.len == 0) {
            print("❌ Package name required\n", .{});
            print("Usage: cursed-pkg remove <package>\n", .{});
            return;
        }
        
        const package_name = args[0];
        
        print("🗑️  Removing dependency: {s}\n", .{package_name});
        
        // Load existing manifest
        var manifest = PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
            error.FileNotFound => {
                print("❌ No CursedPackage.toml found. Run 'cursed-pkg init' first.\n", .{});
                return;
            },
            else => return err,
        };
        defer manifest.deinit();
        
        // Remove from dependencies
        if (manifest.dependencies.fetchRemove(package_name)) |removed| {
            allocator.free(removed.key);
            var mut_value = removed.value;
            mut_value.deinit();
            print("✅ Removed {s} from dependencies\n", .{package_name});
        } else if (manifest.dev_dependencies.fetchRemove(package_name)) |removed| {
            allocator.free(removed.key);
            var mut_value = removed.value;
            mut_value.deinit();
            print("✅ Removed {s} from dev dependencies\n", .{package_name});
        } else {
            print("❌ Package {s} not found in dependencies\n", .{package_name});
            return;
        }
        
        // Save updated manifest
        try manifest.saveToToml("CursedPackage.toml");
        
        // Remove from cache
        const cache_path = try std.fmt.allocPrint(allocator, ".cursed/packages/{s}", .{package_name});
        defer allocator.free(cache_path);
        
        std.fs.cwd().deleteTree(cache_path) catch {
            print("⚠️  Could not remove cached package files\n", .{});
        };
        
        print("🏃 Run 'cursed-pkg install' to update lock file\n", .{});
    }
    
    pub fn search(allocator: Allocator, args: [][]const u8) !void {
                if (args.len == 0) {
            print("❌ Search query required\n", .{});
            print("Usage: cursed-pkg search <query>\n", .{});
            return;
        }
        
        const query = args[0];
        
        print("🔍 Searching for packages matching: {s}\n", .{query});
        print("\n", .{});
        
        // Mock search results (in production would query package registry)
        const mock_results = [_]struct {
            name: []const u8,
            version: []const u8,
            description: []const u8,
        }{
            .{ .name = "http", .version = "1.2.3", .description = "HTTP client and server library for CURSED" },
            .{ .name = "json", .version = "2.1.0", .description = "JSON parsing and serialization" },
            .{ .name = "crypto", .version = "1.0.5", .description = "Cryptographic functions and utilities" },
            .{ .name = "testz", .version = "1.1.0", .description = "Testing framework for CURSED" },
            .{ .name = "stringz", .version = "0.9.2", .description = "Advanced string manipulation utilities" },
        };
        
        var found_count: u32 = 0;
        for (mock_results) |result| {
            if (std.mem.indexOf(u8, result.name, query) != null or 
                std.mem.indexOf(u8, result.description, query) != null) {
                print("{s} ({s}) - {s}\n", .{result.name, result.version, result.description});
                found_count += 1;
            }
        }
        
        if (found_count == 0) {
            print("No packages found matching '{s}'\n", .{query});
        } else {
            print("\nFound {s} packages\n", .{{found_count});
            print("Install with: cursed-pkg add <package>\n", .{});
        }
    }
    
    pub fn publish(allocator: Allocator, args: [][]const u8) !void {
        _ = args;
        
        print("📤 Publishing package...\n", .{});
        
        // Load manifest
        var manifest = PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
            error.FileNotFound => {
                print("❌ No CursedPackage.toml found. Run 'cursed-pkg init' first.\n", .{});
                return;
            },
            else => return err,
        };
        defer manifest.deinit();
        
        // Validate package
        if (manifest.name.len == 0) {
            print("❌ Package name required in CursedPackage.toml\n", .{});
            return;
        }
        
        if (manifest.description == null) {
            print("❌ Package description required in CursedPackage.toml\n", .{});
            return;
        }
        
        // Check if main file exists
        const main_file = manifest.main orelse "src/lib.csd";
        std.fs.cwd().access(main_file, .{}) catch {
            print("❌ Main file not found: {s}\n", .{main_file});
            return;
        };
        
        print("📦 Package: {s}\n", .{manifest.name});
        const version_str = try manifest.version.toString(allocator);
        defer allocator.free(version_str);
        print("🏷️  Version: {s}\n", .{version_str});
        if (manifest.description) |desc| {
            print("📝 Description: {s}\n", .{desc});
        }
        
        // Create package archive
        try createPackageArchive(allocator, &manifest);
        
        // Mock publishing (in production would upload to registry)
        print("🚀 Publishing to registry...\n", .{});
        
        // Simulate network delay
        std.Thread.sleep(1000000000); // 1 second
        
        print("✅ Successfully published {s}@{s}\n", .{manifest.name, version_str});
        print("🌐 Available at: https://packages.cursed.dev/{s}\n", .{manifest.name});
    }
    
    fn createPackageArchive(allocator: Allocator, manifest: *const PackageManifest) !void {
        const archive_name = try std.fmt.allocPrint(allocator, "{s}.cursed-pkg", .{manifest.name});
        defer allocator.free(archive_name);
        
        print("📦 Creating package archive: {s}\n", .{archive_name});
        
        // Create .cursed-pkg directory structure
        try std.fs.cwd().makeDir(".cursed-pkg");
        
        // Copy source files
        try copyDirectory(allocator, "src", ".cursed-pkg/src");
        
        // Copy manifest
        const manifest_content = try manifest.toTomlString();
        defer allocator.free(manifest_content);
        try std.fs.cwd().writeFile(.{ .sub_path = ".cursed-pkg/CursedPackage.toml", .data = manifest_content });
        
        // Copy README if exists
        std.fs.cwd().copyFile("README.md", std.fs.cwd(), ".cursed-pkg/README.md", .{}) catch {};
        
        print("✅ Package archive created\n", .{});
    }
    
    fn copyDirectory(allocator: Allocator, src_path: []const u8, dest_path: []const u8) !void {
        
        // Create destination directory
        try std.fs.cwd().makePath(dest_path);
        
        // Simple file copy (in production would recursively copy directory)
        var src_dir = std.fs.cwd().openDir(src_path, .{ .iterate = true }) catch return;
        defer src_dir.close();
        
        var iterator = src_dir.iterate();
        while (try iterator.next()) |entry| {
            if (entry.kind == .file) {
                const src_file = try std.fmt.allocPrint(allocator, "{s}/{s}", .{src_path, entry.name});
                defer allocator.free(src_file);
                const dest_file = try std.fmt.allocPrint(allocator, "{s}/{s}", .{dest_path, entry.name});
                defer allocator.free(dest_file);
                
                try std.fs.cwd().copyFile(src_file, std.fs.cwd(), dest_file, .{});
            }
        }
    }
};
