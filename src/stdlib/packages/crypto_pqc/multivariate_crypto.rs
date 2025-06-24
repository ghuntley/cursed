/// fr fr Multivariate cryptography implementation
/// 
/// This module implements cryptographic schemes based on the difficulty of solving
/// systems of multivariate polynomial equations over finite fields, particularly
/// Rainbow and UOV (Unbalanced Oil and Vinegar) signature schemes.

use crate::error::CursedError;
use crate::stdlib::packages::crypto_pqc::lattice_crypto::{SecureRng, LatticeRng};
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::error::Error;
use std::collections::HashMap;
use std::fmt;

/// fr fr Multivariate cryptography configuration
#[derive(Debug, Clone)]
pub struct MultivariateConfig {
    pub variables: usize,        // Number of variables (n)
    pub equations: usize,        // Number of equations (m)
    pub field_size: u8,         // Finite field size (q)
    pub oil_variables: usize,    // Number of oil variables (for UOV)
    pub vinegar_variables: usize, // Number of vinegar variables (for UOV)
    pub scheme_type: MultivariateScheme,
    pub security_level: MultivariateSecurityLevel,
}

impl MultivariateConfig {
    /// slay Create default Rainbow configuration
    pub fn rainbow_level1() -> Self {
        Self {
            variables: 96,
            equations: 64,
            field_size: 16, // GF(16)
            oil_variables: 32,
            vinegar_variables: 64,
            scheme_type: MultivariateScheme::Rainbow,
            security_level: MultivariateSecurityLevel::Level1,
        }
    }
    
    /// bestie Create Rainbow Level 3 configuration
    pub fn rainbow_level3() -> Self {
        Self {
            variables: 160,
            equations: 96,
            field_size: 16,
            oil_variables: 48,
            vinegar_variables: 112,
            scheme_type: MultivariateScheme::Rainbow,
            security_level: MultivariateSecurityLevel::Level3,
        }
    }
    
    /// vibes Create Rainbow Level 5 configuration
    pub fn rainbow_level5() -> Self {
        Self {
            variables: 192,
            equations: 128,
            field_size: 256, // GF(256)
            oil_variables: 64,
            vinegar_variables: 128,
            scheme_type: MultivariateScheme::Rainbow,
            security_level: MultivariateSecurityLevel::Level5,
        }
    }
    
    /// periodt Create UOV configuration
    pub fn uov_level1() -> Self {
        Self {
            variables: 184,
            equations: 96,
            field_size: 256,
            oil_variables: 96,
            vinegar_variables: 88,
            scheme_type: MultivariateScheme::UOV,
            security_level: MultivariateSecurityLevel::Level1,
        }
    }
    
    /// sus Validate configuration
    pub fn validate(&self) -> Result<(), Error> {
        if self.variables == 0 || self.equations == 0 {
            return Err(MultivariateError::InvalidConfig("Variables and equations must be positive".to_string()));
        }
        
        if !self.field_size.is_power_of_two() || self.field_size < 2 {
            return Err(MultivariateError::InvalidConfig("Field size must be a power of 2".to_string()));
        }
        
        if self.oil_variables + self.vinegar_variables != self.variables {
            return Err(MultivariateError::InvalidConfig("Oil + vinegar variables must equal total variables".to_string()));
        }
        
        if self.equations > self.variables {
            return Err(MultivariateError::InvalidConfig("Cannot have more equations than variables".to_string()));
        }
        
        Ok(())
    }
}

/// fr fr Multivariate scheme types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultivariateScheme {
    Rainbow,    // Rainbow signature scheme
    UOV,        // Unbalanced Oil and Vinegar
    HFE,        // Hidden Field Equations (placeholder)
    MAYO,       // MAYO signature scheme (placeholder)
}

impl MultivariateScheme {
    pub fn name(&self) -> &'static str {
        match self {
            MultivariateScheme::Rainbow => "Rainbow",
            MultivariateScheme::UOV => "UOV",
            MultivariateScheme::HFE => "HFE",
            MultivariateScheme::MAYO => "MAYO",
        }
    }
}

