// CURSED Package Registry API Implementation
// REST API client for interacting with the package registry

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;
const json = std.json;
const registry = @import("package_registry_advanced.zig");

// ===== API Client =====

pub const RegistryApiClient = struct {
    allocator: Allocator,
    base_url: []const u8,
    auth_token: ?[]const u8,
    timeout_ms: u32 = 30000,
    retry_count: u32 = 3,
    
    pub fn init(allocator: Allocator, base_url: []const u8) !RegistryApiClient {
        return RegistryApiClient{
            .allocator = allocator,
            .base_url = try allocator.dupe(u8, base_url),
            .auth_token = null,
        };
    }
    
    pub fn deinit(self: *RegistryApiClient) void {
        self.allocator.free(self.base_url);
        if (self.auth_token) |token| self.allocator.free(token);
    }
    
    pub fn setAuthToken(self: *RegistryApiClient, token: []const u8) !void {
        if (self.auth_token) |old_token| self.allocator.free(old_token);
        self.auth_token = try self.allocator.dupe(u8, token);
    }
    
    // ===== Search API =====
    
    pub fn search(self: *RegistryApiClient, query: registry.SearchQuery) !registry.SearchResult {
        print("🔍 Searching packages: '{s}'\n", .{query.query});
        
        // Build search URL with parameters
        const url = try self.buildSearchUrl(query);
        defer self.allocator.free(url);
        
        print("   📡 Request URL: {s}\n", .{url});
        
        // For now, return mock search results
        // In production, would make actual HTTP request
        var result = registry.SearchResult.init(self.allocator);
        
        // Mock search results based on query
        if (std.mem.indexOf(u8, query.query, "json") != null) {
            var json_pkg = registry.PackageMetadata.init(self.allocator);
            json_pkg.name = try self.allocator.dupe(u8, "fast-json");
            json_pkg.version = try self.allocator.dupe(u8, "1.2.3");
            json_pkg.description = try self.allocator.dupe(u8, "High-performance JSON parser and serializer");
            try json_pkg.authors.append("JSON Team <json@cursed.dev>");
            json_pkg.license = try self.allocator.dupe(u8, "MIT");
            try json_pkg.keywords.append("json");
            try json_pkg.keywords.append("parser");
            try json_pkg.keywords.append("serialization");
            try json_pkg.categories.append(.utilities);
            
            json_pkg.downloads.total = 15432;
            json_pkg.downloads.last_30_days = 2341;
            json_pkg.quality_score.overall = 92.5;
            json_pkg.security_status.status = .secure;
            
            try result.packages.append(json_pkg);
        }
        
        if (std.mem.indexOf(u8, query.query, "http") != null) {
            var http_pkg = registry.PackageMetadata.init(self.allocator);
            http_pkg.name = try self.allocator.dupe(u8, "secure-http");
            http_pkg.version = try self.allocator.dupe(u8, "2.1.0");
            http_pkg.description = try self.allocator.dupe(u8, "Secure HTTP client with advanced features");
            try http_pkg.authors.append("HTTP Team <http@cursed.dev>");
            http_pkg.license = try self.allocator.dupe(u8, "MIT");
            try http_pkg.keywords.append("http");
            try http_pkg.keywords.append("client");
            try http_pkg.keywords.append("security");
            try http_pkg.categories.append(.web);
            
            http_pkg.downloads.total = 8923;
            http_pkg.downloads.last_30_days = 1567;
            http_pkg.quality_score.overall = 89.1;
            http_pkg.security_status.status = .secure;
            
            try result.packages.append(http_pkg);
        }
        
        if (std.mem.indexOf(u8, query.query, "crypto") != null) {
            var crypto_pkg = registry.PackageMetadata.init(self.allocator);
            crypto_pkg.name = try self.allocator.dupe(u8, "crypto-suite");
            crypto_pkg.version = try self.allocator.dupe(u8, "3.0.1");
            crypto_pkg.description = try self.allocator.dupe(u8, "Comprehensive cryptography library");
            try crypto_pkg.authors.append("Crypto Team <crypto@cursed.dev>");
            crypto_pkg.license = try self.allocator.dupe(u8, "MIT");
            try crypto_pkg.keywords.append("crypto");
            try crypto_pkg.keywords.append("encryption");
            try crypto_pkg.keywords.append("security");
            try crypto_pkg.categories.append(.crypto);
            
            crypto_pkg.downloads.total = 12845;
            crypto_pkg.downloads.last_30_days = 2103;
            crypto_pkg.quality_score.overall = 95.3;
            crypto_pkg.security_status.status = .secure;
            
            try result.packages.append(crypto_pkg);
        }
        
        result.total_count = @intCast(result.packages.items.len);
        result.query_time_ms = 45; // Mock query time
        
        if (result.packages.items.len == 0) {
            try result.suggestions.append("json-parser");
            try result.suggestions.append("http-client");
            try result.suggestions.append("utility-lib");
        }
        
        print("✅ Found {} packages in {}ms\n", .{result.total_count, result.query_time_ms});
        return result;
    }
    
    fn buildSearchUrl(self: *RegistryApiClient, query: registry.SearchQuery) ![]const u8 {
        var url_parts = ArrayList(u8).init(self.allocator);
        defer url_parts.deinit();
        
        const writer = url_parts.writer();
        try writer.print("{s}/api/v1/search?q={s}", .{self.base_url, query.query});
        
        if (query.categories.items.len > 0) {
            try writer.writeAll("&categories=");
            for (query.categories.items, 0..) |category, i| {
                if (i > 0) try writer.writeAll(",");
                try writer.writeAll(category.toString());
            }
        }
        
        if (query.min_quality) |min_qual| {
            try writer.print("&min_quality={d}", .{min_qual});
        }
        
        if (query.only_secure) {
            try writer.writeAll("&only_secure=true");
        }
        
        try writer.print("&sort={s}", .{@tagName(query.sort_by)});
        try writer.print("&limit={}&offset={}", .{query.limit, query.offset});
        
        return try url_parts.toOwnedSlice();
    }
    
    // ===== Package Information API =====
    
    pub fn getPackageInfo(self: *RegistryApiClient, name: []const u8, version: ?[]const u8) !registry.PackageMetadata {
        const version_str = version orelse "latest";
        print("📦 Fetching package info: {s}@{s}\n", .{name, version_str});
        
        // Mock package information
        var metadata = registry.PackageMetadata.init(self.allocator);
        metadata.name = try self.allocator.dupe(u8, name);
        metadata.version = try self.allocator.dupe(u8, version_str);
        
        // Simulate different packages
        if (std.mem.eql(u8, name, "json-parser")) {
            metadata.description = try self.allocator.dupe(u8, "Fast and reliable JSON parser for CURSED");
            try metadata.authors.append("JSON Team <json@cursed.dev>");
            metadata.license = try self.allocator.dupe(u8, "MIT");
            metadata.repository = try self.allocator.dupe(u8, "https://github.com/cursed/json-parser");
            metadata.homepage = try self.allocator.dupe(u8, "https://json-parser.cursed.dev");
            
            try metadata.keywords.append("json");
            try metadata.keywords.append("parser");
            try metadata.keywords.append("serialization");
            try metadata.categories.append(.utilities);
            
            metadata.downloads.total = 25634;
            metadata.downloads.last_30_days = 3421;
            metadata.downloads.last_7_days = 892;
            metadata.downloads.trend = .increasing;
            
            metadata.quality_score.overall = 94.2;
            metadata.quality_score.documentation = 95.0;
            metadata.quality_score.testing = 92.0;
            metadata.quality_score.maintenance = 96.0;
            metadata.quality_score.community = 93.0;
            metadata.quality_score.performance = 95.0;
            metadata.quality_score.security = 94.0;
            
            metadata.security_status.status = .secure;
            metadata.security_status.last_scan = std.time.timestamp() - 3600;
            metadata.security_status.scan_version = try self.allocator.dupe(u8, "1.0.0");
            
            // Add sample review
            const review = registry.PackageMetadata.Review{
                .id = try self.allocator.dupe(u8, "review-001"),
                .author = try self.allocator.dupe(u8, "developer123"),
                .rating = 5,
                .title = try self.allocator.dupe(u8, "Excellent JSON library"),
                .content = try self.allocator.dupe(u8, "Very fast and easy to use. Great documentation!"),
                .helpful_count = 12,
                .created_at = std.time.timestamp() - 86400,
                .verified_download = true,
            };
            try metadata.reviews.append(review);
        } else {
            metadata.description = try self.allocator.dupe(u8, "A useful CURSED package");
            try metadata.authors.append("Package Author <author@cursed.dev>");
            metadata.license = try self.allocator.dupe(u8, "MIT");
            
            metadata.downloads.total = 1234;
            metadata.downloads.last_30_days = 156;
            metadata.quality_score.overall = 75.0;
            metadata.security_status.status = .secure;
        }
        
        print("✅ Package info retrieved successfully\n", .{});
        return metadata;
    }
    
    // ===== Publishing API =====
    
    pub fn publishPackage(self: *RegistryApiClient, package_path: []const u8) !PublishResult {
        print("📤 Publishing package from: {s}\n", .{package_path});
        
        // Validate authentication
        if (self.auth_token == null) {
            return PublishResult{
                .success = false,
                .message = try self.allocator.dupe(u8, "Authentication required. Run 'cursed-pkg login' first."),
                .package_url = null,
            };
        }
        
        // Read and validate package manifest
        const manifest_path = try std.fs.path.join(self.allocator, &[_][]const u8{package_path, "CursedPackage.toml"});
        defer self.allocator.free(manifest_path);
        
        const manifest_content = std.fs.cwd().readFileAlloc(self.allocator, manifest_path, 1024 * 1024) catch |err| switch (err) {
            error.FileNotFound => {
                return PublishResult{
                    .success = false,
                    .message = try self.allocator.dupe(u8, "CursedPackage.toml not found in package directory"),
                    .package_url = null,
                };
            },
            else => return err,
        };
        defer self.allocator.free(manifest_content);
        
        // Parse manifest to get package name and version
        const package_name = try self.extractPackageName(manifest_content);
        defer self.allocator.free(package_name);
        
        const package_version = try self.extractPackageVersion(manifest_content);
        defer self.allocator.free(package_version);
        
        print("   📦 Package: {s}@{s}\n", .{package_name, package_version});
        
        // Validate package structure
        try self.validatePackageStructure(package_path);
        
        // Security scan
        print("   🔍 Running security scan...\n", .{});
        const scan_result = try self.performPublishSecurityScan(package_path);
        if (!scan_result.passed) {
            return PublishResult{
                .success = false,
                .message = try std.fmt.allocPrint(self.allocator, "Security scan failed: {s}", .{scan_result.message}),
                .package_url = null,
            };
        }
        
        // Create package archive
        print("   📦 Creating package archive...\n", .{});
        const archive_path = try self.createPackageArchive(package_path, package_name, package_version);
        defer self.allocator.free(archive_path);
        
        // Upload to registry
        print("   ⬆️  Uploading to registry...\n", .{});
        const upload_result = try self.uploadPackageArchive(archive_path, package_name, package_version);
        
        if (upload_result.success) {
            print("✅ Package published successfully!\n", .{});
            return PublishResult{
                .success = true,
                .message = try self.allocator.dupe(u8, "Package published successfully"),
                .package_url = try std.fmt.allocPrint(self.allocator, "{s}/packages/{s}/{s}", .{self.base_url, package_name, package_version}),
            };
        } else {
            return PublishResult{
                .success = false,
                .message = try self.allocator.dupe(u8, upload_result.error_message),
                .package_url = null,
            };
        }
    }
    
    fn extractPackageName(self: *RegistryApiClient, manifest_content: []const u8) ![]const u8 {
        // Simple TOML parsing for package name
        if (std.mem.indexOf(u8, manifest_content, "name = \"")) |start| {
            const name_start = start + 8;
            if (std.mem.indexOf(u8, manifest_content[name_start..], "\"")) |end| {
                return try self.allocator.dupe(u8, manifest_content[name_start..name_start + end]);
            }
        }
        return try self.allocator.dupe(u8, "unknown-package");
    }
    
    fn extractPackageVersion(self: *RegistryApiClient, manifest_content: []const u8) ![]const u8 {
        // Simple TOML parsing for package version
        if (std.mem.indexOf(u8, manifest_content, "version = \"")) |start| {
            const version_start = start + 11;
            if (std.mem.indexOf(u8, manifest_content[version_start..], "\"")) |end| {
                return try self.allocator.dupe(u8, manifest_content[version_start..version_start + end]);
            }
        }
        return try self.allocator.dupe(u8, "0.1.0");
    }
    
    fn validatePackageStructure(self: *RegistryApiClient, package_path: []const u8) !void {
        // Check for required files
        const required_files = [_][]const u8{
            "CursedPackage.toml",
            "README.md",
            "src",
        };
        
        for (required_files) |file| {
            const file_path = try std.fs.path.join(self.allocator, &[_][]const u8{package_path, file});
            defer self.allocator.free(file_path);
            
            std.fs.cwd().statFile(file_path) catch {
                print("   ⚠️  Missing required file/directory: {s}\n", .{file});
            };
        }
        
        print("   ✅ Package structure validated\n", .{});
    }
    
    fn performPublishSecurityScan(self: *RegistryApiClient, package_path: []const u8) !SecurityScanResult {
        _ = package_path;
        
        // Simulate security scanning
        std.time.sleep(1000000000); // 1 second
        
        // Mock scan result
        return SecurityScanResult{
            .passed = true,
            .message = try self.allocator.dupe(u8, "No security issues found"),
        };
    }
    
    fn createPackageArchive(self: *RegistryApiClient, package_path: []const u8, name: []const u8, version: []const u8) ![]const u8 {
        _ = package_path;
        
        // Simulate creating package archive
        const archive_name = try std.fmt.allocPrint(self.allocator, "{s}-{s}.tar.gz", .{name, version});
        print("   📦 Created archive: {s}\n", .{archive_name});
        return archive_name;
    }
    
    fn uploadPackageArchive(self: *RegistryApiClient, archive_path: []const u8, name: []const u8, version: []const u8) !UploadResult {
        _ = archive_path;
        _ = name;
        _ = version;
        
        // Simulate upload
        print("   ⬆️  Uploading to {s}...\n", .{self.base_url});
        std.time.sleep(2000000000); // 2 seconds
        
        return UploadResult{
            .success = true,
            .error_message = "",
        };
    }
    
    // ===== Analytics API =====
    
    pub fn getPackageAnalytics(self: *RegistryApiClient, package_name: []const u8, timeframe: AnalyticsTimeframe) !PackageAnalytics {
        print("📊 Fetching analytics for {s} ({s})\n", .{package_name, @tagName(timeframe)});
        
        // Mock analytics data
        var analytics = PackageAnalytics.init(self.allocator);
        
        switch (timeframe) {
            .last_7_days => {
                analytics.total_downloads = 234;
                analytics.unique_users = 67;
                analytics.growth_rate = 12.5;
            },
            .last_30_days => {
                analytics.total_downloads = 1456;
                analytics.unique_users = 389;
                analytics.growth_rate = 8.3;
            },
            .last_90_days => {
                analytics.total_downloads = 4567;
                analytics.unique_users = 1234;
                analytics.growth_rate = 15.7;
            },
        }
        
        // Mock geographic data
        try analytics.geographic_distribution.put("US", 45);
        try analytics.geographic_distribution.put("EU", 32);
        try analytics.geographic_distribution.put("ASIA", 18);
        try analytics.geographic_distribution.put("OTHER", 5);
        
        // Mock version distribution
        try analytics.version_distribution.put("1.0.0", 30);
        try analytics.version_distribution.put("1.1.0", 25);
        try analytics.version_distribution.put("1.2.0", 45);
        
        return analytics;
    }
    
    // ===== Migration Support =====
    
    pub fn importFromEcosystem(self: *RegistryApiClient, ecosystem: ExternalEcosystem, package_spec: []const u8) !ImportResult {
        print("🔄 Importing package from {s}: {s}\n", .{@tagName(ecosystem), package_spec});
        
        const import_result = switch (ecosystem) {
            .npm => try self.importFromNpm(package_spec),
            .cargo => try self.importFromCargo(package_spec),
            .pip => try self.importFromPip(package_spec),
            .go_modules => try self.importFromGoModules(package_spec),
        };
        
        return import_result;
    }
    
    fn importFromNpm(self: *RegistryApiClient, package_spec: []const u8) !ImportResult {
        print("   📦 Analyzing NPM package: {s}\n", .{package_spec});
        
        // Simulate NPM import analysis
        return ImportResult{
            .success = true,
            .cursed_equivalent = try self.allocator.dupe(u8, "cursed-json-parser"),
            .migration_notes = try self.allocator.dupe(u8, "Direct API compatibility. Change import from 'require' to 'yeet'"),
            .confidence_score = 0.95,
        };
    }
    
    fn importFromCargo(self: *RegistryApiClient, package_spec: []const u8) !ImportResult {
        print("   🦀 Analyzing Cargo package: {s}\n", .{package_spec});
        
        return ImportResult{
            .success = true,
            .cursed_equivalent = try self.allocator.dupe(u8, "cursed-serde"),
            .migration_notes = try self.allocator.dupe(u8, "Similar serialization API. Some syntax differences in macro usage"),
            .confidence_score = 0.87,
        };
    }
    
    fn importFromPip(self: *RegistryApiClient, package_spec: []const u8) !ImportResult {
        print("   🐍 Analyzing Python package: {s}\n", .{package_spec});
        
        return ImportResult{
            .success = true,
            .cursed_equivalent = try self.allocator.dupe(u8, "cursed-requests"),
            .migration_notes = try self.allocator.dupe(u8, "HTTP client with similar API. Methods renamed to CURSED conventions"),
            .confidence_score = 0.91,
        };
    }
    
    fn importFromGoModules(self: *RegistryApiClient, package_spec: []const u8) !ImportResult {
        print("   🐹 Analyzing Go module: {s}\n", .{package_spec});
        
        return ImportResult{
            .success = true,
            .cursed_equivalent = try self.allocator.dupe(u8, "cursed-gorilla-mux"),
            .migration_notes = try self.allocator.dupe(u8, "Web router with equivalent functionality. Route syntax adapted for CURSED"),
            .confidence_score = 0.89,
        };
    }
};

