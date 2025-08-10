//! API Extractor for CURSED Documentation
//! 
//! Extracts API information from CURSED AST nodes and source code.
//! Handles functions, variables, constants, types, and modules.

use std::collections::HashMap;
use crate::error::CursedError;
use crate::documentation::{
    DocumentedFunction, DocumentedVariable, DocumentedConstant, DocumentedType, DocumentedModule,
    Parameter, TypeField
};
use crate::documentation::comment_parser::{CommentParser, DocumentationComment};

/// API extractor for CURSED source code
pub struct ApiExtractor {
    comment_parser: CommentParser,
}

impl ApiExtractor {
    /// Create a new API extractor
    pub fn new() -> Result<Self, CursedError> {
        let comment_parser = CommentParser::new()?;
        
        Ok(Self {
            comment_parser,
        })
    }

    /// Extract complete API documentation from source code
    pub fn extract_api(&self, source: &str, file_path: &str) -> Result<DocumentedModule, CursedError> {
        let comments = self.comment_parser.parse_comments(source)?;
        let module_docs = self.comment_parser.parse_module_docs(source)?;
        
        let mut module = DocumentedModule {
            name: self.extract_module_name(file_path),
            description: module_docs.description,
            functions: Vec::new(),
            variables: Vec::new(),
            constants: Vec::new(),
            types: Vec::new(),
            examples: module_docs.examples,
            source_file: file_path.to_string(),
            submodules: Vec::new(),
        };

        // Extract functions
        let functions = self.extract_functions(source, &comments)?;
        module.functions = functions;

        // Extract variables
        let variables = self.extract_variables(source, &comments)?;
        module.variables = variables;

        // Extract constants
        let constants = self.extract_constants(source, &comments)?;
        module.constants = constants;

        // Extract types
        let types = self.extract_types(source, &comments)?;
        module.types = types;

        Ok(module)
    }

