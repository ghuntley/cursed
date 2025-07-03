//! Circuit builder for R1CS constraints and zero-knowledge proofs

use crate::error::CursedError;
use crate::stdlib::packages::crypto_zk::field_arithmetic::FieldElement;
use std::collections::HashMap;
use crate::stdlib::packages::CryptoResult;
use crate::stdlib::packages::CryptoHandler;
use crate::stdlib::packages::CryptoError;

/// Result type for crypto operations
/// Wire identifier in a circuit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wire(pub usize);

impl Wire {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
    
    pub fn id(&self) -> usize {
        self.0
    }
}

/// A gate in a circuit representing a constraint
#[derive(Debug, Clone)]
pub struct Gate {
    pub id: usize,
    pub gate_type: GateType,
    pub inputs: Vec<Wire>,
    pub outputs: Vec<Wire>,
    pub coefficients: Vec<FieldElement>,
}

#[derive(Debug, Clone)]
pub enum GateType {
    Add,
    Mul,
    Constant(FieldElement),
    LinearCombination,
    Custom(String),
}

impl Gate {
    pub fn new(
        id: usize,
        gate_type: GateType,
        inputs: Vec<Wire>,
        outputs: Vec<Wire>,
        coefficients: Vec<FieldElement>,
    ) -> Self {
        Self {
            id,
            gate_type,
            inputs,
            outputs,
            coefficients,
        }
    }
    
    pub fn add_gate(id: usize, a: Wire, b: Wire, c: Wire, modulus: Vec<u8>) -> Self {
        Self::new(
            id,
            GateType::Add,
            vec![a, b],
            vec![c],
            vec![
                FieldElement::one(modulus.clone()),
                FieldElement::one(modulus.clone()),
                FieldElement::one(modulus),
            ],
        )
    }
    
    pub fn mul_gate(id: usize, a: Wire, b: Wire, c: Wire, modulus: Vec<u8>) -> Self {
        Self::new(
            id,
            GateType::Mul,
            vec![a, b],
            vec![c],
            vec![
                FieldElement::one(modulus.clone()),
                FieldElement::one(modulus.clone()),
                FieldElement::one(modulus),
            ],
        )
    }
    
    pub fn constant_gate(id: usize, value: FieldElement, output: Wire) -> Self {
        Self::new(
            id,
            GateType::Constant(value.clone()),
            vec![],
            vec![output],
            vec![value],
        )
    }
}

/// An R1CS constraint: (a, b, c) such that <a, x> * <b, x> = <c, x>
#[derive(Debug, Clone)]
pub struct R1CSConstraint {
    pub a: Vec<(Wire, FieldElement)>,
    pub b: Vec<(Wire, FieldElement)>,
    pub c: Vec<(Wire, FieldElement)>,
}

impl R1CSConstraint {
    pub fn new(
        a: Vec<(Wire, FieldElement)>,
        b: Vec<(Wire, FieldElement)>,
        c: Vec<(Wire, FieldElement)>,
    ) -> Self {
        Self { a, b, c }
    }
    
    pub fn add_constraint(
        a_wire: Wire,
        b_wire: Wire,
        c_wire: Wire,
        modulus: Vec<u8>,
    ) -> Self {
        let one = FieldElement::one(modulus);
        Self::new(
            vec![(a_wire, one.clone()), (b_wire, one.clone())],
            vec![(Wire::new(0), one.clone())],
            vec![(c_wire, one)],
        )
    }
    
    pub fn mul_constraint(
        a_wire: Wire,
        b_wire: Wire,
        c_wire: Wire,
        modulus: Vec<u8>,
    ) -> Self {
        let one = FieldElement::one(modulus);
        Self::new(
            vec![(a_wire, one.clone())],
            vec![(b_wire, one.clone())],
            vec![(c_wire, one)],
        )
    }
    
    pub fn verify(&self, witness: &HashMap<Wire, FieldElement>) -> bool {
        let a_val = self.evaluate_linear_combination(&self.a, witness);
        let b_val = self.evaluate_linear_combination(&self.b, witness);
        let c_val = self.evaluate_linear_combination(&self.c, witness);
        
        a_val * b_val == c_val
    }
    
    fn evaluate_linear_combination(
        &self,
        linear_combination: &[(Wire, FieldElement)],
        witness: &HashMap<Wire, FieldElement>,
    ) -> FieldElement {
        let mut result = FieldElement::zero(vec![0]);
        
        for (wire, coeff) in linear_combination {
            if let Some(value) = witness.get(wire) {
                result = result + coeff.clone() * value.clone();
            }
        }
        
        result
    }
}

/// A circuit builder for constructing R1CS circuits
pub struct CircuitBuilder {
    pub gates: Vec<Gate>,
    pub constraints: Vec<R1CSConstraint>,
    pub wire_counter: usize,
    pub public_inputs: Vec<Wire>,
    pub private_inputs: Vec<Wire>,
    pub outputs: Vec<Wire>,
    pub modulus: Vec<u8>,
}

impl CircuitBuilder {
    pub fn new(modulus: Vec<u8>) -> Self {
        Self {
            gates: vec![],
            constraints: vec![],
            wire_counter: 1,
            public_inputs: vec![],
            private_inputs: vec![],
            outputs: vec![],
            modulus,
        }
    }
    
    pub fn allocate_wire(&mut self) -> Wire {
        let wire = Wire::new(self.wire_counter);
        self.wire_counter += 1;
        wire
    }
    
