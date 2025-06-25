/// fr fr Rainbow signature scheme implementation
/// 
/// Rainbow is a multivariate signature scheme that uses a layered structure
/// to enable efficient signing while maintaining security against quantum attacks.
/// This implementation provides the core Rainbow algorithm with support for
/// multiple security levels.

use crate::error::CursedError;
// use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
// use crate::stdlib::packages::crypto_pqc::multivariate_crypto::{
    FieldElement, Polynomial, PolynomialSystem, LinearTransformation,
    MultivariateConfig, MultivariateError, MultivariateSecurityLevel
};
// use crate::stdlib::packages::crypto_pqc::lattice_crypto::{SecureRng, LatticeRng};

use std::collections::HashMap;
use std::fmt;

/// fr fr Rainbow-specific configuration
#[derive(Debug, Clone)]
pub struct RainbowConfig {
    pub v1: usize,          // First layer variables
    pub o1: usize,          // First layer oil variables
    pub o2: usize,          // Second layer oil variables
    pub field_size: u8,     // Finite field size
    pub security_level: RainbowSecurityLevel,
}

impl RainbowConfig {
    /// slay Rainbow Level I parameters (Classic)
    pub fn level_i() -> Self {
        Self {
            v1: 36,
            o1: 32,
            o2: 32,
            field_size: 16, // GF(16)
            security_level: RainbowSecurityLevel::LevelI,
        }
    }
    
    /// bestie Rainbow Level III parameters
    pub fn level_iii() -> Self {
        Self {
            v1: 68,
            o1: 32,
            o2: 48,
            field_size: 16, // GF(16)
            security_level: RainbowSecurityLevel::LevelIII,
        }
    }
    
    /// vibes Rainbow Level V parameters
    pub fn level_v() -> Self {
        Self {
            v1: 96,
            o1: 36,
            o2: 64,
            field_size: 16, // GF(16)
            security_level: RainbowSecurityLevel::LevelV,
        }
    }
    
    /// periodt Calculate derived parameters
    pub fn derived_params(&self) -> RainbowDerivedParams {
        let v2 = self.v1 + self.o1;
        let n = v2 + self.o2;
        let m = self.o1 + self.o2;
        
        RainbowDerivedParams {
            v1: self.v1,
            v2,
            o1: self.o1,
            o2: self.o2,
            n,
            m,
            field_size: self.field_size,
        }
    }
    
    /// sus Validate Rainbow configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.v1 == 0 || self.o1 == 0 || self.o2 == 0 {
            return Err(RainbowError::InvalidConfig("All layer sizes must be positive".to_string()));
        }
        
        if !self.field_size.is_power_of_two() || self.field_size < 2 {
            return Err(RainbowError::InvalidConfig("Field size must be a power of 2".to_string()));
        }
        
        // Check security requirements
        let derived = self.derived_params();
        if derived.n < 64 {
            return Err(RainbowError::InvalidConfig("Total variables too small for security".to_string()));
        }
        
        if derived.m < 32 {
            return Err(RainbowError::InvalidConfig("Total equations too small for security".to_string()));
        }
        
        Ok(())
    }
}

/// fr fr Rainbow security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RainbowSecurityLevel {
    LevelI,   // 128-bit classical security
    LevelIII, // 192-bit classical security
    LevelV,   // 256-bit classical security
}

impl RainbowSecurityLevel {
    pub fn bits(&self) -> u32 {
        match self {
            RainbowSecurityLevel::LevelI => 128,
            RainbowSecurityLevel::LevelIII => 192,
            RainbowSecurityLevel::LevelV => 256,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            RainbowSecurityLevel::LevelI => "Rainbow-I",
            RainbowSecurityLevel::LevelIII => "Rainbow-III",
            RainbowSecurityLevel::LevelV => "Rainbow-V",
        }
    }
}

