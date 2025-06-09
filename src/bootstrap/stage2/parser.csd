// Stage 2 Parser - Written in CURSED minimal subset
// Parses tokens into an Abstract Syntax Tree

import "std/string"
import "std/strconv"

// Operator precedence levels
enum Precedence {
    LOWEST,
    EQUALS,      // ==, !=
    LESSGREATER, // > or <
    SUM,         // +, -
    PRODUCT,     // *, /
    PREFIX,      // -X, !X
    CALL,        // myFunction(X)
    INDEX,       // array[index]
}

// Parser parses tokens into AST
struct Parser {
    tokens: []Token
    cur_token: Token
    peek_token: Token
    position: int
    errors: []string
}

// Create new parser
func new_parser() Parser {
    return Parser{
        tokens: []Token{},
        position: 0,
        errors: []string{},
    }
}

// Parse tokens into AST
func (p *Parser) parse(tokens: []Token) *Program {
    p.tokens = tokens
    p.position = 0
    p.errors = []string{}
    
    if len(tokens) > 0 {
        p.cur_token = tokens[0]
    }
    if len(tokens) > 1 {
        p.peek_token = tokens[1]
    }
    
    program := &Program{
        statements: []Statement{},
    }
    
    for p.cur_token.type != TokenType.EOF {
        stmt := p.parse_statement()
        if stmt != nil {
            program.statements = append(program.statements, stmt)
        }
        p.next_token()
    }
    
    return program
}

// Move to next token
func (p *Parser) next_token() {
    p.position = p.position + 1
    p.cur_token = p.peek_token
    
    if p.position + 1 < len(p.tokens) {
        p.peek_token = p.tokens[p.position + 1]
    } else {
        p.peek_token = Token{type: TokenType.EOF}
    }
}

// Parse statement
func (p *Parser) parse_statement() Statement {
    if p.cur_token.type == TokenType.IMPORT {
        return p.parse_import_statement()
    } else if p.cur_token.type == TokenType.FUNC {
        return p.parse_function_statement()
    } else if p.cur_token.type == TokenType.LET {
        return p.parse_let_statement()
    } else if p.cur_token.type == TokenType.RETURN {
        return p.parse_return_statement()
    } else if p.cur_token.type == TokenType.IF {
        return p.parse_if_statement()
    } else if p.cur_token.type == TokenType.FOR {
        return p.parse_for_statement()
    } else if p.cur_token.type == TokenType.STRUCT {
        return p.parse_struct_statement()
    } else if p.cur_token.type == TokenType.LBRACE {
        return p.parse_block_statement()
    } else {
        return p.parse_expression_statement()
    }
}

// Parse import statement
func (p *Parser) parse_import_statement() Statement {
    if !p.expect_peek(TokenType.STRING) {
        return nil
    }
    
    path := p.cur_token.literal
    p.consume_semicolon()
    
    return &ImportStatement{path: path}
}

// Parse function statement
func (p *Parser) parse_function_statement() Statement {
    if !p.expect_peek(TokenType.IDENT) {
        return nil
    }
    
    name := p.cur_token.literal
    
    if !p.expect_peek(TokenType.LPAREN) {
        return nil
    }
    
    parameters := p.parse_function_parameters()
    
    // Optional return type
    return_type := ""
    if p.peek_token.type == TokenType.IDENT {
        p.next_token()
        return_type = p.cur_token.literal
    }
    
    if !p.expect_peek(TokenType.LBRACE) {
        return nil
    }
    
    body := p.parse_block_statement()
    
    return &FunctionStatement{
        name: name,
        parameters: parameters,
        return_type: return_type,
        body: *body,
    }
}

