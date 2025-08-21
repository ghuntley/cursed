const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // CURSED Debugger executable (standalone minimal version)
    const debugger_exe = b.addExecutable(.{
        .name = "cursed-debug",
        .root_module = b.createModule(.{
            .root_source_file = b.path("debugger_final_beta.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    b.installArtifact(debugger_exe);

    // Run step for debugger
    const run_debug_cmd = b.addRunArtifact(debugger_exe);
    run_debug_cmd.step.dependOn(b.getInstallStep());
    
    if (b.args) |args| {
        run_debug_cmd.addArgs(args);
    }
    
    const run_debug_step = b.step("debug", "Run the CURSED debugger");
    run_debug_step.dependOn(&run_debug_cmd.step);
}
