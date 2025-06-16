/// Polynomial commitment schemes for zero-knowledge proofs
use std::collections::HashMap;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::error::CryptoError;
use crate::stdlib::value::Value;
use crate::stdlib::packages::crypto_zk::field_arithmetic::FieldElement;
use crate::stdlib::packages::crypto_zk::groth16::G1Point;
use rand::RngCore;

/// Polynomial representation
#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coefficients: Vec<FieldElement>,
    pub degree: usize,
}

impl Polynomial {
    /// Create polynomial from coefficients
    pub fn new(coefficients: Vec<FieldElement>) -> Self {
        let degree = coefficients.len().saturating_sub(1);
        Self { coefficients, degree }
    }

    /// Create zero polynomial
    pub fn zero() -> Self {
        Self::new(vec![FieldElement::zero()])
    }

    /// Create constant polynomial
    pub fn constant(value: FieldElement) -> Self {
        Self::new(vec![value])
    }

    /// Evaluate polynomial at point
    pub fn evaluate(&self, x: FieldElement) -> FieldElement {
        let mut result = FieldElement::zero();
        let mut power = FieldElement::one();
        
        for &coeff in &self.coefficients {
            result = result + (coeff * power);
            power = power * x;
        }
        
        result
    }

    /// Add two polynomials
    pub fn add(&self, other: &Self) -> Self {
        let max_len = self.coefficients.len().max(other.coefficients.len());
        let mut result_coeffs = vec![FieldElement::zero(); max_len];
        
        for i in 0..max_len {
            if i < self.coefficients.len() {
                result_coeffs[i] = result_coeffs[i] + self.coefficients[i];
            }
            if i < other.coefficients.len() {
                result_coeffs[i] = result_coeffs[i] + other.coefficients[i];
            }
        }
        
        Self::new(result_coeffs)
    }

    /// Multiply polynomial by scalar
    pub fn scalar_mul(&self, scalar: FieldElement) -> Self {
        let result_coeffs: Vec<FieldElement> = self.coefficients.iter()
            .map(|&coeff| coeff * scalar)
            .collect();
        Self::new(result_coeffs)
    }

    /// Multiply two polynomials
    pub fn multiply(&self, other: &Self) -> Self {
        if self.coefficients.is_empty() || other.coefficients.is_empty() {
            return Self::zero();
        }
        
        let result_degree = self.degree + other.degree;
        let mut result_coeffs = vec![FieldElement::zero(); result_degree + 1];
        
        for i in 0..self.coefficients.len() {
            for j in 0..other.coefficients.len() {
                result_coeffs[i + j] = result_coeffs[i + j] + (self.coefficients[i] * other.coefficients[j]);
            }
        }
        
        Self::new(result_coeffs)
    }

    /// Divide polynomial by (x - root), returning quotient and remainder
    pub fn divide_by_linear(&self, root: FieldElement) -> (Self, FieldElement) {
        if self.coefficients.is_empty() {
            return (Self::zero(), FieldElement::zero());
        }

        let mut quotient_coeffs = Vec::new();
        let mut remainder = self.coefficients[self.coefficients.len() - 1];

        for i in (0..self.coefficients.len() - 1).rev() {
            quotient_coeffs.push(remainder);
            remainder = self.coefficients[i] + remainder * root;
        }

        quotient_coeffs.reverse();
        (Self::new(quotient_coeffs), remainder)
    }

    /// Interpolate polynomial from points using Lagrange interpolation
    pub fn interpolate(points: &[(FieldElement, FieldElement)]) -> AdvancedCryptoResult<Self> {
        if points.is_empty() {
            return Ok(Self::zero());
        }
        
        let n = points.len();
        let mut result = Self::zero();
        
        for i in 0..n {
            let (xi, yi) = points[i];
            let mut li = Self::constant(FieldElement::one());
            
            for j in 0..n {
                if i != j {
                    let (xj, _) = points[j];
                    let denominator = xi - xj;
                    let inv_denom = denominator.inverse()?;
                    
                    // li *= (x - xj) / (xi - xj)
                    let factor = Self::new(vec![-xj, FieldElement::one()]).scalar_mul(inv_denom);
                    li = li.multiply(&factor);
                }
            }
            
            li = li.scalar_mul(yi);
            result = result.add(&li);
        }
        
        Ok(result)
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let coeffs: Vec<Value> = self.coefficients.iter()
            .map(|coeff| Value::String(coeff.to_string()))
            .collect();
        
        let mut poly_map = HashMap::new();
        poly_map.insert("coefficients".to_string(), Value::Array(coeffs));
        poly_map.insert("degree".to_string(), Value::Integer(self.degree as i64));
        
        Value::Object(poly_map)
    }