    /// Extract module name from file path
    fn extract_module_name(&self, file_path: &str) -> String {
        std::path::Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    /// Extract function definitions and documentation
    fn extract_functions(&self, source: &str, comments: &[DocumentationComment]) -> Result<Vec<DocumentedFunction>, CursedError> {
        let mut functions = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        // Function pattern: slay function_name(params) return_type { ... }
        let function_regex = regex::Regex::new(r"^\s*slay\s+(\w+)\s*\((.*?)\)(?:\s*(\w+))?\s*\{").unwrap();

        for (line_num, line) in lines.iter().enumerate() {
            if let Some(captures) = function_regex.captures(line) {
                let function_name = captures.get(1).unwrap().as_str().to_string();
                let params_str = captures.get(2).map_or("", |m| m.as_str());
                let return_type = captures.get(3).map_or("", |m| m.as_str());

                // Find associated documentation
                let doc_comment = self.find_preceding_comment(comments, line_num);
                
                let mut function = DocumentedFunction {
                    name: function_name.clone(),
                    signature: self.build_function_signature(&function_name, params_str, return_type),
                    description: doc_comment.as_ref().map_or(String::new(), |c| c.description.clone()),
                    parameters: doc_comment.as_ref().map_or(Vec::new(), |c| c.parameters.clone()),
                    return_type: return_type.to_string(),
                    return_description: doc_comment.as_ref().map_or(String::new(), |c| c.return_description.clone()),
                    examples: doc_comment.as_ref().map_or(Vec::new(), |c| c.examples.clone()),
                    source_file: String::new(), // Will be set by caller
                    source_line: line_num + 1,
                    visibility: self.determine_visibility(&function_name),
                };

                // Parse parameters from signature if not in documentation
                if function.parameters.is_empty() && !params_str.is_empty() {
                    function.parameters = self.parse_function_parameters(params_str)?;
                }

                // Extract return type from signature if not documented
                if function.return_type.is_empty() && !return_type.is_empty() {
                    function.return_type = return_type.to_string();
                }

                functions.push(function);
            }
        }

        Ok(functions)
    }

    /// Extract variable definitions and documentation
    fn extract_variables(&self, source: &str, comments: &[DocumentationComment]) -> Result<Vec<DocumentedVariable>, CursedError> {
        let mut variables = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        // Variable pattern: sus variable_name type = value
        let variable_regex = regex::Regex::new(r"^\s*sus\s+(\w+)\s+(\w+)\s*=").unwrap();

        for (line_num, line) in lines.iter().enumerate() {
            if let Some(captures) = variable_regex.captures(line) {
                let variable_name = captures.get(1).unwrap().as_str().to_string();
                let variable_type = captures.get(2).unwrap().as_str().to_string();

                // Find associated documentation
                let doc_comment = self.find_preceding_comment(comments, line_num);

                let variable = DocumentedVariable {
                    name: variable_name,
                    var_type: variable_type,
                    description: doc_comment.as_ref().map_or(String::new(), |c| c.description.clone()),
                    source_file: String::new(), // Will be set by caller
                    source_line: line_num + 1,
                };

                variables.push(variable);
            }
        }

        Ok(variables)
    }

    /// Extract constant definitions and documentation
    fn extract_constants(&self, source: &str, comments: &[DocumentationComment]) -> Result<Vec<DocumentedConstant>, CursedError> {
        let mut constants = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        // Constant pattern: Look for variables that are never reassigned (simple heuristic)
        let constant_regex = regex::Regex::new(r"^\s*sus\s+([A-Z_][A-Z0-9_]*)\s+(\w+)\s*=\s*(.+)").unwrap();

        for (line_num, line) in lines.iter().enumerate() {
            if let Some(captures) = constant_regex.captures(line) {
                let constant_name = captures.get(1).unwrap().as_str().to_string();
                let constant_type = captures.get(2).unwrap().as_str().to_string();
                let constant_value = captures.get(3).unwrap().as_str().trim_end_matches(';').to_string();

                // Find associated documentation
                let doc_comment = self.find_preceding_comment(comments, line_num);

                let constant = DocumentedConstant {
                    name: constant_name,
                    const_type: constant_type,
                    value: constant_value,
                    description: doc_comment.as_ref().map_or(String::new(), |c| c.description.clone()),
                    source_file: String::new(), // Will be set by caller
                    source_line: line_num + 1,
                };

                constants.push(constant);
            }
        }

        Ok(constants)
    }

    /// Extract type definitions and documentation
    fn extract_types(&self, source: &str, comments: &[DocumentationComment]) -> Result<Vec<DocumentedType>, CursedError> {
        let mut types = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        // Type pattern: Look for struct-like definitions (simplified)
        let type_regex = regex::Regex::new(r"^\s*type\s+(\w+)\s*\{").unwrap();

        for (line_num, line) in lines.iter().enumerate() {
            if let Some(captures) = type_regex.captures(line) {
                let type_name = captures.get(1).unwrap().as_str().to_string();

                // Find associated documentation
                let doc_comment = self.find_preceding_comment(comments, line_num);

                // Extract fields (simplified - would need more sophisticated parsing)
                let fields = self.extract_type_fields(&lines, line_num)?;

                let doc_type = DocumentedType {
                    name: type_name,
                    type_kind: "struct".to_string(), // Simplified
                    description: doc_comment.as_ref().map_or(String::new(), |c| c.description.clone()),
                    fields,
                    methods: Vec::new(), // Would need method extraction
                    source_file: String::new(), // Will be set by caller
                    source_line: line_num + 1,
                };

                types.push(doc_type);
            }
        }

        Ok(types)
    }

    /// Find the documentation comment that precedes a given line
    fn find_preceding_comment<'a>(&self, comments: &'a [DocumentationComment], line_num: usize) -> Option<&'a DocumentationComment> {
        comments
            .iter()
            .filter(|comment| comment.line_number < line_num && comment.line_number > line_num.saturating_sub(6))
            .max_by_key(|comment| comment.line_number)
    }

    /// Build function signature string
    fn build_function_signature(&self, name: &str, params: &str, return_type: &str) -> String {
        if return_type.is_empty() {
            format!("slay {}({})", name, params)
        } else {
            format!("slay {}({}) {}", name, params, return_type)
        }
    }

    /// Parse function parameters from parameter string
    fn parse_function_parameters(&self, params_str: &str) -> Result<Vec<Parameter>, CursedError> {
        let mut parameters = Vec::new();
        
        if params_str.trim().is_empty() {
            return Ok(parameters);
        }

        // Split parameters by comma
        for param in params_str.split(',') {
            let param = param.trim();
            
            // Parse parameter format: name type
            let parts: Vec<&str> = param.split_whitespace().collect();
            if parts.len() >= 2 {
                let param_name = parts[0].to_string();
                let param_type = parts[1].to_string();
                
                parameters.push(Parameter {
                    name: param_name,
                    param_type,
                    description: String::new(), // Would be filled from documentation
                    default_value: None,
                });
            }
        }

        Ok(parameters)
    }

    /// Determine function visibility based on naming conventions
    fn determine_visibility(&self, function_name: &str) -> String {
        if function_name.starts_with('_') {
            "private".to_string()
        } else {
            "public".to_string()
        }
    }

    /// Extract type fields from type definition
    fn extract_type_fields(&self, lines: &[&str], start_line: usize) -> Result<Vec<TypeField>, CursedError> {
        let mut fields = Vec::new();
        let mut brace_count = 0;
        let mut found_opening_brace = false;

        for (i, line) in lines.iter().enumerate().skip(start_line) {
            for ch in line.chars() {
                if ch == '{' {
                    brace_count += 1;
                    found_opening_brace = true;
                } else if ch == '}' {
                    brace_count -= 1;
                    if brace_count == 0 && found_opening_brace {
                        return Ok(fields);
                    }
                }
            }

            // Parse field if we're inside the type definition
            if found_opening_brace && brace_count > 0 {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('{') && !trimmed.starts_with('}') {
                    if let Some(field) = self.parse_field_line(trimmed)? {
                        fields.push(field);
                    }
                }
            }
        }

        Ok(fields)
    }

    /// Parse a field line from type definition
    fn parse_field_line(&self, line: &str) -> Result<Option<TypeField>, CursedError> {
        // Field pattern: field_name field_type
        let field_regex = regex::Regex::new(r"^\s*(\w+)\s+(\w+)").unwrap();
        
        if let Some(captures) = field_regex.captures(line) {
            let field_name = captures.get(1).unwrap().as_str().to_string();
            let field_type = captures.get(2).unwrap().as_str().to_string();
            
            Ok(Some(TypeField {
                name: field_name,
                field_type,
                description: String::new(), // Would be extracted from comments
                default_value: None,
            }))
        } else {
            Ok(None)
        }
    }

    /// Extract imports and dependencies
    pub fn extract_dependencies(&self, source: &str) -> Result<Vec<String>, CursedError> {
        let mut dependencies = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        // Import pattern: yeet "module_name"
        let import_regex = regex::Regex::new(r#"^\s*yeet\s+"([^"]+)""#).unwrap();

        for line in lines {
            if let Some(captures) = import_regex.captures(line) {
                let module_name = captures.get(1).unwrap().as_str().to_string();
                dependencies.push(module_name);
            }
        }

        Ok(dependencies)
    }

    /// Extract package information
    pub fn extract_package_info(&self, source: &str) -> Result<PackageInfo, CursedError> {
        let lines: Vec<&str> = source.lines().collect();
        
        let mut package_info = PackageInfo {
            name: String::new(),
            version: String::new(),
            description: String::new(),
            dependencies: Vec::new(),
        };

        // Package pattern: vibe package_name
        let package_regex = regex::Regex::new(r"^\s*vibe\s+(\w+)").unwrap();

        for line in lines {
            if let Some(captures) = package_regex.captures(line) {
                package_info.name = captures.get(1).unwrap().as_str().to_string();
                break;
            }
        }

        // Extract dependencies
        package_info.dependencies = self.extract_dependencies(source)?;

        Ok(package_info)
    }

    /// Generate API summary
    pub fn generate_api_summary(&self, module: &DocumentedModule) -> ApiSummary {
        ApiSummary {
            module_name: module.name.clone(),
            total_functions: module.functions.len(),
            public_functions: module.functions.iter().filter(|f| f.visibility == "public").count(),
            private_functions: module.functions.iter().filter(|f| f.visibility == "private").count(),
            total_variables: module.variables.len(),
            total_constants: module.constants.len(),
            total_types: module.types.len(),
            documented_functions: module.functions.iter().filter(|f| !f.description.is_empty()).count(),
            documentation_coverage: if module.functions.is_empty() {
                0.0
            } else {
                (module.functions.iter().filter(|f| !f.description.is_empty()).count() as f64 / module.functions.len() as f64) * 100.0
            },
        }
    }
}

