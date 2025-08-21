# Parser Edge Case Test - Complex Expressions
# These are the specific issues mentioned

# Issue 1: Loop with assignment in body
bestie (i < len(numbers)) {
    total = total + numbers[i]
    i = i + 1
}

# Issue 2: Function with conditional return  
slay factorial(n drip) drip {
    ready n <= 1 {
        damn 1
    } otherwise {
        damn n * factorial(n - 1)
    }
}

# Issue 3: Complex expressions in conditions
slay test_complex_conditions() {
    sus a drip = 10
    sus b drip = 20
    sus numbers []drip = [1, 2, 3, 4, 5]
    
    bestie (a + b < 50 && len(numbers) > 0) {
        vibez.spill("Complex condition works")
        a = a + 1
    }
}

# Issue 4: Nested braces with complex expressions
slay nested_example() {
    sus data []drip = [1, 2, 3]
    bestie (i := 0; i < len(data); i = i + 1) {
        ready data[i] > 1 {
            vibez.spill("Found:", data[i])
        }
    }
}
