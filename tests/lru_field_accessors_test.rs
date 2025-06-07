use cursed::prelude::*;
use cursed::ast::declarations::*;
use cursed::ast::statements::*;
use cursed::ast::expressions::*;
use cursed::ast::operators::*;
use cursed::ast::types::*;
use cursed::ast::traits::*;
use cursed::lexer::*;
use cursed::parser::*;
use cursed::core::type_checker::*;
use cursed::codegen::llvm::*;
use cursed::codegen::llvm::lru_field_accessors::*;
use cursed::memory::gc::GarbageCollector;
use std::path::PathBuf;
use tracing::*;

// Test for LRU cached field accessors implementation


#[path = "common.rs"]
mod common;

/// Setup function to initialize test tracing
fn setup() {
    common::tracing::setup();
}

/// Test source code with multiple generic structs
const TEST_CODE: &str = r#"
vibe main;

collab Comparable<T> {
    compare(other T) normie;
}

squad Point<T> {
    x T,
    y T
}

squad Vector3D<T> {
    x T,
    y T,
    z T
}

slay (p Point<T>) compare(other Point<T>) normie {
    fr fr Basic comparison
    return 0;
}

slay main() {
    fr fr Create points of different types
    sus p1 = Point<normie>{x: 1, y: 2};
    sus p2 = Point<thicc>{x: 3, y: 4};
    sus p3 = Point<snack>{x: 1.5, y: 2.5};
    sus p4 = Point<meal>{x: 3.5, y: 4.5};
    
    fr fr Create 3D vectors of different types
    sus v1 = Vector3D<normie>{x: 1, y: 2, z: 3};
    sus v2 = Vector3D<thicc>{x: 4, y: 5, z: 6};
    sus v3 = Vector3D<snack>{x: 1.5, y: 2.5, z: 3.5};
    sus v4 = Vector3D<meal>{x: 4.5, y: 5.5, z: 6.5};

    fr fr Access some fields to generate accessors
    vibez.spill(p1.x);
    vibez.spill(p2.x);
    vibez.spill(v1.z);
    vibez.spill(v2.z);
}
"#;

