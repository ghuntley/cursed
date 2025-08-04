fr fr CURSED Parser Implementation
fr fr Migrated from parser.zig to pure CURSED

yeet "ast"
yeet "lexer"
yeet "testz"

squad ParserError {
    spill message tea
    spill position normie
}

squad Parser {
    spill tokens []Token
    spill current normie
    spill had_error lit
    spill in_function lit
    spill in_loop lit
    spill scope_depth normie
}

slay createParser(tokens []Token) Parser {
    damn Parser{
        tokens: tokens,
        current: 0,
        had_error: cringe,
        in_function: cringe,
        in_loop: cringe,
        scope_depth: 0
    }
}

slay parseProgram(parser *Parser) Program {
    sus program Program = createProgram()
    
    bestie !isAtEnd(parser) {
        fr fr Skip newlines and semicolons
        if isNewlineOrSemicolon(parser) {
            advance(parser)
            vibes
        }
        
        fr fr Parse package declaration
        if checkVibe(parser) {
            program.package = parsePackageDeclaration(parser)
            vibes
        }
        
        fr fr Parse import statement
        if checkYeet(parser) {
            sus import_stmt ImportStatement = parseImportStatement(parser)
            program.imports.push(import_stmt)
            vibes
        }
        
        fr fr Parse regular statements
        sus stmt Statement = parseStatement(parser)
        program.statements.push(stmt)
    }
    
    damn program
}

slay parsePackageDeclaration(parser *Parser) PackageDeclaration {
    consume(parser, "vibe", "Expected 'vibe'")
    
    if !checkIdentifier(parser) {
        yikes "Expected package name after 'vibe'"
    }
    
    sus name tea = advance(parser).lexeme
    
    damn PackageDeclaration{
        name: name,
        version: none
    }
}

slay parseImportStatement(parser *Parser) ImportStatement {
    consume(parser, "yeet", "Expected 'yeet'")
    
    if !checkStringLiteral(parser) {
        yikes "Expected string literal after 'yeet'"
    }
    
    sus path_token Token = advance(parser)
    sus path tea = stripQuotes(path_token.lexeme)
    
    sus import_stmt ImportStatement = createImportStatement(path)
    
    fr fr Handle alias (as name)
    if matchKeyword(parser, "as") {
        if checkIdentifier(parser) {
            import_stmt.alias = advance(parser).lexeme
        }
    }
    
    damn import_stmt
}

slay parseStatement(parser *Parser) Statement {
    fr fr Function declaration (slay)
    if checkKeyword(parser, "slay") {
        damn parseFunctionStatement(parser)
    }
    
    fr fr Variable declaration (sus/facts)
    if checkKeyword(parser, "sus") or checkKeyword(parser, "facts") {
        damn parseLetStatement(parser)
    }
    
    fr fr Return statement (damn)
    if checkKeyword(parser, "damn") {
        damn parseReturnStatement(parser)
    }
    
    fr fr If statement (lowkey)
    if checkKeyword(parser, "lowkey") {
        damn parseIfStatement(parser)
    }
    
    fr fr While statement (bestie)
    if checkKeyword(parser, "bestie") {
        damn parseWhileStatement(parser)
    }
    
    fr fr Break/continue
    if checkKeyword(parser, "ghosted") {
        advance(parser)
        damn createBreakStatement()
    }
    
    if checkKeyword(parser, "simp") {
        advance(parser)
        damn createContinueStatement()
    }
    
    fr fr Struct declaration (squad)
    if checkKeyword(parser, "squad") {
        damn parseStructStatement(parser)
    }
    
    fr fr Interface declaration (collab)
    if checkKeyword(parser, "collab") {
        damn parseInterfaceStatement(parser)
    }
    
    fr fr Goroutine statement (stan)
    if checkKeyword(parser, "stan") {
        damn parseGoroutineStatement(parser)
    }
    
    fr fr Match expression
    if checkKeyword(parser, "match") {
        sus match_expr Expression = parseMatchExpression(parser)
        damn createExpressionStatement(match_expr)
    }
    
    fr fr Short variable declaration (x := value)
    if isShortDeclaration(parser) {
        damn parseShortDeclaration(parser)
    }
    
    fr fr Assignment statement
    if isAssignment(parser) {
        damn parseAssignmentStatement(parser)
    }
    
    fr fr Expression statement
    sus expr Expression = parseExpression(parser)
    damn createExpressionStatement(expr)
}

