#!/usr/bin/env cursed
fr fr/ fr fr Simple Hybrid Cryptography Example
fr fr/ 
fr fr/ This example shows the most common hybrid cryptography use cases:
fr fr/ - Secure key generation
fr fr/ - Message encryption/decryption  
fr fr/ - Digital signatures
fr fr/ - Secure messaging between parties

yeet "stdlib::crypto_pqc::hybrid_crypto"
yeet "stdlib::io"

sus main() {
    println("🔐 Simple Hybrid Cryptography Example")?;
    println("====================================\n")?;
    
    // Example 1: Basic encryption/decryption
    basic_encryption_example()?;
    
    // Example 2: Digital signatures
    digital_signature_example()?;
    
    // Example 3: Secure messaging
    secure_messaging_example()?;
    
    println("\n✨ All examples completed successfully!")?;
    println("🛡️  Your data is now quantum-resistant!")?;
}

fr fr/ Simple encryption and decryption example
fun basic_encryption_example() -> Result<(), CursedError> {
    println("1. Basic Hybrid Encryption 🔒")?;
    println("-----------------------------")?;
    
    // Generate a secure keypair (this uses X25519 + Kyber by default)
    sus keypair = generate_secure_keypair()?;
    println("✅ Generated quantum-resistant keypair")?;
    
    // Encrypt a message
    sus message = "Hello, post-quantum world! This message is safe from quantum computers.";
    println(f"Original: {message}")?;
    
    sus encrypted = hybrid_encrypt(message.as_bytes(), &keypair)?;
    println("✅ Message encrypted with hybrid cryptography")?;
    
    // Decrypt the message
    sus decrypted = hybrid_decrypt(&encrypted, &keypair)?;
    sus decrypted_text = String::from_utf8(decrypted)?;
    println(f"Decrypted: {decrypted_text}")?;
    
    println()?;
    Ok(())
}

fr fr/ Digital signature example
fun digital_signature_example() -> Result<(), CursedError> {
    println("2. Hybrid Digital Signatures ✍️")?;
    println("-------------------------------")?;
    
    // Generate a keypair for signing
    sus signer_keypair = generate_secure_keypair()?;
    println("✅ Generated signing keypair")?;
    
    // Sign a document
    sus document = "I agree to transfer 100 CURSED tokens to Bob";
    println(f"Document: {document}")?;
    
    sus signature = hybrid_sign(document.as_bytes(), &signer_keypair)?;
    println("✅ Document signed with quantum-resistant signature")?;
    
    // Verify the signature
    sus is_valid = hybrid_verify(document.as_bytes(), &signature, &signer_keypair)?;
    println(f"Signature valid: {is_valid}")?;
    
    // Test with a tampered document
    sus tampered = "I agree to transfer 1000 CURSED tokens to Bob";
    sus tampered_valid = hybrid_verify(tampered.as_bytes(), &signature, &signer_keypair)?;
    println(f"Tampered document valid: {tampered_valid}")?;
    
    println()?;
    Ok(())
}

fr fr/ Secure messaging between two parties
fun secure_messaging_example() -> Result<(), CursedError> {
    println("3. Secure Messaging 💬")?;
    println("---------------------")?;
    
    // Create messaging sessions for Alice and Bob
    sus mut alice = SecureMessagingSession::new(SecurityLevel::Level3)?;
    sus mut bob = SecureMessagingSession::new(SecurityLevel::Level3)?;
    println("✅ Created secure sessions for Alice and Bob")?;
    
    // Set up the communication channel
    alice.set_receiver(bob.sender_keypair.clone())?;
    bob.set_receiver(alice.sender_keypair.clone())?;
    println("✅ Established secure communication channel")?;
    
    // Alice sends a message to Bob
    sus alice_message = "Hi Bob! This message is encrypted and signed.";
    println(f"Alice sends: {alice_message}")?;
    
    sus secure_msg = alice.send_message(alice_message)?;
    println("✅ Message encrypted and signed")?;
    
    // Bob receives the message
    sus received = bob.receive_message(&secure_msg, &alice.sender_keypair)?;
    println(f"Bob receives: {received}")?;
    
    // Bob replies
    sus bob_reply = "Hi Alice! I received your message safely.";
    println(f"Bob replies: {bob_reply}")?;
    
    sus reply_msg = bob.send_message(bob_reply)?;
    sus alice_received = alice.receive_message(&reply_msg, &bob.sender_keypair)?;
    println(f"Alice receives: {alice_received}")?;
    
    println("✅ Secure two-way communication successful!")?;
    
    println()?;
    Ok(())
}