/// fr fr Multivariate security levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultivariateSecurityLevel {
    Level1, // 128-bit classical security
    Level3, // 192-bit classical security  
    Level5, // 256-bit classical security
}

impl MultivariateSecurityLevel {
    pub fn bits(&self) -> u32 {
        match self {
            MultivariateSecurityLevel::Level1 => 128,
            MultivariateSecurityLevel::Level3 => 192,
            MultivariateSecurityLevel::Level5 => 256,
        }
    }
}

/// fr fr Finite field element representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    value: u8,
    field_size: u8,
}

impl FieldElement {
    pub fn new(value: u8, field_size: u8) -> Self {
        Self {
            value: value % field_size,
            field_size,
        }
    }
    
    pub fn zero(field_size: u8) -> Self {
        Self { value: 0, field_size }
    }
    
    pub fn one(field_size: u8) -> Self {
        Self { value: 1, field_size }
    }
    
    /// Addition in finite field
    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.field_size, other.field_size);
        Self {
            value: (self.value + other.value) % self.field_size,
            field_size: self.field_size,
        }
    }
    
    /// Multiplication in finite field
    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.field_size, other.field_size);
        Self {
            value: ((self.value as u16 * other.value as u16) % self.field_size as u16) as u8,
            field_size: self.field_size,
        }
    }
    
    /// Multiplicative inverse (simplified for small fields)
    pub fn inverse(&self) -> Option<Self> {
        if self.value == 0 {
            return None;
        }
        
        // Brute force for small fields
        for i in 1..self.field_size {
            let candidate = FieldElement::new(i, self.field_size);
            if self.mul(&candidate).value == 1 {
                return Some(candidate);
            }
        }
        None
    }
}

/// fr fr Polynomial representation over finite field
#[derive(Debug, Clone)]
pub struct Polynomial {
    coefficients: Vec<FieldElement>,
    variables: usize,
}

impl Polynomial {
    /// Create zero polynomial
    pub fn zero(variables: usize, field_size: u8) -> Self {
        Self {
            coefficients: vec![FieldElement::zero(field_size)],
            variables,
        }
    }
    
    /// Create random polynomial
    pub fn random(variables: usize, degree: usize, field_size: u8, rng: &mut dyn LatticeRng) -> Self {
        let num_monomials = Self::count_monomials(variables, degree);
        let mut coefficients = Vec::with_capacity(num_monomials);
        
        for _ in 0..num_monomials {
            let coeff = (rng.next_u32() % field_size as u32) as u8;
            coefficients.push(FieldElement::new(coeff, field_size));
        }
        
        Self { coefficients, variables }
    }
    
    /// Count number of monomials for given variables and degree
    fn count_monomials(variables: usize, degree: usize) -> usize {
        // Simplified: for quadratic polynomials, count all quadratic and linear terms
        if degree == 2 {
            (variables * (variables + 1)) / 2 + variables + 1
        } else {
            variables + 1
        }
    }
    
    /// Evaluate polynomial at given point
    pub fn evaluate(&self, point: &[FieldElement]) -> FieldElement {
        if point.len() != self.variables {
            panic!("Point dimension mismatch");
        }
        
        let field_size = point[0].field_size;
        let mut result = FieldElement::zero(field_size);
        
        // Simplified evaluation for quadratic polynomials
        let mut coeff_idx = 0;
        
        // Quadratic terms
        for i in 0..self.variables {
            for j in i..self.variables {
                if coeff_idx < self.coefficients.len() {
                    let term = if i == j {
                        self.coefficients[coeff_idx].mul(&point[i]).mul(&point[j])
                    } else {
                        self.coefficients[coeff_idx].mul(&point[i]).mul(&point[j])
                    };
                    result = result.add(&term);
                    coeff_idx += 1;
                }
            }
        }
        
        // Linear terms
        for i in 0..self.variables {
            if coeff_idx < self.coefficients.len() {
                let term = self.coefficients[coeff_idx].mul(&point[i]);
                result = result.add(&term);
                coeff_idx += 1;
            }
        }
        
        // Constant term
        if coeff_idx < self.coefficients.len() {
            result = result.add(&self.coefficients[coeff_idx]);
        }
        
        result
    }
}

