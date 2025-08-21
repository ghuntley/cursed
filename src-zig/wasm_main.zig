const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;

const lexer = @import("lexer.zig");

// WASM-compatible entry point that doesn't use filesystem or command line args
export fn cursed_compile(source_ptr: [*]const u8, source_len: usize) i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = source_ptr[0..source_len];
    
    // Basic lexical analysis
    var lex = lexer.Lexer.init(allocator, source);
    const tokens = lex.tokenize() catch {
        return -1; // Error code
    };
    defer tokens.deinit();

    // For WASM, just return success for now
    return 0;
}

export fn cursed_version() [*:0]const u8 {
    return "CURSED Zig Compiler v1.0.0-wasm";
}

// WASM doesn't have a main function in the traditional sense
pub fn main() void {
    // This function won't be called in WASM environment
}

fn printUsage() void {
    // WASM version doesn't print usage
}

// Simple test function for validation
export fn cursed_test() i32 {
    const test_source = "vibez.spill(\"Hello WASM!\")";
    return cursed_compile(test_source.ptr, test_source.len);
}
