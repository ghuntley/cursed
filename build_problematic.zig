const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Check if LLVM is available 
    const enable_llvm = b.option(bool, "enable-llvm", "Enable LLVM backend") orelse true;

    // WASM build options
    const wasm_backend = b.option(bool, "wasm-backend", "Build WASM backend") orelse false;
    const wasm_target = b.option([]const u8, "wasm-target", "WASM target (browser, wasi, freestanding)") orelse "browser";

    // Determine which main file to use based on target
    const main_file = if (target.result.cpu.arch == .wasm32 or wasm_backend) 
        b.path("src-zig/wasm_main.zig") 
    else 
        b.path("src-zig/main_ast_enabled.zig");

    // Create main executable
    const exe = b.addExecutable(.{
        .name = if (target.result.cpu.arch == .wasm32) "cursed-wasm" else "cursed-zig", 
        .root_module = b.createModule(.{
            .root_source_file = main_file,
            .target = target,
            .optimize = optimize,
        }),
    });

    // WASM-specific configuration
    if (target.result.cpu.arch == .wasm32 or wasm_backend) {
        // Configure for WebAssembly
        exe.root_module.addCMacro("CURSED_WASM_BUILD", "1");
        exe.root_module.addCMacro("CURSED_TARGET_WASM", "1");
        
        // Add WASM target type
        if (std.mem.eql(u8, wasm_target, "wasi")) {
            exe.root_module.addCMacro("CURSED_WASM_WASI", "1");
            exe.linkLibC(); // WASI needs libc
        } else if (std.mem.eql(u8, wasm_target, "freestanding")) {
            exe.root_module.addCMacro("CURSED_WASM_FREESTANDING", "1");
        } else {
            exe.root_module.addCMacro("CURSED_WASM_BROWSER", "1");
        }
        
        if (b.verbose) {
            std.debug.print("✅ Building for WebAssembly target: {s}\n", .{wasm_target});
        }
    } else {
        // Try standard system library linking for native builds
        exe.linkLibC();
    }
    
    // Try to link LLVM if available
    if (enable_llvm) {
        // Try different LLVM versions and paths
        const llvm_paths = [_][]const u8{
            "/usr/lib/llvm-18",
            "/usr/lib/llvm-16",
            "/usr/lib/llvm-15", 
            "/usr/lib/llvm-14",
            "/opt/homebrew/opt/llvm",
            "/usr/local/opt/llvm",
        };
        
        var llvm_found = false;
        for (llvm_paths) |llvm_path| {
            if (std.fs.cwd().access(llvm_path, .{})) |_| {
                const include_path = b.fmt("{s}/include", .{llvm_path});
                const lib_path = b.fmt("{s}/lib", .{llvm_path});
                exe.addIncludePath(.{ .cwd_relative = include_path });
                exe.addLibraryPath(.{ .cwd_relative = lib_path });
                llvm_found = true;
                
                // Also add LLVM C headers path for LLVM 18
                if (std.mem.eql(u8, llvm_path, "/usr/lib/llvm-18")) {
                    exe.addIncludePath(.{ .cwd_relative = "/usr/include/llvm-c-18" });
                }
                break;
            } else |_| continue;
        }
        
        if (llvm_found) {
            exe.linkSystemLibrary("LLVM");
            exe.root_module.addCMacro("CURSED_ENABLE_LLVM", "1");
            if (b.verbose) {
                std.debug.print("✅ LLVM support enabled\n", .{});
            }
        } else {
            exe.root_module.addCMacro("CURSED_DISABLE_LLVM", "1");
            if (b.verbose) {
                std.debug.print("⚠️  LLVM not found, using stub implementation\n", .{});
            }
        }
    } else {
        exe.root_module.addCMacro("CURSED_DISABLE_LLVM", "1");
    }
    
    b.installArtifact(exe);

    // Advanced LSP Server (temporarily disabled - API compatibility issue)
    // const advanced_lsp_server = b.addExecutable(.{
    //     .name = "cursed-lsp-advanced",
    //     .root_module = b.createModule(.{
    //         .root_source_file = b.path("src-zig/advanced_lsp_server.zig"),
    //         .target = target,
    //         .optimize = optimize,
    //     }),
    // });
    // advanced_lsp_server.linkLibC();
    // b.installArtifact(advanced_lsp_server);

    // Simple LSP Server (working with current API)
    const lsp_server = b.addExecutable(.{
        .name = "cursed-lsp",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/lsp_simple_working.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    lsp_server.linkLibC();
    b.installArtifact(lsp_server);

    // Simple Development Tools (working versions)
    
    // Code Formatter
    const formatter = b.addExecutable(.{
        .name = "cursed-fmt", 
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/fmt_simple.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    formatter.linkLibC();
    b.installArtifact(formatter);

    // Code Linter  
    const linter = b.addExecutable(.{
        .name = "cursed-lint",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/lint_simple.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    linter.linkLibC();
    b.installArtifact(linter);

    // Interactive Debugger
    const debug_tool = b.addExecutable(.{
        .name = "cursed-debug",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/debug_simple.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    debug_tool.linkLibC();
    b.installArtifact(debug_tool);

    // Package Manager
    const pkg_manager = b.addExecutable(.{
        .name = "cursed-pkg",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/cursed_pkg.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });
    pkg_manager.linkLibC();
    b.installArtifact(pkg_manager);

    const run_step = b.step("run", "Run the CURSED compiler");
    const run_cmd = b.addRunArtifact(exe);
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    run_step.dependOn(&run_cmd.step);
    
    // Run LSP server (temporarily disabled - API compatibility issue)
    // const run_lsp_step = b.step("run-lsp", "Run the CURSED LSP server");
    // const run_lsp_cmd = b.addRunArtifact(advanced_lsp_server);
    // run_lsp_step.dependOn(&run_lsp_cmd.step);
    
    // WASM build targets
    const wasm_browser_step = b.step("wasm-browser", "Build for WebAssembly (browser target)");
    const wasm_browser = b.addExecutable(.{
        .name = "cursed-browser",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/wasm_main.zig"),
            .target = b.resolveTargetQuery(.{ 
                .cpu_arch = .wasm32, 
                .os_tag = .freestanding,
            }),
            .optimize = optimize,
        }),
    });
    wasm_browser.root_module.addCMacro("CURSED_WASM_BROWSER", "1");
    const wasm_browser_install = b.addInstallArtifact(wasm_browser, .{});
    wasm_browser_step.dependOn(&wasm_browser_install.step);
    
    const wasm_wasi_step = b.step("wasm-wasi", "Build for WebAssembly (WASI target)"); 
    const wasm_wasi = b.addExecutable(.{
        .name = "cursed-wasi",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/wasm_main.zig"),
            .target = b.resolveTargetQuery(.{
                .cpu_arch = .wasm32,
                .os_tag = .wasi,
            }),
            .optimize = optimize,
        }),
    });
    wasm_wasi.root_module.addCMacro("CURSED_WASM_WASI", "1");
    wasm_wasi.linkLibC();
    const wasm_wasi_install = b.addInstallArtifact(wasm_wasi, .{});
    wasm_wasi_step.dependOn(&wasm_wasi_install.step);
    
    // WASM library for JavaScript integration
    const wasm_lib_step = b.step("wasm-lib", "Build CURSED as WASM library");
    const wasm_lib = b.addExecutable(.{
        .name = "cursed-lib",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src-zig/wasm_lib.zig"),
            .target = b.resolveTargetQuery(.{
                .cpu_arch = .wasm32,
                .os_tag = .freestanding,
            }),
            .optimize = .ReleaseSmall, // Optimize for size
        }),
    });
    wasm_lib.root_module.addCMacro("CURSED_WASM_LIB", "1");
    const wasm_lib_install = b.addInstallArtifact(wasm_lib, .{});
    wasm_lib_step.dependOn(&wasm_lib_install.step);
    
    // Convenience step to build all WASM targets
    const wasm_all_step = b.step("wasm-all", "Build all WebAssembly targets");
    wasm_all_step.dependOn(wasm_browser_step);
    wasm_all_step.dependOn(wasm_wasi_step);
    wasm_all_step.dependOn(wasm_lib_step);
    
    // Enterprise toolkit examples
    if (b.option(bool, "enterprise-examples", "Build enterprise examples") orelse false) {
        const enterprise_microservices = b.addExecutable(.{
            .name = "enterprise-microservices",
            .root_module = b.createModule(.{
                .root_source_file = b.path("examples/enterprise_microservices.csd"),
                .target = target,
                .optimize = optimize,
            }),
        });
        
        const enterprise_benchmark = b.addExecutable(.{
            .name = "enterprise-benchmark", 
            .root_module = b.createModule(.{
                .root_source_file = b.path("examples/enterprise_performance_benchmark.csd"),
                .target = target,
                .optimize = optimize,
            }),
        });
        
        b.installArtifact(enterprise_microservices);
        b.installArtifact(enterprise_benchmark);
        
        if (b.verbose) {
            std.debug.print("✅ Enterprise examples built\n", .{});
        }
    }
}