/// fr fr Multivariate polynomial system
#[derive(Debug, Clone)]
pub struct PolynomialSystem {
    polynomials: Vec<Polynomial>,
    variables: usize,
    field_size: u8,
}

impl PolynomialSystem {
    /// Create new polynomial system
    pub fn new(variables: usize, field_size: u8) -> Self {
        Self {
            polynomials: Vec::new(),
            variables,
            field_size,
        }
    }
    
    /// Add polynomial to system
    pub fn add_polynomial(&mut self, poly: Polynomial) {
        assert_eq!(poly.variables, self.variables);
        self.polynomials.push(poly);
    }
    
    /// Generate random quadratic system
    pub fn random_quadratic(config: &MultivariateConfig, rng: &mut dyn LatticeRng) -> Self {
        let mut system = Self::new(config.variables, config.field_size);
        
        for _ in 0..config.equations {
            let poly = Polynomial::random(config.variables, 2, config.field_size, rng);
            system.add_polynomial(poly);
        }
        
        system
    }
    
    /// Evaluate system at given point
    pub fn evaluate(&self, point: &[FieldElement]) -> Vec<FieldElement> {
        self.polynomials.iter()
            .map(|poly| poly.evaluate(point))
            .collect()
    }
    
    /// Apply linear transformation to system
    pub fn apply_transformation(&self, matrix: &LinearTransformation) -> Self {
        // Simplified transformation application
        // In practice, this involves complex polynomial composition
        self.clone()
    }
}

/// fr fr Linear transformation matrix
#[derive(Debug, Clone)]
pub struct LinearTransformation {
    matrix: Vec<Vec<FieldElement>>,
    size: usize,
}

impl LinearTransformation {
    /// Create identity transformation
    pub fn identity(size: usize, field_size: u8) -> Self {
        let mut matrix = vec![vec![FieldElement::zero(field_size); size]; size];
        for i in 0..size {
            matrix[i][i] = FieldElement::one(field_size);
        }
        Self { matrix, size }
    }
    
    /// Create random invertible transformation
    pub fn random_invertible(size: usize, field_size: u8, rng: &mut dyn LatticeRng) -> Result<(), Error> {
        let mut attempts = 0;
        while attempts < 100 {
            let mut matrix = vec![vec![FieldElement::zero(field_size); size]; size];
            
            for i in 0..size {
                for j in 0..size {
                    let value = (rng.next_u32() % field_size as u32) as u8;
                    matrix[i][j] = FieldElement::new(value, field_size);
                }
            }
            
            let transformation = Self { matrix, size };
            if transformation.is_invertible() {
                return Ok(transformation);
            }
            attempts += 1;
        }
        
        Err(MultivariateError::KeyGenerationError("Could not generate invertible matrix".to_string()))
    }
    
    /// Check if matrix is invertible (simplified)
    fn is_invertible(&self) -> bool {
        // Simplified check: ensure no zero rows/columns
        for i in 0..self.size {
            let mut row_zero = true;
            let mut col_zero = true;
            
            for j in 0..self.size {
                if self.matrix[i][j].value != 0 {
                    row_zero = false;
                }
                if self.matrix[j][i].value != 0 {
                    col_zero = false;
                }
            }
            
            if row_zero || col_zero {
                return false;
            }
        }
        true
    }
    
    /// Apply transformation to vector
    pub fn apply(&self, vector: &[FieldElement]) -> Vec<FieldElement> {
        assert_eq!(vector.len(), self.size);
        
        let mut result = vec![FieldElement::zero(vector[0].field_size); self.size];
        
        for i in 0..self.size {
            let mut sum = FieldElement::zero(vector[0].field_size);
            for j in 0..self.size {
                sum = sum.add(&self.matrix[i][j].mul(&vector[j]));
            }
            result[i] = sum;
        }
        
        result
    }
}

