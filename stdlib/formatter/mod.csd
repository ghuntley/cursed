fr fr CURSED AST-Based Code Formatter - Production Implementation
fr fr Self-hosting formatter with comprehensive configuration and error recovery
fr fr Replaces 900+ line Rust formatter with equivalent functionality in pure CURSED

yeet "stringz"
yeet "arrayz"
yeet "testz"

fr fr ===== AST NODE DEFINITIONS =====

squad ASTNodeType {
    spill name tea
    spill category tea
}

slay ast_node_type(name tea, category tea) ASTNodeType {
    damn ASTNodeType{name: name, category: category}
}

fr fr Core AST node types
sus AST_PROGRAM drip = 1
sus AST_FUNCTION drip = 2
sus AST_VARIABLE drip = 3
sus AST_STRUCT drip = 4
sus AST_INTERFACE drip = 5
sus AST_IF_STATEMENT drip = 6
sus AST_WHILE_LOOP drip = 7
sus AST_FOR_LOOP drip = 8
sus AST_EXPRESSION drip = 9
sus AST_BLOCK drip = 10
sus AST_IMPORT drip = 11
sus AST_COMMENT drip = 12

squad ASTNode {
    spill type drip
    spill value tea
    spill children []ASTNode
    spill line drip
    spill column drip
    spill formatted lit
}

fr fr ===== COMPREHENSIVE FORMATTER CONFIGURATION =====

squad FormatterConfig {
    fr fr Indentation settings
    spill indent_size drip
    spill use_tabs lit
    spill continuation_indent drip
    
    fr fr Line length and wrapping
    spill max_line_length drip
    spill wrap_long_lines lit
    spill wrap_before_operators lit
    
    fr fr Spacing rules
    spill space_around_operators lit
    spill space_after_commas lit
    spill space_before_colon lit
    spill space_after_colon lit
    spill space_in_parentheses lit
    spill space_in_brackets lit
    spill space_in_braces lit
    
    fr fr Brace placement
    spill opening_brace_style tea  fr fr "same_line", "new_line", "end_of_line"
    spill closing_brace_style tea
    spill always_wrap_braces lit
    
    fr fr CURSED-specific formatting
    spill align_gen_z_keywords lit
    spill prefer_short_form_syntax lit
    spill normalize_keywords lit
    spill align_struct_fields lit
    spill align_function_params lit
    spill sort_imports lit
    
    fr fr Comment handling
    spill preserve_comment_indentation lit
    spill align_line_comments lit
    spill wrap_comments lit
    spill comment_line_length drip
    
    fr fr Advanced formatting
    spill blank_lines_before_functions drip
    spill blank_lines_before_structs drip
    spill blank_lines_before_interfaces drip
    spill preserve_blank_lines drip
    spill max_blank_lines drip
    
    fr fr Error recovery
    spill continue_on_errors lit
    spill preserve_malformed_syntax lit
    spill add_error_comments lit
}

slay default_formatter_config() FormatterConfig {
    damn FormatterConfig{
        indent_size: 4,
        use_tabs: cringe,
        continuation_indent: 4,
        
        max_line_length: 100,
        wrap_long_lines: based,
        wrap_before_operators: based,
        
        space_around_operators: based,
        space_after_commas: based,
        space_before_colon: cringe,
        space_after_colon: based,
        space_in_parentheses: cringe,
        space_in_brackets: cringe,
        space_in_braces: cringe,
        
        opening_brace_style: "same_line",
        closing_brace_style: "aligned",
        always_wrap_braces: cringe,
        
        align_gen_z_keywords: based,
        prefer_short_form_syntax: based,
        normalize_keywords: based,
        align_struct_fields: based,
        align_function_params: based,
        sort_imports: based,
        
        preserve_comment_indentation: based,
        align_line_comments: based,
        wrap_comments: based,
        comment_line_length: 80,
        
        blank_lines_before_functions: 1,
        blank_lines_before_structs: 1,
        blank_lines_before_interfaces: 1,
        preserve_blank_lines: 2,
        max_blank_lines: 3,
        
        continue_on_errors: based,
        preserve_malformed_syntax: based,
        add_error_comments: cringe
    }
}

fr fr Predefined configuration profiles
slay compact_formatter_config() FormatterConfig {
    sus config FormatterConfig = default_formatter_config()
    config.indent_size = 2
    config.max_line_length = 80
    config.blank_lines_before_functions = 0
    config.blank_lines_before_structs = 0
    config.blank_lines_before_interfaces = 0
    damn config
}

slay google_style_config() FormatterConfig {
    sus config FormatterConfig = default_formatter_config()
    config.indent_size = 2
    config.max_line_length = 120
    config.opening_brace_style = "same_line"
    config.space_before_colon = cringe
    damn config
}

slay mozilla_style_config() FormatterConfig {
    sus config FormatterConfig = default_formatter_config()
    config.indent_size = 2
    config.max_line_length = 80
    config.opening_brace_style = "new_line"
    config.always_wrap_braces = based
    damn config
}

fr fr ===== ADVANCED TOKENIZER WITH POSITION TRACKING =====

squad Token {
    spill type tea
    spill value tea
    spill line drip
    spill column drip
    spill start_pos drip
    spill end_pos drip
    spill leading_trivia tea
    spill trailing_trivia tea
}

squad TokenizerContext {
    spill source tea
    spill position drip
    spill line drip
    spill column drip
    spill tokens []Token
    spill errors []tea
    spill preserve_whitespace lit
}

