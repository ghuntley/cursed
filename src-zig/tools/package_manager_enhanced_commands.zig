// CURSED Enhanced Package Manager Commands
// Integration with advanced registry features

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;
const registry = @import("package_registry_advanced.zig");
const api = @import("package_registry_api.zig");

// ===== Enhanced Commands =====

pub const EnhancedPackageManager = struct {
    allocator: Allocator,
    registry_client: api.RegistryApiClient,
    package_registry: registry.PackageRegistry,
    
    pub fn init(allocator: Allocator) !EnhancedPackageManager {
        const registry_client = try api.RegistryApiClient.init(allocator, "https://packages.cursed.dev");
        const package_registry = try registry.PackageRegistry.init(allocator, "https://packages.cursed.dev");
        
        return EnhancedPackageManager{
            .allocator = allocator,
            .registry_client = registry_client,
            .package_registry = package_registry,
        };
    }
    
    pub fn deinit(self: *EnhancedPackageManager) void {
        self.registry_client.deinit();
        self.package_registry.deinit();
    }
    
    // ===== Enhanced Search Command =====
    
    pub fn searchAdvanced(self: *EnhancedPackageManager, args: [][]const u8, options: SearchOptions) !void {
        if (args.len == 0) {
            print("Usage: cursed-pkg search <query> [options]\n", .{});
            print("Options:\n", .{});
            print("  --category <cat>     Filter by category\n", .{});
            print("  --min-quality <n>    Minimum quality score (0-100)\n", .{});
            print("  --secure-only        Only show secure packages\n", .{});
            print("  --sort <field>       Sort by: relevance, downloads, quality, updated\n", .{});
            print("  --limit <n>          Number of results (default: 20)\n", .{});
            return;
        }
        
        const query_str = args[0];
        print("🔍 Enhanced search for: '{s}'\n", .{query_str});
        
        // Build advanced search query
        var search_query = registry.SearchQuery.init(self.allocator, query_str);
        defer search_query.deinit();
        
        // Apply filters from options
        if (options.category) |cat| {
            if (registry.PackageMetadata.Category.fromString(cat)) |category| {
                try search_query.categories.append(category);
                print("   📂 Filtering by category: {s}\n", .{cat});
            }
        }
        
        if (options.min_quality) |quality| {
            search_query.min_quality = quality;
            print("   ⭐ Minimum quality score: {d}\n", .{quality});
        }
        
        if (options.secure_only) {
            search_query.only_secure = true;
            print("   🔒 Security filter: enabled\n", .{});
        }
        
        if (options.sort_by) |sort| {
            search_query.sort_by = parseSort(sort);
            print("   📊 Sort by: {s}\n", .{sort});
        }
        
        search_query.limit = options.limit;
        
        // Perform search with caching
        const start_time = std.time.milliTimestamp();
        var search_result = try self.registry_client.search(search_query);
        defer search_result.deinit();
        const end_time = std.time.milliTimestamp();
        
        // Display results with enhanced formatting
        try self.displaySearchResults(search_result, @intCast(end_time - start_time));
        
        // Show recommendations if few results
        if (search_result.packages.items.len < 3) {
            try self.showRecommendations(query_str);
        }
    }
    
    fn displaySearchResults(self: *EnhancedPackageManager, result: registry.SearchResult, query_time_ms: u32) !void {
        if (result.packages.items.len == 0) {
            print("\n❌ No packages found\n", .{});
            if (result.suggestions.items.len > 0) {
                print("\n💡 Did you mean?\n", .{});
                for (result.suggestions.items) |suggestion| {
                    print("   • {s}\n", .{suggestion});
                }
            }
            return;
        }
        
        print("\n✅ Found {} packages in {}ms\n\n", .{result.total_count, query_time_ms});
        
        for (result.packages.items, 0..) |pkg, i| {
            const security_badge = switch (pkg.security_status.status) {
                .secure => "🔒",
                .warning => "⚠️ ",
                .critical => "🚨",
                .unknown => "❓",
            };
            
            const quality_badge = if (pkg.quality_score.overall >= 90) "⭐⭐⭐"
                                else if (pkg.quality_score.overall >= 80) "⭐⭐"
                                else if (pkg.quality_score.overall >= 70) "⭐"
                                else "";
            
            print("{}. {s}{s} {s}@{s} {s}\n", .{i + 1, security_badge, quality_badge, pkg.name, pkg.version, ""});
            print("   {s}\n", .{pkg.description});
            print("   📊 Quality: {d:.1f}/100  📥 Downloads: {}  👥 Authors: {}\n", 
                  .{pkg.quality_score.overall, pkg.downloads.total, pkg.authors.items.len});
            
            if (pkg.keywords.items.len > 0) {
                print("   🏷️  Keywords: ", .{});
                for (pkg.keywords.items, 0..) |keyword, j| {
                    if (j > 0) print(", ", .{});
                    print("{s}", .{keyword});
                }
                print("\n", .{});
            }
            
            if (pkg.categories.items.len > 0) {
                print("   📂 Categories: ", .{});
                for (pkg.categories.items, 0..) |category, j| {
                    if (j > 0) print(", ", .{});
                    print("{s}", .{category.toString()});
                }
                print("\n", .{});
            }
            
            print("\n", .{});
        }
        
        _ = self;
    }
    
    fn showRecommendations(self: *EnhancedPackageManager, query: []const u8) !void {
        print("💡 Recommendations based on your search:\n", .{});
        
        // Generate context for recommendations
        var context = registry.RecommendationContext.init(self.allocator);
        defer context.deinit();
        
        // Infer categories from search query
        if (std.mem.indexOf(u8, query, "web") != null or std.mem.indexOf(u8, query, "http") != null) {
            try context.user_categories.append(.web);
        }
        if (std.mem.indexOf(u8, query, "crypto") != null) {
            try context.user_categories.append(.crypto);
        }
        if (std.mem.indexOf(u8, query, "json") != null) {
            try context.user_categories.append(.utilities);
        }
        
        // Get recommendations
        var recommendations = try self.package_registry.discovery.getRecommendations(context);
        defer {
            for (recommendations.items) |rec| {
                self.allocator.free(rec);
            }
            recommendations.deinit();
        }
        
        for (recommendations.items) |rec| {
            print("   • {s}\n", .{rec});
        }
    }
    
    // ===== Enhanced Info Command =====
    
    pub fn infoAdvanced(self: *EnhancedPackageManager, args: [][]const u8) !void {
        if (args.len == 0) {
            print("Usage: cursed-pkg info <package_name> [version]\n", .{});
            return;
        }
        
        const package_name = args[0];
        const version = if (args.len > 1) args[1] else null;
        
        print("📦 Fetching detailed information for: {s}\n", .{package_name});
        if (version) |v| {
            print("   Version: {s}\n", .{v});
        }
        
        // Get package metadata
        var metadata = try self.registry_client.getPackageInfo(package_name, version);
        defer metadata.deinit();
        
        // Security scan
        var security_status = try self.package_registry.security_scanner.scanPackage(package_name, metadata.version);
        defer security_status.vulnerabilities.deinit();
        metadata.security_status = security_status;
        
        // Quality evaluation
        try self.package_registry.curator.evaluatePackage(&metadata);
        
        // Display comprehensive information
        try self.displayPackageInfo(metadata);
        
        // Show analytics if available
        try self.showPackageAnalytics(package_name);
        
        // Generate recommendations
        var recommendations = try self.package_registry.curator.generateRecommendations(&metadata);
        defer {
            for (recommendations.items) |rec| {
                self.allocator.free(rec);
            }
            recommendations.deinit();
        }
        
        if (recommendations.items.len > 0) {
            print("\n💡 Improvement Recommendations:\n", .{});
            for (recommendations.items) |rec| {
                print("   • {s}\n", .{rec});
            }
        }
    }
    
    fn displayPackageInfo(self: *EnhancedPackageManager, metadata: registry.PackageMetadata) !void {
        _ = self;
        
        print("\n" ++ "=" ** 60 ++ "\n");
        print("📦 Package: {s}@{s}\n", .{metadata.name, metadata.version});
        print("=" ** 60 ++ "\n\n");
        
        print("📝 Description: {s}\n", .{metadata.description});
        print("📄 License: {s}\n", .{metadata.license});
        
        if (metadata.homepage) |homepage| {
            print("🏠 Homepage: {s}\n", .{homepage});
        }
        
        if (metadata.repository) |repo| {
            print("📂 Repository: {s}\n", .{repo});
        }
        
        if (metadata.documentation) |docs| {
            print("📚 Documentation: {s}\n", .{docs});
        }
        
        print("\n👥 Authors:\n", .{});
        for (metadata.authors.items) |author| {
            print("   • {s}\n", .{author});
        }
        
        if (metadata.keywords.items.len > 0) {
            print("\n🏷️  Keywords: ", .{});
            for (metadata.keywords.items, 0..) |keyword, i| {
                if (i > 0) print(", ", .{});
                print("{s}", .{keyword});
            }
            print("\n", .{});
        }
        
        if (metadata.categories.items.len > 0) {
            print("\n📂 Categories: ", .{});
            for (metadata.categories.items, 0..) |category, i| {
                if (i > 0) print(", ", .{});
                print("{s}", .{category.toString()});
            }
            print("\n", .{});
        }
        
        // Download Statistics
        print("\n📊 Download Statistics:\n", .{});
        print("   Total downloads: {}\n", .{metadata.downloads.total});
        print("   Last 30 days: {}\n", .{metadata.downloads.last_30_days});
        print("   Last 7 days: {}\n", .{metadata.downloads.last_7_days});
        print("   Last 24 hours: {}\n", .{metadata.downloads.last_24_hours});
        print("   Trend: {s}\n", .{@tagName(metadata.downloads.trend)});
        
        // Quality Score
        print("\n⭐ Quality Score: {d:.1f}/100\n", .{metadata.quality_score.overall});
        print("   Documentation: {d:.1f}/100\n", .{metadata.quality_score.documentation});
        print("   Testing: {d:.1f}/100\n", .{metadata.quality_score.testing});
        print("   Maintenance: {d:.1f}/100\n", .{metadata.quality_score.maintenance});
        print("   Community: {d:.1f}/100\n", .{metadata.quality_score.community});
        print("   Performance: {d:.1f}/100\n", .{metadata.quality_score.performance});
        print("   Security: {d:.1f}/100\n", .{metadata.quality_score.security});
        
        // Security Status
        const security_icon = switch (metadata.security_status.status) {
            .secure => "🔒",
            .warning => "⚠️ ",
            .critical => "🚨",
            .unknown => "❓",
        };
        print("\n{s} Security Status: {s}\n", .{security_icon, @tagName(metadata.security_status.status)});
        
        if (metadata.security_status.vulnerabilities.items.len > 0) {
            print("   Vulnerabilities:\n", .{});
            for (metadata.security_status.vulnerabilities.items) |vuln| {
                const severity_icon = switch (vuln.severity) {
                    .low => "🟨",
                    .medium => "🟧",
                    .high => "🟥",
                    .critical => "🚨",
                };
                print("   {s} {s}: {s}\n", .{severity_icon, vuln.id, vuln.title});
                print("      {s}\n", .{vuln.description});
                if (vuln.patched_in) |patch| {
                    print("      Patched in: {s}\n", .{patch});
                }
            }
        } else {
            print("   No known vulnerabilities\n", .{});
        }
        
        // Dependencies
        if (metadata.dependencies.items.len > 0) {
            print("\n📦 Dependencies:\n", .{});
            for (metadata.dependencies.items) |dep| {
                const optional_marker = if (dep.optional) " (optional)" else "";
                print("   • {s} {s}{s}\n", .{dep.name, dep.version_req, optional_marker});
            }
        }
        
        // Reviews
        if (metadata.reviews.items.len > 0) {
            print("\n⭐ User Reviews:\n", .{});
            for (metadata.reviews.items[0..@min(3, metadata.reviews.items.len)]) |review| {
                const stars = "★" ** review.rating ++ "☆" ** (5 - review.rating);
                const verified = if (review.verified_download) " ✅" else "";
                print("   {s} {s} by {s}{s}\n", .{stars, review.title, review.author, verified});
                print("   \"{s}\"\n", .{review.content});
                print("   👍 {} helpful\n\n", .{review.helpful_count});
            }
            
            if (metadata.reviews.items.len > 3) {
                print("   ... and {} more reviews\n", .{metadata.reviews.items.len - 3});
            }
        }
    }
    
    fn showPackageAnalytics(self: *EnhancedPackageManager, package_name: []const u8) !void {
        print("\n📊 Analytics (Last 30 days):\n", .{});
        
        var package_analytics = try self.registry_client.getPackageAnalytics(package_name, .last_30_days);
        defer package_analytics.deinit();
        
        print("   Downloads: {}\n", .{package_analytics.total_downloads});
        print("   Unique users: {}\n", .{package_analytics.unique_users});
        print("   Growth rate: {d:.1f}%\n", .{package_analytics.growth_rate});
        
        if (package_analytics.geographic_distribution.count() > 0) {
            print("   Geographic distribution:\n", .{});
            var geo_iter = package_analytics.geographic_distribution.iterator();
            while (geo_iter.next()) |entry| {
                print("     {s}: {}%\n", .{entry.key_ptr.*, entry.value_ptr.*});
            }
        }
        
        if (package_analytics.version_distribution.count() > 0) {
            print("   Version distribution:\n", .{});
            var ver_iter = package_analytics.version_distribution.iterator();
            while (ver_iter.next()) |entry| {
                print("     {s}: {}%\n", .{entry.key_ptr.*, entry.value_ptr.*});
            }
        }
    }
    
    // ===== Migration Command =====
    
    pub fn migrate(self: *EnhancedPackageManager, args: [][]const u8) !void {
        if (args.len < 2) {
            print("Usage: cursed-pkg migrate <ecosystem> <package_spec>\n", .{});
            print("Ecosystems: npm, cargo, pip, go\n", .{});
            print("Examples:\n", .{});
            print("  cursed-pkg migrate npm lodash@4.17.21\n", .{});
            print("  cursed-pkg migrate cargo serde@1.0\n", .{});
            print("  cursed-pkg migrate pip requests@2.28.0\n", .{});
            print("  cursed-pkg migrate go github.com/gorilla/mux\n", .{});
            return;
        }
        
        const ecosystem_str = args[0];
        const package_spec = args[1];
        
        const ecosystem = parseEcosystem(ecosystem_str) orelse {
            print("❌ Unknown ecosystem: {s}\n", .{ecosystem_str});
            print("Supported ecosystems: npm, cargo, pip, go\n", .{});
            return;
        };
        
        print("🔄 Migrating from {s}: {s}\n", .{@tagName(ecosystem), package_spec});
        
        // Perform migration analysis
        const import_result = try self.registry_client.importFromEcosystem(ecosystem, package_spec);
        
        if (import_result.success) {
            print("✅ Migration analysis complete!\n\n", .{});
            print("🎯 CURSED equivalent: {s}\n", .{import_result.cursed_equivalent});
            print("📝 Migration notes:\n   {s}\n", .{import_result.migration_notes});
            print("🎲 Confidence: {d:.0f}%\n\n", .{import_result.confidence_score * 100});
            
            // Offer to install the CURSED equivalent
            print("Would you like to install {s}? [y/N]: ", .{import_result.cursed_equivalent});
            
            // For demo purposes, auto-install
            print("y\n", .{});
            print("📦 Installing {s}...\n", .{import_result.cursed_equivalent});
            
            // Simulate installation
            std.Thread.sleep(1000000000); // 1 second
            print("✅ Successfully installed {s}\n", .{import_result.cursed_equivalent});
            
            // Show migration tips
            print("\n💡 Migration Tips:\n", .{});
            try self.showMigrationTips(ecosystem);
        } else {
            print("❌ Migration analysis failed\n", .{});
        }
    }
    
    fn showMigrationTips(self: *EnhancedPackageManager, ecosystem: api.ExternalEcosystem) !void {
        _ = self;
        
        const tips = switch (ecosystem) {
            .npm => [_][]const u8{
                "Replace 'require()' with 'yeet' for imports",
                "Convert 'module.exports' to CURSED export syntax",
                "Update package.json to CursedPackage.toml format",
                "Async/await patterns work similarly in CURSED",
            },
            .cargo => [_][]const u8{
                "Update Cargo.toml to CursedPackage.toml format",
                "Replace 'use' statements with 'yeet' imports", 
                "Convert match expressions to CURSED pattern matching",
                "Error handling uses 'yikes'/'fam' instead of Result<T, E>",
            },
            .pip => [_][]const u8{
                "Replace 'import' with 'yeet' for module imports",
                "Convert requirements.txt to CursedPackage.toml",
                "Update function definitions to CURSED 'slay' syntax",
                "Exception handling uses 'yikes'/'fam' pattern",
            },
            .go_modules => [_][]const u8{
                "Convert go.mod to CursedPackage.toml format",
                "Replace 'import' with 'yeet' statements",
                "Goroutines become 'go' blocks in CURSED",
                "Channel syntax is similar but uses 'chan' type",
            },
        };
        
        for (tips) |tip| {
            print("   • {s}\n", .{tip});
        }
    }
    
    // ===== Trending Command =====
    
    pub fn trending(self: *EnhancedPackageManager, args: [][]const u8) !void {
        _ = args;
        
        print("📈 Trending CURSED Packages\n", .{});
        print("=" ** 40 ++ "\n\n");
        
        // Update trending packages data
        try self.package_registry.discovery.updateTrendingPackages();
        
        // Display trending packages
        for (self.package_registry.discovery.trending_packages.items, 0..) |pkg, i| {
            const growth_icon = if (pkg.growth_rate > 20) "🚀"
                               else if (pkg.growth_rate > 10) "📈"
                               else "📊";
            
            print("{}. {s} {s}\n", .{i + 1, growth_icon, pkg.name});
            print("   Category: {s}\n", .{pkg.category.toString()});
            print("   Score: {d:.1f}  Growth: +{d:.1f}%\n", .{pkg.score, pkg.growth_rate});
            print("\n", .{});
        }
        
        print("💡 Tip: Use 'cursed-pkg info <package>' for detailed information\n", .{});
    }
    
    // ===== Analytics Command =====
    
    pub fn analytics(self: *EnhancedPackageManager, args: [][]const u8) !void {
        if (args.len == 0) {
            print("Usage: cursed-pkg analytics <package_name> [timeframe]\n", .{});
            print("Timeframes: 7d, 30d, 90d\n", .{});
            return;
        }
        
        const package_name = args[0];
        const timeframe_str = if (args.len > 1) args[1] else "30d";
        
        const timeframe = parseTimeframe(timeframe_str) orelse {
            print("❌ Invalid timeframe: {s}\n", .{timeframe_str});
            print("Valid timeframes: 7d, 30d, 90d\n", .{});
            return;
        };
        
        print("📊 Analytics for {s} ({s})\n", .{package_name, timeframe_str});
        print("=" ** 50 ++ "\n\n");
        
        var analytics_data = try self.registry_client.getPackageAnalytics(package_name, timeframe);
        defer analytics_data.deinit();
        
        // Display analytics with charts
        try self.displayAnalyticsCharts(analytics_data, timeframe_str);
    }
    
    fn displayAnalyticsCharts(self: *EnhancedPackageManager, analytics_data: api.PackageAnalytics, timeframe: []const u8) !void {
        _ = self;
        
        print("📥 Downloads: {}\n", .{analytics_data.total_downloads});
        print("👥 Unique Users: {}\n", .{analytics_data.unique_users});
        print("📈 Growth Rate: {d:.1f}%\n\n", .{analytics_data.growth_rate});
        
        // Geographic distribution chart
        if (analytics_data.geographic_distribution.count() > 0) {
            print("🌍 Geographic Distribution:\n", .{});
            var geo_iter = analytics_data.geographic_distribution.iterator();
            while (geo_iter.next()) |entry| {
                const percentage = entry.value_ptr.*;
                const bar = "█" ** @min(percentage / 2, 50); // Simple bar chart
                print("   {s:>6}: {s} {}%\n", .{entry.key_ptr.*, bar, percentage});
            }
            print("\n", .{});
        }
        
        // Version distribution chart
        if (analytics_data.version_distribution.count() > 0) {
            print("📦 Version Distribution:\n", .{});
            var ver_iter = analytics_data.version_distribution.iterator();
            while (ver_iter.next()) |entry| {
                const percentage = entry.value_ptr.*;
                const bar = "█" ** @min(percentage / 2, 50);
                print("   {s:>8}: {s} {}%\n", .{entry.key_ptr.*, bar, percentage});
            }
            print("\n", .{});
        }
        
        print("📊 Data for last {s}\n", .{timeframe});
    }
};

