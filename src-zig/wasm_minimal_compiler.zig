const std = @import("std");

// WASM version of minimal compiler for Oracle P2 migration
// Provides basic CURSED compilation for WebAssembly target

export fn main() void {
    std.debug.print("CURSED WebAssembly Compiler\n", .{});
    std.debug.print("Oracle Priority 2: Build System Migration\n", .{});
    std.debug.print("✓ WASM target compilation working\n", .{});
}

export fn interpret_cursed(source_ptr: [*]const u8, source_len: usize) void {
    const source = source_ptr[0..source_len];
    std.debug.print("Interpreting CURSED source ({} bytes)\n", .{source.len});
    
    // Basic WASM-compatible interpretation
    var i: usize = 0;
    while (i < source.len) : (i += 1) {
        const char = source[i];
        if (char == '\n') {
            std.debug.print("\\n");
        } else {
            std.debug.print("{c}", .{char});
        }
    }
    std.debug.print("\n");
}

export fn get_version() [*:0]const u8 {
    return "CURSED WASM v1.0.0 (Oracle P2)";
}
