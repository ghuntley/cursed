// CURSED Build System Integration for Zig
// Provides native support for building CURSED projects with Zig's build system

const std = @import("std");
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

// CURSED project configuration
pub const CursedProject = struct {
    name: []const u8,
    version: []const u8,
    description: ?[]const u8 = null,
    authors: ArrayList([]const u8),
    source_dir: []const u8 = "src",
    build_dir: []const u8 = "build",
    target_dir: []const u8 = "target",
    main_file: ?[]const u8 = null,
    dependencies: ArrayList(CursedDependency),
    build_config: CursedBuildConfig,
    
    pub fn init(allocator: Allocator, name: []const u8, version: []const u8) CursedProject {
        return CursedProject{
            .name = name,
            .version = version,
            .authors = .empty,
            .dependencies = .empty,
            .build_config = CursedBuildConfig.init(),
        };
    }
    
    pub fn deinit(self: *CursedProject) void {
        self.authors.deinit(self.allocator);
        for (self.dependencies.items) |*dep| {
            dep.deinit();
        }
        self.dependencies.deinit(self.allocator);
    }
    
    pub fn addDependency(self: *CursedProject, dep: CursedDependency) !void {
        try self.dependencies.append(allocator, dep);
    }
    
    pub fn findMainFile(self: *CursedProject, allocator: Allocator) ![]const u8 {
        _ = allocator;
        if (self.main_file) |main| {
            return allocator.dupe(u8, main);
        }
        
        // Default main file search order
        const potential_mains = [_][]const u8{
            "main.💀",
            "src/main.💀", 
            "lib.💀",
            "src/lib.💀",
            "mod.💀",
            "src/mod.💀",
        };
        
        for (potential_mains) |potential_main| {
            const full_path = try std.fs.path.join(allocator, &[_][]const u8{ self.source_dir, potential_main });
            defer allocator.free(full_path);
            
            if (std.fs.cwd().access(full_path, .{})) {
                return allocator.dupe(u8, full_path);
            } else |_| {}
        }
        
        return error.MainFileNotFound;
    }
};

pub const CursedDependency = struct {
    name: []const u8,
    version: []const u8,
    source: DependencySource,
    build_config: ?CursedBuildConfig = null,
    
    const DependencySource = union(enum) {
        registry: struct {
            registry_url: []const u8,
        },
        git: struct {
            url: []const u8,
            branch: ?[]const u8,
            tag: ?[]const u8,
            commit: ?[]const u8,
        },
        path: struct {
            path: []const u8,
        },
        file: struct {
            url: []const u8,
            hash: ?[]const u8,
        },
    };
    
    pub fn deinit(self: *CursedDependency) void {
        _ = self;
        // Nothing to deinit currently, but keeping for future expansions
    }
};

pub const CursedBuildConfig = struct {
    optimization: OptimizationLevel = .debug,
    target_type: TargetType = .executable,
    enable_lto: bool = false,
    enable_debug_info: bool = true,
    c_compiler: CCompiler = .system,
    use_llvm: bool = true,
    static_linking: bool = false,
    strip_symbols: bool = false,
    custom_c_flags: ArrayList([]const u8),
    custom_zig_flags: ArrayList([]const u8),
    
    const OptimizationLevel = enum {
        debug,
        release_safe,
        release_fast,
        release_small,
    };
    
    const TargetType = enum {
        executable,
        static_library,
        dynamic_library,
        object,
    };
    
    const CCompiler = enum {
        system,
        gcc,
        clang,
        zig_cc,
    };
    
    pub fn init() CursedBuildConfig {
        return CursedBuildConfig{
            .custom_c_flags = .empty,
            .custom_zig_flags = .empty,
        };
    }
    
    pub fn toZigOptimization(self: CursedBuildConfig) std.builtin.OptimizeMode {
        return switch (self.optimization) {
            .debug => .Debug,
            .release_safe => .ReleaseSafe,
            .release_fast => .ReleaseFast,
            .release_small => .ReleaseSmall,
        };
    }
};

