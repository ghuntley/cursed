// CURSED Stage 2 Parser
// Recursive descent parser for the CURSED programming language
// Converts tokens into Abstract Syntax Tree (AST)

vibe "cursed::stage2::parser";

yeet "std::collections";
yeet "cursed::stage2::lexer";
yeet "cursed::stage2::error";

// AST Node Types
enum NodeType {
    Program,
    Function,
    Variable,
    Block,
    IfStatement,
    WhileStatement,
    ForStatement,
    ReturnStatement,
    ExpressionStatement,
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    BooleanLiteral,
    Assignment,
    Struct,
    Interface,
    Package,
    Import,
}

// Base AST Node
collab ASTNode {
    slay node_type() -> NodeType;
    slay to_string() -> tea;
}

// Program node (root of AST)
squad Program {
    statements: ASTNode[],
}

impl ASTNode lowkey Program {
    slay node_type() -> NodeType {
        yolo NodeType::Program;
    }
    
    slay to_string() -> tea {
        sus result = "Program {\n";
        lowkey (sus stmt in statements) {
            result = result + "  " + stmt.to_string() + "\n";
        }
        result = result + "}";
        yolo result;
    }
}

// Function declaration
squad FunctionDecl {
    name: tea,
    parameters: Parameter[],
    return_type: tea,
    body: Block,
}

squad Parameter {
    name: tea,
    param_type: tea,
}

impl ASTNode lowkey FunctionDecl {
    slay node_type() -> NodeType {
        yolo NodeType::Function;
    }
    
    slay to_string() -> tea {
        sus params = "";
        lowkey (sus i = 0; i < parameters.length(); i++) {
            bestie (i > 0) {
                params = params + ", ";
            }
            params = params + parameters[i].name + ": " + parameters[i].param_type;
        }
        yolo "slay " + name + "(" + params + ") -> " + return_type + " " + body.to_string();
    }
}

// Variable declaration
squad VariableDecl {
    name: tea,
    var_type: tea,
    value: ASTNode,
    is_mutable: cap,
}

impl ASTNode lowkey VariableDecl {
    slay node_type() -> NodeType {
        yolo NodeType::Variable;
    }
    
    slay to_string() -> tea {
        sus keyword = bestie (is_mutable) { "sus" } highkey { "facts" };
        yolo keyword + " " + name + ": " + var_type + " = " + value.to_string();
    }
}

// Block statement
squad Block {
    statements: ASTNode[],
}

impl ASTNode lowkey Block {
    slay node_type() -> NodeType {
        yolo NodeType::Block;
    }
    
    slay to_string() -> tea {
        sus result = "{\n";
        lowkey (sus stmt in statements) {
            result = result + "  " + stmt.to_string() + "\n";
        }
        result = result + "}";
        yolo result;
    }
}

// If statement
squad IfStatement {
    condition: ASTNode,
    then_block: Block,
    else_block: Block?,
}

impl ASTNode lowkey IfStatement {
    slay node_type() -> NodeType {
        yolo NodeType::IfStatement;
    }
    
    slay to_string() -> tea {
        sus result = "lowkey (" + condition.to_string() + ") " + then_block.to_string();
        bestie (else_block != nocap) {
            result = result + " highkey " + else_block.to_string();
        }
        yolo result;
    }
}

// While statement
squad WhileStatement {
    condition: ASTNode,
    body: Block,
}

impl ASTNode lowkey WhileStatement {
    slay node_type() -> NodeType {
        yolo NodeType::WhileStatement;
    }
    
    slay to_string() -> tea {
        yolo "periodt (" + condition.to_string() + ") " + body.to_string();
    }
}

// Return statement
squad ReturnStatement {
    value: ASTNode?,
}

impl ASTNode lowkey ReturnStatement {
    slay node_type() -> NodeType {
        yolo NodeType::ReturnStatement;
    }
    
    slay to_string() -> tea {
        bestie (value != nocap) {
            yolo "yolo " + value.to_string();
        } highkey {
            yolo "yolo";
        }
    }
}

// Binary expression
squad BinaryExpression {
    left: ASTNode,
    operator: tea,
    right: ASTNode,
}

impl ASTNode lowkey BinaryExpression {
    slay node_type() -> NodeType {
        yolo NodeType::BinaryExpression;
    }
    
    slay to_string() -> tea {
        yolo "(" + left.to_string() + " " + operator + " " + right.to_string() + ")";
    }
}

// Identifier
squad Identifier {
    name: tea,
}

impl ASTNode lowkey Identifier {
    slay node_type() -> NodeType {
        yolo NodeType::Identifier;
    }
    
    slay to_string() -> tea {
        yolo name;
    }
}

// Integer literal
squad IntegerLiteral {
    value: normie,
}

