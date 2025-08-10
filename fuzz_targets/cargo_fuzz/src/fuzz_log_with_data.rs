// Fuzz target for log_with_data in src/runtime/debug_output.rs:390
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
        // TODO: Call log_with_data with fuzzed input
        // Example: log_with_data(input_str);
    }
});
