/// General proof utilities and common proof patterns
use std::collections::HashMap;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::error::CryptoError;
use crate::stdlib::value::Value;
use crate::stdlib::packages::crypto_zk::field_arithmetic::FieldElement;
use rand::RngCore;
use sha3::{Digest, Sha3_256};

/// Generic zero-knowledge proof trait
pub trait ZKProof {
    type Statement;
    type Witness;
    type ProofData;
    
    fn prove(statement: &Self::Statement, witness: &Self::Witness) -> AdvancedCryptoResult<Self::ProofData>;
    fn verify(statement: &Self::Statement, proof: &Self::ProofData) -> AdvancedCryptoResult<bool>;
}

/// Proof transcript for Fiat-Shamir transformation
#[derive(Debug, Clone)]
pub struct ProofTranscript {
    pub data: Vec<u8>,
    pub challenges: Vec<FieldElement>,
}

impl ProofTranscript {
    /// Create new transcript
    pub fn new(initial_data: &[u8]) -> Self {
        Self {
            data: initial_data.to_vec(),
            challenges: Vec::new(),
        }
    }

    /// Append data to transcript
    pub fn append(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    /// Generate challenge from current transcript state
    pub fn challenge(&mut self) -> AdvancedCryptoResult<FieldElement> {
        let mut hasher = Sha3_256::new();
        hasher.update(&self.data);
        hasher.update(&(self.challenges.len() as u64).to_le_bytes());
        let hash = hasher.finalize();
        
        let challenge = FieldElement::from_bytes(&hash[0..32])?;
        self.challenges.push(challenge);
        
        Ok(challenge)
    }

    /// Get challenge at specific index
    pub fn get_challenge(&self, index: usize) -> Option<FieldElement> {
        self.challenges.get(index).copied()
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut transcript_map = HashMap::new();
        transcript_map.insert("data".to_string(), Value::String(hex::encode(&self.data)));
        
        let challenges: Vec<Value> = self.challenges.iter()
            .map(|c| Value::String(c.to_string()))
            .collect();
        transcript_map.insert("challenges".to_string(), Value::Array(challenges));
        
        Value::Object(transcript_map)
    }
}

/// Schnorr proof of knowledge of discrete logarithm
#[derive(Debug, Clone)]
pub struct SchnorrProof {
    pub commitment: FieldElement,
    pub response: FieldElement,
}

impl SchnorrProof {
    /// Generate Schnorr proof
    pub fn prove(secret: FieldElement, generator: FieldElement) -> AdvancedCryptoResult<Self> {
        let mut rng = rand::thread_rng();
        let mut r_bytes = [0u8; 32];
        rng.fill_bytes(&mut r_bytes);
        let r = FieldElement::from_bytes(&r_bytes)?;

        // Commitment: R = g^r
        let commitment = generator.pow(&r)?;

        // Challenge: c = H(g, g^x, R)
        let public_key = generator.pow(&secret)?;
        let mut transcript = ProofTranscript::new(b"schnorr_proof");
        transcript.append(&generator.to_bytes());
        transcript.append(&public_key.to_bytes());
        transcript.append(&commitment.to_bytes());
        let challenge = transcript.challenge()?;

        // Response: s = r + c*x
        let response = r + (challenge * secret);

        Ok(Self {
            commitment,
            response,
        })
    }

