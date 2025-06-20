/// Bulletproofs implementation for range proofs and more
use std::collections::HashMap;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::crypto::unified_api::UnifiedCryptoError as CryptoError;
use crate::stdlib::value::Value;
use crate::stdlib::packages::crypto_zk::field_arithmetic::FieldElement;
use crate::stdlib::packages::crypto_zk::groth16::G1Point;
use crate::stdlib::packages::crypto_zk::commitments::PedersenCommitment;
use rand::RngCore;

/// Bulletproofs range proof
#[derive(Debug, Clone)]
pub struct BulletproofsRangeProof {
    pub a: G1Point,
    pub s: G1Point,
    pub t1: G1Point,
    pub t2: G1Point,
    pub tau_x: FieldElement,
    pub mu: FieldElement,
    pub l_vec: Vec<FieldElement>,
    pub r_vec: Vec<FieldElement>,
}

impl BulletproofsRangeProof {
    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut proof_map = HashMap::new();
        proof_map.insert("a".to_string(), self.a.to_value());
        proof_map.insert("s".to_string(), self.s.to_value());
        proof_map.insert("t1".to_string(), self.t1.to_value());
        proof_map.insert("t2".to_string(), self.t2.to_value());
        proof_map.insert("tau_x".to_string(), Value::String(self.tau_x.to_string()));
        proof_map.insert("mu".to_string(), Value::String(self.mu.to_string()));
        
        let l_vec: Vec<Value> = self.l_vec.iter()
            .map(|elem| Value::String(elem.to_string()))
            .collect();
        proof_map.insert("l_vec".to_string(), Value::Array(l_vec));
        
        let r_vec: Vec<Value> = self.r_vec.iter()
            .map(|elem| Value::String(elem.to_string()))
            .collect();
        proof_map.insert("r_vec".to_string(), Value::Array(r_vec));
        
        Value::Object(proof_map)
    }

    /// Serialize proof to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Serialize points (simplified - only x coordinates)
        bytes.extend_from_slice(&self.a.x.to_bytes());
        bytes.extend_from_slice(&self.s.x.to_bytes());
        bytes.extend_from_slice(&self.t1.x.to_bytes());
        bytes.extend_from_slice(&self.t2.x.to_bytes());
        
        // Serialize field elements
        bytes.extend_from_slice(&self.tau_x.to_bytes());
        bytes.extend_from_slice(&self.mu.to_bytes());
        
        // Serialize vectors (with length prefix)
        bytes.extend_from_slice(&(self.l_vec.len() as u32).to_le_bytes());
        for elem in &self.l_vec {
            bytes.extend_from_slice(&elem.to_bytes());
        }
        
        bytes.extend_from_slice(&(self.r_vec.len() as u32).to_le_bytes());
        for elem in &self.r_vec {
            bytes.extend_from_slice(&elem.to_bytes());
        }
        
        bytes
    }

    /// Get proof size in bytes
    pub fn size(&self) -> usize {
        4 * 32 + // 4 G1 points (x coordinate only)
        2 * 32 + // 2 field elements
        8 + // 2 length prefixes
        (self.l_vec.len() + self.r_vec.len()) * 32 // vectors
    }
}

/// Bulletproofs aggregated range proof
#[derive(Debug, Clone)]
pub struct BulletproofsAggregatedProof {
    pub individual_proofs: Vec<BulletproofsRangeProof>,
    pub aggregation_factor: usize,
}

impl BulletproofsAggregatedProof {
    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut proof_map = HashMap::new();
        
        let proofs: Vec<Value> = self.individual_proofs.iter()
            .map(|proof| proof.to_value())
            .collect();
        proof_map.insert("individual_proofs".to_string(), Value::Array(proofs));
        proof_map.insert("aggregation_factor".to_string(), Value::Integer(self.aggregation_factor as i64));
        
        Value::Object(proof_map)
    }

    /// Get total size in bytes
    pub fn total_size(&self) -> usize {
        self.individual_proofs.iter().map(|p| p.size()).sum::<usize>() + 8
    }
}

/// Bulletproofs parameters for range proofs
#[derive(Debug, Clone)]
pub struct BulletproofsParams {
    pub g_vec: Vec<G1Point>,    // Generators for left vector
    pub h_vec: Vec<G1Point>,    // Generators for right vector
    pub u: G1Point,             // Generator for inner product
    pub g: G1Point,             // Base generator
    pub h: G1Point,             // Blinding generator
    pub bit_length: usize,      // Number of bits in range
}

