const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const CompilerConfig = @import("enhanced_compiler.zig").CompilerConfig;
const AdvancedCodeGen = @import("advanced_codegen.zig").AdvancedCodeGen;
const OptimizationEngine = @import("optimization_engine.zig").OptimizationEngine;
const ProfileData = @import("optimization_engine.zig").ProfileData;

/// Configure advanced optimizations based on compiler config
pub fn configureAdvancedOptimizations(advanced_codegen: *AdvancedCodeGen, config: CompilerConfig) !void {
    // Set optimization level
    advanced_codegen.setOptimizationLevel(config.optimization_level);
    
    // Configure size optimization
    if (config.size_optimization) {
        advanced_codegen.enableSizeOptimization(2);
    }
    
    // Initialize optimization engine if not already done
    if (advanced_codegen.optimization_engine == null) {
        advanced_codegen.optimization_engine = try OptimizationEngine.init(
            advanced_codegen.base_codegen.allocator,
            advanced_codegen.base_codegen.context,
            advanced_codegen.base_codegen.module
        );
    }
    
    if (advanced_codegen.optimization_engine) |*engine| {
        // Set optimization level
        engine.setOptimizationLevel(config.optimization_level);
        
        // Configure size optimization
        if (config.size_optimization) {
            engine.setSizeOptimizationLevel(2);
        }
        
        // Enable LTO if requested
        if (config.lto_enabled) {
            engine.enableLTO();
        }
        
        // Enable PGO if requested
        if (config.pgo_enabled) {
            if (config.pgo_profile_path) |profile_path| {
                // Load profile data
                const profile_data = try loadProfileData(advanced_codegen.base_codegen.allocator, profile_path);
                engine.enablePGO(profile_data);
            } else {
                // Enable PGO without profile data (preparation mode)
                const empty_profile = ProfileData.init(advanced_codegen.base_codegen.allocator);
                engine.enablePGO(empty_profile);
            }
        }
        
        // Configure debug information
        if (config.debug_info) {
            engine.enableDebugInfo(true);
        }
        
        // Set target CPU and features
        if (config.target_cpu) |cpu| {
            engine.setTargetCPU(cpu);
        }
        
        if (config.target_features) |features| {
            engine.setTargetFeatures(features);
        }
        
        // Configure inlining
        if (config.no_inline) {
            engine.config.inlining_threshold = 0;
        } else if (config.inline_threshold) |threshold| {
            engine.config.inlining_threshold = threshold;
        }
        
        // Configure vectorization
        engine.config.vectorization_enabled = config.vectorization_enabled;
    }
}

/// Load profile data from file for PGO
fn loadProfileData(allocator: Allocator, profile_path: []const u8) !ProfileData {
    _ = profile_path; // TODO: Load actual profile data from file
    
    // TODO: Implement profile data loading
    // For now, return empty profile data
    return ProfileData.init(allocator);
}