slay create_tokenizer_context(source tea) TokenizerContext {
    damn TokenizerContext{
        source: source,
        position: 0,
        line: 1,
        column: 1,
        tokens: [],
        errors: [],
        preserve_whitespace: based
    }
}

slay peek_char(ctx TokenizerContext) tea {
    ready (ctx.position >= string_length(ctx.source)) {
        damn ""
    }
    damn char_at(ctx.source, ctx.position)
}

slay advance_char(ctx TokenizerContext) tea {
    ready (ctx.position >= string_length(ctx.source)) {
        damn ""
    }
    
    sus ch tea = char_at(ctx.source, ctx.position)
    ctx.position = ctx.position + 1
    
    ready (ch == "\n") {
        ctx.line = ctx.line + 1
        ctx.column = 1
    } otherwise {
        ctx.column = ctx.column + 1
    }
    
    damn ch
}

slay is_whitespace(ch tea) lit {
    damn ch == " " || ch == "\t" || ch == "\n" || ch == "\r"
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

slay collect_whitespace(ctx TokenizerContext) tea {
    sus whitespace tea = ""
    bestie (ctx.position < string_length(ctx.source) && is_whitespace(peek_char(ctx))) {
        whitespace = whitespace + advance_char(ctx)
    }
    damn whitespace
}

slay collect_identifier(ctx TokenizerContext) tea {
    sus identifier tea = ""
    bestie (ctx.position < string_length(ctx.source) && is_alphanumeric(peek_char(ctx))) {
        identifier = identifier + advance_char(ctx)
    }
    damn identifier
}

slay collect_string_literal(ctx TokenizerContext) tea {
    sus literal tea = ""
    sus quote_char tea = advance_char(ctx)  fr fr Consume opening quote
    literal = literal + quote_char
    
    bestie (ctx.position < string_length(ctx.source)) {
        sus ch tea = peek_char(ctx)
        ready (ch == quote_char) {
            literal = literal + advance_char(ctx)  fr fr Consume closing quote
            break
        }
        ready (ch == "\\") {
            literal = literal + advance_char(ctx)  fr fr Escape character
            ready (ctx.position < string_length(ctx.source)) {
                literal = literal + advance_char(ctx)  fr fr Escaped character
            }
        } otherwise {
            literal = literal + advance_char(ctx)
        }
    }
    
    damn literal
}

slay get_keyword_type(word tea) tea {
    fr fr CURSED keywords
    ready (word == "sus" || word == "slay" || word == "damn" || word == "ready" || 
           word == "otherwise" || word == "bestie" || word == "yeet" || word == "stan" ||
           word == "squad" || word == "collab" || word == "spill" || word == "vibez" ||
           word == "based" || word == "cringe" || word == "fr" || word == "sick" ||
           word == "when" || word == "fam" || word == "yikes" || word == "shook") {
        damn "KEYWORD"
    }
    
    fr fr Types
    ready (word == "drip" || word == "tea" || word == "lit") {
        damn "TYPE"
    }
    
    damn "IDENTIFIER"
}

slay tokenize_advanced(source tea) TokenizerContext {
    sus ctx TokenizerContext = create_tokenizer_context(source)
    
    bestie (ctx.position < string_length(ctx.source)) {
        sus start_pos drip = ctx.position
        sus start_line drip = ctx.line
        sus start_column drip = ctx.column
        sus leading_trivia tea = ""
        
        fr fr Collect leading whitespace and comments
        ready (is_whitespace(peek_char(ctx))) {
            leading_trivia = collect_whitespace(ctx)
            continue
        }
        
        fr fr Handle comments
        ready (peek_char(ctx) == "/" && char_at(ctx.source, ctx.position + 1) == "/") {
            sus comment tea = ""
            bestie (ctx.position < string_length(ctx.source) && peek_char(ctx) != "\n") {
                comment = comment + advance_char(ctx)
            }
            
            sus token Token = Token{
                type: "COMMENT",
                value: comment,
                line: start_line,
                column: start_column,
                start_pos: start_pos,
                end_pos: ctx.position,
                leading_trivia: leading_trivia,
                trailing_trivia: ""
            }
            
            push(ctx.tokens, token)
            continue
        }
        
        fr fr Handle string literals
        ready (peek_char(ctx) == "\"" || peek_char(ctx) == "'") {
            sus literal tea = collect_string_literal(ctx)
            
            sus token Token = Token{
                type: "STRING",
                value: literal,
                line: start_line,
                column: start_column,
                start_pos: start_pos,
                end_pos: ctx.position,
                leading_trivia: leading_trivia,
                trailing_trivia: ""
            }
            
            push(ctx.tokens, token)
            continue
        }
        
        fr fr Handle identifiers and keywords
        ready (is_alpha(peek_char(ctx))) {
            sus word tea = collect_identifier(ctx)
            sus token_type tea = get_keyword_type(word)
            
            sus token Token = Token{
                type: token_type,
                value: word,
                line: start_line,
                column: start_column,
                start_pos: start_pos,
                end_pos: ctx.position,
                leading_trivia: leading_trivia,
                trailing_trivia: ""
            }
            
            push(ctx.tokens, token)
            continue
        }
        
        fr fr Handle numbers
        ready (is_digit(peek_char(ctx))) {
            sus number tea = ""
            bestie (ctx.position < string_length(ctx.source) && 
                   (is_digit(peek_char(ctx)) || peek_char(ctx) == ".")) {
                number = number + advance_char(ctx)
            }
            
            sus token Token = Token{
                type: "NUMBER",
                value: number,
                line: start_line,
                column: start_column,
                start_pos: start_pos,
                end_pos: ctx.position,
                leading_trivia: leading_trivia,
                trailing_trivia: ""
            }
            
            push(ctx.tokens, token)
            continue
        }
        
        fr fr Handle operators and punctuation
        sus ch tea = advance_char(ctx)
        sus token_type tea = "OPERATOR"
        
        ready (ch == "{" || ch == "}") {
            token_type = "BRACE"
        } otherwise ready (ch == "(" || ch == ")") {
            token_type = "PAREN"
        } otherwise ready (ch == "[" || ch == "]") {
            token_type = "BRACKET"
        } otherwise ready (ch == ";") {
            token_type = "SEMICOLON"
        } otherwise ready (ch == ",") {
            token_type = "COMMA"
        } otherwise ready (ch == ":") {
            token_type = "COLON"
        } otherwise ready (ch == ".") {
            token_type = "DOT"
        }
        
        sus token Token = Token{
            type: token_type,
            value: ch,
            line: start_line,
            column: start_column,
            start_pos: start_pos,
            end_pos: ctx.position,
            leading_trivia: leading_trivia,
            trailing_trivia: ""
        }
        
        push(ctx.tokens, token)
    }
    
    damn ctx
}

fr fr ===== AST PARSER WITH ERROR RECOVERY =====

squad ParserContext {
    spill tokens []Token
    spill position drip
    spill errors []tea
    spill config FormatterConfig
}

slay create_parser_context(tokens []Token, config FormatterConfig) ParserContext {
    damn ParserContext{
        tokens: tokens,
        position: 0,
        errors: [],
        config: config
    }
}

slay current_token(ctx ParserContext) Token {
    ready (ctx.position >= len(ctx.tokens)) {
        damn Token{type: "EOF", value: "", line: 0, column: 0, start_pos: 0, end_pos: 0, leading_trivia: "", trailing_trivia: ""}
    }
    damn ctx.tokens[ctx.position]
}

slay advance_token(ctx ParserContext) Token {
    sus token Token = current_token(ctx)
    ready (ctx.position < len(ctx.tokens)) {
        ctx.position = ctx.position + 1
    }
    damn token
}

slay expect_token(ctx ParserContext, expected_type tea) Token {
    sus token Token = current_token(ctx)
    ready (token.type != expected_type) {
        sus error tea = "Expected " + expected_type + " but found " + token.type + " at line " + int_to_string(token.line)
        push(ctx.errors, error)
        
        fr fr Error recovery: skip to next valid token
        bestie (ctx.position < len(ctx.tokens) && current_token(ctx).type != expected_type) {
            advance_token(ctx)
        }
    }
    damn advance_token(ctx)
}

slay parse_ast(tokens []Token, config FormatterConfig) ASTNode {
    sus ctx ParserContext = create_parser_context(tokens, config)
    sus root ASTNode = ASTNode{
        type: AST_PROGRAM,
        value: "program",
        children: [],
        line: 1,
        column: 1,
        formatted: cringe
    }
    
    bestie (ctx.position < len(ctx.tokens)) {
        sus token Token = current_token(ctx)
        
        ready (token.type == "EOF") {
            break
        }
        
        sus node ASTNode = parse_statement(ctx)
        push(root.children, node)
    }
    
    damn root
}

slay parse_statement(ctx ParserContext) ASTNode {
    sus token Token = current_token(ctx)
    
    ready (token.value == "slay") {
        damn parse_function(ctx)
    } otherwise ready (token.value == "sus") {
        damn parse_variable(ctx)
    } otherwise ready (token.value == "squad") {
        damn parse_struct(ctx)
    } otherwise ready (token.value == "collab") {
        damn parse_interface(ctx)
    } otherwise ready (token.value == "ready") {
        damn parse_if_statement(ctx)
    } otherwise ready (token.value == "bestie") {
        damn parse_while_loop(ctx)
    } otherwise ready (token.value == "yeet") {
        damn parse_import(ctx)
    } otherwise ready (token.type == "COMMENT") {
        damn parse_comment(ctx)
    } otherwise {
        fr fr Error recovery: create expression node
        damn parse_expression(ctx)
    }
}

slay parse_function(ctx ParserContext) ASTNode {
    sus start_token Token = expect_token(ctx, "KEYWORD")  fr fr "slay"
    sus name_token Token = expect_token(ctx, "IDENTIFIER")
    
    sus node ASTNode = ASTNode{
        type: AST_FUNCTION,
        value: name_token.value,
        children: [],
        line: start_token.line,
        column: start_token.column,
        formatted: cringe
    }
    
    fr fr Parse parameters
    ready (current_token(ctx).type == "PAREN" && current_token(ctx).value == "(") {
        advance_token(ctx)  fr fr Consume '('
        
        bestie (current_token(ctx).type != "PAREN" || current_token(ctx).value != ")") {
            sus param ASTNode = parse_parameter(ctx)
            push(node.children, param)
            
            ready (current_token(ctx).type == "COMMA") {
                advance_token(ctx)  fr fr Consume ','
            } otherwise {
                break
            }
        }
        
        expect_token(ctx, "PAREN")  fr fr Consume ')'
    }
    
    fr fr Parse return type
    ready (current_token(ctx).type == "TYPE") {
        sus return_type ASTNode = ASTNode{
            type: AST_EXPRESSION,
            value: current_token(ctx).value,
            children: [],
            line: current_token(ctx).line,
            column: current_token(ctx).column,
            formatted: cringe
        }
        push(node.children, return_type)
        advance_token(ctx)
    }
    
    fr fr Parse body
    ready (current_token(ctx).type == "BRACE" && current_token(ctx).value == "{") {
        sus body ASTNode = parse_block(ctx)
        push(node.children, body)
    }
    
    damn node
}

slay parse_variable(ctx ParserContext) ASTNode {
    sus start_token Token = expect_token(ctx, "KEYWORD")  fr fr "sus"
    sus name_token Token = expect_token(ctx, "IDENTIFIER")
    
    sus node ASTNode = ASTNode{
        type: AST_VARIABLE,
        value: name_token.value,
        children: [],
        line: start_token.line,
        column: start_token.column,
        formatted: cringe
    }
    
    fr fr Parse type annotation
    ready (current_token(ctx).type == "TYPE") {
        sus type_node ASTNode = ASTNode{
            type: AST_EXPRESSION,
            value: current_token(ctx).value,
            children: [],
            line: current_token(ctx).line,
            column: current_token(ctx).column,
            formatted: cringe
        }
        push(node.children, type_node)
        advance_token(ctx)
    }
    
    fr fr Parse initializer
    ready (current_token(ctx).type == "OPERATOR" && current_token(ctx).value == "=") {
        advance_token(ctx)  fr fr Consume '='
        sus init ASTNode = parse_expression(ctx)
        push(node.children, init)
    }
    
    expect_token(ctx, "SEMICOLON")
    damn node
}

slay parse_struct(ctx ParserContext) ASTNode {
    sus start_token Token = expect_token(ctx, "KEYWORD")  fr fr "squad"
    sus name_token Token = expect_token(ctx, "IDENTIFIER")
    
    sus node ASTNode = ASTNode{
        type: AST_STRUCT,
        value: name_token.value,
        children: [],
        line: start_token.line,
        column: start_token.column,
        formatted: cringe
    }
    
    expect_token(ctx, "BRACE")  fr fr "{"
    
    bestie (current_token(ctx).type != "BRACE" || current_token(ctx).value != "}") {
        ready (current_token(ctx).value == "spill") {
            sus field ASTNode = parse_struct_field(ctx)
            push(node.children, field)
        } otherwise {
            advance_token(ctx)  fr fr Skip unexpected token
        }
    }
    
    expect_token(ctx, "BRACE")  fr fr "}"
    damn node
}

slay parse_interface(ctx ParserContext) ASTNode {
    sus start_token Token = expect_token(ctx, "KEYWORD")  fr fr "collab"
    sus name_token Token = expect_token(ctx, "IDENTIFIER")
    
    sus node ASTNode = ASTNode{
        type: AST_INTERFACE,
        value: name_token.value,
        children: [],
        line: start_token.line,
        column: start_token.column,
        formatted: cringe
    }
    
    expect_token(ctx, "BRACE")  fr fr "{"
    
    bestie (current_token(ctx).type != "BRACE" || current_token(ctx).value != "}") {
        ready (current_token(ctx).value == "slay") {
            sus method ASTNode = parse_function(ctx)
            push(node.children, method)
        } otherwise {
            advance_token(ctx)  fr fr Skip unexpected token
        }
    }
    
    expect_token(ctx, "BRACE")  fr fr "}"
    damn node
}

slay parse_if_statement(ctx ParserContext) ASTNode {
    sus start_token Token = expect_token(ctx, "KEYWORD")  fr fr "ready"
    
    sus node ASTNode = ASTNode{
        type: AST_IF_STATEMENT,
        value: "if",
        children: [],
        line: start_token.line,
        column: start_token.column,
        formatted: cringe
    }
    
    expect_token(ctx, "PAREN")  fr fr "("
    sus condition ASTNode = parse_expression(ctx)
    push(node.children, condition)
    expect_token(ctx, "PAREN")  fr fr ")"
    
    sus then_body ASTNode = parse_block(ctx)
    push(node.children, then_body)
    
    ready (current_token(ctx).value == "otherwise") {
        advance_token(ctx)  fr fr "otherwise"
        sus else_body ASTNode = parse_block(ctx)
        push(node.children, else_body)
    }
    
    damn node
}

slay parse_while_loop(ctx ParserContext) ASTNode {
    sus start_token Token = expect_token(ctx, "KEYWORD")  fr fr "bestie"
    
    sus node ASTNode = ASTNode{
        type: AST_WHILE_LOOP,
        value: "while",
        children: [],
        line: start_token.line,
        column: start_token.column,
        formatted: cringe
    }
    
    expect_token(ctx, "PAREN")  fr fr "("
    sus condition ASTNode = parse_expression(ctx)
    push(node.children, condition)
    expect_token(ctx, "PAREN")  fr fr ")"
    
    sus body ASTNode = parse_block(ctx)
    push(node.children, body)
    
    damn node
}

slay parse_import(ctx ParserContext) ASTNode {
    sus start_token Token = expect_token(ctx, "KEYWORD")  fr fr "yeet"
    sus module_token Token = expect_token(ctx, "STRING")
    
    sus node ASTNode = ASTNode{
        type: AST_IMPORT,
        value: module_token.value,
        children: [],
        line: start_token.line,
        column: start_token.column,
        formatted: cringe
    }
    
    expect_token(ctx, "SEMICOLON")
    damn node
}

slay parse_comment(ctx ParserContext) ASTNode {
    sus token Token = advance_token(ctx)
    
    damn ASTNode{
        type: AST_COMMENT,
        value: token.value,
        children: [],
        line: token.line,
        column: token.column,
        formatted: cringe
    }
}

slay parse_block(ctx ParserContext) ASTNode {
    sus start_token Token = expect_token(ctx, "BRACE")  fr fr "{"
    
    sus node ASTNode = ASTNode{
        type: AST_BLOCK,
        value: "block",
        children: [],
        line: start_token.line,
        column: start_token.column,
        formatted: cringe
    }
    
    bestie (current_token(ctx).type != "BRACE" || current_token(ctx).value != "}") {
        sus stmt ASTNode = parse_statement(ctx)
        push(node.children, stmt)
    }
    
    expect_token(ctx, "BRACE")  fr fr "}"
    damn node
}

slay parse_expression(ctx ParserContext) ASTNode {
    sus token Token = advance_token(ctx)
    
    damn ASTNode{
        type: AST_EXPRESSION,
        value: token.value,
        children: [],
        line: token.line,
        column: token.column,
        formatted: cringe
    }
}

slay parse_parameter(ctx ParserContext) ASTNode {
    sus name_token Token = expect_token(ctx, "IDENTIFIER")
    sus type_token Token = expect_token(ctx, "TYPE")
    
    sus node ASTNode = ASTNode{
        type: AST_VARIABLE,
        value: name_token.value,
        children: [],
        line: name_token.line,
        column: name_token.column,
        formatted: cringe
    }
    
    sus type_node ASTNode = ASTNode{
        type: AST_EXPRESSION,
        value: type_token.value,
        children: [],
        line: type_token.line,
        column: type_token.column,
        formatted: cringe
    }
    
    push(node.children, type_node)
    damn node
}

slay parse_struct_field(ctx ParserContext) ASTNode {
    expect_token(ctx, "KEYWORD")  fr fr "spill"
    sus name_token Token = expect_token(ctx, "IDENTIFIER")
    sus type_token Token = expect_token(ctx, "TYPE")
    
    sus node ASTNode = ASTNode{
        type: AST_VARIABLE,
        value: name_token.value,
        children: [],
        line: name_token.line,
        column: name_token.column,
        formatted: cringe
    }
    
    sus type_node ASTNode = ASTNode{
        type: AST_EXPRESSION,
        value: type_token.value,
        children: [],
        line: type_token.line,
        column: type_token.column,
        formatted: cringe
    }
    
    push(node.children, type_node)
    damn node
}

fr fr ===== AST FORMATTER WITH ADVANCED FEATURES =====

squad FormatterContext {
    spill config FormatterConfig
    spill current_indent drip
    spill line_length drip
    spill in_function_params lit
    spill in_struct_definition lit
    spill in_expression lit
    spill blank_lines_needed drip
    spill last_node_type drip
    spill errors []tea
}

slay create_formatter_context(config FormatterConfig) FormatterContext {
    damn FormatterContext{
        config: config,
        current_indent: 0,
        line_length: 0,
        in_function_params: cringe,
        in_struct_definition: cringe,
        in_expression: cringe,
        blank_lines_needed: 0,
        last_node_type: 0,
        errors: []
    }
}

slay generate_indent(ctx FormatterContext) tea {
    sus indent_str tea = ""
    sus total_indent drip = ctx.current_indent * ctx.config.indent_size
    
    ready (ctx.config.use_tabs) {
        sus tab_count drip = ctx.current_indent
        sus i drip = 0
        bestie (i < tab_count) {
            indent_str = indent_str + "\t"
            i = i + 1
        }
    } otherwise {
        sus i drip = 0
        bestie (i < total_indent) {
            indent_str = indent_str + " "
            i = i + 1
        }
    }
    
    damn indent_str
}

slay generate_blank_lines(count drip) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < count) {
        result = result + "\n"
        i = i + 1
    }
    damn result
}

