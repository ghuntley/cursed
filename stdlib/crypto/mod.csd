// Standard cryptography library

// Hash functions
fn sha256(data: string) -> string {
    return crypto_sha256(data);
}

fn sha512(data: string) -> string {
    return crypto_sha512(data);
}

// MD5 REMOVED - SECURITY VULNERABILITY
// MD5 is cryptographically broken and vulnerable to collision attacks
// Use sha256() or blake3() instead for secure hashing

fn blake3(data: string) -> string {
    return crypto_blake3(data);
}

fn sha3_256(data: string) -> string {
    return crypto_sha3_256(data);
}

// Random number generation (using cryptographically secure OS CSPRNG)
fn random_bytes(length: int) -> array {
    return crypto_secure_random_bytes(length);
}

fn random_int(min: int, max: int) -> int {
    return crypto_secure_random_int(min, max);
}

fn random_string(length: int) -> string {
    return crypto_secure_random_string(length);
}

// Base encoding/decoding
fn base64_encode(data: string) -> string {
    return crypto_base64_encode(data);
}

fn base64_decode(encoded: string) -> string {
    return crypto_base64_decode(encoded);
}

fn hex_encode(data: array) -> string {
    return crypto_hex_encode(data);
}

fn hex_decode(hex: string) -> array {
    return crypto_hex_decode(hex);
}

// Symmetric encryption (AES-GCM authenticated encryption)
fn aes_gcm_encrypt(data: string, key: string) -> string {
    return crypto_aes_gcm_encrypt(data, key);
}

fn aes_gcm_decrypt(encrypted: string, key: string) -> string {
    return crypto_aes_gcm_decrypt(encrypted, key);
}

// WARNING: Legacy AES functions deprecated - use AES-GCM instead
// These functions may use insecure modes and are not recommended
fn aes_encrypt(data: string, key: string) -> string {
    return crypto_aes_encrypt(data, key);
}

fn aes_decrypt(encrypted: string, key: string) -> string {
    return crypto_aes_decrypt(encrypted, key);
}

// Key derivation
fn pbkdf2(password: string, salt: string, iterations: int, length: int) -> string {
    return crypto_pbkdf2(password, salt, iterations, length);
}

fn scrypt(password: string, salt: string, n: int, r: int, p: int, length: int) -> string {
    return crypto_scrypt(password, salt, n, r, p, length);
}

// Digital signatures (Ed25519)
fn ed25519_keypair() -> map {
    return crypto_ed25519_keypair();
}

fn ed25519_sign(message: string, private_key: string) -> string {
    return crypto_ed25519_sign(message, private_key);
}

fn ed25519_verify(message: string, signature: string, public_key: string) -> bool {
    return crypto_ed25519_verify(message, signature, public_key);
}

// HMAC
fn hmac_sha256(data: string, key: string) -> string {
    return crypto_hmac_sha256(data, key);
}

fn hmac_sha512(data: string, key: string) -> string {
    return crypto_hmac_sha512(data, key);
}

// Password hashing
fn argon2_hash(password: string, salt: string) -> string {
    return crypto_argon2_hash(password, salt);
}

fn argon2_verify(password: string, hash: string) -> bool {
    return crypto_argon2_verify(password, hash);
}

fn bcrypt_hash(password: string, cost: int) -> string {
    return crypto_bcrypt_hash(password, cost);
}

fn bcrypt_verify(password: string, hash: string) -> bool {
    return crypto_bcrypt_verify(password, hash);
}

// Constant-time comparison
fn constant_time_eq(a: string, b: string) -> bool {
    return crypto_constant_time_eq(a, b);
}

// Utilities
fn secure_random() -> float {
    return crypto_secure_random();
}

fn generate_salt(length: int) -> string {
    return crypto_generate_salt(length);
}
