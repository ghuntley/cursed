// Fuzz target for string_last_index_of in src/execution/runtime_functions.rs:4065
// Risk Level: CRITICAL
// Input Types: parsing, memory_buffer

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }
    
    // Convert bytes to string for parsing functions
    if let Ok(input_str) = std::str::from_utf8(data) {
        // TODO: Call string_last_index_of with fuzzed input
        // Example: string_last_index_of(input_str);
    }
});
