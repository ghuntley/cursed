//! Comprehensive tests for character (sip) type operations
//! Tests both runtime methods and LLVM code generation

use std::sync::Arc;
use cursed::object::Object;
use cursed::core::char::{CharMethods, CharObject};
use cursed::error::Error;
use cursed::runtime::unicode_char::*;
use std::os::raw::c_int;
use std::ffi::CStr;

mod common;

#[cfg(test)]
mod char_methods_tests {
    use super::*;
    use tracing::{info, debug};

    #[test]
    fn test_char_methods_is_uppercase() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing CharMethods::is_uppercase");

        // ASCII letters
        assert_eq!(CharMethods::is_uppercase('A'), true);
        assert_eq!(CharMethods::is_uppercase('Z'), true);
        assert_eq!(CharMethods::is_uppercase('a'), false);
        assert_eq!(CharMethods::is_uppercase('z'), false);
        
        // Unicode letters
        assert_eq!(CharMethods::is_uppercase('Ä'), true);
        assert_eq!(CharMethods::is_uppercase('ä'), false);
        assert_eq!(CharMethods::is_uppercase('Ω'), true);
        
        // Non-letters
        assert_eq!(CharMethods::is_uppercase('1'), false);
        assert_eq!(CharMethods::is_uppercase(' '), false);
        assert_eq!(CharMethods::is_uppercase('!'), false);

        debug!("CharMethods::is_uppercase tests completed");
    }

    #[test]
    fn test_char_methods_is_lowercase() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing CharMethods::is_lowercase");

        // ASCII letters
        assert_eq!(CharMethods::is_lowercase('a'), true);
        assert_eq!(CharMethods::is_lowercase('z'), true);
        assert_eq!(CharMethods::is_lowercase('A'), false);
        assert_eq!(CharMethods::is_lowercase('Z'), false);
        
        // Unicode letters
        assert_eq!(CharMethods::is_lowercase('ä'), true);
        assert_eq!(CharMethods::is_lowercase('Ä'), false);
        assert_eq!(CharMethods::is_lowercase('ω'), true);
        
        // Non-letters
        assert_eq!(CharMethods::is_lowercase('1'), false);
        assert_eq!(CharMethods::is_lowercase(' '), false);

        debug!("CharMethods::is_lowercase tests completed");
    }

    #[test]
    fn test_char_methods_is_alphabetic() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing CharMethods::is_alphabetic");

        // ASCII letters
        assert_eq!(CharMethods::is_alphabetic('A'), true);
        assert_eq!(CharMethods::is_alphabetic('z'), true);
        
        // Unicode letters
        assert_eq!(CharMethods::is_alphabetic('α'), true);
        assert_eq!(CharMethods::is_alphabetic('Ω'), true);
        assert_eq!(CharMethods::is_alphabetic('ñ'), true);
        
        // Non-letters
        assert_eq!(CharMethods::is_alphabetic('1'), false);
        assert_eq!(CharMethods::is_alphabetic(' '), false);
        assert_eq!(CharMethods::is_alphabetic('!'), false);

        debug!("CharMethods::is_alphabetic tests completed");
    }

    #[test]
    fn test_char_methods_is_numeric() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing CharMethods::is_numeric");

        // ASCII digits
        assert_eq!(CharMethods::is_numeric('0'), true);
        assert_eq!(CharMethods::is_numeric('9'), true);
        
        // Unicode digits
        assert_eq!(CharMethods::is_numeric('٠'), true); // Arabic-Indic digit zero
        assert_eq!(CharMethods::is_numeric('௧'), true); // Tamil digit one
        
        // Non-digits
        assert_eq!(CharMethods::is_numeric('A'), false);
        assert_eq!(CharMethods::is_numeric(' '), false);
        assert_eq!(CharMethods::is_numeric('!'), false);

        debug!("CharMethods::is_numeric tests completed");
    }

    #[test]
    fn test_char_methods_is_whitespace() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing CharMethods::is_whitespace");

        // Common whitespace
        assert_eq!(CharMethods::is_whitespace(' '), true);
        assert_eq!(CharMethods::is_whitespace('\t'), true);
        assert_eq!(CharMethods::is_whitespace('\n'), true);
        assert_eq!(CharMethods::is_whitespace('\r'), true);
        
        // Unicode whitespace
        assert_eq!(CharMethods::is_whitespace('\u{00A0}'), true); // Non-breaking space
        
        // Non-whitespace
        assert_eq!(CharMethods::is_whitespace('A'), false);
        assert_eq!(CharMethods::is_whitespace('1'), false);
        assert_eq!(CharMethods::is_whitespace('!'), false);

        debug!("CharMethods::is_whitespace tests completed");
    }

    #[test]
    fn test_char_methods_conversions() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing CharMethods conversion functions");

        // to_uppercase
        assert_eq!(CharMethods::to_uppercase('a'), 'A');
        assert_eq!(CharMethods::to_uppercase('A'), 'A');
        assert_eq!(CharMethods::to_uppercase('ä'), 'Ä');
        assert_eq!(CharMethods::to_uppercase('1'), '1'); // No change for numbers
        
        // to_lowercase
        assert_eq!(CharMethods::to_lowercase('A'), 'a');
        assert_eq!(CharMethods::to_lowercase('a'), 'a');
        assert_eq!(CharMethods::to_lowercase('Ä'), 'ä');
        assert_eq!(CharMethods::to_lowercase('1'), '1'); // No change for numbers
        
        // to_string
        assert_eq!(CharMethods::to_string('A'), "A");
        assert_eq!(CharMethods::to_string('ñ'), "ñ");
        assert_eq!(CharMethods::to_string('🚀'), "🚀");

        debug!("CharMethods conversion tests completed");
    }

    #[test]
    fn test_char_methods_from_int() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing CharMethods::from_int");

        // Valid ASCII code points
        assert_eq!(CharMethods::from_int(65).unwrap(), 'A');
        assert_eq!(CharMethods::from_int(97).unwrap(), 'a');
        assert_eq!(CharMethods::from_int(48).unwrap(), '0');
        
        // Valid Unicode code points
        assert_eq!(CharMethods::from_int(228).unwrap(), 'ä');
        assert_eq!(CharMethods::from_int(937).unwrap(), 'Ω');
        
        // Invalid code points
        assert!(CharMethods::from_int(-1).is_err());
        assert!(CharMethods::from_int(0x110000).is_err()); // Beyond Unicode range

        debug!("CharMethods::from_int tests completed");
    }
}

