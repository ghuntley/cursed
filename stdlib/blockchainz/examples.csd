# blockchainz/examples.csd - Blockchain Usage Examples
# Comprehensive examples demonstrating blockchain functionality

yeet "blockchainz/core"
yeet "vibez"

# Example 1: Basic wallet operations
slay demo_wallet_operations() {
    vibez.spill("\n=== Wallet Operations Demo ===")
    
    # Create new wallet
    sus wallet Wallet = create_wallet()
    vibez.spill("Generated wallet:")
    vibez.spill("  Address:", wallet.address)
    vibez.spill("  Public Key:", wallet.public_key[0:16] + "...")
    vibez.spill("  Private Key:", wallet.private_key[0:16] + "...")
    
    # Create another wallet for testing
    sus wallet2 Wallet = create_wallet()
    vibez.spill("\nSecond wallet address:", wallet2.address)
}

# Example 2: Transaction creation and signing
slay demo_transactions() {
    vibez.spill("\n=== Transaction Demo ===")
    
    # Create wallets
    sus sender Wallet = create_wallet()
    sus recipient Wallet = create_wallet()
    
    vibez.spill("Sender:", sender.address)
    vibez.spill("Recipient:", recipient.address)
    
    # Create transaction
    sus tx Transaction = create_transaction(
        sender.address,
        recipient.address,
        1000000,  # 0.01 BTC in satoshis
        50000     # Fee in satoshis
    )
    
    vibez.spill("\nTransaction created:")
    vibez.spill("  Amount:", tx.amount, "satoshis")
    vibez.spill("  Fee:", tx.fee, "satoshis")
    vibez.spill("  Timestamp:", tx.timestamp)
    
    # Sign transaction
    sus signed_tx SignedTransaction = sign_transaction(tx, sender.private_key)
    vibez.spill("  Transaction ID:", signed_tx.txid[0:16] + "...")
    
    # Verify transaction
    sus is_valid lit = verify_transaction(signed_tx)
    vibez.spill("  Transaction valid:", is_valid)
}

# Example 3: Merkle tree operations
slay demo_merkle_tree() {
    vibez.spill("\n=== Merkle Tree Demo ===")
    
    # Create sample transaction IDs
    sus tx_ids []tea = [
        "tx1_hash_0123456789abcdef",
        "tx2_hash_fedcba9876543210", 
        "tx3_hash_1122334455667788",
        "tx4_hash_8877665544332211"
    ]
    
    vibez.spill("Transaction IDs:")
    bestie (sus i drip = 0; i < len(tx_ids); i = i + 1) {
        vibez.spill("  ", i + 1, ":", tx_ids[i])
    }
    
    # Calculate merkle root
    sus merkle_root tea = calculate_merkle_root(tx_ids)
    vibez.spill("\nMerkle Root:", merkle_root[0:32] + "...")
    
    # Build full merkle tree
    sus tree MerkleTree = build_merkle_tree(tx_ids)
    vibez.spill("Merkle tree built with", len(tree.leaves), "leaves")
}

# Example 4: Block creation and mining
slay demo_block_mining() {
    vibez.spill("\n=== Block Mining Demo ===")
    
    # Create sample transactions
    sus wallet1 Wallet = create_wallet()
    sus wallet2 Wallet = create_wallet()
    sus wallet3 Wallet = create_wallet()
    
    sus tx1 Transaction = create_transaction(wallet1.address, wallet2.address, 500000, 25000)
    sus tx2 Transaction = create_transaction(wallet2.address, wallet3.address, 750000, 35000)
    
    sus signed_tx1 SignedTransaction = sign_transaction(tx1, wallet1.private_key)
    sus signed_tx2 SignedTransaction = sign_transaction(tx2, wallet2.private_key)
    
    sus transactions []SignedTransaction = [signed_tx1, signed_tx2]
    
    vibez.spill("Created block with", len(transactions), "transactions")
    
    # Create block
    sus block Block = create_block(transactions)
    vibez.spill("Block header:")
    vibez.spill("  Merkle Root:", block.header.merkle_root[0:32] + "...")
    vibez.spill("  Timestamp:", block.header.timestamp)
    vibez.spill("  Difficulty:", block.header.difficulty)
    
    # Mine block
    vibez.spill("\nMining block (this may take a moment)...")
    sus mined_block Block = mine_block(block, 4)
    
    vibez.spill("Block mined successfully!")
    vibez.spill("  Final hash:", mined_block.hash[0:32] + "...")
    vibez.spill("  Nonce:", mined_block.header.nonce)
    vibez.spill("  Hash starts with zeros:", string_starts_with(mined_block.hash, "0000"))
}

