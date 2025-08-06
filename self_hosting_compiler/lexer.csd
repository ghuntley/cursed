#!/usr/bin/env cursed
# CURSED Self-Hosting Compiler - Lexer Module
# Lexical analysis for the CURSED programming language

yeet "stringz"
yeet "arrayz"
yeet "testz"

# Token types for modern CURSED syntax
enum TokenType {
    # Core keywords
    SLAY,       # function declaration
    SUS,        # mutable variable
    FACTS,      # immutable variable  
    DAMN,       # return statement
    YEET,       # import/throw
    LOWKEY,     # if statement
    HIGHKEY,    # else statement
    PERIODT,    # while loop
    BESTIE,     # for loop
    GHOSTED,    # break
    SIMP,       # continue
    STAN,       # goroutine
    READY,      # defer
    
    # Data structures
    SQUAD,      # struct
    COLLAB,     # interface
    BE_LIKE,    # type alias
    
    # Control flow
    VIBE_CHECK, # switch
    MOOD,       # case
    BASIC,      # default
    
    # Types
    NORMIE,     # integer
    THICC,      # large integer
    SMOL,       # small integer
    MEAL,       # float
    TEA,        # string
    LIT,        # boolean
    DRIP,       # auto type
    
    # Boolean values
    BASED,      # true
    CRINGE,     # false
    
    # Operators
    PLUS,       # +
    MINUS,      # -
    MULTIPLY,   # *
    DIVIDE,     # /
    MODULO,     # %
    ASSIGN,     # =
    EQUAL,      # ==
    NOT_EQUAL,  # !=
    LESS,       # <
    LESS_EQ,    # <=
    GREATER,    # >
    GREATER_EQ, # >=
    AND,        # &&
    OR,         # ||
    NOT,        # !
    
    # Delimiters
    LPAREN,     # (
    RPAREN,     # )
    LBRACE,     # {
    RBRACE,     # }
    LBRACKET,   # [
    RBRACKET,   # ]
    COMMA,      # ,
    SEMICOLON,  # ;
    COLON,      # :
    DOT,        # .
    ARROW,      # ->
    
    # Literals
    IDENTIFIER,
    INTEGER,
    FLOAT,
    STRING,
    
    # Special
    NEWLINE,
    EOF,
    UNKNOWN,
}

# Token structure
squad Token {
    spill token_type TokenType
    spill literal tea
    spill line normie
    spill column normie
    spill position normie
}

# Lexer state
squad Lexer {
    spill source tea
    spill position normie      # current position
    spill read_position normie # next read position
    spill ch tea              # current character
    spill line normie         # current line
    spill column normie       # current column
}

# Initialize a new lexer
slay new_lexer(source tea) Lexer {
    sus lexer Lexer = Lexer{
        source: source,
        position: 0,
        read_position: 0,
        ch: "",
        line: 1,
        column: 0
    }
    
    # Read first character
    read_char(lexer)
    damn lexer
}

# Read next character and advance position
slay read_char(lexer Lexer) {
    lowkey (lexer.read_position >= stringz.length(lexer.source)) {
        lexer.ch = ""  # EOF
    } highkey {
        lexer.ch = stringz.char_at(lexer.source, lexer.read_position)
    }
    
    lexer.position = lexer.read_position
    lexer.read_position = lexer.read_position + 1
    
    lowkey (lexer.ch == "\n") {
        lexer.line = lexer.line + 1
        lexer.column = 0
    } highkey {
        lexer.column = lexer.column + 1
    }
}

# Peek at next character without advancing
slay peek_char(lexer Lexer) tea {
    lowkey (lexer.read_position >= stringz.length(lexer.source)) {
        damn ""  # EOF
    } highkey {
        damn stringz.char_at(lexer.source, lexer.read_position)
    }
}

# Skip whitespace characters
slay skip_whitespace(lexer Lexer) {
    bestie (is_whitespace(lexer.ch)) {
        read_char(lexer)
    }
}

# Check if character is whitespace
slay is_whitespace(ch tea) lit {
    damn ch == " " || ch == "\t" || ch == "\r"
}

