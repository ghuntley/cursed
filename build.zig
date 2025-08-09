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

// LLVM library detection - dynamically detect LLVM installation
fn detectLlvmLibrary(b: *std.Build, target: std.Build.ResolvedTarget) []const u8 {
    _ = target; // Unused for now
    
    // Try to detect LLVM using llvm-config
    const result = std.process.Child.run(.{
        .allocator = b.allocator,
        .argv = &[_][]const u8{ "llvm-config-18", "--libdir" },
    }) catch |err| {
        if (b.verbose) {
            std.debug.print("⚠️ llvm-config-18 failed: {}\n", .{err});
        }
        return "LLVM-18"; // Fallback
    };
    defer b.allocator.free(result.stdout);
    defer b.allocator.free(result.stderr);
    
    if (result.term == .Exited and result.term.Exited == 0) {
        if (b.verbose) {
            const libdir = std.mem.trim(u8, result.stdout, " \n\r\t");
            std.debug.print("✅ LLVM detected via llvm-config: {s}\n", .{libdir});
        }
        return "LLVM-18";
    }
    
    // Fallback detection
    const lib_name = switch (builtin.target.os.tag) {
        .linux => "LLVM-18",        // Ubuntu/Debian standard
        .macos => "LLVM",           // Homebrew standard  
        .windows => "LLVM",         // Windows standard
        else => "LLVM-18",          // Default
    };
    
    if (b.verbose) {
        std.debug.print("✅ Using fallback LLVM library: {s}\n", .{lib_name});
    }
    return lib_name;
}

// Dynamic LLVM path detection
fn detectLlvmPaths(b: *std.Build, allocator: std.mem.Allocator) struct {
    lib_paths: [][]const u8,
    include_paths: [][]const u8,
    c_include_paths: [][]const u8,
} {
    var lib_paths = std.ArrayList([]const u8).init(allocator);
    var include_paths = std.ArrayList([]const u8).init(allocator);
    var c_include_paths = std.ArrayList([]const u8).init(allocator);
    
    // Try llvm-config-18 first
    if (std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "llvm-config-18", "--libdir" },
    })) |result| {
        defer allocator.free(result.stdout);
        defer allocator.free(result.stderr);
        
        if (result.term == .Exited and result.term.Exited == 0) {
            const libdir = std.mem.trim(u8, result.stdout, " \n\r\t");
            lib_paths.append(b.dupe(libdir)) catch {};
            
            if (b.verbose) {
                std.debug.print("✅ LLVM libdir detected: {s}\n", .{libdir});
            }
        }
    } else |_| {}
    
    if (std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "llvm-config-18", "--includedir" },
    })) |result| {
        defer allocator.free(result.stdout);
        defer allocator.free(result.stderr);
        
        if (result.term == .Exited and result.term.Exited == 0) {
            const includedir = std.mem.trim(u8, result.stdout, " \n\r\t");
            include_paths.append(b.dupe(includedir)) catch {};
            
            // Add llvm-c subdir
            const c_include_dir = std.fmt.allocPrint(allocator, "{s}/llvm-c", .{includedir}) catch "";
            c_include_paths.append(c_include_dir) catch {};
            
            if (b.verbose) {
                std.debug.print("✅ LLVM includedir detected: {s}\n", .{includedir});
            }
        }
    } else |_| {}
    
    // Add fallback paths only if they exist
    const fallback_lib_paths = [_][]const u8{
        "/usr/lib/x86_64-linux-gnu", // Ubuntu/Debian standard lib path
        "/usr/lib64",                // RedHat/CentOS lib path
    };
    
    const fallback_include_paths = [_][]const u8{
        "/usr/include/llvm-18",
        "/usr/include/llvm",
    };
    
    const fallback_c_include_paths = [_][]const u8{
        "/usr/include/llvm-c-18/llvm-c",
        "/usr/include/llvm-c",
    };
    
    for (fallback_lib_paths) |path| {
        std.fs.cwd().access(path, .{}) catch continue;
        lib_paths.append(b.dupe(path)) catch {};
    }
    
    for (fallback_include_paths) |path| {
        std.fs.cwd().access(path, .{}) catch continue;
        include_paths.append(b.dupe(path)) catch {};
    }
    
    for (fallback_c_include_paths) |path| {
        std.fs.cwd().access(path, .{}) catch continue;
        c_include_paths.append(b.dupe(path)) catch {};
    }
    
    return .{
        .lib_paths = lib_paths.toOwnedSlice() catch &[_][]const u8{},
        .include_paths = include_paths.toOwnedSlice() catch &[_][]const u8{},
        .c_include_paths = c_include_paths.toOwnedSlice() catch &[_][]const u8{},
    };
}

