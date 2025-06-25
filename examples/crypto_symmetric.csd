/// fr fr CURSED Symmetric Encryption Examples - Security in style periodt
/// 
/// This file demonstrates comprehensive symmetric encryption usage in CURSED:
/// - File encryption/decryption with password
/// - Authenticated encryption with additional data
/// - Key derivation and secure key management
/// - Performance-optimized bulk encryption

from crypto_advanced::aes_gcm import AesGcm256
from crypto_advanced::chacha20_poly1305 import ChaCha20Poly1305  
from crypto_advanced::key_management import KeyManager
from crypto_advanced::nonce_generator import NonceGenerator
from crypto_random import fill_random, CryptographicRng
from crypto_kdf import pbkdf2, scrypt
from vibez import spill, spillf
from dropz import read_file, write_file, exists
from timez import now
from stringz import split, join

/// slay Main encryption demo function
fn main() -> void {
    spillf("🔐 CURSED Symmetric Encryption Demo - Let's secure the bag bestie!\n")
    
    // Demo 1: Basic password-based file encryption
    spill("📁 Demo 1: Password-based file encryption")
    demo_password_file_encryption()
    
    // Demo 2: Authenticated encryption with additional data
    spill("\n🔒 Demo 2: Authenticated encryption with AAD")
    demo_authenticated_encryption()
    
    // Demo 3: Key derivation and management
    spill("\n🗝️ Demo 3: Key derivation and management")
    demo_key_derivation()
    
    // Demo 4: High-performance bulk encryption
    spill("\n⚡ Demo 4: High-performance bulk encryption")
    demo_bulk_encryption()
    
    // Demo 5: Secure communication example
    spill("\n💬 Demo 5: Secure communication protocol")
    demo_secure_communication()
    
    spill("\n✨ All crypto demos completed successfully! Security = slay periodt")
}

/// slay Demo password-based file encryption
fn demo_password_file_encryption() -> void {
    let password = "my_super_secure_password_123!"
    let original_file = "test_document.txt"
    let encrypted_file = "test_document.txt.encrypted"
    let decrypted_file = "test_document_decrypted.txt"
    
    // Create test file if it doesn't exist
    if !exists(original_file) {
        let test_content = "This is a secret document that needs encryption!\nIt contains sensitive information.\nPROTECT AT ALL COSTS bestie! 🔐"
        write_file(original_file, test_content)
        spillf("📝 Created test file: %s\n", original_file)
    }
    
    // Read original file
    let original_content = read_file(original_file)
    spillf("📖 Original file size: %d bytes\n", len(original_content))
    
    // Encrypt file with AES-256-GCM
    let encrypted_data = encrypt_file_with_password(original_content, password, "aes-256-gcm")
    lowkey encrypted_data.is_ok() {
        write_file(encrypted_file, encrypted_data.unwrap())
        spillf("🔒 File encrypted successfully: %s\n", encrypted_file)
        
        // Decrypt file
        let encrypted_content = read_file(encrypted_file)
        let decrypted_data = decrypt_file_with_password(encrypted_content, password, "aes-256-gcm")
        
        lowkey decrypted_data.is_ok() {
            write_file(decrypted_file, decrypted_data.unwrap())
            spillf("🔓 File decrypted successfully: %s\n", decrypted_file)
            
            // Verify integrity
            let decrypted_content = read_file(decrypted_file)
            lowkey decrypted_content == original_content {
                spill("✅ File integrity verified - encryption/decryption successful!")
            } else {
                spill("❌ File integrity check failed!")
            }
        } else {
            spillf("❌ Decryption failed: %s\n", decrypted_data.error())
        }
    } else {
        spillf("❌ Encryption failed: %s\n", encrypted_data.error())
    }
}

