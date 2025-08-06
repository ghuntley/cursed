#!/usr/bin/env cursed
# CURSED Self-Hosting Compiler - Parser Module
# Recursive descent parser for building AST

yeet "lexer"
yeet "stringz"
yeet "arrayz"
yeet "testz"

# AST Node types
enum NodeType {
    PROGRAM,
    FUNCTION_DECLARATION,
    VARIABLE_DECLARATION,
    ASSIGNMENT,
    BLOCK_STATEMENT,
    EXPRESSION_STATEMENT,
    RETURN_STATEMENT,
    IF_STATEMENT,
    WHILE_STATEMENT,
    FOR_STATEMENT,
    CALL_EXPRESSION,
    BINARY_EXPRESSION,
    UNARY_EXPRESSION,
    IDENTIFIER,
    INTEGER_LITERAL,
    FLOAT_LITERAL,
    STRING_LITERAL,
    BOOLEAN_LITERAL,
    STRUCT_DECLARATION,
    INTERFACE_DECLARATION,
}

# AST Node base structure
squad ASTNode {
    spill node_type NodeType
    spill value tea
    spill children []ASTNode
    spill line normie
    spill column normie
}

# Parser state
squad Parser {
    spill tokens []Token
    spill current normie    # current token index
    spill errors []tea     # parse errors
}

# Initialize parser
slay new_parser(tokens []Token) Parser {
    damn Parser{
        tokens: tokens,
        current: 0,
        errors: []
    }
}

