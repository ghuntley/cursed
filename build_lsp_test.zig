const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // LSP Server executable for testing
    const lsp_exe = b.addExecutable(.{
        .name = "cursed-lsp-test",
        .root_source_file = b.path("src-zig/lsp_server_fixed.zig"),
        .target = target,
        .optimize = optimize,
    });
    
    lsp_exe.linkLibC();
    b.installArtifact(lsp_exe);

    // Main compiler executable - minimal version
    const exe = b.addExecutable(.{
        .name = "cursed-zig-test",
        .root_source_file = b.path("src-zig/minimal_main.zig"),
        .target = target,
        .optimize = optimize,
    });
    
    exe.linkLibC();
    b.installArtifact(exe);
}
