//! Minimal configuration tests for the CURSED code formatter

use cursed::tools::{FormatterConfig, BraceStyle};

#[path = "common/mod.rs"]
mod common;

/// Test basic formatting configuration options that actually exist
mod config_option_tests {
    use super::*;

    #[test]
    fn test_default_configuration() {
    // init_tracing!();
        common::tracing::setup();
        
        let config = FormatterConfig::default();
        
        assert_eq!(config.indent_size, 4);
        assert_eq!(config.line_width, 100);
        assert_eq!(config.brace_style, BraceStyle::SameLine);
    }

    #[test]
    fn test_custom_configuration() {
    // init_tracing!();
        common::tracing::setup();
        
        let config = FormatterConfig {
            indent_size: 2,
            line_width: 80,
            brace_style: BraceStyle::NextLine,
        };
        
        assert_eq!(config.indent_size, 2);
        assert_eq!(config.line_width, 80);
        assert_eq!(config.brace_style, BraceStyle::NextLine);
    }

    #[test]
    fn test_brace_style_variants() {
    // init_tracing!();
        common::tracing::setup();
        
        let same_line = BraceStyle::SameLine;
        let next_line = BraceStyle::NextLine;
        let next_line_unindented = BraceStyle::NextLineUnindented;
        
        assert_eq!(same_line, BraceStyle::SameLine);
        assert_eq!(next_line, BraceStyle::NextLine);
        assert_eq!(next_line_unindented, BraceStyle::NextLineUnindented);
        
        assert!(same_line != next_line);
        assert!(next_line != next_line_unindented);
    }
}