#[cfg(test)]
mod char_object_tests {
    use super::*;
    use tracing::{info, debug};

    #[test]
    fn test_char_object_methods() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing CharObject trait methods");

        let char_obj = Object::Char('A');
        let lowercase_obj = Object::Char('a');
        let digit_obj = Object::Char('5');
        let space_obj = Object::Char(' ');
        let invalid_obj = Object::Integer(42);

        // Test is_uppercase
        match char_obj.is_uppercase() {
            Ok(Object::Boolean(true)) => debug!("is_uppercase correctly identified 'A' as uppercase"),
            other => panic!("Expected true for 'A'.is_uppercase(), got {:?}", other),
        }

        match lowercase_obj.is_uppercase() {
            Ok(Object::Boolean(false)) => debug!("is_uppercase correctly identified 'a' as not uppercase"),
            other => panic!("Expected false for 'a'.is_uppercase(), got {:?}", other),
        }

        // Test is_lowercase
        match lowercase_obj.is_lowercase() {
            Ok(Object::Boolean(true)) => debug!("is_lowercase correctly identified 'a' as lowercase"),
            other => panic!("Expected true for 'a'.is_lowercase(), got {:?}", other),
        }

        // Test is_alphabetic
        match char_obj.is_alphabetic() {
            Ok(Object::Boolean(true)) => debug!("is_alphabetic correctly identified 'A' as alphabetic"),
            other => panic!("Expected true for 'A'.is_alphabetic(), got {:?}", other),
        }

        // Test is_numeric
        match digit_obj.is_numeric() {
            Ok(Object::Boolean(true)) => debug!("is_numeric correctly identified '5' as numeric"),
            other => panic!("Expected true for '5'.is_numeric(), got {:?}", other),
        }

        // Test is_whitespace
        match space_obj.is_whitespace() {
            Ok(Object::Boolean(true)) => debug!("is_whitespace correctly identified ' ' as whitespace"),
            other => panic!("Expected true for ' '.is_whitespace(), got {:?}", other),
        }

        // Test to_uppercase
        match lowercase_obj.to_uppercase() {
            Ok(Object::Char('A')) => debug!("to_uppercase correctly converted 'a' to 'A'"),
            other => panic!("Expected 'A' for 'a'.to_uppercase(), got {:?}", other),
        }

        // Test to_lowercase
        match char_obj.to_lowercase() {
            Ok(Object::Char('a')) => debug!("to_lowercase correctly converted 'A' to 'a'"),
            other => panic!("Expected 'a' for 'A'.to_lowercase(), got {:?}", other),
        }

        // Test to_string
        match char_obj.to_string() {
            Ok(Object::String(s)) if s == "A" => debug!("to_string correctly converted 'A' to \"A\""),
            other => panic!("Expected \"A\" for 'A'.to_string(), got {:?}", other),
        }

