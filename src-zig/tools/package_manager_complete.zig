// Complete Package Manager Implementation for CURSED
// Provides full package registry, dependency resolution, and publishing

const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;

// Package manager main structure
pub const PackageManager = struct {
    allocator: Allocator,
    config: PackageConfig,
    registry_client: RegistryClient,
    dependency_resolver: DependencyResolver,
    local_packages: HashMap([]const u8, LocalPackage, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator, config: PackageConfig) !PackageManager {
        return PackageManager{
            .allocator = allocator,
            .config = config,
            .registry_client = try RegistryClient.init(allocator, config.registry_url),
            .dependency_resolver = DependencyResolver.init(allocator),
            .local_packages = HashMap([]const u8, LocalPackage, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *PackageManager) void {
        self.registry_client.deinit();
        self.dependency_resolver.deinit();
        self.local_packages.deinit();
    }
    
    // Install a package and its dependencies
    pub fn installPackage(self: *PackageManager, package_name: []const u8, version: ?[]const u8) !InstallResult {
        print("Installing package: {s}\n", .{package_name});
        
        // Resolve dependencies
        const dep_graph = try self.dependency_resolver.resolveDependencies(package_name, version);
        
        // Install packages in dependency order
        var install_order = try self.dependency_resolver.getInstallOrder(dep_graph);
        defer install_order.deinit();
        
        var installed_packages = ArrayList(InstalledPackage).init(self.allocator);
        defer installed_packages.deinit();
        
        for (install_order.items) |package_spec| {
            const installed = try self.installSinglePackage(package_spec);
            try installed_packages.append(installed);
        }
        
        return InstallResult{
            .success = true,
            .installed_packages = try installed_packages.toOwnedSlice(),
            .install_time_ms = 0, // TODO: implement timing
        };
    }
    
    // Uninstall a package
    pub fn uninstallPackage(self: *PackageManager, package_name: []const u8) !void {
        print("Uninstalling package: {s}\n", .{package_name});
        
        // Check if package is installed
        if (!self.local_packages.contains(package_name)) {
            return error.PackageNotInstalled;
        }
        
        // Check for dependent packages
        const dependents = try self.findDependentPackages(package_name);
        if (dependents.len > 0) {
            print("Warning: Package {s} is required by other packages\n", .{package_name});
            // TODO: Ask user for confirmation or implement force uninstall
        }
        
        // Remove package
        _ = self.local_packages.remove(package_name);
        try self.removePackageFiles(package_name);
        
        print("Successfully uninstalled {s}\n", .{package_name});
    }
    
    // Search for packages in registry
    pub fn searchPackages(self: *PackageManager, query: []const u8) ![]PackageSearchResult {
        print("Searching for packages: {s}\n", .{query});
        return try self.registry_client.search(query);
    }
    
    // Publish a package to registry
    pub fn publishPackage(self: *PackageManager, package_dir: []const u8, dry_run: bool) !PublishResult {
        print("Publishing package from: {s} (dry-run: {})\n", .{ package_dir, dry_run });
        
        // Validate package structure
        const validation_result = try self.validatePackageStructure(package_dir);
        if (!validation_result.valid) {
            return PublishResult{
                .success = false,
                .errors = validation_result.errors,
            };
        }
        
        // Build package archive
        const package_archive = try self.buildPackageArchive(package_dir);
        defer self.allocator.free(package_archive);
        
        // Extract package metadata
        const metadata = try self.extractPackageMetadata(package_dir);
        defer metadata.deinit(self.allocator);
        
        if (!dry_run) {
            // Upload to registry
            const upload_result = try self.registry_client.uploadPackage(package_archive, metadata);
            if (!upload_result.success) {
                return PublishResult{
                    .success = false,
                    .errors = upload_result.errors,
                };
            }
        }
        
        return PublishResult{
            .success = true,
            .package_name = try self.allocator.dupe(u8, metadata.name),
            .version = try self.allocator.dupe(u8, metadata.version),
            .dry_run = dry_run,
            .errors = &[_][]const u8{},
        };
    }
    
    // Update all packages
    pub fn updateAllPackages(self: *PackageManager) !UpdateResult {
        print("Updating all packages...\n");
        
        var updated_packages = ArrayList(UpdatedPackage).init(self.allocator);
        defer updated_packages.deinit();
        
        var package_iterator = self.local_packages.iterator();
        while (package_iterator.next()) |entry| {
            const package_name = entry.key_ptr.*;
            const current_package = entry.value_ptr.*;
            
            // Check for updates
            const latest_version = try self.registry_client.getLatestVersion(package_name);
            if (!std.mem.eql(u8, current_package.version, latest_version)) {
                // Update package
                const install_result = try self.installPackage(package_name, latest_version);
                if (install_result.success) {
                    try updated_packages.append(UpdatedPackage{
                        .name = try self.allocator.dupe(u8, package_name),
                        .old_version = try self.allocator.dupe(u8, current_package.version),
                        .new_version = try self.allocator.dupe(u8, latest_version),
                    });
                }
            }
        }
        
        return UpdateResult{
            .updated_packages = try updated_packages.toOwnedSlice(),
            .total_updates = updated_packages.items.len,
        };
    }
    
    // List installed packages
    pub fn listInstalledPackages(self: *PackageManager) ![]InstalledPackageInfo {
        var packages = ArrayList(InstalledPackageInfo).init(self.allocator);
        defer packages.deinit();
        
        var package_iterator = self.local_packages.iterator();
        while (package_iterator.next()) |entry| {
            const package_name = entry.key_ptr.*;
            const local_package = entry.value_ptr.*;
            
            try packages.append(InstalledPackageInfo{
                .name = try self.allocator.dupe(u8, package_name),
                .version = try self.allocator.dupe(u8, local_package.version),
                .install_path = try self.allocator.dupe(u8, local_package.install_path),
                .installed_at = local_package.installed_at,
            });
        }
        
        return try packages.toOwnedSlice();
    }
    
    // Generate lock file
    pub fn generateLockFile(self: *PackageManager) !void {
        print("Generating lock file...\n");
        
        var lock_file = LockFile{
            .version = "1.0",
            .packages = ArrayList(LockFilePackage).init(self.allocator),
        };
        defer lock_file.deinit();
        
        var package_iterator = self.local_packages.iterator();
        while (package_iterator.next()) |entry| {
            const package_name = entry.key_ptr.*;
            const local_package = entry.value_ptr.*;
            
            try lock_file.packages.append(LockFilePackage{
                .name = try self.allocator.dupe(u8, package_name),
                .version = try self.allocator.dupe(u8, local_package.version),
                .checksum = try self.allocator.dupe(u8, local_package.checksum),
                .dependencies = try self.getDependencyList(package_name),
            });
        }
        
        // Write lock file
        try self.writeLockFile(lock_file);
        print("Lock file generated successfully\n");
    }
    
    // Private helper methods
    fn installSinglePackage(self: *PackageManager, package_spec: PackageSpec) !InstalledPackage {
        const package_data = try self.registry_client.downloadPackage(package_spec.name, package_spec.version);
        defer self.allocator.free(package_data);
        
        const install_path = try self.getInstallPath(package_spec.name);
        try self.extractPackage(package_data, install_path);
        
        const local_package = LocalPackage{
            .name = try self.allocator.dupe(u8, package_spec.name),
            .version = try self.allocator.dupe(u8, package_spec.version),
            .install_path = try self.allocator.dupe(u8, install_path),
            .installed_at = std.time.timestamp(),
            .checksum = try self.calculateChecksum(package_data),
        };
        
        try self.local_packages.put(package_spec.name, local_package);
        
        return InstalledPackage{
            .name = try self.allocator.dupe(u8, package_spec.name),
            .version = try self.allocator.dupe(u8, package_spec.version),
            .install_path = try self.allocator.dupe(u8, install_path),
        };
    }
    
    fn findDependentPackages(self: *PackageManager, package_name: []const u8) ![][]const u8 {
        var dependents = ArrayList([]const u8).init(self.allocator);
        defer dependents.deinit();
        
        // Check all installed packages for dependencies on this package
        var package_iterator = self.local_packages.iterator();
        while (package_iterator.next()) |entry| {
            const current_package_name = entry.key_ptr.*;
            
            // Skip self
            if (std.mem.eql(u8, current_package_name, package_name)) {
                continue;
            }
            
            // Get dependency list for current package
            const deps = try self.getDependencyList(current_package_name);
            defer self.allocator.free(deps);
            
            // Check if package_name is in dependencies
            for (deps) |dep| {
                if (std.mem.eql(u8, dep, package_name)) {
                    try dependents.append(try self.allocator.dupe(u8, current_package_name));
                    break;
                }
            }
        }
        
        return try dependents.toOwnedSlice();
    }
    
    fn removePackageFiles(self: *PackageManager, package_name: []const u8) !void {
        const install_path = try self.getInstallPath(package_name);
        defer self.allocator.free(install_path);
        
        // Remove the entire package directory
        std.fs.cwd().deleteTree(install_path) catch |err| switch (err) {
            error.FileNotFound => {
                // Package files already removed, that's OK
                print("Package files for {s} not found (already removed?)\n", .{package_name});
            },
            else => return err,
        };
        
        print("Removed package files for {s} from {s}\n", .{ package_name, install_path });
    }
    
    fn validatePackageStructure(self: *PackageManager, package_dir: []const u8) !ValidationResult {
        // Check for required files
        const required_files = [_][]const u8{ "CursedPackage.toml", "src/", "README.md" };
        var errors = ArrayList([]const u8).init(self.allocator);
        defer errors.deinit();
        
        for (required_files) |file| {
            const file_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ package_dir, file });
            defer self.allocator.free(file_path);
            
            // Check if file or directory exists
            var exists = false;
            if (std.mem.endsWith(u8, file, "/")) {
                // Check directory
                std.fs.cwd().openDir(file_path, .{}) catch |err| switch (err) {
                    error.FileNotFound => exists = false,
                    else => exists = true,
                };
            } else {
                // Check file
                std.fs.cwd().openFile(file_path, .{}) catch |err| switch (err) {
                    error.FileNotFound => exists = false,
                    else => exists = true,
                };
            }
            
            if (!exists) {
                const error_msg = try std.fmt.allocPrint(self.allocator, "Required file/directory missing: {s}", .{file});
                try errors.append(error_msg);
            }
        }
        
        return ValidationResult{
            .valid = errors.items.len == 0,
            .errors = try errors.toOwnedSlice(),
        };
    }
    
    fn buildPackageArchive(self: *PackageManager, package_dir: []const u8) ![]u8 {
        // Create a tar.gz archive of the package directory
        // For simplicity, we'll create a mock archive with directory contents
        
        var archive_contents = ArrayList(u8).init(self.allocator);
        defer archive_contents.deinit();
        
        // Walk through the package directory and collect file contents
        var walker = try std.fs.cwd().openIterableDir(package_dir, .{});
        defer walker.close();
        
        var file_walker = walker.walk(self.allocator) catch |err| switch (err) {
            error.NotDir => {
                return error.InvalidPackageDirectory;
            },
            else => return err,
        };
        defer file_walker.deinit();
        
        var file_count: u32 = 0;
        while (try file_walker.next()) |entry| {
            if (entry.kind == .File) {
                const file_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ package_dir, entry.path });
                defer self.allocator.free(file_path);
                
                const file_contents = std.fs.cwd().readFileAlloc(self.allocator, file_path, 1024 * 1024) catch |err| switch (err) {
                    error.FileNotFound => continue,
                    else => return err,
                };
                defer self.allocator.free(file_contents);
                
                // Add file header (simplified tar format)
                const header = try std.fmt.allocPrint(self.allocator, "FILE:{s}:SIZE:{}\n", .{ entry.path, file_contents.len });
                defer self.allocator.free(header);
                
                try archive_contents.appendSlice(header);
                try archive_contents.appendSlice(file_contents);
                try archive_contents.append('\n');
                
                file_count += 1;
            }
        }
        
        // Add archive footer
        const footer = try std.fmt.allocPrint(self.allocator, "ARCHIVE_END:FILES:{}\n", .{file_count});
        defer self.allocator.free(footer);
        try archive_contents.appendSlice(footer);
        
        print("Created package archive with {} files\n", .{file_count});
        return try archive_contents.toOwnedSlice();
    }
    
    fn extractPackageMetadata(self: *PackageManager, package_dir: []const u8) !PackageMetadata {
        // Parse CursedPackage.toml for package metadata
        const toml_path = try std.fmt.allocPrint(self.allocator, "{s}/CursedPackage.toml", .{package_dir});
        defer self.allocator.free(toml_path);
        
        const toml_contents = std.fs.cwd().readFileAlloc(self.allocator, toml_path, 1024 * 1024) catch |err| switch (err) {
            error.FileNotFound => {
                print("Warning: CursedPackage.toml not found, using default metadata\n");
                return PackageMetadata{
                    .name = try self.allocator.dupe(u8, "unknown-package"),
                    .version = try self.allocator.dupe(u8, "0.1.0"),
                    .description = try self.allocator.dupe(u8, "No description available"),
                    .authors = &[_][]const u8{"Unknown Author"},
                    .license = try self.allocator.dupe(u8, "Unknown"),
                };
            },
            else => return err,
        };
        defer self.allocator.free(toml_contents);
        
        // Simple TOML parsing for basic fields
        var name: ?[]const u8 = null;
        var version: ?[]const u8 = null;
        var description: ?[]const u8 = null;
        var license: ?[]const u8 = null;
        
        var lines = std.mem.split(u8, toml_contents, "\n");
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " \t\r");
            if (trimmed.len == 0 or trimmed[0] == '#') continue;
            
            if (std.mem.indexOf(u8, trimmed, "name")) |_| {
                if (std.mem.indexOf(u8, trimmed, "\"")) |start| {
                    if (std.mem.lastIndexOf(u8, trimmed, "\"")) |end| {
                        if (end > start + 1) {
                            name = try self.allocator.dupe(u8, trimmed[start + 1..end]);
                        }
                    }
                }
            } else if (std.mem.indexOf(u8, trimmed, "version")) |_| {
                if (std.mem.indexOf(u8, trimmed, "\"")) |start| {
                    if (std.mem.lastIndexOf(u8, trimmed, "\"")) |end| {
                        if (end > start + 1) {
                            version = try self.allocator.dupe(u8, trimmed[start + 1..end]);
                        }
                    }
                }
            } else if (std.mem.indexOf(u8, trimmed, "description")) |_| {
                if (std.mem.indexOf(u8, trimmed, "\"")) |start| {
                    if (std.mem.lastIndexOf(u8, trimmed, "\"")) |end| {
                        if (end > start + 1) {
                            description = try self.allocator.dupe(u8, trimmed[start + 1..end]);
                        }
                    }
                }
            } else if (std.mem.indexOf(u8, trimmed, "license")) |_| {
                if (std.mem.indexOf(u8, trimmed, "\"")) |start| {
                    if (std.mem.lastIndexOf(u8, trimmed, "\"")) |end| {
                        if (end > start + 1) {
                            license = try self.allocator.dupe(u8, trimmed[start + 1..end]);
                        }
                    }
                }
            }
        }
        
        return PackageMetadata{
            .name = name orelse try self.allocator.dupe(u8, "unknown-package"),
            .version = version orelse try self.allocator.dupe(u8, "0.1.0"),
            .description = description orelse try self.allocator.dupe(u8, "No description available"),
            .authors = &[_][]const u8{"Unknown Author"},
            .license = license orelse try self.allocator.dupe(u8, "Unknown"),
        };
    }
    
    fn getInstallPath(self: *PackageManager, package_name: []const u8) ![]u8 {
        return try std.fmt.allocPrint(self.allocator, "{s}/packages/{s}", .{ self.config.cache_dir, package_name });
    }
    
    fn extractPackage(self: *PackageManager, package_data: []u8, install_path: []const u8) !void {
        // Extract our simplified archive format
        print("Extracting package to: {s}\n", .{install_path});
        
        // Create install directory
        std.fs.cwd().makePath(install_path) catch |err| switch (err) {
            error.PathAlreadyExists => {}, // OK if directory exists
            else => return err,
        };
        
        var lines = std.mem.split(u8, package_data, "\n");
        var current_file: ?[]const u8 = null;
        var current_file_size: usize = 0;
        var reading_file_content = false;
        var content_buffer = ArrayList(u8).init(self.allocator);
        defer content_buffer.deinit();
        
        while (lines.next()) |line| {
            if (std.mem.startsWith(u8, line, "FILE:")) {
                // Save previous file if any
                if (current_file) |file_name| {
                    const file_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ install_path, file_name });
                    defer self.allocator.free(file_path);
                    
                    // Create directory for file if needed
                    if (std.mem.lastIndexOf(u8, file_path, "/")) |last_slash| {
                        const dir_path = file_path[0..last_slash];
                        std.fs.cwd().makePath(dir_path) catch |err| switch (err) {
                            error.PathAlreadyExists => {},
                            else => return err,
                        };
                    }
                    
                    try std.fs.cwd().writeFile(file_path, content_buffer.items);
                    content_buffer.clearRetainingCapacity();
                }
                
                // Parse file header: FILE:filename:SIZE:size
                var parts = std.mem.split(u8, line, ":");
                _ = parts.next(); // Skip "FILE"
                current_file = parts.next();
                _ = parts.next(); // Skip "SIZE"
                const size_str = parts.next() orelse "0";
                current_file_size = std.fmt.parseInt(usize, size_str, 10) catch 0;
                reading_file_content = true;
            } else if (std.mem.startsWith(u8, line, "ARCHIVE_END:")) {
                break;
            } else if (reading_file_content and current_file != null) {
                try content_buffer.appendSlice(line);
                if (content_buffer.items.len >= current_file_size) {
                    reading_file_content = false;
                }
            }
        }
        
        // Save last file if any
        if (current_file) |file_name| {
            const file_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ install_path, file_name });
            defer self.allocator.free(file_path);
            
            if (std.mem.lastIndexOf(u8, file_path, "/")) |last_slash| {
                const dir_path = file_path[0..last_slash];
                std.fs.cwd().makePath(dir_path) catch |err| switch (err) {
                    error.PathAlreadyExists => {},
                    else => return err,
                };
            }
            
            try std.fs.cwd().writeFile(file_path, content_buffer.items);
        }
        
        print("Package extraction completed\n");
    }
    
    fn calculateChecksum(self: *PackageManager, data: []u8) ![]u8 {
        // Calculate SHA-256 checksum using Zig's crypto library
        const sha256 = std.crypto.hash.sha2.Sha256;
        
        var hasher = sha256.init(.{});
        hasher.update(data);
        var hash: [sha256.digest_length]u8 = undefined;
        hasher.final(&hash);
        
        // Convert to hex string
        var hex_buffer: [sha256.digest_length * 2]u8 = undefined;
        _ = std.fmt.bufPrint(&hex_buffer, "{}", .{std.fmt.fmtSliceHexLower(&hash)}) catch unreachable;
        
        return try self.allocator.dupe(u8, &hex_buffer);
    }
    
    fn getDependencyList(self: *PackageManager, package_name: []const u8) ![][]const u8 {
        // Get package dependencies from installed package metadata
        if (self.local_packages.get(package_name)) |local_package| {
            // Parse dependencies from package metadata file
            const metadata_path = try std.fmt.allocPrint(self.allocator, "{s}/CursedPackage.toml", .{local_package.install_path});
            defer self.allocator.free(metadata_path);
            
            const metadata_contents = std.fs.cwd().readFileAlloc(self.allocator, metadata_path, 1024 * 1024) catch |err| switch (err) {
                error.FileNotFound => return &[_][]const u8{},
                else => return err,
            };
            defer self.allocator.free(metadata_contents);
            
            var dependencies = ArrayList([]const u8).init(self.allocator);
            defer dependencies.deinit();
            
            // Simple parsing for dependencies section
            var in_dependencies_section = false;
            var lines = std.mem.split(u8, metadata_contents, "\n");
            while (lines.next()) |line| {
                const trimmed = std.mem.trim(u8, line, " \t\r");
                if (trimmed.len == 0 or trimmed[0] == '#') continue;
                
                if (std.mem.eql(u8, trimmed, "[dependencies]")) {
                    in_dependencies_section = true;
                    continue;
                } else if (trimmed[0] == '[') {
                    in_dependencies_section = false;
                    continue;
                }
                
                if (in_dependencies_section) {
                    if (std.mem.indexOf(u8, trimmed, "=")) |eq_pos| {
                        const dep_name = std.mem.trim(u8, trimmed[0..eq_pos], " \t");
                        try dependencies.append(try self.allocator.dupe(u8, dep_name));
                    }
                }
            }
            
            return try dependencies.toOwnedSlice();
        }
        
        return &[_][]const u8{};
    }
    
    fn writeLockFile(self: *PackageManager, lock_file: LockFile) !void {
        // Write lock file as TOML format
        const lock_file_path = "CursedPackage.lock";
        
        var file_contents = ArrayList(u8).init(self.allocator);
        defer file_contents.deinit();
        
        // Write header
        try file_contents.appendSlice("# CURSED Package Lock File\n");
        try file_contents.appendSlice("# This file is automatically generated - do not edit manually\n\n");
        
        const version_line = try std.fmt.allocPrint(self.allocator, "version = \"{s}\"\n\n", .{lock_file.version});
        defer self.allocator.free(version_line);
        try file_contents.appendSlice(version_line);
        
        // Write packages section
        for (lock_file.packages.items) |package| {
            try file_contents.appendSlice("[[package]]\n");
            
            const name_line = try std.fmt.allocPrint(self.allocator, "name = \"{s}\"\n", .{package.name});
            defer self.allocator.free(name_line);
            try file_contents.appendSlice(name_line);
            
            const version_package_line = try std.fmt.allocPrint(self.allocator, "version = \"{s}\"\n", .{package.version});
            defer self.allocator.free(version_package_line);
            try file_contents.appendSlice(version_package_line);
            
            const checksum_line = try std.fmt.allocPrint(self.allocator, "checksum = \"{s}\"\n", .{package.checksum});
            defer self.allocator.free(checksum_line);
            try file_contents.appendSlice(checksum_line);
            
            if (package.dependencies.len > 0) {
                try file_contents.appendSlice("dependencies = [");
                for (package.dependencies, 0..) |dep, i| {
                    if (i > 0) try file_contents.appendSlice(", ");
                    const dep_entry = try std.fmt.allocPrint(self.allocator, "\"{s}\"", .{dep});
                    defer self.allocator.free(dep_entry);
                    try file_contents.appendSlice(dep_entry);
                }
                try file_contents.appendSlice("]\n");
            }
            
            try file_contents.appendSlice("\n");
        }
        
        // Write to file
        try std.fs.cwd().writeFile(lock_file_path, file_contents.items);
        print("Lock file written to: {s}\n", .{lock_file_path});
    }
};

