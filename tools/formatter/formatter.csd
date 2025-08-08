// CURSED Code Formatter - Pure CURSED Implementation
// Self-hosting development tool for consistent code formatting

yeet "stringz"
yeet "arrayz" 
yeet "testz"

// Formatting configuration
squad FormatterConfig {
    spill indent_size drip
    spill max_line_length drip
    spill use_spaces lit
    spill space_around_operators lit
    spill align_struct_fields lit
    spill newline_before_brace lit
    
    // CURSED-specific formatting options
    spill align_gen_z_keywords lit
    spill prefer_short_form_syntax lit
}

// Create default formatter configuration
slay default_formatter_config() FormatterConfig {
    damn FormatterConfig{
        indent_size: 4,
        max_line_length: 100,
        use_spaces: based,
        space_around_operators: based,
        align_struct_fields: based,
        newline_before_brace: cringe,
        align_gen_z_keywords: based,
        prefer_short_form_syntax: based
    }
}

// Formatting context for tracking state
squad FormattingContext {
    spill config FormatterConfig
    spill current_indent drip
    spill line_length drip
    spill in_function_params lit
    spill in_struct_definition lit
    spill in_interface_definition lit
}

// Token types for parsing
squad Token {
    spill type tea
    spill value tea
    spill line drip
    spill column drip
}

// Simple tokenizer for CURSED code
slay tokenize_cursed(source tea) []Token {
    sus tokens []Token = []
    sus i drip = 0
    sus line drip = 1
    sus column drip = 1
    
    bestie (i < len_str(source)) {
        sus char tea = char_at(source, i)
        
        // Handle whitespace
        ready (char == " " || char == "\t" || char == "\n") {
            ready (char == "\n") {
                line = line + 1
                column = 1
            } otherwise {
                column = column + 1
            }
            i = i + 1
            continue
        }
        
        // Handle keywords and identifiers
        ready (is_alpha(char)) {
            sus start drip = i
            sus start_column drip = column
            bestie (i < len_str(source) && (is_alpha(char_at(source, i)) || is_digit(char_at(source, i)) || char_at(source, i) == "_")) {
                i = i + 1
                column = column + 1
            }
            sus word tea = substring(source, start, i)
            sus token_type tea = get_token_type(word)
            push(tokens, Token{type: token_type, value: word, line: line, column: start_column})
            continue
        }
        
        // Handle numbers
        ready (is_digit(char)) {
            sus start drip = i
            sus start_column drip = column
            bestie (i < len_str(source) && (is_digit(char_at(source, i)) || char_at(source, i) == ".")) {
                i = i + 1
                column = column + 1
            }
            sus number tea = substring(source, start, i)
            push(tokens, Token{type: "NUMBER", value: number, line: line, column: start_column})
            continue
        }
        
        // Handle strings
        ready (char == "\"") {
            sus start drip = i
            sus start_column drip = column
            i = i + 1
            column = column + 1
            bestie (i < len_str(source) && char_at(source, i) != "\"") {
                ready (char_at(source, i) == "\n") {
                    line = line + 1
                    column = 1
                } otherwise {
                    column = column + 1
                }
                i = i + 1
            }
            ready (i < len_str(source)) {
                i = i + 1
                column = column + 1
            }
            sus string_literal tea = substring(source, start, i)
            push(tokens, Token{type: "STRING", value: string_literal, line: line, column: start_column})
            continue
        }
        
        // Handle operators and punctuation
        sus operator tea = char
        sus token_type tea = "OPERATOR"
        ready (char == "{" || char == "}") {
            token_type = "BRACE"
        } otherwise ready (char == "(" || char == ")") {
            token_type = "PAREN"
        } otherwise ready (char == "[" || char == "]") {
            token_type = "BRACKET"
        } otherwise ready (char == ";") {
            token_type = "SEMICOLON"
        } otherwise ready (char == ",") {
            token_type = "COMMA"
        }
        
        push(tokens, Token{type: token_type, value: operator, line: line, column: column})
        i = i + 1
        column = column + 1
    }
    
    damn tokens
}

// Determine token type for keywords
slay get_token_type(word tea) tea {
    // CURSED keywords
    ready (word == "sus" || word == "slay" || word == "damn" || word == "ready" || word == "otherwise" || 
           word == "bestie" || word == "yeet" || word == "stan" || word == "squad" || word == "collab" ||
           word == "spill" || word == "vibez" || word == "based" || word == "cringe") {
        damn "KEYWORD"
    }
    
    // Types
    ready (word == "drip" || word == "tea" || word == "lit") {
        damn "TYPE"
    }
    
    damn "IDENTIFIER"
}

