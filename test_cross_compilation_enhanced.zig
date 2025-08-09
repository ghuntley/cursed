const std = @import("std");
const enhanced_compiler = @import("src-zig/enhanced_compiler.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    const source = "sus x drip = 42\nvibez.spill(\"Cross-compilation test:\", x)";
    
    // Test different targets
    const targets = [_][]const u8{
        "linux-x64",
        "linux-arm64", 
        "macos-intel",
        "macos-arm64",
        "windows-x64",
        "wasm32",
    };
    
    for (targets) |target| {
        std.debug.print("\n🎯 Testing target: {s}\n", .{target});
        std.debug.print("==============================\n", .{});
        
        const config = enhanced_compiler.CompilerConfig{
            .backend = .LLVM_Backend,
            .target = target,
            .emit_llvm = true,
            .verbose = false,
        };
        
        const output_name = try std.fmt.allocPrint(allocator, "test_{s}.ll", .{target});
        defer allocator.free(output_name);
        
        enhanced_compiler.compileProgram(allocator, source, "test.csd", config) catch |err| {
            std.debug.print("❌ Failed to compile for {s}: {}\n", .{ target, err });
            continue;
        };
        
        // Check if the target triple is correct in the output
        const ir_file = std.fs.cwd().openFile(output_name, .{}) catch |err| {
            std.debug.print("❌ Could not open IR file {s}: {}\n", .{ output_name, err });
            continue;
        };
        defer ir_file.close();
        
        const ir_content = try ir_file.readToEndAlloc(allocator, 1024 * 1024);
        defer allocator.free(ir_content);
        
        // Look for target triple line
        var lines = std.mem.splitScalar(u8, ir_content, '\n');
        var found_triple: ?[]const u8 = null;
        while (lines.next()) |line| {
            if (std.mem.startsWith(u8, line, "target triple")) {
                found_triple = line;
                break;
            }
        }
        
        if (found_triple) |triple_line| {
            std.debug.print("✅ Generated IR with: {s}\n", .{triple_line});
        } else {
            std.debug.print("⚠️ No target triple found in IR\n", .{});
        }
        
        // Clean up
        std.fs.cwd().deleteFile(output_name) catch {};
    }
    
    std.debug.print("\n🎉 Cross-compilation test complete!\n", .{});
}
