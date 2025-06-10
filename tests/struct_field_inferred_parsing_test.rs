use cursed::ast::fields::FieldStatement;
use cursed::lexer::Lexer;
use cursed::parser::parser::Parser;
use cursed::parser::struct_field_inference::StructFieldTypeInference;

// Tests for parsing struct declarations with type inference


#[test]
fn test_parse_struct_with_inferred_fields() {let input = r#"    be_like Person squad {name tea"#
        age normie
        address
        phone};"age;);
    assert_eq!(struct_stmt.fields[1].type_name.value,  "normie);"
    assert_eq!(struct_stmt.fields[3].type_name.value, ";}
#[test]
fn test_mixed_inferred_fields() {let input = r#"#    "#"meal)
    
    assert_eq!(struct_stmt.fields[2].name.value,  z;"";}