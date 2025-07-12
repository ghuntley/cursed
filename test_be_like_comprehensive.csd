// Comprehensive type alias tests

// Basic type aliases
be_like MyInt = normie
be_like MyString = tea
be_like MyBool = lit

// Type alias variables
sus count MyInt = 42
sus name MyString = "hello"
sus flag MyBool = based

// Nested type aliases
be_like Counter = MyInt
sus total Counter = 100

// Array type aliases
be_like IntArray = [5]normie
sus numbers IntArray = [1, 2, 3, 4, 5]

// Function using type aliases
slay processCount(value MyInt) MyString {
    damn "processed"
}

// Test function call
sus result = processCount(count)

// Output results
vibez.spill(count)
vibez.spill(name)
vibez.spill(flag)
vibez.spill(total)
vibez.spill(result)