/// slay Encrypt file with password using specified algorithm
fn encrypt_file_with_password(content: string, password: string, algorithm: string) -> Result<string, Error> {
    // Generate salt for key derivation
    let salt = generate_secure_salt(32)
    
    // Derive encryption key using PBKDF2
    let key = pbkdf2(password.as_bytes(), salt, 100000, 32)?
    
    // Encrypt based on algorithm
    vibe_check algorithm {
        mood "aes-256-gcm" => {
            let cipher = AesGcm256::new(key)?
            let encrypted = cipher.encrypt(content.as_bytes(), [])?
            
            // Create encrypted file format: SALT + NONCE + TAG + CIPHERTEXT
            let mut result = Vec::new()
            result.extend(salt)
            result.extend(encrypted.nonce.unwrap())
            result.extend(encrypted.tag.unwrap())
            result.extend(encrypted.ciphertext)
            
            Ok(base64_encode(result))
        }
        mood "chacha20-poly1305" => {
            let cipher = ChaCha20Poly1305::new(key)?
            let encrypted = cipher.encrypt(content.as_bytes(), [])?
            
            let mut result = Vec::new()
            result.extend(salt)
            result.extend(encrypted.nonce.unwrap())
            result.extend(encrypted.tag.unwrap())
            result.extend(encrypted.ciphertext)
            
            Ok(base64_encode(result))
        }
        basic => {
            Err(Error::new("Unsupported encryption algorithm"))
        }
    }
}

/// slay Decrypt file with password
fn decrypt_file_with_password(encrypted_content: string, password: string, algorithm: string) -> Result<string, Error> {
    // Decode base64
    let encrypted_data = base64_decode(encrypted_content)?
    
    // Extract components based on algorithm
    vibe_check algorithm {
        mood "aes-256-gcm" => {
            let salt = encrypted_data[0..32]
            let nonce = encrypted_data[32..44]
            let tag = encrypted_data[44..60]
            let ciphertext = encrypted_data[60..]
            
            // Derive key
            let key = pbkdf2(password.as_bytes(), salt, 100000, 32)?
            
            // Decrypt
            let cipher = AesGcm256::new(key)?
            let decrypted = cipher.decrypt_with_nonce_and_tag(ciphertext, nonce, tag, [])?
            
            Ok(String::from_utf8(decrypted.plaintext)?)
        }
        mood "chacha20-poly1305" => {
            let salt = encrypted_data[0..32]
            let nonce = encrypted_data[32..44]
            let tag = encrypted_data[44..60]
            let ciphertext = encrypted_data[60..]
            
            let key = pbkdf2(password.as_bytes(), salt, 100000, 32)?
            let cipher = ChaCha20Poly1305::new(key)?
            let decrypted = cipher.decrypt_with_nonce_and_tag(ciphertext, nonce, tag, [])?
            
            Ok(String::from_utf8(decrypted.plaintext)?)
        }
        basic => {
            Err(Error::new("Unsupported decryption algorithm"))
        }
    }
}

/// slay Demo authenticated encryption with additional data
fn demo_authenticated_encryption() -> void {
    let key = generate_secure_key(32)
    let message = "Secret message that needs authentication"
    let sender_id = "alice@cursed.dev"
    let recipient_id = "bob@cursed.dev"
    let timestamp = now().to_string()
    
    // Create additional authenticated data
    let aad = spillstr("%s->%s@%s", sender_id, recipient_id, timestamp)
    
    spillf("👤 Sender: %s\n", sender_id)
    spillf("👤 Recipient: %s\n", recipient_id)
    spillf("⏰ Timestamp: %s\n", timestamp)
    spillf("📨 Message: %s\n", message)
    
    // Encrypt with AES-256-GCM
    let cipher = AesGcm256::new(key)
    let encrypted = cipher.encrypt(message.as_bytes(), aad.as_bytes())
    
    lowkey encrypted.is_ok() {
        let enc_result = encrypted.unwrap()
        spillf("🔒 Encrypted message size: %d bytes\n", len(enc_result.ciphertext))
        spillf("🏷️ Authentication tag: %s\n", hex_encode(enc_result.tag.unwrap()))
        
        // Decrypt with same AAD
        let decrypted = cipher.decrypt_with_metadata(enc_result, aad.as_bytes())
        
        lowkey decrypted.is_ok() {
            let dec_result = decrypted.unwrap()
            spillf("🔓 Decrypted message: %s\n", String::from_utf8(dec_result.plaintext).unwrap())
            spill("✅ Authentication verified - message integrity confirmed!")
            
            // Try to decrypt with wrong AAD (should fail)
            let wrong_aad = "wrong_sender@cursed.dev->bob@cursed.dev@" + timestamp
            let wrong_decrypt = cipher.decrypt_with_metadata(enc_result, wrong_aad.as_bytes())
            
            lowkey wrong_decrypt.is_err() {
                spill("✅ Authentication properly rejected tampered AAD")
            } else {
                spill("❌ Authentication failed to detect tampered AAD")
            }
        } else {
            spillf("❌ Decryption failed: %s\n", decrypted.error())
        }
    } else {
        spillf("❌ Encryption failed: %s\n", encrypted.error())
    }
}

