use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use serde::{Serialize, Deserialize};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::*;

/// Documentation generator for CURSED source code
#[derive(Debug, Clone)]
pub struct DocGenerator {
    pub output_dir: PathBuf,
    pub template_dir: PathBuf,
    pub config: DocConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocConfig {
    pub title: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub theme: String,
    pub include_private: bool,
    pub include_tests: bool,
    pub include_examples: bool,
    pub output_formats: Vec<String>,
    pub custom_css: Option<String>,
    pub logo_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Documentation {
    pub modules: Vec<ModuleDoc>,
    pub functions: Vec<FunctionDoc>,
    pub types: Vec<TypeDoc>,
    pub constants: Vec<ConstantDoc>,
    pub examples: Vec<ExampleDoc>,
    pub config: DocConfig,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModuleDoc {
    pub name: String,
    pub description: String,
    pub file_path: String,
    pub functions: Vec<FunctionDoc>,
    pub types: Vec<TypeDoc>,
    pub constants: Vec<ConstantDoc>,
    pub examples: Vec<String>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionDoc {
    pub name: String,
    pub description: String,
    pub signature: String,
    pub parameters: Vec<ParameterDoc>,
    pub return_type: String,
    pub examples: Vec<String>,
    pub visibility: String,
    pub file_path: String,
    pub line_number: usize,
    pub complexity: Option<String>,
    pub performance_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParameterDoc {
    pub name: String,
    pub param_type: String,
    pub description: String,
    pub optional: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TypeDoc {
    pub name: String,
    pub description: String,
    pub type_kind: String, // struct, interface, enum, etc.
    pub fields: Vec<FieldDoc>,
    pub methods: Vec<FunctionDoc>,
    pub examples: Vec<String>,
    pub file_path: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldDoc {
    pub name: String,
    pub field_type: String,
    pub description: String,
    pub visibility: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstantDoc {
    pub name: String,
    pub value: String,
    pub const_type: String,
    pub description: String,
    pub file_path: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExampleDoc {
    pub title: String,
    pub description: String,
    pub code: String,
    pub expected_output: Option<String>,
    pub file_path: String,
}

impl DocGenerator {
    /// Create new documentation generator
    pub fn new(output_dir: PathBuf, config: DocConfig) -> Self {
        let template_dir = output_dir.join("templates");
        
        Self {
            output_dir,
            template_dir,
            config,
        }
    }

    /// Generate documentation for entire project
    pub fn generate_docs(&self, source_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("📚 Generating documentation for CURSED project...");

        // Create output directory
        fs::create_dir_all(&self.output_dir)?;
        fs::create_dir_all(&self.template_dir)?;

        // Parse all source files
        let documentation = self.parse_project(source_dir)?;

        // Generate different output formats
        for format in &self.config.output_formats {
            match format.as_str() {
                "html" => self.generate_html(&documentation)?,
                "markdown" => self.generate_markdown(&documentation)?,
                "json" => self.generate_json(&documentation)?,
                "pdf" => self.generate_pdf(&documentation)?,
                _ => eprintln!("⚠️  Unknown output format: {}", format),
            }
        }

        // Copy assets
        self.copy_assets()?;

        println!("✅ Documentation generated successfully in {}", self.output_dir.display());
        Ok(())
    }

    /// Parse entire project and extract documentation
    fn parse_project(&self, source_dir: &Path) -> Result<Documentation, Box<dyn std::error::Error>> {
        let mut modules = Vec::new();
        let mut all_functions = Vec::new();
        let mut all_types = Vec::new();
        let mut all_constants = Vec::new();
        let mut examples = Vec::new();

        // Find all .csd files
        let cursed_files = self.find_cursed_files(source_dir)?;

        for file_path in cursed_files {
            let module_doc = self.parse_file(&file_path)?;
            
            // Collect all items
            all_functions.extend(module_doc.functions.clone());
            all_types.extend(module_doc.types.clone());
            all_constants.extend(module_doc.constants.clone());
            
            modules.push(module_doc);
        }

        // Find example files
        let example_dir = source_dir.join("examples");
        if example_dir.exists() {
            examples = self.parse_examples(&example_dir)?;
        }

        Ok(Documentation {
            modules,
            functions: all_functions,
            types: all_types,
            constants: all_constants,
            examples,
            config: self.config.clone(),
        })
    }

    /// Parse single CURSED file
    fn parse_file(&self, file_path: &Path) -> Result<ModuleDoc, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut lexer = Lexer::new(&content);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        let mut module_doc = ModuleDoc {
            name: file_path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            description: self.extract_module_description(&content),
            file_path: file_path.to_string_lossy().to_string(),
            functions: Vec::new(),
            types: Vec::new(),
            constants: Vec::new(),
            examples: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
        };

        // Extract documentation comments
        let doc_comments = self.extract_doc_comments(&content);

        // Parse AST nodes
        for node in ast.nodes {
            match node {
                AstNode::FunctionDeclaration(func) => {
                    let func_doc = self.parse_function_doc(&func, &doc_comments, file_path)?;
                    module_doc.functions.push(func_doc);
                }
                AstNode::TypeDeclaration(type_decl) => {
                    let type_doc = self.parse_type_doc(&type_decl, &doc_comments, file_path)?;
                    module_doc.types.push(type_doc);
                }
                AstNode::ConstantDeclaration(const_decl) => {
                    let const_doc = self.parse_constant_doc(&const_decl, &doc_comments, file_path)?;
                    module_doc.constants.push(const_doc);
                }
                AstNode::ImportDeclaration(import) => {
                    module_doc.imports.push(format!("{:?}", import));
                }
                AstNode::ExportDeclaration(export) => {
                    module_doc.exports.push(format!("{:?}", export));
                }
                _ => {}
            }
        }

        Ok(module_doc)
    }

    /// Extract documentation comments from source code
    fn extract_doc_comments(&self, content: &str) -> HashMap<usize, String> {
        let mut doc_comments = HashMap::new();
        let mut current_comment = String::new();
        let mut line_num = 1;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("##") {
                // Documentation comment
                current_comment.push_str(&trimmed[2..].trim());
                current_comment.push('\n');
            } else if !trimmed.is_empty() && !current_comment.is_empty() {
                // Non-empty line after doc comment
                doc_comments.insert(line_num, current_comment.trim().to_string());
                current_comment.clear();
            }
            line_num += 1;
        }

        doc_comments
    }

    /// Extract module description from file header
    fn extract_module_description(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut description = String::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("##") {
                description.push_str(&trimmed[2..].trim());
                description.push('\n');
            } else if !trimmed.is_empty() {
                break;
            }
        }

        description.trim().to_string()
    }

    /// Parse function documentation
    fn parse_function_doc(&self, func: &FunctionDeclaration, doc_comments: &HashMap<usize, String>, file_path: &Path) -> Result<FunctionDoc, Box<dyn std::error::Error>> {
        let description = doc_comments.get(&func.line_number).cloned().unwrap_or_default();
        
        let mut parameters = Vec::new();
        for param in &func.parameters {
            parameters.push(ParameterDoc {
                name: param.name.clone(),
                param_type: format!("{:?}", param.param_type),
                description: String::new(), // Extract from doc comment
                optional: param.optional,
                default_value: param.default_value.clone(),
            });
        }

        Ok(FunctionDoc {
            name: func.name.clone(),
            description,
            signature: self.generate_function_signature(func),
            parameters,
            return_type: format!("{:?}", func.return_type),
            examples: Vec::new(),
            visibility: func.visibility.clone(),
            file_path: file_path.to_string_lossy().to_string(),
            line_number: func.line_number,
            complexity: self.calculate_complexity(func),
            performance_notes: Vec::new(),
        })
    }

    /// Parse type documentation
    fn parse_type_doc(&self, type_decl: &TypeDeclaration, doc_comments: &HashMap<usize, String>, file_path: &Path) -> Result<TypeDoc, Box<dyn std::error::Error>> {
        let description = doc_comments.get(&type_decl.line_number).cloned().unwrap_or_default();
        
        let mut fields = Vec::new();
        for field in &type_decl.fields {
            fields.push(FieldDoc {
                name: field.name.clone(),
                field_type: format!("{:?}", field.field_type),
                description: String::new(),
                visibility: field.visibility.clone(),
                optional: field.optional,
            });
        }

        Ok(TypeDoc {
            name: type_decl.name.clone(),
            description,
            type_kind: type_decl.kind.clone(),
            fields,
            methods: Vec::new(),
            examples: Vec::new(),
            file_path: file_path.to_string_lossy().to_string(),
            line_number: type_decl.line_number,
        })
    }

    /// Parse constant documentation
    fn parse_constant_doc(&self, const_decl: &ConstantDeclaration, doc_comments: &HashMap<usize, String>, file_path: &Path) -> Result<ConstantDoc, Box<dyn std::error::Error>> {
        let description = doc_comments.get(&const_decl.line_number).cloned().unwrap_or_default();

        Ok(ConstantDoc {
            name: const_decl.name.clone(),
            value: format!("{:?}", const_decl.value),
            const_type: format!("{:?}", const_decl.const_type),
            description,
            file_path: file_path.to_string_lossy().to_string(),
            line_number: const_decl.line_number,
        })
    }

    /// Generate function signature
    fn generate_function_signature(&self, func: &FunctionDeclaration) -> String {
        let mut sig = format!("slay {}(", func.name);
        
        for (i, param) in func.parameters.iter().enumerate() {
            if i > 0 {
                sig.push_str(", ");
            }
            sig.push_str(&format!("{} {:?}", param.name, param.param_type));
        }
        
        sig.push(')');
        if let Some(ret_type) = &func.return_type {
            sig.push_str(&format!(" {:?}", ret_type));
        }
        
        sig
    }

    /// Calculate function complexity
    fn calculate_complexity(&self, func: &FunctionDeclaration) -> Option<String> {
        // Simple complexity calculation based on AST depth and branching
        let mut complexity = 1;
        
        // Count control flow statements
        complexity += self.count_control_flow(&func.body);
        
        match complexity {
            1..=5 => Some("Low".to_string()),
            6..=10 => Some("Medium".to_string()),
            11..=15 => Some("High".to_string()),
            _ => Some("Very High".to_string()),
        }
    }

    /// Count control flow statements
    fn count_control_flow(&self, body: &[AstNode]) -> usize {
        let mut count = 0;
        for node in body {
            match node {
                AstNode::IfStatement(_) => count += 1,
                AstNode::WhileLoop(_) => count += 1,
                AstNode::ForLoop(_) => count += 1,
                AstNode::SwitchStatement(_) => count += 2,
                _ => {}
            }
        }
        count
    }

    /// Find all CURSED files in directory
    fn find_cursed_files(&self, dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if !self.should_skip_directory(&path) {
                    files.extend(self.find_cursed_files(&path)?);
                }
            } else if path.extension().map_or(false, |ext| ext == "csd") {
                files.push(path);
            }
        }
        
        Ok(files)
    }

    /// Check if directory should be skipped
    fn should_skip_directory(&self, path: &Path) -> bool {
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        matches!(name.as_ref(), "target" | "node_modules" | ".git" | ".svn")
    }

    /// Parse example files
    fn parse_examples(&self, examples_dir: &Path) -> Result<Vec<ExampleDoc>, Box<dyn std::error::Error>> {
        let mut examples = Vec::new();
        
        for entry in fs::read_dir(examples_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "csd") {
                let content = fs::read_to_string(&path)?;
                let title = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                
                examples.push(ExampleDoc {
                    title,
                    description: self.extract_example_description(&content),
                    code: content,
                    expected_output: None,
                    file_path: path.to_string_lossy().to_string(),
                });
            }
        }
        
        Ok(examples)
    }

    /// Extract example description
    fn extract_example_description(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut description = String::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("##") {
                description.push_str(&trimmed[2..].trim());
                description.push(' ');
            } else if !trimmed.is_empty() {
                break;
            }
        }

        description.trim().to_string()
    }

    /// Generate HTML documentation
    fn generate_html(&self, documentation: &Documentation) -> Result<(), Box<dyn std::error::Error>> {
        let html_dir = self.output_dir.join("html");
        fs::create_dir_all(&html_dir)?;

        // Generate index page
        self.generate_html_index(&html_dir, documentation)?;

        // Generate module pages
        for module in &documentation.modules {
            self.generate_html_module(&html_dir, module)?;
        }

        // Generate function reference
        self.generate_html_functions(&html_dir, &documentation.functions)?;

        // Generate type reference
        self.generate_html_types(&html_dir, &documentation.types)?;

        println!("✅ HTML documentation generated");
        Ok(())
    }

    /// Generate HTML index page
    fn generate_html_index(&self, html_dir: &Path, documentation: &Documentation) -> Result<(), Box<dyn std::error::Error>> {
        let html_content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header>
        <h1>{}</h1>
        <p>{}</p>
    </header>
    
    <nav>
        <ul>
            <li><a href="#modules">Modules</a></li>
            <li><a href="#functions">Functions</a></li>
            <li><a href="#types">Types</a></li>
            <li><a href="#examples">Examples</a></li>
        </ul>
    </nav>
    
    <main>
        <section id="modules">
            <h2>Modules</h2>
            <ul>
                {}
            </ul>
        </section>
        
        <section id="functions">
            <h2>Functions</h2>
            <p>{} functions documented</p>
        </section>
        
        <section id="types">
            <h2>Types</h2>
            <p>{} types documented</p>
        </section>
        
        <section id="examples">
            <h2>Examples</h2>
            <p>{} examples available</p>
        </section>
    </main>
</body>
</html>"#,
            documentation.config.title,
            documentation.config.title,
            documentation.config.description,
            documentation.modules.iter()
                .map(|m| format!("<li><a href=\"modules/{}.html\">{}</a> - {}</li>", m.name, m.name, m.description))
                .collect::<Vec<_>>()
                .join("\n"),
            documentation.functions.len(),
            documentation.types.len(),
            documentation.examples.len()
        );

        fs::write(html_dir.join("index.html"), html_content)?;
        Ok(())
    }

    /// Generate HTML module page
    fn generate_html_module(&self, html_dir: &Path, module: &ModuleDoc) -> Result<(), Box<dyn std::error::Error>> {
        let modules_dir = html_dir.join("modules");
        fs::create_dir_all(&modules_dir)?;

        let html_content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Module Documentation</title>
    <link rel="stylesheet" href="../styles.css">
</head>
<body>
    <header>
        <h1>{}</h1>
        <p>{}</p>
    </header>
    
    <nav>
        <a href="../index.html">← Back to Index</a>
    </nav>
    
    <main>
        <section id="functions">
            <h2>Functions</h2>
            {}
        </section>
        
        <section id="types">
            <h2>Types</h2>
            {}
        </section>
        
        <section id="constants">
            <h2>Constants</h2>
            {}
        </section>
    </main>
</body>
</html>"#,
            module.name,
            module.name,
            module.description,
            module.functions.iter()
                .map(|f| format!("<div class=\"function\"><h3>{}</h3><p>{}</p><code>{}</code></div>", f.name, f.description, f.signature))
                .collect::<Vec<_>>()
                .join("\n"),
            module.types.iter()
                .map(|t| format!("<div class=\"type\"><h3>{}</h3><p>{}</p></div>", t.name, t.description))
                .collect::<Vec<_>>()
                .join("\n"),
            module.constants.iter()
                .map(|c| format!("<div class=\"constant\"><h3>{}</h3><p>{}</p></div>", c.name, c.description))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(modules_dir.join(format!("{}.html", module.name)), html_content)?;
        Ok(())
    }

    /// Generate HTML functions page
    fn generate_html_functions(&self, html_dir: &Path, functions: &[FunctionDoc]) -> Result<(), Box<dyn std::error::Error>> {
        let html_content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Functions Reference</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header>
        <h1>Functions Reference</h1>
    </header>
    
    <nav>
        <a href="index.html">← Back to Index</a>
    </nav>
    
    <main>
        {}
    </main>
</body>
</html>"#,
            functions.iter()
                .map(|f| format!(r#"<div class="function-doc">
                    <h2>{}</h2>
                    <p>{}</p>
                    <code>{}</code>
                    <p><strong>Complexity:</strong> {}</p>
                    <p><strong>File:</strong> {}:{}</p>
                </div>"#, 
                    f.name, f.description, f.signature, 
                    f.complexity.as_ref().unwrap_or(&"Unknown".to_string()),
                    f.file_path, f.line_number))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(html_dir.join("functions.html"), html_content)?;
        Ok(())
    }

    /// Generate HTML types page
    fn generate_html_types(&self, html_dir: &Path, types: &[TypeDoc]) -> Result<(), Box<dyn std::error::Error>> {
        let html_content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Types Reference</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header>
        <h1>Types Reference</h1>
    </header>
    
    <nav>
        <a href="index.html">← Back to Index</a>
    </nav>
    
    <main>
        {}
    </main>
</body>
</html>"#,
            types.iter()
                .map(|t| format!(r#"<div class="type-doc">
                    <h2>{}</h2>
                    <p>{}</p>
                    <p><strong>Kind:</strong> {}</p>
                    <p><strong>File:</strong> {}:{}</p>
                </div>"#, 
                    t.name, t.description, t.type_kind, t.file_path, t.line_number))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(html_dir.join("types.html"), html_content)?;
        Ok(())
    }

    /// Generate Markdown documentation
    fn generate_markdown(&self, documentation: &Documentation) -> Result<(), Box<dyn std::error::Error>> {
        let md_dir = self.output_dir.join("markdown");
        fs::create_dir_all(&md_dir)?;

        // Generate README.md
        let readme_content = format!(r#"# {}

{}

## Modules

{}

## Functions

{}

## Types

{}

## Examples

{}
"#,
            documentation.config.title,
            documentation.config.description,
            documentation.modules.iter()
                .map(|m| format!("- [{}](modules/{}.md) - {}", m.name, m.name, m.description))
                .collect::<Vec<_>>()
                .join("\n"),
            documentation.functions.len(),
            documentation.types.len(),
            documentation.examples.len()
        );

        fs::write(md_dir.join("README.md"), readme_content)?;

        // Generate module documentation
        let modules_dir = md_dir.join("modules");
        fs::create_dir_all(&modules_dir)?;

        for module in &documentation.modules {
            self.generate_markdown_module(&modules_dir, module)?;
        }

        println!("✅ Markdown documentation generated");
        Ok(())
    }

    /// Generate Markdown module documentation
    fn generate_markdown_module(&self, modules_dir: &Path, module: &ModuleDoc) -> Result<(), Box<dyn std::error::Error>> {
        let md_content = format!(r#"# {}

{}

## Functions

{}

## Types

{}

## Constants

{}
"#,
            module.name,
            module.description,
            module.functions.iter()
                .map(|f| format!("### {}\n\n{}\n\n```cursed\n{}\n```\n", f.name, f.description, f.signature))
                .collect::<Vec<_>>()
                .join("\n"),
            module.types.iter()
                .map(|t| format!("### {}\n\n{}\n\nType: {}\n", t.name, t.description, t.type_kind))
                .collect::<Vec<_>>()
                .join("\n"),
            module.constants.iter()
                .map(|c| format!("### {}\n\n{}\n\nValue: `{}`\n", c.name, c.description, c.value))
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(modules_dir.join(format!("{}.md", module.name)), md_content)?;
        Ok(())
    }

    /// Generate JSON documentation
    fn generate_json(&self, documentation: &Documentation) -> Result<(), Box<dyn std::error::Error>> {
        let json_dir = self.output_dir.join("json");
        fs::create_dir_all(&json_dir)?;

        let json_content = serde_json::to_string_pretty(documentation)?;
        fs::write(json_dir.join("documentation.json"), json_content)?;

        println!("✅ JSON documentation generated");
        Ok(())
    }

    /// Generate PDF documentation
    fn generate_pdf(&self, documentation: &Documentation) -> Result<(), Box<dyn std::error::Error>> {
        // This would require a PDF generation library like wkhtmltopdf
        // For now, generate HTML and provide conversion instructions
        self.generate_html(documentation)?;
        
        println!("✅ PDF documentation prepared (convert HTML to PDF using wkhtmltopdf)");
        Ok(())
    }

    /// Copy documentation assets
    fn copy_assets(&self) -> Result<(), Box<dyn std::error::Error>> {
        let assets_dir = self.output_dir.join("assets");
        fs::create_dir_all(&assets_dir)?;

        // Create default CSS
        let css_content = r#"
/* CURSED Documentation Styles */
body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    color: #333;
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    background-color: #f8f9fa;
}

header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 2rem;
    border-radius: 10px;
    margin-bottom: 2rem;
}

h1 { color: #2c3e50; }
h2 { color: #34495e; border-bottom: 2px solid #3498db; padding-bottom: 0.5rem; }
h3 { color: #7f8c8d; }

.function-doc, .type-doc, .constant {
    background: white;
    padding: 1.5rem;
    margin-bottom: 1rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

code {
    background: #f1f2f6;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

nav {
    background: white;
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 2rem;
}

nav ul {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    gap: 2rem;
}

nav a {
    color: #3498db;
    text-decoration: none;
    font-weight: 500;
}

nav a:hover {
    text-decoration: underline;
}
"#;

        fs::write(self.output_dir.join("html").join("styles.css"), css_content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_doc_generator_creation() {
        let temp_dir = tempdir().unwrap();
        let config = DocConfig::default();
        let doc_gen = DocGenerator::new(temp_dir.path().to_path_buf(), config);
        
        assert_eq!(doc_gen.output_dir, temp_dir.path());
    }

    #[test]
    fn test_extract_doc_comments() {
        let doc_gen = DocGenerator::new(PathBuf::from("test"), DocConfig::default());
        let content = "## This is a module\n## with documentation\n\nslay test() {}";
        let comments = doc_gen.extract_doc_comments(content);
        
        assert!(!comments.is_empty());
    }
}

impl Default for DocConfig {
    fn default() -> Self {
        Self {
            title: "CURSED Documentation".to_string(),
            description: "Generated documentation for CURSED project".to_string(),
            author: "CURSED Developer".to_string(),
            version: "1.0.0".to_string(),
            theme: "default".to_string(),
            include_private: false,
            include_tests: true,
            include_examples: true,
            output_formats: vec!["html".to_string(), "markdown".to_string()],
            custom_css: None,
            logo_path: None,
        }
    }
}
