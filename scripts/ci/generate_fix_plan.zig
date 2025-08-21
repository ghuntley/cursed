const std = @import("std");
const print = std.debug.print;

// Generate fix_plan.md from Oracle metrics
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Read metrics JSON
    const metrics_file = std.fs.cwd().openFile("cursed_metrics.json", .{}) catch |err| {
        print("❌ Failed to open cursed_metrics.json: {}\n", .{err});
        return;
    };
    defer metrics_file.close();

    const metrics_content = metrics_file.readToEndAlloc(allocator, 10240) catch |err| {
        print("❌ Failed to read metrics: {}\n", .{err});
        return;
    };
    defer allocator.free(metrics_content);

    // Parse basic metrics (simplified)
    var completion: f32 = 0.0;
    var total_issues: u32 = 0;
    var todo_count: u32 = 0;
    var placeholder_count: u32 = 0;
    var total_files: u32 = 0;
    var total_lines: u32 = 0;
    var build_success = false;

    // Simple parsing (not full JSON parser)
    var lines = std.mem.splitSequence(u8, metrics_content, "\n");
    while (lines.next()) |line| {
        if (std.mem.indexOf(u8, line, "overall_completion") != null) {
            if (std.mem.indexOf(u8, line, ":") != null) {
                const colon_pos = std.mem.indexOf(u8, line, ":").?;
                const value_start = colon_pos + 1;
                if (value_start < line.len) {
                    const value_part = std.mem.trim(u8, line[value_start..], " \t,");
                    completion = std.fmt.parseFloat(f32, value_part) catch 0.0;
                }
            }
        }
        if (std.mem.indexOf(u8, line, "total_issues") != null and std.mem.indexOf(u8, line, ":") != null) {
            const colon_pos = std.mem.indexOf(u8, line, ":").?;
            const value_part = std.mem.trim(u8, line[colon_pos + 1..], " \t,");
            total_issues = std.fmt.parseInt(u32, value_part, 10) catch 0;
        }
        if (std.mem.indexOf(u8, line, "todo_count") != null and std.mem.indexOf(u8, line, ":") != null) {
            const colon_pos = std.mem.indexOf(u8, line, ":").?;
            const value_part = std.mem.trim(u8, line[colon_pos + 1..], " \t,");
            todo_count = std.fmt.parseInt(u32, value_part, 10) catch 0;
        }
        if (std.mem.indexOf(u8, line, "placeholder_count") != null and std.mem.indexOf(u8, line, ":") != null) {
            const colon_pos = std.mem.indexOf(u8, line, ":").?;
            const value_part = std.mem.trim(u8, line[colon_pos + 1..], " \t,");
            placeholder_count = std.fmt.parseInt(u32, value_part, 10) catch 0;
        }
        if (std.mem.indexOf(u8, line, "total_files") != null and std.mem.indexOf(u8, line, ":") != null) {
            const colon_pos = std.mem.indexOf(u8, line, ":").?;
            const value_part = std.mem.trim(u8, line[colon_pos + 1..], " \t,");
            total_files = std.fmt.parseInt(u32, value_part, 10) catch 0;
        }
        if (std.mem.indexOf(u8, line, "total_lines") != null and std.mem.indexOf(u8, line, ":") != null) {
            const colon_pos = std.mem.indexOf(u8, line, ":").?;
            const value_part = std.mem.trim(u8, line[colon_pos + 1..], " \t,");
            total_lines = std.fmt.parseInt(u32, value_part, 10) catch 0;
        }
        if (std.mem.indexOf(u8, line, "build_success") != null and std.mem.indexOf(u8, line, ":") != null) {
            const colon_pos = std.mem.indexOf(u8, line, ":").?;
            const value_part = std.mem.trim(u8, line[colon_pos + 1..], " \t,");
            build_success = std.mem.eql(u8, value_part, "true");
        }
    }

    // Generate fix_plan.md content
    const fix_plan_content = try std.fmt.allocPrint(allocator,
        \\# CURSED Rust→Zig Migration Fix Plan
        \\
        \\*Generated automatically by Oracle Metrics - {d}*
        \\
        \\## 📊 Current Status ({s})
        \\
        \\**Overall Completion: {d:.1}%**
        \\
        \\- 🔨 Build Status: {s}
        \\- 📁 Files: {} Zig files ({} lines of code)
        \\- ⚠️ Issues: {} remaining (TODO/PLACEHOLDER)
        \\- 📈 Issue Density: {d:.4} per line
        \\
        \\## 🎯 Priority Fixes
        \\
        \\### High Priority (P0)
        \\
        \\{s}
        \\
        \\### Medium Priority (P1) 
        \\
        \\- [ ] **TODO Items ({})**: Review and implement remaining TODO comments
        \\- [ ] **PLACEHOLDER Functions ({})**: Replace placeholder implementations
        \\- [ ] **Build Fixes**: {s}
        \\
        \\### Low Priority (P2)
        \\
        \\- [ ] **Code Cleanup**: Remove debug prints and temporary code
        \\- [ ] **Documentation**: Update inline documentation
        \\- [ ] **Performance**: Optimize hot paths identified by profiling
        \\
        \\## 🔧 Technical Debt Summary
        \\
        \\| Category | Count | Status |
        \\|----------|-------|--------|
        \\| TODO | {} | {s} |
        \\| PLACEHOLDER | {} | {s} |
        \\| Build Issues | {} | {s} |
        \\| **TOTAL** | **{}** | **{s}** |
        \\
        \\## 📈 Progress Tracking
        \\
        \\- **Completion Estimate**: {d:.1}% based on issue density and build status
        \\- **Codebase Size**: {} files, {} lines (substantial implementation)
        \\- **Migration Quality**: {s}
        \\
        \\## 🚀 Next Steps
        \\
        \\{s}
        \\
        \\## 📋 Evidence-Based Metrics
        \\
        \\This fix plan is automatically generated from static analysis:
        \\
        \\```bash
        \\# Regenerate this plan
        \\zig run scripts/ci/simple_metrics.zig
        \\zig run scripts/ci/generate_fix_plan.zig
        \\```
        \\
        \\**Last Updated**: {d}
        \\
    , .{
        std.time.timestamp(),
        
        // Status badge
        if (completion > 90) "🟢 Ready"
        else if (completion > 70) "🟡 Near Ready"  
        else "🔴 In Progress",
        
        completion * 100.0,
        
        if (build_success) "✅ Success" else "❌ Failed",
        total_files,
        total_lines,
        total_issues,
        @as(f32, @floatFromInt(total_issues)) / @as(f32, @floatFromInt(total_lines)),
        
        // High priority section
        if (!build_success) 
            "- [ ] **BUILD SYSTEM**: Fix compilation errors preventing successful builds\n- [ ] **Critical Runtime**: Address any segfaults or memory issues"
        else
            "- ✅ Build system working correctly\n- [ ] **Performance**: Profile and optimize critical paths",
        
        todo_count,
        placeholder_count,
        
        if (!build_success) "Fix build system first" else "Review and implement gradually",
        
        // Technical debt table
        todo_count,
        if (todo_count > 100) "🔴 High" else if (todo_count > 50) "🟡 Medium" else "🟢 Low",
        
        placeholder_count, 
        if (placeholder_count > 50) "🔴 High" else if (placeholder_count > 20) "🟡 Medium" else "🟢 Low",
        
        if (build_success) @as(u32, 0) else @as(u32, 1),
        if (build_success) "🟢 None" else "🔴 Critical",
        
        total_issues,
        if (total_issues > 400) "🔴 High Debt" 
        else if (total_issues > 200) "🟡 Moderate Debt"
        else "🟢 Low Debt",
        
        completion * 100.0,
        total_files,
        total_lines,
        
        if (completion > 90) "🟢 Excellent - Production ready"
        else if (completion > 80) "🟡 Good - Near production ready"
        else if (completion > 60) "🟡 Fair - Significant progress made"
        else "🔴 Early - Major work remaining",
        
        // Next steps
        if (!build_success)
            "1. **Fix build system** - Address compilation errors\n2. **Core functionality** - Ensure basic interpreter works\n3. **Reduce placeholders** - Implement critical missing functions"
        else if (total_issues > 300)
            "1. **Address TODO items** - Focus on high-impact implementations\n2. **Replace placeholders** - Implement missing functionality\n3. **Test coverage** - Add comprehensive tests"
        else
            "1. **Code review** - Review remaining TODO items\n2. **Performance optimization** - Profile and optimize\n3. **Production readiness** - Final polish and testing",
            
        std.time.timestamp()
    });
    defer allocator.free(fix_plan_content);

    // Write fix_plan.md
    const fix_plan_file = std.fs.cwd().createFile("fix_plan.md", .{}) catch |err| {
        print("❌ Failed to create fix_plan.md: {}\n", .{err});
        return;
    };
    defer fix_plan_file.close();

    _ = fix_plan_file.writeAll(fix_plan_content) catch |err| {
        print("❌ Failed to write fix_plan.md: {}\n", .{err});
        return;
    };

    print("✅ Generated fix_plan.md with current metrics\n", .{});
    print("📊 Status: {d:.1}% complete, {} issues remaining\n", .{completion * 100.0, total_issues});
}