impl ASTNode lowkey IntegerLiteral {
    slay node_type() -> NodeType {
        yolo NodeType::IntegerLiteral;
    }
    
    slay to_string() -> tea {
        yolo value.to_string();
    }
}

// String literal
squad StringLiteral {
    value: tea,
}

impl ASTNode lowkey StringLiteral {
    slay node_type() -> NodeType {
        yolo NodeType::StringLiteral;
    }
    
    slay to_string() -> tea {
        yolo "\"" + value + "\"";
    }
}

// Parser state
squad Parser {
    tokens: Token[],
    current: normie,
    errors: tea[],
}

// Create new parser
slay new_parser(tokens: Token[]) -> Parser {
    yolo Parser {
        tokens: tokens,
        current: 0,
        errors: tea[],
    };
}

// Check if we're at end of tokens
slay is_at_end(parser: Parser) -> cap {
    yolo parser.current >= parser.tokens.length() ||
          parser.tokens[parser.current].token_type == TokenType::Eof;
}

// Get current token
slay current_token(parser: Parser) -> Token {
    bestie (is_at_end(parser)) {
        yolo Token {
            token_type: TokenType::Eof,
            literal: "",
            line: 0,
            column: 0,
            position: 0,
        };
    }
    yolo parser.tokens[parser.current];
}

// Get previous token
slay previous_token(parser: Parser) -> Token {
    yolo parser.tokens[parser.current - 1];
}

// Advance to next token
slay advance(parser: Parser) -> Token {
    bestie (!is_at_end(parser)) {
        parser.current = parser.current + 1;
    }
    yolo previous_token(parser);
}

// Check if current token matches type
slay check(parser: Parser, token_type: TokenType) -> cap {
    bestie (is_at_end(parser)) {
        yolo facts;
    }
    yolo current_token(parser).token_type == token_type;
}

// Match any of the given token types
slay match_token(parser: Parser, types: TokenType[]) -> cap {
    lowkey (sus token_type in types) {
        bestie (check(parser, token_type)) {
            advance(parser);
            yolo truth;
        }
    }
    yolo facts;
}

// Consume token of expected type or error
slay consume(parser: Parser, token_type: TokenType, message: tea) -> Token? {
    bestie (check(parser, token_type)) {
        yolo advance(parser);
    }
    
    sus token = current_token(parser);
    sus error_msg = "Error at line " + token.line.to_string() + 
                   ", column " + token.column.to_string() + ": " + message;
    parser.errors.push(error_msg);
    yolo nocap;
}

// Parse the program (entry point)
slay parse(tokens: Token[]) -> Program? {
    sus parser = new_parser(tokens);
    yolo parse_program(parser);
}

// Parse program
slay parse_program(parser: Parser) -> Program? {
    sus statements = ASTNode[];
    
    periodt (!is_at_end(parser)) {
        sus stmt = parse_statement(parser)?;
        bestie (stmt != nocap) {
            statements.push(stmt);
        }
    }
    
    bestie (parser.errors.length() > 0) {
        error::report_errors(parser.errors);
        yolo nocap;
    }
    
    yolo Program {
        statements: statements,
    };
}

// Parse statement
slay parse_statement(parser: Parser) -> ASTNode? {
    vibe_check (current_token(parser).token_type) {
        mood TokenType::Slay {
            yolo parse_function(parser);
        }
        
        mood TokenType::Sus, TokenType::Facts {
            yolo parse_variable_declaration(parser);
        }
        
        mood TokenType::Lowkey {
            yolo parse_if_statement(parser);
        }
        
        mood TokenType::Periodt {
            yolo parse_while_statement(parser);
        }
        
        mood TokenType::Yolo {
            yolo parse_return_statement(parser);
        }
        
        mood TokenType::LeftBrace {
            yolo parse_block(parser);
        }
        
        basic {
            yolo parse_expression_statement(parser);
        }
    }
}

// Parse function declaration
slay parse_function(parser: Parser) -> FunctionDecl? {
    consume(parser, TokenType::Slay, "Expected 'slay'");
    
    sus name_token = consume(parser, TokenType::Identifier, "Expected function name");
    bestie (name_token == nocap) {
        yolo nocap;
    }
    
    consume(parser, TokenType::LeftParen, "Expected '(' after function name");
    
    sus parameters = Parameter[];
    
    bestie (!check(parser, TokenType::RightParen)) {
        periodt (truth) {
            sus param_name = consume(parser, TokenType::Identifier, "Expected parameter name");
            bestie (param_name == nocap) {
                yolo nocap;
            }
            
            consume(parser, TokenType::Colon, "Expected ':' after parameter name");
            
            sus param_type = consume(parser, TokenType::Identifier, "Expected parameter type");
            bestie (param_type == nocap) {
                yolo nocap;
            }
            
            parameters.push(Parameter {
                name: param_name.literal,
                param_type: param_type.literal,
            });
            
            bestie (!match_token(parser, [TokenType::Comma])) {
                ghosted;
            }
        }
    }
    
    consume(parser, TokenType::RightParen, "Expected ')' after parameters");
    
    // Parse return type
    sus return_type = "void";
    bestie (match_token(parser, [TokenType::Arrow])) {
        sus type_token = consume(parser, TokenType::Identifier, "Expected return type");
        bestie (type_token != nocap) {
            return_type = type_token.literal;
        }
    }
    
    sus body = parse_block(parser)?;
    bestie (body == nocap) {
        yolo nocap;
    }
    
    yolo FunctionDecl {
        name: name_token.literal,
        parameters: parameters,
        return_type: return_type,
        body: body,
    };
}

