use cursed::error::Error;
use cursed::object::Object;
use std::rc::Rc;

#[test]
fn test_cryptz_md5() -> Result<(), Error> {
    // Test data
    let test_string = "test_data";
    let expected_md5 = "eb733a00c0c9d336e65691a37ab54293";
    
    // Call md5sum
    let args = vec![Rc::new(Object::String(test_string.to_string()))];
    let result = cursed::stdlib::cryptz::md5sum(&args)?;
    
    // Verify the result
    match &*result {
        Object::String(hash) => {
            assert_eq!(hash, expected_md5, "MD5 hash doesn't match expected value");
            Ok(())
        },
        _ => Err(Error::Runtime("Expected string result from md5sum".to_string())),
    }
}

#[test]
fn test_cryptz_sha1() -> Result<(), Error> {
    // Test data
    let test_string = "test_data";
    let expected_sha1 = "cd5ea673af61c4defde7abf99ed84a0662c6e3ba";
    
    // Call sha1sum
    let args = vec![Rc::new(Object::String(test_string.to_string()))];
    let result = cursed::stdlib::cryptz::sha1sum(&args)?;
    
    // Verify the result
    match &*result {
        Object::String(hash) => {
            assert_eq!(hash, expected_sha1, "SHA1 hash doesn't match expected value");
            Ok(())
        },
        _ => Err(Error::Runtime("Expected string result from sha1sum".to_string())),
    }
}

#[test]
fn test_cryptz_sha256() -> Result<(), Error> {
    // Test data
    let test_string = "test_data";
    let expected_sha256 = "d5579c46dfcc7f18207013e65b44e4cb4e2c2298f4ac457ba8f82743f31e930b";
    
    // Call sha256sum
    let args = vec![Rc::new(Object::String(test_string.to_string()))];
    let result = cursed::stdlib::cryptz::sha256sum(&args)?;
    
    // Verify the result
    match &*result {
        Object::String(hash) => {
            assert_eq!(hash, expected_sha256, "SHA256 hash doesn't match expected value");
            Ok(())
        },
        _ => Err(Error::Runtime("Expected string result from sha256sum".to_string())),
    }
}

#[test]
fn test_cryptz_hmac_md5() -> Result<(), Error> {
    // Test data
    let test_string = "test_data";
    let test_key = "secret_key";
    let expected_hmac = "97d419c0d1d11bc7f1404d5be63ed651";
    
    // Call hmac with MD5
    let args = vec![
        Rc::new(Object::String(test_string.to_string())),
        Rc::new(Object::String(test_key.to_string())),
        Rc::new(Object::String("md5".to_string())),
    ];
    
    let result = cursed::stdlib::cryptz::hmac(&args)?;
    
    // Verify the result
    match &*result {
        Object::String(hmac) => {
            assert_eq!(hmac, expected_hmac, "HMAC-MD5 doesn't match expected value");
            Ok(())
        },
        _ => Err(Error::Runtime("Expected string result from hmac".to_string())),
    }
}

#[test]
fn test_cryptz_hmac_sha1() -> Result<(), Error> {
    // Test data
    let test_string = "test_data";
    let test_key = "secret_key";
    let expected_hmac = "dd394931df056f3182e659dd480a5b4ae7ac64e7";
    
    // Call hmac with SHA1
    let args = vec![
        Rc::new(Object::String(test_string.to_string())),
        Rc::new(Object::String(test_key.to_string())),
        Rc::new(Object::String("sha1".to_string())),
    ];
    
    let result = cursed::stdlib::cryptz::hmac(&args)?;
    
    // Verify the result
    match &*result {
        Object::String(hmac) => {
            assert_eq!(hmac, expected_hmac, "HMAC-SHA1 doesn't match expected value");
            Ok(())
        },
        _ => Err(Error::Runtime("Expected string result from hmac".to_string())),
    }
}

#[test]
fn test_cryptz_hmac_sha256() -> Result<(), Error> {
    // Test data
    let test_string = "test_data";
    let test_key = "secret_key";
    let expected_hmac = "67c9e8ef665d2d393fafb2a8d894d32e3423342e5a0449caf7e4cba0eec220ce";
    
    // Call hmac with SHA256
    let args = vec![
        Rc::new(Object::String(test_string.to_string())),
        Rc::new(Object::String(test_key.to_string())),
        Rc::new(Object::String("sha256".to_string())),
    ];
    
    let result = cursed::stdlib::cryptz::hmac(&args)?;
    
    // Verify the result
    match &*result {
        Object::String(hmac) => {
            assert_eq!(hmac, expected_hmac, "HMAC-SHA256 doesn't match expected value");
            Ok(())
        },
        _ => Err(Error::Runtime("Expected string result from hmac".to_string())),
    }
}

#[test]
fn test_cryptz_random_bytes() -> Result<(), Error> {
    // Test with requested length
    let length = 16; // 16 bytes
    
    // Call random_bytes
    let args = vec![Rc::new(Object::Integer(length))];
    let result = cursed::stdlib::cryptz::random_bytes(&args)?;
    
    // Verify the result
    match &*result {
        Object::Array(bytes) => {
            // Check that we got the requested number of bytes
            assert_eq!(bytes.len(), length as usize, "Random bytes count doesn't match requested length");
            
            // Check that each element is a byte (0-255)
            for byte in bytes {
                match byte {
                    Object::Integer(b) => {
                        assert!(*b >= 0 && *b <= 255, "Random byte out of range: {}", b);
                    },
                    _ => return Err(Error::Runtime("Expected integer in random bytes array".to_string())),
                }
            }
            
            Ok(())
        },
        _ => Err(Error::Runtime("Expected array result from random_bytes".to_string())),
    }
}

#[test]
fn test_cryptz_errors() -> Result<(), Error> {
    // Test missing arguments
    assert!(cursed::stdlib::cryptz::md5sum(&[]).is_err());
    assert!(cursed::stdlib::cryptz::sha1sum(&[]).is_err());
    assert!(cursed::stdlib::cryptz::sha256sum(&[]).is_err());
    assert!(cursed::stdlib::cryptz::hmac(&[]).is_err());
    assert!(cursed::stdlib::cryptz::hmac(&[Rc::new(Object::String("data".to_string()))]).is_err());
    assert!(cursed::stdlib::cryptz::random_bytes(&[]).is_err());
    
    // Test invalid arguments
    let invalid_args = vec![Rc::new(Object::Boolean(true))];
    assert!(cursed::stdlib::cryptz::md5sum(&invalid_args).is_err());
    
    // Test invalid algorithm for HMAC
    let invalid_algo_args = vec![
        Rc::new(Object::String("data".to_string())),
        Rc::new(Object::String("key".to_string())),
        Rc::new(Object::String("invalid_algo".to_string())),
    ];
    assert!(cursed::stdlib::cryptz::hmac(&invalid_algo_args).is_err());
    
    // Test negative length for random_bytes
    let negative_length = vec![Rc::new(Object::Integer(-10))];
    assert!(cursed::stdlib::cryptz::random_bytes(&negative_length).is_err());
    
    Ok(())
}