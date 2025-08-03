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

    // Link LLVM using llvm-config
    exe.linkLibC();
    exe.linkSystemLibrary("LLVM-18");
    
    // Add LLVM library path
    exe.addLibraryPath(.{ .cwd_relative = "/nix/store/rxp13pg5iidpmvlvy963n8nkkbc246iz-llvm-18.1.8-lib/lib" });
    
    // Add include paths for LLVM
    exe.addIncludePath(.{ .cwd_relative = "/nix/store/19gmdqq62x11wv7ipni6grm5f8clcq7c-llvm-18.1.8-dev/include" });

    b.installArtifact(exe);

    // Create run step
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);

    // Create test suite
    const unit_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/main_simple.zig"),
        .target = target,
        .optimize = optimize,
    });

    unit_tests.linkLibC();
    unit_tests.linkSystemLibrary("LLVM-18");
    unit_tests.addLibraryPath(.{ .cwd_relative = "/nix/store/rxp13pg5iidpmvlvy963n8nkkbc246iz-llvm-18.1.8-lib/lib" });
    unit_tests.addIncludePath(.{ .cwd_relative = "/nix/store/19gmdqq62x11wv7ipni6grm5f8clcq7c-llvm-18.1.8-dev/include" });

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);

    // Create stdlib tests
    const stdlib_tests = b.addTest(.{
        .root_source_file = b.path("stdlib-zig/testz.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_stdlib_tests = b.addRunArtifact(stdlib_tests);
    const stdlib_test_step = b.step("test-stdlib", "Run stdlib tests");
    stdlib_test_step.dependOn(&run_stdlib_tests.step);
}
