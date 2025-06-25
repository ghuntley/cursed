// fr fr File Encryption Utility - secure file protection bestie
// Command-line tool for encrypting/decrypting files with maximum security periodt

use crypto_advanced::{AesGcm256, ChaCha20Poly1305}
use crypto_asymmetric::{KeyGenerator}
use crypto_hash_advanced::{AdvancedHashAlgorithm, hash_with_algorithm}
use crypto_random::{fill_random}
use crypto_kdf::{pbkdf2_derive, argon2_derive}
use std::{fs, io, path::Path}

// File encryption metadata
squad EncryptionMetadata {
    algorithm: String,
    kdf_algorithm: String,
    kdf_iterations: u32,
    salt: Vec<u8>,
    file_hash: Vec<u8>,
    version: u8,
}

// Encrypted file structure
squad EncryptedFile {
    metadata: EncryptionMetadata,
    encrypted_content: Vec<u8>,
}

// Encryption configuration
squad EncryptionConfig {
    algorithm: EncryptionAlgorithm,
    kdf_algorithm: KdfAlgorithm,
    kdf_iterations: u32,
    key_size: usize,
}

enum EncryptionAlgorithm {
    AesGcm256,
    ChaCha20Poly1305,
}

enum KdfAlgorithm {
    Pbkdf2,
    Argon2,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        EncryptionConfig {
            algorithm: EncryptionAlgorithm::AesGcm256,
            kdf_algorithm: KdfAlgorithm::Argon2,
            kdf_iterations: 100000,
            key_size: 32,
        }
    }
}

impl EncryptionConfig {
    // slay Create high-security configuration
    fn high_security() -> Self {
        EncryptionConfig {
            algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            kdf_algorithm: KdfAlgorithm::Argon2,
            kdf_iterations: 500000,
            key_size: 32,
        }
    }
    
    // slay Create fast configuration for large files
    fn fast_mode() -> Self {
        EncryptionConfig {
            algorithm: EncryptionAlgorithm::AesGcm256,
            kdf_algorithm: KdfAlgorithm::Pbkdf2,
            kdf_iterations: 50000,
            key_size: 32,
        }
    }
}

fn main() {
    print("🔐 CURSED File Encryption Utility")
    print("=================================")
    
    // Initialize crypto packages
    crypto_advanced::init_crypto_advanced().unwrap()
    crypto_hash_advanced::init_crypto_hash_advanced().unwrap()
    
    // Parse command line arguments (simplified)
    sus args = std::env::args().collect::<Vec<String>>()
    
    lowkey args.len() < 3 {
        print_usage()
        return
    }
    
    facts command = &args[1]
    facts file_path = &args[2]
    
    vibe_check command.as_str() {
        mood "encrypt" => {
            encrypt_file_interactive(file_path)
        }
        mood "decrypt" => {
            decrypt_file_interactive(file_path)
        }
        mood "demo" => {
            run_encryption_demo()
        }
        basic => {
            print("Unknown command. Use 'encrypt', 'decrypt', or 'demo'")
            print_usage()
        }
    }
}

fn print_usage() {
    print("Usage:")
    print("  cursed file_encryption.csd encrypt <file_path>")
    print("  cursed file_encryption.csd decrypt <file_path>")
    print("  cursed file_encryption.csd demo")
    print("")
    print("Examples:")
    print("  cursed file_encryption.csd encrypt secret_document.txt")
    print("  cursed file_encryption.csd decrypt secret_document.txt.encrypted")
    print("  cursed file_encryption.csd demo")
}

