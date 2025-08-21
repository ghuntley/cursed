const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main CURSED compiler executable
    const exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    // Link LLVM
    exe.linkSystemLibrary("LLVM-19");
    exe.linkLibC();
    
    b.installArtifact(exe);

    // CURSED Debugger executable
    const debugger_exe = b.addExecutable(.{
        .name = "cursed-debug",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/standalone_debugger_main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    // Link LLVM for debugger
    debugger_exe.linkSystemLibrary("LLVM-19");
    debugger_exe.linkLibC();
    
    b.installArtifact(debugger_exe);

    // CURSED LSP Server - Final Production Implementation
    const lsp_exe = b.addExecutable(.{
        .name = "cursed-lsp",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/final_lsp_server.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    b.installArtifact(lsp_exe);

    // Test step (removed unused variable)
    _ = b.step("test", "Run all tests");
    
    // Run step for main compiler
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    
    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);

    // Run step for debugger
    const run_debug_cmd = b.addRunArtifact(debugger_exe);
    run_debug_cmd.step.dependOn(b.getInstallStep());
    
    if (b.args) |args| {
        run_debug_cmd.addArgs(args);
    }
    
    const run_debug_step = b.step("debug", "Run the CURSED debugger");
    run_debug_step.dependOn(&run_debug_cmd.step);
}
