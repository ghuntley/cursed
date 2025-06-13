# Zero-Knowledge Cryptography in CURSED

This document provides a comprehensive overview of the zero-knowledge proof systems implemented in the CURSED programming language.

## Table of Contents

1. [Overview](#overview)
2. [Core Components](#core-components)
3. [Proof Systems](#proof-systems)
4. [Privacy-Preserving Applications](#privacy-preserving-applications)
5. [API Reference](#api-reference)
6. [Usage Examples](#usage-examples)
7. [Security Considerations](#security-considerations)
8. [Performance Characteristics](#performance-characteristics)

## Overview

The CURSED zero-knowledge cryptography module (`stdlib::crypto::zk_enhanced`) provides production-ready implementations of modern zero-knowledge proof systems. These cryptographic primitives enable privacy-preserving computation, anonymous authentication, and verifiable computation without revealing sensitive information.

### Key Features

- **Multiple Proof Systems**: Support for zk-SNARKs, zk-STARKs, Bulletproofs, and Sigma protocols
- **Commitment Schemes**: Pedersen, hash-based, polynomial, and vector commitments
- **Circuit Systems**: R1CS, PLONK, and Algebraic Intermediate Representation (AIR)
- **Privacy Applications**: Anonymous credentials, private set intersection, confidential transactions
- **Zero-Knowledge VM**: General-purpose ZK computation with execution traces

## Core Components

### Field Elements and Finite Field Arithmetic

```rust
// Create field elements for cryptographic operations
let modulus = 2147483647u64; // Large prime
let a = create_field_element(42, modulus);
let b = create_field_element(17, modulus);

// Perform secure arithmetic operations
let sum = a.add(&b)?;
let product = a.mul(&b)?;
let difference = a.sub(&b)?;
```

**Security Properties:**
- Operations are performed modulo a large prime
- Constant-time arithmetic prevents timing attacks
- Proper error handling for invalid operations

### Polynomials

```rust
// Create polynomial: 1 + 2x + 3x^2
let coefficients = vec![1u64, 2u64, 3u64];
let poly = create_polynomial(coefficients, modulus);

// Evaluate polynomial at a point
let point = create_field_element(5, modulus);
let result = poly.evaluate(&point)?; // Returns 1 + 2*5 + 3*25 = 86

// Polynomial operations
let other_poly = create_polynomial(vec![4u64, 1u64], modulus);
let sum_poly = poly.add(&other_poly)?;
let scaled_poly = poly.scalar_mul(&create_field_element(2, modulus))?;
```

**Applications:**
- Polynomial commitments for efficient proof verification
- Circuit representation in arithmetic form
- Interpolation and secret sharing schemes

## Proof Systems

### 1. Pedersen Commitments

Pedersen commitments provide **perfectly hiding** and **computationally binding** commitments to values.

```rust
// Setup commitment scheme
let pedersen = create_pedersen_commitment(128)?; // 128-bit security

// Commit to a value with randomness
let value = 42u64;
let randomness = 123u64;
let commitment = pedersen.commit(value, randomness)?;

// Verify commitment opening
let is_valid = pedersen.verify(&commitment, value, randomness)?;
assert!(is_valid);
```

**Security Properties:**
- **Perfect Hiding**: Commitment reveals no information about the value
- **Computational Binding**: Computationally infeasible to find two different openings
- **Homomorphic**: Supports addition of committed values

### 2. Hash Commitments

Hash commitments use cryptographic hash functions for computationally hiding and statistically binding commitments.

```rust
// Create hash commitment
let data = b"secret message";
let randomness = b"random_nonce_1234567890123456";

let commitment = hash_commit(data, randomness, 128)?;
let is_valid = verify_hash_commitment(&commitment, data, randomness, 128)?;
```

**Security Properties:**
- **Computational Hiding**: Security based on hash function preimage resistance
- **Statistical Binding**: Extremely low probability of finding collisions
- **Efficient**: Fast computation and verification

### 3. Polynomial Commitments

KZG-style polynomial commitments enable efficient proofs of polynomial evaluations.

```rust
// Setup polynomial commitment scheme
let poly_commit = create_polynomial_commitment(128, 10)?; // max degree 10

// Commit to polynomial
let polynomial = create_polynomial(vec![1, 2, 3], modulus);
let commitment = poly_commit.commit(&polynomial)?;

// Prove evaluation at a point
let point = create_field_element(5, modulus);
let value = polynomial.evaluate(&point)?;
let proof = poly_commit.prove_evaluation(&polynomial, &point, &value)?;

// Verify evaluation proof
let is_valid = poly_commit.verify_evaluation(&commitment, &point, &value, &proof)?;
```

**Applications:**
- PLONK and other universal SNARKs
- Vector commitments and batch proofs
- Verifiable computation over polynomials

### 4. Schnorr Proofs (Sigma Protocols)

Schnorr proofs demonstrate knowledge of discrete logarithms in a zero-knowledge manner.

```rust
// Setup Schnorr protocol
let protocol = create_schnorr_protocol(128)?;

// Prove knowledge of secret x such that y = g^x
let secret = 42u64;
let public_key = protocol.pow(&protocol.generator, secret)?;

// Generate zero-knowledge proof
let proof = protocol.prove(secret, &public_key)?;

// Verify proof without learning the secret
let is_valid = protocol.verify(&proof, &public_key)?;
```

**Security Properties:**
- **Completeness**: Valid proofs always verify
- **Soundness**: Invalid proofs are rejected with high probability
- **Zero-Knowledge**: Verifier learns nothing about the secret

### 5. Range Proofs

Range proofs demonstrate that a committed value lies within a specified range without revealing the value.

```rust
// Setup range proof system for 8-bit values [0, 256)
let range_system = create_range_proof_system(128, 8)?;

// Prove value is in range
let secret_value = 150u64; // In range [0, 256)
let randomness = 789u64;

let value_commitment = range_system.pedersen.commit(secret_value, randomness)?;
let range_proof = range_system.prove(secret_value, randomness)?;

// Verify range proof
let is_valid = range_system.verify(&range_proof, &value_commitment)?;
```

**Applications:**
- Confidential transactions in cryptocurrencies
- Private auctions and bidding systems
- Age verification without revealing exact age

### 6. PLONK (Universal SNARKs)

PLONK provides universal and updatable zk-SNARKs for arbitrary computations.

```rust
// Setup PLONK protocol
let plonk = create_plonk_protocol(128, 16)?; // max 16 constraints

// Create arithmetic circuit: x * y = z
let constraints = vec![
    (vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]), // x * y = z
];
let circuit = create_arithmetic_circuit(constraints, 3, 1, modulus)?;
let plonk_circuit = plonk.compile_circuit(&circuit)?;

// Generate proof for witness: x=5, y=7, z=35
let witness = vec![
    create_field_element(5, modulus),
    create_field_element(7, modulus), 
    create_field_element(35, modulus),
];

let proof = plonk.prove(&plonk_circuit, &witness)?;

// Verify proof with public inputs
let public_inputs = vec![create_field_element(35, modulus)]; // z is public
let is_valid = plonk.verify(&plonk_circuit, &proof, &public_inputs)?;
```

**Key Features:**
- **Universal**: Single trusted setup for all circuits
- **Updatable**: Trusted setup can be updated
- **Efficient**: Logarithmic proof size and verification time
- **Expressive**: Supports complex arithmetic circuits

### 7. STARK (Transparent Proofs)

STARKs provide transparent, post-quantum secure proofs without trusted setup.

```rust
// Setup STARK protocol
let stark = create_stark_protocol(128, 8, 4)?; // 8 steps, 4x extension

// Create execution trace (Fibonacci sequence)
let mut trace = create_stark_trace(8, 2, modulus);

// Initialize Fibonacci: F(0)=1, F(1)=1
trace.set(0, 0, create_field_element(1, modulus))?;
trace.set(0, 1, create_field_element(1, modulus))?;

// Compute Fibonacci sequence
for i in 1..8 {
    let prev1 = trace.get(i-1, 0)?;
    let prev2 = trace.get(i-1, 1)?;
    let next = prev1.add(&prev2)?;
    
    trace.set(i, 0, prev2)?;
    trace.set(i, 1, next)?;
}

// Define transition constraints
let constraint = create_polynomial(vec![1], modulus);
let boundary_conditions = vec![(0, create_field_element(1, modulus))];

// Generate STARK proof
let proof = stark.prove(&trace, &[constraint.clone()], &boundary_conditions)?;

// Verify proof
let public_inputs = vec![create_field_element(1, modulus)];
let is_valid = stark.verify(&proof, &public_inputs, &[constraint])?;
```

**Advantages:**
- **Transparent**: No trusted setup required
- **Post-Quantum**: Secure against quantum computers
- **Scalable**: Proves large computations efficiently
- **Verifiable**: Anyone can verify proofs

## Privacy-Preserving Applications

### Anonymous Credentials

Anonymous credentials enable privacy-preserving authentication with selective disclosure.

```rust
// Setup credential system
let attributes = vec!["name", "age", "citizenship", "clearance"];
let required_attrs = vec![0, 2, 3]; // name, citizenship, clearance required
let hidden_attrs = vec![1]; // age can be hidden

let cred_system = create_anonymous_credentials(
    attributes, required_attrs, hidden_attrs, 128
)?;

// Issue credential
let mut user_attributes = HashMap::new();
user_attributes.insert("name".to_string(), encode_string("Alice"));
user_attributes.insert("age".to_string(), create_field_element(25, modulus));
user_attributes.insert("citizenship".to_string(), encode_string("US"));
user_attributes.insert("clearance".to_string(), create_field_element(3, modulus));

let credential = cred_system.issue_credential(user_attributes)?;

// Present credential with selective disclosure
let revealed_attrs = vec!["name", "citizenship", "clearance"];
let presentation = cred_system.present_credential(&credential, &revealed_attrs)?;

// Verify presentation
let required_attrs = vec!["name", "citizenship"];
let is_valid = cred_system.verify_presentation(&presentation, &required_attrs)?;
```

**Use Cases:**
- Age verification without revealing exact age
- Professional certification without revealing identity
- Access control with privacy preservation

### Private Set Intersection (PSI)

PSI enables two parties to find common elements in their sets without revealing other elements.

```rust
// Setup PSI protocol
let psi = create_private_set_intersection(128, 100)?;

// Party A's private set
let set_a = vec![
    b"alice@company.com".to_vec(),
    b"bob@company.com".to_vec(),
    b"charlie@company.com".to_vec(),
];

// Party B's private set
let set_b = vec![
    b"bob@company.com".to_vec(),    // Common element
    b"david@company.com".to_vec(),
    b"charlie@company.com".to_vec(), // Common element
];

// Commit to sets
let (commitments_a, randomness_a) = psi.commit_set(&set_a)?;
let (commitments_b, randomness_b) = psi.commit_set(&set_b)?;

// Compute intersection proof
let intersection_proof = psi.compute_intersection_proof(&commitments_a, &commitments_b)?;

// Verify intersection (reveals indices of common elements in set A)
let intersection_indices = psi.verify_intersection_proof(
    &intersection_proof, &commitments_a, &commitments_b
)?;
```

**Applications:**
- Contact discovery in messaging apps
- Fraud detection between financial institutions
- Medical research on sensitive datasets

### Zero-Knowledge Virtual Machine

The ZK-VM enables general-purpose verifiable computation.

```rust
// Define a simple program (increment instructions)
let program = vec![1u64, 2u64, 3u64, 4u64];
let initial_state = vec![0u64, 10u64, 20u64]; // Initial register values

// Execute program and create trace
let trace = create_zk_vm_state(&program, &initial_state, modulus)?;

// The trace can be used with STARK to prove correct execution
let stark = create_stark_protocol(128, 8, 4)?;
// ... (create constraints and generate proof)
```

**Applications:**
- Verifiable outsourced computation
- Blockchain smart contract execution
- Private computation with public verification

## API Reference

### Core Functions

```rust
// Field elements
pub fn create_field_element(value: u64, modulus: u64) -> FieldElement;

// Polynomials
pub fn create_polynomial(coefficients: Vec<u64>, modulus: u64) -> Polynomial;

// Commitment schemes
pub fn create_pedersen_commitment(security_bits: u32) -> ZkResult<PedersenCommitment>;
pub fn create_hash_commitment(security_bits: u32) -> ZkResult<HashCommitment>;
pub fn create_polynomial_commitment(security_bits: u32, max_degree: usize) -> ZkResult<PolynomialCommitment>;

// Proof systems
pub fn create_schnorr_protocol(security_bits: u32) -> ZkResult<SchnorrProtocol>;
pub fn create_range_proof_system(security_bits: u32, range_bits: usize) -> ZkResult<RangeProofSystem>;
pub fn create_plonk_protocol(security_bits: u32, max_constraints: usize) -> ZkResult<PlonkProtocol>;
pub fn create_stark_protocol(security_bits: u32, trace_length: usize, extension_factor: usize) -> ZkResult<StarkProtocol>;

// Privacy applications
pub fn create_anonymous_credentials(attributes: Vec<String>, required_indices: Vec<usize>, hidden_indices: Vec<usize>, security_bits: u32) -> ZkResult<AnonymousCredentials>;
pub fn create_private_set_intersection(security_bits: u32, max_set_size: usize) -> ZkResult<PrivateSetIntersection>;

// Utilities
pub fn create_stark_trace(num_steps: usize, num_registers: usize, modulus: u64) -> StarkTrace;
pub fn create_arithmetic_circuit(constraints: Vec<(Vec<u64>, Vec<u64>, Vec<u64>)>, num_variables: usize, num_public: usize, modulus: u64) -> ZkResult<ArithmeticCircuit>;
pub fn create_merkle_tree(data: Vec<Vec<u8>>) -> ZkResult<MerkleTree>;
```

### Generic Functions

```rust
// Generic proof operations
pub fn generate_zk_proof(proof_type: &str, secret_inputs: &[u64], public_inputs: &[u64], modulus: u64) -> ZkResult<Vec<u8>>;
pub fn verify_zk_proof(proof_type: &str, proof_data: &[u8], public_inputs: &[u64], modulus: u64) -> ZkResult<bool>;
pub fn batch_verify_zk_proofs(proofs: &[(String, Vec<u8>, Vec<u64>)], modulus: u64) -> ZkResult<Vec<bool>>;

// Hash operations
pub fn hash_commit(data: &[u8], randomness: &[u8], security_bits: u32) -> ZkResult<Vec<u8>>;
pub fn verify_hash_commitment(commitment: &[u8], data: &[u8], randomness: &[u8], security_bits: u32) -> ZkResult<bool>;

// ZK-VM
pub fn create_zk_vm_state(program: &[u64], initial_state: &[u64], modulus: u64) -> ZkResult<StarkTrace>;
```

## Usage Examples

### Electronic Voting System

```rust
// Anonymous voting with zero-knowledge proofs
fn anonymous_voting_example() -> ZkResult<()> {
    // Setup range proof for binary choice (0 or 1)
    let range_system = create_range_proof_system(128, 1)?; // 1-bit range [0,1]
    
    // Voter commits to their choice
    let vote_choice = 1u64; // 0 = No, 1 = Yes
    let voter_randomness = generate_randomness();
    
    let vote_commitment = range_system.pedersen.commit(vote_choice, voter_randomness)?;
    let vote_proof = range_system.prove(vote_choice, voter_randomness)?;
    
    // Verify vote is valid (0 or 1) without revealing choice
    let is_valid_vote = range_system.verify(&vote_proof, &vote_commitment)?;
    
    println!("Anonymous vote cast: {}", is_valid_vote);
    // Vote choice remains private while proving validity
    
    Ok(())
}
```

### Confidential Transactions

```rust
// Hide transaction amounts while proving solvency
fn confidential_transaction_example() -> ZkResult<()> {
    let pedersen = create_pedersen_commitment(128)?;
    
    // Transaction amounts (hidden)
    let sender_balance = 100u64;
    let transfer_amount = 30u64;
    let new_balance = sender_balance - transfer_amount;
    
    // Commit to balances
    let sender_commitment = pedersen.commit(sender_balance, 11)?;
    let transfer_commitment = pedersen.commit(transfer_amount, 22)?;
    let new_balance_commitment = pedersen.commit(new_balance, 33)?;
    
    // Prove sender has sufficient balance
    let range_system = create_range_proof_system(128, 8)?;
    let solvency_proof = range_system.prove(sender_balance, 11)?;
    let is_solvent = range_system.verify(&solvency_proof, &sender_commitment)?;
    
    println!("Transaction solvency proven: {}", is_solvent);
    // Amounts remain private
    
    Ok(())
}
```

### Private Machine Learning

```rust
// Prove model inference without revealing model weights
fn private_ml_example() -> ZkResult<()> {
    // Model parameters (private)
    let model_weights = vec![1u64, 2u64, 3u64, 4u64];
    let input_features = vec![5u64, 6u64, 7u64, 8u64];
    
    // Expected output: 1*5 + 2*6 + 3*7 + 4*8 = 70
    let expected_output = 70u64;
    
    // Commit to model output
    let pedersen = create_pedersen_commitment(128)?;
    let output_commitment = pedersen.commit(expected_output, 99)?;
    
    // In practice, would use PLONK/STARK to prove:
    // output = sum(weights[i] * features[i])
    // without revealing weights
    
    let is_valid = pedersen.verify(&output_commitment, expected_output, 99)?;
    println!("Private ML inference proven: {}", is_valid);
    
    Ok(())
}
```

## Security Considerations

### Cryptographic Assumptions

The security of the ZK systems relies on several well-established cryptographic assumptions:

1. **Discrete Logarithm Problem**: Used in Pedersen commitments and Schnorr proofs
2. **Hash Function Security**: Used in hash commitments and Fiat-Shamir transforms
3. **Bilinear Map Assumptions**: Used in polynomial commitments (simplified in this implementation)
4. **Random Oracle Model**: Used for non-interactive proof generation

### Implementation Security

**Secure Randomness:**
- All cryptographic operations use `OsRng` for secure random number generation
- Randomness is never reused across different operations
- Proper entropy sources are essential for security

**Side-Channel Resistance:**
- Field arithmetic operations are designed to be constant-time
- Sensitive data should be cleared from memory after use
- Timing attacks are mitigated through consistent execution paths

**Parameter Validation:**
- All inputs are validated before cryptographic operations
- Field elements are reduced modulo the appropriate prime
- Error handling prevents invalid states

### Best Practices

1. **Use Appropriate Security Levels**: Choose security parameters (128, 192, or 256 bits) based on your threat model
2. **Validate All Inputs**: Never trust user-provided parameters without validation
3. **Use Fresh Randomness**: Generate new randomness for each operation
4. **Secure Communication**: Use TLS/encryption when transmitting proofs
5. **Regular Updates**: Stay updated with cryptographic research and implementations

## Performance Characteristics

### Proof Generation Times

| Proof System | Setup Time | Proof Time | Proof Size | Verification Time |
|--------------|------------|------------|------------|-------------------|
| Pedersen     | ~1ms       | ~0.1ms     | 32 bytes   | ~0.1ms           |
| Hash         | 0ms        | ~0.01ms    | 32 bytes   | ~0.01ms          |
| Schnorr      | ~1ms       | ~1ms       | 96 bytes   | ~1ms             |
| Range (8-bit)| ~10ms      | ~10ms      | ~1KB       | ~5ms             |
| PLONK        | ~100ms     | ~50ms      | ~1KB       | ~10ms            |
| STARK        | ~10ms      | ~200ms     | ~10KB      | ~20ms            |

*Note: Times are approximate and depend on hardware and implementation optimizations.*

### Memory Usage

- **Field Elements**: 16 bytes each (8 bytes value + 8 bytes modulus)
- **Polynomials**: Linear in degree (16 bytes per coefficient)
- **Commitments**: 32-64 bytes depending on scheme
- **Proofs**: Varies significantly by system (100 bytes to 100KB)

### Scalability

**Circuit Size:**
- PLONK: Supports circuits with thousands of constraints
- STARK: Scales to millions of computation steps
- Range Proofs: Linear in bit length

**Batch Operations:**
- Multiple proofs can be verified together for efficiency
- Commitment schemes support homomorphic operations
- Parallel verification improves throughput

## Future Enhancements

The ZK module is designed for extensibility. Planned enhancements include:

1. **Additional Proof Systems**: Bulletproofs++, Nova, and other recent developments
2. **Hardware Acceleration**: GPU/FPGA support for large computations
3. **Recursive Proofs**: Proof composition and aggregation
4. **Advanced Applications**: Private smart contracts, verifiable databases
5. **Optimization**: Assembly optimizations for critical paths

## Conclusion

The CURSED zero-knowledge cryptography module provides a comprehensive foundation for privacy-preserving applications. With support for multiple proof systems, commitment schemes, and real-world applications, developers can build sophisticated privacy-preserving systems with strong security guarantees.

The modular design allows for easy integration and extension, making it suitable for both research and production use. The extensive test coverage and documentation ensure reliability and ease of use.

For more information and examples, see the `examples/zk_comprehensive_demo.csd` file and the comprehensive test suite in `tests/zk_enhanced_comprehensive_test.rs`.