slay parseFunctionStatement(parser *Parser) FunctionStatement {
    consume(parser, "slay", "Expected 'slay'")
    
    if !checkIdentifier(parser) {
        yikes "Expected function name after 'slay'"
    }
    
    sus name tea = advance(parser).lexeme
    sus func FunctionStatement = createFunctionStatement(name)
    
    fr fr Parse generic type parameters <T, U>
    if matchOperator(parser, "<") {
        bestie !checkOperator(parser, ">") and !isAtEnd(parser) {
            if checkIdentifier(parser) {
                sus param_name tea = advance(parser).lexeme
                sus type_param TypeParameter = createTypeParameter(param_name)
                
                fr fr Parse constraints (T: SomeInterface)
                if matchOperator(parser, ":") {
                    bestie !checkOperator(parser, ",") and !checkOperator(parser, ">") {
                        sus constraint Type = parseType(parser)
                        type_param.constraints.push(constraint)
                        if !matchOperator(parser, "+") {
                            vibes
                        }
                    }
                }
                
                func.type_parameters.push(type_param)
            }
            
            if !matchOperator(parser, ",") {
                vibes
            }
        }
        
        if !matchOperator(parser, ">") {
            yikes "Expected '>' after type parameters"
        }
    }
    
    fr fr Parse parameters
    consume(parser, "(", "Expected '(' after function name")
    
    if !checkOperator(parser, ")") {
        bestie based {
            sus param Parameter = parseParameter(parser)
            func.parameters.push(param)
            
            if !matchOperator(parser, ",") {
                vibes
            }
        }
    }
    
    consume(parser, ")", "Expected ')' after parameters")
    
    fr fr Parse return type
    if !checkOperator(parser, "{") {
        func.return_type = parseType(parser)
    }
    
    fr fr Parse body
    consume(parser, "{", "Expected '{'")
    
    parser.in_function = based
    
    bestie !checkOperator(parser, "}") and !isAtEnd(parser) {
        fr fr Skip newlines
        if matchNewline(parser) {
            vibes
        }
        
        sus stmt Statement = parseStatement(parser)
        func.body.push(stmt)
    }
    
    parser.in_function = cringe
    
    consume(parser, "}", "Expected '}'")
    
    damn func
}

slay parseLetStatement(parser *Parser) LetStatement {
    sus is_mutable lit = matchKeyword(parser, "sus")
    if !is_mutable {
        matchKeyword(parser, "facts")
    }
    
    if !checkIdentifier(parser) {
        yikes "Expected variable name"
    }
    
    sus name tea = advance(parser).lexeme
    
    sus let_stmt LetStatement = createLetStatement(name, is_mutable)
    
    fr fr Parse type annotation
    if matchOperator(parser, ":") {
        let_stmt.var_type = parseType(parser)
    } else if checkType(parser) {
        let_stmt.var_type = parseType(parser)
    }
    
    fr fr Parse initializer
    if matchOperator(parser, "=") or matchOperator(parser, ":=") {
        let_stmt.initializer = parseExpression(parser)
    }
    
    damn let_stmt
}

slay parseParameter(parser *Parser) Parameter {
    if !checkIdentifier(parser) {
        yikes "Expected parameter name"
    }
    
    sus name tea = advance(parser).lexeme
    
    fr fr Parse type (required for parameters in CURSED)
    sus param_type Type = parseType(parser)
    
    sus param Parameter = createParameter(name, param_type)
    
    fr fr Parse default value
    if matchOperator(parser, "=") {
        param.default_value = parseExpression(parser)
    }
    
    damn param
}

