const std = @import("std");
const DebugEnabledCodeGen = @import("debug_enabled_codegen.zig").DebugEnabledCodeGen;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    std.debug.print("🚀 CURSED DWARF Debug Information Generation Test\n", .{});
    std.debug.print("================================================\n", .{});
    
    // Create debug-enabled code generator
    var codegen = DebugEnabledCodeGen.init(allocator, "debug_info_comprehensive_test.csd") catch |err| {
        std.debug.print("❌ Failed to initialize debug codegen: {any}\n", .{err});
        return;
    };
    defer codegen.deinit();
    
    // Generate comprehensive debug program
    std.debug.print("📝 Generating LLVM IR with comprehensive DWARF debug info...\n", .{});
    codegen.generateDebugProgram() catch |err| {
        std.debug.print("❌ Failed to generate debug program: {any}\n", .{err});
        return;
    };
    
    // Write LLVM IR with debug information
    std.debug.print("💾 Writing LLVM IR with debug symbols...\n", .{});
    codegen.writeIRWithDebug("debug_comprehensive.bc") catch |err| {
        std.debug.print("❌ Failed to write debug IR: {any}\n", .{err});
        return;
    };
    
    // Compile to executable with debug information
    std.debug.print("🔨 Compiling to executable with debug symbols...\n", .{});
    codegen.compileToExecutableWithDebug("debug_comprehensive") catch |err| {
        std.debug.print("❌ Failed to compile with debug: {any}\n", .{err});
        return;
    };
    
    std.debug.print("\n✅ DWARF Debug Information Implementation Complete!\n", .{});
    std.debug.print("================================================\n", .{});
    std.debug.print("🔍 Debug the program with:\n", .{});
    std.debug.print("   GDB: gdb ./debug_comprehensive\n", .{});
    std.debug.print("   LLDB: lldb ./debug_comprehensive\n", .{});
    std.debug.print("\n📋 Available debugging features:\n", .{});
    std.debug.print("   • Source location mapping\n", .{});
    std.debug.print("   • Variable inspection (all CURSED types)\n", .{});
    std.debug.print("   • Function parameter debugging\n", .{});
    std.debug.print("   • Stack trace support\n", .{});
    std.debug.print("   • Lexical scope debugging\n", .{});
    std.debug.print("   • DWARF 4 compatibility\n", .{});
    std.debug.print("\n🎯 GDB Commands to try:\n", .{});
    std.debug.print("   (gdb) break debug_test_function\n", .{});
    std.debug.print("   (gdb) run\n", .{});
    std.debug.print("   (gdb) info locals\n", .{});
    std.debug.print("   (gdb) print drip_value\n", .{});
    std.debug.print("   (gdb) print normie_value\n", .{});
    std.debug.print("   (gdb) print meal_value\n", .{});
    std.debug.print("   (gdb) print lit_value\n", .{});
    std.debug.print("   (gdb) backtrace\n", .{});
    std.debug.print("   (gdb) step\n", .{});
    std.debug.print("   (gdb) next\n", .{});
    std.debug.print("\n🎯 LLDB Commands to try:\n", .{});
    std.debug.print("   (lldb) breakpoint set -n debug_test_function\n", .{});
    std.debug.print("   (lldb) run\n", .{});
    std.debug.print("   (lldb) frame variable\n", .{});
    std.debug.print("   (lldb) p drip_value\n", .{});
    std.debug.print("   (lldb) thread backtrace\n", .{});
    std.debug.print("   (lldb) thread step-in\n", .{});
    std.debug.print("   (lldb) thread step-over\n", .{});
}