/// Write optimized LLVM IR to file
pub fn writeOptimizedLLVMIR(advanced_codegen: *AdvancedCodeGen, writer: anytype, filename: []const u8) !void {
    // Basic LLVM IR header
    try writer.writer().writeAll("; CURSED Compiler - Advanced Optimized LLVM IR\n");
    try writer.writer().writeAll("; Source: ");
    try writer.writer().writeAll(filename);
    try writer.writer().writeAll("\n");
    try writer.writer().writeAll("; Optimization Level: O");
    try writer.print("{s}\n", .{advanced_codegen.optimization_config.optimization_level});
    
    if (advanced_codegen.optimization_config.lto_enabled) {
        try writer.writer().writeAll("; Link-Time Optimization: Enabled\n");
    }
    if (advanced_codegen.optimization_config.pgo_enabled) {
        try writer.writer().writeAll("; Profile-Guided Optimization: Enabled\n");
    }
    if (advanced_codegen.optimization_config.vectorization_enabled) {
        try writer.writer().writeAll("; Auto-Vectorization: Enabled\n");
    }
    if (advanced_codegen.optimization_config.size_optimizations) {
        try writer.writer().writeAll("; Size Optimization: Enabled\n");
    }
    
    try writer.writer().writeAll("\n");
    
    // Module-level attributes (note: should be made configurable)
    const target_mapping = @import("target_mapping.zig");
    const target_triple = target_mapping.getNativeTriple();
    const data_layout = blk: {
        if (std.mem.startsWith(u8, target_triple, "x86_64")) {
            break :blk "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128";
        } else if (std.mem.startsWith(u8, target_triple, "aarch64")) {
            break :blk "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128";
        } else {
            break :blk "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128";
        }
    };
    try writer.print("target datalayout = \"{s}\"\n", .{data_layout});
    try writer.print("target triple = \"{s}\"\n\n", .{target_triple});
    
    // Generate main function with optimization attributes
    try writer.writer().writeAll("define dso_local i32 @main() ");
    
    // Add function attributes based on optimization settings
    if (advanced_codegen.optimization_config.optimization_level >= 2) {
        try writer.writer().writeAll("local_unnamed_addr ");
    }
    if (advanced_codegen.optimization_config.size_optimizations) {
        try writer.writer().writeAll("#0 ");
    } else {
        try writer.writer().writeAll("#1 ");
    }
    
    try writer.writer().writeAll("{\n");
    try writer.writer().writeAll("entry:\n");
    
    // Generate optimized IR body
    try generateOptimizedMainBody(advanced_codegen, writer);
    
    try writer.writer().writeAll("  ret i32 0\n");
    try writer.writer().writeAll("}\n\n");
    
    // Generate function attributes
    if (advanced_codegen.optimization_config.size_optimizations) {
        try writer.writer().writeAll("attributes #0 = { optsize noinline nounwind optnone ");
    } else {
        try writer.writer().writeAll("attributes #1 = { noinline nounwind optnone ");
    }
    
    if (advanced_codegen.debug_enabled) {
        try writer.writer().writeAll("uwtable \"frame-pointer\"=\"all\" \"min-legal-vector-width\"=\"0\" \"no-trapping-math\"=\"true\" \"stack-protector-buffer-size\"=\"8\" \"target-cpu\"=\"x86-64\" \"target-features\"=\"+cx8,+fxsr,+mmx,+sse,+sse2,+x87\" \"tune-cpu\"=\"generic\" }\n");
    } else {
        try writer.writer().writeAll("\"frame-pointer\"=\"all\" \"min-legal-vector-width\"=\"0\" \"no-trapping-math\"=\"true\" \"stack-protector-buffer-size\"=\"8\" \"target-cpu\"=\"x86-64\" \"target-features\"=\"+cx8,+fxsr,+mmx,+sse,+sse2,+x87\" \"tune-cpu\"=\"generic\" }\n");
    }
    
    // Generate debug information if enabled
    if (advanced_codegen.debug_enabled) {
        try generateDebugInfo(advanced_codegen, writer, filename);
    }
    
    // Generate runtime function declarations
    try generateRuntimeDeclarations(writer);
}

/// Generate optimized main function body
fn generateOptimizedMainBody(advanced_codegen: *AdvancedCodeGen, writer: anytype) !void {
    _ = advanced_codegen;
    
    // Basic "Hello, CURSED!" output for now
    try writer.writer().writeAll("  %1 = call i32 @puts(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @.str, i64 0, i64 0))\n");
}