// Parse function parameters
func (p *Parser) parse_function_parameters() []Parameter {
    parameters := []Parameter{}
    
    if p.peek_token.type == TokenType.RPAREN {
        p.next_token()
        return parameters
    }
    
    p.next_token()
    
    // Parse first parameter
    if p.cur_token.type == TokenType.IDENT {
        param_name := p.cur_token.literal
        
        if p.expect_peek(TokenType.COLON) && p.expect_peek(TokenType.IDENT) {
            param_type := p.cur_token.literal
            parameters = append(parameters, Parameter{
                name: param_name,
                type: param_type,
            })
        }
    }
    
    // Parse remaining parameters
    for p.peek_token.type == TokenType.COMMA {
        p.next_token() // consume comma
        p.next_token() // move to parameter name
        
        if p.cur_token.type == TokenType.IDENT {
            param_name := p.cur_token.literal
            
            if p.expect_peek(TokenType.COLON) && p.expect_peek(TokenType.IDENT) {
                param_type := p.cur_token.literal
                parameters = append(parameters, Parameter{
                    name: param_name,
                    type: param_type,
                })
            }
        }
    }
    
    if !p.expect_peek(TokenType.RPAREN) {
        return nil
    }
    
    return parameters
}

// Parse let statement
func (p *Parser) parse_let_statement() Statement {
    if !p.expect_peek(TokenType.IDENT) {
        return nil
    }
    
    name := p.cur_token.literal
    
    // Optional type annotation
    var_type := ""
    if p.peek_token.type == TokenType.COLON {
        p.next_token() // consume :
        if !p.expect_peek(TokenType.IDENT) {
            return nil
        }
        var_type = p.cur_token.literal
    }
    
    // Optional initializer
    var value Expression
    if p.peek_token.type == TokenType.ASSIGN {
        p.next_token() // consume =
        p.next_token() // move to expression
        value = p.parse_expression(Precedence.LOWEST)
    }
    
    p.consume_semicolon()
    
    return &LetStatement{
        name: name,
        type: var_type,
        value: value,
    }
}

// Parse return statement
func (p *Parser) parse_return_statement() Statement {
    var value Expression
    
    if p.peek_token.type != TokenType.SEMICOLON && p.peek_token.type != TokenType.EOF {
        p.next_token()
        value = p.parse_expression(Precedence.LOWEST)
    }
    
    p.consume_semicolon()
    
    return &ReturnStatement{value: value}
}

// Parse if statement
func (p *Parser) parse_if_statement() Statement {
    if !p.expect_peek(TokenType.LPAREN) {
        return nil
    }
    
    p.next_token()
    condition := p.parse_expression(Precedence.LOWEST)
    
    if !p.expect_peek(TokenType.RPAREN) {
        return nil
    }
    
    if !p.expect_peek(TokenType.LBRACE) {
        return nil
    }
    
    consequence := p.parse_block_statement()
    
    var alternative *BlockStatement
    if p.peek_token.type == TokenType.ELSE {
        p.next_token()
        
        if !p.expect_peek(TokenType.LBRACE) {
            return nil
        }
        
        alternative = p.parse_block_statement()
    }
    
    result := &IfStatement{
        condition: condition,
        consequence: *consequence,
    }
    
    if alternative != nil {
        result.alternative = *alternative
    }
    
    return result
}

// Parse for statement  
func (p *Parser) parse_for_statement() Statement {
    if !p.expect_peek(TokenType.LPAREN) {
        return nil
    }
    
    // Parse init statement
    var init Statement
    if p.peek_token.type != TokenType.SEMICOLON {
        p.next_token()
        init = p.parse_statement()
    } else {
        p.next_token() // consume semicolon
    }
    
    // Parse condition
    var condition Expression
    if p.peek_token.type != TokenType.SEMICOLON {
        p.next_token()
        condition = p.parse_expression(Precedence.LOWEST)
    }
    
    if !p.expect_peek(TokenType.SEMICOLON) {
        return nil
    }
    
    // Parse update statement
    var update Statement
    if p.peek_token.type != TokenType.RPAREN {
        p.next_token()
        update = p.parse_statement()
    }
    
    if !p.expect_peek(TokenType.RPAREN) {
        return nil
    }
    
    if !p.expect_peek(TokenType.LBRACE) {
        return nil
    }
    
    body := p.parse_block_statement()
    
    return &ForStatement{
        init: init,
        condition: condition,
        update: update,
        body: *body,
    }
}

