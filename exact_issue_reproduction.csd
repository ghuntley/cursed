# Exact reproduction of the parsing issue
# The issue is: "i + 1 { total = total + numbers[i] }" being treated as function names

yeet "vibez"

slay test_exact_issue() {
    sus i drip = 0
    sus total drip = 0
    sus numbers []drip = [1, 2, 3, 4, 5]
    
    # This specific pattern causes problems:
    # The parser sees "i + 1" and then encounters the brace
    # It should NOT treat "i + 1" as a function name
    bestie (i < len(numbers)) {
        total = total + numbers[i]
        # Problem line - this should be parsed as assignment followed by block
        i = i + 1 { total = total + numbers[i] }
    }
    
    vibez.spill("Result:", total)
}

test_exact_issue()
