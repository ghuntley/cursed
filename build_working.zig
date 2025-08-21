const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create minimal CURSED compiler executable
    const exe = b.addExecutable(.{
        .name = "cursed-minimal",
    });
    exe.setTarget(target);
    exe.setBuildMode(optimize);
    exe.root_module.root_source_file = b.path("cursed_minimal.zig");
    
    b.installArtifact(exe);

    // Run command
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the minimal CURSED compiler");
    run_step.dependOn(&run_cmd.step);

    // Demo step
    const demo_step = b.step("demo", "Show CURSED language demo");
    const demo_cmd = b.addRunArtifact(exe);
    demo_cmd.addArg("--demo");
    demo_step.dependOn(&demo_cmd.step);

    // Version step
    const version_step = b.step("version", "Show compiler version");
    const version_cmd = b.addRunArtifact(exe);
    version_cmd.addArg("--version");
    version_step.dependOn(&version_cmd.step);

    // Compatibility check
    const compat_check_step = b.step("check-compat", "Check Zig version compatibility");
    
    const compat_exe = b.addExecutable(.{
        .name = "compat-check",
    });
    compat_exe.setTarget(target);
    compat_exe.setBuildMode(optimize);
    compat_exe.root_module.root_source_file = b.addWriteFiles().add("compat_check.zig",
        \\const std = @import("std");
        \\pub fn main() void {
        \\    const version = @import("builtin").zig_version;
        \\    std.debug.print("Zig version: {}.{}.{}\n", .{version.major, version.minor, version.patch});
        \\    if (version.major == 0 and version.minor >= 15) {
        \\        std.debug.print("✅ Compatible\n", .{});
        \\    } else {
        \\        std.debug.print("❌ Incompatible\n", .{});
        \\    }
        \\}
    );
    
    const run_compat = b.addRunArtifact(compat_exe);
    compat_check_step.dependOn(&run_compat.step);
}