// Parse struct statement
func (p *Parser) parse_struct_statement() Statement {
    if !p.expect_peek(TokenType.IDENT) {
        return nil
    }
    
    name := p.cur_token.literal
    
    if !p.expect_peek(TokenType.LBRACE) {
        return nil
    }
    
    fields := []StructField{}
    
    for p.peek_token.type != TokenType.RBRACE && p.peek_token.type != TokenType.EOF {
        p.next_token()
        
        if p.cur_token.type == TokenType.IDENT {
            field_name := p.cur_token.literal
            
            if p.expect_peek(TokenType.COLON) && p.expect_peek(TokenType.IDENT) {
                field_type := p.cur_token.literal
                fields = append(fields, StructField{
                    name: field_name,
                    type: field_type,
                })
            }
        }
        
        p.consume_semicolon()
    }
    
    if !p.expect_peek(TokenType.RBRACE) {
        return nil
    }
    
    return &StructStatement{
        name: name,
        fields: fields,
    }
}

// Parse block statement
func (p *Parser) parse_block_statement() *BlockStatement {
    statements := []Statement{}
    
    p.next_token()
    
    for p.cur_token.type != TokenType.RBRACE && p.cur_token.type != TokenType.EOF {
        stmt := p.parse_statement()
        if stmt != nil {
            statements = append(statements, stmt)
        }
        p.next_token()
    }
    
    return &BlockStatement{statements: statements}
}

// Parse expression statement
func (p *Parser) parse_expression_statement() Statement {
    expr := p.parse_expression(Precedence.LOWEST)
    
    if p.peek_token.type == TokenType.SEMICOLON {
        p.next_token()
    }
    
    return &ExpressionStatement{expression: expr}
}

// Parse expression with precedence
func (p *Parser) parse_expression(precedence: Precedence) Expression {
    // Parse prefix expressions
    var left Expression
    
    if p.cur_token.type == TokenType.IDENT {
        left = &Identifier{value: p.cur_token.literal}
    } else if p.cur_token.type == TokenType.INT {
        value, _ := strconv.atoi(p.cur_token.literal)
        left = &IntegerLiteral{value: value}
    } else if p.cur_token.type == TokenType.STRING {
        left = &StringLiteral{value: p.cur_token.literal}
    } else if p.cur_token.type == TokenType.TRUE {
        left = &BooleanLiteral{value: true}
    } else if p.cur_token.type == TokenType.FALSE {
        left = &BooleanLiteral{value: false}
    } else if p.cur_token.type == TokenType.BANG || p.cur_token.type == TokenType.MINUS {
        operator := p.cur_token.literal
        p.next_token()
        right := p.parse_expression(Precedence.PREFIX)
        left = &PrefixExpression{operator: operator, right: right}
    } else if p.cur_token.type == TokenType.LPAREN {
        p.next_token()
        left = p.parse_expression(Precedence.LOWEST)
        if !p.expect_peek(TokenType.RPAREN) {
            return nil
        }
    } else if p.cur_token.type == TokenType.LBRACKET {
        left = p.parse_array_literal()
    } else {
        p.add_error("no prefix parse function for " + p.cur_token.literal)
        return nil
    }
    
    // Parse infix expressions
    for p.peek_token.type != TokenType.SEMICOLON && precedence < p.peek_precedence() {
        if p.peek_token.type == TokenType.PLUS ||
           p.peek_token.type == TokenType.MINUS ||
           p.peek_token.type == TokenType.MULTIPLY ||
           p.peek_token.type == TokenType.DIVIDE ||
           p.peek_token.type == TokenType.EQ ||
           p.peek_token.type == TokenType.NOT_EQ ||
           p.peek_token.type == TokenType.LT ||
           p.peek_token.type == TokenType.GT {
            
            p.next_token()
            operator := p.cur_token.literal
            prec := p.cur_precedence()
            p.next_token()
            right := p.parse_expression(prec)
            
            left = &InfixExpression{
                left: left,
                operator: operator,
                right: right,
            }
        } else if p.peek_token.type == TokenType.LPAREN {
            p.next_token()
            left = p.parse_call_expression(left)
        } else if p.peek_token.type == TokenType.LBRACKET {
            p.next_token()
            left = p.parse_index_expression(left)
        } else if p.peek_token.type == TokenType.ASSIGN {
            p.next_token()
            p.next_token()
            value := p.parse_expression(Precedence.LOWEST)
            
            if ident, ok := left.(*Identifier); ok {
                left = &AssignmentExpression{
                    name: *ident,
                    value: value,
                }
            } else {
                p.add_error("invalid assignment target")
                return nil
            }
        } else {
            break
        }
    }
    
    return left
}