# Get current token
slay current_token(parser Parser) Token {
    lowkey (parser.current >= arrayz.array_length(parser.tokens)) {
        # Return EOF token
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

# Get next token
slay peek_token(parser Parser) Token {
    lowkey (parser.current + 1 >= arrayz.array_length(parser.tokens)) {
        # Return EOF token
        damn Token{
            token_type: TokenType.EOF,
            literal: "",
            line: 0,
            column: 0,
            position: 0
        }
    }
    
    damn arrayz.array_get(parser.tokens, parser.current + 1)
}

# Advance to next token
slay advance_token(parser Parser) {
    lowkey (parser.current < arrayz.array_length(parser.tokens) - 1) {
        parser.current = parser.current + 1
    }
}

# Check if current token matches expected type
slay match_token(parser Parser, expected_type TokenType) lit {
    sus token Token = current_token(parser)
    damn token.token_type == expected_type
}

# Consume token if it matches expected type
slay consume_token(parser Parser, expected_type TokenType, error_msg tea) lit {
    lowkey (match_token(parser, expected_type)) {
        advance_token(parser)
        damn based
    } highkey {
        add_error(parser, error_msg)
        damn cringe
    }
}

# Add parse error
slay add_error(parser Parser, message tea) {
    sus token Token = current_token(parser)
    sus error_msg tea = "Parse error at line " + token.line + ", column " + token.column + ": " + message
    arrayz.array_push(parser.errors, error_msg)
}

# Create new AST node
slay new_node(node_type NodeType, value tea, line normie, column normie) ASTNode {
    damn ASTNode{
        node_type: node_type,
        value: value,
        children: [],
        line: line,
        column: column
    }
}

# Add child to AST node
slay add_child(parent ASTNode, child ASTNode) {
    arrayz.array_push(parent.children, child)
}

# Parse program (top-level)
slay parse_program(parser Parser) ASTNode {
    sus program ASTNode = new_node(NodeType.PROGRAM, "program", 1, 1)
    
    # Skip newlines at start
    bestie (match_token(parser, TokenType.NEWLINE)) {
        advance_token(parser)
    }
    
    # Parse top-level declarations
    bestie (!match_token(parser, TokenType.EOF)) {
        sus declaration ASTNode = parse_declaration(parser)
        lowkey (declaration.node_type != NodeType.PROGRAM) {  # Valid node
            add_child(program, declaration)
        }
        
        # Skip optional newlines
        bestie (match_token(parser, TokenType.NEWLINE)) {
            advance_token(parser)
        }
    }
    
    damn program
}

# Parse declaration (function, variable, struct, etc.)
slay parse_declaration(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    
    vibe_check (token.token_type) {
        mood TokenType.SLAY {
            damn parse_function_declaration(parser)
        }
        mood TokenType.SUS {
            damn parse_variable_declaration(parser, cringe)  # mutable
        }
        mood TokenType.FACTS {
            damn parse_variable_declaration(parser, based)   # immutable
        }
        mood TokenType.SQUAD {
            damn parse_struct_declaration(parser)
        }
        mood TokenType.COLLAB {
            damn parse_interface_declaration(parser)
        }
        mood TokenType.YEET {
            damn parse_import_statement(parser)
        }
        basic {
            damn parse_statement(parser)
        }
    }
}

# Parse function declaration
slay parse_function_declaration(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus func_node ASTNode = new_node(NodeType.FUNCTION_DECLARATION, "", token.line, token.column)
    
    # Consume "slay" keyword
    consume_token(parser, TokenType.SLAY, "Expected 'slay'")
    
    # Parse function name
    lowkey (match_token(parser, TokenType.IDENTIFIER)) {
        sus name_token Token = current_token(parser)
        sus name_node ASTNode = new_node(NodeType.IDENTIFIER, name_token.literal, name_token.line, name_token.column)
        add_child(func_node, name_node)
        advance_token(parser)
    } highkey {
        add_error(parser, "Expected function name")
    }
    
    # Parse parameters
    consume_token(parser, TokenType.LPAREN, "Expected '(' after function name")
    
    sus params_node ASTNode = new_node(NodeType.BLOCK_STATEMENT, "parameters", token.line, token.column)
    
    lowkey (!match_token(parser, TokenType.RPAREN)) {
        periodt (based) {
            # Parse parameter: name type
            lowkey (match_token(parser, TokenType.IDENTIFIER)) {
                sus param_name Token = current_token(parser)
                sus param_node ASTNode = new_node(NodeType.IDENTIFIER, param_name.literal, param_name.line, param_name.column)
                advance_token(parser)
                
                # Parse type
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
                advance_token(parser)  # consume comma
            } highkey {
                add_error(parser, "Expected parameter name")
                ghosted
            }
        }
    }
    
    add_child(func_node, params_node)
    consume_token(parser, TokenType.RPAREN, "Expected ')' after parameters")
    
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

# Parse variable declaration
slay parse_variable_declaration(parser Parser, immutable lit) ASTNode {
    sus token Token = current_token(parser)
    sus var_node ASTNode = new_node(NodeType.VARIABLE_DECLARATION, "", token.line, token.column)
    
    # Consume "sus" or "facts"
    advance_token(parser)
    
    # Parse variable name
    lowkey (match_token(parser, TokenType.IDENTIFIER)) {
        sus name_token Token = current_token(parser)
        sus name_node ASTNode = new_node(NodeType.IDENTIFIER, name_token.literal, name_token.line, name_token.column)
        add_child(var_node, name_node)
        advance_token(parser)
    } highkey {
        add_error(parser, "Expected variable name")
    }
    
    # Parse type (optional)
    lowkey (match_token(parser, TokenType.IDENTIFIER) || 
           match_token(parser, TokenType.NORMIE) ||
           match_token(parser, TokenType.TEA) ||
           match_token(parser, TokenType.LIT) ||
           match_token(parser, TokenType.DRIP)) {
        sus type_token Token = current_token(parser)
        sus type_node ASTNode = new_node(NodeType.IDENTIFIER, type_token.literal, type_token.line, type_token.column)
        add_child(var_node, type_node)
        advance_token(parser)
    }
    
    # Parse initializer (optional)
    lowkey (match_token(parser, TokenType.ASSIGN)) {
        advance_token(parser)  # consume =
        sus init_expr ASTNode = parse_expression(parser)
        add_child(var_node, init_expr)
    }
    
    damn var_node
}

# Parse struct declaration
slay parse_struct_declaration(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus struct_node ASTNode = new_node(NodeType.STRUCT_DECLARATION, "", token.line, token.column)
    
    # Consume "squad" keyword
    advance_token(parser)
    
    # Parse struct name
    lowkey (match_token(parser, TokenType.IDENTIFIER)) {
        sus name_token Token = current_token(parser)
        sus name_node ASTNode = new_node(NodeType.IDENTIFIER, name_token.literal, name_token.line, name_token.column)
        add_child(struct_node, name_node)
        advance_token(parser)
    } highkey {
        add_error(parser, "Expected struct name")
    }
    
    # Parse struct body
    consume_token(parser, TokenType.LBRACE, "Expected '{' after struct name")
    
    bestie (!match_token(parser, TokenType.RBRACE) && !match_token(parser, TokenType.EOF)) {
        # Skip newlines
        lowkey (match_token(parser, TokenType.NEWLINE)) {
            advance_token(parser)
            simp
        }
        
        # Parse field: spill name type
        lowkey (match_token(parser, TokenType.IDENTIFIER) && current_token(parser).literal == "spill") {
            advance_token(parser)  # consume "spill"
            
            lowkey (match_token(parser, TokenType.IDENTIFIER)) {
                sus field_name Token = current_token(parser)
                sus field_node ASTNode = new_node(NodeType.IDENTIFIER, field_name.literal, field_name.line, field_name.column)
                advance_token(parser)
                
                # Parse type
                lowkey (match_token(parser, TokenType.IDENTIFIER) || 
                       match_token(parser, TokenType.NORMIE) ||
                       match_token(parser, TokenType.TEA) ||
                       match_token(parser, TokenType.LIT)) {
                    sus type_token Token = current_token(parser)
                    sus type_node ASTNode = new_node(NodeType.IDENTIFIER, type_token.literal, type_token.line, type_token.column)
                    add_child(field_node, type_node)
                    advance_token(parser)
                }
                
                add_child(struct_node, field_node)
            }
        } highkey {
            add_error(parser, "Expected field declaration")
            advance_token(parser)  # skip invalid token
        }
    }
    
    consume_token(parser, TokenType.RBRACE, "Expected '}' after struct body")
    
    damn struct_node
}

# Parse interface declaration
slay parse_interface_declaration(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus interface_node ASTNode = new_node(NodeType.INTERFACE_DECLARATION, "", token.line, token.column)
    
    # Consume "collab" keyword
    advance_token(parser)
    
    # Parse interface name
    lowkey (match_token(parser, TokenType.IDENTIFIER)) {
        sus name_token Token = current_token(parser)
        sus name_node ASTNode = new_node(NodeType.IDENTIFIER, name_token.literal, name_token.line, name_token.column)
        add_child(interface_node, name_node)
        advance_token(parser)
    } highkey {
        add_error(parser, "Expected interface name")
    }
    
    # Parse interface body
    consume_token(parser, TokenType.LBRACE, "Expected '{' after interface name")
    
    bestie (!match_token(parser, TokenType.RBRACE) && !match_token(parser, TokenType.EOF)) {
        # Skip newlines
        lowkey (match_token(parser, TokenType.NEWLINE)) {
            advance_token(parser)
            simp
        }
        
        # Parse method declaration
        sus method_node ASTNode = parse_function_declaration(parser)
        add_child(interface_node, method_node)
    }
    
    consume_token(parser, TokenType.RBRACE, "Expected '}' after interface body")
    
    damn interface_node
}

# Parse import statement
slay parse_import_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus import_node ASTNode = new_node(NodeType.EXPRESSION_STATEMENT, "import", token.line, token.column)
    
    advance_token(parser)  # consume "yeet"
    
    # Parse import path
    lowkey (match_token(parser, TokenType.STRING)) {
        sus path_token Token = current_token(parser)
        sus path_node ASTNode = new_node(NodeType.STRING_LITERAL, path_token.literal, path_token.line, path_token.column)
        add_child(import_node, path_node)
        advance_token(parser)
    } highkey {
        add_error(parser, "Expected import path")
    }
    
    damn import_node
}

# Parse block statement
slay parse_block_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus block_node ASTNode = new_node(NodeType.BLOCK_STATEMENT, "", token.line, token.column)
    
    consume_token(parser, TokenType.LBRACE, "Expected '{'")
    
    bestie (!match_token(parser, TokenType.RBRACE) && !match_token(parser, TokenType.EOF)) {
        # Skip newlines
        lowkey (match_token(parser, TokenType.NEWLINE)) {
            advance_token(parser)
            simp
        }
        
        sus stmt ASTNode = parse_statement(parser)
        lowkey (stmt.node_type != NodeType.PROGRAM) {  # Valid statement
            add_child(block_node, stmt)
        }
    }
    
    consume_token(parser, TokenType.RBRACE, "Expected '}'")
    
    damn block_node
}

# Parse statement
slay parse_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    
    vibe_check (token.token_type) {
        mood TokenType.DAMN {
            damn parse_return_statement(parser)
        }
        mood TokenType.LOWKEY {
            damn parse_if_statement(parser)
        }
        mood TokenType.PERIODT {
            damn parse_while_statement(parser)
        }
        mood TokenType.BESTIE {
            damn parse_for_statement(parser)
        }
        mood TokenType.LBRACE {
            damn parse_block_statement(parser)
        }
        basic {
            damn parse_expression_statement(parser)
        }
    }
}

