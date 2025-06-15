//! Project Template System
//! 
//! Provides templates for initializing new CURSED projects with proper
//! structure, configuration files, and example code.

use crate::build_system::{BuildConfig, ProjectType};
use crate::build_system::build_config::ConfigError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, instrument};

/// Project template manager
#[derive(Debug)]
pub struct TemplateManager {
    /// Available templates
    templates: HashMap<String, ProjectTemplate>,
}

/// Project template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplate {
    /// Template name
    pub name: String,
    
    /// Template description
    pub description: String,
    
    /// Template category
    pub category: TemplateCategory,
    
    /// Files to create
    pub files: Vec<TemplateFile>,
    
    /// Directories to create
    pub directories: Vec<String>,
    
    /// Template variables
    pub variables: HashMap<String, TemplateVariable>,
    
    /// Default build configuration
    pub build_config: BuildConfig,
    
    /// Post-generation scripts
    pub post_scripts: Vec<String>,
}

/// Template categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateCategory {
    /// Command-line application
    CLI,
    /// Library
    Library,
    /// Web application
    Web,
    /// API service
    API,
    /// Desktop application
    Desktop,
    /// Game
    Game,
    /// Custom template
    Custom,
}

/// Template file definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFile {
    /// Relative path within the project
    pub path: PathBuf,
    
    /// File content template
    pub content: String,
    
    /// Whether file is executable
    pub executable: bool,
    
    /// Conditional creation (optional)
    pub condition: Option<String>,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Variable description
    pub description: String,
    
    /// Default value
    pub default: Option<String>,
    
    /// Whether variable is required
    pub required: bool,
    
    /// Variable type
    pub var_type: VariableType,
}

/// Variable types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    String,
    Boolean,
    Number,
    Choice(Vec<String>),
}

/// Template generation context
#[derive(Debug, Clone)]
pub struct TemplateContext {
    /// Project name
    pub project_name: String,
    
    /// Target directory
    pub target_dir: PathBuf,
    
    /// Template variables
    pub variables: HashMap<String, String>,
}

/// Template error types
#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("Template not found: {name}")]
    TemplateNotFound { name: String },
    
    #[error("Invalid template variable: {variable}")]
    InvalidVariable { variable: String },
    
    #[error("Missing required variable: {variable}")]
    MissingVariable { variable: String },
    
    #[error("Template generation failed: {reason}")]
    GenerationFailed { reason: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Template parsing error: {0}")]
    ParseError(String),
}

impl TemplateManager {
    /// Create a new template manager with built-in templates
    pub fn new() -> Self {
        let mut manager = TemplateManager {
            templates: HashMap::new(),
        };
        
        manager.register_builtin_templates();
        manager
    }
    
    /// Register built-in templates
    fn register_builtin_templates(&mut self) {
        // CLI Application Template
        self.register_template(create_cli_template());
        
        // Library Template
        self.register_template(create_library_template());
        
        // Web Application Template
        self.register_template(create_web_template());
        
        // API Service Template
        self.register_template(create_api_template());
        
        // Game Template
        self.register_template(create_game_template());
    }
    
    /// Register a new template
    pub fn register_template(&mut self, template: ProjectTemplate) {
        info!("Registering template: {}", template.name);
        self.templates.insert(template.name.clone(), template);
    }
    
    /// Get available templates
    pub fn list_templates(&self) -> Vec<&ProjectTemplate> {
        self.templates.values().collect()
    }
    
    /// Get templates by category
    pub fn get_templates_by_category(&self, category: &TemplateCategory) -> Vec<&ProjectTemplate> {
        self.templates
            .values()
            .filter(|t| std::mem::discriminant(&t.category) == std::mem::discriminant(category))
            .collect()
    }
    
    /// Generate project from template
    #[instrument(skip(self, context))]
    pub fn generate_project(
        &self,
        template_name: &str,
        context: TemplateContext,
    ) -> Result<(), TemplateError> {
        info!("Generating project '{}' from template '{}'", context.project_name, template_name);
        
        let template = self.templates.get(template_name)
            .ok_or_else(|| TemplateError::TemplateNotFound { name: template_name.to_string() })?;
        
        // Validate template variables
        self.validate_context(template, &context)?;
        
        // Create target directory
        std::fs::create_dir_all(&context.target_dir)?;
        
        // Create directories
        for dir in &template.directories {
            let dir_path = context.target_dir.join(dir);
            debug!("Creating directory: {}", dir_path.display());
            std::fs::create_dir_all(&dir_path)?;
        }
        
        // Generate files
        for file in &template.files {
            // Check condition if specified
            if let Some(condition) = &file.condition {
                if !self.evaluate_condition(condition, &context)? {
                    debug!("Skipping file {} due to condition", file.path.display());
                    continue;
                }
            }
            
            let file_path = context.target_dir.join(&file.path);
            debug!("Generating file: {}", file_path.display());
            
            // Create parent directory if needed
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            // Process template content
            let content = self.process_template_content(&file.content, &context)?;
            
            // Write file
            std::fs::write(&file_path, content)?;
            
            // Set executable permission if needed
            if file.executable {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = std::fs::metadata(&file_path)?.permissions();
                    perms.set_mode(0o755);
                    std::fs::set_permissions(&file_path, perms)?;
                }
            }
        }
        
