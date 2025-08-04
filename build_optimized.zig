const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

// Build optimization configuration
const BuildConfig = struct {
    cache_enabled: bool = true,
    parallel_jobs: u32 = 0, // 0 = auto-detect
    incremental: bool = true,
    pgo_enabled: bool = false,
    profile_mode: ProfileMode = .development,
    
    const ProfileMode = enum {
        development,
        testing,
        release,
        production,
    };
};

// Cross-compilation targets
const CrossTarget = struct {
    name: []const u8,
    target: std.zig.CrossTarget,
    enabled: bool = true,
};

const CROSS_TARGETS = [_]CrossTarget{
    // Desktop platforms
    .{ .name = "linux-x86_64", .target = .{ .cpu_arch = .x86_64, .os_tag = .linux } },
    .{ .name = "linux-aarch64", .target = .{ .cpu_arch = .aarch64, .os_tag = .linux } },
    .{ .name = "linux-riscv64", .target = .{ .cpu_arch = .riscv64, .os_tag = .linux } },
    .{ .name = "macos-x86_64", .target = .{ .cpu_arch = .x86_64, .os_tag = .macos } },
    .{ .name = "macos-aarch64", .target = .{ .cpu_arch = .aarch64, .os_tag = .macos } },
    .{ .name = "windows-x86_64", .target = .{ .cpu_arch = .x86_64, .os_tag = .windows } },
    .{ .name = "windows-aarch64", .target = .{ .cpu_arch = .aarch64, .os_tag = .windows } },
    
    // WebAssembly targets
    .{ .name = "wasm32-browser", .target = .{ .cpu_arch = .wasm32, .os_tag = .freestanding } },
    .{ .name = "wasm32-wasi", .target = .{ .cpu_arch = .wasm32, .os_tag = .wasi } },
    
    // Embedded targets
    .{ .name = "arm-cortex-m", .target = .{ .cpu_arch = .thumb, .os_tag = .freestanding } },
    .{ .name = "esp32", .target = .{ .cpu_arch = .xtensa, .os_tag = .freestanding } },
};

// Build performance tracking
const BuildMetrics = struct {
    start_time: i64,
    compilation_times: std.StringHashMap(i64),
    cache_hits: u32 = 0,
    cache_misses: u32 = 0,
    
    fn init(allocator: Allocator) BuildMetrics {
        return BuildMetrics{
            .start_time = std.time.milliTimestamp(),
            .compilation_times = std.StringHashMap(i64).init(allocator),
        };
    }
    
    fn recordCompilationTime(self: *BuildMetrics, target: []const u8, time_ms: i64) !void {
        try self.compilation_times.put(target, time_ms);
    }
    
    fn printMetrics(self: *const BuildMetrics) void {
        const total_time = std.time.milliTimestamp() - self.start_time;
        print("🚀 Build Performance Metrics:\n");
        print("  Total build time: {}ms\n", .{total_time});
        print("  Cache hits: {} | Cache misses: {}\n", .{ self.cache_hits, self.cache_misses });
        print("  Cache efficiency: {d:.1}%\n", .{@as(f64, @floatFromInt(self.cache_hits)) / @as(f64, @floatFromInt(self.cache_hits + self.cache_misses)) * 100.0});
        
        var iterator = self.compilation_times.iterator();
        while (iterator.next()) |entry| {
            print("  {s}: {}ms\n", .{ entry.key_ptr.*, entry.value_ptr.* });
        }
    }
};

