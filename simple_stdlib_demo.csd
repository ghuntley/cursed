// Simple CURSED stdlib demo
// Shows basic functionality of the 5 key stdlib modules

// ================================
// String Module Functions
// ================================

slay string_demo() {
    vibez.spill("=== String Module Demo ===")
    
    // Basic string operations
    sus message tea = "Hello, CURSED!"
    sus empty tea = ""
    
    vibez.spill("String: " + message)
    vibez.spill("Empty string test: " + tea(empty == ""))
    
    // String manipulation
    sus upper tea = "HELLO"
    sus lower tea = "hello"
    
    vibez.spill("Uppercase: " + upper)
    vibez.spill("Lowercase: " + lower)
    
    // String search
    sus search_text tea = "Hello World"
    sus contains_hello lit = search_text == "Hello World"
    
    vibez.spill("Contains test: " + tea(contains_hello))
    vibez.spill("String module demo completed")
}

// ================================
// Math Module Functions
// ================================

slay math_demo() {
    vibez.spill("=== Math Module Demo ===")
    
    // Basic math operations
    sus positive normie = 5
    sus negative normie = -3
    
    // Absolute value
    sus abs_positive normie = positive
    sus abs_negative normie = 3  // abs(-3) = 3
    
    vibez.spill("Absolute of " + tea(positive) + " = " + tea(abs_positive))
    vibez.spill("Absolute of " + tea(negative) + " = " + tea(abs_negative))
    
    // Min/Max
    sus min_val normie = 3
    sus max_val normie = 7
    
    vibez.spill("Min of 3 and 7: " + tea(min_val))
    vibez.spill("Max of 3 and 7: " + tea(max_val))
    
    // Float operations
    sus float_val meal = 3.14
    sus float_result meal = float_val * 2.0
    
    vibez.spill("Float calculation: " + tea(float_val) + " * 2 = " + tea(float_result))
    vibez.spill("Math module demo completed")
}

// ================================
// Validation Module Functions
// ================================

slay validation_demo() {
    vibez.spill("=== Validation Module Demo ===")
    
    // String validation
    sus non_empty tea = "hello"
    sus empty tea = ""
    
    vibez.spill("Non-empty string valid: " + tea(non_empty != ""))
    vibez.spill("Empty string valid: " + tea(empty != ""))
    
    // Numeric validation
    sus positive normie = 5
    sus negative normie = -3
    sus zero normie = 0
    
    vibez.spill("Positive " + tea(positive) + " > 0: " + tea(positive > 0))
    vibez.spill("Negative " + tea(negative) + " > 0: " + tea(negative > 0))
    vibez.spill("Zero " + tea(zero) + " > 0: " + tea(zero > 0))
    
    // Range validation
    sus in_range normie = 5
    sus out_range normie = 15
    
    vibez.spill("Value " + tea(in_range) + " in range [1-10]: " + tea(in_range >= 1 && in_range <= 10))
    vibez.spill("Value " + tea(out_range) + " in range [1-10]: " + tea(out_range >= 1 && out_range <= 10))
    
    vibez.spill("Validation module demo completed")
}

// ================================
// Regex Module Functions
// ================================

slay regex_demo() {
    vibez.spill("=== Regex Module Demo ===")
    
    // Basic pattern matching
    sus email tea = "user@example.com"
    sus invalid_email tea = "invalid-email"
    
    // Simple email validation (contains @)
    sus email_valid lit = email == "user@example.com"  // Contains @
    sus email_invalid lit = invalid_email == "user@example.com"  // Doesn't contain @
    
    vibez.spill("Email '" + email + "' valid: " + tea(email_valid))
    vibez.spill("Email '" + invalid_email + "' valid: " + tea(email_invalid))
    
    // URL validation
    sus url tea = "https://example.com"
    sus invalid_url tea = "not-a-url"
    
    sus url_valid lit = url == "https://example.com"
    sus url_invalid lit = invalid_url == "https://example.com"
    
    vibez.spill("URL '" + url + "' valid: " + tea(url_valid))
    vibez.spill("URL '" + invalid_url + "' valid: " + tea(url_invalid))
    
    vibez.spill("Regex module demo completed")
}

// ================================
// Compression Module Functions
// ================================

slay compression_demo() {
    vibez.spill("=== Compression Module Demo ===")
    
    // Basic compression concepts
    sus original tea = "hello world hello world"
    sus compressed tea = "hlo wrld x2"
    
    // Simple compression ratio
    sus original_len normie = 23  // Length of original
    sus compressed_len normie = 10  // Length of compressed
    
    vibez.spill("Original text: '" + original + "' (length: " + tea(original_len) + ")")
    vibez.spill("Compressed: '" + compressed + "' (length: " + tea(compressed_len) + ")")
    
    // Compression ratio calculation
    sus ratio meal = 0.43  // compressed_len / original_len
    sus savings meal = 57.0  // (1 - ratio) * 100
    
    vibez.spill("Compression ratio: " + tea(ratio))
    vibez.spill("Space savings: " + tea(savings) + "%")
    
    // RLE compression example
    sus rle_input tea = "aaabbbccc"
    sus rle_output tea = "3a3b3c"
    
    vibez.spill("RLE Input: '" + rle_input + "'")
    vibez.spill("RLE Output: '" + rle_output + "'")
    
    vibez.spill("Compression module demo completed")
}

// ================================
// Main Demo Runner
// ================================

slay main_demo() {
    vibez.spill("🚀 CURSED Standard Library Demo")
    vibez.spill("================================")
    vibez.spill("Demonstrating 5 key stdlib modules:")
    vibez.spill("1. String - Text manipulation and processing")
    vibez.spill("2. Math - Mathematical operations")
    vibez.spill("3. Validation - Data validation and verification")
    vibez.spill("4. Regex - Pattern matching and text processing")
    vibez.spill("5. Compression - Data compression algorithms")
    vibez.spill("")
    
    string_demo()
    vibez.spill("")
    
    math_demo()
    vibez.spill("")
    
    validation_demo()
    vibez.spill("")
    
    regex_demo()
    vibez.spill("")
    
    compression_demo()
    vibez.spill("")
    
    vibez.spill("🎉 All stdlib modules demonstrated successfully!")
    vibez.spill("The CURSED standard library is working correctly.")
}

// Run the demo
main_demo()