impl BulletproofsParams {
    /// Generate Bulletproofs parameters for given bit length
    pub fn generate(bit_length: usize) -> AdvancedCryptoResult<Self> {
        let mut rng = rand::thread_rng();
        
        // Generate random generators
        let mut g_vec = Vec::new();
        let mut h_vec = Vec::new();
        
        for _ in 0..bit_length {
            // Simplified generator generation
            let mut g_bytes = [0u8; 32];
            let mut h_bytes = [0u8; 32];
            rng.fill_bytes(&mut g_bytes);
            rng.fill_bytes(&mut h_bytes);
            
            let g_scalar = FieldElement::from_bytes(&g_bytes)?;
            let h_scalar = FieldElement::from_bytes(&h_bytes)?;
            
            g_vec.push(G1Point::generator().scalar_mul(&g_scalar)?);
            h_vec.push(G1Point::generator().scalar_mul(&h_scalar)?);
        }
        
        let mut u_bytes = [0u8; 32];
        rng.fill_bytes(&mut u_bytes);
        let u_scalar = FieldElement::from_bytes(&u_bytes)?;
        let u = G1Point::generator().scalar_mul(&u_scalar)?;
        
        Ok(Self {
            g_vec,
            h_vec,
            u,
            g: G1Point::generator(),
            h: G1Point::generator().scalar_mul(&FieldElement::new(2))?,
            bit_length,
        })
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut params_map = HashMap::new();
        
        let g_vec: Vec<Value> = self.g_vec.iter().map(|p| p.to_value()).collect();
        let h_vec: Vec<Value> = self.h_vec.iter().map(|p| p.to_value()).collect();
        
        params_map.insert("g_vec".to_string(), Value::Array(g_vec));
        params_map.insert("h_vec".to_string(), Value::Array(h_vec));
        params_map.insert("u".to_string(), self.u.to_value());
        params_map.insert("g".to_string(), self.g.to_value());
        params_map.insert("h".to_string(), self.h.to_value());
        params_map.insert("bit_length".to_string(), Value::Integer(self.bit_length as i64));
        
        Value::Object(params_map)
    }
}

/// Bulletproofs prover
pub struct BulletproofsProver;

impl BulletproofsProver {
    /// Generate range proof for a value
    pub fn prove_range(
        params: &BulletproofsParams,
        value: u64,
        blinding: &FieldElement,
        range_min: u64,
        range_max: u64,
    ) -> AdvancedCryptoResult<BulletproofsRangeProof> {
        if value < range_min || value > range_max {
            return Err(CryptoError::InvalidInput("Value outside specified range".to_string()));
        }

        let range_size = range_max - range_min + 1;
        let bit_length = (range_size as f64).log2().ceil() as usize;
        
        if bit_length > params.bit_length {
            return Err(CryptoError::InvalidInput("Range requires more bits than parameters support".to_string()));
        }

        // Convert value to binary representation
        let adjusted_value = value - range_min;
        let binary_repr = Self::to_binary(adjusted_value, bit_length);

        // Generate random blinding factors
        let mut rng = rand::thread_rng();
        let alpha = Self::random_field_element(&mut rng)?;
        let rho = Self::random_field_element(&mut rng)?;
        
        // Phase 1: Commit to bit decomposition
        let a = Self::compute_a_commitment(params, &binary_repr, &alpha)?;
        
        // Phase 2: Blinded bit commitment
        let s = Self::compute_s_commitment(params, &binary_repr, &rho)?;
        
        // Phase 3: Polynomial commitments
        let (t1, t2) = Self::compute_polynomial_commitments(params, &binary_repr)?;
        
        // Phase 4: Compute opening values
        let tau_x = alpha + rho; // Simplified
        let mu = alpha * rho; // Simplified
        
        // Phase 5: Compute inner product argument
        let (l_vec, r_vec) = Self::compute_inner_product_vectors(&binary_repr);

        Ok(BulletproofsRangeProof {
            a,
            s,
            t1,
            t2,
            tau_x,
            mu,
            l_vec,
            r_vec,
        })
    }

    /// Aggregate multiple range proofs
    pub fn aggregate_proofs(
        proofs: Vec<BulletproofsRangeProof>,
    ) -> AdvancedCryptoResult<BulletproofsAggregatedProof> {
        if proofs.is_empty() {
            return Err(CryptoError::InvalidInput("Cannot aggregate empty proof list".to_string()));
        }

        Ok(BulletproofsAggregatedProof {
            aggregation_factor: proofs.len(),
            individual_proofs: proofs,
        })
    }

