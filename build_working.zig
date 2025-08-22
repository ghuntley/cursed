const std = @import("std");

pub fn build(b: *std.Build) void {
    // Check Zig version and report compatibility
    const zig_version = @import("builtin").zig_version;
    std.log.info("Building with Zig {}.{}.{}", .{zig_version.major, zig_version.minor, zig_version.patch});
    
    if (zig_version.major == 0 and zig_version.minor >= 16) {
        std.log.warn("Using experimental Zig version - some features may not work", .{});
    }
    
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create CURSED interpreter executable
    const exe = b.addExecutable(.{
        .name = "cursed-zig",
    });
    exe.setTarget(target);
    exe.setBuildMode(optimize);
    exe.root_module.root_source_file = b.path("src-zig/main_simple.zig");
    
    b.installArtifact(exe);

    // Run command
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the minimal CURSED compiler");
    run_step.dependOn(&run_cmd.step);

    // Demo step
    const demo_step = b.step("demo", "Show CURSED language demo");
    const demo_cmd = b.addRunArtifact(exe);
    demo_cmd.addArg("--demo");
    demo_step.dependOn(&demo_cmd.step);

    // Version step
    const version_step = b.step("version", "Show compiler version");
    const version_cmd = b.addRunArtifact(exe);
    version_cmd.addArg("--version");
    version_step.dependOn(&version_cmd.step);
    
    // Add compatibility checking
    const compat_check_step = b.step("check-compat", "Check Zig version compatibility");
    
    const compat_exe = b.addExecutable(.{
        .name = "compat-check",
    });
    compat_exe.setTarget(target);
    compat_exe.setBuildMode(optimize);
    compat_exe.root_module.root_source_file = b.addWriteFiles().add("compat_check.zig", 
        \\const std = @import("std");
        \\
        \\pub fn main() !void {
        \\    const version = @import("builtin").zig_version;
        \\    const stdout = std.io.getStdOut().writer();
        \\    
        \\    try stdout.print("=== CURSED Zig API Compatibility System ===\n", .{});
        \\    try stdout.print("Current Zig version: {}.{}.{}\n", .{version.major, version.minor, version.patch});
        \\    try stdout.print("Minimum supported: 0.15.1\n", .{});
        \\    
        \\    if (version.major == 0 and version.minor == 15 and version.patch >= 1) {
        \\        try stdout.print("✅ Compatible Zig version\n", .{});
        \\    } else if (version.major == 0 and version.minor >= 16) {
        \\        try stdout.print("⚠️  Using newer Zig version - may have experimental features\n", .{});
        \\    } else {
        \\        try stdout.print("❌ Unsupported Zig version\n", .{});
        \\        std.process.exit(1);
        \\    }
        \\    
        \\    // Test ArrayList API compatibility
        \\    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        \\    defer _ = gpa.deinit();
        \\    const allocator = gpa.allocator();
        \\    
        \\    var list = std.ArrayList(i32).init(allocator);
        \\    defer list.deinit();
        \\    
        \\    try list.append(42);
        \\    if (list.items.len == 1 and list.items[0] == 42) {
        \\        try stdout.print("✅ ArrayList API compatible\n", .{});
        \\    } else {
        \\        try stdout.print("❌ ArrayList API broken\n", .{});
        \\        return error.APIBroken;
        \\    }
        \\    
        \\    try stdout.print("✅ All compatibility checks passed\n", .{});
        \\}
    );
    
    const run_compat_check = b.addRunArtifact(compat_exe);
    compat_check_step.dependOn(&run_compat_check.step);
    
    // Test step - only test files that exist
    const test_step = b.step("test", "Run compatibility tests");
    
    // Test compatibility layer if it exists
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
        
        std.log.info("Added zig_version.zig compatibility tests", .{});
    }
}