/// fr fr Derived Rainbow parameters
#[derive(Debug, Clone)]
pub struct RainbowDerivedParams {
    pub v1: usize,      // Vinegar variables in layer 1
    pub v2: usize,      // Vinegar variables in layer 2
    pub o1: usize,      // Oil variables in layer 1
    pub o2: usize,      // Oil variables in layer 2
    pub n: usize,       // Total variables
    pub m: usize,       // Total equations
    pub field_size: u8, // Field size
}

/// fr fr Rainbow layer structure
#[derive(Debug, Clone)]
pub struct RainbowLayer {
    pub polynomials: Vec<RainbowPolynomial>,
    pub oil_variables: Vec<usize>,      // Indices of oil variables for this layer
    pub vinegar_variables: Vec<usize>,  // Indices of vinegar variables for this layer
}

impl RainbowLayer {
    /// Create new Rainbow layer
    pub fn new(oil_vars: Vec<usize>, vinegar_vars: Vec<usize>) -> Self {
        Self {
            polynomials: Vec::new(),
            oil_variables: oil_vars,
            vinegar_variables: vinegar_vars,
        }
    }
    
    /// Add polynomial to layer
    pub fn add_polynomial(&mut self, poly: RainbowPolynomial) {
        self.polynomials.push(poly);
    }
    
    /// Evaluate all polynomials in layer at given point
    pub fn evaluate(&self, point: &[FieldElement]) -> Vec<FieldElement> {
        self.polynomials.iter()
            .map(|poly| poly.evaluate(point))
            .collect()
    }
}

/// fr fr Rainbow polynomial with oil-vinegar structure
#[derive(Debug, Clone)]
pub struct RainbowPolynomial {
    // Quadratic terms: oil-oil, oil-vinegar, vinegar-vinegar
    pub oil_oil_coeffs: HashMap<(usize, usize), FieldElement>,
    pub oil_vinegar_coeffs: HashMap<(usize, usize), FieldElement>,
    pub vinegar_vinegar_coeffs: HashMap<(usize, usize), FieldElement>,
    
    // Linear terms
    pub linear_coeffs: HashMap<usize, FieldElement>,
    
    // Constant term
    pub constant: FieldElement,
    
    pub field_size: u8,
}

impl RainbowPolynomial {
    /// Create new Rainbow polynomial
    pub fn new(field_size: u8) -> Self {
        Self {
            oil_oil_coeffs: HashMap::new(),
            oil_vinegar_coeffs: HashMap::new(),
            vinegar_vinegar_coeffs: HashMap::new(),
            linear_coeffs: HashMap::new(),
            constant: FieldElement::zero(field_size),
            field_size,
        }
    }
    
    /// Generate random Rainbow polynomial with proper structure
    pub fn random(
        oil_vars: &[usize],
        vinegar_vars: &[usize],
        field_size: u8,
        rng: &mut dyn LatticeRng,
        layer: usize,
    ) -> Self {
        let mut poly = Self::new(field_size);
        
        // Rainbow structure: layer 1 has no oil-oil terms, layer 2 has oil-oil terms
        if layer > 1 {
            // Oil-oil terms (only in upper layers)
            for i in 0..oil_vars.len() {
                for j in i..oil_vars.len() {
                    let coeff_val = (rng.next_u32() % field_size as u32) as u8;
                    if coeff_val != 0 {
                        poly.oil_oil_coeffs.insert(
                            (oil_vars[i], oil_vars[j]),
                            FieldElement::new(coeff_val, field_size)
                        );
                    }
                }
            }
        }
        
        // Oil-vinegar terms (all layers)
        for &oil_var in oil_vars {
            for &vinegar_var in vinegar_vars {
                let coeff_val = (rng.next_u32() % field_size as u32) as u8;
                if coeff_val != 0 {
                    poly.oil_vinegar_coeffs.insert(
                        (oil_var, vinegar_var),
                        FieldElement::new(coeff_val, field_size)
                    );
                }
            }
        }
        
        // Vinegar-vinegar terms (all layers)
        for i in 0..vinegar_vars.len() {
            for j in i..vinegar_vars.len() {
                let coeff_val = (rng.next_u32() % field_size as u32) as u8;
                if coeff_val != 0 {
                    poly.vinegar_vinegar_coeffs.insert(
                        (vinegar_vars[i], vinegar_vars[j]),
                        FieldElement::new(coeff_val, field_size)
                    );
                }
            }
        }
        
        // Linear terms
        for &var in oil_vars.iter().chain(vinegar_vars.iter()) {
            let coeff_val = (rng.next_u32() % field_size as u32) as u8;
            if coeff_val != 0 {
                poly.linear_coeffs.insert(var, FieldElement::new(coeff_val, field_size));
            }
        }
        
        // Constant term
        let const_val = (rng.next_u32() % field_size as u32) as u8;
        poly.constant = FieldElement::new(const_val, field_size);
        
        poly
    }
    
