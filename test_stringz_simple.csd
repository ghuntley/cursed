// Simple test of stringz functionality

// Test basic stringz operations
sus text tea = "Hello, World!"

// Test Contains function
sus contains_world lit = stringz.Contains(text, "World")
vibez.spill("Contains 'World': ")
vibez.spill(contains_world)

// Test HasPrefix function
sus has_hello lit = stringz.HasPrefix(text, "Hello")
vibez.spill("Starts with 'Hello': ")
vibez.spill(has_hello)

// Test HasSuffix function
sus has_exclamation lit = stringz.HasSuffix(text, "!")
vibez.spill("Ends with '!': ")
vibez.spill(has_exclamation)

// Test ToUpper function
sus upper_text tea = stringz.ToUpper(text)
vibez.spill("Uppercase: ")
vibez.spill(upper_text)

// Test ToLower function
sus lower_text tea = stringz.ToLower(text)
vibez.spill("Lowercase: ")
vibez.spill(lower_text)

// Test Split function
sus parts []tea = stringz.Split("apple,banana,cherry", ",")
vibez.spill("Split parts count: ")
vibez.spill(len(parts))

// Test Join function
sus words []tea = ["Hello", "from", "CURSED"]
sus joined tea = stringz.Join(words, " ")
vibez.spill("Joined: ")
vibez.spill(joined)

vibez.spill("Stringz test complete!")
