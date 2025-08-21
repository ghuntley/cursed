const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // CURSED Debugger executable
    const debugger_exe = b.addExecutable(.{
        .name = "cursed-debug",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/cursed_debug_main.zig"),
        }),
        .target = target,
        .optimize = optimize,
    });
    
    b.installArtifact(debugger_exe);

    // CURSED compiler with debug support
    const cursed_exe = b.addExecutable(.{
        .name = "cursed",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/cursed_debug_main.zig"),
        }),
        .target = target,
        .optimize = optimize,
    });
    
    b.installArtifact(cursed_exe);

    // Run command for debugger
    const run_debugger = b.addRunArtifact(debugger_exe);
    run_debugger.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_debugger.addArgs(args);
    }

    const run_debugger_step = b.step("debug", "Run the CURSED debugger");
    run_debugger_step.dependOn(&run_debugger.step);

    // Test step
    const test_step = b.step("test-debugger", "Test debugger functionality");
    const test_cmd = b.addRunArtifact(debugger_exe);
    test_cmd.addArg("test_basic.csd");
    test_step.dependOn(&test_cmd.step);
}
