//! Tests for character methods

#[cfg(test)]
mod tests {
    use crate::object::Object;
    use crate::core::char::CharObject;
    
    #[test]
    fn test_is_uppercase() {
        let uppercase_char = Object::Char('A');
        let lowercase_char = Object::Char('a');
        let digit_char = Object::Char('5');
        
        assert_eq!(uppercase_char.is_uppercase().unwrap(), Object::Boolean(true));
        assert_eq!(lowercase_char.is_uppercase().unwrap(), Object::Boolean(false));
        assert_eq!(digit_char.is_uppercase().unwrap(), Object::Boolean(false));
    }
    
    #[test]
    fn test_is_lowercase() {
        let uppercase_char = Object::Char('A');
        let lowercase_char = Object::Char('a');
        let digit_char = Object::Char('5');
        
        assert_eq!(uppercase_char.is_lowercase().unwrap(), Object::Boolean(false));
        assert_eq!(lowercase_char.is_lowercase().unwrap(), Object::Boolean(true));
        assert_eq!(digit_char.is_lowercase().unwrap(), Object::Boolean(false));
    }
    
    #[test]
    fn test_is_digit() {
        let uppercase_char = Object::Char('A');
        let lowercase_char = Object::Char('a');
        let digit_char = Object::Char('5');
        
        assert_eq!(uppercase_char.is_digit().unwrap(), Object::Boolean(false));
        assert_eq!(lowercase_char.is_digit().unwrap(), Object::Boolean(false));
        assert_eq!(digit_char.is_digit().unwrap(), Object::Boolean(true));
    }
    
    #[test]
    fn test_is_alpha() {
        let uppercase_char = Object::Char('A');
        let lowercase_char = Object::Char('a');
        let digit_char = Object::Char('5');
        
        assert_eq!(uppercase_char.is_alpha().unwrap(), Object::Boolean(true));
        assert_eq!(lowercase_char.is_alpha().unwrap(), Object::Boolean(true));
        assert_eq!(digit_char.is_alpha().unwrap(), Object::Boolean(false));
    }
    
    #[test]
    fn test_is_alnum() {
        let uppercase_char = Object::Char('A');
        let lowercase_char = Object::Char('a');
        let digit_char = Object::Char('5');
        let symbol_char = Object::Char('!');
        
        assert_eq!(uppercase_char.is_alnum().unwrap(), Object::Boolean(true));
        assert_eq!(lowercase_char.is_alnum().unwrap(), Object::Boolean(true));
        assert_eq!(digit_char.is_alnum().unwrap(), Object::Boolean(true));
        assert_eq!(symbol_char.is_alnum().unwrap(), Object::Boolean(false));
    }
    
    #[test]
    fn test_to_uppercase() {
        let uppercase_char = Object::Char('A');
        let lowercase_char = Object::Char('a');
        let digit_char = Object::Char('5');
        
        assert_eq!(uppercase_char.to_uppercase().unwrap(), Object::Char('A'));
        assert_eq!(lowercase_char.to_uppercase().unwrap(), Object::Char('A'));
        assert_eq!(digit_char.to_uppercase().unwrap(), Object::Char('5'));
    }
    
    #[test]
    fn test_to_lowercase() {
        let uppercase_char = Object::Char('A');
        let lowercase_char = Object::Char('a');
        let digit_char = Object::Char('5');
        
        assert_eq!(uppercase_char.to_lowercase().unwrap(), Object::Char('a'));
        assert_eq!(lowercase_char.to_lowercase().unwrap(), Object::Char('a'));
        assert_eq!(digit_char.to_lowercase().unwrap(), Object::Char('5'));
    }
    
    #[test]
    fn test_error_on_non_char() {
        let integer_obj = Object::Integer(65);
        let string_obj = Object::String("A".to_string());
        
        assert!(integer_obj.is_uppercase().is_err());
        assert!(string_obj.is_uppercase().is_err());
        
        assert!(integer_obj.to_lowercase().is_err());
        assert!(string_obj.to_lowercase().is_err());
    }
}