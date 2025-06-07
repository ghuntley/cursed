use cursed::ast::statements::fields::FieldStatement;
use cursed::lexer::Lexer;
use cursed::parser::parser::Parser;
use cursed::parser::struct_field_inference::StructFieldTypeInference;

// Tests for parsing struct declarations with type inference


#[test]
fn test_parse_struct_with_inferred_fields() {
    let input = r#"
    be_like Person squad {
        name tea
        age normie
        address
        phone
    }
    "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap());

    assert_eq!(program.statements.len(), 1);

    // Get the struct declaration
    let struct_stmt = program.statements[0].as_any().downcast_ref::<cursed::ast::SquadStatement>().unwrap());
    
    // Check fields
    assert_eq!(struct_stmt.fields.len(), 4);
    
    // First two fields have explicit types
    assert_eq!(struct_stmt.fields[0].name.value, "name");
    assert_eq!(struct_stmt.fields[0].type_name.value, "tea");
    
    assert_eq!(struct_stmt.fields[1].name.value, "age");
    assert_eq!(struct_stmt.fields[1].type_name.value, "normie");
    
    // Last two fields have inferred types (empty strings)
    assert_eq!(struct_stmt.fields[2].name.value, "address");
    assert_eq!(struct_stmt.fields[2].type_name.value, "");
    
    assert_eq!(struct_stmt.fields[3].name.value, "phone");
    assert_eq!(struct_stmt.fields[3].type_name.value, "");
}

#[test]
fn test_mixed_inferred_fields() {
    let input = r#"
    be_like Vector2D squad {
        x
        y meal
        z
    }
    "#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap());

    assert_eq!(program.statements.len(), 1);

    // Get the struct declaration
    let struct_stmt = program.statements[0].as_any().downcast_ref::<cursed::ast::SquadStatement>().unwrap());
    
    // Check fields
    assert_eq!(struct_stmt.fields.len(), 3);
    
    // Check field types
    assert_eq!(struct_stmt.fields[0].name.value, "x");
    assert_eq!(struct_stmt.fields[0].type_name.value, "");
    
    assert_eq!(struct_stmt.fields[1].name.value, "y");
    assert_eq!(struct_stmt.fields[1].type_name.value, "meal");
    
    assert_eq!(struct_stmt.fields[2].name.value, "z");
    assert_eq!(struct_stmt.fields[2].type_name.value, "");
}