slay parseType(parser *Parser) Type {
    fr fr Array/slice types []element_type or [size]element_type
    if matchOperator(parser, "[") {
        sus size normie = -1
        if !checkOperator(parser, "]") {
            if checkNumber(parser) {
                sus size_token Token = advance(parser)
                size = parseInt(size_token.lexeme)
            }
        }
        
        consume(parser, "]", "Expected ']'")
        
        sus element_type Type = parseType(parser)
        
        damn createArrayType(element_type, size)
    }
    
    fr fr Map types map[key_type]value_type
    if matchKeyword(parser, "map") {
        consume(parser, "[", "Expected '[' after 'map'")
        
        sus key_type Type = parseType(parser)
        
        consume(parser, "]", "Expected ']'")
        
        sus value_type Type = parseType(parser)
        
        damn createMapType(key_type, value_type)
    }
    
    fr fr Channel types dm<element_type>
    if checkKeyword(parser, "dm") or matchKeyword(parser, "dm") {
        advance(parser)
        if matchOperator(parser, "<") {
            sus element_type Type = parseType(parser)
            
            consume(parser, ">", "Expected '>' after channel element type")
            
            damn createChannelType(element_type)
        }
    }
    
    fr fr Primitive types
    if matchKeyword(parser, "normie") {
        damn createPrimitiveType("normie")
    }
    if matchKeyword(parser, "tea") {
        damn createPrimitiveType("tea")
    }
    if matchKeyword(parser, "lit") {
        damn createPrimitiveType("lit")
    }
    if matchKeyword(parser, "meal") {
        damn createPrimitiveType("meal")
    }
    if matchKeyword(parser, "smol") {
        damn createPrimitiveType("smol")
    }
    if matchKeyword(parser, "thicc") {
        damn createPrimitiveType("thicc")
    }
    
    fr fr Identifier type
    if checkIdentifier(parser) {
        sus name tea = advance(parser).lexeme
        damn createIdentifierType(name)
    }
    
    yikes "Expected type"
}

slay parseExpression(parser *Parser) Expression {
    damn parseOrExpression(parser)
}

slay parseOrExpression(parser *Parser) Expression {
    sus expr Expression = parseAndExpression(parser)
    
    bestie matchOperator(parser, "||") or matchKeyword(parser, "or") {
        sus operator tea = previous(parser).lexeme
        sus right Expression = parseAndExpression(parser)
        expr = createBinaryExpression(expr, operator, right)
    }
    
    damn expr
}

slay parseAndExpression(parser *Parser) Expression {
    sus expr Expression = parseEqualityExpression(parser)
    
    bestie matchOperator(parser, "&&") or matchKeyword(parser, "and") {
        sus operator tea = previous(parser).lexeme
        sus right Expression = parseEqualityExpression(parser)
        expr = createBinaryExpression(expr, operator, right)
    }
    
    damn expr
}

slay parseEqualityExpression(parser *Parser) Expression {
    sus expr Expression = parseComparisonExpression(parser)
    
    bestie matchOperator(parser, "==") or matchOperator(parser, "!=") {
        sus operator tea = previous(parser).lexeme
        sus right Expression = parseComparisonExpression(parser)
        expr = createBinaryExpression(expr, operator, right)
    }
    
    damn expr
}

slay parseComparisonExpression(parser *Parser) Expression {
    sus expr Expression = parseTermExpression(parser)
    
    bestie matchOperator(parser, ">") or matchOperator(parser, ">=") or 
          matchOperator(parser, "<") or matchOperator(parser, "<=") {
        sus operator tea = previous(parser).lexeme
        sus right Expression = parseTermExpression(parser)
        expr = createBinaryExpression(expr, operator, right)
    }
    
    damn expr
}

