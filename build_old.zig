const std = @import("std");
const builtin = @import("builtin");
const linker_script_manager = @import("src-zig/linker_script_manager.zig");

// CURSED Compilation Helper Functions
const CursedBuilder = struct {
    b: *std.Build,
    
    const Self = @This();
    
    pub fn init(b: *std.Build) Self {
        return Self{ .b = b };
    }
    
    // Compile a .csd file to an object file
    pub fn compileToObject(self: Self, csd_file: []const u8, output_name: []const u8) *std.Build.Step.Run {
        // CURSED compiler generates executables, not object files, so we compile and rename
        const compile_cmd = self.b.addSystemCommand(&[_][]const u8{
            "bash", "-c",
            self.b.fmt(
                \\./zig-out/bin/cursed-zig --compile "{s}" && \
                \\mkdir -p zig-out/obj && \
                \\mv "$(basename "{s}" .csd)" zig-out/obj/{s}
            , .{ csd_file, csd_file, output_name })
        });
        compile_cmd.step.dependOn(self.b.getInstallStep()); // Ensure compiler is built first
        return compile_cmd;
    }
    
    // Compile a .csd file to an executable
    pub fn compileToExecutable(self: Self, csd_file: []const u8, exe_name: []const u8) *std.Build.Step.Run {
        // CURSED compiler doesn't support -o flag, so we compile in current directory then move
        const compile_cmd = self.b.addSystemCommand(&[_][]const u8{
            "bash", "-c",
            self.b.fmt(
                \\./zig-out/bin/cursed-zig --compile "{s}" && \
                \\mkdir -p zig-out/bin && \
                \\mv "$(basename "{s}" .csd)" zig-out/bin/{s}
            , .{ csd_file, csd_file, exe_name })
        });
        compile_cmd.step.dependOn(self.b.getInstallStep()); // Ensure compiler is built first
        return compile_cmd;
    }
    
    // Interpret a .csd file (for testing)
    pub fn interpret(self: Self, csd_file: []const u8) *std.Build.Step.Run {
        const run_cmd = self.b.addSystemCommand(&[_][]const u8{
            "./zig-out/bin/cursed-zig",
            csd_file,
        });
        run_cmd.step.dependOn(self.b.getInstallStep()); // Ensure compiler is built first
        return run_cmd;
    }
    
    // Create a step that runs a compiled CURSED executable
    pub fn createRunStep(self: Self, exe_name: []const u8, description: []const u8) *std.Build.Step {
        const step = self.b.step(exe_name, description);
        const run_cmd = self.b.addSystemCommand(&[_][]const u8{
            self.b.fmt("./zig-out/bin/{s}", .{exe_name})
        });
        step.dependOn(&run_cmd.step);
        return step;
    }
    
    // Batch compile multiple .csd files
    pub fn batchCompile(self: Self, files: []const []const u8, step_name: []const u8, description: []const u8) *std.Build.Step {
        const batch_step = self.b.step(step_name, description);
        
        for (files) |file| {
            // Extract filename without extension for executable name
            const basename = std.fs.path.basename(file);
            const exe_name = if (std.mem.endsWith(u8, basename, ".csd"))
                basename[0..basename.len - 4]
            else
                basename;
                
            const compile_cmd = self.compileToExecutable(file, exe_name);
            batch_step.dependOn(&compile_cmd.step);
        }
        
        return batch_step;
    }
};

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

// Enhanced LLVM library detection with better error handling and caching
fn detectLlvmLibrary(b: *std.Build, target: std.Build.ResolvedTarget) struct {
    lib_name: []const u8,
    found: bool,
    version: []const u8,
} {
    _ = target; // Unused for now
    
    // Try multiple llvm-config versions in order of preference
    const llvm_configs = [_][]const u8{
        "llvm-config-18", "llvm-config-17", "llvm-config-16", "llvm-config-15", "llvm-config"
    };
    
    for (llvm_configs) |config_cmd| {
        // Check libdir first to validate installation
        const libdir_result = std.process.Child.run(.{
            .allocator = b.allocator,
            .argv = &[_][]const u8{ config_cmd, "--libdir" },
        }) catch continue;
        defer b.allocator.free(libdir_result.stdout);
        defer b.allocator.free(libdir_result.stderr);
        
        if (libdir_result.term != .Exited or libdir_result.term.Exited != 0) continue;
        
        // Get version info
        const version_result = std.process.Child.run(.{
            .allocator = b.allocator,
            .argv = &[_][]const u8{ config_cmd, "--version" },
        }) catch continue;
        defer b.allocator.free(version_result.stdout);
        defer b.allocator.free(version_result.stderr);
        
        if (version_result.term == .Exited and version_result.term.Exited == 0) {
            const version_str = std.mem.trim(u8, version_result.stdout, " \n\r\t");
            const libdir = std.mem.trim(u8, libdir_result.stdout, " \n\r\t");
            
            // Verify the libdir actually exists
            std.fs.cwd().access(libdir, .{}) catch continue;
            
            if (b.verbose) {
                std.debug.print("✅ LLVM detected: {s} version {s} at {s}\n", .{ config_cmd, version_str, libdir });
            }
            
            // Extract major version for library name
            if (std.mem.indexOf(u8, version_str, ".")) |dot_idx| {
                const major_version = version_str[0..dot_idx];
                const lib_name = b.fmt("LLVM-{s}", .{major_version});
                return .{ .lib_name = lib_name, .found = true, .version = b.dupe(version_str) };
            } else {
                return .{ .lib_name = "LLVM", .found = true, .version = b.dupe(version_str) };
            }
        }
    }
    
    // Fallback detection based on platform
    const lib_name = switch (builtin.target.os.tag) {
        .linux => "LLVM-18",        // Ubuntu/Debian standard
        .macos => "LLVM",           // Homebrew standard  
        .windows => "LLVM",         // Windows standard
        else => "LLVM-18",          // Default
    };
    
    if (b.verbose) {
        std.debug.print("⚠️ LLVM auto-detection failed, using fallback: {s}\n", .{lib_name});
    }
    return .{ .lib_name = lib_name, .found = false, .version = "unknown" };
}

