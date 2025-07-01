use cursed::stdlib::packages::crypto_asymmetric::key_exchange::{x25519_generate_keypair, x448_generate_keypair, dh_generate_keypair};
use cursed::stdlib::packages::crypto_signatures::mod::{quick_ed25519_sign_verify, quick_ecdsa_sign_verify, quick_rsa_sign_verify};
use cursed::stdlib::database::orm::Repository;

fn main() {
    println!("Testing security fixes...");
    
    // Test crypto key generation (should return security errors)
    match x25519_generate_keypair(vec![1, 2, 3]) {
        Ok(_) => println!("ERROR: x25519_generate_keypair should not return Ok!"),
        Err(e) => println!("✅ x25519_generate_keypair correctly returns error: {:?}", e),
    }
    
    match x448_generate_keypair(vec![1, 2, 3]) {
        Ok(_) => println!("ERROR: x448_generate_keypair should not return Ok!"),
        Err(e) => println!("✅ x448_generate_keypair correctly returns error: {:?}", e),
    }
    
    match dh_generate_keypair(vec![1, 2, 3]) {
        Ok(_) => println!("ERROR: dh_generate_keypair should not return Ok!"),
        Err(e) => println!("✅ dh_generate_keypair correctly returns error: {:?}", e),
    }
    
    // Test signature verification (should return security errors)
    match quick_ed25519_sign_verify(b"test message") {
        Ok(true) => println!("ERROR: quick_ed25519_sign_verify should not return Ok(true)!"),
        Ok(false) => println!("ERROR: quick_ed25519_sign_verify should not return Ok(false)!"),
        Err(e) => println!("✅ quick_ed25519_sign_verify correctly returns error: {}", e),
    }
    
    match quick_ecdsa_sign_verify(b"test message", "secp256k1") {
        Ok(true) => println!("ERROR: quick_ecdsa_sign_verify should not return Ok(true)!"),
        Ok(false) => println!("ERROR: quick_ecdsa_sign_verify should not return Ok(false)!"),
        Err(e) => println!("✅ quick_ecdsa_sign_verify correctly returns error: {}", e),
    }
    
    match quick_rsa_sign_verify(b"test message", 2048, "PSS") {
        Ok(true) => println!("ERROR: quick_rsa_sign_verify should not return Ok(true)!"),
        Ok(false) => println!("ERROR: quick_rsa_sign_verify should not return Ok(false)!"),
        Err(e) => println!("✅ quick_rsa_sign_verify correctly returns error: {}", e),
    }
    
    println!("Security fixes verification completed!");
}
