//! Comprehensive tests for mutable reference handling in the type system

use super::*;
use crate::ast::*;
use std::collections::HashMap;

#[cfg(test)]
mod mutable_reference_tests {
    use super::*;

    #[test]
    fn test_mutable_borrow_basic() {
        let mut checker = TypeChecker::new();
        
        // Test basic mutable borrow
        let result = checker.check_mutable_borrow("x");
        assert!(result.is_ok());
        
        // Test double mutable borrow fails
        let second_result = checker.check_mutable_borrow("x");
        assert!(second_result.is_err());
        
        if let Err(error) = second_result {
            assert_eq!(error.error_type, TypeErrorKind::MutableBorrowError);
            assert!(error.message.contains("already borrowed as mutable"));
        }
    }

    #[test]
    fn test_immutable_borrow_basic() {
        let mut checker = TypeChecker::new();
        
        // Test basic immutable borrow
        let result = checker.check_immutable_borrow("x");
        assert!(result.is_ok());
        
        // Test multiple immutable borrows succeed
        let second_result = checker.check_immutable_borrow("x");
        assert!(second_result.is_ok());
        
        // Test mutable borrow after immutable borrow fails
        let mutable_result = checker.check_mutable_borrow("x");
        assert!(mutable_result.is_err());
        
        if let Err(error) = mutable_result {
            assert_eq!(error.error_type, TypeErrorKind::BorrowConflictError);
            assert!(error.message.contains("already borrowed as immutable"));
        }
    }

    #[test]
    fn test_borrow_conflict_mutable_then_immutable() {
        let mut checker = TypeChecker::new();
        
        // Mutable borrow first
        let mutable_result = checker.check_mutable_borrow("x");
        assert!(mutable_result.is_ok());
        
        // Immutable borrow after mutable borrow fails
        let immutable_result = checker.check_immutable_borrow("x");
        assert!(immutable_result.is_err());
        
        if let Err(error) = immutable_result {
            assert_eq!(error.error_type, TypeErrorKind::BorrowConflictError);
            assert!(error.message.contains("already borrowed as mutable"));
        }
    }