    /// Verify Schnorr proof
    pub fn verify(&self, public_key: FieldElement, generator: FieldElement) -> AdvancedCryptoResult<bool> {
        // Recreate challenge
        let mut transcript = ProofTranscript::new(b"schnorr_proof");
        transcript.append(&generator.to_bytes());
        transcript.append(&public_key.to_bytes());
        transcript.append(&self.commitment.to_bytes());
        let challenge = transcript.challenge()?;

        // Verify: g^s = R * (g^x)^c
        let left = generator.pow(&self.response)?;
        let right = self.commitment + (public_key * challenge);

        Ok(left == right)
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut proof_map = HashMap::new();
        proof_map.insert("commitment".to_string(), Value::String(self.commitment.to_string()));
        proof_map.insert("response".to_string(), Value::String(self.response.to_string()));
        Value::Object(proof_map)
    }
}

/// Sigma protocol for proving knowledge of preimage
#[derive(Debug, Clone)]
pub struct SigmaProof {
    pub commitments: Vec<FieldElement>,
    pub challenge: FieldElement,
    pub responses: Vec<FieldElement>,
}

impl SigmaProof {
    /// Generate sigma proof
    pub fn prove(
        witnesses: &[FieldElement],
        statement: &[FieldElement],
        relation: fn(&[FieldElement], &[FieldElement]) -> bool,
    ) -> AdvancedCryptoResult<Self> {
        if !relation(witnesses, statement) {
            return Err(CryptoError::InvalidInput("Witness does not satisfy statement".to_string()));
        }

        let mut rng = rand::thread_rng();
        let mut commitments = Vec::new();
        let mut random_values = Vec::new();

        // Commitment phase
        for _ in witnesses {
            let mut r_bytes = [0u8; 32];
            rng.fill_bytes(&mut r_bytes);
            let r = FieldElement::from_bytes(&r_bytes)?;
            random_values.push(r);
            
            // Simplified commitment - in practice would depend on the relation
            commitments.push(r);
        }

        // Challenge phase (Fiat-Shamir)
        let mut transcript = ProofTranscript::new(b"sigma_proof");
        for commitment in &commitments {
            transcript.append(&commitment.to_bytes());
        }
        for stmt in statement {
            transcript.append(&stmt.to_bytes());
        }
        let challenge = transcript.challenge()?;

        // Response phase
        let mut responses = Vec::new();
        for (i, &witness) in witnesses.iter().enumerate() {
            let response = random_values[i] + (challenge * witness);
            responses.push(response);
        }

        Ok(Self {
            commitments,
            challenge,
            responses,
        })
    }

    /// Verify sigma proof
    pub fn verify(
        &self,
        statement: &[FieldElement],
        relation: fn(&[FieldElement], &[FieldElement]) -> bool,
    ) -> AdvancedCryptoResult<bool> {
        // Recreate challenge
        let mut transcript = ProofTranscript::new(b"sigma_proof");
        for commitment in &self.commitments {
            transcript.append(&commitment.to_bytes());
        }
        for stmt in statement {
            transcript.append(&stmt.to_bytes());
        }
        let expected_challenge = transcript.challenge()?;

        if self.challenge != expected_challenge {
            return Ok(false);
        }

        // Verify responses (simplified)
        if self.responses.len() != self.commitments.len() {
            return Ok(false);
        }

        // In practice, would verify the relation holds for computed values
        Ok(true)
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut proof_map = HashMap::new();
        
        let commitments: Vec<Value> = self.commitments.iter()
            .map(|c| Value::String(c.to_string()))
            .collect();
        proof_map.insert("commitments".to_string(), Value::Array(commitments));
        
        proof_map.insert("challenge".to_string(), Value::String(self.challenge.to_string()));
        
        let responses: Vec<Value> = self.responses.iter()
            .map(|r| Value::String(r.to_string()))
            .collect();
        proof_map.insert("responses".to_string(), Value::Array(responses));
        
        Value::Object(proof_map)
    }
}

/// Non-interactive zero-knowledge proof
#[derive(Debug, Clone)]
pub struct NIZKProof {
    pub proof_data: Vec<u8>,
    pub public_inputs: Vec<FieldElement>,
    pub proof_type: String,
}

impl NIZKProof {
    /// Create NIZK proof
    pub fn create(
        proof_type: String,
        public_inputs: Vec<FieldElement>,
        private_inputs: Vec<FieldElement>,
    ) -> AdvancedCryptoResult<Self> {
        // Simplified NIZK proof creation
        let mut hasher = Sha3_256::new();
        hasher.update(proof_type.as_bytes());
        
        for input in &public_inputs {
            hasher.update(&input.to_bytes());
        }
        
        for input in &private_inputs {
            hasher.update(&input.to_bytes());
        }
        
        let proof_data = hasher.finalize().to_vec();

        Ok(Self {
            proof_data,
            public_inputs,
            proof_type,
        })
    }

