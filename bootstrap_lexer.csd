fr fr Pure CURSED Lexer Implementation
fr fr Tokenizes CURSED source code using only CURSED language constructs

yeet "testz"

squad Token {
    spill type tea
    spill value tea
    spill line normie
    spill column normie
}

squad Lexer {
    spill source tea
    spill position normie
    spill line normie
    spill column normie
    spill tokens []Token
}

slay create_token(type tea, value tea, line normie, column normie) Token {
    damn Token{
        type: type,
        value: value, 
        line: line,
        column: column
    }
}

slay init_lexer(source tea) Lexer {
    damn Lexer{
        source: source,
        position: 0,
        line: 1,
        column: 1,
        tokens: []
    }
}

slay peek_char(lexer Lexer) tea {
    lowkey (lexer.position >= lexer.source.length()) {
        damn ""
    }
    damn lexer.source.char_at(lexer.position)
}

slay next_char(lexer Lexer) tea {
    lowkey (lexer.position >= lexer.source.length()) {
        damn ""
    }
    
    sus ch tea = lexer.source.char_at(lexer.position)
    lexer.position = lexer.position + 1
    
    lowkey (ch == "\n") {
        lexer.line = lexer.line + 1
        lexer.column = 1
    } highkey {
        lexer.column = lexer.column + 1
    }
    
    damn ch
}

slay skip_whitespace(lexer Lexer) {
    bestie true {
        sus ch tea = peek_char(lexer)
        lowkey (ch == " " || ch == "\t" || ch == "\n" || ch == "\r") {
            next_char(lexer)
        } highkey {
            vibes
        }
    }
}

slay skip_comment(lexer Lexer) {
    fr fr Skip single-line comment
    lowkey (peek_char(lexer) == "f" && peek_next_chars(lexer, 5) == "fr fr") {
        bestie true {
            sus ch tea = next_char(lexer)
            lowkey (ch == "\n" || ch == "") {
                vibes
            }
        }
    }
}

slay peek_next_chars(lexer Lexer, count normie) tea {
    sus result tea = ""
    bestie i := 0; i < count && lexer.position + i < lexer.source.length(); i = i + 1 {
        result = result + lexer.source.char_at(lexer.position + i)
    }
    damn result
}

