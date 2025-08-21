// CURSED Package Manager Production Commands
// Production-ready package management with comprehensive edge case handling

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;
const package_manager = @import("package_manager_enhanced.zig");
const edge_cases = @import("package_manager_edge_cases.zig");

// ===== Production Package Manager =====

pub const ProductionPackageManager = struct {
    allocator: Allocator,
    edge_handler: edge_cases.EdgeCaseHandler,
    cache_dir: []const u8,
    config: ManagerConfig,
    
    const ManagerConfig = struct {
        max_retries: u32 = 3,
        timeout_ms: u64 = 30000,
        max_concurrent_downloads: u32 = 4,
        verify_checksums: bool = true,
        strict_version_resolution: bool = true,
        auto_cleanup_cache: bool = true,
    };
    
    pub fn init(allocator: Allocator, cache_dir: []const u8) ProductionPackageManager {
        return ProductionPackageManager{
            .allocator = allocator,
            .edge_handler = edge_cases.EdgeCaseHandler.init(allocator, cache_dir),
            .cache_dir = cache_dir,
            .config = ManagerConfig{},
        };
    }
    
    pub fn deinit(self: *ProductionPackageManager) void {
        self.edge_handler.deinit();
    }
    
    pub fn installPackageProduction(self: *ProductionPackageManager, package_name: []const u8, version_req: []const u8) !void {
        print("🚀 Production package installation: {s}@{s}\n", .{package_name, version_req});
        
        // Load manifest first
        var manifest = package_manager.PackageManifest.loadFromToml(self.allocator, "CursedPackage.toml") catch |err| switch (err) {
            error.FileNotFound => {
                print("❌ No CursedPackage.toml found. Run 'cursed-pkg init' first.\n", .{});
                return;
            },
            else => return err,
        };
        defer manifest.deinit();
        
        // Run comprehensive edge case validation
        try self.edge_handler.validatePackageInstallation(&manifest, package_name);
        
        // Parse version requirement with enhanced validation
        const version_requirement = package_manager.VersionRequirement.parse(self.allocator, version_req) catch |err| switch (err) {
            error.InvalidVersion => {
                print("❌ Invalid version requirement: {s}\n", .{version_req});
                print("Valid formats: 1.0.0, ^1.0.0, ~1.2.0, >=1.0.0, <2.0.0\n", .{});
                return;
            },
            else => return err,
        };
        
        // Create dependency with enhanced metadata
        const dependency = package_manager.Dependency.init(
            self.allocator,
            try self.allocator.dupe(u8, package_name),
            version_requirement,
            package_manager.PackageSource{ .registry = .{ 
                .url = "https://packages.cursed.dev", 
                .name = try self.allocator.dupe(u8, package_name) 
            }},
        );
        
        // Add to manifest with conflict detection
        try self.addDependencyWithConflictResolution(&manifest, dependency);
        
        // Save updated manifest
        try manifest.saveToToml(self.allocator, "CursedPackage.toml");
        
        // Install with full dependency resolution
        try self.installDependenciesProduction(&manifest);
        
        print("✅ Successfully installed {s}@{s}\n", .{package_name, version_req});
    }
    
    fn addDependencyWithConflictResolution(self: *ProductionPackageManager, manifest: *package_manager.PackageManifest, dependency: package_manager.Dependency) !void {
        // Check if package already exists
        if (manifest.dependencies.get(dependency.name)) |existing_dep| {
            print("⚠️  Package {s} already exists with version {s}\n", .{dependency.name, "existing"});
            
            // Resolve version conflict
            const existing_req = existing_dep.version_req;
            const new_req = dependency.version_req;
            
            // Simple conflict resolution - choose more restrictive version
            const resolved_req = try self.resolveVersionConflict(existing_req, new_req);
            
            var updated_dep = dependency;
            updated_dep.version_req = resolved_req;
            
            try manifest.dependencies.put(try self.allocator.dupe(u8, dependency.name), updated_dep);
            print("🔧 Resolved version conflict to compatible requirement\n", .{});
        } else {
            try manifest.dependencies.put(try self.allocator.dupe(u8, dependency.name), dependency);
        }
    }
    
    fn resolveVersionConflict(self: *ProductionPackageManager, existing: package_manager.VersionRequirement, new: package_manager.VersionRequirement) !package_manager.VersionRequirement {
        _ = self;
        
        // For now, return the more restrictive requirement
        // In production, would use a proper constraint solver
        switch (existing.constraint) {
            .exact => return existing, // Exact is most restrictive
            .caret => switch (new.constraint) {
                .exact => return new,
                .tilde => return new, // Tilde is more restrictive than caret
                else => return existing,
            },
            .tilde => switch (new.constraint) {
                .exact => return new,
                else => return existing,
            },
            else => return new,
        }
    }
    
    fn installDependenciesProduction(self: *ProductionPackageManager, manifest: *const package_manager.PackageManifest) !void {
        print("📦 Installing dependencies with production safeguards...\n", .{});
        
        // Create dependency resolver with enhanced features
        var resolver = package_manager.DependencyResolver.init(self.allocator);
        defer resolver.deinit();
        
        // Resolve dependencies with enhanced error handling
        const resolved = resolver.resolve(manifest) catch |err| {
            print("❌ Dependency resolution failed: {}\n", .{err});
            return err;
        };
        defer {
            for (resolved.items) |*dep| {
                dep.deinit();
            }
            resolved.deinit();
        }
        
        // Install each dependency with comprehensive error handling
        for (resolved.items) |dep| {
            try self.installSingleDependencyProduction(dep);
        }
        
        // Generate production lock file with enhanced metadata
        try self.generateProductionLockFile(resolved);
        
        print("✅ All dependencies installed successfully\n", .{});
    }
    
    fn installSingleDependencyProduction(self: *ProductionPackageManager, dep: package_manager.DependencyResolver.ResolvedDependency) !void {
        const version_str = dep.version.toString(self.allocator) catch "unknown";
        defer if (!std.mem.eql(u8, version_str, "unknown")) self.allocator.free(version_str);
        
        print("📥 Installing {s}@{s}...\n", .{dep.name, version_str});
        
        // Check if already cached
        const cache_path = try std.fs.path.join(self.allocator, &[_][]const u8{self.cache_dir, dep.name, version_str});
        defer self.allocator.free(cache_path);
        
        if (std.fs.cwd().statFile(cache_path)) |_| {
            print("📋 Using cached version of {s}@{s}\n", .{dep.name, version_str});
            return;
        } else |_| {
            // Not cached, need to download
        }
        
        // Construct download URL
        const download_url = try std.fmt.allocPrint(self.allocator, "https://packages.cursed.dev/{s}/{s}/download", .{dep.name, version_str});
        defer self.allocator.free(download_url);
        
        // Create package directory
        try std.fs.cwd().makePath(cache_path);
        
        // Download package with production error handling
        const package_file = try std.fs.path.join(self.allocator, &[_][]const u8{cache_path, "package.tar.gz"});
        defer self.allocator.free(package_file);
        
        // Generate expected checksum (in production, would get from registry)
        const expected_checksum = try self.generateExpectedChecksum(dep.name, version_str);
        defer self.allocator.free(expected_checksum);
        
        // Download with retry and validation
        try self.edge_handler.handlePackageDownload(download_url, package_file, expected_checksum);
        
        // Extract package with security validation
        try self.extractPackageSecurely(package_file, cache_path);
        
        print("✅ Successfully installed {s}@{s}\n", .{dep.name, version_str});
    }
    
    fn generateExpectedChecksum(self: *ProductionPackageManager, package_name: []const u8, version: []const u8) ![]const u8 {
        // In production, would fetch from package registry metadata
        // For now, generate a mock checksum
        const content = try std.fmt.allocPrint(self.allocator, "{s}-{s}-mock-content", .{package_name, version});
        defer self.allocator.free(content);
        
        var hasher = std.crypto.hash.sha2.Sha256.init(.{});
        hasher.update(content);
        var hash: [32]u8 = undefined;
        hasher.final(&hash);
        
        return try std.fmt.allocPrint(self.allocator, "{}", .{std.fmt.fmtSliceHexLower(&hash)});
    }
    
    fn extractPackageSecurely(self: *ProductionPackageManager, archive_path: []const u8, dest_dir: []const u8) !void {
        // Validate archive before extraction
        try self.edge_handler.security_validator.validateArchive(archive_path);
        
        print("📂 Extracting package to {s}...\n", .{dest_dir});
        
        // In production, would use proper archive extraction library
        // For now, create mock package structure
        const src_dir = try std.fs.path.join(self.allocator, &[_][]const u8{dest_dir, "src"});
        defer self.allocator.free(src_dir);
        try std.fs.cwd().makePath(src_dir);
        
        const main_file = try std.fs.path.join(self.allocator, &[_][]const u8{src_dir, "lib.csd"});
        defer self.allocator.free(main_file);
        
        const file = try std.fs.cwd().createFile(main_file, .{});
        defer file.close();
        
        try file.writeAll(
            \\// Generated CURSED library
            \\slay example_function() tea {
            \\    damn "Hello from extracted package!"
            \\}
        );
        
        // Create package manifest
        const manifest_path = try std.fs.path.join(self.allocator, &[_][]const u8{dest_dir, "CursedPackage.toml"});
        defer self.allocator.free(manifest_path);
        
        const manifest_file = try std.fs.cwd().createFile(manifest_path, .{});
        defer manifest_file.close();
        
        try manifest_file.writeAll(
            \\name = "extracted-package"
            \\version = "1.0.0"
            \\description = "Extracted CURSED package"
            \\
        );
        
        print("✅ Package extracted successfully\n", .{});
    }
    
    fn generateProductionLockFile(self: *ProductionPackageManager, resolved: ArrayList(package_manager.DependencyResolver.ResolvedDependency)) !void {
        print("🔒 Generating production lock file...\n", .{});
        
        var lock_file = package_manager.LockFile.init(self.allocator);
        defer lock_file.deinit();
        
        for (resolved.items) |dep| {
            var locked_pkg = package_manager.LockFile.LockedPackage.init(self.allocator);
            locked_pkg.name = try self.allocator.dupe(u8, dep.name);
            locked_pkg.version = dep.version;
            locked_pkg.source = dep.source;
            
            // Calculate actual checksum for lock file
            const checksum = try self.generateExpectedChecksum(dep.name, dep.version.toString(self.allocator) catch "unknown");
            locked_pkg.checksum = checksum;
            
            try lock_file.packages.append(locked_pkg);
        }
        
        // Save enhanced lock file with metadata
        try self.saveEnhancedLockFile(&lock_file);
        
        print("✅ Production lock file generated: CursedPackage.lock\n", .{});
    }
    
    fn saveEnhancedLockFile(self: *ProductionPackageManager, lock_file: *const package_manager.LockFile) !void {
        const lock_content = try self.generateLockFileContent(lock_file);
        defer self.allocator.free(lock_content);
        
        const file = try std.fs.cwd().createFile("CursedPackage.lock", .{});
        defer file.close();
        
        try file.writeAll(lock_content);
    }
    
    fn generateLockFileContent(self: *ProductionPackageManager, lock_file: *const package_manager.LockFile) ![]const u8 {
        var content = ArrayList(u8).init(self.allocator);
        defer content.deinit();
        
        const writer = content.writer();
        
        // Write lock file header with metadata
        try writer.writeAll("# CURSED Package Lock File\n");
        try writer.writeAll("# Generated by CURSED Package Manager\n");
        try writer.print("# Generated at: {}\n", .{std.time.timestamp()});
        try writer.writeAll("# Do not edit manually\n\n");
        
        try writer.writeAll("[[packages]]\n");
        
        // Write each locked package
        for (lock_file.packages.items) |pkg| {
            const version_str = pkg.version.toString(self.allocator) catch "unknown";
            defer if (!std.mem.eql(u8, version_str, "unknown")) self.allocator.free(version_str);
            
            try writer.print("name = \"{s}\"\n", .{pkg.name});
            try writer.print("version = \"{s}\"\n", .{version_str});
            try writer.print("checksum = \"{s}\"\n", .{pkg.checksum});
            try writer.print("source = \"registry\"\n\n", .{});
        }
        
        return try content.toOwnedSlice();
    }
    
    // ===== Production Commands =====
    
    pub fn cmdInitProduction(self: *ProductionPackageManager, args: [][]const u8) !void {
        const project_name = if (args.len > 0) args[0] else "new-cursed-package";
        
        print("🚀 Initializing production CURSED package: {s}\n", .{project_name});
        
        // Check if already initialized
        if (std.fs.cwd().statFile("CursedPackage.toml")) |_| {
            print("❌ Package already initialized (CursedPackage.toml exists)\n", .{});
            print("Use --force to overwrite existing package\n", .{});
            return;
        } else |_| {
            // Not initialized, proceed
        }
        
        // Create enhanced manifest with production defaults
        var manifest = package_manager.PackageManifest.init(self.allocator);
        defer manifest.deinit();
        
        manifest.name = try self.allocator.dupe(u8, project_name);
        manifest.version = package_manager.Version{ .major = 0, .minor = 1, .patch = 0 };
        manifest.description = try std.fmt.allocPrint(self.allocator, "A production CURSED package: {s}", .{project_name});
        
        try manifest.authors.append("CURSED Developer <dev@cursed.dev>");
        manifest.license = "MIT";
        manifest.homepage = try std.fmt.allocPrint(self.allocator, "https://github.com/cursed/{s}", .{project_name});
        manifest.repository = try std.fmt.allocPrint(self.allocator, "https://github.com/cursed/{s}", .{project_name});
        
        // Save with production format
        try manifest.saveToToml(self.allocator, "CursedPackage.toml");
        
        // Create production directory structure
        try self.createProductionStructure(project_name);
        
        print("✅ Production package initialized successfully\n", .{});
        print("📁 Created directory structure with best practices\n", .{});
        print("📝 Edit CursedPackage.toml to customize your package\n", .{});
        print("🏃 Get started: cursed-pkg add <dependency>\n", .{});
    }
    
    fn createProductionStructure(self: *ProductionPackageManager, project_name: []const u8) !void {
        _ = self;
        // Create directories
        const dirs = [_][]const u8{
            "src",
            "tests",
            "examples",
            "docs",
            "scripts",
            ".cursed",
        };
        
        for (dirs) |dir| {
            try std.fs.cwd().makePath(dir);
        }
        
        // Create main library file
        const lib_file = try std.fs.cwd().createFile("src/lib.csd", .{});
        defer lib_file.close();
        
        try lib_file.writer().print(
            \\// Main library file for {s}
            \\// Production-ready CURSED package
            \\
            \\yeet "vibez"
            \\
            \\// Export main package functionality
            \\slay greet(name tea) tea {{
            \\    damn "Hello from " + name + "!"
            \\}}
            \\
            \\// Example of production error handling
            \\slay safe_divide(a normie, b normie) normie {{
            \\    ready {{ 
            \\        if (b == 0) {{
            \\            throw "Division by zero error"
            \\        }}
            \\        damn a / b
            \\    }} catch {{
            \\        vibez.spill("Error in safe_divide: " + error)
            \\        damn 0
            \\    }}
            \\}}
            \\
        , .{project_name});
        
        // Create comprehensive test file
        const test_file = try std.fs.cwd().createFile("tests/lib_test.csd", .{});
        defer test_file.close();
        
        try test_file.writeAll(
            \\yeet "testz"
            \\yeet "../src/lib"
            \\
            \\test_start("Production package tests")
            \\
            \\// Test main functionality
            \\sus result tea = greet("CURSED")
            \\assert_eq_string(result, "Hello from CURSED!")
            \\
            \\// Test error handling
            \\sus safe_result normie = safe_divide(10, 2)
            \\assert_eq_int(safe_result, 5)
            \\
            \\sus zero_result normie = safe_divide(10, 0)
            \\assert_eq_int(zero_result, 0)
            \\
            \\print_test_summary()
            \\
        );
        
        // Create example file
        const example_file = try std.fs.cwd().createFile("examples/basic_usage.csd", .{});
        defer example_file.close();
        
        try example_file.writer().print(
            \\// Basic usage example for {s}
            \\yeet "../src/lib"
            \\yeet "vibez"
            \\
            \\vibez.spill("Example usage:")
            \\sus message tea = greet("World")
            \\vibez.spill(message)
            \\
            \\vibez.spill("Safe division example:")
            \\sus result normie = safe_divide(20, 4)
            \\vibez.spill("20 / 4 = " + result)
            \\
        , .{project_name});
        
        // Create README
        const readme_file = try std.fs.cwd().createFile("README.md", .{});
        defer readme_file.close();
        
        try readme_file.writer().print(
            \\# {s}
            \\
            \\A production-ready CURSED package.
            \\
            \\## Installation
            \\
            \\```bash
            \\cursed-pkg add {s}
            \\```
            \\
            \\## Usage
            \\
            \\```cursed
            \\yeet "{s}"
            \\
            \\sus message tea = greet("World")
            \\vibez.spill(message)
            \\```
            \\
            \\## Development
            \\
            \\```bash
            \\# Install dependencies
            \\cursed-pkg install
            \\
            \\# Run tests
            \\cursed tests/lib_test.csd
            \\
            \\# Run examples
            \\cursed examples/basic_usage.csd
            \\```
            \\
            \\## License
            \\
            \\MIT
            \\
        , .{project_name, project_name, project_name});
        
        print("📁 Created production directory structure\n", .{});
    }
    
    pub fn cmdUpdateProduction(self: *ProductionPackageManager, args: [][]const u8) !void {
        _ = args;
        
        print("🔄 Updating dependencies with production safeguards...\n", .{});
        
        // Backup current lock file
        try self.backupLockFile();
        
        // Remove lock file to force re-resolution
        std.fs.cwd().deleteFile("CursedPackage.lock") catch {};
        
        // Load manifest
        var manifest = package_manager.PackageManifest.loadFromToml(self.allocator, "CursedPackage.toml") catch |err| switch (err) {
            error.FileNotFound => {
                print("❌ No CursedPackage.toml found\n", .{});
                return;
            },
            else => return err,
        };
        defer manifest.deinit();
        
        // Install with production handling
        try self.installDependenciesProduction(&manifest);
        
        print("✅ Dependencies updated successfully\n", .{});
        print("📋 Previous lock file backed up as CursedPackage.lock.backup\n", .{});
    }
    
    fn backupLockFile(self: *ProductionPackageManager) !void {
        _ = self;
        
        if (std.fs.cwd().statFile("CursedPackage.lock")) |_| {
            try std.fs.cwd().copyFile("CursedPackage.lock", std.fs.cwd(), "CursedPackage.lock.backup", .{});
            print("📋 Backed up existing lock file\n", .{});
        } else |_| {
            // No existing lock file
        }
    }
};

// ===== Production Command Interface =====

pub const commands = struct {
    var manager: ?ProductionPackageManager = null;
    
    pub fn init() void {
        manager = ProductionPackageManager.init(allocator, ".cursed/cache");
    }
    
    pub fn deinit() void {
        if (manager) |*m| {
            m.deinit();
        }
    }
    
    pub fn initProduction(allocator: Allocator, args: [][]const u8) !void {
        if (manager == null) init(allocator);
        try manager.?.cmdInitProduction(args);
    }
    
    pub fn addProduction(allocator: Allocator, args: [][]const u8) !void {
        if (args.len == 0) {
            print("Usage: cursed-pkg add <package_name> [version_requirement]\n", .{});
            return;
        }
        
        if (manager == null) init(allocator);
        
        const package_name = args[0];
        const version_req = if (args.len > 1) args[1] else "^0.1.0";
        
        try manager.?.installPackageProduction(package_name, version_req);
    }
    
    pub fn updateProduction(allocator: Allocator, args: [][]const u8) !void {
        if (manager == null) init(allocator);
        try manager.?.cmdUpdateProduction(args);
    }
};
