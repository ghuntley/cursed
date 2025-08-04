fr fr CURSED Lexer Implementation - Pure CURSED self-hosting module
fr fr This is the lexer for the CURSED language, written in CURSED itself

yeet "testz"

fr fr Token kinds enumeration
squad TokenKind {
    spill value normie
}

fr fr Token kind constants
facts TOKEN_NUMBER drip = 1
facts TOKEN_INTEGER drip = 2
facts TOKEN_STRING_LITERAL drip = 3
facts TOKEN_STRING drip = 4
facts TOKEN_BOOLEAN drip = 5
facts TOKEN_CHARACTER drip = 6
facts TOKEN_BASED drip = 7
facts TOKEN_IDENTIFIER drip = 8
facts TOKEN_SLAY drip = 9
facts TOKEN_SUS drip = 10
facts TOKEN_FACTS drip = 11
facts TOKEN_YEET drip = 12
facts TOKEN_DAMN drip = 13
facts TOKEN_VIBEZ drip = 14
facts TOKEN_SPILL drip = 15
facts TOKEN_LIT drip = 16
facts TOKEN_NORMIE drip = 17
facts TOKEN_TEA drip = 18
facts TOKEN_MEAL drip = 19
facts TOKEN_DRIP drip = 20
facts TOKEN_BESTIE drip = 21
facts TOKEN_STAN drip = 22
facts TOKEN_SQUAD drip = 23
facts TOKEN_COLLAB drip = 24
facts TOKEN_FLEX drip = 25
facts TOKEN_MATCH drip = 26
facts TOKEN_VIBES drip = 27
facts TOKEN_LEFT_PAREN drip = 28
facts TOKEN_RIGHT_PAREN drip = 29
facts TOKEN_LEFT_BRACE drip = 30
facts TOKEN_RIGHT_BRACE drip = 31
facts TOKEN_LEFT_BRACKET drip = 32
facts TOKEN_RIGHT_BRACKET drip = 33
facts TOKEN_COMMA drip = 34
facts TOKEN_SEMICOLON drip = 35
facts TOKEN_COLON drip = 36
facts TOKEN_DOT drip = 37
facts TOKEN_PLUS drip = 38
facts TOKEN_MINUS drip = 39
facts TOKEN_STAR drip = 40
facts TOKEN_SLASH drip = 41
facts TOKEN_EQUAL drip = 42
facts TOKEN_EQUAL_EQUAL drip = 43
facts TOKEN_BANG drip = 44
facts TOKEN_BANG_EQUAL drip = 45
facts TOKEN_LESS drip = 46
facts TOKEN_LESS_EQUAL drip = 47
facts TOKEN_GREATER drip = 48
facts TOKEN_GREATER_EQUAL drip = 49
facts TOKEN_AMP_AMP drip = 50
facts TOKEN_PIPE_PIPE drip = 51
facts TOKEN_NEWLINE drip = 52
facts TOKEN_EOF drip = 53
facts TOKEN_LINE_COMMENT drip = 54
facts TOKEN_BLOCK_COMMENT drip = 55
facts TOKEN_CRINGE drip = 56
facts TOKEN_BASED_LIT drip = 57

fr fr Token structure
squad Token {
    spill kind normie
    spill lexeme tea
    spill line normie
    spill column normie
}

slay token_init(kind normie, lexeme tea, line normie, column normie) Token {
    sus token Token = Token{
        kind: kind,
        lexeme: lexeme,
        line: line,
        column: column
    }
    damn token
}

fr fr Lexer structure
squad Lexer {
    spill input tea
    spill position normie
    spill line normie
    spill column normie
    spill length normie
}

slay lexer_init(input tea) Lexer {
    sus lexer Lexer = Lexer{
        input: input,
        position: 0,
        line: 1,
        column: 1,
        length: string_length(input)
    }
    damn lexer
}

fr fr Helper function to get string length
slay string_length(s tea) normie {
    sus length normie = 0
    bestie i := 0; i < 10000; i = i + 1 {
        sus char tea = string_char_at(s, i)
        if char == "" {
            vibes
        }
        length = length + 1
    }
    damn length
}

fr fr Helper function to get character at position
slay string_char_at(s tea, pos normie) tea {
    fr fr This is a placeholder - in real implementation would use native string indexing
    if pos >= string_length(s) {
        damn ""
    }
    damn string_substring(s, pos, pos + 1)
}

fr fr Helper function to get substring
slay string_substring(s tea, start normie, end normie) tea {
    fr fr Placeholder implementation
    damn s
}

fr fr Check if lexer is at end of input
slay lexer_is_at_end(lexer Lexer) lit {
    damn lexer.position >= lexer.length
}

