// Build script for CURSED tooling infrastructure
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // LSP Server
    const lsp_exe = b.addExecutable(.{
        .name = "cursed-lsp",
        .root_source_file = .{ .path = "src-zig/tools/lsp_server.zig" },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(lsp_exe);

    // Code Formatter
    const formatter_exe = b.addExecutable(.{
        .name = "cursed-fmt",
        .root_source_file = .{ .path = "src-zig/tools/formatter.zig" },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(formatter_exe);

    // Linter
    const linter_exe = b.addExecutable(.{
        .name = "cursed-lint",
        .root_source_file = .{ .path = "src-zig/tools/linter.zig" },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(linter_exe);

    // Package Manager
    const pkg_exe = b.addExecutable(.{
        .name = "cursed-pkg",
        .root_source_file = .{ .path = "src-zig/tools/package_manager.zig" },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(pkg_exe);

    // Documentation Generator
    const doc_exe = b.addExecutable(.{
        .name = "cursed-doc",
        .root_source_file = .{ .path = "src-zig/tools/doc_generator.zig" },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(doc_exe);

    // Unified Tools Interface
    const tools_exe = b.addExecutable(.{
        .name = "cursed-tools",
        .root_source_file = .{ .path = "src-zig/tools/mod.zig" },
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(tools_exe);

    // Build all tools step
    const build_tools_step = b.step("tools", "Build all CURSED development tools");
    build_tools_step.dependOn(&lsp_exe.step);
    build_tools_step.dependOn(&formatter_exe.step);
    build_tools_step.dependOn(&linter_exe.step);
    build_tools_step.dependOn(&pkg_exe.step);
    build_tools_step.dependOn(&doc_exe.step);
    build_tools_step.dependOn(&tools_exe.step);

    // Test steps
    const test_tools_step = b.step("test-tools", "Test all tooling functionality");
    
    const tools_test = b.addTest(.{
        .root_source_file = .{ .path = "src-zig/tools/mod.zig" },
        .target = target,
        .optimize = optimize,
    });
    test_tools_step.dependOn(&tools_test.step);
}
