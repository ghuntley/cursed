yeet "testz"
yeet "parser"

# Test tokenization
test_start("tokenize basic tokens")
sus tokens := tokenize("sus x := 42")
assert_eq_int(len(tokens), 5) # sus, x, :=, 42, EOF
assert_eq_string(tokens[0].value, "sus")
assert_eq_int(tokens[0].token_type, TokenKeyword)
assert_eq_string(tokens[1].value, "x")
assert_eq_int(tokens[1].token_type, TokenIdentifier)
assert_eq_string(tokens[2].value, ":=")
assert_eq_int(tokens[2].token_type, TokenOperator)
print_test_summary()

test_start("tokenize string literals")
sus tokens2 := tokenize("sus name := \"hello world\"")
assert_eq_int(len(tokens2), 5)
assert_eq_string(tokens2[3].value, "hello world")
assert_eq_int(tokens2[3].token_type, TokenString)
print_test_summary()

test_start("tokenize numbers")
sus tokens3 := tokenize("sus pi := 3.14159")
assert_eq_int(len(tokens3), 5)
assert_eq_string(tokens3[3].value, "3.14159")
assert_eq_int(tokens3[3].token_type, TokenNumber)
print_test_summary()

test_start("tokenize keywords")
sus tokens4 := tokenize("slay test() { damn based }")
assert_eq_string(tokens4[0].value, "slay")
assert_eq_int(tokens4[0].token_type, TokenKeyword)
assert_eq_string(tokens4[4].value, "damn")
assert_eq_int(tokens4[4].token_type, TokenKeyword)
assert_eq_string(tokens4[5].value, "based")
assert_eq_int(tokens4[5].token_type, TokenKeyword)
print_test_summary()

test_start("tokenize operators")
sus tokens5 := tokenize("x + y * z")
assert_eq_string(tokens5[1].value, "+")
assert_eq_int(tokens5[1].token_type, TokenOperator)
assert_eq_string(tokens5[3].value, "*")
assert_eq_int(tokens5[3].token_type, TokenOperator)
print_test_summary()

test_start("tokenize delimiters")
sus tokens6 := tokenize("func(a, b) { return; }")
assert_eq_string(tokens6[1].value, "(")
assert_eq_int(tokens6[1].token_type, TokenDelimiter)
assert_eq_string(tokens6[3].value, ",")
assert_eq_int(tokens6[3].token_type, TokenDelimiter)
print_test_summary()

test_start("tokenize comments")
sus tokens7 := tokenize("sus x := 42 # this is a comment")
assert_eq_string(tokens7[4].value, "# this is a comment")
assert_eq_int(tokens7[4].token_type, TokenComment)
print_test_summary()

# Test keyword recognition
test_start("is_keyword function")
assert_true(is_keyword("sus"))
assert_true(is_keyword("slay"))
assert_true(is_keyword("damn"))
assert_true(is_keyword("shook"))
assert_true(is_keyword("cap"))
assert_true(is_keyword("bestie"))
assert_true(is_keyword("yeet"))
assert_true(is_keyword("vibez"))
assert_true(is_keyword("based"))
assert_true(is_keyword("cringe"))
assert_false(is_keyword("notakeyword"))
assert_false(is_keyword("x"))
print_test_summary()

# Test character classification
test_start("is_operator function")
assert_true(is_operator('+'))
assert_true(is_operator('-'))
assert_true(is_operator('*'))
assert_true(is_operator('/'))
assert_true(is_operator('='))
assert_false(is_operator('a'))
assert_false(is_operator('1'))
print_test_summary()

test_start("is_delimiter function")
assert_true(is_delimiter('('))
assert_true(is_delimiter(')'))
assert_true(is_delimiter('{'))
assert_true(is_delimiter('}'))
assert_true(is_delimiter('['))
assert_true(is_delimiter(']'))
assert_true(is_delimiter(';'))
assert_true(is_delimiter(','))
assert_false(is_delimiter('a'))
assert_false(is_delimiter('+'))
print_test_summary()

