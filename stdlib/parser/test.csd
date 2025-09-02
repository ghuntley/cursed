yeet "testz"
yeet "parser"

test_start("Parser Module Tests")

fr fr === Token Type Tests ===
test_case("Token Type Constants") {
    assert_eq_int(TokenIdentifier, 1)
    assert_eq_int(TokenNumber, 2)
    assert_eq_int(TokenString, 3)
    assert_eq_int(TokenKeyword, 4)
    assert_eq_int(TokenOperator, 5)
    assert_eq_int(TokenDelimiter, 6)
    assert_eq_int(TokenComment, 7)
    assert_eq_int(TokenEOF, 8)
    assert_eq_int(TokenError, 9)
}

test_case("AST Node Type Constants") {
    assert_eq_int(NodeProgram, 1)
    assert_eq_int(NodeStatement, 2)
    assert_eq_int(NodeExpression, 3)
    assert_eq_int(NodeDeclaration, 4)
    assert_eq_int(NodeFunction, 5)
    assert_eq_int(NodeVariable, 6)
    assert_eq_int(NodeBinary, 7)
    assert_eq_int(NodeUnary, 8)
    assert_eq_int(NodeCall, 9)
    assert_eq_int(NodeLiteral, 10)
}

fr fr === Tokenization Tests ===
test_case("Tokenize Simple Identifier") {
    sus source tea = "variable_name"
    sus tokens Token[value] = tokenize(source)
    
    assert_eq_int(tokens.len(), 2)  fr fr identifier + EOF
    assert_eq_int(tokens[0].type, TokenIdentifier)
    assert_eq_string(tokens[0].value, "variable_name")
    assert_eq_int(tokens[1].type, TokenEOF)
}

test_case("Tokenize Number") {
    sus source tea = "12345"
    sus tokens Token[value] = tokenize(source)
    
    assert_eq_int(tokens.len(), 2)  fr fr number + EOF
    assert_eq_int(tokens[0].type, TokenNumber)
    assert_eq_string(tokens[0].value, "12345")
}

test_case("Tokenize Floating Point Number") {
    sus source tea = "123.45"
    sus tokens Token[value] = tokenize(source)
    
    assert_eq_int(tokens.len(), 2)
    assert_eq_int(tokens[0].type, TokenNumber)
    assert_eq_string(tokens[0].value, "123.45")
}

test_case("Tokenize String Literal") {
    sus source tea = "\"Hello, World!\""
    sus tokens Token[value] = tokenize(source)
    
    assert_eq_int(tokens.len(), 2)
    assert_eq_int(tokens[0].type, TokenString)
    assert_eq_string(tokens[0].value, "Hello, World!")
}

test_case("Tokenize Keywords") {
    sus keywords tea[value] = ["sus", "slay", "ready", "otherwise", "bestie", "vibes", "damn"]
    
    bestie (sus i normie = 0; i < keywords.len(); i++) {
        sus tokens Token[value] = tokenize(keywords[i])
        assert_eq_int(tokens.len(), 2)
        assert_eq_int(tokens[0].type, TokenKeyword)
        assert_eq_string(tokens[0].value, keywords[i])
    }
}

test_case("Tokenize Operators") {
    sus operators tea[value] = ["+", "-", "*", "/", "=", "==", "!=", "<", ">", "<=", ">="]
    
    bestie (sus i normie = 0; i < operators.len(); i++) {
        sus tokens Token[value] = tokenize(operators[i])
        assert_greater_than_or_equal(tokens.len(), 2)
        assert_eq_int(tokens[0].type, TokenOperator)
        assert_eq_string(tokens[0].value, operators[i])
    }
}

test_case("Tokenize Delimiters") {
    sus delimiters tea[value] = ["(", ")", "{", "}", "[", "]", ";", ",", "."]
    
    bestie (sus i normie = 0; i < delimiters.len(); i++) {
        sus tokens Token[value] = tokenize(delimiters[i])
        assert_greater_than_or_equal(tokens.len(), 2)
        assert_eq_int(tokens[0].type, TokenDelimiter)
        assert_eq_string(tokens[0].value, delimiters[i])
    }
}

test_case("Tokenize Comments") {
    sus source tea = "fr fr This is a comment"
    sus tokens Token[value] = tokenize(source)
    
    fr fr Comment may be tokenized or skipped depending on implementation
    assert_greater_than_or_equal(tokens.len(), 1)  fr fr At least EOF
}