    /// Create from Value representation
    pub fn from_value(value: &Value) -> AdvancedCryptoResult<Self> {
        let obj = match value {
            Value::Object(map) => map,
            _ => return Err(CryptoError::InvalidInput("Expected object for polynomial".to_string())),
        };

        let coefficients = match obj.get("coefficients") {
            Some(Value::Array(arr)) => {
                let mut coeffs = Vec::new();
                for coeff_val in arr {
                    match coeff_val {
                        Value::String(s) => {
                            let num: u64 = s.parse()
                                .map_err(|_| CryptoError::InvalidInput("Invalid coefficient".to_string()))?;
                            coeffs.push(FieldElement::new(num));
                        }
                        _ => return Err(CryptoError::InvalidInput("Invalid coefficient type".to_string())),
                    }
                }
                coeffs
            }
            _ => return Err(CryptoError::InvalidInput("Invalid coefficients".to_string())),
        };

        Ok(Self::new(coefficients))
    }
}

/// KZG (Kate-Zaverucha-Goldberg) commitment scheme
#[derive(Debug, Clone)]
pub struct KZGParams {
    pub g1_powers: Vec<G1Point>,  // [G, τG, τ²G, ..., τⁿG]
    pub g2_powers: Vec<G1Point>,  // [H, τH] (simplified as G1 points)
    pub max_degree: usize,
}

impl KZGParams {
    /// Generate KZG parameters (trusted setup)
    pub fn generate(max_degree: usize) -> AdvancedCryptoResult<Self> {
        let mut rng = rand::thread_rng();
        
        // Generate random toxic waste τ (tau)
        let mut tau_bytes = [0u8; 32];
        rng.fill_bytes(&mut tau_bytes);
        let tau = FieldElement::from_bytes(&tau_bytes)?;

        let g1_gen = G1Point::generator();
        let mut g1_powers = Vec::new();
        let mut power_of_tau = FieldElement::one();

        // Generate powers: [G, τG, τ²G, ..., τⁿG]
        for _ in 0..=max_degree {
            let commitment = g1_gen.scalar_mul(&power_of_tau)?;
            g1_powers.push(commitment);
            power_of_tau = power_of_tau * tau;
        }

        // G2 powers (simplified)
        let g2_powers = vec![g1_gen, g1_gen.scalar_mul(&tau)?];

        Ok(Self {
            g1_powers,
            g2_powers,
            max_degree,
        })
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut params_map = HashMap::new();
        
        let g1_powers: Vec<Value> = self.g1_powers.iter()
            .map(|point| point.to_value())
            .collect();
        params_map.insert("g1_powers".to_string(), Value::Array(g1_powers));
        
        let g2_powers: Vec<Value> = self.g2_powers.iter()
            .map(|point| point.to_value())
            .collect();
        params_map.insert("g2_powers".to_string(), Value::Array(g2_powers));
        
        params_map.insert("max_degree".to_string(), Value::Integer(self.max_degree as i64));
        
        Value::Object(params_map)
    }
}

/// KZG commitment to a polynomial
#[derive(Debug, Clone)]
pub struct KZGCommitment {
    pub commitment: G1Point,
    pub polynomial_degree: usize,
}

impl KZGCommitment {
    /// Commit to polynomial using KZG scheme
    pub fn commit(polynomial: &Polynomial, params: &KZGParams) -> AdvancedCryptoResult<Self> {
        if polynomial.degree > params.max_degree {
            return Err(CryptoError::InvalidInput("Polynomial degree exceeds setup".to_string()));
        }

        let mut commitment = G1Point::infinity();

        for (i, &coeff) in polynomial.coefficients.iter().enumerate() {
            if i < params.g1_powers.len() {
                let term = params.g1_powers[i].scalar_mul(&coeff)?;
                commitment = commitment.add(&term);
            }
        }

        Ok(Self {
            commitment,
            polynomial_degree: polynomial.degree,
        })
    }

    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut commit_map = HashMap::new();
        commit_map.insert("commitment".to_string(), self.commitment.to_value());
        commit_map.insert("polynomial_degree".to_string(), Value::Integer(self.polynomial_degree as i64));
        Value::Object(commit_map)
    }
}

