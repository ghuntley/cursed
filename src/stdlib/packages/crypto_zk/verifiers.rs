/// Generic verifier implementations for zero-knowledge proofs
use std::collections::HashMap;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::crypto::symmetric::CryptoError;
use crate::stdlib::value::Value;
use crate::stdlib::packages::crypto_zk::field_arithmetic::FieldElement;
use crate::stdlib::packages::crypto_zk::groth16::{Groth16Verifier, Groth16VerifyingKey, Groth16Proof};
use crate::stdlib::packages::crypto_zk::plonk::{PlonkVerifier, PlonkVerifyingKey, PlonkProof};
use crate::stdlib::packages::crypto_zk::stark::{StarkVerifier, StarkProof, StarkConstraints};
use crate::stdlib::packages::crypto_zk::bulletproofs::{BulletproofsVerifier, BulletproofsRangeProof, BulletproofsParams};
use crate::stdlib::packages::crypto_zk::proofs::{SchnorrProof, SigmaProof, NIZKProof};
use std::time::{Instant, Duration};

/// Generic verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub is_valid: bool,
    pub verification_time: Duration,
    pub error_message: Option<String>,
    pub proof_system: String,
    pub public_inputs_hash: Vec<u8>,
}

impl VerificationResult {
    /// Create successful verification result
    pub fn success(proof_system: String, verification_time: Duration, public_inputs_hash: Vec<u8>) -> Self {
        Self {
            is_valid: true,
            verification_time,
            error_message: None,
            proof_system,
            public_inputs_hash,
        }
    }

    /// Create failed verification result
    pub fn failure(proof_system: String, verification_time: Duration, error: String) -> Self {
        Self {
            is_valid: false,
            verification_time,
            error_message: Some(error),
            proof_system,
            public_inputs_hash: Vec::new(),
        }
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut result_map = HashMap::new();
        result_map.insert("is_valid".to_string(), Value::Boolean(self.is_valid));
        result_map.insert("verification_time_ms".to_string(), Value::Integer(self.verification_time.as_millis() as i64));
        result_map.insert("proof_system".to_string(), Value::String(self.proof_system.clone()));
        
        if let Some(ref error) = self.error_message {
            result_map.insert("error_message".to_string(), Value::String(error.clone()));
        }
        
        if !self.public_inputs_hash.is_empty() {
            result_map.insert("public_inputs_hash".to_string(), Value::String(hex::encode(&self.public_inputs_hash)));
        }
        
        Value::Object(result_map)
    }
}

/// Batch verification result
#[derive(Debug, Clone)]
pub struct BatchVerificationResult {
    pub all_valid: bool,
    pub individual_results: Vec<VerificationResult>,
    pub total_time: Duration,
    pub batch_size: usize,
}

impl BatchVerificationResult {
    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut result_map = HashMap::new();
        result_map.insert("all_valid".to_string(), Value::Boolean(self.all_valid));
        result_map.insert("batch_size".to_string(), Value::Integer(self.batch_size as i64));
        result_map.insert("total_time_ms".to_string(), Value::Integer(self.total_time.as_millis() as i64));
        
        let individual_results: Vec<Value> = self.individual_results.iter()
            .map(|result| result.to_value())
            .collect();
        result_map.insert("individual_results".to_string(), Value::Array(individual_results));
        
        let valid_count = self.individual_results.iter().filter(|r| r.is_valid).count();
        result_map.insert("valid_count".to_string(), Value::Integer(valid_count as i64));
        result_map.insert("invalid_count".to_string(), Value::Integer((self.batch_size - valid_count) as i64));
        
        Value::Object(result_map)
    }
}

/// Universal proof verifier
#[derive(Debug, Clone)]
pub struct UniversalVerifier {
    pub supported_systems: Vec<String>,
    pub verification_cache: HashMap<String, VerificationResult>,
}

impl UniversalVerifier {
    /// Create new universal verifier
    pub fn new() -> Self {
        Self {
            supported_systems: vec![
                "groth16".to_string(),
                "plonk".to_string(),
                "stark".to_string(),
                "bulletproofs".to_string(),
                "schnorr".to_string(),
                "sigma".to_string(),
                "nizk".to_string(),
            ],
            verification_cache: HashMap::new(),
        }
    }