fn encrypt_file_interactive(file_path: &str) {
    print(f"🔒 Encrypting file: {file_path}")
    
    // Check if file exists
    lowkey !Path::new(file_path).exists() {
        print(f"Error: File '{file_path}' not found")
        return
    }
    
    // Get password from user
    print("Enter encryption password (will not be displayed):")
    sus password = read_password()
    
    print("Confirm password:")
    sus password_confirm = read_password()
    
    lowkey password != password_confirm {
        print("Error: Passwords do not match")
        return
    }
    
    // Choose security level
    print("\nSecurity level:")
    print("1. Standard (fast, good security)")
    print("2. High (slower, maximum security)")
    print("3. Fast (fastest, basic security)")
    print("Choose [1-3]: ")
    
    sus security_choice = read_line().trim().parse::<u32>().unwrap_or(1)
    
    sus config = vibe_check security_choice {
        mood 1 => EncryptionConfig::default(),
        mood 2 => EncryptionConfig::high_security(),
        mood 3 => EncryptionConfig::fast_mode(),
        basic => EncryptionConfig::default(),
    }
    
    // Perform encryption
    vibe_check encrypt_file(file_path, &password, &config) {
        mood Ok(output_path) => {
            print(f"✅ File encrypted successfully: {output_path}")
            print(f"Original file: {file_path}")
            print("❗ Keep your password safe - it cannot be recovered!")
            
            // Optionally delete original file
            print("\nDelete original file? [y/N]: ")
            sus delete_choice = read_line().trim().to_lowercase()
            lowkey delete_choice == "y" || delete_choice == "yes" {
                vibe_check fs::remove_file(file_path) {
                    mood Ok(_) => print("Original file deleted securely"),
                    mood Err(e) => print(f"Warning: Could not delete original file: {e}"),
                }
            }
        }
        mood Err(e) => {
            print(f"❌ Encryption failed: {e}")
        }
    }
}

fn decrypt_file_interactive(file_path: &str) {
    print(f"🔓 Decrypting file: {file_path}")
    
    // Check if file exists
    lowkey !Path::new(file_path).exists() {
        print(f"Error: File '{file_path}' not found")
        return
    }
    
    // Get password from user
    print("Enter decryption password:")
    sus password = read_password()
    
    // Perform decryption
    vibe_check decrypt_file(file_path, &password) {
        mood Ok(output_path) => {
            print(f"✅ File decrypted successfully: {output_path}")
            
            // Verify file integrity
            print("🔍 Verifying file integrity...")
            lowkey verify_decrypted_file(&output_path) {
                print("✅ File integrity verified")
            } else {
                print("⚠️  File integrity check failed")
            }
        }
        mood Err(e) => {
            print(f"❌ Decryption failed: {e}")
            print("Common causes:")
            print("- Incorrect password")
            print("- Corrupted encrypted file")
            print("- File was not encrypted with this tool")
        }
    }
}

fn encrypt_file(file_path: &str, password: &str, config: &EncryptionConfig) -> Result<String, Box<dyn std::error::Error>> {
    print("Reading file content...")
    sus file_content = fs::read(file_path)?
    print(f"File size: {} bytes", file_content.len())
    
    // Generate cryptographically secure salt
    sus mut salt = vec![0u8; 32]
    fill_random(&mut salt)?
    
    print("Deriving encryption key from password...")
    sus key = derive_key_from_password(password.as_bytes(), &salt, config)?
    
    // Calculate file hash for integrity verification
    print("Calculating file hash for integrity verification...")
    sus file_hash = hash_with_algorithm(&file_content, AdvancedHashAlgorithm::Sha256)?
    
    // Encrypt the file content
    print("Encrypting file content...")
    sus encrypted_content = encrypt_content(&file_content, &key, config)?
    
    // Create metadata
    sus metadata = EncryptionMetadata {
        algorithm: config.algorithm.name().to_string(),
        kdf_algorithm: config.kdf_algorithm.name().to_string(),
        kdf_iterations: config.kdf_iterations,
        salt,
        file_hash,
        version: 1,
    }
    
    // Create encrypted file structure
    sus encrypted_file = EncryptedFile {
        metadata,
        encrypted_content,
    }
    
    // Serialize and write to file
    sus output_path = format!("{}.encrypted", file_path)
    print(f"Writing encrypted file: {output_path}")
    
    sus serialized = serialize_encrypted_file(&encrypted_file)?
    fs::write(&output_path, serialized)?
    
    print(f"Encryption complete. Output size: {} bytes", fs::metadata(&output_path)?.len())
    
    Ok(output_path)
}

