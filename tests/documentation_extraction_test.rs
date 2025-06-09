//! Comprehensive tests for CURSED documentation extraction and analysis system

use cursed::documentation::{
    DocumentationExtractor, DocumentationAnalyzer, CoverageThresholds,
    DocumentationItem, ItemType, ParameterInfo, FieldInfo,
    ValidationIssue, IssueType, Severity
};
use cursed::ast::{
    Program, FunctionStatement, SquadStatement, CollabStatement,
    Parameter, TypeParameter, Field
};
use cursed::ast::expressions::identifiers::Identifier;
use cursed::ast::expressions::literals::NormieExpression;
use cursed::ast::expressions::types::NormieType;
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::declarations::method::MethodSignature;
use cursed::error::SourceLocation;
use std::boxed::Box;

// Helper for setting up test tracing
fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();
}

// Test data builders
fn create_test_function(name: &str, param_names: Vec<&str>) -> FunctionStatement {
    let parameters = param_names.into_iter()
        .map(|name| Parameter {
            name: name.to_string(),
            type_expr: Box::new(NormieType::new()),
        })
        .collect();

    FunctionStatement {
        token: "slay".to_string(),
        name: Identifier::new(name),
        parameters,
        body: BlockStatement::new(),
        return_type: Some(Box::new(NormieType::new())),
        type_parameters: Vec::new(),
        generic_constraints: Vec::new(),
    }
}

fn create_test_struct(name: &str, field_names: Vec<&str>) -> SquadStatement {
    let fields = field_names.into_iter()
        .map(|name| Field {
            name: Identifier::new(name),
            type_expr: Box::new(NormieType::new()),
        })
        .collect();

    SquadStatement {
        token: "squad".to_string(),
        name: Identifier::new(name),
        fields,
        type_parameters: Vec::new(),
        generic_constraints: Vec::new(),
    }
}

fn create_test_interface(name: &str, method_names: Vec<&str>) -> CollabStatement {
    let methods = method_names.into_iter()
        .map(|name| MethodSignature {
            name: Identifier::new(name),
            parameters: Vec::new(),
            return_type: Some(Box::new(NormieType::new())),
        })
        .collect();

    CollabStatement {
        token: "collab".to_string(),
        name: Identifier::new(name),
        methods,
    }
}

fn create_test_program() -> Program {
    Program {
        statements: Vec::new(),
    }
}

#[test]
fn test_documentation_extractor_creation() {
    init_tracing();
    
    let extractor = DocumentationExtractor::new();
    assert_eq!(extractor.stats.total_functions, 0);
    assert_eq!(extractor.stats.total_types, 0);
    assert_eq!(extractor.stats.total_fields, 0);
}

#[test]
fn test_extract_function_documentation() {
    init_tracing();
    
    let mut extractor = DocumentationExtractor::new();
    let function = create_test_function("test_function", vec!["param1", "param2"]);
    
    let result = extractor.extract_from_function(&function);
    assert!(result.is_ok());
    
    let item = result.unwrap();
    assert_eq!(item.name, "test_function");
    assert_eq!(item.item_type, ItemType::Function);
    assert_eq!(item.parameters.len(), 2);
    assert_eq!(item.parameters[0].name, "param1");
    assert_eq!(item.parameters[1].name, "param2");
    assert!(item.return_type.is_some());
}

#[test]
fn test_extract_struct_documentation() {
    init_tracing();
    
    let mut extractor = DocumentationExtractor::new();
    let squad = create_test_struct("TestStruct", vec!["field1", "field2", "field3"]);
    
    let result = extractor.extract_from_struct(&squad);
    assert!(result.is_ok());
    
    let item = result.unwrap();
    assert_eq!(item.name, "TestStruct");
    assert_eq!(item.item_type, ItemType::Struct);
    assert_eq!(item.fields.len(), 3);
    assert_eq!(item.fields[0].name, "field1");
    assert_eq!(item.fields[1].name, "field2");
    assert_eq!(item.fields[2].name, "field3");
}

#[test]
fn test_extract_interface_documentation() {
    init_tracing();
    
    let mut extractor = DocumentationExtractor::new();
    let collab = create_test_interface("TestInterface", vec!["method1", "method2"]);
    
    let result = extractor.extract_from_interface(&collab);
    assert!(result.is_ok());
    
    let item = result.unwrap();
    assert_eq!(item.name, "TestInterface");
    assert_eq!(item.item_type, ItemType::Interface);
    assert_eq!(item.fields.len(), 2); // Methods are stored as fields for interfaces
    assert_eq!(item.fields[0].name, "method1");
    assert_eq!(item.fields[1].name, "method2");
}

