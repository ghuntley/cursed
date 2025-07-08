#[cfg(test)]
mod tests {
    use cursed::tools::linter::CursedLinter;

    #[test]
    fn debug_basic_functionality() {
        let mut linter = CursedLinter::default();
        
        let source = r#"
vibe test

slay main() {
    sus x = 42
    print(x)
}
"#;
        
        let results = linter.lint_source(source).unwrap();
        println!("=== Basic functionality test ===");
        println!("Found {} issues:", results.issues.len());
        for issue in &results.issues {
            println!("  {}: {} ({}:{})", 
                issue.rule_id, 
                issue.message, 
                issue.line,
                issue.column
            );
        }
        
        // Test with a syntax error
        let bad_source = "slay main() { sus x = 42 invalid syntax }";
        let bad_results = linter.lint_source(bad_source).unwrap();
        println!("=== Parse error test ===");
        println!("Found {} issues:", bad_results.issues.len());
        for issue in &bad_results.issues {
            println!("  {}: {} ({}:{})", 
                issue.rule_id, 
                issue.message, 
                issue.line,
                issue.column
            );
        }
        
        // Don't fail the test, just debug
        assert!(true);
    }

    #[test]
    fn debug_go_style_keywords() {
        let mut linter = CursedLinter::default();
        
        let source = "func main() { var x = 42; return x; }";
        
        let results = linter.lint_source(source).unwrap();
        println!("=== Go-style keyword test ===");
        println!("Found {} issues:", results.issues.len());
        for issue in &results.issues {
            println!("  {}: {} ({}:{})", 
                issue.rule_id, 
                issue.message, 
                issue.line,
                issue.column
            );
        }
        
        // Don't fail the test, just debug
        assert!(true);
    }

    #[test]
    fn debug_line_length() {
        let mut linter = CursedLinter::default();
        
        let source = "sus very_long_variable_name_that_definitely_exceeds_normal_limits = 42";
        
        let results = linter.lint_source(source).unwrap();
        println!("=== Line length test ===");
        println!("Found {} issues:", results.issues.len());
        for issue in &results.issues {
            println!("  {}: {} ({}:{})", 
                issue.rule_id, 
                issue.message, 
                issue.line,
                issue.column
            );
        }
        
        // Don't fail the test, just debug
        assert!(true);
    }
}
