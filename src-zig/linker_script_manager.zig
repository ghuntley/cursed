const std = @import("std");
const target_triple_normalization = @import("target_triple_normalization.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

/// Cross-compilation linker script selection manager for Zig build system
pub const LinkerScriptManager = struct {
    allocator: Allocator,
    normalizer: target_triple_normalization.TargetTripleNormalizer,
    project_root: []const u8,
    
    /// Linker script configuration for different platforms
    pub const LinkerConfig = struct {
        script_path: ?[]const u8,
        linker_args: []const []const u8,
        required_libs: []const []const u8,
        memory_layout: ?MemoryLayout,
        
        pub const MemoryLayout = struct {
            flash_start: u64,
            flash_size: u64,
            ram_start: u64,
            ram_size: u64,
        };
    };
    
    /// Platform-specific linker configurations
    const PlatformConfigs = std.StaticStringMap(LinkerConfig).initComptime(.{
        // Linux x86_64 - standard system linker
        .{ "x86_64-unknown-linux-gnu", LinkerConfig{
            .script_path = null, // Use system default
            .linker_args = &[_][]const u8{
                "-Wl,--as-needed",
                "-Wl,--gc-sections",
                "-Wl,--strip-debug",
            },
            .required_libs = &[_][]const u8{ "c", "m", "pthread" },
            .memory_layout = null,
        }},
        
        // Linux ARM64 - cross-compilation setup
        .{ "aarch64-unknown-linux-gnu", LinkerConfig{
            .script_path = null, // Use system default
            .linker_args = &[_][]const u8{
                "-Wl,--as-needed",
                "-Wl,--gc-sections",
                "-Wl,--fix-cortex-a53-843419", // ARM64 CPU errata fix
                "-Wl,--fix-cortex-a53-835769",
            },
            .required_libs = &[_][]const u8{ "c", "m", "pthread" },
            .memory_layout = null,
        }},
        
        // Windows x86_64 MinGW
        .{ "x86_64-pc-windows-gnu", LinkerConfig{
            .script_path = null, // Use MinGW default
            .linker_args = &[_][]const u8{
                "-Wl,--gc-sections",
                "-Wl,--strip-debug",
                "-static-libgcc",
                "-static-libstdc++",
                "-Wl,-Bstatic",
                "-lpthread",
                "-Wl,-Bdynamic",
            },
            .required_libs = &[_][]const u8{ "mingw32", "ws2_32", "kernel32" },
            .memory_layout = null,
        }},
        
        // Windows x86_64 MSVC
        .{ "x86_64-pc-windows-msvc", LinkerConfig{
            .script_path = null, // Use MSVC default
            .linker_args = &[_][]const u8{
                "/SUBSYSTEM:CONSOLE",
                "/OPT:REF",
                "/OPT:ICF",
                "/DEBUG:NONE",
            },
            .required_libs = &[_][]const u8{ "msvcrt", "ws2_32", "kernel32" },
            .memory_layout = null,
        }},
        
        // Windows ARM64
        .{ "aarch64-pc-windows-gnu", LinkerConfig{
            .script_path = null,
            .linker_args = &[_][]const u8{
                "-Wl,--gc-sections",
                "-static-libgcc",
                "-Wl,-Bstatic",
                "-lpthread",
                "-Wl,-Bdynamic",
            },
            .required_libs = &[_][]const u8{ "mingw32", "ws2_32", "kernel32" },
            .memory_layout = null,
        }},
        
        // macOS x86_64
        .{ "x86_64-apple-darwin", LinkerConfig{
            .script_path = null, // Use system default
            .linker_args = &[_][]const u8{
                "-Wl,-dead_strip",
                "-Wl,-S", // Strip debug symbols
            },
            .required_libs = &[_][]const u8{ "System" },
            .memory_layout = null,
        }},
        
        // macOS ARM64 (Apple Silicon)
        .{ "aarch64-apple-darwin", LinkerConfig{
            .script_path = null, // Use system default
            .linker_args = &[_][]const u8{
                "-Wl,-dead_strip",
                "-Wl,-S", // Strip debug symbols
                "-arch", "arm64",
            },
            .required_libs = &[_][]const u8{ "System" },
            .memory_layout = null,
        }},
        
        // Linux x86_64 - musl libc (Alpine Linux, embedded)
        .{ "x86_64-unknown-linux-musl", LinkerConfig{
            .script_path = null, // Use system default
            .linker_args = &[_][]const u8{
                "-Wl,--as-needed",
                "-Wl,--gc-sections",
                "-Wl,--strip-debug",
                "-static",  // musl often used for static linking
                "-Wl,--no-dynamic-linker",
            },
            .required_libs = &[_][]const u8{ "c" }, // musl provides integrated threading
            .memory_layout = null,
        }},
        
        // Linux ARM64 - musl libc (Alpine Linux ARM64, embedded)
        .{ "aarch64-unknown-linux-musl", LinkerConfig{
            .script_path = null,
            .linker_args = &[_][]const u8{
                "-Wl,--as-needed",
                "-Wl,--gc-sections",
                "-Wl,--fix-cortex-a53-843419", // ARM64 CPU errata fix
                "-Wl,--fix-cortex-a53-835769",
                "-static",  // musl static linking for embedded
                "-Wl,--no-dynamic-linker",
            },
            .required_libs = &[_][]const u8{ "c" },
            .memory_layout = null,
        }},

        // WebAssembly
        .{ "wasm32-unknown-unknown", LinkerConfig{
            .script_path = null,
            .linker_args = &[_][]const u8{
                "--strip-debug",
                "--gc-sections",
                "--allow-undefined",
                "--export-dynamic",
                "--import-memory",
                "--initial-memory=1048576", // 1MB initial memory
                "--max-memory=16777216",    // 16MB max memory
            },
            .required_libs = &[_][]const u8{},
            .memory_layout = LinkerConfig.MemoryLayout{
                .flash_start = 0x0,
                .flash_size = 0x1000000,   // 16MB
                .ram_start = 0x10000,
                .ram_size = 0xF0000,       // ~960KB
            },
        }},
        
        // WebAssembly with WASI
        .{ "wasm32-unknown-wasi", LinkerConfig{
            .script_path = null,
            .linker_args = &[_][]const u8{
                "--strip-debug",
                "--gc-sections",
                "--allow-undefined",
                "--export-dynamic",
                "--import-memory",
                "--initial-memory=2097152", // 2MB initial memory
                "--max-memory=33554432",    // 32MB max memory
            },
            .required_libs = &[_][]const u8{},
            .memory_layout = LinkerConfig.MemoryLayout{
                .flash_start = 0x0,
                .flash_size = 0x2000000,   // 32MB
                .ram_start = 0x20000,
                .ram_size = 0x1E0000,      // ~1.875MB
            },
        }},
        
        // WebAssembly with WASI (alternative format)
        .{ "wasm32-wasi", LinkerConfig{
            .script_path = null,
            .linker_args = &[_][]const u8{
                "--strip-debug",
                "--gc-sections",
                "--allow-undefined",
                "--export-dynamic",
                "--import-memory",
                "--initial-memory=2097152", // 2MB initial memory
                "--max-memory=33554432",    // 32MB max memory
            },
            .required_libs = &[_][]const u8{},
            .memory_layout = LinkerConfig.MemoryLayout{
                .flash_start = 0x0,
                .flash_size = 0x2000000,   // 32MB
                .ram_start = 0x20000,
                .ram_size = 0x1E0000,      // ~1.875MB
            },
        }},
        
        // Embedded ARM (generic)
        .{ "aarch64-unknown-none", LinkerConfig{
            .script_path = "linker_scripts/aarch64_embedded.ld",
            .linker_args = &[_][]const u8{
                "-Wl,--gc-sections",
                "-Wl,--print-memory-usage",
                "-nostdlib",
                "-ffreestanding",
            },
            .required_libs = &[_][]const u8{},
            .memory_layout = LinkerConfig.MemoryLayout{
                .flash_start = 0x8000000,  // 128MB
                .flash_size = 0x200000,    // 2MB
                .ram_start = 0x20000000,   // 512MB
                .ram_size = 0x40000,       // 256KB
            },
        }},
    });
    
    /// Default fallback configuration for unknown targets
    const DefaultConfig = LinkerConfig{
        .script_path = null,
        .linker_args = &[_][]const u8{
            "-Wl,--gc-sections",
        },
        .required_libs = &[_][]const u8{},
        .memory_layout = null,
    };
    
    pub fn init(allocator: Allocator, project_root: []const u8) LinkerScriptManager {
        return LinkerScriptManager{
            .allocator = allocator,
            .normalizer = target_triple_normalization.TargetTripleNormalizer.init(allocator),
            .project_root = project_root,
        };
    }
    
    pub fn deinit(self: *LinkerScriptManager) void {
        self.normalizer.deinit(self.allocator);
    }
    
    /// Get linker configuration for a target triple
    pub fn getLinkerConfig(self: *LinkerScriptManager, target_triple: []const u8) !LinkerConfig {
        // Normalize the target triple first
        const normalized = try self.normalizer.normalizeTriple(target_triple);
        const canonical_triple = try normalized.toCanonicalString(self.allocator);
        defer self.allocator.free(canonical_triple);
        
        // Look up platform-specific configuration
        if (PlatformConfigs.get(canonical_triple)) |config| {
            return config;
        }
        
        // Generate dynamic configuration based on target properties
        return self.generateDynamicConfig(normalized);
    }
    
    /// Generate dynamic linker configuration for targets not in static map
    fn generateDynamicConfig(self: *LinkerScriptManager, triple: target_triple_normalization.TargetTripleNormalizer.NormalizedTriple) LinkerConfig {
        var args = std.ArrayList([]const u8){};
        defer args.deinit(self.allocator);
        var libs = std.ArrayList([]const u8){};
        defer libs.deinit(self.allocator);
        
        // Base optimization flags for all targets
        args.append(self.allocator, "-Wl,--gc-sections") catch {};
        
        if (triple.isWindows()) {
            args.append(self.allocator, "-static-libgcc") catch {};
            libs.append(self.allocator, "ws2_32") catch {};
            libs.append(self.allocator, "kernel32") catch {};
        } else if (triple.isLinux()) {
            args.append(self.allocator, "-Wl,--as-needed") catch {};
            libs.append(self.allocator, "c") catch {};
            
            // Handle musl vs glibc differences
            if (triple.abi != null and std.mem.eql(u8, triple.abi.?, "musl")) {
                // musl libc: static linking preferred, integrated threading
                args.append(self.allocator, "-static") catch {};
                args.append(self.allocator, "-Wl,--no-dynamic-linker") catch {};
                // musl doesn't need separate libm or pthread
            } else {
                // glibc: dynamic linking, separate math and threading libs
                libs.append(self.allocator, "m") catch {};
                if (triple.supportsThreading()) {
                    libs.append(self.allocator, "pthread") catch {};
                }
            }
        } else if (triple.isApple()) {
            args.append(self.allocator, "-Wl,-dead_strip") catch {};
            libs.append(self.allocator, "System") catch {};
        }
        
        // ARM64-specific linker flags
        if (triple.isARM64() and triple.isLinux()) {
            args.append(self.allocator, "-Wl,--fix-cortex-a53-843419") catch {};
            args.append(self.allocator, "-Wl,--fix-cortex-a53-835769") catch {};
        }
        
        return LinkerConfig{
            .script_path = null,
            .linker_args = args.toOwnedSlice(self.allocator) catch &[_][]const u8{},
            .required_libs = libs.toOwnedSlice(self.allocator) catch &[_][]const u8{},
            .memory_layout = null,
        };
    }
    
    /// Apply linker configuration to a Zig build executable
    pub fn applyLinkerConfig(
        self: *LinkerScriptManager, 
        exe: *std.Build.Step.Compile, 
        target_triple: []const u8,
        b: *std.Build
    ) !void {
        const config = try self.getLinkerConfig(target_triple);
        const normalized = try self.normalizer.normalizeTriple(target_triple);
        
        // Apply custom linker script if specified
        if (config.script_path) |script_path| {
            const absolute_script_path = try std.fs.path.join(self.allocator, &[_][]const u8{ 
                self.project_root, script_path 
            });
            defer self.allocator.free(absolute_script_path);
            
            // Verify script exists
            std.fs.cwd().access(absolute_script_path, .{}) catch |err| {
                if (b.verbose) {
                    std.debug.print("⚠️ Linker script not found: {s} (error: {s})\n", .{ absolute_script_path, err });
                }
                return;
            };
            
            exe.addObjectFile(.{ .cwd_relative = absolute_script_path });
            
            if (b.verbose) {
                std.debug.print("📋 Applied linker script: {s}\n", .{script_path});
            }
        }
        
        // Apply platform-specific linker arguments
        for (config.linker_args) |arg| {
            if (normalized.isWindows() and std.mem.startsWith(u8, arg, "/")) {
                // MSVC-style argument
                exe.root_module.addCMacro("LINK_ARG", b.fmt("\"{s}\"", .{arg}));
            } else if (std.mem.startsWith(u8, arg, "-Wl,")) {
                // GNU linker argument
                exe.root_module.addCMacro("GNU_LINK_ARG", b.fmt("\"{s}\"", .{arg}));
            } else {
                // Generic linker argument
                exe.root_module.addCMacro("LINKER_FLAG", b.fmt("\"{s}\"", .{arg}));
            }
        }
        
        // Link required system libraries
        for (config.required_libs) |lib| {
            exe.linkSystemLibrary(lib);
            
            if (b.verbose) {
                std.debug.print("🔗 Linked system library: {s}\n", .{lib});
            }
        }
        
        // Set memory layout macros for embedded targets
        if (config.memory_layout) |layout| {
            exe.root_module.addCMacro("FLASH_START", b.fmt("0x{X}", .{layout.flash_start}));
            exe.root_module.addCMacro("FLASH_SIZE", b.fmt("0x{X}", .{layout.flash_size}));
            exe.root_module.addCMacro("RAM_START", b.fmt("0x{X}", .{layout.ram_start}));
            exe.root_module.addCMacro("RAM_SIZE", b.fmt("0x{X}", .{layout.ram_size}));
            
            if (b.verbose) {
                std.debug.print("🗺️ Applied memory layout: Flash=0x{X}+0x{X}, RAM=0x{X}+0x{X}\n", .{
                    layout.flash_start, layout.flash_size, layout.ram_start, layout.ram_size
                });
            }
        }
        
        // Apply target-specific optimizations
        if (normalized.isARM64()) {
            exe.root_module.addCMacro("TARGET_ARM64", "1");
            if (normalized.isApple()) {
                exe.root_module.addCMacro("TARGET_APPLE_SILICON", "1");
            }
        }
        
        if (normalized.isWindows()) {
            exe.root_module.addCMacro("TARGET_WINDOWS", "1");
            if (normalized.abi != null and std.mem.eql(u8, normalized.abi.?, "msvc")) {
                exe.root_module.addCMacro("TARGET_MSVC", "1");
            } else {
                exe.root_module.addCMacro("TARGET_MINGW", "1");
            }
        }
        
        if (normalized.isWebAssembly()) {
            exe.root_module.addCMacro("TARGET_WASM", "1");
            if (std.mem.eql(u8, normalized.os, "wasi")) {
                exe.root_module.addCMacro("TARGET_WASI", "1");
            }
        }
        
        if (b.verbose) {
            const canonical_triple = try normalized.toCanonicalString(self.allocator);
            defer self.allocator.free(canonical_triple);
            std.debug.print("🎯 Applied linker configuration for: {s}\n", .{canonical_triple});
        }
    }
    
    /// Create linker script directories if they don't exist
    pub fn createLinkerScriptDirectories(self: *LinkerScriptManager) !void {
        const linker_scripts_dir = try std.fs.path.join(self.allocator, &[_][]const u8{ 
            self.project_root, "linker_scripts" 
        });
        defer self.allocator.free(linker_scripts_dir);
        
        std.fs.cwd().makePath(linker_scripts_dir) catch |err| switch (err) {
            error.PathAlreadyExists => {}, // Directory already exists, this is fine
            else => return err,
        };
    }
    
    /// Generate a default embedded ARM64 linker script
    pub fn generateEmbeddedARM64LinkerScript(self: *LinkerScriptManager) !void {
        try self.createLinkerScriptDirectories();
        
        const script_path = try std.fs.path.join(self.allocator, &[_][]const u8{ 
            self.project_root, "linker_scripts", "aarch64_embedded.ld" 
        });
        defer self.allocator.free(script_path);
        
        const script_content =
            \\/* ARM64 Embedded Linker Script for CURSED Compiler */
            \\
            \\MEMORY
            \\{
            \\    FLASH (rx) : ORIGIN = 0x8000000, LENGTH = 2M
            \\    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 256K
            \\}
            \\
            \\ENTRY(_start)
            \\
            \\SECTIONS
            \\{
            \\    .text : {
            \\        KEEP(*(.vector_table))
            \\        *(.text*)
            \\        *(.rodata*)
            \\        . = ALIGN(4);
            \\    } > FLASH
            \\
            \\    .data : {
            \\        _data_start = .;
            \\        *(.data*)
            \\        . = ALIGN(4);
            \\        _data_end = .;
            \\    } > RAM AT > FLASH
            \\
            \\    _data_load_start = LOADADDR(.data);
            \\
            \\    .bss : {
            \\        _bss_start = .;
            \\        *(.bss*)
            \\        *(COMMON)
            \\        . = ALIGN(4);
            \\        _bss_end = .;
            \\    } > RAM
            \\
            \\    .heap : {
            \\        _heap_start = .;
            \\        . = . + 32K;
            \\        _heap_end = .;
            \\    } > RAM
            \\
            \\    .stack : {
            \\        . = . + 8K;
            \\        _stack_top = .;
            \\    } > RAM
            \\
            \\    /DISCARD/ : {
            \\        *(.ARM.exidx*)
            \\        *(.ARM.extab*)
            \\    }
            \\}
        ;
        
        const file = std.fs.cwd().createFile(script_path, .{}) catch |err| {
            std.debug.print("Failed to create linker script: {s}\n", .{err});
            return;
        };
        defer file.close();
        
        try file.writer().writeAll(script_content);
        std.debug.print("✅ Generated embedded ARM64 linker script: {s}\n", .{script_path});
    }
    
    /// Validate linker configuration for a target
    pub fn validateLinkerConfig(self: *LinkerScriptManager, target_triple: []const u8) !bool {
        const config = try self.getLinkerConfig(target_triple);
        
        // Check if custom linker script exists
        if (config.script_path) |script_path| {
            const absolute_script_path = try std.fs.path.join(self.allocator, &[_][]const u8{ 
                self.project_root, script_path 
            });
            defer self.allocator.free(absolute_script_path);
            
            std.fs.cwd().access(absolute_script_path, .{}) catch {
                return false; // Script doesn't exist
            };
        }
        
        // Validate that target is supported for cross-compilation
        return self.normalizer.validateForCrossCompilation(target_triple);
    }
    
    /// Get available linker script configurations
    pub fn getAvailableConfigurations(self: *LinkerScriptManager) []const []const u8 {
        _ = self;
        return comptime blk: {
            var configs: [PlatformConfigs.kvs.len][]const u8 = undefined;
            for (PlatformConfigs.kvs, 0..) |kv, i| {
                configs[i] = kv.key;
            }
            break :blk &configs;
        };
    }
    
    /// Print linker configuration info for debugging
    pub fn printLinkerConfigInfo(self: *LinkerScriptManager, target_triple: []const u8, b: *std.Build) !void {
        if (!b.verbose) return;
        
        const config = try self.getLinkerConfig(target_triple);
        const normalized = try self.normalizer.normalizeTriple(target_triple);
        const canonical_triple = try normalized.toCanonicalString(self.allocator);
        defer self.allocator.free(canonical_triple);
        
        std.debug.print("\n📋 Linker Configuration for {s}:\n", .{canonical_triple});
        
        if (config.script_path) |script| {
            std.debug.print("  📜 Custom script: {s}\n", .{script});
        } else {
            std.debug.print("  📜 Using system default linker script\n", .{});
        }
        
        std.debug.print("  🔧 Linker args ({d}):\n", .{config.linker_args.len});
        for (config.linker_args) |arg| {
            std.debug.print("    - {s}\n", .{arg});
        }
        
        std.debug.print("  📚 Required libs ({d}):\n", .{config.required_libs.len});
        for (config.required_libs) |lib| {
            std.debug.print("    - {s}\n", .{lib});
        }
        
        if (config.memory_layout) |layout| {
            std.debug.print("  🗺️ Memory layout:\n", .{});
            std.debug.print("    Flash: 0x{X} + 0x{X} ({d} KB)\n", .{ 
                layout.flash_start, layout.flash_size, layout.flash_size / 1024 
            });
            std.debug.print("    RAM:   0x{X} + 0x{X} ({d} KB)\n", .{ 
                layout.ram_start, layout.ram_size, layout.ram_size / 1024 
            });
        }
        
        std.debug.print("\n", .{});
    }
};

// Tests
const testing = std.testing;

test "LinkerScriptManager basic functionality" {
    var manager = LinkerScriptManager.init(testing.allocator, "/test/project");
    defer manager.deinit(testing.allocator);
    
    // Test getting configuration for known targets
    const linux_config = try manager.getLinkerConfig("x86_64-unknown-linux-gnu");
    try testing.expect(linux_config.script_path == null); // Uses system default
    try testing.expect(linux_config.linker_args.len > 0);
    
    const windows_config = try manager.getLinkerConfig("x86_64-pc-windows-gnu");
    try testing.expect(windows_config.script_path == null);
    try testing.expect(windows_config.required_libs.len > 0);
}

test "LinkerScriptManager ARM64 configuration" {
    var manager = LinkerScriptManager.init(testing.allocator, "/test/project");
    defer manager.deinit(testing.allocator);
    
    // Test ARM64 Linux configuration
    const arm64_linux = try manager.getLinkerConfig("aarch64-unknown-linux-gnu");
    try testing.expect(arm64_linux.linker_args.len > 0);
    
    // Test ARM64 Apple configuration
    const arm64_apple = try manager.getLinkerConfig("aarch64-apple-darwin");
    try testing.expect(arm64_apple.required_libs.len > 0);
}

test "LinkerScriptManager WebAssembly configuration" {
    var manager = LinkerScriptManager.init(testing.allocator, "/test/project");
    defer manager.deinit(testing.allocator);
    
    // Test WASM configuration
    const wasm_config = try manager.getLinkerConfig("wasm32-unknown-unknown");
    try testing.expect(wasm_config.memory_layout != null);
    try testing.expect(wasm_config.memory_layout.?.flash_size > 0);
    
    // Test WASI configuration
    const wasi_config = try manager.getLinkerConfig("wasm32-wasi");
    try testing.expect(wasi_config.memory_layout != null);
}

test "LinkerScriptManager musl targets" {
    var manager = LinkerScriptManager.init(testing.allocator, "/test/project");
    defer manager.deinit(testing.allocator);
    
    // Test musl x86_64 configuration (Alpine Linux)
    const musl_x64_config = try manager.getLinkerConfig("x86_64-unknown-linux-musl");
    try testing.expect(musl_x64_config.script_path == null); // Uses system default
    try testing.expect(musl_x64_config.linker_args.len > 0);
    try testing.expect(std.mem.indexOf(u8, musl_x64_config.linker_args[3], "-static") != null); // static linking
    try testing.expect(musl_x64_config.required_libs.len == 1); // only 'c', no pthread/m needed
    
    // Test musl ARM64 configuration
    const musl_arm64_config = try manager.getLinkerConfig("aarch64-unknown-linux-musl");
    try testing.expect(musl_arm64_config.linker_args.len > 0);
    try testing.expect(musl_arm64_config.required_libs.len == 1); // only 'c'
}

test "LinkerScriptManager validation" {
    var manager = LinkerScriptManager.init(testing.allocator, "/test/project");
    defer manager.deinit(testing.allocator);
    
    // Test validation for supported targets
    try testing.expect(try manager.validateLinkerConfig("x86_64-unknown-linux-gnu"));
    try testing.expect(try manager.validateLinkerConfig("x86_64-unknown-linux-musl"));
    try testing.expect(try manager.validateLinkerConfig("aarch64-unknown-linux-musl"));
    try testing.expect(try manager.validateLinkerConfig("aarch64-apple-darwin"));
    try testing.expect(try manager.validateLinkerConfig("wasm32-unknown-unknown"));
}