#[test]
fn test_extract_parameter_info() {
    init_tracing();
    
    let mut extractor = DocumentationExtractor::new();
    let parameter = Parameter {
        name: "test_param".to_string(),
        type_expr: Box::new(NormieType::new()),
    };
    
    let result = extractor.extract_parameter_info(&parameter);
    assert!(result.is_ok());
    
    let param_info = result.unwrap();
    assert_eq!(param_info.name, "test_param");
    assert_eq!(param_info.type_name, "normie");
    assert!(param_info.description.is_none());
}

#[test]
fn test_extract_field_info() {
    init_tracing();
    
    let mut extractor = DocumentationExtractor::new();
    let field = Field {
        name: Identifier::new("test_field"),
        type_expr: Box::new(NormieType::new()),
    };
    
    let result = extractor.extract_field_info(&field);
    assert!(result.is_ok());
    
    let field_info = result.unwrap();
    assert_eq!(field_info.name, "test_field");
    assert_eq!(field_info.type_name, "normie");
    assert!(field_info.description.is_none());
}

#[test]
fn test_extract_references() {
    init_tracing();
    
    let extractor = DocumentationExtractor::new();
    let documentation = "This function uses [SomeType] and calls [another_function] internally.";
    
    let references = extractor.extract_references(documentation);
    assert_eq!(references.len(), 2);
    assert!(references.contains(&"SomeType".to_string()));
    assert!(references.contains(&"another_function".to_string()));
}

#[test]
fn test_validate_parameters() {
    init_tracing();
    
    let extractor = DocumentationExtractor::new();
    let item = DocumentationItem {
        name: "test_function".to_string(),
        item_type: ItemType::Function,
        documentation: Some("Function with parameters".to_string()),
        location: SourceLocation::new(1, 0),
        parameters: vec![
            ParameterInfo {
                name: "param1".to_string(),
                type_name: "normie".to_string(),
                description: None,
            },
            ParameterInfo {
                name: "param2".to_string(),
                type_name: "tea".to_string(),
                description: None,
            },
        ],
        return_type: Some("normie".to_string()),
        fields: Vec::new(),
        type_parameters: Vec::new(),
        references: Vec::new(),
    };
    
    let issues = extractor.validate_parameters(&item);
    assert_eq!(issues.len(), 2); // Both parameters are undocumented
    assert!(issues[0].contains("param1"));
    assert!(issues[1].contains("param2"));
}

#[test]
fn test_resolve_references() {
    init_tracing();
    
    let mut extractor = DocumentationExtractor::new();
    
    // Add a known symbol
    let known_item = DocumentationItem {
        name: "KnownType".to_string(),
        item_type: ItemType::Struct,
        documentation: None,
        location: SourceLocation::new(1, 0),
        parameters: Vec::new(),
        return_type: None,
        fields: Vec::new(),
        type_parameters: Vec::new(),
        references: Vec::new(),
    };
    extractor.symbols.insert("KnownType".to_string(), known_item);
    
    // Create item with references
    let item_with_refs = DocumentationItem {
        name: "test_function".to_string(),
        item_type: ItemType::Function,
        documentation: None,
        location: SourceLocation::new(2, 0),
        parameters: Vec::new(),
        return_type: None,
        fields: Vec::new(),
        type_parameters: Vec::new(),
        references: vec!["KnownType".to_string(), "UnknownType".to_string()],
    };
    
    let resolved = extractor.resolve_references(&item_with_refs);
    assert_eq!(resolved.len(), 2);
    assert_eq!(resolved.get("KnownType"), Some(&true));
    assert_eq!(resolved.get("UnknownType"), Some(&false));
}

#[test]
fn test_documentation_analyzer_creation() {
    init_tracing();
    
    let analyzer = DocumentationAnalyzer::new();
    assert!(analyzer.coverage_thresholds.function_coverage > 0.0);
    assert!(analyzer.coverage_thresholds.type_coverage > 0.0);
}

#[test]
fn test_analyzer_with_custom_thresholds() {
    init_tracing();
    
    let thresholds = CoverageThresholds {
        function_coverage: 0.9,
        type_coverage: 0.95,
        field_coverage: 0.8,
        overall_coverage: 0.85,
    };
    
    let analyzer = DocumentationAnalyzer::with_thresholds(thresholds.clone());
    assert_eq!(analyzer.coverage_thresholds.function_coverage, 0.9);
    assert_eq!(analyzer.coverage_thresholds.type_coverage, 0.95);
    assert_eq!(analyzer.coverage_thresholds.field_coverage, 0.8);
    assert_eq!(analyzer.coverage_thresholds.overall_coverage, 0.85);
}

