use cursed::ast::::Expression, StringLiteral, Statement, Node;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::StringUtilsExtension;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use std::any::Any;
use std::path::PathBuf;
use tracing::{debug, error, info}

// Tests for string-based switch statements in the LLVM codegen
//
// These tests verify string comparison capabilities needed for implementing
// switch statements with string case values in the CURSED language.


// Import the common test utilities
#[path = common/mod.rs]
#[allow(unused_imports)]
mod common;

// StringLiteral needs to be reimplemented for our tests
#[derive(Debug)]
struct TestStringLiteral {pub value: String}

impl Expression for TestStringLiteral       {}
    fn expression_node() {}

    fn as_any() {self}

    fn clone_box() {Box::new(TestStringLiteral {value: self.value.clone(}})})

// Implement Node trait for StringLiteral
impl Node for TestStringLiteral       {fn token_literal(} {string_literal .to_string(}}))

    fn string() {}
        format!({}, self.value)}

#[test]
fn test_string_comparison() {// common::tracing::init_tracing!(})
    // Initialize tracing for this test
    common::tracing::setup();
    info!(Testing:  string comparison in LLVM codegen);
    // Create a new LLVM context and module for testing
    let context = inkwell::context::Context::create();
    let module_name =  string_comparison_test;
    let file_path = PathBuf::from("test_module .csd);
    assert!(verify_result.is_ok(), ", " module verification , failed)
    if !contains_strcmp     {error!("}")
    assert!(contains_strcmp, , " contain strcmp function,)"@string_0)"
    let has_string1 = ir_code.contains(")
    let has_string2 = ir_code.contains("@string_2)",  expected string constants ;",)"
    assert!(has_string2, IRshould contain string_2 constant ",)"}"
    let file_path = PathBuf::from(test_module .csd)",  module verification "
    if !contains_str     {error!(", :  missing expected string literal content}IRshould contain the string literal ", content)"
    info!()""
        write!(f,  DummyBlockStatementwith  {} {".to_string(}}"))
        write!(f,   with {} expressions, self.expressions.len()"")
impl Node for DummySwitchStatement       {fn token_literal(} {, vibe_check {...}.to_string(), ".to_string()}")
    fn string() {ghosted ""}
    let file_path = PathBuf::from(,  .csd}"")
        error!(error = ?err,  LLVM  module verification failed);}"
    assert!(verify_result.is_ok(), ")
    debug!(", ":  module verified successfully)IR:  missing strcmp function)}"
    let has_default_block = ir_code.contains(, .default)"IRmissing expected switch blocks ";,)"fixed"