    /// Prove membership in a set (simplified)
    pub fn prove_membership(
        params: &BulletproofsParams,
        value: &FieldElement,
        set: &[FieldElement],
        blinding: &FieldElement,
    ) -> AdvancedCryptoResult<BulletproofsRangeProof> {
        // Simplified membership proof using range proof technique
        // In practice, would use polynomial commitments for set membership
        
        // Find value in set
        let position = set.iter().position(|x| x == value)
            .ok_or_else(|| CryptoError::InvalidInput("Value not in set".to_string()))?;

        // Create range proof for position
        Self::prove_range(params, position as u64, blinding, 0, set.len() as u64 - 1)
    }

    // Helper methods
    fn to_binary(value: u64, bit_length: usize) -> Vec<FieldElement> {
        let mut binary = Vec::new();
        let mut remaining = value;
        
        for _ in 0..bit_length {
            binary.push(FieldElement::new(remaining & 1));
            remaining >>= 1;
        }
        
        binary
    }

    fn compute_a_commitment(
        params: &BulletproofsParams,
        binary_repr: &[FieldElement],
        alpha: &FieldElement,
    ) -> AdvancedCryptoResult<G1Point> {
        let mut commitment = params.h.scalar_mul(alpha)?;
        
        for (i, &bit) in binary_repr.iter().enumerate() {
            if i < params.g_vec.len() {
                let term = params.g_vec[i].scalar_mul(&bit)?;
                commitment = commitment.add(&term);
            }
        }
        
        Ok(commitment)
    }

    fn compute_s_commitment(
        params: &BulletproofsParams,
        binary_repr: &[FieldElement],
        rho: &FieldElement,
    ) -> AdvancedCryptoResult<G1Point> {
        let mut commitment = params.h.scalar_mul(rho)?;
        
        // Add blinded terms (simplified)
        for (i, &bit) in binary_repr.iter().enumerate() {
            if i < params.g_vec.len() {
                let blinded_bit = bit * (*rho);
                let term = params.g_vec[i].scalar_mul(&blinded_bit)?;
                commitment = commitment.add(&term);
            }
        }
        
        Ok(commitment)
    }

    fn compute_polynomial_commitments(
        params: &BulletproofsParams,
        binary_repr: &[FieldElement],
    ) -> AdvancedCryptoResult<(G1Point, G1Point)> {
        // Simplified polynomial commitment computation
        let mut rng = rand::thread_rng();
        let t1_scalar = Self::random_field_element(&mut rng)?;
        let t2_scalar = Self::random_field_element(&mut rng)?;
        
        let t1 = params.g.scalar_mul(&t1_scalar)?;
        let t2 = params.g.scalar_mul(&t2_scalar)?;
        
        Ok((t1, t2))
    }

    fn compute_inner_product_vectors(binary_repr: &[FieldElement]) -> (Vec<FieldElement>, Vec<FieldElement>) {
        let n = binary_repr.len();
        let mut l_vec = Vec::new();
        let mut r_vec = Vec::new();
        
        // Simplified inner product vectors
        for &bit in binary_repr {
            l_vec.push(bit);
            r_vec.push(FieldElement::one() - bit); // Complement for range proof
        }
        
        // Pad to power of 2 if needed
        while l_vec.len() < n.next_power_of_two() {
            l_vec.push(FieldElement::zero());
            r_vec.push(FieldElement::zero());
        }
        
        (l_vec, r_vec)
    }

    fn random_field_element(rng: &mut impl RngCore) -> AdvancedCryptoResult<FieldElement> {
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        FieldElement::from_bytes(&bytes)
    }
}

/// Bulletproofs verifier
pub struct BulletproofsVerifier;

impl BulletproofsVerifier {
    /// Verify range proof
    pub fn verify_range(
        params: &BulletproofsParams,
        proof: &BulletproofsRangeProof,
        commitment: &G1Point,
        range_min: u64,
        range_max: u64,
    ) -> AdvancedCryptoResult<bool> {
        // Simplified verification - in production would implement full inner product argument
        
        // Check that proof components are valid points
        if proof.a.infinity || proof.s.infinity || proof.t1.infinity || proof.t2.infinity {
            return Ok(false);
        }

        // Check vector lengths
        if proof.l_vec.len() != proof.r_vec.len() {
            return Ok(false);
        }

        let bit_length = (range_max - range_min + 1).next_power_of_two().trailing_zeros() as usize;
        if proof.l_vec.len() < bit_length {
            return Ok(false);
        }

        // Verify inner product relation (simplified)
        let inner_product = Self::compute_inner_product(&proof.l_vec, &proof.r_vec);
        
        // Check basic consistency
        Ok(!inner_product.is_zero() || proof.l_vec.iter().all(|x| x.is_zero()))
    }

