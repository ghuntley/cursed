const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create module for the minimal compiler
    const cursed_module = b.addModule("cursed_minimal", .{
        .root_source_file = b.path("cursed_minimal.zig"),
    });

    // Create executable using the module
    const exe = b.addExecutable(.{
        .name = "cursed-minimal", 
        .root_module = cursed_module,
    });
    
    exe.setTarget(target);
    exe.setBuildMode(optimize);
    
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

    // Compatibility check
    const compat_module = b.addModule("compat_check", .{
        .root_source_file = b.addWriteFiles().add("compat_check.zig",
            \\const std = @import("std");
            \\
            \\pub fn main() !void {
            \\    const version = @import("builtin").zig_version;
            \\    const stdout = std.io.getStdOut().writer();
            \\    
            \\    try stdout.print("=== CURSED Zig API Compatibility Check ===\n", .{});
            \\    try stdout.print("Zig version: {}.{}.{}\n", .{version.major, version.minor, version.patch});
            \\    
            \\    if (version.major == 0 and version.minor >= 15) {
            \\        try stdout.print("✅ Compatible Zig version\n", .{});
            \\        
            \\        // Test basic ArrayList API
            \\        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
            \\        defer _ = gpa.deinit();
            \\        const allocator = gpa.allocator();
            \\        
            \\        var list = std.ArrayList(i32).init(allocator);
            \\        defer list.deinit();
            \\        try list.append(42);
            \\        
            \\        if (list.items.len == 1) {
            \\            try stdout.print("✅ ArrayList API working\n", .{});
            \\        }
            \\        
            \\        try stdout.print("✅ All API compatibility checks passed\n", .{});
            \\    } else {
            \\        try stdout.print("❌ Incompatible Zig version\n", .{});
            \\        std.process.exit(1);
            \\    }
            \\}
        ),
    });

    const compat_exe = b.addExecutable(.{
        .name = "cursed-compat-check",
        .root_module = compat_module,
    });
    compat_exe.setTarget(target);
    compat_exe.setBuildMode(optimize);

    const compat_step = b.step("check-compat", "Check Zig API compatibility");
    const run_compat = b.addRunArtifact(compat_exe);
    compat_step.dependOn(&run_compat.step);
    
    // Test step
    const test_step = b.step("test", "Run tests");
    
    // Test zig_version.zig if it exists
    const compat_file_path = "src-zig/zig_version.zig";
    const compat_file_exists = std.fs.cwd().access(compat_file_path, .{}) catch false;
    
    if (compat_file_exists) {
        const compat_test = b.addTest(.{
            .target = target,
            .optimize = optimize,
        });
        compat_test.root_module.root_source_file = b.path(compat_file_path);
        
        const run_test = b.addRunArtifact(compat_test);
        test_step.dependOn(&run_test.step);
    } else {
        std.log.info("Compatibility test file not found, skipping", .{});
    }
}
