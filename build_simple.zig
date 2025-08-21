const std = @import("std");

/// Simple compatibility-aware build script for CURSED  
/// Checks Zig version and reports compatibility issues
pub fn build(b: *std.Build) void {
    // Check Zig version for compatibility warnings
    const zig_version = @import("builtin").zig_version;
    
    if (zig_version.major == 0 and zig_version.minor >= 16) {
        std.log.warn("Using Zig {}.{}.{} - some features may be experimental", .{
            zig_version.major, zig_version.minor, zig_version.patch
        });
    }
    
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create main CURSED module - check if it exists
    const main_file_exists = blk: {
        std.fs.cwd().access("src-zig/main.zig", .{}) catch {
            break :blk false;
        };
        break :blk true;
    };

    if (main_file_exists) {
        // Full CURSED compiler
        const cursed_exe = b.addExecutable(.{
            .name = "cursed-zig",
            .root_source_file = b.path("src-zig/main.zig"),
            .target = target,
            .optimize = optimize,
        });
        
        b.installArtifact(cursed_exe);
        
        // Run command
        const run_cmd = b.addRunArtifact(cursed_exe);
        run_cmd.step.dependOn(b.getInstallStep());
        
        if (b.args) |args| {
            run_cmd.addArgs(args);
        }
        
        const run_step = b.step("run", "Run the CURSED compiler");
        run_step.dependOn(&run_cmd.step);
    }

    // Create minimal CURSED compiler (always available)
    const minimal_exe = b.addExecutable(.{
        .name = "cursed-minimal",
        .root_source_file = b.path("cursed_minimal.zig"),
        .target = target,
        .optimize = optimize,
    });
    
    b.installArtifact(minimal_exe);

    // Demo and version steps
    const demo_step = b.step("demo", "Show CURSED language demo");
    const demo_cmd = b.addRunArtifact(minimal_exe);
    demo_cmd.addArg("--demo");
    demo_step.dependOn(&demo_cmd.step);

    const version_step = b.step("version", "Show compiler version");
    const version_cmd = b.addRunArtifact(minimal_exe);
    version_cmd.addArg("--version");
    version_step.dependOn(&version_cmd.step);

    // Test step with basic compatibility tests
    const test_step = b.step("test", "Run all tests");
    
    // Test compatibility layer if it exists
    const compat_test_exists = blk: {
        std.fs.cwd().access("src-zig/zig_version.zig", .{}) catch {
            break :blk false;  
        };
        break :blk true;
    };
    
    if (compat_test_exists) {
        const compat_test = b.addTest(.{
            .root_source_file = b.path("src-zig/zig_version.zig"),
            .target = target,
            .optimize = optimize,
        });
        
        const run_compat_test = b.addRunArtifact(compat_test);
        test_step.dependOn(&run_compat_test.step);
    }

    // Test other components if they exist
    const test_files = [_][]const u8{
        "src-zig/lexer.zig",
        "src-zig/parser.zig",
        "src-zig/ast.zig",
    };
    
    for (test_files) |test_file| {
        const file_exists = blk: {
            std.fs.cwd().access(test_file, .{}) catch {
                break :blk false;
            };
            break :blk true;
        };
        
        if (file_exists) {
            const test_exe = b.addTest(.{
                .root_source_file = b.path(test_file),
                .target = target,
                .optimize = optimize,
            });
            
            const run_test = b.addRunArtifact(test_exe);
            test_step.dependOn(&run_test.step);
        }
    }
    
    // Compatibility check step
    const compat_check_step = b.step("check-compat", "Check Zig version compatibility");
    
    // Create inline compatibility checker
    const compat_checker = b.addExecutable(.{
        .name = "zig-compat-check",
        .root_source_file = b.addWriteFiles().add("compat_check.zig", 
            \\const std = @import("std");
            \\
            \\pub fn main() !void {
            \\    const version = @import("builtin").zig_version;
            \\    const min_major = 0;
            \\    const min_minor = 15;
            \\    const min_patch = 1;
            \\    
            \\    std.log.info("=== Zig Compatibility Check ===", .{});
            \\    std.log.info("Current version: {}.{}.{}", .{version.major, version.minor, version.patch});
            \\    std.log.info("Minimum required: {}.{}.{}", .{min_major, min_minor, min_patch});
            \\    
            \\    if (version.major < min_major or 
            \\        (version.major == min_major and version.minor < min_minor) or
            \\        (version.major == min_major and version.minor == min_minor and version.patch < min_patch)) {
            \\        std.log.err("❌ Unsupported Zig version", .{});
            \\        return error.UnsupportedVersion;
            \\    }
            \\    
            \\    if (version.minor >= 16) {
            \\        std.log.warn("⚠️  Using experimental Zig version", .{});
            \\    } else {
            \\        std.log.info("✅ Compatible Zig version", .{});
            \\    }
            \\}
        ),
        .target = target,
        .optimize = optimize,
    });
    
    const run_compat_check = b.addRunArtifact(compat_checker);
    compat_check_step.dependOn(&run_compat_check.step);
}