# Test parser creation
test_start("create_parser")
sus test_tokens := tokenize("sus x := 42")
sus parser := create_parser(test_tokens)
assert_eq_int(parser.current, 0)
assert_eq_int(len(parser.tokens), 5)
assert_eq_int(len(parser.symbols), 0)
assert_eq_int(len(parser.errors), 0)
print_test_summary()

# Test parser helper functions
test_start("parser helper functions")
sus parser2 := create_parser(tokenize("sus x := 42"))
assert_false(is_at_end(parser2))
assert_eq_string(peek(parser2).value, "sus")

sus first_token := advance(parser2)
assert_eq_string(first_token.value, "sus")
assert_eq_string(peek(parser2).value, "x")

assert_true(check(parser2, "x"))
assert_false(check(parser2, "y"))

assert_true(match(parser2, "x"))
assert_eq_string(peek(parser2).value, ":=")
print_test_summary()

# Test simple expression parsing
test_start("parse simple literal")
sus parser3 := create_parser(tokenize("42"))
sus expr := parse_expression(parser3)
assert_eq_int(expr.node_type, NodeLiteral)
assert_eq_string(expr.value, "42")
print_test_summary()

test_start("parse string literal")
sus parser4 := create_parser(tokenize("\"hello\""))
sus expr2 := parse_expression(parser4)
assert_eq_int(expr2.node_type, NodeLiteral)
assert_eq_string(expr2.value, "hello")
print_test_summary()

test_start("parse boolean literal")
sus parser5 := create_parser(tokenize("based"))
sus expr3 := parse_expression(parser5)
assert_eq_int(expr3.node_type, NodeLiteral)
assert_eq_string(expr3.value, "based")
print_test_summary()

test_start("parse identifier")
sus parser6 := create_parser(tokenize("variable_name"))
sus expr4 := parse_expression(parser6)
assert_eq_int(expr4.node_type, NodeLiteral)
assert_eq_string(expr4.value, "variable_name")
print_test_summary()

# Test binary expression parsing
test_start("parse simple addition")
sus parser7 := create_parser(tokenize("1 + 2"))
sus expr5 := parse_expression(parser7)
assert_eq_int(expr5.node_type, NodeBinary)
assert_eq_string(expr5.value, "+")
assert_eq_int(len(expr5.children), 2)
assert_eq_string(expr5.children[0].value, "1")
assert_eq_string(expr5.children[1].value, "2")
print_test_summary()

test_start("parse complex arithmetic")
sus parser8 := create_parser(tokenize("1 + 2 * 3"))
sus expr6 := parse_expression(parser8)
assert_eq_int(expr6.node_type, NodeBinary)
assert_eq_string(expr6.value, "+")
# Should be: (1 + (2 * 3))
assert_eq_string(expr6.children[0].value, "1")
assert_eq_int(expr6.children[1].node_type, NodeBinary)
assert_eq_string(expr6.children[1].value, "*")
print_test_summary()

test_start("parse comparison")
sus parser9 := create_parser(tokenize("x < y"))
sus expr7 := parse_expression(parser9)
assert_eq_int(expr7.node_type, NodeBinary)
assert_eq_string(expr7.value, "<")
assert_eq_string(expr7.children[0].value, "x")
assert_eq_string(expr7.children[1].value, "y")
print_test_summary()

test_start("parse equality")
sus parser10 := create_parser(tokenize("a == b"))
sus expr8 := parse_expression(parser10)
assert_eq_int(expr8.node_type, NodeBinary)
assert_eq_string(expr8.value, "==")
assert_eq_string(expr8.children[0].value, "a")
assert_eq_string(expr8.children[1].value, "b")
print_test_summary()

# Test unary expression parsing
test_start("parse unary negation")
sus parser11 := create_parser(tokenize("-42"))
sus expr9 := parse_expression(parser11)
assert_eq_int(expr9.node_type, NodeUnary)
assert_eq_string(expr9.value, "-")
assert_eq_int(len(expr9.children), 1)
assert_eq_string(expr9.children[0].value, "42")
print_test_summary()