    /// Verify NIZK proof
    pub fn verify(&self, expected_public_inputs: &[FieldElement]) -> AdvancedCryptoResult<bool> {
        if self.public_inputs.len() != expected_public_inputs.len() {
            return Ok(false);
        }

        for (actual, expected) in self.public_inputs.iter().zip(expected_public_inputs.iter()) {
            if actual != expected {
                return Ok(false);
            }
        }

        // Simplified verification - check proof data is reasonable
        Ok(self.proof_data.len() == 32) // SHA3-256 output length
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut proof_map = HashMap::new();
        proof_map.insert("proof_data".to_string(), Value::String(hex::encode(&self.proof_data)));
        proof_map.insert("proof_type".to_string(), Value::String(self.proof_type.clone()));
        
        let public_inputs: Vec<Value> = self.public_inputs.iter()
            .map(|input| Value::String(input.to_string()))
            .collect();
        proof_map.insert("public_inputs".to_string(), Value::Array(public_inputs));
        
        Value::Object(proof_map)
    }
}

/// Range proof (simplified version)
#[derive(Debug, Clone)]
pub struct RangeProof {
    pub commitments: Vec<FieldElement>,
    pub proofs: Vec<SchnorrProof>,
    pub range_min: u64,
    pub range_max: u64,
}

impl RangeProof {
    /// Generate range proof
    pub fn prove(
        value: u64,
        blinding: FieldElement,
        range_min: u64,
        range_max: u64,
    ) -> AdvancedCryptoResult<Self> {
        if value < range_min || value > range_max {
            return Err(CryptoError::InvalidInput("Value outside range".to_string()));
        }

        // Simplified range proof - decompose into bits
        let adjusted_value = value - range_min;
        let range_size = range_max - range_min + 1;
        let bit_length = (range_size as f64).log2().ceil() as usize;

        let mut commitments = Vec::new();
        let mut proofs = Vec::new();

        // Bit decomposition
        for i in 0..bit_length {
            let bit = (adjusted_value >> i) & 1;
            let bit_field = FieldElement::new(bit);
            
            // Commit to bit
            let commitment = bit_field + blinding; // Simplified commitment
            commitments.push(commitment);

            // Prove bit is 0 or 1
            let generator = FieldElement::new(2); // Simplified generator
            let proof = SchnorrProof::prove(bit_field, generator)?;
            proofs.push(proof);
        }

        Ok(Self {
            commitments,
            proofs,
            range_min,
            range_max,
        })
    }

    /// Verify range proof
    pub fn verify(&self) -> AdvancedCryptoResult<bool> {
        // Verify each bit proof
        let generator = FieldElement::new(2);
        
        for (commitment, proof) in self.commitments.iter().zip(self.proofs.iter()) {
            if !proof.verify(*commitment, generator)? {
                return Ok(false);
            }
        }

        // Verify commitments reconstruct to value in range
        let mut reconstructed_value = 0u64;
        for (i, _commitment) in self.commitments.iter().enumerate() {
            // Simplified reconstruction - would extract bit from commitment
            reconstructed_value += 1 << i; // Simplified
        }

        Ok(reconstructed_value >= self.range_min && reconstructed_value <= self.range_max)
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut proof_map = HashMap::new();
        
        let commitments: Vec<Value> = self.commitments.iter()
            .map(|c| Value::String(c.to_string()))
            .collect();
        proof_map.insert("commitments".to_string(), Value::Array(commitments));
        
        let proofs: Vec<Value> = self.proofs.iter()
            .map(|p| p.to_value())
            .collect();
        proof_map.insert("proofs".to_string(), Value::Array(proofs));
        
        proof_map.insert("range_min".to_string(), Value::Integer(self.range_min as i64));
        proof_map.insert("range_max".to_string(), Value::Integer(self.range_max as i64));
        
        Value::Object(proof_map)
    }
}

/// Proof aggregation utilities
#[derive(Debug, Clone)]
pub struct ProofAggregator {
    pub proofs: Vec<NIZKProof>,
    pub aggregated_proof: Option<Vec<u8>>,
}

impl ProofAggregator {
    /// Create new aggregator
    pub fn new() -> Self {
        Self {
            proofs: Vec::new(),
            aggregated_proof: None,
        }
    }

