const std = @import("std");
const Build = std.Build;
const Allocator = std.mem.Allocator;
const TargetTripleNormalizer = @import("../target_triple_normalization.zig").TargetTripleNormalizer;

/// Enhanced target handling for the CURSED build system
pub const TargetHandler = struct {
    build: *Build,
    normalizer: TargetTripleNormalizer,
    supported_targets: std.StringHashMap(TargetInfo),
    
    pub const TargetInfo = struct {
        canonical_triple: []const u8,
        friendly_name: []const u8,
        description: []const u8,
        supports_llvm: bool,
        supports_threading: bool,
        supports_dynamic_linking: bool,
        cpu_features: struct {
            cpu: []const u8,
            features: []const u8,
        },
        compilation_flags: [][]const u8,
        
        pub fn deinit(self: *TargetInfo, allocator: Allocator) void {
        _ = allocator;
            allocator.free(self.canonical_triple);
            allocator.free(self.friendly_name);
            allocator.free(self.description);
            allocator.free(self.cpu_features.cpu);
            allocator.free(self.cpu_features.features);
            for (self.compilation_flags) |flag| {
                allocator.free(flag);
            }
            allocator.free(self.compilation_flags);
        }
    };
    
    pub const BuildTargetOptions = struct {
        target_string: []const u8,
        optimization: std.builtin.OptimizeMode = .Debug,
        link_mode: Build.Step.Compile.LinkMode = .dynamic,
        strip_debug: bool = false,
        enable_lto: bool = false,
        cpu_features: ?[]const u8 = null,
        additional_flags: []const []const u8 = &.{},
    };
    
    pub fn init(build: *Build) !TargetHandler {
        var handler = TargetHandler{
            .build = build,
            .normalizer = TargetTripleNormalizer.init(build.allocator),
            .supported_targets = std.StringHashMap(TargetInfo){},
        };
        
        try handler.initializeSupportedTargets();
        return handler;
    }
    
    pub fn deinit(self: *TargetHandler) void {
        var iterator = self.supported_targets.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.deinit(self.build.allocator);
        }
        self.supported_targets.deinit(self.allocator);
        self.normalizer.deinit(self.allocator);
    }
    
    /// Initialize the list of supported targets with their characteristics
    fn initializeSupportedTargets(self: *TargetHandler) !void {
        const target_definitions = [_]struct {
            input: []const u8,
            friendly: []const u8,
            description: []const u8,
        }{
            // Linux targets
            .{ .input = "x86_64-unknown-linux-gnu", .friendly = "linux-x64", .description = "Linux x86_64 (GNU)" },
            .{ .input = "x86_64-unknown-linux-musl", .friendly = "linux-x64-musl", .description = "Linux x86_64 (musl)" },
            .{ .input = "aarch64-unknown-linux-gnu", .friendly = "linux-arm64", .description = "Linux ARM64 (GNU)" },
            .{ .input = "aarch64-unknown-linux-musl", .friendly = "linux-arm64-musl", .description = "Linux ARM64 (musl)" },
            .{ .input = "i386-unknown-linux-gnu", .friendly = "linux-i386", .description = "Linux i386 (GNU)" },
            
            // macOS targets
            .{ .input = "x86_64-apple-darwin", .friendly = "macos-x64", .description = "macOS x86_64 (Intel)" },
            .{ .input = "aarch64-apple-darwin", .friendly = "macos-arm64", .description = "macOS ARM64 (Apple Silicon)" },
            
            // Windows targets
            .{ .input = "x86_64-pc-windows-gnu", .friendly = "windows-x64", .description = "Windows x86_64 (MinGW)" },
            .{ .input = "x86_64-pc-windows-msvc", .friendly = "windows-x64-msvc", .description = "Windows x86_64 (MSVC)" },
            .{ .input = "aarch64-pc-windows-gnu", .friendly = "windows-arm64", .description = "Windows ARM64 (MinGW)" },
            .{ .input = "i386-pc-windows-gnu", .friendly = "windows-i386", .description = "Windows i386 (MinGW)" },
            
            // WebAssembly targets
            .{ .input = "wasm32-unknown-unknown", .friendly = "wasm32", .description = "WebAssembly 32-bit" },
            .{ .input = "wasm32-wasi", .friendly = "wasi", .description = "WebAssembly System Interface" },
            
            // Embedded targets
            .{ .input = "thumbv7em-none-eabihf", .friendly = "arm-cortex-m4", .description = "ARM Cortex-M4 (hard float)" },
            .{ .input = "thumbv7m-none-eabi", .friendly = "arm-cortex-m3", .description = "ARM Cortex-M3" },
            .{ .input = "riscv32-unknown-none-elf", .friendly = "riscv32-embedded", .description = "RISC-V 32-bit embedded" },
            .{ .input = "riscv64-unknown-linux-gnu", .friendly = "linux-riscv64", .description = "Linux RISC-V 64-bit" },
        };
        
        for (target_definitions) |target_def| {
            const normalized = try self.normalizer.normalizeTriple(target_def.input);
            const canonical = try normalized.toCanonicalString(self.build.allocator);
            const cpu_features = try self.normalizer.getTargetCpuAndFeatures(target_def.input);
            const compilation_flags = try self.normalizer.getCompilationFlags(target_def.input);
            
            const target_info = TargetInfo{
                .canonical_triple = canonical,
                .friendly_name = try self.build.allocator.dupe(u8, target_def.friendly),
                .description = try self.build.allocator.dupe(u8, target_def.description),
                .supports_llvm = !normalized.isWebAssembly() or std.mem.eql(u8, normalized.os, "wasi"),
                .supports_threading = normalized.supportsThreading(),
                .supports_dynamic_linking = normalized.supportsDynamicLinking(),
                .cpu_features = .{
                    .cpu = try self.build.allocator.dupe(u8, cpu_features.cpu),
                    .features = try self.build.allocator.dupe(u8, cpu_features.features),
                },
                .compilation_flags = compilation_flags,
            };
            
            try self.supported_targets.put(target_def.friendly, target_info);
            // Also register the canonical triple as a key
            try self.supported_targets.put(canonical, target_info);
        }
    }
    
    /// Create a Zig target from a target string (user-friendly or triple)
    pub fn createZigTarget(self: *TargetHandler, target_string: []const u8) !Build.ResolvedTarget {
        // Normalize the target string first
        const normalized = try self.normalizer.normalizeTriple(target_string);
        const canonical = try normalized.toCanonicalString(self.build.allocator);
        defer self.build.allocator.free(canonical);
        
        // Parse into Zig's target query format
        const target_query = try self.parseToZigTargetQuery(normalized);
        
        return self.build.resolveTargetQuery(target_query);
    }
    
    /// Parse normalized triple into Zig's target query format
    fn parseToZigTargetQuery(self: *TargetHandler, normalized: TargetTripleNormalizer.NormalizedTriple) !Build.TargetQuery {
        _ = self;
        
        // Map architecture
        const cpu_arch: std.Target.Cpu.Arch = if (std.mem.eql(u8, normalized.arch, "x86_64"))
            .x86_64
        else if (std.mem.eql(u8, normalized.arch, "aarch64"))
            .aarch64
        else if (std.mem.eql(u8, normalized.arch, "i386"))
            .x86
        else if (std.mem.eql(u8, normalized.arch, "wasm32"))
            .wasm32
        else if (std.mem.eql(u8, normalized.arch, "riscv64"))
            .riscv64
        else
            return error.UnsupportedArchitecture;
        
        // Map OS
        const os_tag: std.Target.Os.Tag = if (std.mem.eql(u8, normalized.os, "linux"))
            .linux
        else if (std.mem.eql(u8, normalized.os, "darwin"))
            .macos
        else if (std.mem.eql(u8, normalized.os, "windows"))
            .windows
        else if (std.mem.eql(u8, normalized.os, "wasi"))
            .wasi
        else if (std.mem.eql(u8, normalized.os, "unknown"))
            .freestanding
        else if (std.mem.eql(u8, normalized.os, "none"))
            .freestanding
        else
            return error.UnsupportedOS;
        
        // Map ABI
        const abi: ?std.Target.Abi = if (normalized.abi) |abi_str| blk: {
            if (std.mem.eql(u8, abi_str, "gnu"))
                break :blk .gnu;
            if (std.mem.eql(u8, abi_str, "msvc"))
                break :blk .msvc;
            if (std.mem.eql(u8, abi_str, "eabi"))
                break :blk .eabi;
            if (std.mem.eql(u8, abi_str, "eabihf"))
                break :blk .eabihf;
            break :blk null;
        } else null;
        
        return Build.TargetQuery{
            .cpu_arch = cpu_arch,
            .os_tag = os_tag,
            .abi = abi,
        };
    }
    
    /// Create a compilation step with target-specific optimizations
    pub fn createCompileStep(
        self: *TargetHandler,
        name: []const u8,
        root_source_file: Build.LazyPath,
        options: BuildTargetOptions,
    ) !*Build.Step.Compile {
        const target = try self.createZigTarget(options.target_string);
        
        // Get target info for optimizations
        const target_info = self.getTargetInfo(options.target_string) orelse {
            return error.UnsupportedTarget;
        };
        
        const exe = self.build.addExecutable(.{
            .name = name,
            .root_source_file = root_source_file,
            .target = target,
            .optimize = options.optimization,
            .link_mode = options.link_mode,
            .strip = options.strip_debug,
        });
        
        // Apply target-specific optimizations
        try self.applyTargetOptimizations(exe, target_info, options);
        
        return exe;
    }
    
    /// Apply target-specific compilation optimizations
    fn applyTargetOptimizations(
        self: *TargetHandler,
        compile_step: *Build.Step.Compile,
        target_info: *const TargetInfo,
        options: BuildTargetOptions,
    ) !void {
        _ = self;
        
        // Set CPU and features if available
        if (target_info.cpu_features.cpu.len > 0 and options.cpu_features == null) {
            // Note: Zig's CPU setting would be done here if the API supports it
            // compile_step.target_info.cpu = target_info.cpu_features.cpu;
        }
        
        // Enable LTO if requested and supported
        if (options.enable_lto and target_info.supports_llvm) {
            compile_step.want_lto = true;
        }
        
        // Add target-specific compilation flags
        for (target_info.compilation_flags) |flag| {
            compile_step.addArg(flag);
        }
        
        // Add additional user-specified flags
        for (options.additional_flags) |flag| {
            compile_step.addArg(flag);
        }
        
        // Configure threading support
        if (!target_info.supports_threading) {
            compile_step.single_threaded = true;
        }
        
        // Configure dynamic linking
        if (!target_info.supports_dynamic_linking) {
            compile_step.linkage = .static;
        }
    }
    
    /// Get target information by target string
    pub fn getTargetInfo(self: *TargetHandler, target_string: []const u8) ?*const TargetInfo {
        // Try direct lookup first
        if (self.supported_targets.get(target_string)) |info| {
            return &info;
        }
        
        // Try normalizing and looking up canonical form
        const normalized = self.normalizer.normalizeTriple(target_string) catch return null;
        const canonical = normalized.toCanonicalString(self.build.allocator) catch return null;
        defer self.build.allocator.free(canonical);
        
        return if (self.supported_targets.get(canonical)) |info| &info else null;
    }
    
    /// List all supported targets
    pub fn listSupportedTargets(self: *TargetHandler) !void {
        var stdout_buffer: [4096]u8 = undefined;
        const stdout = std.fs.File.stdout().writer(stdout_buffer[0..]);
        
        try stdout.writer().print("Supported target platforms:\n\n", .{});
        
        var iterator = self.supported_targets.iterator();
        var printed_targets = std.StringHashMap(void){};
        defer printed_targets.deinit();
        
        while (iterator.next()) |entry| {
            const target_info = entry.value_ptr;
            
            // Skip if we've already printed this canonical triple
            if (printed_targets.contains(target_info.canonical_triple)) continue;
            try printed_targets.put(target_info.canonical_triple, {});
            
            try stdout.writer().print("  {s:<20} - {s}\n", .{ target_info.friendly_name, target_info.description });
            try stdout.writer().print("    Triple: {s}\n", .{target_info.canonical_triple});
            try stdout.writer().print("    LLVM: {s}, Threading: {s}, Dynamic Linking: {s}\n", .{
                if (target_info.supports_llvm) "✓" else "✗",
                if (target_info.supports_threading) "✓" else "✗",
                if (target_info.supports_dynamic_linking) "✓" else "✗",
            });
            if (target_info.cpu_features.cpu.len > 0) {
                try stdout.writer().print("    CPU: {s}", .{target_info.cpu_features.cpu});
                if (target_info.cpu_features.features.len > 0) {
                    try stdout.writer().print(", Features: {s}", .{target_info.cpu_features.features});
                }
                try stdout.writer().print("\n", .{});
            }
            try stdout.writer().print("\n", .{});
        }
    }
    
    /// Validate target for cross-compilation
    pub fn validateTarget(self: *TargetHandler, target_string: []const u8) !bool {
        return self.normalizer.validateForCrossCompilation(target_string);
    }
    
    /// Get recommended build options for a target
    pub fn getRecommendedBuildOptions(self: *TargetHandler, target_string: []const u8) !BuildTargetOptions {
        const target_info = self.getTargetInfo(target_string) orelse {
            return error.UnsupportedTarget;
        };
        
        const normalized = try self.normalizer.normalizeTriple(target_string);
        
        return BuildTargetOptions{
            .target_string = target_string,
            .optimization = if (normalized.isWebAssembly()) .ReleaseSmall else .ReleaseFast,
            .link_mode = if (target_info.supports_dynamic_linking) .dynamic else .static,
            .strip_debug = false,
            .enable_lto = target_info.supports_llvm,
            .cpu_features = if (target_info.cpu_features.features.len > 0) target_info.cpu_features.features else null,
            .additional_flags = &.{},
        };
    }
    
    /// Create multiple target variants for testing
    pub fn createCrossCompilationTargets(
        self: *TargetHandler,
        name: []const u8,
        root_source_file: Build.LazyPath,
        target_list: []const []const u8,
    ) !std.ArrayList(*Build.Step.Compile) {
        var compile_steps = std.ArrayList(*Build.Step.Compile){};
        
        for (target_list) |target_string| {
            const options = try self.getRecommendedBuildOptions(target_string);
            const target_info = self.getTargetInfo(target_string) orelse continue;
            
            const target_name = try std.fmt.allocPrint(
                self.build.allocator,
                "{s}-{s}",
                .{ name, target_info.friendly_name },
            );
            
            const compile_step = try self.createCompileStep(target_name, root_source_file, options);
            try compile_steps.append(allocator, compile_step);
        }
        
        return compile_steps;
    }
};

/// Utility function to create a target handler for build scripts
pub fn createTargetHandler(b: *Build) !TargetHandler {
    return TargetHandler.init(b);
}

/// Common target sets for easy cross-compilation testing
pub const TARGET_SETS = struct {
    pub const DESKTOP = [_][]const u8{
        "linux-x64",
        "macos-x64",
        "macos-arm64",
        "windows-x64",
    };
    
    pub const ARM64_FOCUS = [_][]const u8{
        "linux-arm64",
        "macos-arm64",
        "windows-arm64",
    };
    
    pub const ALL_PLATFORMS = [_][]const u8{
        "linux-x64",
        "linux-arm64",
        "macos-x64",
        "macos-arm64",
        "windows-x64",
        "windows-arm64",
        "wasm32",
    };
    
    pub const EMBEDDED = [_][]const u8{
        "arm-cortex-m4",
        "arm-cortex-m3",
        "riscv32-embedded",
    };
};