        // Generate build configuration
        let config_path = context.target_dir.join("CursedBuild.toml");
        let mut build_config = template.build_config.clone();
        build_config.project.name = context.project_name.clone();
        
        // Process build config with template variables
        self.process_build_config(&mut build_config, &context)?;
        
        build_config.save_to_file(&config_path)
            .map_err(|e| TemplateError::GenerationFailed { reason: e.to_string() })?;
        
        // Run post-generation scripts
        for script in &template.post_scripts {
            self.run_post_script(script, &context)?;
        }
        
        info!("Project '{}' generated successfully", context.project_name);
        Ok(())
    }
    
    /// Validate template context
    fn validate_context(&self, template: &ProjectTemplate, context: &TemplateContext) -> Result<(), TemplateError> {
        for (name, variable) in &template.variables {
            if variable.required && !context.variables.contains_key(name) {
                return Err(TemplateError::MissingVariable { variable: name.clone() });
            }
            
            if let Some(value) = context.variables.get(name) {
                self.validate_variable_value(name, value, variable)?;
            }
        }
        
        Ok(())
    }
    
    /// Validate a variable value
    fn validate_variable_value(
        &self,
        name: &str,
        value: &str,
        variable: &TemplateVariable,
    ) -> Result<(), TemplateError> {
        match &variable.var_type {
            VariableType::String => {
                // String validation (could add length checks, etc.)
                Ok(())
            }
            VariableType::Boolean => {
                if !matches!(value, "true" | "false" | "yes" | "no" | "1" | "0") {
                    return Err(TemplateError::InvalidVariable { variable: name.to_string() });
                }
                Ok(())
            }
            VariableType::Number => {
                value.parse::<f64>()
                    .map_err(|_| TemplateError::InvalidVariable { variable: name.to_string() })?;
                Ok(())
            }
            VariableType::Choice(choices) => {
                if !choices.contains(&value.to_string()) {
                    return Err(TemplateError::InvalidVariable { variable: name.to_string() });
                }
                Ok(())
            }
        }
    }
    
    /// Process template content with variable substitution
    pub fn process_template_content(&self, content: &str, context: &TemplateContext) -> Result<String, TemplateError> {
        let mut result = content.to_string();
        
        // Replace project name
        result = result.replace("{{project_name}}", &context.project_name);
        
        // Replace custom variables
        for (name, value) in &context.variables {
            let placeholder = format!("{{{{{}}}}}", name);
            result = result.replace(&placeholder, value);
        }
        
        Ok(result)
    }
    
    /// Process build configuration with template variables
    fn process_build_config(&self, config: &mut BuildConfig, context: &TemplateContext) -> Result<(), TemplateError> {
        // Update project metadata
        if let Some(description) = context.variables.get("description") {
            config.project.description = Some(description.clone());
        }
        
        if let Some(author) = context.variables.get("author") {
            config.project.authors = Vec::from([author.clone()]);
        }
        
        if let Some(license) = context.variables.get("license") {
            config.project.license = Some(license.clone());
        }
        
        Ok(())
    }
    
    /// Evaluate a template condition
    fn evaluate_condition(&self, condition: &str, context: &TemplateContext) -> Result<bool, TemplateError> {
        // Simple condition evaluation - can be expanded
        if condition.starts_with("var:") {
            let var_name = &condition[4..];
            Ok(context.variables.contains_key(var_name))
        } else if condition.starts_with("eq:") {
            let parts: Vec<&str> = condition[3..].split('=').collect();
            if parts.len() == 2 {
                let var_name = parts[0];
                let expected_value = parts[1];
                Ok(context.variables.get(var_name) == Some(&expected_value.to_string()))
            } else {
                Err(TemplateError::ParseError(format!("Invalid condition: {}", condition)))
            }
        } else {
            Err(TemplateError::ParseError(format!("Unknown condition type: {}", condition)))
        }
    }
    
    /// Run post-generation script
    fn run_post_script(&self, script: &str, context: &TemplateContext) -> Result<(), TemplateError> {
        debug!("Running post-script: {}", script);
        
        let mut cmd = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
        } else {
            std::process::Command::new("sh")
        };
        
        if cfg!(target_os = "windows") {
            cmd.args(["/C", script]);
        } else {
            cmd.args(["-c", script]);
        }
        
        cmd.current_dir(&context.target_dir);
        
        let output = cmd.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TemplateError::GenerationFailed {
                reason: format!("Post-script failed: {}", stderr),
            });
        }
        
        Ok(())
    }
}

