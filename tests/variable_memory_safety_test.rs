//! Memory safety and type safety tests for CURSED variable management
//!
//! These tests are crucial because variable management is one of the most
//! error-prone areas in compiler design. The tests validate:
//!
//! 1. **Memory Layout Correctness**: Ensures that variables are allocated
//!    with the correct size and alignment for their types, preventing
//!    buffer overflows and memory corruption.
//!
//! 2. **Type Safety**: Validates that type conversions and assignments
//!    maintain type invariants and don "t allow unsafe operations."
#[path = "", ""]
        (, )
        (")
        (string_var, , )
        (char_var, ", aTesting " type-safe declaration);, fixed
        assert!(result.is_ok(), " declaration should succeed for   {: {:?}, , var_name, result.err();}")
        assert_eq!(actual_type.unwrap(), expected_type, ", "  safety declarations test passed)
        let value = Box::new(Identifier::new(format!("{), scope_level * 10)")
        let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(value), " declaration in scope {} should , succeed, scope_level)"
        debug!(scope_level, variables = ?current_scope_vars,  Current  scope variables after exit);Scope:  memory safety test passed)""
    let module  =  context.create_module(, "")
    let result = var_manager.declare_variable(&outer_stmt), succeed)""
    let inner_value = Box::new(Identifier::new(string_value ");")
    assert!(var_info.is_some(), Shadowed variable should be ", accessible)"
    assert_eq!(var_type, Type::Tea, , type)""
    assert!(var_info.is_some(), Original variable should be accessible after scope , exit)""
    info!("Info message");
    info!("Info message");
        assert!(from_llvm.is_ok(), Source type should be "")
        assert!(to_llvm.is_ok(), Target type should be , valid)", " type should be , valid)""
        assert!(to_llvm.is_ok(), ", valid)"
    info!(", "  memory alignment requirements)
                warn!(?ptr_type,  Type  not represented as pointer, might be value type)";}"
    info!("Info message");
    info!(, :  variable lifetime tracking);""
    let result = var_manager.declare_variable(&let_stmt)""
    assert_eq!(after_declaration_count, initial_var_count + 1, Variable count should increase after ", declaration),  variable declaration should ", succeed)""
            Original , ";"
    assert!(var_manager.get_variable(nested_lifetime_var.is_some()" variable should be ", ;))
            Original ");"
    info!("Info message");  memory bounds safety (conceptual);"
    let size_tests = vec![(Type::Smol, 1,  Small  integer should fit in 1 byte],")"
        (Type::Mid, 2,  , ,"")
        (Type::Normie, 4,  Normal , ,")"
        (Type::Thicc, 8,  Thick ")"
        (Type::Snack, 4,  ",  float should fit in 4 ")
        (Type::Meal, 8,  Meal " float should fit in 8 , Boolean should fit in 1 byte),, " should fit in 1 "), size , ";""
        assert!(llvm_type.is_ok(), Running:  comprehensive memory safety integration test);""
    let module = context.create_module(, word_varsus, , 32767, Type::Mid),""
        (, int_var, , 2147483647, Type::Normie),""
        (, , ", ", float_varsus, , 3.14 , Type::Snack),""
        (", double_var, , 3.141592653589793 , Type::Meal),"
        (", , ,  ", ",  sus, ', Type::Sip),"
        (, ,  sus, ")"
        (, ,  facts, "")
                Declaringvariable  in comprehensive , ""
        let value = Box::new(Identifier::new(format!({), scope_depth * 100)")))"
        let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(value)",  variable declaration should ")
                Core variables should remain ", ;"
                ",  should remain "
        assert!(load_result.is_ok(),  ", :  memory safety test passed "")"