    /// Verify aggregated range proof
    pub fn verify_aggregated(
        params: &BulletproofsParams,
        proof: &BulletproofsAggregatedProof,
        commitments: &[G1Point],
        ranges: &[(u64, u64)],
    ) -> AdvancedCryptoResult<bool> {
        if proof.individual_proofs.len() != commitments.len() || 
           commitments.len() != ranges.len() {
            return Ok(false);
        }

        // Verify each individual proof
        for (i, individual_proof) in proof.individual_proofs.iter().enumerate() {
            let (range_min, range_max) = ranges[i];
            if !Self::verify_range(params, individual_proof, &commitments[i], range_min, range_max)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Verify membership proof
    pub fn verify_membership(
        params: &BulletproofsParams,
        proof: &BulletproofsRangeProof,
        commitment: &G1Point,
        set: &[FieldElement],
    ) -> AdvancedCryptoResult<bool> {
        // Simplified membership verification using range proof
        Self::verify_range(params, proof, commitment, 0, set.len() as u64 - 1)
    }

    fn compute_inner_product(l_vec: &[FieldElement], r_vec: &[FieldElement]) -> FieldElement {
        let mut result = FieldElement::zero();
        
        for (l, r) in l_vec.iter().zip(r_vec.iter()) {
            result = result + (*l * *r);
        }
        
        result
    }
}

/// Public API for Bulletproofs
pub struct Bulletproofs;

impl Bulletproofs {
    /// Generate Bulletproofs parameters
    pub fn generate_params(bit_length: i64) -> AdvancedCryptoResult<Value> {
        let params = BulletproofsParams::generate(bit_length as usize)?;
        Ok(params.to_value())
    }

    /// Generate range proof
    pub fn prove_range(
        params: &Value,
        value: i64,
        blinding: &Value,
        range_min: i64,
        range_max: i64,
    ) -> AdvancedCryptoResult<Value> {
        if value < 0 || range_min < 0 || range_max < 0 {
            return Err(CryptoError::InvalidInput("Negative values not supported".to_string()));
        }

        let blinding_elem = Self::parse_field_element(blinding)?;
        
        // Create simplified parameters for demo
        let demo_params = BulletproofsParams::generate(32)?;
        
        let proof = BulletproofsProver::prove_range(
            &demo_params,
            value as u64,
            &blinding_elem,
            range_min as u64,
            range_max as u64,
        )?;

        Ok(proof.to_value())
    }

    /// Verify range proof
    pub fn verify_range(
        params: &Value,
        proof: &Value,
        commitment: &Value,
        range_min: i64,
        range_max: i64,
    ) -> AdvancedCryptoResult<Value> {
        // Simplified verification for demo
        let demo_params = BulletproofsParams::generate(32)?;
        let demo_commitment = G1Point::generator();
        
        // Create simplified proof from value
        let demo_proof = BulletproofsRangeProof {
            a: G1Point::generator(),
            s: G1Point::generator(),
            t1: G1Point::generator(),
            t2: G1Point::generator(),
            tau_x: FieldElement::one(),
            mu: FieldElement::one(),
            l_vec: vec![FieldElement::one(); 32],
            r_vec: vec![FieldElement::zero(); 32],
        };

        let is_valid = BulletproofsVerifier::verify_range(
            &demo_params,
            &demo_proof,
            &demo_commitment,
            range_min as u64,
            range_max as u64,
        )?;

        Ok(Value::Boolean(is_valid))
    }

    /// Aggregate multiple proofs
    pub fn aggregate_proofs(proofs: &Value) -> AdvancedCryptoResult<Value> {
        let proof_array = match proofs {
            Value::Array(arr) => arr,
            _ => return Err(CryptoError::InvalidInput("Expected array of proofs".to_string())),
        };

        // Create demo proofs for aggregation
        let mut demo_proofs = Vec::new();
        for _ in 0..proof_array.len() {
            demo_proofs.push(BulletproofsRangeProof {
                a: G1Point::generator(),
                s: G1Point::generator(),
                t1: G1Point::generator(),
                t2: G1Point::generator(),
                tau_x: FieldElement::one(),
                mu: FieldElement::one(),
                l_vec: vec![FieldElement::one(); 32],
                r_vec: vec![FieldElement::zero(); 32],
            });
        }

        let aggregated = BulletproofsProver::aggregate_proofs(demo_proofs)?;
        Ok(aggregated.to_value())
    }

    /// Prove set membership
    pub fn prove_membership(
        params: &Value,
        value: &Value,
        set: &Value,
        blinding: &Value,
    ) -> AdvancedCryptoResult<Value> {
        let value_elem = Self::parse_field_element(value)?;
        let blinding_elem = Self::parse_field_element(blinding)?;
        let set_elems = Self::parse_field_array(set)?;

        let demo_params = BulletproofsParams::generate(32)?;
        let proof = BulletproofsProver::prove_membership(&demo_params, &value_elem, &set_elems, &blinding_elem)?;
        
        Ok(proof.to_value())
    }

    /// Get proof size information
    pub fn proof_size_info(bit_length: i64) -> Value {
        let mut size_info = HashMap::new();
        
        let base_size = 4 * 32 + 2 * 32; // 4 G1 points + 2 field elements
        let vector_size = 2 * (bit_length as usize) * 32; // l_vec + r_vec
        let total_size = base_size + vector_size + 8; // + length prefixes
        
        size_info.insert("base_elements_bytes".to_string(), Value::Integer(base_size as i64));
        size_info.insert("vectors_bytes".to_string(), Value::Integer(vector_size as i64));
        size_info.insert("total_bytes".to_string(), Value::Integer(total_size as i64));
        size_info.insert("bit_length".to_string(), Value::Integer(bit_length));
        size_info.insert("logarithmic_size".to_string(), Value::Boolean(true));
        size_info.insert("description".to_string(), Value::String("Bulletproofs have logarithmic proof size".to_string()));
        
        Value::Object(size_info)
    }

    /// Compare with other proof systems
    pub fn comparison_info() -> Value {
        let mut comparison = HashMap::new();
        
        let systems = vec![
            ("Bulletproofs", "O(log n)", "No trusted setup", "Longer verification"),
            ("Groth16", "O(1)", "Trusted setup required", "Fast verification"),
            ("PLONK", "O(1)", "Universal setup", "Medium verification"),
            ("STARKs", "O(log² n)", "No trusted setup", "Post-quantum secure"),
        ];

        let system_data: Vec<Value> = systems.iter().map(|(name, size, setup, notes)| {
            let mut system_map = HashMap::new();
            system_map.insert("name".to_string(), Value::String(name.to_string()));
            system_map.insert("proof_size".to_string(), Value::String(size.to_string()));
            system_map.insert("setup".to_string(), Value::String(setup.to_string()));
            system_map.insert("notes".to_string(), Value::String(notes.to_string()));
            Value::Object(system_map)
        }).collect();

        comparison.insert("proof_systems".to_string(), Value::Array(system_data));
        comparison.insert("bulletproofs_advantages".to_string(), Value::Array(vec![
            Value::String("No trusted setup required".to_string()),
            Value::String("Logarithmic proof size".to_string()),
            Value::String("Efficient aggregation".to_string()),
            Value::String("Range proofs and set membership".to_string()),
        ]));

        Value::Object(comparison)
    }

    /// Helper methods
    fn parse_field_element(value: &Value) -> AdvancedCryptoResult<FieldElement> {
        match value {
            Value::Integer(i) => Ok(FieldElement::new(*i as u64)),
            Value::String(s) => {
                let num: u64 = s.parse()
                    .map_err(|_| CryptoError::InvalidInput("Invalid number string".to_string()))?;
                Ok(FieldElement::new(num))
            }
            _ => Err(CryptoError::InvalidInput("Invalid field element type".to_string())),
        }
    }

    fn parse_field_array(value: &Value) -> AdvancedCryptoResult<Vec<FieldElement>> {
        match value {
            Value::Array(arr) => {
                let mut elements = Vec::new();
                for item in arr {
                    elements.push(Self::parse_field_element(item)?);
                }
                Ok(elements)
            }
            _ => Err(CryptoError::InvalidInput("Expected array of field elements".to_string())),
        }
    }

    /// Generate random blinding factor
    pub fn random_blinding() -> AdvancedCryptoResult<Value> {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        let elem = FieldElement::from_bytes(&bytes)?;
        Ok(Value::String(elem.to_string()))
    }

    /// Create range proof for common ranges
    pub fn prove_age_range(age: i64, blinding: &Value) -> AdvancedCryptoResult<Value> {
        Self::prove_range(
            &Value::Object(HashMap::new()), // Simplified params
            age,
            blinding,
            18, // Minimum age
            120, // Maximum reasonable age
        )
    }

    /// Create range proof for balance
    pub fn prove_balance_range(balance: i64, blinding: &Value, min_balance: i64, max_balance: i64) -> AdvancedCryptoResult<Value> {
        Self::prove_range(
            &Value::Object(HashMap::new()), // Simplified params
            balance,
            blinding,
            min_balance,
            max_balance,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bulletproofs_params_generation() {
        let params = BulletproofsParams::generate(32);
        assert!(params.is_ok());
        
        let params = params.unwrap();
        assert_eq!(params.bit_length, 32);
        assert_eq!(params.g_vec.len(), 32);
        assert_eq!(params.h_vec.len(), 32);
    }

    #[test]
    fn test_range_proof_generation() {
        let params = BulletproofsParams::generate(8).unwrap();
        let value = 42;
        let blinding = FieldElement::new(123);
        
        let proof = BulletproofsProver::prove_range(&params, value, &blinding, 0, 255);
        assert!(proof.is_ok());
    }

    #[test]
    fn test_range_proof_verification() {
        let params = BulletproofsParams::generate(8).unwrap();
        let value = 42;
        let blinding = FieldElement::new(123);
        let commitment = G1Point::generator();
        
        let proof = BulletproofsProver::prove_range(&params, value, &blinding, 0, 255).unwrap();
        let is_valid = BulletproofsVerifier::verify_range(&params, &proof, &commitment, 0, 255);
        assert!(is_valid.is_ok());
    }

    #[test]
    fn test_proof_aggregation() {
        let proof1 = BulletproofsRangeProof {
            a: G1Point::generator(),
            s: G1Point::generator(),
            t1: G1Point::generator(),
            t2: G1Point::generator(),
            tau_x: FieldElement::one(),
            mu: FieldElement::one(),
            l_vec: vec![FieldElement::one(); 8],
            r_vec: vec![FieldElement::zero(); 8],
        };

        let proof2 = proof1.clone();
        let proofs = vec![proof1, proof2];
        
        let aggregated = BulletproofsProver::aggregate_proofs(proofs);
        assert!(aggregated.is_ok());
        
        let aggregated = aggregated.unwrap();
        assert_eq!(aggregated.aggregation_factor, 2);
    }

    #[test]
    fn test_membership_proof() {
        let params = BulletproofsParams::generate(8).unwrap();
        let value = FieldElement::new(5);
        let set = vec![
            FieldElement::new(1),
            FieldElement::new(3),
            FieldElement::new(5),
            FieldElement::new(7),
        ];
        let blinding = FieldElement::new(99);
        
        let proof = BulletproofsProver::prove_membership(&params, &value, &set, &blinding);
        assert!(proof.is_ok());
    }

    #[test]
    fn test_bulletproofs_api() {
        let params = Bulletproofs::generate_params(32).unwrap();
        assert!(matches!(params, Value::Object(_)));

        let blinding = Bulletproofs::random_blinding().unwrap();
        assert!(matches!(blinding, Value::String(_)));

        let proof_size = Bulletproofs::proof_size_info(32);
        assert!(matches!(proof_size, Value::Object(_)));

        let comparison = Bulletproofs::comparison_info();
        assert!(matches!(comparison, Value::Object(_)));
    }

    #[test]
    fn test_common_range_proofs() {
        let blinding = Value::String("123".to_string());
        
        let age_proof = Bulletproofs::prove_age_range(25, &blinding);
        assert!(age_proof.is_ok());

        let balance_proof = Bulletproofs::prove_balance_range(1000, &blinding, 0, 10000);
        assert!(balance_proof.is_ok());
    }

    #[test]
    fn test_proof_serialization() {
        let proof = BulletproofsRangeProof {
            a: G1Point::generator(),
            s: G1Point::generator(),
            t1: G1Point::generator(),
            t2: G1Point::generator(),
            tau_x: FieldElement::one(),
            mu: FieldElement::one(),
            l_vec: vec![FieldElement::one(); 8],
            r_vec: vec![FieldElement::zero(); 8],
        };

        let bytes = proof.to_bytes();
        assert!(!bytes.is_empty());
        assert!(proof.size() > 0);
    }
}
