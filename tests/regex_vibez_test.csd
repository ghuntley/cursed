vibe main

yeet "vibez"      fr fr For printing results
yeet "regex_vibez" fr fr Regular expression functionality

slay main() {
    vibez.spill("Testing regex_vibez package")
    
    fr fr Test basic pattern matching
    test_basic_matching()
    
    fr fr Test capturing groups
    test_capture_groups()
    
    fr fr Test match finding
    test_find_match()
    
    fr fr Test replacement
    test_replace()
    
    fr fr Test split function
    test_split()
    
    vibez.spill("All regex_vibez tests passed!")
}

fr fr Test basic pattern matching
slay test_basic_matching() {
    vibez.spill("Testing basic pattern matching...")
    
    fr fr Create some test patterns
    tea patterns := []tea{
        "^Hello",      fr fr Starts with Hello
        "world$",      fr fr Ends with world
        "\\d+",         fr fr One or more digits
        "[a-z]+",      fr fr One or more lowercase letters
        "\\b\\w{3}\\b", fr fr 3-letter word
    }
    
    fr fr Test strings
    tea tests := []tea{
        "Hello, world!",
        "The price is $42.99",
        "This is a test of regex functions",
        "Matched cat not dog",
    }
    
    fr fr Test pattern matching
    bestie _, pattern := range patterns {
        vibez.spill("Pattern:", pattern)
        
        fr fr Compile the pattern
        tea regex, err := regex_vibez.Compile(pattern)
        lowkey err != cap {
            vibez.spill("Error compiling pattern:", err)
            continue
        }
        
        fr fr Test against each string
        bestie _, test := range tests {
            tea matches := regex.MatchString(test)
            vibez.spill("  '", test, "' matches:", matches)
            
            lowkey matches {
                fr fr Get the matching text
                tea found := regex.FindString(test)
                vibez.spill("    matched:", found)
            }
        }
        
        vibez.spill("")
    }
    
    fr fr Test specific cases
    tea digitRegex, _ := regex_vibez.Compile("\\d+")
    tea foundDigits := digitRegex.FindString("The price is $42.99")
    lowkey foundDigits != "42" {
        vibez.spill("Failed to find digits. Expected '42', got '", foundDigits, "'")
        yolo
    }
    
    tea wordRegex, _ := regex_vibez.Compile("\\b\\w{3}\\b")
    tea foundWord := wordRegex.FindString("The cat and dog ran")
    lowkey foundWord != "cat" && foundWord != "dog" && foundWord != "ran" && foundWord != "The" {
        vibez.spill("Failed to find 3-letter word. Got '", foundWord, "'")
        yolo
    }
    
    vibez.spill("Basic pattern matching tests passed!")
}

fr fr Test capturing groups
slay test_capture_groups() {
    vibez.spill("Testing capturing groups...")
    
    fr fr Pattern with groups
    tea namePattern := "([A-Z][a-z]+) ([A-Z][a-z]+)"  fr fr First name and last name
    
    fr fr Compile the pattern
    tea regex, err := regex_vibez.Compile(namePattern)
    lowkey err != cap {
        vibez.spill("Error compiling pattern:", err)
        yolo
    }
    
    fr fr Test string
    tea testStr := "John Smith is a developer"
    
    fr fr Find submatches
    tea matches := regex.FindSubmatch(testStr)
    lowkey len(matches) < 3 {
        vibez.spill("Expected at least 3 matches (full + 2 groups), got", len(matches))
        yolo
    }
    
    fr fr Verify matches
    vibez.spill("Full match:", matches[0])
    vibez.spill("First name:", matches[1])
    vibez.spill("Last name:", matches[2])
    
    lowkey matches[0] != "John Smith" {
        vibez.spill("Full match incorrect. Expected 'John Smith', got '", matches[0], "'")
        yolo
    }
    
    lowkey matches[1] != "John" {
        vibez.spill("First name incorrect. Expected 'John', got '", matches[1], "'")
        yolo
    }
    
    lowkey matches[2] != "Smith" {
        vibez.spill("Last name incorrect. Expected 'Smith', got '", matches[2], "'")
        yolo
    }
    
    fr fr Test named groups
    tea emailPattern := "(?P<username>[\\w.]+)@(?P<domain>[\\w.]+\\.\\w+)"  fr fr Email with named groups
    
    fr fr Compile the pattern
    regex, err = regex_vibez.Compile(emailPattern)
    lowkey err != cap {
        vibez.spill("Error compiling pattern:", err)
        yolo
    }
    
    fr fr Test string
    testStr = "Contact us at info@example.com or support@company.org"
    
    fr fr Find named matches
    tea namedMatches := regex.FindStringSubmatchMap(testStr)
    lowkey len(namedMatches) == 0 {
        vibez.spill("No matches found")
        yolo
    }
    
    fr fr Verify matches
    vibez.spill("Username:", namedMatches["username"])
    vibez.spill("Domain:", namedMatches["domain"])
    
    lowkey namedMatches["username"] != "info" {
        vibez.spill("Username incorrect. Expected 'info', got '", namedMatches["username"], "'")
        yolo
    }
    
    lowkey namedMatches["domain"] != "example.com" {
        vibez.spill("Domain incorrect. Expected 'example.com', got '", namedMatches["domain"], "'")
        yolo
    }
    
    vibez.spill("Capturing groups tests passed!")
}

