// Enhanced build script for CURSED development tools
// Integrates pure CURSED development tools with the main build system

const std = @import("std");
const Builder = std.build.Builder;
const LibExeObjStep = std.build.LibExeObjStep;

pub fn build(b: *Builder) void {
    const target = b.standardTargetOptions(.{});
    const mode = b.standardReleaseOptions();

    // Main CURSED interpreter (needed to run pure CURSED tools)
    const cursed = b.addExecutable("cursed-dev", "src-zig/minimal_main.zig");
    cursed.setTarget(target);
    cursed.setBuildMode(mode);
    cursed.linkLibC();
    cursed.install();

    // Development tool CLI wrappers
    const fmt_cli = b.addExecutable("cursed-fmt", "tools/cli/fmt_cli.zig");
    fmt_cli.setTarget(target);
    fmt_cli.setBuildMode(mode);
    fmt_cli.linkLibC();
    fmt_cli.install();

    const lint_cli = b.addExecutable("cursed-lint", "tools/cli/lint_cli.zig");
    lint_cli.setTarget(target);
    lint_cli.setBuildMode(mode);
    lint_cli.linkLibC();
    lint_cli.install();

    const lsp_enhanced_cli = b.addExecutable("cursed-lsp-enhanced", "tools/cli/lsp_enhanced_cli.zig");
    lsp_enhanced_cli.setTarget(target);
    lsp_enhanced_cli.setBuildMode(mode);
    lsp_enhanced_cli.linkLibC();
    lsp_enhanced_cli.install();

    // Build steps for running tools
    const fmt_step = b.step("fmt", "Format CURSED code using enhanced formatter");
    const fmt_run = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed-fmt",
    });
    fmt_step.dependOn(&fmt_run.step);
    fmt_run.step.dependOn(&fmt_cli.install_step.?.step);

    const lint_step = b.step("lint", "Lint CURSED code using enhanced linter");
    const lint_run = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed-lint",
    });
    lint_step.dependOn(&lint_run.step);
    lint_run.step.dependOn(&lint_cli.install_step.?.step);

    const lsp_enhanced_step = b.step("lsp-enhanced", "Run enhanced CURSED LSP server");
    const lsp_enhanced_run = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed-lsp-enhanced",
    });
    lsp_enhanced_step.dependOn(&lsp_enhanced_run.step);
    lsp_enhanced_run.step.dependOn(&lsp_enhanced_cli.install_step.?.step);

    // Test development tools
    const test_tools_step = b.step("test-tools", "Test all development tools");
    const test_fmt = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed-fmt", "--test"
    });
    const test_lint = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed-lint", "--test"
    });
    
    test_tools_step.dependOn(&test_fmt.step);
    test_tools_step.dependOn(&test_lint.step);
    test_fmt.step.dependOn(&fmt_cli.install_step.?.step);
    test_lint.step.dependOn(&lint_cli.install_step.?.step);
}
