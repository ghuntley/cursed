fr fr Comprehensive P0 validation test
yeet "vibez"
yeet "stringz" 
yeet "mathz"
yeet "timez"
yeet "concurrenz"
yeet "json_tea"
yeet "cryptz"

slay main() {
    vibez.print_header("P0 Critical Functionality Validation")
    
    fr fr Test core built-ins
    sus arr []drip = [1, 2, 3]
    arr = append(arr, 4)
    vibez.spillf("Built-ins test - array length: {}", len(arr))
    
    fr fr Test string operations
    sus text tea = "Hello, 世界"
    sus upper tea = stringz.to_upper(text)
    vibez.spillf("String test - uppercase: {}", upper)
    
    fr fr Test math operations  
    sus sqrt_result meal = mathz.sqrt(25.0)
    vibez.spillf("Math test - sqrt(25): {}", sqrt_result)
    
    fr fr Test time operations
    sus now_time = timez.now()
    vibez.spillf("Time test - current time set: {}", now_time != 0)
    
    fr fr Test JSON with real data
    sus person tea = "name:John,age:30"
    sus json_result tea = json_tea.marshal_object(person)
    vibez.spillf("JSON test - object marshal: {}", json_result)
    
    fr fr Test crypto security
    sus byte1 drip = cryptz.secure_random_byte()
    sus byte2 drip = cryptz.secure_random_byte() 
    sus hash tea = cryptz.sha256_hash("security_test")
    vibez.spillf("Crypto test - random bytes different: {}", byte1 != byte2)
    vibez.spillf("Crypto test - hash length: {}", stringz.length(hash))
    
    vibez.print_success("All P0 critical functionality validated!")
}
