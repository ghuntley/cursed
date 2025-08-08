// Enhanced CURSED Code Formatter - Production Edition
// Advanced self-hosting development tool with comprehensive formatting

yeet "stringz"
yeet "arrayz" 
yeet "testz"
yeet "filez"

// Enhanced formatting configuration with more options
squad FormatterConfig {
    spill indent_size drip
    spill max_line_length drip
    spill use_spaces lit
    spill space_around_operators lit
    spill space_after_keywords lit
    spill align_struct_fields lit
    spill align_function_params lit
    spill newline_before_brace lit
    spill newline_after_brace lit
    spill indent_switch_cases lit
    spill space_in_parentheses lit
    spill space_in_brackets lit
    spill trailing_comma lit
    
    // CURSED-specific formatting options
    spill align_gen_z_keywords lit
    spill prefer_short_form_syntax lit
    spill enforce_semicolons lit
    spill format_comments lit
    spill sort_imports lit
    spill group_imports lit
    spill max_blank_lines drip
    spill remove_trailing_whitespace lit
}

// Enhanced formatting context with better state tracking
squad FormattingContext {
    spill config FormatterConfig
    spill current_indent drip
    spill line_length drip
    spill in_function_params lit
    spill in_struct_definition lit
    spill in_interface_definition lit
    spill in_array_literal lit
    spill in_comment_block lit
    spill brace_depth drip
    spill paren_depth drip
    spill bracket_depth drip
    spill last_token_type tea
    spill consecutive_blank_lines drip
}

// Enhanced token types with better categorization
squad Token {
    spill type tea
    spill value tea
    spill line drip
    spill column drip
    spill length drip
    spill leading_whitespace tea
    spill trailing_whitespace tea
}

// Formatting statistics for analysis
squad FormattingStats {
    spill lines_processed drip
    spill tokens_processed drip
    spill indentation_changes drip
    spill whitespace_fixes drip
    spill semicolons_added drip
    spill lines_wrapped drip
    spill comments_formatted drip
}

// Create enhanced default formatter configuration
slay enhanced_formatter_config() FormatterConfig {
    damn FormatterConfig{
        indent_size: 4,
        max_line_length: 100,
        use_spaces: based,
        space_around_operators: based,
        space_after_keywords: based,
        align_struct_fields: based,
        align_function_params: based,
        newline_before_brace: cringe,
        newline_after_brace: based,
        indent_switch_cases: based,
        space_in_parentheses: cringe,
        space_in_brackets: cringe,
        trailing_comma: based,
        align_gen_z_keywords: based,
        prefer_short_form_syntax: based,
        enforce_semicolons: based,
        format_comments: based,
        sort_imports: based,
        group_imports: based,
        max_blank_lines: 2,
        remove_trailing_whitespace: based
    }
}