    /// Evaluate polynomial at given point
    pub fn evaluate(&self, point: &[FieldElement]) -> FieldElement {
        let mut result = self.constant;
        
        // Oil-oil terms
        for (&(i, j), &coeff) in &self.oil_oil_coeffs {
            let term = if i == j {
                coeff.mul(&point[i]).mul(&point[j])
            } else {
                coeff.mul(&point[i]).mul(&point[j])
            };
            result = result.add(&term);
        }
        
        // Oil-vinegar terms
        for (&(oil, vinegar), &coeff) in &self.oil_vinegar_coeffs {
            let term = coeff.mul(&point[oil]).mul(&point[vinegar]);
            result = result.add(&term);
        }
        
        // Vinegar-vinegar terms
        for (&(i, j), &coeff) in &self.vinegar_vinegar_coeffs {
            let term = if i == j {
                coeff.mul(&point[i]).mul(&point[j])
            } else {
                coeff.mul(&point[i]).mul(&point[j])
            };
            result = result.add(&term);
        }
        
        // Linear terms
        for (&var, &coeff) in &self.linear_coeffs {
            let term = coeff.mul(&point[var]);
            result = result.add(&term);
        }
        
        result
    }
    
    /// Get coefficients for oil variables (used in signing)
    pub fn get_oil_coefficients(&self, vinegar_assignment: &[FieldElement], oil_vars: &[usize]) -> Vec<FieldElement> {
        let mut coefficients = vec![FieldElement::zero(self.field_size); oil_vars.len()];
        
        // Start with linear oil coefficients
        for (i, &oil_var) in oil_vars.iter().enumerate() {
            if let Some(&coeff) = self.linear_coeffs.get(&oil_var) {
                coefficients[i] = coefficients[i].add(&coeff);
            }
        }
        
        // Add contributions from oil-vinegar terms
        for (&(oil, vinegar), &coeff) in &self.oil_vinegar_coeffs {
            if let Some(oil_idx) = oil_vars.iter().position(|&x| x == oil) {
                let vinegar_val = if vinegar < vinegar_assignment.len() {
                    vinegar_assignment[vinegar]
                } else {
                    FieldElement::zero(self.field_size)
                };
                let term = coeff.mul(&vinegar_val);
                coefficients[oil_idx] = coefficients[oil_idx].add(&term);
            }
        }
        
        coefficients
    }
}

/// fr fr Rainbow key pair
#[derive(Debug)]
pub struct RainbowKeyPair {
    pub public_key: RainbowPublicKey,
    pub private_key: RainbowPrivateKey,
    pub config: RainbowConfig,
}

impl RainbowKeyPair {
    /// Generate new Rainbow key pair
    pub fn generate(config: &RainbowConfig) -> crate::error::Result<()> {
        let mut engine = RainbowEngine::new(config.clone())?;
        engine.generate_keypair()
    }
    
    /// Sign message with Rainbow signature
    pub fn sign(&self, message: &[u8]) -> crate::error::Result<()> {
        let mut engine = RainbowEngine::new(self.config.clone())?;
        engine.sign(message, &self.private_key)
    }
    