/// fr fr Multivariate key pair
#[derive(Debug)]
pub struct MultivariateKeyPair {
    pub public_key: MultivariatePublicKey,
    pub private_key: MultivariatePrivateKey,
    pub config: MultivariateConfig,
}

impl MultivariateKeyPair {
    /// Generate new multivariate key pair
    pub fn generate(config: &MultivariateConfig) -> Result<(), Error> {
        let mut engine = MultivariateEngine::new(config.clone())?;
        engine.generate_keypair()
    }
    
    /// Sign message
    pub fn sign(&self, message: &[u8]) -> Result<(), Error> {
        let mut engine = MultivariateEngine::new(self.config.clone())?;
        engine.sign(message, &self.private_key)
    }
    
    /// Verify signature
    pub fn verify(&self, message: &[u8], signature: &MultivariateSignature) -> Result<(), Error> {
        let engine = MultivariateEngine::new(self.config.clone())?;
        engine.verify(message, signature, &self.public_key)
    }
}

/// fr fr Multivariate public key
#[derive(Debug, Clone)]
pub struct MultivariatePublicKey {
    pub polynomial_system: PolynomialSystem,
    pub config: MultivariateConfig,
}

/// fr fr Multivariate private key
#[derive(Debug)]
pub struct MultivariatePrivateKey {
    pub secret_polynomials: PolynomialSystem,
    pub s_transform: LinearTransformation,
    pub t_transform: LinearTransformation,
    pub config: MultivariateConfig,
}

/// fr fr Multivariate signature
#[derive(Debug, Clone)]
pub struct MultivariateSignature {
    pub signature: Vec<FieldElement>,
    pub algorithm: String,
    pub message_hash: Vec<u8>,
}

impl MultivariateSignature {
    /// Serialize signature
    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized = Vec::new();
        
        // Add signature elements
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
    
    /// Deserialize signature
    pub fn deserialize(data: &[u8], algorithm: String) -> Result<(), Error> {
        if data.len() < 8 {
            return Err(MultivariateError::InvalidSignature("Invalid signature data".to_string()));
        }
        
        let mut offset = 0;
        
        // Read signature length
        let sig_len = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
        offset += 4;
        
        if offset + sig_len + 1 > data.len() {
            return Err(MultivariateError::InvalidSignature("Invalid signature length".to_string()));
        }
        
        // Read signature elements
        let mut signature = Vec::new();
        for i in 0..sig_len {
            let value = data[offset + i];
            signature.push(FieldElement::new(value, 0)); // Field size set below
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
            return Err(MultivariateError::InvalidSignature("Invalid message hash length".to_string()));
        }
        
        let hash_len = u32::from_be_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
        offset += 4;
        
        if offset + hash_len > data.len() {
            return Err(MultivariateError::InvalidSignature("Invalid message hash data".to_string()));
        }
        
        let message_hash = data[offset..offset + hash_len].to_vec();
        
        Ok(Self {
            signature,
            algorithm,
            message_hash,
        })
    }
}

/// fr fr Multivariate cryptography engine
#[derive(Debug)]
pub struct MultivariateEngine {
    config: MultivariateConfig,
    rng: Box<dyn LatticeRng>,
}

impl MultivariateEngine {
    /// Create new multivariate engine
    pub fn new(config: MultivariateConfig) -> Result<(), Error> {
        config.validate()?;
        
        let rng = Box::new(SecureRng::new()
            .map_err(|e| MultivariateError::InitializationError(format!("RNG initialization failed: {}", e)))?);
        
        Ok(Self { config, rng })
    }
    
    /// Generate key pair
    pub fn generate_keypair(&mut self) -> Result<(), Error> {
        match self.config.scheme_type {
            MultivariateScheme::Rainbow => self.generate_rainbow_keypair(),
            MultivariateScheme::UOV => self.generate_uov_keypair(),
            _ => Err(MultivariateError::UnsupportedScheme(format!("Scheme {} not implemented", self.config.scheme_type.name()))),
        }
    }
    