slay parseTermExpression(parser *Parser) Expression {
    sus expr Expression = parseFactorExpression(parser)
    
    bestie matchOperator(parser, "+") or matchOperator(parser, "-") {
        sus operator tea = previous(parser).lexeme
        sus right Expression = parseFactorExpression(parser)
        expr = createBinaryExpression(expr, operator, right)
    }
    
    damn expr
}

slay parseFactorExpression(parser *Parser) Expression {
    sus expr Expression = parseUnaryExpression(parser)
    
    bestie matchOperator(parser, "*") or matchOperator(parser, "/") or matchOperator(parser, "%") {
        sus operator tea = previous(parser).lexeme
        sus right Expression = parseUnaryExpression(parser)
        expr = createBinaryExpression(expr, operator, right)
    }
    
    damn expr
}

slay parseUnaryExpression(parser *Parser) Expression {
    if matchOperator(parser, "!") or matchOperator(parser, "-") or matchOperator(parser, "+") {
        sus operator tea = previous(parser).lexeme
        sus expr Expression = parseUnaryExpression(parser)
        damn createUnaryExpression(operator, expr)
    }
    
    damn parseCallExpression(parser)
}

slay parseCallExpression(parser *Parser) Expression {
    sus expr Expression = parsePrimaryExpression(parser)
    
    bestie based {
        if matchOperator(parser, "(") {
            sus arguments []Expression = []
            
            if !checkOperator(parser, ")") {
                bestie based {
                    sus arg Expression = parseExpression(parser)
                    arguments.push(arg)
                    
                    if !matchOperator(parser, ",") {
                        vibes
                    }
                }
            }
            
            consume(parser, ")", "Expected ')' after arguments")
            
            expr = createCallExpression(expr, arguments)
        } else if matchOperator(parser, ".") {
            if !checkIdentifier(parser) {
                yikes "Expected property name after '.'"
            }
            
            sus member_name tea = advance(parser).lexeme
            expr = createMemberAccessExpression(expr, member_name)
        } else if matchOperator(parser, "[") {
            sus index Expression = parseExpression(parser)
            consume(parser, "]", "Expected ']' after array index")
            expr = createArrayAccessExpression(expr, index)
        } else {
            vibes
        }
    }
    
    damn expr
}

slay parsePrimaryExpression(parser *Parser) Expression {
    fr fr Boolean literals
    if matchKeyword(parser, "based") {
        damn createBooleanExpression(based)
    }
    
    if matchKeyword(parser, "cringe") {
        damn createBooleanExpression(cringe)
    }
    
    fr fr Number literals
    if checkNumber(parser) {
        sus token Token = advance(parser)
        damn createNumberExpression(token.lexeme)
    }
    
    fr fr String literals
    if checkStringLiteral(parser) {
        sus token Token = advance(parser)
        sus value tea = stripQuotes(token.lexeme)
        damn createStringExpression(value)
    }
    
    fr fr Identifiers
    if checkIdentifier(parser) {
        sus name tea = advance(parser).lexeme
        damn createIdentifierExpression(name)
    }
    
    fr fr Grouped expressions
    if matchOperator(parser, "(") {
        sus expr Expression = parseExpression(parser)
        consume(parser, ")", "Expected ')' after expression")
        damn expr
    }
    
    fr fr Array literals
    if matchOperator(parser, "[") {
        sus elements []Expression = []
        
        if !checkOperator(parser, "]") {
            bestie based {
                sus element Expression = parseExpression(parser)
                elements.push(element)
                
                if !matchOperator(parser, ",") {
                    vibes
                }
            }
        }
        
        consume(parser, "]", "Expected ']' after array elements")
        damn createArrayExpression(elements)
    }
    
    yikes "Unexpected token in expression"
}

fr fr Utility functions
slay advance(parser *Parser) Token {
    if !isAtEnd(parser) {
        parser.current = parser.current + 1
    }
    damn previous(parser)
}