# Parse return statement
slay parse_return_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus return_node ASTNode = new_node(NodeType.RETURN_STATEMENT, "", token.line, token.column)
    
    advance_token(parser)  # consume "damn"
    
    # Parse return expression (optional)
    lowkey (!match_token(parser, TokenType.NEWLINE) && !match_token(parser, TokenType.RBRACE) && !match_token(parser, TokenType.EOF)) {
        sus expr ASTNode = parse_expression(parser)
        add_child(return_node, expr)
    }
    
    damn return_node
}

# Parse if statement
slay parse_if_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus if_node ASTNode = new_node(NodeType.IF_STATEMENT, "", token.line, token.column)
    
    advance_token(parser)  # consume "lowkey"
    
    # Parse condition in parentheses
    consume_token(parser, TokenType.LPAREN, "Expected '(' after 'lowkey'")
    sus condition ASTNode = parse_expression(parser)
    add_child(if_node, condition)
    consume_token(parser, TokenType.RPAREN, "Expected ')' after condition")
    
    # Parse then block
    sus then_block ASTNode = parse_block_statement(parser)
    add_child(if_node, then_block)
    
    # Parse else block (optional)
    lowkey (match_token(parser, TokenType.HIGHKEY)) {
        advance_token(parser)  # consume "highkey"
        sus else_block ASTNode = parse_block_statement(parser)
        add_child(if_node, else_block)
    }
    
    damn if_node
}