# Example 5: Blockchain validation
slay demo_blockchain_validation() {
    vibez.spill("\n=== Blockchain Validation Demo ===")
    
    # Create genesis block
    sus genesis_transactions []SignedTransaction = []
    sus genesis Block = create_block(genesis_transactions)
    genesis.header.previous_hash = "0000000000000000000000000000000000000000000000000000000000000000"
    sus mined_genesis Block = mine_block(genesis, 4)
    
    # Create second block
    sus wallet1 Wallet = create_wallet()
    sus wallet2 Wallet = create_wallet()
    sus tx Transaction = create_transaction(wallet1.address, wallet2.address, 1000000, 50000)
    sus signed_tx SignedTransaction = sign_transaction(tx, wallet1.private_key)
    
    sus block2 Block = create_block([signed_tx])
    block2.header.previous_hash = mined_genesis.hash
    sus mined_block2 Block = mine_block(block2, 4)
    
    # Create blockchain
    sus blockchain []Block = [mined_genesis, mined_block2]
    
    vibez.spill("Created blockchain with", len(blockchain), "blocks")
    vibez.spill("Genesis block hash:", blockchain[0].hash[0:32] + "...")
    vibez.spill("Block 2 previous hash:", blockchain[1].header.previous_hash[0:32] + "...")
    
    # Validate blockchain
    sus is_valid lit = validate_blockchain(blockchain)
    vibez.spill("Blockchain validation result:", is_valid)
}

# Example 6: Cryptographic hash demonstration
slay demo_hash_functions() {
    vibez.spill("\n=== Hash Functions Demo ===")
    
    sus test_data []tea = [
        "Hello, World!",
        "CURSED Blockchain",
        "The quick brown fox jumps over the lazy dog",
        ""
    ]
    
    bestie (sus i drip = 0; i < len(test_data); i = i + 1) {
        sus data tea = test_data[i]
        sus hash tea = sha256(data)
        
        vibez.spill("Input: \"" + data + "\"")
        vibez.spill("SHA-256: " + hash[0:32] + "...")
        vibez.spill()
    }
    
    # Demonstrate hash consistency
    sus hash1 tea = sha256("consistent")
    sus hash2 tea = sha256("consistent")
    vibez.spill("Hash consistency check:")
    vibez.spill("  Hash 1:", hash1[0:16] + "...")
    vibez.spill("  Hash 2:", hash2[0:16] + "...")
    vibez.spill("  Are equal:", hash1 == hash2)
}

# Example 7: Address generation demonstration
slay demo_address_generation() {
    vibez.spill("\n=== Address Generation Demo ===")
    
    # Show multiple wallet addresses
    bestie (sus i drip = 0; i < 5; i = i + 1) {
        sus wallet Wallet = create_wallet()
        vibez.spill("Wallet", i + 1, "address:", wallet.address)
    }
    
    # Demonstrate address derivation
    sus private_key tea = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    sus public_key tea = private_key_to_public_key(private_key)
    sus address tea = public_key_to_address(public_key)
    
    vibez.spill("\nAddress derivation:")
    vibez.spill("  Private key:", private_key[0:16] + "...")
    vibez.spill("  Public key:", public_key[0:16] + "...")
    vibez.spill("  Address:", address)
}

# Example 8: Performance benchmarking
slay demo_performance_benchmark() {
    vibez.spill("\n=== Performance Benchmark ===")
    
    # Hash performance
    sus start_time drip = get_unix_timestamp()
    bestie (sus i drip = 0; i < 100; i = i + 1) {
        sha256("benchmark_data_" + int_to_string(i))
    }
    sus hash_time drip = get_unix_timestamp() - start_time
    vibez.spill("100 SHA-256 hashes completed in", hash_time, "time units")
    
    # Transaction creation performance
    start_time = get_unix_timestamp()
    sus wallet Wallet = create_wallet()
    bestie (sus i drip = 0; i < 50; i = i + 1) {
        sus tx Transaction = create_transaction(
            wallet.address,
            "test_address",
            1000 + i,
            50
        )
        sign_transaction(tx, wallet.private_key)
    }
    sus tx_time drip = get_unix_timestamp() - start_time
    vibez.spill("50 transaction signatures completed in", tx_time, "time units")
}

# Main demonstration runner
slay run_all_blockchain_demos() {
    vibez.spill("CURSED Blockchain Library Demonstration")
    vibez.spill("=====================================")
    vibez.spill("Pure CURSED implementation of blockchain primitives")
    
    demo_wallet_operations()
    demo_transactions()
    demo_merkle_tree()
    demo_block_mining()
    demo_blockchain_validation()
    demo_hash_functions()
    demo_address_generation()
    demo_performance_benchmark()
    
    vibez.spill("\n=== Demo Complete ===")
    vibez.spill("All blockchain functionality demonstrated successfully!")
    vibez.spill("Module info:", get_module_info())
}

# Run demos when module is executed
run_all_blockchain_demos()