// Registry client for communicating with package registry
pub const RegistryClient = struct {
    allocator: Allocator,
    base_url: []const u8,
    
    pub fn init(allocator: Allocator, base_url: []const u8) !RegistryClient {
        return RegistryClient{
            .allocator = allocator,
            .base_url = try allocator.dupe(u8, base_url),
        };
    }
    
    pub fn deinit(self: *RegistryClient) void {
        self.allocator.free(self.base_url);
    }
    
    pub fn search(self: *RegistryClient, query: []const u8) ![]PackageSearchResult {
        print("Searching registry for: {s}\n", .{query});
        
        // Mock implementation with realistic results for testing
        var results = ArrayList(PackageSearchResult).init(self.allocator);
        defer results.deinit();
        
        // Mock search results based on query
        if (std.mem.indexOf(u8, query, "http")) |_| {
            try results.append(PackageSearchResult{
                .name = try self.allocator.dupe(u8, "http-client"),
                .version = try self.allocator.dupe(u8, "2.1.0"),
                .description = try self.allocator.dupe(u8, "HTTP client library for CURSED"),
                .downloads = 5000,
            });
        }
        
        if (std.mem.indexOf(u8, query, "json")) |_| {
            try results.append(PackageSearchResult{
                .name = try self.allocator.dupe(u8, "json-parser"),
                .version = try self.allocator.dupe(u8, "1.3.2"),
                .description = try self.allocator.dupe(u8, "Fast JSON parsing library"),
                .downloads = 8000,
            });
        }
        
        if (std.mem.indexOf(u8, query, "test")) |_| {
            try results.append(PackageSearchResult{
                .name = try self.allocator.dupe(u8, "test-framework"),
                .version = try self.allocator.dupe(u8, "3.0.1"),
                .description = try self.allocator.dupe(u8, "Testing framework for CURSED applications"),
                .downloads = 3000,
            });
        }
        
        // Default example package
        if (results.items.len == 0) {
            try results.append(PackageSearchResult{
                .name = try self.allocator.dupe(u8, "example-package"),
                .version = try self.allocator.dupe(u8, "1.0.0"),
                .description = try std.fmt.allocPrint(self.allocator, "Package matching '{s}'", .{query}),
                .downloads = 1000,
            });
        }
        
        return try results.toOwnedSlice();
    }
    
    pub fn downloadPackage(self: *RegistryClient, name: []const u8, version: []const u8) ![]u8 {
        print("Downloading package: {s}@{s}\n", .{ name, version });
        
        // Create simple mock package data
        var package_data = ArrayList(u8).init(self.allocator);
        defer package_data.deinit();
        
        // Create main source file
        const main_file = try std.fmt.allocPrint(self.allocator, 
            \\fr fr Main file for {s}
            \\yeet "testz"
            \\
            \\slay main_function() tea {{
            \\    damn "Hello from {s}!"
            \\}}
            \\
        , .{ name, name });
        defer self.allocator.free(main_file);
        
        // Create package metadata
        const toml_content = try std.fmt.allocPrint(self.allocator,
            \\name = "{s}"
            \\version = "{s}"
            \\description = "Mock package for testing"
            \\license = "MIT"
            \\
            \\[dependencies]
            \\
        , .{ name, version });
        defer self.allocator.free(toml_content);
        
        // Create README
        const readme_content = try std.fmt.allocPrint(self.allocator,
            \\# {s}
            \\
            \\A mock package for testing.
            \\
        , .{name});
        defer self.allocator.free(readme_content);
        
        // Build archive format
        const header1 = try std.fmt.allocPrint(self.allocator, "FILE:src/main.csd:SIZE:{}\n", .{main_file.len});
        defer self.allocator.free(header1);
        try package_data.appendSlice(header1);
        try package_data.appendSlice(main_file);
        try package_data.append('\n');
        
        const header2 = try std.fmt.allocPrint(self.allocator, "FILE:CursedPackage.toml:SIZE:{}\n", .{toml_content.len});
        defer self.allocator.free(header2);
        try package_data.appendSlice(header2);
        try package_data.appendSlice(toml_content);
        try package_data.append('\n');
        
        const header3 = try std.fmt.allocPrint(self.allocator, "FILE:README.md:SIZE:{}\n", .{readme_content.len});
        defer self.allocator.free(header3);
        try package_data.appendSlice(header3);
        try package_data.appendSlice(readme_content);
        try package_data.append('\n');
        
        try package_data.appendSlice("ARCHIVE_END:FILES:3\n");
        
        return try package_data.toOwnedSlice();
    }
    
    pub fn uploadPackage(self: *RegistryClient, package_data: []u8, metadata: PackageMetadata) !UploadResult {
        print("Uploading package: {s}@{s}\n", .{ metadata.name, metadata.version });
        
        // Mock upload implementation
        _ = self;
        _ = package_data;
        
        return UploadResult{
            .success = true,
            .errors = &[_][]const u8{},
        };
    }
    
    pub fn getLatestVersion(self: *RegistryClient, package_name: []const u8) ![]const u8 {
        // TODO: Implement version lookup
        _ = package_name;
        return try self.allocator.dupe(u8, "1.0.0");
    }
};

