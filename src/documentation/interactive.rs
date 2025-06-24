// # Interactive Documentation Features
//
// Interactive features for the CURSED documentation system including code playground,
// executable examples, API explorer, and real-time code execution capabilities.
//
// ## Features
//
// - **Code Playground**: Interactive code editor with syntax highlighting and execution
// - **Executable Examples**: Run documentation examples directly in the browser
// - **API Explorer**: Interactive testing of API functions and methods
// - **Syntax Highlighting**: Real-time syntax highlighting for CURSED code
// - **Code Folding**: Collapsible code sections for better readability
// - **Error Display**: Real-time error highlighting and explanations

use crate::error::Error as CursedError;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::process::Command;
use tokio::time::timeout;
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

/// Configuration for interactive features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveConfig {
    /// Enable code playground
    pub enable_playground: bool,
    
    /// Enable executable examples
    pub enable_executable_examples: bool,
    
    /// Enable API explorer
    pub enable_api_explorer: bool,
    
    /// Enable syntax highlighting
    pub enable_syntax_highlighting: bool,
    
    /// Enable code folding
    pub enable_code_folding: bool,
    
    /// Maximum execution time for code
    pub max_execution_time: Duration,
    
    /// Maximum memory usage for code execution
    pub max_memory_mb: usize,
    
    /// Allowed imports for sandbox
    pub allowed_imports: Vec<String>,
    
    /// Temporary directory for code execution
    pub temp_dir: PathBuf,
    
    /// Highlight themes available
    pub highlight_themes: Vec<String>,
    
    /// Default highlight theme
    pub default_theme: String,
}

impl Default for InteractiveConfig {
    fn default() -> Self {
        Self {
            enable_playground: true,
            enable_executable_examples: true,
            enable_api_explorer: true,
            enable_syntax_highlighting: true,
            enable_code_folding: true,
            max_execution_time: Duration::from_secs(10),
            max_memory_mb: 128,
            allowed_imports: vec![
                "stdlib::io".to_string(),
                "stdlib::math".to_string(),
                "stdlib::collections".to_string(),
                "stdlib::string".to_string(),
            ],
            temp_dir: std::env::temp_dir().join("cursed_playground"),
            highlight_themes: vec![
                "monokai".to_string(),
                "github".to_string(),
                "solarized-dark".to_string(),
                "solarized-light".to_string(),
                "dracula".to_string(),
            ],
            default_theme: "monokai".to_string(),
        }
    }
}

/// Code execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionRequest {
    /// Session ID for tracking
    pub session_id: String,
    
    /// Code to execute
    pub code: String,
    
    /// Language (should be "cursed" or "csd")
    pub language: String,
    
    /// Input data for the program
    pub input: Option<String>,
    
    /// Execution timeout override
    pub timeout: Option<Duration>,
    
    /// Additional arguments
    pub args: Vec<String>,
    
    /// Environment variables
    pub env: HashMap<String, String>,
    
    /// Working directory override
    pub working_dir: Option<PathBuf>,
}

/// Code execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecutionResult {
    /// Session ID
    pub session_id: String,
    
    /// Whether execution was successful
    pub success: bool,
    
    /// Standard output
    pub stdout: String,
    
    /// Standard error
    pub stderr: String,
    
    /// Exit code
    pub exit_code: Option<i32>,
    
    /// Execution time
    pub execution_time: Duration,
    
    /// Memory usage in bytes
    pub memory_usage: Option<u64>,
    
    /// Compilation errors (if any)
    pub compilation_errors: Vec<CompilationError>,
    
    /// Runtime errors (if any)
    pub runtime_errors: Vec<RuntimeError>,
    
    /// Warnings
    pub warnings: Vec<String>,
}

/// Compilation error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationError {
    /// Error message
    pub message: String,
    
    /// Line number (1-based)
    pub line: usize,
    
    /// Column number (1-based)
    pub column: usize,
    
    /// Error code
    pub error_code: Option<String>,
    
    /// Suggested fix
    pub suggestion: Option<String>,
    
    /// Severity level
    pub severity: ErrorSeverity,
}

