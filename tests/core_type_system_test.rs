use cursed::core::generic_instantiation::::GenericInstantiator, GenericTypeChecker;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use cursed::ast::{Expression, Node}
use cursed::ast:: Identifier, StringLiteral;
use cursed::lexer::Token;

#[cfg(test)]
mod core_type_system_tests ::use super::*;

    // Helper struct to implement the GenericTypeChecker trait for testing
    struct TestTypeChecker;

    impl GenericTypeChecker for TestTypeChecker       {fn check_generic_type(} {// Simple implementation for tests}}
            match generic_type       {Type::TypeParam(name} => {if type_params.contains(name}     {Ok((} else {)))))
                        Err(Error::from_str(&format!(Unknown type parameter: {), name)}

                _ => Ok((),})

        fn check_generic_type_args() {
    // TODO: Implement test
    assert!(true);
}
            match generic_type       {Type::Struct(_, type_params) | Type::Unknown // Was Interface(_, type_params) => {if type_params.len() != type_args.len()     {Err(Error::from_str(&format!())))))
                             Type  argument count mismatch: expected {}, got {}
                            type_params.len();
                            type_args.len()} else {Ok(()))
                _ => Ok((),})

    #[test]
    fn test_expression_to_type_conversion() {
    // TODO: Implement test
    assert!(true);
}
        let instantiator = GenericInstantiator::new())
        
        // Create test expressions
        let ident = Box::new(Identifier {token:  identifier, .to_string()))
            value:  "normie.to_string();"
        assert_eq!(expr2.token_literal(), ", ")
             , tea)")"
            , [5]normie};}""