slay isAtEnd(parser *Parser) lit {
    damn peek(parser).kind == "EOF"
}

slay peek(parser *Parser) Token {
    if parser.current >= parser.tokens.length {
        damn Token{kind: "EOF", lexeme: "", line: 0, column: 0}
    }
    damn parser.tokens[parser.current]
}

slay previous(parser *Parser) Token {
    if parser.current == 0 {
        damn parser.tokens[0]
    }
    damn parser.tokens[parser.current - 1]
}

slay checkKeyword(parser *Parser, keyword tea) lit {
    sus token Token = peek(parser)
    damn token.kind == "IDENTIFIER" and token.lexeme == keyword
}

slay matchKeyword(parser *Parser, keyword tea) lit {
    if checkKeyword(parser, keyword) {
        advance(parser)
        damn based
    }
    damn cringe
}

slay checkOperator(parser *Parser, operator tea) lit {
    sus token Token = peek(parser)
    damn token.lexeme == operator
}

slay matchOperator(parser *Parser, operator tea) lit {
    if checkOperator(parser, operator) {
        advance(parser)
        damn based
    }
    damn cringe
}

slay checkIdentifier(parser *Parser) lit {
    damn peek(parser).kind == "IDENTIFIER"
}

slay checkNumber(parser *Parser) lit {
    damn peek(parser).kind == "NUMBER"
}

slay checkStringLiteral(parser *Parser) lit {
    damn peek(parser).kind == "STRING"
}

slay checkType(parser *Parser) lit {
    sus token Token = peek(parser)
    damn token.kind == "IDENTIFIER" and (
        token.lexeme == "normie" or
        token.lexeme == "tea" or
        token.lexeme == "lit" or
        token.lexeme == "meal" or
        token.lexeme == "smol" or
        token.lexeme == "thicc" or
        token.lexeme == "map" or
        token.lexeme == "dm"
    )
}

slay consume(parser *Parser, expected tea, message tea) {
    if peek(parser).lexeme == expected {
        advance(parser)
        damn
    }
    
    vibez.spill("Parser error: " + message)
    parser.had_error = based
    yikes message
}

slay stripQuotes(str tea) tea {
    if str.length >= 2 and str[0] == '"' and str[str.length - 1] == '"' {
        damn str.substring(1, str.length - 1)
    }
    damn str
}

slay parseInt(str tea) normie {
    fr fr Simple integer parsing - would use stdlib in full implementation
    sus result normie = 0
    sus i normie = 0
    bestie i < str.length {
        if str[i] >= '0' and str[i] <= '9' {
            result = result * 10 + (str[i] - '0')
        } else {
            vibes
        }
        i = i + 1
    }
    damn result
}

slay isShortDeclaration(parser *Parser) lit {
    fr fr Look ahead for := pattern
    sus pos normie = parser.current
    
    fr fr Handle single or multiple identifiers: a, b := ...
    bestie pos < parser.tokens.length and parser.tokens[pos].kind == "IDENTIFIER" {
        pos = pos + 1
        if pos < parser.tokens.length and parser.tokens[pos].lexeme == "," {
            pos = pos + 1
        } else {
            vibes
        }
    }
    
    damn pos < parser.tokens.length and parser.tokens[pos].lexeme == ":="
}

slay isAssignment(parser *Parser) lit {
    fr fr Simple assignment detection
    sus pos normie = parser.current
    
    fr fr Skip over primary expression tokens
    bestie pos < parser.tokens.length {
        sus token_lexeme tea = parser.tokens[pos].lexeme
        if token_lexeme == "=" or token_lexeme == "+=" or
           token_lexeme == "-=" or token_lexeme == "*=" or
           token_lexeme == "/=" or token_lexeme == "%=" {
            damn based
        }
        if token_lexeme == ";" or token_lexeme == "\n" or
           token_lexeme == "{" or token_lexeme == "}" {
            damn cringe
        }
        pos = pos + 1
    }
    
    damn cringe
}