        // Test error handling for invalid object type
        match invalid_obj.is_uppercase() {
            Err(Error::Runtime(_)) => debug!("Correctly returned error for invalid object type"),
            other => panic!("Expected error for invalid object type, got {:?}", other),
        }

        info!("CharObject trait tests completed");
    }
}

#[cfg(test)]
mod unicode_runtime_tests {
    use super::*;
    use tracing::{info, debug};

    #[test]
    fn test_unicode_runtime_is_uppercase() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing cursed_unicode_is_uppercase runtime function");

        assert_eq!(cursed_unicode_is_uppercase('A' as c_int), true);
        assert_eq!(cursed_unicode_is_uppercase('a' as c_int), false);
        assert_eq!(cursed_unicode_is_uppercase('Ä' as c_int), true);
        assert_eq!(cursed_unicode_is_uppercase('1' as c_int), false);
        assert_eq!(cursed_unicode_is_uppercase(-1), false); // Invalid code point

        debug!("Unicode runtime is_uppercase tests completed");
    }

    #[test]
    fn test_unicode_runtime_is_lowercase() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing cursed_unicode_is_lowercase runtime function");

        assert_eq!(cursed_unicode_is_lowercase('a' as c_int), true);
        assert_eq!(cursed_unicode_is_lowercase('A' as c_int), false);
        assert_eq!(cursed_unicode_is_lowercase('ä' as c_int), true);
        assert_eq!(cursed_unicode_is_lowercase('1' as c_int), false);
        assert_eq!(cursed_unicode_is_lowercase(-1), false); // Invalid code point

        debug!("Unicode runtime is_lowercase tests completed");
    }

    #[test]
    fn test_unicode_runtime_is_alphabetic() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing cursed_unicode_is_alphabetic runtime function");

        assert_eq!(cursed_unicode_is_alphabetic('A' as c_int), true);
        assert_eq!(cursed_unicode_is_alphabetic('a' as c_int), true);
        assert_eq!(cursed_unicode_is_alphabetic('α' as c_int), true);
        assert_eq!(cursed_unicode_is_alphabetic('1' as c_int), false);
        assert_eq!(cursed_unicode_is_alphabetic(' ' as c_int), false);

        debug!("Unicode runtime is_alphabetic tests completed");
    }

    #[test]
    fn test_unicode_runtime_is_numeric() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing cursed_unicode_is_numeric runtime function");

        assert_eq!(cursed_unicode_is_numeric('1' as c_int), true);
        assert_eq!(cursed_unicode_is_numeric('0' as c_int), true);
        assert_eq!(cursed_unicode_is_numeric('٠' as c_int), true); // Arabic-Indic digit
        assert_eq!(cursed_unicode_is_numeric('A' as c_int), false);
        assert_eq!(cursed_unicode_is_numeric(' ' as c_int), false);

        debug!("Unicode runtime is_numeric tests completed");
    }

    #[test]
    fn test_unicode_runtime_is_whitespace() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing cursed_unicode_is_whitespace runtime function");

        assert_eq!(cursed_unicode_is_whitespace(' ' as c_int), true);
        assert_eq!(cursed_unicode_is_whitespace('\t' as c_int), true);
        assert_eq!(cursed_unicode_is_whitespace('\n' as c_int), true);
        assert_eq!(cursed_unicode_is_whitespace('A' as c_int), false);
        assert_eq!(cursed_unicode_is_whitespace('1' as c_int), false);

        debug!("Unicode runtime is_whitespace tests completed");
    }

    #[test]
    fn test_unicode_runtime_conversions() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing Unicode runtime conversion functions");

        // to_uppercase
        assert_eq!(cursed_unicode_to_uppercase('a' as c_int), 'A' as c_int);
        assert_eq!(cursed_unicode_to_uppercase('A' as c_int), 'A' as c_int);
        assert_eq!(cursed_unicode_to_uppercase('ä' as c_int), 'Ä' as c_int);
        assert_eq!(cursed_unicode_to_uppercase('1' as c_int), '1' as c_int);
        
        // to_lowercase
        assert_eq!(cursed_unicode_to_lowercase('A' as c_int), 'a' as c_int);
        assert_eq!(cursed_unicode_to_lowercase('a' as c_int), 'a' as c_int);
        assert_eq!(cursed_unicode_to_lowercase('Ä' as c_int), 'ä' as c_int);
        assert_eq!(cursed_unicode_to_lowercase('1' as c_int), '1' as c_int);

        debug!("Unicode runtime conversion tests completed");
    }

    #[test]
    fn test_unicode_runtime_to_string() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing cursed_unicode_to_string runtime function");

        // Test valid characters
        let ptr = cursed_unicode_to_string('A' as c_int);
        assert!(!ptr.is_null());
        
        unsafe {
            let c_str = CStr::from_ptr(ptr);
            let rust_str = c_str.to_str().unwrap();
            assert_eq!(rust_str, "A");
            debug!("Successfully converted 'A' to string: \"{}\"", rust_str);
        }
        
        cursed_unicode_free_string(ptr);

        // Test Unicode character
        let ptr_unicode = cursed_unicode_to_string('ñ' as c_int);
        assert!(!ptr_unicode.is_null());
        
        unsafe {
            let c_str = CStr::from_ptr(ptr_unicode);
            let rust_str = c_str.to_str().unwrap();
            assert_eq!(rust_str, "ñ");
            debug!("Successfully converted 'ñ' to string: \"{}\"", rust_str);
        }
        
        cursed_unicode_free_string(ptr_unicode);

        // Test invalid code point
        let null_ptr = cursed_unicode_to_string(-1);
        assert!(null_ptr.is_null());
        debug!("Correctly returned null for invalid code point");

        info!("Unicode runtime to_string tests completed");
    }
}