// Generate indentation string
slay generate_indent(context FormattingContext) tea {
    sus indent_str tea = ""
    sus total_indent drip = context.current_indent * context.config.indent_size
    sus i drip = 0
    bestie (i < total_indent) {
        ready (context.config.use_spaces) {
            indent_str = concat_str(indent_str, " ")
        } otherwise {
            indent_str = concat_str(indent_str, "\t")
        }
        i = i + 1
    }
    damn indent_str
}

// Format a list of tokens
slay format_tokens(tokens []Token, config FormatterConfig) tea {
    sus context FormattingContext = FormattingContext{
        config: config,
        current_indent: 0,
        line_length: 0,
        in_function_params: cringe,
        in_struct_definition: cringe,
        in_interface_definition: cringe
    }
    
    sus output tea = ""
    sus i drip = 0
    sus line_start lit = based
    
    bestie (i < len(tokens)) {
        sus token Token = tokens[i]
        
        // Handle line start indentation
        ready (line_start) {
            ready (token.type != "NEWLINE") {
                output = concat_str(output, generate_indent(context))
                line_start = cringe
            }
        }
        
        // Handle specific token formatting
        ready (token.type == "KEYWORD") {
            output = concat_str(output, token.value)
            
            // Add space after keywords that need it
            ready (token.value == "sus" || token.value == "slay" || token.value == "ready" || 
                   token.value == "bestie" || token.value == "yeet" || token.value == "squad" || 
                   token.value == "collab" || token.value == "spill") {
                output = concat_str(output, " ")
            }
        } otherwise ready (token.type == "BRACE") {
            ready (token.value == "{") {
                ready (context.config.newline_before_brace) {
                    output = concat_str(output, "\n")
                    output = concat_str(output, generate_indent(context))
                }
                output = concat_str(output, "{")
                output = concat_str(output, "\n")
                context.current_indent = context.current_indent + 1
                line_start = based
            } otherwise {
                context.current_indent = context.current_indent - 1
                output = concat_str(output, "\n")
                output = concat_str(output, generate_indent(context))
                output = concat_str(output, "}")
                output = concat_str(output, "\n")
                line_start = based
            }
        } otherwise ready (token.type == "SEMICOLON") {
            output = concat_str(output, ";")
            output = concat_str(output, "\n")
            line_start = based
        } otherwise ready (token.type == "COMMA") {
            output = concat_str(output, ",")
            ready (context.in_function_params || context.in_struct_definition) {
                output = concat_str(output, " ")
            } otherwise {
                output = concat_str(output, "\n")
                line_start = based
            }
        } otherwise ready (token.type == "OPERATOR") {
            ready (context.config.space_around_operators && 
                   (token.value == "=" || token.value == "+" || token.value == "-" || 
                    token.value == "*" || token.value == "/" || token.value == "==" || 
                    token.value == "!=" || token.value == "<" || token.value == ">")) {
                output = concat_str(output, " ")
                output = concat_str(output, token.value)
                output = concat_str(output, " ")
            } otherwise {
                output = concat_str(output, token.value)
            }
        } otherwise {
            output = concat_str(output, token.value)
            
            // Add space after certain tokens
            ready (token.type == "TYPE" || (token.type == "IDENTIFIER" && i + 1 < len(tokens) && tokens[i + 1].type == "IDENTIFIER")) {
                output = concat_str(output, " ")
            }
        }
        
        i = i + 1
    }
    
    damn output
}

// Main formatting function
slay format_cursed_code(source tea) tea {
    sus config FormatterConfig = default_formatter_config()
    sus tokens []Token = tokenize_cursed(source)
    damn format_tokens(tokens, config)
}

// Format with custom configuration
slay format_cursed_code_with_config(source tea, config FormatterConfig) tea {
    sus tokens []Token = tokenize_cursed(source)
    damn format_tokens(tokens, config)
}

// Helper functions for character checking
slay is_alpha(char tea) lit {
    damn (char >= "a" && char <= "z") || (char >= "A" && char <= "Z") || char == "_"
}

slay is_digit(char tea) lit {
    damn char >= "0" && char <= "9"
}

// Public API
slay main() {
    vibez.spill("CURSED Code Formatter - Self-Hosting Edition")
    
    // Example usage
    sus sample_code tea = "sus x drip=42;slay test(){damn x+1;}"
    vibez.spill("Original:")
    vibez.spill(sample_code)
    
    sus formatted tea = format_cursed_code(sample_code)
    vibez.spill("\nFormatted:")
    vibez.spill(formatted)
}
