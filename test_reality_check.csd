fr fr Reality check test - verify actual vs claimed functionality
yeet "vibez"
yeet "cryptz"
yeet "json_tea"
yeet "httpz"
yeet "dbz"
yeet "user_check"

slay main() {
    vibez.print_header("CURSED Reality Check Test")
    
    fr fr Test 1: Basic I/O (should work after our fixes)
    vibez.spill("Test 1: Basic I/O functionality")
    vibez.spill("  spill() working: YES")
    vibez.spillf("  spillf() working with {}: YES", "args")
    
    fr fr Test 2: Crypto security test
    vibez.spill("Test 2: Crypto security")
    sus byte1 drip = cryptz.secure_random_byte()
    sus byte2 drip = cryptz.secure_random_byte()
    vibez.spillf("  Random bytes different: {}", byte1 != byte2)
    sus hash tea = cryptz.sha256_hash("test")
    vibez.spillf("  SHA256 hash non-mock: {}", hash != "sha256_hash_result")
    
    fr fr Test 3: JSON operations
    vibez.spill("Test 3: JSON operations")
    sus json_basic tea = json_tea.Marshal("hello")
    vibez.spillf("  Basic JSON marshal: {}", json_basic)
    sus obj_test tea = json_tea.marshal_object("name:John,age:30")
    vibez.spillf("  Object marshal working: {}", obj_test != "name:John,age:30")
    
    fr fr Test 4: Try to expose mock implementations
    vibez.spill("Test 4: Mock implementation detection")
    
    fr fr This should reveal if user_check is still mocked
    sus user_lookup tea = user_check.get_user_by_id("123")
    vibez.spillf("  User lookup result: {}", user_lookup)
    
    fr fr This should reveal if database is mocked
    sus db_result tea = dbz.query("SELECT * FROM users")
    vibez.spillf("  Database query result: {}", db_result)
    
    fr fr This should reveal if HTTP is mocked
    sus http_result tea = httpz.get("https://example.com")
    vibez.spillf("  HTTP get result: {}", http_result)
    
    vibez.print_header("Reality Check Complete")
}
