vibe main

yeet "vibez"    fr fr For printing results
yeet "stringz"  fr fr String manipulation package

slay main() {
    vibez.spill("Testing stringz package")
    
    fr fr Test contains function
    tea s := "Hello, world!"
    lowkey stringz.Contains(s, "world") {
        vibez.spill("Contains: 'world' is in the string")
    } highkey {
        vibez.spill("Contains: 'world' is NOT in the string")
    }
    
    lowkey stringz.Contains(s, "universe") {
        vibez.spill("Contains: 'universe' is in the string")
    } highkey {
        vibez.spill("Contains: 'universe' is NOT in the string")
    }
    
    fr fr Test count function
    vibez.spill("Count of 'l' in '" + s + "':", stringz.Count(s, "l"))
    
    fr fr Test prefix/suffix functions
    vibez.spill("HasPrefix 'Hello':", stringz.HasPrefix(s, "Hello"))
    vibez.spill("HasPrefix 'Hi':", stringz.HasPrefix(s, "Hi"))
    vibez.spill("HasSuffix '!':", stringz.HasSuffix(s, "!"))
    vibez.spill("HasSuffix 'world':", stringz.HasSuffix(s, "world"))
    
    fr fr Test split and join functions
    tea parts := stringz.Split(s, ", ")
    vibez.spill("Split result: parts[0] =", parts[0], "parts[1] =", parts[1])
    
    tea joined := stringz.Join(parts, " - ")
    vibez.spill("Joined with ' - ':", joined)
    
    fr fr Test case conversion
    vibez.spill("ToUpper:", stringz.ToUpper(s))
    vibez.spill("ToLower:", stringz.ToLower(s))
    
    fr fr Test trim function
    tea trimmed := stringz.Trim("...Hello...", ".")
    vibez.spill("Trim '.' from '...Hello...':", trimmed)
}