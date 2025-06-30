// Test complex expressions and missing operators
vibe test

slay calculate(x, y, z) {
    // Test missing modulo operator
    facts remainder = x % y
    
    // Test complex expressions with precedence
    facts result = x + y * z - 10 / 2
    
    // Test comparison operators
    lowkey x > y && y < z {
        yolo result + remainder
    } highkey {
        yolo 0
    }
}

slay main() {
    facts a = 15
    facts b = 4
    facts c = 3
    
    // Test function with parameters
    facts answer = calculate(a, b, c)
    
    // Test complex boolean expressions
    lowkey answer != 0 || a >= b {
        // Should work
    }
    
    yolo answer
}
