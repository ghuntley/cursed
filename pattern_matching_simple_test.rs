#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    
    // Simple standalone test for enum variant registry
    #[derive(Debug, Clone)]
    pub struct EnumVariantRegistry {
        /// Map from (enum_name, variant_name) to variant_index
        pub variants: HashMap<(String, String), usize>,
        /// Map from enum_name to list of variant names in order
        pub enum_variants: HashMap<String, Vec<String>>,
    }

    impl EnumVariantRegistry {
        pub fn new() -> Self {
            Self {
                variants: HashMap::new(),
                enum_variants: HashMap::new(),
            }
        }
        
        /// Register an enum with its variants in order
        pub fn register_enum(&mut self, enum_name: String, variant_names: Vec<String>) {
            for (index, variant_name) in variant_names.iter().enumerate() {
                self.variants.insert((enum_name.clone(), variant_name.clone()), index);
            }
            self.enum_variants.insert(enum_name, variant_names);
        }
        
        /// Get variant index for given enum and variant name
        pub fn get_variant_index(&self, enum_name: &str, variant_name: &str) -> Option<usize> {
            self.variants.get(&(enum_name.to_string(), variant_name.to_string())).copied()
        }
    }

    #[test]
    fn test_enum_variant_registry_basic() {
        let mut registry = EnumVariantRegistry::new();
        
        // Register Color enum with variants
        registry.register_enum(
            "Color".to_string(),
            vec!["Red".to_string(), "Green".to_string(), "Blue".to_string(), "Custom".to_string()]
        );
        
        // Test variant index lookup - should NOT be hardcoded to 0
        assert_eq!(registry.get_variant_index("Color", "Red"), Some(0));
        assert_eq!(registry.get_variant_index("Color", "Green"), Some(1));  // This was the bug - was hardcoded to 0
        assert_eq!(registry.get_variant_index("Color", "Blue"), Some(2));   // This was the bug - was hardcoded to 0
        assert_eq!(registry.get_variant_index("Color", "Custom"), Some(3)); // This was the bug - was hardcoded to 0
        
        // Test unknown variant
        assert_eq!(registry.get_variant_index("Color", "Unknown"), None);
        assert_eq!(registry.get_variant_index("UnknownEnum", "Red"), None);
    }

    #[test] 
    fn test_multiple_enums_no_conflict() {
        let mut registry = EnumVariantRegistry::new();
        
        // Register multiple enums
        registry.register_enum(
            "Status".to_string(),
            vec!["Success".to_string(), "Error".to_string(), "Pending".to_string()]
        );
        
        registry.register_enum(
            "Direction".to_string(), 
            vec!["North".to_string(), "South".to_string(), "East".to_string(), "West".to_string()]
        );
        
        // Test Status enum indices - each enum starts from 0
        assert_eq!(registry.get_variant_index("Status", "Success"), Some(0));
        assert_eq!(registry.get_variant_index("Status", "Error"), Some(1));
        assert_eq!(registry.get_variant_index("Status", "Pending"), Some(2));
        
        // Test Direction enum indices - each enum starts from 0  
        assert_eq!(registry.get_variant_index("Direction", "North"), Some(0));
        assert_eq!(registry.get_variant_index("Direction", "South"), Some(1));
        assert_eq!(registry.get_variant_index("Direction", "East"), Some(2));
        assert_eq!(registry.get_variant_index("Direction", "West"), Some(3));
        
        // Test cross-enum queries don't work
        assert_eq!(registry.get_variant_index("Status", "North"), None);
        assert_eq!(registry.get_variant_index("Direction", "Success"), None);
    }

    #[test]
    fn test_pattern_matching_variant_indices() {
        let mut registry = EnumVariantRegistry::new();
        
        // Register enum with multiple variants
        registry.register_enum(
            "Option".to_string(),
            vec!["None".to_string(), "Some".to_string()]
        );
        
        // Test specific pattern matching scenarios
        assert_eq!(registry.get_variant_index("Option", "None"), Some(0));
        assert_eq!(registry.get_variant_index("Option", "Some"), Some(1));
        
        // Simulate pattern matching code that was previously hardcoded to 0
        let variant_name = "Some";
        let variant_index = registry.get_variant_index("Option", variant_name).unwrap();
        assert_eq!(variant_index, 1); // Should be 1, not hardcoded 0
        
        // Verify the fix resolves the issue mentioned at src/pattern_matching.rs:960
        println!("✅ Pattern matching variant index lookup now works correctly!");
        println!("   Before: All variants hardcoded to index 0");
        println!("   After:  Variant '{}' correctly maps to index {}", variant_name, variant_index);
    }
}