/// Create CLI application template
fn create_cli_template() -> ProjectTemplate {
    ProjectTemplate {
        name: "cli".to_string(),
        description: "Command-line application template".to_string(),
        category: TemplateCategory::CLI,
        files: vec![
            TemplateFile {
                path: PathBuf::from("src/main.csd"),
                content: CLI_MAIN_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("README.md"),
                content: CLI_README_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from(".gitignore"),
                content: GITIGNORE_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
        ],
        directories: Vec::from(["src".to_string(), "tests".to_string()]),
        variables: create_common_variables(),
        build_config: BuildConfig::default_for_project("{{project_name}}", ProjectType::Binary),
        post_scripts: Vec::from([]),
    }
}

/// Create library template
fn create_library_template() -> ProjectTemplate {
    ProjectTemplate {
        name: "lib".to_string(),
        description: "Library template".to_string(),
        category: TemplateCategory::Library,
        files: vec![
            TemplateFile {
                path: PathBuf::from("src/lib.csd"),
                content: LIB_MAIN_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("README.md"),
                content: LIB_README_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from(".gitignore"),
                content: GITIGNORE_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
        ],
        directories: Vec::from(["src".to_string(), "tests".to_string(), "examples".to_string()]),
        variables: create_common_variables(),
        build_config: BuildConfig::default_for_project("{{project_name}}", ProjectType::Library),
        post_scripts: Vec::from([]),
    }
}

/// Create web application template
fn create_web_template() -> ProjectTemplate {
    ProjectTemplate {
        name: "web".to_string(),
        description: "Web application template".to_string(),
        category: TemplateCategory::Web,
        files: vec![
            TemplateFile {
                path: PathBuf::from("src/main.csd"),
                content: WEB_MAIN_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("src/server.csd"),
                content: WEB_SERVER_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("static/index.html"),
                content: WEB_INDEX_HTML_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("README.md"),
                content: WEB_README_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from(".gitignore"),
                content: GITIGNORE_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
        ],
        directories: Vec::from(["src".to_string(), "static".to_string(), "templates".to_string()]),
        variables: create_web_variables(),
        build_config: BuildConfig::default_for_project("{{project_name}}", ProjectType::Binary),
        post_scripts: Vec::from([]),
    }
}

/// Create API service template
fn create_api_template() -> ProjectTemplate {
    ProjectTemplate {
        name: "api".to_string(),
        description: "REST API service template".to_string(),
        category: TemplateCategory::API,
        files: vec![
            TemplateFile {
                path: PathBuf::from("src/main.csd"),
                content: API_MAIN_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("src/routes.csd"),
                content: API_ROUTES_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("api.yaml"),
                content: API_SPEC_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("README.md"),
                content: API_README_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from(".gitignore"),
                content: GITIGNORE_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
        ],
        directories: Vec::from(["src".to_string(), "tests".to_string()]),
        variables: create_api_variables(),
        build_config: BuildConfig::default_for_project("{{project_name}}", ProjectType::Binary),
        post_scripts: Vec::from([]),
    }
}

/// Create game template
fn create_game_template() -> ProjectTemplate {
    ProjectTemplate {
        name: "game".to_string(),
        description: "Game application template".to_string(),
        category: TemplateCategory::Game,
        files: vec![
            TemplateFile {
                path: PathBuf::from("src/main.csd"),
                content: GAME_MAIN_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("src/game.csd"),
                content: GAME_LOGIC_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from("README.md"),
                content: GAME_README_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
            TemplateFile {
                path: PathBuf::from(".gitignore"),
                content: GITIGNORE_TEMPLATE.to_string(),
                executable: false,
                condition: None,
            },
        ],
        directories: Vec::from(["src".to_string(), "assets".to_string(), "tests".to_string()]),
        variables: create_game_variables(),
        build_config: BuildConfig::default_for_project("{{project_name}}", ProjectType::Binary),
        post_scripts: Vec::from([]),
    }
}

/// Create common template variables
fn create_common_variables() -> HashMap<String, TemplateVariable> {
    let mut variables = HashMap::new();
    
    variables.insert("description".to_string(), TemplateVariable {
        description: "Project description".to_string(),
        default: Some("A CURSED project".to_string()),
        required: false,
        var_type: VariableType::String,
    });
    
    variables.insert("author".to_string(), TemplateVariable {
        description: "Project author".to_string(),
        default: Some("Your Name <your.email@example.com>".to_string()),
        required: false,
        var_type: VariableType::String,
    });
    
    variables.insert("license".to_string(), TemplateVariable {
        description: "Project license".to_string(),
        default: Some("MIT OR Apache-2.0".to_string()),
        required: false,
        var_type: VariableType::Choice(vec![
            "MIT".to_string(),
            "Apache-2.0".to_string(),
            "MIT OR Apache-2.0".to_string(),
            "GPL-3.0".to_string(),
            "BSD-3-Clause".to_string(),
        ]),
    });
    
    variables
}

/// Create web-specific variables
fn create_web_variables() -> HashMap<String, TemplateVariable> {
    let mut variables = create_common_variables();
    
    variables.insert("port".to_string(), TemplateVariable {
        description: "Server port".to_string(),
        default: Some("8080".to_string()),
        required: false,
        var_type: VariableType::Number,
    });
    
    variables
}

/// Create API-specific variables
fn create_api_variables() -> HashMap<String, TemplateVariable> {
    let mut variables = create_common_variables();
    
    variables.insert("api_version".to_string(), TemplateVariable {
        description: "API version".to_string(),
        default: Some("v1".to_string()),
        required: false,
        var_type: VariableType::String,
    });
    
    variables
}

/// Create game-specific variables
fn create_game_variables() -> HashMap<String, TemplateVariable> {
    let mut variables = create_common_variables();
    
    variables.insert("game_type".to_string(), TemplateVariable {
        description: "Type of game".to_string(),
        default: Some("puzzle".to_string()),
        required: false,
        var_type: VariableType::Choice(vec![
            "puzzle".to_string(),
            "platformer".to_string(),
            "rpg".to_string(),
            "strategy".to_string(),
            "arcade".to_string(),
        ]),
    });
    
    variables
}

// Template constants
const CLI_MAIN_TEMPLATE: &str = r#"// {{project_name}} - A CURSED CLI Application
//
// {{description}}

yeet "std::env";
yeet "std::io";

slay main() -> i32 {
    let args = env::args();
    
    lowkey args.len() > 1 {
        let command = args[1];
        
        lowkey command == "help" {
            show_help();
        } flex {
            io::println("Unknown command: " + command);
            io::println("Use 'help' for usage information.");
            return 1;
        }
    } flex {
        io::println("Hello from {{project_name}}!");
        io::println("Use 'help' for usage information.");
    }
    
    return 0;
}

slay show_help() {
    io::println("{{project_name}} - {{description}}");
    io::println("");
    io::println("Usage:");
    io::println("  {{project_name}} [command]");
    io::println("");
    io::println("Commands:");
    io::println("  help    Show this help message");
}
"#;

const LIB_MAIN_TEMPLATE: &str = r#"// {{project_name}} - A CURSED Library
//
// {{description}}

/// Main library module for {{project_name}}
vibe {{project_name}}_lib;

/// Example function that demonstrates library usage
slay hello(name: str) -> str {
    return "Hello, " + name + "! Welcome to {{project_name}}.";
}

/// Example struct
squad Person {
    name: str,
    age: i32,
}

impl Person {
    /// Create a new person
    slay new(name: str, age: i32) -> Person {
        return Person { name: name, age: age };
    }
    
    /// Get a greeting from this person
    slay greet(&self) -> str {
        return hello(self.name);
    }
}

#[test]
slay test_hello() {
    let result = hello("World");
    assert_eq!(result, "Hello, World! Welcome to {{project_name}}.");
}

#[test]
slay test_person() {
    let person = Person::new("Alice", 30);
    let greeting = person.greet();
    assert_eq!(greeting, "Hello, Alice! Welcome to {{project_name}}.");
}
"#;

const WEB_MAIN_TEMPLATE: &str = r#"// {{project_name}} - A CURSED Web Application
//
// {{description}}

yeet "std::io";
yeet "./server";

slay main() -> i32 {
    io::println("Starting {{project_name}} web server...");
    
    let server = server::WebServer::new({{port}});
    
    lowkey let err = server.start() {
        io::println("Error starting server: " + err.message());
        return 1;
    }
    
    io::println("Server started on port {{port}}");
    return 0;
}
"#;

const WEB_SERVER_TEMPLATE: &str = r#"// Web server implementation for {{project_name}}

yeet "std::net";
yeet "std::io";
yeet "std::thread";
yeet "std::sync";

squad WebServer {
    port: i32,
    routes: Vec<Route>,
}

squad Route {
    path: str,
    method: HttpMethod,
    handler: slay(Request) -> Response,
}

squad Request {
    path: str,
    method: HttpMethod,
    headers: Map<str, str>,
    body: str,
}

squad Response {
    status: i32,
    headers: Map<str, str>,
    body: str,
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl WebServer {
    slay new(port: i32) -> WebServer {
        return WebServer { 
            port: port,
            routes: Vec::new(),
        };
    }
    
    slay add_route(&mut self, path: str, method: HttpMethod, handler: slay(Request) -> Response) {
        let route = Route { path: path, method: method, handler: handler };
        self.routes.push(route);
    }
    
    slay start(&self) -> Result<(), Error> {
        io::println("Starting {{project_name}} server on port " + self.port.to_string());
        
        // Create TCP listener
        let listener = net::TcpListener::bind("127.0.0.1:" + self.port.to_string())?;
        io::println("Server listening on http://127.0.0.1:" + self.port.to_string());
        
        // Handle incoming connections
        bestie connection in listener.incoming() {
            vibe_check connection {
                mood Ok(stream) => {
                    self.handle_connection(stream);
                }
                mood Err(e) => {
                    io::println("Connection error: " + e.to_string());
                }
            }
        }
        
        return Ok(());
    }
    
    slay handle_connection(&self, mut stream: net::TcpStream) {
        let request = self.parse_request(&stream);
        let response = self.route_request(request);
        self.send_response(&stream, response);
    }
    
    slay parse_request(&self, stream: &net::TcpStream) -> Request {
        // Basic HTTP request parsing
        let mut buffer = [0; 1024];
        stream.read(&mut buffer);
        let request_str = str::from_utf8(&buffer).unwrap_or("");
        
        let lines: Vec<&str> = request_str.split("\r\n").collect();
        lowkey lines.len() > 0 {
            let request_line = lines[0];
            let parts: Vec<&str> = request_line.split(" ").collect();
            
            lowkey parts.len() >= 3 {
                let method = vibe_check parts[0] {
                    mood "GET" => HttpMethod::GET,
                    mood "POST" => HttpMethod::POST,
                    mood "PUT" => HttpMethod::PUT,
                    mood "DELETE" => HttpMethod::DELETE,
                    basic => HttpMethod::GET,
                };
                
                return Request {
                    path: parts[1].to_string(),
                    method: method,
                    headers: Map::new(),
                    body: "".to_string(),
                };
            }
        }
        
        return Request {
            path: "/".to_string(),
            method: HttpMethod::GET,
            headers: Map::new(),
            body: "".to_string(),
        };
    }
    
    slay route_request(&self, request: Request) -> Response {
        // Find matching route
        bestie route in &self.routes {
            lowkey route.path == request.path && route.method == request.method {
                return (route.handler)(request);
            }
        }
        
        // Default 404 response
        return Response {
            status: 404,
            headers: Map::new(),
            body: "404 Not Found".to_string(),
        };
    }
    
    slay send_response(&self, mut stream: &net::TcpStream, response: Response) {
        let status_line = "HTTP/1.1 " + response.status.to_string() + " OK\r\n";
        let content_length = "Content-Length: " + response.body.len().to_string() + "\r\n";
        let content_type = "Content-Type: text/html\r\n";
        let connection = "Connection: close\r\n";
        
        let http_response = status_line + content_type + content_length + connection + "\r\n" + response.body;
        
        stream.write(http_response.as_bytes());
        stream.flush();
    }
    
    // Helper methods for common routes
    slay get(&mut self, path: str, handler: slay(Request) -> Response) {
        self.add_route(path, HttpMethod::GET, handler);
    }
    
    slay post(&mut self, path: str, handler: slay(Request) -> Response) {
        self.add_route(path, HttpMethod::POST, handler);
    }
    
    // Static file serving
    slay serve_static(&self, request: Request) -> Response {
        let file_path = "static" + request.path;
        
        vibe_check std::fs::read_to_string(file_path) {
            mood Ok(content) => {
                return Response {
                    status: 200,
                    headers: Map::new(),
                    body: content,
                };
            }
            mood Err(_) => {
                return Response {
                    status: 404,
                    headers: Map::new(),
                    body: "File not found".to_string(),
                };
            }
        }
    }
}
"#;

const WEB_INDEX_HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{project_name}}</title>
</head>
<body>
    <h1>Welcome to {{project_name}}</h1>
    <p>{{description}}</p>
    
    <p>This is a CURSED web application running on the most lit programming language! 🔥</p>
</body>
</html>
"#;

const API_MAIN_TEMPLATE: &str = r#"// {{project_name}} - A CURSED API Service
//
// {{description}}

yeet "std::io";
yeet "./routes";

slay main() -> i32 {
    io::println("Starting {{project_name}} API server...");
    
    let api = routes::ApiServer::new();
    
    lowkey let err = api.start() {
        io::println("Error starting API server: " + err.message());
        return 1;
    }
    
    return 0;
}
"#;

const API_ROUTES_TEMPLATE: &str = r#"// API routes for {{project_name}}

yeet "std::net";
yeet "std::json";
yeet "std::time";
yeet "std::collections";

squad ApiServer {
    port: i32,
    routes: Map<str, RouteHandler>,
    middleware: Vec<Middleware>,
}

squad RouteHandler {
    method: HttpMethod,
    handler: slay(ApiRequest) -> ApiResponse,
}

squad ApiRequest {
    path: str,
    method: HttpMethod,
    headers: Map<str, str>,
    query_params: Map<str, str>,
    body: json::Value,
    user_id: Option<str>,
}

squad ApiResponse {
    status: i32,
    headers: Map<str, str>,
    body: json::Value,
}

squad Middleware {
    name: str,
    handler: slay(ApiRequest) -> Result<ApiRequest, ApiResponse>,
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
}

impl ApiServer {
    slay new(port: i32) -> ApiServer {
        let mut server = ApiServer {
            port: port,
            routes: Map::new(),
            middleware: Vec::new(),
        };
        
        // Set up default routes
        server.setup_routes();
        
        return server;
    }
    
    slay setup_routes(&mut self) {
        // API status endpoint
        self.get("/api/{{api_version}}/status", |req| {
            self.get_status(req)
        });
        
        // Health check endpoint
        self.get("/health", |req| {
            self.health_check(req)
        });
        
        // Example CRUD endpoints
        self.get("/api/{{api_version}}/users", |req| {
            self.list_users(req)
        });
        
        self.post("/api/{{api_version}}/users", |req| {
            self.create_user(req)
        });
        
        self.get("/api/{{api_version}}/users/{id}", |req| {
            self.get_user(req)
        });
        
        self.put("/api/{{api_version}}/users/{id}", |req| {
            self.update_user(req)
        });
        
        self.delete("/api/{{api_version}}/users/{id}", |req| {
            self.delete_user(req)
        });
    }
    
    slay start(&self) -> Result<(), Error> {
        io::println("Starting {{project_name}} API server on port " + self.port.to_string());
        
        let listener = net::TcpListener::bind("127.0.0.1:" + self.port.to_string())?;
        io::println("API server listening on http://127.0.0.1:" + self.port.to_string());
        
        bestie connection in listener.incoming() {
            vibe_check connection {
                mood Ok(stream) => {
                    self.handle_request(stream);
                }
                mood Err(e) => {
                    io::println("Connection error: " + e.to_string());
                }
            }
        }
        
        return Ok(());
    }
    
    slay handle_request(&self, mut stream: net::TcpStream) {
        let request = self.parse_api_request(&stream);
        let response = self.process_request(request);
        self.send_api_response(&stream, response);
    }
    
    slay process_request(&self, mut request: ApiRequest) -> ApiResponse {
        // Apply middleware
        bestie middleware in &self.middleware {
            vibe_check (middleware.handler)(request) {
                mood Ok(updated_request) => {
                    request = updated_request;
                }
                mood Err(error_response) => {
                    return error_response;
                }
            }
        }
        
        // Route the request
        lowkey let Some(route_handler) = self.routes.get(&request.path) {
            lowkey route_handler.method == request.method {
                return (route_handler.handler)(request);
            }
        }
        
        // Method not allowed or route not found
        return self.not_found_response();
    }
    
    // Route helper methods
    slay get(&mut self, path: str, handler: slay(ApiRequest) -> ApiResponse) {
        self.routes.insert(path.to_string(), RouteHandler {
            method: HttpMethod::GET,
            handler: handler,
        });
    }
    
    slay post(&mut self, path: str, handler: slay(ApiRequest) -> ApiResponse) {
        self.routes.insert(path.to_string(), RouteHandler {
            method: HttpMethod::POST,
            handler: handler,
        });
    }
    
    slay put(&mut self, path: str, handler: slay(ApiRequest) -> ApiResponse) {
        self.routes.insert(path.to_string(), RouteHandler {
            method: HttpMethod::PUT,
            handler: handler,
        });
    }
    
    slay delete(&mut self, path: str, handler: slay(ApiRequest) -> ApiResponse) {
        self.routes.insert(path.to_string(), RouteHandler {
            method: HttpMethod::DELETE,
            handler: handler,
        });
    }
    
    // API endpoint implementations
    slay get_status(&self, request: ApiRequest) -> ApiResponse {
        return self.json_response(200, json::object([
            ("status", "ok"),
            ("service", "{{project_name}}"),
            ("version", "{{api_version}}"),
            ("timestamp", time::now().timestamp()),
            ("uptime", "0:00:00") // TODO: Calculate actual uptime
        ]));
    }
    
    slay health_check(&self, request: ApiRequest) -> ApiResponse {
        return self.json_response(200, json::object([
            ("healthy", true),
            ("checks", json::object([
                ("database", "ok"),
                ("memory", "ok"),
                ("disk", "ok")
            ]))
        ]));
    }
    
    slay list_users(&self, request: ApiRequest) -> ApiResponse {
        // Mock user data
        let users = json::array([
            json::object([
                ("id", 1),
                ("name", "John Doe"),
                ("email", "john@example.com")
            ]),
            json::object([
                ("id", 2),
                ("name", "Jane Smith"),
                ("email", "jane@example.com")
            ])
        ]);
        
        return self.json_response(200, json::object([
            ("users", users),
            ("total", 2),
            ("page", 1),
            ("per_page", 10)
        ]));
    }
    
    slay create_user(&self, request: ApiRequest) -> ApiResponse {
        // Validate request body
        lowkey !request.body.has_key("name") || !request.body.has_key("email") {
            return self.error_response(400, "Missing required fields: name, email");
        }
        
        // Mock user creation
        let new_user = json::object([
            ("id", 3),
            ("name", request.body["name"]),
            ("email", request.body["email"]),
            ("created_at", time::now().timestamp())
        ]);
        
        return self.json_response(201, new_user);
    }
    
    slay get_user(&self, request: ApiRequest) -> ApiResponse {
        // Extract user ID from path
        let user_id = self.extract_path_param(&request.path, "id");
        
        // Mock user lookup
        let user = json::object([
            ("id", user_id.parse::<i32>().unwrap_or(0)),
            ("name", "John Doe"),
            ("email", "john@example.com"),
            ("created_at", "2024-01-01T00:00:00Z")
        ]);
        
        return self.json_response(200, user);
    }
    
    slay update_user(&self, request: ApiRequest) -> ApiResponse {
        let user_id = self.extract_path_param(&request.path, "id");
        
        // Mock user update
        let updated_user = json::object([
            ("id", user_id.parse::<i32>().unwrap_or(0)),
            ("name", request.body.get("name").unwrap_or("John Doe")),
            ("email", request.body.get("email").unwrap_or("john@example.com")),
            ("updated_at", time::now().timestamp())
        ]);
        
        return self.json_response(200, updated_user);
    }
    
    slay delete_user(&self, request: ApiRequest) -> ApiResponse {
        let user_id = self.extract_path_param(&request.path, "id");
        
        return self.json_response(200, json::object([
            ("message", "User deleted successfully"),
            ("user_id", user_id)
        ]));
    }
    
    // Helper methods
    slay json_response(&self, status: i32, body: json::Value) -> ApiResponse {
        let mut headers = Map::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
        
        return ApiResponse {
            status: status,
            headers: headers,
            body: body,
        };
    }
    
    slay error_response(&self, status: i32, message: str) -> ApiResponse {
        return self.json_response(status, json::object([
            ("error", true),
            ("message", message),
            ("status", status)
        ]));
    }
    
    slay not_found_response(&self) -> ApiResponse {
        return self.error_response(404, "Route not found");
    }
    
    slay extract_path_param(&self, path: str, param_name: str) -> str {
        // Simple path parameter extraction
        let parts: Vec<&str> = path.split("/").collect();
        // In a real implementation, this would be more sophisticated
        return parts.last().unwrap_or("").to_string();
    }
}
"#;

const API_SPEC_TEMPLATE: &str = r#"openapi: 3.0.0
info:
  title: {{project_name}} API
  description: {{description}}
  version: {{api_version}}
  
servers:
  - url: http://localhost:8080/api/{{api_version}}
    description: Development server

paths:
  /status:
    get:
      summary: Get service status
      responses:
        '200':
          description: Service status
          content:
            application/json:
              schema:
                type: object
                properties:
                  status:
                    type: string
                  service:
                    type: string
                  version:
                    type: string
"#;

const GAME_MAIN_TEMPLATE: &str = r#"// {{project_name}} - A CURSED Game
//
// {{description}}

yeet "std::io";
yeet "./game";

slay main() -> i32 {
    io::println("Starting {{project_name}} game...");
    
    let mut game = game::Game::new();
    
    lowkey let err = game.run() {
        io::println("Game error: " + err.message());
        return 1;
    }
    
    io::println("Thanks for playing {{project_name}}!");
    return 0;
}
"#;

const GAME_LOGIC_TEMPLATE: &str = r#"// Game logic for {{project_name}}

yeet "std::io";

squad Game {
    running: bool,
}

impl Game {
    slay new() -> Game {
        return Game { running: true };
    }
    
    slay run(&mut self) -> Result<(), Error> {
        io::println("Welcome to {{project_name}}!");
        io::println("Type 'quit' to exit.");
        
        periodt self.running {
            io::print("> ");
            let input = io::read_line();
            
            lowkey input.trim() == "quit" {
                self.running = false;
            } flex {
                self.process_command(input.trim());
            }
        }
        
        return Ok(());
    }
    
    slay process_command(&self, command: &str) {
        vibe_check command {
            mood "help" => {
                self.show_help();
            }
            basic => {
                io::println("Unknown command: " + command);
                io::println("Type 'help' for available commands.");
            }
        }
    }
    
    slay show_help(&self) {
        io::println("Available commands:");
        io::println("  help - Show this help");
        io::println("  quit - Exit the game");
    }
}
"#;

const CLI_README_TEMPLATE: &str = r#"# {{project_name}}

{{description}}

## Installation

```bash
cursed-build build --release
```

## Usage

```bash
./target/release/{{project_name}} [command]
```

## Commands

- `help` - Show help information

## Development

```bash
# Build in debug mode
cursed-build build

# Run tests
cursed-build test

# Format code
cursed-build fmt

# Lint code
cursed-build lint
```

## License

{{license}}
"#;

const LIB_README_TEMPLATE: &str = r#"# {{project_name}}

{{description}}

## Usage

Add this to your `CursedBuild.toml`:

```toml
[dependencies]
{{project_name}} = "0.1.0"
```

Then in your CURSED code:

```cursed
yeet "{{project_name}}";

slay main() {
    let greeting = {{project_name}}::hello("World");
    io::println(greeting);
}
```

## API Documentation

TODO: Add API documentation

## Examples

See the `examples/` directory for usage examples.

## Development

```bash
# Build library
cursed-build build

# Run tests
cursed-build test

# Generate documentation
cursed-build doc
```

## License

{{license}}
"#;

const WEB_README_TEMPLATE: &str = r#"# {{project_name}}

{{description}}

## Quick Start

```bash
# Build and run
cursed-build run

# Or build separately
cursed-build build
./target/debug/{{project_name}}
```

The server will start on http://localhost:{{port}}

## Development

```bash
# Build in debug mode
cursed-build build

# Run tests
cursed-build test

# Watch for changes (if available)
cursed-build watch
```

## Features

- Web server on port {{port}}
- Static file serving
- TODO: Add more features

## License

{{license}}
"#;

const API_README_TEMPLATE: &str = r#"# {{project_name}}

{{description}}

## Quick Start

```bash
# Build and run
cursed-build run

# Or build separately
cursed-build build
./target/debug/{{project_name}}
```

## API Documentation

See `api.yaml` for the complete API specification.

### Endpoints

- `GET /api/{{api_version}}/status` - Service status
- Add more endpoints as needed

## Development

```bash
# Build in debug mode
cursed-build build

# Run tests
cursed-build test

# Generate API docs
cursed-build docs
```

## Testing

```bash
# Run all tests
cursed-build test

# Test specific endpoint
curl http://localhost:8080/api/{{api_version}}/status
```

## License

{{license}}
"#;

const GAME_README_TEMPLATE: &str = r#"# {{project_name}}

{{description}}

## Quick Start

```bash
# Build and run
cursed-build run

# Or build separately
cursed-build build
./target/debug/{{project_name}}
```

## Game Controls

- Type commands to interact with the game
- Type 'help' for available commands
- Type 'quit' to exit

## Development

```bash
# Build in debug mode
cursed-build build

# Run tests
cursed-build test

# Add game assets to assets/ directory
```

## Game Features

- {{game_type}} game mechanics
- Interactive command-line interface
- Extensible game engine
- TODO: Add more features

## Assets

Place game assets in the `assets/` directory:
- Images
- Sounds
- Configuration files
- Level data

## License

{{license}}
"#;

const GITIGNORE_TEMPLATE: &str = r#"# CURSED build artifacts
/target/
*.exe
*.dll
*.so
*.dylib

# Debug files
*.pdb
*.dSYM/

# Cache files
.cursed-cache/
*.cache

# IDE files
.vscode/
.idea/
*.swp
*.swo
*~

# OS files
.DS_Store
Thumbs.db

# Log files
*.log

# Environment files
.env
.env.local
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_template_manager_creation() {
        let manager = TemplateManager::new();
        
        let templates = manager.list_templates();
        assert!(templates.len() > 0);
        
        // Check that we have expected templates
        let template_names: Vec<_> = templates.iter().map(|t| &t.name).collect();
        assert!(template_names.contains(&&"cli".to_string()));
        assert!(template_names.contains(&&"lib".to_string()));
        assert!(template_names.contains(&&"web".to_string()));
    }
    
    #[test]
    fn test_template_generation() -> Result<(), Box<dyn std::error::Error>> {
        let manager = TemplateManager::new();
        let dir = tempdir()?;
        
        let mut variables = HashMap::new();
        variables.insert("description".to_string(), "Test CLI app".to_string());
        variables.insert("author".to_string(), "Test Author".to_string());
        
        let context = TemplateContext {
            project_name: "test-cli".to_string(),
            target_dir: dir.path().to_path_buf(),
            variables,
        };
        
        manager.generate_project("cli", context)?;
        
        // Check that files were created
        assert!(dir.path().join("src").exists());
        assert!(dir.path().join("src/main.csd").exists());
        assert!(dir.path().join("README.md").exists());
        assert!(dir.path().join("CursedBuild.toml").exists());
        
        // Check content substitution
        let main_content = std::fs::read_to_string(dir.path().join("src/main.csd"))?;
        assert!(main_content.contains("test-cli"));
        assert!(!main_content.contains("{{project_name}}"));
        
        Ok(())
    }
    
    #[test]
    fn test_template_categories() {
        let manager = TemplateManager::new();
        
        let cli_templates = manager.get_templates_by_category(&TemplateCategory::CLI);
        assert!(cli_templates.len() > 0);
        
        let web_templates = manager.get_templates_by_category(&TemplateCategory::Web);
        assert!(web_templates.len() > 0);
        
        let lib_templates = manager.get_templates_by_category(&TemplateCategory::Library);
        assert!(lib_templates.len() > 0);
    }
}
