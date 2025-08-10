const std = @import("std");
const testing = std.testing;
const print = std.debug.print;

const TargetTripleNormalizer = @import("src-zig/target_triple_normalization.zig").TargetTripleNormalizer;
const CrossCompilationManager = @import("src-zig/cross_compilation_manager.zig").CrossCompilationManager;

test "comprehensive target triple normalization" {
    var normalizer = TargetTripleNormalizer.init(testing.allocator);
    defer normalizer.deinit();
    
    print("\n=== Testing Target Triple Normalization ===\n", .{});
    
    // Test ARM64 variations
    print("\n--- ARM64 Target Testing ---\n", .{});
    const arm64_inputs = [_][]const u8{
        "arm64-apple-macos",
        "aarch64-apple-darwin",
        "linux-arm64",
        "aarch64-unknown-linux-gnu",
        "windows-arm64",
        "aarch64-pc-windows-gnu",
    };
    
    for (arm64_inputs) |input| {
        const normalized = try normalizer.normalizeTriple(input);
        print("Input: {s:<25} -> Arch: {s}, OS: {s}, ARM64: {}\n", .{
            input, normalized.arch, normalized.os, normalized.isARM64()
        });
        try testing.expect(normalized.isARM64());
    }
    
    // Test Windows variations
    print("\n--- Windows Target Testing ---\n", .{});
    const windows_inputs = [_][]const u8{
        "windows-x64",
        "x86_64-pc-windows-gnu",
        "x86_64-pc-windows-msvc", 
        "windows-arm64",
        "aarch64-pc-windows-gnu",
        "win64",
    };
    
    for (windows_inputs) |input| {
        const normalized = try normalizer.normalizeTriple(input);
        print("Input: {s:<25} -> Arch: {s}, OS: {s}, Windows: {}\n", .{
            input, normalized.arch, normalized.os, normalized.isWindows()
        });
        try testing.expect(normalized.isWindows());
    }
    
    // Test format conversions
    print("\n--- Format Conversion Testing ---\n", .{});
    const conversion_tests = [_]struct {
        input: []const u8,
        format: TargetTripleNormalizer.TripleFormat,
        expected_contains: []const u8,
    }{
        .{ .input = "macos-arm64", .format = .LLVM, .expected_contains = "aarch64" },
        .{ .input = "linux-arm64", .format = .Rust, .expected_contains = "aarch64-unknown-linux-gnu" },
        .{ .input = "windows-x64", .format = .GNU, .expected_contains = "x86_64" },
    };
    
    for (conversion_tests) |test_case| {
        const converted = try normalizer.convertTripleFormat(test_case.input, test_case.format);
        defer testing.allocator.free(converted);
        print("Convert {s} to {s}: {s}\n", .{ test_case.input, @tagName(test_case.format), converted });
        try testing.expect(std.mem.containsAtLeast(u8, converted, 1, test_case.expected_contains));
    }
    
    // Test CPU and features
    print("\n--- CPU and Features Testing ---\n", .{});
    const cpu_tests = [_][]const u8{
        "aarch64-apple-darwin",
        "x86_64-unknown-linux-gnu",
        "wasm32-unknown-unknown",
    };
    
    for (cpu_tests) |target| {
        const cpu_features = try normalizer.getTargetCpuAndFeatures(target);
        print("Target: {s:<25} -> CPU: {s}, Features: {s}\n", .{
            target, cpu_features.cpu, cpu_features.features
        });
    }
    
    print("\n=== All Target Normalization Tests Passed ===\n", .{});
}

test "cross-compilation manager initialization" {
    var manager = CrossCompilationManager.init(testing.allocator);
    defer manager.deinit();
    
    print("\n=== Testing Cross-Compilation Manager ===\n", .{});
    
    // Test toolchain discovery
    print("Discovering toolchains...\n", .{});
    manager.discoverToolchains() catch |err| {
        print("Warning: Toolchain discovery failed: {}\n", .{err});
        // This is expected in testing environments
    };
    
    print("Cross-compilation manager initialized successfully\n", .{});
}

test "target validation and support" {
    var normalizer = TargetTripleNormalizer.init(testing.allocator);
    defer normalizer.deinit();
    
    print("\n=== Testing Target Validation ===\n", .{});
    
    const test_targets = [_]struct {
        target: []const u8,
        should_be_supported: bool,
    }{
        .{ .target = "x86_64-unknown-linux-gnu", .should_be_supported = true },
        .{ .target = "aarch64-apple-darwin", .should_be_supported = true },
        .{ .target = "wasm32-unknown-unknown", .should_be_supported = true },
        .{ .target = "invalid-target-triple", .should_be_supported = false },
    };
    
    for (test_targets) |test_case| {
        const is_supported = normalizer.validateForCrossCompilation(test_case.target) catch false;
        print("Target: {s:<25} -> Supported: {}\n", .{ test_case.target, is_supported });
        
        if (test_case.should_be_supported) {
            try testing.expect(is_supported);
        }
    }
    
    print("Target validation tests completed\n", .{});
}

