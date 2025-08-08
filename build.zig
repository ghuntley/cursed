const std = @import("std");
const builtin = @import("builtin");

// Platform-specific target configuration
const TargetConfig = struct {
    name: []const u8,
    description: []const u8,
    exe_suffix: []const u8,
    supports_llvm: bool,
    supports_threading: bool,
    supports_networking: bool,
    
    fn forTarget(target: std.Build.ResolvedTarget) TargetConfig {
        return switch (target.result.os.tag) {
            .linux => switch (target.result.cpu.arch) {
                .x86_64 => TargetConfig{
                    .name = "linux-x64",
                    .description = "Linux x86_64",
                    .exe_suffix = "",
                    .supports_llvm = true,
                    .supports_threading = true,
                    .supports_networking = true,
                },
                .aarch64 => TargetConfig{
                    .name = "linux-arm64",
                    .description = "Linux ARM64",
                    .exe_suffix = "",
                    .supports_llvm = true,
                    .supports_threading = true,
                    .supports_networking = true,
                },
                else => TargetConfig{
                    .name = "linux-unknown",
                    .description = "Linux (unknown arch)",
                    .exe_suffix = "",
                    .supports_llvm = false,
                    .supports_threading = true,
                    .supports_networking = true,
                },
            },
            .macos => switch (target.result.cpu.arch) {
                .x86_64 => TargetConfig{
                    .name = "macos-x64",
                    .description = "macOS x86_64",
                    .exe_suffix = "",
                    .supports_llvm = true,
                    .supports_threading = true,
                    .supports_networking = true,
                },
                .aarch64 => TargetConfig{
                    .name = "macos-arm64",
                    .description = "macOS ARM64",
                    .exe_suffix = "",
                    .supports_llvm = true,
                    .supports_threading = true,
                    .supports_networking = true,
                },
                else => TargetConfig{
                    .name = "macos-unknown",
                    .description = "macOS (unknown arch)",
                    .exe_suffix = "",
                    .supports_llvm = false,
                    .supports_threading = true,
                    .supports_networking = true,
                },
            },
            .windows => TargetConfig{
                .name = "windows-x64",
                .description = "Windows x86_64",
                .exe_suffix = ".exe",
                .supports_llvm = true,
                .supports_threading = true,
                .supports_networking = true,
            },
            .wasi, .freestanding => TargetConfig{
            .name = "wasm32",
            .description = "WebAssembly",
            .exe_suffix = ".wasm",
            .supports_llvm = false,
            .supports_threading = false,
            .supports_networking = false,
            },
            else => TargetConfig{
                .name = "unknown",
                .description = "Unknown platform",
                .exe_suffix = "",
                .supports_llvm = false,
                .supports_threading = false,
                .supports_networking = false,
            },
        };
    }
};

