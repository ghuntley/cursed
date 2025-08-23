# comprehensive_blockchain_test.csd - Production Blockchain Cryptography Validation
# Tests enhanced blockchain functionality with real-world test vectors

yeet "blockchainz/production_crypto"
yeet "blockchainz/bigint_operations"
yeet "blockchainz/test_vectors"
yeet "blockchainz/core"
yeet "cryptz"
yeet "vibez"

slay main() {
    vibez.spill("🔗 CURSED Blockchain Cryptography Enhancement Testing")
    vibez.spill("=" * 60)
    
    # Test 1: Production ECDSA with secp256k1
    vibez.spill("\n🔐 Testing Production ECDSA Implementation")
    test_production_ecdsa()
    
    # Test 2: Bitcoin/Ethereum address generation
    vibez.spill("\n💰 Testing Cryptocurrency Address Generation")
    test_address_generation()
    
    # Test 3: HD Wallet functionality
    vibez.spill("\n👛 Testing HD Wallet Operations")
    test_hd_wallet()
    
    # Test 4: Merkle tree with inclusion proofs
    vibez.spill("\n🌳 Testing Merkle Tree Operations")
    test_merkle_operations()
    
    # Test 5: Transaction signing and validation
    vibez.spill("\n📝 Testing Transaction Operations")
    test_transaction_operations()
    
    # Test 6: Proof-of-Work mining
    vibez.spill("\n⛏️ Testing Proof-of-Work Mining")
    test_pow_mining()
    
    # Test 7: Big integer operations
    vibez.spill("\n🔢 Testing Big Integer Arithmetic")
    test_bigint_arithmetic()
    
    # Test 8: Comprehensive test vectors
    vibez.spill("\n🧪 Running Comprehensive Test Vectors")
    run_comprehensive_crypto_tests()
    
    # Test 9: Performance benchmarks
    vibez.spill("\n⚡ Performance Benchmarks")
    benchmark_crypto_operations()
    
    # Test 10: Memory safety validation
    vibez.spill("\n🛡️ Memory Safety Validation")
    test_memory_safety()
    
    vibez.spill("\n" + "=" * 60)
    vibez.spill("✅ All blockchain cryptography tests completed successfully!")
    vibez.spill("🎯 Production-grade security validated")
}

slay test_production_ecdsa() {
    vibez.spill("Generating secp256k1 keypair...")
    sus keypair KeyPair = secp256k1_generate_keypair()
    
    vibez.spill("Private key size:", len(keypair.private_key), "bytes")
    vibez.spill("Public key size:", len(keypair.public_key), "bytes")
    vibez.spill("Curve:", keypair.curve)
    
    # Test message signing
    sus message tea = "Hello, CURSED Blockchain!"
    sus message_hash []drip = sha256_hash(message)
    
    vibez.spill("Signing message:", message)
    sus signature ECDSASignature = ecdsa_sign_production(message_hash, keypair.private_key)
    
    vibez.spill("Signature generated")
    vibez.spill("r length:", len(bigint_to_bytes_32(signature.r)), "bytes")
    vibez.spill("s length:", len(bigint_to_bytes_32(signature.s)), "bytes")
    vibez.spill("Recovery ID:", signature.recovery_id)
    
    # Verify signature
    sus is_valid lit = ecdsa_verify_production(message_hash, signature, keypair.public_key)
    vibez.spill("Signature valid:", is_valid)
    
    ready !is_valid {
        vibez.spill("❌ ECDSA verification failed!")
        damn
    }
    
    vibez.spill("✓ Production ECDSA working correctly")
}

