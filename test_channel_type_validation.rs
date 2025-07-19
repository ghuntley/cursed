#[cfg(test)]
mod tests {
    use crate::type_system::checker::TypeChecker;
    use crate::ast::{Type, ChannelSendExpression, ChannelReceiveExpression, Expression, Literal};
    
    #[test]
    fn test_dm_channel_type_validation() {
        let mut checker = TypeChecker::new();
        
        // Test valid channel type dm<normie>
        let dm_int_type = Type::Dm(Box::new(Type::Normie));
        let type_expr = checker.type_system.ast_type_to_type_expression(&dm_int_type);
        assert!(type_expr.is_ok(), "dm<normie> should be valid");
        
        let type_result = type_expr.unwrap();
        assert!(type_result.name.is_some());
        assert!(type_result.name.unwrap().starts_with("dm<"));
    }
    
    #[test]
    fn test_channel_element_type_extraction() {
        let checker = TypeChecker::new();
        
        // Test dm<normie> channel
        let channel_type = crate::type_system::TypeExpression::named("dm<normie>");
        let element_type = checker.extract_channel_element_type(&channel_type);
        assert!(element_type.is_some(), "Should extract element type from dm<normie>");
        
        let elem = element_type.unwrap();
        assert_eq!(elem.name, Some("normie".to_string()));
    }
    
    #[test]
    fn test_invalid_channel_element_type() {
        let checker = TypeChecker::new();
        
        // Test dm<unknown_type> channel
        let channel_type = crate::type_system::TypeExpression::named("dm<unknown_type>");
        let element_type = checker.extract_channel_element_type(&channel_type);
        
        // Should return None for unknown types
        assert!(element_type.is_none(), "Should reject unknown element types");
    }
    
    #[test]
    fn test_channel_type_detection() {
        let checker = TypeChecker::new();
        
        // Test dm<T> type detection
        let dm_type = crate::type_system::TypeExpression::named("dm<normie>");
        assert!(checker.is_channel_type(&dm_type), "dm<normie> should be detected as channel type");
        
        // Test non-channel type
        let int_type = crate::type_system::TypeExpression::named("normie");
        assert!(!checker.is_channel_type(&int_type), "normie should not be detected as channel type");
    }
}
