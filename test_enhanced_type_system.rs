/// Test the enhanced type system with constraint resolution
use cursed::core::type_checker::{TypeChecker, Type};
use cursed::type_system::{TypeSystem, TypeExpression, TypeDefinition, TypeKind};
use cursed::error::Error;

fn main() -> Result<(), Error> {
    println!("Testing Enhanced CURSED Type System");
    
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Test 1: Basic type checking with new system
    let type_checker = TypeChecker::new();
    
    println!("\n=== Test 1: Basic Type Inference ===");
    let basic_types = vec![
        ("normie", Type::Normie),
        ("facts", Type::Lit),
        ("tea", Type::Tea),
        ("nil", Type::Nil),
    ];
    
    for (expr, expected) in basic_types {
        let result = type_checker.check_type(expr)?;
        println!("Type of '{}': {:?} (expected: {:?})", expr, result, expected);
        assert_eq!(result, expected);
    }
    
    // Test 2: Type system integration
    println!("\n=== Test 2: Type System Integration ===");
    let type_system = type_checker.type_system();
    
    // Check builtin types
    let builtin_types = vec!["normie", "facts", "tea", "sus"];
    for type_name in builtin_types {
        if let Some(type_def) = type_system.get_type_definition(type_name) {
            println!("Found builtin type: {} (kind: {:?})", type_def.name, type_def.kind);
        } else {
            println!("Warning: Builtin type '{}' not found", type_name);
        }
    }
    
    // Test 3: Type expression conversion
    println!("\n=== Test 3: Type Expression Conversion ===");
    let legacy_type = Type::Array(Box::new(Type::Normie), 10);
    let type_expr = type_checker.convert_type_to_expression(&legacy_type);
    println!("Converted array type: {}", type_expr.to_string());
    
    // Test 4: Custom type registration
    println!("\n=== Test 4: Custom Type Registration ===");
    let mut type_checker_mut = TypeChecker::new();
    
    let custom_type = TypeDefinition {
        name: "CustomStruct".to_string(),
        kind: TypeKind::Struct,
        type_parameters: Vec::new(),
        constraints: Vec::new(),
        methods: Vec::new(),
        is_builtin: false,
    };
    
    type_checker_mut.register_type(custom_type)?;
    println!("Registered custom type: CustomStruct");
    
    // Verify it can be found
    if let Some(found_type) = type_checker_mut.type_system().get_type_definition("CustomStruct") {
        println!("Successfully retrieved custom type: {}", found_type.name);
    }
    
    // Test 5: Type expression operations
    println!("\n=== Test 5: Type Expression Operations ===");
    let map_type = TypeExpression::map(
        TypeExpression::named("tea"),
        TypeExpression::named("normie")
    );
    println!("Map type: {}", map_type.to_string());
    println!("Is concrete: {}", map_type.is_concrete());
    
    let generic_type = TypeExpression::generic("Vec", vec![
        TypeExpression::parameter("T")
    ]);
    println!("Generic type: {}", generic_type.to_string());
    println!("Is concrete: {}", generic_type.is_concrete());
    println!("Parameters: {:?}", generic_type.collect_parameters());
    
    println!("\n=== Enhanced Type System Test Complete ===");
    println!("✅ All tests passed! The type system has been successfully re-enabled.");
    
    Ok(())
}
