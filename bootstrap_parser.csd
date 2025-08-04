fr fr Bootstrap Parser Implementation in Pure CURSED
fr fr Parses CURSED source code into an Abstract Syntax Tree

fr fr Token types
sus TOKEN_IDENTIFIER normie = 1
sus TOKEN_NUMBER normie = 2  
sus TOKEN_STRING normie = 3
sus TOKEN_KEYWORD normie = 4
sus TOKEN_OPERATOR normie = 5
sus TOKEN_EOF normie = 6

fr fr AST Node types
sus AST_PROGRAM normie = 100
sus AST_FUNCTION normie = 101
sus AST_VARIABLE normie = 102
sus AST_ASSIGNMENT normie = 103
sus AST_CALL normie = 104
sus AST_BLOCK normie = 105
sus AST_RETURN normie = 106

fr fr Token structure
squad Token {
    spill token_type normie
    spill literal tea
    spill line normie
    spill column normie
}

fr fr AST Node structure
squad ASTNode {
    spill node_type normie
    spill value tea
    spill children []ASTNode
    spill line normie
    spill column normie
}

fr fr Parser state
squad Parser {
    spill tokens []Token
    spill current normie
    spill current_token Token
}

fr fr Create new parser
slay new_parser(tokens []Token) Parser {
    sus parser Parser = Parser{
        tokens: tokens,
        current: 0,
        current_token: tokens[0]
    }
    damn parser
}

fr fr Advance to next token
slay advance(parser Parser) {
    parser.current = parser.current + 1
    bestie (parser.current < parser.tokens.len()) {
        parser.current_token = parser.tokens[parser.current]
    } capish {
        fr fr Create EOF token
        parser.current_token = Token{
            token_type: TOKEN_EOF,
            literal: "",
            line: 0,
            column: 0
        }
    }
}

fr fr Check if current token matches expected type
slay match_token(parser Parser, expected_type normie) lit {
    damn parser.current_token.token_type == expected_type
}

fr fr Check if current token matches expected literal
slay match_literal(parser Parser, expected tea) lit {
    damn parser.current_token.literal == expected
}

fr fr Create new AST node
slay new_ast_node(node_type normie, value tea, line normie, column normie) ASTNode {
    sus children []ASTNode = []ASTNode{}
    damn ASTNode{
        node_type: node_type,
        value: value,
        children: children,
        line: line,
        column: column
    }
}

fr fr Add child to AST node
slay add_child(parent ASTNode, child ASTNode) {
    parent.children.push(child)
}

fr fr Parse program (top level)
slay parse_program(parser Parser) ASTNode {
    sus program ASTNode = new_ast_node(AST_PROGRAM, "program", 1, 1)
    
    bestie (!match_token(parser, TOKEN_EOF)) {
        sus stmt ASTNode = parse_statement(parser)
        add_child(program, stmt)
    }
    
    damn program
}

fr fr Parse statement
slay parse_statement(parser Parser) ASTNode {
    fr fr Check for function declaration: slay name() { }
    bestie (match_literal(parser, "slay")) {
        damn parse_function(parser)
    } capish bestie (match_literal(parser, "sus")) {
        damn parse_variable_declaration(parser)
    } capish bestie (match_literal(parser, "damn")) {
        damn parse_return_statement(parser)
    } capish {
        fr fr Default to expression
        damn parse_expression(parser)
    }
}

fr fr Parse function declaration
slay parse_function(parser Parser) ASTNode {
    sus func_node ASTNode = new_ast_node(AST_FUNCTION, "function", 
                                         parser.current_token.line, 
                                         parser.current_token.column)
    
    advance(parser)  fr fr skip 'slay'
    
    fr fr Function name
    bestie (match_token(parser, TOKEN_IDENTIFIER)) {
        sus name_node ASTNode = new_ast_node(AST_VARIABLE, parser.current_token.literal,
                                            parser.current_token.line,
                                            parser.current_token.column)
        add_child(func_node, name_node)
        advance(parser)
    }
    
    fr fr Skip parameters for now - expect ()
    bestie (match_literal(parser, "(")) {
        advance(parser)
        fr fr Skip to closing paren
        bestie (match_literal(parser, ")")) {
            advance(parser)
        }
    }
    
    fr fr Parse function body { }
    bestie (match_literal(parser, "{")) {
        sus body ASTNode = parse_block(parser)
        add_child(func_node, body)
    }
    
    damn func_node
}

fr fr Parse variable declaration: sus name type = value
slay parse_variable_declaration(parser Parser) ASTNode {
    sus var_node ASTNode = new_ast_node(AST_VARIABLE, "variable",
                                        parser.current_token.line,
                                        parser.current_token.column)
    
    advance(parser)  fr fr skip 'sus'
    
    fr fr Variable name
    bestie (match_token(parser, TOKEN_IDENTIFIER)) {
        var_node.value = parser.current_token.literal
        advance(parser)
    }
    
    fr fr Skip type for now
    bestie (match_token(parser, TOKEN_IDENTIFIER)) {
        advance(parser)
    }
    
    fr fr Assignment
    bestie (match_literal(parser, "=")) {
        advance(parser)
        sus value_node ASTNode = parse_expression(parser)
        add_child(var_node, value_node)
    }
    
    damn var_node
}