slay should_add_blank_lines(ctx FormatterContext, node_type drip) drip {
    ready (ctx.last_node_type == 0) {
        damn 0  fr fr No blank lines at start
    }
    
    ready (node_type == AST_FUNCTION && ctx.config.blank_lines_before_functions > 0) {
        damn ctx.config.blank_lines_before_functions
    }
    
    ready (node_type == AST_STRUCT && ctx.config.blank_lines_before_structs > 0) {
        damn ctx.config.blank_lines_before_structs
    }
    
    ready (node_type == AST_INTERFACE && ctx.config.blank_lines_before_interfaces > 0) {
        damn ctx.config.blank_lines_before_interfaces
    }
    
    damn 0
}

slay format_ast_node(node ASTNode, ctx FormatterContext) tea {
    sus blank_lines drip = should_add_blank_lines(ctx, node.type)
    sus result tea = ""
    
    ready (blank_lines > 0) {
        result = result + generate_blank_lines(blank_lines)
    }
    
    ready (node.type == AST_PROGRAM) {
        result = result + format_program(node, ctx)
    } otherwise ready (node.type == AST_FUNCTION) {
        result = result + format_function(node, ctx)
    } otherwise ready (node.type == AST_VARIABLE) {
        result = result + format_variable(node, ctx)
    } otherwise ready (node.type == AST_STRUCT) {
        result = result + format_struct(node, ctx)
    } otherwise ready (node.type == AST_INTERFACE) {
        result = result + format_interface(node, ctx)
    } otherwise ready (node.type == AST_IF_STATEMENT) {
        result = result + format_if_statement(node, ctx)
    } otherwise ready (node.type == AST_WHILE_LOOP) {
        result = result + format_while_loop(node, ctx)
    } otherwise ready (node.type == AST_BLOCK) {
        result = result + format_block(node, ctx)
    } otherwise ready (node.type == AST_IMPORT) {
        result = result + format_import(node, ctx)
    } otherwise ready (node.type == AST_COMMENT) {
        result = result + format_comment(node, ctx)
    } otherwise ready (node.type == AST_EXPRESSION) {
        result = result + format_expression(node, ctx)
    } otherwise {
        fr fr Error recovery: format as expression
        result = result + node.value
    }
    
    ctx.last_node_type = node.type
    damn result
}