// Parse variable declaration
slay parse_variable_declaration(parser: Parser) -> VariableDecl? {
    sus is_mutable = current_token(parser).token_type == TokenType::Sus;
    advance(parser); // consume 'sus' or 'facts'
    
    sus name_token = consume(parser, TokenType::Identifier, "Expected variable name");
    bestie (name_token == nocap) {
        yolo nocap;
    }
    
    // Optional type annotation
    sus var_type = "auto";
    bestie (match_token(parser, [TokenType::Colon])) {
        sus type_token = consume(parser, TokenType::Identifier, "Expected type");
        bestie (type_token != nocap) {
            var_type = type_token.literal;
        }
    }
    
    consume(parser, TokenType::Assign, "Expected '=' in variable declaration");
    
    sus value = parse_expression(parser)?;
    bestie (value == nocap) {
        yolo nocap;
    }
    
    consume(parser, TokenType::Semicolon, "Expected ';' after variable declaration");
    
    yolo VariableDecl {
        name: name_token.literal,
        var_type: var_type,
        value: value,
        is_mutable: is_mutable,
    };
}

// Parse if statement
slay parse_if_statement(parser: Parser) -> IfStatement? {
    consume(parser, TokenType::Lowkey, "Expected 'lowkey'");
    consume(parser, TokenType::LeftParen, "Expected '(' after 'lowkey'");
    
    sus condition = parse_expression(parser)?;
    bestie (condition == nocap) {
        yolo nocap;
    }
    
    consume(parser, TokenType::RightParen, "Expected ')' after if condition");
    
    sus then_block = parse_block(parser)?;
    bestie (then_block == nocap) {
        yolo nocap;
    }
    
    sus else_block = nocap;
    bestie (match_token(parser, [TokenType::Highkey])) {
        else_block = parse_block(parser)?;
    }
    
    yolo IfStatement {
        condition: condition,
        then_block: then_block,
        else_block: else_block,
    };
}

// Parse while statement
slay parse_while_statement(parser: Parser) -> WhileStatement? {
    consume(parser, TokenType::Periodt, "Expected 'periodt'");
    consume(parser, TokenType::LeftParen, "Expected '(' after 'periodt'");
    
    sus condition = parse_expression(parser)?;
    bestie (condition == nocap) {
        yolo nocap;
    }
    
    consume(parser, TokenType::RightParen, "Expected ')' after while condition");
    
    sus body = parse_block(parser)?;
    bestie (body == nocap) {
        yolo nocap;
    }
    
    yolo WhileStatement {
        condition: condition,
        body: body,
    };
}

// Parse return statement
slay parse_return_statement(parser: Parser) -> ReturnStatement? {
    consume(parser, TokenType::Yolo, "Expected 'yolo'");
    
    sus value = nocap;
    bestie (!check(parser, TokenType::Semicolon)) {
        value = parse_expression(parser)?;
    }
    
    consume(parser, TokenType::Semicolon, "Expected ';' after return statement");
    
    yolo ReturnStatement {
        value: value,
    };
}

// Parse block statement
slay parse_block(parser: Parser) -> Block? {
    consume(parser, TokenType::LeftBrace, "Expected '{'");
    
    sus statements = ASTNode[];
    
    periodt (!check(parser, TokenType::RightBrace) && !is_at_end(parser)) {
        sus stmt = parse_statement(parser)?;
        bestie (stmt != nocap) {
            statements.push(stmt);
        }
    }
    
    consume(parser, TokenType::RightBrace, "Expected '}'");
    
    yolo Block {
        statements: statements,
    };
}

// Parse expression statement
slay parse_expression_statement(parser: Parser) -> ASTNode? {
    sus expr = parse_expression(parser)?;
    consume(parser, TokenType::Semicolon, "Expected ';' after expression");
    yolo expr;
}

// Parse expression
slay parse_expression(parser: Parser) -> ASTNode? {
    yolo parse_logical_or(parser);
}