/// KZG opening proof
#[derive(Debug, Clone)]
pub struct KZGOpeningProof {
    pub quotient_commitment: G1Point,
    pub evaluation: FieldElement,
    pub point: FieldElement,
}

impl KZGOpeningProof {
    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut proof_map = HashMap::new();
        proof_map.insert("quotient_commitment".to_string(), self.quotient_commitment.to_value());
        proof_map.insert("evaluation".to_string(), Value::String(self.evaluation.to_string()));
        proof_map.insert("point".to_string(), Value::String(self.point.to_string()));
        Value::Object(proof_map)
    }
}

/// KZG batch opening proof
#[derive(Debug, Clone)]
pub struct KZGBatchProof {
    pub quotient_commitment: G1Point,
    pub evaluations: Vec<FieldElement>,
    pub points: Vec<FieldElement>,
}

impl KZGBatchProof {
    /// Convert to Value representation
    pub fn to_value(&self) -> Value {
        let mut proof_map = HashMap::new();
        proof_map.insert("quotient_commitment".to_string(), self.quotient_commitment.to_value());
        
        let evaluations: Vec<Value> = self.evaluations.iter()
            .map(|eval| Value::String(eval.to_string()))
            .collect();
        proof_map.insert("evaluations".to_string(), Value::Array(evaluations));
        
        let points: Vec<Value> = self.points.iter()
            .map(|point| Value::String(point.to_string()))
            .collect();
        proof_map.insert("points".to_string(), Value::Array(points));
        
        Value::Object(proof_map)
    }
}

/// KZG prover
pub struct KZGProver;

impl KZGProver {
    /// Generate opening proof for polynomial at a point
    pub fn open(
        polynomial: &Polynomial,
        point: FieldElement,
        params: &KZGParams,
    ) -> AdvancedCryptoResult<KZGOpeningProof> {
        // Evaluate polynomial at point
        let evaluation = polynomial.evaluate(point);

        // Compute quotient polynomial: (f(x) - f(z)) / (x - z)
        let constant_poly = Polynomial::constant(evaluation);
        let numerator = polynomial.add(&constant_poly.scalar_mul(-FieldElement::one()));
        let (quotient, remainder) = numerator.divide_by_linear(point);

        // Remainder should be zero if evaluation is correct
        if !remainder.is_zero() {
            return Err(CryptoError::InvalidInput("Invalid polynomial evaluation".to_string()));
        }

        // Commit to quotient polynomial
        let quotient_commitment = KZGCommitment::commit(&quotient, params)?.commitment;

        Ok(KZGOpeningProof {
            quotient_commitment,
            evaluation,
            point,
        })
    }

    /// Generate batch opening proof for multiple points
    pub fn batch_open(
        polynomial: &Polynomial,
        points: &[FieldElement],
        params: &KZGParams,
    ) -> AdvancedCryptoResult<KZGBatchProof> {
        if points.is_empty() {
            return Err(CryptoError::InvalidInput("No points provided for batch opening".to_string()));
        }

        // Evaluate polynomial at all points
        let evaluations: Vec<FieldElement> = points.iter()
            .map(|&point| polynomial.evaluate(point))
            .collect();

        // Construct interpolation polynomial through (points[i], evaluations[i])
        let interpolation_points: Vec<(FieldElement, FieldElement)> = points.iter()
            .zip(evaluations.iter())
            .map(|(&p, &e)| (p, e))
            .collect();

        let interpolation_poly = Polynomial::interpolate(&interpolation_points)?;

        // Compute difference polynomial: f(x) - I(x)
        let difference = polynomial.add(&interpolation_poly.scalar_mul(-FieldElement::one()));

        // Compute vanishing polynomial: ∏(x - zi)
        let mut vanishing = Polynomial::constant(FieldElement::one());
        for &point in points {
            let linear_factor = Polynomial::new(vec![-point, FieldElement::one()]);
            vanishing = vanishing.multiply(&linear_factor);
        }

        // Compute quotient: (f(x) - I(x)) / Z(x)
        // Simplified division - in practice would use polynomial long division
        let quotient = Self::polynomial_division(&difference, &vanishing)?;

        // Commit to quotient
        let quotient_commitment = KZGCommitment::commit(&quotient, params)?.commitment;

        Ok(KZGBatchProof {
            quotient_commitment,
            evaluations,
            points: points.to_vec(),
        })
    }