/// Runtime error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeError {
    /// Error message
    pub message: String,
    
    /// Stack trace
    pub stack_trace: Vec<String>,
    
    /// Error type
    pub error_type: String,
    
    /// Line number where error occurred
    pub line: Option<usize>,
    
    /// Function where error occurred
    pub function: Option<String>,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Fatal,
}

/// API method call request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCallRequest {
    /// Session ID
    pub session_id: String,
    
    /// Method name to call
    pub method_name: String,
    
    /// Method parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Expected return type
    pub expected_return_type: Option<String>,
    
    /// Timeout for the call
    pub timeout: Option<Duration>,
}

/// API method call result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCallResult {
    /// Session ID
    pub session_id: String,
    
    /// Whether call was successful
    pub success: bool,
    
    /// Return value
    pub result: serde_json::Value,
    
    /// Error message if failed
    pub error: Option<String>,
    
    /// Execution time
    pub execution_time: Duration,
    
    /// Method signature that was called
    pub method_signature: String,
    
    /// Parameter validation errors
    pub validation_errors: Vec<String>,
}

/// Syntax highlighting request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxHighlightRequest {
    /// Code to highlight
    pub code: String,
    
    /// Language
    pub language: String,
    
    /// Theme to use
    pub theme: String,
    
    /// Line numbers to highlight
    pub highlight_lines: Vec<usize>,
    
    /// Enable line numbers
    pub show_line_numbers: bool,
}

/// Syntax highlighting result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxHighlightResult {
    /// Highlighted HTML
    pub highlighted_html: String,
    
    /// CSS styles needed
    pub css_styles: String,
    
    /// JavaScript for interactive features
    pub javascript: Option<String>,
    
    /// Detected tokens
    pub tokens: Vec<SyntaxToken>,
}

/// Syntax token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxToken {
    /// Token type
    pub token_type: String,
    
    /// Token value
    pub value: String,
    
    /// Start position
    pub start: usize,
    
    /// End position  
    pub end: usize,
    
    /// Line number
    pub line: usize,
    
    /// Column number
    pub column: usize,
}

/// Interactive documentation manager
pub struct InteractiveDocumentation {
    /// Configuration
    config: InteractiveConfig,
    
    /// Active execution sessions
    active_sessions: HashMap<String, ExecutionSession>,
    
    /// Syntax highlighter
    syntax_highlighter: SyntaxHighlighter,
    
    /// Code playground
    code_playground: CodePlayground,
    
    /// API explorer
    api_explorer: ApiExplorer,
    
    /// Example executor
    example_executor: ExampleExecutor,
}

/// Active execution session
#[derive(Debug, Clone)]
struct ExecutionSession {
    id: String,
    created_at: SystemTime,
    last_activity: SystemTime,
    working_dir: PathBuf,
    environment: HashMap<String, String>,
    execution_count: usize,
}

impl InteractiveDocumentation {
    /// Create a new interactive documentation manager
    #[instrument(skip(config))]
    pub fn new(config: InteractiveConfig) -> Result<(), Error> {
        info!("Creating interactive documentation manager");
        
        // Create temp directory if it doesn't exist
        if !config.temp_dir.exists() {
            std::fs::create_dir_all(&config.temp_dir)
                .map_err(|e| CursedError::Runtime(format!("Failed to create temp directory: {}", e)))?;
        }
        
        let syntax_highlighter = SyntaxHighlighter::new(&config)?;
        let code_playground = CodePlayground::new(&config)?;
        let api_explorer = ApiExplorer::new(&config)?;
        let example_executor = ExampleExecutor::new(&config)?;
        
        Ok(Self {
            config,
            active_sessions: HashMap::new(),
            syntax_highlighter,
            code_playground,
            api_explorer,
            example_executor,
        })
    }
    
    /// Execute code in playground
    #[instrument(skip(self, request))]
    pub async fn execute_code(&mut self, request: CodeExecutionRequest) -> Result<(), Error> {
        info!("Executing code for session: {}", request.session_id);
        
        // Get or create session
        let session = self.get_or_create_session(&request.session_id).await?;
        
        // Execute code
        let result = self.code_playground.execute_code(request, session).await?;
        
        // Update session activity
        if let Some(session) = self.active_sessions.get_mut(&result.session_id) {
            session.last_activity = SystemTime::now();
            session.execution_count += 1;
        }
        
        Ok(result)
    }
    
