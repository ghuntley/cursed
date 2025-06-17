/// Method represents a method on a type for CURSED reflection
use crate::stdlib::lookin_glass::{Type, Value};
use std::fmt;

/// Represents a method on a type
#[derive(Debug, Clone)]
pub struct Method {
    /// Name of the method
    pub name: String,
    /// Package path where the method is defined
    pub pkg_path: String,
    /// Type of the method (function signature)
    pub method_type: Type,
    /// Function value representing the method
    pub func: Value,
    /// Index of this method in the type's method set
    pub index: usize,
}

impl Method {
    /// Create a new Method
    pub fn new(
        name: String,
        pkg_path: String,
        method_type: Type,
        func: Value,
        index: usize,
    ) -> Self {
        Self {
            name,
            pkg_path,
            method_type,
            func,
            index,
        }
    }

    /// Get the method name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the package path
    pub fn pkg_path(&self) -> &str {
        &self.pkg_path
    }

    /// Get the method type (function signature)
    pub fn method_type(&self) -> &Type {
        &self.method_type
    }

    /// Get the function value
    pub fn func(&self) -> &Value {
        &self.func
    }

    /// Get the method index
    pub fn index(&self) -> usize {
        self.index
    }

    /// Check if this method is exported (accessible from other packages)
    pub fn is_exported(&self) -> bool {
        self.pkg_path.is_empty() && 
        self.name.chars().next().map_or(false, |c| c.is_uppercase())
    }

    /// Check if this method can be called
    pub fn is_callable(&self) -> bool {
        self.func.is_valid() && self.method_type.is_func()
    }

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
    }

    /// Get the method signature as a string
    pub fn signature(&self) -> String {
        if !self.method_type.is_func() {
            return "invalid".to_string();
        }

        let mut parts = Vec::new();
        
        // Add receiver (skip in the in_types since it's the first parameter)
        if let Some(receiver) = self.receiver_type() {
            parts.push(format!("({}) ", receiver));
        }

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
    }

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
    name: String,
    pkg_path: String,
    method_type: Type,
    func: Option<Value>,
    index: usize,
}

impl MethodBuilder {
    /// Create a new builder
    pub fn new(name: String, method_type: Type) -> Self {
        Self {
            name,
            pkg_path: String::new(),
            method_type,
            func: None,
            index: 0,
        }
    }

    /// Set the package path
    pub fn pkg_path(mut self, pkg_path: String) -> Self {
        self.pkg_path = pkg_path;
        self
    }

    /// Set the function value
    pub fn func(mut self, func: Value) -> Self {
        self.func = Some(func);
        self
    }

    /// Set the method index
    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    /// Build the Method
    pub fn build(self) -> Method {
        Method {
            name: self.name,
            pkg_path: self.pkg_path,
            method_type: self.method_type,
            func: self.func.unwrap_or_else(|| Value::invalid()),
            index: self.index,
        }
    }
}

/// Collection of methods for a type
#[derive(Debug, Clone)]
pub struct MethodSet {
    methods: Vec<Method>,
}

impl MethodSet {
    /// Create a new empty method set
    pub fn new() -> Self {
        Self {
            methods: Vec::new(),
        }
    }

    /// Create a method set from a vector of methods
    pub fn from_methods(methods: Vec<Method>) -> Self {
        Self { methods }
    }

    /// Get the number of methods
    pub fn len(&self) -> usize {
        self.methods.len()
    }

    /// Check if the method set is empty
    pub fn is_empty(&self) -> bool {
        self.methods.is_empty()
    }

    /// Get a method by index
    pub fn method(&self, index: usize) -> Option<&Method> {
        self.methods.get(index)
    }

    /// Get a method by name
    pub fn method_by_name(&self, name: &str) -> Option<&Method> {
        self.methods.iter().find(|m| m.name == name)
    }

    /// Get all methods
    pub fn methods(&self) -> &[Method] {
        &self.methods
    }

