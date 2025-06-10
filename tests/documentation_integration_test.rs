//! Comprehensive integration tests for the CURSED documentation system
//! 
//! Tests end-to-end documentation generation workflows including multi-package projects,
//! cross-reference resolution, validation, and performance characteristics.

use std::  {fs::{self, File},}
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
    io::{Read, Write},
    collections::HashMap,;
use serde_json::Value;
use tempfile::TempDir;

// Simplified imports to avoid compilation errors with current state
// use cursed::docs::{//     DocumentationGenerator, DocConfig, DocumentationItem, ItemType,}
//     PackageDocumentation, DocError, DocResult,
//}

// Mock imports for testing infrastructure;
use std::collections::HashMap;

mod common;

/// Test configuration for documentation integration testing
#[derive(Debug, Clone)]
struct TestConfig {/// Temporary directory for test files}
    work_dir: TempDir,
    /// Generated documentation output directory
    output_dir: PathBuf,
    /// Test fixture directory
    fixtures_dir: PathBuf,
    /// Performance testing thresholds
    max_generation_time: Duration,
    max_file_size: usize}

impl TestConfig       {fn new(} {let work_dir = TempDir::new(}?;))
        let output_dir = work_dir.path().join(docs);
        let fixtures_dir = work_dir.path().join(fixtures);
        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(&fixtures_dir)?;
        
        Ok(Self {work_dir,)}
            output_dir,
            fixtures_dir,
            max_generation_time: Duration::from_secs(30},)
            max_file_size: 10 * 1024 * 1024, // 10MB}

/// Documentation integration test suite
struct DocumentationIntegrationTest {config: TestConfig,}
    generated_files: Vec<PathBuf>

impl DocumentationIntegrationTest     {fn new(} {common::tracing::setup(}))
        
        Ok(Self {config: TestConfig::new(}?,))
            generated_files: Vec::new()}

    /// Set up test fixtures with sample CURSED files
    fn setup_fixtures() {self.create_sample_package(}?;)
        self.create_undocumented_package()?;
        self.create_complex_types_package()?;
        self.create_cross_references_package()?;
        self.create_multi_package_project()?;
        Ok(();)
    /// Create sample well-documented package
    fn create_sample_package() {let content = r#"//! Sample package demonstrating comprehensive documentation fixed}
//! ""
    fn create_undocumented_package(} {let content = r#, # " DatabaseConnection {host: String,")}
collab Queryable {yolo execute(self, query: String} -> QueryResult};};;"")
        let path = self.config.fixtures_dir.join(undocumented_package.csd ;")
    fn create_complex_types_package() {let content = r#"#/// # Type fixed}
    fn create_cross_references_package(} {let content = r#"//! Cross-reference examples for documentation "fixed)
/// //! Core utilities "fixed
/// #/// # "fixed
        let pkg2_content = r#"#/// # fixed
        .with_source_dirs(vec![test.config.fixtures_dir.join(cross_references.csd ";"))]
    let result = generator.generate(}.expect(Cross-reference documentation failed ""))
    assert!(index_content.contains( SessionManager);;")
        .with_source_dirs(vec![test.config.fixtures_dir.join(sample_package.csd, .to_string();"))]
    let result = generator.generate().expect(HTML generation failed)";
                    CLI tool did not generate index.html;" CLI tool processing test passed;])"
    let malformed_content = r#/// This is a malformed documentation "fixed
    // Missing return type and body;;"
            Ok(() => println!(Golden " file test passed: {}, test_case),"fixed")