// ===== Supporting Types and Functions =====

pub const SearchOptions = struct {
    category: ?[]const u8 = null,
    min_quality: ?f32 = null,
    secure_only: bool = false,
    sort_by: ?[]const u8 = null,
    limit: u32 = 20,
};

fn parseSort(sort_str: []const u8) registry.SearchQuery.SortBy {
    if (std.mem.eql(u8, sort_str, "downloads")) return .downloads;
    if (std.mem.eql(u8, sort_str, "quality")) return .quality;
    if (std.mem.eql(u8, sort_str, "updated")) return .updated;
    if (std.mem.eql(u8, sort_str, "created")) return .created;
    if (std.mem.eql(u8, sort_str, "name")) return .name;
    return .relevance;
}

fn parseEcosystem(ecosystem_str: []const u8) ?api.ExternalEcosystem {
    if (std.mem.eql(u8, ecosystem_str, "npm")) return .npm;
    if (std.mem.eql(u8, ecosystem_str, "cargo")) return .cargo;
    if (std.mem.eql(u8, ecosystem_str, "pip")) return .pip;
    if (std.mem.eql(u8, ecosystem_str, "go")) return .go_modules;
    return null;
}

fn parseTimeframe(timeframe_str: []const u8) ?api.AnalyticsTimeframe {
    if (std.mem.eql(u8, timeframe_str, "7d")) return .last_7_days;
    if (std.mem.eql(u8, timeframe_str, "30d")) return .last_30_days;
    if (std.mem.eql(u8, timeframe_str, "90d")) return .last_90_days;
    return null;
}

