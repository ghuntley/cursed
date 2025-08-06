const std = @import("std");

pub fn build(b: *std.Build) void {
    _ = b.standardTargetOptions(.{});
    _ = b.standardOptimizeOption(.{});

    // Create a simple CURSED compilation step
    const cursed_compile = b.addSystemCommand(&[_][]const u8{
        "../zig-out/bin/cursed-zig", "main.csd"
    });
    
    const run_step = b.step("run", "Run the CURSED program");
    run_step.dependOn(&cursed_compile.step);
    
    const compile_step = b.step("compile", "Compile the CURSED program");
    compile_step.dependOn(&cursed_compile.step);
}
