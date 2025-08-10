// CURSED Advanced Package Registry System
// Comprehensive package management ecosystem with discovery, curation, and security

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;
const json = std.json;
const http = std.http;

// ===== Registry Core Types =====

pub const PackageRegistry = struct {
    allocator: Allocator,
    registry_url: []const u8,
    auth_token: ?[]const u8,
    cache: RegistryCache,
    analytics: AnalyticsEngine,
    security_scanner: SecurityScanner,
    curator: PackageCurator,
    discovery: DiscoveryEngine,
    
    pub fn init(allocator: Allocator, registry_url: []const u8) !PackageRegistry {
        return PackageRegistry{
            .allocator = allocator,
            .registry_url = try allocator.dupe(u8, registry_url),
            .auth_token = null,
            .cache = RegistryCache.init(allocator),
            .analytics = AnalyticsEngine.init(allocator),
            .security_scanner = SecurityScanner.init(allocator),
            .curator = PackageCurator.init(allocator),
            .discovery = DiscoveryEngine.init(allocator),
        };
    }
    
    pub fn deinit(self: *PackageRegistry) void {
        self.allocator.free(self.registry_url);
        if (self.auth_token) |token| self.allocator.free(token);
        self.cache.deinit();
        self.analytics.deinit();
        self.security_scanner.deinit();
        self.curator.deinit();
        self.discovery.deinit();
    }
    
    pub fn setAuthToken(self: *PackageRegistry, token: []const u8) !void {
        if (self.auth_token) |old_token| self.allocator.free(old_token);
        self.auth_token = try self.allocator.dupe(u8, token);
    }
};

// ===== Package Metadata =====

