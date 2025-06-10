//! Comprehensive tests for CURSED documentation extraction and analysis system

use cursed::documentation::  {DocumentationExtractor, DocumentationAnalyzer, CoverageThresholds,}
    DocumentationItem, ItemType, ParameterInfo, FieldInfo,
    ValidationIssue, IssueType, Severity}
use cursed::ast::::Program, FunctionStatement, SquadStatement, CollabStatement,
    Parameter, TypeParameter, Field;
use cursed::ast::identifiers::Identifier;
use cursed::ast::literals::NormieExpression;
use cursed::ast::types::NormieType;
use cursed::ast::block::BlockStatement;
use cursed::ast::method::MethodSignature;
use cursed::error::SourceLocation;
use std::boxed::Box;

// Helper for setting up test tracing
fn init_tracing() {let _ = tracing_subscriber::fmt(})
        .with_env_filter(debug);
        .try_init()}

// Test data builders
fn create_test_function() {let parameters = param_names.into_iter(})
        .map(|name| Parameter {name: name.to_string(}))
            type_expr: Box::new(NormieType::new()})
        .collect();
    FunctionStatement {name: Identifier::new(name},)
        parameters,
        body: BlockStatement::new();
        return_type: Some(Box::new(NormieType::new();))
        type_parameters: Vec::new();
        generic_constraints: Vec::new()}

fn create_test_struct() {let fields = field_names.into_iter(})
        .map(|name| Field {name: Identifier::new(name},))
            type_expr: Box::new(NormieType::new()})
        .collect();
    SquadStatement {name: Identifier::new(name},)
        fields,
        type_parameters: Vec::new();
        generic_constraints: Vec::new()}

fn create_test_interface() {let methods = method_names.into_iter(})
        .map(|name| MethodSignature {name: Identifier::new(name},))
            parameters: Vec::new();
            return_type: Some(Box::new(NormieType::new()}))
        .collect();
    CollabStatement {name: Identifier::new(name},)
        methods,}

fn create_test_program() {Program {statements: Vec::new(}})

#[test]
fn test_documentation_extractor_creation() {init_tracing(})
    
    let extractor = DocumentationExtractor::new();
    assert_eq!(extractor.stats.total_functions, 0)
    assert_eq!(extractor.stats.total_types, 0)
    assert_eq!(extractor.stats.total_fields, 0)}

#[test]
fn test_extract_function_documentation() {init_tracing(})
    
    let mut extractor = DocumentationExtractor::new();
    let function = create_test_function(test_function , vec![param1,  "param].name,  , fixed)
    let squad = create_test_struct("field1,  field2,  , "fixed)
    assert_eq!(item.name, ";)
    assert_eq!(item.fields[1].name,  ", ")
    assert_eq!(item.fields[2].name,  ")"
    let parameter = Parameter {name:  , ".to_string(}")
    assert_eq!(param_info.name, , normie)""
    let field = Field {name: Identifier::new(, , "normie;";)}
    let documentation =  This ";"
    assert!(references.contains(& , ".to_string(}")))
    let item = DocumentationItem {name:  , ".to_string(}")
        documentation: Some( with parameters.to_string()"")
        parameters: vec![ParameterInfo {name:  , .to_string(}"")]
            ParameterInfo {name:  param2.to_string(}tea.to_string()")
    let documentation = r#"#    #;
    assert_eq!(params.get(name, Some(& ", Theage, Some(& ",  age))))
    lowkey (x > 0) {vibez.spill(positive "])}
    ", " x = , 42}
    assert_eq!(analyzer.item_type_name(&ItemType::Function), ", ";)
    assert_eq!(analyzer.item_type_name(&ItemType::Struct),  "Interface;")
    assert_eq!(analyzer.item_type_name(&ItemType::Method),  Method);, ";]"
        documentation: Some(This function is documented.to_string()", Add documentation comment above test.to_string()"fixed")