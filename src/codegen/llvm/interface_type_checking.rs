//! Interface Type Checking for LLVM Code Generation
//!
//! This module provides compile-time interface type checking and validation
//! for the CURSED compiler's LLVM backend.

use crate::ast::{InterfaceStatement, Type as AstType, Expression, Statement, StructStatement, FunctionDeclaration};
use crate::error::{CursedError, SourceLocation};
use crate::type_system::interface_compliance::{InterfaceComplianceChecker, InterfaceMethodRequirement, ConcreteMethodImplementation, ReceiverType};
use crate::codegen::llvm::register_tracker::RegisterTracker;
use std::collections::HashMap;

/// Interface type checker for LLVM codegen
#[derive(Debug, Clone)]
pub struct InterfaceTypeChecker {
    /// Interface compliance checker
    compliance_checker: InterfaceComplianceChecker,
    /// Type to interface mapping
    type_interfaces: HashMap<String, Vec<String>>,
    /// Interface inheritance graph
    interface_inheritance: HashMap<String, Vec<String>>,
    /// Compile-time interface validation results
    validation_cache: HashMap<String, bool>,
    /// Error context for reporting
    current_location: Option<SourceLocation>,
}

impl InterfaceTypeChecker {
    /// Create new interface type checker
    pub fn new() -> Self {
        Self {
            compliance_checker: InterfaceComplianceChecker::new(),
            type_interfaces: HashMap::new(),
            interface_inheritance: HashMap::new(),
            validation_cache: HashMap::new(),
            current_location: None,
        }
    }

    /// Process interface definitions and build type system
    pub fn process_interfaces(&mut self, statements: &[Statement]) -> Result<(), CursedError> {
        // First pass: register all interface definitions
        for statement in statements {
            if let Statement::Interface(interface) = statement {
                self.register_interface(interface)?;
            }
        }

        // Second pass: process struct definitions and their method implementations
        for statement in statements {
            if let Statement::Struct(struct_def) = statement {
                self.process_struct_methods(struct_def)?;
            }
        }

        // Third pass: validate all interface implementations
        self.validate_all_implementations()?;

        Ok(())
    }

    /// Register interface definition
    fn register_interface(&mut self, interface: &InterfaceStatement) -> Result<(), CursedError> {
        // Register with compliance checker
        self.compliance_checker.register_interface(interface)?;

        // Build inheritance graph
        if !interface.extends.is_empty() {
            self.interface_inheritance.insert(interface.name.clone(), interface.extends.clone());
        }

        Ok(())
    }

    /// Process struct method implementations
    fn process_struct_methods(&mut self, struct_def: &StructStatement) -> Result<(), CursedError> {
        // Extract method implementations from struct
        let mut method_implementations = Vec::new();
        
        // TODO: In a real implementation, this would analyze the struct's methods
        // For now, we'll create a placeholder implementation
        
        // Register method implementations
        if !method_implementations.is_empty() {
            self.compliance_checker.register_type_methods(&struct_def.name, method_implementations)?;
        }

        Ok(())
    }

    /// Validate all interface implementations
    fn validate_all_implementations(&mut self) -> Result<(), CursedError> {
        // Check all type-interface pairs for compliance
        for (type_name, interfaces) in &self.type_interfaces {
            for interface_name in interfaces {
                let is_compliant = self.compliance_checker.check_interface_compliance(type_name, interface_name)?;
                let cache_key = format!("{}::{}", type_name, interface_name);
                self.validation_cache.insert(cache_key, is_compliant);
                
                if !is_compliant {
                    return Err(CursedError::TypeError(format!(
                        "Type '{}' does not implement interface '{}' correctly",
                        type_name, interface_name
                    )));
                }
            }
        }

        Ok(())
    }

    /// Check if type implements interface (with caching)
    pub fn check_interface_implementation(&self, type_name: &str, interface_name: &str) -> Result<bool, CursedError> {
        let cache_key = format!("{}::{}", type_name, interface_name);
        
        if let Some(&result) = self.validation_cache.get(&cache_key) {
            return Ok(result);
        }

        // Fallback to runtime check
        self.compliance_checker.check_interface_compliance(type_name, interface_name)
            .map_err(|e| CursedError::CompilerError(e.to_string()))
    }