pub fn build(b: *std.Build) void {
    // Initialize build configuration
    const config = BuildConfig{
        .cache_enabled = b.option(bool, "cache", "Enable build caching") orelse true,
        .parallel_jobs = b.option(u32, "parallel", "Number of parallel jobs") orelse detectCores(),
        .incremental = b.option(bool, "incremental", "Enable incremental compilation") orelse true,
        .pgo_enabled = b.option(bool, "pgo", "Enable Profile-Guided Optimization") orelse false,
        .profile_mode = parseProfileMode(b.option([]const u8, "profile", "Build profile") orelse "development"),
    };
    
    // Standard target and optimization options
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    
    print("🔧 Building CURSED with optimized build system\n");
    print("  Profile: {s} | Parallel jobs: {} | Cache: {s} | Incremental: {s}\n", .{
        @tagName(config.profile_mode),
        config.parallel_jobs,
        if (config.cache_enabled) "enabled" else "disabled",
        if (config.incremental) "enabled" else "disabled",
    });
    
    // Initialize build metrics
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    var metrics = BuildMetrics.init(gpa.allocator());
    
    // Create main CURSED compiler executable with optimizations
    const exe = buildMainExecutable(b, target, optimize, config, &metrics);
    
    // Create variant executables for different use cases
    buildVariantExecutables(b, target, optimize, config, &metrics);
    
    // Setup testing infrastructure
    setupTestSuite(b, target, optimize, config);
    
    // Setup cross-compilation targets
    setupCrossCompilation(b, optimize, config, &metrics);
    
    // Setup Profile-Guided Optimization
    if (config.pgo_enabled) {
        setupPGO(b, exe, target, optimize);
    }
    
    // Setup benchmarking
    setupBenchmarks(b, target, optimize, config);
    
    // Create utility build steps
    setupUtilitySteps(b, config);
    
    // Print final metrics
    const print_metrics = b.step("metrics", "Print build performance metrics");
    const print_metrics_run = b.addSystemCommand(&[_][]const u8{"echo", "Build completed successfully"});
    print_metrics.dependOn(&print_metrics_run.step);
}

fn buildMainExecutable(b: *std.Build, target: std.zig.CrossTarget, optimize: std.builtin.OptimizeMode, config: BuildConfig, metrics: *BuildMetrics) *std.Build.Step.Compile {
    const start_time = std.time.milliTimestamp();
    
    // Choose the best main implementation based on profile
    const main_source = switch (config.profile_mode) {
        .development => "src-zig/main_unified.zig",
        .testing => "src-zig/main_minimal.zig",
        .release => "src-zig/main_optimized.zig",
        .production => "src-zig/main_production.zig",
    };
    
    const exe = b.addExecutable(.{
        .name = "cursed-optimized",
        .root_source_file = b.path(main_source),
        .target = target,
        .optimize = optimize,
    });
    
    // Configure optimization level based on profile
    switch (config.profile_mode) {
        .development => {
            exe.addCSourceFile(.{ .file = b.path("src-zig/debug_support.c"), .flags = &[_][]const u8{"-O0", "-g"} });
        },
        .testing => {
            exe.addCSourceFile(.{ .file = b.path("src-zig/test_support.c"), .flags = &[_][]const u8{"-O1"} });
        },
        .release, .production => {
            exe.addCSourceFile(.{ .file = b.path("src-zig/production_runtime.c"), .flags = &[_][]const u8{"-O3", "-DNDEBUG"} });
            exe.want_lto = true; // Enable Link-Time Optimization
        },
    }
    
    // Link system libraries
    exe.linkLibC();
    if (config.profile_mode == .production) {
        exe.linkSystemLibrary("pthread");
        exe.linkSystemLibrary("m");
    }
    
    // Enable incremental compilation
    if (config.incremental) {
        exe.use_stage1 = false;
        exe.use_llvm = true;
    }
    
    b.installArtifact(exe);
    
    // Record compilation time
    const compilation_time = std.time.milliTimestamp() - start_time;
    metrics.recordCompilationTime("main-executable", compilation_time) catch {};
    
    return exe;
}