slay isNewlineOrSemicolon(parser *Parser) lit {
    sus token Token = peek(parser)
    damn token.lexeme == "\n" or token.lexeme == ";"
}

slay checkVibe(parser *Parser) lit {
    damn checkKeyword(parser, "vibe")
}

slay checkYeet(parser *Parser) lit {
    damn checkKeyword(parser, "yeet")
}

slay matchNewline(parser *Parser) lit {
    damn matchOperator(parser, "\n")
}

fr fr Additional parsing methods for completeness
slay parseReturnStatement(parser *Parser) Statement {
    consume(parser, "damn", "Expected 'damn'")
    
    sus value Expression = none
    if !isNewlineOrSemicolon(parser) and !checkOperator(parser, "}") {
        value = parseExpression(parser)
    }
    
    damn createReturnStatement(value)
}

slay parseIfStatement(parser *Parser) Statement {
    consume(parser, "lowkey", "Expected 'lowkey'")
    
    sus condition Expression = parseExpression(parser)
    
    consume(parser, "{", "Expected '{' after if condition")
    
    sus then_body []Statement = []
    bestie !checkOperator(parser, "}") and !isAtEnd(parser) {
        if matchNewline(parser) {
            vibes
        }
        
        sus stmt Statement = parseStatement(parser)
        then_body.push(stmt)
    }
    
    consume(parser, "}", "Expected '}' after if body")
    
    sus else_body []Statement = none
    if matchKeyword(parser, "else") {
        consume(parser, "{", "Expected '{' after else")
        
        else_body = []
        bestie !checkOperator(parser, "}") and !isAtEnd(parser) {
            if matchNewline(parser) {
                vibes
            }
            
            sus stmt Statement = parseStatement(parser)
            else_body.push(stmt)
        }
        
        consume(parser, "}", "Expected '}' after else body")
    }
    
    damn createIfStatement(condition, then_body, else_body)
}

slay parseWhileStatement(parser *Parser) Statement {
    consume(parser, "bestie", "Expected 'bestie'")
    
    sus condition Expression = parseExpression(parser)
    
    consume(parser, "{", "Expected '{' after while condition")
    
    parser.in_loop = based
    
    sus body []Statement = []
    bestie !checkOperator(parser, "}") and !isAtEnd(parser) {
        if matchNewline(parser) {
            vibes
        }
        
        sus stmt Statement = parseStatement(parser)
        body.push(stmt)
    }
    
    parser.in_loop = cringe
    
    consume(parser, "}", "Expected '}' after while body")
    
    damn createWhileStatement(condition, body)
}

slay parseStructStatement(parser *Parser) Statement {
    consume(parser, "squad", "Expected 'squad'")
    
    if !checkIdentifier(parser) {
        yikes "Expected struct name after 'squad'"
    }
    
    sus name tea = advance(parser).lexeme
    
    consume(parser, "{", "Expected '{' after struct name")
    
    sus fields []StructField = []
    bestie !checkOperator(parser, "}") and !isAtEnd(parser) {
        if matchNewline(parser) {
            vibes
        }
        
        if checkKeyword(parser, "spill") {
            advance(parser)
            
            if !checkIdentifier(parser) {
                yikes "Expected field name after 'spill'"
            }
            
            sus field_name tea = advance(parser).lexeme
            sus field_type Type = parseType(parser)
            
            sus field StructField = createStructField(field_name, field_type)
            fields.push(field)
        }
    }
    
    consume(parser, "}", "Expected '}' after struct body")
    
    damn createStructStatement(name, fields)
}

