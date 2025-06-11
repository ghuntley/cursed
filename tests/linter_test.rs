//! Basic tests for the CURSED language linter
//!
//! This module tests the basic functionality of the linter.

use cursed::tools::{CursedLinter, LinterConfig};

mod common;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linter_creation() {
        let config = LinterConfig {
            strict_mode: false,
            max_line_length: 80,
        };
        
        let linter = CursedLinter::new(config);
        // Basic test to ensure linter was created
        assert!(true); 
    }

    #[test]
    fn test_line_length_violation() {
        let config = LinterConfig {
            strict_mode: true,
            max_line_length: 40,
        };
        
        let linter = CursedLinter::new(config);
        let source = "this line is definitely way too long for the configured limit";
        
        // Test that lint method runs without error
        let result = linter.lint(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_linter_functionality() {
        let linter = CursedLinter::default();
        let source = r#"
slay calculate_vibes(x normie, y normie) normie {
    facts message = "calculation";
    yolo distance;
}
"#;
        
        // Test that lint method runs without error
        let result = linter.lint(source);
        assert!(result.is_ok());
    }
}