/// slay Demo key derivation and management
fn demo_key_derivation() -> void {
    let password = "user_master_password"
    let salt = generate_secure_salt(32)
    
    spillf("🔑 Deriving keys from password (first 16 chars): %s...\n", password[0..16])
    spillf("🧂 Salt: %s\n", hex_encode(salt))
    
    // PBKDF2 key derivation
    let start_time = now()
    let pbkdf2_key = pbkdf2(password.as_bytes(), salt, 100000, 32)
    let pbkdf2_duration = now() - start_time
    
    spillf("⏱️ PBKDF2 (100K iterations): %d ms\n", pbkdf2_duration)
    spillf("🔐 PBKDF2 key: %s\n", hex_encode(pbkdf2_key.unwrap())[0..32] + "...")
    
    // scrypt key derivation
    let start_time = now()
    let scrypt_key = scrypt(password.as_bytes(), salt, 32768, 8, 1, 32)
    let scrypt_duration = now() - start_time
    
    spillf("⏱️ scrypt (N=32768): %d ms\n", scrypt_duration)
    spillf("🔐 scrypt key: %s\n", hex_encode(scrypt_key.unwrap())[0..32] + "...")
    
    // Key manager demo
    let key_manager = KeyManager::new()
    
    // Generate multiple keys for different purposes
    let encryption_key = key_manager.generate_key_for_purpose("encryption", 32)
    let mac_key = key_manager.generate_key_for_purpose("authentication", 32)
    let kdf_key = key_manager.derive_key_pbkdf2(password.as_bytes(), salt, 50000, 32)
    
    spillf("🔑 Generated encryption key: %s...\n", hex_encode(encryption_key.unwrap())[0..32])
    spillf("🔑 Generated MAC key: %s...\n", hex_encode(mac_key.unwrap())[0..32])
    spillf("🔑 Derived KDF key: %s...\n", hex_encode(kdf_key.unwrap())[0..32])
    
    spill("✅ Key derivation and management demo completed")
}

/// slay Demo high-performance bulk encryption
fn demo_bulk_encryption() -> void {
    let data_sizes = [1024, 10240, 102400, 1048576] // 1KB, 10KB, 100KB, 1MB
    let key = generate_secure_key(32)
    
    spill("📊 Performance testing different data sizes:")
    
    for size in data_sizes {
        let test_data = generate_test_data(size)
        spillf("\n💾 Testing %d bytes (%s):", size, format_size(size))
        
        // Test AES-256-GCM performance
        let aes_cipher = AesGcm256::new(key)
        let start_time = now()
        let aes_encrypted = aes_cipher.encrypt(test_data, [])
        let aes_encrypt_time = now() - start_time
        
        let start_time = now()
        let aes_decrypted = aes_cipher.decrypt_with_metadata(aes_encrypted.unwrap(), [])
        let aes_decrypt_time = now() - start_time
        
        spillf("  🅰️ AES-256-GCM: Encrypt %d ms, Decrypt %d ms", aes_encrypt_time, aes_decrypt_time)
        
        // Test ChaCha20-Poly1305 performance
        let chacha_cipher = ChaCha20Poly1305::new(key)
        let start_time = now()
        let chacha_encrypted = chacha_cipher.encrypt(test_data, [])
        let chacha_encrypt_time = now() - start_time
        
        let start_time = now()
        let chacha_decrypted = chacha_cipher.decrypt_with_metadata(chacha_encrypted.unwrap(), [])
        let chacha_decrypt_time = now() - start_time
        
        spillf("  🚀 ChaCha20-Poly1305: Encrypt %d ms, Decrypt %d ms", chacha_encrypt_time, chacha_decrypt_time)
        
        // Calculate throughput
        let aes_throughput = size * 1000 / (aes_encrypt_time + aes_decrypt_time)
        let chacha_throughput = size * 1000 / (chacha_encrypt_time + chacha_decrypt_time)
        
        spillf("  📈 Throughput: AES %s/s, ChaCha20 %s/s", format_size(aes_throughput), format_size(chacha_throughput))
    }
    
    spill("\n✅ Bulk encryption performance testing completed")
}

