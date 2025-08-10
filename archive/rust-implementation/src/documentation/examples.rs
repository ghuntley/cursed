/// Examples generator for documentation
use crate::error::CursedError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExampleGenerator {
    pub examples: HashMap<String, String>,
    pub categories: HashMap<String, Vec<String>>,
}

impl ExampleGenerator {
    pub fn new() -> Self {
        Self {
            examples: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    pub fn add_example(&mut self, name: String, code: String, category: String) {
        self.examples.insert(name.clone(), code);
        self.categories.entry(category).or_insert_with(Vec::new).push(name);
    }

    pub fn generate_examples(&self) -> Result<String, CursedError> {
        let mut output = String::new();
        output.push_str("# Examples\n\n");
        
        for (category, examples) in &self.categories {
            output.push_str(&format!("## {}\n\n", category));
            
            for example in examples {
                if let Some(code) = self.examples.get(example) {
                    output.push_str(&format!("### {}\n\n", example));
                    output.push_str("```cursed\n");
                    output.push_str(code);
                    output.push_str("\n```\n\n");
                }
            }
        }
        
        Ok(output)
    }

    pub fn get_example(&self, name: &str) -> Option<&String> {
        self.examples.get(name)
    }
}