    /// Add a method to the set
    pub fn add_method(&mut self, method: Method) {
        self.methods.push(method);
    }

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
    }

    /// Check if a method with the given name exists
    pub fn has_method(&self, name: &str) -> bool {
        self.methods.iter().any(|m| m.name == name)
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::lookin_glass::{Type, Kind, Value};

    fn create_func_type() -> Type {
        // Create a simple function type: func(string) int
        Type::new(Kind::Func, "func(string) int".to_string(), "".to_string())
    }

    fn create_test_value() -> Value {
        Value::invalid() // For testing purposes
    }

    #[test]
    fn test_method_creation() {
        let method_type = create_func_type();
        let func_value = create_test_value();
        
        let method = Method::new(
            "TestMethod".to_string(),
            "".to_string(),
            method_type,
            func_value,
            0,
        );

        assert_eq!(method.name(), "TestMethod");
        assert_eq!(method.pkg_path(), "");
        assert_eq!(method.index(), 0);
        assert!(method.is_exported());
    }

    #[test]
    fn test_method_builder() {
        let method_type = create_func_type();
        let func_value = create_test_value();
        
        let method = Method::builder("TestMethod".to_string(), method_type)
            .pkg_path("test".to_string())
            .func(func_value)
            .index(5)
            .build();

        assert_eq!(method.name(), "TestMethod");
        assert_eq!(method.pkg_path(), "test");
        assert_eq!(method.index(), 5);
        assert!(!method.is_exported()); // Not exported due to pkg_path
    }

    #[test]
    fn test_unexported_method() {
        let method_type = create_func_type();
        let method = Method::builder("privateMethod".to_string(), method_type)
            .build();

        assert!(!method.is_exported()); // Starts with lowercase
    }

    #[test]
    fn test_method_set() {
        let mut method_set = MethodSet::new();
        assert!(method_set.is_empty());
        assert_eq!(method_set.len(), 0);

        let method1 = Method::builder("Method1".to_string(), create_func_type()).build();
        let method2 = Method::builder("Method2".to_string(), create_func_type()).build();

        method_set.add_method(method1);
        method_set.add_method(method2);

        assert_eq!(method_set.len(), 2);
        assert!(!method_set.is_empty());
        assert!(method_set.has_method("Method1"));
        assert!(method_set.has_method("Method2"));
        assert!(!method_set.has_method("Method3"));

        assert!(method_set.method_by_name("Method1").is_some());
        assert!(method_set.method_by_name("Method3").is_none());

        let exported = method_set.exported_names();
        assert_eq!(exported.len(), 2);
        assert!(exported.contains(&"Method1".to_string()));
        assert!(exported.contains(&"Method2".to_string()));
    }

    #[test]
    fn test_method_set_removal() {
        let mut method_set = MethodSet::new();
        let method = Method::builder("TestMethod".to_string(), create_func_type()).build();
        
        method_set.add_method(method);
        assert_eq!(method_set.len(), 1);

        let removed = method_set.remove_method("TestMethod");
        assert!(removed.is_some());
        assert_eq!(method_set.len(), 0);

        let not_found = method_set.remove_method("NonExistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_method_set_sorting() {
        let mut method_set = MethodSet::new();
        
        method_set.add_method(Method::builder("ZMethod".to_string(), create_func_type()).build());
        method_set.add_method(Method::builder("AMethod".to_string(), create_func_type()).build());
        method_set.add_method(Method::builder("MMethod".to_string(), create_func_type()).build());

        method_set.sort_by_name();

        let methods = method_set.methods();
        assert_eq!(methods[0].name(), "AMethod");
        assert_eq!(methods[1].name(), "MMethod");
        assert_eq!(methods[2].name(), "ZMethod");
    }

    #[test]
    fn test_method_display() {
        let method_type = create_func_type();
        let method = Method::builder("TestMethod".to_string(), method_type).build();
        
        let display = format!("{}", method);
        assert!(display.contains("TestMethod"));
    }
}