pub const PackageMetadata = struct {
    name: []const u8,
    version: []const u8,
    description: []const u8,
    authors: ArrayList([]const u8),
    license: []const u8,
    repository: ?[]const u8,
    homepage: ?[]const u8,
    documentation: ?[]const u8,
    keywords: ArrayList([]const u8),
    categories: ArrayList(Category),
    dependencies: ArrayList(Dependency),
    downloads: DownloadStats,
    quality_score: QualityScore,
    security_status: SecurityStatus,
    reviews: ArrayList(Review),
    created_at: i64,
    updated_at: i64,
    
    pub const Category = enum {
        web,
        cli,
        database,
        crypto,
        networking,
        parsing,
        graphics,
        audio,
        games,
        development_tools,
        testing,
        utilities,
        algorithms,
        data_structures,
        machine_learning,
        science,
        mathematics,
        
        pub fn toString(self: Category) []const u8 {
            return switch (self) {
                .web => "web",
                .cli => "cli",
                .database => "database",
                .crypto => "crypto",
                .networking => "networking",
                .parsing => "parsing",
                .graphics => "graphics",
                .audio => "audio",
                .games => "games",
                .development_tools => "development-tools",
                .testing => "testing",
                .utilities => "utilities",
                .algorithms => "algorithms",
                .data_structures => "data-structures",
                .machine_learning => "machine-learning",
                .science => "science",
                .mathematics => "mathematics",
            };
        }
        
        pub fn fromString(str: []const u8) ?Category {
            const categories = std.StaticStringMap(Category).initComptime(.{
                .{ "web", .web },
                .{ "cli", .cli },
                .{ "database", .database },
                .{ "crypto", .crypto },
                .{ "networking", .networking },
                .{ "parsing", .parsing },
                .{ "graphics", .graphics },
                .{ "audio", .audio },
                .{ "games", .games },
                .{ "development-tools", .development_tools },
                .{ "testing", .testing },
                .{ "utilities", .utilities },
                .{ "algorithms", .algorithms },
                .{ "data-structures", .data_structures },
                .{ "machine-learning", .machine_learning },
                .{ "science", .science },
                .{ "mathematics", .mathematics },
            });
            return categories.get(str);
        }
    };
    
    pub const Dependency = struct {
        name: []const u8,
        version_req: []const u8,
        optional: bool = false,
        features: ArrayList([]const u8),
    };
    
    pub const DownloadStats = struct {
        total: u64,
        last_30_days: u64,
        last_7_days: u64,
        last_24_hours: u64,
        peak_daily: u64,
        trend: Trend,
        
        pub const Trend = enum { increasing, stable, decreasing };
    };
    
    pub const QualityScore = struct {
        overall: f32, // 0.0 to 100.0
        documentation: f32,
        testing: f32,
        maintenance: f32,
        community: f32,
        performance: f32,
        security: f32,
        
        pub fn calculate(self: *QualityScore) void {
            self.overall = (self.documentation + self.testing + self.maintenance + 
                           self.community + self.performance + self.security) / 6.0;
        }
    };
    
    pub const SecurityStatus = struct {
        status: Status,
        vulnerabilities: ArrayList(Vulnerability),
        last_scan: i64,
        scan_version: []const u8,
        
        pub const Status = enum { secure, warning, critical, unknown };
        
        pub const Vulnerability = struct {
            id: []const u8,
            severity: Severity,
            title: []const u8,
            description: []const u8,
            affected_versions: []const u8,
            patched_in: ?[]const u8,
            
            pub const Severity = enum { low, medium, high, critical };
        };
    };
    
    pub const Review = struct {
        id: []const u8,
        author: []const u8,
        rating: u8, // 1-5 stars
        title: []const u8,
        content: []const u8,
        helpful_count: u32,
        created_at: i64,
        verified_download: bool,
    };
    
    pub fn init(allocator: Allocator) PackageMetadata {
        return PackageMetadata{
            .name = "",
            .version = "",
            .description = "",
            .authors = ArrayList([]const u8).init(allocator),
            .license = "",
            .repository = null,
            .homepage = null,
            .documentation = null,
            .keywords = ArrayList([]const u8).init(allocator),
            .categories = ArrayList(Category).init(allocator),
            .dependencies = ArrayList(Dependency).init(allocator),
            .downloads = DownloadStats{ .total = 0, .last_30_days = 0, .last_7_days = 0, 
                                     .last_24_hours = 0, .peak_daily = 0, .trend = .stable },
            .quality_score = QualityScore{ .overall = 0, .documentation = 0, .testing = 0, 
                                         .maintenance = 0, .community = 0, .performance = 0, .security = 0 },
            .security_status = SecurityStatus{ .status = .unknown, .vulnerabilities = ArrayList(SecurityStatus.Vulnerability).init(allocator),
                                             .last_scan = 0, .scan_version = "" },
            .reviews = ArrayList(Review).init(allocator),
            .created_at = std.time.timestamp(),
            .updated_at = std.time.timestamp(),
        };
    }
    
    pub fn deinit(self: *PackageMetadata) void {
        self.authors.deinit();
        self.keywords.deinit();
        self.categories.deinit();
        self.dependencies.deinit();
        self.security_status.vulnerabilities.deinit();
        self.reviews.deinit();
    }
};

// ===== Registry Cache =====