    /// Call API method
    #[instrument(skip(self, request))]
    pub async fn call_api_method(&mut self, request: ApiCallRequest) -> Result<(), Error> {
        info!("Calling API method '{}' for session: {}", request.method_name, request.session_id);
        
        // Get or create session
        let session = self.get_or_create_session(&request.session_id).await?;
        
        // Call API method
        let result = self.api_explorer.call_method(request, session).await?;
        
        Ok(result)
    }
    
    /// Highlight syntax
    #[instrument(skip(self, request))]
    pub async fn highlight_syntax(&self, request: SyntaxHighlightRequest) -> Result<(), Error> {
        debug!("Highlighting {} lines of {} code", request.code.len(), request.language);
        
        self.syntax_highlighter.highlight(request).await
    }
    
    /// Execute documentation example
    #[instrument(skip(self, example_code, example_id))]
    pub async fn execute_example(
        &mut self,
        example_code: &str,
        example_id: &str,
        session_id: Option<String>,
    ) -> Result<(), Error> {
        let session_id = session_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        
        info!("Executing documentation example '{}' for session: {}", example_id, session_id);
        
        let request = CodeExecutionRequest {
            session_id: session_id.clone(),
            code: example_code.to_string(),
            language: "cursed".to_string(),
            input: None,
            timeout: Some(self.config.max_execution_time),
            args: Vec::new(),
            env: HashMap::new(),
            working_dir: None,
        };
        
        self.example_executor.execute_example(request, example_id).await
    }
    
    /// Get available API methods for explorer
    pub async fn get_available_api_methods(&self) -> Result<(), Error> {
        self.api_explorer.get_available_methods().await
    }
    
    /// Get syntax highlighting themes
    pub fn get_syntax_themes(&self) -> Vec<String> {
        self.config.highlight_themes.clone()
    }
    
    /// Get or create execution session
    async fn get_or_create_session(&mut self, session_id: &str) -> Result<(), Error> {
        if !self.active_sessions.contains_key(session_id) {
            let session_dir = self.config.temp_dir.join(session_id);
            if !session_dir.exists() {
                std::fs::create_dir_all(&session_dir)
                    .map_err(|e| CursedError::Runtime(format!("Failed to create session directory: {}", e)))?;
            }
            
            let session = ExecutionSession {
                id: session_id.to_string(),
                created_at: SystemTime::now(),
                last_activity: SystemTime::now(),
                working_dir: session_dir,
                environment: HashMap::new(),
                execution_count: 0,
            };
            
            self.active_sessions.insert(session_id.to_string(), session);
        }
        
        Ok(self.active_sessions.get_mut(session_id).unwrap())
    }
    
    /// Cleanup old sessions
    pub async fn cleanup_old_sessions(&mut self, max_age: Duration) -> Result<(), Error> {
        let now = SystemTime::now();
        let mut to_remove = Vec::new();
        
        for (session_id, session) in &self.active_sessions {
            if let Ok(age) = now.duration_since(session.last_activity) {
                if age > max_age {
                    to_remove.push(session_id.clone());
                }
            }
        }
        
        for session_id in to_remove {
            if let Some(session) = self.active_sessions.remove(&session_id) {
                // Cleanup session directory
                if session.working_dir.exists() {
                    if let Err(e) = std::fs::remove_dir_all(&session.working_dir) {
                        warn!("Failed to remove session directory {}: {}", session.working_dir.display(), e);
                    }
                }
                info!("Cleaned up session: {}", session_id);
            }
        }
        
        Ok(())
    }
    