slay is_alpha(ch tea) lit {
    damn (ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || ch == "_"
}

slay is_digit(ch tea) lit {
    damn ch >= "0" && ch <= "9"
}

slay is_alphanumeric(ch tea) lit {
    damn is_alpha(ch) || is_digit(ch)
}

slay read_identifier(lexer Lexer) tea {
    sus start normie = lexer.position
    
    bestie true {
        sus ch tea = peek_char(lexer)
        lowkey (is_alphanumeric(ch)) {
            next_char(lexer)
        } highkey {
            vibes
        }
    }
    
    damn lexer.source.substring(start, lexer.position)
}

slay read_number(lexer Lexer) tea {
    sus start normie = lexer.position
    sus has_dot lit = cringe
    
    bestie true {
        sus ch tea = peek_char(lexer)
        lowkey (is_digit(ch)) {
            next_char(lexer)
        } highkey lowkey (ch == "." && !has_dot) {
            has_dot = based
            next_char(lexer)
        } highkey {
            vibes
        }
    }
    
    damn lexer.source.substring(start, lexer.position)
}

slay read_string(lexer Lexer) tea {
    sus quote tea = next_char(lexer) fr fr consume opening quote
    sus start normie = lexer.position
    
    bestie true {
        sus ch tea = peek_char(lexer)
        lowkey (ch == quote) {
            next_char(lexer) fr fr consume closing quote
            vibes
        } highkey lowkey (ch == "") {
            fr fr Unterminated string
            vibes
        } highkey {
            next_char(lexer)
        }
    }
    
    damn lexer.source.substring(start, lexer.position - 1)
}

slay get_keyword_type(identifier tea) tea {
    lowkey (identifier == "slay") {
        damn "FUNCTION"
    } highkey lowkey (identifier == "sus") {
        damn "VARIABLE"
    } highkey lowkey (identifier == "yeet") {
        damn "IMPORT"
    } highkey lowkey (identifier == "vibez") {
        damn "VIBEZ"
    } highkey lowkey (identifier == "spill") {
        damn "SPILL"
    } highkey lowkey (identifier == "damn") {
        damn "RETURN"
    } highkey lowkey (identifier == "lowkey") {
        damn "IF"
    } highkey lowkey (identifier == "highkey") {
        damn "ELSE"
    } highkey lowkey (identifier == "bestie") {
        damn "FOR"
    } highkey lowkey (identifier == "squad") {
        damn "STRUCT"
    } highkey lowkey (identifier == "collab") {
        damn "INTERFACE"
    } highkey lowkey (identifier == "based") {
        damn "TRUE"
    } highkey lowkey (identifier == "cringe") {
        damn "FALSE"
    } highkey lowkey (identifier == "vibes") {
        damn "BREAK"
    } highkey lowkey (identifier == "stan") {
        damn "GOROUTINE"
    } highkey lowkey (identifier == "normie") {
        damn "TYPE_INT"
    } highkey lowkey (identifier == "tea") {
        damn "TYPE_STRING"
    } highkey lowkey (identifier == "lit") {
        damn "TYPE_BOOL"
    } highkey {
        damn "IDENTIFIER"
    }
}

slay tokenize(lexer Lexer) []Token {
    bestie lexer.position < lexer.source.length() {
        skip_whitespace(lexer)
        
        lowkey (lexer.position >= lexer.source.length()) {
            vibes
        }
        
        fr fr Check for comments
        lowkey (peek_char(lexer) == "f" && peek_next_chars(lexer, 5) == "fr fr") {
            skip_comment(lexer)
            yikes
        }
        
        sus ch tea = peek_char(lexer)
        sus line normie = lexer.line
        sus column normie = lexer.column
        
        lowkey (is_alpha(ch)) {
            fr fr Identifier or keyword
            sus identifier tea = read_identifier(lexer)
            sus token_type tea = get_keyword_type(identifier)
            sus token Token = create_token(token_type, identifier, line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (is_digit(ch)) {
            fr fr Number
            sus number tea = read_number(lexer)
            sus token Token = create_token("NUMBER", number, line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "\"" || ch == "'") {
            fr fr String literal
            sus string_val tea = read_string(lexer)
            sus token Token = create_token("STRING", string_val, line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "{") {
            next_char(lexer)
            sus token Token = create_token("LBRACE", "{", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "}") {
            next_char(lexer)
            sus token Token = create_token("RBRACE", "}", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "(") {
            next_char(lexer)
            sus token Token = create_token("LPAREN", "(", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == ")") {
            next_char(lexer)
            sus token Token = create_token("RPAREN", ")", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "[") {
            next_char(lexer)
            sus token Token = create_token("LBRACKET", "[", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "]") {
            next_char(lexer)
            sus token Token = create_token("RBRACKET", "]", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == ",") {
            next_char(lexer)
            sus token Token = create_token("COMMA", ",", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == ";") {
            next_char(lexer)
            sus token Token = create_token("SEMICOLON", ";", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == ".") {
            next_char(lexer)
            sus token Token = create_token("DOT", ".", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "=") {
            next_char(lexer)
            lowkey (peek_char(lexer) == "=") {
                next_char(lexer)
                sus token Token = create_token("EQ", "==", line, column)
                lexer.tokens.push(token)
            } highkey {
                sus token Token = create_token("ASSIGN", "=", line, column)
                lexer.tokens.push(token)
            }
            
        } highkey lowkey (ch == "!") {
            next_char(lexer)
            lowkey (peek_char(lexer) == "=") {
                next_char(lexer)
                sus token Token = create_token("NE", "!=", line, column)
                lexer.tokens.push(token)
            } highkey {
                sus token Token = create_token("NOT", "!", line, column)
                lexer.tokens.push(token)
            }
            
        } highkey lowkey (ch == "<") {
            next_char(lexer)
            lowkey (peek_char(lexer) == "=") {
                next_char(lexer)
                sus token Token = create_token("LE", "<=", line, column)
                lexer.tokens.push(token)
            } highkey {
                sus token Token = create_token("LT", "<", line, column)
                lexer.tokens.push(token)
            }
            
        } highkey lowkey (ch == ">") {
            next_char(lexer)
            lowkey (peek_char(lexer) == "=") {
                next_char(lexer)
                sus token Token = create_token("GE", ">=", line, column)
                lexer.tokens.push(token)
            } highkey {
                sus token Token = create_token("GT", ">", line, column)
                lexer.tokens.push(token)
            }
            
        } highkey lowkey (ch == "+") {
            next_char(lexer)
            sus token Token = create_token("PLUS", "+", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "-") {
            next_char(lexer)
            sus token Token = create_token("MINUS", "-", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "*") {
            next_char(lexer)
            sus token Token = create_token("MULTIPLY", "*", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == "/") {
            next_char(lexer)
            sus token Token = create_token("DIVIDE", "/", line, column)
            lexer.tokens.push(token)
            
        } highkey lowkey (ch == ":") {
            next_char(lexer)
            lowkey (peek_char(lexer) == "=") {
                next_char(lexer)
                sus token Token = create_token("DEFINE", ":=", line, column)
                lexer.tokens.push(token)
            } highkey {
                sus token Token = create_token("COLON", ":", line, column)
                lexer.tokens.push(token)
            }
            
        } highkey {
            fr fr Unknown character - skip it
            next_char(lexer)
        }
    }
    
    fr fr Add EOF token
    sus eof_token Token = create_token("EOF", "", lexer.line, lexer.column)
    lexer.tokens.push(eof_token)
    
    damn lexer.tokens
}

fr fr Test the lexer
slay test_lexer() {
    test_start("CURSED Lexer Test")
    
    sus source tea = "slay main() {\n  vibez.spill(\"Hello CURSED!\")\n}"
    sus lexer Lexer = init_lexer(source)
    sus tokens []Token = tokenize(lexer)
    
    vibez.spill("Tokenized " + tokens.length() + " tokens")
    
    fr fr Check that we got some tokens
    assert_true(tokens.length() > 5)
    
    fr fr Check first token is 'slay'
    assert_eq_string(tokens[0].type, "FUNCTION")
    assert_eq_string(tokens[0].value, "slay")
    
    print_test_summary()
}

test_lexer()