    /// Generate LLVM IR for interface type checking
    pub fn generate_type_checking_ir(&self, type_name: &str, interface_name: &str) -> Result<String, CursedError> {
        let mut ir_code = String::new();
        let mut register_tracker = RegisterTracker::new();

        // Generate compile-time interface checking
        if self.check_interface_implementation(type_name, interface_name)? {
            // Generate optimized path for statically verified interfaces
            ir_code.push_str(&format!(
                r#"
; Compile-time verified interface implementation: {} -> {}
define i1 @check_interface_{}_{}_static() {{
entry:
    ret i1 true
}}

"#,
                type_name, interface_name, type_name, interface_name
            ));
        } else {
            // Generate runtime check for dynamic interfaces
            ir_code.push_str(&format!(
                r#"
; Runtime interface checking: {} -> {}
define i1 @check_interface_{}_{}_runtime(i8* %type_info, i8* %interface_info) {{
entry:
    %result = call i1 @cursed_runtime_check_interface(i8* %type_info, i8* %interface_info)
    ret i1 %result
}}

"#,
                type_name, interface_name, type_name, interface_name
            ));
        }

        // Generate interface casting function
        ir_code.push_str(&self.generate_interface_cast_ir(type_name, interface_name)?);

        // Generate type assertion function
        ir_code.push_str(&self.generate_type_assertion_ir(type_name, interface_name)?);

        Ok(ir_code)
    }

    /// Generate LLVM IR for interface casting
    fn generate_interface_cast_ir(&self, type_name: &str, interface_name: &str) -> Result<String, CursedError> {
        let mut ir_code = String::new();

        // Generate safe interface cast with type checking
        ir_code.push_str(&format!(
            r#"
; Safe interface cast: {} to {}
define i8* @cast_to_interface_{}_{}_safe(i8* %object) {{
entry:
    ; Check if object implements interface
    %implements = call i1 @check_interface_{}_{}_static()
    br i1 %implements, label %cast_success, label %cast_failure

cast_success:
    ; Get vtable for this type-interface pair
    %vtable = call i8* @get_vtable_{}_{}_static()
    
    ; Create interface value
    %interface_value = call i8* @create_interface_value(i8* %vtable, i8* %object, i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.type.{}, i32 0, i32 0))
    ret i8* %interface_value

cast_failure:
    ; Return null for failed cast
    ret i8* null
}}

"#,
            type_name, interface_name,
            type_name, interface_name,
            type_name, interface_name,
            type_name, interface_name,
            type_name.len() + 1, type_name.len() + 1, type_name
        ));

        // Generate unsafe interface cast (no type checking)
        ir_code.push_str(&format!(
            r#"
; Unsafe interface cast: {} to {} (no type checking)
define i8* @cast_to_interface_{}_{}_unsafe(i8* %object) {{
entry:
    ; Get vtable for this type-interface pair
    %vtable = call i8* @get_vtable_{}_{}_static()
    
    ; Create interface value without type checking
    %interface_value = call i8* @create_interface_value(i8* %vtable, i8* %object, i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.type.{}, i32 0, i32 0))
    ret i8* %interface_value
}}

"#,
            type_name, interface_name,
            type_name, interface_name,
            type_name, interface_name,
            type_name.len() + 1, type_name.len() + 1, type_name
        ));

        Ok(ir_code)
    }

    /// Generate LLVM IR for type assertion
    fn generate_type_assertion_ir(&self, type_name: &str, interface_name: &str) -> Result<String, CursedError> {
        let mut ir_code = String::new();

        // Generate type assertion with panic on failure
        ir_code.push_str(&format!(
            r#"
; Type assertion: {} to {}
define i8* @assert_interface_{}_{}_panic(i8* %object) {{
entry:
    ; Check if object implements interface
    %implements = call i1 @check_interface_{}_{}_static()
    br i1 %implements, label %assert_success, label %assert_failure

assert_success:
    ; Get vtable for this type-interface pair
    %vtable = call i8* @get_vtable_{}_{}_static()
    
    ; Create interface value
    %interface_value = call i8* @create_interface_value(i8* %vtable, i8* %object, i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.type.{}, i32 0, i32 0))
    ret i8* %interface_value

assert_failure:
    ; Panic with type assertion error
    call void @cursed_panic_type_assertion(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.type.{}, i32 0, i32 0), i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.interface.{}, i32 0, i32 0))
    unreachable
}}