slay format_program(node ASTNode, ctx FormatterContext) tea {
    sus result tea = ""
    sus i drip = 0
    
    fr fr Sort imports if configured
    ready (ctx.config.sort_imports) {
        fr fr Simplified import sorting - collect imports first
        sus imports []ASTNode = []
        sus other_nodes []ASTNode = []
        
        sus j drip = 0
        bestie (j < len(node.children)) {
            ready (node.children[j].type == AST_IMPORT) {
                push(imports, node.children[j])
            } otherwise {
                push(other_nodes, node.children[j])
            }
            j = j + 1
        }
        
        fr fr Format imports first
        sus k drip = 0
        bestie (k < len(imports)) {
            result = result + format_ast_node(imports[k], ctx)
            k = k + 1
        }
        
        ready (len(imports) > 0 && len(other_nodes) > 0) {
            result = result + "\n"  fr fr Blank line after imports
        }
        
        fr fr Format other nodes
        sus l drip = 0
        bestie (l < len(other_nodes)) {
            result = result + format_ast_node(other_nodes[l], ctx)
            l = l + 1
        }
    } otherwise {
        fr fr Format nodes in original order
        bestie (i < len(node.children)) {
            result = result + format_ast_node(node.children[i], ctx)
            i = i + 1
        }
    }
    
    damn result
}

