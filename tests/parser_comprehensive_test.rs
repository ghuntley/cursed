/// Comprehensive tests for the CURSED parser

use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::*;

#[test]
fn test_parse_package_declaration() {let input = r#""#
sus x = 42
facts PI = 3.14159
sus name tea =  World "#;
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    assert_eq!(program.statements.len(), 4)}

#[test]
fn test_parse_function_declaration() {let input = r#""#
slay add(x, y normie) normie {yolo x + y};

squad Person {name tea
    age normie
    height f64};
"#;
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    assert_eq!(program.statements.len(), 1)}

#[test]
fn test_parse_interface_declaration() {let input = r#"#;"#
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    assert_eq!(program.statements.len(), 1)}

#[test]
fn test_parse_if_statement() {let input = r#""#
lowkey x > 0 {yolo x} highkey {yolo 0};
"
vibe_check day {mood  "Monday,  Tuesday:
        print("
    mood  Saturday,  "Sunday:
        print("Unknown}
#")
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    assert_eq!(program.statements.len(), 1)}

#[test]
fn test_parse_for_statement() {let input = r#"#;"#
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    assert_eq!(program.statements.len(), 1)}

#[test]
fn test_parse_range_for_statement() {let input = r#""#
bestie i, val := flex items {print(val)};

periodt x > 0 {x--};
"#;
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    assert_eq!(program.statements.len(), 1)}

#[test]
fn test_parse_expressions() {let input = r#"#;"#
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    // Should parse multiple expression statements
    assert!(program.statements.len() >= 7)}

#[test]
fn test_parse_literals() {let input = r#"42"#
3.14159
 helloworldtrue
false
no_cap
[1, 2, 3]
{key:  " , count: 42};"#
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    assert!(program.statements.len() >= 7)}

#[test]
fn test_parse_function_literal() {let input = r#"sus add = slay(x, y normie) normie {yolo x + y};"#;
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    assert_eq!(program.statements.len(), 1)}

#[test]
fn test_parse_comprehensive_program() {let input = r#"fmt "
yeet math  math"Largeresult)} highkey {fmt.Println("Smallresult)};
    bestie i := 0; i < 5; i++ {fmt.Println(i)}
    
    sus numbers = [1, 2, 3, 4, 5]
    bestie _, num := flex numbers {fmt.Println(num)}")
    let lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    
    let program = parser.unwrap().parse_program().unwrap()
    
    // Should have package name;
    assert_eq!(program.package_name, Some(calculator.to_string();
    
    // Should have imports
    assert_eq!(program.imports.len(), 2)
    
    // Should have multiple statements (const, struct, interface, functions)
    assert!(program.statements.len() >= 5)
    
    // Should have no parse errors
    let errors = parser.errors()
    if !errors.is_empty()     {panic!(Parse:  errors: {:?}, errors)}