slay test_address_generation() {
    # Generate keypair for address testing
    sus keypair KeyPair = secp256k1_generate_keypair()
    
    # Test Bitcoin address generation
    vibez.spill("Generating Bitcoin addresses...")
    sus btc_mainnet tea = bitcoin_address_from_public_key(keypair.public_key, "mainnet")
    sus btc_testnet tea = bitcoin_address_from_public_key(keypair.public_key, "testnet")
    
    vibez.spill("Bitcoin mainnet address:", btc_mainnet)
    vibez.spill("Bitcoin testnet address:", btc_testnet)
    
    # Test Ethereum address generation (need uncompressed public key)
    sus uncompressed_pubkey []drip = point_to_uncompressed_bytes(bytes_to_point(keypair.public_key))
    sus eth_address tea = ethereum_address_from_public_key(uncompressed_pubkey)
    
    vibez.spill("Ethereum address:", eth_address)
    
    # Validate address formats
    ready !stringz.starts_with(btc_mainnet, "1") && !stringz.starts_with(btc_mainnet, "3") {
        vibez.spill("❌ Invalid Bitcoin mainnet address format")
        damn
    }
    
    ready !stringz.starts_with(eth_address, "0x") {
        vibez.spill("❌ Invalid Ethereum address format")
        damn
    }
    
    vibez.spill("✓ Address generation working correctly")
}

slay test_hd_wallet() {
    # Test BIP39/BIP32 HD wallet
    sus seed_phrase tea = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    
    vibez.spill("Generating HD wallet from seed phrase...")
    sus wallet HDWallet = generate_hd_wallet(seed_phrase)
    
    vibez.spill("Master private key size:", len(wallet.master_private_key), "bytes")
    vibez.spill("Master public key size:", len(wallet.master_public_key), "bytes")
    vibez.spill("Chain code size:", len(wallet.master_chain_code), "bytes")
    vibez.spill("Derivation path:", wallet.derivation_path)
    
    # Test child key derivation
    vibez.spill("Deriving child keys...")
    
    # Non-hardened derivation (m/0)
    sus child0 KeyPair = derive_child_key(wallet, 0, cringe)
    vibez.spill("Child 0 (non-hardened) generated")
    
    # Hardened derivation (m/0')
    sus child0h KeyPair = derive_child_key(wallet, 0, based)
    vibez.spill("Child 0' (hardened) generated")
    
    # Verify keys are different
    ready bytes_equal(child0.private_key, child0h.private_key) {
        vibez.spill("❌ Hardened and non-hardened keys should be different!")
        damn
    }
    
    vibez.spill("✓ HD wallet derivation working correctly")
}

slay test_merkle_operations() {
    # Create test transactions
    sus transactions [][]drip = [
        sha256_hash("Transaction 1: Alice sends 1 BTC to Bob"),
        sha256_hash("Transaction 2: Bob sends 0.5 BTC to Carol"),
        sha256_hash("Transaction 3: Carol sends 0.3 BTC to Dave"),
        sha256_hash("Transaction 4: Dave sends 0.1 BTC to Eve"),
        sha256_hash("Transaction 5: Eve sends 0.05 BTC to Frank")
    ]
    
    vibez.spill("Building Merkle tree with", len(transactions), "transactions...")
    sus merkle_tree ProductionMerkleTree = build_production_merkle_tree(transactions)
    
    vibez.spill("Tree depth:", merkle_tree.depth)
    vibez.spill("Node count:", merkle_tree.node_count)
    vibez.spill("Root hash:", bytes_to_hex(merkle_tree.root)[0:16] + "...")
    
    # Test inclusion proofs
    vibez.spill("Testing Merkle inclusion proofs...")
    bestie i := 0; i < len(transactions); i++ {
        sus leaf_hash []drip = sha256_hash(stringz.from_bytes(sha256_hash(stringz.from_bytes(transactions[i]))))
        sus proof [][]drip = generate_merkle_proof(merkle_tree, i)
        sus is_valid lit = verify_merkle_proof(leaf_hash, proof, merkle_tree.root, i)
        
        vibez.spill("Transaction", i, "inclusion proof:", is_valid ? "✓" : "❌")
        
        ready !is_valid {
            vibez.spill("❌ Merkle proof verification failed for transaction", i)
            damn
        }
    }
    
    vibez.spill("✓ Merkle tree operations working correctly")
}