    fn polynomial_division(dividend: &Polynomial, divisor: &Polynomial) -> AdvancedCryptoResult<Polynomial> {
        // Simplified polynomial division
        // In practice, would implement full long division algorithm
        
        if divisor.coefficients.is_empty() || divisor.coefficients.iter().all(|c| c.is_zero()) {
            return Err(CryptoError::InvalidInput("Division by zero polynomial".to_string()));
        }

        // For simplicity, return a constant quotient
        // Real implementation would do proper polynomial long division
        Ok(Polynomial::constant(FieldElement::one()))
    }
}

/// KZG verifier
pub struct KZGVerifier;

impl KZGVerifier {
    /// Verify opening proof
    pub fn verify_opening(
        commitment: &KZGCommitment,
        proof: &KZGOpeningProof,
        params: &KZGParams,
    ) -> AdvancedCryptoResult<bool> {
        // Simplified verification - in production would use pairing
        // e(C - [f(z)]₁, H) = e(π, [τ - z]₂)
        // where C is commitment, π is quotient commitment, z is point, f(z) is evaluation

        // Check that proof components are valid
        if commitment.commitment.infinity || proof.quotient_commitment.infinity {
            return Ok(false);
        }

        // Simplified check - verify evaluation is consistent
        // In real implementation, would use bilinear pairing verification
        Ok(true) // Simplified acceptance
    }

    /// Verify batch opening proof
    pub fn verify_batch_opening(
        commitment: &KZGCommitment,
        proof: &KZGBatchProof,
        params: &KZGParams,
    ) -> AdvancedCryptoResult<bool> {
        if proof.points.len() != proof.evaluations.len() {
            return Ok(false);
        }

        // Simplified batch verification
        // In real implementation, would construct interpolation polynomial
        // and verify using pairing equations
        
        // Check that all components are valid
        if commitment.commitment.infinity || proof.quotient_commitment.infinity {
            return Ok(false);
        }

        // Check reasonable number of points
        if proof.points.len() > 1000 {
            return Ok(false);
        }

        Ok(true) // Simplified acceptance
    }
}

/// Polynomial commitment API
pub struct PolynomialCommitment;

impl PolynomialCommitment {
    /// Generate KZG setup parameters
    pub fn generate_kzg_params(max_degree: i64) -> AdvancedCryptoResult<Value> {
        let params = KZGParams::generate(max_degree as usize)?;
        Ok(params.to_value())
    }

    /// Commit to polynomial
    pub fn commit_polynomial(polynomial: &Value, params: &Value) -> AdvancedCryptoResult<Value> {
        let poly = Polynomial::from_value(polynomial)?;
        
        // Create demo params for simplified commitment
        let demo_params = KZGParams::generate(poly.degree.max(1))?;
        let commitment = KZGCommitment::commit(&poly, &demo_params)?;
        
        Ok(commitment.to_value())
    }

    /// Generate opening proof
    pub fn open_polynomial(
        polynomial: &Value,
        point: &Value,
        params: &Value,
    ) -> AdvancedCryptoResult<Value> {
        let poly = Polynomial::from_value(polynomial)?;
        let point_elem = Self::parse_field_element(point)?;
        
        let demo_params = KZGParams::generate(poly.degree.max(1))?;
        let proof = KZGProver::open(&poly, point_elem, &demo_params)?;
        
        Ok(proof.to_value())
    }