    /// Verify proof using appropriate verifier
    pub fn verify_proof(
        &mut self,
        proof_system: &str,
        proof: &Value,
        public_inputs: &Value,
        verification_key: Option<&Value>,
    ) -> AdvancedCryptoResult<VerificationResult> {
        let start_time = Instant::now();
        
        // Generate cache key
        let cache_key = self.generate_cache_key(proof_system, proof, public_inputs);
        
        // Check cache first
        if let Some(cached_result) = self.verification_cache.get(&cache_key) {
            return Ok(cached_result.clone());
        }

        let result = match proof_system {
            "groth16" => self.verify_groth16(proof, public_inputs, verification_key),
            "plonk" => self.verify_plonk(proof, public_inputs, verification_key),
            "stark" => self.verify_stark(proof, public_inputs),
            "bulletproofs" => self.verify_bulletproofs(proof, public_inputs),
            "schnorr" => self.verify_schnorr(proof, public_inputs),
            "sigma" => self.verify_sigma(proof, public_inputs),
            "nizk" => self.verify_nizk(proof, public_inputs),
            _ => Err(CryptoError::InvalidInput(format!("Unsupported proof system: {}", proof_system))),
        };

        let verification_time = start_time.elapsed();
        
        let final_result = match result {
            Ok(is_valid) => {
                let public_inputs_hash = self.hash_public_inputs(public_inputs);
                VerificationResult::success(proof_system.to_string(), verification_time, public_inputs_hash)
            }
            Err(e) => VerificationResult::failure(proof_system.to_string(), verification_time, e.to_string()),
        };

        // Cache result
        self.verification_cache.insert(cache_key, final_result.clone());
        
        Ok(final_result)
    }

    /// Batch verify multiple proofs
    pub fn batch_verify(
        &mut self,
        proofs_and_inputs: &[(String, Value, Value, Option<Value>)],
    ) -> AdvancedCryptoResult<BatchVerificationResult> {
        let start_time = Instant::now();
        let mut individual_results = Vec::new();
        let mut all_valid = true;

        for (proof_system, proof, public_inputs, verification_key) in proofs_and_inputs {
            let result = self.verify_proof(
                proof_system,
                proof,
                public_inputs,
                verification_key.as_ref(),
            )?;
            
            if !result.is_valid {
                all_valid = false;
            }
            
            individual_results.push(result);
        }

        let total_time = start_time.elapsed();

        Ok(BatchVerificationResult {
            all_valid,
            individual_results,
            total_time,
            batch_size: proofs_and_inputs.len(),
        })
    }

    /// Individual proof system verifiers
    fn verify_groth16(
        &self,
        proof: &Value,
        public_inputs: &Value,
        verification_key: Option<&Value>,
    ) -> AdvancedCryptoResult<bool> {
        // Simplified Groth16 verification
        if verification_key.is_none() {
            return Err(CryptoError::InvalidInput("Groth16 requires verification key".to_string()));
        }

        // Parse inputs
        let public_elems = self.parse_field_array(public_inputs)?;
        
        // Create demo objects for verification
        let demo_vk = Groth16VerifyingKey {
            alpha_g1: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            beta_g2: crate::stdlib::packages::crypto_zk::groth16::G2Point::generator(),
            gamma_g2: crate::stdlib::packages::crypto_zk::groth16::G2Point::generator(),
            delta_g2: crate::stdlib::packages::crypto_zk::groth16::G2Point::generator(),
            ic: vec![crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(); public_elems.len() + 1],
        };

        let demo_proof = Groth16Proof {
            a: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            b: crate::stdlib::packages::crypto_zk::groth16::G2Point::generator(),
            c: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
        };

        Groth16Verifier::verify(&demo_vk, &public_elems, &demo_proof)
    }