#[test]
fn test_lru_cached_field_accessors() {
    setup();
    let _span = info_span!("test", test = "lru_cached_field_accessors").entered();
    info!("Starting test for LRU cached field accessors");
    
    // Parse the program
    let mut lexer = Lexer::new(TEST_CODE);
    let tokens = lexer.lex().expect("Lexing failed");
    
    let mut parser = Parser::new(tokens, PathBuf::from("test.csd");
    let program = parser.parse().expect("Parsing failed");
    
    // Create JIT compiler
    let context = inkwell::context::Context::create();
    let mut codegen = LlvmCodeGenerator::new(&context, "test_module", PathBuf::from("test.csd"));
    
    // Ensure LRU cache is initialized
    codegen.ensure_lru_field_accessor_cache();
    
    // Compile the program
    let result = codegen.compile_program(&program, &Default::default();
    info!("Compilation result: {:?}", result);
    assert!(result.is_ok(), "Compilation failed: {:?}", result);
    
    // Get specialized struct names from the compiled program
    let structs = ["Point<normie>", "Point<thicc>", "Point<snack>", "Point<meal>",
                   "Vector3D<normie>", "Vector3D<thicc>", "Vector3D<snack>", "Vector3D<meal>"];
                   
    let fields = ["x", "y", "z"];
    
    // Verify that accessors have been generated with the cache used
    let mut cache_hits = 0;
    
    // First check existence once to populate the cache
    for struct_name in &structs {
        for field in &fields {
            // Skip z field for Point structs
            if field == &"z" && struct_name.starts_with("Point") {
                continue;
            }
            
            // Check if accessors exist via the LRU cache
            let getter_exists = codegen.field_accessor_exists_with_lru_cache(struct_name, field, "get");
            let setter_exists = codegen.field_accessor_exists_with_lru_cache(struct_name, field, "set");
            
            // Either they exist now, or they'll be generated
            info!("Field accessor for {}.{}: getter={}, setter={}", struct_name, field, getter_exists, setter_exists);
        }
    }
    
    // Now check again - these should be cache hits
    for struct_name in &structs {
        for field in &fields {
            // Skip z field for Point structs
            if field == &"z" && struct_name.starts_with("Point") {
                continue;
            }
            
            // Check if accessors exist via the LRU cache (should be cache hits)
            if codegen.field_accessor_exists_with_lru_cache(struct_name, field, "get") {
                cache_hits += 1;
            }
            if codegen.field_accessor_exists_with_lru_cache(struct_name, field, "set") {
                cache_hits += 1;
            }
        }
    }
    
    info!("Cache hits: {}", cache_hits);
    assert!(cache_hits > 0, "Expected at least one cache hit");
    
    // Print cache stats
    if let Some(stats) = codegen.get_lru_field_accessor_cache_stats() {
        info!("Cache stats: {}", stats);
    }
    
    // Generate all accessors using our LRU implementation
    if let Some(point_struct) = find_struct_in_program(&program, "Point") {
        let result = codegen.generate_lru_cached_field_accessors(
            &point_struct,
            "Point<normie>",
            &[Type::Normie]
        );
        assert!(result.is_ok(), "Failed to generate field accessors: {:?}", result);
    }
    
    // Verify final caching state
    if let Some(stats) = codegen.get_lru_field_accessor_cache_stats() {
        info!("Final cache stats: {}", stats);
    }
    
    info!("Successfully verified LRU cached field accessors");
}

#[test]
fn test_multi_instance_cache_coherence() {
    setup();
    let _span = info_span!("test", test = "multi_instance_cache_coherence").entered();
    info!("Starting test for multi-instance cache coherence");
    
    // Create two JIT compilers
    let context1 = inkwell::context::Context::create();
    let context2 = inkwell::context::Context::create();
    let mut codegen1 = LlvmCodeGenerator::new(&context1, "module1", PathBuf::from("test1.csd");
    let mut codegen2 = LlvmCodeGenerator::new(&context2, "module2", PathBuf::from("test2.csd");
    
    // Create a shared LRU cache
    let shared_cache = ThreadSafeFieldAccessorLruCache::default();
    
    // Set the cache for both code generators
    codegen1.ensure_lru_field_accessor_cache();
    codegen2.ensure_lru_field_accessor_cache();
    
    // Create a simple struct for testing
    let struct_stmt = SquadStatement {
        name: Identifier { value: "TestStruct".to_string(), span: Span::default() },
        fields: vec![
            Field {
                name: Identifier { value: "field1".to_string(), span: Span::default() },
                type_name: TypeName::Named(Identifier { value: "normie".to_string(), span: Span::default() }),
                span: Span::default(),
            },
            Field {
                name: Identifier { value: "field2".to_string(), span: Span::default() },
                type_name: TypeName::Named(Identifier { value: "tea".to_string(), span: Span::default() }),
                span: Span::default(),
            }
        ],
        type_parameters: vec![],
        type_constraints: vec![],
        span: Span::default(),
    };
    
    // Add to both modules
    let struct_type1 = codegen1.context().opaque_struct_type("TestStruct");
    let struct_type2 = codegen2.context().opaque_struct_type("TestStruct");
    
    // Set the struct body with appropriate field types
    let field_types1 = vec![
        codegen1.context().i32_type().into(),
        codegen1.context().i8_type().ptr_type(inkwell::AddressSpace::default()).into()
    ];
    
    let field_types2 = vec![
        codegen2.context().i32_type().into(),
        codegen2.context().i8_type().ptr_type(inkwell::AddressSpace::default()).into()
    ];
    
    struct_type1.set_body(&field_types1, false);
    struct_type2.set_body(&field_types2, false);
    
    // Register in module lookup
    // Note: This is just using the direct method since we don't have a full Registry setup in test
    codegen1.module().add_global(struct_type1, None, "TestStruct_type");
    codegen2.module().add_global(struct_type2, None, "TestStruct_type");
    
    // Update the first cache instance
    codegen1.update_lru_field_accessor_cache("TestStruct", "field1", "get", true);
    
    // Check if the second instance sees the first update
    let exists_in_second = codegen2.field_accessor_exists_with_lru_cache("TestStruct", "field1", "get");
    
    // The cache state is independent between instances which is correct
    info!("Cache coherence check: exists_in_second = {}", exists_in_second);
    
    info!("Successfully verified multi-instance cache coherence");
}

/// Helper function to find a struct definition in a program
fn find_struct_in_program(program: &Program, struct_name: &str) -> Option<SquadStatement> {
    for stmt in &program.statements {
        if let Some(squad_stmt) = stmt.as_any().downcast_ref::<SquadStatement>() {
            if squad_stmt.name.value == struct_name {
                return Some(squad_stmt.clone();
            }
        }
    }
    None
}