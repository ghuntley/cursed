# Specific parser edge cases that are failing
# Issue: Complex expressions being parsed as function names

slay problematic_function(numbers []drip) drip {
    sus total drip = 0
    sus i drip = 0
    
    # This should parse correctly: i + 1 should NOT be treated as function name
    bestie (i < len(numbers)) {
        total = total + numbers[i]
        i = i + 1 { total = total + numbers[i] }  # This line is problematic
    }
    
    damn total
}

slay another_issue(n drip) drip {
    # This should parse correctly: ready n <= 1 should NOT be treated as function name  
    ready n <= 1 { damn 1 } otherwise { damn n * another_issue(n - 1) }
}

slay test_main() {
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus result drip = problematic_function(numbers)
    vibez.spill("Result:", result)
    
    sus fact_result drip = another_issue(5)
    vibez.spill("Factorial:", fact_result)
}

test_main()
