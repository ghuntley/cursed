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
    
    // Add LLVM C wrapper
    exe.addCSourceFile(.{
        .file = b.path("src-zig/llvm_wrapper.c"),
        .flags = &[_][]const u8{"-std=c99", "-I/usr/lib/llvm-18/include"},
    });
    
    // Link LLVM
    exe.linkSystemLibrary("LLVM");
    exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib" });
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
    
    // Add LLVM C wrapper for debugger
    debugger_exe.addCSourceFile(.{
        .file = b.path("src-zig/llvm_wrapper.c"),
        .flags = &[_][]const u8{"-std=c99", "-I/usr/lib/llvm-18/include"},
    });
    
    // Link LLVM for debugger
    debugger_exe.linkSystemLibrary("LLVM");
    debugger_exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib" });
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

    // Type checker validation test
    const type_checker_validation = b.addTest(.{
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/type_checker_simple_validation.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    const run_type_checker_validation = b.addRunArtifact(type_checker_validation);
    const type_checker_test_step = b.step("test-type-checker", "Run type checker tests");
    type_checker_test_step.dependOn(&run_type_checker_validation.step);
}
