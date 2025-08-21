const std = @import("std");
const print = std.debug.print;
const fs = std.fs;
const process = std.process;
const ArrayList = std.ArrayList;

/// Simple regression test runner that tests basic compilation without parser dependencies
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit(allocator);
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    print("=== CURSED REGRESSION TEST SUITE ===\n", .{});
    
    // Test basic .csd file parsing with the built cursed-zig binary
    const test_dirs = [_][]const u8{
        "tests/regression/parser",
        "tests/regression/stdlib", 
        "tests/regression/memory",
        "tests/regression/errors",
        "tests/regression/roundtrip"
    };
    
    var total_tests: u32 = 0;
    var passed_tests: u32 = 0;
    var failed_tests: u32 = 0;
    
    for (test_dirs) |test_dir| {
        print("\nTesting directory: {s}\n", .{test_dir});
        
        var dir = fs.cwd().openDir(test_dir, .{ .iterate = true }) catch |err| {
            print("⚠️  Could not open directory {s}: {}\n", .{ test_dir, err });
            continue;
        };
        defer dir.close();
        
        var iterator = dir.iterate();
        while (try iterator.next()) |entry| {
            if (entry.kind == .file and std.mem.endsWith(u8, entry.name, ".csd")) {
                total_tests += 1;
                
                const test_path = try std.fmt.allocPrint(allocator, "{s}/{s}", .{ test_dir, entry.name });
                defer allocator.free(test_path);
                
                // Test if file is syntactically valid by running cursed-zig on it
                const result = std.process.Child.run(.{
                    .allocator = allocator,
                    .argv = &[_][]const u8{ "./zig-out/bin/cursed-zig", test_path },
                    .max_output_bytes = 1024 * 1024,
                }) catch |err| {
                    print("❌ {s} - Execution failed: {}\n", .{ entry.name, err });
                    failed_tests += 1;
                    continue;
                };
                defer allocator.free(result.stdout);
                defer allocator.free(result.stderr);
                
                // Check for different categories of test results
                if (std.mem.indexOf(u8, test_dir, "errors") != null) {
                    // Error tests should fail gracefully (not crash)
                    if (result.term == .Exited and result.term.Exited <= 1) {
                        print("✅ {s} - Error handled gracefully\n", .{entry.name});
                        passed_tests += 1;
                    } else {
                        print("❌ {s} - Crashed or unexpected error\n", .{entry.name});
                        failed_tests += 1;
                    }
                } else {
                    // Other tests should succeed
                    if (result.term == .Exited and result.term.Exited == 0) {
                        print("✅ {s} - Passed\n", .{entry.name});
                        passed_tests += 1;
                    } else {
                        print("❌ {s} - Failed (exit code: {})\n", .{ entry.name, result.term });
                        if (result.stderr.len > 0) {
                            print("   Error: {s}\n", .{result.stderr});
                        }
                        failed_tests += 1;
                    }
                }
            }
        }
    }
    
    print("\n=== TEST SUMMARY ===\n", .{});
    print("Total Tests: {d}\n", .{total_tests});
    print("Passed: {d} ({d:.1}%)\n", .{ passed_tests, if (total_tests > 0) @as(f64, @floatFromInt(passed_tests)) / @as(f64, @floatFromInt(total_tests)) * 100.0 else 0.0 });
    print("Failed: {d} ({d:.1}%)\n", .{ failed_tests, if (total_tests > 0) @as(f64, @floatFromInt(failed_tests)) / @as(f64, @floatFromInt(total_tests)) * 100.0 else 0.0 });
    print("====================\n", .{});
    
    if (failed_tests > 0) {
        process.exit(1);
    }
}
