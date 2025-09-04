vibe main
yeet "stringz"
yeet "vibez"

slay main_character() {
    vibez.spill("=== String Concat Debug ===")
    
    sus base_str = "CURSED"
    vibez.spill("base_str:", base_str)
    vibez.spill("base_str length:", stringz.length(base_str))
    
    sus long_str = base_str
    vibez.spill("Initial long_str:", long_str)
    vibez.spill("Initial length:", stringz.length(long_str))
    
    long_str = stringz.concat(long_str, base_str)
    vibez.spill("After 1st concat:", long_str)
    vibez.spill("Length after 1st:", stringz.length(long_str))
    
    long_str = stringz.concat(long_str, base_str)
    vibez.spill("After 2nd concat:", long_str)
    vibez.spill("Length after 2nd:", stringz.length(long_str))
    
    long_str = stringz.concat(long_str, base_str)
    vibez.spill("After 3rd concat:", long_str)
    vibez.spill("Final length:", stringz.length(long_str))
}
