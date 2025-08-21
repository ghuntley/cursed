# This should demonstrate the exact parsing issue
slay test_function() {
    sus i drip = 0
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus total drip = 0
    
    # This line is problematic according to the error description:
    # "i + 1 { total = total + numbers[i] }" should not be treated as a function name
    bestie (i < 5) {
        i + 1 { total = total + numbers[i] }  # This should be parsed correctly
    }
    
    # Similarly this should not be treated as function name
    ready n <= 1 { damn 1 } otherwise { damn n * 2 }
}

test_function()