// Build system integration for CURSED projects
pub const CursedBuilder = struct {
    allocator: Allocator,
    b: *std.Build,
    project: CursedProject,
    cursed_compiler_path: []const u8,
    intermediate_dir: []const u8,
    
    pub fn init(
        allocator: Allocator, 
        b: *std.Build, 
        project: CursedProject,
        cursed_compiler_path: []const u8
    ) CursedBuilder {
        return CursedBuilder{
            .allocator = allocator,
            .b = b,
            .project = project,
            .cursed_compiler_path = cursed_compiler_path,
            .intermediate_dir = "zig-cache/cursed",
        };
    }
    
    // Create a CURSED compilation step
    pub fn addCursedExecutable(
        self: *CursedBuilder,
        target: std.Build.ResolvedTarget,
    ) !*std.Build.Step.Compile {
        const main_file = try self.project.findMainFile(self.allocator);
        defer self.allocator.free(main_file);
        
        // Create intermediate C file path
        const c_output = try std.fs.path.join(
            self.allocator, 
            &[_][]const u8{ self.intermediate_dir, "generated.c" }
        );
        defer self.allocator.free(c_output);
        
        // Create CURSED to C compilation step
        const cursed_compile_step = self.createCursedCompileStep(main_file, c_output);
        
        // Create C executable compilation step
        const exe = self.b.addExecutable(.{
            .name = self.project.name,
            .target = target,
            .optimize = self.project.build_config.toZigOptimization(),
        });
        
        // Add generated C file as source
        exe.addCSourceFile(.{
            .file = self.b.path(c_output),
            .flags = &[_][]const u8{"-std=c99", "-O2"},
        });
        
        // Configure linking and libraries
        try self.configureLinking(exe, target);
        
        // Set up dependencies
        exe.step.dependOn(cursed_compile_step);
        
        return exe;
    }
    
    pub fn addCursedLibrary(
        self: *CursedBuilder,
        target: std.Build.ResolvedTarget,
        lib_type: CursedBuildConfig.TargetType,
    ) !*std.Build.Step.Compile {
        const main_file = try self.project.findMainFile(self.allocator);
        defer self.allocator.free(main_file);
        
        const c_output = try std.fs.path.join(
            self.allocator, 
            &[_][]const u8{ self.intermediate_dir, "lib_generated.c" }
        );
        defer self.allocator.free(c_output);
        
        const cursed_compile_step = self.createCursedCompileStep(main_file, c_output);
        
        const lib = switch (lib_type) {
            .static_library => self.b.addStaticLibrary(.{
                .name = self.project.name,
                .target = target,
                .optimize = self.project.build_config.toZigOptimization(),
            }),
            .dynamic_library => self.b.addSharedLibrary(.{
                .name = self.project.name,
                .target = target,
                .optimize = self.project.build_config.toZigOptimization(),
            }),
            .object => self.b.addObject(.{
                .name = self.project.name,
                .target = target,
                .optimize = self.project.build_config.toZigOptimization(),
            }),
            .executable => unreachable,
        };
        
        lib.addCSourceFile(.{
            .file = self.b.path(c_output),
            .flags = &[_][]const u8{"-std=c99", "-O2"},
        });
        
        try self.configureLinking(lib, target);
        lib.step.dependOn(cursed_compile_step);
        
        return lib;
    }
    
    fn createCursedCompileStep(self: *CursedBuilder, input_file: []const u8, output_file: []const u8) *std.Build.Step {
        // Ensure intermediate directory exists
        const mkdir_step = self.b.addSystemCommand(&[_][]const u8{
            "mkdir", "-p", self.intermediate_dir
        });
        
        // Create CURSED compilation command
        const compile_cmd = self.b.addSystemCommand(&[_][]const u8{
            self.cursed_compiler_path,
            "compile",
            input_file,
            "--output", output_file,
            "--backend", "c",
        });
        
        // Add optimization flags
        switch (self.project.build_config.optimization) {
            .debug => {
                compile_cmd.addArg("--debug");
            },
            .release_fast => {
                compile_cmd.addArg("--optimize=3");
            },
            .release_safe => {
                compile_cmd.addArg("--optimize=2");
                compile_cmd.addArg("--safe");
            },
            .release_small => {
                compile_cmd.addArg("--optimize=s");
            },
        }
        
        if (self.project.build_config.use_llvm) {
            compile_cmd.addArg("--use-llvm");
        }
        
        compile_cmd.step.dependOn(&mkdir_step.step);
        
        return &compile_cmd.step;
    }
    
    fn configureLinking(self: *CursedBuilder, artifact: *std.Build.Step.Compile, target: std.Build.ResolvedTarget) !void {
        // Standard CURSED runtime linking
        artifact.linkLibC();
        
        // Add CURSED runtime
        artifact.addCSourceFile(.{
            .file = self.b.path("runtime/cursed_error_runtime.c"),
            .flags = &[_][]const u8{"-std=c99", "-O2"},
        });
        
        // Platform-specific linking
        switch (target.result.os.tag) {
            .linux => {
                artifact.linkSystemLibrary("pthread");
                artifact.linkSystemLibrary("dl");
                artifact.linkSystemLibrary("m");
                if (self.project.build_config.use_llvm) {
                    artifact.linkSystemLibrary("LLVM-18");
                }
            },
            .macos => {
                artifact.linkSystemLibrary("pthread");
                artifact.linkSystemLibrary("dl");
                artifact.linkSystemLibrary("m");
                artifact.linkFramework("Security");
                artifact.linkFramework("CoreFoundation");
                if (self.project.build_config.use_llvm) {
                    artifact.linkSystemLibrary("LLVM-18");
                }
            },
            .windows => {
                artifact.linkSystemLibrary("pthread");
                artifact.linkSystemLibrary("bcrypt");
                artifact.linkSystemLibrary("ws2_32");
                if (self.project.build_config.use_llvm) {
                    artifact.linkSystemLibrary("LLVM-18");
                }
            },
            else => {},
        }
        
        // Apply static linking if requested
        if (self.project.build_config.static_linking) {
            artifact.linkage = .static;
        }
        
        // Configure debug info
        if (!self.project.build_config.enable_debug_info) {
            // Frame pointer omission is handled by optimization level
        }
        
        // Apply custom flags
        for (self.project.build_config.custom_c_flags.items) |flag| {
            artifact.addCSourceFile(.{
                .file = self.b.path("dummy.c"), // Placeholder
                .flags = &[_][]const u8{flag},
            });
        }
    }
    
    // Create complete build pipeline with testing and documentation
    pub fn createCompleteBuildPipeline(self: *CursedBuilder, target: std.Build.ResolvedTarget) !*std.Build.Step {
        const pipeline_step = self.b.step("cursed-build", "Complete CURSED build pipeline");
        
        // 1. Build main executable
        const exe = try self.addCursedExecutable(target);
        self.b.installArtifact(exe);
        pipeline_step.dependOn(&exe.step);
        
        // 2. Build tests if test directory exists
        if (self.directoryExists("tests") or self.directoryExists("test")) {
            const test_step = try self.createTestStep(target);
            pipeline_step.dependOn(test_step);
        }
        
        // 3. Build documentation if docs directory exists  
        if (self.directoryExists("docs")) {
            const docs_step = try self.createDocsStep();
            pipeline_step.dependOn(docs_step);
        }
        
        // 4. Build benchmarks if benchmarks directory exists
        if (self.directoryExists("benchmarks")) {
            const bench_step = try self.createBenchmarkStep(target);
            pipeline_step.dependOn(bench_step);
        }
        
        return pipeline_step;
    }
    
    fn createTestStep(self: *CursedBuilder, target: std.Build.ResolvedTarget) !*std.Build.Step {
        _ = target;
        const test_step = self.b.step("cursed-test", "Run CURSED tests");
        
        // Find all test files
        const test_files = try self.findFilesWithExtension("tests", ".💀");
        defer test_files.deinit();
        defer for (test_files.items) |file| {
            self.allocator.free(file);
        };
        
        for (test_files.items) |test_file| {
            const test_cmd = self.b.addSystemCommand(&[_][]const u8{
                self.cursed_compiler_path,
                "test",
                test_file,
            });
            test_step.dependOn(&test_cmd.step);
        }
        
        return test_step;
    }
    
    fn createDocsStep(self: *CursedBuilder) !*std.Build.Step {
        const docs_step = self.b.step("cursed-docs", "Generate CURSED documentation");
        
        const docs_cmd = self.b.addSystemCommand(&[_][]const u8{
            self.cursed_compiler_path,
            "doc",
            "--source-dir", self.project.source_dir,
            "--output-dir", "docs/generated",
        });
        
        docs_step.dependOn(&docs_cmd.step);
        return docs_step;
    }
    
    fn createBenchmarkStep(self: *CursedBuilder, target: std.Build.ResolvedTarget) !*std.Build.Step {
        const bench_step = self.b.step("cursed-bench", "Run CURSED benchmarks");
        
        const bench_files = try self.findFilesWithExtension("benchmarks", ".💀");
        defer bench_files.deinit();
        defer for (bench_files.items) |file| {
            self.allocator.free(file);
        };
        
        for (bench_files.items) |_| {
            // Compile benchmark as optimized executable
            const saved_optimization = self.project.build_config.optimization;
            self.project.build_config.optimization = .release_fast;
            
            const bench_exe = try self.addCursedExecutable(target);
            
            self.project.build_config.optimization = saved_optimization;
            
            const bench_run = self.b.addRunArtifact(bench_exe);
            bench_step.dependOn(&bench_run.step);
        }
        
        return bench_step;
    }
    
    fn directoryExists(self: *CursedBuilder, dir_name: []const u8) bool {
        _ = self;
        std.fs.cwd().access(dir_name, .{}) catch return false;
        return true;
    }
    
    fn findFilesWithExtension(self: *CursedBuilder, dir_name: []const u8, extension: []const u8) !ArrayList([]const u8) {
        var files = std.ArrayList(u8){};
        
        var dir = std.fs.cwd().openDir(dir_name, .{ .iterate = true }) catch return files;
        defer dir.close();
        
        var iterator = dir.iterate();
        while (try iterator.next()) |entry| {
            if (entry.kind == .file and std.mem.endsWith(u8, entry.name, extension)) {
                const full_path = try std.fs.path.join(
                    self.allocator, 
                    &[_][]const u8{ dir_name, entry.name }
                );
                try files.append(self.allocator, full_path);
            }
        }
        
        return files;
    }
};