/// Package information
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
}

/// API summary statistics
#[derive(Debug, Clone)]
pub struct ApiSummary {
    pub module_name: String,
    pub total_functions: usize,
    pub public_functions: usize,
    pub private_functions: usize,
    pub total_variables: usize,
    pub total_constants: usize,
    pub total_types: usize,
    pub documented_functions: usize,
    pub documentation_coverage: f64,
}

impl Default for ApiExtractor {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_extraction() {
        let extractor = ApiExtractor::new().unwrap();
        let source = r#"
fr fr Adds two numbers
fr fr @param a (normie) First number
fr fr @param b (normie) Second number
fr fr @return (normie) Sum of the numbers
slay add(a normie, b normie) normie {
    damn a + b
}

slay undocumented_function() {
    damn 42
}
"#;

        let comments = extractor.comment_parser.parse_comments(source).unwrap();
        let functions = extractor.extract_functions(source, &comments).unwrap();
        
        assert_eq!(functions.len(), 2);
        assert_eq!(functions[0].name, "add");
        assert!(!functions[0].description.is_empty());
        assert_eq!(functions[0].parameters.len(), 2);
        assert_eq!(functions[1].name, "undocumented_function");
        assert!(functions[1].description.is_empty());
    }

    #[test]
    fn test_variable_extraction() {
        let extractor = ApiExtractor::new().unwrap();
        let source = r#"
fr fr A magic number
sus MAGIC_NUMBER normie = 42

sus regular_var tea = "hello"
"#;

        let comments = extractor.comment_parser.parse_comments(source).unwrap();
        let variables = extractor.extract_variables(source, &comments).unwrap();
        
        assert_eq!(variables.len(), 2);
        assert_eq!(variables[0].name, "MAGIC_NUMBER");
        assert_eq!(variables[0].var_type, "normie");
    }

