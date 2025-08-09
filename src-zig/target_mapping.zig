const std = @import("std");

// Map user-friendly names to triples
const target_map = std.StaticStringMap([]const u8).initComptime(.{
        // Linux targets
        .{ "linux-x64", "x86_64-unknown-linux-gnu" },
        .{ "linux-x86_64", "x86_64-unknown-linux-gnu" },
        .{ "linux-arm64", "aarch64-unknown-linux-gnu" },
        .{ "linux-aarch64", "aarch64-unknown-linux-gnu" },
        .{ "linux-i386", "i386-unknown-linux-gnu" },
        .{ "linux-riscv64", "riscv64-unknown-linux-gnu" },
        
        // macOS targets
        .{ "macos-x64", "x86_64-apple-darwin" },
        .{ "macos-intel", "x86_64-apple-darwin" },
        .{ "macos-arm64", "aarch64-apple-darwin" },
        .{ "macos-apple-silicon", "aarch64-apple-darwin" },
        .{ "darwin-x64", "x86_64-apple-darwin" },
        .{ "darwin-arm64", "aarch64-apple-darwin" },
        
        // Windows targets  
        .{ "windows-x64", "x86_64-pc-windows-gnu" },
        .{ "windows-x86_64", "x86_64-pc-windows-gnu" },
        .{ "windows-i386", "i386-pc-windows-gnu" },
        .{ "windows-arm64", "aarch64-pc-windows-gnu" },
        .{ "win32", "i386-pc-windows-gnu" },
        .{ "win64", "x86_64-pc-windows-gnu" },
        
        // WebAssembly targets
        .{ "wasm", "wasm32-unknown-unknown" },
        .{ "wasm32", "wasm32-unknown-unknown" },
        .{ "wasm64", "wasm64-unknown-unknown" },
        .{ "wasi", "wasm32-wasi" },
        
        // Embedded targets
        .{ "arm-cortex-m4", "thumbv7em-none-eabihf" },
        .{ "arm-cortex-m3", "thumbv7m-none-eabi" },
        .{ "riscv32", "riscv32-unknown-none-elf" },
        
        // Short aliases
        .{ "x64", "x86_64-unknown-linux-gnu" },
        .{ "arm64", "aarch64-unknown-linux-gnu" },
    });

/// Maps user-friendly target names to LLVM target triples
pub fn targetToLLVMTriple(target: []const u8) ?[]const u8 {
    // First try the mapping table
    if (target_map.get(target)) |mapped| {
        return mapped;
    }
    
    // If not found in map and contains dashes, assume it's already a triple
    if (std.mem.indexOf(u8, target, "-") != null) {
        return target;
    }
    
    return null;
}

/// Get the native target triple for the current platform
pub fn getNativeTriple() []const u8 {
    const builtin = @import("builtin");
    const arch = builtin.cpu.arch;
    const os = builtin.os.tag;
    
    return switch (os) {
        .linux => switch (arch) {
            .x86_64 => "x86_64-unknown-linux-gnu",
            .aarch64 => "aarch64-unknown-linux-gnu",
            .x86 => "i386-unknown-linux-gnu", 
            .riscv64 => "riscv64-unknown-linux-gnu",
            else => "x86_64-unknown-linux-gnu", // fallback
        },
        .macos => switch (arch) {
            .x86_64 => "x86_64-apple-darwin",
            .aarch64 => "aarch64-apple-darwin",
            else => "x86_64-apple-darwin", // fallback
        },
        .windows => switch (arch) {
            .x86_64 => "x86_64-pc-windows-gnu",
            .x86 => "i386-pc-windows-gnu",
            .aarch64 => "aarch64-pc-windows-gnu",
            else => "x86_64-pc-windows-gnu", // fallback
        },
        .freestanding => switch (arch) {
            .wasm32 => "wasm32-unknown-unknown",
            .wasm64 => "wasm64-unknown-unknown",
            else => "x86_64-unknown-linux-gnu", // fallback
        },
        else => "x86_64-unknown-linux-gnu", // fallback
    };
}

/// Validate that a target triple is supported
pub fn validateTargetTriple(triple: []const u8) bool {
    // Check if it follows proper triple format
    var parts_iter = std.mem.splitScalar(u8, triple, '-');
    var part_count: u8 = 0;
    while (parts_iter.next()) |_| {
        part_count += 1;
        if (part_count > 4) return false; // Too many parts
    }
    
    return part_count >= 3; // At least arch-vendor-os
}

/// Get appropriate CPU and features for a target
pub fn getTargetCpuAndFeatures(triple: []const u8) struct { cpu: []const u8, features: []const u8 } {
    if (std.mem.startsWith(u8, triple, "x86_64")) {
        return .{ .cpu = "x86-64", .features = "" };
    } else if (std.mem.startsWith(u8, triple, "aarch64")) {
        return .{ .cpu = "generic", .features = "" };
    } else if (std.mem.startsWith(u8, triple, "i386")) {
        return .{ .cpu = "i686", .features = "" };
    } else if (std.mem.startsWith(u8, triple, "wasm32")) {
        return .{ .cpu = "generic", .features = "+simd128" };
    } else if (std.mem.startsWith(u8, triple, "riscv64")) {
        return .{ .cpu = "generic-rv64", .features = "" };
    } else {
        return .{ .cpu = "generic", .features = "" };
    }
}

/// Check if target supports specific features
pub fn targetSupportsFeature(triple: []const u8, feature: []const u8) bool {
    if (std.mem.eql(u8, feature, "threads")) {
        return !std.mem.startsWith(u8, triple, "wasm32-unknown-unknown");
    } else if (std.mem.eql(u8, feature, "dynamic_linking")) {
        return !std.mem.startsWith(u8, triple, "wasm32") and 
               !std.mem.containsAtLeast(u8, triple, 1, "none-eabi");
    } else if (std.mem.eql(u8, feature, "exceptions")) {
        return !std.mem.startsWith(u8, triple, "wasm32");
    }
    
    return true; // Default to supported
}

/// Get file extension for the target
pub fn getExecutableExtension(triple: []const u8) []const u8 {
    if (std.mem.containsAtLeast(u8, triple, 1, "windows")) {
        return ".exe";
    } else if (std.mem.startsWith(u8, triple, "wasm32")) {
        return ".wasm";
    } else {
        return "";
    }
}

test "target mapping" {
    const testing = std.testing;
    
    // Test user-friendly names
    try testing.expectEqualStrings(targetToLLVMTriple("linux-x64").?, "x86_64-unknown-linux-gnu");
    try testing.expectEqualStrings(targetToLLVMTriple("macos-arm64").?, "aarch64-apple-darwin");
    try testing.expectEqualStrings(targetToLLVMTriple("windows-x64").?, "x86_64-pc-windows-gnu");
    try testing.expectEqualStrings(targetToLLVMTriple("wasm32").?, "wasm32-unknown-unknown");
    
    // Test direct triples (should pass through)
    try testing.expectEqualStrings(targetToLLVMTriple("x86_64-unknown-linux-musl").?, "x86_64-unknown-linux-musl");
    
    // Test validation
    try testing.expect(validateTargetTriple("x86_64-unknown-linux-gnu"));
    try testing.expect(validateTargetTriple("wasm32-unknown-unknown"));
    try testing.expect(!validateTargetTriple("invalid"));
    try testing.expect(!validateTargetTriple("too-many-parts-here-fail"));
}