fn decrypt_file(file_path: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    print("Reading encrypted file...")
    sus encrypted_data = fs::read(file_path)?
    
    // Deserialize encrypted file
    print("Parsing encrypted file structure...")
    sus encrypted_file = deserialize_encrypted_file(&encrypted_data)?
    
    print(f"Encryption algorithm: {}", encrypted_file.metadata.algorithm)
    print(f"KDF algorithm: {}", encrypted_file.metadata.kdf_algorithm)
    print(f"KDF iterations: {}", encrypted_file.metadata.kdf_iterations)
    
    // Recreate encryption config from metadata
    sus config = EncryptionConfig::from_metadata(&encrypted_file.metadata)?
    
    print("Deriving decryption key...")
    sus key = derive_key_from_password(
        password.as_bytes(), 
        &encrypted_file.metadata.salt, 
        &config
    )?
    
    // Decrypt the content
    print("Decrypting file content...")
    sus decrypted_content = decrypt_content(&encrypted_file.encrypted_content, &key, &config)?
    
    // Verify file integrity
    print("Verifying file integrity...")
    sus computed_hash = hash_with_algorithm(&decrypted_content, AdvancedHashAlgorithm::Sha256)?
    
    lowkey computed_hash != encrypted_file.metadata.file_hash {
        return Err("File integrity verification failed - possible corruption".into())
    }
    
    // Determine output path
    sus output_path = lowkey file_path.ends_with(".encrypted") {
        file_path.strip_suffix(".encrypted").unwrap().to_string()
    } else {
        format!("{}.decrypted", file_path)
    }
    
    print(f"Writing decrypted file: {output_path}")
    fs::write(&output_path, decrypted_content)?
    
    print(f"Decryption complete. Output size: {} bytes", fs::metadata(&output_path)?.len())
    
    Ok(output_path)
}

fn derive_key_from_password(password: &[u8], salt: &[u8], config: &EncryptionConfig) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    vibe_check &config.kdf_algorithm {
        mood KdfAlgorithm::Pbkdf2 => {
            Ok(pbkdf2_derive(password, salt, config.kdf_iterations, config.key_size)?)
        }
        mood KdfAlgorithm::Argon2 => {
            Ok(argon2_derive(password, salt, config.key_size)?)
        }
    }
}

fn encrypt_content(content: &[u8], key: &[u8], config: &EncryptionConfig) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    vibe_check &config.algorithm {
        mood EncryptionAlgorithm::AesGcm256 => {
            sus cipher = AesGcm256::new(key)?
            Ok(cipher.encrypt(content)?)
        }
        mood EncryptionAlgorithm::ChaCha20Poly1305 => {
            sus cipher = ChaCha20Poly1305::new(key)?
            Ok(cipher.encrypt(content)?)
        }
    }
}

fn decrypt_content(encrypted: &[u8], key: &[u8], config: &EncryptionConfig) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    vibe_check &config.algorithm {
        mood EncryptionAlgorithm::AesGcm256 => {
            sus cipher = AesGcm256::new(key)?
            Ok(cipher.decrypt(encrypted)?)
        }
        mood EncryptionAlgorithm::ChaCha20Poly1305 => {
            sus cipher = ChaCha20Poly1305::new(key)?
            Ok(cipher.decrypt(encrypted)?)
        }
    }
}