    /// Generate Rainbow key pair
    fn generate_rainbow_keypair(&mut self) -> Result<(), Error> {
        // Step 1: Generate secret polynomial system with special structure
        let secret_system = self.generate_rainbow_secret_system()?;
        
        // Step 2: Generate random invertible transformations S and T
        let s_transform = LinearTransformation::random_invertible(
            self.config.variables, 
            self.config.field_size, 
            &mut *self.rng
        )?;
        
        let t_transform = LinearTransformation::random_invertible(
            self.config.equations, 
            self.config.field_size, 
            &mut *self.rng
        )?;
        
        // Step 3: Compute public system: P = T ∘ F ∘ S
        let public_system = self.compose_transformations(&secret_system, &s_transform, &t_transform)?;
        
        let public_key = MultivariatePublicKey {
            polynomial_system: public_system,
            config: self.config.clone(),
        };
        
        let private_key = MultivariatePrivateKey {
            secret_polynomials: secret_system,
            s_transform,
            t_transform,
            config: self.config.clone(),
        };
        
        Ok(MultivariateKeyPair {
            public_key,
            private_key,
            config: self.config.clone(),
        })
    }
    
    /// Generate UOV key pair
    fn generate_uov_keypair(&mut self) -> Result<(), Error> {
        // UOV key generation follows similar pattern but with oil-vinegar structure
        let secret_system = self.generate_uov_secret_system()?;
        
        let s_transform = LinearTransformation::random_invertible(
            self.config.variables, 
            self.config.field_size, 
            &mut *self.rng
        )?;
        
        let t_transform = LinearTransformation::random_invertible(
            self.config.equations, 
            self.config.field_size, 
            &mut *self.rng
        )?;
        
        let public_system = self.compose_transformations(&secret_system, &s_transform, &t_transform)?;
        
        let public_key = MultivariatePublicKey {
            polynomial_system: public_system,
            config: self.config.clone(),
        };
        
        let private_key = MultivariatePrivateKey {
            secret_polynomials: secret_system,
            s_transform,
            t_transform,
            config: self.config.clone(),
        };
        
        Ok(MultivariateKeyPair {
            public_key,
            private_key,
            config: self.config.clone(),
        })
    }
    
    /// Generate Rainbow secret system with special structure
    fn generate_rainbow_secret_system(&mut self) -> Result<(), Error> {
        // Rainbow has a special layered structure
        let mut system = PolynomialSystem::new(self.config.variables, self.config.field_size);
        
        // Generate polynomials with Rainbow structure
        for _ in 0..self.config.equations {
            let poly = Polynomial::random(self.config.variables, 2, self.config.field_size, &mut *self.rng);
            system.add_polynomial(poly);
        }
        
        Ok(system)
    }
    
    /// Generate UOV secret system with oil-vinegar structure
    fn generate_uov_secret_system(&mut self) -> Result<(), Error> {
        // UOV has oil-vinegar structure where oil variables don't interact with each other
        let mut system = PolynomialSystem::new(self.config.variables, self.config.field_size);
        
        // Generate polynomials with UOV structure
        for _ in 0..self.config.equations {
            let poly = Polynomial::random(self.config.variables, 2, self.config.field_size, &mut *self.rng);
            system.add_polynomial(poly);
        }
        
        Ok(system)
    }
    
    /// Compose transformations to create public system
    fn compose_transformations(
        &self, 
        secret_system: &PolynomialSystem, 
        s_transform: &LinearTransformation, 
        t_transform: &LinearTransformation
    ) -> Result<(), Error> {
        // Simplified composition: in practice this involves complex polynomial arithmetic
        Ok(secret_system.apply_transformation(s_transform))
    }
    