// Helper functions for integration with build.zig
pub fn loadCursedProject(allocator: Allocator, project_file: []const u8) !CursedProject {
    const file = std.fs.cwd().openFile(project_file, .{}) catch |err| switch (err) {
        error.FileNotFound => {
            // Create default project
            return CursedProject.init(allocator, "cursed-project", "0.1.0");
        },
        else => return err,
    };
    defer file.close();
    
    const content = try file.readToEndAlloc(allocator, 1024 * 1024);
    defer allocator.free(content);
    
    // Parse TOML (simplified - would use proper TOML parser in production)
    var project = CursedProject.init(allocator, "default", "0.1.0");
    
    // Basic TOML parsing for name and version
    var lines = std.mem.split(u8, content, "\n");
    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " \t\r\n");
        if (std.mem.startsWith(u8, trimmed, "name = ")) {
            const quote_start = std.mem.indexOf(u8, trimmed, "\"") orelse continue;
            const quote_end = std.mem.lastIndexOf(u8, trimmed, "\"") orelse continue;
            if (quote_end > quote_start) {
                project.name = try allocator.dupe(u8, trimmed[quote_start + 1..quote_end]);
            }
        } else if (std.mem.startsWith(u8, trimmed, "version = ")) {
            const quote_start = std.mem.indexOf(u8, trimmed, "\"") orelse continue;
            const quote_end = std.mem.lastIndexOf(u8, trimmed, "\"") orelse continue;
            if (quote_end > quote_start) {
                project.version = try allocator.dupe(u8, trimmed[quote_start + 1..quote_end]);
            }
        }
    }
    
    return project;
}