fr fr Advance to next character
slay lexer_advance(lexer Lexer) tea {
    if lexer_is_at_end(lexer) {
        damn ""
    }
    sus char tea = string_char_at(lexer.input, lexer.position)
    lexer.position = lexer.position + 1
    lexer.column = lexer.column + 1
    damn char
}

fr fr Peek at current character without advancing
slay lexer_peek(lexer Lexer) tea {
    if lexer_is_at_end(lexer) {
        damn ""
    }
    damn string_char_at(lexer.input, lexer.position)
}

fr fr Peek at next character
slay lexer_peek_next(lexer Lexer) tea {
    if lexer.position + 1 >= lexer.length {
        damn ""
    }
    damn string_char_at(lexer.input, lexer.position + 1)
}

fr fr Match expected character
slay lexer_match(lexer Lexer, expected tea) lit {
    if lexer_is_at_end(lexer) {
        damn cringe
    }
    if lexer_peek(lexer) != expected {
        damn cringe
    }
    lexer.position = lexer.position + 1
    lexer.column = lexer.column + 1
    damn based
}

fr fr Skip whitespace characters
slay lexer_skip_whitespace(lexer Lexer) {
    bestie !lexer_is_at_end(lexer) {
        sus c tea = lexer_peek(lexer)
        if c == " " || c == "\r" || c == "\t" {
            lexer_advance(lexer)
        } else {
            vibes
        }
    }
}

fr fr Create token with current lexeme
slay lexer_make_token(lexer Lexer, kind normie, line normie, column normie) Token {
    sus start normie = lexer.position - 1
    if start < 0 {
        start = 0
    }
    sus lexeme tea = string_substring(lexer.input, start, lexer.position)
    damn token_init(kind, lexeme, line, column)
}

fr fr Check if character is digit
slay is_digit(c tea) lit {
    damn c == "0" || c == "1" || c == "2" || c == "3" || c == "4" || 
         c == "5" || c == "6" || c == "7" || c == "8" || c == "9"
}

fr fr Check if character is alphabetic
slay is_alpha(c tea) lit {
    damn (c >= "a" && c <= "z") || (c >= "A" && c <= "Z") || c == "_"
}

fr fr Check if character is alphanumeric
slay is_alnum(c tea) lit {
    damn is_alpha(c) || is_digit(c)
}

fr fr Parse number token
slay lexer_number(lexer Lexer, line normie, column normie) Token {
    sus start normie = lexer.position
    
    bestie is_digit(lexer_peek(lexer)) {
        lexer_advance(lexer)
    }
    
    fr fr Look for decimal point
    if lexer_peek(lexer) == "." && is_digit(lexer_peek_next(lexer)) {
        lexer_advance(lexer)  fr fr consume '.'
        bestie is_digit(lexer_peek(lexer)) {
            lexer_advance(lexer)
        }
    }
    
    sus lexeme tea = string_substring(lexer.input, start, lexer.position)
    damn token_init(TOKEN_NUMBER, lexeme, line, column)
}

fr fr Parse string literal token
slay lexer_string_literal(lexer Lexer, line normie, column normie) Token {
    sus start normie = lexer.position - 1  fr fr include opening quote
    
    bestie lexer_peek(lexer) != "\"" && !lexer_is_at_end(lexer) {
        if lexer_peek(lexer) == "\n" {
            lexer.line = lexer.line + 1
            lexer.column = 1
        }
        if lexer_peek(lexer) == "\\" {
            lexer_advance(lexer)  fr fr skip escape character
            if !lexer_is_at_end(lexer) {
                lexer_advance(lexer)  fr fr skip escaped character
            }
        } else {
            lexer_advance(lexer)
        }
    }
    
    if lexer_is_at_end(lexer) {
        fr fr TODO: Return error token for unterminated string
        damn token_init(TOKEN_EOF, "", line, column)
    }
    
    fr fr consume closing quote
    lexer_advance(lexer)
    
    sus lexeme tea = string_substring(lexer.input, start, lexer.position)
    damn token_init(TOKEN_STRING_LITERAL, lexeme, line, column)
}

fr fr Parse identifier or keyword token
slay lexer_identifier(lexer Lexer, line normie, column normie) Token {
    sus start normie = lexer.position
    
    bestie is_alnum(lexer_peek(lexer)) {
        lexer_advance(lexer)
    }
    
    sus lexeme tea = string_substring(lexer.input, start, lexer.position)
    
    fr fr Check for keywords
    sus kind normie = get_keyword_type(lexeme)
    
    damn token_init(kind, lexeme, line, column)
}

