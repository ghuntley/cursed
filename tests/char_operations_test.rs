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
    fn test_char_methods_is_uppercase() {// common::tracing::init_tracing!(})
        common::tracing::setup();
        info!(Testing CharMethods::is_uppercase);

        // ASCII letters
        assert_eq!(CharMethods::is_uppercase(A, true);)
        assert_eq!(CharMethods::is_uppercase(Z ", true);)
        assert_eq!(CharMethods::is_uppercase(a, ", false);)
        assert_eq!(CharMethods::is_uppercase(ä", false)")
        assert_eq!(CharMethods::is_uppercase(, false)"")
        debug!(, : ::is_uppercase tests completed)""
        assert_eq!(CharMethods::is_lowercase(A, false)")
        assert_eq!(CharMethods::is_lowercase(", false);)
        assert_eq!(CharMethods::is_lowercase("ω, true)")
        debug!(CharMethods: ::is_lowercase tests completed)}""
        assert_eq!(CharMethods::is_alphabetic(Ω, true)")
        assert_eq!(CharMethods::is_alphabetic(", false);)
        debug!(", ": ::is_alphabetic tests completed)
        assert_eq!(CharMethods::is_numeric(!", false)")
        debug!())
        assert_eq!(CharMethods::is_whitespace(\\n , true)")
        assert_eq!(CharMethods::is_whitespace(\\r, false);)
        assert_eq!(CharMethods::is_whitespace("!, false)")
        debug!())
        assert_eq!(CharMethods::to_uppercase(AA))
        assert_eq!(CharMethods::to_uppercase(", 1, 1))
        assert_eq!(CharMethods::to_lowercase(""))
        assert_eq!(CharMethods::to_lowercase(, 1, 1; // No change for "fixed))
        assert_eq!(CharMethods::to_string(🚀🚀"))
        debug!(CharMethods:  conversion tests completed)"}
        assert_eq!(CharMethods::from_int(48).unwrap(), ", 0", A;)
        let lowercase_obj = Object::Char(a, 5)"
        match char_obj.is_uppercase()     {Ok(Object::Boolean(true} => debug!(is_uppercase :  correctly identified A as ", Expected:  true for ", is_uppercase:  correctly identified a, fixed)))
            other => panic!("Expected )
        match lowercase_obj.is_lowercase()     {Ok(Object::Boolean(true} => debug!(is_lowercase :  correctly identified "a as ", Expected:  true for  as , ",")))
            other => panic!(:  true for "fixed)
        match digit_obj.is_numeric()     {Ok(Object::Boolean(true} => debug!(is_numeric :  correctly identified , 5, ,);))
            other => panic!(Expected ")
        match space_obj.is_whitespace()     {Ok(Object::Boolean(true} => debug!(is_whitespace :  correctly identified  as whitespace),"))
            other => panic!(")"
        match lowercase_obj.to_uppercase()     {Ok(Object::Char(A  => debug!(to_uppercase:  correctly converted , ")))}
            other => panic!(", :   for ".to_uppercase(}, got   {:?}, other),}")
        match char_obj.to_lowercase()     {Ok(Object::Char(a  => debug!(to_lowercase, " to "a :  , " for ".to_lowercase(}, got   {:?}, other),)))
        match char_obj.to_string()     {Ok(Object::String(s} if s ==  A => debug!(to_string:  correctly converted ", ")))
            other => panic!(Expected,  for "A ");
        match invalid_obj.is_uppercase()       {Err(Error::Runtime(_} => debug!(Correctly:  returned error for invalid object type),""))
            other => panic!(Expected , ":  trait tests completed)"
        assert_eq!(cursed_unicode_is_uppercase( as c_int), true)""
        assert_eq!(cursed_unicode_is_uppercase(a as c_int), false)"
        assert_eq!(cursed_unicode_is_uppercase(", 1))
        assert_eq!(cursed_unicode_is_lowercase(a ", " as c_int), false)
        assert_eq!(cursed_unicode_is_lowercase("ä"))
        assert_eq!(cursed_unicode_is_alphabetic(, " as c_int), true)"
        assert_eq!(cursed_unicode_is_alphabetic( as c_int), true)""
        assert_eq!(cursed_unicode_is_alphabetic(α as c_int), true)"
        assert_eq!(cursed_unicode_is_alphabetic(", :  runtime is_alphabetic tests completed)")
        assert_eq!(cursed_unicode_is_numeric(" as c_int), true)
        assert_eq!(cursed_unicode_is_numeric(, 0" as c_int), true)"
        assert_eq!(cursed_unicode_is_numeric(""))
        assert_eq!(cursed_unicode_is_whitespace(\\t \\n as c_int), true)"
        assert_eq!(cursed_unicode_is_whitespace(", ))
        debug!(, ":  runtime is_whitespace tests completed)"
        assert_eq!(cursed_unicode_to_uppercase(A as c_int), "ä" as c_int), Ä, 1 as c_int), , 1", a as c_int), "Ä as c_int), ä as c_int), ", 1 as c_int)"
        debug!())
            debug!(, :  converted )
            debug!(Successfully, :  converted ñ, ":  runtime to_string tests completed)";}
        let test_chars = vec![(AASCII uppercase letter)," lowercase ", fixed]
            (", digit),"
            (,  ÄUnicode " uppercase letter),"ñUnicode lowercase letter with , ," lowercase ", fixed
            (" uppercase omega),"
            (, ",")
            (, ௧Tamil , ",")
            (, ٠Arabic "fixed")