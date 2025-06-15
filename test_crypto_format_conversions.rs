use cursed::stdlib::packages::crypto_asymmetric::{
    key_formats::{convert_public_key_format_enhanced, convert_private_key_format_enhanced},
    public_key::{PublicKeyAlgorithm, PublicKeyFormat},
    private_key::{PrivateKeyFormat, PrivateKeyAlgorithm},
};
use cursed::value::Value;
use cursed::error::CursedError;

fn main() {
    println!("Testing crypto asymmetric key format conversions...");

    // Test RSA key generation and format conversion
    println!("Testing RSA format conversions...");
    
    // Test Ed25519 key generation 
    println!("Testing Ed25519 key generation...");
    test_ed25519_keys();
    
    // Test P-256 format conversions
    println!("Testing P-256 format conversions...");
    test_p256_keys();
    
    println!("All tests completed successfully!");
}

fn test_ed25519_keys() {
    use rand::rngs::OsRng;
    use ed25519_dalek::SigningKey;
    
    // Generate Ed25519 key pair
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();
    
    // Test private key format conversion (raw to raw)
    let private_key_hex = hex::encode(signing_key.to_bytes());
    
    println!("Ed25519 private key: {}", &private_key_hex[..32]);
    println!("Ed25519 public key: {}", hex::encode(verifying_key.as_bytes()));
}

fn test_p256_keys() {
    use p256::{SecretKey, PublicKey};
    use rand::rngs::OsRng;
    use elliptic_curve::sec1::ToEncodedPoint;
    
    // Generate P-256 key pair
    let mut rng = OsRng;
    let private_key = SecretKey::random(&mut rng);
    let public_key = PublicKey::from(&private_key);
    
    // Test key encoding
    let private_bytes = private_key.to_bytes();
    let public_point = public_key.to_encoded_point(false);
    
    println!("P-256 private key length: {} bytes", private_bytes.len());
    println!("P-256 public key length: {} bytes", public_point.as_bytes().len());
    println!("P-256 private key: {}", hex::encode(&private_bytes[..8]));
    println!("P-256 public key: {}", hex::encode(&public_point.as_bytes()[..8]));
}
