# blockchainz - Blockchain Utilities Module

## Overview
Complete blockchain utilities library providing cryptographic primitives, hashing algorithms, merkle trees, and blockchain data structures - all implemented in pure CURSED.

## Features

### Core Blockchain Primitives
- **Block Structure**: Complete block representation with headers, transactions, and validation
- **Transaction Management**: Transaction creation, signing, verification, and serialization
- **Merkle Trees**: Binary merkle tree implementation with proof generation and verification
- **Address Generation**: Wallet address generation from public keys with checksums

### Cryptographic Primitives
- **SHA-256**: Pure CURSED implementation of SHA-256 hashing
- **RIPEMD-160**: Hash function for address generation
- **ECDSA**: Elliptic curve digital signatures for transaction signing
- **Base58**: Encoding/decoding for user-friendly addresses

### Blockchain Operations
- **Chain Validation**: Block and chain integrity verification
- **Mining Simulation**: Proof-of-work mining with difficulty adjustment
- **Wallet Operations**: Key generation, address creation, balance tracking
- **Network Protocols**: P2P message handling and block propagation simulation

## Usage Examples

### Basic Hash Operations
```cursed
yeet "blockchainz"

sus data tea = "Hello, Blockchain!"
sus hash tea = sha256(data)
vibez.spill("SHA-256:", hash)

sus merkle_root tea = calculate_merkle_root(["tx1", "tx2", "tx3", "tx4"])
vibez.spill("Merkle Root:", merkle_root)
```

### Transaction Creation
```cursed
yeet "blockchainz"

sus wallet Wallet = create_wallet()
sus transaction Transaction = create_transaction(
    wallet.address,
    "recipient_address",
    1000000,  # Amount in satoshis
    50000     # Fee
)

sus signed_tx SignedTransaction = sign_transaction(transaction, wallet.private_key)
vibez.spill("Transaction ID:", signed_tx.txid)
```

### Block Mining
```cursed
yeet "blockchainz"

sus block Block = create_block([
    create_transaction("addr1", "addr2", 500000, 10000),
    create_transaction("addr3", "addr4", 750000, 15000)
])

sus mined_block MinedBlock = mine_block(block, 4)  # Difficulty: 4 leading zeros
vibez.spill("Mined block hash:", mined_block.hash)
vibez.spill("Nonce:", mined_block.nonce)
```

## API Reference

### Types
- `Wallet`: Cryptocurrency wallet with keys and address
- `Transaction`: Blockchain transaction structure
- `Block`: Blockchain block with header and transactions
- `MerkleTree`: Binary merkle tree for transaction verification
- `BlockchainNetwork`: P2P network simulation

### Core Functions
- `sha256(data: tea) -> tea`: SHA-256 hash calculation
- `create_wallet() -> Wallet`: Generate new wallet with keypair
- `create_transaction(...) -> Transaction`: Build new transaction
- `mine_block(block: Block, difficulty: drip) -> MinedBlock`: Mine block with PoW
- `validate_blockchain(chain: []Block) -> lit`: Validate entire blockchain

Built for educational and experimental blockchain development in CURSED.