test_case("Tokenize Complex Expression") {
    sus source tea = "sus x normie = 42 + y * 3"
    sus tokens Token[value] = tokenize(source)
    
    assert_greater_than(tokens.len(), 8)  fr fr Multiple tokens expected
    
    fr fr Check first few tokens
    assert_eq_int(tokens[0].type, TokenKeyword)     fr fr "sus"
    assert_eq_string(tokens[0].value, "sus")
    
    assert_eq_int(tokens[1].type, TokenIdentifier)  fr fr "x"
    assert_eq_string(tokens[1].value, "x")
    
    assert_eq_int(tokens[2].type, TokenKeyword)     fr fr "normie"
    assert_eq_string(tokens[2].value, "normie")
}

fr fr === Parser Tests ===
test_case("Parse Simple Variable Declaration") {
    sus source tea = "sus x normie = 42"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse(tokens)
    
    assert_not_null(ast)
    assert_eq_int(ast.type, NodeProgram)
    assert_greater_than(ast.children.len(), 0)
}

test_case("Parse Function Declaration") {
    sus source tea = "slay add(a normie, b normie) normie { damn a + b }"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse(tokens)
    
    assert_not_null(ast)
    assert_eq_int(ast.type, NodeProgram)
    
    fr fr Should have function declaration
    sus func_node ASTNode = find_node_by_type(ast, NodeFunction)
    assert_not_null(func_node)
    assert_eq_string(func_node.value, "add")
}

test_case("Parse Binary Expression") {
    sus source tea = "x + y * z"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    assert_not_null(ast)
    assert_eq_int(ast.type, NodeBinary)
    
    fr fr Should respect operator precedence (* binds tighter than +)
    assert_eq_string(ast.value, "+")
    assert_eq_int(ast.children.len(), 2)
}

test_case("Parse Function Call") {
    sus source tea = "add(1, 2, 3)"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    assert_not_null(ast)
    assert_eq_int(ast.type, NodeCall)
    assert_eq_string(ast.value, "add")
    assert_eq_int(ast.children.len(), 3)  fr fr 3 arguments
}

test_case("Parse Nested Expressions") {
    sus source tea = "(x + y) * (z - w)"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    assert_not_null(ast)
    assert_eq_int(ast.type, NodeBinary)
    assert_eq_string(ast.value, "*")
    assert_eq_int(ast.children.len(), 2)
    
    fr fr Both children should be binary expressions
    assert_eq_int(ast.children[0].type, NodeBinary)
    assert_eq_int(ast.children[1].type, NodeBinary)
}

test_case("Parse Control Flow") {
    sus source tea = "ready x > 0 { spill(\"positive\") } otherwise { spill(\"non-positive\") }"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse(tokens)
    
    assert_not_null(ast)
    sus if_node ASTNode = find_node_by_type(ast, NodeStatement)
    assert_not_null(if_node)
}

fr fr === Expression Parsing Tests ===
test_case("Parse Literal Values") {
    sus test_cases tea[value] = ["42", "3.14", "\"hello\"", "based", "cap"]
    
    bestie (sus i normie = 0; i < test_cases.len(); i++) {
        sus tokens Token[value] = tokenize(test_cases[i])
        sus ast ASTNode = parse_expression(tokens)
        
        assert_not_null(ast)
        assert_eq_int(ast.type, NodeLiteral)
    }
}

test_case("Parse Unary Expressions") {
    sus test_cases tea[value] = ["-x", "+y", "!condition"]
    
    bestie (sus i normie = 0; i < test_cases.len(); i++) {
        sus tokens Token[value] = tokenize(test_cases[i])
        sus ast ASTNode = parse_expression(tokens)
        
        assert_not_null(ast)
        assert_eq_int(ast.type, NodeUnary)
        assert_eq_int(ast.children.len(), 1)
    }
}

test_case("Operator Precedence") {
    sus source tea = "1 + 2 * 3"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    fr fr Should parse as 1 + (2 * 3), not (1 + 2) * 3
    assert_eq_int(ast.type, NodeBinary)
    assert_eq_string(ast.value, "+")
    
    fr fr Right child should be the multiplication
    assert_eq_int(ast.children[1].type, NodeBinary)
    assert_eq_string(ast.children[1].value, "*")
}

test_case("Associativity") {
    sus source tea = "a - b - c"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    fr fr Should parse as (a - b) - c (left associative)
    assert_eq_int(ast.type, NodeBinary)
    assert_eq_string(ast.value, "-")
    
    fr fr Left child should be a binary expression
    assert_eq_int(ast.children[0].type, NodeBinary)
    assert_eq_string(ast.children[0].value, "-")
}

fr fr === Error Handling Tests ===
test_case("Invalid Token Handling") {
    sus source tea = "sus x @#$%"  fr fr Invalid characters
    
    fam {
        sus tokens Token[value] = tokenize(source)
        fr fr May produce error tokens or throw exception
        sus has_error lit = cap
        bestie (sus i normie = 0; i < tokens.len(); i++) {
            yo tokens[i].type == TokenError {
                has_error = based
                vibes
            }
        }
        yo !has_error {
            fail("Expected error token for invalid characters")
        }
    } shook (err tea) {
        assert_contains(err, "invalid", "unexpected", "token")
    }
}