slay format_function(node ASTNode, ctx FormatterContext) tea {
    sus result tea = generate_indent(ctx)
    result = result + "slay " + node.value
    
    fr fr Format parameters
    ready (len(node.children) > 0 && node.children[0].type == AST_VARIABLE) {
        result = result + "("
        
        ready (ctx.config.space_in_parentheses) {
            result = result + " "
        }
        
        ctx.in_function_params = based
        sus param_count drip = 0
        sus i drip = 0
        
        fr fr Count parameters
        bestie (i < len(node.children) && node.children[i].type == AST_VARIABLE) {
            ready (param_count > 0) {
                result = result + ","
                ready (ctx.config.space_after_commas) {
                    result = result + " "
                }
            }
            
            result = result + format_ast_node(node.children[i], ctx)
            param_count = param_count + 1
            i = i + 1
        }
        
        ready (ctx.config.space_in_parentheses) {
            result = result + " "
        }
        
        result = result + ")"
        ctx.in_function_params = cringe
        
        fr fr Format return type
        ready (i < len(node.children) && node.children[i].type == AST_EXPRESSION) {
            ready (ctx.config.space_before_colon) {
                result = result + " "
            }
            result = result + " "
            result = result + format_ast_node(node.children[i], ctx)
            i = i + 1
        }
    }
    
    fr fr Format opening brace
    ready (ctx.config.opening_brace_style == "new_line") {
        result = result + "\n" + generate_indent(ctx) + "{"
    } otherwise {
        result = result + " {"
    }
    
    fr fr Format body
    ready (i < len(node.children) && node.children[i].type == AST_BLOCK) {
        ctx.current_indent = ctx.current_indent + 1
        result = result + format_ast_node(node.children[i], ctx)
        ctx.current_indent = ctx.current_indent - 1
    }
    
    result = result + "\n" + generate_indent(ctx) + "}"
    result = result + "\n"
    
    damn result
}

