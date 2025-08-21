yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "networkz"
yeet "cryptz"
yeet "filez"

slay main() {
    vibez.spill("=== CURSED Full Ecosystem Test ===")
    
    // Test core math
    sus pi_val drip = mathz.PI
    sus sqrt_result drip = mathz.sqrt(25)
    vibez.spill("PI:", pi_val, "sqrt(25):", sqrt_result)
    
    // Test string operations
    sus message tea = "Hello CURSED World"
    sus msg_length drip = stringz.len_string(message)
    vibez.spill("Message:", message, "Length:", msg_length)
    
    // Test file operations
    sus test_content tea = "This is a test file content"
    filez.write_file("ecosystem_test.txt", test_content)
    sus read_content tea = filez.read_file("ecosystem_test.txt")
    vibez.spill("File round-trip successful:", read_content)
    
    // Test cryptographic operations
    sus hash_input tea = "Hello Cryptography"
    sus hash_result tea = cryptz.sha256(hash_input)
    vibez.spill("SHA256 of", hash_input, ":", hash_result)
    
    // Test network capabilities
    sus api_url tea = "https://api.github.com/repos/ghuntley/cursed"
    vibez.spill("Would fetch:", api_url)
    
    vibez.spill("=== Full ecosystem test completed successfully! ===")
}