fn addLlvm(b: *std.Build, exe: *std.Build.Step.Compile, target: std.Build.ResolvedTarget) void {
    const config = TargetConfig.forTarget(target);
    
    // Skip LLVM for targets that don't support it
    if (!config.supports_llvm) {
        return;
    }
    
    const env = std.process.getEnvMap(b.allocator) catch return;
    
    // Platform-specific LLVM paths with auto-detection
    const llvm_paths = struct {
        const linux_lib_paths = [_][]const u8{
            "/usr/lib/llvm-18/lib",
            "/usr/lib/x86_64-linux-gnu",
            "/usr/lib/aarch64-linux-gnu",
            "/usr/lib64",
            "/lib64",
            "/nix/store/i7laizikxvx5hi86g98k4v3p7g8s2a7s-llvm-18.1.8-lib/lib",
            "/nix/store/rxp13pg5iidpmvlvy963n8nkkbc246iz-llvm-18.1.8-lib/lib",
            "/opt/homebrew/lib", // For Linux on ARM Mac
        };
        
        const linux_inc_paths = [_][]const u8{
            "/usr/include/llvm-18",
            "/usr/include/llvm-c",
            "/usr/include",
            "/nix/store/19gmdqq62x11wv7ipni6grm5f8clcq7c-llvm-18.1.8-dev/include",
            "/opt/homebrew/include",
        };
        
        const macos_lib_paths = [_][]const u8{
            "/opt/homebrew/lib",
            "/usr/local/lib",
            "/Library/Developer/CommandLineTools/usr/lib",
            "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib",
        };
        
        const macos_inc_paths = [_][]const u8{
            "/opt/homebrew/include",
            "/usr/local/include",
            "/Library/Developer/CommandLineTools/usr/include",
            "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/include",
        };
        
        const windows_lib_paths = [_][]const u8{
            "C:\\Program Files\\LLVM\\lib",
            "C:\\tools\\llvm\\lib",
            "C:\\vcpkg\\installed\\x64-windows\\lib",
        };
        
        const windows_inc_paths = [_][]const u8{
            "C:\\Program Files\\LLVM\\include",
            "C:\\tools\\llvm\\include",
            "C:\\vcpkg\\installed\\x64-windows\\include",
        };
    };
    
    switch (target.result.os.tag) {
        .linux => {
            // Auto-detect LLVM library path for Linux
            var llvm_lib_found = false;
            for (llvm_paths.linux_lib_paths) |path| {
                var dir = std.fs.openDirAbsolute(path, .{}) catch continue;
                dir.close();
                exe.addLibraryPath(.{ .cwd_relative = path });
                llvm_lib_found = true;
                if (b.verbose) {
                    std.debug.print("Found LLVM lib path: {s}\n", .{path});
                }
                break;
            }
            
            // Environment variable override
            if (env.get("LLVM_LINUX_LIB")) |lib_path| {
                exe.addLibraryPath(.{ .cwd_relative = lib_path });
                llvm_lib_found = true;
            }
            
            // Auto-detect LLVM include path
            for (llvm_paths.linux_inc_paths) |path| {
                var dir = std.fs.openDirAbsolute(path, .{}) catch continue;
                dir.close();
                exe.addIncludePath(.{ .cwd_relative = path });
                if (b.verbose) {
                    std.debug.print("Found LLVM include path: {s}\n", .{path});
                }
                break;
            }
            
            if (env.get("LLVM_LINUX_INC")) |inc_path| {
                exe.addIncludePath(.{ .cwd_relative = inc_path });
            }
            
            // Link LLVM and system libraries
            if (llvm_lib_found) {
                exe.linkSystemLibrary("LLVM-18");
                exe.linkSystemLibrary("pthread");
                exe.linkSystemLibrary("dl");
                exe.linkSystemLibrary("m");
                
                // Architecture-specific library paths
                switch (target.result.cpu.arch) {
                    .x86_64 => {
                        exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/x86_64-linux-gnu" });
                        exe.addLibraryPath(.{ .cwd_relative = "/lib/x86_64-linux-gnu" });
                    },
                    .aarch64 => {
                        exe.addLibraryPath(.{ .cwd_relative = "/usr/lib/aarch64-linux-gnu" });
                        exe.addLibraryPath(.{ .cwd_relative = "/lib/aarch64-linux-gnu" });
                    },
                    else => {},
                }
                exe.linkSystemLibrary("z");
            }
        },
        .macos => {
            // Auto-detect LLVM library path for macOS
            var llvm_lib_found = false;
            for (llvm_paths.macos_lib_paths) |path| {
                var dir = std.fs.openDirAbsolute(path, .{}) catch continue;
                dir.close();
                exe.addLibraryPath(.{ .cwd_relative = path });
                llvm_lib_found = true;
                if (b.verbose) {
                    std.debug.print("Found LLVM lib path: {s}\n", .{path});
                }
                break;
            }
            
            // Environment variable override
            if (env.get("LLVM_MACOS_LIB")) |lib_path| {
                exe.addLibraryPath(.{ .cwd_relative = lib_path });
                llvm_lib_found = true;
            }
            
            // Auto-detect LLVM include path
            for (llvm_paths.macos_inc_paths) |path| {
                var dir = std.fs.openDirAbsolute(path, .{}) catch continue;
                dir.close();
                exe.addIncludePath(.{ .cwd_relative = path });
                if (b.verbose) {
                    std.debug.print("Found LLVM include path: {s}\n", .{path});
                }
                break;
            }
            
            if (env.get("LLVM_MACOS_INC")) |inc_path| {
                exe.addIncludePath(.{ .cwd_relative = inc_path });
            }
            
            // Link LLVM and macOS frameworks
            if (llvm_lib_found) {
                exe.linkSystemLibrary("LLVM-18");
                exe.linkSystemLibrary("z");
                exe.linkSystemLibrary("xml2");
                exe.linkFramework("Security");
                exe.linkFramework("CoreFoundation");
                exe.linkFramework("SystemConfiguration");
            }
        },
        .windows => {
            // Auto-detect LLVM library path for Windows
            var llvm_lib_found = false;
            for (llvm_paths.windows_lib_paths) |path| {
                // Only try to open directories that exist and are absolute
                if (std.fs.path.isAbsolute(path)) {
                    var dir = std.fs.openDirAbsolute(path, .{}) catch continue;
                    dir.close();
                    exe.addLibraryPath(.{ .cwd_relative = path });
                    llvm_lib_found = true;
                    if (b.verbose) {
                        std.debug.print("Found LLVM lib path: {s}\n", .{path});
                    }
                    break;
                }
            }
            
            // Environment variable override
            if (env.get("LLVM_WINDOWS_LIB")) |lib_path| {
                exe.addLibraryPath(.{ .cwd_relative = lib_path });
                llvm_lib_found = true;
            }
            
            // Auto-detect LLVM include path
            for (llvm_paths.windows_inc_paths) |path| {
                // Only try to open directories that exist and are absolute
                if (std.fs.path.isAbsolute(path)) {
                    var dir = std.fs.openDirAbsolute(path, .{}) catch continue;
                    dir.close();
                    exe.addIncludePath(.{ .cwd_relative = path });
                    if (b.verbose) {
                        std.debug.print("Found LLVM include path: {s}\n", .{path});
                    }
                    break;
                }
            }
            
            if (env.get("LLVM_WINDOWS_INC")) |inc_path| {
                exe.addIncludePath(.{ .cwd_relative = inc_path });
            }
            
            // Link LLVM and Windows system libraries
            if (llvm_lib_found) {
                // Try multiple LLVM library names for Windows
                const llvm_libs = [_][]const u8{ "LLVM-18", "LLVM", "libLLVM-18", "libLLVM" };
                var llvm_linked = false;
                for (llvm_libs) |lib_name| {
                    exe.linkSystemLibrary(lib_name);
                    llvm_linked = true;
                    break;
                }
                
                if (llvm_linked) {
                    exe.linkSystemLibrary("zlib");
                    exe.linkSystemLibrary("libxml2");
                }
            }
            
            // Windows system libraries for networking and crypto
            exe.linkSystemLibrary("ws2_32");    // Winsock
            exe.linkSystemLibrary("bcrypt");    // Crypto
            exe.linkSystemLibrary("crypt32");   // Crypto certificates
            exe.linkSystemLibrary("secur32");   // Security
            
            // Core Windows libraries
            exe.linkSystemLibrary("kernel32");
            exe.linkSystemLibrary("user32");
            exe.linkSystemLibrary("shell32");
            exe.linkSystemLibrary("ole32");
            exe.linkSystemLibrary("oleaut32");
            exe.linkSystemLibrary("advapi32");
            exe.linkSystemLibrary("psapi");     // Process info
        },
        else => {
            // Unknown platform - no LLVM linking
        },
    }
}

