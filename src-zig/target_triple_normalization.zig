const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;

/// Comprehensive target triple normalization system for ARM64 and Windows platforms
pub const TargetTripleNormalizer = struct {
    allocator: Allocator,
    
    /// Architecture mappings for different target triple formats
    const ArchMapping = std.StaticStringMap([]const u8).initComptime(.{
        // ARM64 variations
        .{ "arm64", "aarch64" },
        .{ "aarch64", "aarch64" },
        .{ "arm64_64", "aarch64" },
        .{ "aarch64_64", "aarch64" },
        .{ "arm64e", "aarch64" }, // Apple Silicon with pointer authentication
        
        // x86_64 variations
        .{ "x86_64", "x86_64" },
        .{ "x64", "x86_64" },
        .{ "amd64", "x86_64" },
        .{ "x86-64", "x86_64" },
        
        // x86 variations
        .{ "x86", "i386" },
        .{ "i386", "i386" },
        .{ "i486", "i386" },
        .{ "i586", "i386" },
        .{ "i686", "i386" },
        
        // RISC-V variations
        .{ "riscv64", "riscv64" },
        .{ "riscv32", "riscv32" },
        
        // WebAssembly variations
        .{ "wasm32", "wasm32" },
        .{ "wasm64", "wasm64" },
    });
    
    /// Vendor mappings for different target triple formats
    const VendorMapping = std.StaticStringMap([]const u8).initComptime(.{
        // Apple variations
        .{ "apple", "apple" },
        .{ "darwin", "apple" },
        
        // Microsoft variations
        .{ "pc", "pc" },
        .{ "microsoft", "pc" },
        .{ "win", "pc" },
        .{ "windows", "pc" },
        
        // Linux/GNU variations
        .{ "unknown", "unknown" },
        .{ "gnu", "unknown" },
        .{ "linux", "unknown" },
        
        // Embedded/None
        .{ "none", "none" },
        .{ "embedded", "none" },
    });
    
    /// OS mappings for different target triple formats
    const OSMapping = std.StaticStringMap([]const u8).initComptime(.{
        // Linux variations
        .{ "linux", "linux" },
        .{ "linux-gnu", "linux" },
        .{ "linux-musl", "linux" },
        .{ "gnu", "linux" },
        
        // macOS/Darwin variations
        .{ "macos", "darwin" },
        .{ "darwin", "darwin" },
        .{ "osx", "darwin" },
        .{ "mac", "darwin" },
        
        // Windows variations
        .{ "windows", "windows" },
        .{ "win32", "windows" },
        .{ "mingw", "windows" },
        .{ "cygwin", "windows" },
        .{ "msvc", "windows" },
        
        // WebAssembly variations
        .{ "wasi", "wasi" },
        .{ "unknown", "unknown" },
        .{ "freestanding", "freestanding" },
        
        // Embedded variations
        .{ "none", "none" },
        .{ "eabi", "none" },
        .{ "eabihf", "none" },
        .{ "elf", "none" },
    });
    
    /// ABI mappings for different target triple formats
    const ABIMapping = std.StaticStringMap([]const u8).initComptime(.{
        // GNU ABI variations
        .{ "gnu", "gnu" },
        .{ "gnueabi", "gnu" },
        .{ "gnueabihf", "gnu" },
        .{ "eabi", "eabi" },
        .{ "eabihf", "eabihf" },
        
        // Microsoft ABI variations
        .{ "msvc", "msvc" },
        .{ "gnu", "gnu" }, // MinGW
        
        // Apple ABI variations
        .{ "darwin", "darwin" },
        .{ "macho", "darwin" },
        
        // WebAssembly ABI
        .{ "unknown", "unknown" },
        .{ "wasi", "wasi" },
        
        // Embedded ABI
        .{ "elf", "elf" },
        .{ "none", "none" },
    });
    
    /// Normalized target triple structure
    pub const NormalizedTriple = struct {
        arch: []const u8,
        vendor: []const u8,
        os: []const u8,
        abi: ?[]const u8,
        
        pub fn toString(self: @This(), allocator: Allocator) ![]const u8 {
            if (self.abi) |abi| {
                return std.fmt.allocPrint(allocator, "{s}-{s}-{s}-{s}", .{ self.arch, self.vendor, self.os, abi });
            } else {
                return std.fmt.allocPrint(allocator, "{s}-{s}-{s}", .{ self.arch, self.vendor, self.os });
            }
        }
        
        pub fn toCanonicalString(self: @This(), allocator: Allocator) ![]const u8 {
            // Generate the most standard/canonical form of the triple
            const canonical_arch = normalizeArch(self.arch);
            const canonical_vendor = normalizeVendor(self.vendor, self.os);
            const canonical_os = normalizeOS(self.os);
            const canonical_abi = if (self.abi) |abi| normalizeABI(abi, self.os) else null;
            
            if (canonical_abi) |abi| {
                return std.fmt.allocPrint(allocator, "{s}-{s}-{s}-{s}", .{ canonical_arch, canonical_vendor, canonical_os, abi });
            } else {
                return std.fmt.allocPrint(allocator, "{s}-{s}-{s}", .{ canonical_arch, canonical_vendor, canonical_os });
            }
        }
        
        pub fn isARM64(self: @This()) bool {
            return std.mem.eql(u8, self.arch, "aarch64");
        }
        
        pub fn isWindows(self: @This()) bool {
            return std.mem.eql(u8, self.os, "windows");
        }
        
        pub fn isApple(self: @This()) bool {
            return std.mem.eql(u8, self.vendor, "apple") or std.mem.eql(u8, self.os, "darwin");
        }
        
        pub fn isLinux(self: @This()) bool {
            return std.mem.eql(u8, self.os, "linux");
        }
        
        pub fn isWebAssembly(self: @This()) bool {
            return std.mem.startsWith(u8, self.arch, "wasm");
        }
        
        pub fn supportsThreading(self: @This()) bool {
            return !self.isWebAssembly() or std.mem.eql(u8, self.os, "wasi");
        }
        
        pub fn supportsDynamicLinking(self: @This()) bool {
            return !self.isWebAssembly() and !std.mem.containsAtLeast(u8, self.os, 1, "none");
        }
        
        pub fn getFileExtension(self: @This()) []const u8 {
            if (self.isWindows()) return ".exe";
            if (self.isWebAssembly()) return ".wasm";
            return "";
        }
    };
    
    pub fn init(allocator: Allocator) TargetTripleNormalizer {
        return TargetTripleNormalizer{
            .allocator = allocator,
        };
    }
    
    pub fn deinit(self: *TargetTripleNormalizer) void {
        _ = self;
    }
    
    /// Parse and normalize a target triple from various formats
    pub fn normalizeTriple(self: *TargetTripleNormalizer, input: []const u8) !NormalizedTriple {
        // Handle special cases first
        if (std.mem.eql(u8, input, "native")) {
            return self.getNativeTriple();
        }
        
        // Handle user-friendly names
        if (self.parseUserFriendlyName(input)) |triple| {
            return triple;
        }
        
        // Parse as standard target triple (arch-vendor-os-abi or arch-vendor-os)
        return self.parseStandardTriple(input);
    }
    
    /// Get the normalized triple for the current native platform
    pub fn getNativeTriple(self: *TargetTripleNormalizer) NormalizedTriple {
        _ = self;
        const builtin = @import("builtin");
        const arch = builtin.cpu.arch;
        const os = builtin.os.tag;
        
        const normalized_arch = switch (arch) {
            .x86_64 => "x86_64",
            .aarch64 => "aarch64",
            .x86 => "i386",
            .riscv64 => "riscv64",
            .wasm32 => "wasm32",
            else => "unknown",
        };
        
        return switch (os) {
            .linux => NormalizedTriple{
                .arch = normalized_arch,
                .vendor = "unknown",
                .os = "linux",
                .abi = "gnu",
            },
            .macos => NormalizedTriple{
                .arch = normalized_arch,
                .vendor = "apple",
                .os = "darwin",
                .abi = null,
            },
            .windows => NormalizedTriple{
                .arch = normalized_arch,
                .vendor = "pc",
                .os = "windows",
                .abi = "gnu", // Default to MinGW for better LLVM compatibility
            },
            .freestanding, .wasi => NormalizedTriple{
                .arch = normalized_arch,
                .vendor = "unknown",
                .os = if (os == .wasi) "wasi" else "unknown",
                .abi = null,
            },
            else => NormalizedTriple{
                .arch = normalized_arch,
                .vendor = "unknown",
                .os = "unknown",
                .abi = null,
            },
        };
    }
    
    /// Parse user-friendly target names
    fn parseUserFriendlyName(self: *TargetTripleNormalizer, name: []const u8) ?NormalizedTriple {
        _ = self;
        
        // Linux targets
        if (std.mem.eql(u8, name, "linux-x64") or std.mem.eql(u8, name, "linux-x86_64")) {
            return NormalizedTriple{ .arch = "x86_64", .vendor = "unknown", .os = "linux", .abi = "gnu" };
        }
        if (std.mem.eql(u8, name, "linux-arm64") or std.mem.eql(u8, name, "linux-aarch64")) {
            return NormalizedTriple{ .arch = "aarch64", .vendor = "unknown", .os = "linux", .abi = "gnu" };
        }
        
        // macOS targets
        if (std.mem.eql(u8, name, "macos-x64") or std.mem.eql(u8, name, "macos-intel")) {
            return NormalizedTriple{ .arch = "x86_64", .vendor = "apple", .os = "darwin", .abi = null };
        }
        if (std.mem.eql(u8, name, "macos-arm64") or std.mem.eql(u8, name, "macos-apple-silicon")) {
            return NormalizedTriple{ .arch = "aarch64", .vendor = "apple", .os = "darwin", .abi = null };
        }
        
        // Windows targets
        if (std.mem.eql(u8, name, "windows-x64") or std.mem.eql(u8, name, "win64")) {
            return NormalizedTriple{ .arch = "x86_64", .vendor = "pc", .os = "windows", .abi = "gnu" };
        }
        if (std.mem.eql(u8, name, "windows-i386") or std.mem.eql(u8, name, "win32")) {
            return NormalizedTriple{ .arch = "i386", .vendor = "pc", .os = "windows", .abi = "gnu" };
        }
        if (std.mem.eql(u8, name, "windows-arm64")) {
            return NormalizedTriple{ .arch = "aarch64", .vendor = "pc", .os = "windows", .abi = "gnu" };
        }
        if (std.mem.eql(u8, name, "windows-msvc") or std.mem.eql(u8, name, "windows-x64-msvc")) {
            return NormalizedTriple{ .arch = "x86_64", .vendor = "pc", .os = "windows", .abi = "msvc" };
        }
        
        // WebAssembly targets
        if (std.mem.eql(u8, name, "wasm") or std.mem.eql(u8, name, "wasm32")) {
            return NormalizedTriple{ .arch = "wasm32", .vendor = "unknown", .os = "unknown", .abi = null };
        }
        if (std.mem.eql(u8, name, "wasi") or std.mem.eql(u8, name, "wasm32-wasi")) {
            return NormalizedTriple{ .arch = "wasm32", .vendor = "unknown", .os = "wasi", .abi = null };
        }
        
        return null;
    }
    
    /// Parse standard target triple format
    fn parseStandardTriple(self: *TargetTripleNormalizer, triple: []const u8) !NormalizedTriple {
        _ = self;
        
        var parts = std.mem.splitScalar(u8, triple, '-');
        
        const arch_part = parts.next() orelse return error.InvalidTriple;
        const vendor_part = parts.next() orelse return error.InvalidTriple;
        const os_part = parts.next() orelse return error.InvalidTriple;
        const abi_part = parts.next(); // Optional
        
        const arch = normalizeArch(arch_part);
        const vendor = normalizeVendor(vendor_part, os_part);
        const os = normalizeOS(os_part);
        const abi = if (abi_part) |abi_str| normalizeABI(abi_str, os) else null;
        
        return NormalizedTriple{
            .arch = arch,
            .vendor = vendor,
            .os = os,
            .abi = abi,
        };
    }
    
    /// Convert between different target triple formats
    pub fn convertTripleFormat(self: *TargetTripleNormalizer, input: []const u8, format: TripleFormat) ![]const u8 {
        const normalized = try self.normalizeTriple(input);
        
        return switch (format) {
            .LLVM => normalized.toCanonicalString(self.allocator),
            .Rust => self.toRustTriple(normalized),
            .GNU => self.toGNUTriple(normalized),
            .Apple => self.toAppleTriple(normalized),
            .Zig => normalized.toString(self.allocator),
        };
    }
    
    pub const TripleFormat = enum {
        LLVM,
        Rust,
        GNU,
        Apple,
        Zig,
    };
    
    fn toRustTriple(self: *TargetTripleNormalizer, triple: NormalizedTriple) ![]const u8 {
        // Rust uses specific triple formats
        if (triple.isARM64() and triple.isApple()) {
            return std.fmt.allocPrint(self.allocator, "aarch64-apple-darwin", .{});
        } else if (triple.isARM64() and triple.isLinux()) {
            return std.fmt.allocPrint(self.allocator, "aarch64-unknown-linux-gnu", .{});
        } else if (triple.isARM64() and triple.isWindows()) {
            const abi = if (triple.abi) |a| a else "gnu";
            return std.fmt.allocPrint(self.allocator, "aarch64-pc-windows-{s}", .{abi});
        }
        
        return triple.toCanonicalString(self.allocator);
    }
    
    fn toGNUTriple(self: *TargetTripleNormalizer, triple: NormalizedTriple) ![]const u8 {
        // GNU toolchain specific formats
        if (triple.isARM64()) {
            if (triple.isLinux()) {
                return std.fmt.allocPrint(self.allocator, "aarch64-linux-gnu", .{});
            }
        }
        
        return triple.toCanonicalString(self.allocator);
    }
    
    fn toAppleTriple(self: *TargetTripleNormalizer, triple: NormalizedTriple) ![]const u8 {
        // Apple-specific triple formats
        if (triple.isApple()) {
            if (triple.isARM64()) {
                return std.fmt.allocPrint(self.allocator, "arm64-apple-macos", .{});
            } else {
                return std.fmt.allocPrint(self.allocator, "x86_64-apple-macos", .{});
            }
        }
        
        return triple.toCanonicalString(self.allocator);
    }
    
    /// Validate if a target triple is supported for cross-compilation
    pub fn validateForCrossCompilation(self: *TargetTripleNormalizer, triple_str: []const u8) !bool {
        const triple = try self.normalizeTriple(triple_str);
        
        // Check if architecture is supported
        const supported_archs = [_][]const u8{ "x86_64", "aarch64", "i386", "wasm32" };
        var arch_supported = false;
        for (supported_archs) |arch| {
            if (std.mem.eql(u8, triple.arch, arch)) {
                arch_supported = true;
                break;
            }
        }
        
        if (!arch_supported) {
            return false;
        }
        
        // Check OS/platform support
        const supported_os = [_][]const u8{ "linux", "darwin", "windows", "wasi", "unknown" };
        var os_supported = false;
        for (supported_os) |os| {
            if (std.mem.eql(u8, triple.os, os)) {
                os_supported = true;
                break;
            }
        }
        
        return arch_supported and os_supported;
    }
    
    /// Get CPU and feature recommendations for a target
    pub fn getTargetCpuAndFeatures(self: *TargetTripleNormalizer, triple_str: []const u8) !struct { cpu: []const u8, features: []const u8 } {
        const triple = try self.normalizeTriple(triple_str);
        
        if (triple.isARM64()) {
            if (triple.isApple()) {
                return .{ .cpu = "apple-a14", .features = "+neon,+fp-armv8,+crc" };
            } else {
                return .{ .cpu = "generic", .features = "+neon" };
            }
        } else if (std.mem.eql(u8, triple.arch, "x86_64")) {
            if (triple.isApple()) {
                return .{ .cpu = "core2", .features = "+sse3,+ssse3" };
            } else {
                return .{ .cpu = "x86-64", .features = "" };
            }
        } else if (triple.isWebAssembly()) {
            return .{ .cpu = "generic", .features = "+simd128" };
        }
        
        return .{ .cpu = "generic", .features = "" };
    }
    
    /// Generate target-specific compilation flags  
    /// Returns owned slice of owned strings that must be freed by caller
    pub fn getCompilationFlags(self: *TargetTripleNormalizer, triple_str: []const u8) ![][]const u8 {
        const triple = try self.normalizeTriple(triple_str);
        var flags: std.ArrayList([]const u8) = .empty;
        
        // Architecture-specific flags
        if (triple.isARM64()) {
            try flags.append(self.allocator, try self.allocator.dupe(u8, "-march=armv8-a"));
            if (triple.isApple()) {
                try flags.append(self.allocator, try self.allocator.dupe(u8, "-mcpu=apple-a14"));
            }
        } else if (std.mem.eql(u8, triple.arch, "x86_64")) {
            try flags.append(self.allocator, try self.allocator.dupe(u8, "-march=x86-64"));
        }
        
        // OS-specific flags
        if (triple.isWindows()) {
            try flags.append(self.allocator, try self.allocator.dupe(u8, "-D_WIN32"));
            if (triple.abi != null and std.mem.eql(u8, triple.abi.?, "msvc")) {
                try flags.append(self.allocator, try self.allocator.dupe(u8, "-fms-extensions"));
            }
        } else if (triple.isLinux()) {
            try flags.append(self.allocator, try self.allocator.dupe(u8, "-D_GNU_SOURCE"));
        } else if (triple.isApple()) {
            try flags.append(self.allocator, try self.allocator.dupe(u8, "-D_DARWIN_C_SOURCE"));
        }
        
        // ABI-specific flags
        if (triple.abi) |abi| {
            if (std.mem.eql(u8, abi, "eabihf")) {
                try flags.append(self.allocator, try self.allocator.dupe(u8, "-mfloat-abi=hard"));
            } else if (std.mem.eql(u8, abi, "eabi")) {
                try flags.append(self.allocator, try self.allocator.dupe(u8, "-mfloat-abi=soft"));
            }
        }
        
        return flags.toOwnedSlice(self.allocator);
    }
    
    /// Free compilation flags returned by getCompilationFlags
    pub fn freeCompilationFlags(self: *TargetTripleNormalizer, flags: [][]const u8) void {
        for (flags) |flag| {
            self.allocator.free(flag);
        }
        self.allocator.free(flags);
    }
};