    #[test]
    fn test_dependency_extraction() {
        let extractor = ApiExtractor::new().unwrap();
        let source = r#"
yeet "testz"
yeet "math"
yeet "string"
"#;

        let dependencies = extractor.extract_dependencies(source).unwrap();
        assert_eq!(dependencies.len(), 3);
        assert!(dependencies.contains(&"testz".to_string()));
        assert!(dependencies.contains(&"math".to_string()));
        assert!(dependencies.contains(&"string".to_string()));
    }

    #[test]
    fn test_api_summary_generation() {
        let extractor = ApiExtractor::new().unwrap();
        let module = DocumentedModule {
            name: "test_module".to_string(),
            description: "Test module".to_string(),
            functions: vec![
                DocumentedFunction {
                    name: "func1".to_string(),
                    signature: "slay func1()".to_string(),
                    description: "Documented function".to_string(),
                    parameters: Vec::new(),
                    return_type: String::new(),
                    return_description: String::new(),
                    examples: Vec::new(),
                    source_file: "test.csd".to_string(),
                    source_line: 1,
                    visibility: "public".to_string(),
                },
                DocumentedFunction {
                    name: "func2".to_string(),
                    signature: "slay func2()".to_string(),
                    description: String::new(), // Undocumented
                    parameters: Vec::new(),
                    return_type: String::new(),
                    return_description: String::new(),
                    examples: Vec::new(),
                    source_file: "test.csd".to_string(),
                    source_line: 5,
                    visibility: "public".to_string(),
                }
            ],
            variables: Vec::new(),
            constants: Vec::new(),
            types: Vec::new(),
            examples: Vec::new(),
            source_file: "test.csd".to_string(),
            submodules: Vec::new(),
        };

        let summary = extractor.generate_api_summary(&module);
        assert_eq!(summary.total_functions, 2);
        assert_eq!(summary.documented_functions, 1);
        assert_eq!(summary.documentation_coverage, 50.0);
    }
}