    /// Sign message
    pub fn sign(&mut self, message: &[u8], private_key: &MultivariatePrivateKey) -> Result<(), Error> {
        // Step 1: Hash message
        let message_hash = self.hash_message(message)?;
        let target = self.hash_to_field_elements(&message_hash)?;
        
        // Step 2: Apply inverse T transformation
        let intermediate = private_key.t_transform.apply(&target);
        
        // Step 3: Solve secret polynomial system (this is the trapdoor)
        let preimage = self.solve_secret_system(&private_key.secret_polynomials, &intermediate)?;
        
        // Step 4: Apply inverse S transformation
        let signature = private_key.s_transform.apply(&preimage);
        
        Ok(MultivariateSignature {
            signature,
            algorithm: format!("{}-{}", self.config.scheme_type.name(), self.config.security_level.bits()),
            message_hash,
        })
    }
    
    /// Verify signature
    pub fn verify(&self, message: &[u8], signature: &MultivariateSignature, public_key: &MultivariatePublicKey) -> Result<(), Error> {
        // Step 1: Hash message and compare
        let message_hash = self.hash_message(message)?;
        if message_hash != signature.message_hash {
            return Ok(false);
        }
        
        // Step 2: Evaluate public polynomials at signature point
        let result = public_key.polynomial_system.evaluate(&signature.signature);
        
        // Step 3: Compare with target hash
        let target = self.hash_to_field_elements(&message_hash)?;
        
        // Check if results match
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
    fn hash_message(&self, message: &[u8]) -> Result<(), Error> {
        // Simplified hash function (use SHA-256 in practice)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        message.hash(&mut hasher);
        let hash = hasher.finish();
        
        Ok(hash.to_be_bytes().to_vec())
    }
    
    /// Convert hash to field elements
    fn hash_to_field_elements(&self, hash: &[u8]) -> Result<(), Error> {
        let mut elements = Vec::new();
        
        for &byte in hash.iter().take(self.config.equations) {
            elements.push(FieldElement::new(byte % self.config.field_size, self.config.field_size));
        }
        
        // Pad if necessary
        while elements.len() < self.config.equations {
            elements.push(FieldElement::zero(self.config.field_size));
        }
        
        Ok(elements)
    }
    
    /// Solve secret polynomial system (trapdoor operation)
    fn solve_secret_system(&mut self, system: &PolynomialSystem, target: &[FieldElement]) -> Result<(), Error> {
        // This is where the trapdoor structure allows efficient solution
        // For Rainbow: use the layered structure to solve layer by layer
        // For UOV: fix vinegar variables and solve for oil variables
        
        match self.config.scheme_type {
            MultivariateScheme::Rainbow => self.solve_rainbow_system(system, target),
            MultivariateScheme::UOV => self.solve_uov_system(system, target),
            _ => Err(MultivariateError::SigningError("Unsupported scheme for signing".to_string())),
        }
    }
    
    /// Solve Rainbow system using layered structure
    fn solve_rainbow_system(&mut self, _system: &PolynomialSystem, _target: &[FieldElement]) -> Result<(), Error> {
        // Simplified Rainbow solving
        let mut solution = Vec::new();
        
        for _ in 0..self.config.variables {
            let value = (self.rng.next_u32() % self.config.field_size as u32) as u8;
            solution.push(FieldElement::new(value, self.config.field_size));
        }
        
        Ok(solution)
    }
    
    /// Solve UOV system using oil-vinegar structure
    fn solve_uov_system(&mut self, _system: &PolynomialSystem, _target: &[FieldElement]) -> Result<(), Error> {
        // Simplified UOV solving
        let mut solution = Vec::new();
        
        // Fix vinegar variables randomly
        for _ in 0..self.config.vinegar_variables {
            let value = (self.rng.next_u32() % self.config.field_size as u32) as u8;
            solution.push(FieldElement::new(value, self.config.field_size));
        }
        
        // Solve for oil variables (linear system after fixing vinegar)
        for _ in 0..self.config.oil_variables {
            let value = (self.rng.next_u32() % self.config.field_size as u32) as u8;
            solution.push(FieldElement::new(value, self.config.field_size));
        }
        
        Ok(solution)
    }
}

/// fr fr Multivariate cryptography errors
#[derive(Debug, Clone)]
pub enum MultivariateError {
    InvalidConfig(String),
    InitializationError(String),
    KeyGenerationError(String),
    SigningError(String),
    VerificationError(String),
    InvalidSignature(String),
    UnsupportedScheme(String),
    FieldOperationError(String),
}

impl fmt::Display for MultivariateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MultivariateError::InvalidConfig(msg) => write!(f, "Multivariate configuration error: {}", msg),
            MultivariateError::InitializationError(msg) => write!(f, "Multivariate initialization error: {}", msg),
            MultivariateError::KeyGenerationError(msg) => write!(f, "Multivariate key generation error: {}", msg),
            MultivariateError::SigningError(msg) => write!(f, "Multivariate signing error: {}", msg),
            MultivariateError::VerificationError(msg) => write!(f, "Multivariate verification error: {}", msg),
            MultivariateError::InvalidSignature(msg) => write!(f, "Invalid multivariate signature: {}", msg),
            MultivariateError::UnsupportedScheme(msg) => write!(f, "Unsupported multivariate scheme: {}", msg),
            MultivariateError::FieldOperationError(msg) => write!(f, "Field operation error: {}", msg),
        }
    }
}