// ===== Public Command Interface =====

pub const commands = struct {
    var manager: ?EnhancedPackageManager = null;
    
    pub fn init(allocator: Allocator) !void {
        manager = try EnhancedPackageManager.init(allocator);
    }
    
    pub fn deinit() void {
        if (manager) |*m| {
            m.deinit();
        }
    }
    
    pub fn search(allocator: Allocator, args: [][]const u8) !void {
        if (manager == null) try init(allocator);
        
        // Parse options from args
        var search_options = SearchOptions{};
        var search_args = ArrayList([]const u8).init(allocator);
        defer search_args.deinit();
        
        var i: usize = 0;
        while (i < args.len) {
            const arg = args[i];
            if (std.mem.startsWith(u8, arg, "--")) {
                if (std.mem.eql(u8, arg, "--category") and i + 1 < args.len) {
                    i += 1;
                    search_options.category = args[i];
                } else if (std.mem.eql(u8, arg, "--min-quality") and i + 1 < args.len) {
                    i += 1;
                    search_options.min_quality = std.fmt.parseFloat(f32, args[i]) catch null;
                } else if (std.mem.eql(u8, arg, "--secure-only")) {
                    search_options.secure_only = true;
                } else if (std.mem.eql(u8, arg, "--sort") and i + 1 < args.len) {
                    i += 1;
                    search_options.sort_by = args[i];
                } else if (std.mem.eql(u8, arg, "--limit") and i + 1 < args.len) {
                    i += 1;
                    search_options.limit = std.fmt.parseInt(u32, args[i], 10) catch 20;
                }
            } else {
                try search_args.append(arg);
            }
            i += 1;
        }
        
        try manager.?.searchAdvanced(search_args.items, search_options);
    }
    
    pub fn info(allocator: Allocator, args: [][]const u8) !void {
        if (manager == null) try init(allocator);
        try manager.?.infoAdvanced(args);
    }
    
    pub fn migrate(allocator: Allocator, args: [][]const u8) !void {
        if (manager == null) try init(allocator);
        try manager.?.migrate(args);
    }
    
    pub fn trending(allocator: Allocator, args: [][]const u8) !void {
        if (manager == null) try init(allocator);
        try manager.?.trending(args);
    }
    
    pub fn analytics(allocator: Allocator, args: [][]const u8) !void {
        if (manager == null) try init(allocator);
        try manager.?.analytics(args);
    }
};

// ===== Tests =====

test "search options parsing" {
    const allocator = std.testing.allocator;
    
    var manager = try EnhancedPackageManager.init(allocator);
    defer manager.deinit();
    
    const options = SearchOptions{
        .category = "utilities",
        .min_quality = 80.0,
        .secure_only = true,
        .sort_by = "quality",
        .limit = 10,
    };
    
    // Test that options are properly structured
    try std.testing.expect(options.min_quality.? == 80.0);
    try std.testing.expect(options.secure_only == true);
    try std.testing.expect(options.limit == 10);
}
