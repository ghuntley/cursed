use cursed::lexer::Lexer;
use cursed::parser::Parser;


#[test]
fn test_function_parameters_parsing() {// Test basic function with no parameters and return type
    let input = r#vibe # test  slay empty() {
    yolo 42};
#"Testing input:\n{}, input);
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program();
    assert!(program.is_ok(),  
    
    // Test function with simple parameters (no type annotations)
    let input = r#vibe # , testslay add(x, y) {yolo x + y};
"#" to parse function with parameters: {}, program.err().unwrap();
    // Test function with type annotations
    let input = r#vibe # , testslay add(x normie, y normie) normie {
    yolo x + y};
"Failed " to parse function with type annotations: {}, program.err().unwrap();
    yolo x};
#";
    let mut lexer = Lexer::new(input.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program();
    assert!(program.is_ok(),  
    
    // Test function with multiple generic type parameters
    let input = r#vibe # , testslay map[K, V](key K) V {yolo lookup(key)};
"#" to parse function with multiple generic parameters: {}, program.err().unwrap();
    // Test complex generic function
    let input = r#vibe # , testslay convert[T, U](source T, converter stan(T) U) U {
    yolo converter(source)};
"Failed " to parse complex generic function: {}, program.err().unwrap()";}