# Check if character is a letter
slay is_letter(ch tea) lit {
    damn (ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || ch == "_"
}

# Check if character is a digit
slay is_digit(ch tea) lit {
    damn ch >= "0" && ch <= "9"
}

# Read identifier or keyword
slay read_identifier(lexer Lexer) tea {
    sus start_pos normie = lexer.position
    
    bestie (is_letter(lexer.ch) || is_digit(lexer.ch)) {
        read_char(lexer)
    }
    
    damn stringz.substring(lexer.source, start_pos, lexer.position)
}

# Read numeric literal
slay read_number(lexer Lexer) tea {
    sus start_pos normie = lexer.position
    sus has_dot lit = cringe
    
    bestie (is_digit(lexer.ch) || (lexer.ch == "." && !has_dot)) {
        lowkey (lexer.ch == ".") {
            has_dot = based
        }
        read_char(lexer)
    }
    
    damn stringz.substring(lexer.source, start_pos, lexer.position)
}

# Read string literal
slay read_string(lexer Lexer) tea {
    sus start_pos normie = lexer.position + 1  # Skip opening quote
    read_char(lexer)  # Move past opening quote
    
    bestie (lexer.ch != "\"" && lexer.ch != "") {
        lowkey (lexer.ch == "\\") {
            read_char(lexer)  # Skip escape character
            lowkey (lexer.ch != "") {
                read_char(lexer)  # Skip escaped character
            }
        } highkey {
            read_char(lexer)
        }
    }
    
    sus result tea = stringz.substring(lexer.source, start_pos, lexer.position)
    damn result
}

# Read single-line comment
slay read_comment(lexer Lexer) tea {
    sus start_pos normie = lexer.position
    
    bestie (lexer.ch != "\n" && lexer.ch != "") {
        read_char(lexer)
    }
    
    damn stringz.substring(lexer.source, start_pos, lexer.position)
}

# Lookup keyword or return IDENTIFIER
slay lookup_keyword(identifier tea) TokenType {
    # Create keyword mapping
    sus keywords map[tea]TokenType = {
        "slay": TokenType.SLAY,
        "sus": TokenType.SUS,
        "facts": TokenType.FACTS,
        "damn": TokenType.DAMN,
        "yeet": TokenType.YEET,
        "lowkey": TokenType.LOWKEY,
        "highkey": TokenType.HIGHKEY,
        "periodt": TokenType.PERIODT,
        "bestie": TokenType.BESTIE,
        "ghosted": TokenType.GHOSTED,
        "simp": TokenType.SIMP,
        "stan": TokenType.STAN,
        "ready": TokenType.READY,
        "squad": TokenType.SQUAD,
        "collab": TokenType.COLLAB,
        "be_like": TokenType.BE_LIKE,
        "vibe_check": TokenType.VIBE_CHECK,
        "mood": TokenType.MOOD,
        "basic": TokenType.BASIC,
        "normie": TokenType.NORMIE,
        "thicc": TokenType.THICC,
        "smol": TokenType.SMOL,
        "meal": TokenType.MEAL,
        "tea": TokenType.TEA,
        "lit": TokenType.LIT,
        "drip": TokenType.DRIP,
        "based": TokenType.BASED,
        "cringe": TokenType.CRINGE,
    }
    
    lowkey (keywords[identifier] != null) {
        damn keywords[identifier]
    }
    
    damn TokenType.IDENTIFIER
}

# Create new token
slay new_token(token_type TokenType, literal tea, line normie, column normie, position normie) Token {
    damn Token{
        token_type: token_type,
        literal: literal,
        line: line,
        column: column,
        position: position
    }
}

# Get next token from lexer
slay next_token(lexer Lexer) Token {
    skip_whitespace(lexer)
    
    sus current_line normie = lexer.line
    sus current_column normie = lexer.column
    sus current_position normie = lexer.position
    
    vibe_check (lexer.ch) {
        mood "=" {
            lowkey (peek_char(lexer) == "=") {
                read_char(lexer)
                read_char(lexer)
                damn new_token(TokenType.EQUAL, "==", current_line, current_column, current_position)
            } highkey {
                read_char(lexer)
                damn new_token(TokenType.ASSIGN, "=", current_line, current_column, current_position)
            }
        }
        
        mood "!" {
            lowkey (peek_char(lexer) == "=") {
                read_char(lexer)
                read_char(lexer)
                damn new_token(TokenType.NOT_EQUAL, "!=", current_line, current_column, current_position)
            } highkey {
                read_char(lexer)
                damn new_token(TokenType.NOT, "!", current_line, current_column, current_position)
            }
        }
        
        mood "<" {
            lowkey (peek_char(lexer) == "=") {
                read_char(lexer)
                read_char(lexer)
                damn new_token(TokenType.LESS_EQ, "<=", current_line, current_column, current_position)
            } highkey {
                read_char(lexer)
                damn new_token(TokenType.LESS, "<", current_line, current_column, current_position)
            }
        }
        
        mood ">" {
            lowkey (peek_char(lexer) == "=") {
                read_char(lexer)
                read_char(lexer)
                damn new_token(TokenType.GREATER_EQ, ">=", current_line, current_column, current_position)
            } highkey {
                read_char(lexer)
                damn new_token(TokenType.GREATER, ">", current_line, current_column, current_position)
            }
        }
        
        mood "&" {
            lowkey (peek_char(lexer) == "&") {
                read_char(lexer)
                read_char(lexer)
                damn new_token(TokenType.AND, "&&", current_line, current_column, current_position)
            } highkey {
                read_char(lexer)
                damn new_token(TokenType.UNKNOWN, "&", current_line, current_column, current_position)
            }
        }
        
        mood "|" {
            lowkey (peek_char(lexer) == "|") {
                read_char(lexer)
                read_char(lexer)
                damn new_token(TokenType.OR, "||", current_line, current_column, current_position)
            } highkey {
                read_char(lexer)
                damn new_token(TokenType.UNKNOWN, "|", current_line, current_column, current_position)
            }
        }
        
        mood "-" {
            lowkey (peek_char(lexer) == ">") {
                read_char(lexer)
                read_char(lexer)
                damn new_token(TokenType.ARROW, "->", current_line, current_column, current_position)
            } highkey {
                read_char(lexer)
                damn new_token(TokenType.MINUS, "-", current_line, current_column, current_position)
            }
        }
        
        mood "+" {
            read_char(lexer)
            damn new_token(TokenType.PLUS, "+", current_line, current_column, current_position)
        }
        
        mood "*" {
            read_char(lexer)
            damn new_token(TokenType.MULTIPLY, "*", current_line, current_column, current_position)
        }
        
        mood "/" {
            lowkey (peek_char(lexer) == "/") {
                sus comment tea = read_comment(lexer)
                damn new_token(TokenType.UNKNOWN, comment, current_line, current_column, current_position)
            } highkey {
                read_char(lexer)
                damn new_token(TokenType.DIVIDE, "/", current_line, current_column, current_position)
            }
        }
        
        mood "%" {
            read_char(lexer)
            damn new_token(TokenType.MODULO, "%", current_line, current_column, current_position)
        }
        
        mood "(" {
            read_char(lexer)
            damn new_token(TokenType.LPAREN, "(", current_line, current_column, current_position)
        }
        
        mood ")" {
            read_char(lexer)
            damn new_token(TokenType.RPAREN, ")", current_line, current_column, current_position)
        }
        
        mood "{" {
            read_char(lexer)
            damn new_token(TokenType.LBRACE, "{", current_line, current_column, current_position)
        }
        
        mood "}" {
            read_char(lexer)
            damn new_token(TokenType.RBRACE, "}", current_line, current_column, current_position)
        }
        
        mood "[" {
            read_char(lexer)
            damn new_token(TokenType.LBRACKET, "[", current_line, current_column, current_position)
        }
        
        mood "]" {
            read_char(lexer)
            damn new_token(TokenType.RBRACKET, "]", current_line, current_column, current_position)
        }
        
        mood "," {
            read_char(lexer)
            damn new_token(TokenType.COMMA, ",", current_line, current_column, current_position)
        }
        
        mood ";" {
            read_char(lexer)
            damn new_token(TokenType.SEMICOLON, ";", current_line, current_column, current_position)
        }
        
        mood ":" {
            read_char(lexer)
            damn new_token(TokenType.COLON, ":", current_line, current_column, current_position)
        }
        
        mood "." {
            read_char(lexer)
            damn new_token(TokenType.DOT, ".", current_line, current_column, current_position)
        }
        
        mood "\"" {
            sus string_literal tea = read_string(lexer)
            read_char(lexer)  # Skip closing quote
            damn new_token(TokenType.STRING, string_literal, current_line, current_column, current_position)
        }
        
        mood "\n" {
            read_char(lexer)
            damn new_token(TokenType.NEWLINE, "\n", current_line, current_column, current_position)
        }
        
        mood "" {
            damn new_token(TokenType.EOF, "", current_line, current_column, current_position)
        }
        
        basic {
            lowkey (is_letter(lexer.ch)) {
                sus identifier tea = read_identifier(lexer)
                sus token_type TokenType = lookup_keyword(identifier)
                damn new_token(token_type, identifier, current_line, current_column, current_position)
            } highkey lowkey (is_digit(lexer.ch)) {
                sus number tea = read_number(lexer)
                sus token_type TokenType = TokenType.INTEGER
                lowkey (stringz.contains(number, ".")) {
                    token_type = TokenType.FLOAT
                }
                damn new_token(token_type, number, current_line, current_column, current_position)
            } highkey {
                sus ch tea = lexer.ch
                read_char(lexer)
                damn new_token(TokenType.UNKNOWN, ch, current_line, current_column, current_position)
            }
        }
    }
}

# Tokenize entire source code
slay tokenize(source tea) []Token {
    sus lexer Lexer = new_lexer(source)
    sus tokens []Token = []
    
    periodt (based) {
        sus token Token = next_token(lexer)
        
        # Skip comments and unnecessary tokens for parsing
        lowkey (token.token_type != TokenType.UNKNOWN) {
            arrayz.array_push(tokens, token)
        }
        
        lowkey (token.token_type == TokenType.EOF) {
            ghosted
        }
    }
    
    damn tokens
}

# Print token for debugging
slay print_token(token Token) {
    vibez.spill("Token{type: " + token.token_type + ", literal: \"" + token.literal + "\", line: " + token.line + ", column: " + token.column + "}")
}
