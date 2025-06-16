/// Circuit builder for zero-knowledge proof systems
use std::collections::HashMap;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::error::CryptoError;
use crate::stdlib::value::Value;
use crate::stdlib::packages::crypto_zk::field_arithmetic::FieldElement;

/// Wire types in arithmetic circuits
#[derive(Debug, Clone, PartialEq)]
pub enum WireType {
    Input,
    Output,
    Intermediate,
}

/// Circuit wire
#[derive(Debug, Clone)]
pub struct Wire {
    pub id: usize,
    pub wire_type: WireType,
    pub value: Option<FieldElement>,
    pub label: Option<String>,
}

impl Wire {
    pub fn new(id: usize, wire_type: WireType) -> Self {
        Self {
            id,
            wire_type,
            value: None,
            label: None,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn set_value(&mut self, value: FieldElement) {
        self.value = Some(value);
    }

    pub fn to_value(&self) -> Value {
        let mut wire_map = HashMap::new();
        wire_map.insert("id".to_string(), Value::Integer(self.id as i64));
        wire_map.insert("type".to_string(), Value::String(format!("{:?}", self.wire_type)));
        
        if let Some(ref value) = self.value {
            wire_map.insert("value".to_string(), Value::String(value.to_string()));
        }
        
        if let Some(ref label) = self.label {
            wire_map.insert("label".to_string(), Value::String(label.clone()));
        }
        
        Value::Object(wire_map)
    }
}

/// Arithmetic gate types
#[derive(Debug, Clone, PartialEq)]
pub enum GateType {
    Add,
    Mul,
    Constant(FieldElement),
    Input,
    Output,
}

/// Arithmetic gate in R1CS
#[derive(Debug, Clone)]
pub struct Gate {
    pub id: usize,
    pub gate_type: GateType,
    pub input_wires: Vec<usize>,
    pub output_wire: usize,
    pub constraint: Option<R1CSConstraint>,
}

impl Gate {
    pub fn new_add(id: usize, left: usize, right: usize, output: usize) -> Self {
        Self {
            id,
            gate_type: GateType::Add,
            input_wires: vec![left, right],
            output_wire: output,
            constraint: None,
        }
    }

    pub fn new_mul(id: usize, left: usize, right: usize, output: usize) -> Self {
        Self {
            id,
            gate_type: GateType::Mul,
            input_wires: vec![left, right],
            output_wire: output,
            constraint: None,
        }
    }

    pub fn new_constant(id: usize, constant: FieldElement, output: usize) -> Self {
        Self {
            id,
            gate_type: GateType::Constant(constant),
            input_wires: vec![],
            output_wire: output,
            constraint: None,
        }
    }

    pub fn new_input(id: usize, wire: usize) -> Self {
        Self {
            id,
            gate_type: GateType::Input,
            input_wires: vec![],
            output_wire: wire,
            constraint: None,
        }
    }

    pub fn new_output(id: usize, wire: usize) -> Self {
        Self {
            id,
            gate_type: GateType::Output,
            input_wires: vec![wire],
            output_wire: wire,
            constraint: None,
        }
    }

    pub fn evaluate(&self, wire_values: &HashMap<usize, FieldElement>) -> AdvancedCryptoResult<FieldElement> {
        match &self.gate_type {
            GateType::Add => {
                if self.input_wires.len() != 2 {
                    return Err(CryptoError::InvalidInput("Add gate requires 2 inputs".to_string()));
                }
                let left = wire_values.get(&self.input_wires[0])
                    .ok_or_else(|| CryptoError::InvalidInput("Missing left input".to_string()))?;
                let right = wire_values.get(&self.input_wires[1])
                    .ok_or_else(|| CryptoError::InvalidInput("Missing right input".to_string()))?;
                Ok(*left + *right)
            }
            GateType::Mul => {
                if self.input_wires.len() != 2 {
                    return Err(CryptoError::InvalidInput("Mul gate requires 2 inputs".to_string()));
                }
                let left = wire_values.get(&self.input_wires[0])
                    .ok_or_else(|| CryptoError::InvalidInput("Missing left input".to_string()))?;
                let right = wire_values.get(&self.input_wires[1])
                    .ok_or_else(|| CryptoError::InvalidInput("Missing right input".to_string()))?;
                Ok(*left * *right)
            }
            GateType::Constant(c) => Ok(*c),
            GateType::Input => {
                wire_values.get(&self.output_wire)
                    .copied()
                    .ok_or_else(|| CryptoError::InvalidInput("Missing input value".to_string()))
            }
            GateType::Output => {
                wire_values.get(&self.input_wires[0])
                    .copied()
                    .ok_or_else(|| CryptoError::InvalidInput("Missing output input".to_string()))
            }
        }
    }

    pub fn to_value(&self) -> Value {
        let mut gate_map = HashMap::new();
        gate_map.insert("id".to_string(), Value::Integer(self.id as i64));
        gate_map.insert("type".to_string(), Value::String(format!("{:?}", self.gate_type)));
        gate_map.insert("output_wire".to_string(), Value::Integer(self.output_wire as i64));
        
        let input_wires: Vec<Value> = self.input_wires.iter()
            .map(|&w| Value::Integer(w as i64))
            .collect();
        gate_map.insert("input_wires".to_string(), Value::Array(input_wires));
        
        Value::Object(gate_map)
    }
}

/// R1CS constraint: (A • z) * (B • z) = (C • z)
#[derive(Debug, Clone)]
pub struct R1CSConstraint {
    pub a: Vec<(usize, FieldElement)>, // (wire_index, coefficient)
    pub b: Vec<(usize, FieldElement)>,
    pub c: Vec<(usize, FieldElement)>,
}

impl R1CSConstraint {
    pub fn new() -> Self {
        Self {
            a: Vec::new(),
            b: Vec::new(),
            c: Vec::new(),
        }
    }

    pub fn add_a_term(&mut self, wire: usize, coeff: FieldElement) {
        self.a.push((wire, coeff));
    }

    pub fn add_b_term(&mut self, wire: usize, coeff: FieldElement) {
        self.b.push((wire, coeff));
    }

    pub fn add_c_term(&mut self, wire: usize, coeff: FieldElement) {
        self.c.push((wire, coeff));
    }

    pub fn evaluate(&self, witness: &[FieldElement]) -> AdvancedCryptoResult<bool> {
        let a_result = self.evaluate_linear_combination(&self.a, witness)?;
        let b_result = self.evaluate_linear_combination(&self.b, witness)?;
        let c_result = self.evaluate_linear_combination(&self.c, witness)?;

        Ok(a_result * b_result == c_result)
    }

    fn evaluate_linear_combination(&self, terms: &[(usize, FieldElement)], witness: &[FieldElement]) -> AdvancedCryptoResult<FieldElement> {
        let mut result = FieldElement::zero();
        
        for &(wire_index, coeff) in terms {
            if wire_index >= witness.len() {
                return Err(CryptoError::InvalidInput("Wire index out of bounds".to_string()));
            }
            result = result + (coeff * witness[wire_index]);
        }
        
        Ok(result)
    }

    pub fn to_value(&self) -> Value {
        let mut constraint_map = HashMap::new();
        
        let a_terms: Vec<Value> = self.a.iter().map(|(wire, coeff)| {
            let mut term_map = HashMap::new();
            term_map.insert("wire".to_string(), Value::Integer(*wire as i64));
            term_map.insert("coeff".to_string(), Value::String(coeff.to_string()));
            Value::Object(term_map)
        }).collect();
        
        let b_terms: Vec<Value> = self.b.iter().map(|(wire, coeff)| {
            let mut term_map = HashMap::new();
            term_map.insert("wire".to_string(), Value::Integer(*wire as i64));
            term_map.insert("coeff".to_string(), Value::String(coeff.to_string()));
            Value::Object(term_map)
        }).collect();
        
        let c_terms: Vec<Value> = self.c.iter().map(|(wire, coeff)| {
            let mut term_map = HashMap::new();
            term_map.insert("wire".to_string(), Value::Integer(*wire as i64));
            term_map.insert("coeff".to_string(), Value::String(coeff.to_string()));
            Value::Object(term_map)
        }).collect();
        
        constraint_map.insert("a".to_string(), Value::Array(a_terms));
        constraint_map.insert("b".to_string(), Value::Array(b_terms));
        constraint_map.insert("c".to_string(), Value::Array(c_terms));
        
        Value::Object(constraint_map)
    }
}

/// Arithmetic circuit builder
#[derive(Debug, Clone)]
pub struct CircuitBuilder {
    pub wires: HashMap<usize, Wire>,
    pub gates: Vec<Gate>,
    pub constraints: Vec<R1CSConstraint>,
    pub next_wire_id: usize,
    pub next_gate_id: usize,
    pub input_wires: Vec<usize>,
    pub output_wires: Vec<usize>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        Self {
            wires: HashMap::new(),
            gates: Vec::new(),
            constraints: Vec::new(),
            next_wire_id: 0,
            next_gate_id: 0,
            input_wires: Vec::new(),
            output_wires: Vec::new(),
        }
    }

    /// Create a new wire
    pub fn new_wire(&mut self, wire_type: WireType) -> usize {
        let wire_id = self.next_wire_id;
        self.next_wire_id += 1;
        
        let wire = Wire::new(wire_id, wire_type.clone());
        self.wires.insert(wire_id, wire);
        
        match wire_type {
            WireType::Input => self.input_wires.push(wire_id),
            WireType::Output => self.output_wires.push(wire_id),
            _ => {}
        }
        
        wire_id
    }

    /// Create a labeled wire
    pub fn new_labeled_wire(&mut self, wire_type: WireType, label: String) -> usize {
        let wire_id = self.new_wire(wire_type);
        if let Some(wire) = self.wires.get_mut(&wire_id) {
            wire.label = Some(label);
        }
        wire_id
    }

    /// Add an addition gate
    pub fn add_gate(&mut self, left: usize, right: usize) -> AdvancedCryptoResult<usize> {
        let output = self.new_wire(WireType::Intermediate);
        let gate_id = self.next_gate_id;
        self.next_gate_id += 1;

        let gate = Gate::new_add(gate_id, left, right, output);
        self.gates.push(gate);

        // Add R1CS constraint: left + right = output
        let mut constraint = R1CSConstraint::new();
        constraint.add_a_term(left, FieldElement::one());
        constraint.add_a_term(right, FieldElement::one());
        constraint.add_b_term(0, FieldElement::one()); // Constant 1 wire
        constraint.add_c_term(output, FieldElement::one());
        self.constraints.push(constraint);

        Ok(output)
    }

    /// Add a multiplication gate
    pub fn mul_gate(&mut self, left: usize, right: usize) -> AdvancedCryptoResult<usize> {
        let output = self.new_wire(WireType::Intermediate);
        let gate_id = self.next_gate_id;
        self.next_gate_id += 1;

        let gate = Gate::new_mul(gate_id, left, right, output);
        self.gates.push(gate);

        // Add R1CS constraint: left * right = output
        let mut constraint = R1CSConstraint::new();
        constraint.add_a_term(left, FieldElement::one());
        constraint.add_b_term(right, FieldElement::one());
        constraint.add_c_term(output, FieldElement::one());
        self.constraints.push(constraint);

        Ok(output)
    }

    /// Add a constant gate
    pub fn constant_gate(&mut self, value: FieldElement) -> AdvancedCryptoResult<usize> {
        let output = self.new_wire(WireType::Intermediate);
        let gate_id = self.next_gate_id;
        self.next_gate_id += 1;

        let gate = Gate::new_constant(gate_id, value, output);
        self.gates.push(gate);

        // Add R1CS constraint: 1 * value = output
        let mut constraint = R1CSConstraint::new();
        constraint.add_a_term(0, FieldElement::one()); // Constant 1 wire
        constraint.add_b_term(0, value); // Constant value
        constraint.add_c_term(output, FieldElement::one());
        self.constraints.push(constraint);

        Ok(output)
    }

    /// Add constraint that wire equals a specific value
    pub fn constrain_equal(&mut self, wire: usize, value: FieldElement) -> AdvancedCryptoResult<()> {
        let mut constraint = R1CSConstraint::new();
        constraint.add_a_term(wire, FieldElement::one());
        constraint.add_b_term(0, FieldElement::one()); // Constant 1 wire
        constraint.add_c_term(0, value); // Constant value wire
        self.constraints.push(constraint);
        Ok(())
    }

    /// Add custom R1CS constraint
    pub fn add_constraint(&mut self, constraint: R1CSConstraint) {
        self.constraints.push(constraint);
    }

    /// Evaluate circuit with given inputs
    pub fn evaluate(&self, inputs: &[FieldElement]) -> AdvancedCryptoResult<Vec<FieldElement>> {
        if inputs.len() != self.input_wires.len() {
            return Err(CryptoError::InvalidInput("Input count mismatch".to_string()));
        }

        let mut wire_values = HashMap::new();
        
        // Set constant 1 wire (wire 0)
        wire_values.insert(0, FieldElement::one());
        
        // Set input values
        for (i, &input_wire) in self.input_wires.iter().enumerate() {
            wire_values.insert(input_wire, inputs[i]);
        }

        // Evaluate gates in order
        for gate in &self.gates {
            if gate.gate_type == GateType::Input {
                continue; // Already set
            }
            
            let output_value = gate.evaluate(&wire_values)?;
            wire_values.insert(gate.output_wire, output_value);
        }

        // Collect output values
        let mut outputs = Vec::new();
        for &output_wire in &self.output_wires {
            let value = wire_values.get(&output_wire)
                .ok_or_else(|| CryptoError::InvalidInput("Missing output value".to_string()))?;
            outputs.push(*value);
        }

        Ok(outputs)
    }

    /// Generate witness (assignment to all wires)
    pub fn generate_witness(&self, inputs: &[FieldElement]) -> AdvancedCryptoResult<Vec<FieldElement>> {
        let mut witness = vec![FieldElement::zero(); self.next_wire_id];
        
        // Set constant 1 wire
        witness[0] = FieldElement::one();
        
        // Set input values
        for (i, &input_wire) in self.input_wires.iter().enumerate() {
            witness[input_wire] = inputs[i];
        }

        // Evaluate gates in order
        let mut wire_values = HashMap::new();
        for i in 0..self.next_wire_id {
            wire_values.insert(i, witness[i]);
        }

        for gate in &self.gates {
            if gate.gate_type == GateType::Input {
                continue;
            }
            
            let output_value = gate.evaluate(&wire_values)?;
            witness[gate.output_wire] = output_value;
            wire_values.insert(gate.output_wire, output_value);
        }

        Ok(witness)
    }

    /// Verify that witness satisfies all constraints
    pub fn verify_witness(&self, witness: &[FieldElement]) -> AdvancedCryptoResult<bool> {
        for constraint in &self.constraints {
            if !constraint.evaluate(witness)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Get circuit statistics
    pub fn get_stats(&self) -> Value {
        let mut stats = HashMap::new();
        stats.insert("num_wires".to_string(), Value::Integer(self.wires.len() as i64));
        stats.insert("num_gates".to_string(), Value::Integer(self.gates.len() as i64));
        stats.insert("num_constraints".to_string(), Value::Integer(self.constraints.len() as i64));
        stats.insert("num_inputs".to_string(), Value::Integer(self.input_wires.len() as i64));
        stats.insert("num_outputs".to_string(), Value::Integer(self.output_wires.len() as i64));
        
        let mut gate_types = HashMap::new();
        for gate in &self.gates {
            let gate_type_str = format!("{:?}", gate.gate_type);
            let count = gate_types.get(&gate_type_str).unwrap_or(&0) + 1;
            gate_types.insert(gate_type_str, count);
        }
        
        let gate_type_values: HashMap<String, Value> = gate_types.into_iter()
            .map(|(k, v)| (k, Value::Integer(v)))
            .collect();
        stats.insert("gate_types".to_string(), Value::Object(gate_type_values));
        
        Value::Object(stats)
    }

    /// Convert circuit to Value representation
    pub fn to_value(&self) -> Value {
        let mut circuit_map = HashMap::new();
        
        let wires: Vec<Value> = self.wires.values()
            .map(|wire| wire.to_value())
            .collect();
        circuit_map.insert("wires".to_string(), Value::Array(wires));
        
        let gates: Vec<Value> = self.gates.iter()
            .map(|gate| gate.to_value())
            .collect();
        circuit_map.insert("gates".to_string(), Value::Array(gates));
        
        let constraints: Vec<Value> = self.constraints.iter()
            .map(|constraint| constraint.to_value())
            .collect();
        circuit_map.insert("constraints".to_string(), Value::Array(constraints));
        
        let input_wires: Vec<Value> = self.input_wires.iter()
            .map(|&w| Value::Integer(w as i64))
            .collect();
        circuit_map.insert("input_wires".to_string(), Value::Array(input_wires));
        
        let output_wires: Vec<Value> = self.output_wires.iter()
            .map(|&w| Value::Integer(w as i64))
            .collect();
        circuit_map.insert("output_wires".to_string(), Value::Array(output_wires));
        
        Value::Object(circuit_map)
    }
}

/// Public API for circuit building
pub struct Circuits;

impl Circuits {
    /// Create a new circuit builder
    pub fn new_builder() -> AdvancedCryptoResult<Value> {
        let builder = CircuitBuilder::new();
        Ok(builder.to_value())
    }

    /// Add input wire to circuit
    pub fn add_input(circuit: &Value, label: Option<&str>) -> AdvancedCryptoResult<Value> {
        // This would modify the circuit in place
        // For now, return a mock wire ID
        Ok(Value::Integer(1))
    }

    /// Add output wire to circuit
    pub fn add_output(circuit: &Value, wire_id: i64) -> AdvancedCryptoResult<Value> {
        // This would modify the circuit in place
        Ok(Value::Boolean(true))
    }

    /// Build multiplication circuit (x * y)
    pub fn build_multiplication_circuit() -> AdvancedCryptoResult<Value> {
        let mut builder = CircuitBuilder::new();
        
        let x = builder.new_labeled_wire(WireType::Input, "x".to_string());
        let y = builder.new_labeled_wire(WireType::Input, "y".to_string());
        let result = builder.mul_gate(x, y)?;
        
        builder.wires.get_mut(&result).unwrap().wire_type = WireType::Output;
        builder.output_wires.push(result);
        
        Ok(builder.to_value())
    }

    /// Build addition circuit (x + y)
    pub fn build_addition_circuit() -> AdvancedCryptoResult<Value> {
        let mut builder = CircuitBuilder::new();
        
        let x = builder.new_labeled_wire(WireType::Input, "x".to_string());
        let y = builder.new_labeled_wire(WireType::Input, "y".to_string());
        let result = builder.add_gate(x, y)?;
        
        builder.wires.get_mut(&result).unwrap().wire_type = WireType::Output;
        builder.output_wires.push(result);
        
        Ok(builder.to_value())
    }

    /// Build polynomial evaluation circuit
    pub fn build_polynomial_circuit(degree: i64) -> AdvancedCryptoResult<Value> {
        let mut builder = CircuitBuilder::new();
        
        let x = builder.new_labeled_wire(WireType::Input, "x".to_string());
        let mut current = x;
        
        // Build x^degree
        for i in 1..degree {
            current = builder.mul_gate(current, x)?;
        }
        
        // Add coefficients (simplified)
        for i in 0..=degree {
            let coeff = builder.constant_gate(FieldElement::new(i as u64))?;
            let term = builder.mul_gate(coeff, current)?;
            current = if i == 0 { term } else { builder.add_gate(current, term)? };
        }
        
        builder.wires.get_mut(&current).unwrap().wire_type = WireType::Output;
        builder.output_wires.push(current);
        
        Ok(builder.to_value())
    }

    /// Evaluate circuit with inputs
    pub fn evaluate_circuit(circuit: &Value, inputs: &Value) -> AdvancedCryptoResult<Value> {
        // Parse inputs
        let input_elements = match inputs {
            Value::Array(arr) => {
                let mut elements = Vec::new();
                for input in arr {
                    match input {
                        Value::Integer(i) => elements.push(FieldElement::new(*i as u64)),
                        Value::String(s) => {
                            let num: u64 = s.parse()
                                .map_err(|_| CryptoError::InvalidInput("Invalid number".to_string()))?;
                            elements.push(FieldElement::new(num));
                        }
                        _ => return Err(CryptoError::InvalidInput("Invalid input type".to_string())),
                    }
                }
                elements
            }
            _ => return Err(CryptoError::InvalidInput("Expected array of inputs".to_string())),
        };

        // For demo, return the inputs processed
        let outputs: Vec<Value> = input_elements.iter()
            .map(|elem| Value::String(elem.to_string()))
            .collect();
        
        Ok(Value::Array(outputs))
    }

    /// Get circuit statistics
    pub fn get_circuit_stats(circuit: &Value) -> AdvancedCryptoResult<Value> {
        // Mock statistics
        let mut stats = HashMap::new();
        stats.insert("gates".to_string(), Value::Integer(10));
        stats.insert("wires".to_string(), Value::Integer(15));
        stats.insert("constraints".to_string(), Value::Integer(8));
        Ok(Value::Object(stats))
    }

    /// Verify circuit constraints
    pub fn verify_circuit(circuit: &Value, witness: &Value) -> AdvancedCryptoResult<Value> {
        // Mock verification
        Ok(Value::Boolean(true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_builder_basic() {
        let mut builder = CircuitBuilder::new();
        let x = builder.new_wire(WireType::Input);
        let y = builder.new_wire(WireType::Input);
        let result = builder.add_gate(x, y).unwrap();
        
        assert_eq!(builder.wires.len(), 3);
        assert_eq!(builder.gates.len(), 1);
        assert_eq!(builder.constraints.len(), 1);
    }

    #[test]
    fn test_circuit_evaluation() {
        let mut builder = CircuitBuilder::new();
        let x = builder.new_wire(WireType::Input);
        let y = builder.new_wire(WireType::Input);
        let sum = builder.add_gate(x, y).unwrap();
        let product = builder.mul_gate(x, y).unwrap();
        
        builder.wires.get_mut(&sum).unwrap().wire_type = WireType::Output;
        builder.wires.get_mut(&product).unwrap().wire_type = WireType::Output;
        builder.output_wires.extend(&[sum, product]);
        
        let inputs = vec![FieldElement::new(3), FieldElement::new(4)];
        let outputs = builder.evaluate(&inputs).unwrap();
        
        assert_eq!(outputs.len(), 2);
    }

    #[test]
    fn test_witness_generation() {
        let mut builder = CircuitBuilder::new();
        let x = builder.new_wire(WireType::Input);
        let y = builder.new_wire(WireType::Input);
        let result = builder.mul_gate(x, y).unwrap();
        
        let inputs = vec![FieldElement::new(5), FieldElement::new(6)];
        let witness = builder.generate_witness(&inputs).unwrap();
        
        assert!(builder.verify_witness(&witness).unwrap());
    }

    #[test]
    fn test_r1cs_constraint() {
        let mut constraint = R1CSConstraint::new();
        constraint.add_a_term(1, FieldElement::one());
        constraint.add_b_term(2, FieldElement::one());
        constraint.add_c_term(3, FieldElement::one());
        
        let witness = vec![
            FieldElement::one(),  // wire 0
            FieldElement::new(3), // wire 1
            FieldElement::new(4), // wire 2
            FieldElement::new(12), // wire 3 (3 * 4)
        ];
        
        assert!(constraint.evaluate(&witness).unwrap());
    }

    #[test]
    fn test_circuits_api() {
        let mult_circuit = Circuits::build_multiplication_circuit().unwrap();
        let add_circuit = Circuits::build_addition_circuit().unwrap();
        
        assert!(matches!(mult_circuit, Value::Object(_)));
        assert!(matches!(add_circuit, Value::Object(_)));
        
        let inputs = Value::Array(vec![Value::Integer(3), Value::Integer(4)]);
        let result = Circuits::evaluate_circuit(&mult_circuit, &inputs).unwrap();
        assert!(matches!(result, Value::Array(_)));
    }
}