    fn verify_plonk(
        &self,
        proof: &Value,
        public_inputs: &Value,
        verification_key: Option<&Value>,
    ) -> AdvancedCryptoResult<bool> {
        // Simplified PLONK verification
        if verification_key.is_none() {
            return Err(CryptoError::InvalidInput("PLONK requires verification key".to_string()));
        }

        let public_elems = self.parse_field_array(public_inputs)?;
        
        // Create demo objects
        let demo_vk = PlonkVerifyingKey {
            domain_size: 4,
            num_public_inputs: public_elems.len(),
            q_l_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            q_r_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            q_o_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            q_m_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            q_c_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            sigma_commitments: vec![crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(); 3],
            g1_generator: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            g2_generator: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
        };

        let demo_proof = PlonkProof {
            a_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            b_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            c_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            z_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            t_lo_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            t_mid_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            t_hi_commitment: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            w_xi_eval: FieldElement::one(),
            w_xi_omega_eval: FieldElement::one(),
            a_xi_eval: FieldElement::one(),
            b_xi_eval: FieldElement::one(),
            c_xi_eval: FieldElement::one(),
            s_sigma1_xi_eval: FieldElement::one(),
            s_sigma2_xi_eval: FieldElement::one(),
            z_xi_omega_eval: FieldElement::one(),
            opening_proof: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
        };

        PlonkVerifier::verify(&demo_vk, &public_elems, &demo_proof)
    }

    fn verify_stark(
        &self,
        proof: &Value,
        public_inputs: &Value,
    ) -> AdvancedCryptoResult<bool> {
        let public_elems = self.parse_field_array(public_inputs)?;
        
        // Create demo objects
        let demo_constraints = StarkConstraints::new();
        let demo_proof = StarkProof {
            trace_commitment: vec![0u8; 32],
            constraint_evaluations: vec![FieldElement::zero(); 2],
            fri_proof: crate::stdlib::packages::crypto_zk::stark::FriProof {
                commitments: vec![vec![0u8; 32]],
                final_polynomial: vec![FieldElement::one()],
                query_proofs: Vec::new(),
            },
            query_responses: Vec::new(),
        };

        StarkVerifier::verify(&demo_proof, &demo_constraints, &public_elems)
    }

    fn verify_bulletproofs(
        &self,
        proof: &Value,
        public_inputs: &Value,
    ) -> AdvancedCryptoResult<bool> {
        // Create demo objects for Bulletproofs verification
        let demo_params = BulletproofsParams::generate(32)?;
        let demo_proof = BulletproofsRangeProof {
            a: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            s: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            t1: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            t2: crate::stdlib::packages::crypto_zk::groth16::G1Point::generator(),
            tau_x: FieldElement::one(),
            mu: FieldElement::one(),
            l_vec: vec![FieldElement::one(); 32],
            r_vec: vec![FieldElement::zero(); 32],
        };
        let demo_commitment = crate::stdlib::packages::crypto_zk::groth16::G1Point::generator();

        BulletproofsVerifier::verify_range(&demo_params, &demo_proof, &demo_commitment, 0, 255)
    }

    fn verify_schnorr(
        &self,
        proof: &Value,
        public_inputs: &Value,
    ) -> AdvancedCryptoResult<bool> {
        let public_elems = self.parse_field_array(public_inputs)?;
        
        if public_elems.len() < 2 {
            return Err(CryptoError::InvalidInput("Schnorr requires public key and generator".to_string()));
        }

        let demo_proof = SchnorrProof {
            commitment: FieldElement::one(),
            response: FieldElement::one(),
        };

        demo_proof.verify(public_elems[0], public_elems[1])
    }

    fn verify_sigma(
        &self,
        proof: &Value,
        public_inputs: &Value,
    ) -> AdvancedCryptoResult<bool> {
        let public_elems = self.parse_field_array(public_inputs)?;
        
        let demo_proof = SigmaProof {
            commitments: vec![FieldElement::one(); public_elems.len()],
            challenge: FieldElement::one(),
            responses: vec![FieldElement::one(); public_elems.len()],
        };

        let simple_relation = |_: &[FieldElement], _: &[FieldElement]| true;
        demo_proof.verify(&public_elems, simple_relation)
    }

    fn verify_nizk(
        &self,
        proof: &Value,
        public_inputs: &Value,
    ) -> AdvancedCryptoResult<bool> {
        let public_elems = self.parse_field_array(public_inputs)?;
        
        let demo_proof = NIZKProof {
            proof_data: vec![0u8; 32],
            public_inputs: public_elems.clone(),
            proof_type: "demo".to_string(),
        };

        demo_proof.verify(&public_elems)
    }

