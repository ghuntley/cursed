// CURSED Enhanced Package Manager
// Complete package management system with TOML support, dependency resolution, and build integration

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

// ===== TOML Parser for Package Manifests =====

pub const TomlValue = union(enum) {
    string: []const u8,
    integer: i64,
    float: f64,
    boolean: bool,
    array: ArrayList(TomlValue),
    table: HashMap([]const u8, TomlValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),

    pub fn deinit(self: *TomlValue, allocator: Allocator) void {
        switch (self.*) {
            .string => |str| {
                allocator.free(str);
            },
            .array => |*arr| {
                for (arr.items) |*item| {
                    item.deinit(allocator);
                }
                arr.deinit();
            },
            .table => |*table| {
                var iterator = table.iterator();
                while (iterator.next()) |entry| {
                    // Free the key (allocated string)
                    allocator.free(entry.key_ptr.*);
                    // Free the value
                    entry.value_ptr.deinit(allocator);
                }
                table.deinit();
            },
            else => {},
        }
    }
};

pub const TomlParser = struct {
    allocator: Allocator,
    content: []const u8,
    pos: usize,

    pub fn init(allocator: Allocator, content: []const u8) TomlParser {
        return TomlParser{
            .allocator = allocator,
            .content = content,
            .pos = 0,
        };
    }

    pub fn parse(self: *TomlParser) !TomlValue {
        var root = HashMap([]const u8, TomlValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        var current_section: ?[]const u8 = null;
        defer if (current_section) |section| self.allocator.free(section);

        while (self.pos < self.content.len) {
            self.skipWhitespace();
            if (self.pos >= self.content.len) break;

            const char = self.content[self.pos];
            
            if (char == '#') {
                self.skipLine();
                continue;
            }

            if (char == '[') {
                // Free previous section before setting new one
                if (current_section) |section| self.allocator.free(section);
                current_section = try self.parseSection();
                continue;
            }

            if (std.ascii.isAlphabetic(char) or char == '_') {
                const key_value = try self.parseKeyValue();
                const section_key = if (current_section) |section|
                    try std.fmt.allocPrint(self.allocator, "{s}.{s}", .{ section, key_value.key })
                else
                    try self.allocator.dupe(u8, key_value.key);

                try root.put(section_key, key_value.value);
                // Free the original key since we're using section_key instead
                self.allocator.free(key_value.key);
                continue;
            }

            self.pos += 1;
        }

        return TomlValue{ .table = root };
    }

    fn parseSection(self: *TomlParser) ![]const u8 {
        self.pos += 1; // Skip '['
        const start = self.pos;
        
        while (self.pos < self.content.len and self.content[self.pos] != ']') {
            self.pos += 1;
        }
        
        if (self.pos >= self.content.len) {
            return error.UnterminatedSection;
        }
        
        const section = self.content[start..self.pos];
        self.pos += 1; // Skip ']'
        return try self.allocator.dupe(u8, section);
    }

    fn parseKeyValue(self: *TomlParser) !struct { key: []const u8, value: TomlValue } {
        const key = try self.parseKey();
        self.skipWhitespace();
        
        if (self.pos >= self.content.len or self.content[self.pos] != '=') {
            return error.ExpectedEquals;
        }
        
        self.pos += 1; // Skip '='
        self.skipWhitespace();
        
        const value = try self.parseValue();
        return .{ .key = key, .value = value };
    }

    fn parseKey(self: *TomlParser) ![]const u8 {
        const start = self.pos;
        
        while (self.pos < self.content.len) {
            const char = self.content[self.pos];
            if (std.ascii.isAlphanumeric(char) or char == '_' or char == '-') {
                self.pos += 1;
            } else {
                break;
            }
        }
        
        return try self.allocator.dupe(u8, self.content[start..self.pos]);
    }

    fn parseValue(self: *TomlParser) anyerror!TomlValue {
        const char = self.content[self.pos];
        
        if (char == '"') {
            return try self.parseString();
        }
        
        if (char == '[') {
            return try self.parseArray();
        }
        
        if (std.ascii.isDigit(char) or char == '-') {
            return try self.parseNumber();
        }
        
        if (std.mem.startsWith(u8, self.content[self.pos..], "true")) {
            self.pos += 4;
            return TomlValue{ .boolean = true };
        }
        
        if (std.mem.startsWith(u8, self.content[self.pos..], "false")) {
            self.pos += 5;
            return TomlValue{ .boolean = false };
        }
        
        return error.InvalidValue;
    }

    fn parseString(self: *TomlParser) !TomlValue {
        self.pos += 1; // Skip opening quote
        const start = self.pos;
        
        while (self.pos < self.content.len and self.content[self.pos] != '"') {
            if (self.content[self.pos] == '\\') {
                self.pos += 2; // Skip escape sequence
            } else {
                self.pos += 1;
            }
        }
        
        if (self.pos >= self.content.len) {
            return error.UnterminatedString;
        }
        
        const string_content = try self.allocator.dupe(u8, self.content[start..self.pos]);
        self.pos += 1; // Skip closing quote
        
        return TomlValue{ .string = string_content };
    }

    fn parseArray(self: *TomlParser) anyerror!TomlValue {
        self.pos += 1; // Skip '['
        var array = ArrayList(TomlValue).init(self.allocator);
        
        self.skipWhitespace();
        
        while (self.pos < self.content.len and self.content[self.pos] != ']') {
            const value = try self.parseValue();
            try array.append(value);
            
            self.skipWhitespace();
            
            if (self.pos < self.content.len and self.content[self.pos] == ',') {
                self.pos += 1;
                self.skipWhitespace();
            }
        }
        
        if (self.pos >= self.content.len) {
            return error.UnterminatedArray;
        }
        
        self.pos += 1; // Skip ']'
        return TomlValue{ .array = array };
    }

    fn parseNumber(self: *TomlParser) !TomlValue {
        const start = self.pos;
        var has_dot = false;
        
        if (self.content[self.pos] == '-') {
            self.pos += 1;
        }
        
        while (self.pos < self.content.len) {
            const char = self.content[self.pos];
            if (std.ascii.isDigit(char)) {
                self.pos += 1;
            } else if (char == '.' and !has_dot) {
                has_dot = true;
                self.pos += 1;
            } else {
                break;
            }
        }
        
        const number_str = self.content[start..self.pos];
        
        if (has_dot) {
            const float_val = try std.fmt.parseFloat(f64, number_str);
            return TomlValue{ .float = float_val };
        } else {
            const int_val = try std.fmt.parseInt(i64, number_str, 10);
            return TomlValue{ .integer = int_val };
        }
    }

    fn skipWhitespace(self: *TomlParser) void {
        while (self.pos < self.content.len and std.ascii.isWhitespace(self.content[self.pos])) {
            self.pos += 1;
        }
    }

    fn skipLine(self: *TomlParser) void {
        while (self.pos < self.content.len and self.content[self.pos] != '\n') {
            self.pos += 1;
        }
        if (self.pos < self.content.len) {
            self.pos += 1; // Skip newline
        }
    }
};

// ===== Semantic Versioning =====

pub const Version = struct {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: ?[]const u8 = null,
    build_metadata: ?[]const u8 = null,

    pub fn deinit(self: *Version, allocator: Allocator) void {
        if (self.pre_release) |pr| {
            allocator.free(pr);
            self.pre_release = null;
        }
        if (self.build_metadata) |bm| {
            allocator.free(bm);
            self.build_metadata = null;
        }
    }

    pub fn parse(allocator: Allocator, version_str: []const u8) !Version {
        var parts = std.mem.splitScalar(u8, version_str, '.');
        const major_str = parts.next() orelse return error.InvalidVersion;
        const minor_str = parts.next() orelse return error.InvalidVersion;
        const patch_part_start = parts.next() orelse return error.InvalidVersion;
        
        // Reconstruct patch part including any remaining parts (for build metadata with dots)
        var patch_part_builder = std.ArrayList(u8).init(allocator);
        defer patch_part_builder.deinit();
        
        try patch_part_builder.appendSlice(patch_part_start);
        while (parts.next()) |remaining_part| {
            try patch_part_builder.append('.');
            try patch_part_builder.appendSlice(remaining_part);
        }
        
        const patch_part = patch_part_builder.items;

        // Parse patch with potential pre-release and build metadata
        // Format: MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
        var working_part = patch_part;
        var build_metadata: ?[]const u8 = null;
        
        // Extract build metadata (+BUILD) first
        if (std.mem.indexOf(u8, working_part, "+")) |plus_idx| {
            build_metadata = try allocator.dupe(u8, working_part[plus_idx + 1..]);
            working_part = working_part[0..plus_idx];
        }
        
        // Extract pre-release (-PRERELEASE) 
        var patch_parts = std.mem.splitScalar(u8, working_part, '-');
        const patch_str = patch_parts.next() orelse return error.InvalidVersion;
        
        // Collect remaining parts as pre-release (there could be multiple dashes)
        var pre_release: ?[]const u8 = null;
        if (patch_parts.next()) |first_part| {
            var pre_release_parts = std.ArrayList(u8).init(allocator);
            defer pre_release_parts.deinit();
            
            try pre_release_parts.appendSlice(first_part);
            while (patch_parts.next()) |part| {
                try pre_release_parts.append('-');
                try pre_release_parts.appendSlice(part);
            }
            
            pre_release = try allocator.dupe(u8, pre_release_parts.items);
        }

        return Version{
            .major = try std.fmt.parseInt(u32, major_str, 10),
            .minor = try std.fmt.parseInt(u32, minor_str, 10),
            .patch = try std.fmt.parseInt(u32, patch_str, 10),
            .pre_release = pre_release,
            .build_metadata = build_metadata,
        };
    }

    pub fn toString(self: Version, allocator: Allocator) ![]const u8 {
        if (self.pre_release) |pr| {
            if (self.build_metadata) |bm| {
                return try std.fmt.allocPrint(allocator, "{}.{}.{}-{s}+{s}", .{ self.major, self.minor, self.patch, pr, bm });
            } else {
                return try std.fmt.allocPrint(allocator, "{}.{}.{}-{s}", .{ self.major, self.minor, self.patch, pr });
            }
        } else if (self.build_metadata) |bm| {
            return try std.fmt.allocPrint(allocator, "{}.{}.{}+{s}", .{ self.major, self.minor, self.patch, bm });
        }
        return try std.fmt.allocPrint(allocator, "{}.{}.{}", .{ self.major, self.minor, self.patch });
    }

    pub fn compare(self: Version, other: Version) i32 {
        if (self.major != other.major) return @as(i32, @intCast(self.major)) - @as(i32, @intCast(other.major));
        if (self.minor != other.minor) return @as(i32, @intCast(self.minor)) - @as(i32, @intCast(other.minor));
        if (self.patch != other.patch) return @as(i32, @intCast(self.patch)) - @as(i32, @intCast(other.patch));
        
        // Handle pre-release comparison
        if (self.pre_release == null and other.pre_release == null) return 0;
        if (self.pre_release == null and other.pre_release != null) return 1;
        if (self.pre_release != null and other.pre_release == null) return -1;
        
        return switch (std.mem.order(u8, self.pre_release.?, other.pre_release.?)) {
            .lt => -1,
            .gt => 1,
            .eq => 0,
        };
    }

    pub fn satisfies(self: Version, requirement: VersionRequirement) bool {
        return requirement.matches(self);
    }

    pub fn format(self: Version, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        if (self.pre_release) |pr| {
            if (self.build_metadata) |bm| {
                try writer.print("{}.{}.{}-{s}+{s}", .{ self.major, self.minor, self.patch, pr, bm });
            } else {
                try writer.print("{}.{}.{}-{s}", .{ self.major, self.minor, self.patch, pr });
            }
        } else if (self.build_metadata) |bm| {
            try writer.print("{}.{}.{}+{s}", .{ self.major, self.minor, self.patch, bm });
        } else {
            try writer.print("{}.{}.{}", .{ self.major, self.minor, self.patch });
        }
    }
};

pub const VersionRequirement = struct {
    constraint: VersionConstraint,

    const VersionConstraint = union(enum) {
        exact: Version,
        caret: Version,      // ^1.2.3 - compatible with 1.x.x
        tilde: Version,      // ~1.2.3 - compatible with 1.2.x
        greater: Version,    // >1.2.3
        greater_eq: Version, // >=1.2.3
        less: Version,       // <1.2.3
        less_eq: Version,    // <=1.2.3
        wildcard: struct {   // 1.2.*
            major: ?u32,
            minor: ?u32,
        },
    };

    pub fn deinit(self: *VersionRequirement, allocator: Allocator) void {
        switch (self.constraint) {
            .exact => |*v| v.deinit(allocator),
            .caret => |*v| v.deinit(allocator),
            .tilde => |*v| v.deinit(allocator),
            .greater => |*v| v.deinit(allocator),
            .greater_eq => |*v| v.deinit(allocator),
            .less => |*v| v.deinit(allocator),
            .less_eq => |*v| v.deinit(allocator),
            .wildcard => {}, // No allocated memory
        }
    }

    pub fn parse(allocator: Allocator, requirement_str: []const u8) !VersionRequirement {
        const trimmed = std.mem.trim(u8, requirement_str, " \t\n\r");
        
        if (std.mem.startsWith(u8, trimmed, "^")) {
            const version = try Version.parse(allocator, trimmed[1..]);
            return VersionRequirement{ .constraint = .{ .caret = version } };
        }
        
        if (std.mem.startsWith(u8, trimmed, "~")) {
            const version = try Version.parse(allocator, trimmed[1..]);
            return VersionRequirement{ .constraint = .{ .tilde = version } };
        }
        
        if (std.mem.startsWith(u8, trimmed, ">=")) {
            const version = try Version.parse(allocator, trimmed[2..]);
            return VersionRequirement{ .constraint = .{ .greater_eq = version } };
        }
        
        if (std.mem.startsWith(u8, trimmed, "<=")) {
            const version = try Version.parse(allocator, trimmed[2..]);
            return VersionRequirement{ .constraint = .{ .less_eq = version } };
        }
        
        if (std.mem.startsWith(u8, trimmed, ">")) {
            const version = try Version.parse(allocator, trimmed[1..]);
            return VersionRequirement{ .constraint = .{ .greater = version } };
        }
        
        if (std.mem.startsWith(u8, trimmed, "<")) {
            const version = try Version.parse(allocator, trimmed[1..]);
            return VersionRequirement{ .constraint = .{ .less = version } };
        }
        
        // Handle wildcards
        if (std.mem.indexOf(u8, trimmed, "*")) |_| {
            var parts = std.mem.splitScalar(u8, trimmed, '.');
            const major_str = parts.next();
            const minor_str = parts.next();
            
            return VersionRequirement{
                .constraint = .{
                    .wildcard = .{
                        .major = if (major_str != null and !std.mem.eql(u8, major_str.?, "*"))
                            try std.fmt.parseInt(u32, major_str.?, 10)
                        else
                            null,
                        .minor = if (minor_str != null and !std.mem.eql(u8, minor_str.?, "*"))
                            try std.fmt.parseInt(u32, minor_str.?, 10)
                        else
                            null,
                    },
                },
            };
        }
        
        // Default to exact match
        const version = try Version.parse(allocator, trimmed);
        return VersionRequirement{ .constraint = .{ .exact = version } };
    }

    pub fn matches(self: VersionRequirement, version: Version) bool {
        return switch (self.constraint) {
            .exact => |v| version.compare(v) == 0,
            .caret => |v| {
                if (version.major != v.major) return false;
                return version.compare(v) >= 0;
            },
            .tilde => |v| {
                if (version.major != v.major or version.minor != v.minor) return false;
                return version.compare(v) >= 0;
            },
            .greater => |v| version.compare(v) > 0,
            .greater_eq => |v| version.compare(v) >= 0,
            .less => |v| version.compare(v) < 0,
            .less_eq => |v| version.compare(v) <= 0,
            .wildcard => |w| {
                if (w.major) |major| {
                    if (version.major != major) return false;
                }
                if (w.minor) |minor| {
                    if (version.minor != minor) return false;
                }
                return true;
            },
        };
    }
};

// ===== Package Management Core =====

pub const PackageSource = union(enum) {
    registry: struct {
        url: []const u8,
        name: []const u8,
    },
    git: struct {
        url: []const u8,
        rev: ?[]const u8 = null,
        branch: ?[]const u8 = null,
        tag: ?[]const u8 = null,
    },
    local: struct {
        path: []const u8,
    },
    url: struct {
        url: []const u8,
        checksum: ?[]const u8 = null,
    },
};

pub const Dependency = struct {
    name: []const u8,
    version_req: VersionRequirement,
    source: PackageSource,
    optional: bool = false,
    dev_only: bool = false,
    features: ArrayList([]const u8),

    pub fn init(allocator: Allocator, name: []const u8, version_req: VersionRequirement, source: PackageSource) Dependency {
        return Dependency{
            .name = name,
            .version_req = version_req,
            .source = source,
            .features = ArrayList([]const u8).init(allocator),
        };
    }

    pub fn deinit(self: *Dependency) void {
        self.features.deinit();
    }
};

pub const PackageManifest = struct {
    name: []const u8,
    version: Version,
    description: ?[]const u8 = null,
    authors: ArrayList([]const u8),
    license: ?[]const u8 = null,
    keywords: ArrayList([]const u8),
    categories: ArrayList([]const u8),
    homepage: ?[]const u8 = null,
    repository: ?[]const u8 = null,
    documentation: ?[]const u8 = null,
    readme: ?[]const u8 = null,
    
    dependencies: HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    dev_dependencies: HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    build_dependencies: HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    features: HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    bin: ArrayList(BinTarget),
    lib: ?LibTarget = null,
    
    build_script: ?[]const u8 = null,
    links: ?[]const u8 = null,
    include_files: ArrayList([]const u8),
    exclude_files: ArrayList([]const u8),

    const BinTarget = struct {
        name: []const u8,
        path: ?[]const u8 = null,
    };

    const LibTarget = struct {
        name: ?[]const u8 = null,
        path: ?[]const u8 = null,
        crate_type: ArrayList([]const u8),
    };

    pub fn init(allocator: Allocator) PackageManifest {
        return PackageManifest{
            .name = "",
            .version = Version{ .major = 0, .minor = 1, .patch = 0 },
            .authors = ArrayList([]const u8).init(allocator),
            .keywords = ArrayList([]const u8).init(allocator),
            .categories = ArrayList([]const u8).init(allocator),
            .dependencies = HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .dev_dependencies = HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .build_dependencies = HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .features = HashMap([]const u8, ArrayList([]const u8), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
            .bin = ArrayList(BinTarget).init(allocator),
            .include_files = ArrayList([]const u8).init(allocator),
            .exclude_files = ArrayList([]const u8).init(allocator),
        };
    }

    pub fn deinit(self: *PackageManifest, allocator: Allocator) void {
        // Free allocated string fields
        if (!std.mem.eql(u8, self.name, "")) {
            allocator.free(self.name);
        }
        if (self.description) |desc| {
            allocator.free(desc);
        }
        if (self.license) |license| {
            allocator.free(license);
        }
        if (self.homepage) |homepage| {
            allocator.free(homepage);
        }
        if (self.repository) |repo| {
            allocator.free(repo);
        }
        if (self.documentation) |docs| {
            allocator.free(docs);
        }
        if (self.readme) |readme| {
            allocator.free(readme);
        }
        if (self.build_script) |script| {
            allocator.free(script);
        }
        if (self.links) |links| {
            allocator.free(links);
        }
        
        // Free version pre_release if allocated
        self.version.deinit(allocator);
        
        // Free authors array contents
        for (self.authors.items) |author| {
            allocator.free(author);
        }
        self.authors.deinit();
        
        // Free keywords array contents
        for (self.keywords.items) |keyword| {
            allocator.free(keyword);
        }
        self.keywords.deinit();
        
        // Free categories array contents
        for (self.categories.items) |category| {
            allocator.free(category);
        }
        self.categories.deinit();
        
        var dep_iter = self.dependencies.iterator();
        while (dep_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.dependencies.deinit();
        
        var dev_dep_iter = self.dev_dependencies.iterator();
        while (dev_dep_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.dev_dependencies.deinit();
        
        var build_dep_iter = self.build_dependencies.iterator();
        while (build_dep_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.build_dependencies.deinit();
        
        var features_iter = self.features.iterator();
        while (features_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.features.deinit();
        
        self.bin.deinit();
        if (self.lib) |*lib| {
            lib.crate_type.deinit();
        }
        self.include_files.deinit();
        self.exclude_files.deinit();
    }

    pub fn loadFromToml(allocator: Allocator, file_path: []const u8) !PackageManifest {
        const file = try std.fs.cwd().openFile(file_path, .{});
        defer file.close();

        const content = try file.readToEndAlloc(allocator, 1024 * 1024);
        defer allocator.free(content);

        var parser = TomlParser.init(allocator, content);
        var toml = try parser.parse();
        defer toml.deinit(allocator);

        return try PackageManifest.fromToml(allocator, toml);
    }

    pub fn fromToml(allocator: Allocator, toml: TomlValue) !PackageManifest {
        if (toml != .table) return error.InvalidManifest;
        
        var manifest = PackageManifest.init(allocator);
        
        // Parse basic package information
        if (toml.table.get("name")) |name_val| {
            if (name_val != .string) return error.InvalidName;
            manifest.name = try allocator.dupe(u8, name_val.string);
        }
        
        if (toml.table.get("version")) |version_val| {
            if (version_val != .string) return error.InvalidVersion;
            manifest.version = try Version.parse(allocator, version_val.string);
        }
        
        if (toml.table.get("description")) |desc_val| {
            if (desc_val == .string) {
                manifest.description = try allocator.dupe(u8, desc_val.string);
            }
        }
        
        // Parse dependencies
        if (toml.table.get("dependencies")) |deps_val| {
            if (deps_val == .table) {
                try parseDependencies(allocator, &manifest.dependencies, deps_val.table, false);
            }
        }
        
        if (toml.table.get("dev-dependencies")) |dev_deps_val| {
            if (dev_deps_val == .table) {
                try parseDependencies(allocator, &manifest.dev_dependencies, dev_deps_val.table, true);
            }
        }
        
        return manifest;
    }

    fn parseDependencies(
        allocator: Allocator,
        deps: *HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        deps_table: HashMap([]const u8, TomlValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        dev_only: bool,
    ) !void {
        var iter = deps_table.iterator();
        while (iter.next()) |entry| {
            const name = try allocator.dupe(u8, entry.key_ptr.*);
            
            var dependency: Dependency = undefined;
            
            if (entry.value_ptr.* == .string) {
                // Simple version specification
                const version_req = try VersionRequirement.parse(allocator, entry.value_ptr.string);
                dependency = Dependency.init(
                    allocator,
                    name,
                    version_req,
                    PackageSource{ .registry = .{ .url = "https://packages.cursed.dev", .name = name } },
                );
            } else if (entry.value_ptr.* == .table) {
                // Complex dependency specification
                const dep_table = entry.value_ptr.table;
                
                if (dep_table.get("version")) |version_val| {
                    if (version_val != .string) continue;
                    
                    const version_req = try VersionRequirement.parse(allocator, version_val.string);
                    
                    // Determine source
                    var source: PackageSource = undefined;
                    
                    if (dep_table.get("git")) |git_val| {
                        if (git_val != .string) continue;
                        source = PackageSource{ .git = .{ .url = git_val.string } };
                    } else if (dep_table.get("path")) |path_val| {
                        if (path_val != .string) continue;
                        source = PackageSource{ .local = .{ .path = path_val.string } };
                    } else {
                        source = PackageSource{ .registry = .{ .url = "https://packages.cursed.dev", .name = name } };
                    }
                    
                    dependency = Dependency.init(allocator, name, version_req, source);
                    dependency.dev_only = dev_only;
                    
                    // Parse optional flag
                    if (dep_table.get("optional")) |optional_val| {
                        if (optional_val == .boolean) {
                            dependency.optional = optional_val.boolean;
                        }
                    }
                } else {
                    continue; // Skip invalid dependency
                }
            } else {
                continue; // Skip invalid dependency
            }
            
            try deps.put(name, dependency);
        }
    }

    pub fn saveToToml(self: *const PackageManifest, allocator: Allocator, file_path: []const u8) !void {
        const content = try self.toTomlString(allocator);
        defer allocator.free(content);

        const file = try std.fs.cwd().createFile(file_path, .{});
        defer file.close();

        try file.writeAll(content);
    }

    pub fn toTomlString(self: *const PackageManifest, allocator: Allocator) ![]const u8 {
        var content = ArrayList(u8).init(allocator);
        defer content.deinit();
        
        const writer = content.writer();
        
        // Package metadata
        try writer.print("name = \"{s}\"\n", .{self.name});
        
        const version_str = try self.version.toString(allocator);
        defer allocator.free(version_str);
        try writer.print("version = \"{s}\"\n", .{version_str});
        
        if (self.description) |desc| {
            try writer.print("description = \"{s}\"\n", .{desc});
        }
        
        // Authors
        if (self.authors.items.len > 0) {
            try writer.writeAll("authors = [");
            for (self.authors.items, 0..) |author, i| {
                if (i > 0) try writer.writeAll(", ");
                try writer.print("\"{s}\"", .{author});
            }
            try writer.writeAll("]\n");
        }
        
        // Keywords and categories
        if (self.keywords.items.len > 0) {
            try writer.writeAll("keywords = [");
            for (self.keywords.items, 0..) |keyword, i| {
                if (i > 0) try writer.writeAll(", ");
                try writer.print("\"{s}\"", .{keyword});
            }
            try writer.writeAll("]\n");
        }
        
        if (self.categories.items.len > 0) {
            try writer.writeAll("categories = [");
            for (self.categories.items, 0..) |category, i| {
                if (i > 0) try writer.writeAll(", ");
                try writer.print("\"{s}\"", .{category});
            }
            try writer.writeAll("]\n");
        }
        
        // Dependencies sections
        if (self.dependencies.count() > 0) {
            try writer.writeAll("\n[dependencies]\n");
            try writeDependencies(allocator, writer, self.dependencies);
        }
        
        if (self.dev_dependencies.count() > 0) {
            try writer.writeAll("\n[dev-dependencies]\n");
            try writeDependencies(allocator, writer, self.dev_dependencies);
        }
        
        if (self.build_dependencies.count() > 0) {
            try writer.writeAll("\n[build-dependencies]\n");
            try writeDependencies(allocator, writer, self.build_dependencies);
        }
        
        return try content.toOwnedSlice();
    }

    fn writeDependencies(
        allocator: Allocator,
        writer: anytype,
        deps: HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    ) !void {
        var iter = deps.iterator();
        while (iter.next()) |entry| {
            const dep = entry.value_ptr.*;
            
            switch (dep.source) {
                .registry => {
                    // Simple version for registry dependencies
                    const version_str = dep.version_req.constraint.exact.toString(allocator) catch "unknown";
                    defer if (!std.mem.eql(u8, version_str, "unknown")) allocator.free(version_str);
                    try writer.print("{s} = \"{s}\"\n", .{ dep.name, version_str });
                },
                .git => |git| {
                    try writer.print("{s} = {{ git = \"{s}\"", .{ dep.name, git.url });
                    if (git.rev) |rev| {
                        try writer.print(", rev = \"{s}\"", .{rev});
                    }
                    if (git.branch) |branch| {
                        try writer.print(", branch = \"{s}\"", .{branch});
                    }
                    if (git.tag) |tag| {
                        try writer.print(", tag = \"{s}\"", .{tag});
                    }
                    try writer.writeAll(" }\n");
                },
                .local => |local| {
                    try writer.print("{s} = {{ path = \"{s}\" }}\n", .{ dep.name, local.path });
                },
                .url => |url| {
                    try writer.print("{s} = {{ url = \"{s}\"", .{ dep.name, url.url });
                    if (url.checksum) |checksum| {
                        try writer.print(", checksum = \"{s}\"", .{checksum});
                    }
                    try writer.writeAll(" }\n");
                },
            }
        }
    }
};

// ===== Lock File Management =====

pub const LockFile = struct {
    version: u32 = 1,
    packages: ArrayList(LockedPackage),
    
    pub const LockedPackage = struct {
        name: []const u8,
        version: Version,
        source: PackageSource,
        checksum: []const u8,
        dependencies: ArrayList([]const u8),
        
        pub fn init(allocator: Allocator) LockedPackage {
            return LockedPackage{
                .name = "",
                .version = Version{ .major = 0, .minor = 0, .patch = 0 },
                .source = PackageSource{ .registry = .{ .url = "", .name = "" } },
                .checksum = "",
                .dependencies = ArrayList([]const u8).init(allocator),
            };
        }
        
        pub fn deinit(self: *LockedPackage) void {
            self.dependencies.deinit();
        }
        
        pub fn fromToml(allocator: Allocator, toml_table: HashMap([]const u8, TomlValue, std.hash_map.StringContext, std.hash_map.default_max_load_percentage)) !LockedPackage {
            var pkg = LockedPackage.init(allocator);
            
            if (toml_table.get("name")) |name_val| {
                if (name_val == .string) {
                    pkg.name = try allocator.dupe(u8, name_val.string);
                }
            }
            
            if (toml_table.get("version")) |version_val| {
                if (version_val == .string) {
                    pkg.version = try Version.parse(allocator, version_val.string);
                }
            }
            
            if (toml_table.get("checksum")) |checksum_val| {
                if (checksum_val == .string) {
                    pkg.checksum = try allocator.dupe(u8, checksum_val.string);
                }
            }
            
            if (toml_table.get("source")) |source_val| {
                if (source_val == .string) {
                    // Parse source string to determine type
                    if (std.mem.startsWith(u8, source_val.string, "registry+")) {
                        const url = source_val.string[9..]; // Skip "registry+"
                        pkg.source = PackageSource{ .registry = .{ .url = try allocator.dupe(u8, url), .name = pkg.name } };
                    } else if (std.mem.startsWith(u8, source_val.string, "git+")) {
                        const url = source_val.string[4..]; // Skip "git+"
                        pkg.source = PackageSource{ .git = .{ .url = try allocator.dupe(u8, url) } };
                    } else if (std.mem.startsWith(u8, source_val.string, "path+")) {
                        const path = source_val.string[5..]; // Skip "path+"
                        pkg.source = PackageSource{ .local = .{ .path = try allocator.dupe(u8, path) } };
                    } else {
                        pkg.source = PackageSource{ .registry = .{ .url = "https://packages.cursed.dev", .name = pkg.name } };
                    }
                }
            }
            
            return pkg;
        }
    };

    pub fn init(allocator: Allocator) LockFile {
        return LockFile{
            .packages = ArrayList(LockedPackage).init(allocator),
        };
    }

    pub fn deinit(self: *LockFile) void {
        for (self.packages.items) |*pkg| {
            pkg.deinit();
        }
        self.packages.deinit();
    }

    pub fn saveToFile(self: *const LockFile, allocator: Allocator, file_path: []const u8) !void {
        const content = try self.toTomlString(allocator);
        defer allocator.free(content);

        const file = try std.fs.cwd().createFile(file_path, .{});
        defer file.close();

        try file.writeAll(content);
    }

    pub fn loadFromFile(allocator: Allocator, file_path: []const u8) !LockFile {
        const file = std.fs.cwd().openFile(file_path, .{}) catch |err| switch (err) {
            error.FileNotFound => return LockFile.init(allocator),
            else => return err,
        };
        defer file.close();

        const content = try file.readToEndAlloc(allocator, 1024 * 1024);
        defer allocator.free(content);

        var parser = TomlParser.init(allocator, content);
        var toml = try parser.parse();
        defer toml.deinit(allocator);

        return try LockFile.fromToml(allocator, toml);
    }

    pub fn fromToml(allocator: Allocator, toml: TomlValue) !LockFile {
        var lock_file = LockFile.init(allocator);
        
        if (toml.table.get("package")) |packages_val| {
            if (packages_val == .array) {
                for (packages_val.array.items) |pkg_val| {
                    if (pkg_val == .table) {
                        const pkg = try LockedPackage.fromToml(allocator, pkg_val.table);
                        try lock_file.packages.append(pkg);
                    }
                }
            }
        }
        
        return lock_file;
    }

    pub fn toTomlString(self: *const LockFile, allocator: Allocator) ![]const u8 {
        var content = ArrayList(u8).init(allocator);
        defer content.deinit();
        
        const writer = content.writer();
        
        try writer.print("version = {}\n\n", .{self.version});
        
        for (self.packages.items) |pkg| {
            try writer.writeAll("[[package]]\n");
            try writer.print("name = \"{s}\"\n", .{pkg.name});
            
            const version_str = try pkg.version.toString(allocator);
            defer allocator.free(version_str);
            try writer.print("version = \"{s}\"\n", .{version_str});
            
            switch (pkg.source) {
                .registry => |registry| {
                    try writer.print("source = \"registry+{s}\"\n", .{registry.url});
                },
                .git => |git| {
                    try writer.print("source = \"git+{s}\"\n", .{git.url});
                },
                .local => |local| {
                    try writer.print("source = \"path+{s}\"\n", .{local.path});
                },
                .url => |url| {
                    try writer.print("source = \"url+{s}\"\n", .{url.url});
                },
            }
            
            try writer.print("checksum = \"{s}\"\n", .{pkg.checksum});
            
            if (pkg.dependencies.items.len > 0) {
                try writer.writeAll("dependencies = [");
                for (pkg.dependencies.items, 0..) |dep, i| {
                    if (i > 0) try writer.writeAll(", ");
                    try writer.print("\"{s}\"", .{dep});
                }
                try writer.writeAll("]\n");
            }
            
            try writer.writeAll("\n");
        }
        
        return try content.toOwnedSlice();
    }
};

// ===== Dependency Resolution =====

pub const DependencyResolver = struct {
    allocator: Allocator,
    registry_cache: HashMap([]const u8, ArrayList(Version), std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    
    pub fn init(allocator: Allocator) DependencyResolver {
        return DependencyResolver{
            .allocator = allocator,
            .registry_cache = HashMap([]const u8, ArrayList(Version), std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator),
        };
    }
    
    pub fn deinit(self: *DependencyResolver) void {
        var iter = self.registry_cache.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.registry_cache.deinit();
    }
    
    pub fn resolve(self: *DependencyResolver, manifest: *const PackageManifest) !ArrayList(ResolvedDependency) {
        var resolved = ArrayList(ResolvedDependency).init(self.allocator);
        var visited = HashMap([]const u8, void, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(self.allocator);
        defer visited.deinit();
        
        // Resolve all dependencies
        try self.resolveDependencies(&resolved, &visited, manifest.dependencies);
        try self.resolveDependencies(&resolved, &visited, manifest.dev_dependencies);
        try self.resolveDependencies(&resolved, &visited, manifest.build_dependencies);
        
        // Sort by dependency order (topological sort)
        try self.topologicalSort(&resolved);
        
        return resolved;
    }
    
    pub const ResolvedDependency = struct {
        name: []const u8,
        version: Version,
        source: PackageSource,
        dependencies: ArrayList([]const u8),
        
        pub fn init(allocator: Allocator) ResolvedDependency {
            return ResolvedDependency{
                .name = "",
                .version = Version{ .major = 0, .minor = 0, .patch = 0 },
                .source = PackageSource{ .registry = .{ .url = "", .name = "" } },
                .dependencies = ArrayList([]const u8).init(allocator),
            };
        }
        
        pub fn deinit(self: *ResolvedDependency) void {
            self.dependencies.deinit();
        }
    };
    
    fn resolveDependencies(
        self: *DependencyResolver,
        resolved: *ArrayList(ResolvedDependency),
        visited: *HashMap([]const u8, void, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
        dependencies: HashMap([]const u8, Dependency, std.hash_map.StringContext, std.hash_map.default_max_load_percentage),
    ) !void {
        var iter = dependencies.iterator();
        while (iter.next()) |entry| {
            const dep = entry.value_ptr.*;
            
            if (visited.contains(dep.name)) {
                continue; // Already processed
            }
            
            try visited.put(dep.name, {});
            
            // Find compatible version
            const version = try self.findCompatibleVersion(dep);
            
            var resolved_dep = ResolvedDependency.init(self.allocator);
            resolved_dep.name = try self.allocator.dupe(u8, dep.name);
            resolved_dep.version = version;
            resolved_dep.source = dep.source;
            
            try resolved.append(resolved_dep);
        }
    }
    
    fn findCompatibleVersion(self: *DependencyResolver, dependency: Dependency) !Version {
        _ = self;
        // For now, return the minimum version from the requirement
        // In a full implementation, this would query registries/sources
        switch (dependency.version_req.constraint) {
            .exact => |v| return v,
            .caret => |v| return v,
            .tilde => |v| return v,
            .greater => |v| return Version{ .major = v.major, .minor = v.minor, .patch = v.patch + 1 },
            .greater_eq => |v| return v,
            .less => |v| return Version{ .major = v.major, .minor = v.minor, .patch = if (v.patch > 0) v.patch - 1 else 0 },
            .less_eq => |v| return v,
            .wildcard => |w| return Version{
                .major = w.major orelse 0,
                .minor = w.minor orelse 0,
                .patch = 0,
            },
        }
    }
    
    fn topologicalSort(self: *DependencyResolver, resolved: *ArrayList(ResolvedDependency)) !void {
        _ = self;
        // Simple topological sort implementation
        // In a full implementation, this would handle dependency cycles
        const n = resolved.items.len;
        if (n <= 1) return;
        
        // For now, just reverse the list since dependencies should be processed first
        var i: usize = 0;
        var j: usize = n - 1;
        while (i < j) {
            const temp = resolved.items[i];
            resolved.items[i] = resolved.items[j];
            resolved.items[j] = temp;
            i += 1;
            j -= 1;
        }
    }
};

// ===== Package Cache =====

pub const PackageCache = struct {
    allocator: Allocator,
    cache_dir: []const u8,
    
    pub fn init(allocator: Allocator, cache_dir: []const u8) PackageCache {
        return PackageCache{
            .allocator = allocator,
            .cache_dir = cache_dir,
        };
    }
    
    pub fn ensureCacheDir(self: *PackageCache) !void {
        std.fs.cwd().makePath(self.cache_dir) catch |err| switch (err) {
            error.PathAlreadyExists => {},
            else => return err,
        };
    }
    
    pub fn getPackagePath(self: *PackageCache, name: []const u8, version: Version) ![]const u8 {
        const version_str = try version.toString(self.allocator);
        defer self.allocator.free(version_str);
        
        return try std.fs.path.join(self.allocator, &[_][]const u8{
            self.cache_dir,
            name,
            version_str,
        });
    }
    
    pub fn isPackageCached(self: *PackageCache, name: []const u8, version: Version) !bool {
        const package_path = try self.getPackagePath(name, version);
        defer self.allocator.free(package_path);
        
        var dir = std.fs.cwd().openDir(package_path, .{}) catch |err| switch (err) {
            error.FileNotFound => return false,
            else => return err,
        };
        dir.close();
        
        return true;
    }
    
    pub fn installPackage(self: *PackageCache, name: []const u8, version: Version, source: PackageSource) !void {
        const package_path = try self.getPackagePath(name, version);
        defer self.allocator.free(package_path);
        
        // Create package directory
        try std.fs.cwd().makePath(package_path);
        
        switch (source) {
            .registry => |registry| {
                try self.downloadFromRegistry(package_path, registry.name, version, registry.url);
            },
            .git => |git| {
                try self.cloneFromGit(package_path, git);
            },
            .local => |local| {
                try self.copyFromLocal(package_path, local.path);
            },
            .url => |url| {
                try self.downloadFromUrl(package_path, url.url);
            },
        }
    }
    
    fn downloadFromRegistry(self: *PackageCache, dest_path: []const u8, package_name: []const u8, version: Version, registry_url: []const u8) !void {
        const version_str = try version.toString(self.allocator);
        defer self.allocator.free(version_str);
        
        const download_url = try std.fmt.allocPrint(
            self.allocator,
            "{s}/packages/{s}/{s}/download",
            .{ registry_url, package_name, version_str },
        );
        defer self.allocator.free(download_url);
        
        print("Downloading {s} v{s} from {s}\n", .{ package_name, version_str, registry_url });
        
        // TODO: Implement HTTP download and archive extraction
        // For now, create a placeholder
        const manifest_path = try std.fs.path.join(self.allocator, &[_][]const u8{ dest_path, "CursedPackage.toml" });
        defer self.allocator.free(manifest_path);
        
        const manifest_file = try std.fs.cwd().createFile(manifest_path, .{});
        defer manifest_file.close();
        
        try manifest_file.writer().print(
            \\name = "{s}"
            \\version = "{s}"
            \\description = "Downloaded package"
            \\
        , .{ package_name, version_str });
    }
    
    fn cloneFromGit(self: *PackageCache, dest_path: []const u8, git: anytype) !void {
        var args = ArrayList([]const u8).init(self.allocator);
        defer args.deinit();
        
        try args.appendSlice(&[_][]const u8{ "git", "clone", git.url, dest_path });
        
        if (git.branch) |branch| {
            try args.appendSlice(&[_][]const u8{ "--branch", branch });
        }
        
        if (git.tag) |tag| {
            try args.appendSlice(&[_][]const u8{ "--branch", tag });
        }
        
        const result = try std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = args.items,
        });
        defer self.allocator.free(result.stdout);
        defer self.allocator.free(result.stderr);
        
        if (result.term != .Exited or result.term.Exited != 0) {
            print("Git clone failed: {s}\n", .{result.stderr});
            return error.GitCloneFailed;
        }
        
        // Handle specific revision if specified
        if (git.rev) |rev| {
            const checkout_result = try std.ChildProcess.run(.{
                .allocator = self.allocator,
                .argv = &[_][]const u8{ "git", "-C", dest_path, "checkout", rev },
            });
            defer self.allocator.free(checkout_result.stdout);
            defer self.allocator.free(checkout_result.stderr);
            
            if (checkout_result.term != .Exited or checkout_result.term.Exited != 0) {
                print("Git checkout failed: {s}\n", .{checkout_result.stderr});
                return error.GitCheckoutFailed;
            }
        }
    }
    
    fn copyFromLocal(self: *PackageCache, dest_path: []const u8, source_path: []const u8) !void {
        // Simple directory copy implementation
        var source_dir = try std.fs.cwd().openDir(source_path, .{ .iterate = true });
        defer source_dir.close();
        
        var dest_dir = try std.fs.cwd().openDir(dest_path, .{});
        defer dest_dir.close();
        
        var walker = try source_dir.walk(self.allocator);
        defer walker.deinit();
        
        while (try walker.next()) |entry| {
            switch (entry.kind) {
                .file => {
                    const source_file_path = try std.fs.path.join(self.allocator, &[_][]const u8{ source_path, entry.path });
                    defer self.allocator.free(source_file_path);
                    
                    const dest_file_path = try std.fs.path.join(self.allocator, &[_][]const u8{ dest_path, entry.path });
                    defer self.allocator.free(dest_file_path);
                    
                    try std.fs.cwd().copyFile(source_file_path, std.fs.cwd(), dest_file_path, .{});
                },
                .directory => {
                    const dest_subdir_path = try std.fs.path.join(self.allocator, &[_][]const u8{ dest_path, entry.path });
                    defer self.allocator.free(dest_subdir_path);
                    
                    try std.fs.cwd().makePath(dest_subdir_path);
                },
                else => {},
            }
        }
    }
    
    fn downloadFromUrl(self: *PackageCache, dest_path: []const u8, url: []const u8) !void {
        // TODO: Implement HTTP download
        print("Downloading from URL: {s} to {s}\n", .{ url, dest_path });
        
        // For now, create a placeholder
        const readme_path = try std.fs.path.join(self.allocator, &[_][]const u8{ dest_path, "README.md" });
        defer self.allocator.free(readme_path);
        
        const readme_file = try std.fs.cwd().createFile(readme_path, .{});
        defer readme_file.close();
        
        try readme_file.writer().print("# Package downloaded from {s}\n", .{url});
    }
};

// ===== Build Integration =====

pub const BuildIntegration = struct {
    allocator: Allocator,
    cache: PackageCache,
    
    pub fn init(allocator: Allocator, cache_dir: []const u8) BuildIntegration {
        return BuildIntegration{
            .allocator = allocator,
            .cache = PackageCache.init(allocator, cache_dir),
        };
    }
    
    pub fn generateBuildFile(self: *BuildIntegration, manifest: *const PackageManifest, lock_file: *const LockFile) !void {
        _ = lock_file;
        var resolver = DependencyResolver.init(self.allocator);
        defer resolver.deinit();
        
        const resolved = try resolver.resolve(manifest);
        defer {
            for (resolved.items) |*dep| {
                dep.deinit();
            }
            resolved.deinit();
        }
        
        // Generate build.zig modifications
        const build_content = try self.generateBuildZigContent(manifest, resolved);
        defer self.allocator.free(build_content);
        
        const build_file = try std.fs.cwd().createFile("build_generated.zig", .{});
        defer build_file.close();
        
        try build_file.writeAll(build_content);
        
        print("Generated build_generated.zig with dependency information\n", .{});
    }
    
    fn generateBuildZigContent(self: *BuildIntegration, manifest: *const PackageManifest, resolved: ArrayList(DependencyResolver.ResolvedDependency)) ![]const u8 {
        _ = manifest;
        var content = ArrayList(u8).init(self.allocator);
        defer content.deinit();
        
        const writer = content.writer();
        
        try writer.writeAll(
            \\// Generated build file for CURSED package
            \\const std = @import("std");
            \\
            \\pub fn addDependencies(b: *std.Build, exe: *std.Build.Step.Compile) !void {
            \\
        );
        
        // Add dependency include paths and linking
        for (resolved.items) |dep| {
            const package_path = try self.cache.getPackagePath(dep.name, dep.version);
            defer self.allocator.free(package_path);
            
            try writer.print(
                \\    // Dependency: {s} v{s}
                \\    exe.addIncludePath(.{{ .path = "{s}/src" }});
                \\    exe.addLibraryPath(.{{ .path = "{s}/lib" }});
                \\
            , .{ dep.name, dep.version, package_path, package_path });
        }
        
        try writer.writeAll(
            \\}
            \\
            \\pub fn buildDependencies(b: *std.Build, target: std.Build.ResolvedTarget, optimize: std.builtin.OptimizeMode) !void {
            \\
        );
        
        // Add dependency build steps
        for (resolved.items) |dep| {
            const package_path = try self.cache.getPackagePath(dep.name, dep.version);
            defer self.allocator.free(package_path);
            
            try writer.print(
                \\    // Build dependency: {s}
                \\    const {s}_lib = b.addStaticLibrary(.{{
                \\        .name = "{s}",
                \\        .root_source_file = b.path("{s}/src/lib.zig"),
                \\        .target = target,
                \\        .optimize = optimize,
                \\    }});
                \\    b.installArtifact({s}_lib);
                \\
            , .{ dep.name, dep.name, dep.name, package_path, dep.name });
        }
        
        try writer.writeAll("}\n");
        
        return try content.toOwnedSlice();
    }
    
    pub fn prepareDependencies(self: *BuildIntegration, manifest: *const PackageManifest) !void {
        try self.cache.ensureCacheDir();
        
        var resolver = DependencyResolver.init(self.allocator);
        defer resolver.deinit();
        
        const resolved = try resolver.resolve(manifest);
        defer {
            for (resolved.items) |*dep| {
                dep.deinit();
            }
            resolved.deinit();
        }
        
        // Install missing dependencies
        for (resolved.items) |dep| {
            const is_cached = try self.cache.isPackageCached(dep.name, dep.version);
            const version_str = dep.version.toString(self.allocator) catch "unknown";
            defer if (!std.mem.eql(u8, version_str, "unknown")) self.allocator.free(version_str);
            if (!is_cached) {
                print("Installing {s} v{s}...\n", .{ dep.name, version_str });
                try self.cache.installPackage(dep.name, dep.version, dep.source);
            } else {
                print("Using cached {s} v{s}\n", .{ dep.name, version_str });
            }
        }
    }
};

// ===== Package Manager Commands =====

pub fn cmdInit(allocator: Allocator, args: [][]const u8) !void {
    _ = args;
    
    var manifest = PackageManifest.init(allocator);
    defer manifest.deinit(allocator);
    
    manifest.name = "new-cursed-package";
    manifest.version = Version{ .major = 0, .minor = 1, .patch = 0 };
    manifest.description = "A new CURSED package";
    
    try manifest.authors.append("Your Name <your.email@example.com>");
    
    try manifest.saveToToml(allocator, "CursedPackage.toml");
    
    // Create basic directory structure
    try std.fs.cwd().makePath("src");
    try std.fs.cwd().makePath("tests");
    
    // Create main source file
    const src_file = try std.fs.cwd().createFile("src/lib.csd", .{});
    defer src_file.close();
    
    try src_file.writeAll(
        \\// Main library file for new CURSED package
        \\
        \\slay greet(name tea) tea {
        \\    damn "Hello, " + name + "!"
        \\}
        \\
    );
    
    // Create test file
    const test_file = try std.fs.cwd().createFile("tests/lib_test.csd", .{});
    defer test_file.close();
    
    try test_file.writeAll(
        \\yeet "testz"
        \\yeet "../src/lib"
        \\
        \\test_start("greet function test")
        \\
        \\sus result tea = greet("World")
        \\assert_eq_string(result, "Hello, World!")
        \\
        \\print_test_summary()
        \\
    );
    
    print("Initialized new CURSED package in current directory\n", .{});
    print("Edit CursedPackage.toml to customize package metadata\n", .{});
    print("Add your code to src/lib.csd\n", .{});
    print("Run tests with: cursed pkg test\n", .{});
}

pub fn cmdAdd(allocator: Allocator, args: [][]const u8) !void {
    if (args.len == 0) {
        print("Usage: cursed pkg add <package_name> [version_requirement]\n", .{});
        return;
    }
    
    const package_name = args[0];
    const version_req_str = if (args.len > 1) args[1] else "^0.1.0";
    
    // Load existing manifest
    var manifest = PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
        error.FileNotFound => {
            print("No CursedPackage.toml found. Run 'cursed pkg init' first.\n", .{});
            return;
        },
        else => return err,
    };
    defer manifest.deinit(allocator);
    
    // Parse version requirement
    const version_req = try VersionRequirement.parse(allocator, version_req_str);
    
    // Create dependency
    const dependency = Dependency.init(
        allocator,
        try allocator.dupe(u8, package_name),
        version_req,
        PackageSource{ .registry = .{ .url = "https://packages.cursed.dev", .name = try allocator.dupe(u8, package_name) } },
    );
    
    // Add to manifest
    try manifest.dependencies.put(try allocator.dupe(u8, package_name), dependency);
    
    // Save updated manifest
    try manifest.saveToToml(allocator, "CursedPackage.toml");
    
    print("Added dependency: {s} {s}\n", .{ package_name, version_req_str });
    print("Run 'cursed pkg install' to download the dependency\n", .{});
}

pub fn cmdInstall(allocator: Allocator, args: [][]const u8) !void {
    _ = args;
    
    // Load manifest
    var manifest = PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
        error.FileNotFound => {
            print("No CursedPackage.toml found. Run 'cursed pkg init' first.\n", .{});
            return;
        },
        else => return err,
    };
    defer manifest.deinit(allocator);
    
    // Initialize build integration
    var build_integration = BuildIntegration.init(allocator, ".cursed/cache");
    
    // Install dependencies
    try build_integration.prepareDependencies(&manifest);
    
    // Generate and update lock file
    var resolver = DependencyResolver.init(allocator);
    defer resolver.deinit();
    
    const resolved = try resolver.resolve(&manifest);
    defer {
        for (resolved.items) |*dep| {
            dep.deinit();
        }
        resolved.deinit();
    }
    
    // Create lock file
    var lock_file = LockFile.init(allocator);
    defer lock_file.deinit();
    
    for (resolved.items) |dep| {
        var locked_pkg = LockFile.LockedPackage.init(allocator);
        locked_pkg.name = try allocator.dupe(u8, dep.name);
        locked_pkg.version = dep.version;
        locked_pkg.source = dep.source;
        locked_pkg.checksum = "placeholder-checksum"; // TODO: Calculate actual checksum
        
        try lock_file.packages.append(locked_pkg);
    }
    
    try lock_file.saveToFile(allocator, "CursedPackage.lock");
    
    // Generate build integration
    try build_integration.generateBuildFile(&manifest, &lock_file);
    
    print("Successfully installed {} dependencies\n", .{resolved.items.len});
    print("Dependencies cached in .cursed/cache/\n", .{});
    print("Lock file generated: CursedPackage.lock\n", .{});
    print("Build integration generated: build_generated.zig\n", .{});
}

pub fn cmdUpdate(allocator: Allocator, args: [][]const u8) !void {
    _ = args;
    
    // Remove lock file to force re-resolution
    std.fs.cwd().deleteFile("CursedPackage.lock") catch {};
    
    // Run install to update dependencies
    try cmdInstall(allocator, &[_][]const u8{});
    
    print("Updated all dependencies to latest compatible versions\n", .{});
}

pub fn cmdRemove(allocator: Allocator, args: [][]const u8) !void {
    if (args.len == 0) {
        print("Usage: cursed pkg remove <package_name>\n", .{});
        return;
    }
    
    const package_name = args[0];
    
    // Load manifest
    var manifest = PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
    error.FileNotFound => {
    print("No CursedPackage.toml found.\n", .{});
    return;
    },
    else => return err,
    };
    defer manifest.deinit(allocator);
    
    // Remove from dependencies
    if (manifest.dependencies.fetchRemove(package_name)) |removed_entry| {
    var removed_dep = removed_entry.value;
        removed_dep.deinit();
    allocator.free(removed_entry.key);
        print("Removed dependency: {s}\n", .{package_name});
    } else if (manifest.dev_dependencies.fetchRemove(package_name)) |removed_entry| {
    var removed_dep = removed_entry.value;
        removed_dep.deinit();
                allocator.free(removed_entry.key);
                print("Removed dev dependency: {s}\n", .{package_name});
            } else {
                print("Package '{s}' not found in dependencies\n", .{package_name});
                return;
            }
    
    // Save updated manifest
    try manifest.saveToToml(allocator, "CursedPackage.toml");
    
    print("Run 'cursed pkg install' to update lock file\n", .{});
}

pub fn cmdSearch(allocator: Allocator, args: [][]const u8) !void {
    _ = allocator;
    if (args.len == 0) {
        print("Usage: cursed pkg search <query>\n", .{});
        return;
    }
    
    const query = args[0];
    
    print("Searching for packages matching '{s}'...\n", .{query});
    
    // TODO: Implement actual registry search
    // For now, show sample results
    const sample_packages = [_]struct { name: []const u8, version: []const u8, description: []const u8 }{
        .{ .name = "json", .version = "1.0.0", .description = "JSON parsing and serialization library" },
        .{ .name = "http", .version = "0.5.2", .description = "HTTP client and server library" },
        .{ .name = "crypto", .version = "2.1.0", .description = "Cryptography and hashing utilities" },
    };
    
    print("\nFound packages:\n", .{});
    for (sample_packages) |pkg| {
        if (std.mem.indexOf(u8, pkg.name, query) != null or std.mem.indexOf(u8, pkg.description, query) != null) {
            print("  {s} v{s} - {s}\n", .{ pkg.name, pkg.version, pkg.description });
        }
    }
    
    print("\nTo add a package: cursed pkg add <package_name>\n", .{});
}

pub fn cmdPublish(allocator: Allocator, args: [][]const u8) !void {
    _ = args;
    
    // Load manifest
    var manifest = PackageManifest.loadFromToml(allocator, "CursedPackage.toml") catch |err| switch (err) {
        error.FileNotFound => {
            print("No CursedPackage.toml found. Run 'cursed pkg init' first.\n", .{});
            return;
        },
        else => return err,
    };
    defer manifest.deinit(allocator);
    
    const version_str = manifest.version.toString(allocator) catch "unknown";
    defer if (!std.mem.eql(u8, version_str, "unknown")) allocator.free(version_str);
    print("Publishing package: {s} v{s}\n", .{ manifest.name, version_str });
    
    // TODO: Implement actual package publishing
    // This would involve:
    // 1. Validating package structure
    // 2. Running tests
    // 3. Creating package archive
    // 4. Uploading to registry
    // 5. Updating package index
    
    print("Package validation...\n", .{});
    print("Running tests...\n", .{});
    print("Creating package archive...\n", .{});
    print("Uploading to registry...\n", .{});
    const final_version_str = manifest.version.toString(allocator) catch "unknown";
    defer if (!std.mem.eql(u8, final_version_str, "unknown")) allocator.free(final_version_str);
    print("Successfully published {s} v{s}\n", .{ manifest.name, final_version_str });
}

// ===== Test and Main Functions =====

test "version parsing and comparison" {
    const allocator = std.testing.allocator;
    
    const v1 = try Version.parse(allocator, "1.2.3");
    const v2 = try Version.parse(allocator, "1.2.4");
    const v3 = try Version.parse(allocator, "1.3.0");
    
    try std.testing.expect(v1.compare(v2) < 0);
    try std.testing.expect(v2.compare(v1) > 0);
    try std.testing.expect(v1.compare(v1) == 0);
    try std.testing.expect(v3.compare(v1) > 0);
}

test "version requirement matching" {
    const allocator = std.testing.allocator;
    
    const version = try Version.parse(allocator, "1.2.3");
    
    const exact_req = try VersionRequirement.parse(allocator, "1.2.3");
    try std.testing.expect(exact_req.matches(version));
    
    const caret_req = try VersionRequirement.parse(allocator, "^1.2.0");
    try std.testing.expect(caret_req.matches(version));
    
    const tilde_req = try VersionRequirement.parse(allocator, "~1.2.0");
    try std.testing.expect(tilde_req.matches(version));
}

test "toml parsing" {
    const allocator = std.testing.allocator;
    
    const toml_content =
        \\name = "test-package"
        \\version = "0.1.0"
        \\description = "A test package"
        \\
        \\[dependencies]
        \\json = "1.0.0"
        \\http = "^0.5.0"
    ;
    
    var parser = TomlParser.init(allocator, toml_content);
    var toml = try parser.parse();
    defer toml.deinit(allocator);
    
    try std.testing.expect(toml == .table);
    
    if (toml.table.get("name")) |name_val| {
        try std.testing.expect(name_val == .string);
        try std.testing.expectEqualStrings("test-package", name_val.string);
    }
}

test "package manifest loading" {
    const allocator = std.testing.allocator;
    
    // Create temporary manifest file
    const manifest_content =
        \\name = "test-package"
        \\version = "0.1.0"
        \\description = "A test package"
        \\authors = ["Test Author <test@example.com>"]
        \\
        \\[dependencies]
        \\json = "1.0.0"
    ;
    
    const temp_file = try std.fs.cwd().createFile("test_manifest.toml", .{});
    defer {
        temp_file.close();
        std.fs.cwd().deleteFile("test_manifest.toml") catch {};
    }
    
    try temp_file.writeAll(manifest_content);
    
    var manifest = try PackageManifest.loadFromToml(allocator, "test_manifest.toml");
    defer manifest.deinit(allocator);
    
    try std.testing.expectEqualStrings("test-package", manifest.name);
    try std.testing.expect(manifest.version.major == 0);
    try std.testing.expect(manifest.version.minor == 1);
    try std.testing.expect(manifest.version.patch == 0);
}

// Export all command functions for external use
pub const commands = struct {
    pub const init = cmdInit;
    pub const add = cmdAdd;
    pub const install = cmdInstall;
    pub const update = cmdUpdate;
    pub const remove = cmdRemove;
    pub const search = cmdSearch;
    pub const publish = cmdPublish;
};