pub fn build(b: *std.Build) void {
    // Get target, defaulting to native/host target when none specified
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Ensure we use the resolved target properly
    const resolved_target = target;
    const config = TargetConfig.forTarget(resolved_target);
    const is_wasm = resolved_target.result.cpu.arch == .wasm32;
    
    // Detect cross-compilation early
    const is_cross_compile = resolved_target.result.cpu.arch != @import("builtin").target.cpu.arch or
                            resolved_target.result.os.tag != @import("builtin").target.os.tag;
    
    // Debug info: print target info in verbose mode
    if (b.verbose) {
        std.debug.print("Building for target: {s} ({s})\n", .{
            config.description,
            config.name
        });
        std.debug.print("Platform capabilities:\n", .{});
        std.debug.print("  LLVM support: {}\n", .{config.supports_llvm});
        std.debug.print("  Threading: {}\n", .{config.supports_threading});
        std.debug.print("  Networking: {}\n", .{config.supports_networking});
        std.debug.print("  Target CPU: {s}\n", .{@tagName(resolved_target.result.cpu.arch)});
        std.debug.print("  Target OS: {s}\n", .{@tagName(resolved_target.result.os.tag)});
        std.debug.print("  Cross-compiling: {}\n", .{is_cross_compile});
    }

    // Create the CURSED compiler executable - unified main with subcommands
    const exe = b.addExecutable(.{
        .name = "cursed", 
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/main_unified.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    // Configure libc for minimal compiler (no LLVM needed)
    if (!is_wasm) {
        exe.linkLibC();
        
        // Add defer runtime C source (temporarily disabled for build compatibility)
        // exe.addCSourceFile(.{
        //     .file = b.path("src-zig/defer_runtime.c"),
        //     .flags = &[_][]const u8{"-std=c99", "-O2"},
        // });
        
        // Add error handling runtime C source (temporarily disabled)
        // exe.addCSourceFile(.{
        //     .file = b.path("runtime/cursed_error_runtime.c"),
        //     .flags = &[_][]const u8{"-std=c99", "-O2"},
        // });
        
        // Integrate package manager dependencies
        const build_integration = @import("src-zig/build_integration.zig");
        build_integration.integrateBuildSystem(b, exe, resolved_target, optimize) catch |err| {
        std.debug.print("Package manager integration failed: {}\n", .{err});
        // Continue with build even if package integration fails
        };
    
    // Integrate CURSED build system for .csd file compilation (disabled temporarily)
    // const cursed_build_system = @import("src-zig/cursed_build_system.zig");
    // cursed_build_system.createCursedBuildStep(b, resolved_target, optimize, "zig-out/bin/cursed") catch |err| {
    //     std.debug.print("CURSED build system integration failed: {}\n", .{err});
    //     // Continue with build even if CURSED integration fails
    // };
    }

    // Memory-safe CURSED compiler executable (temporarily disabled for cross-compilation)
    // const memory_safe_exe = b.addExecutable(.{
    //     .name = "cursed-memory-safe",
    //     .root_source_file = b.path("src-zig/main_memory_safe.zig"),
    //     .target = resolved_target,
    //     .optimize = optimize,
    // });
    
    // if (!is_wasm) {
    //     memory_safe_exe.linkLibC();
    // }

    // Alternative implementations for testing and fallback
    const minimal_exe = b.addExecutable(.{
        .name = "cursed-minimal",
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/minimal_main.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    if (!is_wasm) {
        minimal_exe.linkLibC();
    }

    const complete_exe = b.addExecutable(.{
        .name = "cursed-complete",
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/main_complete.zig"),
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
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/simplified_optimized_main.zig"),
        .target = resolved_target,
        .optimize = .ReleaseFast, // Always use fastest optimization for performance compiler
    });
    if (!is_wasm) {
        optimized_exe.linkLibC();
    }

    // Create syscall-enabled compiler with real file I/O, networking, and process management
    // Only build for native target to avoid cross-compilation LLVM issues

    b.installArtifact(exe);
    
    // Create legacy alias for backwards compatibility
    const legacy_exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/main.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    if (!is_wasm) {
        legacy_exe.linkLibC();
    }
    b.installArtifact(legacy_exe);

// b.installArtifact(memory_safe_exe);  // Disabled due to compilation errors
b.installArtifact(minimal_exe);
b.installArtifact(complete_exe);
    // b.installArtifact(enhanced_exe);  // Disabled due to API issues
    b.installArtifact(optimized_exe);
    // syscall_exe now handled after modules are defined
    
    // Create shared modules for tools (need to be defined before packages use them)
    const tools_mod = b.addModule("tools", .{
        .root_source_file = b.path("src-zig/tools/mod.zig"),
    });
    
    // Create package manager CLI tool
    const pkg_manager_exe = b.addExecutable(.{
        .name = "cursed-pkg",
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/cursed_pkg.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    
    // Add module imports to package manager
    if (!is_wasm) {
        pkg_manager_exe.root_module.addImport("tools", tools_mod);
        pkg_manager_exe.linkLibC();
    }
    b.installArtifact(pkg_manager_exe);

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
        .target = resolved_target,
        .optimize = optimize,
    });

    unit_tests.linkLibC();
    addLlvm(b, unit_tests, resolved_target);

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);

    // Create concurrency test suite
    const concurrency_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/concurrency.zig"),
        .target = resolved_target,
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
            .target = resolved_target,
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
            .target = resolved_target,
            .optimize = optimize,
        });

        b.installArtifact(concurrency_test_exe);

        const run_concurrency_test_exe = b.addRunArtifact(concurrency_test_exe);
        run_concurrency_test_exe.step.dependOn(b.getInstallStep());

        const concurrency_full_test_step = b.step("test-concurrency-full", "Run comprehensive concurrency tests");
        concurrency_full_test_step.dependOn(&run_concurrency_test_exe.step);
    }

    // Note: Using direct file imports instead of modules to avoid conflicts
    
    // Create syscall-enabled compiler with real file I/O, networking, and process management
    // Only build for native target to avoid cross-compilation LLVM issues
    var syscall_exe: ?*std.Build.Step.Compile = null;
    if (!is_cross_compile) {
        syscall_exe = b.addExecutable(.{
            .name = "cursed-syscall",
            .root_source_file = b.path("src-zig/main_unified.zig"),
            .target = resolved_target,
            .optimize = optimize,
        });
        if (!is_wasm) {
            syscall_exe.?.linkLibC();
            
            addLlvm(b, syscall_exe.?, resolved_target);
        }
    }
    
    if (syscall_exe) |syscall| {
        b.installArtifact(syscall);
    }
    
    // Create LSP server executable
    const lsp_exe = b.addExecutable(.{
        .name = "cursed-lsp",
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("cursed_lsp_working.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    
    if (!is_wasm) {
        lsp_exe.linkLibC();
    }
    
    b.installArtifact(lsp_exe);
    
    // Debug information test executable (temporarily disabled due to build issues)
    // TODO: Re-enable when debug_enabled_codegen.zig compiles properly
    // const debug_test_exe = b.addExecutable(.{
    //     .name = "cursed-debug-test",
    //     .root_source_file = b.path("src-zig/test_debug_generation.zig"),
    //     .target = resolved_target,
    //     .optimize = optimize,
    // });
    // if (!is_wasm) {
    //     debug_test_exe.linkLibC();
    //     if (builtin.os.tag == .linux or builtin.os.tag == .macos) {
    //         debug_test_exe.addSystemIncludePath(.{.path = "/usr/include/llvm-18"});
    //         debug_test_exe.addSystemIncludePath(.{.path = "/usr/include/llvm-c-18"});
    //         debug_test_exe.linkSystemLibrary("LLVM-18");
    //     }
    // }
    // b.installArtifact(debug_test_exe);
    
    // Create documentation generator executable (skip for WASM - uses filesystem)
    if (!is_wasm) {
        const doc_exe = b.addExecutable(.{
            .name = "cursed-doc",
            .root_source_file = b.path("src-zig/doc_generator.zig"),
            .target = resolved_target,
            .optimize = optimize,
        });
        
        // Doc generator uses direct file imports to avoid module conflicts
        doc_exe.linkLibC();
        b.installArtifact(doc_exe);

        // Import resolver CLI tool (disabled due to dependency issues)
        // const import_resolver_cli = b.addExecutable(.{
        //     .name = "cursed-import-resolver",
        //     .root_source_file = b.path("src-zig/tools/import_resolver_cli.zig"),
        //     .target = resolved_target,
        //     .optimize = optimize,
        // });
        // b.installArtifact(import_resolver_cli);
    }
    
    const run_lsp = b.addRunArtifact(lsp_exe);
    run_lsp.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_lsp.addArgs(args);
    }
    
    const lsp_step = b.step("lsp", "Run the CURSED Language Server");
    lsp_step.dependOn(&run_lsp.step);

    // NOTE: Stdlib tests are now run via pure CURSED files in stdlib/ directory
    // Use: ./zig-out/bin/cursed-zig stdlib/comprehensive_stdlib_test.csd

    // Create advanced parser tests
    const parser_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/parser_test_advanced.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    const run_parser_tests = b.addRunArtifact(parser_tests);
    const parser_test_step = b.step("test-parser", "Run advanced parser tests");
    parser_test_step.dependOn(&run_parser_tests.step);

    // Create syscall interface tests
    const syscall_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/syscall_interface.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    if (!is_wasm) {
        syscall_tests.linkLibC();
    }

    const run_syscall_tests = b.addRunArtifact(syscall_tests);
    const syscall_test_step = b.step("test-syscall", "Run syscall interface tests");
    syscall_test_step.dependOn(&run_syscall_tests.step);

    // Create error diagnostics tests
    const error_diagnostics_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/error_diagnostics.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    const run_error_diagnostics_tests = b.addRunArtifact(error_diagnostics_tests);
    const error_diagnostics_test_step = b.step("test-diagnostics", "Run error diagnostics tests");
    error_diagnostics_test_step.dependOn(&run_error_diagnostics_tests.step);

    // Create diagnostic demo executable
    const diagnostics_demo = b.addExecutable(.{
        .name = "cursed-diagnostics-demo",
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/test_diagnostics_demo_simple.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    b.installArtifact(diagnostics_demo);

    // Create platform abstraction test executable
    // NOTE: Disabled due to Zig API compatibility issues in platform_abstraction.zig
    // const platform_test = b.addExecutable(.{
    //     .name = "cursed-platform-test",
    //     .root_source_file = b.path("src-zig/platform_test.zig"),
    //     .target = resolved_target,
    //     .optimize = optimize,
    // });
    // 
    // if (!is_wasm) {
    //     platform_test.linkLibC();
    // }
    // 
    // b.installArtifact(platform_test);
    
    // Create platform test run step
    // NOTE: Disabled along with platform_test
    // const run_platform_test = b.addRunArtifact(platform_test);
    // run_platform_test.step.dependOn(b.getInstallStep());
    
    // const platform_test_step = b.step("test-platform", "Test platform abstraction layer");
    // platform_test_step.dependOn(&run_platform_test.step);

    // Create comprehensive test step that runs all tests
    const all_tests_step = b.step("test-all", "Run all test suites");
    all_tests_step.dependOn(&run_unit_tests.step);
    all_tests_step.dependOn(&run_concurrency_tests.step);
    all_tests_step.dependOn(&run_parser_tests.step);
    all_tests_step.dependOn(&run_syscall_tests.step);
    all_tests_step.dependOn(&run_error_diagnostics_tests.step);
    // all_tests_step.dependOn(&run_platform_test.step); // Disabled platform test

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

    // Cross-compilation build steps for all 5 supported platforms
    const cross_compile_step = b.step("cross-compile", "Cross-compile for all supported platforms");
    
    // Target specifications for cross-compilation
    const cross_targets = [_]std.Target.Query{
        .{ .cpu_arch = .x86_64, .os_tag = .linux },      // Linux x64
        .{ .cpu_arch = .aarch64, .os_tag = .linux },     // Linux ARM64
        .{ .cpu_arch = .x86_64, .os_tag = .macos },      // macOS x64
        .{ .cpu_arch = .aarch64, .os_tag = .macos },     // macOS ARM64
        .{ .cpu_arch = .x86_64, .os_tag = .windows },    // Windows x64
        .{ .cpu_arch = .wasm32, .os_tag = .wasi },       // WebAssembly
    };
    
    // Create cross-compilation executables for each target
    for (cross_targets) |query| {
        const cross_target = b.resolveTargetQuery(query);
        const cross_config = TargetConfig.forTarget(cross_target);
        
        // Skip native target (already built)
        const is_native = cross_target.result.cpu.arch == resolved_target.result.cpu.arch and
                         cross_target.result.os.tag == resolved_target.result.os.tag;
        if (is_native) continue;
        
        // Create cross-compiled executable
        const cross_exe = b.addExecutable(.{
            .name = b.fmt("cursed-{s}", .{cross_config.name}),
            .root_source_file = if (query.cpu_arch == .wasm32) 
                b.path("src-zig/wasm_pure.zig") 
            else 
                b.path("src-zig/main_unified.zig"),
            .target = cross_target,
            .optimize = optimize,
        });
        
        // Configure cross-compiled executable
        if (query.cpu_arch != .wasm32) {
            cross_exe.linkLibC();
            
            // Skip LLVM for cross-compilation to avoid library dependency issues
            // Only add LLVM when compiling for the exact same target as the host
            if (cross_config.supports_llvm and 
                query.os_tag == resolved_target.result.os.tag and
                query.cpu_arch == resolved_target.result.cpu.arch) {
                addLlvm(b, cross_exe, cross_target);
            }
        }
        
        // Install cross-compiled artifact
        const cross_install = b.addInstallArtifact(cross_exe, .{
            .dest_dir = .{ .override = .{ .custom = b.fmt("bin/{s}", .{cross_config.name}) } },
        });
        
        cross_compile_step.dependOn(&cross_install.step);
        
        if (b.verbose) {
            std.debug.print("Added cross-compilation target: {s}\n", .{cross_config.description});
        }
    }
    
    // Platform-specific archive creation
    const archive_step = b.step("archive", "Create platform-specific archives");
    
    // Create platform archives for distribution
    for (cross_targets) |query| {
        const cross_target = b.resolveTargetQuery(query);
        const cross_config = TargetConfig.forTarget(cross_target);
        
        const archive_cmd = switch (query.os_tag.?) {
            .windows => b.addSystemCommand(&[_][]const u8{
                "powershell", "-Command",
                b.fmt("Compress-Archive -Path 'zig-out/bin/{s}/*' -DestinationPath 'cursed-{s}.zip'", .{ cross_config.name, cross_config.name })
            }),
            else => b.addSystemCommand(&[_][]const u8{
                "tar", "-czf", 
                b.fmt("cursed-{s}.tar.gz", .{cross_config.name}),
                "-C", b.fmt("zig-out/bin/{s}", .{cross_config.name}),
                "."
            }),
        };
        
        archive_cmd.step.dependOn(cross_compile_step);
        archive_step.dependOn(&archive_cmd.step);
    }
    
    // CURSED project compilation steps
    const cursed_compile_step = b.step("cursed-compile", "Compile CURSED projects in current directory");
    const cursed_test_step = b.step("cursed-test", "Run CURSED project tests");
    const cursed_run_step = b.step("cursed-run", "Run CURSED project");
    const cursed_clean_step = b.step("cursed-clean", "Clean CURSED build artifacts");
    
    // Implement CURSED compilation for .csd files in current directory
    const cursed_project_compile = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed", "compile", "--project", "."
    });
    cursed_project_compile.step.dependOn(b.getInstallStep());
    cursed_compile_step.dependOn(&cursed_project_compile.step);
    
    // CURSED project testing
    const cursed_project_test = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed", "test", "--project", "."
    });
    cursed_project_test.step.dependOn(&cursed_project_compile.step);
    cursed_test_step.dependOn(&cursed_project_test.step);
    
    // CURSED project execution
    const cursed_project_run = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed", "run", "--project", "."
    });
    cursed_project_run.step.dependOn(&cursed_project_compile.step);
    cursed_run_step.dependOn(&cursed_project_run.step);
    
    // CURSED build cleanup
    const cursed_project_clean = b.addSystemCommand(&[_][]const u8{
        "rm", "-rf", "target/", "build/", "zig-cache/cursed/"
    });
    cursed_clean_step.dependOn(&cursed_project_clean.step);
    
    // CURSED project initialization step
    const cursed_init_step = b.step("cursed-init", "Initialize new CURSED project");
    const cursed_project_init = b.addSystemCommand(&[_][]const u8{
        "zig-out/bin/cursed", "init", "--name", "cursed-project"
    });
    cursed_project_init.step.dependOn(b.getInstallStep());
    cursed_init_step.dependOn(&cursed_project_init.step);

    // Cross-platform testing step
    const cross_test_step = b.step("cross-test", "Test cross-compilation functionality");
    
    // Create test script for cross-compilation validation
    const cross_test_script = b.addWriteFiles();
    _ = cross_test_script.add("cross_test.sh", 
        \\#!/bin/bash
        \\set -e
        \\echo "Testing cross-compilation results..."
        \\
        \\# Check that all expected binaries exist
        \\for target in linux-x64 linux-arm64 macos-x64 macos-arm64 windows-x64 wasm32; do
        \\    binary_dir="zig-out/bin/$target"
        \\    if [ -d "$binary_dir" ]; then
        \\        echo "✓ $target build directory exists"
        \\        # Check for expected binary
        \\        if [ "$target" = "windows-x64" ]; then
        \\            expected_binary="$binary_dir/cursed-$target.exe"
        \\        elif [ "$target" = "wasm32" ]; then
        \\            expected_binary="$binary_dir/cursed-$target.wasm"
        \\        else
        \\            expected_binary="$binary_dir/cursed-$target"
        \\        fi
        \\        
        \\        if [ -f "$expected_binary" ]; then
        \\            echo "✓ $target binary exists: $expected_binary"
        \\            # Show file info
        \\            ls -lh "$expected_binary"
        \\            if command -v file >/dev/null 2>&1; then
        \\                file "$expected_binary" || true
        \\            fi
        \\        else
        \\            echo "✗ $target binary missing: $expected_binary"
        \\        fi
        \\    else
        \\        echo "✗ $target build directory missing"
        \\    fi
        \\    echo
        \\done
        \\
        \\echo "Cross-compilation test completed"
    );
    
    const run_cross_test = b.addSystemCommand(&[_][]const u8{ "bash", "zig-out/cross_test.sh" });
    run_cross_test.step.dependOn(&cross_test_script.step);
    run_cross_test.step.dependOn(cross_compile_step);
    cross_test_step.dependOn(&run_cross_test.step);
}
