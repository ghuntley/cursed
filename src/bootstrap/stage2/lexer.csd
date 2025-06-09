// Stage 2 Lexer - Written in CURSED minimal subset
// Tokenizes CURSED source code into tokens for parsing

import "std/string"
import "std/char"

// Token types for the minimal subset
enum TokenType {
    ILLEGAL,
    EOF,
    
    // Identifiers and literals
    IDENT,
    INT,
    STRING,
    
    // Operators
    ASSIGN,    // =
    PLUS,      // +
    MINUS,     // -
    MULTIPLY,  // *
    DIVIDE,    // /
    LT,        // <
    GT,        // >
    EQ,        // ==
    NOT_EQ,    // !=
    BANG,      // !
    
    // Delimiters
    COMMA,     // ,
    SEMICOLON, // ;
    LPAREN,    // (
    RPAREN,    // )
    LBRACE,    // {
    RBRACE,    // }
    LBRACKET,  // [
    RBRACKET,  // ]
    
    // Keywords
    FUNC,
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
    STRUCT,
    IMPORT,
    FOR,
    WHILE,
}

// Token represents a lexical token
struct Token {
    type: TokenType
    literal: string
    line: int
    column: int
}

// Lexer tokenizes CURSED source code
struct Lexer {
    input: string
    position: int      // current position in input (points to current char)
    read_position: int // current reading position in input (after current char)
    ch: byte          // current char under examination
    line: int         // current line number
    column: int       // current column number
    errors: []string  // lexer errors
}

// Create a new lexer
func new_lexer() Lexer {
    return Lexer{
        input: "",
        position: 0,
        read_position: 0,
        ch: 0,
        line: 1,
        column: 0,
        errors: []string{},
    }
}

// Tokenize source code into tokens
func (l *Lexer) tokenize(input: string) []Token {
    l.input = input
    l.position = 0
    l.read_position = 0
    l.line = 1
    l.column = 0
    l.errors = []string{}
    
    l.read_char()
    
    tokens := []Token{}
    
    for {
        tok := l.next_token()
        tokens = append(tokens, tok)
        
        if tok.type == TokenType.EOF {
            break
        }
    }
    
    return tokens
}

// Read the next character and advance position
func (l *Lexer) read_char() {
    if l.read_position >= len(l.input) {
        l.ch = 0 // ASCII NUL represents "EOF"
    } else {
        l.ch = l.input[l.read_position]
    }
    
    l.position = l.read_position
    l.read_position = l.read_position + 1
    
    if l.ch == '\n' {
        l.line = l.line + 1
        l.column = 0
    } else {
        l.column = l.column + 1
    }
}

// Peek at the next character without advancing
func (l *Lexer) peek_char() byte {
    if l.read_position >= len(l.input) {
        return 0
    }
    return l.input[l.read_position]
}

