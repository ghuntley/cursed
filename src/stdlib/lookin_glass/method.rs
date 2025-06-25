/// Method represents a method on a type for CURSED reflection
// use crate::stdlib::lookin_glass::{Type, Value};
use std::fmt;

/// Represents a method on a type
#[derive(Debug, Clone)]
pub struct Method {
    /// Name of the method
    /// Package path where the method is defined
    /// Type of the method (function signature)
    /// Function value representing the method
    /// Index of this method in the type's method set
impl Method {
    /// Create a new Method
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    /// Get the method name
    pub fn name(&self) -> &str {
        &self.name
    /// Get the package path
    pub fn pkg_path(&self) -> &str {
        &self.pkg_path
    /// Get the method type (function signature)
    pub fn method_type(&self) -> &Type {
        &self.method_type
    /// Get the function value
    pub fn func(&self) -> &Value {
        &self.func
    /// Get the method index
    pub fn index(&self) -> usize {
        self.index
    /// Check if this method is exported (accessible from other packages)
    pub fn is_exported(&self) -> bool {
        self.pkg_path.is_empty() && 
        self.name.chars().next().map_or(false, |c| c.is_uppercase())
    /// Check if this method can be called
    pub fn is_callable(&self) -> bool {
        self.func.is_valid() && self.method_type.is_func()
    /// Get the number of input parameters (excluding receiver)
    pub fn num_in(&self) -> usize {
        if self.method_type.is_func() {
            // Subtract 1 for the receiver
            self.method_type.num_in().saturating_sub(1)
        } else {
            0
        }
    }

    /// Get the number of output parameters
    pub fn num_out(&self) -> usize {
        if self.method_type.is_func() {
            self.method_type.num_out()
        } else {
            0
        }
    }

    /// Get the input parameter type at index i (excluding receiver)
    pub fn in_type(&self, i: usize) -> Option<Type> {
        if self.method_type.is_func() && i + 1 < self.method_type.num_in() {
            // Add 1 to skip the receiver
            self.method_type.in_type(i + 1)
        } else {
            None
        }
    }

    /// Get the output parameter type at index i
    pub fn out_type(&self, i: usize) -> Option<Type> {
        if self.method_type.is_func() && i < self.method_type.num_out() {
            self.method_type.out_type(i)
        } else {
            None
        }
    }

    /// Get the receiver type (first parameter of the method)
    pub fn receiver_type(&self) -> Option<Type> {
        if self.method_type.is_func() && self.method_type.num_in() > 0 {
            self.method_type.in_type(0)
        } else {
            None
        }
    }

    /// Check if this method is variadic (takes variable number of arguments)
    pub fn is_variadic(&self) -> bool {
        self.method_type.is_variadic()
    /// Get the method signature as a string
    pub fn signature(&self) -> String {
        if !self.method_type.is_func() {
            return "invalid".to_string();
        let mut parts = Vec::new();
        
        // Add receiver (skip in the in_types since it's the first parameter)
        if let Some(receiver) = self.receiver_type() {
            parts.push(format!("({}) ", receiver));
        // Method name
        parts.push(self.name.clone());

        // Input parameters (excluding receiver)
        let mut inputs = Vec::new();
        for i in 0..self.num_in() {
            if let Some(param_type) = self.in_type(i) {
                inputs.push(param_type.to_string());
            }
        }
        
        if self.is_variadic() && !inputs.is_empty() {
            if let Some(last) = inputs.last_mut() {
                *last = format!("...{}", last);
            }
        }
        
        parts.push(format!("({})", inputs.join(", ")));

        // Output parameters
        let mut outputs = Vec::new();
        for i in 0..self.num_out() {
            if let Some(out_type) = self.out_type(i) {
                outputs.push(out_type.to_string());
            }
        }

        if !outputs.is_empty() {
            if outputs.len() == 1 {
                parts.push(format!(" {}", outputs[0]));
            } else {
                parts.push(format!(" ({})", outputs.join(", ")));
            }
        }

        parts.join("")
    /// Create a method builder for fluent construction
    pub fn builder(name: String, method_type: Type) -> MethodBuilder {
        MethodBuilder::new(name, method_type)
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.signature())
    }
}

/// Builder for creating Method instances
pub struct MethodBuilder {
impl MethodBuilder {
    /// Create a new builder
    pub fn new(name: String, method_type: Type) -> Self {
        Self {
        }
    }

    /// Set the package path
    pub fn pkg_path(mut self, pkg_path: String) -> Self {
        self.pkg_path = pkg_path;
        self
    /// Set the function value
    pub fn func(mut self, func: Value) -> Self {
        self.func = Some(func);
        self
    /// Set the method index
    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    /// Build the Method
    pub fn build(self) -> Method {
        Method {
        }
    }
/// Collection of methods for a type
#[derive(Debug, Clone)]
pub struct MethodSet {
impl MethodSet {
    /// Create a new empty method set
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create a method set from a vector of methods
    pub fn from_methods(methods: Vec<Method>) -> Self {
        Self { methods }
    }

    /// Get the number of methods
    pub fn len(&self) -> usize {
        self.methods.len()
    /// Check if the method set is empty
    pub fn is_empty(&self) -> bool {
        self.methods.is_empty()
    /// Get a method by index
    pub fn method(&self, index: usize) -> Option<&Method> {
        self.methods.get(index)
    /// Get a method by name
    pub fn method_by_name(&self, name: &str) -> Option<&Method> {
        self.methods.iter().find(|m| m.name == name)
    /// Get all methods
    pub fn methods(&self) -> &[Method] {
        &self.methods
    /// Add a method to the set
    pub fn add_method(&mut self, method: Method) {
        self.methods.push(method);
    /// Remove a method by name
    pub fn remove_method(&mut self, name: &str) -> Option<Method> {
        if let Some(pos) = self.methods.iter().position(|m| m.name == name) {
            Some(self.methods.remove(pos))
        } else {
            None
        }
    }

    /// Get all exported method names
    pub fn exported_names(&self) -> Vec<String> {
        self.methods
            .iter()
            .filter(|m| m.is_exported())
            .map(|m| m.name.clone())
            .collect()
    /// Check if a method with the given name exists
    pub fn has_method(&self, name: &str) -> bool {
        self.methods.iter().any(|m| m.name == name)
    /// Sort methods by name
    pub fn sort_by_name(&mut self) {
        self.methods.sort_by(|a, b| a.name.cmp(&b.name));
    }
}

impl Default for MethodSet {
    fn default() -> Self {
        Self::new()
    }
}

