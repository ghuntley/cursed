// Fuzz target for add_external_root in src/memory/root_set.rs:356
// Risk Level: HIGH
// Input Types: memory_buffer

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }
    
    // Convert bytes to string for parsing functions
    if let Ok(input_str) = std::str::from_utf8(data) {
        // TODO: Call add_external_root with fuzzed input
        // Example: add_external_root(input_str);
    }
});