// Parse logical OR
slay parse_logical_or(parser: Parser) -> ASTNode? {
    sus expr = parse_logical_and(parser)?;
    
    periodt (match_token(parser, [TokenType::LogicalOr])) {
        sus operator = previous_token(parser).literal;
        sus right = parse_logical_and(parser)?;
        bestie (right == nocap) {
            yolo nocap;
        }
        expr = BinaryExpression {
            left: expr,
            operator: operator,
            right: right,
        };
    }
    
    yolo expr;
}

// Parse logical AND
slay parse_logical_and(parser: Parser) -> ASTNode? {
    sus expr = parse_equality(parser)?;
    
    periodt (match_token(parser, [TokenType::LogicalAnd])) {
        sus operator = previous_token(parser).literal;
        sus right = parse_equality(parser)?;
        bestie (right == nocap) {
            yolo nocap;
        }
        expr = BinaryExpression {
            left: expr,
            operator: operator,
            right: right,
        };
    }
    
    yolo expr;
}

// Parse equality
slay parse_equality(parser: Parser) -> ASTNode? {
    sus expr = parse_comparison(parser)?;
    
    periodt (match_token(parser, [TokenType::Equal, TokenType::NotEqual])) {
        sus operator = previous_token(parser).literal;
        sus right = parse_comparison(parser)?;
        bestie (right == nocap) {
            yolo nocap;
        }
        expr = BinaryExpression {
            left: expr,
            operator: operator,
            right: right,
        };
    }
    
    yolo expr;
}

// Parse comparison
slay parse_comparison(parser: Parser) -> ASTNode? {
    sus expr = parse_term(parser)?;
    
    periodt (match_token(parser, [TokenType::GreaterThan, TokenType::GreaterThanEqual,
                                  TokenType::LessThan, TokenType::LessThanEqual])) {
        sus operator = previous_token(parser).literal;
        sus right = parse_term(parser)?;
        bestie (right == nocap) {
            yolo nocap;
        }
        expr = BinaryExpression {
            left: expr,
            operator: operator,
            right: right,
        };
    }
    
    yolo expr;
}

// Parse term (addition/subtraction)
slay parse_term(parser: Parser) -> ASTNode? {
    sus expr = parse_factor(parser)?;
    
    periodt (match_token(parser, [TokenType::Minus, TokenType::Plus])) {
        sus operator = previous_token(parser).literal;
        sus right = parse_factor(parser)?;
        bestie (right == nocap) {
            yolo nocap;
        }
        expr = BinaryExpression {
            left: expr,
            operator: operator,
            right: right,
        };
    }
    
    yolo expr;
}

// Parse factor (multiplication/division)
slay parse_factor(parser: Parser) -> ASTNode? {
    sus expr = parse_unary(parser)?;
    
    periodt (match_token(parser, [TokenType::Divide, TokenType::Multiply, TokenType::Modulo])) {
        sus operator = previous_token(parser).literal;
        sus right = parse_unary(parser)?;
        bestie (right == nocap) {
            yolo nocap;
        }
        expr = BinaryExpression {
            left: expr,
            operator: operator,
            right: right,
        };
    }
    
    yolo expr;
}

// Parse unary expression
slay parse_unary(parser: Parser) -> ASTNode? {
    bestie (match_token(parser, [TokenType::Not, TokenType::Minus])) {
        sus operator = previous_token(parser).literal;
        sus right = parse_unary(parser)?;
        bestie (right == nocap) {
            yolo nocap;
        }
        // For now, create a binary expression with null left operand
        yolo BinaryExpression {
            left: nocap,
            operator: operator,
            right: right,
        };
    }
    
    yolo parse_primary(parser);
}

// Parse primary expression
slay parse_primary(parser: Parser) -> ASTNode? {
    vibe_check (current_token(parser).token_type) {
        mood TokenType::Truth, TokenType::NoTruth {
            sus token = advance(parser);
            yolo StringLiteral {
                value: token.literal,
            };
        }
        
        mood TokenType::Integer {
            sus token = advance(parser);
            sus value = token.literal.parse_int()?;
            yolo IntegerLiteral {
                value: value,
            };
        }
        
        mood TokenType::String {
            sus token = advance(parser);
            yolo StringLiteral {
                value: token.literal,
            };
        }
        
        mood TokenType::Identifier {
            sus token = advance(parser);
            yolo Identifier {
                name: token.literal,
            };
        }
        
        mood TokenType::LeftParen {
            advance(parser); // consume '('
            sus expr = parse_expression(parser)?;
            consume(parser, TokenType::RightParen, "Expected ')' after expression");
            yolo expr;
        }
        
        basic {
            sus token = current_token(parser);
            sus error_msg = "Unexpected token: " + token.literal + 
                           " at line " + token.line.to_string();
            parser.errors.push(error_msg);
            yolo nocap;
        }
    }
}
