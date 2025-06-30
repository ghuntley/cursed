// Self-Reading Program Demo
// Demonstrates how to read current source file and manipulate file data

import "stdlib::io";
import "stdlib::vibecheck";

// Function to get command line arguments
slay get_args() -> array {
    // CURSED programs receive command line args through environment
    // This is a conceptual example - actual implementation would be in runtime
    sus args = ["self_reading_example.csd"];  // Simulated args
    return args;
}

// Get current source file path from command line or caller info
slay get_current_file() -> string {
    // Method 1: Use vibecheck.caller to get source file info
    let (pc, file, line, ok) = vibecheck.caller(0);
    if ok {
        return file;
    }
    
    // Method 2: Use command line arguments as fallback
    sus args = get_args();
    if args.len() > 0 {
        return args[0];
    }
    
    // Method 3: Default fallback
    return "self_reading_example.csd";
}

// Read current source file as string
slay read_current_source() -> string {
    sus current_file = get_current_file();
    return io.read_file(current_file);
}

// Read current source file as byte array
slay read_current_source_bytes() -> array {
    sus current_file = get_current_file();
    return io.read_file_bytes(current_file);
}

// Convert bytes to binary representation
slay bytes_to_binary(bytes: array) -> string {
    sus binary_str = "";
    
    for byte in bytes {
        // Convert each byte to 8-bit binary string
        sus binary_byte = "";
        sus value = byte;
        
        // Convert to binary (8 bits)
        for i in [7, 6, 5, 4, 3, 2, 1, 0] {
            if value >= (1 << i) {
                binary_byte += "1";
                value -= (1 << i);
            } else {
                binary_byte += "0";
            }
        }
        
        binary_str += binary_byte + " ";
    }
    
    return binary_str;
}

// Convert string to byte array
slay string_to_bytes(text: string) -> array {
    sus bytes = [];
    
    // In CURSED, strings can be iterated character by character
    for char in text {
        // Get ASCII/UTF-8 byte value of character
        sus byte_val = char.as_bytes()[0];  // Get first byte
        bytes.push(byte_val);
    }
    
    return bytes;
}

// Analyze file content
slay analyze_source(content: string) -> void {
    io.println("=== Source File Analysis ===");
    io.println(&format!("File size: {} characters", content.len()));
    io.println(&format!("Number of lines: {}", content.lines().count()));
    
    // Count CURSED-specific keywords
    sus keyword_counts = map[string]int{
        "slay": content.matches("slay").count(),
        "sus": content.matches("sus").count(),
        "lowkey": content.matches("lowkey").count(),
        "facts": content.matches("facts").count(),
        "fr": content.matches("fr").count(),
        "bestie": content.matches("bestie").count(),
    };
    
    io.println("Keyword frequencies:");
    for (keyword, count) in keyword_counts {
        io.println(&format!("  {}: {}", keyword, count));
    }
}

// Basic file I/O operations demo
slay file_io_demo() -> void {
    io.println("=== File I/O Operations Demo ===");
    
    // Write to a temporary file
    sus temp_file = "temp_demo.txt";
    sus content = "Hello from CURSED! 🚀\nThis is a test file.";
    
    if io.write_file(temp_file, content) {
        io.println(&format!("✅ Successfully wrote to {}", temp_file));
        
        // Read it back
        sus read_content = io.read_file(temp_file);
        io.println(&format!("📖 Read content: {}", read_content));
        
        // Read as bytes
        sus bytes = io.read_file_bytes(temp_file);
        io.println(&format!("📊 File size in bytes: {}", bytes.len()));
        
        // Convert to binary representation
        sus binary = bytes_to_binary(bytes);
        io.println(&format!("🔢 Binary representation (first 64 chars): {}", 
                          binary.substring(0, 64)));
        
        // Clean up
        if io.delete_file(temp_file) {
            io.println(&format!("🗑️  Cleaned up {}", temp_file));
        }
    } else {
        io.println("❌ Failed to write file");
    }
}

// Runtime introspection demo using vibecheck
slay runtime_introspection_demo() -> void {
    io.println("=== Runtime Introspection Demo ===");
    
    // Get caller information
    let (pc, file, line, ok) = vibecheck.caller(0);
    if ok {
        let func_info = vibecheck.func_for_pc(pc);
        io.println(&format!("📍 Current location:"));
        io.println(&format!("  Function: {}", func_info.name()));
        io.println(&format!("  File: {}", file));
        io.println(&format!("  Line: {}", line));
        io.println(&format!("  PC: 0x{:x}", pc));
    }
    
    // Get version info
    io.println(&format!("🔧 CURSED Version: {}", vibecheck.version()));
    io.println(&format!("💻 Architecture: {}", vibecheck.goarch()));
    io.println(&format!("🖥️  Operating System: {}", vibecheck.goos()));
}

// Main program
slay main() -> void {
    io.println("🔥 CURSED Self-Reading Program Demo");
    io.println("===================================");
    
    // Runtime introspection
    runtime_introspection_demo();
    io.println("");
    
    // Get current file path
    sus current_file = get_current_file();
    io.println(&format!("📄 Current source file: {}", current_file));
    
    // Read own source code
    if io.file_exists(current_file) {
        io.println("📖 Reading own source code...");
        
        // Read as string
        sus source_content = read_current_source();
        analyze_source(source_content);
        io.println("");
        
        // Read as bytes
        sus source_bytes = read_current_source_bytes();
        io.println(&format!("📊 Source file size: {} bytes", source_bytes.len()));
        
        // Show first few bytes in different formats
        sus first_bytes = source_bytes[0..min(16, source_bytes.len())];
        io.println("🔢 First 16 bytes:");
        io.println(&format!("  Hex: {}", bytes_to_hex(first_bytes)));
        io.println(&format!("  Binary: {}", bytes_to_binary(first_bytes)));
        io.println(&format!("  Decimal: {:?}", first_bytes));
        
    } else {
        io.println(&format!("❌ Could not find source file: {}", current_file));
    }
    
    io.println("");
    
    // General file I/O demo
    file_io_demo();
    
    io.println("✨ Demo completed!");
}

// Helper function to convert bytes to hex
slay bytes_to_hex(bytes: array) -> string {
    sus hex_str = "";
    for byte in bytes {
        hex_str += &format!("{:02x} ", byte);
    }
    return hex_str;
}

// Helper function for minimum of two values
slay min(a: int, b: int) -> int {
    if a < b { return a; } else { return b; }
}