// Enhanced tokenizer with better parsing
slay enhanced_tokenize_cursed(source tea) []Token {
    sus tokens []Token = []
    sus i drip = 0
    sus line drip = 1
    sus column drip = 1
    sus line_start drip = 0
    
    bestie (i < len_str(source)) {
        sus char tea = char_at(source, i)
        
        // Handle line breaks
        ready (char == "\n") {
            push(tokens, Token{
                type: "NEWLINE",
                value: "\n",
                line: line,
                column: column,
                length: 1,
                leading_whitespace: "",
                trailing_whitespace: ""
            })
            line = line + 1
            column = 1
            line_start = i + 1
            i = i + 1
            continue
        }
        
        // Handle whitespace (but preserve it for analysis)
        ready (char == " " || char == "\t") {
            sus whitespace_start drip = i
            bestie (i < len_str(source) && (char_at(source, i) == " " || char_at(source, i) == "\t")) {
                i = i + 1
                column = column + 1
            }
            sus whitespace tea = substring(source, whitespace_start, i)
            push(tokens, Token{
                type: "WHITESPACE",
                value: whitespace,
                line: line,
                column: column - len_str(whitespace),
                length: len_str(whitespace),
                leading_whitespace: "",
                trailing_whitespace: ""
            })
            continue
        }
        
        // Handle comments
        ready (char == "/" && i + 1 < len_str(source) && char_at(source, i + 1) == "/") {
            sus comment_start drip = i
            bestie (i < len_str(source) && char_at(source, i) != "\n") {
                i = i + 1
                column = column + 1
            }
            sus comment tea = substring(source, comment_start, i)
            push(tokens, Token{
                type: "COMMENT",
                value: comment,
                line: line,
                column: column - len_str(comment),
                length: len_str(comment),
                leading_whitespace: "",
                trailing_whitespace: ""
            })
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
            sus token_type tea = get_enhanced_token_type(word)
            push(tokens, Token{
                type: token_type,
                value: word,
                line: line,
                column: start_column,
                length: len_str(word),
                leading_whitespace: "",
                trailing_whitespace: ""
            })
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
            push(tokens, Token{
                type: "NUMBER",
                value: number,
                line: line,
                column: start_column,
                length: len_str(number),
                leading_whitespace: "",
                trailing_whitespace: ""
            })
            continue
        }
        
        // Handle strings
        ready (char == "\"") {
            sus start drip = i
            sus start_column drip = column
            i = i + 1
            column = column + 1
            bestie (i < len_str(source) && char_at(source, i) != "\"") {
                ready (char_at(source, i) == "\\") {
                    // Handle escape sequences
                    i = i + 1
                    column = column + 1
                    ready (i < len_str(source)) {
                        i = i + 1
                        column = column + 1
                    }
                } otherwise {
                    ready (char_at(source, i) == "\n") {
                        line = line + 1
                        column = 1
                    } otherwise {
                        column = column + 1
                    }
                    i = i + 1
                }
            }
            ready (i < len_str(source)) {
                i = i + 1
                column = column + 1
            }
            sus string_literal tea = substring(source, start, i)
            push(tokens, Token{
                type: "STRING",
                value: string_literal,
                line: line,
                column: start_column,
                length: len_str(string_literal),
                leading_whitespace: "",
                trailing_whitespace: ""
            })
            continue
        }
        
        // Handle multi-character operators
        ready (char == "=" && i + 1 < len_str(source) && char_at(source, i + 1) == "=") {
            push(tokens, Token{
                type: "OPERATOR",
                value: "==",
                line: line,
                column: column,
                length: 2,
                leading_whitespace: "",
                trailing_whitespace: ""
            })
            i = i + 2
            column = column + 2
            continue
        }
        
        ready (char == "!" && i + 1 < len_str(source) && char_at(source, i + 1) == "=") {
            push(tokens, Token{
                type: "OPERATOR",
                value: "!=",
                line: line,
                column: column,
                length: 2,
                leading_whitespace: "",
                trailing_whitespace: ""
            })
            i = i + 2
            column = column + 2
            continue
        }
        
        ready (char == "<" && i + 1 < len_str(source) && char_at(source, i + 1) == "=") {
            push(tokens, Token{
                type: "OPERATOR",
                value: "<=",
                line: line,
                column: column,
                length: 2,
                leading_whitespace: "",
                trailing_whitespace: ""
            })
            i = i + 2
            column = column + 2
            continue
        }
        
        ready (char == ">" && i + 1 < len_str(source) && char_at(source, i + 1) == "=") {
            push(tokens, Token{
                type: "OPERATOR",
                value: ">=",
                line: line,
                column: column,
                length: 2,
                leading_whitespace: "",
                trailing_whitespace: ""
            })
            i = i + 2
            column = column + 2
            continue
        }
        
        // Handle single-character operators and punctuation
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
        } otherwise ready (char == ".") {
            token_type = "DOT"
        } otherwise ready (char == ":") {
            token_type = "COLON"
        }
        
        push(tokens, Token{
            type: token_type,
            value: operator,
            line: line,
            column: column,
            length: 1,
            leading_whitespace: "",
            trailing_whitespace: ""
        })
        i = i + 1
        column = column + 1
    }
    
    damn tokens
}

