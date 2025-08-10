#!/usr/bin/env rust

//! Test the CursedLinter with code that should generate issues

use cursed::{CursedLinter, LinterConfig};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing CursedLinter with problematic code...");

    // Create a linter with strict configuration
    let mut linter = CursedLinter::strict_linter();
    
    // Read the test file
    let source_code = fs::read_to_string("test_linter_issues.csd")?;

    // Run linting analysis
    match linter.lint_source(&source_code) {
        Ok(result) => {
            println!("✅ Linter analysis completed!");
            println!("Total issues found: {}", result.issues.len());
            println!("Files analyzed: {}", result.summary.files_analyzed);
            println!("Lines analyzed: {}", result.summary.lines_analyzed);
            println!("Analysis time: {}ms", result.metrics.analysis_time_ms);
            
            if result.issues.is_empty() {
                println!("No issues found.");
            } else {
                println!("\nIssues found:");
                for (i, issue) in result.issues.iter().enumerate() {
                    println!("{}. {} [{}] {} (line {})", 
                        i + 1,
                        issue.severity.as_str().to_uppercase(),
                        issue.rule_id,
                        issue.message,
                        issue.line
                    );
                    if let Some(suggestion) = &issue.suggestion {
                        println!("   Suggestion: {}", suggestion);
                    }
                }
            }
            
            // Summary by severity
            println!("\nSummary by severity:");
            for (severity, count) in &result.summary.by_severity {
                println!("  {}: {}", severity.as_str(), count);
            }
        },
        Err(e) => {
            println!("❌ Linter analysis failed: {}", e);
            return Err(e.into());
        }
    }

    println!("\n✅ CursedLinter testing completed!");
    Ok(())
}