pub const RegistryCache = struct {
    allocator: Allocator,
    package_cache: HashMap([]const u8, PackageMetadata, std.hash_map.StringContext, 80),
    search_cache: HashMap([]const u8, SearchResult, std.hash_map.StringContext, 80),
    ttl_cache: HashMap([]const u8, i64, std.hash_map.StringContext, 80),
    cache_duration: i64 = 3600, // 1 hour in seconds
    
    pub fn init(allocator: Allocator) RegistryCache {
        return RegistryCache{
            .allocator = allocator,
            .package_cache = HashMap([]const u8, PackageMetadata, std.hash_map.StringContext, 80).init(allocator),
            .search_cache = HashMap([]const u8, SearchResult, std.hash_map.StringContext, 80).init(allocator),
            .ttl_cache = HashMap([]const u8, i64, std.hash_map.StringContext, 80).init(allocator),
        };
    }
    
    pub fn deinit(self: *RegistryCache) void {
        // Clean up cached data
        var package_iter = self.package_cache.iterator();
        while (package_iter.next()) |entry| {
            var metadata = entry.value_ptr;
            metadata.deinit();
        }
        
        var search_iter = self.search_cache.iterator();
        while (search_iter.next()) |entry| {
            var result = entry.value_ptr;
            result.deinit();
        }
        
        self.package_cache.deinit();
        self.search_cache.deinit();
        self.ttl_cache.deinit();
    }
    
    pub fn get(self: *RegistryCache, key: []const u8) ?PackageMetadata {
        if (self.isExpired(key)) {
            self.invalidate(key);
            return null;
        }
        return self.package_cache.get(key);
    }
    
    pub fn put(self: *RegistryCache, key: []const u8, metadata: PackageMetadata) !void {
        const key_copy = try self.allocator.dupe(u8, key);
        try self.package_cache.put(key_copy, metadata);
        try self.ttl_cache.put(key_copy, std.time.timestamp());
    }
    
    fn isExpired(self: *RegistryCache, key: []const u8) bool {
        if (self.ttl_cache.get(key)) |timestamp| {
            return (std.time.timestamp() - timestamp) > self.cache_duration;
        }
        return true;
    }
    
    fn invalidate(self: *RegistryCache, key: []const u8) void {
        if (self.package_cache.getPtr(key)) |metadata| {
            metadata.deinit();
        }
        _ = self.package_cache.remove(key);
        _ = self.ttl_cache.remove(key);
    }
};

// ===== Search System =====

pub const SearchQuery = struct {
    query: []const u8,
    categories: ArrayList(PackageMetadata.Category),
    min_quality: ?f32 = null,
    only_secure: bool = false,
    sort_by: SortBy = .relevance,
    limit: u32 = 20,
    offset: u32 = 0,
    
    pub const SortBy = enum {
        relevance,
        downloads,
        updated,
        created,
        quality,
        name,
    };
    
    pub fn init(allocator: Allocator, query: []const u8) SearchQuery {
        return SearchQuery{
            .query = query,
            .categories = ArrayList(PackageMetadata.Category).init(allocator),
        };
    }
    
    pub fn deinit(self: *SearchQuery) void {
        self.categories.deinit();
    }
};

pub const SearchResult = struct {
    packages: ArrayList(PackageMetadata),
    total_count: u32,
    query_time_ms: u32,
    suggestions: ArrayList([]const u8),
    
    pub fn init(allocator: Allocator) SearchResult {
        return SearchResult{
            .packages = ArrayList(PackageMetadata).init(allocator),
            .total_count = 0,
            .query_time_ms = 0,
            .suggestions = ArrayList([]const u8).init(allocator),
        };
    }
    
    pub fn deinit(self: *SearchResult) void {
        for (self.packages.items) |*pkg| {
            pkg.deinit();
        }
        self.packages.deinit();
        self.suggestions.deinit();
    }
};

// ===== Security Scanner =====

