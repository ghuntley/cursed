slay main() {
    vibez.spill("=== CURSED Character Type (sip) Comprehensive Test ===")
    
    fr fr Basic character literals
    sus char_a sip = 'a'
    sus char_B sip = 'B'
    sus char_5 sip = '5'
    sus char_space sip = ' '
    
    vibez.spill("Basic characters:")
    vibez.spill("char_a: " + char_a)
    vibez.spill("char_B: " + char_B)
    vibez.spill("char_5: " + char_5)
    
    fr fr Escape sequences
    sus char_newline sip = '\n'
    sus char_tab sip = '\t'
    sus char_backslash sip = '\\'
    sus char_quote sip = '\''
    sus char_null sip = '\0'
    
    vibez.spill("Escape sequences:")
    vibez.spill("newline char: " + char_newline)
    vibez.spill("tab char: " + char_tab)
    vibez.spill("backslash: " + char_backslash)
    vibez.spill("quote: " + char_quote)
    vibez.spill("null char: " + char_null)
    
    fr fr Character in conditionals
    lowkey char_a == 'a' {
        vibez.spill("Character comparison works: char_a == 'a'")
    }
    
    lowkey char_5 != 'x' {
        vibez.spill("Character inequality works: char_5 != 'x'")
    }
    
    fr fr Test function with character parameter
    sus result sip = testCharFunction('Z')
    vibez.spill("Function returned character: " + result)
    
    vibez.spill("=== Character type implementation COMPLETE! ===")
    
    yolo 0
}

slay testCharFunction(c sip) sip {
    vibez.spill("Received character parameter: " + c)
    yolo c
}