slay parseInterfaceStatement(parser *Parser) Statement {
    consume(parser, "collab", "Expected 'collab'")
    
    if !checkIdentifier(parser) {
        yikes "Expected interface name after 'collab'"
    }
    
    sus name tea = advance(parser).lexeme
    
    consume(parser, "{", "Expected '{' after interface name")
    
    sus methods []InterfaceMethod = []
    bestie !checkOperator(parser, "}") and !isAtEnd(parser) {
        if matchNewline(parser) {
            vibes
        }
        
        if checkKeyword(parser, "slay") {
            sus method InterfaceMethod = parseInterfaceMethod(parser)
            methods.push(method)
        }
    }
    
    consume(parser, "}", "Expected '}' after interface body")
    
    damn createInterfaceStatement(name, methods)
}

slay parseInterfaceMethod(parser *Parser) InterfaceMethod {
    consume(parser, "slay", "Expected 'slay'")
    
    if !checkIdentifier(parser) {
        yikes "Expected method name"
    }
    
    sus name tea = advance(parser).lexeme
    
    consume(parser, "(", "Expected '(' after method name")
    
    sus parameters []Parameter = []
    if !checkOperator(parser, ")") {
        bestie based {
            sus param Parameter = parseParameter(parser)
            parameters.push(param)
            
            if !matchOperator(parser, ",") {
                vibes
            }
        }
    }
    
    consume(parser, ")", "Expected ')' after parameters")
    
    sus return_type Type = none
    if !isNewlineOrSemicolon(parser) and !checkOperator(parser, "}") {
        return_type = parseType(parser)
    }
    
    damn createInterfaceMethod(name, parameters, return_type)
}

slay parseGoroutineStatement(parser *Parser) Statement {
    consume(parser, "stan", "Expected 'stan'")
    
    consume(parser, "{", "Expected '{' after 'stan'")
    
    sus body []Statement = []
    bestie !checkOperator(parser, "}") and !isAtEnd(parser) {
        if matchNewline(parser) {
            vibes
        }
        
        sus stmt Statement = parseStatement(parser)
        body.push(stmt)
    }
    
    consume(parser, "}", "Expected '}' after goroutine body")
    
    damn createGoroutineStatement(body)
}

slay parseMatchExpression(parser *Parser) Expression {
    consume(parser, "match", "Expected 'match'")
    
    sus value Expression = parseExpression(parser)
    
    consume(parser, "{", "Expected '{' after match value")
    
    sus cases []MatchCase = []
    bestie !checkOperator(parser, "}") and !isAtEnd(parser) {
        if matchNewline(parser) {
            vibes
        }
        
        sus match_case MatchCase = parseMatchCase(parser)
        cases.push(match_case)
    }
    
    consume(parser, "}", "Expected '}' after match cases")
    
    damn createMatchExpression(value, cases)
}

slay parseMatchCase(parser *Parser) MatchCase {
    sus pattern Expression = parseExpression(parser)
    
    consume(parser, "=>", "Expected '=>' after match pattern")
    
    sus body Expression = parseExpression(parser)
    
    damn createMatchCase(pattern, body)
}

slay parseShortDeclaration(parser *Parser) Statement {
    sus names []tea = []
    
    fr fr Parse variable names
    if matchOperator(parser, "(") {
        fr fr Tuple destructuring: (a, b, c) := (1, 2, 3)
        bestie !checkOperator(parser, ")") and !isAtEnd(parser) {
            if !checkIdentifier(parser) {
                yikes "Expected identifier"
            }
            
            names.push(advance(parser).lexeme)
            
            if !matchOperator(parser, ",") {
                vibes
            }
        }
        
        consume(parser, ")", "Expected ')'")
    } else {
        fr fr Single variable or comma-separated: a, b := 1, 2
        if !checkIdentifier(parser) {
            yikes "Expected identifier"
        }
        
        names.push(advance(parser).lexeme)
        
        bestie matchOperator(parser, ",") {
            if !checkIdentifier(parser) {
                yikes "Expected identifier"
            }
            
            names.push(advance(parser).lexeme)
        }
    }
    
    consume(parser, ":=", "Expected ':=' in short declaration")
    
    fr fr Parse values
    sus values []Expression = []
    
    if matchOperator(parser, "(") {
        fr fr Tuple values: (1, 2, 3)
        bestie !checkOperator(parser, ")") and !isAtEnd(parser) {
            sus value Expression = parseExpression(parser)
            values.push(value)
            
            if !matchOperator(parser, ",") {
                vibes
            }
        }
        
        consume(parser, ")", "Expected ')'")
    } else {
        fr fr Single value or comma-separated: 1, 2
        sus value Expression = parseExpression(parser)
        values.push(value)
        
        bestie matchOperator(parser, ",") {
            sus next_value Expression = parseExpression(parser)
            values.push(next_value)
        }
    }
    
    damn createShortDeclarationStatement(names, values)
}