/// Normalize architecture component
fn normalizeArch(arch: []const u8) []const u8 {
    return TargetTripleNormalizer.ArchMapping.get(arch) orelse arch;
}

/// Normalize vendor component
fn normalizeVendor(vendor: []const u8, os: []const u8) []const u8 {
    _ = os;
    return TargetTripleNormalizer.VendorMapping.get(vendor) orelse vendor;
}

/// Normalize OS component
fn normalizeOS(os: []const u8) []const u8 {
    return TargetTripleNormalizer.OSMapping.get(os) orelse os;
}

/// Normalize ABI component
fn normalizeABI(abi: []const u8, os: []const u8) []const u8 {
    _ = os;
    return TargetTripleNormalizer.ABIMapping.get(abi) orelse abi;
}

test "ARM64 triple normalization" {
    var normalizer = TargetTripleNormalizer.init(testing.allocator);
    defer normalizer.deinit(testing.allocator);
    
    // Test ARM64 variations
    const arm64_triple = try normalizer.normalizeTriple("arm64-apple-macos");
    try testing.expect(arm64_triple.isARM64());
    try testing.expect(arm64_triple.isApple());
    
    const aarch64_triple = try normalizer.normalizeTriple("aarch64-unknown-linux-gnu");
    try testing.expect(aarch64_triple.isARM64());
    try testing.expect(aarch64_triple.isLinux());
    
    // Test user-friendly names
    const linux_arm64 = try normalizer.normalizeTriple("linux-arm64");
    try testing.expect(linux_arm64.isARM64());
    try testing.expect(linux_arm64.isLinux());
}

