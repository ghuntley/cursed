// Pure WASM implementation - alias to minimal compiler
const wasm_minimal = @import("wasm_minimal_compiler.zig");

// Re-export all WASM functions
pub const main = wasm_minimal.main;
pub const interpret_cursed = wasm_minimal.interpret_cursed;
pub const get_version = wasm_minimal.get_version;