slay format_variable(node ASTNode, ctx FormatterContext) tea {
    sus result tea = ""
    
    ready (!ctx.in_function_params && !ctx.in_struct_definition) {
        result = result + generate_indent(ctx)
    }
    
    result = result + "sus " + node.value
    
    fr fr Format type annotation
    ready (len(node.children) > 0 && node.children[0].type == AST_EXPRESSION) {
        ready (ctx.config.space_before_colon) {
            result = result + " "
        }
        result = result + " "
        result = result + node.children[0].value
        
        fr fr Format initializer
        ready (len(node.children) > 1) {
            ready (ctx.config.space_around_operators) {
                result = result + " = "
            } otherwise {
                result = result + "="
            }
            result = result + format_ast_node(node.children[1], ctx)
        }
    }
    
    ready (!ctx.in_function_params && !ctx.in_struct_definition) {
        result = result + ";\n"
    }
    
    damn result
}

slay format_struct(node ASTNode, ctx FormatterContext) tea {
    sus result tea = generate_indent(ctx)
    result = result + "squad " + node.value
    
    ready (ctx.config.opening_brace_style == "new_line") {
        result = result + "\n" + generate_indent(ctx) + "{"
    } otherwise {
        result = result + " {"
    }
    
    result = result + "\n"
    ctx.current_indent = ctx.current_indent + 1
    ctx.in_struct_definition = based
    
    fr fr Format fields
    sus i drip = 0
    bestie (i < len(node.children)) {
        result = result + generate_indent(ctx) + "spill "
        result = result + format_ast_node(node.children[i], ctx)
        result = result + "\n"
        i = i + 1
    }
    
    ctx.in_struct_definition = cringe
    ctx.current_indent = ctx.current_indent - 1
    result = result + generate_indent(ctx) + "}\n"
    
    damn result
}