    pub fn allocate_public_input(&mut self) -> Wire {
        let wire = self.allocate_wire();
        self.public_inputs.push(wire);
        wire
    }
    
    pub fn allocate_private_input(&mut self) -> Wire {
        let wire = self.allocate_wire();
        self.private_inputs.push(wire);
        wire
    }
    
    pub fn allocate_output(&mut self) -> Wire {
        let wire = self.allocate_wire();
        self.outputs.push(wire);
        wire
    }
    
    pub fn add_gate(&mut self, gate: Gate) -> CryptoResult<()> {
        self.gates.push(gate);
        Ok(())
    }
    
    pub fn add_constraint(&mut self, constraint: R1CSConstraint) -> CryptoResult<()> {
        self.constraints.push(constraint);
        Ok(())
    }
    
    pub fn add_addition(&mut self, a: Wire, b: Wire) -> CryptoResult<Wire> {
        let output = self.allocate_wire();
        let gate = Gate::add_gate(self.gates.len(), a, b, output, self.modulus.clone());
        self.add_gate(gate)?;
        
        let constraint = R1CSConstraint::add_constraint(a, b, output, self.modulus.clone());
        self.add_constraint(constraint)?;
        
        Ok(output)
    }
    
    pub fn add_multiplication(&mut self, a: Wire, b: Wire) -> CryptoResult<Wire> {
        let output = self.allocate_wire();
        let gate = Gate::mul_gate(self.gates.len(), a, b, output, self.modulus.clone());
        self.add_gate(gate)?;
        
        let constraint = R1CSConstraint::mul_constraint(a, b, output, self.modulus.clone());
        self.add_constraint(constraint)?;
        
        Ok(output)
    }
    
    pub fn add_constant(&mut self, value: FieldElement) -> CryptoResult<Wire> {
        let output = self.allocate_wire();
        let gate = Gate::constant_gate(self.gates.len(), value, output);
        self.add_gate(gate)?;
        Ok(output)
    }
    
    pub fn build_circuit(&self) -> CryptoResult<Circuit> {
        Ok(Circuit {
            gates: self.gates.clone(),
            constraints: self.constraints.clone(),
            public_inputs: self.public_inputs.clone(),
            private_inputs: self.private_inputs.clone(),
            outputs: self.outputs.clone(),
            num_wires: self.wire_counter,
        })
    }
    
    pub fn verify_constraints(&self, witness: &HashMap<Wire, FieldElement>) -> bool {
        self.constraints.iter().all(|constraint| constraint.verify(witness))
    }
    
    pub fn get_num_constraints(&self) -> usize {
        self.constraints.len()
    }
    
    pub fn get_num_wires(&self) -> usize {
        self.wire_counter
    }
}

/// A complete circuit with constraints and wire assignments
#[derive(Debug, Clone)]
pub struct Circuit {
    pub gates: Vec<Gate>,
    pub constraints: Vec<R1CSConstraint>,
    pub public_inputs: Vec<Wire>,
    pub private_inputs: Vec<Wire>,
    pub outputs: Vec<Wire>,
    pub num_wires: usize,
}

impl Circuit {
    pub fn verify(&self, witness: &HashMap<Wire, FieldElement>) -> bool {
        self.constraints.iter().all(|constraint| constraint.verify(witness))
    }
    
    pub fn get_public_input_assignment(&self, witness: &HashMap<Wire, FieldElement>) -> Vec<FieldElement> {
        self.public_inputs
            .iter()
            .filter_map(|wire| witness.get(wire).cloned())
            .collect()
    }
    
    pub fn get_private_input_assignment(&self, witness: &HashMap<Wire, FieldElement>) -> Vec<FieldElement> {
        self.private_inputs
            .iter()
            .filter_map(|wire| witness.get(wire).cloned())
            .collect()
    }
    
    pub fn get_output_assignment(&self, witness: &HashMap<Wire, FieldElement>) -> Vec<FieldElement> {
        self.outputs
            .iter()
            .filter_map(|wire| witness.get(wire).cloned())
            .collect()
    }
}

/// Collection of circuit utilities
pub struct Circuits;

impl Circuits {
    pub fn new_builder(modulus: Vec<u8>) -> CircuitBuilder {
        CircuitBuilder::new(modulus)
    }
    
    pub fn verify_circuit(circuit: &Circuit, witness: &HashMap<Wire, FieldElement>) -> bool {
        circuit.verify(witness)
    }
    
    pub fn create_witness(assignments: Vec<(Wire, FieldElement)>) -> HashMap<Wire, FieldElement> {
        assignments.into_iter().collect()
    }
}

/// Cryptographic operations handler
/// Initialize crypto processing
pub fn init_circuit_builder() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let key = handler.generate_key()?;
    if key.len() != 32 {
        return Err(CryptoError::KeyGenerationFailed);
    }
    println!("🔐 Crypto processing (circuit_builder) initialized");
    Ok(())
}

/// Test crypto functionality
pub fn test_circuit_builder() -> CryptoResult<()> {
    let handler = CryptoHandler::new();
    let data = b"Hello, CURSED Crypto!";
    let hash = handler.hash_sha256(data);
    if hash.len() != 32 {
        return Err(CursedError::runtime_error(&"Crypto hash test failed".to_string()));
    }
    Ok(())
}