#[test]
fn test_calculate_coverage() {
    init_tracing();
    
    let analyzer = DocumentationAnalyzer::new();
    let mut extraction_result = create_test_extraction_result();
    
    // Set up test data with some documented and undocumented items
    extraction_result.stats.total_functions = 5;
    extraction_result.stats.documented_functions = 3;
    extraction_result.stats.total_types = 4;
    extraction_result.stats.documented_types = 2;
    extraction_result.stats.total_fields = 10;
    extraction_result.stats.documented_fields = 6;
    
    let coverage = analyzer.calculate_coverage(&extraction_result);
    
    assert_eq!(coverage.total_functions, 5);
    assert_eq!(coverage.documented_functions, 3);
    assert_eq!(coverage.function_coverage, 0.6); // 3/5
    
    assert_eq!(coverage.total_types, 4);
    assert_eq!(coverage.documented_types, 2);
    assert_eq!(coverage.type_coverage, 0.5); // 2/4
    
    assert_eq!(coverage.total_public_fields, 10);
    assert_eq!(coverage.documented_public_fields, 6);
    assert_eq!(coverage.field_coverage, 0.6); // 6/10
    
    // Overall coverage: (3+2+6)/(5+4+10) = 11/19 ≈ 0.58
    assert!((coverage.overall_coverage - 0.5789).abs() < 0.001);
}

#[test]
fn test_validate_completeness() {
    init_tracing();
    
    let analyzer = DocumentationAnalyzer::new();
    let extraction_result = create_test_extraction_result_with_undocumented_items();
    let coverage = analyzer.calculate_coverage(&extraction_result);
    
    let issues = analyzer.validate_completeness(&extraction_result, &coverage);
    
    // Should find issues for undocumented functions and types
    assert!(!issues.is_empty());
    
    // Check that missing documentation issues are found
    let missing_doc_issues: Vec<_> = issues.iter()
        .filter(|issue| issue.issue_type == IssueType::MissingDocumentation)
        .collect();
    assert!(!missing_doc_issues.is_empty());
}

#[test]
fn test_string_similarity() {
    init_tracing();
    
    let analyzer = DocumentationAnalyzer::new();
    
    assert!(analyzer.string_similarity("test", "test") > 0.9);
    assert!(analyzer.string_similarity("test", "Test") > 0.7);
    assert!(analyzer.string_similarity("test", "best") > 0.5);
    assert!(analyzer.string_similarity("test", "xyz") < 0.5);
    assert_eq!(analyzer.string_similarity("", "test"), 0.0);
    assert_eq!(analyzer.string_similarity("test", ""), 0.0);
}

#[test]
fn test_find_similar_symbols() {
    init_tracing();
    
    let analyzer = DocumentationAnalyzer::new();
    let mut symbols = std::collections::HashSet::new();
    symbols.insert("TestType".to_string());
    symbols.insert("TestFunction".to_string());
    symbols.insert("OtherType".to_string());
    symbols.insert("CompletelyDifferent".to_string());
    
    let suggestions = analyzer.find_similar_symbols("TestTpe", &symbols);
    
    // Should find TestType as similar
    assert!(suggestions.contains(&"TestType".to_string()));
    assert!(suggestions.len() <= 3); // Limited to 3 suggestions
}

#[test]
fn test_extract_param_tags() {
    init_tracing();
    
    let analyzer = DocumentationAnalyzer::new();
    let documentation = r#"
    This is a function description.
    @param name The name parameter
    @param age The age parameter  
    @return The result value
    "#;
    
    let params = analyzer.extract_param_tags(documentation);
    assert_eq!(params.len(), 2);
    assert_eq!(params.get("name"), Some(&"The name parameter".to_string()));
    assert_eq!(params.get("age"), Some(&"The age parameter".to_string()));
}

#[test]
fn test_extract_code_examples() {
    init_tracing();
    
    let analyzer = DocumentationAnalyzer::new();
    let documentation = r#"
    This function does something.
    
    Example usage:
    ```
    sus x = 42
    facts result = test_function(x)
    ```
    
    Another example:
    ```
    lowkey (x > 0) {
        vibez.spill("positive")
    }
    ```
    "#;
    
    let examples = analyzer.extract_code_examples(documentation);
    assert_eq!(examples.len(), 2);
    assert!(examples[0].contains("sus x = 42"));
    assert!(examples[1].contains("lowkey (x > 0)"));
}

#[test]
fn test_severity_levels() {
    init_tracing();
    
    let analyzer = DocumentationAnalyzer::new();
    
    assert_eq!(
        analyzer.severity_for_missing_docs(&ItemType::Function),
        Severity::Warning
    );
    assert_eq!(
        analyzer.severity_for_missing_docs(&ItemType::Struct),
        Severity::Warning
    );
    assert_eq!(
        analyzer.severity_for_missing_docs(&ItemType::Field),
        Severity::Info
    );
}

