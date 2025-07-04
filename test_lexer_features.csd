fr fr Test new lexer features

slay test_features() {
    fr fr Test string escapes
    sus escaped_string tea = "Hello\nWorld\t!\\"Quoted\\""
    sus raw_string tea = `Raw string with \n and \t literals`
    
    fr fr Test number formats
    sus binary_num normie = 0b1010      fr fr binary 10
    sus octal_num normie = 0o755        fr fr octal 493
    sus hex_num normie = 0xFF           fr fr hex 255
    
    fr fr Test assignment operators (when parser supports them)
    sus counter normie = 5
    fr fr counter += 3  // Will be: counter = counter + 3
    fr fr counter -= 1  // Will be: counter = counter - 1
    
    vibez.spill("Lexer features test completed!")
}

slay main() {
    test_features()
}