# Parse while statement
slay parse_while_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus while_node ASTNode = new_node(NodeType.WHILE_STATEMENT, "", token.line, token.column)
    
    advance_token(parser)  # consume "periodt"
    
    # Parse condition in parentheses
    consume_token(parser, TokenType.LPAREN, "Expected '(' after 'periodt'")
    sus condition ASTNode = parse_expression(parser)
    add_child(while_node, condition)
    consume_token(parser, TokenType.RPAREN, "Expected ')' after condition")
    
    # Parse body
    sus body ASTNode = parse_block_statement(parser)
    add_child(while_node, body)
    
    damn while_node
}

# Parse for statement
slay parse_for_statement(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    sus for_node ASTNode = new_node(NodeType.FOR_STATEMENT, "", token.line, token.column)
    
    advance_token(parser)  # consume "bestie"
    
    # Parse condition in parentheses
    consume_token(parser, TokenType.LPAREN, "Expected '(' after 'bestie'")
    sus condition ASTNode = parse_expression(parser)
    add_child(for_node, condition)
    consume_token(parser, TokenType.RPAREN, "Expected ')' after condition")
    
    # Parse body
    sus body ASTNode = parse_block_statement(parser)
    add_child(for_node, body)
    
    damn for_node
}

# Parse expression statement
slay parse_expression_statement(parser Parser) ASTNode {
    sus expr ASTNode = parse_expression(parser)
    sus stmt_node ASTNode = new_node(NodeType.EXPRESSION_STATEMENT, "", expr.line, expr.column)
    add_child(stmt_node, expr)
    damn stmt_node
}

# Parse expression
slay parse_expression(parser Parser) ASTNode {
    damn parse_logical_or(parser)
}

# Parse logical OR expression
slay parse_logical_or(parser Parser) ASTNode {
    sus left ASTNode = parse_logical_and(parser)
    
    bestie (match_token(parser, TokenType.OR)) {
        sus op_token Token = current_token(parser)
        advance_token(parser)
        sus right ASTNode = parse_logical_and(parser)
        
        sus binary_node ASTNode = new_node(NodeType.BINARY_EXPRESSION, op_token.literal, op_token.line, op_token.column)
        add_child(binary_node, left)
        add_child(binary_node, right)
        left = binary_node
    }
    
    damn left
}

