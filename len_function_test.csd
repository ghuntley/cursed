# Test len() function for both strings and arrays

# Test string length
sus text tea = "hello world"
sus text_len drip = len(text)
vibez.spill("String length:", text_len)

# Test another string 
sus short_text tea = "hi"
sus short_len drip = len(short_text)
vibez.spill("Short string length:", short_len)

# Test array length
sus numbers []drip = [1, 2, 3, 4, 5]
sus array_len drip = len(numbers)
vibez.spill("Array length:", array_len)

# Test empty string
sus empty tea = ""
sus empty_len drip = len(empty)
vibez.spill("Empty string length:", empty_len)

# Test with length() alias
sus alt_len drip = length("test")
vibez.spill("Alternative length function:", alt_len)
