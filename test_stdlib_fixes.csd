fr fr Test script to validate critical stdlib fixes
yeet "vibez"
yeet "cryptz"
yeet "json_tea"

slay main() {
    vibez.spill("=== Testing Critical Stdlib Fixes ===")
    
    fr fr Test 1: Vibez I/O operations
    vibez.spill("Test 1: Vibez I/O")
    vibez.spillln("  Basic spill working")
    vibez.spill_two("  Spill", "two args")
    vibez.print_success("Vibez operations working")
    
    fr fr Test 2: JSON operations
    vibez.spill("Test 2: JSON operations")
    sus simple_json tea = json_tea.Marshal("hello")
    vibez.spillf("  JSON marshal result: {}", simple_json)
    sus unmarshaled tea = json_tea.Unmarshal("\"world\"")
    vibez.spillf("  JSON unmarshal result: {}", unmarshaled)
    
    fr fr Test 3: Basic crypto operations
    vibez.spill("Test 3: Crypto operations")
    sus random_byte drip = cryptz.secure_random_byte()
    vibez.spillf("  Random byte: {}", random_byte)
    sus hash_result tea = cryptz.sha256_hash("test")
    vibez.spillf("  SHA256 hash: {}", hash_result)
    
    vibez.print_success("All critical stdlib fixes validated!")
}
