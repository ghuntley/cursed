const std = @import("std");
const print = std.debug.print;

const type_system = @import("src-zig/type_system.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        print("Usage: {s} <file.csd>\n", .{args[0]});
        print("Example: {s} type_test.csd\n", .{args[0]});
        return;
    }

    const filename = args[1];

    // Read source file
    const source = std.fs.cwd().readFileAlloc(allocator, filename, 1024 * 1024) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);

    print("📁 Read {s} ({} bytes)\n", .{ filename, source.len });
    print("📄 Content:\n{s}\n", .{source});

    // Initialize type checker
    var checker = type_system.TypeChecker.init(allocator) catch |err| {
        print("❌ Type checker initialization error: {}\n", .{err});
        return;
    };
    defer checker.deinit();

    print("🔧 Type checker initialized\n", .{});

    // Simple validation: check for basic patterns that our type system supports
    var line_iter = std.mem.split(u8, source, "\n");
    var line_number: u32 = 1;
    var has_errors = false;

    print("🔍 Performing basic type validation:\n", .{});

    while (line_iter.next()) |line| {
        defer line_number += 1;
        const trimmed = std.mem.trim(u8, line, " \t\r");
        if (trimmed.len == 0) continue;

        print("   Line {}: {s}\n", .{ line_number, trimmed });

        // Check variable declarations (sus pattern)
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            if (validateVariableDeclaration(&checker, trimmed)) {
                print("     ✅ Valid variable declaration\n", .{});
            } else {
                print("     ❌ Invalid variable declaration\n", .{});
                has_errors = true;
            }
        }
        // Check vibez.spill calls
        else if (std.mem.indexOf(u8, trimmed, "vibez.spill(") != null) {
            if (validateVibesSpillCall(&checker, trimmed)) {
                print("     ✅ Valid vibez.spill call\n", .{});
            } else {
                print("     ❌ Invalid vibez.spill call\n", .{});
                has_errors = true;
            }
        }
        // Check other expressions
        else {
            print("     ℹ️  Other expression (not validated in basic mode)\n", .{});
        }
    }

    if (!has_errors) {
        print("✅ Type checking passed for {s}\n", .{filename});
        print("🎉 All basic type patterns are valid!\n", .{});
    } else {
        print("❌ Type checking found errors in {s}\n", .{filename});
    }
}

fn validateVariableDeclaration(checker: *type_system.TypeChecker, line: []const u8) bool {
    _ = checker;
    
    // Basic pattern: sus <name> <type> = <value>
    if (std.mem.indexOf(u8, line, " drip ") != null and 
        std.mem.indexOf(u8, line, " = ") != null) {
        // Check if it ends with a number (basic integer validation)
        var parts = std.mem.split(u8, line, " = ");
        _ = parts.next(); // skip the declaration part
        if (parts.next()) |value_part| {
            const trimmed_value = std.mem.trim(u8, value_part, " \t");
            if (std.fmt.parseInt(i64, trimmed_value, 10)) |_| {
                return true; // Valid integer assignment to drip type
            } else |_| {
                return false;
            }
        }
    }
    
    // Accept other patterns for now
    return true;
}

fn validateVibesSpillCall(checker: *type_system.TypeChecker, line: []const u8) bool {
    // Check if vibez object and spill method exist in type environment
    if (checker.environment.getType("vibez")) |vibez_type| {
        if (vibez_type.getMethod("spill")) |_| {
            // Basic syntax check: must have parentheses
            if (std.mem.indexOf(u8, line, "(") != null and 
                std.mem.indexOf(u8, line, ")") != null) {
                return true;
            }
        }
    }
    return false;
}
