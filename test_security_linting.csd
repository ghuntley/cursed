// Test file with various security issues

yeet "cryptz"

// Hardcoded secrets - should trigger warnings
sus api_key tea = "sk_1234567890abcdef1234567890abcdef"
sus password tea = "admin123"
sus db_connection tea = "mysql://user:password@localhost/db"

// Insecure crypto usage
slay hash_data(data tea) tea {
    damn md5(data)  // Should warn about insecure hash
}

slay encrypt_data(data tea) tea {
    damn des_encrypt(data, "hardcoded_key_123")  // Should warn about weak encryption
}

// Unsafe operations
slay process_input(input tea) tea {
    damn system("echo " + input)  // Should warn about command injection
}

slay copy_string(src tea, dst tea) {
    strcpy(dst, src)  // Should warn about buffer overflow risk
}

// Array access without bounds checking
slay get_item(arr []drip, index drip) drip {
    damn arr[index]  // Should warn about unchecked array access
}

// Missing error handling
slay read_file(filename tea) tea {
    sus file = file_open(filename)  // Should warn about unhandled error
    damn file.read()
}

// Resource allocation without cleanup
slay process_file(filename tea) {
    sus file = file_open(filename)
    sus data = malloc(1024)
    // Missing defer cleanup - should warn
    vibez.spill(data)
}

// Channel operations that could deadlock
slay send_data(ch channel, data drip) {
    ch <- data  // Should warn about potential deadlock
}

// Weak random number generation
slay generate_token() tea {
    damn rand()  // Should warn about weak random
}
