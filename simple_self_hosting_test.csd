// Simple Self-Hosting Test
slay main() {
    vibez.spill("CURSED Self-Hosting Test")
    vibez.spill("=======================")
    
    // Test basic arithmetic
    sus a normie = 10
    sus b normie = 20
    sus result normie = a + b
    vibez.spill("Arithmetic test: {} + {} = {}", a, b, result)
    
    // Test string operations
    sus message tea = "Self-hosting compiler"
    sus length normie = len(message)
    vibez.spill("String test: '{}' has {} characters", message, length)
    
    // Test boolean operations
    sus flag lit = based
    sus check lit = (result > 25)
    vibez.spill("Boolean test: flag={}, check={}", flag, check)
    
    // Test arrays
    sus numbers := [1, 2, 3, 4, 5]
    sus first normie = numbers[0]
    sus array_length normie = len(numbers)
    vibez.spill("Array test: first={}, length={}", first, array_length)
    
    // Test control flow
    lowkey result > 25 {
        vibez.spill("Control flow test: condition passed")
    } highkey {
        vibez.spill("Control flow test: condition failed")
    }
    
    // Test manual loop simulation
    sus counter normie = 0
    counter = counter + 1
    vibez.spill("Loop iteration: {}", counter)
    counter = counter + 1
    vibez.spill("Loop iteration: {}", counter)
    
    vibez.spill("=======================")
    vibez.spill("Self-hosting test completed successfully!")
}