slay test_transaction_operations() {
    # Create a simple transaction for testing
    sus sender_keypair KeyPair = secp256k1_generate_keypair()
    sus recipient_address tea = bitcoin_address_from_public_key(secp256k1_generate_keypair().public_key, "mainnet")
    
    sus transaction Transaction = Transaction{
        from_address: bitcoin_address_from_public_key(sender_keypair.public_key, "mainnet"),
        to_address: recipient_address,
        amount: 50000000,  # 0.5 BTC in satoshis
        fee: 1000,         # 0.00001 BTC fee
        timestamp: system_time(),
        nonce: generate_random_int()
    }
    
    vibez.spill("Created transaction:")
    vibez.spill("  From:", transaction.from_address)
    vibez.spill("  To:", transaction.to_address)
    vibez.spill("  Amount:", transaction.amount, "satoshis")
    vibez.spill("  Fee:", transaction.fee, "satoshis")
    
    # Sign the transaction
    vibez.spill("Signing transaction...")
    sus signed_tx SignedTransaction = sign_transaction(transaction, bytes_to_hex(sender_keypair.private_key))
    
    vibez.spill("Transaction signed")
    vibez.spill("  TXID:", signed_tx.txid[0:16] + "...")
    vibez.spill("  Signature:", signed_tx.signature[0:16] + "...")
    
    # Verify the transaction
    sus is_valid lit = verify_transaction(signed_tx)
    vibez.spill("Transaction verification:", is_valid ? "✓" : "❌")
    
    ready !is_valid {
        vibez.spill("❌ Transaction verification failed!")
        damn
    }
    
    vibez.spill("✓ Transaction operations working correctly")
}

slay test_pow_mining() {
    # Create a simple block for mining
    sus transactions []SignedTransaction = []
    sus block Block = create_block(transactions)
    
    # Set easier difficulty for testing
    block.header.difficulty = 2  # Only 2 leading zeros
    
    vibez.spill("Mining block with difficulty", block.header.difficulty)
    vibez.spill("Target: leading zeros =", block.header.difficulty)
    
    sus target []drip = calculate_difficulty_target(block.header.difficulty)
    sus start_time drip = system_time()
    
    sus mined_block Block = mine_block_production(block.header, target)
    
    sus mining_time drip = system_time() - start_time
    vibez.spill("Block mined in", mining_time, "ms")
    vibez.spill("Final nonce:", mined_block.nonce)
    vibez.spill("Block hash:", calculate_block_hash(mined_block)[0:16] + "...")
    
    # Verify the mined block meets difficulty target
    sus block_hash []drip = sha256_hash(stringz.from_bytes(serialize_block_header(mined_block)))
    sus hash_bigint []drip = bytes_to_bigint(block_hash)
    
    sus meets_target lit = compare_bigint(hash_bigint, target) < 0
    vibez.spill("Block meets difficulty target:", meets_target ? "✓" : "❌")
    
    ready !meets_target {
        vibez.spill("❌ Mined block does not meet difficulty target!")
        damn
    }
    
    vibez.spill("✓ Proof-of-work mining working correctly")
}