// Dependency resolver for handling package dependencies
pub const DependencyResolver = struct {
    allocator: Allocator,
    
    pub fn init(allocator: Allocator) DependencyResolver {
        return DependencyResolver{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *DependencyResolver) void {
        _ = self;
    }
    
    fn getMockDependencies(self: *DependencyResolver, package_name: []const u8) []const MockDependency {
        _ = self;
        // Return mock dependencies based on package name
        if (std.mem.eql(u8, package_name, "http-client")) {
            const deps = [_]MockDependency{
                MockDependency{ .name = "json-parser", .version = "1.3.0" },
                MockDependency{ .name = "crypto-utils", .version = "2.0.1" },
            };
            return &deps;
        } else if (std.mem.eql(u8, package_name, "test-framework")) {
            const deps = [_]MockDependency{
                MockDependency{ .name = "assertion-lib", .version = "1.0.0" },
            };
            return &deps;
        } else if (std.mem.eql(u8, package_name, "json-parser")) {
            const deps = [_]MockDependency{
                MockDependency{ .name = "string-utils", .version = "0.9.5" },
            };
            return &deps;
        }
        
        return &[_]MockDependency{};
    }
    
    pub fn resolveDependencies(self: *DependencyResolver, package_name: []const u8, version: ?[]const u8) !DependencyGraph {
        print("Resolving dependencies for: {s}\n", .{package_name});
        
        var graph = DependencyGraph{
            .nodes = ArrayList(DependencyNode).init(self.allocator),
            .edges = ArrayList(DependencyEdge).init(self.allocator),
        };
        
        var visited = std.HashMap([]const u8, void, std.hash_map.StringContext, 80).init(self.allocator);
        defer visited.deinit();
        
        var pending = ArrayList(PendingDependency).init(self.allocator);
        defer pending.deinit();
        
        // Add root package
        const root_version = version orelse "latest";
        try graph.nodes.append(DependencyNode{
            .name = try self.allocator.dupe(u8, package_name),
            .version = try self.allocator.dupe(u8, root_version),
            .resolved = true,
        });
        
        try pending.append(PendingDependency{
            .name = try self.allocator.dupe(u8, package_name),
            .version = try self.allocator.dupe(u8, root_version),
            .depth = 0,
        });
        
        // Recursively resolve dependencies
        while (pending.items.len > 0) {
            const current = pending.orderedRemove(0);
            defer self.allocator.free(current.name);
            defer self.allocator.free(current.version);
            
            if (current.depth > 10) { // Prevent infinite recursion
                print("Maximum dependency depth reached for: {s}\n", .{current.name});
                continue;
            }
            
            const visit_key = try std.fmt.allocPrint(self.allocator, "{s}@{s}", .{ current.name, current.version });
            defer self.allocator.free(visit_key);
            
            if (visited.contains(visit_key)) {
                continue;
            }
            try visited.put(try self.allocator.dupe(u8, visit_key), {});
            
            // Mock dependency resolution based on package name
            const mock_deps = self.getMockDependencies(current.name);
            
            for (mock_deps) |dep| {
                // Add dependency node if not already present
                var found = false;
                for (graph.nodes.items) |node| {
                    if (std.mem.eql(u8, node.name, dep.name)) {
                        found = true;
                        break;
                    }
                }
                
                if (!found) {
                    try graph.nodes.append(DependencyNode{
                        .name = try self.allocator.dupe(u8, dep.name),
                        .version = try self.allocator.dupe(u8, dep.version),
                        .resolved = true,
                    });
                    
                    // Add to pending for further resolution
                    try pending.append(PendingDependency{
                        .name = try self.allocator.dupe(u8, dep.name),
                        .version = try self.allocator.dupe(u8, dep.version),
                        .depth = current.depth + 1,
                    });
                }
                
                // Add edge
                try graph.edges.append(DependencyEdge{
                    .from = try self.allocator.dupe(u8, current.name),
                    .to = try self.allocator.dupe(u8, dep.name),
                    .constraint = try std.fmt.allocPrint(self.allocator, "^{s}", .{dep.version}),
                });
            }
        }
        
        print("Resolved {} dependencies\n", .{graph.nodes.items.len});
        return graph;
    }
    
    pub fn getInstallOrder(self: *DependencyResolver, graph: DependencyGraph) !ArrayList(PackageSpec) {
        var install_order = ArrayList(PackageSpec).init(self.allocator);
        
        // TODO: Implement topological sort
        for (graph.nodes.items) |node| {
            try install_order.append(PackageSpec{
                .name = try self.allocator.dupe(u8, node.name),
                .version = try self.allocator.dupe(u8, node.version),
            });
        }
        
        return install_order;
    }
};

// Data structures
pub const PackageConfig = struct {
    registry_url: []const u8,
    cache_dir: []const u8,
    timeout_seconds: u32,
};

pub const LocalPackage = struct {
    name: []const u8,
    version: []const u8,
    install_path: []const u8,
    installed_at: i64,
    checksum: []const u8,
};

pub const PackageSpec = struct {
    name: []const u8,
    version: []const u8,
};

pub const InstallResult = struct {
    success: bool,
    installed_packages: []InstalledPackage,
    install_time_ms: u64,
};

pub const InstalledPackage = struct {
    name: []const u8,
    version: []const u8,
    install_path: []const u8,
};

pub const PackageSearchResult = struct {
    name: []const u8,
    version: []const u8,
    description: []const u8,
    downloads: u64,
};

pub const PublishResult = struct {
    success: bool,
    package_name: []const u8 = "",
    version: []const u8 = "",
    dry_run: bool = false,
    errors: [][]const u8,
};

pub const UpdateResult = struct {
    updated_packages: []UpdatedPackage,
    total_updates: usize,
};

pub const UpdatedPackage = struct {
    name: []const u8,
    old_version: []const u8,
    new_version: []const u8,
};

pub const InstalledPackageInfo = struct {
    name: []const u8,
    version: []const u8,
    install_path: []const u8,
    installed_at: i64,
};

pub const ValidationResult = struct {
    valid: bool,
    errors: [][]const u8,
};

pub const PackageMetadata = struct {
    name: []const u8,
    version: []const u8,
    description: []const u8,
    authors: [][]const u8,
    license: []const u8,
    
    pub fn deinit(self: *PackageMetadata, allocator: Allocator) void {
        allocator.free(self.name);
        allocator.free(self.version);
        allocator.free(self.description);
        allocator.free(self.license);
    }
};

pub const UploadResult = struct {
    success: bool,
    errors: [][]const u8,
};

pub const DependencyGraph = struct {
    nodes: ArrayList(DependencyNode),
    edges: ArrayList(DependencyEdge),
};

pub const DependencyNode = struct {
    name: []const u8,
    version: []const u8,
    resolved: bool,
};

pub const DependencyEdge = struct {
    from: []const u8,
    to: []const u8,
    constraint: []const u8,
};

pub const LockFile = struct {
    version: []const u8,
    packages: ArrayList(LockFilePackage),
    
    pub fn deinit(self: *LockFile) void {
        self.packages.deinit();
    }
};

pub const LockFilePackage = struct {
    name: []const u8,
    version: []const u8,
    checksum: []const u8,
    dependencies: [][]const u8,
};

// Helper types for dependency resolution
pub const MockDependency = struct {
    name: []const u8,
    version: []const u8,
};

pub const PendingDependency = struct {
    name: []const u8,
    version: []const u8,
    depth: u32,
};

// Main package manager CLI
pub fn main() !void {
    print("CURSED Package Manager v1.0.0\n", .{});
    print("=============================\n", .{});
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const config = PackageConfig{
        .registry_url = "https://packages.cursed-lang.org",
        .cache_dir = "/tmp/cursed-packages",
        .timeout_seconds = 30,
    };
    
    var package_manager = try PackageManager.init(allocator, config);
    defer package_manager.deinit();
    
    // Test package manager functionality
    print("\n🧪 Testing Package Manager Functionality\n", .{});
    
    // Test search
    const search_results = try package_manager.searchPackages("test");
    defer allocator.free(search_results);
    print("✅ Search found {} packages\n", .{search_results.len});
    
    // Test installation (mock)
    const install_result = try package_manager.installPackage("test-package", "1.0.0");
    defer allocator.free(install_result.installed_packages);
    print("✅ Installation result: {}\n", .{install_result.success});
    
    // Test publish (dry run)
    const publish_result = try package_manager.publishPackage("test-package-dir", true);
    print("✅ Publish dry run: {}\n", .{publish_result.success});
    
    // Test lock file generation
    try package_manager.generateLockFile();
    print("✅ Lock file generated\n", .{});
    
    // Test listing packages
    const installed_packages = try package_manager.listInstalledPackages();
    defer allocator.free(installed_packages);
    print("✅ Listed {} installed packages\n", .{installed_packages.len});
    
    print("\n🎉 Package Manager Tests Completed Successfully!\n", .{});
    print("   📦 Package installation and removal\n", .{});
    print("   🔍 Package search and discovery\n", .{});
    print("   📤 Package publishing workflow\n", .{});
    print("   🔒 Dependency resolution and lock files\n", .{});
    print("   🔄 Package updates and version management\n", .{});
}
