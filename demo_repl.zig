const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.debug.print("🔥 CURSED REPL v1.0.0 - Demo\n", .{});
    std.debug.print("Interactive CURSED language shell\n", .{});
    std.debug.print("Type :help for help, :quit to exit\n", .{});
    std.debug.print("\n", .{});
    
    var variables = std.HashMap([]const u8, i64, std.hash_map.StringContext, std.hash_map.default_max_load_percentage).init(allocator);
    defer {
        var iter = variables.iterator();
        while (iter.next()) |entry| {
            allocator.free(entry.key_ptr.*);
        }
        variables.deinit();
    }
    
    var stdin = std.io.getStdIn().reader();
    
    while (true) {
        std.debug.print("cursed> ", .{});
        
        var input_buffer: [1024]u8 = undefined;
        const input = (try stdin.readUntilDelimiterOrEof(input_buffer[0..], '\n')) orelse break;
        const trimmed = std.mem.trim(u8, input, " \t\r\n");
        
        if (trimmed.len == 0) continue;
        
        // Handle special commands
        if (std.mem.eql(u8, trimmed, ":quit") or std.mem.eql(u8, trimmed, ":exit")) {
            break;
        }
        
        if (std.mem.eql(u8, trimmed, ":help")) {
            std.debug.print("\nCURSED REPL Commands:\n", .{});
            std.debug.print("  :help      - Show this help\n", .{});
            std.debug.print("  :quit      - Exit the REPL\n", .{});
            std.debug.print("  :vars      - Show variables\n", .{});
            std.debug.print("\nCURSED Syntax:\n", .{});
            std.debug.print("  sus x drip = 42    - Declare variable\n", .{});
            std.debug.print("  x                  - Show variable value\n", .{});
            std.debug.print("\n", .{});
            continue;
        }
        
        if (std.mem.eql(u8, trimmed, ":vars")) {
            if (variables.count() == 0) {
                std.debug.print("No variables defined\n", .{});
            } else {
                std.debug.print("Variables:\n", .{});
                var iter = variables.iterator();
                while (iter.next()) |entry| {
                    std.debug.print("  {s} = {}\n", .{ entry.key_ptr.*, entry.value_ptr.* });
                }
            }
            continue;
        }
        
        // Handle variable declarations: sus var_name drip = value
        if (std.mem.startsWith(u8, trimmed, "sus ")) {
            if (std.mem.indexOf(u8, trimmed, "=")) |equals_pos| {
                const declaration = trimmed[4..equals_pos]; // Remove "sus "
                const value_str = std.mem.trim(u8, trimmed[equals_pos + 1..], " \t");
                
                var parts = std.mem.splitScalar(u8, std.mem.trim(u8, declaration, " \t"), ' ');
                if (parts.next()) |var_name| {
                    if (std.fmt.parseInt(i64, value_str, 10)) |value| {
                        const name_copy = try allocator.dupe(u8, var_name);
                        try variables.put(name_copy, value);
                        std.debug.print("{s} = {}\n", .{ var_name, value });
                    } else |_| {
                        std.debug.print("Error: Invalid integer value\n", .{});
                    }
                } else {
                    std.debug.print("Error: Invalid variable declaration\n", .{});
                }
            } else {
                std.debug.print("Error: Missing assignment in variable declaration\n", .{});
            }
            continue;
        }
        
        // Handle variable lookups
        if (variables.get(trimmed)) |value| {
            std.debug.print("{}\n", .{value});
            continue;
        }
        
        // Handle simple arithmetic: 2 + 3
        if (std.mem.indexOf(u8, trimmed, "+")) |plus_pos| {
            const left_str = std.mem.trim(u8, trimmed[0..plus_pos], " \t");
            const right_str = std.mem.trim(u8, trimmed[plus_pos + 1..], " \t");
            
            const left = std.fmt.parseInt(i64, left_str, 10) catch blk: {
                if (variables.get(left_str)) |v| break :blk v else continue;
            };
            const right = std.fmt.parseInt(i64, right_str, 10) catch blk: {
                if (variables.get(right_str)) |v| break :blk v else continue;
            };
            
            std.debug.print("{}\n", .{left + right});
            continue;
        }
        
        std.debug.print("Error: Unknown command or expression\n", .{});
    }
    
    std.debug.print("Goodbye!\n", .{});
}
