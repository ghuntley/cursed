const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create emergency interpreter that bypasses ArrayList API issues
    const exe = b.addExecutable(.{
        .name = "cursed-emergency",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/emergency_main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    b.installArtifact(exe);

    // Create legacy cursed-zig alias that points to the working interpreter
    const legacy_exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/emergency_main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    b.installArtifact(legacy_exe);

    // Create run step for emergency interpreter
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the emergency CURSED interpreter");
    run_step.dependOn(&run_cmd.step);

    // Create test step using working interpreter
    const test_step = b.step("test", "Run CURSED tests with working interpreter");
    
    const comprehensive_test_run = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed-zig", "comprehensive_stdlib_test.csd"
    });
    comprehensive_test_run.step.dependOn(b.getInstallStep());
    test_step.dependOn(&comprehensive_test_run.step);
}
