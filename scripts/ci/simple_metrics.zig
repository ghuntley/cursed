const std = @import("std");
const print = std.debug.print;

// Simple Oracle Metrics for CURSED Rust→Zig Migration
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const project_root = ".";
    const src_dir = "src-zig";

    // Count TODO/FIXME/PLACEHOLDER in Zig files
    var total_issues: u32 = 0;
    var todo_count: u32 = 0;
    var fixme_count: u32 = 0;
    var placeholder_count: u32 = 0;
    var total_files: u32 = 0;
    var total_lines: u32 = 0;

    // Test build
    const build_result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{ "zig", "build" },
        .cwd = project_root,
    }) catch |err| {
        print("Build test failed: {}\n", .{err});
        return;
    };
    defer allocator.free(build_result.stdout);
    defer allocator.free(build_result.stderr);
    
    const build_success = (build_result.term == .Exited and build_result.term.Exited == 0);

    // Scan source directory
    var dir = std.fs.cwd().openDir(src_dir, .{ .iterate = true }) catch |err| {
        print("Failed to open {s}: {}\n", .{ src_dir, err });
        return;
    };
    defer dir.close();

    var iterator = dir.iterate();
    while (iterator.next() catch null) |entry| {
        if (entry.kind == .file and std.mem.endsWith(u8, entry.name, ".zig")) {
            const file_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ src_dir, entry.name });
            defer allocator.free(file_path);
            
            const file = std.fs.cwd().openFile(file_path, .{}) catch continue;
            defer file.close();

            const file_size = file.getEndPos() catch continue;
            const contents = allocator.alloc(u8, file_size) catch continue;
            defer allocator.free(contents);

            _ = file.readAll(contents) catch continue;

            total_files += 1;
            var lines = std.mem.splitSequence(u8, contents, "\n");
            while (lines.next()) |line| {
                total_lines += 1;
                
                if (std.mem.indexOf(u8, line, "TODO") != null) {
                    todo_count += 1;
                    total_issues += 1;
                }
                if (std.mem.indexOf(u8, line, "FIXME") != null) {
                    fixme_count += 1;
                    total_issues += 1;
                }
                if (std.mem.indexOf(u8, line, "placeholder") != null or std.mem.indexOf(u8, line, "PLACEHOLDER") != null) {
                    placeholder_count += 1;
                    total_issues += 1;
                }
            }
        }
    }

    // Calculate rough completion estimate
    const issue_density = if (total_lines > 0) 
        @as(f32, @floatFromInt(total_issues)) / @as(f32, @floatFromInt(total_lines))
    else 0.0;
    
    // Base completion estimate: fewer issues = higher completion
    var completion_estimate = 0.85 - (issue_density * 100.0);
    if (build_success) {
        completion_estimate += 0.1;
    }
    completion_estimate = @max(0.0, @min(1.0, completion_estimate));

    // Output summary
    print("\n" ++ "=" ** 60 ++ "\n", .{});
    print("🔮 ORACLE METRICS - CURSED Rust→Zig Migration Status\n", .{});
    print("=" ** 60 ++ "\n\n", .{});
    
    print("📈 Overall Completion: {d:.1}% ({s})\n", .{ 
        completion_estimate * 100.0,
        if (completion_estimate > 0.9) "🟢 Ready" 
        else if (completion_estimate > 0.7) "🟡 Near Ready" 
        else "🔴 In Progress" 
    });
    
    print("🔨 Build Status: {s}\n", .{if (build_success) "✅ Success" else "❌ Failed"});
    print("📊 Total Files: {} Zig files, {} lines of code\n", .{ total_files, total_lines });
    print("⚠️  Total Issues: {} (TODO/FIXME/PLACEHOLDER)\n", .{total_issues});
    
    print("\n📋 Issues Breakdown:\n", .{});
    print("  • TODO: {}\n", .{todo_count});
    print("  • FIXME: {}\n", .{fixme_count});
    print("  • PLACEHOLDER: {}\n", .{placeholder_count});
    
    print("\n📊 Key Evidence-Based Metrics:\n", .{});
    print("  • Issue Density: {d:.4} per line\n", .{issue_density});
    print("  • Lines of Code: {} (substantial codebase)\n", .{total_lines});
    print("  • Code Files: {} components\n", .{total_files});
    
    if (total_issues > 600) {
        print("  • Status: 🔴 High technical debt - many placeholders remain\n", .{});
    } else if (total_issues > 300) {
        print("  • Status: 🟡 Moderate progress - reducing placeholders\n", .{});
    } else {
        print("  • Status: 🟢 Good progress - low technical debt\n", .{});
    }
    
    print("\n" ++ "=" ** 60 ++ "\n", .{});
    print("📊 Evidence-based metrics complete. Found {} total issues.\n", .{total_issues});
    print("=" ** 60 ++ "\n\n", .{});

    // Output JSON for CI
    const json_file = std.fs.cwd().createFile("cursed_metrics.json", .{}) catch |err| {
        print("Failed to create JSON file: {}\n", .{err});
        return;
    };
    defer json_file.close();
    
    const json_content = try std.fmt.allocPrint(allocator, 
        \\{{
        \\  "timestamp": {},
        \\  "overall_completion": {d:.3},
        \\  "build_success": {},
        \\  "total_files": {},
        \\  "total_lines": {},
        \\  "total_issues": {},
        \\  "todo_count": {},
        \\  "fixme_count": {},
        \\  "placeholder_count": {},
        \\  "issue_density": {d:.6}
        \\}}
        , .{
            std.time.timestamp(),
            completion_estimate,
            build_success,
            total_files,
            total_lines,
            total_issues,
            todo_count,
            fixme_count,
            placeholder_count,
            issue_density
        });
    defer allocator.free(json_content);
    
    _ = json_file.writeAll(json_content) catch |err| {
        print("Failed to write JSON: {}\n", .{err});
    };
    
    print("📄 Metrics JSON written to: cursed_metrics.json\n", .{});
}
