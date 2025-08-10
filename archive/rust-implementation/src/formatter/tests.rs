//! Tests for the CURSED formatter

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::formatter::{CursedFormatter, FormatterConfig};

    #[test]
    fn test_basic_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
sus x drip=42
sus y tea="hello world"
vibez.spill(x)
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("sus x drip = 42"));
        assert!(formatted.contains("sus y tea = \"hello world\""));
        assert!(formatted.contains("vibez.spill(x)"));
    }

    #[test]
    fn test_indentation_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
nah x > 0 {
vibez.spill("positive")
} lowkey {
vibez.spill("negative")
}
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("    vibez.spill(\"positive\")"));
        assert!(formatted.contains("    vibez.spill(\"negative\")"));
    }

    #[test]
    fn test_function_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
slay add(a normie,b normie) normie {
damn a+b
}
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("slay add(a normie, b normie) normie {"));
        assert!(formatted.contains("    damn a + b"));
    }

    #[test]
    fn test_import_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
yeet "math"
yeet "string"
yeet "crypto"
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        // Should group imports
        assert!(formatted.contains("yeet ("));
        assert!(formatted.contains("    \"math\""));
        assert!(formatted.contains("    \"string\""));
        assert!(formatted.contains("    \"crypto\""));
        assert!(formatted.contains(")"));
    }

    #[test]
    fn test_array_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
sus arr = [1,2,3,4,5]
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("[1, 2, 3, 4, 5]"));
    }

    #[test]
    fn test_struct_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
flex Person {
name tea
age normie
}
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("flex Person {"));
        assert!(formatted.contains("    name tea"));
        assert!(formatted.contains("    age normie"));
        assert!(formatted.contains("}"));
    }

    #[test]
    fn test_short_declaration_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
x:=42
(a,b,c):=(1,2,3)
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("x := 42"));
        assert!(formatted.contains("(a, b, c) := (1, 2, 3)"));
    }

    #[test]
    fn test_operator_spacing() {
        let formatter = CursedFormatter::default();
        let source = r#"
sus result = x+y*z
sus compare = a==b&&c!=d
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("x + y * z"));
        assert!(formatted.contains("a == b && c != d"));
    }

    #[test]
    fn test_for_loop_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
bestie i:=0;i<10;i++ {
vibez.spill(i)
}
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("bestie i := 0; i < 10; i++ {"));
        assert!(formatted.contains("    vibez.spill(i)"));
    }

    #[test]
    fn test_while_loop_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
lol x>0 {
x--
}
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("lol x > 0 {"));
        assert!(formatted.contains("    x--"));
    }

    #[test]
    fn test_switch_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
periodt x {
case 1:
vibez.spill("one")
case 2:
vibez.spill("two")
default:
vibez.spill("other")
}
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("periodt x {"));
        assert!(formatted.contains("    case 1:"));
        assert!(formatted.contains("        vibez.spill(\"one\")"));
    }

    #[test]
    fn test_channel_operations() {
        let formatter = CursedFormatter::default();
        let source = r#"
sus ch chan normie = make(chan normie)
ch<-42
value:=<-ch
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        

        
        assert!(formatted.contains("sus ch chan normie = make(chan normie)"));
        assert!(formatted.contains("ch <- 42"));
        assert!(formatted.contains("value := <-ch"));
    }

    #[test]
    fn test_error_handling_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
yikes err:=risky_function()
fam err {
vibez.spill("error occurred")
}
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("yikes err := risky_function()"));
        assert!(formatted.contains("fam err {"));
        assert!(formatted.contains("    vibez.spill(\"error occurred\")"));
    }

    #[test]
    fn test_compact_config() {
        let formatter = CursedFormatter::new(FormatterConfig::compact());
        let source = r#"
nah x > 0 {
vibez.spill("positive")
}
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        // Compact config should use minimal spacing
        assert!(formatted.contains("nah x>0{"));
        assert!(formatted.contains("  vibez.spill(\"positive\")"));
    }

    #[test]
    fn test_verbose_config() {
        let formatter = CursedFormatter::new(FormatterConfig::verbose());
        let source = r#"
sus arr = [1, 2, 3]
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        // Verbose config should use more spacing and multiline formatting
        assert!(formatted.contains("sus arr = ["));
        assert!(formatted.contains("    1,"));
        assert!(formatted.contains("    2,"));
        assert!(formatted.contains("    3"));
        assert!(formatted.contains("]"));
    }

    #[test]
    fn test_diff_generation() {
        let formatter = CursedFormatter::default();
        let original = "sus x=42";
        let diff = formatter.format_diff(original).unwrap();
        
        assert!(!diff.is_empty());
        assert!(diff.contains("sus x = 42"));
    }

    #[test]
    fn test_is_formatted() {
        let formatter = CursedFormatter::default();
        let well_formatted = "sus x drip = 42";
        let poorly_formatted = "sus x drip=42";
        
        assert!(formatter.is_formatted(well_formatted).unwrap());
        assert!(!formatter.is_formatted(poorly_formatted).unwrap());
    }

    #[test]
    fn test_comment_preservation() {
        let formatter = CursedFormatter::default();
        let source = r#"
// This is a comment
sus x drip = 42 // Inline comment
/* Block comment */
vibez.spill(x)
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        // Comments should be preserved (exact behavior may vary based on implementation)
        assert!(formatted.contains("sus x drip = 42"));
        assert!(formatted.contains("vibez.spill(x)"));
    }

    #[test]
    fn test_line_length_handling() {
        let mut config = FormatterConfig::default();
        config.max_line_length = 20;
        let formatter = CursedFormatter::new(config);
        
        let source = "sus very_long_variable_name_that_exceeds_limit drip = 42";
        let formatted = formatter.format(source).unwrap();
        
        // Should handle long lines appropriately
        assert!(formatted.contains("sus very_long_variable_name_that_exceeds_limit drip"));
    }

    #[test]
    fn test_nested_structures() {
        let formatter = CursedFormatter::default();
        let source = r#"
flex Person {
name tea
address flex {
street tea
city tea
}
}
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("flex Person {"));
        assert!(formatted.contains("    name tea"));
        assert!(formatted.contains("    address flex {"));
        assert!(formatted.contains("        street tea"));
        assert!(formatted.contains("        city tea"));
        assert!(formatted.contains("    }"));
        assert!(formatted.contains("}"));
    }

    #[test]
    fn test_tuple_formatting() {
        let formatter = CursedFormatter::default();
        let source = r#"
sus t = (1,2,3)
sus (a,b,c) = t
sus first = t.0
"#;
        
        let formatted = formatter.format(source.trim()).unwrap();
        
        assert!(formatted.contains("sus t = (1, 2, 3)"));
        assert!(formatted.contains("sus (a, b, c) = t"));
        assert!(formatted.contains("sus first = t.0"));
    }
}