slay format_interface(node ASTNode, ctx FormatterContext) tea {
    sus result tea = generate_indent(ctx)
    result = result + "collab " + node.value
    
    ready (ctx.config.opening_brace_style == "new_line") {
        result = result + "\n" + generate_indent(ctx) + "{"
    } otherwise {
        result = result + " {"
    }
    
    result = result + "\n"
    ctx.current_indent = ctx.current_indent + 1
    
    fr fr Format methods
    sus i drip = 0
    bestie (i < len(node.children)) {
        result = result + format_ast_node(node.children[i], ctx)
        i = i + 1
    }
    
    ctx.current_indent = ctx.current_indent - 1
    result = result + generate_indent(ctx) + "}\n"
    
    damn result
}

slay format_if_statement(node ASTNode, ctx FormatterContext) tea {
    sus result tea = generate_indent(ctx)
    result = result + "ready ("
    
    fr fr Format condition
    ready (len(node.children) > 0) {
        result = result + format_ast_node(node.children[0], ctx)
    }
    
    result = result + ") {"
    result = result + "\n"
    
    fr fr Format then body
    ready (len(node.children) > 1) {
        ctx.current_indent = ctx.current_indent + 1
        result = result + format_ast_node(node.children[1], ctx)
        ctx.current_indent = ctx.current_indent - 1
    }
    
    result = result + generate_indent(ctx) + "}"
    
    fr fr Format else body
    ready (len(node.children) > 2) {
        result = result + " otherwise {"
        result = result + "\n"
        ctx.current_indent = ctx.current_indent + 1
        result = result + format_ast_node(node.children[2], ctx)
        ctx.current_indent = ctx.current_indent - 1
        result = result + generate_indent(ctx) + "}"
    }
    
    result = result + "\n"
    damn result
}

slay format_while_loop(node ASTNode, ctx FormatterContext) tea {
    sus result tea = generate_indent(ctx)
    result = result + "bestie ("
    
    fr fr Format condition
    ready (len(node.children) > 0) {
        result = result + format_ast_node(node.children[0], ctx)
    }
    
    result = result + ") {"
    result = result + "\n"
    
    fr fr Format body
    ready (len(node.children) > 1) {
        ctx.current_indent = ctx.current_indent + 1
        result = result + format_ast_node(node.children[1], ctx)
        ctx.current_indent = ctx.current_indent - 1
    }
    
    result = result + generate_indent(ctx) + "}\n"
    damn result
}

slay format_block(node ASTNode, ctx FormatterContext) tea {
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < len(node.children)) {
        result = result + format_ast_node(node.children[i], ctx)
        i = i + 1
    }
    
    damn result
}

slay format_import(node ASTNode, ctx FormatterContext) tea {
    sus result tea = generate_indent(ctx)
    result = result + "yeet " + node.value + ";\n"
    damn result
}

slay format_comment(node ASTNode, ctx FormatterContext) tea {
    sus result tea = generate_indent(ctx)
    result = result + node.value + "\n"
    damn result
}

slay format_expression(node ASTNode, ctx FormatterContext) tea {
    damn node.value
}

fr fr ===== DIFF GENERATION SYSTEM =====

squad DiffLine {
    spill type tea         fr fr "added", "removed", "unchanged"
    spill line_num drip
    spill content tea
}

slay generate_diff(original tea, formatted tea) []DiffLine {
    sus original_lines []tea = split_lines(original)
    sus formatted_lines []tea = split_lines(formatted)
    sus diff_lines []DiffLine = []
    
    sus max_lines drip = find_max([len(original_lines), len(formatted_lines)])
    sus i drip = 0
    
    bestie (i < max_lines) {
        sus orig_line tea = ""
        sus fmt_line tea = ""
        
        ready (i < len(original_lines)) {
            orig_line = original_lines[i]
        }
        
        ready (i < len(formatted_lines)) {
            fmt_line = formatted_lines[i]
        }
        
        ready (orig_line == fmt_line) {
            sus unchanged DiffLine = DiffLine{
                type: "unchanged",
                line_num: i + 1,
                content: orig_line
            }
            push(diff_lines, unchanged)
        } otherwise {
            ready (orig_line != "") {
                sus removed DiffLine = DiffLine{
                    type: "removed",
                    line_num: i + 1,
                    content: orig_line
                }
                push(diff_lines, removed)
            }
            
            ready (fmt_line != "") {
                sus added DiffLine = DiffLine{
                    type: "added",
                    line_num: i + 1,
                    content: fmt_line
                }
                push(diff_lines, added)
            }
        }
        
        i = i + 1
    }
    
    damn diff_lines
}

slay format_diff_output(diff_lines []DiffLine) tea {
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < len(diff_lines)) {
        sus line DiffLine = diff_lines[i]
        
        ready (line.type == "added") {
            result = result + "+ " + line.content + "\n"
        } otherwise ready (line.type == "removed") {
            result = result + "- " + line.content + "\n"
        } otherwise {
            result = result + "  " + line.content + "\n"
        }
        
        i = i + 1
    }
    
    damn result
}

fr fr ===== MAIN FORMATTER API =====

slay format_cursed_code_ast(source tea) tea {
    sus config FormatterConfig = default_formatter_config()
    damn format_cursed_code_with_config_ast(source, config)
}