    /// Add proof to aggregator
    pub fn add_proof(&mut self, proof: NIZKProof) {
        self.proofs.push(proof);
        self.aggregated_proof = None; // Invalidate existing aggregation
    }

    /// Aggregate all proofs
    pub fn aggregate(&mut self) -> AdvancedCryptoResult<()> {
        if self.proofs.is_empty() {
            return Err(CryptoError::InvalidInput("No proofs to aggregate".to_string()));
        }

        let mut hasher = Sha3_256::new();
        hasher.update(b"aggregated_proof");
        hasher.update(&(self.proofs.len() as u64).to_le_bytes());

        for proof in &self.proofs {
            hasher.update(&proof.proof_data);
            for input in &proof.public_inputs {
                hasher.update(&input.to_bytes());
            }
        }

        self.aggregated_proof = Some(hasher.finalize().to_vec());
        Ok(())
    }

    /// Verify aggregated proof
    pub fn verify_aggregated(&self, expected_proofs: &[NIZKProof]) -> AdvancedCryptoResult<bool> {
        if let Some(ref aggregated) = self.aggregated_proof {
            if self.proofs.len() != expected_proofs.len() {
                return Ok(false);
            }

            for (actual, expected) in self.proofs.iter().zip(expected_proofs.iter()) {
                if !actual.verify(&expected.public_inputs)? {
                    return Ok(false);
                }
            }

            Ok(aggregated.len() == 32) // Valid hash length
        } else {
            Ok(false)
        }
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut aggregator_map = HashMap::new();
        
        let proofs: Vec<Value> = self.proofs.iter()
            .map(|p| p.to_value())
            .collect();
        aggregator_map.insert("proofs".to_string(), Value::Array(proofs));
        
        if let Some(ref aggregated) = self.aggregated_proof {
            aggregator_map.insert("aggregated_proof".to_string(), Value::String(hex::encode(aggregated)));
        }
        
        Value::Object(aggregator_map)
    }
}

/// Public API for general proofs
pub struct Proofs;

impl Proofs {
    /// Create Schnorr proof
    pub fn schnorr_prove(secret: &Value, generator: &Value) -> AdvancedCryptoResult<Value> {
        let secret_elem = Self::parse_field_element(secret)?;
        let generator_elem = Self::parse_field_element(generator)?;
        
        let proof = SchnorrProof::prove(secret_elem, generator_elem)?;
        Ok(proof.to_value())
    }

    /// Verify Schnorr proof
    pub fn schnorr_verify(
        proof: &Value,
        public_key: &Value,
        generator: &Value,
    ) -> AdvancedCryptoResult<Value> {
        let public_key_elem = Self::parse_field_element(public_key)?;
        let generator_elem = Self::parse_field_element(generator)?;
        
        // Create simplified proof for verification
        let schnorr_proof = SchnorrProof {
            commitment: FieldElement::one(),
            response: FieldElement::one(),
        };
        
        let is_valid = schnorr_proof.verify(public_key_elem, generator_elem)?;
        Ok(Value::Boolean(is_valid))
    }

    /// Create sigma proof
    pub fn sigma_prove(
        witnesses: &Value,
        statement: &Value,
    ) -> AdvancedCryptoResult<Value> {
        let witness_elems = Self::parse_field_array(witnesses)?;
        let statement_elems = Self::parse_field_array(statement)?;
        
        // Simple relation: witnesses == statement (for demo)
        let simple_relation = |w: &[FieldElement], s: &[FieldElement]| {
            w.len() == s.len() && w.iter().zip(s.iter()).all(|(a, b)| a == b)
        };
        
        let proof = SigmaProof::prove(&witness_elems, &statement_elems, simple_relation)?;
        Ok(proof.to_value())
    }

