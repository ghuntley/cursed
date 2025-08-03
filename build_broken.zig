const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Create the CURSED compiler executable
    const exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_source_file = b.path("src-zig/main_simple.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Configure libc and system integration
    exe.linkLibC();
    
    // Use hardcoded NixOS paths for LLVM
    exe.addLibraryPath(.{ .cwd_relative = "/nix/store/rxp13pg5iidpmvlvy963n8nkkbc246iz-llvm-18.1.8-lib/lib" });
    exe.addIncludePath(.{ .cwd_relative = "/nix/store/19gmdqq62x11wv7ipni6grm5f8clcq7c-llvm-18.1.8-dev/include" });
    
    exe.linkSystemLibrary("LLVM-18");

    b.installArtifact(exe);

    // Create run step
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);
}