"#,
            type_name, interface_name,
            type_name, interface_name,
            type_name, interface_name,
            type_name, interface_name,
            type_name.len() + 1, type_name.len() + 1, type_name,
            type_name.len() + 1, type_name.len() + 1, type_name,
            interface_name.len() + 1, interface_name.len() + 1, interface_name
        ));

        // Generate type assertion with boolean result
        ir_code.push_str(&format!(
            r#"
; Type assertion: {} to {} (boolean result)
define {{i8*, i1}} @assert_interface_{}_{}_bool(i8* %object) {{
entry:
    ; Check if object implements interface
    %implements = call i1 @check_interface_{}_{}_static()
    br i1 %implements, label %assert_success, label %assert_failure

assert_success:
    ; Get vtable for this type-interface pair
    %vtable = call i8* @get_vtable_{}_{}_static()
    
    ; Create interface value
    %interface_value = call i8* @create_interface_value(i8* %vtable, i8* %object, i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.type.{}, i32 0, i32 0))
    
    ; Create success result
    %success_result = insertvalue {{i8*, i1}} undef, i8* %interface_value, 0
    %success_result_final = insertvalue {{i8*, i1}} %success_result, i1 true, 1
    ret {{i8*, i1}} %success_result_final

assert_failure:
    ; Create failure result
    %failure_result = insertvalue {{i8*, i1}} undef, i8* null, 0
    %failure_result_final = insertvalue {{i8*, i1}} %failure_result, i1 false, 1
    ret {{i8*, i1}} %failure_result_final
}}

"#,
            type_name, interface_name,
            type_name, interface_name,
            type_name, interface_name,
            type_name, interface_name,
            type_name.len() + 1, type_name.len() + 1, type_name
        ));

        Ok(ir_code)
    }

    /// Generate interface hierarchy checking
    pub fn generate_interface_hierarchy_check(&self, derived_interface: &str, base_interface: &str) -> Result<String, CursedError> {
        let mut ir_code = String::new();

        // Check if derived interface extends base interface
        if let Some(extends) = self.interface_inheritance.get(derived_interface) {
            if extends.contains(&base_interface.to_string()) {
                // Generate compile-time verified hierarchy check
                ir_code.push_str(&format!(
                    r#"
; Compile-time verified interface hierarchy: {} extends {}
define i1 @check_interface_hierarchy_{}_{}_static() {{
entry:
    ret i1 true
}}

"#,
                    derived_interface, base_interface,
                    derived_interface, base_interface
                ));
            } else {
                // Generate runtime hierarchy check
                ir_code.push_str(&format!(
                    r#"
; Runtime interface hierarchy check: {} extends {}
define i1 @check_interface_hierarchy_{}_{}_runtime() {{
entry:
    %result = call i1 @cursed_runtime_check_interface_hierarchy(i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.interface.{}, i32 0, i32 0), i8* getelementptr inbounds ([{} x i8], [{} x i8]* @str.interface.{}, i32 0, i32 0))
    ret i1 %result
}}

"#,
                    derived_interface, base_interface,
                    derived_interface, base_interface,
                    derived_interface.len() + 1, derived_interface.len() + 1, derived_interface,
                    base_interface.len() + 1, base_interface.len() + 1, base_interface
                ));
            }
        }

        Ok(ir_code)
    }

    /// Generate vtable lookup functions
    pub fn generate_vtable_lookup_functions(&self) -> Result<String, CursedError> {
        let mut ir_code = String::new();

        // Generate static vtable lookup functions for all known type-interface pairs
        for (type_name, interfaces) in &self.type_interfaces {
            for interface_name in interfaces {
                ir_code.push_str(&format!(
                    r#"
; Static vtable lookup for {} implementing {}
define i8* @get_vtable_{}_{}_static() {{
entry:
    ret i8* bitcast (%vtable.{}* @vtable.{}.{} to i8*)
}}

"#,
                    type_name, interface_name,
                    type_name, interface_name,
                    interface_name, interface_name, type_name
                ));
            }
        }

        // Generate runtime vtable lookup function
        ir_code.push_str(&format!(
            r#"
; Runtime vtable lookup
declare i8* @cursed_runtime_get_vtable(i8*, i8*)

define i8* @get_vtable_runtime(i8* %type_name, i8* %interface_name) {{
entry:
    %vtable = call i8* @cursed_runtime_get_vtable(i8* %type_name, i8* %interface_name)
    ret i8* %vtable
}}

"#
        ));

        Ok(ir_code)
    }

    /// Add type-interface association
    pub fn add_type_interface_association(&mut self, type_name: String, interface_name: String) {
        self.type_interfaces
            .entry(type_name)
            .or_insert_with(Vec::new)
            .push(interface_name);
    }

    /// Get interfaces implemented by type
    pub fn get_interfaces_for_type(&self, type_name: &str) -> Option<&Vec<String>> {
        self.type_interfaces.get(type_name)
    }

    /// Check if interface extends another interface
    pub fn interface_extends(&self, derived: &str, base: &str) -> bool {
        if let Some(extends) = self.interface_inheritance.get(derived) {
            extends.contains(&base.to_string())
        } else {
            false
        }
    }

    /// Get all interfaces in hierarchy
    pub fn get_interface_hierarchy(&self, interface_name: &str) -> Vec<String> {
        let mut hierarchy = vec![interface_name.to_string()];
        
        if let Some(extends) = self.interface_inheritance.get(interface_name) {
            for base_interface in extends {
                hierarchy.extend(self.get_interface_hierarchy(base_interface));
            }
        }
        
        hierarchy
    }

    /// Set current source location for error reporting
    pub fn set_current_location(&mut self, location: Option<SourceLocation>) {
        self.current_location = location;
    }

    /// Get current source location
    pub fn get_current_location(&self) -> Option<&SourceLocation> {
        self.current_location.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_interface_type_checking() {
        let mut checker = InterfaceTypeChecker::new();
        
        // Create test interface
        let interface = InterfaceStatement {
            name: "TestInterface".to_string(),
            type_parameters: vec![],
            extends: vec![],
            compositions: vec![],
            methods: vec![
                MethodSignature {
                    name: "test_method".to_string(),
                    receiver: None,
                    parameters: vec![],
                    return_type: Some(AstType::Normie),
                }
            ],
            visibility: Visibility::Public,
        };
        
        // Test interface registration
        assert!(checker.register_interface(&interface).is_ok());
        
        // Test type checking IR generation
        let ir = checker.generate_type_checking_ir("TestType", "TestInterface");
        assert!(ir.is_ok());
        
        let ir_code = ir.unwrap();
        assert!(ir_code.contains("check_interface_TestType_TestInterface"));
    }

    #[test]
    fn test_interface_hierarchy() {
        let mut checker = InterfaceTypeChecker::new();
        
        // Test hierarchy checking
        checker.interface_inheritance.insert("DerivedInterface".to_string(), vec!["BaseInterface".to_string()]);
        
        assert!(checker.interface_extends("DerivedInterface", "BaseInterface"));
        assert!(!checker.interface_extends("BaseInterface", "DerivedInterface"));
        
        let hierarchy = checker.get_interface_hierarchy("DerivedInterface");
        assert!(hierarchy.contains(&"DerivedInterface".to_string()));
        assert!(hierarchy.contains(&"BaseInterface".to_string()));
    }
}