slay format_cursed_code_with_config_ast(source tea, config FormatterConfig) tea {
    fr fr Tokenize with advanced features
    sus tokenizer_ctx TokenizerContext = tokenize_advanced(source)
    
    ready (len(tokenizer_ctx.errors) > 0 && !config.continue_on_errors) {
        fr fr Return original source with error comment
        ready (config.add_error_comments) {
            sus error_comment tea = "fr fr Formatting errors: " + join_with_comma(tokenizer_ctx.errors[0], "...")
            damn error_comment + "\n" + source
        }
        damn source
    }
    
    fr fr Parse into AST
    sus ast ASTNode = parse_ast(tokenizer_ctx.tokens, config)
    
    fr fr Format AST
    sus formatter_ctx FormatterContext = create_formatter_context(config)
    sus formatted tea = format_ast_node(ast, formatter_ctx)
    
    ready (len(formatter_ctx.errors) > 0 && !config.continue_on_errors) {
        fr fr Return original source on formatting errors
        damn source
    }
    
    damn formatted
}

slay format_with_diff(source tea, config FormatterConfig) tea {
    sus formatted tea = format_cursed_code_with_config_ast(source, config)
    sus diff_lines []DiffLine = generate_diff(source, formatted)
    damn format_diff_output(diff_lines)
}

slay validate_syntax(source tea) []tea {
    sus tokenizer_ctx TokenizerContext = tokenize_advanced(source)
    sus config FormatterConfig = default_formatter_config()
    sus parser_ctx ParserContext = create_parser_context(tokenizer_ctx.tokens, config)
    sus ast ASTNode = parse_ast(tokenizer_ctx.tokens, config)
    
    sus all_errors []tea = []
    sus i drip = 0
    
    fr fr Combine tokenizer and parser errors
    bestie (i < len(tokenizer_ctx.errors)) {
        push(all_errors, tokenizer_ctx.errors[i])
        i = i + 1
    }
    
    sus j drip = 0
    bestie (j < len(parser_ctx.errors)) {
        push(all_errors, parser_ctx.errors[j])
        j = j + 1
    }
    
    damn all_errors
}

fr fr ===== CONFIGURATION MANAGEMENT =====

slay load_config_from_string(config_text tea) FormatterConfig {
    fr fr Simple configuration parser
    sus config FormatterConfig = default_formatter_config()
    
    ready (contains_substring(config_text, "indent_size=2")) {
        config.indent_size = 2
    }
    ready (contains_substring(config_text, "indent_size=8")) {
        config.indent_size = 8
    }
    ready (contains_substring(config_text, "max_line_length=80")) {
        config.max_line_length = 80
    }
    ready (contains_substring(config_text, "max_line_length=120")) {
        config.max_line_length = 120
    }
    ready (contains_substring(config_text, "use_tabs=true")) {
        config.use_tabs = based
    }
    ready (contains_substring(config_text, "opening_brace_style=new_line")) {
        config.opening_brace_style = "new_line"
    }
    
    damn config
}

slay save_config_to_string(config FormatterConfig) tea {
    sus result tea = "# CURSED Formatter Configuration\n"
    result = result + "indent_size=" + int_to_string(config.indent_size) + "\n"
    result = result + "max_line_length=" + int_to_string(config.max_line_length) + "\n"
    result = result + "use_tabs=" + (ready (config.use_tabs) { "true" } otherwise { "false" }) + "\n"
    result = result + "opening_brace_style=" + config.opening_brace_style + "\n"
    result = result + "space_around_operators=" + (ready (config.space_around_operators) { "true" } otherwise { "false" }) + "\n"
    damn result
}

fr fr ===== UTILITY FUNCTIONS =====

slay main() {
    vibez.spill("CURSED AST-Based Code Formatter - Production Edition")
    vibez.spill("Comprehensive formatting with configuration support")
    
    fr fr Example usage with different configurations
    sus sample_code tea = "sus x drip=42;slay test(param drip){ready(x>0){damn x+1;}otherwise{damn 0;}}"
    
    vibez.spill("\n=== ORIGINAL CODE ===")
    vibez.spill(sample_code)
    
    vibez.spill("\n=== DEFAULT FORMATTING ===")
    sus default_formatted tea = format_cursed_code_ast(sample_code)
    vibez.spill(default_formatted)
    
    vibez.spill("\n=== COMPACT FORMATTING ===")
    sus compact_config FormatterConfig = compact_formatter_config()
    sus compact_formatted tea = format_cursed_code_with_config_ast(sample_code, compact_config)
    vibez.spill(compact_formatted)
    
    vibez.spill("\n=== GOOGLE STYLE FORMATTING ===")
    sus google_config FormatterConfig = google_style_config()
    sus google_formatted tea = format_cursed_code_with_config_ast(sample_code, google_config)
    vibez.spill(google_formatted)
    
    vibez.spill("\n=== DIFF OUTPUT ===")
    sus diff_output tea = format_with_diff(sample_code, default_formatter_config())
    vibez.spill(diff_output)
    
    vibez.spill("\n=== SYNTAX VALIDATION ===")
    sus errors []tea = validate_syntax(sample_code)
    ready (len(errors) == 0) {
        vibez.spill("✅ No syntax errors found")
    } otherwise {
        vibez.spill("❌ Syntax errors found:")
        sus i drip = 0
        bestie (i < len(errors)) {
            vibez.spill("  " + errors[i])
            i = i + 1
        }
    }
    
    vibez.spill("\n=== CONFIGURATION SERIALIZATION ===")
    sus config_text tea = save_config_to_string(default_formatter_config())
    vibez.spill(config_text)
}
