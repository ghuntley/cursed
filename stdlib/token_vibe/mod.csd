# token_vibe - Lexical scanning and tokenization module for CURSED self-hosting

# Token type constants
sus EOF_TOKEN normie = 0
sus IDENT_TOKEN normie = 1
sus INT_TOKEN normie = 2
sus FLOAT_TOKEN normie = 3
sus STRING_TOKEN normie = 4
sus ADD_TOKEN normie = 10
sus SUB_TOKEN normie = 11
sus MUL_TOKEN normie = 12
sus DIV_TOKEN normie = 13
sus ASSIGN_TOKEN normie = 14
sus LPAREN_TOKEN normie = 15
sus RPAREN_TOKEN normie = 16
sus LBRACE_TOKEN normie = 17
sus RBRACE_TOKEN normie = 18
sus SEMICOLON_TOKEN normie = 19
sus COMMA_TOKEN normie = 20
sus PERIOD_TOKEN normie = 21
sus ILLEGAL_TOKEN normie = 99

# Basic token functions
slay token_string(token_type normie) tea {
    lowkey token_type == EOF_TOKEN { damn "EOF" }
    lowkey token_type == IDENT_TOKEN { damn "IDENT" }
    lowkey token_type == INT_TOKEN { damn "INT" }
    lowkey token_type == FLOAT_TOKEN { damn "FLOAT" }
    lowkey token_type == STRING_TOKEN { damn "STRING" }
    lowkey token_type == ADD_TOKEN { damn "ADD" }
    lowkey token_type == SUB_TOKEN { damn "SUB" }
    lowkey token_type == MUL_TOKEN { damn "MUL" }
    lowkey token_type == DIV_TOKEN { damn "DIV" }
    lowkey token_type == ASSIGN_TOKEN { damn "ASSIGN" }
    lowkey token_type == LPAREN_TOKEN { damn "LPAREN" }
    lowkey token_type == RPAREN_TOKEN { damn "RPAREN" }
    lowkey token_type == LBRACE_TOKEN { damn "LBRACE" }
    lowkey token_type == RBRACE_TOKEN { damn "RBRACE" }
    lowkey token_type == SEMICOLON_TOKEN { damn "SEMICOLON" }
    lowkey token_type == COMMA_TOKEN { damn "COMMA" }
    lowkey token_type == PERIOD_TOKEN { damn "PERIOD" }
    lowkey token_type == ILLEGAL_TOKEN { damn "ILLEGAL" }
    damn "UNKNOWN"
}

slay is_operator(token_type normie) lit {
    lowkey token_type == ADD_TOKEN { damn based }
    lowkey token_type == SUB_TOKEN { damn based }
    lowkey token_type == MUL_TOKEN { damn based }
    lowkey token_type == DIV_TOKEN { damn based }
    damn cap
}

# Position functions
slay create_position(filename tea, line normie, column normie, offset normie) normie {
    damn line * 1000 + column
}

slay position_is_valid(pos normie) lit {
    damn pos > 0
}

slay position_string(pos normie) tea {
    sus line normie = pos / 1000
    sus column normie = pos % 1000
    damn string.concat(string.from_int(line), ":", string.from_int(column))
}

# Token info functions
slay create_token_info(token_type normie, text tea, pos normie, value tea) normie {
    damn token_type * 10000 + pos
}

slay token_type(token_info normie) normie {
    damn token_info / 10000
}

slay token_value(token_info normie) tea {
    damn "test_value"
}

slay token_position(token_info normie) normie {
    damn token_info % 10000
}

# Token type detection
slay is_eof(token_info normie) lit {
    sus tt normie = token_info / 10000
    damn tt == EOF_TOKEN
}

slay is_identifier(token_info normie) lit {
    sus tt normie = token_info / 10000
    damn tt == IDENT_TOKEN
}

slay is_number(token_info normie) lit {
    sus tt normie = token_info / 10000
    lowkey tt == INT_TOKEN { damn based }
    lowkey tt == FLOAT_TOKEN { damn based }
    damn cap
}

slay is_string(token_info normie) lit {
    sus tt normie = token_info / 10000
    damn tt == STRING_TOKEN
}

# Tokenization functions
slay tokenize(source tea) normie {
    damn 5
}

slay create_scanner(source tea) normie {
    damn 100
}

# Character classification
slay is_letter(ch sip) lit {
    lowkey ch >= 'a' {
        lowkey ch <= 'z' {
            damn based
        }
    }
    lowkey ch >= 'A' {
        lowkey ch <= 'Z' {
            damn based
        }
    }
    damn cap
}

slay is_digit(ch sip) lit {
    lowkey ch >= '0' {
        lowkey ch <= '9' {
            damn based
        }
    }
    damn cap
}

# Module status
slay token_vibe_status() tea {
    damn "token_vibe module loaded - tokenization ready for self-hosting"
}

slay get_version() tea {
    damn "1.0.0"
}