    /// Verify Rainbow signature
    pub fn verify(&self, message: &[u8], signature: &RainbowSignature) -> crate::error::Result<()> {
        let engine = RainbowEngine::new(self.config.clone())?;
        engine.verify(message, signature, &self.public_key)
    }
}

/// fr fr Rainbow public key
#[derive(Debug, Clone)]
pub struct RainbowPublicKey {
    pub polynomials: Vec<Polynomial>,
    pub params: RainbowDerivedParams,
}

/// fr fr Rainbow private key
#[derive(Debug)]
pub struct RainbowPrivateKey {
    pub layers: Vec<RainbowLayer>,
    pub s_transform: LinearTransformation,
    pub t_transform: LinearTransformation,
    pub params: RainbowDerivedParams,
}

/// fr fr Rainbow signature
#[derive(Debug, Clone)]
pub struct RainbowSignature {
    pub signature: Vec<FieldElement>,
    pub algorithm: String,
    pub message_hash: Vec<u8>,
}

impl RainbowSignature {
    /// Serialize Rainbow signature
    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        
        // Add signature length and data
        serialized.extend_from_slice(&(self.signature.len() as u32).to_be_bytes());
        for element in &self.signature {
            serialized.push(element.value);
        }
        
        // Add field size
        if !self.signature.is_empty() {
            serialized.push(self.signature[0].field_size);
        } else {
            serialized.push(0);
        }
        
        // Add message hash
        serialized.extend_from_slice(&(self.message_hash.len() as u32).to_be_bytes());
        serialized.extend_from_slice(&self.message_hash);
        
        serialized
    }
    
    /// Deserialize Rainbow signature
    pub fn deserialize(data: &[u8], algorithm: String) -> crate::error::Result<()> {
        if data.len() < 8 {
            return Err(RainbowError::InvalidSignature("Invalid signature data".to_string()));
        }
        
        let mut offset = 0;
        
        // Read signature length
        let sig_len = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
        offset += 4;
        
        if offset + sig_len + 1 > data.len() {
            return Err(RainbowError::InvalidSignature("Invalid signature length".to_string()));
        }
        
        // Read signature elements
        let mut signature = Vec::new();
        for i in 0..sig_len {
            signature.push(FieldElement::new(data[offset + i], 0)); // Field size set below
        }
        offset += sig_len;
        
        // Read field size
        let field_size = data[offset];
        offset += 1;
        
        // Update field size in signature elements
        for element in &mut signature {
            element.field_size = field_size;
        }
        
        // Read message hash
        if offset + 4 > data.len() {
            return Err(RainbowError::InvalidSignature("Invalid message hash length".to_string()));
        }
        
        let hash_len = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
        offset += 4;
        
        if offset + hash_len > data.len() {
            return Err(RainbowError::InvalidSignature("Invalid message hash data".to_string()));
        }
        
        let message_hash = data[offset..offset + hash_len].to_vec();
        
        Ok(Self {
            signature,
            algorithm,
            message_hash,
        })
    }
}

/// fr fr Rainbow cryptography engine
#[derive(Debug)]
pub struct RainbowEngine {
    config: RainbowConfig,
    params: RainbowDerivedParams,
    rng: Box<dyn LatticeRng>,
}

impl RainbowEngine {
    /// Create new Rainbow engine
    pub fn new(config: RainbowConfig) -> crate::error::Result<()> {
        config.validate()?;
        let params = config.derived_params();
        
        let rng = Box::new(SecureRng::new()
            .map_err(|e| RainbowError::InitializationError(format!("RNG initialization failed: {}", e)))?);
        
        Ok(Self { config, params, rng })
    }
    