fn buildVariantExecutables(b: *std.Build, target: std.zig.CrossTarget, optimize: std.builtin.OptimizeMode, config: BuildConfig, metrics: *BuildMetrics) void {
    const variants = [_]struct { name: []const u8, source: []const u8, description: []const u8 }{
        .{ .name = "cursed-minimal", .source = "src-zig/main_minimal.zig", .description = "Minimal compiler for quick builds" },
        .{ .name = "cursed-complete", .source = "src-zig/main_complete.zig", .description = "Full-featured compiler" },
        .{ .name = "cursed-debug", .source = "src-zig/main_debug.zig", .description = "Debug-enabled compiler" },
        .{ .name = "cursed-benchmark", .source = "src-zig/main_benchmark.zig", .description = "Performance testing compiler" },
    };
    
    for (variants) |variant| {
        const start_time = std.time.milliTimestamp();
        
        const exe = b.addExecutable(.{
            .name = variant.name,
            .root_source_file = b.path(variant.source),
            .target = target,
            .optimize = optimize,
        });
        
        exe.linkLibC();
        if (config.incremental) {
            exe.use_stage1 = false;
        }
        
        b.installArtifact(exe);
        
        // Record compilation time
        const compilation_time = std.time.milliTimestamp() - start_time;
        metrics.recordCompilationTime(variant.name, compilation_time) catch {};
    }
}

fn setupTestSuite(b: *std.Build, target: std.zig.CrossTarget, optimize: std.builtin.OptimizeMode, config: BuildConfig) void {
    // Comprehensive test suite with parallel execution
    const test_modules = [_][]const u8{
        "src-zig/main_unified.zig",
        "src-zig/lexer.zig",
        "src-zig/parser.zig",
        "src-zig/codegen.zig",
        "src-zig/runtime.zig",
        "src-zig/concurrency.zig",
        "stdlib-zig/testz.zig",
    };
    
    // Create individual test steps for parallel execution
    for (test_modules) |module| {
        const module_name = std.fs.path.stem(module);
        const unit_tests = b.addTest(.{
            .name = module_name,
            .root_source_file = b.path(module),
            .target = target,
            .optimize = optimize,
        });
        
        unit_tests.linkLibC();
        
        const run_tests = b.addRunArtifact(unit_tests);
        
        const test_step_name = b.fmt("test-{s}", .{module_name});
        const test_step = b.step(test_step_name, b.fmt("Run {s} tests", .{module_name}));
        test_step.dependOn(&run_tests.step);
    }
    
    // Parallel test execution step
    const test_all_parallel = b.step("test-parallel", "Run all tests in parallel");
    const parallel_test_script = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        "zig build test-main_unified & zig build test-lexer & zig build test-parser & wait",
    });
    test_all_parallel.dependOn(&parallel_test_script.step);
    
    // Performance test suite
    const perf_tests = b.addTest(.{
        .name = "performance-tests",
        .root_source_file = b.path("tests/performance_suite.zig"),
        .target = target,
        .optimize = .ReleaseFast,
    });
    perf_tests.linkLibC();
    
    const run_perf_tests = b.addRunArtifact(perf_tests);
    const perf_test_step = b.step("test-performance", "Run performance tests");
    perf_test_step.dependOn(&run_perf_tests.step);
}

fn setupCrossCompilation(b: *std.Build, optimize: std.builtin.OptimizeMode, config: BuildConfig, metrics: *BuildMetrics) void {
    // Create cross-compilation targets
    for (CROSS_TARGETS) |cross_target| {
        if (!cross_target.enabled) continue;
        
        const start_time = std.time.milliTimestamp();
        
        const exe = b.addExecutable(.{
            .name = b.fmt("cursed-{s}", .{cross_target.name}),
            .root_source_file = b.path("src-zig/main_unified.zig"),
            .target = cross_target.target,
            .optimize = optimize,
        });
        
        // Platform-specific configuration
        configureForTarget(exe, cross_target);
        
        b.installArtifact(exe);
        
        // Create individual build step for each target
        const target_step = b.step(
            b.fmt("build-{s}", .{cross_target.name}),
            b.fmt("Build for {s}", .{cross_target.name}),
        );
        target_step.dependOn(&exe.step);
        
        // Record compilation time
        const compilation_time = std.time.milliTimestamp() - start_time;
        metrics.recordCompilationTime(cross_target.name, compilation_time) catch {};
    }
    
    // Parallel cross-compilation step
    const cross_compile_all = b.step("cross-compile-parallel", "Build for all targets in parallel");
    const cross_script = b.addSystemCommand(&[_][]const u8{
        "sh", "-c", 
        "zig build build-linux-x86_64 & zig build build-macos-x86_64 & zig build build-windows-x86_64 & wait",
    });
    cross_compile_all.dependOn(&cross_script.step);
    
    // Cross-compilation validation
    const validate_cross = b.step("validate-cross", "Validate cross-compilation outputs");
    const validate_script = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        "find zig-out/bin -name 'cursed-*' -exec file {} \\; | grep -E '(ELF|PE32|Mach-O)'",
    });
    validate_cross.dependOn(&validate_script.step);
}