test_start("parse logical not")
sus parser12 := create_parser(tokenize("!flag"))
sus expr10 := parse_expression(parser12)
assert_eq_int(expr10.node_type, NodeUnary)
assert_eq_string(expr10.value, "!")
assert_eq_string(expr10.children[0].value, "flag")
print_test_summary()

# Test function call parsing
test_start("parse simple function call")
sus parser13 := create_parser(tokenize("func()"))
sus expr11 := parse_expression(parser13)
assert_eq_int(expr11.node_type, NodeCall)
assert_eq_string(expr11.value, "call")
assert_eq_int(len(expr11.children), 1) # callee only
assert_eq_string(expr11.children[0].value, "func")
print_test_summary()

test_start("parse function call with arguments")
sus parser14 := create_parser(tokenize("func(1, 2)"))
sus expr12 := parse_expression(parser14)
assert_eq_int(expr12.node_type, NodeCall)
assert_eq_int(len(expr12.children), 3) # callee + 2 arguments
assert_eq_string(expr12.children[0].value, "func")
assert_eq_string(expr12.children[1].value, "1")
assert_eq_string(expr12.children[2].value, "2")
print_test_summary()

# Test member access parsing
test_start("parse member access")
sus parser15 := create_parser(tokenize("obj.field"))
sus expr13 := parse_expression(parser15)
assert_eq_int(expr13.node_type, NodeBinary)
assert_eq_string(expr13.value, ".")
assert_eq_string(expr13.children[0].value, "obj")
assert_eq_string(expr13.children[1].value, "field")
print_test_summary()

# Test variable declaration parsing
test_start("parse variable declaration")
sus parser16 := create_parser(tokenize("sus x := 42"))
sus stmt := parse_statement(parser16)
assert_eq_int(stmt.node_type, NodeVariable)
assert_eq_string(stmt.value, "x")
assert_eq_int(len(stmt.children), 1)
assert_eq_string(stmt.children[0].value, "42")
print_test_summary()

test_start("parse typed variable declaration")
sus parser17 := create_parser(tokenize("sus count normie := 100"))
sus stmt2 := parse_statement(parser17)
assert_eq_int(stmt2.node_type, NodeVariable)
assert_eq_string(stmt2.value, "count")
print_test_summary()

# Test function declaration parsing
test_start("parse simple function declaration")
sus parser18 := create_parser(tokenize("slay test() { damn 42 }"))
sus stmt3 := parse_statement(parser18)
assert_eq_int(stmt3.node_type, NodeFunction)
assert_eq_string(stmt3.value, "test")
assert_eq_int(len(stmt3.children), 2) # params + body
print_test_summary()

test_start("parse function with parameters")
sus parser19 := create_parser(tokenize("slay add(a normie, b normie) normie { damn a + b }"))
sus stmt4 := parse_statement(parser19)
assert_eq_int(stmt4.node_type, NodeFunction)
assert_eq_string(stmt4.value, "add")
assert_eq_int(len(stmt4.children), 2)
# Parameters should be parsed
assert_eq_int(len(stmt4.children[0].children), 2) # 2 parameters
print_test_summary()

# Test return statement parsing
test_start("parse return statement")
sus parser20 := create_parser(tokenize("damn 42"))
sus stmt5 := parse_statement(parser20)
assert_eq_int(stmt5.node_type, NodeStatement)
assert_eq_string(stmt5.value, "return")
assert_eq_int(len(stmt5.children), 1)
assert_eq_string(stmt5.children[0].value, "42")
print_test_summary()

test_start("parse empty return")
sus parser21 := create_parser(tokenize("damn"))
sus stmt6 := parse_statement(parser21)
assert_eq_int(stmt6.node_type, NodeStatement)
assert_eq_string(stmt6.value, "return")
assert_eq_int(len(stmt6.children), 0)
print_test_summary()

# Test constant declaration parsing
test_start("parse constant declaration")
sus parser22 := create_parser(tokenize("facts PI := 3.14159"))
sus stmt7 := parse_statement(parser22)
assert_eq_int(stmt7.node_type, NodeDeclaration)
assert_eq_string(stmt7.value, "PI")
assert_eq_int(len(stmt7.children), 1)
assert_eq_string(stmt7.children[0].value, "3.14159")
print_test_summary()

