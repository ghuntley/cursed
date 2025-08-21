# Debug test to identify parsing issues
yeet "vibez"

slay broken_parsing_test() {
    sus i drip = 0
    sus numbers []drip = [1, 2, 3]
    
    # This line demonstrates the issue: 
    # "i + 1" followed by brace should be treated as two separate things
    i + 1 { vibez.spill("Inside block") }  # This might be parsed incorrectly
    
    # This should work fine
    i = i + 1
    { vibez.spill("Block after assignment") }
}

broken_parsing_test()