pub const SecurityScanner = struct {
    allocator: Allocator,
    vulnerability_db: HashMap([]const u8, ArrayList(PackageMetadata.SecurityStatus.Vulnerability), std.hash_map.StringContext, 80),
    scan_policies: ScanPolicies,
    
    pub const ScanPolicies = struct {
        enable_dependency_scan: bool = true,
        enable_code_analysis: bool = true,
        enable_license_check: bool = true,
        enable_malware_detection: bool = true,
        quarantine_critical: bool = true,
    };
    
    pub fn init(allocator: Allocator) SecurityScanner {
        return SecurityScanner{
            .allocator = allocator,
            .vulnerability_db = HashMap([]const u8, ArrayList(PackageMetadata.SecurityStatus.Vulnerability), std.hash_map.StringContext, 80).init(allocator),
            .scan_policies = ScanPolicies{},
        };
    }
    
    pub fn deinit(self: *SecurityScanner) void {
        var iter = self.vulnerability_db.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.vulnerability_db.deinit();
    }
    
    pub fn scanPackage(self: *SecurityScanner, package_name: []const u8, version: []const u8) !PackageMetadata.SecurityStatus {
        print("🔍 Scanning package {s}@{s} for security vulnerabilities...\n", .{package_name, version});
        
        var status = PackageMetadata.SecurityStatus{
            .status = .secure,
            .vulnerabilities = ArrayList(PackageMetadata.SecurityStatus.Vulnerability).init(self.allocator),
            .last_scan = std.time.timestamp(),
            .scan_version = "1.0.0",
        };
        
        // Simulate vulnerability scanning
        if (std.mem.eql(u8, package_name, "vulnerable-package")) {
            const vuln = PackageMetadata.SecurityStatus.Vulnerability{
                .id = "CVE-2024-1234",
                .severity = .high,
                .title = "Buffer overflow in string handling",
                .description = "A buffer overflow vulnerability exists in the string handling code",
                .affected_versions = "< 1.2.3",
                .patched_in = "1.2.3",
            };
            try status.vulnerabilities.append(vuln);
            status.status = .warning;
        }
        
        // Code analysis simulation
        if (self.scan_policies.enable_code_analysis) {
            try self.performCodeAnalysis(package_name, &status);
        }
        
        // License validation
        if (self.scan_policies.enable_license_check) {
            try self.validateLicense(package_name, &status);
        }
        
        print("✅ Security scan completed: {s}\n", .{@tagName(status.status)});
        return status;
    }
    
    fn performCodeAnalysis(self: *SecurityScanner, package_name: []const u8, status: *PackageMetadata.SecurityStatus) !void {
        _ = self;
        _ = package_name;
        _ = status;
        // Simulate static code analysis
        print("   📊 Running static code analysis...\n", .{});
    }
    
    fn validateLicense(self: *SecurityScanner, package_name: []const u8, status: *PackageMetadata.SecurityStatus) !void {
        _ = self;
        _ = package_name;
        _ = status;
        // Simulate license validation
        print("   📜 Validating package license...\n", .{});
    }
};

// ===== Package Curator =====