test "Windows triple normalization" {
    var normalizer = TargetTripleNormalizer.init(testing.allocator);
    defer normalizer.deinit(testing.allocator);
    
    // Test Windows variations
    const windows_x64 = try normalizer.normalizeTriple("windows-x64");
    try testing.expect(windows_x64.isWindows());
    try testing.expectEqualStrings(windows_x64.arch, "x86_64");
    
    const windows_arm64 = try normalizer.normalizeTriple("windows-arm64");
    try testing.expect(windows_arm64.isWindows());
    try testing.expect(windows_arm64.isARM64());
    
    const msvc_triple = try normalizer.normalizeTriple("x86_64-pc-windows-msvc");
    try testing.expect(msvc_triple.isWindows());
    try testing.expectEqualStrings(msvc_triple.abi.?, "msvc");
}

test "Triple format conversion" {
    var normalizer = TargetTripleNormalizer.init(testing.allocator);
    defer normalizer.deinit(testing.allocator);
    
    // Test LLVM format conversion
    const llvm_triple = try normalizer.convertTripleFormat("macos-arm64", .LLVM);
    defer testing.allocator.free(llvm_triple);
    try testing.expectEqualStrings(llvm_triple, "aarch64-apple-darwin");
    
    // Test Rust format conversion
    const rust_triple = try normalizer.convertTripleFormat("linux-arm64", .Rust);
    defer testing.allocator.free(rust_triple);
    try testing.expectEqualStrings(rust_triple, "aarch64-unknown-linux-gnu");
}