// Parse array literal
func (p *Parser) parse_array_literal() Expression {
    elements := []Expression{}
    
    if p.peek_token.type == TokenType.RBRACKET {
        p.next_token()
        return &ArrayLiteral{elements: elements}
    }
    
    p.next_token()
    elements = append(elements, p.parse_expression(Precedence.LOWEST))
    
    for p.peek_token.type == TokenType.COMMA {
        p.next_token()
        p.next_token()
        elements = append(elements, p.parse_expression(Precedence.LOWEST))
    }
    
    if !p.expect_peek(TokenType.RBRACKET) {
        return nil
    }
    
    return &ArrayLiteral{elements: elements}
}

// Parse call expression
func (p *Parser) parse_call_expression(fn: Expression) Expression {
    args := p.parse_expression_list(TokenType.RPAREN)
    return &CallExpression{function: fn, arguments: args}
}

// Parse index expression
func (p *Parser) parse_index_expression(left: Expression) Expression {
    p.next_token()
    index := p.parse_expression(Precedence.LOWEST)
    
    if !p.expect_peek(TokenType.RBRACKET) {
        return nil
    }
    
    return &IndexExpression{left: left, index: index}
}

// Parse expression list (for function arguments)
func (p *Parser) parse_expression_list(end: TokenType) []Expression {
    args := []Expression{}
    
    if p.peek_token.type == end {
        p.next_token()
        return args
    }
    
    p.next_token()
    args = append(args, p.parse_expression(Precedence.LOWEST))
    
    for p.peek_token.type == TokenType.COMMA {
        p.next_token()
        p.next_token()
        args = append(args, p.parse_expression(Precedence.LOWEST))
    }
    
    if !p.expect_peek(end) {
        return nil
    }
    
    return args
}

// Get current token precedence
func (p *Parser) cur_precedence() Precedence {
    return p.token_precedence(p.cur_token.type)
}

// Get peek token precedence
func (p *Parser) peek_precedence() Precedence {
    return p.token_precedence(p.peek_token.type)
}

// Get token precedence
func (p *Parser) token_precedence(token_type: TokenType) Precedence {
    if token_type == TokenType.EQ || token_type == TokenType.NOT_EQ {
        return Precedence.EQUALS
    } else if token_type == TokenType.LT || token_type == TokenType.GT {
        return Precedence.LESSGREATER
    } else if token_type == TokenType.PLUS || token_type == TokenType.MINUS {
        return Precedence.SUM
    } else if token_type == TokenType.MULTIPLY || token_type == TokenType.DIVIDE {
        return Precedence.PRODUCT
    } else if token_type == TokenType.LPAREN {
        return Precedence.CALL
    } else if token_type == TokenType.LBRACKET {
        return Precedence.INDEX
    }
    
    return Precedence.LOWEST
}

// Expect peek token and advance
func (p *Parser) expect_peek(token_type: TokenType) bool {
    if p.peek_token.type == token_type {
        p.next_token()
        return true
    }
    
    p.add_error("expected " + string(token_type) + ", got " + string(p.peek_token.type))
    return false
}

// Consume optional semicolon
func (p *Parser) consume_semicolon() {
    if p.peek_token.type == TokenType.SEMICOLON {
        p.next_token()
    }
}

// Add parser error
func (p *Parser) add_error(msg: string) {
    error_msg := "Line " + string(p.cur_token.line) + ": " + msg
    p.errors = append(p.errors, error_msg)
}

// Check if parser has errors
func (p *Parser) has_errors() bool {
    return len(p.errors) > 0
}

// Get parser errors
func (p *Parser) get_errors() []string {
    return p.errors
}
