const std = @import("std");
const print = std.debug.print;

// This script compiles time module tests directly through the LLVM pipeline
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: test_time_compilation <time_test.💀.💀>\n", .{});
        return;
    }

    const test_file = args[1];
    print("🔥 Testing time module compilation: {s}\n", .{test_file});

    // Test compilation via system call to avoid LLVM dependency issues
    const compile_result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{
            "zig", "run", "-lc", "-lllvm", "src-zig/cursed_compiler_main.zig", 
            "--", test_file, "test_time_output", "--binary"
        },
    }) catch |err| {
        print("❌ Compilation failed with error: {any}\n", .{err});
        return;
    };
    defer allocator.free(compile_result.stdout);
    defer allocator.free(compile_result.stderr);

    print("Compilation output:\n{s}\n", .{compile_result.stdout});
    if (compile_result.stderr.len > 0) {
        print("Compilation stderr:\n{s}\n", .{compile_result.stderr});
    }

    if (compile_result.term.Exited != 0) {
        print("❌ Compilation failed with exit code: {d}\n", .{compile_result.term.Exited});
        return;
    }

    print("✅ Compilation successful!\n", .{});

    // Test execution
    print("🏃 Testing execution...\n", .{});
    const exec_result = std.process.Child.run(.{
        .allocator = allocator,
        .argv = &[_][]const u8{"./test_time_output"},
    }) catch |err| {
        print("❌ Execution failed: {any}\n", .{err});
        return;
    };
    defer allocator.free(exec_result.stdout);
    defer allocator.free(exec_result.stderr);

    print("Program output:\n{s}\n", .{exec_result.stdout});
    if (exec_result.stderr.len > 0) {
        print("Program stderr:\n{s}\n", .{exec_result.stderr});
    }

    if (exec_result.term.Exited != 0) {
        print("❌ Program failed with exit code: {d}\n", .{exec_result.term.Exited});
        return;
    }

    print("✅ Time module test completed successfully!\n", .{});
}
