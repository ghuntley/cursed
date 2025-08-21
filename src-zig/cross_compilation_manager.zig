const std = @import("std");
const builtin = @import("builtin");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const print = std.debug.print;

const TargetTripleNormalizer = @import("target_triple_normalization.zig").TargetTripleNormalizer;

/// Comprehensive cross-compilation manager with ARM64 and Windows specialization
pub const CrossCompilationManager = struct {
    allocator: Allocator,
    normalizer: TargetTripleNormalizer,
    toolchain_cache: std.StringHashMap(ToolchainInfo),
    compilation_cache: std.StringHashMap(CompilationResult),
    
    pub const ToolchainInfo = struct {
        target_triple: []const u8,
        compiler_path: []const u8,
        linker_path: ?[]const u8,
        sysroot: ?[]const u8,
        library_paths: [][]const u8,
        include_paths: [][]const u8,
        available: bool,
        version: []const u8,
        
        pub fn deinit(self: *ToolchainInfo, allocator: Allocator) void {
            allocator.free(self.target_triple);
            allocator.free(self.compiler_path);
            if (self.linker_path) |path| allocator.free(path);
            if (self.sysroot) |path| allocator.free(path);
            for (self.library_paths) |path| allocator.free(path);
            allocator.free(self.library_paths);
            for (self.include_paths) |path| allocator.free(path);
            allocator.free(self.include_paths);
            allocator.free(self.version);
        }
    };
    
    pub const CompilationResult = struct {
        target_triple: []const u8,
        output_path: []const u8,
        success: bool,
        build_time_ms: u64,
        output_size_bytes: u64,
        warnings: [][]const u8,
        errors: [][]const u8,
        
        pub fn deinit(self: *CompilationResult, allocator: Allocator) void {
            allocator.free(self.target_triple);
            allocator.free(self.output_path);
            for (self.warnings) |warning| allocator.free(warning);
            allocator.free(self.warnings);
            for (self.errors) |error_msg| allocator.free(error_msg);
            allocator.free(self.errors);
        }
    };
    
    pub const CompilationOptions = struct {
        source_files: []const []const u8,
        output_name: []const u8,
        target_triple: []const u8,
        optimization_level: OptimizationLevel = .release_safe,
        link_mode: LinkMode = .dynamic,
        enable_lto: bool = false,
        enable_debug_info: bool = true,
        enable_sanitizers: bool = false,
        additional_flags: []const []const u8 = &.{},
        library_dependencies: []const []const u8 = &.{},
        include_directories: []const []const u8 = &.{},
        define_macros: []const []const u8 = &.{},
        
        pub const OptimizationLevel = enum {
            debug,
            release_safe,
            release_fast,
            release_small,
        };
        
        pub const LinkMode = enum {
            static,
            dynamic,
            pie,
        };
    };
    
    pub fn init(allocator: Allocator) CrossCompilationManager {
        return CrossCompilationManager{
            .allocator = allocator,
            .normalizer = TargetTripleNormalizer.init(allocator),
            .toolchain_cache = std.StringHashMap(ToolchainInfo).init(allocator),
            .compilation_cache = std.StringHashMap(CompilationResult).init(allocator),
        };
    }
    
    pub fn deinit(self: *CrossCompilationManager) void {
        // Clean up toolchain cache
        var toolchain_iter = self.toolchain_cache.iterator();
        while (toolchain_iter.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        self.toolchain_cache.deinit(allocator);
        
        // Clean up compilation cache
        var compilation_iter = self.compilation_cache.iterator();
        while (compilation_iter.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        self.compilation_cache.deinit(allocator);
        
        self.normalizer.deinit(allocator);
    }
    
    /// Discover and cache available toolchains for cross-compilation
    pub fn discoverToolchains(self: *CrossCompilationManager) !void {
        const target_priorities = [_][]const u8{
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
            "x86_64-apple-darwin",
            "aarch64-apple-darwin",
            "x86_64-pc-windows-gnu",
            "x86_64-pc-windows-msvc",
            "aarch64-pc-windows-gnu",
            "wasm32-unknown-unknown",
            "wasm32-wasi",
        };
        
        for (target_priorities) |target| {
            self.discoverToolchainForTarget(target) catch |err| {
                print("Warning: Failed to discover toolchain for {s}: {}\n", .{ target, err });
                continue;
            };
        }
    }
    
    /// Discover toolchain for a specific target
    fn discoverToolchainForTarget(self: *CrossCompilationManager, target: []const u8) !void {
        const normalized = try self.normalizer.normalizeTriple(target);
        
        var toolchain_info = ToolchainInfo{
            .target_triple = try self.allocator.dupe(u8, target),
            .compiler_path = "",
            .linker_path = null,
            .sysroot = null,
            .library_paths = &.{},
            .include_paths = &.{},
            .available = false,
            .version = "",
        };
        
        // Try to find appropriate compiler
        if (normalized.isWindows()) {
            try self.discoverWindowsToolchain(&toolchain_info, normalized);
        } else if (normalized.isApple()) {
            try self.discoverAppleToolchain(&toolchain_info, normalized);
        } else if (normalized.isLinux()) {
            try self.discoverLinuxToolchain(&toolchain_info, normalized);
        } else if (normalized.isWebAssembly()) {
            try self.discoverWasmToolchain(&toolchain_info, normalized);
        }
        
        try self.toolchain_cache.put(target, toolchain_info);
    }
    
    /// Discover Windows-specific toolchain (MinGW, MSVC, Clang)
    fn discoverWindowsToolchain(
        self: *CrossCompilationManager,
        toolchain: *ToolchainInfo,
        normalized: TargetTripleNormalizer.NormalizedTriple,
    ) !void {
        // Try different Windows toolchain approaches
        const potential_compilers = if (normalized.abi != null and std.mem.eql(u8, normalized.abi.?, "msvc"))
            [_][]const u8{ "cl", "clang-cl", "zig cc" }
        else
            [_][]const u8{ "x86_64-w64-mingw32-gcc", "aarch64-w64-mingw32-gcc", "clang", "zig cc" };
        
        for (potential_compilers) |compiler| {
            if (self.findExecutable(compiler)) |path| {
                toolchain.compiler_path = try self.allocator.dupe(u8, path);
                toolchain.available = true;
                
                // Get version information
                toolchain.version = try self.getCompilerVersion(path);
                
                // Set up Windows-specific paths
                if (std.mem.containsAtLeast(u8, compiler, 1, "mingw")) {
                    try self.setupMingwPaths(toolchain, normalized);
                } else if (std.mem.eql(u8, compiler, "cl") or std.mem.eql(u8, compiler, "clang-cl")) {
                    try self.setupMsvcPaths(toolchain);
                }
                
                break;
            }
        }
    }
    
    /// Discover Apple toolchain (Xcode, cross-compilation)
    fn discoverAppleToolchain(
        self: *CrossCompilationManager,
        toolchain: *ToolchainInfo,
        normalized: TargetTripleNormalizer.NormalizedTriple,
    ) !void {
        const potential_compilers = [_][]const u8{
            "clang",
            "xcrun clang",
            "zig cc",
        };
        
        for (potential_compilers) |compiler| {
            if (self.findExecutable(compiler)) |path| {
                toolchain.compiler_path = try self.allocator.dupe(u8, path);
                toolchain.available = true;
                toolchain.version = try self.getCompilerVersion(path);
                
                // Set up Apple SDK paths
                if (normalized.isARM64()) {
                    try self.setupAppleSiliconPaths(toolchain);
                } else {
                    try self.setupAppleIntelPaths(toolchain);
                }
                
                break;
            }
        }
    }
    
    /// Discover Linux toolchain (GCC, Clang, cross-compilers)
    fn discoverLinuxToolchain(
        self: *CrossCompilationManager,
        toolchain: *ToolchainInfo,
        normalized: TargetTripleNormalizer.NormalizedTriple,
    ) !void {
        const arch_prefix = if (normalized.isARM64()) "aarch64-linux-gnu-" else "";
        
        const potential_compilers = [_][]const u8{
            try std.fmt.allocPrint(self.allocator, "{s}gcc", .{arch_prefix}),
            try std.fmt.allocPrint(self.allocator, "{s}clang", .{arch_prefix}),
            "clang",
            "gcc",
            "zig cc",
        };
        defer {
            if (arch_prefix.len > 0) {
                self.allocator.free(potential_compilers[0]);
                self.allocator.free(potential_compilers[1]);
            }
        }
        
        for (potential_compilers) |compiler| {
            if (self.findExecutable(compiler)) |path| {
                toolchain.compiler_path = try self.allocator.dupe(u8, path);
                toolchain.available = true;
                toolchain.version = try self.getCompilerVersion(path);
                
                // Set up Linux system paths
                try self.setupLinuxSystemPaths(toolchain, normalized);
                break;
            }
        }
    }
    
    /// Discover WebAssembly toolchain
    fn discoverWasmToolchain(
        self: *CrossCompilationManager,
        toolchain: *ToolchainInfo,
        normalized: TargetTripleNormalizer.NormalizedTriple,
    ) !void {
        _ = normalized;
        const potential_compilers = [_][]const u8{
            "emcc",
            "clang",
            "zig cc",
        };
        
        for (potential_compilers) |compiler| {
            if (self.findExecutable(compiler)) |path| {
                toolchain.compiler_path = try self.allocator.dupe(u8, path);
                toolchain.available = true;
                toolchain.version = try self.getCompilerVersion(path);
                
                // WebAssembly doesn't need complex path setup
                break;
            }
        }
    }
    
    /// Setup MinGW-specific paths and libraries
    fn setupMingwPaths(self: *CrossCompilationManager, toolchain: *ToolchainInfo, normalized: TargetTripleNormalizer.NormalizedTriple) !void {
        _ = normalized;
        
        var lib_paths: std.ArrayList([]const u8) = .empty;
        var include_paths: std.ArrayList([]const u8) = .empty;
        
        // Common MinGW paths
        const mingw_prefixes = [_][]const u8{
            "/usr/x86_64-w64-mingw32",
            "/usr/aarch64-w64-mingw32", 
            "/usr/local/x86_64-w64-mingw32",
            "/opt/mingw",
        };
        
        for (mingw_prefixes) |prefix| {
            const lib_path = try std.fmt.allocPrint(self.allocator, "{s}/lib", .{prefix});
            const include_path = try std.fmt.allocPrint(self.allocator, "{s}/include", .{prefix});
            
            // Check if paths exist
            if (std.fs.accessAbsolute(lib_path, .{})) {
                try lib_paths.append(self.allocator, lib_path);
            } else |_| {
                self.allocator.free(lib_path);
            }
            
            if (std.fs.accessAbsolute(include_path, .{})) {
                try include_paths.append(self.allocator, include_path);
            } else |_| {
                self.allocator.free(include_path);
            }
        }
        
        toolchain.library_paths = try lib_paths.toOwnedSlice(self.allocator);
        toolchain.include_paths = try include_paths.toOwnedSlice(self.allocator);
    }
    
    /// Setup MSVC paths and libraries with proper Visual Studio detection
    fn setupMsvcPaths(self: *CrossCompilationManager, toolchain: *ToolchainInfo) !void {
        var lib_paths: std.ArrayList([]const u8) = .empty;
        var include_paths: std.ArrayList([]const u8) = .empty;
        
        // Try to find Visual Studio installations
        const vs_paths = [_][]const u8{
            "C:\\Program Files\\Microsoft Visual Studio\\2022\\Professional",
            "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community",
            "C:\\Program Files\\Microsoft Visual Studio\\2022\\Enterprise",
            "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Professional",
            "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Community",
            "C:\\Program Files\\Microsoft Visual Studio\\2019\\Enterprise",
        };
        
        var vs_root: ?[]const u8 = null;
        for (vs_paths) |vs_path| {
            if (std.fs.accessAbsolute(vs_path, .{})) |_| {
                vs_root = try self.allocator.dupe(u8, vs_path);
                break;
            } else |_| {}
        }
        
        // Try Windows SDK paths
        const sdk_paths = [_][]const u8{
            "C:\\Program Files (x86)\\Windows Kits\\10",
            "C:\\Program Files\\Windows Kits\\10",
        };
        
        var sdk_root: ?[]const u8 = null;
        for (sdk_paths) |sdk_path| {
            if (std.fs.accessAbsolute(sdk_path, .{})) |_| {
                sdk_root = try self.allocator.dupe(u8, sdk_path);
                break;
            } else |_| {}
        }
        
        // Add Visual Studio paths if found
        if (vs_root) |vs_path| {
            // Find MSVC version directory
            const msvc_dir = try std.fmt.allocPrint(self.allocator, "{s}\\VC\\Tools\\MSVC", .{vs_path});
            defer self.allocator.free(msvc_dir);
            
            // Try to find the latest MSVC version
            if (std.fs.openDirAbsolute(msvc_dir, .{ .iterate = true })) |dir| {
                defer dir.close();
                
                var iter = dir.iterate();
                var latest_version: ?[]const u8 = null;
                
                while (iter.next() catch null) |entry| {
                    if (entry.kind == .directory) {
                        if (latest_version == null or std.mem.order(u8, entry.name, latest_version.?) == .gt) {
                            if (latest_version) |old_version| {
                                self.allocator.free(old_version);
                            }
                            latest_version = try self.allocator.dupe(u8, entry.name);
                        }
                    }
                }
                
                if (latest_version) |version| {
                    defer self.allocator.free(version);
                    
                    // Add MSVC library paths
                    const lib_path = try std.fmt.allocPrint(
                        self.allocator,
                        "{s}\\VC\\Tools\\MSVC\\{s}\\lib\\x64",
                        .{ vs_path, version }
                    );
                    try lib_paths.append(self.allocator, lib_path);
                    
                    // Add MSVC include paths
                    const include_path = try std.fmt.allocPrint(
                        self.allocator,
                        "{s}\\VC\\Tools\\MSVC\\{s}\\include",
                        .{ vs_path, version }
                    );
                    try include_paths.append(self.allocator, include_path);
                }
            } else |_| {}
        }
        
        // Add Windows SDK paths if found
        if (sdk_root) |sdk_path| {
            // Find latest Windows SDK version
            const lib_dir = try std.fmt.allocPrint(self.allocator, "{s}\\Lib", .{sdk_path});
            defer self.allocator.free(lib_dir);
            
            if (std.fs.openDirAbsolute(lib_dir, .{ .iterate = true })) |dir| {
                defer dir.close();
                
                var iter = dir.iterate();
                var latest_version: ?[]const u8 = null;
                
                while (iter.next() catch null) |entry| {
                    if (entry.kind == .directory and std.mem.startsWith(u8, entry.name, "10.")) {
                        if (latest_version == null or std.mem.order(u8, entry.name, latest_version.?) == .gt) {
                            if (latest_version) |old_version| {
                                self.allocator.free(old_version);
                            }
                            latest_version = try self.allocator.dupe(u8, entry.name);
                        }
                    }
                }
                
                if (latest_version) |version| {
                    defer self.allocator.free(version);
                    
                    // Add Windows SDK library paths
                    const sdk_lib_path = try std.fmt.allocPrint(
                        self.allocator,
                        "{s}\\Lib\\{s}\\um\\x64",
                        .{ sdk_path, version }
                    );
                    try lib_paths.append(self.allocator, sdk_lib_path);
                    
                    const sdk_lib_path2 = try std.fmt.allocPrint(
                        self.allocator,
                        "{s}\\Lib\\{s}\\ucrt\\x64",
                        .{ sdk_path, version }
                    );
                    try lib_paths.append(self.allocator, sdk_lib_path2);
                    
                    // Add Windows SDK include paths
                    const sdk_include_path = try std.fmt.allocPrint(
                        self.allocator,
                        "{s}\\Include\\{s}\\um",
                        .{ sdk_path, version }
                    );
                    try include_paths.append(self.allocator, sdk_include_path);
                    
                    const sdk_include_path2 = try std.fmt.allocPrint(
                        self.allocator,
                        "{s}\\Include\\{s}\\ucrt",
                        .{ sdk_path, version }
                    );
                    try include_paths.append(self.allocator, sdk_include_path2);
                    
                    const sdk_include_path3 = try std.fmt.allocPrint(
                        self.allocator,
                        "{s}\\Include\\{s}\\shared",
                        .{ sdk_path, version }
                    );
                    try include_paths.append(self.allocator, sdk_include_path3);
                }
            } else |_| {}
        }
        
        // Add fallback system paths
        if (lib_paths.items.len == 0) {
            // Fallback MSVC paths for standard installations
            const fallback_libs = [_][]const u8{
                "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Community\\VC\\Tools\\MSVC\\14.29.30133\\lib\\x64",
                "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.36.32532\\lib\\x64",
                "C:\\Windows\\System32",
                "C:\\Windows\\SysWOW64",
            };
            
            for (fallback_libs) |fallback_path| {
                if (std.fs.accessAbsolute(fallback_path, .{})) |_| {
                    try lib_paths.append(self.allocator, try self.allocator.dupe(u8, fallback_path));
                } else |_| {}
            }
        }
        
        toolchain.library_paths = try lib_paths.toOwnedSlice(self.allocator);
        toolchain.include_paths = try include_paths.toOwnedSlice(self.allocator);
        
        std.log.info("MSVC setup complete: {} lib paths, {} include paths", .{ toolchain.library_paths.len, toolchain.include_paths.len });
    }
    
    /// Setup Apple Silicon specific paths
    fn setupAppleSiliconPaths(self: *CrossCompilationManager, toolchain: *ToolchainInfo) !void {
        var lib_paths: std.ArrayList([]const u8) = .empty;
        var include_paths: std.ArrayList([]const u8) = .empty;
        
        // Apple Silicon SDK paths
        const sdk_paths = [_][]const u8{
            "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk",
            "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk",
        };
        
        for (sdk_paths) |sdk_path| {
            if (std.fs.accessAbsolute(sdk_path, .{})) |_| {
                const lib_path = try std.fmt.allocPrint(self.allocator, "{s}/usr/lib", .{sdk_path});
                const include_path = try std.fmt.allocPrint(self.allocator, "{s}/usr/include", .{sdk_path});
                
                try lib_paths.append(self.allocator, lib_path);
                try include_paths.append(self.allocator, include_path);
                
                toolchain.sysroot = try self.allocator.dupe(u8, sdk_path);
                break;
            } else |_| {}
        }
        
        toolchain.library_paths = try lib_paths.toOwnedSlice(self.allocator);
        toolchain.include_paths = try include_paths.toOwnedSlice(self.allocator);
    }
    
    /// Setup Apple Intel specific paths
    fn setupAppleIntelPaths(self: *CrossCompilationManager, toolchain: *ToolchainInfo) !void {
        // Similar to Apple Silicon but might have different SDK versions
        try self.setupAppleSiliconPaths(toolchain);
    }
    
    /// Setup Linux system paths for cross-compilation with enhanced ARM64 support
    fn setupLinuxSystemPaths(self: *CrossCompilationManager, toolchain: *ToolchainInfo, normalized: TargetTripleNormalizer.NormalizedTriple) !void {
        var lib_paths: std.ArrayList([]const u8) = .empty;
        var include_paths: std.ArrayList([]const u8) = .empty;
        
        if (normalized.isARM64()) {
            // Enhanced ARM64 cross-compilation library paths
            const arm64_lib_paths = [_][]const u8{
                // Primary ARM64 cross-compilation paths
                "/usr/aarch64-linux-gnu/lib",
                "/usr/lib/aarch64-linux-gnu",
                "/lib/aarch64-linux-gnu",
                
                // Cross-compiler toolchain paths
                "/usr/lib/gcc-cross/aarch64-linux-gnu",
                "/usr/aarch64-linux-gnu/lib64",
                
                // Alternative cross-compilation installations
                "/opt/cross/aarch64-linux-gnu/lib",
                "/usr/local/aarch64-linux-gnu/lib",
                
                // Musl libc paths for Alpine/embedded
                "/usr/aarch64-linux-musl/lib",
                "/usr/lib/aarch64-linux-musl",
                
                // Container-based cross-compilation
                "/usr/lib/llvm-*/lib/clang/*/lib/linux",
                "/usr/lib/gcc/aarch64-linux-gnu/*/",
            };
            
            const arm64_include_paths = [_][]const u8{
                // Primary ARM64 headers
                "/usr/aarch64-linux-gnu/include",
                "/usr/include/aarch64-linux-gnu",
                
                // Cross-compiler headers  
                "/usr/lib/gcc-cross/aarch64-linux-gnu/*/include",
                "/usr/aarch64-linux-gnu/include/c++/*",
                
                // Alternative toolchain headers
                "/opt/cross/aarch64-linux-gnu/include",
                "/usr/local/aarch64-linux-gnu/include",
                
                // Musl headers
                "/usr/aarch64-linux-musl/include",
            };
            
            // Check and add ARM64 library paths
            for (arm64_lib_paths) |lib_path| {
                if (std.mem.containsAtLeast(u8, lib_path, 1, "*")) {
                    // Handle glob patterns for version-specific paths
                    try self.addGlobPaths(&lib_paths, lib_path);
                } else {
                    if (std.fs.accessAbsolute(lib_path, .{})) |_| {
                        try lib_paths.append(self.allocator, try self.allocator.dupe(u8, lib_path));
                        std.log.debug("Added ARM64 lib path: {s}", .{lib_path});
                    } else |_| {}
                }
            }
            
            // Check and add ARM64 include paths
            for (arm64_include_paths) |include_path| {
                if (std.mem.containsAtLeast(u8, include_path, 1, "*")) {
                    // Handle glob patterns for version-specific paths
                    try self.addGlobPaths(&include_paths, include_path);
                } else {
                    if (std.fs.accessAbsolute(include_path, .{})) |_| {
                        try include_paths.append(self.allocator, try self.allocator.dupe(u8, include_path));
                        std.log.debug("Added ARM64 include path: {s}", .{include_path});
                    } else |_| {}
                }
            }
            
            // Add GCC multilib paths if available
            const gcc_multilib_cmd = [_][]const u8{ "aarch64-linux-gnu-gcc", "-print-multi-lib" };
            if (self.executeSimpleCommand(&gcc_multilib_cmd)) |output| {
                defer self.allocator.free(output);
                var lines = std.mem.splitScalar(u8, output, '\n');
                while (lines.next()) |line| {
                    if (line.len > 0 and !std.mem.eql(u8, line, ".;")) {
                        if (std.mem.indexOf(u8, line, ";")) |sep_pos| {
                            const multilib_path = line[0..sep_pos];
                            const full_path = try std.fmt.allocPrint(
                                self.allocator,
                                "/usr/lib/gcc-cross/aarch64-linux-gnu/*/{}",
                                .{multilib_path}
                            );
                            try self.addGlobPaths(&lib_paths, full_path);
                            self.allocator.free(full_path);
                        }
                    }
                }
            } else |_| {}
            
        } else {
            // x86_64 paths (existing logic)
            const arch_paths = [_][]const u8{ "/usr/lib/x86_64-linux-gnu", "/usr/lib64", "/usr/lib", "/lib64", "/lib" };
            
            for (arch_paths) |lib_path| {
                if (std.fs.accessAbsolute(lib_path, .{})) |_| {
                    try lib_paths.append(self.allocator, try self.allocator.dupe(u8, lib_path));
                } else |_| {}
            }
        }
        
        // Add standard system include paths for all architectures
        const system_includes = [_][]const u8{ 
            "/usr/include", 
            "/usr/local/include", 
            "/usr/include/linux",  // Kernel headers
            "/usr/src/linux-headers-*/include", // Version-specific kernel headers
        };
        
        for (system_includes) |include_path| {
            if (std.mem.containsAtLeast(u8, include_path, 1, "*")) {
                try self.addGlobPaths(&include_paths, include_path);
            } else {
                if (std.fs.accessAbsolute(include_path, .{})) |_| {
                    try include_paths.append(self.allocator, try self.allocator.dupe(u8, include_path));
                } else |_| {}
            }
        }
        
        toolchain.library_paths = try lib_paths.toOwnedSlice(self.allocator);
        toolchain.include_paths = try include_paths.toOwnedSlice(self.allocator);
        
        std.log.info("Linux {} setup: {} lib paths, {} include paths", .{
            if (normalized.isARM64()) "ARM64" else "x86_64", 
            toolchain.library_paths.len, 
            toolchain.include_paths.len 
        });
    }
    
    /// Helper to add glob-pattern paths by expanding wildcards
    fn addGlobPaths(self: *CrossCompilationManager, paths: *std.ArrayList([]const u8), pattern: []const u8) !void {
        if (std.mem.indexOf(u8, pattern, "*")) |star_pos| {
            const prefix = pattern[0..star_pos];
            const suffix = pattern[star_pos + 1..];
            
            // Extract directory to search
            const dir_end = if (std.mem.lastIndexOfScalar(u8, prefix, '/')) |pos| pos else 0;
            const search_dir = if (dir_end > 0) prefix[0..dir_end] else "/";
            
            if (std.fs.openDirAbsolute(search_dir, .{ .iterate = true })) |dir| {
                defer dir.close();
                
                var iter = dir.iterate();
                while (iter.next() catch null) |entry| {
                    if (entry.kind == .directory) {
                        const full_pattern_path = try std.fmt.allocPrint(
                            self.allocator, 
                            "{s}{s}{s}",
                            .{ prefix[0..dir_end], entry.name, suffix }
                        );
                        defer self.allocator.free(full_pattern_path);
                        
                        if (std.fs.accessAbsolute(full_pattern_path, .{})) |_| {
                            try paths.append(self.allocator, try self.allocator.dupe(u8, full_pattern_path));
                        } else |_| {}
                    }
                }
            } else |_| {}
        }
    }
    
    /// Execute a simple command and return output (helper for toolchain detection)
    fn executeSimpleCommand(self: *CrossCompilationManager, command: []const []const u8) ![]const u8 {
        var child = std.process.Child.init(command, self.allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Ignore;
        
        try child.spawn();
        
        const stdout = try child.stdout.?.readToEndAlloc(self.allocator, 1024);
        
        const exit_status = try child.wait();
        
        switch (exit_status) {
            .Exited => |code| {
                if (code == 0) {
                    return stdout;
                } else {
                    self.allocator.free(stdout);
                    return error.CommandFailed;
                }
            },
            else => {
                self.allocator.free(stdout);
                return error.CommandFailed;
            },
        }
    }
    
    /// Compile for a specific target with comprehensive error handling
    pub fn compile(self: *CrossCompilationManager, options: CompilationOptions) !CompilationResult {
        const start_time = std.time.milliTimestamp();
        
        // Normalize target triple
        const normalized = try self.normalizer.normalizeTriple(options.target_triple);
        const canonical_triple = try normalized.toCanonicalString(self.allocator);
        defer self.allocator.free(canonical_triple);
        
        // Get toolchain info
        const toolchain = self.toolchain_cache.get(canonical_triple) orelse {
            return error.ToolchainNotFound;
        };
        
        if (!toolchain.available) {
            return error.ToolchainUnavailable;
        }
        
        // Build compilation command
        const command = try self.buildCompilationCommand(options, toolchain, normalized);
        defer {
            for (command) |arg| self.allocator.free(arg);
            self.allocator.free(command);
        }
        
        // Execute compilation
        const result = try self.executeCompilation(command, options);
        
        // Record compilation time
        const end_time = std.time.milliTimestamp();
        const build_time = @as(u64, @intCast(end_time - start_time));
        
        var final_result = result;
        final_result.build_time_ms = build_time;
        
        // Cache result for future reference
        try self.compilation_cache.put(canonical_triple, final_result);
        
        return final_result;
    }
    
    /// Build the complete compilation command with all flags and options
    fn buildCompilationCommand(
        self: *CrossCompilationManager,
        options: CompilationOptions,
        toolchain: ToolchainInfo,
        normalized: TargetTripleNormalizer.NormalizedTriple,
    ) ![][]const u8 {
        var command: std.ArrayList([]const u8) = .empty;
        
        // Compiler
        try command.append(self.allocator, try self.allocator.dupe(u8, toolchain.compiler_path));
        
        // Source files
        for (options.source_files) |source| {
            try command.append(self.allocator, try self.allocator.dupe(u8, source));
        }
        
        // Output name
        try command.append(self.allocator, try self.allocator.dupe(u8, "-o"));
        const output_name = try std.fmt.allocPrint(
            self.allocator,
            "{s}{s}",
            .{ options.output_name, normalized.getFileExtension() },
        );
        try command.append(self.allocator, output_name);
        
        // Target specification
        if (!std.mem.containsAtLeast(u8, toolchain.compiler_path, 1, "zig")) {
            const target_flag = try std.fmt.allocPrint(self.allocator, "--target={s}", .{options.target_triple});
            try command.append(self.allocator, target_flag);
        } else {
            // Zig-specific target format
            const target_flag = try std.fmt.allocPrint(self.allocator, "-target={s}", .{options.target_triple});
            try command.append(self.allocator, target_flag);
        }
        
        // Optimization flags
        const opt_flag = switch (options.optimization_level) {
            .debug => try self.allocator.dupe(u8, "-O0"),
            .release_safe => try self.allocator.dupe(u8, "-O2"),
            .release_fast => try self.allocator.dupe(u8, "-O3"),
            .release_small => try self.allocator.dupe(u8, "-Os"),
        };
        try command.append(self.allocator, opt_flag);
        
        // Debug information
        if (options.enable_debug_info) {
            try command.append(self.allocator, try self.allocator.dupe(u8, "-g"));
        }
        
        // Link mode
        switch (options.link_mode) {
            .static => try command.append(self.allocator, try self.allocator.dupe(u8, "-static")),
            .pie => try command.append(self.allocator, try self.allocator.dupe(u8, "-pie")),
            .dynamic => {}, // Default
        }
        
        // LTO
        if (options.enable_lto) {
            try command.append(self.allocator, try self.allocator.dupe(u8, "-flto"));
        }
        
        // Sanitizers
        if (options.enable_sanitizers and !normalized.isWebAssembly()) {
            try command.append(self.allocator, try self.allocator.dupe(u8, "-fsanitize=address"));
            try command.append(self.allocator, try self.allocator.dupe(u8, "-fsanitize=undefined"));
        }
        
        // Include directories
        for (options.include_directories) |include_dir| {
            const include_flag = try std.fmt.allocPrint(self.allocator, "-I{s}", .{include_dir});
            try command.append(self.allocator, include_flag);
        }
        
        // System include directories from toolchain
        for (toolchain.include_paths) |include_dir| {
            const include_flag = try std.fmt.allocPrint(self.allocator, "-I{s}", .{include_dir});
            try command.append(self.allocator, include_flag);
        }
        
        // Library directories from toolchain
        for (toolchain.library_paths) |lib_dir| {
            const lib_flag = try std.fmt.allocPrint(self.allocator, "-L{s}", .{lib_dir});
            try command.append(self.allocator, lib_flag);
        }
        
        // Library dependencies
        for (options.library_dependencies) |lib| {
            const lib_flag = try std.fmt.allocPrint(self.allocator, "-l{s}", .{lib});
            try command.append(self.allocator, lib_flag);
        }
        
        // Define macros
        for (options.define_macros) |macro| {
            const define_flag = try std.fmt.allocPrint(self.allocator, "-D{s}", .{macro});
            try command.append(self.allocator, define_flag);
        }
        
        // Sysroot
        if (toolchain.sysroot) |sysroot| {
            const sysroot_flag = try std.fmt.allocPrint(self.allocator, "--sysroot={s}", .{sysroot});
            try command.append(self.allocator, sysroot_flag);
        }
        
        // Target-specific flags
        const target_flags = try self.normalizer.getCompilationFlags(options.target_triple);
        defer {
            for (target_flags) |flag| self.allocator.free(flag);
            self.allocator.free(target_flags);
        }
        
        for (target_flags) |flag| {
            try command.append(self.allocator, try self.allocator.dupe(u8, flag));
        }
        
        // Additional user flags
        for (options.additional_flags) |flag| {
            try command.append(self.allocator, try self.allocator.dupe(u8, flag));
        }
        
        return command.toOwnedSlice(self.allocator);
    }
    
    /// Execute the compilation command with timeout and capture results
    fn executeCompilation(self: *CrossCompilationManager, command: [][]const u8, options: CompilationOptions) !CompilationResult {
        var child = std.process.Child.init(command, self.allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Pipe;
        
        // Set up timeout handling to prevent hanging
        const timeout_ms = 300_000; // 5 minutes timeout for cross-compilation
        
        try child.spawn();
        
        // Create a thread to monitor timeout
        const child_pid = child.id;
        var timeout_reached = std.atomic.Value(bool).init(false);
        
        const TimeoutMonitor = struct {
            fn monitor(timeout_flag: *std.atomic.Value(bool), pid: std.process.Child.Id, timeout: u64) void {
                std.time.sleep(timeout * std.time.ns_per_ms);
                timeout_flag.store(true, .release);
                
                // Try to kill the hung process
                if (builtin.os.tag == .linux or builtin.os.tag == .macos) {
                    _ = std.posix.kill(@intCast(pid), std.posix.SIG.TERM) catch {};
                } else if (builtin.os.tag == .windows) {
                    // Windows process termination would go here
                    std.log.warn("Windows process timeout handling not implemented");
                }
            }
        };
        
        const timeout_thread = try std.Thread.spawn(.{}, TimeoutMonitor.monitor, .{ &timeout_reached, child_pid, timeout_ms });
        defer timeout_thread.join();
        
        // Read output with smaller buffer sizes to avoid hanging
        const stdout = child.stdout.?.readToEndAlloc(self.allocator, 512 * 1024) catch |err| {
            if (timeout_reached.load(.acquire)) {
                std.log.err("Cross-compilation timed out after {}ms for target {s}", .{ timeout_ms, options.target_triple });
                return error.CompilationTimeout;
            }
            return err;
        };
        defer self.allocator.free(stdout);
        
        const stderr = child.stderr.?.readToEndAlloc(self.allocator, 512 * 1024) catch |err| {
            if (timeout_reached.load(.acquire)) {
                std.log.err("Cross-compilation timed out after {}ms for target {s}", .{ timeout_ms, options.target_triple });
                return error.CompilationTimeout;
            }
            return err;
        };
        defer self.allocator.free(stderr);
        
        const exit_status = child.wait() catch |err| {
            if (timeout_reached.load(.acquire)) {
                std.log.err("Cross-compilation timed out after {}ms for target {s}", .{ timeout_ms, options.target_triple });
                return error.CompilationTimeout;
            }
            return err;
        };
        
        // Check if timeout was reached during execution
        if (timeout_reached.load(.acquire)) {
            std.log.err("Cross-compilation process was terminated due to timeout for target {s}", .{options.target_triple});
            return error.CompilationTimeout;
        }
        
        const success = switch (exit_status) {
            .Exited => |code| code == 0,
            else => false,
        };
        
        // Parse warnings and errors from stderr
        var warnings: std.ArrayList([]const u8) = .empty;
        var errors: std.ArrayList([]const u8) = .empty;
        
        var lines = std.mem.splitScalar(u8, stderr, '\n');
        while (lines.next()) |line| {
            if (line.len == 0) continue;
            
            if (std.mem.containsAtLeast(u8, line, 1, "warning:")) {
                try warnings.append(self.allocator, try self.allocator.dupe(u8, line));
            } else if (std.mem.containsAtLeast(u8, line, 1, "error:")) {
                try errors.append(self.allocator, try self.allocator.dupe(u8, line));
            }
        }
        
        // Get output file size
        const output_name = try std.fmt.allocPrint(
            self.allocator,
            "{s}{s}",
            .{ options.output_name, (try self.normalizer.normalizeTriple(options.target_triple)).getFileExtension() },
        );
        defer self.allocator.free(output_name);
        
        const output_size = if (success) blk: {
            const stat = std.fs.cwd().statFile(output_name) catch break :blk 0;
            break :blk stat.size;
        } else 0;
        
        return CompilationResult{
            .target_triple = try self.allocator.dupe(u8, options.target_triple),
            .output_path = try self.allocator.dupe(u8, output_name),
            .success = success,
            .build_time_ms = 0, // Will be set by caller
            .output_size_bytes = output_size,
            .warnings = try warnings.toOwnedSlice(self.allocator),
            .errors = try errors.toOwnedSlice(self.allocator),
        };
    }
    
    /// Utility functions
    fn findExecutable(self: *CrossCompilationManager, name: []const u8) ?[]const u8 {
        _ = self;
        
        // Simple check - in a real implementation, you'd search PATH
        const test_command = std.fmt.allocPrint(
            std.heap.page_allocator,
            "which {s}",
            .{name},
        ) catch return null;
        defer std.heap.page_allocator.free(test_command);
        
        var child = std.process.Child.init(&[_][]const u8{ "sh", "-c", test_command }, std.heap.page_allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Ignore;
        
        child.spawn() catch return null;
        const stdout = child.stdout.?.readToEndAlloc(std.heap.page_allocator, 1024) catch return null;
        defer std.heap.page_allocator.free(stdout);
        
        const exit_status = child.wait() catch return null;
        
        if (exit_status == .Exited and exit_status.Exited == 0 and stdout.len > 0) {
            const trimmed = std.mem.trim(u8, stdout, " \n\r\t");
            return std.heap.page_allocator.dupe(u8, trimmed) catch null;
        }
        
        return null;
    }
    
    fn getCompilerVersion(self: *CrossCompilationManager, compiler_path: []const u8) ![]const u8 {
        _ = self;
        
        const version_command = [_][]const u8{ compiler_path, "--version" };
        
        var child = std.process.Child.init(&version_command, std.heap.page_allocator);
        child.stdout_behavior = .Pipe;
        child.stderr_behavior = .Ignore;
        
        child.spawn() catch return try std.heap.page_allocator.dupe(u8, "unknown");
        
        const stdout = child.stdout.?.readToEndAlloc(std.heap.page_allocator, 1024) catch {
            return try std.heap.page_allocator.dupe(u8, "unknown");
        };
        defer std.heap.page_allocator.free(stdout);
        
        _ = child.wait() catch return try std.heap.page_allocator.dupe(u8, "unknown");
        
        // Extract first line as version
        if (std.mem.indexOf(u8, stdout, "\n")) |newline_pos| {
            return try std.heap.page_allocator.dupe(u8, stdout[0..newline_pos]);
        } else {
            return try std.heap.page_allocator.dupe(u8, std.mem.trim(u8, stdout, " \n\r\t"));
        }
    }
    
    /// High-level methods for common operations
    
    /// Cross-compile a project for multiple targets
    pub fn crossCompileProject(
        self: *CrossCompilationManager,
        project_path: []const u8,
        targets: []const []const u8,
        base_options: CompilationOptions,
    ) ![]CompilationResult {
        _ = project_path;
        var results: std.ArrayList(CompilationResult) = .empty;
        
        for (targets) |target| {
            var options = base_options;
            options.target_triple = target;
            
            print("Compiling for target: {s}\n", .{target});
            
            const result = self.compile(options) catch |err| {
                print("  ❌ Failed: {}\n", .{err});
                
                // Create a failed result
                try results.append(self.allocator, CompilationResult{
                    .target_triple = try self.allocator.dupe(u8, target),
                    .output_path = try self.allocator.dupe(u8, ""),
                    .success = false,
                    .build_time_ms = 0,
                    .output_size_bytes = 0,
                    .warnings = &.{},
                    .errors = &[_][]const u8{try std.fmt.allocPrint(self.allocator, "Compilation error: {}", .{err})},
                });
                continue;
            };
            
            if (result.success) {
                print("  ✅ Success: {s} ({} bytes, {} ms)\n", .{ result.output_path, result.output_size_bytes, result.build_time_ms });
            } else {
                print("  ❌ Failed: {} errors\n", .{result.errors.len});
            }
            
            try results.append(allocator, result);
        }
        
        return results.toOwnedSlice(allocator);
    }
    
    /// Generate a comprehensive cross-compilation report
    pub fn generateCompilationReport(self: *CrossCompilationManager, results: []const CompilationResult) !void {
        _ = self;
        var stdout_buffer: [4096]u8 = undefined;
        const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
        
        try stdout.print("\n=== Cross-Compilation Report ===\n\n");
        
        var successful = @as(u32, 0);
        var total_build_time = @as(u64, 0);
        var total_output_size = @as(u64, 0);
        
        for (results) |result| {
            try stdout.print("Target: {s}\n", .{result.target_triple});
            try stdout.print("  Status: {s}\n", .{if (result.success) "✅ Success" else "❌ Failed"});
            try stdout.print("  Build Time: {} ms\n", .{result.build_time_ms});
            try stdout.print("  Output Size: {} bytes\n", .{result.output_size_bytes});
            try stdout.print("  Warnings: {}\n", .{result.warnings.len});
            try stdout.print("  Errors: {}\n", .{result.errors.len});
            
            if (result.errors.len > 0) {
                try stdout.print("  Error Messages:\n");
                for (result.errors) |error_msg| {
                    try stdout.print("    {s}\n", .{error_msg});
                }
            }
            
            try stdout.print("\n");
            
            if (result.success) {
                successful += 1;
                total_build_time += result.build_time_ms;
                total_output_size += result.output_size_bytes;
            }
        }
        
        try stdout.print("Summary:\n");
        try stdout.print("  Successful: {}/{} targets ({d:.1}%)\n", .{ successful, results.len, @as(f64, @floatFromInt(successful)) / @as(f64, @floatFromInt(results.len)) * 100.0 });
        try stdout.print("  Total Build Time: {} ms\n", .{total_build_time});
        try stdout.print("  Total Output Size: {} bytes\n", .{total_output_size});
        
        if (successful > 0) {
            try stdout.print("  Average Build Time: {d:.1} ms\n", .{@as(f64, @floatFromInt(total_build_time)) / @as(f64, @floatFromInt(successful))});
            try stdout.print("  Average Output Size: {d:.1} bytes\n", .{@as(f64, @floatFromInt(total_output_size)) / @as(f64, @floatFromInt(successful))});
        }
    }
};