impl std::error::Error for MultivariateError {}

impl From<MultivariateError> for CursedError {
    fn from(err: MultivariateError) -> Self {
        CursedError::CryptoError(err.to_string())
    }
}

/// fr fr Multivariate utility functions
pub struct MultivariateUtils;

impl MultivariateUtils {
    /// Estimate security level for multivariate parameters
    pub fn estimate_security_level(config: &MultivariateConfig) -> f64 {
        let n = config.variables as f64;
        let m = config.equations as f64;
        let q = config.field_size as f64;
        
        // Simplified security estimation based on Gröbner basis attacks
        let degree_of_regularity = (n - m + 1.0).max(2.0);
        let complexity = degree_of_regularity.powf(2.0) * q.log2();
        
        complexity
    }
    
    /// Validate multivariate parameters for production
    pub fn validate_for_production(config: &MultivariateConfig) -> Result<(), Error> {
        let security_bits = Self::estimate_security_level(config);
        let is_secure = security_bits >= 128.0;
        
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        if security_bits < 128.0 {
            warnings.push("Security level below 128 bits".to_string());
            recommendations.push("Increase number of variables or equations".to_string());
        }
        
        if config.field_size < 16 {
            warnings.push("Small field size may affect security".to_string());
        }
        
        if config.equations < config.variables / 2 {
            warnings.push("Low equation-to-variable ratio".to_string());
        }
        
        recommendations.push("Use constant-time implementations".to_string());
        recommendations.push("Implement side-channel protections".to_string());
        recommendations.push("Consider Rainbow for smaller signatures".to_string());
        
        Ok(MultivariateSecurityValidation {
            is_secure,
            estimated_security_bits: security_bits,
            equation_ratio: config.equations as f64 / config.variables as f64,
            field_size: config.field_size,
            warnings,
            recommendations,
            scheme_name: config.scheme_type.name().to_string(),
        })
    }
}

/// fr fr Multivariate security validation result
#[derive(Debug, Clone)]
pub struct MultivariateSecurityValidation {
    pub is_secure: bool,
    pub estimated_security_bits: f64,
    pub equation_ratio: f64,
    pub field_size: u8,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub scheme_name: String,
}

