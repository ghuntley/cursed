//! Path utilities for interface inheritance paths
//!
//! This module provides extension traits and utilities for working with
//! interface inheritance paths represented as vectors of strings.

/// Extension trait to provide string representation for Vec<String> paths
pub trait PathStringRepresentation {
    /// Convert a path represented as Vec<String> to a string representation
    fn to_string_representation(&self) -> String;
}

impl PathStringRepresentation for Vec<String> {
    fn to_string_representation(&self) -> String {
        if self.is_empty() {
            "Empty path".to_string()
        } else {
            self.join(" -> ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_string_representation() {
        let path = vec!["Child".to_string(), "Parent".to_string(), "GrandParent".to_string()];
        assert_eq!(path.to_string_representation(), "Child -> Parent -> GrandParent");
        
        let empty_path: Vec<String> = vec![];
        assert_eq!(empty_path.to_string_representation(), "Empty path");
    }
}
