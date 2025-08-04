fr fr Simple bootstrap lexer test without external dependencies
fr fr Testing basic lexer functionality in pure CURSED

fr fr Simple token structure
squad Token {
    spill token_type normie
    spill literal tea
    spill line normie
    spill column normie
}

fr fr Basic lexer state
squad Lexer {
    spill input tea
    spill position normie
    spill ch tea
    spill line normie
    spill column normie
}

fr fr Create new lexer
slay new_lexer(input tea) Lexer {
    sus lexer Lexer = Lexer{
        input: input,
        position: 0,
        ch: "",
        line: 1,
        column: 0
    }
    
    fr fr Read first character
    bestie (input.len() > 0) {
        lexer.ch = input.substring(0, 1)
    }
    
    damn lexer
}

fr fr Read next character
slay read_char(lexer Lexer) {
    lexer.position = lexer.position + 1
    bestie (lexer.position >= lexer.input.len()) {
        lexer.ch = ""
    } capish {
        lexer.ch = lexer.input.substring(lexer.position, lexer.position + 1)
    }
    
    bestie (lexer.ch == "\n") {
        lexer.line = lexer.line + 1
        lexer.column = 0
    } capish {
        lexer.column = lexer.column + 1
    }
}

fr fr Check if character is letter
slay is_letter(ch tea) lit {
    damn (ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || ch == "_"
}

fr fr Check if character is digit  
slay is_digit(ch tea) lit {
    damn ch >= "0" && ch <= "9"
}

fr fr Read identifier
slay read_identifier(lexer Lexer) tea {
    sus start normie = lexer.position
    
    bestie (is_letter(lexer.ch) || is_digit(lexer.ch)) {
        read_char(lexer)
    }
    
    damn lexer.input.substring(start, lexer.position)
}

fr fr Create token
slay new_token(token_type normie, literal tea, line normie, column normie) Token {
    damn Token{
        token_type: token_type,
        literal: literal,
        line: line,
        column: column
    }
}

fr fr Simple token types (numbers)
fr fr 1 = identifier, 2 = number, 3 = string, 4 = keyword, 5 = operator, 6 = eof

fr fr Get next token
slay next_token(lexer Lexer) Token {
    fr fr Skip whitespace
    bestie (lexer.ch == " " || lexer.ch == "\t" || lexer.ch == "\r") {
        read_char(lexer)
    }
    
    sus tok_line normie = lexer.line
    sus tok_column normie = lexer.column
    
    fr fr Check for different token types
    bestie (lexer.ch == "") {
        damn new_token(6, "", tok_line, tok_column)  fr fr EOF
    } capish bestie (is_letter(lexer.ch)) {
        sus literal tea = read_identifier(lexer)
        
        fr fr Check for keywords
        bestie (literal == "slay") {
            damn new_token(4, literal, tok_line, tok_column)  fr fr keyword
        } capish bestie (literal == "sus") {
            damn new_token(4, literal, tok_line, tok_column)  fr fr keyword
        } capish bestie (literal == "damn") {
            damn new_token(4, literal, tok_line, tok_column)  fr fr keyword
        } capish {
            damn new_token(1, literal, tok_line, tok_column)  fr fr identifier
        }
    } capish bestie (is_digit(lexer.ch)) {
        sus start normie = lexer.position
        bestie (is_digit(lexer.ch)) {
            read_char(lexer)
        }
        sus literal tea = lexer.input.substring(start, lexer.position)
        damn new_token(2, literal, tok_line, tok_column)  fr fr number
    } capish {
        sus ch tea = lexer.ch
        read_char(lexer)
        damn new_token(5, ch, tok_line, tok_column)  fr fr operator/symbol
    }
}

fr fr Test the lexer
slay test_lexer() {
    sus test_input tea = "slay hello() { sus x normie = 42 }"
    sus lexer Lexer = new_lexer(test_input)
    
    vibez.spill("Testing lexer with input:", test_input)
    
    bestie (based) {
        sus token Token = next_token(lexer)
        vibez.spill("Token type:", token.token_type, "literal:", token.literal)
        
        bestie (token.token_type == 6) {  fr fr EOF
            vibes
        }
    }
}

fr fr Main function
slay main() {
    vibez.spill("Bootstrap Lexer Test")
    test_lexer()
    vibez.spill("Lexer test complete")
}

main()