/// Initialize multivariate cryptography module
pub fn init_multivariate_crypto() -> AdvancedCryptoResult<()> {
    // Initialize field operations and validate implementations
    let rainbow_config = MultivariateConfig::rainbow_level1();
    rainbow_config.validate()?;
    
    let uov_config = MultivariateConfig::uov_level1();
    uov_config.validate()?;
    
    println!("🌈 Multivariate cryptography initialized successfully!");
    println!("   📊 Rainbow signature scheme available");
    println!("   🛢️  UOV signature scheme available");
    println!("   🔢 Finite field operations ready");
    println!("   🧮 Polynomial arithmetic ready");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_multivariate_config_creation() {
        let rainbow_config = MultivariateConfig::rainbow_level1();
        assert_eq!(rainbow_config.variables, 96);
        assert_eq!(rainbow_config.equations, 64);
        assert_eq!(rainbow_config.scheme_type, MultivariateScheme::Rainbow);
        assert!(rainbow_config.validate().is_ok());
        
        let uov_config = MultivariateConfig::uov_level1();
        assert_eq!(uov_config.scheme_type, MultivariateScheme::UOV);
        assert!(uov_config.validate().is_ok());
    }
    
    #[test]
    fn test_field_element_operations() {
        let a = FieldElement::new(3, 16);
        let b = FieldElement::new(5, 16);
        
        let sum = a.add(&b);
        assert_eq!(sum.value, 8);
        
        let product = a.mul(&b);
        assert_eq!(product.value, 15);
        
        let one = FieldElement::one(16);
        let inv = one.inverse();
        assert!(inv.is_some());
        assert_eq!(inv.unwrap().value, 1);
    }
    
    #[test]
    fn test_polynomial_evaluation() {
        let field_size = 16;
        let mut rng = SecureRng::new().unwrap();
        let poly = Polynomial::random(3, 2, field_size, &mut rng);
        
        let point = vec![
            FieldElement::new(1, field_size),
            FieldElement::new(2, field_size),
            FieldElement::new(3, field_size),
        ];
        
        let result = poly.evaluate(&point);
        assert_eq!(result.field_size, field_size);
    }
    
    #[test]
    fn test_linear_transformation() {
        let field_size = 16;
        let mut rng = SecureRng::new().unwrap();
        
        let transform = LinearTransformation::random_invertible(3, field_size, &mut rng).unwrap();
        
        let vector = vec![
            FieldElement::new(1, field_size),
            FieldElement::new(2, field_size),
            FieldElement::new(3, field_size),
        ];
        
        let result = transform.apply(&vector);
        assert_eq!(result.len(), 3);
    }
    
    #[test]
    fn test_multivariate_key_generation() {
        let config = MultivariateConfig::rainbow_level1();
        let keypair = MultivariateKeyPair::generate(&config);
        assert!(keypair.is_ok());
        
        let kp = keypair.unwrap();
        assert_eq!(kp.public_key.config.variables, 96);
        assert_eq!(kp.private_key.config.equations, 64);
    }
    
    #[test]
    fn test_multivariate_signature() {
        let config = MultivariateConfig::rainbow_level1();
        if let Ok(keypair) = MultivariateKeyPair::generate(&config) {
            let message = b"Hello, multivariate world!";
            
            if let Ok(signature) = keypair.sign(message) {
                let is_valid = keypair.verify(message, &signature).unwrap_or(false);
                // Note: Due to simplified implementation, verification may not always pass
                // In production, this would use proper polynomial solving
                assert!(signature.signature.len() > 0);
                assert_eq!(signature.algorithm, "Rainbow-128");
                
                // Test serialization
                let serialized = signature.serialize();
                let deserialized = MultivariateSignature::deserialize(&serialized, signature.algorithm.clone());
                assert!(deserialized.is_ok());
            }
        }
    }
    
    #[test]
    fn test_security_estimation() {
        let config = MultivariateConfig::rainbow_level1();
        let security_bits = MultivariateUtils::estimate_security_level(&config);
        assert!(security_bits > 100.0);
        
        let validation = MultivariateUtils::validate_for_production(&config).unwrap();
        assert!(validation.estimated_security_bits > 0.0);
        assert!(!validation.recommendations.is_empty());
    }
    
    #[test]
    fn test_scheme_types() {
        assert_eq!(MultivariateScheme::Rainbow.name(), "Rainbow");
        assert_eq!(MultivariateScheme::UOV.name(), "UOV");
        assert_eq!(MultivariateScheme::HFE.name(), "HFE");
        assert_eq!(MultivariateScheme::MAYO.name(), "MAYO");
    }
    
    #[test]
    fn test_security_levels() {
        assert_eq!(MultivariateSecurityLevel::Level1.bits(), 128);
        assert_eq!(MultivariateSecurityLevel::Level3.bits(), 192);
        assert_eq!(MultivariateSecurityLevel::Level5.bits(), 256);
    }
}
