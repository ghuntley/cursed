const std = @import("std");
const print = std.debug.print;
const ArrayList = std.ArrayList;
const HashMap = std.HashMap;
const Allocator = std.mem.Allocator;
const StringHashMap = std.StringHashMap;

// Oracle Metrics System - Evidence-Based CURSED Rust→Zig Migration Tracking
// Provides objective completion metrics for accurate progress reporting

const MetricsError = error{
    FileReadError,
    ParseError,
    OutputError,
    OutOfMemory,
};

const ComponentType = enum {
    lexer,
    parser,
    interpreter,
    codegen,
    llvm_backend,
    concurrency,
    memory_management,
    type_system,
    error_handling,
    stdlib,
    tools,
    tests,
    documentation,
    unknown,

    pub fn fromFilename(filename: []const u8) ComponentType {
        if (std.mem.indexOf(u8, filename, "lexer")) |_| return .lexer;
        if (std.mem.indexOf(u8, filename, "parser")) |_| return .parser;
        if (std.mem.indexOf(u8, filename, "interpreter")) |_| return .interpreter;
        if (std.mem.indexOf(u8, filename, "codegen")) |_| return .codegen;
        if (std.mem.indexOf(u8, filename, "llvm")) |_| return .llvm_backend;
        if (std.mem.indexOf(u8, filename, "concurrency") != null or std.mem.indexOf(u8, filename, "goroutine") != null) return .concurrency;
        if (std.mem.indexOf(u8, filename, "memory") != null or std.mem.indexOf(u8, filename, "gc") != null) return .memory_management;
        if (std.mem.indexOf(u8, filename, "type")) |_| return .type_system;
        if (std.mem.indexOf(u8, filename, "error")) |_| return .error_handling;
        if (std.mem.indexOf(u8, filename, "stdlib")) |_| return .stdlib;
        if (std.mem.indexOf(u8, filename, "lsp") != null or std.mem.indexOf(u8, filename, "debugger") != null or std.mem.indexOf(u8, filename, "cli") != null) return .tools;
        if (std.mem.indexOf(u8, filename, "test")) |_| return .tests;
        if (std.mem.indexOf(u8, filename, "doc")) |_| return .documentation;
        return .unknown;
    }
};

const IssueType = enum {
    todo,
    fixme,
    placeholder,
    hack,
    warning,

    pub fn fromString(s: []const u8) ?IssueType {
        if (std.mem.eql(u8, s, "TODO")) return .todo;
        if (std.mem.eql(u8, s, "FIXME")) return .fixme;
        if (std.mem.eql(u8, s, "PLACEHOLDER") or std.mem.indexOf(u8, s, "placeholder") != null) return .placeholder;
        if (std.mem.eql(u8, s, "HACK")) return .hack;
        if (std.mem.eql(u8, s, "WARNING")) return .warning;
        return null;
    }

    pub fn toString(self: IssueType) []const u8 {
        return switch (self) {
            .todo => "TODO",
            .fixme => "FIXME",
            .placeholder => "PLACEHOLDER",
            .hack => "HACK",
            .warning => "WARNING",
        };
    }
};

const Issue = struct {
    file: []const u8,
    line: u32,
    column: u32,
    issue_type: IssueType,
    component: ComponentType,
    description: []const u8,
    severity: u8, // 1-5 scale
};

const TestResult = struct {
    name: []const u8,
    passed: bool,
    duration_ms: u64,
    component: ComponentType,
};

const ComponentMetrics = struct {
    name: []const u8,
    total_files: u32,
    lines_of_code: u32,
    todo_count: u32,
    fixme_count: u32,
    placeholder_count: u32,
    test_pass_rate: f32,
    estimated_completion: f32,
    critical_issues: u32,
};

const ProjectMetrics = struct {
    total_files: u32,
    total_lines: u32,
    total_issues: u32,
    issues_by_type: std.EnumArray(IssueType, u32),
    components: std.EnumArray(ComponentType, ComponentMetrics),
    overall_completion: f32,
    build_success: bool,
    test_results: ArrayList(TestResult),
    generation_timestamp: i64,
};