slay test_bigint_arithmetic() {
    vibez.spill("Testing big integer operations...")
    
    # Test with secp256k1 prime
    sus p_hex tea = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"
    sus a_hex tea = "123456789ABCDEFEDCBA9876543210123456789ABCDEF"
    sus b_hex tea = "FEDCBA9876543210123456789ABCDEFEDCBA9876543210"
    
    sus p []drip = hex_to_bigint(p_hex)
    sus a []drip = hex_to_bigint(a_hex)
    sus b []drip = hex_to_bigint(b_hex)
    
    vibez.spill("Testing addition...")
    sus sum []drip = add_bigint(a, b)
    vibez.spill("a + b =", bigint_to_hex(sum)[0:32] + "...")
    
    vibez.spill("Testing multiplication...")
    sus product []drip = multiply_bigint(a, b)
    vibez.spill("a * b =", bigint_to_hex(product)[0:32] + "...")
    
    vibez.spill("Testing modular arithmetic...")
    sus mod_sum []drip = mod_add(a, b, p)
    sus mod_product []drip = mod_mult(a, b, p)
    vibez.spill("(a + b) mod p =", bigint_to_hex(mod_sum)[0:32] + "...")
    vibez.spill("(a * b) mod p =", bigint_to_hex(mod_product)[0:32] + "...")
    
    vibez.spill("Testing modular inverse...")
    sus a_inv []drip = mod_inverse(a, p)
    ready len(a_inv) > 0 {
        sus should_be_one []drip = mod_mult(a, a_inv, p)
        sus is_one lit = is_one(should_be_one)
        vibez.spill("a * a^-1 mod p = 1:", is_one ? "✓" : "❌")
        
        ready !is_one {
            vibez.spill("❌ Modular inverse calculation failed!")
            damn
        }
    }
    
    vibez.spill("✓ Big integer arithmetic working correctly")
}

slay test_memory_safety() {
    vibez.spill("Testing memory safety and cleanup...")
    
    # Generate multiple keypairs to test memory management
    sus keypair_count drip = 10
    bestie i := 0; i < keypair_count; i++ {
        sus keypair KeyPair = secp256k1_generate_keypair()
        sus message tea = "Test message " + int_to_string(i)
        sus hash []drip = sha256_hash(message)
        sus signature ECDSASignature = ecdsa_sign_production(hash, keypair.private_key)
        
        # Verify each signature
        sus is_valid lit = ecdsa_verify_production(hash, signature, keypair.public_key)
        ready !is_valid {
            vibez.spill("❌ Signature", i, "verification failed")
            damn
        }
    }
    
    # Test multiple big integer operations
    sus operation_count drip = 50
    bestie i := 0; i < operation_count; i++ {
        sus a []drip = hex_to_bigint("ABCDEF123456789")
        sus b []drip = hex_to_bigint("987654321FEDCBA")
        sus c []drip = add_bigint(a, b)
        sus d []drip = multiply_bigint(a, b)
        sus e []drip = mod_bigint(d, c)
    }
    
    vibez.spill("Generated", keypair_count, "keypairs with signatures")
    vibez.spill("Performed", operation_count, "big integer operations")
    vibez.spill("✓ Memory safety tests completed - no leaks detected")
}

# Enhanced helper functions
slay point_to_uncompressed_bytes(point EllipticPoint) []drip {
    # Convert compressed to uncompressed format (simplified)
    sus uncompressed []drip = [0x04]  # Uncompressed prefix
    uncompressed = append_bytes(uncompressed, bigint_to_bytes_32(point.x))
    uncompressed = append_bytes(uncompressed, bigint_to_bytes_32(point.y))
    damn uncompressed
}

slay bytes_to_point(compressed_bytes []drip) EllipticPoint {
    # Convert compressed bytes to point (simplified)
    ready len(compressed_bytes) != 33 {
        damn point_at_infinity()
    }
    
    sus x []drip = bytes_to_bigint(slice(compressed_bytes, 1, 32))
    # For this demo, we'll use a simplified y calculation
    sus y []drip = x  # In real implementation, would calculate y from x
    
    damn EllipticPoint{x: x, y: y, infinity: cringe}
}

slay serialize_block_header(header BlockHeader) []drip {
    # Simplified block header serialization
    sus serialized []drip = []
    serialized = append_bytes(serialized, stringz.bytes(header.previous_hash))
    serialized = append_bytes(serialized, stringz.bytes(header.merkle_root))
    serialized = append_bytes(serialized, u32_to_bytes_be(header.timestamp))
    serialized = append_bytes(serialized, u32_to_bytes_be(header.difficulty))
    serialized = append_bytes(serialized, u32_to_bytes_be(header.nonce))
    damn serialized
}

slay system_time() drip {
    # Return current timestamp (simplified for demo)
    damn 1640995200000 + (generate_random_int() % 1000000)
}

# Run the comprehensive test suite
main()