# Test type alias parsing
test_start("parse type alias")
sus parser23 := create_parser(tokenize("be_like MyInt = normie"))
sus stmt8 := parse_statement(parser23)
assert_eq_int(stmt8.node_type, NodeDeclaration)
assert_eq_string(stmt8.value, "MyInt")
print_test_summary()

# Test program parsing
test_start("parse simple program")
sus program_source := "sus x := 42\nslay test() { damn x }"
sus parser24 := create_parser(tokenize(program_source))
sus program := parse_program(parser24)
assert_eq_int(program.node_type, NodeProgram)
assert_eq_string(program.value, "program")
assert_eq_int(len(program.children), 2) # 2 statements
print_test_summary()

# Test symbol table management
test_start("symbol table operations")
sus parser25 := create_parser(tokenize("sus x := 42"))
sus symbol := Symbol{
    name: "x",
    symbol_type: "normie",
    scope: 0,
    declared_line: 1
}
add_symbol(parser25, symbol)
assert_eq_int(len(parser25.symbols), 1)

sus found_symbol := resolve_symbol(parser25, "x")
assert_true(found_symbol != cringe)
assert_eq_string(found_symbol.name, "x")
assert_eq_string(found_symbol.symbol_type, "normie")

sus not_found := resolve_symbol(parser25, "y")
assert_true(not_found == cringe)
print_test_summary()

# Test type inference
test_start("type inference for literals")
sus parser26 := create_parser(tokenize("42"))
sus int_node := parse_expression(parser26)
sus int_type := infer_type(int_node, parser26)
assert_eq_string(int_type, "normie")

sus parser27 := create_parser(tokenize("3.14"))
sus float_node := parse_expression(parser27)
sus float_type := infer_type(float_node, parser27)
assert_eq_string(float_type, "meal")

sus parser28 := create_parser(tokenize("based"))
sus bool_node := parse_expression(parser28)
sus bool_type := infer_type(bool_node, parser28)
assert_eq_string(bool_type, "lit")
print_test_summary()

test_start("type inference for binary expressions")
sus parser29 := create_parser(tokenize("1 + 2"))
sus add_node := parse_expression(parser29)
sus add_type := infer_type(add_node, parser29)
assert_eq_string(add_type, "normie")

sus parser30 := create_parser(tokenize("1.0 + 2"))
sus mixed_node := parse_expression(parser30)
sus mixed_type := infer_type(mixed_node, parser30)
assert_eq_string(mixed_type, "meal") # Type promotion to float
print_test_summary()

# Test error handling
test_start("parse error handling")
sus parser31 := create_parser(tokenize("sus := 42")) # Missing identifier
sus stmt9 := parse_statement(parser31)
sus errors := get_errors(parser31)
assert_true(len(errors) > 0)
print_test_summary()

test_start("error recovery")
sus parser32 := create_parser(tokenize("sus bad syntax here; sus x := 42"))
# Should recover and continue parsing
sus program2 := parse_program(parser32)
assert_eq_int(program2.node_type, NodeProgram)
print_test_summary()

# Test AST utility functions
test_start("count AST nodes")
sus parser33 := create_parser(tokenize("sus x := 42; sus y := 100"))
sus program3 := parse_program(parser33)
sus var_count := count_nodes(program3, NodeVariable)
assert_eq_int(var_count, 2)
print_test_summary()

test_start("find nodes by value")
sus parser34 := create_parser(tokenize("sus test := 42; slay test() { damn test }"))
sus program4 := parse_program(parser34)
sus test_nodes := find_nodes_by_value(program4, "test")
assert_true(len(test_nodes) >= 2) # Variable and function both named "test"
print_test_summary()

# Test complex parsing scenarios
test_start("parse nested expressions")
sus parser35 := create_parser(tokenize("(1 + 2) * (3 - 4)"))
sus complex_expr := parse_expression(parser35)
assert_eq_int(complex_expr.node_type, NodeBinary)
assert_eq_string(complex_expr.value, "*")
print_test_summary()

