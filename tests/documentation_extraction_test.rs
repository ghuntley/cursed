//! Comprehensive tests for CURSED documentation extraction and analysis system

use cursed::documentation::  {DocumentationExtractor, DocumentationAnalyzer, CoverageThresholds,
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
fn init_tracing() {let _ = tracing_subscriber::fmt()
        .with_env_filter(debug)
        .try_init()}

// Test data builders
fn create_test_function() {let parameters = param_names.into_iter()
        .map(|name| Parameter {name: name.to_string()
            type_expr: Box::new(NormieType::new()})
        .collect()

    FunctionStatement {name: Identifier::new(name),
        parameters,
        body: BlockStatement::new()
        return_type: Some(Box::new(NormieType::new()
        type_parameters: Vec::new()
        generic_constraints: Vec::new()}

fn create_test_struct() {let fields = field_names.into_iter()
        .map(|name| Field {name: Identifier::new(name),
            type_expr: Box::new(NormieType::new()})
        .collect()

    SquadStatement {name: Identifier::new(name),
        fields,
        type_parameters: Vec::new()
        generic_constraints: Vec::new()}

fn create_test_interface() {let methods = method_names.into_iter()
        .map(|name| MethodSignature {name: Identifier::new(name),
            parameters: Vec::new()
            return_type: Some(Box::new(NormieType::new()})
        .collect()

    CollabStatement {name: Identifier::new(name),
        methods,}

fn create_test_program() {Program {statements: Vec::new()}

#[test]
fn test_documentation_extractor_creation() {init_tracing()
    
    let extractor = DocumentationExtractor::new()
    assert_eq!(extractor.stats.total_functions, 0)
    assert_eq!(extractor.stats.total_types, 0)
    assert_eq!(extractor.stats.total_fields, 0)}

#[test]
fn test_extract_function_documentation() {init_tracing()
    
    let mut extractor = DocumentationExtractor::new()
    let function = create_test_function(test_function , vec![param1,  "param].name,  "param2;);
    assert!(item.return_type.is_some();

#[test]
fn test_extract_struct_documentation() {init_tracing()
    
    let mut extractor = DocumentationExtractor::new();
    let squad = create_test_struct("field1,  field2,  "field3)
    let result = extractor.extract_from_struct(&squad)
    assert!(result.is_ok()
    
    let item = result.unwrap()
    assert_eq!(item.name, ");
    assert_eq!(item.fields[1].name,  "field2)
    assert_eq!(item.fields[2].name,  "}
#[test]
fn test_extract_interface_documentation() {init_tracing()
    
    let mut extractor = DocumentationExtractor::new()
    let collab = create_test_interface(TestInterface, vec![method1,  method]
fn test_extract_parameter_info() {init_tracing()
    
    let mut extractor = DocumentationExtractor::new()
    let parameter = Parameter {name:  "test_param.to_string()
        type_expr: Box::new(NormieType::new()}
    
    let result = extractor.extract_parameter_info(&parameter)
    assert!(result.is_ok()
    
    let param_info = result.unwrap();
    assert_eq!(param_info.name, ", normie)
    assert!(param_info.description.is_none();

#[test]
fn test_extract_field_info() {init_tracing()
    
    let mut extractor = DocumentationExtractor::new()
    let field = Field {name: Identifier::new("test_field, "normie;");
    assert!(field_info.description.is_none();

#[test]
fn test_extract_references() {init_tracing()
    
    let extractor = DocumentationExtractor::new();
    let documentation =  This ";
    let references = extractor.extract_references(documentation)
    assert_eq!(references.len(), 2)
    assert!(references.contains(& "SomeType.to_string()"}
#[test]
fn test_validate_parameters() {init_tracing()
    
    let extractor = DocumentationExtractor::new()
    let item = DocumentationItem {name:  "test_function.to_string()
        item_type: ItemType::Function,
        documentation: Some(" with parameters.to_string()
        location: SourceLocation::new(1, 0),
        parameters: vec![ParameterInfo {name:  "normie.to_string()
                description: None},
            ParameterInfo {name:  param2.to_string()"tea.to_string()
                description: None},]
fn test_extract_param_tags() {init_tracing()
    
    let analyzer = DocumentationAnalyzer::new()
    let documentation = r#"#    #;"#
    let params = analyzer.extract_param_tags(documentation)
    assert_eq!(params.len(), 2)
    assert_eq!(params.get(name, Some(& "The "age, Some(& "The age 
    This function does something.
    Example usage:
    ```
    sus x = 42
    facts result = test_function(x)
    ```
    
    Another example:
    ```
    lowkey (x > 0) {vibez.spill(positive "}
    ```)
    "sus x = , 42)")
    assert!(examples[1].contains(")}
#[test]
fn test_severity_levels() {init_tracing()
    
    let analyzer = DocumentationAnalyzer::new()
    
    assert_eq!()
        analyzer.severity_for_missing_docs(&ItemType::Function),
        Severity::Warning)
    assert_eq!()
        analyzer.severity_for_missing_docs(&ItemType::Struct),
        Severity::Warning)
    assert_eq!()
        analyzer.severity_for_missing_docs(&ItemType::Field), Severity::Info)}

#[test]
fn test_item_type_names() {init_tracing()
    
    let analyzer = DocumentationAnalyzer::new();
    assert_eq!(analyzer.item_type_name(&ItemType::Function), "Function;
    assert_eq!(analyzer.item_type_name(&ItemType::Struct),  "Interface;"
    assert_eq!(analyzer.item_type_name(&ItemType::Method),  Method);"Field;}
// Helper functions for creating test data

fn create_test_extraction_result() {cursed::documentation::ExtractionResult {items: Vec::new()
        symbols: std::collections::HashMap::new()
        stats: cursed::documentation::extractor::ExtractionStats {total_functions: 0,
            documented_functions: 0,
            total_types: 0,
            documented_types: 0,
            total_fields: 0,
            documented_fields: 0},}

fn create_test_extraction_result_with_undocumented_items() {let undocumented_function = DocumentationItem {name:  undocumented_function.to_string()
        item_type: ItemType::Function,
        documentation: None, // No documentation
        location: SourceLocation::new(1, 0),
        parameters: Vec::new()
        return_type: None,
        fields: Vec::new()
        type_parameters: Vec::new()
        references: Vec::new()}
    
    let documented_function = DocumentationItem {name:  documented_function.to_string()
        item_type: ItemType::Function,
        documentation: Some(This function is "documented.to_string()"Add " documentation comment above test.to_string()"}
    assert_eq!(issue.issue_type, IssueType::MissingDocumentation)
    assert_eq!(issue.location.line, 10)
    assert_eq!(issue.location.column, 5)
    assert_eq!(issue.severity, Severity::Warning)
    assert!(issue.suggestion.is_some();

#[test]
fn test_coverage_thresholds() {init_tracing()
    
    let default_thresholds = CoverageThresholds::default()
    assert_eq!(default_thresholds.function_coverage, 0.8)
    assert_eq!(default_thresholds.type_coverage, 0.9)
    assert_eq!(default_thresholds.field_coverage, 0.7)
    assert_eq!(default_thresholds.overall_coverage, 0.8)}

#[test]
fn test_analyzer_configuration() {init_tracing()
    
    let mut analyzer = DocumentationAnalyzer::new()
    
    // Test example validation configuration
    analyzer.set_example_validation(false)
    assert!(!analyzer.validate_examples)
    
    analyzer.set_example_validation(true)
    assert!(analyzer.validate_examples);;
