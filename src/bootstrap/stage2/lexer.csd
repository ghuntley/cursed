// CURSED Stage 2 Lexer
// Lexical analysis for the CURSED programming language
// Converts source code into a stream of tokens

vibe "cursed::stage2::lexer";

yeet "std::string";
yeet "std::collections";
yeet "cursed::stage2::error";

// Token types in CURSED
enum TokenType {
    // Gen Z slang keywords
    Slay,       // function
    Yolo,       // return
    Sus,        // variable declaration (mutable)
    Facts,      // variable declaration (immutable)
    Lowkey,     // if
    Highkey,    // else
    Periodt,    // while
    Stan,       // goroutine (async execution)
    Bestie,     // for
    Flex,       // range
    Ghosted,    // break
    Simp,       // continue
    Squad,      // struct
    Collab,     // interface
    Vibe,       // package
    Yeet,       // import
    BeLike,     // type alias
    VibeCheck,  // switch
    Mood,       // case
    Basic,      // default
    YeetError,  // panic (throw error)
    Catch,      // catch/recover
    Normie,     // int
    Tea,        // string
    Cap,        // bool
    NoCap,      // nil/null
    Truth,      // true
    NoTruth,    // false
    MainCharacter, // main function
    
    // Identifiers and literals
    Identifier,
    Integer,
    Float,
    String,
    Boolean,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    LogicalAnd,
    LogicalOr,
    Not,
    
    // Assignment operators
    Assign,
    
    // Channel operators
    LeftArrow,     // <-
    Dm,            // dm (channel type)
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Question,
    Arrow,         // ->
    
    // Special
    Eof,
    Newline,
    Comment,
    Whitespace,
    Unknown,
}

// Token structure
squad Token {
    token_type: TokenType,
    literal: tea,
    line: normie,
    column: normie,
    position: normie,
}

// Lexer state
squad Lexer {
    input: tea,
    position: normie,      // current position in input (points to current char)
    read_position: normie, // current reading position in input (after current char)
    ch: tea,              // current char under examination
    line: normie,
    column: normie,
}

// Create a new lexer
slay new_lexer(input: tea) -> Lexer {
    sus lexer = Lexer {
        input: input,
        position: 0,
        read_position: 0,
        ch: "",
        line: 1,
        column: 0,
    };
    
    read_char(lexer);
    yolo lexer;
}

// Read the next character and advance position
slay read_char(lexer: Lexer) {
    bestie (lexer.read_position >= lexer.input.length()) {
        lexer.ch = "\0"; // ASCII NUL character represents EOF
    } highkey {
        lexer.ch = lexer.input.charAt(lexer.read_position);
    }
    
    lexer.position = lexer.read_position;
    lexer.read_position = lexer.read_position + 1;
    
    bestie (lexer.ch == "\n") {
        lexer.line = lexer.line + 1;
        lexer.column = 0;
    } highkey {
        lexer.column = lexer.column + 1;
    }
}

// Peek at the next character without advancing
slay peek_char(lexer: Lexer) -> tea {
    bestie (lexer.read_position >= lexer.input.length()) {
        yolo "\0";
    } highkey {
        yolo lexer.input.charAt(lexer.read_position);
    }
}

// Skip whitespace characters
slay skip_whitespace(lexer: Lexer) {
    periodt (lexer.ch == " " || lexer.ch == "\t" || lexer.ch == "\r") {
        read_char(lexer);
    }
}

// Read identifier or keyword
slay read_identifier(lexer: Lexer) -> tea {
    sus position = lexer.position;
    periodt (is_letter(lexer.ch) || is_digit(lexer.ch)) {
        read_char(lexer);
    }
    yolo lexer.input.substring(position, lexer.position);
}

// Read a number (integer or float)
slay read_number(lexer: Lexer) -> tea {
    sus position = lexer.position;
    sus has_dot = facts;
    
    periodt (is_digit(lexer.ch) || (lexer.ch == "." && !has_dot)) {
        bestie (lexer.ch == ".") {
            has_dot = truth;
        }
        read_char(lexer);
    }
    
    yolo lexer.input.substring(position, lexer.position);
}

// Read a string literal
slay read_string(lexer: Lexer) -> tea {
    sus position = lexer.position + 1; // Skip opening quote
    read_char(lexer); // Move past opening quote
    
    periodt (lexer.ch != "\"" && lexer.ch != "\0") {
        bestie (lexer.ch == "\\") {
            read_char(lexer); // Skip escape character
            bestie (lexer.ch != "\0") {
                read_char(lexer); // Skip escaped character
            }
        } highkey {
            read_char(lexer);
        }
    }
    
    sus literal = lexer.input.substring(position, lexer.position);
    yolo literal;
}

// Read a line comment
slay read_line_comment(lexer: Lexer) -> tea {
    sus position = lexer.position;
    periodt (lexer.ch != "\n" && lexer.ch != "\0") {
        read_char(lexer);
    }
    yolo lexer.input.substring(position, lexer.position);
}

