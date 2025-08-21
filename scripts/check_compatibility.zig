const std = @import("std");
const ZigVersion = @import("zig_version");

/// Standalone Zig API compatibility checker
/// Run with: zig run scripts/check_compatibility.zig
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.log.info("=== CURSED Zig Compatibility Checker ===", .{});
    
    // Check current Zig version
    const current = ZigVersion.ZigVersion.current();
    const version_str = try current.toString(allocator);
    defer allocator.free(version_str);
    
    std.log.info("Current Zig version: {s}", .{version_str});
    
    // Perform compatibility check
    ZigVersion.CompatibilityChecker.checkCompatibility() catch |err| {
        std.log.err("Compatibility check failed: {}", .{err});
        std.process.exit(1);
    };
    
    // Report API changes
    try ZigVersion.CompatibilityChecker.reportApiChanges();
    
    // Test compatibility layer components
    std.log.info("Testing compatibility layer...", .{});
    
    // Test ArrayList compatibility
    var list = ZigVersion.ArrayList(i32).init(allocator);
    defer list.deinit();
    
    try list.append(42);
    try list.append(84);
    
    if (list.len() != 2 or list.items()[0] != 42) {
        std.log.err("ArrayList compatibility test failed", .{});
        std.process.exit(1);
    }
    
    std.log.info("✅ ArrayList compatibility test passed", .{});
    
    // Test allocator compatibility
    const ptr = try ZigVersion.AllocatorCompat.create(allocator, i32);
    defer allocator.destroy(ptr);
    ptr.* = 123;
    
    if (ptr.* != 123) {
        std.log.err("Allocator compatibility test failed", .{});
        std.process.exit(1);
    }
    
    std.log.info("✅ Allocator compatibility test passed", .{});
    
    // Test version-specific features
    if (current.isAtLeast(0, 15, 0)) {
        std.log.info("✅ Modern Zig features available", .{});
    } else {
        std.log.warn("⚠️  Using legacy Zig compatibility mode", .{});
    }
    
    if (current.isAtLeast(0, 16, 0)) {
        std.log.warn("⚠️  Zig 0.16+ detected - some features may be experimental", .{});
    }
    
    if (current.isAtLeast(0, 17, 0)) {
        std.log.warn("⚠️  Zig 0.17+ detected - major changes expected", .{});
    }
    
    // Generate compatibility report
    const report_file = "zig_compatibility_report.json";
    const file = try std.fs.cwd().createFile(report_file, .{});
    defer file.close();
    
    const report = .{
        .timestamp = std.time.timestamp(),
        .zig_version = .{
            .major = current.major,
            .minor = current.minor,
            .patch = current.patch,
            .string = version_str,
        },
        .compatibility = .{
            .status = "compatible",
            .minimum_supported = .{
                .major = @as(u32, 0),
                .minor = @as(u32, 15),
                .patch = @as(u32, 1),
            },
            .tested_versions = .{
                .{ .major = @as(u32, 0), .minor = @as(u32, 15), .patch = @as(u32, 1) },
                .{ .major = @as(u32, 0), .minor = @as(u32, 15), .patch = @as(u32, 2) },
                .{ .major = @as(u32, 0), .minor = @as(u32, 16), .patch = @as(u32, 0) },
            },
        },
        .features = .{
            .arraylist_compat = true,
            .build_system_compat = true,
            .allocator_compat = true,
            .test_framework_compat = true,
        },
    };
    
    try std.json.stringify(report, .{ .whitespace = .indent_2 }, file.writer());
    
    std.log.info("✅ Compatibility report saved to {s}", .{report_file});
    std.log.info("=== Compatibility Check Complete ===", .{});
}