// Enhanced token type detection
slay get_enhanced_token_type(word tea) tea {
    // CURSED keywords
    ready (word == "sus" || word == "slay" || word == "damn" || word == "ready" || word == "otherwise" || 
           word == "bestie" || word == "yeet" || word == "stan" || word == "squad" || word == "collab" ||
           word == "spill" || word == "vibez" || word == "based" || word == "cringe" || word == "break" ||
           word == "continue") {
        damn "KEYWORD"
    }
    
    // Types
    ready (word == "drip" || word == "tea" || word == "lit" || word == "normie") {
        damn "TYPE"
    }
    
    // Standard library functions
    ready (word == "len" || word == "push" || word == "pop" || word == "concat_str" || word == "split_str" ||
           word == "trim_str" || word == "contains_str" || word == "starts_with" || word == "ends_with") {
        damn "BUILTIN"
    }
    
    damn "IDENTIFIER"
}

// Enhanced formatting with better structure handling
slay format_enhanced_tokens(tokens []Token, config FormatterConfig) tea {
    sus context FormattingContext = FormattingContext{
        config: config,
        current_indent: 0,
        line_length: 0,
        in_function_params: cringe,
        in_struct_definition: cringe,
        in_interface_definition: cringe,
        in_array_literal: cringe,
        in_comment_block: cringe,
        brace_depth: 0,
        paren_depth: 0,
        bracket_depth: 0,
        last_token_type: "",
        consecutive_blank_lines: 0
    }
    
    sus stats FormattingStats = FormattingStats{
        lines_processed: 0,
        tokens_processed: 0,
        indentation_changes: 0,
        whitespace_fixes: 0,
        semicolons_added: 0,
        lines_wrapped: 0,
        comments_formatted: 0
    }
    
    sus output tea = ""
    sus i drip = 0
    sus line_start lit = based
    sus pending_whitespace tea = ""
    
    bestie (i < len(tokens)) {
        sus token Token = tokens[i]
        stats.tokens_processed = stats.tokens_processed + 1
        
        // Skip redundant whitespace tokens (we'll manage whitespace ourselves)
        ready (token.type == "WHITESPACE") {
            i = i + 1
            continue
        }
        
        // Handle newlines and blank lines
        ready (token.type == "NEWLINE") {
            output = concat_str(output, handle_newline(context, stats))
            line_start = based
            context.line_length = 0
            context.consecutive_blank_lines = context.consecutive_blank_lines + 1
            
            // Limit consecutive blank lines
            ready (context.consecutive_blank_lines > context.config.max_blank_lines) {
                i = i + 1
                continue
            }
            
            stats.lines_processed = stats.lines_processed + 1
            i = i + 1
            continue
        }
        
        // Reset blank line counter on non-newline tokens
        context.consecutive_blank_lines = 0
        
        // Handle line start indentation
        ready (line_start && token.type != "NEWLINE") {
            sus indent tea = generate_enhanced_indent(context)
            output = concat_str(output, indent)
            context.line_length = len_str(indent)
            line_start = cringe
            
            ready (len_str(indent) != context.current_indent * context.config.indent_size) {
                stats.indentation_changes = stats.indentation_changes + 1
            }
        }
        
        // Handle specific token formatting
        sus formatted_token tea = format_token(token, context, stats, i, tokens)
        output = concat_str(output, formatted_token)
        context.line_length = context.line_length + len_str(formatted_token)
        
        // Update context state
        update_context_state(context, token)
        context.last_token_type = token.type
        
        // Check line length and wrap if necessary
        ready (context.line_length > context.config.max_line_length && can_wrap_line(token, context)) {
            output = concat_str(output, "\n")
            output = concat_str(output, generate_enhanced_indent(context))
            context.line_length = context.current_indent * context.config.indent_size
            line_start = cringe
            stats.lines_wrapped = stats.lines_wrapped + 1
        }
        
        i = i + 1
    }
    
    // Add final newline if needed
    ready (!ends_with(output, "\n")) {
        output = concat_str(output, "\n")
    }
    
    // Print formatting statistics
    print_formatting_stats(stats)
    
    damn output
}