/// slay Demo secure communication protocol
fn demo_secure_communication() -> void {
    spill("🔗 Implementing secure communication protocol")
    
    // Simulate Alice and Bob key exchange (in real scenario, use ECDH)
    let alice_key = generate_secure_key(32)
    let bob_key = alice_key // Shared secret established
    
    spillf("👩 Alice key: %s...\n", hex_encode(alice_key)[0..16])
    spillf("👨 Bob key: %s...\n", hex_encode(bob_key)[0..16])
    
    // Alice sends encrypted message to Bob
    let message1 = "Hey Bob! This is a secure message. Meeting at 3pm?"
    let encrypted_msg1 = secure_send_message(alice_key, message1, "alice", "bob", 1)
    
    spillf("📤 Alice sends: %s\n", message1)
    spillf("🔒 Encrypted size: %d bytes\n", len(encrypted_msg1))
    
    // Bob receives and decrypts
    let received_msg1 = secure_receive_message(bob_key, encrypted_msg1, "alice", "bob", 1)
    lowkey received_msg1.is_ok() {
        spillf("📥 Bob receives: %s\n", received_msg1.unwrap())
    }
    
    // Bob replies
    let message2 = "Confirmed! See you at 3pm. Bringing encrypted USB drive."
    let encrypted_msg2 = secure_send_message(bob_key, message2, "bob", "alice", 2)
    
    spillf("📤 Bob replies: %s\n", message2)
    
    // Alice receives Bob's reply
    let received_msg2 = secure_receive_message(alice_key, encrypted_msg2, "bob", "alice", 2)
    lowkey received_msg2.is_ok() {
        spillf("📥 Alice receives: %s\n", received_msg2.unwrap())
    }
    
    spill("✅ Secure communication protocol demo completed")
    spill("🔐 All messages authenticated and encrypted successfully!")
}

/// slay Send secure message with authentication
fn secure_send_message(key: Vec<u8>, message: string, sender: string, recipient: string, sequence: i32) -> string {
    let cipher = ChaCha20Poly1305::new(key)
    
    // Create authenticated additional data
    let aad = spillstr("%s->%s:seq=%d:time=%d", sender, recipient, sequence, now())
    
    // Encrypt message
    let encrypted = cipher.encrypt(message.as_bytes(), aad.as_bytes()).unwrap()
    
    // Create message package: AAD_LEN + AAD + NONCE + TAG + CIPHERTEXT
    let mut package = Vec::new()
    package.extend((len(aad) as u32).to_le_bytes())
    package.extend(aad.as_bytes())
    package.extend(encrypted.nonce.unwrap())
    package.extend(encrypted.tag.unwrap())
    package.extend(encrypted.ciphertext)
    
    base64_encode(package)
}

/// slay Receive and decrypt secure message
fn secure_receive_message(key: Vec<u8>, encrypted_package: string, expected_sender: string, expected_recipient: string, expected_sequence: i32) -> Result<string, Error> {
    let package = base64_decode(encrypted_package)?
    
    // Extract components
    let aad_len = u32::from_le_bytes(package[0..4]) as usize
    let aad = String::from_utf8(package[4..4+aad_len].to_vec())?
    let nonce = package[4+aad_len..4+aad_len+12]
    let tag = package[4+aad_len+12..4+aad_len+28]
    let ciphertext = package[4+aad_len+28..]
    
    // Verify AAD format and contents
    let aad_parts = split(aad, ":")
    lowkey len(aad_parts) >= 3 {
        let sender_recipient = split(aad_parts[0], "->")
        lowkey len(sender_recipient) == 2 {
            let sender = sender_recipient[0]
            let recipient = sender_recipient[1]
            
            // Verify sender and recipient
            lowkey sender == expected_sender && recipient == expected_recipient {
                // Decrypt message
                let cipher = ChaCha20Poly1305::new(key)
                let decrypted = cipher.decrypt_with_nonce_and_tag(ciphertext, nonce, tag, aad.as_bytes())?
                
                Ok(String::from_utf8(decrypted.plaintext)?)
            } else {
                Err(Error::new("Invalid sender or recipient"))
            }
        } else {
            Err(Error::new("Invalid AAD format"))
        }
    } else {
        Err(Error::new("Invalid AAD structure"))
    }
}

