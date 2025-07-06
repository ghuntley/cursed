sus main() {
    // Test basic character literals
    sus char_a sip = 'a';
    sus char_b sip = 'B'; 
    sus char_digit sip = '5';
    sus char_space sip = ' ';
    
    // Test escape sequences
    sus char_newline sip = '\n';
    sus char_tab sip = '\t';
    sus char_quote sip = '\'';
    sus char_backslash sip = '\\';
    sus char_null sip = '\0';
    
    // Test character in expressions
    sus result sip = char_a;
    
    return result;
}