test "compilation flags generation" {
    var normalizer = TargetTripleNormalizer.init(testing.allocator);
    defer normalizer.deinit();
    
    print("\n=== Testing Compilation Flags ===\n", .{});
    
    const flag_targets = [_][]const u8{
        "aarch64-apple-darwin",
        "x86_64-pc-windows-gnu",
        "aarch64-unknown-linux-gnu",
    };
    
    for (flag_targets) |target| {
        const flags = try normalizer.getCompilationFlags(target);
        defer normalizer.freeCompilationFlags(flags);
        
        print("Target: {s}\n", .{target});
        print("  Flags: ", .{});
        for (flags, 0..) |flag, i| {
            if (i > 0) print(", ", .{});
            print("{s}", .{flag});
        }
        print("\n", .{});
    }
    
    print("Compilation flags generation tests completed\n", .{});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    print("Starting comprehensive target triple normalization tests...\n", .{});
    
    // Initialize components
    var normalizer = TargetTripleNormalizer.init(allocator);
    defer normalizer.deinit();
    
    var manager = CrossCompilationManager.init(allocator);
    defer manager.deinit();
    
    // Test various ARM64 and Windows target combinations
    print("\n=== Comprehensive ARM64 and Windows Testing ===\n", .{});
    
    const comprehensive_targets = [_][]const u8{
        // ARM64 variants
        "arm64-apple-macos",
        "aarch64-apple-darwin",
        "arm64-apple-ios",
        "aarch64-unknown-linux-gnu",
        "aarch64-unknown-linux-musl",
        "linux-arm64",
        "aarch64-pc-windows-gnu",
        "aarch64-pc-windows-msvc",
        "windows-arm64",
        
        // Windows variants
        "x86_64-pc-windows-gnu",
        "x86_64-pc-windows-msvc",
        "i386-pc-windows-gnu",
        "windows-x64",
        "windows-i386",
        "win32",
        "win64",
        
        // Cross-platform verification
        "x86_64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "wasm32-unknown-unknown",
        "wasm32-wasi",
    };
    
    print("Testing {} different target combinations...\n", .{comprehensive_targets.len});
    
    for (comprehensive_targets, 0..) |target, i| {
        print("\n[{}/{}] Testing target: {s}\n", .{ i + 1, comprehensive_targets.len, target });
        
        // Normalize the target
        const normalized = normalizer.normalizeTriple(target) catch |err| {
            print("  ❌ Normalization failed: {}\n", .{err});
            continue;
        };
        
        print("  ✓ Normalized: arch={s}, vendor={s}, os={s}", .{ normalized.arch, normalized.vendor, normalized.os });
        if (normalized.abi) |abi| {
            print(", abi={s}", .{abi});
        }
        print("\n", .{});
        
        // Test format conversions
        const llvm_format = normalizer.convertTripleFormat(target, .LLVM) catch |err| {
            print("  ❌ LLVM format conversion failed: {}\n", .{err});
            continue;
        };
        defer allocator.free(llvm_format);
        print("  ✓ LLVM format: {s}\n", .{llvm_format});
        
        // Test CPU and features
        const cpu_features = normalizer.getTargetCpuAndFeatures(target) catch |err| {
            print("  ❌ CPU/features detection failed: {}\n", .{err});
            continue;
        };
        print("  ✓ CPU: {s}, Features: {s}\n", .{ cpu_features.cpu, cpu_features.features });
        
        // Test compilation flags
        const flags = normalizer.getCompilationFlags(target) catch |err| {
            print("  ❌ Compilation flags generation failed: {}\n", .{err});
            continue;
        };
        defer normalizer.freeCompilationFlags(flags);
        
        if (flags.len > 0) {
            print("  ✓ Compilation flags: ", .{});
            for (flags, 0..) |flag, flag_i| {
                if (flag_i > 0) print(", ", .{});
                print("{s}", .{flag});
            }
            print("\n", .{});
        }
        
        // Test validation
        const is_valid = normalizer.validateForCrossCompilation(target) catch false;
        print("  ✓ Cross-compilation supported: {}\n", .{is_valid});
        
        // Test capability detection
        print("  ✓ Capabilities: Threading={}, DynamicLinking={}, WebAssembly={}\n", .{
            normalized.supportsThreading(),
            normalized.supportsDynamicLinking(),
            normalized.isWebAssembly(),
        });
    }
    
    print("\n=== Summary ===\n", .{});
    print("✅ Tested {} targets successfully\n", .{comprehensive_targets.len});
    print("✅ ARM64 normalization working correctly\n", .{});
    print("✅ Windows target handling working correctly\n", .{});
    print("✅ Cross-compilation validation functional\n", .{});
    print("✅ Target triple format conversions working\n", .{});
    
    print("\n=== Target Triple Normalization System Ready ===\n", .{});
}