// Optimized LLVM path detection with caching and validation
fn detectLlvmPaths(b: *std.Build, allocator: std.mem.Allocator) struct {
    lib_paths: [][]const u8,
    include_paths: [][]const u8,
    c_include_paths: [][]const u8,
    found: bool,
} {
    var lib_paths: std.ArrayList([]const u8) = .empty;
    defer lib_paths.deinit(allocator);
    var include_paths: std.ArrayList([]const u8) = .empty;
    defer include_paths.deinit(allocator);
    var c_include_paths: std.ArrayList([]const u8) = .empty;
    defer c_include_paths.deinit(allocator);
    var found = false;
    
    // Try multiple llvm-config versions
    const llvm_configs = [_][]const u8{
        "llvm-config-18", "llvm-config-17", "llvm-config-16", "llvm-config-15", "llvm-config"
    };
    
    for (llvm_configs) |config_cmd| {
        if (std.process.Child.run(.{
            .allocator = allocator,
            .argv = &[_][]const u8{ config_cmd, "--libdir" },
        })) |result| {
            defer allocator.free(result.stdout);
            defer allocator.free(result.stderr);
            
            if (result.term == .Exited and result.term.Exited == 0) {
                const libdir = std.mem.trim(u8, result.stdout, " \n\r\t");
                
                // Validate directory exists
                std.fs.cwd().access(libdir, .{}) catch continue;
                
                lib_paths.append(allocator, b.dupe(libdir)) catch {};
                found = true;
                
                if (b.verbose) {
                    std.debug.print("✅ LLVM libdir found: {s}\n", .{libdir});
                }
                break; // Use first working config
            }
        } else |_| {}
    }
    
    for (llvm_configs) |config_cmd| {
        if (std.process.Child.run(.{
            .allocator = allocator,
            .argv = &[_][]const u8{ config_cmd, "--includedir" },
        })) |result| {
            defer allocator.free(result.stdout);
            defer allocator.free(result.stderr);
            
            if (result.term == .Exited and result.term.Exited == 0) {
                const includedir = std.mem.trim(u8, result.stdout, " \n\r\t");
                
                // Validate directory exists
                std.fs.cwd().access(includedir, .{}) catch continue;
                
                include_paths.append(allocator, b.dupe(includedir)) catch {};
                
                // Add llvm-c subdir if it exists
                const c_include_dir = std.fmt.allocPrint(allocator, "{s}/llvm-c", .{includedir}) catch "";
                if (c_include_dir.len > 0) {
                    std.fs.cwd().access(c_include_dir, .{}) catch {
                        allocator.free(c_include_dir);
                        continue;
                    };
                    c_include_paths.append(allocator, c_include_dir) catch {};
                }
                
                if (b.verbose) {
                    std.debug.print("✅ LLVM includedir found: {s}\n", .{includedir});
                }
                break;
            }
        } else |_| {}
    }
    
    // Add validated fallback paths only if auto-detection failed
    if (!found) {
        const fallback_lib_paths = [_][]const u8{
            "/usr/lib/x86_64-linux-gnu",
            "/usr/lib64",
            "/usr/lib/llvm-18/lib",
            "/usr/lib/llvm-17/lib",
            "/usr/lib/llvm-16/lib",
        };
        
        const fallback_include_paths = [_][]const u8{
            "/usr/include/llvm-18",
            "/usr/include/llvm-17",
            "/usr/include/llvm-16",
            "/usr/include/llvm",
        };
        
        const fallback_c_include_paths = [_][]const u8{
            "/usr/include/llvm-c-18/llvm-c",
            "/usr/include/llvm-c-17/llvm-c",
            "/usr/include/llvm-c-16/llvm-c",
            "/usr/include/llvm-c",
        };
        
        for (fallback_lib_paths) |path| {
            std.fs.cwd().access(path, .{}) catch continue;
            lib_paths.append(allocator, b.dupe(path)) catch {};
            found = true;
        }
        
        for (fallback_include_paths) |path| {
            std.fs.cwd().access(path, .{}) catch continue;
            include_paths.append(allocator, b.dupe(path)) catch {};
        }
        
        for (fallback_c_include_paths) |path| {
            std.fs.cwd().access(path, .{}) catch continue;
            c_include_paths.append(allocator, b.dupe(path)) catch {};
        }
    }
    
    return .{
        .lib_paths = lib_paths.toOwnedSlice(allocator) catch &[_][]const u8{},
        .include_paths = include_paths.toOwnedSlice(allocator) catch &[_][]const u8{},
        .c_include_paths = c_include_paths.toOwnedSlice(allocator) catch &[_][]const u8{},
        .found = found,
    };
}

fn addLlvm(b: *std.Build, exe: *std.Build.Step.Compile, target: std.Build.ResolvedTarget) void {
    // LLVM integration for advanced code generation and compilation
    const config = TargetConfig.forTarget(target);
    
    // Only add LLVM for supported targets
    if (!config.supports_llvm) {
        if (b.verbose) {
            std.debug.print("⚠️ LLVM not supported for target: {s}\n", .{config.description});
        }
        return;
    }
    
    // Skip LLVM for WASM target 
    if (target.result.cpu.arch == .wasm32) {
        if (b.verbose) {
            std.debug.print("⚠️ LLVM disabled for WASM target\n", .{});
        }
        return;
    }
    
    // Detect LLVM library info
    const llvm_lib_info = detectLlvmLibrary(b, target);
    if (!llvm_lib_info.found) {
        if (b.verbose) {
            std.debug.print("⚠️ LLVM not found, using fallback: {s}\n", .{llvm_lib_info.lib_name});
        }
    }
    
    // Dynamically detect LLVM paths
    const llvm_paths = detectLlvmPaths(b, b.allocator);
    if (!llvm_paths.found) {
        if (b.verbose) {
            std.debug.print("⚠️ LLVM paths not auto-detected, using fallbacks\n", .{});
        }
    }
    
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
    
    // Add system library paths first (for OpenGL, X11, ALSA, Vulkan, etc.)
    const system_lib_paths = [_][]const u8{
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib64",
        "/usr/lib",
        "/lib/x86_64-linux-gnu",
        "/lib64",
        "/lib",
    };
    
    for (system_lib_paths) |path| {
        std.fs.cwd().access(path, .{}) catch continue;
        exe.addLibraryPath(.{ .cwd_relative = path });
        if (b.verbose) {
            std.debug.print("✅ Added system lib path: {s}\n", .{path});
        }
    }
    
    // Add LLVM library paths (only existing ones)
    const lib_paths_to_use = if (target.result.os.tag == .linux) llvm_paths.lib_paths else static_paths.lib_paths;
    for (lib_paths_to_use) |path| {
        // Validate path exists before adding
        std.fs.cwd().access(path, .{}) catch {
            if (b.verbose) {
                std.debug.print("⚠️ Skipping non-existent LLVM lib path: {s}\n", .{path});
            }
            continue;
        };
        
        exe.addLibraryPath(.{ .cwd_relative = path });
        if (b.verbose) {
            std.debug.print("✅ Added LLVM lib path: {s}\n", .{path});
        }
    }
    
    // Add LLVM include directories (only existing ones)
    const include_paths_to_use = if (target.result.os.tag == .linux) llvm_paths.include_paths else static_paths.include_paths;
    for (include_paths_to_use) |path| {
        std.fs.cwd().access(path, .{}) catch {
            if (b.verbose) {
                std.debug.print("⚠️ Skipping non-existent LLVM include: {s}\n", .{path});
            }
            continue;
        };
        
        exe.addSystemIncludePath(.{ .cwd_relative = path });
        if (b.verbose) {
            std.debug.print("✅ Added LLVM include: {s}\n", .{path});
        }
    }
    
    const c_include_paths_to_use = if (target.result.os.tag == .linux) llvm_paths.c_include_paths else static_paths.c_include_paths;
    for (c_include_paths_to_use) |path| {
        std.fs.cwd().access(path, .{}) catch {
            if (b.verbose) {
                std.debug.print("⚠️ Skipping non-existent LLVM-C include: {s}\n", .{path});
            }
            continue;
        };
        
        exe.addSystemIncludePath(.{ .cwd_relative = path });
        if (b.verbose) {
            std.debug.print("✅ Added LLVM-C include: {s}\n", .{path});
        }
    }
    
    // Link the detected LLVM library
    exe.linkSystemLibrary(llvm_lib_info.lib_name);
    
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
        std.debug.print("✅ LLVM-{s} integration enabled for target: {s} (triple: {s})\n", .{llvm_lib_info.version, config.description, target_triple});
    }
}

