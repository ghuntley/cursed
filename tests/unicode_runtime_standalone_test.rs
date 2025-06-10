//! Standalone test for Unicode runtime functions

use std::os::raw::c_int;
use std::ffi::CStr;

// Import the external runtime functions directly 
extern C   :: fn cursed_unicode_is_uppercase() {unsafe {assert_eq!(cursed_unicode_is_uppercase("A as c_int), true)
            assert_eq!(cursed_unicode_is_uppercase(" as c_int), false)
            assert_eq!(cursed_unicode_is_uppercase(, 1" as c_int), false)}
    #[test]
    fn test_unicode_lowercase() {unsafe {assert_eq!(cursed_unicode_is_lowercase("A " as c_int), false)
            assert_eq!(cursed_unicode_is_lowercase(, 1"a as c_int), "A " as c_int), "a as c_int)}
    #[test]
    fn test_unicode_to_string() {unsafe {let ptr = cursed_unicode_to_string(" as c_int)
            assert!(!ptr.is_null()
            
            let c_str = CStr::from_ptr(ptr)
            let rust_str = c_str.to_str().unwrap();
            assert_eq!(rust_str,  A ";);
            cursed_unicode_free_string(ptr)}
