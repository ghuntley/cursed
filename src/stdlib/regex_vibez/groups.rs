/// VibeGroups - Named capture group functionality
use super::pattern::VibePattern;
use std::collections::HashMap;

/// VibeGroups provides access to named capture group information
/// This provides enhanced functionality for working with named groups in regex patterns
#[derive(Debug, Clone)]
pub struct VibeGroups {
    pattern: VibePattern,
}

impl VibeGroups {
    /// Create a new VibeGroups instance from a VibePattern
    pub fn new(pattern: VibePattern) -> Self {
        Self { pattern }
    }

    /// Get all group names from the pattern
    pub fn group_names(&self) -> Vec<String> {
        self.pattern.group_names()
    }

    /// Get a map of group names to their capture group indexes
    pub fn named_groups(&self) -> HashMap<String, i32> {
        self.pattern.named_groups()
    }

    /// Find all named groups and their values in the given string
    pub fn find_groups_string(&self, s: &str) -> HashMap<String, String> {
        self.pattern.find_groups_string(s)
    }

    /// Check if a group name exists in the pattern
    pub fn has_group(&self, name: &str) -> bool {
        self.named_groups().contains_key(name)
    }

    /// Get the index of a named group, returns -1 if not found
    pub fn group_index(&self, name: &str) -> i32 {
        self.named_groups().get(name).copied().unwrap_or(-1)
    }

    /// Get all group names that have matches in the given string
    pub fn matched_groups(&self, s: &str) -> Vec<String> {
        let groups = self.find_groups_string(s);
        groups.into_iter()
            .filter(|(_, value)| !value.is_empty())
            .map(|(name, _)| name)
            .collect()
    }

    /// Get the value of a specific named group, returns empty string if not found
    pub fn get_group_value(&self, s: &str, name: &str) -> String {
        self.find_groups_string(s)
            .get(name)
            .cloned()
            .unwrap_or_default()
    }

    /// Check if a named group has a match in the given string
    pub fn group_has_match(&self, s: &str, name: &str) -> bool {
        let groups = self.find_groups_string(s);
        groups.get(name).map(|v| !v.is_empty()).unwrap_or(false)
    }

    /// Get all groups with their values, including empty matches
    pub fn all_groups_with_values(&self, s: &str) -> HashMap<String, Option<String>> {
        let groups = self.find_groups_string(s);
        let mut result = HashMap::new();
        
        for name in self.group_names() {
            if !name.is_empty() {
                let value = groups.get(&name).cloned();
                result.insert(name, value);
            }
        }
        
        result
    }

    /// Extract groups from multiple matches in the string
    pub fn find_all_groups_string(&self, s: &str, n: i32) -> Vec<HashMap<String, String>> {
        let mut results = Vec::new();
        let submatch_all = self.pattern.find_all_string_submatch(s, n);
        let group_names = self.group_names();
        
        for submatch in submatch_all {
            let mut group_map = HashMap::new();
            
            // Match submatches with group names
            for (i, name) in group_names.iter().enumerate() {
                if !name.is_empty() && i < submatch.len() {
                    group_map.insert(name.clone(), submatch[i].clone());
                }
            }
            
            results.push(group_map);
        }
        
        results
    }

    /// Get statistics about the groups in this pattern
    pub fn group_statistics(&self) -> GroupStatistics {
        let names = self.group_names();
        let named_count = names.iter().filter(|n| !n.is_empty()).count();
        let total_count = names.len();
        
        GroupStatistics {
            total_groups: total_count as i32,
            named_groups: named_count as i32,
            unnamed_groups: (total_count - named_count) as i32,
            group_names: names.into_iter().filter(|n| !n.is_empty()).collect(),
        }
    }

    /// Validate that all expected group names exist in the pattern
    pub fn validate_groups(&self, expected_groups: &[&str]) -> GroupValidationResult {
        let available_groups: Vec<String> = self.group_names()
            .into_iter()
            .filter(|n| !n.is_empty())
            .collect();
        
        let mut missing = Vec::new();
        let mut present = Vec::new();
        
        for &expected in expected_groups {
            if available_groups.contains(&expected.to_string()) {
                present.push(expected.to_string());
            } else {
                missing.push(expected.to_string());
            }
        }
        
        GroupValidationResult {
            is_valid: missing.is_empty(),
            missing_groups: missing,
            present_groups: present,
            available_groups,
        }
    }
}

/// Statistics about groups in a regex pattern
#[derive(Debug, Clone)]
pub struct GroupStatistics {
    pub total_groups: i32,
    pub named_groups: i32,
    pub unnamed_groups: i32,
    pub group_names: Vec<String>,
}

/// Result of group validation
#[derive(Debug, Clone)]
pub struct GroupValidationResult {
    pub is_valid: bool,
    pub missing_groups: Vec<String>,
    pub present_groups: Vec<String>,
    pub available_groups: Vec<String>,
}

