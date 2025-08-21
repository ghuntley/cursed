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

    // Run command
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);
}