const MetricsCollector = struct {
    allocator: Allocator,
    project_root: []const u8,
    metrics: ProjectMetrics,
    issues: ArrayList(Issue),

    const Self = @This();

    pub fn init(allocator: Allocator, project_root: []const u8) Self {
        var components = std.EnumArray(ComponentType, ComponentMetrics).initUndefined();
        for (&components.values, 0..) |*component_metrics, i| {
            const component: ComponentType = @enumFromInt(i);
            component_metrics.* = ComponentMetrics{
                .name = @tagName(component),
                .total_files = 0,
                .lines_of_code = 0,
                .todo_count = 0,
                .fixme_count = 0,
                .placeholder_count = 0,
                .test_pass_rate = 0.0,
                .estimated_completion = 0.0,
                .critical_issues = 0,
            };
        }
        
        return Self{
            .allocator = allocator,
            .project_root = project_root,
            .metrics = ProjectMetrics{
                .total_files = 0,
                .total_lines = 0,
                .total_issues = 0,
                .issues_by_type = std.EnumArray(IssueType, u32).initFill(0),
                .components = components,
                .overall_completion = 0.0,
                .build_success = false,
                .test_results = ArrayList(TestResult){},
                .generation_timestamp = std.time.timestamp(),
            },
            .issues = ArrayList(Issue){},
        };
    }

    pub fn deinit(self: *Self) void {
        self.issues.deinit(self.allocator);
        self.metrics.test_results.deinit(self.allocator);
    }

    pub fn collectMetrics(self: *Self) MetricsError!void {
        print("🔍 Oracle Metrics: Starting comprehensive CURSED migration analysis...\n", .{});
        
        try self.scanSourceFiles();
        try self.runBuildTest();
        try self.runUnitTests();
        try self.calculateCompletionMetrics();
        
        print("✅ Oracle Metrics: Analysis complete. Found {} issues across {} files\n", 
              .{ self.metrics.total_issues, self.metrics.total_files });
    }

    fn scanSourceFiles(self: *Self) MetricsError!void {
        const src_dir = try std.fmt.allocPrint(self.allocator, "{s}/src-zig", .{self.project_root});
        defer self.allocator.free(src_dir);

        try self.scanDirectory(src_dir);
        
        // Also scan examples and test directories
        const examples_dir = try std.fmt.allocPrint(self.allocator, "{s}/examples", .{self.project_root});
        defer self.allocator.free(examples_dir);
        self.scanDirectory(examples_dir) catch {};

        const tests_dir = try std.fmt.allocPrint(self.allocator, "{s}/test_suite", .{self.project_root});
        defer self.allocator.free(tests_dir);
        self.scanDirectory(tests_dir) catch {};
    }

    fn scanDirectory(self: *Self, dir_path: []const u8) MetricsError!void {
        var dir = std.fs.cwd().openDir(dir_path, .{ .iterate = true }) catch |err| {
            if (err == error.FileNotFound) return;
            return MetricsError.FileReadError;
        };
        defer dir.close();

        var iterator = dir.iterate();
        while (iterator.next() catch null) |entry| {
            if (entry.kind == .directory) {
                if (std.mem.eql(u8, entry.name, ".git") or 
                    std.mem.eql(u8, entry.name, "zig-cache") or
                    std.mem.eql(u8, entry.name, "zig-out")) {
                    continue;
                }
                const sub_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ dir_path, entry.name });
                defer self.allocator.free(sub_path);
                try self.scanDirectory(sub_path);
            } else if (entry.kind == .file) {
                if (std.mem.endsWith(u8, entry.name, ".zig") or 
                    std.mem.endsWith(u8, entry.name, ".💀")) {
                    const file_path = try std.fmt.allocPrint(self.allocator, "{s}/{s}", .{ dir_path, entry.name });
                    defer self.allocator.free(file_path);
                    try self.scanFile(file_path, entry.name);
                }
            }
        }
    }

    fn scanFile(self: *Self, file_path: []const u8, filename: []const u8) MetricsError!void {
        const file = std.fs.cwd().openFile(file_path, .{}) catch |err| {
            switch (err) {
                error.FileNotFound => return,
                else => return MetricsError.FileReadError,
            }
        };
        defer file.close();

        const file_size = file.getEndPos() catch return MetricsError.FileReadError;
        const contents = self.allocator.alloc(u8, file_size) catch return MetricsError.OutOfMemory;
        defer self.allocator.free(contents);

        _ = file.readAll(contents) catch return MetricsError.FileReadError;

        const component = ComponentType.fromFilename(filename);
        self.metrics.total_files += 1;

        var line_count: u32 = 0;
        var lines = std.mem.splitSequence(u8, contents, "\n");
        while (lines.next()) |line| {
            line_count += 1;
            self.metrics.total_lines += 1;
            try self.scanLineForIssues(file_path, line, line_count, component);
        }

        try self.updateComponentMetrics(component, 1, line_count);
    }

    fn scanLineForIssues(self: *Self, file_path: []const u8, line: []const u8, line_num: u32, component: ComponentType) MetricsError!void {
        const patterns = [_][]const u8{ "TODO", "FIXME", "PLACEHOLDER", "placeholder", "HACK", "XXX" };
        
        for (patterns) |pattern| {
            if (std.mem.indexOf(u8, line, pattern)) |pos| {
                const issue_type = IssueType.fromString(pattern) orelse .todo;
                
                // Extract description (everything after the pattern)
                const desc_start = @min(pos + pattern.len + 1, line.len);
                var description = line[desc_start..];
                if (description.len > 100) {
                    description = description[0..100];
                }
                
                const issue = Issue{
                    .file = try self.allocator.dupe(u8, file_path),
                    .line = line_num,
                    .column = @intCast(pos),
                    .issue_type = issue_type,
                    .component = component,
                    .description = try self.allocator.dupe(u8, description),
                    .severity = self.calculateSeverity(issue_type, description),
                };
                
                try self.issues.append(self.allocator, issue);
                self.metrics.total_issues += 1;
                
                // Update type counters
                const current = self.metrics.issues_by_type.get(issue_type);
                self.metrics.issues_by_type.set(issue_type, current + 1);
                
                try self.updateComponentIssueCount(component, issue_type);
            }
        }
    }

    fn calculateSeverity(self: *Self, issue_type: IssueType, description: []const u8) u8 {
        _ = self;
        var severity: u8 = switch (issue_type) {
            .fixme => 4,
            .hack => 4,
            .todo => 3,
            .placeholder => 2,
            .warning => 1,
        };

        // Increase severity for critical components
        if (std.mem.indexOf(u8, description, "crash") != null or
            std.mem.indexOf(u8, description, "panic") != null or
            std.mem.indexOf(u8, description, "memory") != null or
            std.mem.indexOf(u8, description, "security") != null) {
            severity = @min(5, severity + 2);
        }

        return severity;
    }

    fn updateComponentMetrics(self: *Self, component: ComponentType, file_count: u32, lines: u32) MetricsError!void {
        const existing = &self.metrics.components.values[@intFromEnum(component)];
        existing.total_files += file_count;
        existing.lines_of_code += lines;
    }

    fn updateComponentIssueCount(self: *Self, component: ComponentType, issue_type: IssueType) MetricsError!void {
        const metrics = &self.metrics.components.values[@intFromEnum(component)];
        switch (issue_type) {
            .todo => metrics.todo_count += 1,
            .fixme => metrics.fixme_count += 1,
            .placeholder => metrics.placeholder_count += 1,
            else => {},
        }
    }

    fn runBuildTest(self: *Self) MetricsError!void {
        print("🔨 Testing build process...\n", .{});
        
        const result = std.process.Child.run(.{
            .allocator = self.allocator,
            .argv = &[_][]const u8{ "zig", "build" },
            .cwd = self.project_root,
        }) catch {
            self.metrics.build_success = false;
            return;
        };
        defer self.allocator.free(result.stdout);
        defer self.allocator.free(result.stderr);

        self.metrics.build_success = (result.term == .Exited and result.term.Exited == 0);
        
        if (self.metrics.build_success) {
            print("✅ Build successful\n", .{});
        } else {
            print("❌ Build failed\n", .{});
        }
    }

    fn runUnitTests(self: *Self) MetricsError!void {
        print("🧪 Running unit tests...\n", .{});
        
        // Test core components
        const test_commands = [_]struct { name: []const u8, cmd: []const u8, component: ComponentType }{
            .{ .name = "lexer_test", .cmd = "zig test src-zig/lexer.zig", .component = .lexer },
            .{ .name = "parser_test", .cmd = "zig test src-zig/parser.zig", .component = .parser },
            .{ .name = "ast_test", .cmd = "zig test src-zig/ast.zig", .component = .parser },
            .{ .name = "interpreter_test", .cmd = "zig test src-zig/interpreter.zig", .component = .interpreter },
            .{ .name = "type_system_test", .cmd = "zig test src-zig/type_system_runtime.zig", .component = .type_system },
            .{ .name = "gc_test", .cmd = "zig test src-zig/gc.zig", .component = .memory_management },
            .{ .name = "concurrency_test", .cmd = "zig test src-zig/concurrency.zig", .component = .concurrency },
        };

        var total_tests: u32 = 0;
        var passed_tests: u32 = 0;

        for (test_commands) |test_info| {
            const start_time = std.time.milliTimestamp();
            
            var cmd_parts = std.mem.splitSequence(u8, test_info.cmd, " ");
            var argv = ArrayList([]const u8){};
            defer argv.deinit(self.allocator);
            
            while (cmd_parts.next()) |part| {
                try argv.append(self.allocator, part);
            }

            const result = std.process.Child.run(.{
                .allocator = self.allocator,
                .argv = argv.items,
                .cwd = self.project_root,
            }) catch {
                continue;
            };
            defer self.allocator.free(result.stdout);
            defer self.allocator.free(result.stderr);

            const end_time = std.time.milliTimestamp();
            const duration: u64 = @intCast(end_time - start_time);
            const passed = (result.term == .Exited and result.term.Exited == 0);
            
            if (passed) passed_tests += 1;
            total_tests += 1;

            const test_result = TestResult{
                .name = try self.allocator.dupe(u8, test_info.name),
                .passed = passed,
                .duration_ms = duration,
                .component = test_info.component,
            };
            try self.metrics.test_results.append(self.allocator, test_result);

            // Update component test pass rate
            const component_metrics = &self.metrics.components.values[@intFromEnum(test_info.component)];
            component_metrics.test_pass_rate = if (passed) 100.0 else 0.0;
        }

        print("📊 Tests: {}/{} passed ({d:.1}%)\n", .{ passed_tests, total_tests, 
            if (total_tests > 0) @as(f32, @floatFromInt(passed_tests * 100)) / @as(f32, @floatFromInt(total_tests)) else 0.0 });
    }

    fn calculateCompletionMetrics(self: *Self) MetricsError!void {
        print("📐 Calculating completion metrics...\n", .{});

        // Component completion estimates based on issue density and severity
        for (&self.metrics.components.values, 0..) |*metrics, i| {
            const component: ComponentType = @enumFromInt(i);
            _ = component; // Unused for now
            
            const issue_density = if (metrics.lines_of_code > 0) 
                @as(f32, @floatFromInt(metrics.todo_count + metrics.fixme_count + metrics.placeholder_count)) / @as(f32, @floatFromInt(metrics.lines_of_code))
            else 0.0;
            
            // Completion estimate: Higher issue density = lower completion
            // Base completion starts at 80% for existing code, reduced by issue density
            var completion = 0.8 - (issue_density * 10.0);
            
            // Factor in critical issues
            if (metrics.fixme_count > 0) {
                completion -= 0.1 * @as(f32, @floatFromInt(metrics.fixme_count)) / 10.0;
            }
            
            // Factor in test pass rate
            completion = (completion + metrics.test_pass_rate / 100.0) / 2.0;
            
            // Clamp between 0 and 1
            metrics.estimated_completion = @max(0.0, @min(1.0, completion));
        }

        // Overall completion is weighted average
        var total_weight: f32 = 0.0;
        var weighted_completion: f32 = 0.0;
        
        for (self.metrics.components.values) |metrics| {
            const weight = @as(f32, @floatFromInt(metrics.lines_of_code));
            total_weight += weight;
            weighted_completion += metrics.estimated_completion * weight;
        }
        
        self.metrics.overall_completion = if (total_weight > 0) weighted_completion / total_weight else 0.0;
        
        print("🎯 Overall estimated completion: {d:.1}%\n", .{self.metrics.overall_completion * 100.0});
    }

    pub fn outputJSON(self: *Self, output_path: []const u8) MetricsError!void {
        const file = std.fs.cwd().createFile(output_path, .{}) catch return MetricsError.OutputError;
        defer file.close();

        const writer = file.writer();
        
        try writer.writeAll("{\n");
        try writer.print("  \"timestamp\": {},\n", .{self.metrics.generation_timestamp});
        try writer.print("  \"overall_completion\": {d:.3},\n", .{self.metrics.overall_completion});
        try writer.print("  \"build_success\": {},\n", .{self.metrics.build_success});
        try writer.print("  \"total_files\": {},\n", .{self.metrics.total_files});
        try writer.print("  \"total_lines\": {},\n", .{self.metrics.total_lines});
        try writer.print("  \"total_issues\": {},\n", .{self.metrics.total_issues});
        
        // Issues by type
        try writer.writeAll("  \"issues_by_type\": {\n");
        var first = true;
        inline for (std.meta.fields(IssueType), 0..) |field, i| {
            const issue_type: IssueType = @enumFromInt(i);
            const count = self.metrics.issues_by_type.get(issue_type);
            if (count > 0) {
                if (!first) try writer.writeAll(",\n");
                try writer.print("    \"{s}\": {}", .{ field.name, count });
                first = false;
            }
        }
        try writer.writeAll("\n  },\n");
        
        // Components
        try writer.writeAll("  \"components\": {\n");
        first = true;
        for (self.metrics.components.values, 0..) |component_metrics, i| {
            const component: ComponentType = @enumFromInt(i);
            if (component_metrics.total_files > 0) {
                if (!first) try writer.writeAll(",\n");
                try writer.print("    \"{s}\": {{\n", .{@tagName(component)});
                try writer.print("      \"total_files\": {},\n", .{component_metrics.total_files});
                try writer.print("      \"lines_of_code\": {},\n", .{component_metrics.lines_of_code});
                try writer.print("      \"todo_count\": {},\n", .{component_metrics.todo_count});
                try writer.print("      \"fixme_count\": {},\n", .{component_metrics.fixme_count});
                try writer.print("      \"placeholder_count\": {},\n", .{component_metrics.placeholder_count});
                try writer.print("      \"test_pass_rate\": {d:.1},\n", .{component_metrics.test_pass_rate});
                try writer.print("      \"estimated_completion\": {d:.3}\n", .{component_metrics.estimated_completion});
                try writer.writeAll("    }");
                first = false;
            }
        }
        try writer.writeAll("\n  },\n");
        
        // Test results
        try writer.writeAll("  \"test_results\": [\n");
        for (self.metrics.test_results.items, 0..) |test_result, i| {
            if (i > 0) try writer.writeAll(",\n");
            try writer.print("    {{\n");
            try writer.print("      \"name\": \"{s}\",\n", .{test_result.name});
            try writer.print("      \"passed\": {},\n", .{test_result.passed});
            try writer.print("      \"duration_ms\": {},\n", .{test_result.duration_ms});
            try writer.print("      \"component\": \"{s}\"\n", .{@tagName(test_result.component)});
            try writer.writeAll("    }");
        }
        try writer.writeAll("\n  ]\n");
        
        try writer.writeAll("}\n");
        
        print("📄 Metrics JSON written to: {s}\n", .{output_path});
    }

    pub fn outputSummary(self: *Self) void {
        print("\n" ++ "=" ** 60 ++ "\n", .{});
        print("🔮 ORACLE METRICS - CURSED Rust→Zig Migration Status\n", .{});
        print("=" ** 60 ++ "\n\n", .{});
        
        print("📈 Overall Completion: {d:.1}% ({s})\n", .{ 
            self.metrics.overall_completion * 100.0,
            if (self.metrics.overall_completion > 0.9) "🟢 Ready" 
            else if (self.metrics.overall_completion > 0.7) "🟡 Near Ready" 
            else "🔴 In Progress" 
        });
        
        print("🔨 Build Status: {s}\n", .{if (self.metrics.build_success) "✅ Success" else "❌ Failed"});
        print("📊 Total Files: {} Zig files, {} lines of code\n", .{ self.metrics.total_files, self.metrics.total_lines });
        print("⚠️  Total Issues: {} (TODO/FIXME/PLACEHOLDER)\n", .{self.metrics.total_issues});
        
        print("\n📋 Issues Breakdown:\n", .{});
        inline for (std.meta.fields(IssueType), 0..) |field, i| {
            const issue_type: IssueType = @enumFromInt(i);
            const count = self.metrics.issues_by_type.get(issue_type);
            if (count > 0) {
                print("  • {s}: {}\n", .{ field.name, count });
            }
        }
        
        print("\n🧩 Component Analysis:\n", .{});
        for (self.metrics.components.values, 0..) |metrics, i| {
            const component: ComponentType = @enumFromInt(i);
            if (metrics.total_files > 0) {
                const completion_pct = metrics.estimated_completion * 100.0;
                const status_icon = if (completion_pct > 90) "🟢" else if (completion_pct > 70) "🟡" else "🔴";
                
                print("  {s} {s}: {d:.0}% complete ({} files, {} issues)\n", .{
                    status_icon,
                    @tagName(component),
                    completion_pct,
                    metrics.total_files,
                    metrics.todo_count + metrics.fixme_count + metrics.placeholder_count
                });
            }
        }
        
        const total_passed = blk: {
            var count: u32 = 0;
            for (self.metrics.test_results.items) |test_result| {
                if (test_result.passed) count += 1;
            }
            break :blk count;
        };
        
        print("\n🧪 Test Summary: {}/{} tests passing\n", .{ total_passed, self.metrics.test_results.items.len });
        
        print("\n" ++ "=" ** 60 ++ "\n", .{});
        print("📊 Evidence-based metrics complete. Use JSON for CI integration.\n", .{});
        print("=" ** 60 ++ "\n\n", .{});
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    const project_root = if (args.len > 1) args[1] else ".";
    const output_json = if (args.len > 2) args[2] else "cursed_metrics.json";

    var collector = MetricsCollector.init(allocator, project_root);
    defer collector.deinit();

    try collector.collectMetrics();
    try collector.outputJSON(output_json);
    collector.outputSummary();
}
