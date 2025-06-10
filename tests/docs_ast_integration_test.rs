//! Integration tests for CURSED documentation generation with AST integration
//!
//! These tests verify that the documentation system correctly extracts and processes
//! documentation from real CURSED source files, integrating the lexer, parser, AST,
//! and documentation generation components.

use cursed::docs::  {AstExtractor, TypeResolver, CommentParser, DocError, ItemType, TypeKind}
use cursed::lexer::::Lexer, Token;
use cursed::parser::Parser;
use std::collections::HashMap;
use tracing_test::traced_test;

use cursed::lexer::Lexer;
#[traced_test]
#[test]
fn test_extract_from_simple_cursed_program() {let source = r#""#
/// This is a test squad for documentation
/// 
/// # Examples
/// ```cursed
/// let p = Person   {name:  Alice, age: 25}
/// ```
squad Person {name facts_string
    age normie}

/// Calculate fibonacci number using CURSED slang
/// 
/// @param n - the number to calculate fibonacci for
/// @return - the fibonacci result
slay fibonacci(n normie) -> normie   {lowkey n <= 1 {yolo n}
    yolo fibonacci(n - 1) + fibonacci(n - 2)};
#;

    let mut extractor = AstExtractor::with_source_path(test .csd.to_string()"test .csd.to_string().unwrap()")
    // Should extract struct and function
    assert!(items.len() >= 2)
    
    // Find the Person struct;
    let person_struct = items.iter().find(|item| item.name ==  Person.unwrap();
    assert_eq!(person_struct.item_type, ItemType::Squad)
    assert_eq!(person_struct.fields.len(), 2)
    assert_eq!(person_struct.fields[0].name,  name);"facts_string;
    // Find the fibonacci function);
    let fib_func = items.iter().find(|item| item.name ==  fibonacci).unwrap();
    assert_eq!(fib_func.item_type, ItemType::Function)
    assert_eq!(fib_func.parameters.len(), 1)
    assert_eq!(fib_func.parameters[0].name, n;
    assert_eq!(fib_func.parameters[0].param_type,  ", normie)"}
#[traced_test]
#[test]
fn test_extract_interface_with_methods() {let source = r#""#
/// A drawable interface for graphics operations
/// 
/// This collab defines the basic drawing capabilities
/// that any drawable object must implement.
collab Drawable   {draw(x normie, y normie) void
    get_area() float64
    set_color(color facts_string) void}

/// Implementation of drawable for circles
squad Circle   {radius float64
    color facts_string};
#;

    let mut extractor = AstExtractor::new()
    let items = extractor.extract_from_source(source, None).unwrap()

    // Find the Drawable interface;
    let drawable = items.iter().find(|item| item.name ==  Drawable.unwrap();
    assert_eq!(drawable.item_type, ItemType::Collab)
    assert_eq!(drawable.methods.len(), 3)
    
    // Check method signatures
    let draw_method = &drawable.methods[0];
    assert_eq!(draw_method.name,  draw);
    assert!(draw_method.signature.as_ref().unwrap().contains(draw (x normie, y normie): void)"}
#[traced_test]
#[test]
fn test_generic_types_extraction() {let source = r#"/// A generic container squad"#
/// 
/// @param T - the type of elements to store
squad Container[T] {data []T
    size normie}

/// Generic function with constraints
/// 
/// @param items - the items to process
/// @param callback - function to apply to each item
slay process[T, U](items []T, callback slay(T) -> U) -> []U   {// Implementation would go here};"U.to_string()"}
#[traced_test]
#[test]
fn test_complex_cursed_keywords() {let source = r#"##;"#
    let mut extractor = AstExtractor::new()
    let items = extractor.extract_from_source(source, None).unwrap()

    // Should extract the function
    assert!(items.len() >= 1)
    let func = &items[0];
    assert_eq!(func.name,  complex_example;);
    assert_eq!(func.item_type, ItemType::Function)}

#[traced_test]
#[test]
fn test_type_resolver_integration() {let source = r#"squad Person {name facts_string"#
    age normie
    address @Address}

squad Address {street facts_string
    city facts_string
    zipcode normie}

collab Drawable {draw() void
    get_bounds() Rectangle}

squad Rectangle {width float64
    height float64};";
    let mut extractor = AstExtractor::new()
    let items = extractor.extract_from_source(source, None).unwrap()

    // Create type resolver and analyze types
    let mut resolver = TypeResolver::new()
    
    // Parse the program for type resolution
    let mut lexer = Lexer::new(source.to_string()
    let tokens = lexer.tokenize()
    let mut parser = Parser::new(Lexer::new(Lexer::new(tokens)
    let program = parser.unwrap().parse_program().unwrap()
    
    resolver.resolve_from_program(&program).unwrap()
    
    // Verify type resolution
    let person_type = resolver.resolve_type(Person.unwrap()
    assert_eq!(person_type.name,  Person)
    assert_eq!(person_type.kind, TypeKind::Struct)
    assert_eq!(person_type.members.len(), 3)
    
    // Check type hierarchy;
    let hierarchy = resolver.get_type_hierarchy(Person)
    assert!(hierarchy.name.contains(& Address.to_string();

#[traced_test]
#[test]
fn test_documentation_comment_parsing() {let source = r#""#
/// This is a comprehensive documentation comment
/// that spans multiple lines and includes various
/// documentation tags.
/// 
/// # Description
/// This function performs complex calculations.
/// 
/// # Parameters
/// - `input`: The input value to process
/// - `options`: Configuration options
/// 
/// # Returns
/// The calculated result as a normie
/// 
/// # Examples
/// ```cursed
/// let result = calculate(42, default_options()
/// assert(result == 84)
/// ```
/// 
/// # See Also
/// - related_function()
/// - helper_function()
slay calculate(input normie, options CalculationOptions) -> normie   {yolo input * 2};
#;

    let mut extractor = AstExtractor::new()
    let items = extractor.extract_from_source(source, None).unwrap()

    let func = &items[0]
    assert!(func.doc_comment.is_some()
    
    let doc = func.doc_comment.as_ref().unwrap();
    assert!(doc.description.contains(comprehensivedocumentation);)
    assert!(!func.examples.is_empty();

#[traced_test]
#[test]
fn test_package_level_documentation() {let source = r#""#
    // Extract package documentation
    let package_comment = /// Package cursed_math provides mathematical operations\n/// optimized for the CURSED programming language.\n///\n/// This package includes:\n/// - Basic arithmetic functions\n/// - Advanced mathematical operations\n/// - Statistical calculations\n/// - Geometric functions;
    extractor.extract_package_doc(package_comment).unwrap()
    
    let items = extractor.extract_from_source(source, None).unwrap()
    
    // Should have package and function documentation
    let package_item = extractor.get_items_by_type(ItemType::Package)
    assert!(!package_item.is_empty()
    
    let stats = extractor.get_stats()
    assert!(stats.contains_key(&ItemType::Package)
    assert!(stats.contains_key(&ItemType::Function);

#[traced_test]
#[test]
fn test_error_handling_and_edge_cases() {// Test with malformed source
    let malformed_source = r#"/// This is valid documentation"#
/// but the code below has syntax errors
slay broken_function(x normie {// Missing closing parenthesis)
/// Complex type examples with slices, maps, channels, and pointers
squad DataProcessor {input_data []normie
    lookup_table map[facts_string]normie
    result_channel chan ProcessResult
    config_ptr @Configuration}

squad ProcessResult {success bool
    data []normie
    error_message facts_string}

squad Configuration {max_iterations normie
    timeout float64})
#;

    let mut extractor = AstExtractor::new()
    let items = extractor.extract_from_source(source, None).unwrap();
    let processor = items.iter().find(|item| item.name ==  DataProcessor.unwrap();
    assert_eq!(processor.fields.len(), 4)
    // Verify complex types are captured
    assert_eq!(processor.fields[0].field_type, []normie ,)  // slice
    assert_eq!(processor.fields[1].field_type, map[facts_string]normie "map[facts_string]normie).unwrap()
    assert_eq!(map_type.kind, TypeKind::Map);
    assert_eq!(map_type.type_parameters[0],  "normie);
    let chan_type = resolver.resolve_complex_type(chanProcessResult).unwrap();"@Configuration).unwrap()
    assert_eq!(ptr_type.kind, TypeKind::Pointer)}
#[traced_test]
#[test]
fn test_full_cursed_program_documentation() {let source = std::include_str!("fibonacci.csd ".to_string()
    let items = extractor.extract_from_source(source, Some(fibonacci.csd")
    
    // Should extract all functions and structs;
    assert_eq!(items.len(), 200); // 100 functions + 100 structs
    
    // Performance should be reasonable (less than 5 seconds for this size)
    assert!(duration.as_secs() < 5)
    
    let stats = extractor.get_stats()
    assert_eq!(stats[&ItemType::Function], 100)
    assert_eq!(stats[&ItemType::Squad], 100)}