const std = @import("std");
const builtin = @import("builtin");

/// Zig API Compatibility Layer
/// Abstracts away version differences to ensure stable builds
/// across multiple Zig versions (0.15.1+)
pub const ZigVersion = struct {
    major: u32,
    minor: u32,
    patch: u32,
    
    const Self = @This();
    
    pub fn current() Self {
        return Self{
            .major = builtin.zig_version.major,
            .minor = builtin.zig_version.minor,
            .patch = builtin.zig_version.patch,
        };
    }
    
    pub fn isAtLeast(self: Self, major: u32, minor: u32, patch: u32) bool {
        if (self.major > major) return true;
        if (self.major < major) return false;
        if (self.minor > minor) return true;
        if (self.minor < minor) return false;
        return self.patch >= patch;
    }
    
    pub fn toString(self: Self, allocator: std.mem.Allocator) ![]u8 {
        return std.fmt.allocPrint(allocator, "{}.{}.{}", .{ self.major, self.minor, self.patch });
    }
};

/// ArrayList compatibility wrapper for Zig 0.15.1+
pub fn ArrayList(comptime T: type) type {
    return struct {
        inner: std.ArrayList(T),
        allocator: std.mem.Allocator,
        
        const Self = @This();
        
        pub fn init(allocator: std.mem.Allocator) Self {
            return Self{
                .inner = std.ArrayList(T){},
                .allocator = allocator,
            };
        }
        
        pub fn deinit(self: *Self) void {
            self.inner.deinit(self.allocator);
        }
        
        pub fn append(self: *Self, item: T) !void {
            try self.inner.append(self.allocator, item);
        }
        
        pub fn appendSlice(self: *Self, slice: []const T) !void {
            try self.inner.appendSlice(self.allocator, slice);
        }
        
        pub fn items(self: Self) []T {
            return self.inner.items;
        }
        
        pub fn len(self: Self) usize {
            return self.inner.items.len;
        }
        
        pub fn clearAndFree(self: *Self) void {
            self.inner.clearAndFree(self.allocator);
        }
    };
}

/// Build system compatibility wrappers
pub const BuildCompat = struct {
    /// ExecutableOptions wrapper for version compatibility
    pub fn ExecutableOptions(comptime _: *std.Build) type {
        const version = ZigVersion.current();
        
        if (version.isAtLeast(0, 15, 0)) {
            return struct {
                name: []const u8,
                root_source_file: ?std.Build.LazyPath = null,
                root_module: ?*std.Build.Module = null,
                target: std.Build.ResolvedTarget,
                optimize: std.builtin.OptimizeMode,
                version: ?std.SemanticVersion = null,
                max_rss: ?usize = null,
                link_libc: ?bool = null,
                single_threaded: ?bool = null,
                pic: ?bool = null,
                strip: ?bool = null,
                unwind_tables: ?bool = null,
                omit_frame_pointer: ?bool = null,
                sanitize_thread: ?bool = null,
                error_tracing: ?bool = null,
                use_llvm: ?bool = null,
                use_lld: ?bool = null,
            };
        } else {
            return struct {
                name: []const u8,
                root_source_file: ?std.Build.FileSource = null,
                target: std.zig.CrossTarget,
                optimize: std.builtin.Mode,
            };
        }
    }
    
    /// Create executable with version-appropriate options
    pub fn addExecutable(b: *std.Build, options: anytype) *std.Build.Step.Compile {
        // For Zig 0.15.1, pass through all options
        return b.addExecutable(options);
    }
    
    /// Module creation wrapper
    pub fn addModule(b: *std.Build, name: []const u8, options: anytype) *std.Build.Module {
        // For Zig 0.15.1, use the standard API
        return b.addModule(name, .{
            .root_source_file = options.root_source_file,
        });
    }
    
    /// Path wrapper for different Zig versions
    pub fn LazyPath(path: []const u8) std.Build.LazyPath {
        // For newer Zig versions, use cwd_relative
        return std.Build.LazyPath{ .cwd_relative = path };
    }
};

/// Test API compatibility
pub const TestCompat = struct {
    pub fn expectEqual(expected: anytype, actual: anytype) !void {
        const version = ZigVersion.current();
        
        if (version.isAtLeast(0, 15, 0)) {
            try std.testing.expectEqual(expected, actual);
        } else {
            try std.testing.expect(expected == actual);
        }
    }
    
    pub fn expectEqualStrings(expected: []const u8, actual: []const u8) !void {
        const version = ZigVersion.current();
        
        if (version.isAtLeast(0, 15, 0)) {
            try std.testing.expectEqualStrings(expected, actual);
        } else {
            try std.testing.expect(std.mem.eql(u8, expected, actual));
        }
    }
};

/// Allocator compatibility
pub const AllocatorCompat = struct {
    pub fn create(allocator: std.mem.Allocator, comptime T: type) !*T {
        const version = ZigVersion.current();
        
        if (version.isAtLeast(0, 15, 0)) {
            return try allocator.create(T);
        } else {
            return try allocator.create(T);
        }
    }
    
    pub fn alloc(allocator: std.mem.Allocator, comptime T: type, len: usize) ![]T {
        const version = ZigVersion.current();
        
        if (version.isAtLeast(0, 15, 0)) {
            return try allocator.alloc(T, len);
        } else {
            return try allocator.alloc(T, len);
        }
    }
};

/// Version detection and warnings
pub const CompatibilityChecker = struct {
    pub fn checkCompatibility() !void {
        const current = ZigVersion.current();
        const min_version = ZigVersion{ .major = 0, .minor = 15, .patch = 1 };
        const max_tested_version = ZigVersion{ .major = 0, .minor = 16, .patch = 0 };
        
        if (!current.isAtLeast(min_version.major, min_version.minor, min_version.patch)) {
            std.log.err("Unsupported Zig version {}.{}.{}", .{ current.major, current.minor, current.patch });
            std.log.err("Minimum required version: {}.{}.{}", .{ min_version.major, min_version.minor, min_version.patch });
            return error.UnsupportedZigVersion;
        }
        
        if (current.isAtLeast(max_tested_version.major, max_tested_version.minor, max_tested_version.patch)) {
            std.log.warn("Using untested Zig version {}.{}.{}", .{ current.major, current.minor, current.patch });
            std.log.warn("Latest tested version: {}.{}.{}", .{ max_tested_version.major, max_tested_version.minor, max_tested_version.patch });
            std.log.warn("Build may fail or produce warnings", .{});
        }
        
        std.log.info("Using Zig version {}.{}.{} (compatible)", .{ current.major, current.minor, current.patch });
    }
    
    pub fn reportApiChanges() !void {
        const current = ZigVersion.current();
        
        // Report known API changes
        if (current.isAtLeast(0, 16, 0)) {
            std.log.warn("Zig 0.16+ detected - some APIs may have changed", .{});
        }
        
        if (current.isAtLeast(0, 17, 0)) {
            std.log.warn("Zig 0.17+ detected - major API overhaul expected", .{});
        }
    }
};

// Tests for the compatibility layer
test "zig version detection" {
    const version = ZigVersion.current();
    try TestCompat.expectEqual(true, version.major >= 0);
    try TestCompat.expectEqual(true, version.minor >= 12);
}

test "arraylist compatibility" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var list = .empty;
    defer list.deinit();
    
    try list.append(42);
    try TestCompat.expectEqual(@as(usize, 1), list.len());
    try TestCompat.expectEqual(@as(i32, 42), list.items()[0]);
}

test "build compatibility layer" {
    const version = ZigVersion.current();
    try TestCompat.expectEqual(true, version.isAtLeast(0, 12, 0));
}
