// Fuzz target for with_public_key in src/stdlib/net/protocols/ssh.rs:136
// Risk Level: HIGH
// Input Types: memory_buffer, file_io

#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }
    
    // Convert bytes to string for parsing functions
    if let Ok(input_str) = std::str::from_utf8(data) {
        // TODO: Call with_public_key with fuzzed input
        // Example: with_public_key(input_str);
    }
});