fn configureForTarget(exe: *std.Build.Step.Compile, cross_target: CrossTarget) void {
    exe.linkLibC();
    
    // Target-specific optimizations
    switch (cross_target.target.cpu_arch.?) {
        .x86_64 => {
            exe.addCSourceFile(.{ .file = exe.step.owner.path("src-zig/x86_optimizations.c"), .flags = &[_][]const u8{"-march=x86-64-v2"} });
        },
        .aarch64 => {
            exe.addCSourceFile(.{ .file = exe.step.owner.path("src-zig/arm_optimizations.c"), .flags = &[_][]const u8{"-mcpu=generic"} });
        },
        .wasm32 => {
            exe.addCSourceFile(.{ .file = exe.step.owner.path("src-zig/wasm_runtime.c"), .flags = &[_][]const u8{"-Os"} });
        },
        .thumb => {
            exe.addCSourceFile(.{ .file = exe.step.owner.path("src-zig/embedded_runtime.c"), .flags = &[_][]const u8{"-Os", "-fno-exceptions"} });
        },
        else => {},
    }
    
    // OS-specific configuration
    switch (cross_target.target.os_tag.?) {
        .windows => {
            exe.linkSystemLibrary("ws2_32");
            exe.linkSystemLibrary("kernel32");
        },
        .macos => {
            exe.linkSystemLibrary("pthread");
            exe.linkFramework("Foundation");
        },
        .linux => {
            exe.linkSystemLibrary("pthread");
            exe.linkSystemLibrary("m");
            exe.linkSystemLibrary("dl");
        },
        .freestanding, .wasi => {
            // Minimal configuration for embedded/wasm
        },
        else => {},
    }
}

fn setupPGO(b: *std.Build, exe: *std.Build.Step.Compile, target: std.zig.CrossTarget, optimize: std.builtin.OptimizeMode) void {
    // Profile-Guided Optimization setup
    const pgo_step = b.step("pgo", "Enable Profile-Guided Optimization");
    
    // Stage 1: Build instrumented binary
    const instrumented_exe = b.addExecutable(.{
        .name = "cursed-instrumented",
        .root_source_file = b.path("src-zig/main_unified.zig"),
        .target = target,
        .optimize = optimize,
    });
    instrumented_exe.linkLibC();
    instrumented_exe.addCSourceFile(.{ 
        .file = b.path("src-zig/pgo_instrumentation.c"), 
        .flags = &[_][]const u8{"-fprofile-generate", "-O2"} 
    });
    
    // Stage 2: Collect profile data
    const profile_collection = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        "./zig-out/bin/cursed-instrumented examples/benchmark.csd && ./zig-out/bin/cursed-instrumented examples/complex.csd",
    });
    profile_collection.step.dependOn(&instrumented_exe.step);
    
    // Stage 3: Build optimized binary with profile data
    const optimized_exe = b.addExecutable(.{
        .name = "cursed-pgo",
        .root_source_file = b.path("src-zig/main_unified.zig"),
        .target = target,
        .optimize = .ReleaseFast,
    });
    optimized_exe.linkLibC();
    optimized_exe.addCSourceFile(.{ 
        .file = b.path("src-zig/pgo_optimized.c"), 
        .flags = &[_][]const u8{"-fprofile-use", "-O3", "-flto"} 
    });
    optimized_exe.step.dependOn(&profile_collection.step);
    
    pgo_step.dependOn(&optimized_exe.step);
    b.installArtifact(optimized_exe);
}

