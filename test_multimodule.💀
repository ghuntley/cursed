// Multi-module stdlib interaction test
// Tests: mathz + stringz + other modules working together
// Expected: All modules should work together in both modes

import mathz
import stringz
import timez

fn main() {
    yap("=== Multi-Module Stdlib Test ===")
    
    // Test 1: Math + String interaction
    let num_result = mathz.add_two(10, 5)
    yap("Math result:")
    yap(num_result)
    
    let str1 = "Result: "
    let str2 = "15"
    let combined_str = stringz.concat(str1, str2)
    yap("String concat result:")
    yap(combined_str)
    
    // Test 2: Multiple mathz operations
    let math1 = mathz.add_two(1, 2)
    let math2 = mathz.add_two(3, 4)
    let math_combined = math1 + math2
    yap("Combined math operations:")
    yap(math_combined)
    
    // Test 3: String operations
    let greeting = "Hello"
    let target = "CURSED"
    let full_greeting = stringz.concat(greeting, target)
    yap("Full greeting:")
    yap(full_greeting)
    
    // Test 4: Time module basic test
    let current_time = timez.current_timestamp()
    yap("Current timestamp:")
    yap(current_time)
    
    // Test 5: All modules together
    let final_math = mathz.add_two(20, 22)
    let final_str = stringz.concat("Final: ", "42")
    yap("Final combined test:")
    yap(final_math)
    yap(final_str)
    
    yap("=== Multi-Module Test Complete ===")
}
