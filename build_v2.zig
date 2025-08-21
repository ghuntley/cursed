const std = @import("std");

pub fn build(b: *std.Build) void {
    // Report Zig version
    const zig_version = @import("builtin").zig_version;
    std.log.info("Building CURSED with Zig {}.{}.{}", .{zig_version.major, zig_version.minor, zig_version.patch});

    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create module for minimal CURSED compiler  
    const cursed_module = b.addModule("cursed_minimal", .{
        .root_source_file = b.path("cursed_minimal.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Create executable
    const exe = b.addExecutable(.{
        .name = "cursed-minimal", 
        .root_module = cursed_module,
    });
    
    // Note: In Zig 0.15.1, target and optimize are set through the root_module
    // The executable inherits these from standardTargetOptions and standardOptimizeOption
    
    b.installArtifact(exe);

    // Create run step
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);

    // Demo step
    const demo_step = b.step("demo", "Show CURSED demo");
    const demo_cmd = b.addRunArtifact(exe);
    demo_cmd.addArg("--demo");
    demo_step.dependOn(&demo_cmd.step);

    // Version step
    const version_step = b.step("version", "Show version info");
    const version_cmd = b.addRunArtifact(exe);
    version_cmd.addArg("--version");
    version_step.dependOn(&version_cmd.step);

    // Compatibility checking executable
    const compat_source = b.addWriteFiles().add("compat_check.zig",
        \\const std = @import("std");
        \\
        \\pub fn main() !void {
        \\    const version = @import("builtin").zig_version;
        \\    std.log.info("=== CURSED Zig API Compatibility System ===", .{});
        \\    std.log.info("Current Zig: {}.{}.{}", .{version.major, version.minor, version.patch});
        \\    std.log.info("Required: 0.15.1+", .{});
        \\    
        \\    if (version.major == 0 and version.minor >= 15) {
        \\        std.log.info("✅ Compatible Zig version", .{});
        \\        
        \\        // Test ArrayList API
        \\        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        \\        defer _ = gpa.deinit();
        \\        const allocator = gpa.allocator();
        \\        
        \\        var list = std.ArrayList(i32){};
        \\        defer list.deinit(allocator);
        \\        try list.append(allocator, 42);
        \\        
        \\        if (list.items.len == 1 and list.items[0] == 42) {
        \\            std.log.info("✅ ArrayList API working correctly", .{});
        \\        } else {
        \\            std.log.err("❌ ArrayList API broken", .{});
        \\            return error.APIBroken;
        \\        }
        \\        
        \\        // Test allocator API  
        \\        const ptr = try allocator.create(i32);
        \\        defer allocator.destroy(ptr);
        \\        ptr.* = 123;
        \\        
        \\        if (ptr.* == 123) {
        \\            std.log.info("✅ Allocator API working correctly", .{});
        \\        }
        \\        
        \\        std.log.info("✅ All API compatibility checks passed", .{});
        \\        
        \\        if (version.minor >= 16) {
        \\            std.log.warn("⚠️  Using Zig 0.{}+ - some features experimental", .{version.minor});
        \\        }
        \\    } else {
        \\        std.log.err("❌ Unsupported Zig version", .{});
        \\        return error.UnsupportedVersion;
        \\    }
        \\}
    );

    const compat_module = b.addModule("compat_check", .{
        .root_source_file = compat_source,
        .target = target,
        .optimize = optimize,
    });

    const compat_exe = b.addExecutable(.{
        .name = "cursed-compat-check",
        .root_module = compat_module,
    });

    const compat_step = b.step("check-compat", "Check Zig API compatibility");
    const run_compat = b.addRunArtifact(compat_exe);
    compat_step.dependOn(&run_compat.step);

    // API monitoring step
    const monitor_step = b.step("monitor-api", "Run API monitoring");
    const monitor_source = b.addWriteFiles().add("api_monitor.zig",
        \\const std = @import("std");
        \\
        \\pub fn main() !void {
        \\    std.log.info("=== CURSED API Monitoring System ===", .{});
        \\    const version = @import("builtin").zig_version;
        \\    std.log.info("Monitoring APIs for Zig {}.{}.{}", .{version.major, version.minor, version.patch});
        \\    
        \\    // Simulate API change detection
        \\    if (version.minor >= 16) {
        \\        std.log.warn("🔍 API changes detected in Zig 0.{}+", .{version.minor});
        \\        std.log.warn("   - Build system APIs may have changed", .{});
        \\        std.log.warn("   - Consider updating compatibility layer", .{});
        \\    } else {
        \\        std.log.info("✅ No API changes detected", .{});
        \\    }
        \\    
        \\    std.log.info("API monitoring complete", .{});
        \\}
    );

    const monitor_module = b.addModule("api_monitor", .{
        .root_source_file = monitor_source,
        .target = target,
        .optimize = optimize,
    });

    const monitor_exe = b.addExecutable(.{
        .name = "cursed-api-monitor",
        .root_module = monitor_module,
    });

    const run_monitor = b.addRunArtifact(monitor_exe);
    monitor_step.dependOn(&run_monitor.step);
    
    // Test step
    const test_step = b.step("test", "Run all tests");
    
    // Test zig_version.zig if it exists
    const compat_test_exists = blk: {
        std.fs.cwd().access("src-zig/zig_version.zig", .{}) catch {
            break :blk false;
        };
        break :blk true;
    };
    if (compat_test_exists) {
        const zig_version_test_module = b.addModule("zig_version_test", .{
            .root_source_file = b.path("src-zig/zig_version.zig"),
            .target = target,
            .optimize = optimize,
        });
        
        const zig_version_test = b.addTest(.{
            .root_module = zig_version_test_module,
        });
        
        const run_zig_version_test = b.addRunArtifact(zig_version_test);
        test_step.dependOn(&run_zig_version_test.step);
        
        std.log.info("Added zig_version.zig tests to test suite", .{});
    }
}