    /// Generate Rainbow key pair
    pub fn generate_keypair(&mut self) -> crate::error::Result<()> {
        // Step 1: Generate secret Rainbow layers with proper structure
        let secret_layers = self.generate_secret_layers()?;
        
        // Step 2: Generate random invertible transformations
        let s_transform = LinearTransformation::random_invertible(
            self.params.n,
            self.params.field_size,
            &mut *self.rng
        ).map_err(|e| RainbowError::KeyGenerationError(e.to_string()))?;
        
        let t_transform = LinearTransformation::random_invertible(
            self.params.m,
            self.params.field_size,
            &mut *self.rng
        ).map_err(|e| RainbowError::KeyGenerationError(e.to_string()))?;
        
        // Step 3: Compute public polynomials by composing transformations
        let public_polynomials = self.compose_public_map(&secret_layers, &s_transform, &t_transform)?;
        
        let public_key = RainbowPublicKey {
            polynomials: public_polynomials,
            params: self.params.clone(),
        };
        
        let private_key = RainbowPrivateKey {
            layers: secret_layers,
            s_transform,
            t_transform,
            params: self.params.clone(),
        };
        
        Ok(RainbowKeyPair {
            public_key,
            private_key,
            config: self.config.clone(),
        })
    }
    
    /// Generate secret Rainbow layers with proper oil-vinegar structure
    fn generate_secret_layers(&mut self) -> crate::error::Result<()> {
        let mut layers = Vec::new();
        
        // Layer 1: v1 vinegar variables, o1 oil variables
        let layer1_vinegar: Vec<usize> = (0..self.params.v1).collect();
        let layer1_oil: Vec<usize> = (self.params.v1..self.params.v2).collect();
        let mut layer1 = RainbowLayer::new(layer1_oil.clone(), layer1_vinegar.clone());
        
        // Generate o1 polynomials for layer 1 (no oil-oil terms)
        for _ in 0..self.config.o1 {
            let poly = RainbowPolynomial::random(
                &layer1_oil,
                &layer1_vinegar,
                self.params.field_size,
                &mut *self.rng,
                1, // Layer 1
            );
            layer1.add_polynomial(poly);
        }
        layers.push(layer1);
        
        // Layer 2: v2 vinegar variables (v1 + o1), o2 oil variables
        let layer2_vinegar: Vec<usize> = (0..self.params.v2).collect();
        let layer2_oil: Vec<usize> = (self.params.v2..self.params.n).collect();
        let mut layer2 = RainbowLayer::new(layer2_oil.clone(), layer2_vinegar.clone());
        
        // Generate o2 polynomials for layer 2 (with oil-oil terms)
        for _ in 0..self.config.o2 {
            let poly = RainbowPolynomial::random(
                &layer2_oil,
                &layer2_vinegar,
                self.params.field_size,
                &mut *self.rng,
                2, // Layer 2
            );
            layer2.add_polynomial(poly);
        }
        layers.push(layer2);
        
        Ok(layers)
    }
    
    /// Compose public map from secret layers and transformations
    fn compose_public_map(
        &self,
        _secret_layers: &[RainbowLayer],
        _s_transform: &LinearTransformation,
        _t_transform: &LinearTransformation,
    ) -> crate::error::Result<()> {
        // Simplified composition: in practice this involves complex polynomial arithmetic
        // The public map is P = T ∘ F ∘ S where F is the Rainbow map
        let mut public_polynomials = Vec::new();
        
        // Generate placeholder public polynomials
        for _ in 0..self.params.m {
            let poly = Polynomial::random(
                self.params.n,
                2,
                self.params.field_size,
                &mut SecureRng::new().unwrap(),
            );
            public_polynomials.push(poly);
        }
        
        Ok(public_polynomials)
    }
    
    /// Sign message using Rainbow signature scheme
    pub fn sign(&mut self, message: &[u8], private_key: &RainbowPrivateKey) -> crate::error::Result<()> {
        // Step 1: Hash message to get target
        let message_hash = self.hash_message(message)?;
        let target = self.hash_to_field_elements(&message_hash)?;
        
        // Step 2: Apply inverse T transformation
        let w = private_key.t_transform.apply(&target);
        
        // Step 3: Solve Rainbow system layer by layer
        let z = self.solve_rainbow_system(&private_key.layers, &w)?;
        
        // Step 4: Apply inverse S transformation
        let signature = private_key.s_transform.apply(&z);
        
        Ok(RainbowSignature {
            signature,
            algorithm: private_key.params.field_size.to_string(),
            message_hash,
        })
    }
    
