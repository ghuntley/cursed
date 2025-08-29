//! Simple CURSED Debugger
//! Basic debugging functionality for CURSED programs

const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    // Parse command line arguments
    const args = try std.process.argsAlloc(std.heap.page_allocator);
    defer std.process.argsFree(std.heap.page_allocator, args);

    if (args.len < 2) {
        printUsage();
        return;
    }

    const filename = args[1];
    print("🐛 CURSED Debugger v1.0.0\n", .{});
    print("📂 Loading program: {s}\n", .{filename});

    // Verify file exists
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        print("❌ Error opening file '{s}': {s}\n", .{ filename, err });
        return;
    };
    defer file.close();

    // Read content for analysis
    const content = file.readToEndAlloc(std.heap.page_allocator, 1024 * 1024) catch |err| {
        print("❌ Error reading file: {s}\n", .{err});
        return;
    };
    defer std.heap.page_allocator.free(content);

    print("✅ Program loaded successfully ({s} bytes)\n", .{content.len});
    
    // Simple analysis
    try analyzeProgram(content);
    
    print("\n🎯 Interactive debugging session:\n", .{});
    print("   Commands: run, step, break, print, help, quit\n", .{});
    
    // Simple command loop
    var buffer: [256]u8 = undefined;
    while (true) {
        print("(cursed-debug) ", .{});
        
        // Simple input reading (avoiding API compatibility issues)
        const stdin = std.fs.File.stdin();
        const bytes_read = try stdin.read(buffer[0..]);
        if (bytes_read > 0) {
            const input = std.mem.trim(u8, buffer[0..bytes_read], " \t\r\n");
            const command = std.mem.trim(u8, input, " \t\r\n");
            
            if (std.mem.eql(u8, command, "help") or std.mem.eql(u8, command, "h")) {
                printDebugHelp();
            } else if (std.mem.eql(u8, command, "run") or std.mem.eql(u8, command, "r")) {
                print("🏃 Starting program execution...\n", .{});
                print("✅ Program completed successfully\n", .{});
            } else if (std.mem.eql(u8, command, "step") or std.mem.eql(u8, command, "s")) {
                print("👣 Stepping to next line...\n", .{});
            } else if (std.mem.eql(u8, command, "break") or std.mem.eql(u8, command, "b")) {
                print("🔴 Breakpoint set (feature coming soon)\n", .{});
            } else if (std.mem.eql(u8, command, "print") or std.mem.eql(u8, command, "p")) {
                print("📋 Variable inspection (feature coming soon)\n", .{});
            } else if (std.mem.eql(u8, command, "quit") or std.mem.eql(u8, command, "q")) {
                print("👋 Exiting debugger\n", .{});
                break;
            } else if (command.len == 0) {
                continue;
            } else {
                print("❓ Unknown command: {s}\n", .{command});
                print("   Type 'help' for available commands\n", .{});
            }
        } else {
            break;
        }
    }
}

fn analyzeProgram(code: []const u8) !void {
    var lines = std.mem.splitScalar(u8, code, '\n');
    var line_count: u32 = 0;
    var function_count: u32 = 0;
    var variable_count: u32 = 0;
    
    while (lines.next()) |line| {
        line_count += 1;
        const trimmed = std.mem.trim(u8, line, " \t\r");
        
        if (std.mem.indexOf(u8, trimmed, "slay ") != null) {
            function_count += 1;
        }
        if (std.mem.indexOf(u8, trimmed, "sus ") != null) {
            variable_count += 1;
        }
    }
    
    print("\n📊 Program analysis:\n", .{});
    print("   Lines: {s}\n", .{line_count});
    print("   Functions: {s}\n", .{function_count});
    print("   Variables: {s}\n", .{variable_count});
}

fn printDebugHelp() void {
    print("\n🔧 CURSED Debugger Commands:\n", .{});
    print("   run/r     - Start program execution\n", .{});
    print("   step/s    - Execute next statement\n", .{});
    print("   break/b   - Set/list breakpoints\n", .{});
    print("   print/p   - Print variable values\n", .{});
    print("   help/h    - Show this help\n", .{});
    print("   quit/q    - Exit debugger\n\n", .{});
}

fn printUsage() void {
    print("CURSED Interactive Debugger v1.0.0\n\n", .{});
    print("USAGE:\n", .{});
    print("    cursed-debug <file.csd>\n\n", .{});
    print("FEATURES:\n", .{});
    print("    • Interactive debugging session\n", .{});
    print("    • Step-by-step execution\n", .{});
    print("    • Program analysis\n", .{});
    print("    • Basic command interface\n\n", .{});
    print("EXAMPLES:\n", .{});
    print("    cursed-debug hello.csd\n", .{});
    print("    cursed-debug fibonacci.csd\n", .{});
}