#[test]
fn test_item_type_names() {
    init_tracing();
    
    let analyzer = DocumentationAnalyzer::new();
    
    assert_eq!(analyzer.item_type_name(&ItemType::Function), "Function");
    assert_eq!(analyzer.item_type_name(&ItemType::Struct), "Struct");
    assert_eq!(analyzer.item_type_name(&ItemType::Interface), "Interface");
    assert_eq!(analyzer.item_type_name(&ItemType::Method), "Method");
    assert_eq!(analyzer.item_type_name(&ItemType::Field), "Field");
}

// Helper functions for creating test data

fn create_test_extraction_result() -> cursed::documentation::ExtractionResult {
    cursed::documentation::ExtractionResult {
        items: Vec::new(),
        symbols: std::collections::HashMap::new(),
        stats: cursed::documentation::extractor::ExtractionStats {
            total_functions: 0,
            documented_functions: 0,
            total_types: 0,
            documented_types: 0,
            total_fields: 0,
            documented_fields: 0,
        },
    }
}

fn create_test_extraction_result_with_undocumented_items() -> cursed::documentation::ExtractionResult {
    let undocumented_function = DocumentationItem {
        name: "undocumented_function".to_string(),
        item_type: ItemType::Function,
        documentation: None, // No documentation
        location: SourceLocation::new(1, 0),
        parameters: Vec::new(),
        return_type: None,
        fields: Vec::new(),
        type_parameters: Vec::new(),
        references: Vec::new(),
    };
    
    let documented_function = DocumentationItem {
        name: "documented_function".to_string(),
        item_type: ItemType::Function,
        documentation: Some("This function is documented".to_string()),
        location: SourceLocation::new(2, 0),
        parameters: Vec::new(),
        return_type: None,
        fields: Vec::new(),
        type_parameters: Vec::new(),
        references: Vec::new(),
    };
    
    let items = vec![undocumented_function, documented_function];
    let mut symbols = std::collections::HashMap::new();
    for item in &items {
        symbols.insert(item.name.clone(), item.clone());
    }
    
    cursed::documentation::ExtractionResult {
        items,
        symbols,
        stats: cursed::documentation::extractor::ExtractionStats {
            total_functions: 2,
            documented_functions: 1,
            total_types: 0,
            documented_types: 0,
            total_fields: 0,
            documented_fields: 0,
        },
    }
}

#[test]
fn test_comprehensive_documentation_analysis() {
    init_tracing();
    
    let mut extractor = DocumentationExtractor::new();
    let analyzer = DocumentationAnalyzer::new();
    
    // Create a test program with various items
    let program = create_test_program();
    
    // Extract documentation (would be empty for this test)
    let extraction_result = extractor.extract_documentation(&program);
    assert!(extraction_result.is_ok());
    
    let extraction_result = extraction_result.unwrap();
    
    // Analyze the extraction result
    let analysis_result = analyzer.analyze(&extraction_result);
    assert!(analysis_result.is_ok());
    
    let analysis = analysis_result.unwrap();
    
    // Verify analysis structure
    assert!(analysis.coverage.overall_coverage >= 0.0);
    assert!(analysis.coverage.overall_coverage <= 1.0);
    assert_eq!(analysis.link_validation.total_links, 0); // No links in empty program
    assert_eq!(analysis.example_validation.total_examples, 0); // No examples
}

#[test]
fn test_validation_issue_creation() {
    init_tracing();
    
    let issue = ValidationIssue {
        issue_type: IssueType::MissingDocumentation,
        location: SourceLocation::new(10, 5),
        message: "Function 'test' is missing documentation".to_string(),
        severity: Severity::Warning,
        suggestion: Some("Add documentation comment above test".to_string()),
    };
    
    assert_eq!(issue.issue_type, IssueType::MissingDocumentation);
    assert_eq!(issue.location.line, 10);
    assert_eq!(issue.location.column, 5);
    assert_eq!(issue.severity, Severity::Warning);
    assert!(issue.suggestion.is_some());
}

#[test]
fn test_coverage_thresholds() {
    init_tracing();
    
    let default_thresholds = CoverageThresholds::default();
    assert_eq!(default_thresholds.function_coverage, 0.8);
    assert_eq!(default_thresholds.type_coverage, 0.9);
    assert_eq!(default_thresholds.field_coverage, 0.7);
    assert_eq!(default_thresholds.overall_coverage, 0.8);
}

#[test]
fn test_analyzer_configuration() {
    init_tracing();
    
    let mut analyzer = DocumentationAnalyzer::new();
    
    // Test example validation configuration
    analyzer.set_example_validation(false);
    assert!(!analyzer.validate_examples);
    
    analyzer.set_example_validation(true);
    assert!(analyzer.validate_examples);
}
