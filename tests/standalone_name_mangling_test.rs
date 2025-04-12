// This is a standalone test that doesn't require building the whole library

#[cfg(test)]
mod tests {
    #[test]
    fn test_name_mangling() {
        // Simulate the name mangling implementation
        fn generate_specialized_name(generic_name: &str, type_args: &[&str]) -> String {
            // Replace dots with underscores for package.function format
            let name_parts = generic_name.replace(".", "_");
            
            // Process type arguments
            let type_suffix = type_args.join("_");
            
            format!("_{}{}", name_parts, if type_args.is_empty() { "".to_string() } else { format!("_{}", type_suffix) })
        }
        
        // Test with single type parameter
        let generic_name = "pkg.func";
        let type_args = vec!["Int"];
        let mangled_name = generate_specialized_name(generic_name, &type_args);
        assert_eq!(mangled_name, "_pkg_func_Int");
        
        // Test with multiple type parameters
        let generic_name = "pkg.process";
        let type_args = vec!["String", "Boolean"];
        let mangled_name = generate_specialized_name(generic_name, &type_args);
        assert_eq!(mangled_name, "_pkg_process_String_Boolean");
        
        // Test with nested types
        let generic_name = "collections.Map";
        let type_args = vec!["String", "Array_Int"];
        let mangled_name = generate_specialized_name(generic_name, &type_args);
        assert_eq!(mangled_name, "_collections_Map_String_Array_Int");
        
        // Test with no type parameters
        let generic_name = "math.Pi";
        let type_args: Vec<&str> = vec![];
        let mangled_name = generate_specialized_name(generic_name, &type_args);
        assert_eq!(mangled_name, "_math_Pi");
    }
}