fr fr Parse return statement: damn value
slay parse_return_statement(parser Parser) ASTNode {
    sus return_node ASTNode = new_ast_node(AST_RETURN, "return",
                                           parser.current_token.line,
                                           parser.current_token.column)
    
    advance(parser)  fr fr skip 'damn'
    
    fr fr Return value
    sus value_node ASTNode = parse_expression(parser)
    add_child(return_node, value_node)
    
    damn return_node
}

fr fr Parse block: { statements }
slay parse_block(parser Parser) ASTNode {
    sus block_node ASTNode = new_ast_node(AST_BLOCK, "block",
                                          parser.current_token.line,
                                          parser.current_token.column)
    
    advance(parser)  fr fr skip '{'
    
    bestie (!match_literal(parser, "}") && !match_token(parser, TOKEN_EOF)) {
        sus stmt ASTNode = parse_statement(parser)
        add_child(block_node, stmt)
    }
    
    bestie (match_literal(parser, "}")) {
        advance(parser)  fr fr skip '}'
    }
    
    damn block_node
}

fr fr Parse expression (simplified)
slay parse_expression(parser Parser) ASTNode {
    sus expr_node ASTNode = new_ast_node(AST_VARIABLE, "expression",
                                         parser.current_token.line,
                                         parser.current_token.column)
    
    bestie (match_token(parser, TOKEN_IDENTIFIER)) {
        expr_node.value = parser.current_token.literal
        advance(parser)
    } capish bestie (match_token(parser, TOKEN_NUMBER)) {
        expr_node.value = parser.current_token.literal
        advance(parser)
    } capish bestie (match_token(parser, TOKEN_STRING)) {
        expr_node.value = parser.current_token.literal
        advance(parser)
    } capish {
        fr fr Skip unknown tokens
        advance(parser)
    }
    
    damn expr_node
}

fr fr Print AST node (debug)
slay print_ast_node(node ASTNode, indent normie) {
    fr fr Print indentation
    bestie (i := 0; i < indent; i = i + 1) {
        vibez.spill("  ")
    }
    
    vibez.spill("Node type:", node.node_type, "value:", node.value)
    
    fr fr Print children
    bestie (i := 0; i < node.children.len(); i = i + 1) {
        print_ast_node(node.children[i], indent + 1)
    }
}

fr fr Simple lexer for testing (copy from previous)
slay tokenize_simple(input tea) []Token {
    sus tokens []Token = []Token{}
    sus pos normie = 0
    sus line normie = 1
    sus col normie = 1
    
    bestie (pos < input.len()) {
        sus ch tea = input.substring(pos, pos + 1)
        
        bestie (ch == " " || ch == "\t" || ch == "\r") {
            fr fr Skip whitespace
            pos = pos + 1
            col = col + 1
        } capish bestie (ch == "\n") {
            pos = pos + 1
            line = line + 1
            col = 1
        } capish bestie ((ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || ch == "_") {
            fr fr Identifier/keyword
            sus start normie = pos
            bestie (pos < input.len()) {
                sus current tea = input.substring(pos, pos + 1)
                bestie (!((current >= "a" && current <= "z") || 
                         (current >= "A" && current <= "Z") || 
                         (current >= "0" && current <= "9") || 
                         current == "_")) {
                    vibes
                }
                pos = pos + 1
            }
            
            sus literal tea = input.substring(start, pos)
            sus token_type normie = TOKEN_IDENTIFIER
            
            fr fr Check for keywords
            bestie (literal == "slay" || literal == "sus" || literal == "damn") {
                token_type = TOKEN_KEYWORD
            }
            
            sus token Token = Token{
                token_type: token_type,
                literal: literal,
                line: line,
                column: col
            }
            tokens.push(token)
            col = col + (pos - start)
        } capish bestie (ch >= "0" && ch <= "9") {
            fr fr Number
            sus start normie = pos
            bestie (pos < input.len()) {
                sus current tea = input.substring(pos, pos + 1)
                bestie (!(current >= "0" && current <= "9")) {
                    vibes
                }
                pos = pos + 1
            }
            
            sus literal tea = input.substring(start, pos)
            sus token Token = Token{
                token_type: TOKEN_NUMBER,
                literal: literal,
                line: line,
                column: col
            }
            tokens.push(token)
            col = col + (pos - start)
        } capish {
            fr fr Single character operators/symbols
            sus token Token = Token{
                token_type: TOKEN_OPERATOR,
                literal: ch,
                line: line,
                column: col
            }
            tokens.push(token)
            pos = pos + 1
            col = col + 1
        }
    }
    
    fr fr Add EOF token
    sus eof_token Token = Token{
        token_type: TOKEN_EOF,
        literal: "",
        line: line,
        column: col
    }
    tokens.push(eof_token)
    
    damn tokens
}

fr fr Test the parser
slay test_parser() {
    vibez.spill("Bootstrap Parser Test")
    
    sus test_input tea = "slay hello() { sus x normie = 42 damn x }"
    vibez.spill("Parsing:", test_input)
    
    sus tokens []Token = tokenize_simple(test_input)
    vibez.spill("Tokenized", tokens.len(), "tokens")
    
    sus parser Parser = new_parser(tokens)
    sus ast ASTNode = parse_program(parser)
    
    vibez.spill("AST structure:")
    print_ast_node(ast, 0)
    
    vibez.spill("Parser test complete")
}

fr fr Main function
slay main() {
    test_parser()
}

main()
