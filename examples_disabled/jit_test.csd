// Test CURSED JIT execution with basic arithmetic
sus x = 5
sus y = 10
facts result = x + y

// Test function definition and execution
slay add_numbers(a: sus, b: sus) -> sus {
    periodt a + b
}

sus sum = add_numbers(15, 25)
