const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Create executable with no LLVM for now (let's just get it to compile)
    const exe = b.addExecutable(.{
        .name = "cursed-zig", 
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    // Try standard system library linking
    exe.linkLibC();
    
    b.installArtifact(exe);

    const run_step = b.step("run", "Run the CURSED compiler");
    const run_cmd = b.addRunArtifact(exe);
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    run_step.dependOn(&run_cmd.step);
}
