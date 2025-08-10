// Fuzz target for apply_all in src/formatter/rules.rs:474
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
        // TODO: Call apply_all with fuzzed input
        // Example: apply_all(input_str);
    }
});
