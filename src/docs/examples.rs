//! Example Generation and Validation
//! 
//! Automatically generates and validates code examples for documentation.

use crate::docs::generator::{Example, DocumentationItem};
use crate::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use std::collections::HashMap;

/// Example generator and validator
pub struct ExampleGenerator {
    temp_dir: PathBuf,
    cursed_binary: PathBuf,
    validate_examples: bool,
}

impl ExampleGenerator {
    /// Create new example generator
    pub fn new(cursed_binary: PathBuf) -> Result<Self, Error> {
        let temp_dir = std::env::temp_dir().join("cursed_doc_examples");
        fs::create_dir_all(&temp_dir).map_err(Error::Io)?;
        
        Ok(Self {
            temp_dir,
            cursed_binary,
            validate_examples: true,
        })
    }

    /// Generate examples for documentation items
    pub fn generate_examples(&self, items: &mut [DocumentationItem]) -> Result<(), Error> {
        for item in items {
            match &item.kind {
                crate::docs::generator::ItemKind::Function => {
                    if let Some(example) = self.generate_function_example(item)? {
                        item.examples.push(example);
                    }
                }
                crate::docs::generator::ItemKind::Struct => {
                    if let Some(example) = self.generate_struct_example(item)? {
                        item.examples.push(example);
                    }
                }
                crate::docs::generator::ItemKind::Interface => {
                    if let Some(example) = self.generate_interface_example(item)? {
                        item.examples.push(example);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Generate example for a function
    fn generate_function_example(&self, item: &DocumentationItem) -> Result<Option<Example>, Error> {
        let mut example_code = String::new();
        
        // Import statement
        example_code.push_str(&format!("import \"{}\";\n\n", item.module));
        
        // Function call example
        example_code.push_str("slay main() {\n");
        
        // Generate function call based on parameters
        let call = self.generate_function_call(&item.name, &item.parameters)?;
        
        if let Some(return_type) = &item.return_type {
            if return_type != "()" && return_type != "void" {
                example_code.push_str(&format!("    facts result = {};\n", call));
                example_code.push_str("    println(result)?;\n");
            } else {
                example_code.push_str(&format!("    {};\n", call));
            }
        } else {
            example_code.push_str(&format!("    {};\n", call));
        }
        
        example_code.push_str("}\n");
        
        let example = Example {
            title: Some(format!("Using {}", item.name)),
            description: Some(format!("Basic usage example for the {} function", item.name)),
            code: example_code,
            language: "cursed".to_string(),
            output: None,
        };
        
        // Validate example if enabled
        if self.validate_examples {
            if let Err(e) = self.validate_example(&example) {
                eprintln!("Warning: Generated example for {} failed validation: {}", item.name, e);
                return Ok(None);
            }
        }
        
        Ok(Some(example))
    }

    /// Generate example for a struct
    fn generate_struct_example(&self, item: &DocumentationItem) -> Result<Option<Example>, Error> {
        let mut example_code = String::new();
        
        // Import statement
        example_code.push_str(&format!("import \"{}\";\n\n", item.module));
        
        example_code.push_str("slay main() {\n");
        
        // Generate struct instantiation
        example_code.push_str(&format!("    facts instance = {} {{\n", item.name));
        
        // Add field examples based on parameters (which represent fields)
        for param in &item.parameters {
            let default_value = self.generate_default_value(&param.type_name)?;
            example_code.push_str(&format!("        {}: {},\n", param.name, default_value));
        }
        
        example_code.push_str("    };\n");
        example_code.push_str("    println(instance)?;\n");
        example_code.push_str("}\n");
        
        let example = Example {
            title: Some(format!("Creating {}", item.name)),
            description: Some(format!("Example of creating and using a {} instance", item.name)),
            code: example_code,
            language: "cursed".to_string(),
            output: None,
        };
        
        // Validate example if enabled
        if self.validate_examples {
            if let Err(e) = self.validate_example(&example) {
                eprintln!("Warning: Generated example for {} failed validation: {}", item.name, e);
                return Ok(None);
            }
        }
        
        Ok(Some(example))
    }

    /// Generate example for an interface
    fn generate_interface_example(&self, item: &DocumentationItem) -> Result<Option<Example>, Error> {
        let mut example_code = String::new();
        
        // Import statement
        example_code.push_str(&format!("import \"{}\";\n\n", item.module));
        
        // Generate implementation example
        example_code.push_str(&format!("squad Example{} {{\n", item.name));
        example_code.push_str("    // Implementation fields\n");
        example_code.push_str("}\n\n");
        
        example_code.push_str(&format!("impl {} for Example{} {{\n", item.name, item.name));
        
        // Add method implementations based on parameters (which represent methods)
        for param in &item.parameters {
            if let Some(signature) = &param.default_value {
                // Use the method signature from the default_value field
                example_code.push_str(&format!("    {} {{\n", signature));
                example_code.push_str("        // Implementation\n");
                example_code.push_str("    }\n\n");
            }
        }
        
        example_code.push_str("}\n\n");
        
        example_code.push_str("slay main() {\n");
        example_code.push_str(&format!("    facts instance = Example{}{{}};\n", item.name));
        example_code.push_str("    // Use the interface methods\n");
        example_code.push_str("}\n");
        
        let example = Example {
            title: Some(format!("Implementing {}", item.name)),
            description: Some(format!("Example of implementing the {} interface", item.name)),
            code: example_code,
            language: "cursed".to_string(),
            output: None,
        };
        
        Ok(Some(example))
    }

    /// Generate function call with appropriate parameters
    fn generate_function_call(&self, name: &str, parameters: &[crate::docs::generator::Parameter]) -> Result<String, Error> {
        let mut call = format!("{}(", name);
        
        let mut param_values = Vec::new();
        for param in parameters {
            let value = self.generate_default_value(&param.type_name)?;
            param_values.push(value);
        }
        
        call.push_str(&param_values.join(", "));
        call.push(')');
        
        Ok(call)
    }

    /// Generate default value for a type
    fn generate_default_value(&self, type_name: &Option<String>) -> Result<String, Error> {
        match type_name.as_deref() {
            Some("i32") | Some("i64") | Some("int") => Ok("42".to_string()),
            Some("f32") | Some("f64") | Some("float") => Ok("3.14".to_string()),
            Some("bool") => Ok("true".to_string()),
            Some("string") | Some("String") | Some("&str") => Ok("\"example\"".to_string()),
            Some("char") => Ok("'x'".to_string()),
            Some(t) if t.starts_with("Vec<") => Ok("vec![]".to_string()),
            Some(t) if t.starts_with("HashMap<") => Ok("HashMap::new()".to_string()),
            Some(t) if t.starts_with("Option<") => Ok("None".to_string()),
            Some(t) if t.starts_with("Result<") => Ok("Ok(())".to_string()),
            Some(t) if t.ends_with("[]") => Ok("[]".to_string()),
            _ => Ok("Default::default()".to_string()),
        }
    }

    /// Validate that an example compiles correctly
    fn validate_example(&self, example: &Example) -> Result<(), Error> {
        if !self.validate_examples {
            return Ok(());
        }

        // Create temporary file
        let temp_file = self.temp_dir.join(format!("example_{}.csd", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()));
        
        fs::write(&temp_file, &example.code).map_err(Error::Io)?;
        
        // Try to compile the example
        let output = Command::new(&self.cursed_binary)
            .arg("check")
            .arg(&temp_file)
            .output()
            .map_err(|e| Error::General(format!("Failed to run cursed compiler: {}", e)))?;
        
        // Clean up temp file
        let _ = fs::remove_file(&temp_file);
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::General(format!("Example validation failed: {}", stderr)));
        }
        
        Ok(())
    }

    /// Extract examples from test files
    pub fn extract_examples_from_tests(&self, test_dir: &Path) -> Result<HashMap<String, Vec<Example>>, Error> {
        let mut examples = HashMap::new();
        
        if !test_dir.exists() {
            return Ok(examples);
        }
        
        // Walk through test directory
        for entry in fs::read_dir(test_dir).map_err(Error::Io)? {
            let entry = entry.map_err(Error::Io)?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "csd" || ext == "rs") {
                let test_examples = self.extract_examples_from_file(&path)?;
                for (key, mut test_examples) in test_examples {
                    examples.entry(key).or_insert_with(Vec::new).append(&mut test_examples);
                }
            }
        }
        
        Ok(examples)
    }

    /// Extract examples from a single test file
    fn extract_examples_from_file(&self, file_path: &Path) -> Result<HashMap<String, Vec<Example>>, Error> {
        let content = fs::read_to_string(file_path).map_err(Error::Io)?;
        let mut examples = HashMap::new();
        
        let lines: Vec<&str> = content.split("\n").collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Look for test functions
            if line.starts_with("#[test]") || line.contains("fn test_") {
                if let Some((test_name, example)) = self.extract_test_example(&lines, i)? {
                    examples.entry(test_name).or_insert_with(Vec::new).push(example);
                }
            }
            i += 1;
        }
        
        Ok(examples)
    }

    /// Extract example from a test function
    fn extract_test_example(&self, lines: &[&str], start: usize) -> Result<Option<(String, Example)>, Error> {
        // Find the test function
        let mut func_start = start;
        while func_start < lines.len() && !lines[func_start].contains("fn ") {
            func_start += 1;
        }
        
        if func_start >= lines.len() {
            return Ok(None);
        }
        
        // Extract function name
        let func_line = lines[func_start];
        let test_name = if let Some(fn_pos) = func_line.find("fn ") {
            let after_fn = &func_line[fn_pos + 3..];
            if let Some(paren_pos) = after_fn.find('(') {
                after_fn[..paren_pos].trim().to_string()
            } else {
                "unknown_test".to_string()
            }
        } else {
            "unknown_test".to_string()
        };
        
        // Extract function body
        let mut body_lines = Vec::new();
        let mut brace_count = 0;
        let mut in_body = false;
        
        for i in func_start..lines.len() {
            let line = lines[i];
            
            if line.contains('{') {
                brace_count += line.matches('{').count();
                in_body = true;
            }
            
            if in_body {
                body_lines.push(line.to_string());
            }
            
            if line.contains('}') {
                brace_count -= line.matches('}').count();
                if brace_count == 0 && in_body {
                    break;
                }
            }
        }
        
        if body_lines.is_empty() {
            return Ok(None);
        }
        
        // Clean up the example code
        let code = self.clean_test_code(body_lines.join("\n"))?;
        
        let example = Example {
            title: Some(format!("Test: {}", test_name)),
            description: Some(format!("Example extracted from test function {}", test_name)),
            code,
            language: "cursed".to_string(),
            output: None,
        };
        
        Ok(Some((test_name, example)))
    }

    /// Clean test code to make it suitable for documentation
    fn clean_test_code(&self, code: String) -> Result<String, Error> {
        let mut cleaned = code;
        
        // Remove test-specific annotations
        cleaned = cleaned.replace("#[test]", "");
        cleaned = cleaned.replace("assert_eq!", "// assert_eq!");
        cleaned = cleaned.replace("assert!", "// assert!");
        cleaned = cleaned.replace("panic!", "// panic!");
        
        // Add main function wrapper if needed
        if !cleaned.contains("slay main") && !cleaned.contains("fn main") {
            cleaned = format!("slay main() {{\n{}\n}}", cleaned);
        }
        
        // Clean up whitespace
        let lines: Vec<String> = cleaned.split("\n")
            .map(|line| line.trim_start())
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.to_string())
            .collect();
        
        Ok(lines.join("\n"))
    }