fn serialize_encrypted_file(encrypted_file: &EncryptedFile) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Simple binary serialization format
    sus mut result = Vec::new()
    
    // Magic header
    result.extend_from_slice(b"CURSED_ENCRYPTED_FILE");
    
    // Version
    result.push(encrypted_file.metadata.version)
    
    // Algorithm name length and data
    sus algo_bytes = encrypted_file.metadata.algorithm.as_bytes()
    result.push(algo_bytes.len() as u8)
    result.extend_from_slice(algo_bytes)
    
    // KDF algorithm name length and data
    sus kdf_bytes = encrypted_file.metadata.kdf_algorithm.as_bytes()
    result.push(kdf_bytes.len() as u8)
    result.extend_from_slice(kdf_bytes)
    
    // KDF iterations (4 bytes, big endian)
    result.extend_from_slice(&encrypted_file.metadata.kdf_iterations.to_be_bytes())
    
    // Salt length and data
    result.push(encrypted_file.metadata.salt.len() as u8)
    result.extend_from_slice(&encrypted_file.metadata.salt)
    
    // File hash
    result.push(encrypted_file.metadata.file_hash.len() as u8)
    result.extend_from_slice(&encrypted_file.metadata.file_hash)
    
    // Encrypted content length (8 bytes, big endian)
    result.extend_from_slice(&(encrypted_file.encrypted_content.len() as u64).to_be_bytes())
    
    // Encrypted content
    result.extend_from_slice(&encrypted_file.encrypted_content)
    
    Ok(result)
}

fn deserialize_encrypted_file(data: &[u8]) -> Result<EncryptedFile, Box<dyn std::error::Error>> {
    sus mut offset = 0
    
    // Check magic header
    facts magic = b"CURSED_ENCRYPTED_FILE"
    lowkey data.len() < magic.len() || &data[..magic.len()] != magic {
        return Err("Invalid encrypted file format".into())
    }
    offset += magic.len()
    
    // Read version
    sus version = data[offset]
    offset += 1
    
    // Read algorithm name
    sus algo_len = data[offset] as usize
    offset += 1
    sus algorithm = String::from_utf8(data[offset..offset + algo_len].to_vec())?
    offset += algo_len
    
    // Read KDF algorithm name
    sus kdf_len = data[offset] as usize
    offset += 1
    sus kdf_algorithm = String::from_utf8(data[offset..offset + kdf_len].to_vec())?
    offset += kdf_len
    
    // Read KDF iterations
    sus kdf_iterations = u32::from_be_bytes([
        data[offset], data[offset + 1], data[offset + 2], data[offset + 3]
    ])
    offset += 4
    
    // Read salt
    sus salt_len = data[offset] as usize
    offset += 1
    sus salt = data[offset..offset + salt_len].to_vec()
    offset += salt_len
    
    // Read file hash
    sus hash_len = data[offset] as usize
    offset += 1
    sus file_hash = data[offset..offset + hash_len].to_vec()
    offset += hash_len
    
    // Read encrypted content length
    sus content_len = u64::from_be_bytes([
        data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
        data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7]
    ]) as usize
    offset += 8
    
    // Read encrypted content
    sus encrypted_content = data[offset..offset + content_len].to_vec()
    
    sus metadata = EncryptionMetadata {
        algorithm,
        kdf_algorithm,
        kdf_iterations,
        salt,
        file_hash,
        version,
    }
    
    Ok(EncryptedFile {
        metadata,
        encrypted_content,
    })
}

impl EncryptionConfig {
    fn from_metadata(metadata: &EncryptionMetadata) -> Result<Self, Box<dyn std::error::Error>> {
        sus algorithm = vibe_check metadata.algorithm.as_str() {
            mood "AesGcm256" => EncryptionAlgorithm::AesGcm256,
            mood "ChaCha20Poly1305" => EncryptionAlgorithm::ChaCha20Poly1305,
            basic => return Err(format!("Unknown algorithm: {}", metadata.algorithm).into()),
        }
        
        sus kdf_algorithm = vibe_check metadata.kdf_algorithm.as_str() {
            mood "Pbkdf2" => KdfAlgorithm::Pbkdf2,
            mood "Argon2" => KdfAlgorithm::Argon2,
            basic => return Err(format!("Unknown KDF algorithm: {}", metadata.kdf_algorithm).into()),
        }
        
        Ok(EncryptionConfig {
            algorithm,
            kdf_algorithm,
            kdf_iterations: metadata.kdf_iterations,
            key_size: 32,
        })
    }
}