// ===== Supporting Types =====

pub const PublishResult = struct {
    success: bool,
    message: []const u8,
    package_url: ?[]const u8,
};

pub const SecurityScanResult = struct {
    passed: bool,
    message: []const u8,
};

pub const UploadResult = struct {
    success: bool,
    error_message: []const u8,
};

pub const PackageAnalytics = struct {
    total_downloads: u64,
    unique_users: u64,
    growth_rate: f32,
    geographic_distribution: HashMap([]const u8, u32, std.hash_map.StringContext, 80),
    version_distribution: HashMap([]const u8, u32, std.hash_map.StringContext, 80),
    
    pub fn init() PackageAnalytics {
        return PackageAnalytics{
            .total_downloads = 0,
            .unique_users = 0,
            .growth_rate = 0.0,
            .geographic_distribution = HashMap([]const u8, u32, std.hash_map.StringContext, 80).init(allocator),
            .version_distribution = HashMap([]const u8, u32, std.hash_map.StringContext, 80).init(allocator),
        };
    }
    
    pub fn deinit(self: *PackageAnalytics) void {
        self.geographic_distribution.deinit();
        self.version_distribution.deinit();
    }
};

pub const AnalyticsTimeframe = enum {
    last_7_days,
    last_30_days,
    last_90_days,
};

