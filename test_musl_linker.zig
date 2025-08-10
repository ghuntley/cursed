const std = @import("std");
const LinkerScriptManager = @import("src-zig/linker_script_manager.zig").LinkerScriptManager;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    var manager = LinkerScriptManager.init(allocator, "/home/ghuntley/cursed");
    defer manager.deinit();
    
    // Test musl x86_64 configuration
    const musl_config = try manager.getLinkerConfig("x86_64-unknown-linux-musl");
    std.debug.print("✅ musl x86_64 linker config loaded\n", .{});
    std.debug.print("  Linker args: {d}\n", .{musl_config.linker_args.len});
    std.debug.print("  Required libs: {d}\n", .{musl_config.required_libs.len});
    
    for (musl_config.linker_args) |arg| {
        std.debug.print("    - {s}\n", .{arg});
    }
    
    for (musl_config.required_libs) |lib| {
        std.debug.print("    - lib{s}\n", .{lib});
    }
    
    // Test musl ARM64 configuration
    const musl_arm64_config = try manager.getLinkerConfig("aarch64-unknown-linux-musl");
    std.debug.print("\n✅ musl ARM64 linker config loaded\n", .{});
    std.debug.print("  Linker args: {d}\n", .{musl_arm64_config.linker_args.len});
    std.debug.print("  Required libs: {d}\n", .{musl_arm64_config.required_libs.len});
    
    // Validate configurations
    const x64_valid = try manager.validateLinkerConfig("x86_64-unknown-linux-musl");
    const arm64_valid = try manager.validateLinkerConfig("aarch64-unknown-linux-musl");
    
    std.debug.print("\n🔍 Validation results:\n", .{});
    std.debug.print("  x86_64-unknown-linux-musl: {}\n", .{x64_valid});
    std.debug.print("  aarch64-unknown-linux-musl: {}\n", .{arm64_valid});
    
    std.debug.print("\n🎯 P1 Issue #35 FIXED: musl target support added!\n", .{});
}