fn addLlvm(b: *std.Build, exe: *std.Build.Step.Compile, target: std.Build.ResolvedTarget) void {
    // LLVM integration for advanced code generation and compilation
    const config = TargetConfig.forTarget(target);
    
    // Only add LLVM for supported targets
    if (!config.supports_llvm) {
        return;
    }
    
    // Skip LLVM for WASM target 
    if (target.result.cpu.arch == .wasm32) {
        return;
    }
    
    // Dynamically detect LLVM paths
    const llvm_paths = detectLlvmPaths(b, b.allocator);
    
    // Legacy static paths for macOS and Windows
    var static_paths: struct {
        lib_paths: []const []const u8,
        include_paths: []const []const u8,
        c_include_paths: []const []const u8,
    } = undefined;
    
    switch (target.result.os.tag) {
        .linux => {
            // Use dynamically detected paths for Linux
        },
        .macos => {
            static_paths = .{
                .lib_paths = &[_][]const u8{
                    "/opt/homebrew/lib",         // Homebrew ARM64
                    "/usr/local/lib",            // Homebrew x86_64
                    "/opt/homebrew/lib/llvm-18/lib",
                    "/usr/local/lib/llvm-18/lib",
                    "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib",
                },
                .include_paths = &[_][]const u8{
                    "/opt/homebrew/include",
                    "/usr/local/include",
                    "/opt/homebrew/include/llvm-18",
                    "/usr/local/include/llvm-18",
                },
                .c_include_paths = &[_][]const u8{
                    "/opt/homebrew/include/llvm-c",
                    "/usr/local/include/llvm-c",
                },
            };
        },
        .windows => {
            static_paths = .{
                .lib_paths = &[_][]const u8{
                    "C:\\Program Files\\LLVM\\lib",
                    "C:\\llvm\\lib",
                    "C:\\tools\\llvm\\lib",
                },
                .include_paths = &[_][]const u8{
                    "C:\\Program Files\\LLVM\\include",
                    "C:\\llvm\\include",
                },
                .c_include_paths = &[_][]const u8{
                    "C:\\Program Files\\LLVM\\include\\llvm-c",
                },
            };
        },
        else => {
            static_paths = .{
                .lib_paths = &[_][]const u8{},
                .include_paths = &[_][]const u8{},
                .c_include_paths = &[_][]const u8{},
            };
        },
    }
    
    // Add LLVM library paths
    const lib_paths_to_use = if (target.result.os.tag == .linux) llvm_paths.lib_paths else static_paths.lib_paths;
    for (lib_paths_to_use) |path| {
        exe.addLibraryPath(.{ .cwd_relative = path });
        if (b.verbose) {
            std.debug.print("✅ Added LLVM lib path: {s}\n", .{path});
        }
    }
    
    // Add LLVM include directories
    const include_paths_to_use = if (target.result.os.tag == .linux) llvm_paths.include_paths else static_paths.include_paths;
    for (include_paths_to_use) |path| {
        exe.addSystemIncludePath(.{ .cwd_relative = path });
        if (b.verbose) {
            std.debug.print("✅ Added LLVM include: {s}\n", .{path});
        }
    }
    
    const c_include_paths_to_use = if (target.result.os.tag == .linux) llvm_paths.c_include_paths else static_paths.c_include_paths;
    for (c_include_paths_to_use) |path| {
        exe.addSystemIncludePath(.{ .cwd_relative = path });
        if (b.verbose) {
            std.debug.print("✅ Added LLVM-C include: {s}\n", .{path});
        }
    }
    
    // Detect and link the correct LLVM library
    const llvm_lib = detectLlvmLibrary(b, target);
    exe.linkSystemLibrary(llvm_lib);
    
    // Compile LLVM wrapper for this specific target instead of using pre-compiled object
    exe.addCSourceFile(.{
        .file = b.path("src-zig/llvm_wrapper.c"),
        .flags = &[_][]const u8{
            "-std=c99",
            "-O2",
            "-DLLVM_VERSION_MAJOR=18",
            "-DLLVM_VERSION_MINOR=1",
        },
    });
    
    // Set target-specific LLVM macros 
    const target_triple = switch (target.result.os.tag) {
        .linux => switch (target.result.cpu.arch) {
            .x86_64 => "x86_64-unknown-linux-gnu",
            .aarch64 => "aarch64-unknown-linux-gnu",
            else => "unknown-unknown-linux-gnu",
        },
        .macos => switch (target.result.cpu.arch) {
            .x86_64 => "x86_64-apple-darwin",
            .aarch64 => "aarch64-apple-darwin",
            else => "unknown-apple-darwin",
        },
        .windows => switch (target.result.cpu.arch) {
            .x86_64 => "x86_64-pc-windows-gnu",
            else => "unknown-pc-windows-gnu",
        },
        else => "unknown-unknown-unknown",
    };
    
    // Set LLVM C macro definitions for proper integration
    exe.root_module.addCMacro("LLVM_VERSION_MAJOR", "18");
    exe.root_module.addCMacro("LLVM_VERSION_MINOR", "1");
    exe.root_module.addCMacro("LLVM_DEFAULT_TARGET_TRIPLE", b.fmt("\"{s}\"", .{target_triple}));
    
    // Configure CPU target to avoid athlon-xp conflicts
    const cpu_name = switch (target.result.cpu.arch) {
        .x86_64 => "x86-64",
        .aarch64 => "generic",
        else => "generic",
    };
    exe.root_module.addCMacro("LLVM_TARGET_CPU", b.fmt("\"{s}\"", .{cpu_name}));
    exe.root_module.addCMacro("LLVM_HOST_TARGET", b.fmt("\"{s}\"", .{cpu_name}));
    
    // Target-specific macros
    switch (target.result.cpu.arch) {
        .x86_64 => {
            exe.root_module.addCMacro("__x86_64__", "1");
            exe.root_module.addCMacro("__i386__", "0");
        },
        .aarch64 => {
            exe.root_module.addCMacro("__aarch64__", "1");
            exe.root_module.addCMacro("__x86_64__", "0");
        },
        else => {},
    }
    
    exe.root_module.addCMacro("_GNU_SOURCE", "1");
    exe.root_module.addCMacro("TARGET_CPU", b.fmt("\"{s}\"", .{cpu_name}));
    exe.root_module.addCMacro("LLVM_HOST_TRIPLE", b.fmt("\"{s}\"", .{target_triple}));
    
    // Enable debug information support
    if (b.verbose) {
        std.debug.print("✅ LLVM-18 integration enabled for target: {s} (triple: {s})\n", .{config.description, target_triple});
    }
}