    /// Generate comprehensive example collection
    pub fn generate_example_collection(&self, items: &[DocumentationItem], test_dir: Option<&Path>) -> Result<Vec<Example>, Error> {
        let mut all_examples = Vec::new();
        
        // Generate examples for each documented item
        for item in items {
            all_examples.extend(item.examples.clone());
        }
        
        // Extract examples from tests if test directory is provided
        if let Some(test_dir) = test_dir {
            let test_examples = self.extract_examples_from_tests(test_dir)?;
            for (_, examples) in test_examples {
                all_examples.extend(examples);
            }
        }
        
        // Generate additional comprehensive examples
        all_examples.extend(self.generate_comprehensive_examples()?);
        
        Ok(all_examples)
    }

    /// Generate comprehensive examples showcasing language features
    fn generate_comprehensive_examples(&self) -> Result<Vec<Example>, Error> {
        let mut examples = Vec::new();
        
        // Basic syntax example
        examples.push(Example {
            title: Some("Basic CURSED Syntax".to_string()),
            description: Some("Overview of basic CURSED language syntax and features".to_string()),
            code: r#"// CURSED Hello World
slay main() {
    println("Hello, CURSED!")?;
    
    // Variables
    sus x = 42;
    facts name = "CURSED";
    
    // Control flow
    lowkey (x > 0) {
        println("Positive number")?;
    } highkey {
        println("Non-positive number")?;
    }
    
    // Loops
    lowkey (sus i = 0; i < 5; i++) {
        println("Iteration: {}", i)?;
    }
    
    // Functions
    facts result = calculate(10, 20);
    println("Result: {}", result)?;
}

slay calculate(a: i32, b: i32) -> i32 {
    a + b
}"#.to_string(),
            language: "cursed".to_string(),
            output: Some("Hello, CURSED!\nPositive number\nIteration: 0\nIteration: 1\nIteration: 2\nIteration: 3\nIteration: 4\nResult: 30".to_string()),
        });
        
        // Error handling example
        examples.push(Example {
            title: Some("Error Handling".to_string()),
            description: Some("Demonstrating CURSED error handling with the ? operator".to_string()),
            code: r#"import "stdlib::io";

slay risky_operation() -> Result<i32, string> {
    // Simulate an operation that might fail
    Ok(42)
}

slay main() {
    // Using the ? operator for error propagation
    facts result = risky_operation()?;
    println("Success: {}", result)?;
    
    // Error handling with pattern matching
    bestie risky_operation() {
        Ok(value) => println("Got value: {}", value)?,
        Err(error) => println("Error: {}", error)?,
    }
}"#.to_string(),
            language: "cursed".to_string(),
            output: Some("Success: 42\nGot value: 42".to_string()),
        });
        
        // Struct and interface example
        examples.push(Example {
            title: Some("Structs and Interfaces".to_string()),
            description: Some("Defining and using structs and interfaces in CURSED".to_string()),
            code: r#"// Define a struct
squad Person {
    name: string,
    age: i32,
}

// Define an interface
collab Greetable {
    slay greet() -> string;
}

// Implement interface for struct
impl Greetable for Person {
    slay greet() -> string {
        format!("Hello, I'm {}", self.name)
    }
}

slay main() {
    facts person = Person {
        name: "Alice",
        age: 30,
    };
    
    facts greeting = person.greet();
    println(greeting)?;
}"#.to_string(),
            language: "cursed".to_string(),
            output: Some("Hello, I'm Alice".to_string()),
        });
        
        Ok(examples)
    }

    /// Set validation option
    pub fn set_validation(&mut self, validate: bool) {
        self.validate_examples = validate;
    }
}

impl Drop for ExampleGenerator {
    fn drop(&mut self) {
        // Clean up temporary directory
        let _ = fs::remove_dir_all(&self.temp_dir);
    }
}