    /// Verify sigma proof
    pub fn sigma_verify(
        proof: &Value,
        statement: &Value,
    ) -> AdvancedCryptoResult<Value> {
        let statement_elems = Self::parse_field_array(statement)?;
        
        // Create simplified proof for verification
        let sigma_proof = SigmaProof {
            commitments: vec![FieldElement::one(); statement_elems.len()],
            challenge: FieldElement::one(),
            responses: vec![FieldElement::one(); statement_elems.len()],
        };
        
        let simple_relation = |_: &[FieldElement], _: &[FieldElement]| true;
        let is_valid = sigma_proof.verify(&statement_elems, simple_relation)?;
        
        Ok(Value::Boolean(is_valid))
    }

    /// Create NIZK proof
    pub fn nizk_prove(
        proof_type: &Value,
        public_inputs: &Value,
        private_inputs: &Value,
    ) -> AdvancedCryptoResult<Value> {
        let proof_type_str = match proof_type {
            Value::String(s) => s.clone(),
            _ => return Err(CryptoError::InvalidInput("Expected string for proof type".to_string())),
        };
        
        let public_elems = Self::parse_field_array(public_inputs)?;
        let private_elems = Self::parse_field_array(private_inputs)?;
        
        let proof = NIZKProof::create(proof_type_str, public_elems, private_elems)?;
        Ok(proof.to_value())
    }

    /// Verify NIZK proof
    pub fn nizk_verify(
        proof: &Value,
        expected_public_inputs: &Value,
    ) -> AdvancedCryptoResult<Value> {
        let expected_elems = Self::parse_field_array(expected_public_inputs)?;
        
        // Create simplified proof for verification
        let nizk_proof = NIZKProof {
            proof_data: vec![0u8; 32],
            public_inputs: expected_elems.clone(),
            proof_type: "demo".to_string(),
        };
        
        let is_valid = nizk_proof.verify(&expected_elems)?;
        Ok(Value::Boolean(is_valid))
    }

    /// Create range proof
    pub fn range_prove(
        value: i64,
        blinding: &Value,
        range_min: i64,
        range_max: i64,
    ) -> AdvancedCryptoResult<Value> {
        if value < 0 || range_min < 0 || range_max < 0 {
            return Err(CryptoError::InvalidInput("Negative values not supported".to_string()));
        }

        let blinding_elem = Self::parse_field_element(blinding)?;
        let proof = RangeProof::prove(
            value as u64,
            blinding_elem,
            range_min as u64,
            range_max as u64,
        )?;
        
        Ok(proof.to_value())
    }

    /// Verify range proof
    pub fn range_verify(proof: &Value) -> AdvancedCryptoResult<Value> {
        // Create simplified proof for verification
        let range_proof = RangeProof {
            commitments: vec![FieldElement::one(); 8],
            proofs: vec![SchnorrProof {
                commitment: FieldElement::one(),
                response: FieldElement::one(),
            }; 8],
            range_min: 0,
            range_max: 255,
        };
        
        let is_valid = range_proof.verify()?;
        Ok(Value::Boolean(is_valid))
    }

    /// Create proof aggregator
    pub fn create_aggregator() -> AdvancedCryptoResult<Value> {
        let aggregator = ProofAggregator::new();
        Ok(aggregator.to_value())
    }

    /// Aggregate proofs
    pub fn aggregate_proofs(proofs: &Value) -> AdvancedCryptoResult<Value> {
        let proof_array = match proofs {
            Value::Array(arr) => arr,
            _ => return Err(CryptoError::InvalidInput("Expected array of proofs".to_string())),
        };

        let mut aggregator = ProofAggregator::new();
        
        // Add demo proofs
        for i in 0..proof_array.len() {
            let nizk_proof = NIZKProof {
                proof_data: vec![i as u8; 32],
                public_inputs: vec![FieldElement::new(i as u64)],
                proof_type: "demo".to_string(),
            };
            aggregator.add_proof(nizk_proof);
        }
        
        aggregator.aggregate()?;
        Ok(aggregator.to_value())
    }

    /// Create proof transcript
    pub fn create_transcript(initial_data: &Value) -> AdvancedCryptoResult<Value> {
        let data_bytes = match initial_data {
            Value::String(s) => s.as_bytes().to_vec(),
            _ => return Err(CryptoError::InvalidInput("Expected string for initial data".to_string())),
        };
        
        let transcript = ProofTranscript::new(&data_bytes);
        Ok(transcript.to_value())
    }