// Get the next token
func (l *Lexer) next_token() Token {
    l.skip_whitespace()
    
    tok := Token{
        line: l.line,
        column: l.column,
    }
    
    // Handle single character tokens
    if l.ch == '=' {
        if l.peek_char() == '=' {
            l.read_char()
            tok.type = TokenType.EQ
            tok.literal = "=="
        } else {
            tok.type = TokenType.ASSIGN
            tok.literal = "="
        }
    } else if l.ch == '+' {
        tok.type = TokenType.PLUS
        tok.literal = "+"
    } else if l.ch == '-' {
        tok.type = TokenType.MINUS
        tok.literal = "-"
    } else if l.ch == '*' {
        tok.type = TokenType.MULTIPLY
        tok.literal = "*"
    } else if l.ch == '/' {
        tok.type = TokenType.DIVIDE
        tok.literal = "/"
    } else if l.ch == '<' {
        tok.type = TokenType.LT
        tok.literal = "<"
    } else if l.ch == '>' {
        tok.type = TokenType.GT
        tok.literal = ">"
    } else if l.ch == '!' {
        if l.peek_char() == '=' {
            l.read_char()
            tok.type = TokenType.NOT_EQ
            tok.literal = "!="
        } else {
            tok.type = TokenType.BANG
            tok.literal = "!"
        }
    } else if l.ch == ',' {
        tok.type = TokenType.COMMA
        tok.literal = ","
    } else if l.ch == ';' {
        tok.type = TokenType.SEMICOLON
        tok.literal = ";"
    } else if l.ch == '(' {
        tok.type = TokenType.LPAREN
        tok.literal = "("
    } else if l.ch == ')' {
        tok.type = TokenType.RPAREN
        tok.literal = ")"
    } else if l.ch == '{' {
        tok.type = TokenType.LBRACE
        tok.literal = "{"
    } else if l.ch == '}' {
        tok.type = TokenType.RBRACE
        tok.literal = "}"
    } else if l.ch == '[' {
        tok.type = TokenType.LBRACKET
        tok.literal = "["
    } else if l.ch == ']' {
        tok.type = TokenType.RBRACKET
        tok.literal = "]"
    } else if l.ch == '"' {
        tok.literal = l.read_string()
        tok.type = TokenType.STRING
    } else if l.ch == 0 {
        tok.type = TokenType.EOF
        tok.literal = ""
    } else if char.is_letter(l.ch) {
        tok.literal = l.read_identifier()
        tok.type = l.lookup_identifier_type(tok.literal)
        return tok // early return to avoid read_char
    } else if char.is_digit(l.ch) {
        tok.literal = l.read_number()
        tok.type = TokenType.INT
        return tok // early return to avoid read_char
    } else {
        tok.type = TokenType.ILLEGAL
        tok.literal = string(l.ch)
        l.add_error("Illegal character: " + tok.literal)
    }
    
    l.read_char()
    return tok
}

// Skip whitespace characters
func (l *Lexer) skip_whitespace() {
    for l.ch == ' ' || l.ch == '\t' || l.ch == '\n' || l.ch == '\r' {
        l.read_char()
    }
}

// Read identifier
func (l *Lexer) read_identifier() string {
    position := l.position
    for char.is_letter(l.ch) || char.is_digit(l.ch) || l.ch == '_' {
        l.read_char()
    }
    return l.input[position:l.position]
}

// Read number
func (l *Lexer) read_number() string {
    position := l.position
    for char.is_digit(l.ch) {
        l.read_char()
    }
    return l.input[position:l.position]
}

// Read string literal
func (l *Lexer) read_string() string {
    position := l.position + 1
    for {
        l.read_char()
        if l.ch == '"' || l.ch == 0 {
            break
        }
    }
    return l.input[position:l.position]
}

// Lookup identifier type (keyword vs identifier)
func (l *Lexer) lookup_identifier_type(ident: string) TokenType {
    keywords := map[string]TokenType{
        "func":   TokenType.FUNC,
        "let":    TokenType.LET,
        "if":     TokenType.IF,
        "else":   TokenType.ELSE,
        "return": TokenType.RETURN,
        "true":   TokenType.TRUE,
        "false":  TokenType.FALSE,
        "struct": TokenType.STRUCT,
        "import": TokenType.IMPORT,
        "for":    TokenType.FOR,
        "while":  TokenType.WHILE,
    }
    
    if keyword_type, exists := keywords[ident]; exists {
        return keyword_type
    }
    
    return TokenType.IDENT
}

// Add error to error list
func (l *Lexer) add_error(msg: string) {
    error_msg := "Line " + string(l.line) + ", Column " + string(l.column) + ": " + msg
    l.errors = append(l.errors, error_msg)
}

// Check if lexer has errors
func (l *Lexer) has_errors() bool {
    return len(l.errors) > 0
}

// Get lexer errors
func (l *Lexer) get_errors() []string {
    return l.errors
}
