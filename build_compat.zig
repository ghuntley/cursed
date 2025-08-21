const std = @import("std");
const ZigVersion = @import("src-zig/zig_version.zig");

/// Compatibility-aware build script for CURSED
/// Uses the zig_version compatibility layer to handle API differences
pub fn build(b: *std.Build) void {
    // Check Zig version compatibility at build time
    ZigVersion.CompatibilityChecker.checkCompatibility() catch |err| {
        std.log.err("Zig version compatibility check failed: {}", .{err});
        std.process.exit(1);
    };
    
    // Report any known API changes
    ZigVersion.CompatibilityChecker.reportApiChanges() catch {};
    
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Use compatibility layer for module creation
    const cursed_module = ZigVersion.BuildCompat.addModule(b, "cursed", .{
        .root_source_file = ZigVersion.BuildCompat.LazyPath("src-zig/main.zig"),
    });
    
    // Add zig_version module to all executables
    const zig_version_module = ZigVersion.BuildCompat.addModule(b, "zig_version", .{
        .root_source_file = ZigVersion.BuildCompat.LazyPath("src-zig/zig_version.zig"),
    });
    
    // Create main CURSED compiler executable with compatibility layer
    const cursed_exe = ZigVersion.BuildCompat.addExecutable(b, .{
        .name = "cursed-zig",
        .root_module = cursed_module,
        .target = target,
        .optimize = optimize,
    });
    
    // Add compatibility module to executable
    cursed_exe.root_module.addImport("zig_version", zig_version_module);
    
    b.installArtifact(cursed_exe);
    
    // Create minimal CURSED compiler
    const minimal_module = ZigVersion.BuildCompat.addModule(b, "cursed_minimal", .{
        .root_source_file = ZigVersion.BuildCompat.LazyPath("cursed_minimal.zig"),
    });
    
    const minimal_exe = ZigVersion.BuildCompat.addExecutable(b, .{
        .name = "cursed-minimal",
        .root_module = minimal_module, 
        .target = target,
        .optimize = optimize,
    });
    
    minimal_exe.root_module.addImport("zig_version", zig_version_module);
    
    b.installArtifact(minimal_exe);
    
    // Create LSP server
    const lsp_module = ZigVersion.BuildCompat.addModule(b, "cursed_lsp", .{
        .root_source_file = ZigVersion.BuildCompat.LazyPath("src-zig/lsp.zig"),
    });
    
    const lsp_exe = ZigVersion.BuildCompat.addExecutable(b, .{
        .name = "cursed-lsp",
        .root_module = lsp_module,
        .target = target,
        .optimize = optimize,
    });
    
    lsp_exe.root_module.addImport("zig_version", zig_version_module);
    
    b.installArtifact(lsp_exe);
    
    // Create formatter
    const fmt_exe = ZigVersion.BuildCompat.addExecutable(b, .{
        .name = "cursed-fmt",
        .root_module = ZigVersion.BuildCompat.addModule(b, "cursed_fmt", .{
            .root_source_file = ZigVersion.BuildCompat.LazyPath("src-zig/formatter.zig"),
        }),
        .target = target,
        .optimize = optimize,
    });
    
    fmt_exe.root_module.addImport("zig_version", zig_version_module);
    
    b.installArtifact(fmt_exe);
    
    // Create linter
    const lint_exe = ZigVersion.BuildCompat.addExecutable(b, .{
        .name = "cursed-lint",
        .root_module = ZigVersion.BuildCompat.addModule(b, "cursed_lint", .{
            .root_source_file = ZigVersion.BuildCompat.LazyPath("src-zig/linter.zig"),
        }),
        .target = target,
        .optimize = optimize,
    });
    
    lint_exe.root_module.addImport("zig_version", zig_version_module);
    
    b.installArtifact(lint_exe);
    
    // Tests with compatibility layer
    const test_step = b.step("test", "Run all tests");
    
    // Test the compatibility layer itself
    const zig_version_test = b.addTest(.{
        .root_module = ZigVersion.BuildCompat.addModule(b, "zig_version_test", .{
            .root_source_file = ZigVersion.BuildCompat.LazyPath("src-zig/zig_version.zig"),
        }),
    });
    
    const run_zig_version_test = b.addRunArtifact(zig_version_test);
    test_step.dependOn(&run_zig_version_test.step);
    
    // Test other components
    const test_files = [_][]const u8{
        "src-zig/lexer.zig",
        "src-zig/parser.zig", 
        "src-zig/ast.zig",
        "src-zig/interpreter.zig",
    };
    
    for (test_files) |test_file| {
        // Check if file exists before creating test
        const file_path = std.fmt.allocPrint(b.allocator, "{s}", .{test_file}) catch continue;
        defer b.allocator.free(file_path);
        
        if (std.fs.cwd().access(file_path, .{})) {
            const test_exe = b.addTest(.{
                .root_module = ZigVersion.BuildCompat.addModule(b, test_file, .{
                    .root_source_file = ZigVersion.BuildCompat.LazyPath(test_file),
                }),
            });
            
            test_exe.root_module.addImport("zig_version", zig_version_module);
            
            const run_test = b.addRunArtifact(test_exe);
            test_step.dependOn(&run_test.step);
        } else |_| {
            std.log.info("Skipping test for non-existent file: {s}", .{test_file});
        }
    }
    
    // Run steps with compatibility checks
    const run_cmd = b.addRunArtifact(cursed_exe);
    run_cmd.step.dependOn(b.getInstallStep());
    
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    
    const run_step = b.step("run", "Run the CURSED compiler");
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
    const compat_checker = ZigVersion.BuildCompat.addModule(b, "compat_checker", .{
        .root_source_file = ZigVersion.BuildCompat.LazyPath("scripts/check_compatibility.zig"),
    });
    const compat_exe = ZigVersion.BuildCompat.addExecutable(b, .{
        .name = "compat-checker",
        .root_module = compat_checker,
    });
    compat_exe.root_module.addImport("zig_version", zig_version_module);
    const compat_cmd = b.addRunArtifact(compat_exe);
    compat_check_step.dependOn(&compat_cmd.step);
}