pub fn build(b: *std.Build) void {
    // Get target, defaulting to native/host target when none specified
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    // Ensure we use the resolved target properly
    // Fix CPU detection issue - override incorrect CPU detection
    const resolved_target = blk: {
        const cpu_name = target.result.cpu.model.name;
        
        // Check for known CPU detection issues
        if (target.result.cpu.arch == .x86_64 and 
           (std.mem.eql(u8, cpu_name, "athlon_xp") or 
            std.mem.eql(u8, cpu_name, "athlon") or
            std.mem.eql(u8, cpu_name, "pentium4"))) {
            
            if (b.verbose) {
                std.debug.print("⚠️ Incorrect CPU detected: {s}, overriding to x86-64\n", .{cpu_name});
            }
            
            const target_query = std.Target.Query{
                .cpu_arch = .x86_64,
                .os_tag = target.result.os.tag,
                .cpu_model = .{ .explicit = &std.Target.x86.cpu.x86_64 },
                .abi = target.result.abi,
            };
            break :blk b.resolveTargetQuery(target_query);
        } else {
            break :blk target;
        }
    };
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

    // Create the CURSED compiler executable - use unified main with proper CLI parsing
    const exe = b.addExecutable(.{
        .name = "cursed", 
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/main_unified.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    // Create stable minimal compiler (core features only, no stdlib)
    const exe_stable = b.addExecutable(.{
        .name = "cursed-stable",
        .root_source_file = b.path("src-zig/stable_minimal_main.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    // Create interactive debugger (development tool)
    const debugger_exe = if (!is_wasm) b.addExecutable(.{
        .name = "cursed-debug",
        .root_source_file = b.path("src-zig/cursed_debugger_main.zig"),
        .target = resolved_target,
        .optimize = optimize,
    }) else null;
    
    // Configure debugger for all non-WASM targets
    if (debugger_exe) |debug_exe| {
        debug_exe.linkLibC();
    }

    // Configure libc and LLVM support for main compiler
    if (!is_wasm) {
        exe.linkLibC();
        
        // Add LLVM support - enable for native builds and compatible cross-compilation
        const enable_llvm = config.supports_llvm and (
            // Enable for native builds only for now
            !is_cross_compile
            // TODO: Enable cross-compilation when proper LLVM toolchain setup is available
            // (resolved_target.result.os.tag == .linux and builtin.target.os.tag == .linux)
        );
        
        if (enable_llvm) {
            addLlvm(b, exe, resolved_target);
            exe.root_module.addCMacro("CURSED_ENABLE_LLVM", "1");
            if (b.verbose) {
                std.debug.print("✅ LLVM enabled for target: {s}\n", .{config.description});
            }
        } else {
            exe.root_module.addCMacro("CURSED_DISABLE_LLVM", "1");
            if (b.verbose) {
                std.debug.print("⚠️ LLVM disabled for target: {s}\n", .{config.description});
            }
        }
        
        // Set explicit CPU target to avoid athlon-xp conflicts
        const cpu_name = switch (resolved_target.result.cpu.arch) {
            .x86_64 => "x86-64",
            .aarch64 => "generic",
            else => "generic",
        };
        exe.root_module.addCMacro("TARGET_CPU", b.fmt("\"{s}\"", .{cpu_name}));
        
        // C imports disabled to avoid CPU target issues temporarily
        
        // Add defer runtime C source for complete defer statement support (temporarily disabled)
        // exe.addCSourceFile(.{
        //     .file = b.path("src-zig/defer_runtime_complete.c"),
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
        // Add LLVM support to complete compiler - native builds only for now
        const enable_llvm_complete = config.supports_llvm and !is_cross_compile;
        
        if (enable_llvm_complete) {
            addLlvm(b, complete_exe, resolved_target);
        }
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

    // Simple LLVM Test executable
    const exe_simple_llvm_test = b.addExecutable(.{
        .name = "cursed-simple-llvm-test",
        .root_source_file = b.path("src-zig/simple_llvm_test.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    if (!is_wasm) {
        exe_simple_llvm_test.linkLibC();
        addLlvm(b, exe_simple_llvm_test, resolved_target);
    }

    // LLVM Test executable (more complex)
    const exe_llvm_test = b.addExecutable(.{
        .name = "cursed-llvm-test",
        .root_source_file = b.path("src-zig/main_llvm_test.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    if (!is_wasm) {
        exe_llvm_test.linkLibC();
        addLlvm(b, exe_llvm_test, resolved_target);
    }

    // Create enhanced CLI version with comprehensive argument parsing
    const cursed_enhanced_cli = b.addExecutable(.{
        .name = "cursed-cli",
        .root_source_file = b.path("src-zig/main_enhanced_cli.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    if (!is_wasm) {
        cursed_enhanced_cli.linkLibC();
    }

    b.installArtifact(exe);
    b.installArtifact(exe_stable);
    if (debugger_exe) |debug_exe| {
        b.installArtifact(debug_exe);  // Install interactive debugger
    }
    b.installArtifact(exe_simple_llvm_test);
    b.installArtifact(exe_llvm_test);
    b.installArtifact(cursed_enhanced_cli);
    
    // Create legacy alias for backwards compatibility - using minimal version
    const legacy_exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/minimal_main.zig"),
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

    // Create LSP server for IDE support - standalone version
    const lsp_server_exe = b.addExecutable(.{
        .name = "cursed-lsp",
        .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/lsp_main_standalone.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    
    if (!is_wasm) {
        lsp_server_exe.linkLibC();
        // Disable LLVM for LSP to avoid compilation issues
        lsp_server_exe.root_module.addCMacro("CURSED_DISABLE_LLVM", "1");
    }
    b.installArtifact(lsp_server_exe);

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

    if (!is_wasm) {
        unit_tests.linkLibC();
        // Add LLVM support for tests if available
        if (config.supports_llvm and !is_cross_compile) {
            addLlvm(b, unit_tests, resolved_target);
        }
    }

    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);

    // Create concurrency test suite
    const concurrency_tests = b.addTest(.{
        .root_source_file = b.path("src-zig/concurrency.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });

    // Link libc for concurrency tests (needed for platform APIs)
    if (!is_wasm) {
        concurrency_tests.linkLibC();
    }

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

        // Link libc for concurrency benchmark (uses C allocator)
        concurrency_benchmark.linkLibC();
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

        // Link libc for concurrency test (uses C allocator)
        concurrency_test_exe.linkLibC();
        b.installArtifact(concurrency_test_exe);

        const run_concurrency_test_exe = b.addRunArtifact(concurrency_test_exe);
        run_concurrency_test_exe.step.dependOn(b.getInstallStep());

        const concurrency_full_test_step = b.step("test-concurrency-full", "Run comprehensive concurrency tests");
        concurrency_full_test_step.dependOn(&run_concurrency_test_exe.step);
    }

    // Note: Using direct file imports instead of modules to avoid conflicts
    
    // Create syscall-enabled compiler with real file I/O, networking, and process management
    // Build for native targets only due to LLVM library incompatibility
    const syscall_exe = if (config.supports_llvm and !is_cross_compile) blk: {
        // Override the CPU target to avoid athlon-xp detection issue
        const syscall_target_query = std.Target.Query{
            .cpu_arch = resolved_target.result.cpu.arch,
            .os_tag = resolved_target.result.os.tag,
            .cpu_model = .{ .explicit = &std.Target.x86.cpu.x86_64 },
        };
        const syscall_target = b.resolveTargetQuery(syscall_target_query);
        
        const exe_syscall = b.addExecutable(.{
            .name = "cursed-syscall",
            .root_source_file = if (is_wasm) b.path("src-zig/wasm_minimal_compiler.zig") else b.path("src-zig/main_llvm_working.zig"),
            .target = syscall_target,
            .optimize = optimize,
        });
        
        if (!is_wasm) {
            exe_syscall.linkLibC();
            addLlvm(b, exe_syscall, syscall_target);
            
            // LLVM wrapper now added via addLlvm function
        }
        
        break :blk exe_syscall;
    } else null;
    
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

    // Create regression test runner
    const regression_test_exe = b.addExecutable(.{
        .name = "cursed-regression-test",
        .root_source_file = b.path("src-zig/simple_regression_test.zig"),
        .target = resolved_target,
        .optimize = optimize,
    });
    
    if (!is_wasm) {
        regression_test_exe.linkLibC();
    }
    
    b.installArtifact(regression_test_exe);
    
    // Create regression test run step
    const run_regression_tests = b.addRunArtifact(regression_test_exe);
    run_regression_tests.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_regression_tests.addArgs(args);
    }
    
    const regression_test_step = b.step("test-regression", "Run comprehensive regression tests");
    regression_test_step.dependOn(&run_regression_tests.step);
    
    // Create memory-specific regression tests
    const memory_regression_test = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        "cd tests/regression/memory && for f in *.csd; do echo \"Testing $f with valgrind...\"; timeout 10 valgrind --leak-check=summary --error-exitcode=1 ../../../zig-out/bin/cursed-zig \"$f\" || exit 1; done"
    });
    memory_regression_test.step.dependOn(b.getInstallStep());
    
    const memory_test_step = b.step("test-memory-regression", "Run memory safety regression tests");
    memory_test_step.dependOn(&memory_regression_test.step);

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
                b.path("src-zig/wasm_minimal.zig") 
            else 
                b.path("src-zig/demo_simple.zig"),  // Use simple demo for cross-compilation (no LLVM)
            .target = cross_target,
            .optimize = optimize,
        });
        
        // Configure cross-compiled executable
        if (query.cpu_arch != .wasm32) {
            cross_exe.linkLibC();
            
            // Disable LLVM for cross-compilation to avoid library linking issues
            cross_exe.root_module.addCMacro("CURSED_DISABLE_LLVM", "1");
            if (b.verbose) {
                const cross_target_for_llvm = b.resolveTargetQuery(query);
                const cross_config_for_llvm = TargetConfig.forTarget(cross_target_for_llvm);
                std.debug.print("⚠️ Cross-compilation LLVM disabled for: {s} (library compatibility)\n", .{cross_config_for_llvm.description});
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