pub const PackageCurator = struct {
    allocator: Allocator,
    curation_rules: ArrayList(CurationRule),
    quality_metrics: QualityMetrics,
    
    pub const CurationRule = struct {
        name: []const u8,
        description: []const u8,
        weight: f32,
        evaluator: *const fn(*PackageMetadata) f32,
    };
    
    pub const QualityMetrics = struct {
        documentation_weight: f32 = 0.20,
        testing_weight: f32 = 0.25,
        maintenance_weight: f32 = 0.20,
        community_weight: f32 = 0.15,
        performance_weight: f32 = 0.10,
        security_weight: f32 = 0.10,
    };
    
    pub fn init(allocator: Allocator) PackageCurator {
        var curator = PackageCurator{
            .allocator = allocator,
            .curation_rules = ArrayList(CurationRule).init(allocator),
            .quality_metrics = QualityMetrics{},
        };
        
        // Initialize default curation rules
        curator.initDefaultRules() catch {};
        
        return curator;
    }
    
    pub fn deinit(self: *PackageCurator) void {
        self.curation_rules.deinit();
    }
    
    fn initDefaultRules(self: *PackageCurator) !void {
        // Documentation rule
        try self.curation_rules.append(CurationRule{
            .name = "documentation",
            .description = "Evaluates documentation quality and completeness",
            .weight = 20.0,
            .evaluator = evaluateDocumentation,
        });
        
        // Testing rule
        try self.curation_rules.append(CurationRule{
            .name = "testing",
            .description = "Evaluates test coverage and quality",
            .weight = 25.0,
            .evaluator = evaluateTesting,
        });
        
        // Maintenance rule
        try self.curation_rules.append(CurationRule{
            .name = "maintenance",
            .description = "Evaluates maintenance activity and responsiveness",
            .weight = 20.0,
            .evaluator = evaluateMaintenance,
        });
    }
    
    pub fn evaluatePackage(self: *PackageCurator, metadata: *PackageMetadata) !void {
        print("🎯 Evaluating package quality: {s}\n", .{metadata.name});
        
        var total_score: f32 = 0.0;
        var total_weight: f32 = 0.0;
        
        for (self.curation_rules.items) |rule| {
            const score = rule.evaluator(metadata);
            total_score += score * rule.weight;
            total_weight += rule.weight;
            
            print("   📊 {s}: {d:.1f}/100\n", .{rule.name, score});
        }
        
        metadata.quality_score.overall = if (total_weight > 0) total_score / total_weight else 0.0;
        metadata.quality_score.calculate();
        
        print("✅ Overall quality score: {d:.1f}/100\n", .{metadata.quality_score.overall});
    }
    
    pub fn generateRecommendations(self: *PackageCurator, metadata: *PackageMetadata) !ArrayList([]const u8) {
        var recommendations = ArrayList([]const u8).init(self.allocator);
        
        if (metadata.quality_score.documentation < 60.0) {
            try recommendations.append("Improve documentation with more examples and API references");
        }
        
        if (metadata.quality_score.testing < 70.0) {
            try recommendations.append("Add more comprehensive test coverage");
        }
        
        if (metadata.quality_score.security < 80.0) {
            try recommendations.append("Address security vulnerabilities and improve secure coding practices");
        }
        
        if (metadata.keywords.items.len < 3) {
            try recommendations.append("Add more descriptive keywords to improve discoverability");
        }
        
        return recommendations;
    }
};

// Quality evaluation functions
fn evaluateDocumentation(metadata: *PackageMetadata) f32 {
    var score: f32 = 0.0;
    
    // Has description
    if (metadata.description.len > 10) score += 20.0;
    
    // Has documentation URL
    if (metadata.documentation != null) score += 30.0;
    
    // Has repository
    if (metadata.repository != null) score += 20.0;
    
    // Has keywords
    if (metadata.keywords.items.len >= 3) score += 30.0;
    
    return @min(score, 100.0);
}

fn evaluateTesting(metadata: *PackageMetadata) f32 {
    // Simulate test coverage analysis
    _ = metadata;
    return 85.0; // Mock score
}

fn evaluateMaintenance(metadata: *PackageMetadata) f32 {
    const now = std.time.timestamp();
    const days_since_update = @as(f32, @floatFromInt(now - metadata.updated_at)) / (24.0 * 3600.0);
    
    if (days_since_update < 30) return 100.0;
    if (days_since_update < 90) return 80.0;
    if (days_since_update < 180) return 60.0;
    if (days_since_update < 365) return 40.0;
    return 20.0;
}

// ===== Analytics Engine =====

