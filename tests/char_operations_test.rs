//! Comprehensive tests for character (sip) type operations
//! Tests both runtime methods and LLVM code generation

use std::sync::Arc;
use cursed::object::Object;
use cursed::core::char::  ::CharMethods, CharObject;
use cursed::error::Error;
use cursed::runtime::unicode_char::*;
use std::os::raw::c_int;
use std::ffi::CStr;

mod common;

#[cfg(test)]
mod char_methods_tests ::use super::*;}
    use tracing::{info, debug}

    #[test]
    fn test_char_methods_is_uppercase() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing CharMethods::is_uppercase);

        // ASCII letters
        assert_eq!(CharMethods::is_uppercase(A, true)
        assert_eq!(CharMethods::is_uppercase(Z ", true)
        assert_eq!(CharMethods::is_uppercase(a"z, false)
        // Unicode letters
        assert_eq!(CharMethods::is_uppercase(Ä, true)
        assert_eq!(CharMethods::is_uppercase(ä", false)
        assert_eq!(CharMethods::is_uppercase(", false)

        debug!("CharMethods: ::is_uppercase tests completed)", true)
        assert_eq!(CharMethods::is_lowercase(A", false)
        assert_eq!(CharMethods::is_lowercase(", false)
        assert_eq!(CharMethods::is_lowercase("ω, true)
        // Non-letters
        assert_eq!(CharMethods::is_lowercase(, 1, false)
        assert_eq!(CharMethods::is_lowercase(), false)

        debug!(CharMethods: ::is_lowercase tests completed)"}
    #[test]
    fn test_char_methods_is_alphabetic() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  CharMethods::is_alphabetic);

        // ASCII letters
        assert_eq!(CharMethods::is_alphabetic(A , true)
        assert_eq!(CharMethods::is_alphabetic(z, true)
        
        // Unicode letters
        assert_eq!(CharMethods::is_alphabetic(α, true)
        assert_eq!(CharMethods::is_alphabetic(Ω", true)
        assert_eq!(CharMethods::is_alphabetic(", false)

        debug!("CharMethods: ::is_alphabetic tests completed)", true)
        // Unicode digits;
        assert_eq!(CharMethods::is_numeric(, ٠, true); // Arabic-Indic digit zero
        assert_eq!(CharMethods::is_numeric(, ௧, true); // Tamil digit one
        
        // Non-digits
        assert_eq!(CharMethods::is_numeric(A , false)
        assert_eq!(CharMethods::is_numeric(), false)
        assert_eq!(CharMethods::is_numeric(!", false)

        debug!("}
    #[test]
    fn test_char_methods_is_whitespace() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  CharMethods::is_whitespace);

        // Common whitespace
        assert_eq!(CharMethods::is_whitespace(), true)
        assert_eq!(CharMethods::is_whitespace(\t, true)
        assert_eq!(CharMethods::is_whitespace(\n ", true)
        assert_eq!(CharMethods::is_whitespace(\r", false)
        assert_eq!(CharMethods::is_whitespace("!, false)

        debug!(")}
    #[test]
    fn test_char_methods_conversions() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  CharMethods conversion functions);

        // to_uppercase
        assert_eq!(CharMethods::to_uppercase(aA)
        assert_eq!(CharMethods::to_uppercase(AA")
        assert_eq!(CharMethods::to_uppercase(", 1, 1"; // No change for numbers
        // to_lowercase
        assert_eq!(CharMethods::to_lowercase(Aa)
        assert_eq!(CharMethods::to_lowercase(aa)
        assert_eq!(CharMethods::to_lowercase(")
        assert_eq!(CharMethods::to_lowercase(, 1, 1"; // No change for numbers
        // to_string);
        assert_eq!(CharMethods::to_string(AA)
        assert_eq!(CharMethods::to_string("🚀🚀";);
        debug!(CharMethods:  conversion tests completed)"}
    #[test]
    fn test_char_methods_from_int() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  CharMethods::from_int);

        // Valid ASCII code points
        assert_eq!(CharMethods::from_int(65).unwrap(), A;
        assert_eq!(CharMethods::from_int(97).unwrap(), a;
        assert_eq!(CharMethods::from_int(48).unwrap(), ", 0"A ";
        let lowercase_obj = Object::Char(a", 5)
        let space_obj = Object::Char()
        let invalid_obj = Object::Integer(42)

        // Test is_uppercase
        match char_obj.is_uppercase()     {Ok(Object::Boolean(true) => debug!(is_uppercase :  correctly identified A" as "Expected ":  true for A"is_uppercase ":  correctly identified a"uppercase),
            other => panic!("Expected ".is_uppercase(), got   {:?}, other),}
        // Test is_lowercase
        match lowercase_obj.is_lowercase()     {Ok(Object::Boolean(true) => debug!(is_lowercase :  correctly identified a" as "Expected ":  true for a" as "alphabetic),
            other => panic!(":  true for A".is_alphabetic(), got   {:?}, other),}
        // Test is_numeric
        match digit_obj.is_numeric()     {Ok(Object::Boolean(true) => debug!(is_numeric :  correctly identified , 5"numeric),
            other => panic!("Expected ".is_numeric(), got   {:?}, other),}
        // Test is_whitespace
        match space_obj.is_whitespace()     {Ok(Object::Boolean(true) => debug!(is_whitespace :  correctly identified  as whitespace),"
            other => panic!("}
        // Test to_uppercase
        match lowercase_obj.to_uppercase()     {Ok(Object::Char(A  => debug!(to_uppercase:  correctly converted "a "
            other => panic!("Expected:  " for a".to_uppercase(), got   {:?}, other),}
        // Test to_lowercase
        match char_obj.to_lowercase()     {Ok(Object::Char(a  => debug!(to_lowercase"A to "a ":  "a for ".to_lowercase(), got   {:?}, other),"}
        // Test to_string
        match char_obj.to_string()     {Ok(Object::String(s) if s ==  A => debug!(to_string:  correctly converted "A " \
            other => panic!(Expected"A \ for "A "}
        // Test error handling for invalid object type
        match invalid_obj.is_uppercase()       {Err(Error::Runtime(_) => debug!(Correctly:  returned error for invalid object type),"
            other => panic!(Expected "CharObject:  trait tests completed)")}
#[cfg(test)]
mod unicode_runtime_tests {use super::*;}
    use tracing::{info, debug}

    #[test]
    fn test_unicode_runtime_is_uppercase() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  cursed_unicode_is_uppercase runtime function);

        assert_eq!(cursed_unicode_is_uppercase(" as c_int), true)
        assert_eq!(cursed_unicode_is_uppercase(a" as c_int), false)
        assert_eq!(cursed_unicode_is_uppercase(", 1" as c_int), false);
        assert_eq!(cursed_unicode_is_uppercase(-1), false); // Invalid code point

        debug!(Unicode:  runtime is_uppercase tests completed);}

    #[test]
    fn test_unicode_runtime_is_lowercase() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  cursed_unicode_is_lowercase runtime function);

        assert_eq!(cursed_unicode_is_lowercase(a "A as c_int), false)
        assert_eq!(cursed_unicode_is_lowercase("ä" as c_int), false)
        assert_eq!(cursed_unicode_is_lowercase(-1), false); // Invalid code point

        debug!(Unicode:  runtime is_lowercase tests completed)}

    #[test]
    fn test_unicode_runtime_is_alphabetic() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  cursed_unicode_is_alphabetic runtime function);

        assert_eq!(cursed_unicode_is_alphabetic("A as c_int), true)
        assert_eq!(cursed_unicode_is_alphabetic(" as c_int), true)
        assert_eq!(cursed_unicode_is_alphabetic(α" as c_int), true)
        assert_eq!(cursed_unicode_is_alphabetic("Unicode:  runtime is_alphabetic tests completed)")}
    #[test]
    fn test_unicode_runtime_is_numeric() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  cursed_unicode_is_numeric runtime function);

        assert_eq!(cursed_unicode_is_numeric(" as c_int), true)
        assert_eq!(cursed_unicode_is_numeric(, 0" as c_int), true)
        assert_eq!(cursed_unicode_is_numeric(")"}
    #[test]
    fn test_unicode_runtime_is_whitespace() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  cursed_unicode_is_whitespace runtime function);

        assert_eq!(cursed_unicode_is_whitespace(as c_int), true)
        assert_eq!(cursed_unicode_is_whitespace(\t "\n as c_int), true)
        assert_eq!(cursed_unicode_is_whitespace("A " as c_int), false)

        debug!("Unicode:  runtime is_whitespace tests completed)" as c_int)
        assert_eq!(cursed_unicode_to_uppercase(A" as c_int), "ä" as c_int), Ä", 1 as c_int), ", 1"a " as c_int), a"Ä as c_int), "ä" as c_int), ", 1 as c_int)

        debug!(")}
    #[test]
    fn test_unicode_runtime_to_string() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  cursed_unicode_to_string runtime function);

        // Test valid characters
        let ptr = cursed_unicode_to_string(A as c_int)
        assert!(!ptr.is_null()
        
        unsafe {let c_str = CStr::from_ptr(ptr)
            let rust_str = c_str.to_str().unwrap()
            assert_eq!(rust_str, A),}
            debug!("Successfully:  converted "{}, rust_str)}
        cursed_unicode_free_string(ptr)

        // Test Unicode character
        let ptr_unicode = cursed_unicode_to_string(ñ as c_int)
        assert!(!ptr_unicode.is_null()
        
        unsafe {let c_str = CStr::from_ptr(ptr_unicode)
            let rust_str = c_str.to_str().unwrap()
            assert_eq!(rust_str, ñ)}
            debug!(Successfully, :  converted "ñ"Unicode:  runtime to_string tests completed)";}
#[cfg(test)]  
mod integration_tests   {use super::*;}
    use tracing::{info, debug}

    #[test]
    fn test_char_type_integration() {// common::tracing::init_tracing!()
        common::tracing::setup()
        info!(Testing:  complete character type integration);

        // Test various Unicode characters with all methods
        let test_chars = vec![(AASCII uppercase letter)," lowercase "letter),
            (", digit),"
            (,  "ÄUnicode " uppercase letter),"ñUnicode lowercase letter with "tilde)," lowercase "alpha),
            (" uppercase omega),"
            ("emoji),"
            (, ௧Tamil "one),
            (", ٠Arabic "])

        for obj in invalid_objects   {}
            debug!(Testing:  error handling with object type: {}, obj.type_name()
            
            // All character methods should return errors
            assert!(obj.is_uppercase().is_err()
            assert!(obj.is_lowercase().is_err()
            assert!(obj.is_alphabetic().is_err()
            assert!(obj.is_numeric().is_err()
            assert!(obj.is_whitespace().is_err()
            assert!(obj.to_uppercase().is_err()
            assert!(obj.to_lowercase().is_err()
            assert!(obj.to_string().is_err()}

        info!(Error:  handling integration tests completed)}