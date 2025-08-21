# Test case to reproduce the exact parser bug described
# Issue: "i + 1 { total = total + numbers[i] }" being treated as function name

sus i drip = 0
sus numbers []drip = [1, 2, 3, 4, 5] 
sus total drip = 0

# This should fail with the described error
i + 1 { total = total + numbers[i] }

# This should also fail
ready n <= 1 { damn 1 } otherwise