pub const AnalyticsEngine = struct {
    allocator: Allocator,
    events: ArrayList(AnalyticsEvent),
    aggregated_stats: HashMap([]const u8, PackageStats, std.hash_map.StringContext, 80),
    
    pub const AnalyticsEvent = struct {
        event_type: EventType,
        package_name: []const u8,
        timestamp: i64,
        user_id: ?[]const u8,
        metadata: HashMap([]const u8, []const u8, std.hash_map.StringContext, 80),
        
        pub const EventType = enum {
            download,
            install,
            search,
            view,
            star,
            review,
        };
    };
    
    pub const PackageStats = struct {
        total_downloads: u64,
        unique_users: u64,
        daily_downloads: ArrayList(DailyStats),
        geographic_distribution: HashMap([]const u8, u64, std.hash_map.StringContext, 80),
        
        pub const DailyStats = struct {
            date: []const u8,
            downloads: u64,
            installs: u64,
            unique_users: u64,
        };
    };
    
    pub fn init(allocator: Allocator) AnalyticsEngine {
        return AnalyticsEngine{
            .allocator = allocator,
            .events = ArrayList(AnalyticsEvent).init(allocator),
            .aggregated_stats = HashMap([]const u8, PackageStats, std.hash_map.StringContext, 80).init(allocator),
        };
    }
    
    pub fn deinit(self: *AnalyticsEngine) void {
        for (self.events.items) |*event| {
            event.metadata.deinit();
        }
        self.events.deinit();
        
        var stats_iter = self.aggregated_stats.iterator();
        while (stats_iter.next()) |entry| {
            var stats = entry.value_ptr;
            stats.daily_downloads.deinit();
            stats.geographic_distribution.deinit();
        }
        self.aggregated_stats.deinit();
    }
    
    pub fn recordEvent(self: *AnalyticsEngine, event: AnalyticsEvent) !void {
        try self.events.append(event);
        try self.updateAggregatedStats(event);
    }
    
    fn updateAggregatedStats(self: *AnalyticsEngine, event: AnalyticsEvent) !void {
        if (self.aggregated_stats.getPtr(event.package_name)) |stats| {
            switch (event.event_type) {
                .download => stats.total_downloads += 1,
                .install => {},
                else => {},
            }
        } else {
            var new_stats = PackageStats{
                .total_downloads = if (event.event_type == .download) 1 else 0,
                .unique_users = 0,
                .daily_downloads = ArrayList(PackageStats.DailyStats).init(self.allocator),
                .geographic_distribution = HashMap([]const u8, u64, std.hash_map.StringContext, 80).init(self.allocator),
            };
            
            const key = try self.allocator.dupe(u8, event.package_name);
            try self.aggregated_stats.put(key, new_stats);
        }
    }
    
    pub fn generateInsights(self: *AnalyticsEngine, package_name: []const u8) !ArrayList([]const u8) {
        var insights = ArrayList([]const u8).init(self.allocator);
        
        if (self.aggregated_stats.get(package_name)) |stats| {
            if (stats.total_downloads > 1000) {
                try insights.append("Popular package with high download count");
            }
            
            if (stats.total_downloads > 0) {
                try insights.append("Active package with recent downloads");
            }
        } else {
            try insights.append("New package with limited usage data");
        }
        
        return insights;
    }
};

// ===== Discovery Engine =====