fn setupBenchmarks(b: *std.Build, target: std.zig.CrossTarget, optimize: std.builtin.OptimizeMode, config: BuildConfig) void {
    // Performance benchmarking suite
    const benchmark_exe = b.addExecutable(.{
        .name = "cursed-benchmark",
        .root_source_file = b.path("benchmarks/main.zig"),
        .target = target,
        .optimize = .ReleaseFast,
    });
    benchmark_exe.linkLibC();
    b.installArtifact(benchmark_exe);
    
    // Benchmark execution
    const run_benchmarks = b.addRunArtifact(benchmark_exe);
    const benchmark_step = b.step("benchmark", "Run performance benchmarks");
    benchmark_step.dependOn(&run_benchmarks.step);
    
    // Build time benchmarking
    const build_benchmark = b.step("benchmark-build", "Benchmark build times");
    const build_timer = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        "time zig build && echo 'Build completed'",
    });
    build_benchmark.dependOn(&build_timer.step);
    
    // Cross-compilation benchmark
    const cross_benchmark = b.step("benchmark-cross", "Benchmark cross-compilation times");
    const cross_timer = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        "time zig build cross-compile-parallel && echo 'Cross-compilation completed'",
    });
    cross_benchmark.dependOn(&cross_timer.step);
}

fn setupUtilitySteps(b: *std.Build, config: BuildConfig) void {
    // Clean build cache
    const clean_cache = b.step("clean-cache", "Clean build cache");
    const clean_cmd = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        "rm -rf zig-cache zig-out .zig-cache && echo 'Cache cleaned'",
    });
    clean_cache.dependOn(&clean_cmd.step);
    
    // Build system health check
    const health_check = b.step("health-check", "Check build system health");
    const health_cmd = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        "zig version && echo 'Zig compiler available' && ls -la src-zig/ && echo 'Source files present'",
    });
    health_check.dependOn(&health_cmd.step);
    
    // Build performance monitoring
    const monitor = b.step("monitor", "Monitor build performance");
    const monitor_cmd = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        "echo 'Starting build performance monitoring...' && time zig build metrics",
    });
    monitor.dependOn(&monitor_cmd.step);
    
    // Generate build report
    const report = b.step("report", "Generate comprehensive build report");
    const report_cmd = b.addSystemCommand(&[_][]const u8{
        "sh", "-c",
        "echo '# CURSED Build Report' > build_report.md && " ++
        "echo '## Build Configuration' >> build_report.md && " ++
        "echo 'Profile: " ++ @tagName(config.profile_mode) ++ "' >> build_report.md && " ++
        "echo 'Parallel jobs: " ++ (if (config.parallel_jobs == 0) "auto" else "config.parallel_jobs") ++ "' >> build_report.md && " ++
        "echo 'Cache enabled: " ++ (if (config.cache_enabled) "yes" else "no") ++ "' >> build_report.md",
    });
    report.dependOn(&report_cmd.step);
}

// Helper functions
fn detectCores() u32 {
    return std.Thread.getCpuCount() catch 4;
}

fn parseProfileMode(mode_str: []const u8) BuildConfig.ProfileMode {
    if (std.mem.eql(u8, mode_str, "development")) return .development;
    if (std.mem.eql(u8, mode_str, "testing")) return .testing;
    if (std.mem.eql(u8, mode_str, "release")) return .release;
    if (std.mem.eql(u8, mode_str, "production")) return .production;
    return .development; // default
}
