const std = @import("std");

fn addLlvm(b: *std.Build, exe: *std.Build.Step.Compile, target: std.Build.ResolvedTarget) void {
    const env = std.process.getEnvMap(b.allocator) catch return;
    
    // Try to auto-detect LLVM paths on NixOS
    const potential_llvm_lib_paths = [_][]const u8{
        "/nix/store/i7laizikxvx5hi86g98k4v3p7g8s2a7s-llvm-18.1.8-lib/lib",
        "/nix/store/rxp13pg5iidpmvlvy963n8nkkbc246iz-llvm-18.1.8-lib/lib", // Fallback from original config
        "/usr/lib/llvm-18/lib",
        "/usr/lib64",
        "/lib64",
    };
    
    const potential_llvm_inc_paths = [_][]const u8{
        "/nix/store/19gmdqq62x11wv7ipni6grm5f8clcq7c-llvm-18.1.8-dev/include",
        "/usr/include/llvm-18",
        "/usr/include",
    };
    
    switch (target.result.os.tag) {
        .linux => {
            // Auto-detect LLVM library path
            var llvm_lib_found = false;
            for (potential_llvm_lib_paths) |path| {
                var dir = std.fs.openDirAbsolute(path, .{}) catch continue;
                dir.close();
                exe.addLibraryPath(.{ .cwd_relative = path });
                llvm_lib_found = true;
                break;
            }
            
            // Use environment variable override if provided
            if (env.get("LLVM_LINUX_LIB")) |lib_path| {
                exe.addLibraryPath(.{ .cwd_relative = lib_path });
                llvm_lib_found = true;
            }
            
            // Auto-detect LLVM include path
            for (potential_llvm_inc_paths) |path| {
                var dir = std.fs.openDirAbsolute(path, .{}) catch continue;
                dir.close();
                exe.addIncludePath(.{ .cwd_relative = path });
                break;
            }
            
            if (env.get("LLVM_LINUX_INC")) |inc_path| {
                exe.addIncludePath(.{ .cwd_relative = inc_path });
            }
            
            // Only link LLVM if we found the library path
            if (llvm_lib_found) {
                exe.linkSystemLibrary("LLVM-18");
            }
        },
        .windows => {
            exe.linkSystemLibrary("LLVM-18");
            if (env.get("LLVM_WINDOWS_LIB")) |lib_path| {
                exe.addLibraryPath(.{ .cwd_relative = lib_path });
            }
            if (env.get("LLVM_WINDOWS_INC")) |inc_path| {
                exe.addIncludePath(.{ .cwd_relative = inc_path });
            }
            exe.linkSystemLibrary("z");
            exe.linkSystemLibrary("xml2");
            exe.linkSystemLibrary("psapi");
            exe.linkSystemLibrary("bcrypt");
        },
        .macos => {
            exe.linkSystemLibrary("LLVM-18");
            if (env.get("LLVM_MACOS_LIB")) |lib_path| {
                exe.addLibraryPath(.{ .cwd_relative = lib_path });
            }
            if (env.get("LLVM_MACOS_INC")) |inc_path| {
                exe.addIncludePath(.{ .cwd_relative = inc_path });
            }
            exe.linkFramework("Security");
            exe.linkFramework("CoreFoundation");
            exe.linkSystemLibrary("z");
            exe.linkSystemLibrary("xml2");
        },
        else => {},
    }
}

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Cross-compilation configuration
    const resolved_target = target;
    const is_wasm = target.result.cpu.arch == .wasm32;
    _ = target.result.os.tag; // For potential future Windows-specific logic

    // Create the CURSED compiler executable - use minimal working version
    const exe = b.addExecutable(.{
        .name = "cursed-zig", 
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_main.zig") else b.path("src-zig/main_concurrency_minimal.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    // Configure libc for minimal compiler (no LLVM needed)
    if (!is_wasm) {
        exe.linkLibC();
    }

    // Memory-safe CURSED compiler executable
    const memory_safe_exe = b.addExecutable(.{
        .name = "cursed-memory-safe",
        .root_source_file = b.path("src-zig/main_memory_safe.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    
    if (!is_wasm) {
        memory_safe_exe.linkLibC();
    }

    // Alternative implementations for testing and fallback
    const minimal_exe = b.addExecutable(.{
        .name = "cursed-minimal",
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_main.zig") else b.path("src-zig/minimal_main.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    if (!is_wasm) {
        minimal_exe.linkLibC();
    }

    const complete_exe = b.addExecutable(.{
        .name = "cursed-complete",
        .root_source_file = b.path("src-zig/main_complete.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    if (!is_wasm) {
        complete_exe.linkLibC();
    }

    // Enhanced compiler with improved error reporting and debugging (disabled due to API issues)
    // const enhanced_exe = b.addExecutable(.{
    //     .name = "cursed-enhanced",
    //     .root_source_file = b.path("src-zig/enhanced_main.zig"),
    //     .target = resolved_target,
    //     .optimize = optimize,
    // });
    // enhanced_exe.linkLibC();

    // Create performance-optimized compiler
    const optimized_exe = b.addExecutable(.{
        .name = "cursed-optimized",
        .root_source_file = b.path("src-zig/simplified_optimized_main.zig"),
        .target = resolved_target,
        .optimize = .ReleaseFast, // Always use fastest optimization for performance compiler
    });
    if (!is_wasm) {
        optimized_exe.linkLibC();
    }

    // Create syscall-enabled compiler with real file I/O, networking, and process management
    const syscall_exe = b.addExecutable(.{
        .name = "cursed-syscall",
        .root_source_file = b.path("src-zig/main_unified.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    if (!is_wasm) {
        syscall_exe.linkLibC();
        
        addLlvm(b, syscall_exe, resolved_target);
    }

    b.installArtifact(exe);
b.installArtifact(memory_safe_exe);
b.installArtifact(minimal_exe);
b.installArtifact(complete_exe);
    // b.installArtifact(enhanced_exe);  // Disabled due to API issues
    b.installArtifact(optimized_exe);
    b.installArtifact(syscall_exe);

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
        .root_source_file = b.path("src-zig/main_unified.zig"),
        .target = target,
        .optimize = optimize,
    });

    unit_tests.linkLibC();
    addLlvm(b, unit_tests, target);

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);

    // Create concurrency test suite
    const concurrency_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/concurrency.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_concurrency_tests = b.addRunArtifact(concurrency_tests);
    const concurrency_test_step = b.step("test-concurrency", "Run concurrency tests");
    concurrency_test_step.dependOn(&run_concurrency_tests.step);

    // Create concurrency benchmark executable (skip for WASM - no threading support)
    if (!is_wasm) {
        const concurrency_benchmark = b.addExecutable(.{
            .name = "cursed-concurrency-benchmark",
            .root_source_file = b.path("src-zig/concurrency_benchmark.zig"),
            .target = target,
            .optimize = optimize,
        });

        b.installArtifact(concurrency_benchmark);

        const run_benchmark = b.addRunArtifact(concurrency_benchmark);
        run_benchmark.step.dependOn(b.getInstallStep());

        const benchmark_step = b.step("benchmark", "Run concurrency benchmarks");
        benchmark_step.dependOn(&run_benchmark.step);

        // Create comprehensive concurrency test executable
        const concurrency_test_exe = b.addExecutable(.{
            .name = "cursed-concurrency-test",
            .root_source_file = b.path("src-zig/concurrency_test.zig"),
            .target = target,
            .optimize = optimize,
        });

        b.installArtifact(concurrency_test_exe);

        const run_concurrency_test_exe = b.addRunArtifact(concurrency_test_exe);
        run_concurrency_test_exe.step.dependOn(b.getInstallStep());

        const concurrency_full_test_step = b.step("test-concurrency-full", "Run comprehensive concurrency tests");
        concurrency_full_test_step.dependOn(&run_concurrency_test_exe.step);
    }

    // NOTE: Stdlib tests are now run via pure CURSED files in stdlib/ directory
    // Use: ./zig-out/bin/cursed-zig stdlib/comprehensive_stdlib_test.csd

    // Create advanced parser tests
    const parser_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/parser_test_advanced.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_parser_tests = b.addRunArtifact(parser_tests);
    const parser_test_step = b.step("test-parser", "Run advanced parser tests");
    parser_test_step.dependOn(&run_parser_tests.step);

    // Create syscall interface tests
    const syscall_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/syscall_interface.zig"),
        .target = target,
        .optimize = optimize,
    });

    if (!is_wasm) {
        syscall_tests.linkLibC();
    }

    const run_syscall_tests = b.addRunArtifact(syscall_tests);
    const syscall_test_step = b.step("test-syscall", "Run syscall interface tests");
    syscall_test_step.dependOn(&run_syscall_tests.step);

    // Create comprehensive test step that runs all tests
    const all_tests_step = b.step("test-all", "Run all test suites");
    all_tests_step.dependOn(&run_unit_tests.step);
    all_tests_step.dependOn(&run_concurrency_tests.step);
    all_tests_step.dependOn(&run_parser_tests.step);
    all_tests_step.dependOn(&run_syscall_tests.step);

    // Self-hosting compilation targets
    const selfhost_stage2_step = b.step("selfhost-stage2", "Compile Stage 2 CURSED compiler using Zig compiler");
    const selfhost_stage3_step = b.step("selfhost-stage3", "Compile Stage 3 CURSED compiler using Stage 2");
    const selfhost_step = b.step("selfhost", "Complete self-hosting bootstrap pipeline");

    // Stage 2: Run CURSED compiler in interpretation mode (compilation mode not yet implemented)
    const stage2_run = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed-zig", "src/bootstrap/stage2/main.csd"
    });
    stage2_run.step.dependOn(b.getInstallStep());
    selfhost_stage2_step.dependOn(&stage2_run.step);

    // Stage 3: Compile CURSED compiler using Stage 2 compiler
    const stage3_run = b.addSystemCommand(&[_][]const u8{
        "./cursed-stage2", "--compile", "src/bootstrap/stage2/main.csd", "-o", "cursed-stage3"
    });
    stage3_run.step.dependOn(&stage2_run.step);
    selfhost_stage3_step.dependOn(&stage3_run.step);

    // Complete self-hosting pipeline
    const bootstrap_validation_run = b.addSystemCommand(&[_][]const u8{
        "./bootstrap_complete.sh"
    });
    bootstrap_validation_run.step.dependOn(&stage3_run.step);
    selfhost_step.dependOn(&bootstrap_validation_run.step);

    // Self-hosting validation test
    const selfhost_test_step = b.step("selfhost-test", "Test self-hosting compilation pipeline");
    const test_stage2_run = b.addSystemCommand(&[_][]const u8{
        "./cursed-stage2", "src/bootstrap/stage2/test_simple.csd"
    });
    test_stage2_run.step.dependOn(&stage2_run.step);
    selfhost_test_step.dependOn(&test_stage2_run.step);
}
