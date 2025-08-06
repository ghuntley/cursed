// WASM-compatible main that avoids all POSIX dependencies
const std = @import("std");
const builtin = @import("builtin");

const lexer = @import("lexer.zig");
const parser = @import("parser.zig");

// WASM exports - these are the only functions available in WASM
export fn cursed_compile_wasm(source_ptr: [*]const u8, source_len: usize) i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = source_ptr[0..source_len];
    
    // Basic lexical analysis without filesystem dependencies
    var lex = lexer.Lexer.init(allocator, source);
    const tokens = lex.tokenize() catch {
        return -1; // Error code
    };
    defer tokens.deinit();

    // Basic parsing without filesystem dependencies
    var p = parser.Parser.init(allocator, tokens.items);
    const ast = p.parse() catch {
        return -2; // Parse error
    };
    defer ast.deinit();

    return 0; // Success
}

export fn cursed_version_wasm() [*:0]const u8 {
    return "CURSED v1.0.0-wasm";
}

export fn cursed_test_wasm() i32 {
    const test_source = "vibez.spill(\"Hello WASM!\")";
    return cursed_compile_wasm(test_source.ptr, test_source.len);
}

// Memory allocation export for WASM host
export fn cursed_alloc(size: usize) ?[*]u8 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const memory = allocator.alloc(u8, size) catch return null;
    return memory.ptr;
}

export fn cursed_free(ptr: [*]u8, size: usize) void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const slice = ptr[0..size];
    allocator.free(slice);
}

// WASM doesn't have a main function in traditional sense
pub fn main() void {
    // This won't be called in WASM environment
}

// Simple tokenization for WASM without complex dependencies
export fn cursed_tokenize_wasm(source_ptr: [*]const u8, source_len: usize, output_ptr: [*]u8, output_len: usize) i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = source_ptr[0..source_len];
    const output = output_ptr[0..output_len];
    
    var lex = lexer.Lexer.init(allocator, source);
    const tokens = lex.tokenize() catch return -1;
    defer tokens.deinit();

    // Simple output format: just return token count
    if (output.len >= 4) {
        const count: u32 = @intCast(tokens.items.len);
        output[0] = @intCast(count & 0xFF);
        output[1] = @intCast((count >> 8) & 0xFF);
        output[2] = @intCast((count >> 16) & 0xFF);
        output[3] = @intCast((count >> 24) & 0xFF);
        return @intCast(tokens.items.len);
    }
    
    return -1;
}
