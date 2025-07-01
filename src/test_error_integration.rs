#[cfg(test)]
mod tests {
    use crate::error::CursedError;
    use crate::stdlib::collections::CollectionsError;
    use crate::stdlib::string::StringError;

    #[test]
    fn test_collections_error_conversion() {
        let collections_error = CollectionsError::IndexOutOfBounds { index: 5, size: 3 };
        let cursed_error: CursedError = collections_error.into();
        
        match cursed_error {
            CursedError::CollectionsError(msg) => {
                assert!(msg.contains("Index 5 out of bounds"));
                assert!(msg.contains("size 3"));
            }
            _ => panic!("Expected CollectionsError variant"),
        }
    }

    #[test]
    fn test_string_error_conversion() {
        let string_error = StringError::InvalidUtf8 { position: 10 };
        let cursed_error: CursedError = string_error.into();
        
        match cursed_error {
            CursedError::StringError(msg) => {
                assert!(msg.contains("Invalid UTF-8"));
                assert!(msg.contains("position 10"));
            }
            _ => panic!("Expected StringError variant"),
        }
    }

    #[test]
    fn test_error_constructor_functions() {
        let collections_error = CursedError::collections_error("Test collections error");
        match collections_error {
            CursedError::CollectionsError(msg) => {
                assert_eq!(msg, "Test collections error");
            }
            _ => panic!("Expected CollectionsError variant"),
        }

        let string_error = CursedError::string_error("Test string error");
        match string_error {
            CursedError::StringError(msg) => {
                assert_eq!(msg, "Test string error");
            }
            _ => panic!("Expected StringError variant"),
        }
    }

    #[test]
    fn test_error_display() {
        let collections_error = CursedError::collections_error("Test error");
        let display_str = format!("{}", collections_error);
        assert_eq!(display_str, "Collections error: Test error");

        let string_error = CursedError::string_error("Test string issue");
        let display_str = format!("{}", string_error);
        assert_eq!(display_str, "String error: Test string issue");
    }
}
