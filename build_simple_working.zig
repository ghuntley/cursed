const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create the main CURSED interpreter executable
    const exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_source_file = b.path("src-zig/main_simple.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Link LLVM libraries
    exe.linkSystemLibrary("LLVM");
    exe.addLibraryPath(std.build.LazyPath{ .cwd_relative = "/usr/lib/llvm-18/lib" });
    exe.addIncludePath(std.build.LazyPath{ .cwd_relative = "/usr/lib/llvm-18/include" });
    
    // Link C runtime
    exe.linkLibC();
    
    // Add LLVM C wrapper
    exe.addCSourceFile(.{
        .file = b.path("src-zig/llvm_wrapper.c"),
        .flags = &[_][]const u8{
            "-std=c99",
            "-I/usr/lib/llvm-18/include",
        },
    });

    b.installArtifact(exe);

    // Add run step
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED interpreter");
    run_step.dependOn(&run_cmd.step);
}