fr fr Get token kind for keyword or identifier
slay get_keyword_type(text tea) normie {
    fr fr CURSED keywords
    if text == "slay" { damn TOKEN_SLAY }
    if text == "sus" { damn TOKEN_SUS }
    if text == "facts" { damn TOKEN_FACTS }
    if text == "yeet" { damn TOKEN_YEET }
    if text == "damn" { damn TOKEN_DAMN }
    if text == "vibez" { damn TOKEN_VIBEZ }
    if text == "spill" { damn TOKEN_SPILL }
    if text == "lit" { damn TOKEN_LIT }
    if text == "normie" { damn TOKEN_NORMIE }
    if text == "tea" { damn TOKEN_TEA }
    if text == "meal" { damn TOKEN_MEAL }
    if text == "drip" { damn TOKEN_DRIP }
    if text == "bestie" { damn TOKEN_BESTIE }
    if text == "stan" { damn TOKEN_STAN }
    if text == "squad" { damn TOKEN_SQUAD }
    if text == "collab" { damn TOKEN_COLLAB }
    if text == "flex" { damn TOKEN_FLEX }
    if text == "match" { damn TOKEN_MATCH }
    if text == "vibes" { damn TOKEN_VIBES }
    if text == "based" { damn TOKEN_BASED_LIT }
    if text == "cringe" { damn TOKEN_CRINGE }
    
    fr fr Default to identifier
    damn TOKEN_IDENTIFIER
}

fr fr Get next token from input
slay lexer_next_token(lexer Lexer) Token {
    lexer_skip_whitespace(lexer)
    
    if lexer_is_at_end(lexer) {
        damn token_init(TOKEN_EOF, "", lexer.line, lexer.column)
    }
    
    sus c tea = lexer_advance(lexer)
    sus start_line normie = lexer.line
    sus start_column normie = lexer.column - 1
    
    fr fr Single character tokens
    if c == "(" { damn lexer_make_token(lexer, TOKEN_LEFT_PAREN, start_line, start_column) }
    if c == ")" { damn lexer_make_token(lexer, TOKEN_RIGHT_PAREN, start_line, start_column) }
    if c == "{" { damn lexer_make_token(lexer, TOKEN_LEFT_BRACE, start_line, start_column) }
    if c == "}" { damn lexer_make_token(lexer, TOKEN_RIGHT_BRACE, start_line, start_column) }
    if c == "[" { damn lexer_make_token(lexer, TOKEN_LEFT_BRACKET, start_line, start_column) }
    if c == "]" { damn lexer_make_token(lexer, TOKEN_RIGHT_BRACKET, start_line, start_column) }
    if c == "," { damn lexer_make_token(lexer, TOKEN_COMMA, start_line, start_column) }
    if c == ";" { damn lexer_make_token(lexer, TOKEN_SEMICOLON, start_line, start_column) }
    if c == ":" { damn lexer_make_token(lexer, TOKEN_COLON, start_line, start_column) }
    if c == "." { damn lexer_make_token(lexer, TOKEN_DOT, start_line, start_column) }
    
    fr fr Operators
    if c == "+" { damn lexer_make_token(lexer, TOKEN_PLUS, start_line, start_column) }
    if c == "-" { damn lexer_make_token(lexer, TOKEN_MINUS, start_line, start_column) }
    if c == "*" { damn lexer_make_token(lexer, TOKEN_STAR, start_line, start_column) }
    if c == "/" { damn lexer_make_token(lexer, TOKEN_SLASH, start_line, start_column) }
    
    fr fr Comparison operators
    if c == "=" {
        if lexer_match(lexer, "=") {
            damn lexer_make_token(lexer, TOKEN_EQUAL_EQUAL, start_line, start_column)
        }
        damn lexer_make_token(lexer, TOKEN_EQUAL, start_line, start_column)
    }
    
    if c == "!" {
        if lexer_match(lexer, "=") {
            damn lexer_make_token(lexer, TOKEN_BANG_EQUAL, start_line, start_column)
        }
        damn lexer_make_token(lexer, TOKEN_BANG, start_line, start_column)
    }
    
    if c == "<" {
        if lexer_match(lexer, "=") {
            damn lexer_make_token(lexer, TOKEN_LESS_EQUAL, start_line, start_column)
        }
        damn lexer_make_token(lexer, TOKEN_LESS, start_line, start_column)
    }
    
    if c == ">" {
        if lexer_match(lexer, "=") {
            damn lexer_make_token(lexer, TOKEN_GREATER_EQUAL, start_line, start_column)
        }
        damn lexer_make_token(lexer, TOKEN_GREATER, start_line, start_column)
    }
    
    fr fr Logical operators
    if c == "&" {
        if lexer_match(lexer, "&") {
            damn lexer_make_token(lexer, TOKEN_AMP_AMP, start_line, start_column)
        }
    }
    
    if c == "|" {
        if lexer_match(lexer, "|") {
            damn lexer_make_token(lexer, TOKEN_PIPE_PIPE, start_line, start_column)
        }
    }
    
    fr fr String literals
    if c == "\"" {
        damn lexer_string_literal(lexer, start_line, start_column)
    }
    
    fr fr Newline
    if c == "\n" {
        lexer.line = lexer.line + 1
        lexer.column = 1
        damn lexer_make_token(lexer, TOKEN_NEWLINE, start_line, start_column)
    }
    
    fr fr Numbers
    if is_digit(c) {
        lexer.position = lexer.position - 1  fr fr back up to re-read
        lexer.column = lexer.column - 1
        damn lexer_number(lexer, start_line, start_column)
    }
    
    fr fr Identifiers and keywords
    if is_alpha(c) {
        lexer.position = lexer.position - 1  fr fr back up to re-read
        lexer.column = lexer.column - 1
        damn lexer_identifier(lexer, start_line, start_column)
    }
    
    fr fr Default to EOF for unrecognized characters
    damn token_init(TOKEN_EOF, "", start_line, start_column)
}

