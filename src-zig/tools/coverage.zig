// CURSED Code Coverage Analysis System
// Provides comprehensive code coverage tracking and reporting

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const print = std.debug.print;

// Import CURSED compiler components
const lexer = @import("../lexer.zig");
const parser = @import("../parser.zig");
const ast = @import("../ast.zig");

// Coverage data structures
pub const CoverageData = struct {
    file_path: []const u8,
    lines_total: u32,
    lines_covered: u32,
    functions_total: u32,
    functions_covered: u32,
    branches_total: u32,
    branches_covered: u32,
    line_coverage: std.StringHashMap(bool),
    function_coverage: std.StringHashMap(bool),
    branch_coverage: std.StringHashMap(bool),
    
    const Self = @This();
    
    pub fn init(allocator: Allocator, file_path: []const u8) Self {
        return Self{
            .file_path = file_path,
            .lines_total = 0,
            .lines_covered = 0,
            .functions_total = 0,
            .functions_covered = 0,
            .branches_total = 0,
            .branches_covered = 0,
            .line_coverage = std.StringHashMap(bool).init(allocator),
            .function_coverage = std.StringHashMap(bool).init(allocator),
            .branch_coverage = std.StringHashMap(bool).init(allocator),
        };
    }
    
    pub fn deinit(self: *Self) void {
        self.line_coverage.deinit();
        self.function_coverage.deinit();
        self.branch_coverage.deinit();
    }
    
    pub fn getLineCoveragePercent(self: *const Self) f32 {
        if (self.lines_total == 0) return 0.0;
        return @as(f32, @floatFromInt(self.lines_covered)) / @as(f32, @floatFromInt(self.lines_total)) * 100.0;
    }
    
    pub fn getFunctionCoveragePercent(self: *const Self) f32 {
        if (self.functions_total == 0) return 0.0;
        return @as(f32, @floatFromInt(self.functions_covered)) / @as(f32, @floatFromInt(self.functions_total)) * 100.0;
    }
    
    pub fn getBranchCoveragePercent(self: *const Self) f32 {
        if (self.branches_total == 0) return 0.0;
        return @as(f32, @floatFromInt(self.branches_covered)) / @as(f32, @floatFromInt(self.branches_total)) * 100.0;
    }
};

pub const CoverageReport = struct {
    files: ArrayList(CoverageData),
    total_lines: u32,
    covered_lines: u32,
    total_functions: u32,
    covered_functions: u32,
    total_branches: u32,
    covered_branches: u32,
    
    const Self = @This();
    
    pub fn init() Self {
        return Self{
            .files = ArrayList(CoverageData).init(allocator),
            .total_lines = 0,
            .covered_lines = 0,
            .total_functions = 0,
            .covered_functions = 0,
            .total_branches = 0,
            .covered_branches = 0,
        };
    }
    
    pub fn deinit(self: *Self) void {
        for (self.files.items) |*file| {
            file.deinit();
        }
        self.files.deinit();
    }
    
    pub fn addFile(self: *Self, file_data: CoverageData) !void {
        try self.files.append(file_data);
        self.total_lines += file_data.lines_total;
        self.covered_lines += file_data.lines_covered;
        self.total_functions += file_data.functions_total;
        self.covered_functions += file_data.functions_covered;
        self.total_branches += file_data.branches_total;
        self.covered_branches += file_data.branches_covered;
    }
    
    pub fn getOverallCoveragePercent(self: *const Self) f32 {
        if (self.total_lines == 0) return 0.0;
        return @as(f32, @floatFromInt(self.covered_lines)) / @as(f32, @floatFromInt(self.total_lines)) * 100.0;
    }
};