    /// Get session statistics
    pub fn get_session_statistics(&self) -> SessionStatistics {
        let now = SystemTime::now();
        let active_count = self.active_sessions.len();
        let total_executions: usize = self.active_sessions.values()
            .map(|s| s.execution_count)
            .sum();
        
        let average_session_age = if active_count > 0 {
            let total_age: u64 = self.active_sessions.values()
                .filter_map(|s| now.duration_since(s.created_at).ok())
                .map(|d| d.as_secs())
                .sum();
            Some(Duration::from_secs(total_age / active_count as u64))
        } else {
            None
        };
        
        SessionStatistics {
            active_sessions: active_count,
            total_executions,
            average_session_age,
            oldest_session: self.active_sessions.values()
                .map(|s| s.created_at)
                .min(),
        }
    }
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatistics {
    pub active_sessions: usize,
    pub total_executions: usize,
    pub average_session_age: Option<Duration>,
    pub oldest_session: Option<SystemTime>,
}

/// API method information for explorer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMethodInfo {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Vec<ApiParameterInfo>,
    pub return_type: String,
    pub is_async: bool,
    pub examples: Vec<String>,
}

/// API parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiParameterInfo {
    pub name: String,
    pub param_type: String,
    pub description: Option<String>,
    pub is_optional: bool,
    pub default_value: Option<serde_json::Value>,
}

/// Syntax highlighter implementation
pub struct SyntaxHighlighter {
    config: InteractiveConfig,
    highlight_rules: HashMap<String, Vec<HighlightRule>>,
}

/// Syntax highlight rule
#[derive(Debug, Clone)]
struct HighlightRule {
    pattern: regex::Regex,
    token_type: String,
    style_class: String,
}

impl SyntaxHighlighter {
    /// Create new syntax highlighter
    pub fn new(config: &InteractiveConfig) -> Result<(), Error> {
        let mut highlight_rules = HashMap::new();
        
        // Initialize CURSED language rules
        let cursed_rules = Self::create_cursed_highlight_rules()?;
        highlight_rules.insert("cursed".to_string(), cursed_rules);
        highlight_rules.insert("csd".to_string(), highlight_rules["cursed"].clone());
        
        Ok(Self {
            config: config.clone(),
            highlight_rules,
        })
    }
    