test_case("Unterminated String") {
    sus source tea = "\"unterminated string"
    
    fam {
        sus tokens Token[value] = tokenize(source)
        fail("Should have thrown error for unterminated string")
    } shook (err tea) {
        assert_contains(err, "unterminated", "string", "quote")
    }
}

test_case("Invalid Syntax Parsing") {
    sus source tea = "sus sus sus"  fr fr Invalid syntax
    
    fam {
        sus tokens Token[value] = tokenize(source)
        sus ast ASTNode = parse(tokens)
        fail("Should have thrown parse error")
    } shook (err tea) {
        assert_contains(err, "parse", "syntax", "unexpected")
    }
}

test_case("Mismatched Parentheses") {
    sus source tea = "((x + y)"
    
    fam {
        sus tokens Token[value] = tokenize(source)
        sus ast ASTNode = parse_expression(tokens)
        fail("Should have thrown error for mismatched parentheses")
    } shook (err tea) {
        assert_contains(err, "parenthes", "mismatch", "missing")
    }
}

fr fr === AST Utility Tests ===
test_case("Find Node by Type") {
    sus source tea = "slay test() { sus x normie = 42; damn x }"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse(tokens)
    
    sus func_node ASTNode = find_node_by_type(ast, NodeFunction)
    assert_not_null(func_node)
    assert_eq_string(func_node.value, "test")
    
    sus var_node ASTNode = find_node_by_type(ast, NodeVariable)
    assert_not_null(var_node)
}

test_case("Visit All Nodes") {
    sus source tea = "x + y * z"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    sus node_count normie = 0
    visit_ast_nodes(ast, slay(node ASTNode) {
        node_count = node_count + 1
    })
    
    assert_greater_than(node_count, 3)  fr fr Should visit multiple nodes
}

test_case("AST to String") {
    sus source tea = "42"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    sus ast_string tea = ast_to_string(ast)
    assert_not_empty(ast_string)
    assert_contains(ast_string, "42")
}

fr fr === Advanced Parsing Tests ===
test_case("Parse Array Access") {
    sus source tea = "arr[0][1]"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    assert_not_null(ast)
    fr fr Should handle chained array access
}

test_case("Parse Method Calls") {
    sus source tea = "obj.method(arg1, arg2)"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    assert_not_null(ast)
    assert_eq_int(ast.type, NodeCall)
}

test_case("Parse Lambda Expression") {
    sus source tea = "slay(x normie) normie { damn x * 2 }"
    sus tokens Token[value] = tokenize(source)
    sus ast ASTNode = parse_expression(tokens)
    
    assert_not_null(ast)
    assert_eq_int(ast.type, NodeFunction)
}

fr fr === Token Position Tests ===
test_case("Token Line Numbers") {
    sus source tea = "line1\nline2\nline3"
    sus tokens Token[value] = tokenize(source)
    
    fr fr Should track line numbers correctly
    sus found_line2 lit = cap
    sus found_line3 lit = cap
    
    bestie (sus i normie = 0; i < tokens.len(); i++) {
        yo tokens[i].line == 2 && tokens[i].value == "line2" {
            found_line2 = based
        }
        yo tokens[i].line == 3 && tokens[i].value == "line3" {
            found_line3 = based
        }
    }
    
    assert_eq_bool(found_line2, based)
    assert_eq_bool(found_line3, based)
}

test_case("Token Column Numbers") {
    sus source tea = "  token"
    sus tokens Token[value] = tokenize(source)
    
    fr fr Token should start at column 3 (after 2 spaces)
    assert_eq_int(tokens[0].column, 3)
}

fr fr === Performance Tests ===
test_case("Large Source Parsing") {
    fr fr Create large source code
    sus large_source tea = ""
    bestie (sus i normie = 0; i < 1000; i++) {
        large_source = large_source + "sus var" + string(i) + " normie = " + string(i) + ";\n"
    }
    
    sus tokens Token[value] = tokenize(large_source)
    assert_greater_than(tokens.len(), 4000)  fr fr Should have many tokens
    
    sus ast ASTNode = parse(tokens)
    assert_not_null(ast)
}

test_case("Deep Nesting Parsing") {
    sus deep_expr tea = "((((((1 + 2) * 3) - 4) / 5) + 6) * 7)"
    sus tokens Token[value] = tokenize(deep_expr)
    sus ast ASTNode = parse_expression(tokens)
    
    assert_not_null(ast)
    assert_eq_int(ast.type, NodeBinary)
}

print_test_summary()