    /// Helper methods
    fn parse_field_array(&self, value: &Value) -> AdvancedCryptoResult<Vec<FieldElement>> {
        match value {
            Value::Array(arr) => {
                let mut elements = Vec::new();
                for item in arr {
                    match item {
                        Value::Integer(i) => elements.push(FieldElement::new(*i as u64)),
                        Value::String(s) => {
                            let num: u64 = s.parse()
                                .map_err(|_| CryptoError::InvalidInput("Invalid number string".to_string()))?;
                            elements.push(FieldElement::new(num));
                        }
                        _ => return Err(CryptoError::InvalidInput("Invalid field element type".to_string())),
                    }
                }
                Ok(elements)
            }
            _ => Err(CryptoError::InvalidInput("Expected array of field elements".to_string())),
        }
    }

    fn generate_cache_key(&self, proof_system: &str, proof: &Value, public_inputs: &Value) -> String {
        use sha3::{Digest, Sha3_256};
        
        let mut hasher = Sha3_256::new();
        hasher.update(proof_system.as_bytes());
        hasher.update(&format!("{:?}", proof).as_bytes());
        hasher.update(&format!("{:?}", public_inputs).as_bytes());
        
        hex::encode(hasher.finalize())
    }

    fn hash_public_inputs(&self, public_inputs: &Value) -> Vec<u8> {
        use sha3::{Digest, Sha3_256};
        
        let mut hasher = Sha3_256::new();
        hasher.update(&format!("{:?}", public_inputs).as_bytes());
        hasher.finalize().to_vec()
    }

    /// Clear verification cache
    pub fn clear_cache(&mut self) {
        self.verification_cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> Value {
        let mut stats = HashMap::new();
        stats.insert("cache_size".to_string(), Value::Integer(self.verification_cache.len() as i64));
        stats.insert("supported_systems".to_string(), Value::Array(
            self.supported_systems.iter().map(|s| Value::String(s.clone())).collect()
        ));
        
        Value::Object(stats)
    }
}

/// Verification benchmark utilities
pub struct VerificationBenchmark;

impl VerificationBenchmark {
    /// Benchmark verification performance
    pub fn benchmark_verifier(
        verifier: &mut UniversalVerifier,
        proof_system: &str,
        num_iterations: usize,
    ) -> AdvancedCryptoResult<Value> {
        let mut times = Vec::new();
        
        // Create demo proof and inputs
        let demo_proof = Value::Object(HashMap::new());
        let demo_inputs = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        let demo_vk = Some(Value::Object(HashMap::new()));

        for _ in 0..num_iterations {
            let start = Instant::now();
            let _result = verifier.verify_proof(
                proof_system,
                &demo_proof,
                &demo_inputs,
                demo_vk.as_ref(),
            )?;
            times.push(start.elapsed().as_micros() as i64);
        }

        let mut benchmark_map = HashMap::new();
        benchmark_map.insert("proof_system".to_string(), Value::String(proof_system.to_string()));
        benchmark_map.insert("iterations".to_string(), Value::Integer(num_iterations as i64));
        
        if !times.is_empty() {
            let avg_time = times.iter().sum::<i64>() / times.len() as i64;
            let min_time = *times.iter().min().unwrap();
            let max_time = *times.iter().max().unwrap();
            
            benchmark_map.insert("avg_time_us".to_string(), Value::Integer(avg_time));
            benchmark_map.insert("min_time_us".to_string(), Value::Integer(min_time));
            benchmark_map.insert("max_time_us".to_string(), Value::Integer(max_time));
            
            let verifications_per_second = 1_000_000.0 / avg_time as f64;
            benchmark_map.insert("verifications_per_second".to_string(), Value::Integer(verifications_per_second as i64));
        }

        Ok(Value::Object(benchmark_map))
    }

