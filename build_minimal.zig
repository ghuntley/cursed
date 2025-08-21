const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create minimal CURSED compiler - ensure cursed_minimal.zig exists
    const minimal_exe = b.addExecutable(.{
        .name = "cursed-minimal",
        .target = target,
        .optimize = optimize,
    });
    
    minimal_exe.root_module.root_source_file = b.path("cursed_minimal.zig");
    
    b.installArtifact(minimal_exe);

    // Run commands
    const run_cmd = b.addRunArtifact(minimal_exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the minimal CURSED compiler");
    run_step.dependOn(&run_cmd.step);

    // Demo step
    const demo_step = b.step("demo", "Show CURSED language demo");
    const demo_cmd = b.addRunArtifact(minimal_exe);
    demo_cmd.addArg("--demo");
    demo_step.dependOn(&demo_cmd.step);

    // Version step with compatibility info
    const version_step = b.step("version", "Show compiler and Zig version");
    const version_cmd = b.addRunArtifact(minimal_exe);
    version_cmd.addArg("--version");
    version_step.dependOn(&version_cmd.step);
    
    // Compatibility check step
    const compat_check_step = b.step("check-compat", "Check Zig version compatibility");
    
    const compat_checker = b.addExecutable(.{
        .name = "zig-compat-check",
        .target = target,
        .optimize = optimize,
    });
    
    compat_checker.root_module.root_source_file = b.addWriteFiles().add("compat_check.zig", 
        \\const std = @import("std");
        \\
        \\pub fn main() !void {
        \\    const version = @import("builtin").zig_version;
        \\    std.log.info("=== CURSED Zig Compatibility Check ===", .{});
        \\    std.log.info("Current Zig version: {}.{}.{}", .{version.major, version.minor, version.patch});
        \\    std.log.info("Minimum required: 0.15.1", .{});
        \\    
        \\    if (version.major == 0 and version.minor == 15 and version.patch >= 1) {
        \\        std.log.info("✅ Compatible Zig version", .{});
        \\    } else if (version.major == 0 and version.minor >= 16) {
        \\        std.log.warn("⚠️  Using newer Zig version - some features may be experimental", .{});
        \\    } else {
        \\        std.log.err("❌ Unsupported Zig version", .{});
        \\        return error.UnsupportedVersion;
        \\    }
        \\    
        \\    // Test basic build features
        \\    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        \\    defer _ = gpa.deinit();
        \\    const allocator = gpa.allocator();
        \\    
        \\    var list = std.ArrayList(i32).init(allocator);
        \\    defer list.deinit();
        \\    
        \\    try list.append(42);
        \\    if (list.items.len == 1 and list.items[0] == 42) {
        \\        std.log.info("✅ ArrayList API working", .{});
        \\    } else {
        \\        std.log.err("❌ ArrayList API broken", .{});
        \\        return error.APIBroken;
        \\    }
        \\    
        \\    std.log.info("=== Compatibility Check Complete ===", .{});
        \\}
    );
    
    const run_compat_check = b.addRunArtifact(compat_checker);
    compat_check_step.dependOn(&run_compat_check.step);
    
    // Test step
    const test_step = b.step("test", "Run compatibility tests");
    
    // Test zig_version.zig if it exists
    const compat_file_exists = blk: {
        std.fs.cwd().access("src-zig/zig_version.zig", .{}) catch {
            break :blk false;
        };
        break :blk true;
    };
    
    if (compat_file_exists) {
        const compat_test = b.addTest(.{
            .target = target,
            .optimize = optimize,
        });
        compat_test.root_module.root_source_file = b.path("src-zig/zig_version.zig");
        
        const run_compat_test = b.addRunArtifact(compat_test);
        test_step.dependOn(&run_compat_test.step);
    }
}