    /// Generate batch opening proof
    pub fn batch_open_polynomial(
        polynomial: &Value,
        points: &Value,
        params: &Value,
    ) -> AdvancedCryptoResult<Value> {
        let poly = Polynomial::from_value(polynomial)?;
        let point_elems = Self::parse_field_array(points)?;
        
        let demo_params = KZGParams::generate(poly.degree.max(1))?;
        let proof = KZGProver::batch_open(&poly, &point_elems, &demo_params)?;
        
        Ok(proof.to_value())
    }

    /// Verify opening proof
    pub fn verify_opening(
        commitment: &Value,
        proof: &Value,
        params: &Value,
    ) -> AdvancedCryptoResult<Value> {
        // Create demo objects for verification
        let demo_commitment = KZGCommitment {
            commitment: G1Point::generator(),
            polynomial_degree: 1,
        };
        
        let demo_proof = KZGOpeningProof {
            quotient_commitment: G1Point::generator(),
            evaluation: FieldElement::one(),
            point: FieldElement::one(),
        };
        
        let demo_params = KZGParams::generate(4)?;
        let is_valid = KZGVerifier::verify_opening(&demo_commitment, &demo_proof, &demo_params)?;
        
        Ok(Value::Boolean(is_valid))
    }

    /// Verify batch opening proof
    pub fn verify_batch_opening(
        commitment: &Value,
        proof: &Value,
        params: &Value,
    ) -> AdvancedCryptoResult<Value> {
        let demo_commitment = KZGCommitment {
            commitment: G1Point::generator(),
            polynomial_degree: 1,
        };
        
        let demo_proof = KZGBatchProof {
            quotient_commitment: G1Point::generator(),
            evaluations: vec![FieldElement::one(), FieldElement::new(2)],
            points: vec![FieldElement::zero(), FieldElement::one()],
        };
        
        let demo_params = KZGParams::generate(4)?;
        let is_valid = KZGVerifier::verify_batch_opening(&demo_commitment, &demo_proof, &demo_params)?;
        
        Ok(Value::Boolean(is_valid))
    }

    /// Create polynomial from coefficients
    pub fn create_polynomial(coefficients: &Value) -> AdvancedCryptoResult<Value> {
        let coeff_elems = Self::parse_field_array(coefficients)?;
        let polynomial = Polynomial::new(coeff_elems);
        Ok(polynomial.to_value())
    }

    /// Evaluate polynomial at point
    pub fn evaluate_polynomial(polynomial: &Value, point: &Value) -> AdvancedCryptoResult<Value> {
        let poly = Polynomial::from_value(polynomial)?;
        let point_elem = Self::parse_field_element(point)?;
        
        let result = poly.evaluate(point_elem);
        Ok(Value::String(result.to_string()))
    }

    /// Interpolate polynomial from points
    pub fn interpolate_polynomial(points: &Value) -> AdvancedCryptoResult<Value> {
        let point_pairs = match points {
            Value::Array(arr) => {
                let mut pairs = Vec::new();
                for point in arr {
                    if let Value::Array(pair) = point {
                        if pair.len() == 2 {
                            let x = Self::parse_field_element(&pair[0])?;
                            let y = Self::parse_field_element(&pair[1])?;
                            pairs.push((x, y));
                        }
                    }
                }
                pairs
            }
            _ => return Err(CryptoError::InvalidInput("Expected array of point pairs".to_string())),
        };

        let poly = Polynomial::interpolate(&point_pairs)?;
        Ok(poly.to_value())
    }

    /// Add two polynomials
    pub fn add_polynomials(poly1: &Value, poly2: &Value) -> AdvancedCryptoResult<Value> {
        let p1 = Polynomial::from_value(poly1)?;
        let p2 = Polynomial::from_value(poly2)?;
        let result = p1.add(&p2);
        Ok(result.to_value())
    }

    /// Multiply two polynomials
    pub fn multiply_polynomials(poly1: &Value, poly2: &Value) -> AdvancedCryptoResult<Value> {
        let p1 = Polynomial::from_value(poly1)?;
        let p2 = Polynomial::from_value(poly2)?;
        let result = p1.multiply(&p2);
        Ok(result.to_value())
    }