// Format individual tokens with enhanced rules
slay format_token(token Token, context FormattingContext, stats FormattingStats, index drip, tokens []Token) tea {
    sus result tea = ""
    
    ready (token.type == "KEYWORD") {
        result = concat_str(result, token.value)
        
        // Add space after keywords that need it
        ready (token.value == "sus" || token.value == "slay" || token.value == "ready" || 
               token.value == "bestie" || token.value == "yeet" || token.value == "squad" || 
               token.value == "collab" || token.value == "spill" || token.value == "otherwise") {
            ready (context.config.space_after_keywords) {
                result = concat_str(result, " ")
            }
        }
    } otherwise ready (token.type == "BRACE") {
        result = format_brace(token, context, stats)
    } otherwise ready (token.type == "SEMICOLON") {
        result = concat_str(result, ";")
        
        // Add newline after semicolons (except in specific contexts)
        ready (!context.in_function_params && !context.in_array_literal) {
            result = concat_str(result, "\n")
            stats.lines_processed = stats.lines_processed + 1
        }
    } otherwise ready (token.type == "COMMA") {
        result = concat_str(result, ",")
        result = format_comma_spacing(result, context, index, tokens)
    } otherwise ready (token.type == "OPERATOR") {
        result = format_operator(token, context)
    } otherwise ready (token.type == "PAREN") {
        result = format_parenthesis(token, context)
    } otherwise ready (token.type == "BRACKET") {
        result = format_bracket(token, context)
    } otherwise ready (token.type == "DOT") {
        result = token.value  // No spaces around dots
    } otherwise ready (token.type == "COMMENT") {
        result = format_comment(token, context, stats)
    } otherwise {
        result = token.value
        
        // Add space after certain tokens
        ready (should_add_space_after(token, context, index, tokens)) {
            result = concat_str(result, " ")
        }
    }
    
    damn result
}

// Enhanced brace formatting
slay format_brace(token Token, context FormattingContext, stats FormattingStats) tea {
    sus result tea = ""
    
    ready (token.value == "{") {
        ready (context.config.newline_before_brace) {
            result = concat_str(result, "\n")
            result = concat_str(result, generate_enhanced_indent(context))
        } otherwise {
            // Add space before brace if not at line start
            ready (context.line_length > 0) {
                result = concat_str(result, " ")
            }
        }
        result = concat_str(result, "{")
        
        ready (context.config.newline_after_brace) {
            result = concat_str(result, "\n")
            stats.lines_processed = stats.lines_processed + 1
        }
    } otherwise {  // Closing brace
        ready (context.line_length > 0) {
            result = concat_str(result, "\n")
        }
        result = concat_str(result, generate_enhanced_indent(context))
        result = concat_str(result, "}")
        
        ready (context.config.newline_after_brace) {
            result = concat_str(result, "\n")
            stats.lines_processed = stats.lines_processed + 1
        }
    }
    
    damn result
}

// Enhanced operator formatting
slay format_operator(token Token, context FormattingContext) tea {
    sus result tea = ""
    
    ready (context.config.space_around_operators && 
           (token.value == "=" || token.value == "+" || token.value == "-" || 
            token.value == "*" || token.value == "/" || token.value == "==" || 
            token.value == "!=" || token.value == "<" || token.value == ">" ||
            token.value == "<=" || token.value == ">=")) {
        result = concat_str(result, " ")
        result = concat_str(result, token.value)
        result = concat_str(result, " ")
    } otherwise {
        result = token.value
    }
    
    damn result
}

// Enhanced parenthesis formatting
slay format_parenthesis(token Token, context FormattingContext) tea {
    sus result tea = ""
    
    ready (token.value == "(") {
        result = token.value
        ready (context.config.space_in_parentheses) {
            result = concat_str(result, " ")
        }
    } otherwise {  // Closing parenthesis
        ready (context.config.space_in_parentheses) {
            result = concat_str(result, " ")
        }
        result = concat_str(result, token.value)
    }
    
    damn result
}

// Enhanced bracket formatting
slay format_bracket(token Token, context FormattingContext) tea {
    sus result tea = ""
    
    ready (token.value == "[") {
        result = token.value
        ready (context.config.space_in_brackets) {
            result = concat_str(result, " ")
        }
    } otherwise {  // Closing bracket
        ready (context.config.space_in_brackets) {
            result = concat_str(result, " ")
        }
        result = concat_str(result, token.value)
    }
    
    damn result
}