    /// Create highlight rules for CURSED language
    fn create_cursed_highlight_rules() -> Result<(), Error> {
        let mut rules = Vec::new();
        
        // Keywords (Gen Z slang)
        let keywords = [
            "slay", "yolo", "sus", "facts", "lowkey", "highkey", "periodt",
            "bestie", "flex", "squad", "collab", "vibe_check", "mood", "basic", "stan"
        ];
        let keyword_pattern = format!(r"\b({})\b", keywords.join("|"));
        rules.push(HighlightRule {
            pattern: regex::Regex::new(&keyword_pattern)
                .map_err(|e| CursedError::Runtime(format!("Invalid regex: {}", e)))?,
            token_type: "keyword".to_string(),
            style_class: "keyword".to_string(),
        });
        
        // String literals
        rules.push(HighlightRule {
            pattern: regex::Regex::new(r#""([^"\\]|\\.)*""#)
                .map_err(|e| CursedError::Runtime(format!("Invalid regex: {}", e)))?,
            token_type: "string".to_string(),
            style_class: "string".to_string(),
        });
        
        // Comments
        rules.push(HighlightRule {
            pattern: regex::Regex::new(r"//.*$")
                .map_err(|e| CursedError::Runtime(format!("Invalid regex: {}", e)))?,
            token_type: "comment".to_string(),
            style_class: "comment".to_string(),
        });
        
        // Numbers
        rules.push(HighlightRule {
            pattern: regex::Regex::new(r"\b\d+(\.\d+)?\b")
                .map_err(|e| CursedError::Runtime(format!("Invalid regex: {}", e)))?,
            token_type: "number".to_string(),
            style_class: "number".to_string(),
        });
        
        // Identifiers
        rules.push(HighlightRule {
            pattern: regex::Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b")
                .map_err(|e| CursedError::Runtime(format!("Invalid regex: {}", e)))?,
            token_type: "identifier".to_string(),
            style_class: "identifier".to_string(),
        });
        
        Ok(rules)
    }
    
    /// Highlight code
    pub async fn highlight(&self, request: SyntaxHighlightRequest) -> Result<(), Error> {
        let rules = self.highlight_rules.get(&request.language)
            .ok_or_else(|| CursedError::Runtime(format!("Unsupported language: {}", request.language)))?;
        
        let mut tokens = Vec::new();
        let mut highlighted_html = String::new();
        
        // Process each line
        for (line_num, line) in request.code.split("\n").enumerate() {
            let line_num_1based = line_num + 1;
            
            // Check if this line should be highlighted
            let is_highlighted = request.highlight_lines.contains(&line_num_1based);
            
            if request.show_line_numbers {
                highlighted_html.push_str(&format!(
                    r#"<span class="line-number">{:4}</span>"#, 
                    line_num_1based
                ));
            }
            
            if is_highlighted {
                highlighted_html.push_str(r#"<span class="highlighted-line">"#);
            }
            
            // Apply syntax highlighting to the line
            let (line_tokens, line_html) = self.highlight_line(line, line_num_1based, rules)?;
            tokens.extend(line_tokens);
            highlighted_html.push_str(&line_html);
            
            if is_highlighted {
                highlighted_html.push_str("</span>");
            }
            
            highlighted_html.push('\n');
        }
        
        // Generate CSS styles
        let css_styles = self.generate_css_styles(&request.theme)?;
        
        // Generate JavaScript for interactive features
        let javascript = if self.config.enable_code_folding {
            Some(self.generate_interactive_javascript())
        } else {
            None
        };
        
        Ok(SyntaxHighlightResult {
            highlighted_html,
            css_styles,
            javascript,
            tokens,
        })
    }
    
    /// Highlight a single line
    pub fn highlight_line(
        &self,
        line: &str,
        line_num: usize,
        rules: &[HighlightRule],
    ) -> Result<(), Error> {
        let mut tokens = Vec::new();
        let mut highlighted = String::new();
        let mut processed = 0;
        
        while processed < line.len() {
            let remaining = &line[processed..];
            let mut matched = false;
            
            // Try each highlight rule
            for rule in rules {
                if let Some(mat) = rule.pattern.find(remaining) {
                    if mat.start() == 0 {
                        // Found a match at the beginning
                        let token_value = mat.as_str();
                        let start_pos = processed;
                        let end_pos = processed + mat.end();
                        
                        tokens.push(SyntaxToken {
                            token_type: rule.token_type.clone(),
                            value: token_value.to_string(),
                            start: start_pos,
                            end: end_pos,
                            line: line_num,
                            column: start_pos + 1,
                        });
                        
                        highlighted.push_str(&format!(
                            r#"<span class="{}">{}</span>"#,
                            rule.style_class,
                            html_escape::encode_text(token_value).to_string()
                        ));
                        
                        processed = end_pos;
                        matched = true;
                        break;
                    }
                }
            }
            
            if !matched {
                // No rule matched, add the character as-is
                let ch = line.chars().nth(processed).unwrap_or(' ');
                highlighted.push(ch);
                processed += ch.len_utf8();
            }
        }
        
        Ok((tokens, highlighted))
    }
    
    /// Generate CSS styles for theme
    pub fn generate_css_styles(&self, theme: &str) -> Result<(), Error> {
        let css = match theme {
            "monokai" => include_str!("../../web/assets/themes/monokai.css"),
            "github" => include_str!("../../web/assets/themes/github.css"),
            "solarized-dark" => include_str!("../../web/assets/themes/solarized-dark.css"),
            "solarized-light" => include_str!("../../web/assets/themes/solarized-light.css"),
            "dracula" => include_str!("../../web/assets/themes/dracula.css"),
            _ => include_str!("../../web/assets/themes/monokai.css"), // Default
        };
        
        Ok(css.to_string())
    }
    
    /// Generate JavaScript for interactive features
    pub fn generate_interactive_javascript(&self) -> String {
        r#"
        // Code folding functionality
        function toggleCodeFold(element) {
            const content = element.nextElementSibling;
            const isCollapsed = content.style.display === 'none';
            content.style.display = isCollapsed ? 'block' : 'none';
            element.textContent = isCollapsed ? '[-]' : '[+]';
        }
        
        // Line highlighting
        function highlightLine(lineNumber) {
            const lines = document.querySelectorAll('.line-number');
            lines.forEach(line => line.classList.remove('highlighted'));
            if (lines[lineNumber - 1]) {
                lines[lineNumber - 1].classList.add('highlighted');
            }
        }
        
        // Copy code functionality
        function copyCode(button) {
            const codeBlock = button.parentElement.nextElementSibling;
            const code = codeBlock.textContent;
            navigator.clipboard.writeText(code).then(() => {
                button.textContent = 'Copied!';
                setTimeout(() => button.textContent = 'Copy', 2000);
            });
        }
        "#.to_string()
    }
}

/// Code playground implementation
pub struct CodePlayground {
    config: InteractiveConfig,
}

impl CodePlayground {
    pub fn new(config: &InteractiveConfig) -> Result<(), Error> {
        Ok(Self {
            config: config.clone(),
        })
    }
    
    /// Execute code in sandbox
    pub async fn execute_code(
        &self,
        request: CodeExecutionRequest,
        session: &ExecutionSession,
    ) -> Result<(), Error> {
        let start_time = std::time::Instant::now();
        
        // Write code to temporary file
        let code_file = session.working_dir.join("main.csd");
        std::fs::write(&code_file, &request.code)
            .map_err(|e| CursedError::Runtime(format!("Failed to write code file: {}", e)))?;
        
        // Compile the code
        let compile_result = self.compile_code(&code_file, &session.working_dir).await?;
        if !compile_result.success {
            return Ok(CodeExecutionResult {
                session_id: request.session_id,
                success: false,
                stdout: String::new(),
                stderr: compile_result.stderr,
                exit_code: compile_result.exit_code,
                execution_time: start_time.elapsed(),
                memory_usage: None,
                compilation_errors: compile_result.compilation_errors,
                runtime_errors: Vec::new(),
                warnings: compile_result.warnings,
            });
        }
        
        // Execute the compiled program
        let execution_timeout = request.timeout.unwrap_or(self.config.max_execution_time);
        let exec_result = self.execute_compiled_program(&session.working_dir, &request, execution_timeout).await?;
        
        Ok(CodeExecutionResult {
            session_id: request.session_id,
            success: exec_result.success,
            stdout: exec_result.stdout,
            stderr: exec_result.stderr,
            exit_code: exec_result.exit_code,
            execution_time: start_time.elapsed(),
            memory_usage: exec_result.memory_usage,
            compilation_errors: Vec::new(),
            runtime_errors: exec_result.runtime_errors,
            warnings: Vec::new(),
        })
    }
    
    /// Compile CURSED code
    pub async fn compile_code(
        &self,
        code_file: &Path,
        working_dir: &Path,
    ) -> Result<(), Error> {
        let mut command = Command::new("cursed");
        command
            .arg("compile")
            .arg(code_file)
            .arg("-o")
            .arg("main")
            .current_dir(working_dir);
        
        let output = timeout(
            Duration::from_secs(30),
            command.output()
        ).await
        .map_err(|_| CursedError::Runtime("Compilation timed out".to_string()))?
        .map_err(|e| CursedError::Runtime(format!("Failed to run compiler: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Parse compilation errors
        let compilation_errors = self.parse_compilation_errors(&stderr);
        let warnings = self.parse_compilation_warnings(&stderr);
        
        Ok(CompilationResult {
            success: output.status.success(),
            stdout,
            stderr,
            exit_code: output.status.code(),
            compilation_errors,
            warnings,
        })
    }
    
    /// Execute compiled program
    pub async fn execute_compiled_program(
        &self,
        working_dir: &Path,
        request: &CodeExecutionRequest,
        execution_timeout: Duration,
    ) -> Result<(), Error> {
        let executable = working_dir.join("main");
        
        let mut command = Command::new(&executable);
        command.current_dir(working_dir);
        
        // Add arguments
        for arg in &request.args {
            command.arg(arg);
        }
        
        // Add environment variables
        for (key, value) in &request.env {
            command.env(key, value);
        }
        
        // Handle input
        if let Some(ref _input) = request.input {
            command.stdin(std::process::Stdio::piped());
        }
        
        let output = timeout(
            execution_timeout,
            command.output()
        ).await
        .map_err(|_| CursedError::Runtime("Execution timed out".to_string()))?
        .map_err(|e| CursedError::Runtime(format!("Failed to execute program: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Parse runtime errors
        let runtime_errors = self.parse_runtime_errors(&stderr);
        
        Ok(ExecutionResult {
            success: output.status.success(),
            stdout,
            stderr,
            exit_code: output.status.code(),
            memory_usage: None, // Would need system monitoring
            runtime_errors,
        })
    }
    
    /// Parse compilation errors from stderr
    pub fn parse_compilation_errors(&self, stderr: &str) -> Vec<CompilationError> {
        let mut errors = Vec::new();
        
        for line in stderr.split("\n") {
            if line.contains("error:") {
                // Parse error line: "file.csd:10:5: error: message"
                if let Some(error) = self.parse_error_line(line, ErrorSeverity::Error) {
                    errors.push(error);
                }
            }
        }
        
        errors
    }
    
    /// Parse compilation warnings from stderr
    pub fn parse_compilation_warnings(&self, stderr: &str) -> Vec<String> {
        let mut warnings = Vec::new();
        
        for line in stderr.split("\n") {
            if line.contains("warning:") {
                warnings.push(line.to_string());
            }
        }
        
        warnings
    }
    
    /// Parse runtime errors from stderr
    pub fn parse_runtime_errors(&self, stderr: &str) -> Vec<RuntimeError> {
        let mut errors = Vec::new();
        
        for line in stderr.split("\n") {
            if line.contains("runtime error:") || line.contains("panic:") {
                errors.push(RuntimeError {
                    message: line.to_string(),
                    stack_trace: Vec::new(), // Would parse stack trace
                    error_type: "RuntimeError".to_string(),
                    line: None,
                    function: None,
                });
            }
        }
        
        errors
    }
    
    /// Parse a single error line
    fn parse_error_line(&self, line: &str, severity: ErrorSeverity) -> Option<CompilationError> {
        // Simple parsing - would be more sophisticated in practice
        if let Some(colon_pos) = line.find(':') {
            let message = line[colon_pos + 1..].trim().to_string();
            
            Some(CompilationError {
                message,
                line: 1, // Would parse actual line number
                column: 1, // Would parse actual column number
                error_code: None,
                suggestion: None,
                severity,
            })
        } else {
            None
        }
    }
}

/// Compilation result
#[derive(Debug)]
struct CompilationResult {
    success: bool,
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
    compilation_errors: Vec<CompilationError>,
    warnings: Vec<String>,
}

/// Execution result
#[derive(Debug)]
struct ExecutionResult {
    success: bool,
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
    memory_usage: Option<u64>,
    runtime_errors: Vec<RuntimeError>,
}

/// API explorer implementation
pub struct ApiExplorer {
    config: InteractiveConfig,
    available_methods: HashMap<String, ApiMethodInfo>,
}

impl ApiExplorer {
    pub fn new(config: &InteractiveConfig) -> Result<(), Error> {
        let mut available_methods = HashMap::new();
        
        // Initialize with some built-in API methods
        available_methods.insert("print".to_string(), ApiMethodInfo {
            name: "print".to_string(),
            description: Some("Print a message to stdout".to_string()),
            parameters: vec![ApiParameterInfo {
                name: "message".to_string(),
                param_type: "string".to_string(),
                description: Some("Message to print".to_string()),
                is_optional: false,
                default_value: None,
            }],
            return_type: "void".to_string(),
            is_async: false,
            examples: vec!["print(\"Hello, World!\")".to_string()],
        });
        
        Ok(Self {
            config: config.clone(),
            available_methods,
        })
    }
    
    /// Call an API method
    pub async fn call_method(
        &self,
        request: ApiCallRequest,
        _session: &ExecutionSession,
    ) -> Result<(), Error> {
        let start_time = std::time::Instant::now();
        
        // Look up method
        let method_info = self.available_methods.get(&request.method_name)
            .ok_or_else(|| CursedError::Runtime(format!("Unknown method: {}", request.method_name)))?;
        
        // Validate parameters
        let validation_errors = self.validate_parameters(&request.parameters, &method_info.parameters);
        if !validation_errors.is_empty() {
            return Ok(ApiCallResult {
                session_id: request.session_id,
                success: false,
                result: serde_json::Value::Null,
                error: Some("Parameter validation failed".to_string()),
                execution_time: start_time.elapsed(),
                method_signature: self.format_method_signature(method_info),
                validation_errors,
            });
        }
        
        // Execute method (mock implementation)
        let result = self.execute_method(&request.method_name, &request.parameters).await?;
        
        Ok(ApiCallResult {
            session_id: request.session_id,
            success: true,
            result,
            error: None,
            execution_time: start_time.elapsed(),
            method_signature: self.format_method_signature(method_info),
            validation_errors: Vec::new(),
        })
    }
    
    /// Get available methods
    pub async fn get_available_methods(&self) -> Result<(), Error> {
        Ok(self.available_methods.values().cloned().collect())
    }
    
    /// Validate method parameters
    pub fn validate_parameters(
        &self,
        provided: &HashMap<String, serde_json::Value>,
        expected: &[ApiParameterInfo],
    ) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Check required parameters
        for param in expected {
            if !param.is_optional && !provided.contains_key(&param.to_string()) {
                errors.push(format!("Missing required parameter: {}", param.to_string()));
            }
        }
        
        // Check parameter types (simplified)
        for (name, value) in provided {
            if let Some(param) = expected.iter().find(|p| p.to_string() == *name) {
                if !self.is_value_compatible_with_type(value, &param.param_type) {
                    errors.push(format!("Parameter '{}' has wrong type", name));
                }
            } else {
                errors.push(format!("Unknown parameter: {}", name));
            }
        }
        
        errors
    }
    
    /// Check if value is compatible with expected type
    fn is_value_compatible_with_type(&self, value: &serde_json::Value, expected_type: &str) -> bool {
        match expected_type {
            "string" => value.is_string(),
            "number" => value.is_number(),
            "boolean" => value.is_boolean(),
            "array" => value.is_array(),
            "object" => value.is_object(),
            _ => true, // Allow any type for unknown types
        }
    }
    
    /// Execute a method (mock implementation)
    pub async fn execute_method(
        &self,
        method_name: &str,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<(), Error> {
        match method_name {
            "print" => {
                let message = parameters.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Hello, World!");
                
                info!("API call - print: {}", message);
                Ok(serde_json::json!({
                    "printed": message,
                    "success": true
                }))
            }
            _ => {
                Err(CursedError::Runtime(format!("Method not implemented: {}", method_name)))
            }
        }
    }
    
    /// Format method signature for display
    pub fn format_method_signature(&self, method_info: &ApiMethodInfo) -> String {
        let params: Vec<String> = method_info.parameters.iter().map(|p| {
            if p.is_optional {
                format!("{}?: {}", p.to_string(), p.param_type)
            } else {
                format!("{}: {}", p.to_string(), p.param_type)
            }
        }).collect();
        
        format!("{}({}) -> {}", method_info.to_string(), params.join(", "), method_info.return_type)
    }
}

/// Example executor implementation
pub struct ExampleExecutor {
    config: InteractiveConfig,
}

impl ExampleExecutor {
    pub fn new(config: &InteractiveConfig) -> Result<(), Error> {
        Ok(Self {
            config: config.clone(),
        })
    }
    
    /// Execute a documentation example
    pub async fn execute_example(
        &self,
        request: CodeExecutionRequest,
        example_id: &str,
    ) -> Result<(), Error> {
        info!("Executing example: {}", example_id);
        
        // Create a temporary playground for the example
        let playground = CodePlayground::new(&self.config)?;
        
        // Create a temporary session for the example
        let session_dir = self.config.temp_dir.join(format!("example_{}", example_id));
        if !session_dir.exists() {
            std::fs::create_dir_all(&session_dir)
                .map_err(|e| CursedError::Runtime(format!("Failed to create example directory: {}", e)))?;
        }
        
        let session = ExecutionSession {
            id: format!("example_{}", example_id),
            created_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            working_dir: session_dir,
            environment: HashMap::new(),
            execution_count: 0,
        };
        
        // Execute the example
        let result = playground.execute_code(request, &session).await?;
        
        // Cleanup example directory
        if session.working_dir.exists() {
            let _ = std::fs::remove_dir_all(&session.working_dir);
        }
        
        Ok(result)
    }
}