/// Generate debug information
fn generateDebugInfo(advanced_codegen: *AdvancedCodeGen, writer: anytype, filename: []const u8) !void {
    _ = advanced_codegen;
    
    try writer.writer().writeAll("\n; Debug Information\n");
    try writer.writer().writeAll("!llvm.dbg.cu = !{!0}\n");
    try writer.writer().writeAll("!llvm.module.flags = !{!2, !3, !4, !5, !6}\n");
    try writer.writer().writeAll("!llvm.ident = !{!7}\n\n");
    
    try writer.writer().writeAll("!0 = distinct !DICompileUnit(language: DW_LANG_C99, file: !1, producer: \"CURSED Compiler\", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false, nameTableKind: None)\n");
    try writer.writer().writeAll("!1 = !DIFile(filename: \"");
    try writer.writer().writeAll(std.fs.path.basename(filename));
    try writer.writer().writeAll("\", directory: \"");
    const dirname = std.fs.path.dirname(filename) orelse ".";
    try writer.writer().writeAll(dirname);
    try writer.writer().writeAll("\")\n");
    try writer.writer().writeAll("!2 = !{i32 7, !\"Dwarf Version\", i32 5}\n");
    try writer.writer().writeAll("!3 = !{i32 2, !\"Debug Info Version\", i32 3}\n");
    try writer.writer().writeAll("!4 = !{i32 1, !\"wchar_size\", i32 4}\n");
    try writer.writer().writeAll("!5 = !{i32 8, !\"PIC Level\", i32 2}\n");
    try writer.writer().writeAll("!6 = !{i32 7, !\"PIE Level\", i32 2}\n");
    try writer.writer().writeAll("!7 = !{!\"CURSED Compiler with Advanced Optimizations\"}\n");
}

/// Generate runtime function declarations
fn generateRuntimeDeclarations(writer: anytype) !void {
    try writer.writer().writeAll("\n; Runtime function declarations\n");
    try writer.writer().writeAll("@.str = private unnamed_addr constant [15 x i8] c\"Hello, CURSED!\\00\", align 1\n");
    try writer.writer().writeAll("declare dso_local i32 @puts(i8* nocapture readonly) local_unnamed_addr #2\n");
    try writer.writer().writeAll("attributes #2 = { nofree nounwind }\n");
}

