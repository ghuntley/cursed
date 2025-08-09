/// CURSED Programming Language - Enhanced CLI Framework
/// 
/// Comprehensive command-line interface with proper argument parsing,
/// subcommands, validation, and professional user experience.

const std = @import("std");
const builtin = @import("builtin");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const HashMap = std.HashMap;

/// Configuration file support
const ConfigFile = struct {
    profile: []const u8,
    optimization_level: u8,
    target: []const u8,
    verbose: bool,
    color: bool,
    max_errors: usize,
    
    pub fn init() ConfigFile {
        return ConfigFile{
            .profile = "release",
            .optimization_level = 2,
            .target = "native",
            .verbose = false,
            .color = true,
            .max_errors = 100,
        };
    }
    
    pub fn loadFromFile(allocator: Allocator, path: []const u8) !ConfigFile {
        _ = allocator;
        _ = path;
        // TODO: Implement TOML parsing for configuration files
        return ConfigFile.init();
    }
};

/// Argument parsing and validation
pub const ArgParser = struct {
    allocator: Allocator,
    program_name: []const u8,
    global_args: GlobalArgs,
    subcommand: ?Subcommand,
    
    pub const GlobalArgs = struct {
        verbose: bool = false,
        quiet: bool = false,
        color: ColorMode = .auto,
        optimization: u8 = 2,
        target: []const u8 = "native",
        profile: []const u8 = "release",
        explain: ?[]const u8 = null,
        list_error_codes: bool = false,
        max_errors: usize = 100,
        json_errors: bool = false,
        platform_info: bool = false,
        version_verbose: bool = false,
        runtime_stats: bool = false,
        hardware_info: bool = false,
        file: ?[]const u8 = null,
        
        pub fn shouldShowColor(self: GlobalArgs) bool {
            return switch (self.color) {
                .auto => true, // Simplified for now - TODO: Implement proper TTY detection
                .always => true,
                .never => false,
            };
        }
    };
    
    pub const ColorMode = enum {
        auto,
        always,
        never,
        
        pub fn fromString(s: []const u8) ?ColorMode {
            if (std.mem.eql(u8, s, "auto")) return .auto;
            if (std.mem.eql(u8, s, "always")) return .always;
            if (std.mem.eql(u8, s, "never")) return .never;
            return null;
        }
    };
    
    pub const Subcommand = union(enum) {
        compile: CompileArgs,
        run: RunArgs,
        test_cmd: TestArgs,
        coverage: CoverageArgs,
        debug: DebugArgs,
        repl: ReplArgs,
        pkg: PkgArgs,
        lint: LintArgs,
        fmt: FmtArgs,
        doc: DocArgs,
        lsp: LspArgs,
        build: BuildArgs,
        clean: CleanArgs,
        check: CheckArgs,
        explain: ExplainArgs,
        version: VersionArgs,
        
        pub const CompileArgs = struct {
            input: ?[]const u8 = null,
            output: ?[]const u8 = null,
            emit_ir: bool = false,
            emit_asm: bool = false,
            no_link: bool = false,
            native_only: bool = false,
            check_deps: bool = false,
            optimize: bool = false,
            opt_level: ?u8 = null,
            enable_pgo: bool = false,
            pgo_profile: ?[]const u8 = null,
            pgo_generate: bool = false,
            enable_lto: bool = false,
            lto_level: ?[]const u8 = null,
            size_opt: bool = false,
            size_level: ?[]const u8 = null,
            enable_bolt: bool = false,
            bolt_profile: ?[]const u8 = null,
            pass_pipeline: ?[]const u8 = null,
            enable_inlining: bool = false,
            inline_threshold: ?u32 = null,
            aggressive_inline: bool = false,
            inline_generics: bool = false,
            inline_interfaces: bool = false,
            benchmark: bool = false,
        };
        
        pub const RunArgs = struct {
            input: []const u8,
            jit: bool = false,
            interpreter: bool = false,
        };
        
        pub const TestArgs = struct {
            test_dir: []const u8 = "stdlib",
            pattern: []const u8 = "test_*.csd",
            filter: ?[]const u8 = null,
            parallel: bool = false,
            timeout: u64 = 30,
            fail_fast: bool = false,
            output_format: []const u8 = "pretty",
            coverage: bool = false,
            coverage_format: []const u8 = "html",
            coverage_threshold: f64 = 80.0,
        };
        
        pub const CoverageArgs = struct {
            // Coverage-specific args would go here
            subcmd: CoverageSubcommand,
        };
        
        pub const CoverageSubcommand = union(enum) {
            run: CoverageRunArgs,
            report: CoverageReportArgs,
            instrument: CoverageInstrumentArgs,
            
            pub const CoverageRunArgs = struct {
                test_command: []const u8 = "cursed test",
                output_dir: []const u8 = "coverage",
                source_dirs: ArrayList([]const u8),
                exclude_patterns: ArrayList([]const u8),
                formats: ArrayList([]const u8),
                threshold: f64 = 80.0,
                collect_branches: bool = false,
                collect_functions: bool = false,
                instrument: bool = false,
            };
            
            pub const CoverageReportArgs = struct {
                coverage_data: []const u8,
                output_dir: []const u8 = "coverage",
                formats: ArrayList([]const u8),
            };
            
            pub const CoverageInstrumentArgs = struct {
                source_dirs: ArrayList([]const u8),
                output_dir: []const u8,
            };
        };
        
        pub const DebugArgs = struct {
            input: []const u8,
            breakpoints: ArrayList([]const u8),
            watch_vars: ArrayList([]const u8),
            debug_symbols: bool = true,
            memory_debug: bool = false,
            trace: bool = false,
        };
        
        pub const ReplArgs = struct {
            startup_file: ?[]const u8 = null,
            history_file: ?[]const u8 = null,
            no_history: bool = false,
            multiline: bool = true,
        };
        
        pub const PkgArgs = struct {
            subcmd: PkgSubcommand,
        };
        
        pub const PkgSubcommand = union(enum) {
            init: PkgInitArgs,
            add: PkgAddArgs,
            remove: PkgRemoveArgs,
            update: PkgUpdateArgs,
            list: PkgListArgs,
            search: PkgSearchArgs,
            publish: PkgPublishArgs,
            install: PkgInstallArgs,
            
            pub const PkgInitArgs = struct {
                name: ?[]const u8 = null,
                template: []const u8 = "basic",
            };
            
            pub const PkgAddArgs = struct {
                package: []const u8,
                version: ?[]const u8 = null,
                dev: bool = false,
            };
            
            pub const PkgRemoveArgs = struct {
                package: []const u8,
            };
            
            pub const PkgUpdateArgs = struct {
                package: ?[]const u8 = null,
            };
            
            pub const PkgListArgs = struct {
                outdated: bool = false,
                tree: bool = false,
            };
            
            pub const PkgSearchArgs = struct {
                query: []const u8,
                limit: usize = 10,
            };
            
            pub const PkgPublishArgs = struct {
                dry_run: bool = false,
                allow_dirty: bool = false,
            };
            
            pub const PkgInstallArgs = struct {
                global: bool = false,
                force: bool = false,
            };
        };
        
        pub const LintArgs = struct {
            input: []const u8,
            fix: bool = false,
            rules: ?[]const u8 = null,
            config: ?[]const u8 = null,
        };
        
        pub const FmtArgs = struct {
            input: []const u8,
            check: bool = false,
            diff: bool = false,
            backup: bool = false,
        };
        
        pub const DocArgs = struct {
            input: []const u8,
            output_dir: []const u8 = "docs",
            format: []const u8 = "html",
            private: bool = false,
            include_source: bool = true,
        };
        
        pub const LspArgs = struct {
            port: ?u16 = null,
            stdio: bool = true,
            log_file: ?[]const u8 = null,
            log_level: []const u8 = "info",
        };
        
        pub const BuildArgs = struct {
            target: ?[]const u8 = null,
            profile: ?[]const u8 = null,
            features: ArrayList([]const u8),
            jobs: ?usize = null,
        };
        
        pub const CleanArgs = struct {
            target_dir: bool = false,
            packages: bool = false,
            all: bool = false,
        };
        
        pub const CheckArgs = struct {
            input: []const u8,
            no_deps: bool = false,
            workspace: bool = false,
        };
        
        pub const ExplainArgs = struct {
            error_code: []const u8,
        };
        
        pub const VersionArgs = struct {
            verbose: bool = false,
        };
    };
    
    pub fn init(allocator: Allocator, program_name: []const u8) ArgParser {
        return ArgParser{
            .allocator = allocator,
            .program_name = program_name,
            .global_args = GlobalArgs{},
            .subcommand = null,
        };
    }
    
    pub fn parse(self: *ArgParser, args: [][]const u8) !void {
        if (args.len == 0) {
            try self.printHelp();
            return;
        }
        
        var i: usize = 0;
        while (i < args.len) {
            const arg = args[i];
            
            // Handle global flags
            if (std.mem.eql(u8, arg, "--verbose") or std.mem.eql(u8, arg, "-v")) {
                self.global_args.verbose = true;
            } else if (std.mem.eql(u8, arg, "--quiet") or std.mem.eql(u8, arg, "-q")) {
                self.global_args.quiet = true;
            } else if (std.mem.startsWith(u8, arg, "--color=")) {
                const color_value = arg[8..];
                self.global_args.color = ColorMode.fromString(color_value) orelse {
                    return error.InvalidColorMode;
                };
            } else if (std.mem.startsWith(u8, arg, "--optimization=") or std.mem.startsWith(u8, arg, "-O")) {
                const opt_str = if (std.mem.startsWith(u8, arg, "--optimization=")) arg[15..] else arg[2..];
                self.global_args.optimization = std.fmt.parseUnsigned(u8, opt_str, 10) catch {
                    return error.InvalidOptimizationLevel;
                };
            } else if (std.mem.startsWith(u8, arg, "--target=")) {
                self.global_args.target = arg[9..];
            } else if (std.mem.eql(u8, arg, "--help") or std.mem.eql(u8, arg, "-h")) {
                try self.printHelp();
                return;
            } else if (std.mem.eql(u8, arg, "--version")) {
                try self.printVersion();
                return;
            } else if (std.mem.eql(u8, arg, "--version-verbose")) {
                self.global_args.version_verbose = true;
                try self.printVersionVerbose();
                return;
            } else if (std.mem.eql(u8, arg, "--platform-info")) {
                self.global_args.platform_info = true;
                try self.printPlatformInfo();
                return;
            } else if (std.mem.eql(u8, arg, "--list-error-codes")) {
                self.global_args.list_error_codes = true;
                try self.printErrorCodes();
                return;
            } else if (std.mem.startsWith(u8, arg, "--explain=")) {
                self.global_args.explain = arg[10..];
                try self.explainError(arg[10..]);
                return;
            } else if (std.mem.startsWith(u8, arg, "--max-errors=")) {
                const count_str = arg[13..];
                self.global_args.max_errors = std.fmt.parseUnsigned(usize, count_str, 10) catch {
                    return error.InvalidMaxErrors;
                };
            } else if (std.mem.eql(u8, arg, "--json-errors")) {
                self.global_args.json_errors = true;
            } else if (std.mem.eql(u8, arg, "--runtime-stats")) {
                self.global_args.runtime_stats = true;
                try self.printRuntimeStats();
                return;
            } else if (std.mem.eql(u8, arg, "--hardware-info")) {
                self.global_args.hardware_info = true;
                try self.printHardwareInfo();
                return;
            } else if (std.mem.eql(u8, arg, "compile")) {
                self.subcommand = Subcommand{ .compile = try self.parseCompileArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "run")) {
                self.subcommand = Subcommand{ .run = try self.parseRunArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "test")) {
                self.subcommand = Subcommand{ .test_cmd = try self.parseTestArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "coverage")) {
                self.subcommand = Subcommand{ .coverage = try self.parseCoverageArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "debug")) {
                self.subcommand = Subcommand{ .debug = try self.parseDebugArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "repl")) {
                self.subcommand = Subcommand{ .repl = try self.parseReplArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "pkg")) {
                self.subcommand = Subcommand{ .pkg = try self.parsePkgArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "lint")) {
                self.subcommand = Subcommand{ .lint = try self.parseLintArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "fmt")) {
                self.subcommand = Subcommand{ .fmt = try self.parseFmtArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "doc")) {
                self.subcommand = Subcommand{ .doc = try self.parseDocArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "lsp")) {
                self.subcommand = Subcommand{ .lsp = try self.parseLspArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "build")) {
                self.subcommand = Subcommand{ .build = try self.parseBuildArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "clean")) {
                self.subcommand = Subcommand{ .clean = try self.parseCleanArgs(args[i+1..]) };
                break;
            } else if (std.mem.eql(u8, arg, "check")) {
                self.subcommand = Subcommand{ .check = try self.parseCheckArgs(args[i+1..]) };
                break;
            } else if (!std.mem.startsWith(u8, arg, "-")) {
                // Backward compatibility: treat as file to run
                self.global_args.file = arg;
                self.subcommand = Subcommand{ .run = Subcommand.RunArgs{ .input = arg } };
                break;
            } else {
                print("Unknown argument: {s}\n", .{arg});
                return error.UnknownArgument;
            }
            
            i += 1;
        }
        
        // If no subcommand and no file, show help
        if (self.subcommand == null and self.global_args.file == null) {
            try self.printHelp();
        }
    }
    
    fn parseCompileArgs(self: *ArgParser, args: [][]const u8) !Subcommand.CompileArgs {
        _ = self;
        var compile_args = Subcommand.CompileArgs{};
        
        var i: usize = 0;
        while (i < args.len) {
            const arg = args[i];
            
            if (std.mem.startsWith(u8, arg, "--output=")) {
                compile_args.output = arg[9..];
            } else if (std.mem.startsWith(u8, arg, "-o=")) {
                compile_args.output = arg[3..];
            } else if (std.mem.eql(u8, arg, "--emit-ir")) {
                compile_args.emit_ir = true;
            } else if (std.mem.eql(u8, arg, "--emit-asm")) {
                compile_args.emit_asm = true;
            } else if (std.mem.eql(u8, arg, "--no-link")) {
                compile_args.no_link = true;
            } else if (std.mem.eql(u8, arg, "--native-only")) {
                compile_args.native_only = true;
            } else if (std.mem.eql(u8, arg, "--check-deps")) {
                compile_args.check_deps = true;
            } else if (std.mem.eql(u8, arg, "--optimize")) {
                compile_args.optimize = true;
            } else if (std.mem.startsWith(u8, arg, "--opt-level=")) {
                const level_str = arg[12..];
                compile_args.opt_level = std.fmt.parseUnsigned(u8, level_str, 10) catch {
                    return error.InvalidOptLevel;
                };
            } else if (std.mem.eql(u8, arg, "--enable-pgo")) {
                compile_args.enable_pgo = true;
            } else if (std.mem.startsWith(u8, arg, "--pgo-profile=")) {
                compile_args.pgo_profile = arg[14..];
                compile_args.enable_pgo = true;
            } else if (std.mem.eql(u8, arg, "--pgo-generate")) {
                compile_args.pgo_generate = true;
            } else if (std.mem.eql(u8, arg, "--enable-lto")) {
                compile_args.enable_lto = true;
            } else if (std.mem.startsWith(u8, arg, "--lto-level=")) {
                compile_args.lto_level = arg[12..];
                compile_args.enable_lto = true;
            } else if (std.mem.eql(u8, arg, "--size-opt")) {
                compile_args.size_opt = true;
            } else if (std.mem.startsWith(u8, arg, "--size-level=")) {
                compile_args.size_level = arg[13..];
                compile_args.size_opt = true;
            } else if (std.mem.eql(u8, arg, "--enable-bolt")) {
                compile_args.enable_bolt = true;
            } else if (std.mem.startsWith(u8, arg, "--bolt-profile=")) {
                compile_args.bolt_profile = arg[15..];
                compile_args.enable_bolt = true;
            } else if (std.mem.startsWith(u8, arg, "--pass-pipeline=")) {
                compile_args.pass_pipeline = arg[16..];
            } else if (std.mem.eql(u8, arg, "--enable-inlining")) {
                compile_args.enable_inlining = true;
            } else if (std.mem.startsWith(u8, arg, "--inline-threshold=")) {
                const threshold_str = arg[19..];
                compile_args.inline_threshold = std.fmt.parseUnsigned(u32, threshold_str, 10) catch {
                    return error.InvalidInlineThreshold;
                };
                compile_args.enable_inlining = true;
            } else if (std.mem.eql(u8, arg, "--aggressive-inline")) {
                compile_args.aggressive_inline = true;
                compile_args.enable_inlining = true;
            } else if (std.mem.eql(u8, arg, "--inline-generics")) {
                compile_args.inline_generics = true;
                compile_args.enable_inlining = true;
            } else if (std.mem.eql(u8, arg, "--inline-interfaces")) {
                compile_args.inline_interfaces = true;
                compile_args.enable_inlining = true;
            } else if (std.mem.eql(u8, arg, "--benchmark")) {
                compile_args.benchmark = true;
            } else if (!std.mem.startsWith(u8, arg, "-")) {
                // Input file
                if (compile_args.input == null) {
                    compile_args.input = arg;
                } else {
                    return error.MultipleInputFiles;
                }
            } else {
                print("Unknown compile argument: {s}\n", .{arg});
                return error.UnknownCompileArgument;
            }
            
            i += 1;
        }
        
        return compile_args;
    }
    
    fn parseRunArgs(self: *ArgParser, args: [][]const u8) !Subcommand.RunArgs {
        _ = self;
        var run_args = Subcommand.RunArgs{ .input = "" };
        
        var i: usize = 0;
        while (i < args.len) {
            const arg = args[i];
            
            if (std.mem.eql(u8, arg, "--jit")) {
                run_args.jit = true;
            } else if (std.mem.eql(u8, arg, "--interpreter")) {
                run_args.interpreter = true;
            } else if (!std.mem.startsWith(u8, arg, "-")) {
                // Input file
                if (run_args.input.len == 0) {
                    run_args.input = arg;
                } else {
                    return error.MultipleInputFiles;
                }
            } else {
                print("Unknown run argument: {s}\n", .{arg});
                return error.UnknownRunArgument;
            }
            
            i += 1;
        }
        
        if (run_args.input.len == 0) {
            return error.NoInputFile;
        }
        
        return run_args;
    }
    
    fn parseTestArgs(self: *ArgParser, args: [][]const u8) !Subcommand.TestArgs {
        _ = self;
        var test_args = Subcommand.TestArgs{};
        
        var i: usize = 0;
        while (i < args.len) {
            const arg = args[i];
            
            if (std.mem.startsWith(u8, arg, "--test-dir=")) {
                test_args.test_dir = arg[11..];
            } else if (std.mem.startsWith(u8, arg, "--pattern=")) {
                test_args.pattern = arg[10..];
            } else if (std.mem.startsWith(u8, arg, "--filter=")) {
            test_args.filter = arg[9..];
            } else if (std.mem.startsWith(u8, arg, "-f=")) {
                test_args.filter = arg[3..];
            } else if (std.mem.eql(u8, arg, "--parallel") or std.mem.eql(u8, arg, "-p")) {
                test_args.parallel = true;
            } else if (std.mem.startsWith(u8, arg, "--timeout=")) {
                const timeout_str = arg[10..];
                test_args.timeout = std.fmt.parseUnsigned(u64, timeout_str, 10) catch {
                    return error.InvalidTimeout;
                };
            } else if (std.mem.eql(u8, arg, "--fail-fast")) {
                test_args.fail_fast = true;
            } else if (std.mem.startsWith(u8, arg, "--format=")) {
                test_args.output_format = arg[9..];
            } else if (std.mem.eql(u8, arg, "--coverage")) {
                test_args.coverage = true;
            } else if (std.mem.startsWith(u8, arg, "--coverage-format=")) {
                test_args.coverage_format = arg[18..];
            } else if (std.mem.startsWith(u8, arg, "--coverage-threshold=")) {
                const threshold_str = arg[21..];
                test_args.coverage_threshold = std.fmt.parseFloat(f64, threshold_str) catch {
                    return error.InvalidCoverageThreshold;
                };
            } else {
                print("Unknown test argument: {s}\n", .{arg});
                return error.UnknownTestArgument;
            }
            
            i += 1;
        }
        
        return test_args;
    }
    
    // Placeholder implementations for other subcommand parsers
    fn parseCoverageArgs(self: *ArgParser, args: [][]const u8) !Subcommand.CoverageArgs {
        _ = args;
        return Subcommand.CoverageArgs{ .subcmd = Subcommand.CoverageSubcommand{ .run = Subcommand.CoverageSubcommand.CoverageRunArgs{
            .source_dirs = ArrayList([]const u8).init(self.allocator),
            .exclude_patterns = ArrayList([]const u8).init(self.allocator),
            .formats = ArrayList([]const u8).init(self.allocator),
        } } };
    }
    
    fn parseDebugArgs(self: *ArgParser, args: [][]const u8) !Subcommand.DebugArgs {
        _ = args;
        return Subcommand.DebugArgs{
            .input = "",
            .breakpoints = ArrayList([]const u8).init(self.allocator),
            .watch_vars = ArrayList([]const u8).init(self.allocator),
        };
    }
    
    fn parseReplArgs(self: *ArgParser, args: [][]const u8) !Subcommand.ReplArgs {
        _ = self;
        _ = args;
        return Subcommand.ReplArgs{};
    }
    
    fn parsePkgArgs(self: *ArgParser, args: [][]const u8) !Subcommand.PkgArgs {
        _ = self;
        _ = args;
        return Subcommand.PkgArgs{ .subcmd = Subcommand.PkgSubcommand{ .list = Subcommand.PkgSubcommand.PkgListArgs{} } };
    }
    
    fn parseLintArgs(self: *ArgParser, args: [][]const u8) !Subcommand.LintArgs {
        _ = self;
        _ = args;
        return Subcommand.LintArgs{ .input = "" };
    }
    
    fn parseFmtArgs(self: *ArgParser, args: [][]const u8) !Subcommand.FmtArgs {
        _ = self;
        _ = args;
        return Subcommand.FmtArgs{ .input = "" };
    }
    
    fn parseDocArgs(self: *ArgParser, args: [][]const u8) !Subcommand.DocArgs {
        _ = self;
        _ = args;
        return Subcommand.DocArgs{ .input = "" };
    }
    
    fn parseLspArgs(self: *ArgParser, args: [][]const u8) !Subcommand.LspArgs {
        _ = self;
        _ = args;
        return Subcommand.LspArgs{};
    }
    
    fn parseBuildArgs(self: *ArgParser, args: [][]const u8) !Subcommand.BuildArgs {
        _ = args;
        return Subcommand.BuildArgs{ .features = ArrayList([]const u8).init(self.allocator) };
    }
    
    fn parseCleanArgs(self: *ArgParser, args: [][]const u8) !Subcommand.CleanArgs {
        _ = self;
        _ = args;
        return Subcommand.CleanArgs{};
    }
    
    fn parseCheckArgs(self: *ArgParser, args: [][]const u8) !Subcommand.CheckArgs {
        _ = self;
        _ = args;
        return Subcommand.CheckArgs{ .input = "" };
    }
    
    pub fn printHelp(self: *ArgParser) !void {
        const use_color = self.global_args.shouldShowColor();
        
        if (use_color) {
            print("\x1b[1;36mCURSED Programming Language\x1b[0m v1.0.0\n", .{});
            print("\x1b[32mGen Z slang meets Go-like grammar\x1b[0m\n\n", .{});
        } else {
            print("CURSED Programming Language v1.0.0\n", .{});
            print("Gen Z slang meets Go-like grammar\n\n", .{});
        }
        
        print("USAGE:\n", .{});
        print("    {s} [GLOBAL_OPTIONS] <SUBCOMMAND> [ARGS...]\n", .{self.program_name});
        print("    {s} [GLOBAL_OPTIONS] <FILE>  # Backward compatibility\n\n", .{self.program_name});
        
        print("GLOBAL OPTIONS:\n", .{});
        print("    -v, --verbose              Enable verbose output\n", .{});
        print("    -q, --quiet                Suppress output\n", .{});
        print("        --color <WHEN>         Control colored output [auto, always, never]\n", .{});
        print("    -O, --optimization <LEVEL> Optimization level [0-3, s, z, debug, release]\n", .{});
        print("        --target <TARGET>      Target architecture [native, wasm, wasm32, wasm64]\n", .{});
        print("        --profile <PROFILE>    Build profile [debug, release, test]\n", .{});
        print("        --explain <CODE>       Explain an error code\n", .{});
        print("        --list-error-codes     List all available error codes\n", .{});
        print("        --max-errors <COUNT>   Maximum number of errors to report\n", .{});
        print("        --json-errors          Output errors in JSON format\n", .{});
        print("        --platform-info        Show platform information\n", .{});
        print("        --version-verbose      Show detailed version information\n", .{});
        print("        --runtime-stats        Show runtime statistics\n", .{});
        print("        --hardware-info        Show hardware feature detection\n", .{});
        print("    -h, --help                 Print this help message\n", .{});
        print("        --version              Print version information\n\n", .{});
        
        print("SUBCOMMANDS:\n", .{});
        print("    compile                    Compile CURSED source to executable\n", .{});
        print("    run                        Execute CURSED source file\n", .{});
        print("    test                       Run CURSED tests\n", .{});
        print("    coverage                   Code coverage analysis\n", .{});
        print("    debug                      Interactive debugging\n", .{});
        print("    repl                       Start interactive REPL\n", .{});
        print("    pkg                        Package management\n", .{});
        print("    lint                       Lint CURSED source code\n", .{});
        print("    fmt                        Format CURSED source code\n", .{});
        print("    doc                        Generate documentation\n", .{});
        print("    lsp                        Start Language Server Protocol server\n", .{});
        print("    build                      Build project\n", .{});
        print("    clean                      Clean build artifacts\n", .{});
        print("    check                      Check code without building\n\n", .{});
        
        print("Use '{s} <SUBCOMMAND> --help' for more information on a specific command.\n", .{self.program_name});
    }
    
    pub fn printVersion(self: *ArgParser) !void {
        _ = self;
        print("CURSED v1.0.0-zig\n", .{});
    }
    
    pub fn printVersionVerbose(self: *ArgParser) !void {
        const use_color = self.global_args.shouldShowColor();
        
        if (use_color) {
            print("\x1b[1;36mCURSED Programming Language\x1b[0m\n", .{});
            print("Version: \x1b[1;32mv1.0.0-zig\x1b[0m\n", .{});
            print("Build Date: \x1b[33m2025-01-09\x1b[0m\n", .{});
            print("Commit: \x1b[33munified-cli-enhancement\x1b[0m\n", .{});
            print("Host: \x1b[36m{s}\x1b[0m\n", .{@tagName(builtin.os.tag)});
            print("Target: \x1b[36m{s}\x1b[0m\n", .{self.global_args.target});
        } else {
            print("CURSED Programming Language\n", .{});
            print("Version: v1.0.0-zig\n", .{});
            print("Build Date: 2025-01-09\n", .{});
            print("Commit: unified-cli-enhancement\n", .{});
            print("Host: {s}\n", .{@tagName(builtin.os.tag)});
            print("Target: {s}\n", .{self.global_args.target});
        }
    }
    
    pub fn printPlatformInfo(self: *ArgParser) !void {
        const use_color = self.global_args.shouldShowColor();
        
        if (use_color) {
            print("\x1b[1;36mPlatform Information:\x1b[0m\n", .{});
            print("OS: \x1b[32m{s}\x1b[0m\n", .{@tagName(builtin.os.tag)});
            print("Architecture: \x1b[32m{s}\x1b[0m\n", .{@tagName(builtin.cpu.arch)});
            print("Endianness: \x1b[32m{s}\x1b[0m\n", .{@tagName(builtin.cpu.arch.endian())});
            print("Pointer Width: \x1b[32m{} bits\x1b[0m\n", .{@bitSizeOf(usize)});
        } else {
            print("Platform Information:\n", .{});
            print("OS: {s}\n", .{@tagName(builtin.os.tag)});
            print("Architecture: {s}\n", .{@tagName(builtin.cpu.arch)});
            print("Endianness: {s}\n", .{@tagName(builtin.cpu.arch.endian())});
            print("Pointer Width: {} bits\n", .{@bitSizeOf(usize)});
        }
    }
    
    pub fn printErrorCodes(self: *ArgParser) !void {
        const use_color = self.global_args.shouldShowColor();
        
        if (use_color) {
            print("\x1b[1;36mAvailable Error Codes:\x1b[0m\n", .{});
            print("\x1b[33mE0001\x1b[0m - Syntax error\n", .{});
            print("\x1b[33mE0002\x1b[0m - Type mismatch\n", .{});
            print("\x1b[33mE0003\x1b[0m - Undefined variable\n", .{});
            print("\x1b[33mE0004\x1b[0m - Function not found\n", .{});
            print("\x1b[33mE0005\x1b[0m - Import error\n", .{});
        } else {
            print("Available Error Codes:\n", .{});
            print("E0001 - Syntax error\n", .{});
            print("E0002 - Type mismatch\n", .{});
            print("E0003 - Undefined variable\n", .{});
            print("E0004 - Function not found\n", .{});
            print("E0005 - Import error\n", .{});
        }
    }
    
    pub fn explainError(self: *ArgParser, error_code: []const u8) !void {
        const use_color = self.global_args.shouldShowColor();
        
        if (use_color) {
            print("\x1b[1;36mError Code {s}:\x1b[0m\n", .{error_code});
        } else {
            print("Error Code {s}:\n", .{error_code});
        }
        
        if (std.mem.eql(u8, error_code, "E0001")) {
            print("Syntax Error - The parser encountered invalid CURSED syntax.\n", .{});
            print("This usually means missing semicolons, mismatched braces, or invalid keywords.\n", .{});
        } else if (std.mem.eql(u8, error_code, "E0002")) {
            print("Type Mismatch - An operation was attempted on incompatible types.\n", .{});
            print("For example, trying to add a string to an integer without conversion.\n", .{});
        } else if (std.mem.eql(u8, error_code, "E0003")) {
            print("Undefined Variable - A variable was used before being declared.\n", .{});
            print("Make sure to declare variables with 'sus' before using them.\n", .{});
        } else if (std.mem.eql(u8, error_code, "E0004")) {
            print("Function Not Found - A function call references an unknown function.\n", .{});
            print("Check that the function is defined or imported correctly.\n", .{});
        } else if (std.mem.eql(u8, error_code, "E0005")) {
            print("Import Error - Failed to import a module.\n", .{});
            print("Check that the module exists and the import path is correct.\n", .{});
        } else {
            print("Unknown error code. Use --list-error-codes to see available codes.\n", .{});
        }
    }
    
    pub fn printRuntimeStats(self: *ArgParser) !void {
        print("Runtime Statistics:\n", .{});
        print("  Memory Usage: <runtime stats not yet implemented>\n", .{});
        print("  Compilation Time: <runtime stats not yet implemented>\n", .{});
        print("  Optimization Level: {}\n", .{self.global_args.optimization});
    }
    
    pub fn printHardwareInfo(self: *ArgParser) !void {
        _ = self;
        print("Hardware Information:\n", .{});
        print("  CPU Features: <hardware detection not yet implemented>\n", .{});
        print("  SIMD Support: <hardware detection not yet implemented>\n", .{});
        print("  Memory: <hardware detection not yet implemented>\n", .{});
    }
    
    pub fn validate(self: *ArgParser) !void {
        // Validate conflicting options
        if (self.global_args.verbose and self.global_args.quiet) {
            return error.ConflictingOptions;
        }
        
        // Validate subcommand requirements
        if (self.subcommand) |subcmd| {
            switch (subcmd) {
                .compile => |compile_args| {
                    if (compile_args.input == null and !compile_args.check_deps) {
                        return error.CompileNeedsInput;
                    }
                },
                .run => |run_args| {
                    if (run_args.input.len == 0) {
                        return error.RunNeedsInput;
                    }
                },
                .debug => |debug_args| {
                    if (debug_args.input.len == 0) {
                        return error.DebugNeedsInput;
                    }
                },
                else => {},
            }
        }
    }
};

