const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const working_jit = @import("working_jit_engine.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    
    if (args.len < 2) {
        print("Usage: {s} <file.csd> [--jit]\n", .{args[0]});
        return;
    }
    
    const filename = args[1];
    const use_jit = args.len > 2 and std.mem.eql(u8, args[2], "--jit");
    
    // Read source file
    const source = readSourceFile(allocator, filename) catch |err| {
        print("❌ Error reading file {s}: {any}\n", .{ filename, err });
        return;
    };
    defer allocator.free(source);
    
    print("📁 Read {s} ({} bytes)\n", .{ filename, source.len });
    
    if (use_jit) {
        print("🚀 Using Working JIT Execution Engine\n", .{});
        
        var jit_engine = working_jit.WorkingJITEngine.init(allocator);
        defer jit_engine.deinit();
        
        jit_engine.compileAndExecute(source) catch |err| {
            print("❌ JIT execution failed: {any}\n", .{err});
            return;
        };
        
        // Show JIT statistics
        jit_engine.getStats();
    } else {
        print("💡 Use --jit flag to enable JIT compilation and execution\n", .{});
        
        // Also demonstrate the working JIT engine
        print("🧪 Demo: Running Working JIT Engine test...\n", .{});
        working_jit.WorkingJITEngine.test_working_jit(allocator) catch |err| {
            print("❌ JIT test failed: {any}\n", .{err});
        };
    }
}

fn readSourceFile(allocator: Allocator, filename: []const u8) ![]u8 {
    const file = std.fs.cwd().openFile(filename, .{}) catch |err| {
        return err;
    };
    defer file.close();
    
    const file_size = try file.getEndPos();
    const contents = try allocator.alloc(u8, file_size);
    _ = try file.readAll(contents);
    
    return contents;
}