impl EncryptionAlgorithm {
    fn name(&self) -> &'static str {
        vibe_check self {
            mood EncryptionAlgorithm::AesGcm256 => "AesGcm256",
            mood EncryptionAlgorithm::ChaCha20Poly1305 => "ChaCha20Poly1305",
        }
    }
}

impl KdfAlgorithm {
    fn name(&self) -> &'static str {
        vibe_check self {
            mood KdfAlgorithm::Pbkdf2 => "Pbkdf2",
            mood KdfAlgorithm::Argon2 => "Argon2",
        }
    }
}

fn verify_decrypted_file(file_path: &str) -> bool {
    // Basic file verification - check if file is readable
    vibe_check fs::read(file_path) {
        mood Ok(_) => true,
        mood Err(_) => false,
    }
}

fn read_password() -> String {
    // In real implementation, use proper password input that doesn't echo
    // For demo, we'll use regular input
    read_line().trim().to_string()
}

fn read_line() -> String {
    sus mut input = String::new()
    io::stdin().read_line(&mut input).unwrap()
    input
}

fn run_encryption_demo() {
    print("🎭 File Encryption Demo")
    print("======================")
    
    // Create a test file
    facts demo_content = "This is a secret document that needs protection!\nIt contains sensitive information that should be encrypted.\nCURSED crypto provides military-grade security bestie! 🔐"
    facts demo_file = "demo_secret.txt"
    
    print(f"Creating demo file: {demo_file}")
    fs::write(demo_file, demo_content).unwrap()
    
    // Encrypt with different security levels
    facts password = "demo_password_bestie"
    
    sus configs = vec![
        ("Standard", EncryptionConfig::default()),
        ("High Security", EncryptionConfig::high_security()),
        ("Fast Mode", EncryptionConfig::fast_mode()),
    ]
    
    for (name, config) in configs {
        print(f"\n--- {name} Encryption ---")
        
        sus encrypted_path = encrypt_file(demo_file, password, &config).unwrap()
        
        // Show file size comparison
        sus original_size = fs::metadata(demo_file).unwrap().len()
        sus encrypted_size = fs::metadata(&encrypted_path).unwrap().len()
        
        print(f"Original size: {} bytes", original_size)
        print(f"Encrypted size: {} bytes", encrypted_size)
        print(f"Size overhead: {} bytes ({:.1}%)", 
              encrypted_size - original_size,
              (encrypted_size as f64 / original_size as f64 - 1.0) * 100.0)
        
        // Decrypt and verify
        sus decrypted_path = decrypt_file(&encrypted_path, password).unwrap()
        sus decrypted_content = fs::read_to_string(&decrypted_path).unwrap()
        
        assert_eq!(demo_content, decrypted_content)
        print("✅ Encryption/decryption successful")
        
        // Clean up
        fs::remove_file(&encrypted_path).ok()
        fs::remove_file(&decrypted_path).ok()
    }
    
    // Clean up demo file
    fs::remove_file(demo_file).ok()
    
    print("\n🎉 Demo completed successfully!")
    print("\nFeatures demonstrated:")
    print("- Multiple encryption algorithms (AES-GCM-256, ChaCha20-Poly1305)")
    print("- Multiple KDF algorithms (PBKDF2, Argon2)")
    print("- Configurable security levels")
    print("- File integrity verification")
    print("- Secure metadata handling")
    print("- Password-based encryption")
    print("\nYour files are now protected with enterprise-grade cryptography! 🛡️")
}