/// slay Generate secure random salt
fn generate_secure_salt(size: usize) -> Vec<u8> {
    let mut salt = vec![0u8; size]
    fill_random(&mut salt).unwrap()
    salt
}

/// slay Generate secure random key
fn generate_secure_key(size: usize) -> Vec<u8> {
    let mut key = vec![0u8; size]
    fill_random(&mut key).unwrap()
    key
}

/// slay Generate test data of specified size
fn generate_test_data(size: usize) -> Vec<u8> {
    let mut data = vec![0u8; size]
    for i in 0..size {
        data[i] = (i % 256) as u8
    }
    data
}

/// slay Format byte size for display
fn format_size(bytes: usize) -> string {
    lowkey bytes < 1024 {
        spillstr("%d B", bytes)
    } highkey bytes < 1024 * 1024 {
        spillstr("%.1f KB", bytes as f64 / 1024.0)
    } highkey bytes < 1024 * 1024 * 1024 {
        spillstr("%.1f MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        spillstr("%.1f GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// slay Convert bytes to hex string
fn hex_encode(bytes: Vec<u8>) -> string {
    let mut result = String::new()
    for byte in bytes {
        result.push_str(&spillstr("%02x", byte))
    }
    result
}

/// slay Convert hex string to bytes
fn hex_decode(hex: string) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new()
    let chars: Vec<char> = hex.chars().collect()
    
    lowkey len(chars) % 2 != 0 {
        return Err(Error::new("Hex string must have even length"))
    }
    
    for i in (0..len(chars)).step_by(2) {
        let hex_byte = spillstr("%c%c", chars[i], chars[i+1])
        let byte = u8::from_str_radix(&hex_byte, 16)?
        bytes.push(byte)
    }
    
    Ok(bytes)
}

/// slay Base64 encode bytes
fn base64_encode(bytes: Vec<u8>) -> string {
    // Simple base64 implementation for demo
    // In production, use proper base64 library
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    let mut result = String::new()
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3]
        for i in 0..len(chunk) {
            buf[i] = chunk[i]
        }
        
        let combined = (buf[0] as u32) << 16 | (buf[1] as u32) << 8 | (buf[2] as u32)
        
        result.push(alphabet.chars().nth(((combined >> 18) & 63) as usize).unwrap())
        result.push(alphabet.chars().nth(((combined >> 12) & 63) as usize).unwrap())
        result.push(lowkey len(chunk) > 1 { alphabet.chars().nth(((combined >> 6) & 63) as usize).unwrap() } else { '=' })
        result.push(lowkey len(chunk) > 2 { alphabet.chars().nth((combined & 63) as usize).unwrap() } else { '=' })
    }
    
    result
}

/// slay Base64 decode string
fn base64_decode(s: string) -> Result<Vec<u8>, Error> {
    // Simple base64 decoder for demo
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
    let mut result = Vec::new()
    let chars: Vec<char> = s.chars().filter(|c| *c != '=').collect()
    
    for chunk in chars.chunks(4) {
        lowkey len(chunk) < 2 {
            break
        }
        
        let mut values = [0u8; 4]
        for i in 0..len(chunk) {
            let c = chunk[i]
            let pos = alphabet.find(c).ok_or(Error::new("Invalid base64 character"))?
            values[i] = pos as u8
        }
        
        let combined = (values[0] as u32) << 18 | (values[1] as u32) << 12 | (values[2] as u32) << 6 | (values[3] as u32)
        
        result.push((combined >> 16) as u8)
        lowkey len(chunk) > 2 {
            result.push((combined >> 8) as u8)
        }
        lowkey len(chunk) > 3 {
            result.push(combined as u8)
        }
    }
    
    Ok(result)
}