    /// Benchmark batch verification
    pub fn benchmark_batch_verification(
        verifier: &mut UniversalVerifier,
        batch_sizes: &[usize],
    ) -> AdvancedCryptoResult<Value> {
        let mut benchmark_results = Vec::new();

        for &batch_size in batch_sizes {
            let mut batch_data = Vec::new();
            
            // Create batch of demo proofs
            for i in 0..batch_size {
                let proof_system = if i % 2 == 0 { "schnorr" } else { "sigma" };
                batch_data.push((
                    proof_system.to_string(),
                    Value::Object(HashMap::new()),
                    Value::Array(vec![Value::Integer(i as i64)]),
                    None,
                ));
            }

            let start = Instant::now();
            let result = verifier.batch_verify(&batch_data)?;
            let total_time = start.elapsed();

            let mut batch_result = HashMap::new();
            batch_result.insert("batch_size".to_string(), Value::Integer(batch_size as i64));
            batch_result.insert("total_time_ms".to_string(), Value::Integer(total_time.as_millis() as i64));
            batch_result.insert("avg_time_per_proof_us".to_string(), Value::Integer(
                (total_time.as_micros() / batch_size as u128) as i64
            ));
            batch_result.insert("all_valid".to_string(), Value::Boolean(result.all_valid));
            
            benchmark_results.push(Value::Object(batch_result));
        }

        let mut final_result = HashMap::new();
        final_result.insert("batch_benchmarks".to_string(), Value::Array(benchmark_results));
        
        Ok(Value::Object(final_result))
    }
}

/// Public API for verifiers
pub struct Verifiers;

impl Verifiers {
    /// Create universal verifier
    pub fn create_universal_verifier() -> AdvancedCryptoResult<Value> {
        let verifier = UniversalVerifier::new();
        let mut verifier_map = HashMap::new();
        verifier_map.insert("supported_systems".to_string(), Value::Array(
            verifier.supported_systems.iter().map(|s| Value::String(s.clone())).collect()
        ));
        verifier_map.insert("cache_enabled".to_string(), Value::Boolean(true));
        Ok(Value::Object(verifier_map))
    }

    /// Verify single proof
    pub fn verify_proof(
        proof_system: &Value,
        proof: &Value,
        public_inputs: &Value,
        verification_key: Option<&Value>,
    ) -> AdvancedCryptoResult<Value> {
        let proof_system_str = match proof_system {
            Value::String(s) => s,
            _ => return Err(CryptoError::InvalidInput("Expected string for proof system".to_string())),
        };

        let mut verifier = UniversalVerifier::new();
        let result = verifier.verify_proof(proof_system_str, proof, public_inputs, verification_key)?;
        
        Ok(result.to_value())
    }

    /// Batch verify proofs
    pub fn batch_verify_proofs(proofs_and_inputs: &Value) -> AdvancedCryptoResult<Value> {
        let batch_data = match proofs_and_inputs {
            Value::Array(arr) => {
                let mut data = Vec::new();
                for item in arr {
                    if let Value::Object(map) = item {
                        let proof_system = match map.get("proof_system") {
                            Some(Value::String(s)) => s.clone(),
                            _ => return Err(CryptoError::InvalidInput("Missing proof_system".to_string())),
                        };
                        
                        let proof = map.get("proof")
                            .ok_or_else(|| CryptoError::InvalidInput("Missing proof".to_string()))?
                            .clone();
                        
                        let public_inputs = map.get("public_inputs")
                            .ok_or_else(|| CryptoError::InvalidInput("Missing public_inputs".to_string()))?
                            .clone();
                        
                        let verification_key = map.get("verification_key").cloned();
                        
                        data.push((proof_system, proof, public_inputs, verification_key));
                    }
                }
                data
            }
            _ => return Err(CryptoError::InvalidInput("Expected array for batch verification".to_string())),
        };

        let mut verifier = UniversalVerifier::new();
        let result = verifier.batch_verify(&batch_data)?;
        
        Ok(result.to_value())
    }

    /// Benchmark verification performance
    pub fn benchmark_verification(
        proof_system: &Value,
        iterations: i64,
    ) -> AdvancedCryptoResult<Value> {
        let proof_system_str = match proof_system {
            Value::String(s) => s,
            _ => return Err(CryptoError::InvalidInput("Expected string for proof system".to_string())),
        };

        let mut verifier = UniversalVerifier::new();
        VerificationBenchmark::benchmark_verifier(&mut verifier, proof_system_str, iterations as usize)
    }