/// Enhanced error handling with structured error reporting
pub const ErrorReporter = struct {
    allocator: Allocator,
    max_errors: usize,
    json_output: bool,
    use_color: bool,
    error_count: usize,
    
    pub fn init(allocator: Allocator, max_errors: usize, json_output: bool, use_color: bool) ErrorReporter {
        return ErrorReporter{
            .allocator = allocator,
            .max_errors = max_errors,
            .json_output = json_output,
            .use_color = use_color,
            .error_count = 0,
        };
    }
    
    pub fn reportError(self: *ErrorReporter, error_code: []const u8, message: []const u8, file: []const u8, line: u32, column: u32) void {
        if (self.error_count >= self.max_errors) {
            return;
        }
        
        self.error_count += 1;
        
        if (self.json_output) {
            print("{{\"type\":\"error\",\"code\":\"{s}\",\"message\":\"{s}\",\"file\":\"{s}\",\"line\":{},\"column\":{}}}\n", 
                  .{ error_code, message, file, line, column });
        } else if (self.use_color) {
            print("\x1b[1;31merror\x1b[0m[\x1b[33m{s}\x1b[0m]: {s}\n", .{ error_code, message });
            print("  \x1b[36m-->\x1b[0m {s}:{}:{}\n", .{ file, line, column });
        } else {
            print("error[{s}]: {s}\n", .{ error_code, message });
            print("  --> {s}:{}:{}\n", .{ file, line, column });
        }
    }
    
    pub fn reportWarning(self: *ErrorReporter, message: []const u8, file: []const u8, line: u32, column: u32) void {
        if (self.json_output) {
            print("{{\"type\":\"warning\",\"message\":\"{s}\",\"file\":\"{s}\",\"line\":{},\"column\":{}}}\n", 
                  .{ message, file, line, column });
        } else if (self.use_color) {
            print("\x1b[1;33mwarning\x1b[0m: {s}\n", .{message});
            print("  \x1b[36m-->\x1b[0m {s}:{}:{}\n", .{ file, line, column });
        } else {
            print("warning: {s}\n", .{message});
            print("  --> {s}:{}:{}\n", .{ file, line, column });
        }
    }
    
    pub fn hasErrors(self: *ErrorReporter) bool {
        return self.error_count > 0;
    }
    
    pub fn getErrorCount(self: *ErrorReporter) usize {
        return self.error_count;
    }
};

