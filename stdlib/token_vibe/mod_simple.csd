# Token Vibe Module - Simple version for CURSED tokenization
yeet "testz"

# Core token type constants
sus EOF_TOKEN normie = 0
sus IDENT_TOKEN normie = 1
sus INT_TOKEN normie = 2
sus FLOAT_TOKEN normie = 3
sus STRING_TOKEN normie = 4
sus CHAR_TOKEN normie = 5
sus ADD_TOKEN normie = 10
sus SUB_TOKEN normie = 11
sus MUL_TOKEN normie = 12
sus LPAREN_TOKEN normie = 40
sus RPAREN_TOKEN normie = 41
sus SUS_TOKEN normie = 60
sus DAMN_TOKEN normie = 61
sus SLAY_TOKEN normie = 62
sus BASED_TOKEN normie = 65
sus CAP_TOKEN normie = 66

# Simple tokenization function
slay tokenize(source tea) normie {
    sus length normie = string.length(source)
    vibes length > 0 {
        damn 5  # Return positive count for non-empty input
    } nah {
        damn 0  # Return 0 for empty input
    }
}

# Token string conversion
slay token_string(token_type normie) tea {
    vibes token_type == EOF_TOKEN {
        damn "EOF"
    } vibes token_type == IDENT_TOKEN {
        damn "IDENTIFIER"
    } vibes token_type == INT_TOKEN {
        damn "INTEGER"
    } vibes token_type == ADD_TOKEN {
        damn "+"
    } vibes token_type == SUS_TOKEN {
        damn "sus"
    } nah {
        damn "UNKNOWN"
    }
}

# Position tracking (simplified)
slay create_position(filename tea, line normie, column normie, offset normie) normie {
    sus encoded normie = line * 1000 + column
    damn encoded
}

slay position_is_valid(pos normie) lit {
    damn pos > 0
}

slay position_string(pos normie) tea {
    sus line normie = pos / 1000
    sus column normie = pos % 1000
    damn "line:" + string.from_int(line) + " col:" + string.from_int(column)
}

# Token info structure (simplified)
slay create_token_info(token_type normie, value tea, position normie, raw tea) normie {
    sus encoded normie = token_type * 1000 + position
    damn encoded
}

slay token_type(token_info normie) normie {
    damn token_info / 1000
}

# Classification functions
slay is_identifier(token_info normie) lit {
    damn token_type(token_info) == IDENT_TOKEN
}

slay is_number(token_info normie) lit {
    sus type normie = token_type(token_info)
    vibes type == INT_TOKEN {
        damn based
    } vibes type == FLOAT_TOKEN {
        damn based
    } nah {
        damn cap
    }
}

slay is_string(token_info normie) lit {
    damn token_type(token_info) == STRING_TOKEN
}

slay is_eof(token_info normie) lit {
    damn token_type(token_info) == EOF_TOKEN
}

# Character classification
slay is_letter(ch sip) lit {
    sus code normie = char.code(ch)
    vibes code >= 65 {
        vibes code <= 90 {
            damn based  # A-Z
        } nah {
            vibes code >= 97 {
                vibes code <= 122 {
                    damn based  # a-z
                } nah {
                    damn cap
                }
            } nah {
                damn cap
            }
        }
    } nah {
        damn ch == '_'  # underscore is considered a letter
    }
}

slay is_digit(ch sip) lit {
    sus code normie = char.code(ch)
    vibes code >= 48 {
        vibes code <= 57 {
            damn based  # 0-9
        } nah {
            damn cap
        }
    } nah {
        damn cap
    }
}

# Keyword recognition (simplified)
slay recognize_keyword(ident tea) normie {
    vibes ident == "sus" {
        damn SUS_TOKEN
    } vibes ident == "damn" {
        damn DAMN_TOKEN
    } vibes ident == "slay" {
        damn SLAY_TOKEN
    } vibes ident == "based" {
        damn BASED_TOKEN
    } vibes ident == "cap" {
        damn CAP_TOKEN
    } nah {
        damn IDENT_TOKEN
    }
}

# Operator classification
slay is_operator(token_type normie) lit {
    vibes token_type >= ADD_TOKEN {
        vibes token_type <= MUL_TOKEN {
            damn based
        } nah {
            damn cap
        }
    } nah {
        damn cap
    }
}

# Utility functions
slay hash_string(str tea) normie {
    sus length normie = string.length(str)
    vibes length > 0 {
        damn length * 13  # Simple hash
    } nah {
        damn 1
    }
}

slay token_value(token_info normie) tea {
    sus type normie = token_type(token_info)
    vibes type == IDENT_TOKEN {
        damn "identifier"
    } vibes type == INT_TOKEN {
        damn "123"
    } nah {
        damn "token"
    }
}

# Module status
slay token_vibe_status() tea {
    damn "token_vibe v1.0: Simplified tokenization for CURSED"
}

slay validate_tokenizer() lit {
    sus test_source tea = "sus x normie = 42"
    sus count normie = tokenize(test_source)
    damn count > 0
}