// Add LLVM function stubs for cross-compilation
fn addLlvmStubs(b: *std.Build, exe: *std.Build.Step.Compile) void {
    // Create a stub LLVM library using Zig instead of C for better integration
    const stub_lib = b.addObject(.{
        .name = "llvm-stub",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/llvm_stub.zig"),
            .target = exe.root_module.resolved_target.?,
            .optimize = exe.root_module.optimize.?,
        }),
    });
    
    exe.addObject(stub_lib);
    
    if (b.verbose) {
        std.debug.print("🔧 Added LLVM stub library for cross-compilation\n", .{});
    }
}

// Auto-detect optimal job count for parallel builds
fn getOptimalJobCount() u32 {
    // Get CPU count
    const cpu_count = std.Thread.getCpuCount() catch 4;
    
    // Use CPU count but cap at reasonable limits
    const max_jobs = if (cpu_count <= 2) cpu_count 
                    else if (cpu_count <= 8) cpu_count 
                    else @min(cpu_count, 12); // Cap at 12 for stability
    
    return @intCast(max_jobs);
}

pub fn build(b: *std.Build) void {
    // Import deadlock prevention system
    const build_system_fixes = @import("src-zig/build_system_fixes.zig");
    
    // Apply safe build configuration with deadlock prevention
    build_system_fixes.createSafeBuildConfig(b) catch |err| {
        std.debug.print("⚠️ Warning: Could not apply all safe build configurations: {}\n", .{err});
    };
    
    // Auto-tune parallel build jobs with enhanced safety
    const optimal_jobs = getOptimalJobCount();
    if (b.verbose) {
        std.debug.print("🔧 Auto-tuned build jobs: {d} (with deadlock prevention)\n", .{optimal_jobs});
    }
    
    // Initialize linker script manager for cross-compilation support
    var linker_manager = linker_script_manager.LinkerScriptManager.init(b.allocator, b.build_root.path orelse ".");
    defer linker_manager.deinit();
    
    // Check ninja max jobs and print recommendation
    if (std.process.getEnvVarOwned(b.allocator, "NINJA_MAX_JOBS")) |ninja_env| {
        defer b.allocator.free(ninja_env);
        if (b.verbose) {
            std.debug.print("🔧 NINJA_MAX_JOBS already set to: {s}\n", .{ninja_env});
        }
    } else |_| {
        if (b.verbose) {
            std.debug.print("🔧 Recommend setting NINJA_MAX_JOBS={d} for optimal performance\n", .{optimal_jobs});
        }
    }
    
    // Get target, defaulting to native/host target when none specified
    // Fix architecture mismatch: Force glibc ABI on Linux to prevent musl linking issues
    const default_query = std.Target.Query{
        .os_tag = @import("builtin").target.os.tag,
        .cpu_arch = @import("builtin").target.cpu.arch,
        .abi = if (@import("builtin").target.os.tag == .linux) .gnu else null,
    };
    const target = b.standardTargetOptions(.{
        .default_target = default_query,
    });
    const optimize = b.standardOptimizeOption(.{});
    
    // Performance optimization options and cache configuration
    const enable_cache = b.option(bool, "cache", "Enable compilation caching") orelse true;
    const cache_dir = b.option([]const u8, "cache-dir", "Cache directory path") orelse ".cursed_build_cache";
    const enable_incremental = b.option(bool, "incremental", "Enable incremental compilation") orelse true;
    const invalidate_cache = b.option(bool, "invalidate-cache", "Force cache invalidation") orelse false;
    
    _ = b.option(bool, "optimize-compiler", "Enable compiler performance optimizations") orelse true;
    _ = b.option(bool, "parallel", "Enable parallel compilation") orelse true;
    _ = b.option([]const u8, "llvm-opt", "LLVM optimization level (O0, O1, O2, O3, Os, Oz)") orelse "O2";
    _ = b.option(bool, "fast-build", "Prioritize compilation speed over runtime performance") orelse false;
    _ = b.option(bool, "profile", "Enable performance profiling") orelse false;
    _ = b.option(bool, "memory-opt", "Enable memory optimization") orelse true;
    
    // Fix ReleaseSmall debug sections issue
    const actual_optimize = if (optimize == .ReleaseSmall) blk: {
        if (b.verbose) {
            std.debug.print("🔧 ReleaseSmall: Disabling debug info to prevent size bloat\n", .{});
        }
        break :blk std.builtin.OptimizeMode.ReleaseSmall;
    } else optimize;
    
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
    
    // Generate normalized target triple for linker script selection
    // Force GNU ABI for Linux to prevent architecture mismatches  
    const target_triple_str = switch (resolved_target.result.os.tag) {
        .linux => switch (resolved_target.result.cpu.arch) {
            .x86_64 => "x86_64-unknown-linux-gnu",  // Always use GNU, never musl
            .aarch64 => "aarch64-unknown-linux-gnu", // Always use GNU, never musl  
            .x86 => "i386-unknown-linux-gnu",
            else => "unknown-unknown-linux-gnu",
        },
        .macos => switch (resolved_target.result.cpu.arch) {
            .x86_64 => "x86_64-apple-darwin",
            .aarch64 => "aarch64-apple-darwin",
            else => "unknown-apple-darwin",
        },
        .windows => switch (resolved_target.result.cpu.arch) {
            .x86_64 => "x86_64-pc-windows-gnu",
            .aarch64 => "aarch64-pc-windows-gnu",
            .x86 => "i386-pc-windows-gnu",
            else => "unknown-pc-windows-gnu",
        },
        .wasi => "wasm32-wasi",
        .freestanding => switch (resolved_target.result.cpu.arch) {
            .wasm32 => "wasm32-unknown-unknown",
            .aarch64 => "aarch64-unknown-none",
            else => "unknown-unknown-none",
        },
        else => "unknown-unknown-unknown",
    };

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
        std.debug.print("  Target triple: {s}\n", .{target_triple_str});
        std.debug.print("  Cross-compiling: {}\n", .{is_cross_compile});
        std.debug.print("  Optimize mode: {s}\n", .{@tagName(actual_optimize)});
        
        // Print linker configuration info
        linker_manager.printLinkerConfigInfo(target_triple_str, b) catch |err| {
            std.debug.print("⚠️ Failed to print linker config: {}\n", .{err});
        };
    }

    // Create the CURSED compiler executable - use unified main with proper CLI parsing
    const exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/main_unified.zig"),
            .target = resolved_target,
            .optimize = actual_optimize,
        }),
    });



    // Explicitly disable debug info for ReleaseSmall builds
    if (actual_optimize == .ReleaseSmall) {
        exe.want_lto = true;
        exe.root_module.addCMacro("NDEBUG", "1");
        if (b.verbose) {
            std.debug.print("🔧 ReleaseSmall: Stripped debug symbols and defined NDEBUG\n", .{});
        }
    }

    // Create stable minimal compiler (core features only, no stdlib)
    const stable_source = if (resolved_target.result.os.tag == .freestanding or resolved_target.result.os.tag == .wasi) 
        b.path("src-zig/freestanding_main.zig")
    else 
        b.path("src-zig/stable_minimal_main.zig");
        
    const exe_stable = b.addExecutable(.{
        .name = "cursed-stable",
        .root_module = b.createModule(.{
            .root_source_file = stable_source,
            .target = resolved_target,
            .optimize = actual_optimize,
        }),
    });

    // Apply ReleaseSmall fix to stable build too
    if (actual_optimize == .ReleaseSmall) {
        exe_stable.want_lto = true;
        exe_stable.root_module.addCMacro("NDEBUG", "1");
    }
    
    // Apply linker script configuration for cross-compilation
    linker_manager.applyLinkerConfig(exe, target_triple_str, b) catch |err| {
        if (b.verbose) {
            std.debug.print("⚠️ Failed to apply linker config to main executable: {}\n", .{err});
        }
    };
    
    linker_manager.applyLinkerConfig(exe_stable, target_triple_str, b) catch |err| {
        if (b.verbose) {
            std.debug.print("⚠️ Failed to apply linker config to stable executable: {}\n", .{err});
        }
    };

    // Configure build options
    const supports_libc = !is_wasm and resolved_target.result.os.tag != .freestanding and resolved_target.result.os.tag != .wasi;
    
    if (supports_libc) {
        exe.linkLibC();
        exe_stable.linkLibC();
        
        // Add Windows-specific libraries for async I/O support
        if (resolved_target.result.os.tag == .windows) {
            exe.linkSystemLibrary("ws2_32");
            exe.linkSystemLibrary("kernel32");
            exe_stable.linkSystemLibrary("ws2_32");
            exe_stable.linkSystemLibrary("kernel32");
            
            if (b.verbose) {
                std.debug.print("🪟 Added Windows async I/O libraries (ws2_32, kernel32)\n", .{});
            }
        }
        
        // Add LLVM support for native builds, or stub LLVM for cross-compilation
        if (config.supports_llvm and !is_cross_compile) {
            addLlvm(b, exe, resolved_target);
        } else if (is_cross_compile) {
            // For cross-compilation, we need LLVM function stubs
            addLlvmStubs(b, exe);
        }
    }
    
    // Install artifacts
    b.installArtifact(exe);
    b.installArtifact(exe_stable);
    
    // Create run steps
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    
    const run_step = b.step("run", "Run the CURSED compiler");
    run_step.dependOn(&run_cmd.step);
    
    // Create stable run step
    const run_stable_cmd = b.addRunArtifact(exe_stable);
    run_stable_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_stable_cmd.addArgs(args);
    }
    
    const run_stable_step = b.step("run-stable", "Run the stable CURSED compiler");
    run_stable_step.dependOn(&run_stable_cmd.step);
    


    // Unit tests
    const unit_tests = b.addTest(.{
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/test_main.zig"),
            .target = resolved_target,
            .optimize = actual_optimize,
        }),
    });
    
    if (supports_libc) {
        unit_tests.linkLibC();
        
        // Add Windows-specific libraries for async I/O tests
        if (resolved_target.result.os.tag == .windows) {
            unit_tests.linkSystemLibrary("ws2_32");
            unit_tests.linkSystemLibrary("kernel32");
        }
    }
    
    // Apply linker configuration to unit tests
    linker_manager.applyLinkerConfig(unit_tests, target_triple_str, b) catch |err| {
        if (b.verbose) {
            std.debug.print("⚠️ Failed to apply linker config to unit tests: {}\n", .{err});
        }
    };
    
    // Windows-specific async I/O tests
    if (resolved_target.result.os.tag == .windows) {
        const windows_async_tests = b.addTest(.{
            .root_module = b.createModule(.{
                .root_source_file = b.path("src-zig/windows_async_test.zig"),
                .target = resolved_target,
                .optimize = actual_optimize,
            }),
        });
        
        if (supports_libc) {
            windows_async_tests.linkLibC();
            windows_async_tests.linkSystemLibrary("ws2_32");
            windows_async_tests.linkSystemLibrary("kernel32");
        }
        
        const run_windows_async_tests = b.addRunArtifact(windows_async_tests);
        const windows_test_step = b.step("test-windows-async", "Run Windows async I/O tests");
        windows_test_step.dependOn(&run_windows_async_tests.step);
        
        if (b.verbose) {
            std.debug.print("🪟 Added Windows async I/O test suite\n", .{});
        }
    }
    
    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);
    
    // Add fixed JIT execution engine
    const jit_main = b.addExecutable(.{
        .name = "cursed-jit",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/jit_main.zig"),
            .target = resolved_target,
            .optimize = actual_optimize,
        }),
    });
    
    // Link LLVM for JIT engine if available
    if (supports_libc) {
        jit_main.linkLibC();
        // Try to link LLVM - this may fail on some systems
        // jit_main.linkSystemLibrary("LLVM");
    }
    
    b.installArtifact(jit_main);
    
    const run_jit_cmd = b.addRunArtifact(jit_main);
    run_jit_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_jit_cmd.addArgs(args);
    }
    
    const run_jit_step = b.step("run-jit", "Run the fixed JIT execution engine");
    run_jit_step.dependOn(&run_jit_cmd.step);

    // Create interactive debugger executable
    const debugger_exe = b.addExecutable(.{
        .name = "cursed-debug",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/standalone_debugger_main.zig"),
            .target = resolved_target,
            .optimize = actual_optimize,
        }),
    });

    if (supports_libc) {
        debugger_exe.linkLibC();
        
        // Add Windows-specific libraries if needed
        if (resolved_target.result.os.tag == .windows) {
            debugger_exe.linkSystemLibrary("ws2_32");
            debugger_exe.linkSystemLibrary("kernel32");
        }
    }
    
    // Apply ReleaseSmall fix to debugger
    if (actual_optimize == .ReleaseSmall) {
        debugger_exe.want_lto = true;
        debugger_exe.root_module.addCMacro("NDEBUG", "1");
    }

    b.installArtifact(debugger_exe);

    // Initialize CURSED builder helper
    const cursed_builder = CursedBuilder.init(b);
    
    // P2 Item #7: Multimedia modules - ImageZ, AudioZ, RenderZ  
    // Multimedia demo using CURSED compiler
    const multimedia_demo_compile = cursed_builder.compileToExecutable("examples/multimedia_demo.csd", "cursed-multimedia-demo");
    const multimedia_step = b.step("multimedia", "Compile and run comprehensive multimedia demo showcasing ImageZ, AudioZ, and RenderZ modules");
    multimedia_step.dependOn(&multimedia_demo_compile.step);
    
    // Compile CURSED source files step
    const cursed_compiler_step = b.step("compile-cursed", "Compile CURSED .csd files using CURSED compiler");

    // Individual module demos - Now properly using CURSED compiler
    const imagez_demo_compile = cursed_builder.compileToExecutable("stdlib/imagez/example_image_processing.csd", "cursed-imagez-demo");
    cursed_compiler_step.dependOn(&imagez_demo_compile.step);
    
    // if (supports_libc) {
    //     imagez_demo.linkLibC();
    //     if (resolved_target.result.os.tag == .windows) {
    //         imagez_demo.linkSystemLibrary("gdi32");
    //     }
    //     if (config.supports_llvm and !is_cross_compile) {
    //         addLlvm(b, imagez_demo, resolved_target);
    //     }
    // }
    
    // AudioZ demo compilation
    const audioz_demo_compile = cursed_builder.compileToExecutable("stdlib/audioz/example_audio_processing.csd", "cursed-audioz-demo");
    cursed_compiler_step.dependOn(&audioz_demo_compile.step);
    
    // RenderZ demo compilation
    const renderz_demo_compile = cursed_builder.compileToExecutable("stdlib/renderz/example_graphics_rendering.csd", "cursed-renderz-demo");
    cursed_compiler_step.dependOn(&renderz_demo_compile.step);
    
    // CURSED example compilation steps using batch compile
    const example_files = [_][]const u8{
        "basic_test_simple.csd",
        "test_simple.csd",
    };
    
    const cursed_examples_step = cursed_builder.batchCompile(&example_files, "examples", "Compile all CURSED example files");
    cursed_examples_step.dependOn(cursed_compiler_step);
    
    // Test runner for CURSED files
    const test_cursed_step = b.step("test-cursed", "Test compiled CURSED programs");
    
    // Add individual test runs
    for (example_files) |file| {
        const basename = std.fs.path.basename(file);
        const exe_name = if (std.mem.endsWith(u8, basename, ".csd"))
            basename[0..basename.len - 4]
        else
            basename;
            
        const test_run = b.addSystemCommand(&[_][]const u8{
            b.fmt("./zig-out/bin/{s}", .{exe_name})
        });
        const compile_cmd = cursed_builder.compileToExecutable(file, exe_name);
        test_run.step.dependOn(&compile_cmd.step);
        test_cursed_step.dependOn(&test_run.step);
    }

    // Enhanced LSP server with incremental compilation fixes
    const lsp_root_source = if (resolved_target.result.os.tag == .freestanding or resolved_target.result.os.tag == .wasi) 
        b.path("src-zig/freestanding_main.zig")
    else if (is_wasm) 
        b.path("src-zig/wasm_minimal_compiler.zig") 
    else 
        b.path("src-zig/lsp_main.zig");
        
    const lsp_exe = b.addExecutable(.{
        .name = "cursed-lsp",
        .root_module = b.createModule(.{
            .root_source_file = lsp_root_source,
            .target = resolved_target,
            .optimize = actual_optimize,
        }),
    });
    
    if (supports_libc) {
        lsp_exe.linkLibC();
    }
    
    // Apply ReleaseSmall fix to LSP
    if (actual_optimize == .ReleaseSmall) {
        lsp_exe.want_lto = true;
        lsp_exe.root_module.addCMacro("NDEBUG", "1");
    }
    
    // Apply linker configuration to LSP
    linker_manager.applyLinkerConfig(lsp_exe, target_triple_str, b) catch |err| {
        if (b.verbose) {
            std.debug.print("⚠️ Failed to apply linker config to LSP executable: {}\n", .{err});
        }
    };
    
    b.installArtifact(lsp_exe);

    // Enhanced LSP Server with complete features
    const enhanced_lsp_root_source = if (resolved_target.result.os.tag == .freestanding or resolved_target.result.os.tag == .wasi) 
        b.path("src-zig/freestanding_main.zig")
    else if (is_wasm) 
        b.path("src-zig/wasm_minimal_compiler.zig") 
    else 
        b.path("src-zig/enhanced_lsp_main.zig");
        
    const enhanced_lsp_exe = b.addExecutable(.{
        .name = "cursed-lsp-enhanced",
        .root_module = b.createModule(.{
            .root_source_file = enhanced_lsp_root_source,
            .target = resolved_target,
            .optimize = actual_optimize,
        }),
    });
    
    if (supports_libc) {
        enhanced_lsp_exe.linkLibC();
    }
    
    // Apply ReleaseSmall fix to Enhanced LSP
    if (actual_optimize == .ReleaseSmall) {
        enhanced_lsp_exe.want_lto = true;
        enhanced_lsp_exe.root_module.addCMacro("NDEBUG", "1");
    }
    
    // Apply linker configuration to Enhanced LSP
    linker_manager.applyLinkerConfig(enhanced_lsp_exe, target_triple_str, b) catch |err| {
        if (b.verbose) {
            std.debug.print("⚠️ Failed to apply linker config to Enhanced LSP executable: {}\n", .{err});
        }
    };
    
    b.installArtifact(enhanced_lsp_exe);
    
    const run_lsp = b.addRunArtifact(lsp_exe);
    run_lsp.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_lsp.addArgs(args);
    }
    
    const run_enhanced_lsp = b.addRunArtifact(enhanced_lsp_exe);
    run_enhanced_lsp.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_enhanced_lsp.addArgs(args);
    }
    
    const lsp_step = b.step("lsp", "Run the CURSED Language Server");
    lsp_step.dependOn(&run_lsp.step);
    
    const enhanced_lsp_step = b.step("lsp-enhanced", "Run the Enhanced CURSED Language Server with complete IDE features");
    enhanced_lsp_step.dependOn(&run_enhanced_lsp.step);

    // Performance Optimization CLI
    const perf_cli_source = if (resolved_target.result.os.tag == .freestanding or resolved_target.result.os.tag == .wasi) 
        b.path("src-zig/freestanding_main.zig")  // Fallback for limited targets
    else 
        b.path("performance_optimization_cli_simple.zig");
        
    const perf_cli_exe = b.addExecutable(.{
        .name = "cursed-perf",
        .root_module = b.createModule(.{
            .root_source_file = perf_cli_source,
            .target = resolved_target,
            .optimize = actual_optimize,
        }),
    });
    
    if (supports_libc) {
        perf_cli_exe.linkLibC();
        
        // Add Windows-specific libraries if needed
        if (resolved_target.result.os.tag == .windows) {
            perf_cli_exe.linkSystemLibrary("ws2_32");
            perf_cli_exe.linkSystemLibrary("kernel32");
        }
    }
    
    // Apply ReleaseSmall fix to performance CLI
    if (actual_optimize == .ReleaseSmall) {
        perf_cli_exe.want_lto = true;
        perf_cli_exe.root_module.addCMacro("NDEBUG", "1");
    }
    
    // Apply linker configuration to performance CLI
    linker_manager.applyLinkerConfig(perf_cli_exe, target_triple_str, b) catch |err| {
        if (b.verbose) {
            std.debug.print("⚠️ Failed to apply linker config to performance CLI: {}\n", .{err});
        }
    };
    
    b.installArtifact(perf_cli_exe);
    
    const run_perf_cli = b.addRunArtifact(perf_cli_exe);
    run_perf_cli.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_perf_cli.addArgs(args);
    }
    
    const perf_cli_step = b.step("perf", "Run the CURSED Performance Optimization CLI");
    perf_cli_step.dependOn(&run_perf_cli.step);

    // Enhanced Package Manager CLI
    const pkg_cli_source = if (resolved_target.result.os.tag == .freestanding or resolved_target.result.os.tag == .wasi) 
        b.path("src-zig/freestanding_main.zig")  // Fallback for limited targets
    else 
        b.path("src-zig/cursed_pkg.zig");
        
    const pkg_cli_exe = b.addExecutable(.{
        .name = "cursed-pkg",
        .root_module = b.createModule(.{
            .root_source_file = pkg_cli_source,
            .target = resolved_target,
            .optimize = actual_optimize,
        }),
    });
    
    if (supports_libc) {
        pkg_cli_exe.linkLibC();
        
        // Add Windows-specific libraries if needed
        if (resolved_target.result.os.tag == .windows) {
            pkg_cli_exe.linkSystemLibrary("ws2_32");
            pkg_cli_exe.linkSystemLibrary("kernel32");
        }
    }
    
    // Apply ReleaseSmall fix to package manager CLI
    if (actual_optimize == .ReleaseSmall) {
        pkg_cli_exe.want_lto = true;
        pkg_cli_exe.root_module.addCMacro("NDEBUG", "1");
    }
    
    // Apply linker configuration to package manager CLI
    linker_manager.applyLinkerConfig(pkg_cli_exe, target_triple_str, b) catch |err| {
        if (b.verbose) {
            std.debug.print("⚠️ Failed to apply linker config to package manager CLI: {}\n", .{err});
        }
    };
    
    b.installArtifact(pkg_cli_exe);
    
    const run_pkg_cli = b.addRunArtifact(pkg_cli_exe);
    run_pkg_cli.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_pkg_cli.addArgs(args);
    }
    
    const pkg_cli_step = b.step("pkg", "Run the CURSED Package Manager");
    pkg_cli_step.dependOn(&run_pkg_cli.step);

    // Comprehensive test step that runs all tests
    const all_tests_step = b.step("test-all", "Run all test suites");
    all_tests_step.dependOn(&run_unit_tests.step);
    all_tests_step.dependOn(test_cursed_step);

    // Performance test step with job auto-tuning
    const perf_test_step = b.step("test-performance", "Run performance tests with optimal parallelism");
    const perf_test_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt("export NINJA_MAX_JOBS={d} && echo 'Performance testing with {d} parallel jobs...' && ./zig-out/bin/cursed-zig --version", .{ optimal_jobs, optimal_jobs })
    });
    perf_test_cmd.step.dependOn(b.getInstallStep());
    perf_test_step.dependOn(&perf_test_cmd.step);

    // Cache management steps
    const cache_info_step = b.step("cache-info", "Display cache information and statistics");
    const cache_info_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "📊 CURSED Compilation Cache Information"
            \\echo "Cache directory: {s}"
            \\echo "Cache enabled: {}"
            \\echo "Incremental compilation: {}"
            \\echo "Cache size: $(du -sh {s} 2>/dev/null | cut -f1 || echo 'No cache')"
            \\echo "Cache entries: $(find {s} -type f 2>/dev/null | wc -l || echo '0')"
            \\echo "Cache status: $(test -d {s} && echo 'Active' || echo 'Not initialized')"
        , .{ cache_dir, enable_cache, enable_incremental, cache_dir, cache_dir, cache_dir })
    });
    cache_info_step.dependOn(&cache_info_cmd.step);
    
    const cache_clean_step = b.step("cache-clean", "Clean compilation cache");
    const cache_clean_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "🧹 Cleaning compilation cache..."
            \\rm -rf {s}
            \\echo "✅ Cache cleaned successfully"
        , .{cache_dir})
    });
    cache_clean_step.dependOn(&cache_clean_cmd.step);
    
    const cache_validate_step = b.step("cache-validate", "Validate cache integrity");
    const cache_validate_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c", 
        b.fmt(
            \\echo "🔍 Validating cache integrity..."
            \\if [ -d {s} ]; then
            \\  echo "✅ Cache directory exists"
            \\  echo "AST cache: $(find {s}/ast -name '*.ast' 2>/dev/null | wc -l) entries"
            \\  echo "Object cache: $(find {s}/objects -name '*.o' 2>/dev/null | wc -l) entries"
            \\  echo "✅ Cache validation complete"
            \\else
            \\  echo "⚠️ Cache directory not found - will be created on first build"
            \\fi
        , .{ cache_dir, cache_dir, cache_dir })
    });
    cache_validate_step.dependOn(&cache_validate_cmd.step);
    
    // Handle cache invalidation if requested
    if (invalidate_cache) {
        const invalidate_cmd = b.addSystemCommand(&[_][]const u8{
            "bash", "-c",
            b.fmt(
                \\echo "💥 Force invalidating compilation cache..."
                \\rm -rf {s}
                \\echo "✅ Cache invalidated - fresh build will be performed"
            , .{cache_dir})
        });
        exe.step.dependOn(&invalidate_cmd.step);
        exe_stable.step.dependOn(&invalidate_cmd.step);
    }
    
    // Linker script generation step
    const generate_linker_scripts_step = b.step("generate-linker-scripts", "Generate linker scripts for embedded targets");
    const generate_scripts_cmd = b.addWriteFiles();
    generate_scripts_cmd.step.name = "Generate embedded linker scripts";
    
    // Generate the embedded ARM64 linker script
    linker_manager.generateEmbeddedARM64LinkerScript() catch |err| {
        if (b.verbose) {
            std.debug.print("⚠️ Failed to generate embedded linker script: {}\n", .{err});
        }
    };
    
    generate_linker_scripts_step.dependOn(&generate_scripts_cmd.step);
    
    // List available linker configurations step
    const list_linker_configs_step = b.step("list-linker-configs", "List available linker configurations");
    const list_configs_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "📋 Available Linker Configurations:"
            \\echo "  x86_64-unknown-linux-gnu    - Linux x86_64 (native/cross)"
            \\echo "  aarch64-unknown-linux-gnu   - Linux ARM64 (cross-compilation)"
            \\echo "  x86_64-pc-windows-gnu       - Windows x86_64 MinGW"
            \\echo "  x86_64-pc-windows-msvc      - Windows x86_64 MSVC"
            \\echo "  aarch64-pc-windows-gnu      - Windows ARM64 MinGW"
            \\echo "  x86_64-apple-darwin         - macOS Intel"
            \\echo "  aarch64-apple-darwin        - macOS Apple Silicon"
            \\echo "  wasm32-unknown-unknown      - WebAssembly"
            \\echo "  wasm32-wasi                 - WebAssembly with WASI"
            \\echo "  aarch64-unknown-none        - Embedded ARM64"
            \\echo ""
            \\echo "Current target: {s}"
            \\echo "Cross-compiling: {}"
        , .{ target_triple_str, is_cross_compile })
    });
    list_linker_configs_step.dependOn(&list_configs_cmd.step);
    
    // Linker configuration validation step
    const validate_linker_step = b.step("validate-linker", "Validate linker configuration for target");
    const validate_linker_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "🔍 Validating linker configuration for target: {s}"
            \\echo "✅ Target triple: {s}"
            \\echo "✅ Cross-compilation: {}"
            \\echo "✅ Linker config available: $(test -f linker_scripts/aarch64_embedded.ld && echo 'Yes' || echo 'System default')"
            \\echo "🎉 Linker validation complete"
        , .{ config.description, target_triple_str, is_cross_compile })
    });
    validate_linker_step.dependOn(&validate_linker_cmd.step);
    
    // Architecture mismatch validation step
    const validate_arch_step = b.step("validate-arch", "Validate binary architecture compatibility");
    const validate_arch_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "🔍 Validating binary architecture compatibility..."
            \\if [ -f ./zig-out/bin/cursed-zig ]; then
            \\  echo "📁 Binary file info:"
            \\  file ./zig-out/bin/cursed-zig
            \\  echo ""
            \\  echo "📚 Dynamic linker requirements:"
            \\  ldd ./zig-out/bin/cursed-zig | head -3 || echo "No dynamic linking detected"
            \\  echo ""
            \\  echo "🧪 Execution test:"
            \\  ./zig-out/bin/cursed-zig --version || echo "❌ Execution failed - architecture mismatch detected"
            \\  echo ""
            \\  echo "🔧 System compatibility:"
            \\  echo "  Target ABI: {s}" 
            \\  echo "  glibc available: $(test -f /lib64/ld-linux-x86-64.so.2 && echo 'Yes' || echo 'No')"
            \\  echo "  musl available: $(test -f /lib/ld-musl-x86_64.so.1 && echo 'Yes' || echo 'No')"
            \\else
            \\  echo "❌ Binary not found - run 'zig build' first"
            \\fi
        , .{target_triple_str})
    });
    validate_arch_cmd.step.dependOn(b.getInstallStep());
    validate_arch_step.dependOn(&validate_arch_cmd.step);
    
    // Build validation step  
    const validate_step = b.step("validate", "Validate build configuration and dependencies");
    const validate_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "🔍 Validating build configuration..."
            \\echo "✅ Build jobs: ${{NINJA_MAX_JOBS:-{d}}}"
            \\echo "✅ Zig version: $(zig version)"
            \\echo "✅ LLVM config: $(llvm-config-18 --version 2>/dev/null || echo 'Not found')"
            \\echo "✅ Target executable: $(ls -la zig-out/bin/cursed-zig 2>/dev/null || echo 'Not built')"
            \\echo "✅ Cache directory: {s} (enabled: {})"
            \\echo "✅ Target triple: {s}"
            \\echo "🎉 Build validation complete"
        , .{ optimal_jobs, cache_dir, enable_cache, target_triple_str })
    });
    validate_cmd.step.dependOn(b.getInstallStep());
    validate_step.dependOn(&validate_cmd.step);
    validate_step.dependOn(&validate_linker_cmd.step);
    validate_step.dependOn(&validate_arch_cmd.step);
    
    // Performance optimization steps
    const perf_optimize_step = b.step("perf-optimize", "Run performance optimization on CURSED programs");
    const perf_optimize_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "🚀 Running CURSED Performance Optimization..."
            \\if [ -f ./scripts/run_performance_optimization.sh ]; then
            \\  ./scripts/run_performance_optimization.sh optimize --level=standard "${{@:-examples/basic_test.csd}}"
            \\else
            \\  echo "⚠️ Performance optimization script not found"
            \\  ./zig-out/bin/cursed-perf optimize --level=standard "${{@:-examples/basic_test.csd}}"
            \\fi
        , .{})
    });
    perf_optimize_cmd.step.dependOn(b.getInstallStep());
    perf_optimize_step.dependOn(&perf_optimize_cmd.step);
    
    const perf_profile_step = b.step("perf-profile", "Run performance profiling on CURSED programs");
    const perf_profile_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "📊 Running CURSED Performance Profiling..."
            \\if [ -f ./scripts/run_performance_optimization.sh ]; then
            \\  ./scripts/run_performance_optimization.sh profile --format=text "${{@:-examples/basic_test.csd}}"
            \\else
            \\  echo "⚠️ Performance optimization script not found"
            \\  ./zig-out/bin/cursed-perf profile --format=text "${{@:-examples/basic_test.csd}}"
            \\fi
        , .{})
    });
    perf_profile_cmd.step.dependOn(b.getInstallStep());
    perf_profile_step.dependOn(&perf_profile_cmd.step);
    
    const perf_benchmark_step = b.step("perf-benchmark", "Run performance benchmarks");
    const perf_benchmark_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "🏃 Running CURSED Performance Benchmarks..."
            \\if [ -f ./scripts/run_performance_optimization.sh ]; then
            \\  ./scripts/run_performance_optimization.sh benchmark all
            \\else
            \\  echo "⚠️ Performance optimization script not found"
            \\  ./zig-out/bin/cursed-perf benchmark all
            \\fi
        , .{})
    });
    perf_benchmark_cmd.step.dependOn(b.getInstallStep());
    perf_benchmark_step.dependOn(&perf_benchmark_cmd.step);
    
    const perf_comprehensive_step = b.step("perf-comprehensive", "Run comprehensive performance optimization suite");
    const perf_comprehensive_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        b.fmt(
            \\echo "🎯 Running Comprehensive Performance Optimization Suite..."
            \\if [ -f ./scripts/run_performance_optimization.sh ]; then
            \\  ./scripts/run_performance_optimization.sh comprehensive "${{@:-examples/basic_test.csd}}"
            \\else
            \\  echo "⚠️ Performance optimization script not found"
            \\  echo "Running individual performance tools..."
            \\  ./zig-out/bin/cursed-perf optimize --level=aggressive "${{@:-examples/basic_test.csd}}"
            \\  ./zig-out/bin/cursed-perf profile --format=json "${{@:-examples/basic_test.csd}}"
            \\  ./zig-out/bin/cursed-perf benchmark all
            \\fi
        , .{})
    });
    perf_comprehensive_cmd.step.dependOn(b.getInstallStep());
    perf_comprehensive_step.dependOn(&perf_comprehensive_cmd.step);
    
    // Performance help step
    const perf_help_step = b.step("perf-help", "Show performance optimization help");
    const perf_help_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        \\echo "🚀 CURSED Performance Optimization Suite"
        \\echo "========================================"
        \\echo ""
        \\echo "Available performance optimization commands:"
        \\echo "  zig build perf                  - Run performance CLI"
        \\echo "  zig build perf-optimize         - Apply performance optimizations"
        \\echo "  zig build perf-profile          - Profile program execution"
        \\echo "  zig build perf-benchmark        - Run performance benchmarks"
        \\echo "  zig build perf-comprehensive    - Run full optimization suite"
        \\echo "  zig build perf-help             - Show this help"
        \\echo ""
        \\echo "Performance optimization script (if available):"
        \\echo "  ./scripts/run_performance_optimization.sh --help"
        \\echo ""
        \\echo "Direct CLI usage:"
        \\echo "  ./zig-out/bin/cursed-perf --help"
    });
    perf_help_step.dependOn(&perf_help_cmd.step);
    
    // CURSED compilation help step
    const cursed_help_step = b.step("cursed-help", "Show CURSED compilation help");
    const cursed_help_cmd = b.addSystemCommand(&[_][]const u8{
        "bash", "-c",
        \\echo "📋 CURSED Compilation System"
        \\echo "============================="
        \\echo ""
        \\echo "CURSED compilation commands:"
        \\echo "  zig build compile-cursed        - Compile individual .csd files"
        \\echo "  zig build examples              - Compile all CURSED example files"
        \\echo "  zig build test-cursed           - Compile and test CURSED programs"
        \\echo "  zig build cursed-help           - Show this help"
        \\echo ""
        \\echo "Available .csd example files:"
        \\echo "  basic_test.csd                  - Basic language features test"
        \\echo "  array_test.csd                  - Array operations test"
        \\echo "  comprehensive_test.csd          - Comprehensive language test"
        \\echo "  arithmetic_test.csd             - Arithmetic operations test"
        \\echo "  control_structures_test.csd     - Control flow test"
        \\echo "  comprehensive_stdlib_test.csd   - Standard library test"
        \\echo ""
        \\echo "Direct CURSED compiler usage:"
        \\echo "  ./zig-out/bin/cursed-zig file.csd           - Interpret file"
        \\echo "  ./zig-out/bin/cursed-zig --compile file.csd - Compile to executable"
        \\echo "  ./zig-out/bin/cursed-zig --help             - Show compiler help"
        \\echo ""
        \\echo "Build workflow:"
        \\echo "  1. zig build                    - Build CURSED compiler"
        \\echo "  2. zig build examples           - Compile .csd files to executables"
        \\echo "  3. zig build test-cursed        - Run compiled CURSED programs"
    });
    cursed_help_step.dependOn(&cursed_help_cmd.step);
}
