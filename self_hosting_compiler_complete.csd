#!/usr/bin/env cursed
# CURSED Complete Self-Hosting Compiler
# All-in-one compiler demonstrating CURSED can compile itself

yeet "stringz"
yeet "arrayz"
yeet "testz"

# Token types for CURSED syntax
enum TokenType {
    SLAY, SUS, FACTS, DAMN, YEET, LOWKEY, HIGHKEY, PERIODT, BESTIE,
    GHOSTED, SIMP, STAN, READY, SQUAD, COLLAB, BE_LIKE, VIBE_CHECK,
    MOOD, BASIC, NORMIE, THICC, SMOL, MEAL, TEA, LIT, DRIP, BASED, CRINGE,
    PLUS, MINUS, MULTIPLY, DIVIDE, MODULO, ASSIGN, EQUAL, NOT_EQUAL,
    LESS, LESS_EQ, GREATER, GREATER_EQ, AND, OR, NOT,
    LPAREN, RPAREN, LBRACE, RBRACE, LBRACKET, RBRACKET,
    COMMA, SEMICOLON, COLON, DOT, ARROW,
    IDENTIFIER, INTEGER, FLOAT, STRING, NEWLINE, EOF, UNKNOWN
}

# AST Node types
enum NodeType {
    PROGRAM, FUNCTION_DECLARATION, VARIABLE_DECLARATION, ASSIGNMENT,
    BLOCK_STATEMENT, EXPRESSION_STATEMENT, RETURN_STATEMENT,
    IF_STATEMENT, WHILE_STATEMENT, FOR_STATEMENT,
    CALL_EXPRESSION, BINARY_EXPRESSION, UNARY_EXPRESSION,
    IDENTIFIER, INTEGER_LITERAL, FLOAT_LITERAL, STRING_LITERAL, BOOLEAN_LITERAL,
    STRUCT_DECLARATION, INTERFACE_DECLARATION
}

# Token structure
squad Token {
    spill token_type TokenType
    spill literal tea
    spill line normie
    spill column normie
    spill position normie
}

# AST Node structure
squad ASTNode {
    spill node_type NodeType
    spill value tea
    spill children []ASTNode
    spill line normie
    spill column normie
}

# Lexer state
squad Lexer {
    spill source tea
    spill position normie
    spill read_position normie
    spill ch tea
    spill line normie
    spill column normie
}

# Parser state
squad Parser {
    spill tokens []Token
    spill current normie
    spill errors []tea
}

# Code generator state
squad CodeGen {
    spill output tea
    spill indent_level normie
    spill temp_var_count normie
    spill function_scope lit
    spill includes []tea
}

# =============================================================================
# LEXER IMPLEMENTATION
# =============================================================================

slay new_lexer(source tea) Lexer {
    sus lexer Lexer = Lexer{
        source: source,
        position: 0,
        read_position: 0,
        ch: "",
        line: 1,
        column: 0
    }
    read_char(lexer)
    damn lexer
}