    #[test]
    fn test_dereference_valid_reference() {
        let checker = TypeChecker::new();
        
        // Test dereference of mutable reference
        let int_type = TypeExpression::named("normie");
        let mut_ref_type = TypeExpression::mutable_reference(int_type.clone());
        
        let result = checker.check_dereference(&mut_ref_type);
        assert!(result.is_ok());
        
        if let Ok(pointee_type) = result {
            assert_eq!(pointee_type.name, Some("normie".to_string()));
        }
        
        // Test dereference of immutable reference
        let immut_ref_type = TypeExpression::immutable_reference(int_type.clone());
        let result2 = checker.check_dereference(&immut_ref_type);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_dereference_invalid_type() {
        let checker = TypeChecker::new();
        
        // Test dereference of non-reference type
        let int_type = TypeExpression::named("normie");
        let result = checker.check_dereference(&int_type);
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.error_type, TypeErrorKind::InvalidDereferenceError);
            assert!(error.message.contains("Cannot dereference non-reference type"));
        }
    }

    #[test]
    fn test_mutable_assignment_valid() {
        let checker = TypeChecker::new();
        
        let int_type = TypeExpression::named("normie");
        let mut_ref_type = TypeExpression::mutable_reference(int_type.clone());
        
        // Assignment through mutable reference should succeed
        let result = checker.check_mutable_assignment(&mut_ref_type, &int_type);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mutable_assignment_immutable_reference() {
        let checker = TypeChecker::new();
        
        let int_type = TypeExpression::named("normie");
        let immut_ref_type = TypeExpression::immutable_reference(int_type.clone());
        
        // Assignment through immutable reference should fail
        let result = checker.check_mutable_assignment(&immut_ref_type, &int_type);
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.error_type, TypeErrorKind::MutabilityViolationError);
            assert!(error.message.contains("Cannot assign through immutable reference"));
        }
    }

    #[test]
    fn test_mutable_assignment_type_mismatch() {
        let checker = TypeChecker::new();
        
        let int_type = TypeExpression::named("normie");
        let string_type = TypeExpression::named("tea");
        let mut_ref_type = TypeExpression::mutable_reference(int_type);
        
        // Assignment with wrong type should fail
        let result = checker.check_mutable_assignment(&mut_ref_type, &string_type);
        assert!(result.is_err());
        
        if let Err(error) = result {
            assert_eq!(error.error_type, TypeErrorKind::TypeMismatch);
            assert!(error.message.contains("Type mismatch in assignment"));
        }
    }

    #[test]
    fn test_move_semantics() {
        let mut checker = TypeChecker::new();
        
        // Test basic move
        let result = checker.check_move("x");
        assert!(result.is_ok());
        
        // Test use after move fails
        let second_move = checker.check_move("x");
        assert!(second_move.is_err());
        
        if let Err(error) = second_move {
            assert_eq!(error.error_type, TypeErrorKind::UseAfterFreeError);
            assert!(error.message.contains("value has already been moved"));
        }
        
        // Test borrow after move fails
        let borrow_after_move = checker.check_mutable_borrow("x");
        assert!(borrow_after_move.is_err());
        
        if let Err(error) = borrow_after_move {
            assert_eq!(error.error_type, TypeErrorKind::UseAfterFreeError);
            assert!(error.message.contains("value has been moved"));
        }
    }

    #[test]
    fn test_move_while_borrowed() {
        let mut checker = TypeChecker::new();
        
        // Borrow variable first
        let borrow_result = checker.check_mutable_borrow("x");
        assert!(borrow_result.is_ok());
        
        // Try to move while borrowed - should fail
        let move_result = checker.check_move("x");
        assert!(move_result.is_err());
        
        if let Err(error) = move_result {
            assert_eq!(error.error_type, TypeErrorKind::BorrowConflictError);
            assert!(error.message.contains("currently borrowed as mutable"));
        }
    }

    #[test]
    fn test_scope_based_borrow_release() {
        let mut checker = TypeChecker::new();
        
        // Simulate entering a new scope
        checker.scopes.push(HashMap::new());
        
        // Borrow in the new scope
        let borrow_result = checker.check_mutable_borrow("x");
        assert!(borrow_result.is_ok());
        
        // Leave the scope
        let scope_depth = checker.scopes.len();
        checker.scopes.pop();
        checker.release_scope_borrows(scope_depth);
        
        // Should be able to borrow again after scope exit
        let second_borrow = checker.check_mutable_borrow("x");
        assert!(second_borrow.is_ok());
    }

    #[test]
    fn test_multiple_immutable_borrows() {
        let mut checker = TypeChecker::new();
        
        // Multiple immutable borrows should succeed
        let borrow1 = checker.check_immutable_borrow("x");
        let borrow2 = checker.check_immutable_borrow("x");
        let borrow3 = checker.check_immutable_borrow("x");
        
        assert!(borrow1.is_ok());
        assert!(borrow2.is_ok());
        assert!(borrow3.is_ok());
        
        // Verify all borrows are tracked
        assert_eq!(checker.active_borrows.len(), 3);
        
        // All should be immutable borrows
        for borrow in &checker.active_borrows {
            assert!(!borrow.is_mutable);
            assert_eq!(borrow.variable, "x");
        }
    }

    #[test]
    fn test_pointer_types() {
        let checker = TypeChecker::new();
        
        let int_type = TypeExpression::named("normie");
        let mut_ptr_type = TypeExpression::mutable_pointer(int_type.clone());
        let immut_ptr_type = TypeExpression::immutable_pointer(int_type.clone());
        
        // Test dereference of pointers
        let mut_deref = checker.check_dereference(&mut_ptr_type);
        let immut_deref = checker.check_dereference(&immut_ptr_type);
        
        assert!(mut_deref.is_ok());
        assert!(immut_deref.is_ok());
        
        // Test assignment through mutable pointer
        let mut_assign = checker.check_mutable_assignment(&mut_ptr_type, &int_type);
        assert!(mut_assign.is_ok());
        
        // Test assignment through immutable pointer should fail
        let immut_assign = checker.check_mutable_assignment(&immut_ptr_type, &int_type);
        assert!(immut_assign.is_err());
    }

    #[test]
    fn test_type_expression_constructors() {
        let int_type = TypeExpression::named("normie");
        
        // Test mutable reference constructor
        let mut_ref = TypeExpression::mutable_reference(int_type.clone());
        if let TypeKind::Reference(pointee, is_mutable) = &mut_ref.kind {
            assert!(is_mutable);
            assert_eq!(pointee.name, Some("normie".to_string()));
        } else {
            panic!("Expected Reference type");
        }
        
        // Test immutable reference constructor
        let immut_ref = TypeExpression::immutable_reference(int_type.clone());
        if let TypeKind::Reference(pointee, is_mutable) = &immut_ref.kind {
            assert!(!is_mutable);
            assert_eq!(pointee.name, Some("normie".to_string()));
        } else {
            panic!("Expected Reference type");
        }
        
        // Test mutable pointer constructor
        let mut_ptr = TypeExpression::mutable_pointer(int_type.clone());
        if let TypeKind::Pointer(pointee, is_mutable) = &mut_ptr.kind {
            assert!(is_mutable);
            assert_eq!(pointee.name, Some("normie".to_string()));
        } else {
            panic!("Expected Pointer type");
        }
        
        // Test immutable pointer constructor
        let immut_ptr = TypeExpression::immutable_pointer(int_type.clone());
        if let TypeKind::Pointer(pointee, is_mutable) = &immut_ptr.kind {
            assert!(!is_mutable);
            assert_eq!(pointee.name, Some("normie".to_string()));
        } else {
            panic!("Expected Pointer type");
        }
    }
}