pub const DiscoveryEngine = struct {
    allocator: Allocator,
    recommendation_cache: HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, 80),
    trending_packages: ArrayList(TrendingPackage),
    
    pub const TrendingPackage = struct {
        name: []const u8,
        score: f32,
        growth_rate: f32,
        category: PackageMetadata.Category,
    };
    
    pub fn init(allocator: Allocator) DiscoveryEngine {
        return DiscoveryEngine{
            .allocator = allocator,
            .recommendation_cache = HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, 80).init(allocator),
            .trending_packages = ArrayList(TrendingPackage).init(allocator),
        };
    }
    
    pub fn deinit(self: *DiscoveryEngine) void {
        var iter = self.recommendation_cache.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.recommendation_cache.deinit();
        self.trending_packages.deinit();
    }
    
    pub fn getRecommendations(self: *DiscoveryEngine, context: RecommendationContext) !ArrayList([]const u8) {
        print("🔍 Generating package recommendations...\n", .{});
        
        var recommendations = ArrayList([]const u8).init(self.allocator);
        
        // Category-based recommendations
        for (context.user_categories.items) |category| {
            const category_recs = try self.getCategoryRecommendations(category);
            defer category_recs.deinit();
            
            for (category_recs.items) |rec| {
                try recommendations.append(try self.allocator.dupe(u8, rec));
            }
        }
        
        // Usage pattern recommendations
        if (context.current_dependencies.items.len > 0) {
            const pattern_recs = try self.getPatternRecommendations(context.current_dependencies);
            defer pattern_recs.deinit();
            
            for (pattern_recs.items) |rec| {
                try recommendations.append(try self.allocator.dupe(u8, rec));
            }
        }
        
        return recommendations;
    }
    
    fn getCategoryRecommendations(self: *DiscoveryEngine, category: PackageMetadata.Category) !ArrayList([]const u8) {
        var recommendations = ArrayList([]const u8).init(self.allocator);
        
        // Simulate category-based recommendations
        switch (category) {
            .web => {
                try recommendations.append("http-client");
                try recommendations.append("json-parser");
                try recommendations.append("web-framework");
            },
            .cli => {
                try recommendations.append("arg-parser");
                try recommendations.append("terminal-colors");
                try recommendations.append("progress-bar");
            },
            .crypto => {
                try recommendations.append("hash-algorithms");
                try recommendations.append("encryption-utils");
                try recommendations.append("secure-random");
            },
            else => {
                try recommendations.append("utility-library");
            },
        }
        
        return recommendations;
    }
    
    fn getPatternRecommendations(self: *DiscoveryEngine, dependencies: ArrayList([]const u8)) !ArrayList([]const u8) {
        _ = self;
        var recommendations = ArrayList([]const u8).init(self.allocator);
        
        // Analyze dependency patterns
        var has_web = false;
        var has_testing = false;
        
        for (dependencies.items) |dep| {
            if (std.mem.indexOf(u8, dep, "http") != null or std.mem.indexOf(u8, dep, "web") != null) {
                has_web = true;
            }
            if (std.mem.indexOf(u8, dep, "test") != null) {
                has_testing = true;
            }
        }
        
        if (has_web and !has_testing) {
            try recommendations.append("web-testing-framework");
        }
        
        if (has_web) {
            try recommendations.append("request-validation");
            try recommendations.append("security-middleware");
        }
        
        return recommendations;
    }
    
    pub fn updateTrendingPackages(self: *DiscoveryEngine) !void {
        print("📈 Updating trending packages...\n", .{});
        
        // Clear existing trending packages
        self.trending_packages.clearAndFree();
        
        // Simulate trending package detection
        const trending_data = [_]TrendingPackage{
            TrendingPackage{ .name = "fast-json", .score = 95.5, .growth_rate = 23.4, .category = .utilities },
            TrendingPackage{ .name = "secure-http", .score = 92.1, .growth_rate = 18.7, .category = .web },
            TrendingPackage{ .name = "crypto-suite", .score = 89.3, .growth_rate = 15.2, .category = .crypto },
        };
        
        for (trending_data) |pkg| {
            try self.trending_packages.append(pkg);
        }
        
        print("✅ Updated trending packages list\n", .{});
    }
};

pub const RecommendationContext = struct {
    user_categories: ArrayList(PackageMetadata.Category),
    current_dependencies: ArrayList([]const u8),
    project_type: ?ProjectType = null,
    
    pub const ProjectType = enum { web_app, cli_tool, library, game, desktop_app };
    
    pub fn init(allocator: Allocator) RecommendationContext {
        return RecommendationContext{
            .user_categories = ArrayList(PackageMetadata.Category).init(allocator),
            .current_dependencies = ArrayList([]const u8).init(allocator),
        };
    }
    
    pub fn deinit(self: *RecommendationContext) void {
        self.user_categories.deinit();
        self.current_dependencies.deinit();
    }
};

// ===== Export for external use =====

pub const registry_types = struct {
    pub const Registry = PackageRegistry;
    pub const Metadata = PackageMetadata;
    pub const Search = SearchQuery;
    pub const Result = SearchResult;
    pub const Security = SecurityScanner;
    pub const Curator = PackageCurator;
    pub const Analytics = AnalyticsEngine;
    pub const Discovery = DiscoveryEngine;
    pub const RecommendationCtx = RecommendationContext;
};