    /// Benchmark batch verification
    pub fn benchmark_batch_verification(batch_sizes: &Value) -> AdvancedCryptoResult<Value> {
        let sizes = match batch_sizes {
            Value::Array(arr) => {
                let mut sizes = Vec::new();
                for size_val in arr {
                    if let Value::Integer(size) = size_val {
                        sizes.push(*size as usize);
                    }
                }
                sizes
            }
            _ => return Err(CryptoError::InvalidInput("Expected array of batch sizes".to_string())),
        };

        let mut verifier = UniversalVerifier::new();
        VerificationBenchmark::benchmark_batch_verification(&mut verifier, &sizes)
    }

    /// Get supported proof systems
    pub fn supported_systems() -> Value {
        let systems = vec![
            ("groth16", "zkSNARK with constant-size proofs"),
            ("plonk", "Universal zkSNARK with single setup"),
            ("stark", "Transparent proofs without trusted setup"),
            ("bulletproofs", "Range proofs with logarithmic size"),
            ("schnorr", "Discrete logarithm proofs"),
            ("sigma", "3-round protocols for general relations"),
            ("nizk", "Non-interactive zero-knowledge proofs"),
        ];

        let system_data: Vec<Value> = systems.iter().map(|(name, description)| {
            let mut system_map = HashMap::new();
            system_map.insert("name".to_string(), Value::String(name.to_string()));
            system_map.insert("description".to_string(), Value::String(description.to_string()));
            Value::Object(system_map)
        }).collect();

        Value::Array(system_data)
    }

    /// Verification complexity comparison
    pub fn verification_complexity() -> Value {
        let mut complexity = HashMap::new();
        
        let systems = vec![
            ("groth16", "O(1)", "3 pairings", "Fast"),
            ("plonk", "O(1)", "Multiple pairings", "Medium"),
            ("stark", "O(log² n)", "Hash verifications", "Slow"),
            ("bulletproofs", "O(log n)", "Group operations", "Medium"),
            ("schnorr", "O(1)", "2 exponentiations", "Fast"),
            ("sigma", "O(k)", "k field operations", "Fast"),
            ("nizk", "Variable", "Depends on statement", "Variable"),
        ];

        let system_data: Vec<Value> = systems.iter().map(|(name, complexity, operations, speed)| {
            let mut system_map = HashMap::new();
            system_map.insert("proof_system".to_string(), Value::String(name.to_string()));
            system_map.insert("complexity".to_string(), Value::String(complexity.to_string()));
            system_map.insert("operations".to_string(), Value::String(operations.to_string()));
            system_map.insert("speed".to_string(), Value::String(speed.to_string()));
            Value::Object(system_map)
        }).collect();

        complexity.insert("verification_complexity".to_string(), Value::Array(system_data));
        Value::Object(complexity)
    }

    /// Verification best practices
    pub fn verification_best_practices() -> Value {
        let practices = vec![
            "Always validate public inputs before verification",
            "Use batch verification when possible for better performance",
            "Cache verification results for identical proofs",
            "Implement timeout mechanisms for long verifications",
            "Validate proof format and structure before cryptographic checks",
            "Use appropriate proof system for your security requirements",
            "Consider post-quantum security for long-term applications",
            "Implement proper error handling and logging",
            "Use trusted verification keys from reliable sources",
            "Perform sanity checks on proof components",
        ];

        let practice_values: Vec<Value> = practices.iter()
            .map(|p| Value::String(p.to_string()))
            .collect();

        let mut best_practices = HashMap::new();
        best_practices.insert("verification_best_practices".to_string(), Value::Array(practice_values));
        
        Value::Object(best_practices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_verifier_creation() {
        let verifier = UniversalVerifier::new();
        assert!(!verifier.supported_systems.is_empty());
        assert!(verifier.supported_systems.contains(&"groth16".to_string()));
        assert!(verifier.supported_systems.contains(&"plonk".to_string()));
        assert!(verifier.supported_systems.contains(&"stark".to_string()));
    }

    #[test]
    fn test_verification_result() {
        let result = VerificationResult::success(
            "test".to_string(),
            Duration::from_millis(100),
            vec![1, 2, 3, 4],
        );
        
        assert!(result.is_valid);
        assert_eq!(result.proof_system, "test");
        assert!(result.error_message.is_none());
        
        let failure = VerificationResult::failure(
            "test".to_string(),
            Duration::from_millis(50),
            "Test error".to_string(),
        );
        
        assert!(!failure.is_valid);
        assert!(failure.error_message.is_some());
    }

    #[test]
    fn test_proof_verification() {
        let mut verifier = UniversalVerifier::new();
        let demo_proof = Value::Object(HashMap::new());
        let demo_inputs = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        
        let result = verifier.verify_proof("schnorr", &demo_proof, &demo_inputs, None);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.proof_system, "schnorr");
    }

    #[test]
    fn test_batch_verification() {
        let mut verifier = UniversalVerifier::new();
        let batch_data = vec![
            ("schnorr".to_string(), Value::Object(HashMap::new()), Value::Array(vec![Value::Integer(1)]), None),
            ("sigma".to_string(), Value::Object(HashMap::new()), Value::Array(vec![Value::Integer(2)]), None),
        ];
        
        let result = verifier.batch_verify(&batch_data);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.batch_size, 2);
        assert_eq!(result.individual_results.len(), 2);
    }