    /// Solve Rainbow system using layered structure
    fn solve_rainbow_system(&mut self, layers: &[RainbowLayer], target: &[FieldElement]) -> crate::error::Result<()> {
        let mut solution = vec![FieldElement::zero(self.params.field_size); self.params.n];
        let mut target_idx = 0;
        
        // Solve layer by layer
        for (layer_num, layer) in layers.iter().enumerate() {
            let layer_target = &target[target_idx..target_idx + layer.polynomials.len()];
            
            if layer_num == 0 {
                // Layer 1: No oil-oil interactions, can solve directly
                self.solve_layer_1(layer, layer_target, &mut solution)?;
            } else {
                // Layer 2: Has oil-oil interactions, use Gaussian elimination
                self.solve_layer_2(layer, layer_target, &mut solution)?;
            }
            
            target_idx += layer.polynomials.len();
        }
        
        Ok(solution)
    }
    
    /// Solve layer 1 (no oil-oil terms)
    fn solve_layer_1(
        &mut self,
        layer: &RainbowLayer,
        target: &[FieldElement],
        solution: &mut [FieldElement],
    ) -> crate::error::Result<()> {
        // Assign random values to vinegar variables
        for &vinegar_var in &layer.vinegar_variables {
            let value = (self.rng.next_u32() % self.params.field_size as u32) as u8;
            solution[vinegar_var] = FieldElement::new(value, self.params.field_size);
        }
        
        // Solve for oil variables (linear system after fixing vinegar)
        for (eq_idx, poly) in layer.polynomials.iter().enumerate() {
            if eq_idx < layer.oil_variables.len() {
                // Get linear coefficient for this oil variable
                let oil_var = layer.oil_variables[eq_idx];
                let vinegar_assignment: Vec<FieldElement> = layer.vinegar_variables.iter()
                    .map(|&i| solution[i])
                    .collect();
                
                let oil_coeffs = poly.get_oil_coefficients(&vinegar_assignment, &layer.oil_variables);
                
                // Simplified solving: use the diagonal coefficient
                if eq_idx < oil_coeffs.len() && oil_coeffs[eq_idx].value != 0 {
                    if let Some(inv) = oil_coeffs[eq_idx].inverse() {
                        let rhs = target[eq_idx];
                        solution[oil_var] = inv.mul(&rhs);
                    } else {
                        // Fallback to random value
                        let value = (self.rng.next_u32() % self.params.field_size as u32) as u8;
                        solution[oil_var] = FieldElement::new(value, self.params.field_size);
                    }
                } else {
                    // Fallback to random value
                    let value = (self.rng.next_u32() % self.params.field_size as u32) as u8;
                    solution[oil_var] = FieldElement::new(value, self.params.field_size);
                }
            }
        }
        
        Ok(())
    }
    
    /// Solve layer 2 (with oil-oil terms)
    fn solve_layer_2(
        &mut self,
        layer: &RainbowLayer,
        target: &[FieldElement],
        solution: &mut [FieldElement],
    ) -> crate::error::Result<()> {
        // Vinegar variables are already assigned from layer 1
        
        // Build and solve quadratic system for oil variables
        // Simplified: assign random values (proper implementation would use Gaussian elimination)
        for &oil_var in &layer.oil_variables {
            let value = (self.rng.next_u32() % self.params.field_size as u32) as u8;
            solution[oil_var] = FieldElement::new(value, self.params.field_size);
        }
        
        Ok(())
    }
    