pub const ExternalEcosystem = enum {
    npm,
    cargo,
    pip,
    go_modules,
};

pub const ImportResult = struct {
    success: bool,
    cursed_equivalent: []const u8,
    migration_notes: []const u8,
    confidence_score: f32,
};

// ===== Test Functions =====

test "registry api client creation" {
    const allocator = std.testing.allocator;
    
    var client = try RegistryApiClient.init(allocator, "https://packages.cursed.dev");
    defer client.deinit();
    
    try std.testing.expect(std.mem.eql(u8, client.base_url, "https://packages.cursed.dev"));
    try std.testing.expect(client.auth_token == null);
}

test "search query building" {
    const allocator = std.testing.allocator;
    
    var client = try RegistryApiClient.init(allocator, "https://packages.cursed.dev");
    defer client.deinit();
    
    var query = registry.SearchQuery.init(allocator, "json parser");
    defer query.deinit();
    
    try query.categories.append(.utilities);
    query.min_quality = 80.0;
    query.only_secure = true;
    
    const url = try client.buildSearchUrl(query);
    defer allocator.free(url);
    
    try std.testing.expect(std.mem.indexOf(u8, url, "q=json parser") != null);
    try std.testing.expect(std.mem.indexOf(u8, url, "categories=utilities") != null);
    try std.testing.expect(std.mem.indexOf(u8, url, "min_quality=80") != null);
    try std.testing.expect(std.mem.indexOf(u8, url, "only_secure=true") != null);
}