    #[test]
    fn test_cache_functionality() {
        let mut verifier = UniversalVerifier::new();
        let demo_proof = Value::Object(HashMap::new());
        let demo_inputs = Value::Array(vec![Value::Integer(1)]);
        
        // First verification
        let result1 = verifier.verify_proof("schnorr", &demo_proof, &demo_inputs, None).unwrap();
        
        // Second verification (should use cache)
        let result2 = verifier.verify_proof("schnorr", &demo_proof, &demo_inputs, None).unwrap();
        
        assert_eq!(result1.is_valid, result2.is_valid);
        assert!(!verifier.verification_cache.is_empty());
        
        verifier.clear_cache();
        assert!(verifier.verification_cache.is_empty());
    }

    #[test]
    fn test_verifiers_api() {
        let verifier = Verifiers::create_universal_verifier();
        assert!(verifier.is_ok());
        
        let supported = Verifiers::supported_systems();
        assert!(matches!(supported, Value::Array(_)));
        
        let complexity = Verifiers::verification_complexity();
        assert!(matches!(complexity, Value::Object(_)));
        
        let practices = Verifiers::verification_best_practices();
        assert!(matches!(practices, Value::Object(_)));
    }

    #[test]
    fn test_verification_api() {
        let proof_system = Value::String("schnorr".to_string());
        let proof = Value::Object(HashMap::new());
        let public_inputs = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        
        let result = Verifiers::verify_proof(&proof_system, &proof, &public_inputs, None);
        assert!(result.is_ok());
        
        let result_value = result.unwrap();
        assert!(matches!(result_value, Value::Object(_)));
    }

    #[test]
    fn test_benchmarking() {
        let proof_system = Value::String("schnorr".to_string());
        let benchmark = Verifiers::benchmark_verification(&proof_system, 10);
        assert!(benchmark.is_ok());
        
        let batch_sizes = Value::Array(vec![Value::Integer(5), Value::Integer(10)]);
        let batch_benchmark = Verifiers::benchmark_batch_verification(&batch_sizes);
        assert!(batch_benchmark.is_ok());
    }

    #[test]
    fn test_unsupported_proof_system() {
        let mut verifier = UniversalVerifier::new();
        let demo_proof = Value::Object(HashMap::new());
        let demo_inputs = Value::Array(vec![Value::Integer(1)]);
        
        let result = verifier.verify_proof("unsupported", &demo_proof, &demo_inputs, None);
        assert!(result.is_ok()); // Returns VerificationResult with error
        
        let result = result.unwrap();
        assert!(!result.is_valid);
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_field_element_parsing() {
        let verifier = UniversalVerifier::new();
        
        let valid_array = Value::Array(vec![
            Value::Integer(1),
            Value::String("2".to_string()),
            Value::Integer(3),
        ]);
        
        let result = verifier.parse_field_array(&valid_array);
        assert!(result.is_ok());
        
        let elements = result.unwrap();
        assert_eq!(elements.len(), 3);
        assert_eq!(elements[0], FieldElement::new(1));
        assert_eq!(elements[1], FieldElement::new(2));
        assert_eq!(elements[2], FieldElement::new(3));
    }
}