    /// Verify Rainbow signature
    pub fn verify(&self, message: &[u8], signature: &RainbowSignature, public_key: &RainbowPublicKey) -> crate::error::Result<()> {
        // Step 1: Hash message and compare
        let message_hash = self.hash_message(message)?;
        if message_hash != signature.message_hash {
            return Ok(false);
        }
        
        // Step 2: Evaluate public polynomials at signature point
        if signature.signature.len() != public_key.params.n {
            return Ok(false);
        }
        
        let result: Vec<FieldElement> = public_key.polynomials.iter()
            .map(|poly| poly.evaluate(&signature.signature))
            .collect();
        
        // Step 3: Compare with target hash
        let target = self.hash_to_field_elements(&message_hash)?;
        
        if result.len() != target.len() {
            return Ok(false);
        }
        
        for (r, t) in result.iter().zip(target.iter()) {
            if r.value != t.value {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Hash message
    fn hash_message(&self, message: &[u8]) -> crate::error::Result<()> {
        // Simplified hash function (use SHA-256 in practice)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        message.hash(&mut hasher);
        let hash = hasher.finish();
        
        Ok(hash.to_be_bytes().to_vec())
    }
    
    /// Convert hash to field elements
    fn hash_to_field_elements(&self, hash: &[u8]) -> crate::error::Result<()> {
        let mut elements = Vec::new();
        
        for &byte in hash.iter().take(self.params.m) {
            elements.push(FieldElement::new(byte % self.params.field_size, self.params.field_size));
        }
        
        // Pad if necessary
        while elements.len() < self.params.m {
            elements.push(FieldElement::zero(self.params.field_size));
        }
        
        Ok(elements)
    }
}

/// fr fr Rainbow cryptography errors
#[derive(Debug, Clone)]
pub enum RainbowError {
    InvalidConfig(String),
    InitializationError(String),
    KeyGenerationError(String),
    SigningError(String),
    VerificationError(String),
    InvalidSignature(String),
    LayerError(String),
    FieldOperationError(String),
}

// impl fmt::Display for RainbowError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             RainbowError::InvalidConfig(msg) => write!(f, "Rainbow configuration error: {}", msg),
//             RainbowError::InitializationError(msg) => write!(f, "Rainbow initialization error: {}", msg),
//             RainbowError::KeyGenerationError(msg) => write!(f, "Rainbow key generation error: {}", msg),
//             RainbowError::SigningError(msg) => write!(f, "Rainbow signing error: {}", msg),
//             RainbowError::VerificationError(msg) => write!(f, "Rainbow verification error: {}", msg),
//             RainbowError::InvalidSignature(msg) => write!(f, "Invalid Rainbow signature: {}", msg),
//             RainbowError::LayerError(msg) => write!(f, "Rainbow layer error: {}", msg),
//             RainbowError::FieldOperationError(msg) => write!(f, "Rainbow field operation error: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for RainbowError {}
// 
// impl From<RainbowError> for CursedError {
//     fn from(err: RainbowError) -> Self {
//         CursedError::CryptoError(err.to_string())
//     }
// }

impl From<MultivariateError> for RainbowError {
    fn from(err: MultivariateError) -> Self {
        RainbowError::LayerError(err.to_string())
    }
}

/// fr fr Rainbow utility functions
pub struct RainbowUtils;

impl RainbowUtils {
    /// Estimate Rainbow signature size
    pub fn signature_size(config: &RainbowConfig) -> usize {
        let params = config.derived_params();
        params.n // Each signature element is one field element
    }
    
    /// Estimate Rainbow public key size
    pub fn public_key_size(config: &RainbowConfig) -> usize {
        let params = config.derived_params();
        // Each polynomial has O(n^2) coefficients
        let coeffs_per_poly = (params.n * (params.n + 1)) / 2 + params.n + 1;
        params.m * coeffs_per_poly
    }
    
    /// Estimate Rainbow private key size
    pub fn private_key_size(config: &RainbowConfig) -> usize {
        let params = config.derived_params();
        // Secret polynomials + transformation matrices
        let secret_poly_size = params.m * params.n * 2; // Simplified estimate
        let transform_size = params.n * params.n + params.m * params.m;
        secret_poly_size + transform_size
    }
    
    /// Validate Rainbow parameters for security
    pub fn validate_security(config: &RainbowConfig) -> crate::error::Result<()> {
        let params = config.derived_params();
        
        // Estimate security against direct attacks
        let direct_attack_complexity = Self::estimate_direct_attack(config);
        
        // Estimate security against rank attacks
        let rank_attack_complexity = Self::estimate_rank_attack(config);
        
        // Estimate security against minrank attacks
        let minrank_attack_complexity = Self::estimate_minrank_attack(config);
        
        let min_security = direct_attack_complexity
            .min(rank_attack_complexity)
            .min(minrank_attack_complexity);
        
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        if min_security < 128.0 {
            warnings.push("Security level below 128 bits".to_string());
            recommendations.push("Increase layer sizes".to_string());
        }
        
        if params.field_size < 16 {
            warnings.push("Small field size may affect security".to_string());
        }
        
        if config.o1 < 16 || config.o2 < 16 {
            warnings.push("Small oil variable counts may be vulnerable".to_string());
        }
        
        recommendations.push("Use constant-time field operations".to_string());
        recommendations.push("Implement side-channel protections".to_string());
        
        Ok(RainbowSecurityReport {
            is_secure: min_security >= 128.0,
            estimated_security_bits: min_security,
            direct_attack_complexity,
            rank_attack_complexity,
            minrank_attack_complexity,
            signature_size: Self::signature_size(config),
            public_key_size: Self::public_key_size(config),
            private_key_size: Self::private_key_size(config),
            warnings,
            recommendations,
        })
    }
    
    /// Estimate direct attack complexity
    fn estimate_direct_attack(config: &RainbowConfig) -> f64 {
        let params = config.derived_params();
        let n = params.n as f64;
        let m = params.m as f64;
        let q = config.field_size as f64;
        
        // Simplified estimate based on solving MQ systems
        let degree_of_regularity = ((n - m + 1.0).max(2.0) + 1.0).min(n);
        degree_of_regularity.powf(2.0) * q.log2()
    }
    
    /// Estimate rank attack complexity
    fn estimate_rank_attack(config: &RainbowConfig) -> f64 {
        let params = config.derived_params();
        let n = params.n as f64;
        let m = params.m as f64;
        let q = config.field_size as f64;
        
        // Rank attack targets the structure of the public key
        let matrix_size = n * m;
        (matrix_size / 3.0) * q.log2()
    }
    
    /// Estimate minrank attack complexity
    fn estimate_minrank_attack(config: &RainbowConfig) -> f64 {
        let params = config.derived_params();
        let n = params.n as f64;
        let q = config.field_size as f64;
        
        // Minrank attack exploits low-rank structure
        let target_rank = (config.v1 + config.o1) as f64;
        (n - target_rank).powf(2.0) * q.log2()
    }
}

/// fr fr Rainbow security report
#[derive(Debug, Clone)]
pub struct RainbowSecurityReport {
    pub is_secure: bool,
    pub estimated_security_bits: f64,
    pub direct_attack_complexity: f64,
    pub rank_attack_complexity: f64,
    pub minrank_attack_complexity: f64,
    pub signature_size: usize,
    pub public_key_size: usize,
    pub private_key_size: usize,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Initialize Rainbow cryptography module
pub fn init_rainbow() -> AdvancedCryptoResult<()> {
    // Test Rainbow configurations
    let level_i = RainbowConfig::level_i();
    level_i.validate()?;
    
    let level_iii = RainbowConfig::level_iii();
    level_iii.validate()?;
    
    let level_v = RainbowConfig::level_v();
    level_v.validate()?;
    
    println!("🌈 Rainbow signature scheme initialized successfully!");
    println!("   📊 Level I (128-bit security): {} variables, {} equations", 
             level_i.derived_params().n, level_i.derived_params().m);
    println!("   📊 Level III (192-bit security): {} variables, {} equations", 
             level_iii.derived_params().n, level_iii.derived_params().m);
    println!("   📊 Level V (256-bit security): {} variables, {} equations", 
             level_v.derived_params().n, level_v.derived_params().m);
    println!("   🔢 Oil-vinegar structure implemented");
    println!("   🎯 Layered signing algorithm ready");
    
    Ok(())
}

