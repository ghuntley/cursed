yeet "stringz"
yeet "vibez"

# Test stringz module functionality
vibez.spill("Testing stringz module...")

# Test basic string operations
sus text tea = "Hello, World!"
vibez.spill("Original text: " + text)
vibez.spill("Length: " + stringz.Length(text))
vibez.spill("Uppercase: " + stringz.ToUpper(text))
vibez.spill("Lowercase: " + stringz.ToLower(text))

# Test string searching
vibez.spill("Contains 'World': " + stringz.Contains(text, "World"))
vibez.spill("Index of 'World': " + stringz.IndexOf(text, "World"))
vibez.spill("Has prefix 'Hello': " + stringz.HasPrefix(text, "Hello"))
vibez.spill("Has suffix '!': " + stringz.HasSuffix(text, "!"))

# Test string manipulation
sus trimmed tea = stringz.Trim("  Hello World  ")
vibez.spill("Trimmed: '" + trimmed + "'")

sus replaced tea = stringz.Replace(text, "World", "CURSED")
vibez.spill("Replaced: " + replaced)

# Test string splitting and joining
sus csv_data tea = "apple,banana,orange"
sus fruits [tea] = stringz.Split(csv_data, ",")
vibez.spill("Split CSV: " + fruits[0] + ", " + fruits[1] + ", " + fruits[2])

sus joined tea = stringz.Join(fruits, " | ")
vibez.spill("Joined with pipes: " + joined)

# Test string reversal
sus reversed tea = stringz.Reverse("CURSED")
vibez.spill("Reversed 'CURSED': " + reversed)

vibez.spill("stringz module test complete!")
