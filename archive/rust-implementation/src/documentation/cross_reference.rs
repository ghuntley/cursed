/// Cross-reference generator for documentation
use crate::error::CursedError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CrossReference {
    pub references: HashMap<String, Vec<String>>,
    pub definitions: HashMap<String, String>,
}

impl CrossReference {
    pub fn new() -> Self {
        Self {
            references: HashMap::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn add_reference(&mut self, item: String, location: String) {
        self.references.entry(item).or_insert_with(Vec::new).push(location);
    }

    pub fn add_definition(&mut self, item: String, definition: String) {
        self.definitions.insert(item, definition);
    }

    pub fn generate_cross_reference(&self) -> Result<String, CursedError> {
        let mut output = String::new();
        output.push_str("# Cross-Reference\n\n");
        
        for (item, definition) in &self.definitions {
            output.push_str(&format!("## {}\n\n", item));
            output.push_str(&format!("{}\n\n", definition));
            
            if let Some(refs) = self.references.get(item) {
                output.push_str("### References\n\n");
                for reference in refs {
                    output.push_str(&format!("- {}\n", reference));
                }
                output.push_str("\n");
            }
        }
        
        Ok(output)
    }
}