/// Configuration validation and loading
pub fn loadConfig(allocator: Allocator, config_path: ?[]const u8) !ConfigFile {
    if (config_path) |path| {
        return ConfigFile.loadFromFile(allocator, path);
    } else {
        // Try default locations
        const default_paths = [_][]const u8{
            ".cursed.toml",
            "cursed.toml",
            "~/.config/cursed/config.toml",
        };
        
        for (default_paths) |path| {
            if (std.fs.cwd().access(path, .{})) {
                return ConfigFile.loadFromFile(allocator, path);
            } else |_| {
                continue;
            }
        }
    }
    
    // Return default config if no file found
    return ConfigFile.init();
}

/// Version information and build metadata
pub fn printVersionInfo() void {
    print("CURSED Programming Language v1.0.0-zig\n", .{});
    print("Enhanced CLI Framework with comprehensive argument parsing\n", .{});
    print("Build: unified-cli-enhancement\n", .{});
    print("Features: LLVM, concurrency, package manager, LSP, debugging\n", .{});
}

/// Test the CLI framework
pub fn testCLI() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var parser = ArgParser.init(allocator, "cursed");
    
    // Test argument parsing
    const test_args = [_][]const u8{ "compile", "--optimize", "--output=test", "main.csd" };
    try parser.parse(&test_args);
    
    // Test validation
    try parser.validate();
    
    print("CLI framework test passed!\n", .{});
}