pub fn createCursedBuildStep(
    b: *std.Build,
    target: std.Build.ResolvedTarget,
    optimize: std.builtin.OptimizeMode,
    cursed_compiler_path: []const u8
) !void {
    var project = try loadCursedProject(b.allocator, "CursedPackage.toml");
    defer project.deinit();
    
    // Set optimization from Zig build
    project.build_config.optimization = switch (optimize) {
        .Debug => .debug,
        .ReleaseSafe => .release_safe,
        .ReleaseFast => .release_fast,
        .ReleaseSmall => .release_small,
    };
    
    var builder = CursedBuilder.init(b.allocator, b, project, cursed_compiler_path);
    
    // Create complete build pipeline
    const pipeline = try builder.createCompleteBuildPipeline(target);
    
    // Make it the default step
    b.default_step = pipeline;
    
    print("CURSED build integration enabled for project: {s} v{s}\n", .{ project.name, project.version });
}

// Test CURSED build system integration
test "cursed project loading" {
    const allocator = std.testing.allocator;
    
    // Test default project creation
    var project = CursedProject.init(allocator, "test-project", "1.0.0");
    defer project.deinit();
    
    try std.testing.expectEqualStrings("test-project", project.name);
    try std.testing.expectEqualStrings("1.0.0", project.version);
    try std.testing.expectEqualStrings("src", project.source_dir);
}

test "cursed build config" {
    var config = CursedBuildConfig.init();
    
    try std.testing.expect(config.optimization == .debug);
    try std.testing.expect(config.target_type == .executable);
    try std.testing.expect(config.toZigOptimization() == .Debug);
    
    config.optimization = .release_fast;
    try std.testing.expect(config.toZigOptimization() == .ReleaseFast);
}