# Parse logical AND expression
slay parse_logical_and(parser Parser) ASTNode {
    sus left ASTNode = parse_equality(parser)
    
    bestie (match_token(parser, TokenType.AND)) {
        sus op_token Token = current_token(parser)
        advance_token(parser)
        sus right ASTNode = parse_equality(parser)
        
        sus binary_node ASTNode = new_node(NodeType.BINARY_EXPRESSION, op_token.literal, op_token.line, op_token.column)
        add_child(binary_node, left)
        add_child(binary_node, right)
        left = binary_node
    }
    
    damn left
}

# Parse equality expression
slay parse_equality(parser Parser) ASTNode {
    sus left ASTNode = parse_comparison(parser)
    
    bestie (match_token(parser, TokenType.EQUAL) || match_token(parser, TokenType.NOT_EQUAL)) {
        sus op_token Token = current_token(parser)
        advance_token(parser)
        sus right ASTNode = parse_comparison(parser)
        
        sus binary_node ASTNode = new_node(NodeType.BINARY_EXPRESSION, op_token.literal, op_token.line, op_token.column)
        add_child(binary_node, left)
        add_child(binary_node, right)
        left = binary_node
    }
    
    damn left
}

# Parse comparison expression
slay parse_comparison(parser Parser) ASTNode {
    sus left ASTNode = parse_addition(parser)
    
    bestie (match_token(parser, TokenType.LESS) || match_token(parser, TokenType.LESS_EQ) ||
           match_token(parser, TokenType.GREATER) || match_token(parser, TokenType.GREATER_EQ)) {
        sus op_token Token = current_token(parser)
        advance_token(parser)
        sus right ASTNode = parse_addition(parser)
        
        sus binary_node ASTNode = new_node(NodeType.BINARY_EXPRESSION, op_token.literal, op_token.line, op_token.column)
        add_child(binary_node, left)
        add_child(binary_node, right)
        left = binary_node
    }
    
    damn left
}

# Parse addition/subtraction expression
slay parse_addition(parser Parser) ASTNode {
    sus left ASTNode = parse_multiplication(parser)
    
    bestie (match_token(parser, TokenType.PLUS) || match_token(parser, TokenType.MINUS)) {
        sus op_token Token = current_token(parser)
        advance_token(parser)
        sus right ASTNode = parse_multiplication(parser)
        
        sus binary_node ASTNode = new_node(NodeType.BINARY_EXPRESSION, op_token.literal, op_token.line, op_token.column)
        add_child(binary_node, left)
        add_child(binary_node, right)
        left = binary_node
    }
    
    damn left
}

# Parse multiplication/division expression
slay parse_multiplication(parser Parser) ASTNode {
    sus left ASTNode = parse_unary(parser)
    
    bestie (match_token(parser, TokenType.MULTIPLY) || match_token(parser, TokenType.DIVIDE) || match_token(parser, TokenType.MODULO)) {
        sus op_token Token = current_token(parser)
        advance_token(parser)
        sus right ASTNode = parse_unary(parser)
        
        sus binary_node ASTNode = new_node(NodeType.BINARY_EXPRESSION, op_token.literal, op_token.line, op_token.column)
        add_child(binary_node, left)
        add_child(binary_node, right)
        left = binary_node
    }
    
    damn left
}

# Parse unary expression
slay parse_unary(parser Parser) ASTNode {
    lowkey (match_token(parser, TokenType.NOT) || match_token(parser, TokenType.MINUS)) {
        sus op_token Token = current_token(parser)
        advance_token(parser)
        sus operand ASTNode = parse_unary(parser)
        
        sus unary_node ASTNode = new_node(NodeType.UNARY_EXPRESSION, op_token.literal, op_token.line, op_token.column)
        add_child(unary_node, operand)
        damn unary_node
    }
    
    damn parse_postfix(parser)
}