fr fr Tokenize entire input into array of tokens
slay lexer_tokenize(lexer Lexer) []Token {
    fr fr Simplified token array - in real implementation would use dynamic array
    sus tokens []Token = []
    
    bestie !lexer_is_at_end(lexer) {
        sus token Token = lexer_next_token(lexer)
        fr fr Skip comments and newlines
        if token.kind != TOKEN_NEWLINE && token.kind != TOKEN_LINE_COMMENT && token.kind != TOKEN_BLOCK_COMMENT {
            fr fr In real implementation would append to dynamic array
        }
        if token.kind == TOKEN_EOF {
            vibes
        }
    }
    
    damn tokens
}

fr fr Test basic lexer functionality
slay test_lexer_basic() {
    test_start("Lexer Basic Tokens")
    
    sus input tea = "slay main_character() { }"
    sus lexer Lexer = lexer_init(input)
    
    sus token1 Token = lexer_next_token(lexer)
    assert_eq_int(token1.kind, TOKEN_SLAY)
    
    sus token2 Token = lexer_next_token(lexer)
    assert_eq_int(token2.kind, TOKEN_IDENTIFIER)
    
    sus token3 Token = lexer_next_token(lexer)
    assert_eq_int(token3.kind, TOKEN_LEFT_PAREN)
    
    sus token4 Token = lexer_next_token(lexer)
    assert_eq_int(token4.kind, TOKEN_RIGHT_PAREN)
    
    sus token5 Token = lexer_next_token(lexer)
    assert_eq_int(token5.kind, TOKEN_LEFT_BRACE)
}

slay test_lexer_numbers() {
    test_start("Lexer Numbers")
    
    sus input tea = "42 3.14"
    sus lexer Lexer = lexer_init(input)
    
    sus token1 Token = lexer_next_token(lexer)
    assert_eq_int(token1.kind, TOKEN_NUMBER)
    
    sus token2 Token = lexer_next_token(lexer)
    assert_eq_int(token2.kind, TOKEN_NUMBER)
}

slay test_lexer_strings() {
    test_start("Lexer Strings")
    
    sus input tea = "\"hello world\""
    sus lexer Lexer = lexer_init(input)
    
    sus token Token = lexer_next_token(lexer)
    assert_eq_int(token.kind, TOKEN_STRING_LITERAL)
}

slay test_lexer_keywords() {
    test_start("Lexer Keywords")
    
    sus input tea = "sus facts yeet damn"
    sus lexer Lexer = lexer_init(input)
    
    sus token1 Token = lexer_next_token(lexer)
    assert_eq_int(token1.kind, TOKEN_SUS)
    
    sus token2 Token = lexer_next_token(lexer)
    assert_eq_int(token2.kind, TOKEN_FACTS)
    
    sus token3 Token = lexer_next_token(lexer)
    assert_eq_int(token3.kind, TOKEN_YEET)
    
    sus token4 Token = lexer_next_token(lexer)
    assert_eq_int(token4.kind, TOKEN_DAMN)
}

fr fr Run all lexer tests
slay run_lexer_tests() {
    test_lexer_basic()
    test_lexer_numbers()
    test_lexer_strings()
    test_lexer_keywords()
    print_test_summary()
}