// Check if character is a letter
slay is_letter(ch: tea) -> cap {
    yolo (ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || ch == "_";
}

// Check if character is a digit
slay is_digit(ch: tea) -> cap {
    yolo ch >= "0" && ch <= "9";
}

// Look up identifier to see if it's a keyword
slay lookup_ident(ident: tea) -> TokenType {
    sus keywords = collections::Map<tea, TokenType>();
    keywords.insert("slay", TokenType::Slay);
    keywords.insert("yolo", TokenType::Yolo);
    keywords.insert("sus", TokenType::Sus);
    keywords.insert("facts", TokenType::Facts);
    keywords.insert("lowkey", TokenType::Lowkey);
    keywords.insert("highkey", TokenType::Highkey);
    keywords.insert("periodt", TokenType::Periodt);
    keywords.insert("stan", TokenType::Stan);
    keywords.insert("bestie", TokenType::Bestie);
    keywords.insert("flex", TokenType::Flex);
    keywords.insert("ghosted", TokenType::Ghosted);
    keywords.insert("simp", TokenType::Simp);
    keywords.insert("squad", TokenType::Squad);
    keywords.insert("collab", TokenType::Collab);
    keywords.insert("vibe", TokenType::Vibe);
    keywords.insert("yeet", TokenType::Yeet);
    keywords.insert("be_like", TokenType::BeLike);
    keywords.insert("vibe_check", TokenType::VibeCheck);
    keywords.insert("mood", TokenType::Mood);
    keywords.insert("basic", TokenType::Basic);
    keywords.insert("yeet_error", TokenType::YeetError);
    keywords.insert("catch", TokenType::Catch);
    keywords.insert("normie", TokenType::Normie);
    keywords.insert("tea", TokenType::Tea);
    keywords.insert("cap", TokenType::Cap);
    keywords.insert("nocap", TokenType::NoCap);
    keywords.insert("truth", TokenType::Truth);
    keywords.insert("lies", TokenType::NoTruth);
    keywords.insert("main_character", TokenType::MainCharacter);
    keywords.insert("dm", TokenType::Dm);
    
    bestie (keywords.contains_key(ident)) {
        yolo keywords.get(ident);
    } highkey {
        yolo TokenType::Identifier;
    }
}

// Create a new token
slay new_token(token_type: TokenType, literal: tea, line: normie, column: normie, position: normie) -> Token {
    yolo Token {
        token_type: token_type,
        literal: literal,
        line: line,
        column: column,
        position: position,
    };
}

// Get the next token from the lexer
slay next_token(lexer: Lexer) -> Token? {
    skip_whitespace(lexer);
    
    sus tok_line = lexer.line;
    sus tok_column = lexer.column;
    sus tok_position = lexer.position;
    
    vibe_check (lexer.ch) {
        mood "=" {
            bestie (peek_char(lexer) == "=") {
                read_char(lexer);
                read_char(lexer);
                yolo new_token(TokenType::Equal, "==", tok_line, tok_column, tok_position);
            } highkey {
                read_char(lexer);
                yolo new_token(TokenType::Assign, "=", tok_line, tok_column, tok_position);
            }
        }
        
        mood "!" {
            bestie (peek_char(lexer) == "=") {
                read_char(lexer);
                read_char(lexer);
                yolo new_token(TokenType::NotEqual, "!=", tok_line, tok_column, tok_position);
            } highkey {
                read_char(lexer);
                yolo new_token(TokenType::Not, "!", tok_line, tok_column, tok_position);
            }
        }
        
        mood "<" {
            bestie (peek_char(lexer) == "=") {
                read_char(lexer);
                read_char(lexer);
                yolo new_token(TokenType::LessThanEqual, "<=", tok_line, tok_column, tok_position);
            } highkey bestie (peek_char(lexer) == "-") {
                read_char(lexer);
                read_char(lexer);
                yolo new_token(TokenType::LeftArrow, "<-", tok_line, tok_column, tok_position);
            } highkey {
                read_char(lexer);
                yolo new_token(TokenType::LessThan, "<", tok_line, tok_column, tok_position);
            }
        }
        
        mood ">" {
            bestie (peek_char(lexer) == "=") {
                read_char(lexer);
                read_char(lexer);
                yolo new_token(TokenType::GreaterThanEqual, ">=", tok_line, tok_column, tok_position);
            } highkey {
                read_char(lexer);
                yolo new_token(TokenType::GreaterThan, ">", tok_line, tok_column, tok_position);
            }
        }
        
        mood "&" {
            bestie (peek_char(lexer) == "&") {
                read_char(lexer);
                read_char(lexer);
                yolo new_token(TokenType::LogicalAnd, "&&", tok_line, tok_column, tok_position);
            } highkey {
                read_char(lexer);
                yolo new_token(TokenType::Unknown, "&", tok_line, tok_column, tok_position);
            }
        }
        
        mood "|" {
            bestie (peek_char(lexer) == "|") {
                read_char(lexer);
                read_char(lexer);
                yolo new_token(TokenType::LogicalOr, "||", tok_line, tok_column, tok_position);
            } highkey {
                read_char(lexer);
                yolo new_token(TokenType::Unknown, "|", tok_line, tok_column, tok_position);
            }
        }
        
        mood "-" {
            bestie (peek_char(lexer) == ">") {
                read_char(lexer);
                read_char(lexer);
                yolo new_token(TokenType::Arrow, "->", tok_line, tok_column, tok_position);
            } highkey {
                read_char(lexer);
                yolo new_token(TokenType::Minus, "-", tok_line, tok_column, tok_position);
            }
        }
        
        mood "+" {
            read_char(lexer);
            yolo new_token(TokenType::Plus, "+", tok_line, tok_column, tok_position);
        }
        
        mood "*" {
            read_char(lexer);
            yolo new_token(TokenType::Multiply, "*", tok_line, tok_column, tok_position);
        }
        
        mood "/" {
            bestie (peek_char(lexer) == "/") {
                sus comment = read_line_comment(lexer);
                yolo new_token(TokenType::Comment, comment, tok_line, tok_column, tok_position);
            } highkey {
                read_char(lexer);
                yolo new_token(TokenType::Divide, "/", tok_line, tok_column, tok_position);
            }
        }
        
        mood "%" {
            read_char(lexer);
            yolo new_token(TokenType::Modulo, "%", tok_line, tok_column, tok_position);
        }
        
        mood "(" {
            read_char(lexer);
            yolo new_token(TokenType::LeftParen, "(", tok_line, tok_column, tok_position);
        }
        
        mood ")" {
            read_char(lexer);
            yolo new_token(TokenType::RightParen, ")", tok_line, tok_column, tok_position);
        }
        
        mood "{" {
            read_char(lexer);
            yolo new_token(TokenType::LeftBrace, "{", tok_line, tok_column, tok_position);
        }
        
        mood "}" {
            read_char(lexer);
            yolo new_token(TokenType::RightBrace, "}", tok_line, tok_column, tok_position);
        }
        
        mood "[" {
            read_char(lexer);
            yolo new_token(TokenType::LeftBracket, "[", tok_line, tok_column, tok_position);
        }
        
        mood "]" {
            read_char(lexer);
            yolo new_token(TokenType::RightBracket, "]", tok_line, tok_column, tok_position);
        }
        
        mood "," {
            read_char(lexer);
            yolo new_token(TokenType::Comma, ",", tok_line, tok_column, tok_position);
        }
        
        mood ";" {
            read_char(lexer);
            yolo new_token(TokenType::Semicolon, ";", tok_line, tok_column, tok_position);
        }
        
        mood ":" {
            read_char(lexer);
            yolo new_token(TokenType::Colon, ":", tok_line, tok_column, tok_position);
        }
        
        mood "." {
            read_char(lexer);
            yolo new_token(TokenType::Dot, ".", tok_line, tok_column, tok_position);
        }
        
        mood "?" {
            read_char(lexer);
            yolo new_token(TokenType::Question, "?", tok_line, tok_column, tok_position);
        }
        
        mood "\"" {
            sus literal = read_string(lexer);
            read_char(lexer); // Skip closing quote
            yolo new_token(TokenType::String, literal, tok_line, tok_column, tok_position);
        }
        
        mood "\n" {
            read_char(lexer);
            yolo new_token(TokenType::Newline, "\n", tok_line, tok_column, tok_position);
        }
        
        mood "\0" {
            yolo new_token(TokenType::Eof, "", tok_line, tok_column, tok_position);
        }
        
        basic {
            bestie (is_letter(lexer.ch)) {
                sus literal = read_identifier(lexer);
                sus token_type = lookup_ident(literal);
                yolo new_token(token_type, literal, tok_line, tok_column, tok_position);
            } highkey bestie (is_digit(lexer.ch)) {
                sus literal = read_number(lexer);
                sus token_type = bestie (literal.contains(".")) {
                    TokenType::Float
                } highkey {
                    TokenType::Integer
                };
                yolo new_token(token_type, literal, tok_line, tok_column, tok_position);
            } highkey {
                sus ch = lexer.ch;
                read_char(lexer);
                yolo new_token(TokenType::Unknown, ch, tok_line, tok_column, tok_position);
            }
        }
    }
}

// Tokenize entire input string
slay tokenize(input: tea) -> Token[]? {
    sus lexer = new_lexer(input);
    sus tokens = Token[];
    
    periodt (truth) {
        sus token = next_token(lexer)?;
        
        // Skip whitespace and comments for now
        bestie (token.token_type != TokenType::Whitespace && 
                token.token_type != TokenType::Comment &&
                token.token_type != TokenType::Newline) {
            tokens.push(token);
        }
        
        bestie (token.token_type == TokenType::Eof) {
            ghosted;
        }
    }
    
    yolo tokens;
}