test_start("parse chained function calls")
sus parser36 := create_parser(tokenize("obj.method().value"))
sus chain_expr := parse_expression(parser36)
assert_eq_int(chain_expr.node_type, NodeBinary) # Final member access
assert_eq_string(chain_expr.value, ".")
print_test_summary()

test_start("parse method calls with arguments")
sus parser37 := create_parser(tokenize("obj.method(1, 2, 3)"))
sus method_call := parse_expression(parser37)
# Should be parsed as member access followed by function call
assert_eq_int(method_call.node_type, NodeCall)
print_test_summary()

# Test edge cases
test_start("parse empty program")
sus parser38 := create_parser(tokenize(""))
sus empty_program := parse_program(parser38)
assert_eq_int(empty_program.node_type, NodeProgram)
assert_eq_int(len(empty_program.children), 0)
print_test_summary()

test_start("parse whitespace only")
sus tokens_ws := tokenize("   \n\t  ")
assert_eq_int(len(tokens_ws), 1) # Just EOF
assert_eq_int(tokens_ws[0].token_type, TokenEOF)
print_test_summary()

test_start("parse comments only")
sus tokens_comment := tokenize("# This is a comment\n# Another comment")
assert_eq_int(len(tokens_comment), 3) # 2 comments + EOF
assert_eq_int(tokens_comment[0].token_type, TokenComment)
assert_eq_int(tokens_comment[1].token_type, TokenComment)
print_test_summary()

# Test operator precedence
test_start("operator precedence - multiplication before addition")
sus parser39 := create_parser(tokenize("1 + 2 * 3"))
sus prec_expr := parse_expression(parser39)
# Should parse as: 1 + (2 * 3)
assert_eq_int(prec_expr.node_type, NodeBinary)
assert_eq_string(prec_expr.value, "+")
assert_eq_string(prec_expr.children[0].value, "1")
assert_eq_int(prec_expr.children[1].node_type, NodeBinary)
assert_eq_string(prec_expr.children[1].value, "*")
print_test_summary()

test_start("operator precedence - comparison before logical")
sus parser40 := create_parser(tokenize("a < b && c > d"))
sus logic_expr := parse_expression(parser40)
# Should parse as: (a < b) && (c > d)
assert_eq_int(logic_expr.node_type, NodeBinary)
assert_eq_string(logic_expr.value, "&&")
assert_eq_int(logic_expr.children[0].node_type, NodeBinary)
assert_eq_string(logic_expr.children[0].value, "<")
print_test_summary()

# Test comprehensive parsing
test_start("parse comprehensive program")
sus comprehensive_source := `
    # Variable declarations
    sus x normie := 42
    sus name tea := "test"
    sus flag lit := based
    
    # Constant declaration
    facts PI := 3.14159
    
    # Type alias
    be_like MyType = normie
    
    # Function declaration
    slay calculate(a normie, b normie) normie {
        sus result := a + b * 2
        damn result
    }
    
    # Expression statement
    calculate(10, 20)
`

sus tokens_comp := tokenize(comprehensive_source)
sus parser41 := create_parser(tokens_comp)
sus comp_program := parse_program(parser41)

assert_eq_int(comp_program.node_type, NodeProgram)
assert_true(len(comp_program.children) >= 6) # Multiple statements

# Check symbol table was populated
assert_true(len(parser41.symbols) >= 4) # x, name, flag, PI, calculate

# Should have no parse errors for valid syntax
sus comp_errors := get_errors(parser41)
assert_eq_int(len(comp_errors), 0)

print_test_summary()

# Integration test - full parse and analysis
test_start("integration test - full source analysis")
sus source_code := `
    sus factorial slay(n normie) normie {
        shook n <= 1 {
            damn 1
        } cap {
            damn n * factorial(n - 1)
        }
    }
    
    sus result := factorial(5)
    vibez.spill(result)
`

sus ast, parse_errors := parse_source(source_code)
assert_eq_int(len(parse_errors), 0)
assert_eq_int(ast.node_type, NodeProgram)
assert_true(len(ast.children) >= 2)

# Verify type checking works
assert_true(check_types(ast, cringe))

print_test_summary()

vibez.spill("All parser tests completed!")
