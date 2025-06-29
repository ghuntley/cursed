// Standalone test for dot operator parsing

// Minimal implementations for testing
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    String,
    LeftParen,
    RightParen,
    Dot,
    Comma,
    Eof,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    String(String),
    MemberAccess(MemberAccessExpression),
    Call(CallExpression),
}

#[derive(Debug, Clone)]
pub struct MemberAccessExpression {
    pub object: Box<Expression>,
    pub property: String,
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub enum CursedError {
    SyntaxError(String),
}

impl CursedError {
    pub fn syntax_error(msg: &str) -> Self {
        CursedError::SyntaxError(msg.to_string())
    }
}

pub struct Lexer {
    input: Vec<char>,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() { break; }
            
            let start = self.current;
            let c = self.advance();
            
            match c {
                '.' => tokens.push(Token { kind: TokenKind::Dot, lexeme: ".".to_string(), line: self.line }),
                '(' => tokens.push(Token { kind: TokenKind::LeftParen, lexeme: "(".to_string(), line: self.line }),
                ')' => tokens.push(Token { kind: TokenKind::RightParen, lexeme: ")".to_string(), line: self.line }),
                ',' => tokens.push(Token { kind: TokenKind::Comma, lexeme: ",".to_string(), line: self.line }),
                '"' => {
                    let string_val = self.string();
                    tokens.push(Token { kind: TokenKind::String, lexeme: string_val, line: self.line });
                }
                _ if c.is_alphabetic() || c == '_' => {
                    let identifier = self.identifier(start);
                    tokens.push(Token { kind: TokenKind::Identifier, lexeme: identifier, line: self.line });
                }
                _ => {} // Skip unknown characters
            }
        }
        
        tokens.push(Token { kind: TokenKind::Eof, lexeme: "".to_string(), line: self.line });
        tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let c = self.input[self.current];
            self.current += 1;
            if c == '\n' {
                self.line += 1;
            }
            c
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.input[self.current].is_whitespace() {
            self.advance();
        }
    }

    fn identifier(&mut self, start: usize) -> String {
        while !self.is_at_end() && (self.input[self.current].is_alphanumeric() || self.input[self.current] == '_') {
            self.advance();
        }
        self.input[start..self.current].iter().collect()
    }

    fn string(&mut self) -> String {
        let mut value = String::new();
        while !self.is_at_end() && self.input[self.current] != '"' {
            value.push(self.advance());
        }
        if !self.is_at_end() {
            self.advance(); // consume closing "
        }
        value
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, CursedError> {
        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expression, CursedError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&TokenKind::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&TokenKind::Dot) {
                let property = self.consume(TokenKind::Identifier, "Expected property name after '.'")?;
                expr = Expression::MemberAccess(MemberAccessExpression {
                    object: Box::new(expr),
                    property: property.lexeme.clone(),
                });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, CursedError> {
        let mut arguments = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            loop {
                arguments.push(self.parse_expression()?);
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightParen, "Expected ')' after arguments")?;

        Ok(Expression::Call(CallExpression {
            function: Box::new(callee),
            arguments,
        }))
    }

    fn parse_primary(&mut self) -> Result<Expression, CursedError> {
        if self.match_token(&TokenKind::Identifier) {
            let token = self.previous();
            return Ok(Expression::Identifier(token.lexeme.clone()));
        }

        if self.match_token(&TokenKind::String) {
            let token = self.previous();
            return Ok(Expression::String(token.lexeme.clone()));
        }

        Err(CursedError::syntax_error("Expected expression"))
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() { return false; }
        &self.peek().kind == kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, CursedError> {
        if self.check(&kind) {
            return Ok(self.advance());
        }
        Err(CursedError::syntax_error(message))
    }
}

fn test_case(source: &str, description: &str) {
    println!("\n🧪 Testing: {}", description);
    println!("Source: {}", source);
    
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.scan_tokens();
    
    println!("Tokens: {:?}", tokens.iter().map(|t| format!("{}:{}", t.lexeme, format!("{:?}", t.kind))).collect::<Vec<_>>());
    
    let mut parser = Parser::new(tokens);
    match parser.parse_expression() {
        Ok(ast) => {
            println!("✅ Success! AST: {:#?}", ast);
        }
        Err(error) => {
            println!("❌ Error: {:?}", error);
        }
    }
}

fn main() {
    println!("🚀 Testing dot operator parsing for CURSED language");
    println!("{}", "=".repeat(50));

    // Test simple member access
    test_case("vibez.spill", "Simple member access");
    
    // Test member access with function call
    test_case("vibez.spill(\"hello\")", "Member access with function call");
    
    // Test chained member access
    test_case("obj.member.method", "Chained member access");
    
    // Test chained member access with call
    test_case("obj.member.method()", "Chained member access with call");
    
    // Test complex case
    test_case("console.log(\"test\", value)", "Complex member access call");

    println!("\n{}", "=".repeat(50));
    println!("✨ Dot operator parsing test complete!");
}