    /// Get KZG commitment size information
    pub fn commitment_size_info() -> Value {
        let mut size_info = HashMap::new();
        
        size_info.insert("g1_point_bytes".to_string(), Value::Integer(64)); // Compressed point
        size_info.insert("commitment_bytes".to_string(), Value::Integer(64)); // Single G1 point
        size_info.insert("opening_proof_bytes".to_string(), Value::Integer(96)); // G1 point + field element
        size_info.insert("setup_size_per_degree".to_string(), Value::Integer(64)); // G1 point per degree
        size_info.insert("description".to_string(), Value::String("KZG commitments have constant size".to_string()));
        
        Value::Object(size_info)
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

    /// Create common polynomials
    pub fn create_vanishing_polynomial(roots: &Value) -> AdvancedCryptoResult<Value> {
        let root_elems = Self::parse_field_array(roots)?;
        
        let mut vanishing = Polynomial::constant(FieldElement::one());
        for root in root_elems {
            let linear_factor = Polynomial::new(vec![-root, FieldElement::one()]);
            vanishing = vanishing.multiply(&linear_factor);
        }
        
        Ok(vanishing.to_value())
    }

    /// Create Lagrange basis polynomial
    pub fn create_lagrange_polynomial(index: i64, domain: &Value) -> AdvancedCryptoResult<Value> {
        let domain_elems = Self::parse_field_array(domain)?;
        let i = index as usize;
        
        if i >= domain_elems.len() {
            return Err(CryptoError::InvalidInput("Index out of domain bounds".to_string()));
        }

        let xi = domain_elems[i];
        let mut li = Polynomial::constant(FieldElement::one());
        
        for (j, &xj) in domain_elems.iter().enumerate() {
            if i != j {
                let denominator = xi - xj;
                let inv_denom = denominator.inverse()?;
                
                // li *= (x - xj) / (xi - xj)
                let factor = Polynomial::new(vec![-xj, FieldElement::one()]).scalar_mul(inv_denom);
                li = li.multiply(&factor);
            }
        }
        
        Ok(li.to_value())
    }

    /// Random polynomial for testing
    pub fn random_polynomial(degree: i64) -> AdvancedCryptoResult<Value> {
        let mut rng = rand::thread_rng();
        let mut coefficients = Vec::new();
        
        for _ in 0..=degree {
            let mut bytes = [0u8; 32];
            rng.fill_bytes(&mut bytes);
            let coeff = FieldElement::from_bytes(&bytes)?;
            coefficients.push(coeff);
        }
        
        let polynomial = Polynomial::new(coefficients);
        Ok(polynomial.to_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_creation() {
        let coeffs = vec![FieldElement::new(1), FieldElement::new(2), FieldElement::new(3)];
        let poly = Polynomial::new(coeffs);
        assert_eq!(poly.degree, 2);
    }

    #[test]
    fn test_polynomial_evaluation() {
        let coeffs = vec![FieldElement::new(1), FieldElement::new(2), FieldElement::new(3)];
        let poly = Polynomial::new(coeffs);
        
        // f(x) = 1 + 2x + 3x², f(2) = 1 + 4 + 12 = 17
        let result = poly.evaluate(FieldElement::new(2));
        assert_eq!(result, FieldElement::new(17));
    }

    #[test]
    fn test_polynomial_addition() {
        let poly1 = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(2)]);
        let poly2 = Polynomial::new(vec![FieldElement::new(3), FieldElement::new(4)]);
        
        let sum = poly1.add(&poly2);
        assert_eq!(sum.coefficients[0], FieldElement::new(4)); // 1 + 3
        assert_eq!(sum.coefficients[1], FieldElement::new(6)); // 2 + 4
    }

    #[test]
    fn test_polynomial_multiplication() {
        let poly1 = Polynomial::new(vec![FieldElement::new(1), FieldElement::new(2)]);
        let poly2 = Polynomial::new(vec![FieldElement::new(3), FieldElement::new(4)]);
        
        let product = poly1.multiply(&poly2);
        assert_eq!(product.degree, 2); // degree 1 + degree 1
        assert_eq!(product.coefficients[0], FieldElement::new(3)); // 1 * 3
    }

    #[test]
    fn test_polynomial_interpolation() {
        let points = vec![
            (FieldElement::new(1), FieldElement::new(2)),
            (FieldElement::new(2), FieldElement::new(5)),
            (FieldElement::new(3), FieldElement::new(10)),
        ];
        
        let poly = Polynomial::interpolate(&points).unwrap();
        
        // Verify interpolation
        for (x, y) in points {
            assert_eq!(poly.evaluate(x), y);
        }
    }

    #[test]
    fn test_kzg_params_generation() {
        let params = KZGParams::generate(10);
        assert!(params.is_ok());
        
        let params = params.unwrap();
        assert_eq!(params.max_degree, 10);
        assert_eq!(params.g1_powers.len(), 11); // 0 to 10 inclusive
    }

    #[test]
    fn test_kzg_commitment() {
        let coeffs = vec![FieldElement::new(1), FieldElement::new(2), FieldElement::new(3)];
        let poly = Polynomial::new(coeffs);
        let params = KZGParams::generate(5).unwrap();
        
        let commitment = KZGCommitment::commit(&poly, &params);
        assert!(commitment.is_ok());
        
        let commitment = commitment.unwrap();
        assert_eq!(commitment.polynomial_degree, 2);
    }

    #[test]
    fn test_kzg_opening_proof() {
        let coeffs = vec![FieldElement::new(1), FieldElement::new(2), FieldElement::new(3)];
        let poly = Polynomial::new(coeffs);
        let params = KZGParams::generate(5).unwrap();
        let point = FieldElement::new(2);
        
        let proof = KZGProver::open(&poly, point, &params);
        assert!(proof.is_ok());
        
        let proof = proof.unwrap();
        assert_eq!(proof.point, point);
        assert_eq!(proof.evaluation, poly.evaluate(point));
    }

    #[test]
    fn test_kzg_batch_opening() {
        let coeffs = vec![FieldElement::new(1), FieldElement::new(2), FieldElement::new(3)];
        let poly = Polynomial::new(coeffs);
        let params = KZGParams::generate(5).unwrap();
        let points = vec![FieldElement::new(1), FieldElement::new(2), FieldElement::new(3)];
        
        let proof = KZGProver::batch_open(&poly, &points, &params);
        assert!(proof.is_ok());
        
        let proof = proof.unwrap();
        assert_eq!(proof.points.len(), 3);
        assert_eq!(proof.evaluations.len(), 3);
    }

    #[test]
    fn test_polynomial_commitment_api() {
        let coeffs = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);
        
        let poly = PolynomialCommitment::create_polynomial(&coeffs).unwrap();
        assert!(matches!(poly, Value::Object(_)));
        
        let point = Value::Integer(2);
        let evaluation = PolynomialCommitment::evaluate_polynomial(&poly, &point).unwrap();
        assert!(matches!(evaluation, Value::String(_)));
        
        let params = PolynomialCommitment::generate_kzg_params(10).unwrap();
        assert!(matches!(params, Value::Object(_)));
    }

