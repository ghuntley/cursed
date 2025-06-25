/// Simple ByteFit Demo - Demonstrates core ByteFit functionality
/// 
/// This example shows basic usage of the ByteFit byte manipulation library

use cursed::stdlib::bytefit::*;

fn main() {
    println!("=== ByteFit Simple Demo ===\n");
    
    // Basic Operations
    println!("--- Basic Operations ---");
    basic_operations_demo();
    
    // Search Functions
    println!("--- Search Functions ---");
    search_functions_demo();
    
    // Transformation Functions
    println!("--- Transformation Functions ---");
    transformation_demo();
    
    // Buffer Operations
    println!("--- Buffer Operations ---");
    buffer_demo();
    
    // Binary Operations
    println!("--- Binary Operations ---");
    binary_demo();
    
    // Pattern Matching
    println!("--- Pattern Matching ---");
    pattern_demo();
    
    println!("\n=== ByteFit Demo Complete! ===");
}

fn basic_operations_demo() {
    let data1 = b"hello";
    let data2 = b"world";
    let data3 = b"hello";
    
    println!("compare('hello', 'world'): {}", compare(data1, data2));
    println!("equal('hello', 'hello'): {}", equal(data1, data3));
    println!("equal_fold('Hello', 'HELLO'): {}", equal_fold(b"Hello", b"HELLO"));
    
    let repeated = repeat(b"abc", 3);
    println!("repeat('abc', 3): {}", String::from_utf8_lossy(&repeated));
    
    if let Ok(runes_result) = runes(b"Hello") {
        println!("runes('Hello'): {} characters", runes_result.len());
    }
    
    println!();
}

fn search_functions_demo() {
    let text = b"The quick brown fox jumps over the lazy dog";
    
    println!("contains(text, 'fox'): {}", contains(text, b"fox"));
    println!("contains_any(text, 'xyz'): {}", contains_any(text, "xyz"));
    println!("contains_rune(text, 'q'): {}", contains_rune(text, 'q'));
    
    println!("count(text, 'the'): {}", count(text, b"the"));
    println!("index(text, 'fox'): {}", index(text, b"fox"));
    println!("last_index_byte(text, 'o'): {}", last_index_byte(text, b'o'));
    
    println!("has_prefix(text, 'The'): {}", has_prefix(text, b"The"));
    println!("has_suffix(text, 'dog'): {}", has_suffix(text, b"dog"));
    
    println!();
}

fn transformation_demo() {
    let parts = vec![b"hello", b"world", b"test"];
    let joined = join(&parts, b", ");
    println!("join(['hello', 'world', 'test'], ', '): {}", String::from_utf8_lossy(&joined));
    
    let text = b"hello world hello";
    let replaced = replace(text, b"hello", b"hi", 1);
    println!("replace('hello world hello', 'hello', 'hi', 1): {}", String::from_utf8_lossy(&replaced));
    
    let replaced_all = replace_all(text, b"hello", b"hi");
    println!("replace_all('hello world hello', 'hello', 'hi'): {}", String::from_utf8_lossy(&replaced_all));
    
    if let Ok(upper) = to_upper(b"Hello World") {
        println!("to_upper('Hello World'): {}", String::from_utf8_lossy(&upper));
    }
    
    if let Ok(lower) = to_lower(b"Hello World") {
        println!("to_lower('Hello World'): {}", String::from_utf8_lossy(&lower));
    }
    
    if let Ok(title) = to_title(b"hello world") {
        println!("to_title('hello world'): {}", String::from_utf8_lossy(&title));
    }
    
    println!();
}

fn buffer_demo() {
    let buf = new_fit_buffer(None);
    
    // Build content using chained operations
    buf.append_string("Hello ")
       .append_string("World! ")
       .append_int(2024, 10)
       .append_string(" - ")
       .append_bool(true);
    
    println!("Buffer contents: {}", buf.string());
    println!("Buffer length: {}, capacity: {}", buf.len(), buf.cap());
    
    // Clone and modify
    let buf2 = buf.clone_buffer();
    buf2.replace_all(b" ", b"_");
    println!("Modified clone: {}", buf2.string());
    
    // Trimming
    let buf3 = new_fit_buffer(Some(b"  spaced content  ".to_vec()));
    if let Ok(_) = buf3.trim_space() {
        println!("Trimmed buffer: '{}'", buf3.string());
    }
    
    println!();
}

fn binary_demo() {
    let data = b"Hello, Binary!";
    
    // Hex encoding/decoding
    let hex_encoded = to_hex(data);
    println!("Hex encoded: {}", String::from_utf8_lossy(&hex_encoded));
    
    if let Ok(hex_decoded) = from_hex(&hex_encoded) {
        println!("Hex decoded: {}", String::from_utf8_lossy(&hex_decoded));
    }
    
    // Base64 encoding/decoding
    let base64_encoded = to_base64(data);
    println!("Base64 encoded: {}", String::from_utf8_lossy(&base64_encoded));
    
    if let Ok(base64_decoded) = from_base64(&base64_encoded) {
        println!("Base64 decoded: {}", String::from_utf8_lossy(&base64_decoded));
    }
    
    // Bitwise operations
    let a = vec![0xAA, 0xBB];
    let b = vec![0x55, 0x44];
    
    let and_result = and(&a, &b);
    let or_result = or(&a, &b);
    let xor_result = xor(&a, &b);
    let not_result = not(&a);
    
    println!("Bitwise AND: {:02X?}", and_result);
    println!("Bitwise OR:  {:02X?}", or_result);
    println!("Bitwise XOR: {:02X?}", xor_result);
    println!("Bitwise NOT: {:02X?}", not_result);
    
    // Shift operations
    let shift_data = vec![0x12, 0x34];
    let left_shifted = shift_left(&shift_data, 4);
    let right_shifted = shift_right(&shift_data, 4);
    
    println!("Left shift 4:  {:02X?}", left_shifted);
    println!("Right shift 4: {:02X?}", right_shifted);
    
    println!();
}

fn pattern_demo() {
    // Wildcard matching
    let patterns = vec![b"h*o", b"h?llo", b"*.txt", b"test*"];
    let texts = vec![b"hello", b"hello", b"file.txt", b"testing"];
    
    for (pattern, text) in patterns.iter().zip(texts.iter()) {
        let matches = wildcard_match(pattern, text);
        println!("wildcard_match('{}', '{}'): {}", 
                String::from_utf8_lossy(pattern), 
                String::from_utf8_lossy(text), 
                matches);
    }
    
    // Regular expression matching
    let test_text = b"The price is $123.45 and ID is ABC123";
    
    if let Ok(digit_match) = regex_match(r"\d+", test_text) {
        println!("regex_match('\\d+', text): {}", digit_match);
    }
    
    // Find all matches
    if let Ok(all_digits) = regex_find_all(r"\d+", test_text, -1) {
        println!("Found {} digit sequences:", all_digits.len());
        for (i, digits) in all_digits.iter().enumerate() {
            println!("  [{}]: {}", i, String::from_utf8_lossy(digits));
        }
    }
    
    // Replace with regex
    if let Ok(replaced) = regex_replace(r"\d+", test_text, b"XXX") {
        println!("regex_replace('\\d+', text, 'XXX'): {}", String::from_utf8_lossy(&replaced));
    }
    
    println!();
}
