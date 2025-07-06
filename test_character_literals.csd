// Test character literals in CURSED
sus main() {
    // Test single character literals
    sip char_a = 'a';
    sip char_b = 'B';
    sip char_space = ' ';
    sip char_digit = '5';
    sip char_newline = '\n';
    sip char_tab = '\t';
    sip char_quote = '\'';
    sip char_backslash = '\\';
    
    // Test character display
    spill("Character a: ");
    spill(char_a);
    spill("\n");
    
    spill("Character B: ");
    spill(char_b);
    spill("\n");
    
    spill("Character space: ");
    spill(char_space);
    spill("\n");
    
    spill("Character digit: ");
    spill(char_digit);
    spill("\n");
    
    spill("Character newline: ");
    spill(char_newline);
    spill("\n");
    
    spill("Character tab: ");
    spill(char_tab);
    spill("\n");
    
    spill("Character quote: ");
    spill(char_quote);
    spill("\n");
    
    spill("Character backslash: ");
    spill(char_backslash);
    spill("\n");
    
    // Test character in boolean contexts
    lit is_truthy = char_a;
    spill("Character 'a' is truthy: ");
    spill(is_truthy);
    spill("\n");
    
    sip null_char = '\0';
    lit is_null_truthy = null_char;
    spill("Null character is truthy: ");
    spill(is_null_truthy);
    spill("\n");
}
