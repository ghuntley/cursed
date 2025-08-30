const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // CURSED Compiler executable (full implementation)
    const cursed_exe = b.addExecutable(.{
        .name = "cursed-compiler",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/cursed_compiler_main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    cursed_exe.linkLibC();
    
    // Link against LLVM libraries
    cursed_exe.linkSystemLibrary("LLVM-18");
    cursed_exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib/" });
    cursed_exe.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-c-18/" });
    cursed_exe.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-18/" });

    // Install the executable
    b.installArtifact(cursed_exe);

    // Create a run step for the compiler
    const run_cmd = b.addRunArtifact(cursed_exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);

    // Test LLVM Pipeline executable
    const test_pipeline_exe = b.addExecutable(.{
        .name = "test-llvm-pipeline",
        .root_module = b.createModule(.{
            .root_source_file = b.path("test_llvm_pipeline.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    test_pipeline_exe.linkLibC();
    test_pipeline_exe.linkSystemLibrary("LLVM-18");
    test_pipeline_exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib/" });
    test_pipeline_exe.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-c-18/" });
    test_pipeline_exe.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-18/" });

    b.installArtifact(test_pipeline_exe);

    const test_pipeline_run_cmd = b.addRunArtifact(test_pipeline_exe);
    test_pipeline_run_cmd.step.dependOn(b.getInstallStep());

    const test_pipeline_step = b.step("test-pipeline", "Test LLVM IR Pipeline");
    test_pipeline_step.dependOn(&test_pipeline_run_cmd.step);

    // Simple compilation test
    const simple_test_exe = b.addExecutable(.{
        .name = "test-simple-compile",
        .root_module = b.createModule(.{
            .root_source_file = b.path("test_simple_compile.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    simple_test_exe.linkLibC();
    simple_test_exe.linkSystemLibrary("LLVM-18");
    simple_test_exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib/" });
    simple_test_exe.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-c-18/" });
    simple_test_exe.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-18/" });

    b.installArtifact(simple_test_exe);

    const simple_test_run_cmd = b.addRunArtifact(simple_test_exe);
    simple_test_run_cmd.step.dependOn(b.getInstallStep());

    const simple_test_step = b.step("test-simple", "Test simple CURSED compilation");
    simple_test_step.dependOn(&simple_test_run_cmd.step);

    // IR and Binary demo
    const demo_exe = b.addExecutable(.{
        .name = "demo-ir-and-binary",
        .root_module = b.createModule(.{
            .root_source_file = b.path("demo_ir_and_binary.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    demo_exe.linkLibC();
    demo_exe.linkSystemLibrary("LLVM-18");
    demo_exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib/" });
    demo_exe.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-c-18/" });
    demo_exe.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-18/" });

    b.installArtifact(demo_exe);

    const demo_run_cmd = b.addRunArtifact(demo_exe);
    demo_run_cmd.step.dependOn(b.getInstallStep());

    const demo_step = b.step("demo", "Demo IR generation and binary compilation");
    demo_step.dependOn(&demo_run_cmd.step);

    // Unit tests
    const unit_tests = b.addTest(.{
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/cursed_compiler_main.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    
    unit_tests.linkLibC();
    
    // Link against LLVM libraries for tests
    unit_tests.linkSystemLibrary("LLVM-18");
    unit_tests.addLibraryPath(.{ .cwd_relative = "/usr/lib/llvm-18/lib/" });
    unit_tests.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-c-18/" });
    unit_tests.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-18/" });

    const run_unit_tests = b.addRunArtifact(unit_tests);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);
}