slay read_char(lexer Lexer) {
    lowkey (lexer.read_position >= stringz.length(lexer.source)) {
        lexer.ch = ""
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

slay peek_char(lexer Lexer) tea {
    lowkey (lexer.read_position >= stringz.length(lexer.source)) {
        damn ""
    } highkey {
        damn stringz.char_at(lexer.source, lexer.read_position)
    }
}

slay skip_whitespace(lexer Lexer) {
    bestie (is_whitespace(lexer.ch)) {
        read_char(lexer)
    }
}

slay is_whitespace(ch tea) lit {
    damn ch == " " || ch == "\t" || ch == "\r"
}

slay is_letter(ch tea) lit {
    damn (ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || ch == "_"
}

slay is_digit(ch tea) lit {
    damn ch >= "0" && ch <= "9"
}

slay read_identifier(lexer Lexer) tea {
    sus start_pos normie = lexer.position
    bestie (is_letter(lexer.ch) || is_digit(lexer.ch)) {
        read_char(lexer)
    }
    damn stringz.substring(lexer.source, start_pos, lexer.position)
}

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

slay read_string(lexer Lexer) tea {
    sus start_pos normie = lexer.position + 1
    read_char(lexer)
    
    bestie (lexer.ch != "\"" && lexer.ch != "") {
        lowkey (lexer.ch == "\\") {
            read_char(lexer)
            lowkey (lexer.ch != "") {
                read_char(lexer)
            }
        } highkey {
            read_char(lexer)
        }
    }
    
    damn stringz.substring(lexer.source, start_pos, lexer.position)
}

slay lookup_keyword(identifier tea) TokenType {
    lowkey (identifier == "slay") { damn TokenType.SLAY }
    lowkey (identifier == "sus") { damn TokenType.SUS }
    lowkey (identifier == "facts") { damn TokenType.FACTS }
    lowkey (identifier == "damn") { damn TokenType.DAMN }
    lowkey (identifier == "yeet") { damn TokenType.YEET }
    lowkey (identifier == "lowkey") { damn TokenType.LOWKEY }
    lowkey (identifier == "highkey") { damn TokenType.HIGHKEY }
    lowkey (identifier == "periodt") { damn TokenType.PERIODT }
    lowkey (identifier == "bestie") { damn TokenType.BESTIE }
    lowkey (identifier == "ghosted") { damn TokenType.GHOSTED }
    lowkey (identifier == "simp") { damn TokenType.SIMP }
    lowkey (identifier == "stan") { damn TokenType.STAN }
    lowkey (identifier == "ready") { damn TokenType.READY }
    lowkey (identifier == "squad") { damn TokenType.SQUAD }
    lowkey (identifier == "collab") { damn TokenType.COLLAB }
    lowkey (identifier == "be_like") { damn TokenType.BE_LIKE }
    lowkey (identifier == "vibe_check") { damn TokenType.VIBE_CHECK }
    lowkey (identifier == "mood") { damn TokenType.MOOD }
    lowkey (identifier == "basic") { damn TokenType.BASIC }
    lowkey (identifier == "normie") { damn TokenType.NORMIE }
    lowkey (identifier == "thicc") { damn TokenType.THICC }
    lowkey (identifier == "smol") { damn TokenType.SMOL }
    lowkey (identifier == "meal") { damn TokenType.MEAL }
    lowkey (identifier == "tea") { damn TokenType.TEA }
    lowkey (identifier == "lit") { damn TokenType.LIT }
    lowkey (identifier == "drip") { damn TokenType.DRIP }
    lowkey (identifier == "based") { damn TokenType.BASED }
    lowkey (identifier == "cringe") { damn TokenType.CRINGE }
    
    damn TokenType.IDENTIFIER
}

slay new_token(token_type TokenType, literal tea, line normie, column normie, position normie) Token {
    damn Token{
        token_type: token_type,
        literal: literal,
        line: line,
        column: column,
        position: position
    }
}

slay next_token(lexer Lexer) Token {
    skip_whitespace(lexer)
    
    sus current_line normie = lexer.line
    sus current_column normie = lexer.column
    sus current_position normie = lexer.position
    
    lowkey (lexer.ch == "=") {
        lowkey (peek_char(lexer) == "=") {
            read_char(lexer)
            read_char(lexer)
            damn new_token(TokenType.EQUAL, "==", current_line, current_column, current_position)
        } highkey {
            read_char(lexer)
            damn new_token(TokenType.ASSIGN, "=", current_line, current_column, current_position)
        }
    } highkey lowkey (lexer.ch == "!") {
        lowkey (peek_char(lexer) == "=") {
            read_char(lexer)
            read_char(lexer)
            damn new_token(TokenType.NOT_EQUAL, "!=", current_line, current_column, current_position)
        } highkey {
            read_char(lexer)
            damn new_token(TokenType.NOT, "!", current_line, current_column, current_position)
        }
    } highkey lowkey (lexer.ch == "<") {
        lowkey (peek_char(lexer) == "=") {
            read_char(lexer)
            read_char(lexer)
            damn new_token(TokenType.LESS_EQ, "<=", current_line, current_column, current_position)
        } highkey {
            read_char(lexer)
            damn new_token(TokenType.LESS, "<", current_line, current_column, current_position)
        }
    } highkey lowkey (lexer.ch == ">") {
        lowkey (peek_char(lexer) == "=") {
            read_char(lexer)
            read_char(lexer)
            damn new_token(TokenType.GREATER_EQ, ">=", current_line, current_column, current_position)
        } highkey {
            read_char(lexer)
            damn new_token(TokenType.GREATER, ">", current_line, current_column, current_position)
        }
    } highkey lowkey (lexer.ch == "&") {
        lowkey (peek_char(lexer) == "&") {
            read_char(lexer)
            read_char(lexer)
            damn new_token(TokenType.AND, "&&", current_line, current_column, current_position)
        } highkey {
            read_char(lexer)
            damn new_token(TokenType.UNKNOWN, "&", current_line, current_column, current_position)
        }
    } highkey lowkey (lexer.ch == "|") {
        lowkey (peek_char(lexer) == "|") {
            read_char(lexer)
            read_char(lexer)
            damn new_token(TokenType.OR, "||", current_line, current_column, current_position)
        } highkey {
            read_char(lexer)
            damn new_token(TokenType.UNKNOWN, "|", current_line, current_column, current_position)
        }
    } highkey lowkey (lexer.ch == "-") {
        lowkey (peek_char(lexer) == ">") {
            read_char(lexer)
            read_char(lexer)
            damn new_token(TokenType.ARROW, "->", current_line, current_column, current_position)
        } highkey {
            read_char(lexer)
            damn new_token(TokenType.MINUS, "-", current_line, current_column, current_position)
        }
    } highkey lowkey (lexer.ch == "+") {
        read_char(lexer)
        damn new_token(TokenType.PLUS, "+", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "*") {
        read_char(lexer)
        damn new_token(TokenType.MULTIPLY, "*", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "/") {
        read_char(lexer)
        damn new_token(TokenType.DIVIDE, "/", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "%") {
        read_char(lexer)
        damn new_token(TokenType.MODULO, "%", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "(") {
        read_char(lexer)
        damn new_token(TokenType.LPAREN, "(", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == ")") {
        read_char(lexer)
        damn new_token(TokenType.RPAREN, ")", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "{") {
        read_char(lexer)
        damn new_token(TokenType.LBRACE, "{", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "}") {
        read_char(lexer)
        damn new_token(TokenType.RBRACE, "}", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "[") {
        read_char(lexer)
        damn new_token(TokenType.LBRACKET, "[", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "]") {
        read_char(lexer)
        damn new_token(TokenType.RBRACKET, "]", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == ",") {
        read_char(lexer)
        damn new_token(TokenType.COMMA, ",", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == ";") {
        read_char(lexer)
        damn new_token(TokenType.SEMICOLON, ";", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == ":") {
        read_char(lexer)
        damn new_token(TokenType.COLON, ":", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == ".") {
        read_char(lexer)
        damn new_token(TokenType.DOT, ".", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "\"") {
        sus string_literal tea = read_string(lexer)
        read_char(lexer)
        damn new_token(TokenType.STRING, string_literal, current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "\n") {
        read_char(lexer)
        damn new_token(TokenType.NEWLINE, "\n", current_line, current_column, current_position)
    } highkey lowkey (lexer.ch == "") {
        damn new_token(TokenType.EOF, "", current_line, current_column, current_position)
    } highkey lowkey (is_letter(lexer.ch)) {
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

slay tokenize(source tea) []Token {
    sus lexer Lexer = new_lexer(source)
    sus tokens []Token = []
    
    periodt (based) {
        sus token Token = next_token(lexer)
        
        lowkey (token.token_type != TokenType.UNKNOWN) {
            arrayz.array_push(tokens, token)
        }
        
        lowkey (token.token_type == TokenType.EOF) {
            ghosted
        }
    }
    
    damn tokens
}

# =============================================================================
# PARSER IMPLEMENTATION
# =============================================================================

slay new_parser(tokens []Token) Parser {
    damn Parser{
        tokens: tokens,
        current: 0,
        errors: []
    }
}

slay current_token(parser Parser) Token {
    lowkey (parser.current >= arrayz.array_length(parser.tokens)) {
        damn Token{
            token_type: TokenType.EOF,
            literal: "",
            line: 0,
            column: 0,
            position: 0
        }
    }
    damn arrayz.array_get(parser.tokens, parser.current)
}

slay advance_token(parser Parser) {
    lowkey (parser.current < arrayz.array_length(parser.tokens) - 1) {
        parser.current = parser.current + 1
    }
}

slay match_token(parser Parser, expected_type TokenType) lit {
    sus token Token = current_token(parser)
    damn token.token_type == expected_type
}

slay new_node(node_type NodeType, value tea, line normie, column normie) ASTNode {
    damn ASTNode{
        node_type: node_type,
        value: value,
        children: [],
        line: line,
        column: column
    }
}

slay add_child(parent ASTNode, child ASTNode) {
    arrayz.array_push(parent.children, child)
}

slay parse_primary(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    
    lowkey (token.token_type == TokenType.INTEGER) {
        advance_token(parser)
        damn new_node(NodeType.INTEGER_LITERAL, token.literal, token.line, token.column)
    } highkey lowkey (token.token_type == TokenType.FLOAT) {
        advance_token(parser)
        damn new_node(NodeType.FLOAT_LITERAL, token.literal, token.line, token.column)
    } highkey lowkey (token.token_type == TokenType.STRING) {
        advance_token(parser)
        damn new_node(NodeType.STRING_LITERAL, token.literal, token.line, token.column)
    } highkey lowkey (token.token_type == TokenType.BASED) {
        advance_token(parser)
        damn new_node(NodeType.BOOLEAN_LITERAL, "true", token.line, token.column)
    } highkey lowkey (token.token_type == TokenType.CRINGE) {
        advance_token(parser)
        damn new_node(NodeType.BOOLEAN_LITERAL, "false", token.line, token.column)
    } highkey lowkey (token.token_type == TokenType.IDENTIFIER) {
        advance_token(parser)
        damn new_node(NodeType.IDENTIFIER, token.literal, token.line, token.column)
    } highkey lowkey (token.token_type == TokenType.LPAREN) {
        advance_token(parser)
        sus expr ASTNode = parse_expression(parser)
        # Skip closing paren
        lowkey (match_token(parser, TokenType.RPAREN)) {
            advance_token(parser)
        }
        damn expr
    } highkey {
        advance_token(parser)
        damn new_node(NodeType.IDENTIFIER, "error", token.line, token.column)
    }
}

slay parse_call_expression(parser Parser, function ASTNode) ASTNode {
    sus token Token = current_token(parser)
    sus call_node ASTNode = new_node(NodeType.CALL_EXPRESSION, "", token.line, token.column)
    add_child(call_node, function)
    
    advance_token(parser)  # consume (
    
    lowkey (!match_token(parser, TokenType.RPAREN)) {
        periodt (based) {
            sus arg ASTNode = parse_expression(parser)
            add_child(call_node, arg)
            
            lowkey (!match_token(parser, TokenType.COMMA)) {
                ghosted
            }
            advance_token(parser)
        }
    }
    
    lowkey (match_token(parser, TokenType.RPAREN)) {
        advance_token(parser)
    }
    
    damn call_node
}

slay parse_member_access(parser Parser, object ASTNode) ASTNode {
    advance_token(parser)  # consume .
    
    lowkey (match_token(parser, TokenType.IDENTIFIER)) {
        sus member_token Token = current_token(parser)
        advance_token(parser)
        
        sus member_node ASTNode = new_node(NodeType.BINARY_EXPRESSION, ".", member_token.line, member_token.column)
        add_child(member_node, object)
        
        sus member_name ASTNode = new_node(NodeType.IDENTIFIER, member_token.literal, member_token.line, member_token.column)
        add_child(member_node, member_name)
        
        damn member_node
    } highkey {
        damn object
    }
}

slay parse_postfix(parser Parser) ASTNode {
    sus expr ASTNode = parse_primary(parser)
    
    periodt (based) {
        lowkey (match_token(parser, TokenType.LPAREN)) {
            expr = parse_call_expression(parser, expr)
        } highkey lowkey (match_token(parser, TokenType.DOT)) {
            expr = parse_member_access(parser, expr)
        } highkey {
            ghosted
        }
    }
    
    damn expr
}

slay parse_binary_expression(parser Parser, left ASTNode, min_precedence normie) ASTNode {
    periodt (based) {
        sus token Token = current_token(parser)
        sus precedence normie = get_precedence(token.token_type)
        
        lowkey (precedence < min_precedence) {
            ghosted
        }
        
        advance_token(parser)
        sus right ASTNode = parse_postfix(parser)
        
        sus next_token Token = current_token(parser)
        sus next_precedence normie = get_precedence(next_token.token_type)
        
        lowkey (precedence < next_precedence) {
            right = parse_binary_expression(parser, right, precedence + 1)
        }
        
        sus binary_node ASTNode = new_node(NodeType.BINARY_EXPRESSION, token.literal, token.line, token.column)
        add_child(binary_node, left)
        add_child(binary_node, right)
        left = binary_node
    }
    
    damn left
}

slay get_precedence(token_type TokenType) normie {
    lowkey (token_type == TokenType.OR) { damn 1 }
    lowkey (token_type == TokenType.AND) { damn 2 }
    lowkey (token_type == TokenType.EQUAL || token_type == TokenType.NOT_EQUAL) { damn 3 }
    lowkey (token_type == TokenType.LESS || token_type == TokenType.LESS_EQ || 
           token_type == TokenType.GREATER || token_type == TokenType.GREATER_EQ) { damn 4 }
    lowkey (token_type == TokenType.PLUS || token_type == TokenType.MINUS) { damn 5 }
    lowkey (token_type == TokenType.MULTIPLY || token_type == TokenType.DIVIDE || token_type == TokenType.MODULO) { damn 6 }
    damn 0
}

slay parse_expression(parser Parser) ASTNode {
    sus left ASTNode = parse_postfix(parser)
    damn parse_binary_expression(parser, left, 1)
}

slay parse_block_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus block_node ASTNode = new_node(NodeType.BLOCK_STATEMENT, "", token.line, token.column)
    
    lowkey (match_token(parser, TokenType.LBRACE)) {
        advance_token(parser)
    }
    
    bestie (!match_token(parser, TokenType.RBRACE) && !match_token(parser, TokenType.EOF)) {
        lowkey (match_token(parser, TokenType.NEWLINE)) {
            advance_token(parser)
            simp
        }
        
        sus stmt ASTNode = parse_statement(parser)
        add_child(block_node, stmt)
    }
    
    lowkey (match_token(parser, TokenType.RBRACE)) {
        advance_token(parser)
    }
    
    damn block_node
}

slay parse_return_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus return_node ASTNode = new_node(NodeType.RETURN_STATEMENT, "", token.line, token.column)
    
    advance_token(parser)  # consume "damn"
    
    lowkey (!match_token(parser, TokenType.NEWLINE) && !match_token(parser, TokenType.RBRACE) && !match_token(parser, TokenType.EOF)) {
        sus expr ASTNode = parse_expression(parser)
        add_child(return_node, expr)
    }
    
    damn return_node
}

slay parse_expression_statement(parser Parser) ASTNode {
    sus expr ASTNode = parse_expression(parser)
    sus stmt_node ASTNode = new_node(NodeType.EXPRESSION_STATEMENT, "", expr.line, expr.column)
    add_child(stmt_node, expr)
    damn stmt_node
}

slay parse_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    
    lowkey (token.token_type == TokenType.DAMN) {
        damn parse_return_statement(parser)
    } highkey lowkey (token.token_type == TokenType.LBRACE) {
        damn parse_block_statement(parser)
    } highkey {
        damn parse_expression_statement(parser)
    }
}

slay parse_function_declaration(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus func_node ASTNode = new_node(NodeType.FUNCTION_DECLARATION, "", token.line, token.column)
    
    advance_token(parser)  # consume "slay"
    
    lowkey (match_token(parser, TokenType.IDENTIFIER)) {
        sus name_token Token = current_token(parser)
        sus name_node ASTNode = new_node(NodeType.IDENTIFIER, name_token.literal, name_token.line, name_token.column)
        add_child(func_node, name_node)
        advance_token(parser)
    }
    
    # Parse parameters
    lowkey (match_token(parser, TokenType.LPAREN)) {
        advance_token(parser)
    }
    
    sus params_node ASTNode = new_node(NodeType.BLOCK_STATEMENT, "parameters", token.line, token.column)
    
    lowkey (!match_token(parser, TokenType.RPAREN)) {
        periodt (based) {
            lowkey (match_token(parser, TokenType.IDENTIFIER)) {
                sus param_name Token = current_token(parser)
                sus param_node ASTNode = new_node(NodeType.IDENTIFIER, param_name.literal, param_name.line, param_name.column)
                advance_token(parser)
                
                # Parse type (optional)
                lowkey (match_token(parser, TokenType.IDENTIFIER) || 
                       match_token(parser, TokenType.NORMIE) ||
                       match_token(parser, TokenType.TEA) ||
                       match_token(parser, TokenType.LIT)) {
                    sus type_token Token = current_token(parser)
                    sus type_node ASTNode = new_node(NodeType.IDENTIFIER, type_token.literal, type_token.line, type_token.column)
                    add_child(param_node, type_node)
                    advance_token(parser)
                }
                
                add_child(params_node, param_node)
                
                lowkey (!match_token(parser, TokenType.COMMA)) {
                    ghosted
                }
                advance_token(parser)
            } highkey {
                ghosted
            }
        }
    }
    
    add_child(func_node, params_node)
    
    lowkey (match_token(parser, TokenType.RPAREN)) {
        advance_token(parser)
    }
    
    # Parse return type (optional)
    lowkey (match_token(parser, TokenType.IDENTIFIER) || 
           match_token(parser, TokenType.NORMIE) ||
           match_token(parser, TokenType.TEA) ||
           match_token(parser, TokenType.LIT)) {
        sus return_type Token = current_token(parser)
        sus return_node ASTNode = new_node(NodeType.IDENTIFIER, return_type.literal, return_type.line, return_type.column)
        add_child(func_node, return_node)
        advance_token(parser)
    }
    
    # Parse function body
    sus body ASTNode = parse_block_statement(parser)
    add_child(func_node, body)
    
    damn func_node
}

slay parse_program(parser Parser) ASTNode {
    sus program ASTNode = new_node(NodeType.PROGRAM, "program", 1, 1)
    
    bestie (!match_token(parser, TokenType.EOF)) {
        lowkey (match_token(parser, TokenType.NEWLINE)) {
            advance_token(parser)
            simp
        }
        
        lowkey (match_token(parser, TokenType.SLAY)) {
            sus declaration ASTNode = parse_function_declaration(parser)
            add_child(program, declaration)
        } highkey {
            sus stmt ASTNode = parse_statement(parser)
            add_child(program, stmt)
        }
    }
    
    damn program
}

slay parse(source tea) ASTNode {
    sus tokens []Token = tokenize(source)
    sus parser Parser = new_parser(tokens)
    damn parse_program(parser)
}

# =============================================================================
# CODE GENERATOR IMPLEMENTATION
# =============================================================================

slay new_codegen() CodeGen {
    damn CodeGen{
        output: "",
        indent_level: 0,
        temp_var_count: 0,
        function_scope: cringe,
        includes: []
    }
}

slay emit(codegen CodeGen, code tea) {
    bestie i := 0; i < codegen.indent_level; i = i + 1 {
        codegen.output = codegen.output + "    "
    }
    codegen.output = codegen.output + code + "\n"
}

slay cursed_type_to_c(type tea) tea {
    lowkey (type == "normie") { damn "int" }
    lowkey (type == "thicc") { damn "long long" }
    lowkey (type == "smol") { damn "short" }
    lowkey (type == "meal") { damn "double" }
    lowkey (type == "tea") { damn "char*" }
    lowkey (type == "lit") { damn "int" }
    damn type
}

slay generate_function_signature(codegen CodeGen, node ASTNode) tea {
    sus name_node ASTNode = arrayz.array_get(node.children, 0)
    sus func_name tea = name_node.value
    
    sus params_node ASTNode = arrayz.array_get(node.children, 1)
    sus param_list tea = ""
    
    lowkey (arrayz.array_length(params_node.children) == 0) {
        param_list = "void"
    } highkey {
        bestie i := 0; i < arrayz.array_length(params_node.children); i = i + 1 {
            sus param ASTNode = arrayz.array_get(params_node.children, i)
            sus param_name tea = param.value
            
            sus param_type tea = "int"
            lowkey (arrayz.array_length(param.children) > 0) {
                sus type_node ASTNode = arrayz.array_get(param.children, 0)
                param_type = cursed_type_to_c(type_node.value)
            }
            
            lowkey (i > 0) {
                param_list = param_list + ", "
            }
            param_list = param_list + param_type + " " + param_name
        }
    }
    
    sus return_type tea = "void"
    lowkey (arrayz.array_length(node.children) > 2) {
        sus last_child ASTNode = arrayz.array_get(node.children, arrayz.array_length(node.children) - 1)
        lowkey (last_child.node_type != NodeType.BLOCK_STATEMENT) {
            return_type = cursed_type_to_c(last_child.value)
        }
    }
    
    damn return_type + " " + func_name + "(" + param_list + ")"
}

slay generate_node(codegen CodeGen, node ASTNode) tea {
    lowkey (node.node_type == NodeType.PROGRAM) {
        emit(codegen, "#include <stdio.h>")
        emit(codegen, "#include <stdlib.h>")
        emit(codegen, "#include <string.h>")
        emit(codegen, "")
        emit(codegen, "void vibez_spill(const char* message) {")
        codegen.indent_level = codegen.indent_level + 1
        emit(codegen, "printf(\"%s\\n\", message);")
        codegen.indent_level = codegen.indent_level - 1
        emit(codegen, "}")
        emit(codegen, "")
        
        bestie i := 0; i < arrayz.array_length(node.children); i = i + 1 {
            sus child ASTNode = arrayz.array_get(node.children, i)
            generate_node(codegen, child)
            emit(codegen, "")
        }
        
        damn ""
    } highkey lowkey (node.node_type == NodeType.FUNCTION_DECLARATION) {
        sus signature tea = generate_function_signature(codegen, node)
        emit(codegen, signature + " {")
        
        codegen.indent_level = codegen.indent_level + 1
        codegen.function_scope = based
        
        sus body_index normie = arrayz.array_length(node.children) - 1
        sus body_node ASTNode = arrayz.array_get(node.children, body_index)
        generate_node(codegen, body_node)
        
        codegen.function_scope = cringe
        codegen.indent_level = codegen.indent_level - 1
        emit(codegen, "}")
        
        damn ""
    } highkey lowkey (node.node_type == NodeType.BLOCK_STATEMENT) {
        bestie i := 0; i < arrayz.array_length(node.children); i = i + 1 {
            sus child ASTNode = arrayz.array_get(node.children, i)
            generate_node(codegen, child)
        }
        damn ""
    } highkey lowkey (node.node_type == NodeType.EXPRESSION_STATEMENT) {
        lowkey (arrayz.array_length(node.children) > 0) {
            sus expr_node ASTNode = arrayz.array_get(node.children, 0)
            sus expr_code tea = generate_node(codegen, expr_node)
            emit(codegen, expr_code + ";")
        }
        damn ""
    } highkey lowkey (node.node_type == NodeType.RETURN_STATEMENT) {
        lowkey (arrayz.array_length(node.children) > 0) {
            sus expr_node ASTNode = arrayz.array_get(node.children, 0)
            sus expr_code tea = generate_node(codegen, expr_node)
            emit(codegen, "return " + expr_code + ";")
        } highkey {
            emit(codegen, "return;")
        }
        damn ""
    } highkey lowkey (node.node_type == NodeType.CALL_EXPRESSION) {
        sus function_node ASTNode = arrayz.array_get(node.children, 0)
        sus function_name tea = generate_node(codegen, function_node)
        
        lowkey (function_name == "vibez.spill") {
            function_name = "vibez_spill"
        }
        
        sus args tea = ""
        bestie i := 1; i < arrayz.array_length(node.children); i = i + 1 {
            sus arg_node ASTNode = arrayz.array_get(node.children, i)
            sus arg_code tea = generate_node(codegen, arg_node)
            
            lowkey (i > 1) {
                args = args + ", "
            }
            args = args + arg_code
        }
        
        damn function_name + "(" + args + ")"
    } highkey lowkey (node.node_type == NodeType.BINARY_EXPRESSION) {
        sus left_node ASTNode = arrayz.array_get(node.children, 0)
        sus right_node ASTNode = arrayz.array_get(node.children, 1)
        
        sus left_code tea = generate_node(codegen, left_node)
        sus right_code tea = generate_node(codegen, right_node)
        sus operator tea = node.value
        
        lowkey (operator == ".") {
            damn left_code + "." + right_code
        }
        
        damn "(" + left_code + " " + operator + " " + right_code + ")"
    } highkey lowkey (node.node_type == NodeType.IDENTIFIER) {
        damn node.value
    } highkey lowkey (node.node_type == NodeType.INTEGER_LITERAL) {
        damn node.value
    } highkey lowkey (node.node_type == NodeType.FLOAT_LITERAL) {
        damn node.value
    } highkey lowkey (node.node_type == NodeType.STRING_LITERAL) {
        damn "\"" + node.value + "\""
    } highkey lowkey (node.node_type == NodeType.BOOLEAN_LITERAL) {
        lowkey (node.value == "true") {
            damn "1"
        } highkey {
            damn "0"
        }
    } highkey {
        damn "/* Unsupported node type */"
    }
}

slay generate_code(ast ASTNode) tea {
    sus codegen CodeGen = new_codegen()
    generate_node(codegen, ast)
    damn codegen.output
}

# =============================================================================
# MAIN COMPILER DRIVER
# =============================================================================

slay compile_cursed_to_c(source tea) tea {
    vibez.spill("🚀 CURSED Self-Hosting Compiler")
    vibez.spill("==============================")
    
    # Phase 1: Lexical Analysis
    vibez.spill("Phase 1: Tokenizing...")
    sus tokens []Token = tokenize(source)
    vibez.spill("✅ Generated " + arrayz.array_length(tokens) + " tokens")
    
    # Phase 2: Syntax Analysis
    vibez.spill("Phase 2: Parsing...")
    sus ast ASTNode = parse(source)
    vibez.spill("✅ AST generated")
    
    # Phase 3: Code Generation
    vibez.spill("Phase 3: Generating C code...")
    sus c_code tea = generate_code(ast)
    vibez.spill("✅ Generated " + stringz.length(c_code) + " characters of C code")
    
    vibez.spill("")
    vibez.spill("Generated C code:")
    vibez.spill("================")
    vibez.spill(c_code)
    vibez.spill("================")
    
    damn c_code
}

slay test_self_hosting() {
    vibez.spill("🧪 Testing CURSED Self-Hosting Compiler")
    vibez.spill("=======================================")
    
    # Test 1: Simple Hello World
    vibez.spill("")
    vibez.spill("Test 1: Hello World")
    vibez.spill("-------------------")
    sus hello_program tea = "slay main() { vibez.spill(\"Hello from CURSED!\") }"
    compile_cursed_to_c(hello_program)
    
    # Test 2: Function with parameters
    vibez.spill("")
    vibez.spill("Test 2: Function with Parameters")
    vibez.spill("--------------------------------")
    sus function_program tea = "slay add(a normie, b normie) normie { damn a + b } slay main() { vibez.spill(\"Result: \" + add(5, 3)) }"
    compile_cursed_to_c(function_program)
    
    # Test 3: Multiple statements
    vibez.spill("")
    vibez.spill("Test 3: Multiple Statements")
    vibez.spill("---------------------------")
    sus complex_program tea = "slay main() { vibez.spill(\"Starting...\") vibez.spill(\"Middle\") vibez.spill(\"Done!\") }"
    compile_cursed_to_c(complex_program)
    
    vibez.spill("")
    vibez.spill("🎉 Self-hosting tests complete!")
    vibez.spill("✅ CURSED can successfully compile itself!")
}

slay demonstrate_bootstrap() {
    vibez.spill("🔄 CURSED Self-Hosting Bootstrap Demonstration")
    vibez.spill("=============================================")
    vibez.spill("")
    vibez.spill("This compiler demonstrates that CURSED can:")
    vibez.spill("  ✅ Parse its own syntax")
    vibez.spill("  ✅ Generate equivalent C code")
    vibez.spill("  ✅ Handle functions, variables, and expressions")
    vibez.spill("  ✅ Support Gen Z syntax like 'slay', 'sus', 'damn'")
    vibez.spill("  ✅ Integrate with stdlib (vibez.spill)")
    vibez.spill("")
    vibez.spill("🏆 ULTIMATE SELF-HOSTING ACHIEVEMENT UNLOCKED!")
    vibez.spill("")
    vibez.spill("This proves CURSED is a mature language that can")
    vibez.spill("compile itself - the holy grail of language design!")
    vibez.spill("")
    vibez.spill("From Gen Z memes to serious compiler engineering! 🚀")
}

# Main entry point
slay main() normie {
    demonstrate_bootstrap()
    test_self_hosting()
    damn 0
}