fr fr Test find match functions
slay test_find_match() {
    vibez.spill("Testing find match functions...")
    
    fr fr Pattern for finding words
    tea wordPattern := "\\b\\w+\\b"
    
    fr fr Compile the pattern
    tea regex, err := regex_vibez.Compile(wordPattern)
    lowkey err != cap {
        vibez.spill("Error compiling pattern:", err)
        yolo
    }
    
    fr fr Test string
    tea testStr := "The quick brown fox jumps over the lazy dog"
    
    fr fr FindString - find first match
    tea firstWord := regex.FindString(testStr)
    vibez.spill("First word:", firstWord)
    lowkey firstWord != "The" {
        vibez.spill("Expected 'The', got '", firstWord, "'")
        yolo
    }
    
    fr fr FindAllString - find all matches
    tea allWords := regex.FindAllString(testStr, -1)  fr fr -1 means all matches
    vibez.spill("All words:", allWords)
    lowkey len(allWords) != 9 {
        vibez.spill("Expected 9 words, got", len(allWords))
        yolo
    }
    
    fr fr FindAllString with limit
    tea limitedWords := regex.FindAllString(testStr, 3)  fr fr Only first 3 matches
    vibez.spill("First 3 words:", limitedWords)
    lowkey len(limitedWords) != 3 {
        vibez.spill("Expected 3 words, got", len(limitedWords))
        yolo
    }
    
    fr fr FindStringIndex - find index of match
    tea firstIndex := regex.FindStringIndex(testStr)
    vibez.spill("First word index:", firstIndex)
    lowkey firstIndex[0] != 0 || firstIndex[1] != 3 {
        vibez.spill("Expected [0,3], got ", firstIndex)
        yolo
    }
    
    fr fr FindAllStringIndex - find all indexes
    tea allIndexes := regex.FindAllStringIndex(testStr, 3)
    vibez.spill("First 3 word indexes:", allIndexes)
    lowkey len(allIndexes) != 3 {
        vibez.spill("Expected 3 index pairs, got", len(allIndexes))
        yolo
    }
    
    vibez.spill("Find match tests passed!")
}

fr fr Test replacement functions
slay test_replace() {
    vibez.spill("Testing replacement functions...")
    
    fr fr Pattern for finding numbers
    tea numPattern := "\\d+"
    
    fr fr Compile the pattern
    tea regex, err := regex_vibez.Compile(numPattern)
    lowkey err != cap {
        vibez.spill("Error compiling pattern:", err)
        yolo
    }
    
    fr fr Test string
    tea testStr := "The price is $42.99 for 3 items"
    
    fr fr ReplaceAllString - replace all matches with a fixed string
    tea replaced := regex.ReplaceAllString(testStr, "XX")
    vibez.spill("Numbers replaced with XX:", replaced)
    lowkey replaced != "The price is $XX.XX for XX items" {
        vibez.spill("Incorrect replacement. Got '", replaced, "'")
        yolo
    }
    
    fr fr ReplaceAllStringFunc - replace using a function
    tea replacedFunc := regex.ReplaceAllStringFunc(testStr, slay(match tea) tea {
        fr fr Double the number
        tea num := normie(match)
        yolo tea(num * 2)
    })
    
    vibez.spill("Numbers doubled:", replacedFunc)
    lowkey !regex_vibez.Contains(replacedFunc, "$84") || !regex_vibez.Contains(replacedFunc, "6 items") {
        vibez.spill("Incorrect functional replacement. Got '", replacedFunc, "'")
        yolo
    }
    
    fr fr Test capture group replacement
    tea namePattern := "([A-Z][a-z]+) ([A-Z][a-z]+)"
    regex, err = regex_vibez.Compile(namePattern)
    lowkey err != cap {
        vibez.spill("Error compiling pattern:", err)
        yolo
    }
    
    tea nameStr := "John Smith is here and Jane Doe is there"
    
    tea nameSwapped := regex.ReplaceAllString(nameStr, "$2, $1")
    vibez.spill("Names with last name first:", nameSwapped)
    lowkey !regex_vibez.Contains(nameSwapped, "Smith, John") || !regex_vibez.Contains(nameSwapped, "Doe, Jane") {
        vibez.spill("Incorrect group replacement. Got '", nameSwapped, "'")
        yolo
    }
    
    vibez.spill("Replacement tests passed!")
}

fr fr Test split function
slay test_split() {
    vibez.spill("Testing split function...")
    
    fr fr Pattern for splitting by punctuation or whitespace
    tea splitPattern := "[\\s,.;:!?]+"
    
    fr fr Compile the pattern
    tea regex, err := regex_vibez.Compile(splitPattern)
    lowkey err != cap {
        vibez.spill("Error compiling pattern:", err)
        yolo
    }
    
    fr fr Test string
    tea testStr := "Hello, world! This is a test; with various punctuation."
    
    fr fr Split the string
    tea parts := regex.Split(testStr, -1)  fr fr -1 means all parts
    vibez.spill("Split parts:", parts)
    lowkey len(parts) < 8 {
        vibez.spill("Expected at least 8 parts, got", len(parts))
        yolo
    }
    
    fr fr Check specific parts
    lowkey parts[0] != "Hello" {
        vibez.spill("First part should be 'Hello', got '", parts[0], "'")
        yolo
    }
    
    lowkey parts[1] != "world" {
        vibez.spill("Second part should be 'world', got '", parts[1], "'")
        yolo
    }
    
    fr fr Split with limit
    tea limitedParts := regex.Split(testStr, 3)  fr fr Only first 3 parts
    vibez.spill("Limited split (3):", limitedParts)
    lowkey len(limitedParts) != 3 {
        vibez.spill("Expected 3 parts, got", len(limitedParts))
        yolo
    }
    
    vibez.spill("Split tests passed!")
}