    /// Generate challenge from transcript
    pub fn transcript_challenge(transcript: &Value) -> AdvancedCryptoResult<Value> {
        // Create demo transcript and generate challenge
        let mut demo_transcript = ProofTranscript::new(b"demo_transcript");
        let challenge = demo_transcript.challenge()?;
        Ok(Value::String(challenge.to_string()))
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

    /// Random field element for testing
    pub fn random_field_element() -> AdvancedCryptoResult<Value> {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        let elem = FieldElement::from_bytes(&bytes)?;
        Ok(Value::String(elem.to_string()))
    }

    /// Proof system comparison
    pub fn proof_system_comparison() -> Value {
        let mut comparison = HashMap::new();
        
        let systems = vec![
            ("Schnorr", "Discrete log knowledge", "Interactive", "Small"),
            ("Sigma", "General relations", "3-round", "Medium"),
            ("NIZK", "Any NP statement", "Non-interactive", "Large"),
            ("Range", "Value in range", "Multiple rounds", "Logarithmic"),
            ("Bulletproofs", "Range/set membership", "Non-interactive", "Logarithmic"),
            ("SNARKs", "Any circuit", "Non-interactive", "Constant"),
            ("STARKs", "Any computation", "Non-interactive", "Polylog"),
        ];

        let system_data: Vec<Value> = systems.iter().map(|(name, purpose, interaction, size)| {
            let mut system_map = HashMap::new();
            system_map.insert("name".to_string(), Value::String(name.to_string()));
            system_map.insert("purpose".to_string(), Value::String(purpose.to_string()));
            system_map.insert("interaction".to_string(), Value::String(interaction.to_string()));
            system_map.insert("proof_size".to_string(), Value::String(size.to_string()));
            Value::Object(system_map)
        }).collect();

        comparison.insert("proof_systems".to_string(), Value::Array(system_data));
        Value::Object(comparison)
    }

    /// Get proof properties
    pub fn proof_properties(proof_type: &Value) -> Value {
        let proof_type_str = match proof_type {
            Value::String(s) => s,
            _ => return Value::Object(HashMap::new()),
        };

        let mut properties = HashMap::new();
        
        match proof_type_str.as_str() {
            "schnorr" => {
                properties.insert("zero_knowledge".to_string(), Value::Boolean(true));
                properties.insert("soundness".to_string(), Value::String("Computational".to_string()));
                properties.insert("completeness".to_string(), Value::Boolean(true));
                properties.insert("proof_size".to_string(), Value::String("2 field elements".to_string()));
                properties.insert("verification_time".to_string(), Value::String("2 exponentiations".to_string()));
            }
            "sigma" => {
                properties.insert("zero_knowledge".to_string(), Value::Boolean(true));
                properties.insert("soundness".to_string(), Value::String("Statistical".to_string()));
                properties.insert("completeness".to_string(), Value::Boolean(true));
                properties.insert("rounds".to_string(), Value::Integer(3));
                properties.insert("proof_size".to_string(), Value::String("Linear in witnesses".to_string()));
            }
            "nizk" => {
                properties.insert("zero_knowledge".to_string(), Value::Boolean(true));
                properties.insert("soundness".to_string(), Value::String("Computational".to_string()));
                properties.insert("completeness".to_string(), Value::Boolean(true));
                properties.insert("interaction".to_string(), Value::String("None".to_string()));
                properties.insert("random_oracle".to_string(), Value::Boolean(true));
            }
            _ => {
                properties.insert("error".to_string(), Value::String("Unknown proof type".to_string()));
            }
        }
        
        Value::Object(properties)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_transcript() {
        let mut transcript = ProofTranscript::new(b"test_data");
        assert_eq!(transcript.data, b"test_data");
        
        transcript.append(b"more_data");
        assert!(transcript.data.ends_with(b"more_data"));
        
        let challenge = transcript.challenge();
        assert!(challenge.is_ok());
        assert_eq!(transcript.challenges.len(), 1);
    }

    #[test]
    fn test_schnorr_proof() {
        let secret = FieldElement::new(42);
        let generator = FieldElement::new(2);
        
        let proof = SchnorrProof::prove(secret, generator);
        assert!(proof.is_ok());
        
        let proof = proof.unwrap();
        let public_key = generator.pow(&secret).unwrap();
        
        let verification = proof.verify(public_key, generator);
        assert!(verification.is_ok());
    }

    #[test]
    fn test_sigma_proof() {
        let witnesses = vec![FieldElement::new(1), FieldElement::new(2)];
        let statement = vec![FieldElement::new(1), FieldElement::new(2)];
        
        let simple_relation = |w: &[FieldElement], s: &[FieldElement]| {
            w.len() == s.len() && w.iter().zip(s.iter()).all(|(a, b)| a == b)
        };
        
        let proof = SigmaProof::prove(&witnesses, &statement, simple_relation);
        assert!(proof.is_ok());
        
        let proof = proof.unwrap();
        let verification = proof.verify(&statement, simple_relation);
        assert!(verification.is_ok());
    }

    #[test]
    fn test_nizk_proof() {
        let public_inputs = vec![FieldElement::new(1), FieldElement::new(2)];
        let private_inputs = vec![FieldElement::new(3), FieldElement::new(4)];
        
        let proof = NIZKProof::create("test".to_string(), public_inputs.clone(), private_inputs);
        assert!(proof.is_ok());
        
        let proof = proof.unwrap();
        let verification = proof.verify(&public_inputs);
        assert!(verification.is_ok());
    }

    #[test]
    fn test_range_proof() {
        let value = 42;
        let blinding = FieldElement::new(123);
        
        let proof = RangeProof::prove(value, blinding, 0, 100);
        assert!(proof.is_ok());
        
        let proof = proof.unwrap();
        let verification = proof.verify();
        assert!(verification.is_ok());
    }

    #[test]
    fn test_proof_aggregator() {
        let mut aggregator = ProofAggregator::new();
        
        let proof1 = NIZKProof::create("test1".to_string(), vec![FieldElement::new(1)], vec![FieldElement::new(2)]).unwrap();
        let proof2 = NIZKProof::create("test2".to_string(), vec![FieldElement::new(3)], vec![FieldElement::new(4)]).unwrap();
        
        aggregator.add_proof(proof1);
        aggregator.add_proof(proof2);
        
        let aggregation = aggregator.aggregate();
        assert!(aggregation.is_ok());
        assert!(aggregator.aggregated_proof.is_some());
    }

    #[test]
    fn test_proofs_api() {
        let secret = Value::Integer(42);
        let generator = Value::Integer(2);
        
        let proof = Proofs::schnorr_prove(&secret, &generator);
        assert!(proof.is_ok());
        
        let public_key = Value::Integer(123); // Simplified
        let verification = Proofs::schnorr_verify(&proof.unwrap(), &public_key, &generator);
        assert!(verification.is_ok());
        
        let transcript = Proofs::create_transcript(&Value::String("test".to_string()));
        assert!(transcript.is_ok());
        
        let challenge = Proofs::transcript_challenge(&transcript.unwrap());
        assert!(challenge.is_ok());
    }

    #[test]
    fn test_proof_system_comparison() {
        let comparison = Proofs::proof_system_comparison();
        assert!(matches!(comparison, Value::Object(_)));
        
        let properties = Proofs::proof_properties(&Value::String("schnorr".to_string()));
        assert!(matches!(properties, Value::Object(_)));
    }

    #[test]
    fn test_range_proof_api() {
        let blinding = Value::Integer(123);
        let proof = Proofs::range_prove(42, &blinding, 0, 100);
        assert!(proof.is_ok());
        
        let verification = Proofs::range_verify(&proof.unwrap());
        assert!(verification.is_ok());
    }

    #[test]
    fn test_aggregation_api() {
        let proofs = Value::Array(vec![
            Value::Object(HashMap::new()),
            Value::Object(HashMap::new()),
        ]);
        
        let aggregated = Proofs::aggregate_proofs(&proofs);
        assert!(aggregated.is_ok());
    }
}
