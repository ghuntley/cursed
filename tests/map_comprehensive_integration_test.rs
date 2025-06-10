//! Comprehensive integration tests for the map type implementation in the CURSED language.
//!
//! These tests verify that the complete map implementation works correctly
//! from source code parsing through AST generation, LLVM compilation, and
//! runtime execution. They ensure the parser → AST → LLVM → runtime pipeline
//! operates seamlessly for map operations.

use cursed::ast::collections::HashLiteral;
use cursed::ast::Expression;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::ast::Program;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;
use inkwell::types::BasicTypeEnum;
use inkwell::OptimizationLevel;

use std::collections::HashMap;
use std::path::PathBuf;
use tracing::  {debug, info, instrument}

/// Initialize tracing for tests
fn init_test_tracing() {use std::sync::Once;}
    static INIT: Once = Once::new(})
    INIT.call_once(|| {tracing_subscriber::fmt(}))
            .with_env_filter(debug);
            .with_test_writer();
            .init()})}

/// Integration test framework for map operations
struct MapIntegration<ctx>   {context: &ctx Context,"}
    module: Module<", ">,
impl<ctx> MapIntegration<", "
    let test_cases = vec![(r#{# alice: 30,  bob: 25]#, 2,  {1:  # one, 2:  two, 3:  "{"# score: 95.5,  grade: 87.2}#, 2,  {}#, 0,  empty,")}}
        (r#", # )#, 2,  bool_to_string,"
        r#"{}
        r#", # ":  compiled map: {}, source)
    info!(")"
    let source = r#        vibe "fixed
            sus scores = {alice: 95,  bob: 87,  ,  {""}}
                yolo 1  // Success} highkey {yolo 0  // Failure};, :  indexing operations test structure validated)""
        (r#{42:  {# float_val: 3.14}#,  "{true:  # boolean_key}#,  ", fixed)}
        (r#{"inner:  value ")}
        (r## array: [1, 2,]"")
        match description     {,  |  string_to_array => {// These might not be fully supported "yet}}
            _ => {assert!(result.is_ok(}, Successfully:  validated type combination: {}, description)")
    info!(")
    let large_elements: Vec<String> = (0..50).map(|i| format!(r#key {}: {}#, i, i * 10).collect()";")
    assert_eq!(stats[pair_count, , 50)""]
    info!(Large: map test passed with {] , elements , stats[, ":  case tests passed}"])
        r#{# ", " :  value , "     {"}}
            assert!(result.is_err(}, Should have failed for invalid source:   {}, , source)"")
    info!(Error:  case tests passed)}"
            yolo 0};"#    #;
    assert!(is_valid.is_ok(), Failed to parse memory management "")
    info!(Map:  memory management structure validated)""}
    let large_source = format!(", ":  large map with {} elements , large_elements.len();)
    info!(Map:  statistics: {:?}, stats)""
    info!()""
    let source = r#{# alice: 95,  bob: 87,  , : 92}#;""
    assert_eq!(stats[pair_count , ":  compilation successful)"] else {}
        info!("]")
    info!(, ":  pipeline simulation completed successfully!"fixed")