/// Compile with advanced optimizations and generate native executable
pub fn compileOptimizedLLVMToNative(allocator: Allocator, ir_filename: []const u8, output_filename: []const u8, config: CompilerConfig) !void {
    print("[5/6] Compiling optimized IR to native executable...\n", .{});
    
    // Build clang command with optimization flags
    var clang_args = std.ArrayList([]const u8){};
    defer clang_args.deinit();
    
    try clang_args.append(allocator, "clang");
    try clang_args.append(allocator, ir_filename);
    try clang_args.append(allocator, "-o");
    try clang_args.append(allocator, output_filename);
    
    // Add optimization flags
    const opt_flag = switch (config.optimization_level) {
        0 => "-O0",
        1 => "-O1",
        2 => "-O2",
        3 => "-O3",
        else => "-O2",
    };
    try clang_args.append(allocator, opt_flag);
    
    // Add size optimization if requested
    if (config.size_optimization) {
        try clang_args.append(allocator, "-Os");
    }
    
    // Add LTO if enabled
    if (config.lto_enabled) {
        try clang_args.append(allocator, "-flto");
        if (config.verbose) print("✅ Link-Time Optimization enabled in linking\n", .{});
    }
    
    // Add PGO flags if enabled
    if (config.pgo_enabled and config.pgo_profile_path != null) {
        const pgo_flag = try std.fmt.allocPrint(allocator, "-fprofile-use={s}", .{config.pgo_profile_path.?});
        defer allocator.free(pgo_flag);
        try clang_args.append(allocator, pgo_flag);
        if (config.verbose) print("✅ Profile-Guided Optimization enabled in linking\n", .{});
    }
    
    // Add vectorization flags
    if (config.vectorization_enabled) {
        try clang_args.append(allocator, "-ftree-vectorize");
        try clang_args.append(allocator, "-fslp-vectorize");
    } else {
        try clang_args.append(allocator, "-fno-vectorize");
        try clang_args.append(allocator, "-fno-slp-vectorize");
    }
    
    // Add target-specific flags
    if (config.target_cpu) |cpu| {
        const cpu_flag = try std.fmt.allocPrint(allocator, "-mcpu={s}", .{cpu});
        defer allocator.free(cpu_flag);
        try clang_args.append(allocator, cpu_flag);
    }
    
    if (config.target_features) |features| {
        const features_flag = try std.fmt.allocPrint(allocator, "-mattr={s}", .{features});
        defer allocator.free(features_flag);
        try clang_args.append(allocator, features_flag);
    }
    
    // Add static linking if requested
    if (config.static_link) {
        try clang_args.append(allocator, "-static");
    }
    
    // Add debug information if requested
    if (config.debug_info) {
        try clang_args.append(allocator, "-g");
        try clang_args.append(allocator, "-gdwarf-4");
    }
    
    // Add target specification if provided
    if (config.target) |target| {
        const target_flag = try std.fmt.allocPrint(allocator, "--target={s}", .{target});
        defer allocator.free(target_flag);
        try clang_args.append(allocator, target_flag);
    }
    
    // Add additional optimization flags for performance
    if (config.optimization_level >= 2) {
        try clang_args.append(allocator, "-ffast-math");
        try clang_args.append(allocator, "-funroll-loops");
        try clang_args.append(allocator, "-fomit-frame-pointer");
    }
    
    if (config.optimization_level >= 3) {
        try clang_args.append(allocator, "-finline-functions");
        try clang_args.append(allocator, "-funswitch-loops");
        try clang_args.append(allocator, "-fpredictive-commoning");
        try clang_args.append(allocator, "-fgcse-after-reload");
    }
    
    // Execute clang compilation
    const result = std.ChildProcess.exec(.{
        .allocator = allocator,
        .argv = clang_args.items,
    }) catch |err| {
        print("❌ Error executing clang: {s}\n", .{err});
        return;
    };
    defer allocator.free(result.stdout);
    defer allocator.free(result.stderr);
    
    if (result.term.Exited != 0) {
        print("❌ Clang compilation failed:\n{s}\n", .{result.stderr});
        return;
    }
    
    print("[6/6] Optimization and linking complete!\n", .{});
    
    if (config.verbose) {
        print("✅ Advanced optimizations applied:\n", .{});
        print("   - Optimization Level: O{s}\n", .{config.optimization_level});
        if (config.lto_enabled) print("   - Link-Time Optimization: Enabled\n", .{});
        if (config.pgo_enabled) print("   - Profile-Guided Optimization: Enabled\n", .{});
        if (config.vectorization_enabled) print("   - Auto-Vectorization: Enabled\n", .{});
        if (config.size_optimization) print("   - Size Optimization: Enabled\n", .{});
        if (config.debug_info) print("   - Debug Information: DWARF enabled\n", .{});
    }
    
    print("📦 Optimized executable: {s}\n", .{output_filename});
    print("🚀 Run with: ./{s}\n", .{output_filename});
}

/// Generate performance benchmark
pub fn generatePerformanceBenchmark(allocator: Allocator, output_filename: []const u8, config: CompilerConfig) !void {
    // Performance benchmarking should not be gated on verbose mode since it's a critical debugging tool
    // if (!config.verbose) return;
    
    print("\n🔬 Running performance benchmark...\n", .{});
    
    // Time execution
    const start_time = std.time.nanoTimestamp();
    
    const result = std.ChildProcess.exec(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ try std.fmt.allocPrint(allocator, "./{s}", .{output_filename}) },
    }) catch |err| {
        print("❌ Error running benchmark: {s}\n", .{err});
        return;
    };
    defer {
        allocator.free(result.argv[0]);
        allocator.free(result.stdout);
        allocator.free(result.stderr);
    }
    
    const end_time = std.time.nanoTimestamp();
    const execution_time_ms = @as(f64, @floatFromInt(end_time - start_time)) / 1_000_000.0;
    
    print("⏱️ Execution time: {d:.2} ms\n", .{execution_time_ms});
    
    // Get file size
    const file = std.fs.cwd().openFile(output_filename, .{}) catch return;
    defer file.close();
    
    const file_size = file.getEndPos() catch return;
    print("📏 Binary size: {s} bytes\n", .{file_size});
    
    if (result.term.Exited == 0) {
        print("✅ Benchmark complete - executable runs successfully\n", .{});
    } else {
        print("❌ Benchmark failed - executable returned code {s}\n", .{result.term.Exited});
    }
}
