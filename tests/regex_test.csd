vibe regex_test

yeet (
    "vibez"
    "regex_vibez"
)

slay main() {
    sus pattern tea = "\\w+"
    sus input tea = "This is a test of the regex functionality."
    
    // Test matches
    lowkey regex_vibez.matches(pattern, input) {
        vibez.println("Pattern matches: found 'This'")
    }
    
    // Test find_all to get all words
    sus words = regex_vibez.find_all(pattern, input)
    vibez.println("Found words: ")
    bestie i := 0; i < len(words); i++ {
        vibez.println(words[i])
    }
    
    // Test find
    sus first_word = regex_vibez.find(pattern, input)
    vibez.println("First word: " + first_word)
    
    // Test split
    sus parts = regex_vibez.split("\\s+", input)
    vibez.println("Split parts: ")
    bestie i := 0; i < len(parts); i++ {
        vibez.println(parts[i])
    }
    
    // Test replace_all
    sus replaced = regex_vibez.replace_all("test", input, "DEMO")
    vibez.println("Replaced: " + replaced)
    
    // Test capture groups with extract
    sus email_pattern tea = "(\\w+)@(\\w+)\\.(\\w+)"
    sus email tea = "user@example.com"
    sus captures = regex_vibez.extract(email_pattern, email)
    
    lowkey len(captures) >= 4 {
        vibez.println("Full match: " + captures[0])
        vibez.println("Username: " + captures[1])
        vibez.println("Domain: " + captures[2])
        vibez.println("TLD: " + captures[3])
    }
}