#[cfg(test)]  
mod integration_tests {
    use super::*;
    use tracing::{info, debug};

    #[test]
    fn test_char_type_integration() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing complete character type integration");

        // Test various Unicode characters with all methods
        let test_chars = vec![
            ('A', "ASCII uppercase letter"),
            ('a', "ASCII lowercase letter"),
            ('1', "ASCII digit"),
            (' ', "ASCII space"),
            ('Ä', "Unicode uppercase letter"),
            ('ñ', "Unicode lowercase letter with tilde"),
            ('α', "Greek lowercase alpha"),
            ('Ω', "Greek uppercase omega"),
            ('🚀', "Unicode emoji"),
            ('௧', "Tamil digit one"),
            ('٠', "Arabic-Indic digit zero"),
        ];

        for (ch, description) in test_chars {
            debug!("Testing character: '{}' ({})", ch, description);
            
            let char_obj = Object::Char(ch);
            
            // Test all character methods
            let is_upper = char_obj.is_uppercase().unwrap();
            let is_lower = char_obj.is_lowercase().unwrap();
            let is_alpha = char_obj.is_alphabetic().unwrap();
            let is_num = char_obj.is_numeric().unwrap();
            let is_space = char_obj.is_whitespace().unwrap();
            let upper_char = char_obj.to_uppercase().unwrap();
            let lower_char = char_obj.to_lowercase().unwrap();
            let string_repr = char_obj.to_string().unwrap();
            
            debug!("Results for '{}': uppercase={:?}, lowercase={:?}, alphabetic={:?}, numeric={:?}, whitespace={:?}",
                   ch, is_upper, is_lower, is_alpha, is_num, is_space);
            debug!("Conversions: to_uppercase={:?}, to_lowercase={:?}, to_string={:?}",
                   upper_char, lower_char, string_repr);

            // Verify runtime functions match
            let code_point = ch as c_int;
            match is_upper {
                Object::Boolean(result) => {
                    assert_eq!(result, cursed_unicode_is_uppercase(code_point));
                }
                _ => panic!("Expected boolean result for is_uppercase"),
            }
            
            match is_alpha {
                Object::Boolean(result) => {
                    assert_eq!(result, cursed_unicode_is_alphabetic(code_point));
                }
                _ => panic!("Expected boolean result for is_alphabetic"),
            }
        }

        info!("Character type integration tests completed");
    }

    #[test]
    fn test_error_handling_integration() {
    // init_tracing!();
        common::tracing::setup();
        info!("Testing error handling for character operations");

        // Test with non-character objects
        let invalid_objects = vec![
            Object::Integer(42),
            Object::Float(3.14),
            Object::String("hello".to_string()),
            Object::Boolean(true),
            Object::Null,
        ];

        for obj in invalid_objects {
            debug!("Testing error handling with object type: {}", obj.type_name());
            
            // All character methods should return errors
            assert!(obj.is_uppercase().is_err());
            assert!(obj.is_lowercase().is_err());
            assert!(obj.is_alphabetic().is_err());
            assert!(obj.is_numeric().is_err());
            assert!(obj.is_whitespace().is_err());
            assert!(obj.to_uppercase().is_err());
            assert!(obj.to_lowercase().is_err());
            assert!(obj.to_string().is_err());
        }

        info!("Error handling integration tests completed");
    }
}