test "Cross-compilation validation" {
    var normalizer = TargetTripleNormalizer.init(testing.allocator);
    defer normalizer.deinit(testing.allocator);
    
    // Test supported targets
    try testing.expect(try normalizer.validateForCrossCompilation("aarch64-apple-darwin"));
    try testing.expect(try normalizer.validateForCrossCompilation("x86_64-pc-windows-gnu"));
    try testing.expect(try normalizer.validateForCrossCompilation("wasm32-unknown-unknown"));
    
    // Test unsupported targets (should still return true for our supported set)
    try testing.expect(try normalizer.validateForCrossCompilation("aarch64-unknown-linux-gnu"));
}

test "Target features and CPU selection" {
    var normalizer = TargetTripleNormalizer.init(testing.allocator);
    defer normalizer.deinit(testing.allocator);
    
    // Test ARM64 Apple Silicon
    const apple_cpu = try normalizer.getTargetCpuAndFeatures("aarch64-apple-darwin");
    try testing.expectEqualStrings(apple_cpu.cpu, "apple-a14");
    try testing.expect(std.mem.containsAtLeast(u8, apple_cpu.features, 1, "neon"));
    
    // Test x86_64
    const x64_cpu = try normalizer.getTargetCpuAndFeatures("x86_64-unknown-linux-gnu");
    try testing.expectEqualStrings(x64_cpu.cpu, "x86-64");
    
    // Test WebAssembly
    const wasm_cpu = try normalizer.getTargetCpuAndFeatures("wasm32-unknown-unknown");
    try testing.expectEqualStrings(wasm_cpu.cpu, "generic");
    try testing.expect(std.mem.containsAtLeast(u8, wasm_cpu.features, 1, "simd128"));
}