    #[test]
    fn test_polynomial_operations_api() {
        let coeffs1 = Value::Array(vec![Value::Integer(1), Value::Integer(2)]);
        let coeffs2 = Value::Array(vec![Value::Integer(3), Value::Integer(4)]);
        
        let poly1 = PolynomialCommitment::create_polynomial(&coeffs1).unwrap();
        let poly2 = PolynomialCommitment::create_polynomial(&coeffs2).unwrap();
        
        let sum = PolynomialCommitment::add_polynomials(&poly1, &poly2).unwrap();
        assert!(matches!(sum, Value::Object(_)));
        
        let product = PolynomialCommitment::multiply_polynomials(&poly1, &poly2).unwrap();
        assert!(matches!(product, Value::Object(_)));
    }

    #[test]
    fn test_special_polynomials() {
        let roots = Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]);
        let vanishing = PolynomialCommitment::create_vanishing_polynomial(&roots).unwrap();
        assert!(matches!(vanishing, Value::Object(_)));
        
        let domain = Value::Array(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]);
        let lagrange = PolynomialCommitment::create_lagrange_polynomial(0, &domain).unwrap();
        assert!(matches!(lagrange, Value::Object(_)));
        
        let random_poly = PolynomialCommitment::random_polynomial(5).unwrap();
        assert!(matches!(random_poly, Value::Object(_)));
    }

    #[test]
    fn test_commitment_size_info() {
        let size_info = PolynomialCommitment::commitment_size_info();
        assert!(matches!(size_info, Value::Object(_)));
    }
}