# Parse postfix expression (function calls, member access)
slay parse_postfix(parser Parser) ASTNode {
    sus expr ASTNode = parse_primary(parser)
    
    periodt (based) {
        lowkey (match_token(parser, TokenType.LPAREN)) {
            # Function call
            sus call_token Token = current_token(parser)
            advance_token(parser)  # consume (
            
            sus call_node ASTNode = new_node(NodeType.CALL_EXPRESSION, "", call_token.line, call_token.column)
            add_child(call_node, expr)
            
            # Parse arguments
            lowkey (!match_token(parser, TokenType.RPAREN)) {
                periodt (based) {
                    sus arg ASTNode = parse_expression(parser)
                    add_child(call_node, arg)
                    
                    lowkey (!match_token(parser, TokenType.COMMA)) {
                        ghosted
                    }
                    advance_token(parser)  # consume comma
                }
            }
            
            consume_token(parser, TokenType.RPAREN, "Expected ')' after arguments")
            expr = call_node
        } highkey lowkey (match_token(parser, TokenType.DOT)) {
            # Member access
            advance_token(parser)  # consume .
            
            lowkey (match_token(parser, TokenType.IDENTIFIER)) {
                sus member_token Token = current_token(parser)
                advance_token(parser)
                
                sus member_node ASTNode = new_node(NodeType.BINARY_EXPRESSION, ".", member_token.line, member_token.column)
                add_child(member_node, expr)
                
                sus member_name ASTNode = new_node(NodeType.IDENTIFIER, member_token.literal, member_token.line, member_token.column)
                add_child(member_node, member_name)
                
                expr = member_node
            } highkey {
                add_error(parser, "Expected member name after '.'")
            }
        } highkey {
            ghosted
        }
    }
    
    damn expr
}

# Parse primary expression
slay parse_primary(parser Parser) ASTNode {
    sus token Token = current_token(parser)
    
    vibe_check (token.token_type) {
        mood TokenType.INTEGER {
            advance_token(parser)
            damn new_node(NodeType.INTEGER_LITERAL, token.literal, token.line, token.column)
        }
        mood TokenType.FLOAT {
            advance_token(parser)
            damn new_node(NodeType.FLOAT_LITERAL, token.literal, token.line, token.column)
        }
        mood TokenType.STRING {
            advance_token(parser)
            damn new_node(NodeType.STRING_LITERAL, token.literal, token.line, token.column)
        }
        mood TokenType.BASED {
            advance_token(parser)
            damn new_node(NodeType.BOOLEAN_LITERAL, "true", token.line, token.column)
        }
        mood TokenType.CRINGE {
            advance_token(parser)
            damn new_node(NodeType.BOOLEAN_LITERAL, "false", token.line, token.column)
        }
        mood TokenType.IDENTIFIER {
            advance_token(parser)
            damn new_node(NodeType.IDENTIFIER, token.literal, token.line, token.column)
        }
        mood TokenType.LPAREN {
            advance_token(parser)  # consume (
            sus expr ASTNode = parse_expression(parser)
            consume_token(parser, TokenType.RPAREN, "Expected ')' after expression")
            damn expr
        }
        basic {
            add_error(parser, "Unexpected token: " + token.literal)
            advance_token(parser)  # skip invalid token
            damn new_node(NodeType.IDENTIFIER, "error", token.line, token.column)
        }
    }
}

# Parse entire source code
slay parse(source tea) ASTNode {
    sus tokens []Token = tokenize(source)
    sus parser Parser = new_parser(tokens)
    sus ast ASTNode = parse_program(parser)
    
    # Report any parsing errors
    lowkey (arrayz.array_length(parser.errors) > 0) {
        vibez.spill("Parse errors:")
        bestie i := 0; i < arrayz.array_length(parser.errors); i = i + 1 {
            sus error tea = arrayz.array_get(parser.errors, i)
            vibez.spill("  " + error)
        }
    }
    
    damn ast
}

# Print AST for debugging
slay print_ast(node ASTNode, indent normie) {
    sus spaces tea = ""
    bestie i := 0; i < indent; i = i + 1 {
        spaces = spaces + "  "
    }
    
    vibez.spill(spaces + node.node_type + ": " + node.value)
    
    bestie i := 0; i < arrayz.array_length(node.children); i = i + 1 {
        sus child ASTNode = arrayz.array_get(node.children, i)
        print_ast(child, indent + 1)
    }
}