// Coverage analyzer
pub const CoverageAnalyzer = struct {
    allocator: Allocator,
    coverage_data: std.StringHashMap(CoverageData),
    runtime_data: std.StringHashMap(std.StringHashMap(bool)),
    
    const Self = @This();
    
    pub fn init() Self {
        return Self{
            .allocator = allocator,
            .coverage_data = std.StringHashMap(CoverageData).init(allocator),
            .runtime_data = std.StringHashMap(std.StringHashMap(bool)).init(allocator),
        };
    }
    
    pub fn deinit(self: *Self) void {
        var iter = self.coverage_data.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.coverage_data.deinit();
        
        var runtime_iter = self.runtime_data.iterator();
        while (runtime_iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.runtime_data.deinit();
    }
    
    pub fn analyzeFile(self: *Self, file_path: []const u8) !CoverageData {
        const source = try std.fs.cwd().readFileAlloc(self.allocator, file_path, 1024 * 1024);
        defer self.allocator.free(source);
        
        var lexer_instance = lexer.Lexer.init(source);
        const tokens = try lexer_instance.tokenize(self.allocator);
        defer self.allocator.free(tokens);
        
        var parser_instance = parser.Parser.init(self.allocator, tokens);
        const ast_tree = try parser_instance.parse();
        defer ast_tree.deinit();
        
        var coverage = CoverageData.init(self.allocator, file_path);
        try self.analyzeAST(&coverage, ast_tree.root);
        
        return coverage;
    }
    
    fn analyzeAST(self: *Self, coverage: *CoverageData, node: *ast.AstNode) !void {
        switch (node.node_type) {
            .FunctionDeclaration => |func_decl| {
                coverage.functions_total += 1;
                const func_key = try std.fmt.allocPrint(self.allocator, "{}:{}", .{ func_decl.name, node.line });
                defer self.allocator.free(func_key);
                try coverage.function_coverage.put(func_key, false);
                
                // Analyze function body
                if (func_decl.body) |body| {
                    try self.analyzeAST(coverage, body);
                }
            },
            .IfStatement => |if_stmt| {
                // Count branches
                coverage.branches_total += 1;
                const branch_key = try std.fmt.allocPrint(self.allocator, "if:{}", .{node.line});
                defer self.allocator.free(branch_key);
                try coverage.branch_coverage.put(branch_key, false);
                
                if (if_stmt.else_branch) |else_branch| {
                    coverage.branches_total += 1;
                    const else_key = try std.fmt.allocPrint(self.allocator, "else:{}", .{node.line});
                    defer self.allocator.free(else_key);
                    try coverage.branch_coverage.put(else_key, false);
                    try self.analyzeAST(coverage, else_branch);
                }
                
                try self.analyzeAST(coverage, if_stmt.then_branch);
            },
            .WhileLoop => |while_loop| {
                coverage.branches_total += 2; // enter and skip loop
                const loop_key = try std.fmt.allocPrint(self.allocator, "while:{}", .{node.line});
                defer self.allocator.free(loop_key);
                try coverage.branch_coverage.put(loop_key, false);
                
                try self.analyzeAST(coverage, while_loop.body);
            },
            .ExpressionStatement, .VariableDeclaration, .Assignment => {
                // Count executable lines
                coverage.lines_total += 1;
                const line_key = try std.fmt.allocPrint(self.allocator, "{}", .{node.line});
                defer self.allocator.free(line_key);
                try coverage.line_coverage.put(line_key, false);
            },
            else => {},
        }
        
        // Recursively analyze children
        if (node.children) |children| {
            for (children) |child| {
                try self.analyzeAST(coverage, child);
            }
        }
    }
    
    pub fn instrumentFile(self: *Self, input_path: []const u8, output_path: []const u8) !void {
        const source = try std.fs.cwd().readFileAlloc(self.allocator, input_path, 1024 * 1024);
        defer self.allocator.free(source);
        
        const instrumented = try self.addCoverageInstrumentation(source, input_path);
        defer self.allocator.free(instrumented);
        
        const output_file = try std.fs.cwd().createFile(output_path, .{});
        defer output_file.close();
        
        try output_file.writeAll(instrumented);
    }
    
    fn addCoverageInstrumentation(self: *Self, source: []const u8, file_path: []const u8) ![]u8 {
        var result = ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        // Add coverage runtime import at the top
        try result.appendSlice("yeet \"coverage_runtime\"\n");
        
        var lines = std.mem.split(u8, source, "\n");
        var line_number: u32 = 1;
        
        while (lines.next()) |line| {
            // Add coverage tracking for executable lines
            if (self.isExecutableLine(line)) {
                const coverage_call = try std.fmt.allocPrint(
                    self.allocator,
                    "coverage_runtime.recordLine(\"{s}\", {});\n",
                    .{ file_path, line_number }
                );
                defer self.allocator.free(coverage_call);
                try result.appendSlice(coverage_call);
            }
            
            // Add original line
            try result.appendSlice(line);
            try result.append('\n');
            
            line_number += 1;
        }
        
        return result.toOwnedSlice();
    }
    
    fn isExecutableLine(self: *Self, line: []const u8) bool {
        _ = self;
        const trimmed = std.mem.trim(u8, line, " \t");
        
        // Skip empty lines and comments
        if (trimmed.len == 0 or std.mem.startsWith(u8, trimmed, "#")) {
            return false;
        }
        
        // Skip import statements
        if (std.mem.startsWith(u8, trimmed, "yeet")) {
            return false;
        }
        
        // Skip function declarations (we track function calls separately)
        if (std.mem.startsWith(u8, trimmed, "slay")) {
            return false;
        }
        
        return true;
    }
    
    pub fn recordExecution(self: *Self, file_path: []const u8, line: u32) !void {
        var file_data = self.runtime_data.get(file_path) orelse blk: {
            const new_map = std.StringHashMap(bool).init(self.allocator);
            try self.runtime_data.put(file_path, new_map);
            break :blk self.runtime_data.getPtr(file_path).?;
        };
        
        const line_key = try std.fmt.allocPrint(self.allocator, "{}", .{line});
        defer self.allocator.free(line_key);
        
        try file_data.put(line_key, true);
    }
    
    pub fn generateReport(self: *Self, format: ReportFormat) ![]u8 {
        switch (format) {
            .HTML => return try self.generateHTMLReport(),
            .JSON => return try self.generateJSONReport(),
            .LCOV => return try self.generateLCOVReport(),
            .Console => return try self.generateConsoleReport(),
        }
    }
    
    fn generateHTMLReport(self: *Self) ![]u8 {
        var result = ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        try result.appendSlice(
            \\<!DOCTYPE html>
            \\<html lang="en">
            \\<head>
            \\    <meta charset="UTF-8">
            \\    <meta name="viewport" content="width=device-width, initial-scale=1.0">
            \\    <title>CURSED Code Coverage Report</title>
            \\    <style>
            \\        body { font-family: Arial, sans-serif; margin: 20px; }
            \\        .header { background: #f5f5f5; padding: 20px; border-radius: 5px; }
            \\        .summary { display: flex; gap: 20px; margin: 20px 0; }
            \\        .metric { background: white; padding: 15px; border-radius: 5px; border-left: 4px solid #007acc; }
            \\        .file-list { margin-top: 20px; }
            \\        .file-item { margin: 10px 0; padding: 15px; background: #f9f9f9; border-radius: 5px; }
            \\        .coverage-bar { width: 100%; height: 20px; background: #eee; border-radius: 10px; overflow: hidden; }
            \\        .coverage-fill { height: 100%; background: linear-gradient(90deg, #ff4444, #ffaa00, #44ff44); }
            \\        .covered { background: #d4edda; }
            \\        .uncovered { background: #f8d7da; }
            \\    </style>
            \\</head>
            \\<body>
            \\    <div class="header">
            \\        <h1>CURSED Code Coverage Report</h1>
            \\        <p>Generated on: 
        );
        
        const timestamp = std.time.timestamp();
        const time_str = try std.fmt.allocPrint(self.allocator, "{}", .{timestamp});
        defer self.allocator.free(time_str);
        try result.appendSlice(time_str);
        
        try result.appendSlice("</p>\n    </div>\n");
        
        // Add coverage summary
        var coverage_iter = self.coverage_data.iterator();
        var total_lines: u32 = 0;
        var covered_lines: u32 = 0;
        
        while (coverage_iter.next()) |entry| {
            total_lines += entry.value_ptr.lines_total;
            covered_lines += entry.value_ptr.lines_covered;
        }
        
        const overall_percent = if (total_lines > 0) 
            @as(f32, @floatFromInt(covered_lines)) / @as(f32, @floatFromInt(total_lines)) * 100.0 
        else 
            0.0;
        
        const summary = try std.fmt.allocPrint(self.allocator,
            \\    <div class="summary">
            \\        <div class="metric">
            \\            <h3>Overall Coverage</h3>
            \\            <div class="coverage-bar">
            \\                <div class="coverage-fill" style="width: {d:.1f}%"></div>
            \\            </div>
            \\            <p>{d:.1f}% ({}/{} lines)</p>
            \\        </div>
            \\    </div>
            \\
        , .{ overall_percent, overall_percent, covered_lines, total_lines });
        defer self.allocator.free(summary);
        try result.appendSlice(summary);
        
        try result.appendSlice("</body>\n</html>");
        
        return result.toOwnedSlice();
    }
    
    fn generateJSONReport(self: *Self) ![]u8 {
        var result = ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        try result.appendSlice("{\n  \"coverage\": {\n");
        
        var coverage_iter = self.coverage_data.iterator();
        var first = true;
        
        while (coverage_iter.next()) |entry| {
            if (!first) try result.appendSlice(",\n");
            first = false;
            
            const file_json = try std.fmt.allocPrint(self.allocator,
                \\    "{s}": {{
                \\      "lines": {{
                \\        "total": {},
                \\        "covered": {},
                \\        "percent": {d:.2f}
                \\      }},
                \\      "functions": {{
                \\        "total": {},
                \\        "covered": {},
                \\        "percent": {d:.2f}
                \\      }},
                \\      "branches": {{
                \\        "total": {},
                \\        "covered": {},
                \\        "percent": {d:.2f}
                \\      }}
                \\    }}
            , .{
                entry.key_ptr.*,
                entry.value_ptr.lines_total,
                entry.value_ptr.lines_covered,
                entry.value_ptr.getLineCoveragePercent(),
                entry.value_ptr.functions_total,
                entry.value_ptr.functions_covered,
                entry.value_ptr.getFunctionCoveragePercent(),
                entry.value_ptr.branches_total,
                entry.value_ptr.branches_covered,
                entry.value_ptr.getBranchCoveragePercent(),
            });
            defer self.allocator.free(file_json);
            try result.appendSlice(file_json);
        }
        
        try result.appendSlice("\n  }\n}");
        
        return result.toOwnedSlice();
    }
    
    fn generateLCOVReport(self: *Self) ![]u8 {
        var result = ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        var coverage_iter = self.coverage_data.iterator();
        while (coverage_iter.next()) |entry| {
            const file_path = entry.key_ptr.*;
            const coverage = entry.value_ptr.*;
            
            const file_section = try std.fmt.allocPrint(self.allocator,
                \\TN:
                \\SF:{s}
                \\LF:{}
                \\LH:{}
                \\end_of_record
                \\
            , .{ file_path, coverage.lines_total, coverage.lines_covered });
            defer self.allocator.free(file_section);
            try result.appendSlice(file_section);
        }
        
        return result.toOwnedSlice();
    }
    
    fn generateConsoleReport(self: *Self) ![]u8 {
        var result = ArrayList(u8).init(self.allocator);
        defer result.deinit();
        
        try result.appendSlice("CURSED Code Coverage Report\n");
        try result.appendSlice("===========================\n\n");
        
        var coverage_iter = self.coverage_data.iterator();
        while (coverage_iter.next()) |entry| {
            const file_path = entry.key_ptr.*;
            const coverage = entry.value_ptr.*;
            
            const file_report = try std.fmt.allocPrint(self.allocator,
                \\File: {s}
                \\  Lines:     {d:.1f}% ({}/{})
                \\  Functions: {d:.1f}% ({}/{})
                \\  Branches:  {d:.1f}% ({}/{})
                \\
                \\
            , .{
                file_path,
                coverage.getLineCoveragePercent(),
                coverage.lines_covered,
                coverage.lines_total,
                coverage.getFunctionCoveragePercent(),
                coverage.functions_covered,
                coverage.functions_total,
                coverage.getBranchCoveragePercent(),
                coverage.branches_covered,
                coverage.branches_total,
            });
            defer self.allocator.free(file_report);
            try result.appendSlice(file_report);
        }
        
        return result.toOwnedSlice();
    }
};

pub const ReportFormat = enum {
    HTML,
    JSON,
    LCOV,
    Console,
};

// Coverage runtime support (to be linked with instrumented code)
pub const CoverageRuntime = struct {
    coverage_data: std.StringHashMap(std.StringHashMap(bool)),
    allocator: Allocator,
    
    const Self = @This();
    
    var instance: ?*Self = null;
    
    pub fn init(allocator: Allocator) !*Self {
        const self = try allocator.create(Self);
        self.* = Self{
            .coverage_data = std.StringHashMap(std.StringHashMap(bool)).init(allocator),
            .allocator = allocator,
        };
        instance = self;
        return self;
    }
    
    pub fn deinit(self: *Self) void {
        var iter = self.coverage_data.iterator();
        while (iter.next()) |entry| {
            entry.value_ptr.deinit();
        }
        self.coverage_data.deinit();
        self.allocator.destroy(self);
        instance = null;
    }
    
    pub fn recordLine(file_path: []const u8, line: u32) void {
        if (instance) |self| {
            self.recordLineExecution(file_path, line) catch |err| {
                print("Coverage recording error: {}\n", .{err});
            };
        }
    }
    
    fn recordLineExecution(self: *Self, file_path: []const u8, line: u32) !void {
        var file_data = self.coverage_data.getPtr(file_path) orelse blk: {
            const new_map = std.StringHashMap(bool).init(self.allocator);
            try self.coverage_data.put(file_path, new_map);
            break :blk self.coverage_data.getPtr(file_path).?;
        };
        
        const line_key = try std.fmt.allocPrint(self.allocator, "{}", .{line});
        defer self.allocator.free(line_key);
        
        try file_data.put(line_key, true);
    }
    
    pub fn saveReport(self: *Self, output_path: []const u8, format: ReportFormat) !void {
        var analyzer = CoverageAnalyzer.init(self.allocator);
        defer analyzer.deinit();
        
        // Transfer runtime data to analyzer
        var iter = self.coverage_data.iterator();
        while (iter.next()) |entry| {
            try analyzer.runtime_data.put(entry.key_ptr.*, entry.value_ptr.*);
        }
        
        const report = try analyzer.generateReport(format);
        defer self.allocator.free(report);
        
        const output_file = try std.fs.cwd().createFile(output_path, .{});
        defer output_file.close();
        
        try output_file.writeAll(report);
    }
};