slay parseAssignmentStatement(parser *Parser) Statement {
    sus target Expression = parseExpression(parser)
    
    if !matchOperator(parser, "=") and !matchOperator(parser, "+=") and 
       !matchOperator(parser, "-=") and !matchOperator(parser, "*=") and
       !matchOperator(parser, "/=") and !matchOperator(parser, "%=") {
        yikes "Expected assignment operator"
    }
    
    sus operator tea = previous(parser).lexeme
    sus value Expression = parseExpression(parser)
    
    damn createAssignmentStatement(target, operator, value)
}

fr fr Test functions for validation
slay test_parseBasicFunction() {
    test_start("Parse Basic Function")
    
    sus tokens []Token = [
        Token{kind: "KEYWORD", lexeme: "slay", line: 1, column: 1},
        Token{kind: "IDENTIFIER", lexeme: "test", line: 1, column: 6},
        Token{kind: "OPERATOR", lexeme: "(", line: 1, column: 10},
        Token{kind: "OPERATOR", lexeme: ")", line: 1, column: 11},
        Token{kind: "OPERATOR", lexeme: "{", line: 1, column: 13},
        Token{kind: "OPERATOR", lexeme: "}", line: 1, column: 15},
        Token{kind: "EOF", lexeme: "", line: 1, column: 16}
    ]
    
    sus parser Parser = createParser(tokens)
    sus program Program = parseProgram(parser)
    
    assert_true(program.statements.length == 1)
    assert_true(!parser.had_error)
    
    test_passed()
}

slay test_parseVariableDeclaration() {
    test_start("Parse Variable Declaration")
    
    sus tokens []Token = [
        Token{kind: "KEYWORD", lexeme: "sus", line: 1, column: 1},
        Token{kind: "IDENTIFIER", lexeme: "x", line: 1, column: 5},
        Token{kind: "IDENTIFIER", lexeme: "normie", line: 1, column: 7},
        Token{kind: "OPERATOR", lexeme: "=", line: 1, column: 14},
        Token{kind: "NUMBER", lexeme: "42", line: 1, column: 16},
        Token{kind: "EOF", lexeme: "", line: 1, column: 18}
    ]
    
    sus parser Parser = createParser(tokens)
    sus program Program = parseProgram(parser)
    
    assert_true(program.statements.length == 1)
    assert_true(!parser.had_error)
    
    test_passed()
}

slay test_parseExpression() {
    test_start("Parse Expression")
    
    sus tokens []Token = [
        Token{kind: "NUMBER", lexeme: "42", line: 1, column: 1},
        Token{kind: "OPERATOR", lexeme: "+", line: 1, column: 4},
        Token{kind: "NUMBER", lexeme: "24", line: 1, column: 6},
        Token{kind: "EOF", lexeme: "", line: 1, column: 8}
    ]
    
    sus parser Parser = createParser(tokens)
    sus program Program = parseProgram(parser)
    
    assert_true(program.statements.length == 1)
    assert_true(!parser.had_error)
    
    test_passed()
}

slay runParserTests() {
    test_parseBasicFunction()
    test_parseVariableDeclaration()
    test_parseExpression()
    print_test_summary()
}

fr fr Entry point for testing
slay main() {
    runParserTests()
}
