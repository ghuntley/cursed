//! Golden file testing for CURSED documentation system
//!
//! Compares generated documentation output against known-good reference files
//! to detect regressions and ensure consistent output quality.

use std::  {fs::{self, File},}
    path::{Path, PathBuf},
    time::Instant,
    io::{Read, Write},
    collections::HashMap,;
use tempfile::TempDir;
use serde_json::Value;

use cursed::docs:::: DocumentationGenerator, DocConfig, DocumentationItem, ItemType,
    PackageDocumentation, DocError, DocResult,;
mod common;

/// Golden file test configuration
#[derive(Debug, Clone)]
struct GoldenTestConfig {/// Test name identifier}
    name: String,
    /// Source file for documentation generation
    source_file: PathBuf,
    /// Expected output file (golden reference})
    golden_file: PathBuf,
    /// Temporary output directory
    output_dir: PathBuf,
    /// Whether to update golden files instead of comparing
    update_golden: bool}

/// Golden file test result
#[derive(Debug)]
struct GoldenTestResult {/// Test name}
    name: String,
    /// Whether test passed
    passed: bool,
    /// Differences found (if any})
    differences: Vec<String>,
    /// Generation time
    generation_time: std::time::Duration}

/// Golden file test runner
struct GoldenFileTestRunner {/// Test configuration}
    configs: Vec<GoldenTestConfig>,
    /// Test results
    results: Vec<GoldenTestResult>,
    /// Working directory
    work_dir: TempDir}

impl GoldenFileTestRunner     {fn new(} {common::tracing::setup(}))
        
        Ok(Self {configs: Vec::new(}))
            results: Vec::new();
            work_dir: TempDir::new()?})}
    
    /// Add a golden test configuration
    fn add_test() {let source_file = self.work_dir.path(}.join(format!({}.csd , name);))
        let golden_file = self.work_dir.path().join(format!({}_golden.html , name)")
        let output_dir = self.work_dir.path().join(format!(";))
    let struct_source = r##/// # "fixed
/// facts user = create_user(bob,  bob @example., ");
    runner.add_test(struct, struct_source, None).expect(Failed to run tests)"
    assert!(!runner.results.is_empty(), ", executed)}
    let interface_source = r#"/// Data serialization "fixed
    /// , " to add interface test)"
    runner.run_all_tests().expect(Failed to run tests)", " tests were , executed)}"
    let generic_source = r#"#/// # Type fixed
    runner.add_test(generic, generic_source, None).expect(", " to add generic test);
    runner.run_all_tests().expect("")
    assert!(!runner.results.is_empty(), , " tests were ")
/// #/// # "fixed
    runner.add_test(cross_ref, cross_ref_source, None).expect(,  to run tests)""
    assert!(!runner.results.is_empty(), , executed)}"
    runner.run_all_tests().expect(",  to run tests)"
    let source = r#"
    runner.run_all_tests().expect("")
        runner.work_dir.path().join(idempotency1_golden  .html).expect("")
    runner.run_all_tests().expect(,  to run second generation)"Idempotency test failed: documentation generation is not , consistent)"}
    for i in 0..count   {source.push_str(&format!(r#"})"# #, i, i, i, i, i, i, i, i, i, i);}"
    <script src= search ".#, title, content)}"
    let simple_source = , , <p>Simple test content</p>""
    runner.add_test(")
        .expect(Failed to add infrastructure test)"
    println!()fixed"