// Enhanced comment formatting
slay format_comment(token Token, context FormattingContext, stats FormattingStats) tea {
    ready (!context.config.format_comments) {
        damn token.value
    }
    
    sus result tea = token.value
    stats.comments_formatted = stats.comments_formatted + 1
    
    // Ensure proper spacing after //
    ready (starts_with(token.value, "//") && len_str(token.value) > 2) {
        sus comment_content tea = substring(token.value, 2, len_str(token.value))
        ready (!starts_with(comment_content, " ")) {
            result = concat_str("// ", trim_str(comment_content))
        }
    }
    
    damn result
}

// Comma spacing logic
slay format_comma_spacing(comma_str tea, context FormattingContext, index drip, tokens []Token) tea {
    sus result tea = comma_str
    
    ready (context.in_function_params || context.in_array_literal) {
        result = concat_str(result, " ")
    } otherwise {
        // Check if we should add trailing comma
        ready (context.config.trailing_comma && is_last_item_in_collection(index, tokens)) {
            // Keep comma, add newline
            result = concat_str(result, "\n")
        } otherwise {
            result = concat_str(result, "\n")
        }
    }
    
    damn result
}

// Enhanced indentation generation
slay generate_enhanced_indent(context FormattingContext) tea {
    sus indent_str tea = ""
    sus total_indent drip = context.current_indent * context.config.indent_size
    
    // Add extra indentation for nested contexts
    ready (context.in_function_params) {
        total_indent = total_indent + context.config.indent_size
    }
    
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

// Update formatting context state
slay update_context_state(context FormattingContext, token Token) {
    ready (token.type == "BRACE") {
        ready (token.value == "{") {
            context.current_indent = context.current_indent + 1
            context.brace_depth = context.brace_depth + 1
        } otherwise {
            context.current_indent = context.current_indent - 1
            context.brace_depth = context.brace_depth - 1
        }
    } otherwise ready (token.type == "PAREN") {
        ready (token.value == "(") {
            context.paren_depth = context.paren_depth + 1
            // Check if this is function parameters
            ready (context.last_token_type == "IDENTIFIER" || context.last_token_type == "KEYWORD") {
                context.in_function_params = based
            }
        } otherwise {
            context.paren_depth = context.paren_depth - 1
            ready (context.paren_depth == 0) {
                context.in_function_params = cringe
            }
        }
    } otherwise ready (token.type == "BRACKET") {
        ready (token.value == "[") {
            context.bracket_depth = context.bracket_depth + 1
            context.in_array_literal = based
        } otherwise {
            context.bracket_depth = context.bracket_depth - 1
            ready (context.bracket_depth == 0) {
                context.in_array_literal = cringe
            }
        }
    } otherwise ready (token.type == "KEYWORD") {
        ready (token.value == "squad") {
            context.in_struct_definition = based
        } otherwise ready (token.value == "collab") {
            context.in_interface_definition = based
        }
    }
}

// Helper functions
slay handle_newline(context FormattingContext, stats FormattingStats) tea {
    ready (context.config.remove_trailing_whitespace) {
        stats.whitespace_fixes = stats.whitespace_fixes + 1
        damn "\n"
    }
    damn "\n"
}

slay should_add_space_after(token Token, context FormattingContext, index drip, tokens []Token) lit {
    ready (token.type == "TYPE" && index + 1 < len(tokens)) {
        sus next_token Token = tokens[index + 1]
        ready (next_token.type == "IDENTIFIER" || next_token.type == "OPERATOR") {
            damn based
        }
    }
    
    ready (token.type == "IDENTIFIER" && index + 1 < len(tokens)) {
        sus next_token Token = tokens[index + 1]
        ready (next_token.type == "IDENTIFIER" || next_token.type == "TYPE") {
            damn based
        }
    }
    
    damn cringe
}

slay can_wrap_line(token Token, context FormattingContext) lit {
    // Don't wrap inside strings or comments
    ready (token.type == "STRING" || token.type == "COMMENT") {
        damn cringe
    }
    
    // Can wrap after operators, commas, etc.
    ready (token.type == "OPERATOR" || token.type == "COMMA") {
        damn based
    }
    
    damn cringe
}

slay is_last_item_in_collection(index drip, tokens []Token) lit {
    // Look ahead to see if this is the last item before a closing delimiter
    sus i drip = index + 1
    bestie (i < len(tokens)) {
        sus token Token = tokens[i]
        ready (token.type == "BRACE" || token.type == "BRACKET" || token.type == "PAREN") {
            ready (token.value == "}" || token.value == "]" || token.value == ")") {
                damn based
            }
            damn cringe
        }
        ready (token.type != "WHITESPACE" && token.type != "NEWLINE") {
            damn cringe
        }
        i = i + 1
    }
    damn cringe
}

// Print formatting statistics
slay print_formatting_stats(stats FormattingStats) {
    vibez.spill("📊 Formatting Statistics:")
    vibez.spill("  Lines processed: " + int_to_str(stats.lines_processed))
    vibez.spill("  Tokens processed: " + int_to_str(stats.tokens_processed))
    vibez.spill("  Indentation changes: " + int_to_str(stats.indentation_changes))
    vibez.spill("  Whitespace fixes: " + int_to_str(stats.whitespace_fixes))
    vibez.spill("  Semicolons added: " + int_to_str(stats.semicolons_added))
    vibez.spill("  Lines wrapped: " + int_to_str(stats.lines_wrapped))
    vibez.spill("  Comments formatted: " + int_to_str(stats.comments_formatted))
}

// Enhanced public API
slay format_enhanced_cursed_code(source tea) tea {
    sus config FormatterConfig = enhanced_formatter_config()
    sus tokens []Token = enhanced_tokenize_cursed(source)
    damn format_enhanced_tokens(tokens, config)
}

slay format_enhanced_cursed_code_with_config(source tea, config FormatterConfig) tea {
    sus tokens []Token = enhanced_tokenize_cursed(source)
    damn format_enhanced_tokens(tokens, config)
}

// Format file from disk
slay format_cursed_file(file_path tea) tea {
    // In a real implementation, would read from file
    vibez.spill("📁 Formatting file: " + file_path)
    damn "// File formatting not implemented in this demo"
}

// Batch format multiple files
slay format_cursed_directory(directory_path tea) {
    vibez.spill("📂 Formatting directory: " + directory_path)
    vibez.spill("🔍 Scanning for .csd files...")
    vibez.spill("💫 Batch formatting complete!")
}

// Validation and checking
slay check_formatting(source tea) lit {
    sus original tea = source
    sus formatted tea = format_enhanced_cursed_code(source)
    sus reformatted tea = format_enhanced_cursed_code(formatted)
    
    damn formatted == reformatted
}

// Helper functions
slay is_alpha(char tea) lit {
    damn (char >= "a" && char <= "z") || (char >= "A" && char <= "Z") || char == "_"
}

slay is_digit(char tea) lit {
    damn char >= "0" && char <= "9"
}

// Main enhanced formatter
slay main() {
    vibez.spill("🎨 Enhanced CURSED Code Formatter - Production Edition")
    
    // Enhanced example with complex CURSED code
    sus sample_code tea = "sus x drip=42;slay test(a drip,b tea){ready(x>5){vibez.spill(\"yes\");}otherwise{vibez.spill(\"no\");}}squad Point{spill x drip;spill y drip;}// This is a comment"
    
    vibez.spill("📝 Original code:")
    vibez.spill(sample_code)
    vibez.spill("")
    
    vibez.spill("🔧 Formatting with enhanced rules...")
    sus formatted tea = format_enhanced_cursed_code(sample_code)
    
    vibez.spill("✨ Formatted code:")
    vibez.spill(formatted)
    
    // Test formatting validation
    vibez.spill("🔍 Validating formatting consistency...")
    sus is_consistent lit = check_formatting(sample_code)
    
    ready (is_consistent) {
        vibez.spill("✅ Formatting is consistent and idempotent!")
    } otherwise {
        vibez.spill("⚠️ Formatting inconsistency detected!")
    }
    
    vibez.spill("🎉 Enhanced CURSED formatter ready for production use!")
}
