use crate::error::Error;
//! Main CURSED REPL Implementation
//! 
//! Provides the core interactive Read-Eval-Print Loop functionality
//! with comprehensive features for development productivity.

use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use rustyline::{Editor, Result as RustylineResult, DefaultEditor};
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;

use crate::repl::{
    ReplConfig, ReplState, InputType, ReplOutput, ReplResult,
    SyntaxHighlighter, CommandSystem, SessionManager, TabCompletion,
    MultiLineEditor, BuildIntegration, ReplEvaluator
};

use crate::error::CursedError;

/// Main CURSED REPL structure
pub struct CursedRepl {
    config: ReplConfig,
    state: ReplState,
    editor: DefaultEditor,
    syntax_highlighter: SyntaxHighlighter,
    command_system: CommandSystem,
    session_manager: SessionManager,
    multi_line_editor: MultiLineEditor,
    build_integration: BuildIntegration,
    evaluator: ReplEvaluator,
    current_input: String,
    line_number: usize,
}

impl CursedRepl {
    /// Create a new CURSED REPL instance
    pub fn new() -> Self {
        let config = ReplConfig::default();
        let mut editor = DefaultEditor::new().expect("Failed to create line editor");
        
        // Configure editor
        editor.set_auto_add_history(true);
        editor.set_tab_stop(4);
        
        // Tab completion setup will be added later
        // let tab_completion = TabCompletion::new();
        // editor.set_helper(Some(tab_completion));

        let mut evaluator = ReplEvaluator::new().expect("Failed to create evaluator");
        
        // Try to initialize LLVM code generation
        if let Err(e) = evaluator.initialize_codegen() {
            eprintln!("Warning: Could not initialize LLVM codegen: {}", e);
        }

        Self {
            config,
            state: ReplState::Interactive,
            editor,
            syntax_highlighter: SyntaxHighlighter::new(),
            command_system: CommandSystem::new(),
            session_manager: SessionManager::new(),
            multi_line_editor: MultiLineEditor::new(),
            build_integration: BuildIntegration::new(),
            evaluator,
            current_input: String::new(),
            line_number: 1,
        }
    }

    /// Configure REPL with verbose output
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.config.verbose = verbose;
        self
    }

    /// Configure REPL with history support
    pub fn with_history(mut self, enable: bool) -> Self {
        self.config.enable_history = enable;
        self
    }

    /// Configure REPL with syntax highlighting
    pub fn with_syntax_highlighting(mut self, enable: bool) -> Self {
        self.config.enable_syntax_highlighting = enable;
        self
    }

    /// Configure REPL timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set working directory for project context
    pub fn with_working_directory(mut self, dir: &str) -> ReplResult<Self> {
        let path = PathBuf::from(dir);
        if !path.exists() {
            return Err(CursedError::repl_error(format!("Directory does not exist: {}", dir)));
        }
        
        self.config.working_directory = Some(path.clone());
        self.build_integration.set_working_directory(path)?;
        Ok(self)
    }

    /// Load and execute a CURSED file
    pub fn load_file(&mut self, file_path: &str) -> ReplResult<()> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| CursedError::repl_error(format!("Failed to read file {}: {}", file_path, e)))?;

        if self.config.verbose {
            println!("📁 Loading file: {}", file_path);
        }

        self.execute_code(&content)?;
        
        if self.config.verbose {
            println!("✅ File loaded successfully");
        }

        Ok(())
    }

    /// Start the interactive REPL loop
    pub fn run(&mut self) -> ReplResult<()> {
        self.setup()?;
        self.print_welcome();
        
        loop {
            match self.state {
                ReplState::Interactive => {
                    if let Err(e) = self.handle_interactive_input() {
                        if self.config.verbose {
                            eprintln!("🔥 Error: {}", e);
                        } else {
                            eprintln!("Error: {}", e);
                        }
                        self.state = ReplState::Error(e.to_string());
                    }
                }
                ReplState::MultiLine => {
                    if let Err(e) = self.handle_multiline_input() {
                        eprintln!("Error: {}", e);
                        self.state = ReplState::Interactive;
                        self.current_input.clear();
                    }
                }
                ReplState::Command => {
                    // Commands are handled in interactive mode
                    self.state = ReplState::Interactive;
                }
                ReplState::Exiting => {
                    self.cleanup()?;
                    break;
                }
                ReplState::Error(ref _msg) => {
                    // Reset to interactive mode after error
                    self.state = ReplState::Interactive;
                    self.current_input.clear();
                }
            }
        }

        Ok(())
    }

    /// Setup REPL environment
    fn setup(&mut self) -> ReplResult<()> {
        // Initialize history
        if self.config.enable_history {
            if let Some(history_file) = &self.config.history_file {
                if let Err(e) = self.editor.load_history(history_file) {
                    if self.config.verbose {
                        eprintln!("Warning: Could not load history: {}", e);
                    }
                }
            }
        }

        // Initialize session
        self.session_manager.initialize()?;

        // Setup build integration if in project directory
        if let Some(ref working_dir) = self.config.working_directory {
            self.build_integration.scan_project(working_dir)?;
        }

        Ok(())
    }

    /// Print welcome message
    fn print_welcome(&self) {
        println!("🔥 CURSED REPL v{}", crate::VERSION);
        println!("Welcome to the most fire programming language! 🚀");
        println!("Type :help for available commands or :exit to quit");
        
        if let Some(ref dir) = self.config.working_directory {
            println!("📁 Working directory: {}", dir.display());
        }
        
        println!();
    }

    /// Handle interactive input
    fn handle_interactive_input(&mut self) -> ReplResult<()> {
        let prompt = if self.current_input.is_empty() {
            &self.config.prompt
        } else {
            &self.config.continuation_prompt
        };

        let readline = self.editor.readline(prompt);
        
        match readline {
            Ok(line) => {
                let input = self.parse_input(&line);
                self.handle_input(input)?;
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                self.current_input.clear();
                self.state = ReplState::Interactive;
            }
            Err(ReadlineError::Eof) => {
                println!();
                self.state = ReplState::Exiting;
            }
            Err(err) => {
                return Err(CursedError::repl_error(format!("Input error: {}", err)));
            }
        }

        Ok(())
    }

    /// Handle multi-line input
    fn handle_multiline_input(&mut self) -> ReplResult<()> {
        let line = self.multi_line_editor.read_line(&self.config.continuation_prompt)?;
        
        if self.multi_line_editor.is_complete(&self.current_input, &line) {
            self.current_input.push_str(&line);
            let input = InputType::Code(self.current_input.clone());
            self.handle_input(input)?;
            self.current_input.clear();
            self.state = ReplState::Interactive;
        } else {
            self.current_input.push_str(&line);
            self.current_input.push('\n');
        }

        Ok(())
    }

    /// Parse input string into InputType
    fn parse_input(&self, line: &str) -> InputType {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            return InputType::Empty;
        }

        if trimmed.starts_with(':') {
            // Built-in command
            let parts: Vec<&str> = trimmed[1..].split_whitespace().collect();
            if parts.is_empty() {
                return InputType::Empty;
            }
            
            let command = parts[0].to_string();
            let args = parts[1..].iter().map(|s| s.to_string()).collect();
            return InputType::Command(command, args);
        }

        // CURSED code
        InputType::Code(line.to_string())
    }

    /// Handle parsed input
    fn handle_input(&mut self, input: InputType) -> ReplResult<()> {
        match input {
            InputType::Empty => {
                // Just print a new prompt
                Ok(())
            }
            InputType::EndOfInput => {
                self.state = ReplState::Exiting;
                Ok(())
            }
            InputType::Command(cmd, args) => {
                self.handle_command(&cmd, &args)
            }
            InputType::Code(code) => {
                // Check if we need multi-line input
                if self.multi_line_editor.needs_continuation(&code) {
                    self.current_input = code;
                    self.state = ReplState::MultiLine;
                    Ok(())
                } else {
                    self.execute_code(&code)
                }
            }
        }
    }

    /// Handle built-in commands
    fn handle_command(&mut self, command: &str, args: &[String]) -> ReplResult<()> {
        let start_time = Instant::now();
        
        let result = self.command_system.execute(command, args, &mut self.session_manager, &mut self.build_integration);
        
        let execution_time = start_time.elapsed();
        
        match result {
            Ok(output) => {
                let formatted_output = if self.config.verbose {
                    ReplOutput::success(output).with_timing(execution_time)
                } else {
                    ReplOutput::success(output)
                };
                
                self.print_output(&formatted_output);
                
                // Handle special commands
                if command == "exit" || command == "quit" {
                    self.state = ReplState::Exiting;
                }
                
                Ok(())
            }
            Err(e) => {
                let error_output = ReplOutput::error(e.to_string()).with_timing(execution_time);
                self.print_output(&error_output);
                Ok(())
            }
        }
    }

    /// Execute CURSED code
    fn execute_code(&mut self, code: &str) -> ReplResult<()> {
        // Apply syntax highlighting if enabled
        let highlighted_code = if self.config.enable_syntax_highlighting {
            self.syntax_highlighter.highlight(code)
        } else {
            code.to_string()
        };

        if self.config.verbose {
            println!("🔥 Executing: {}", highlighted_code);
        }

        // Use the evaluator to parse and execute the code
        match self.evaluator.evaluate(code, &mut self.session_manager) {
            Ok(output) => {
                self.print_output(&output);
                
                // Update tab completion with new variables and functions
                self.update_tab_completion();
                
                self.line_number += 1;
                Ok(())
            }
            Err(e) => {
                let error_output = ReplOutput::error(e.to_string());
                self.print_output(&error_output);
                Ok(())
            }
        }
    }

    /// Update tab completion with current session state
    fn update_tab_completion(&mut self) {
        // Get variables and functions from evaluator
        let variables: Vec<String> = self.evaluator.get_variables()
            .into_iter()
            .map(|(name, _, _)| name)
            .collect();
        
        let functions: Vec<String> = self.evaluator.get_functions()
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        
        // Update tab completion (when implemented)
        // self.tab_completion.update_variables(variables);
        // self.tab_completion.update_functions(functions);
    }

    /// Print formatted output
    fn print_output(&self, output: &ReplOutput) {
        if output.is_error {
            if self.config.verbose {
                print!("🔥 ");
            }
            print!("Error: {}", output.content);
        } else {
            print!("{}", output.content);
        }

        if let Some(duration) = output.execution_time {
            if self.config.verbose {
                print!(" ({}ms)", duration.as_millis());
            }
        }

        println!();
    }

    /// Cleanup REPL resources
    fn cleanup(&mut self) -> ReplResult<()> {
        // Save history
        if self.config.enable_history {
            if let Some(history_file) = &self.config.history_file {
                if let Err(e) = self.editor.save_history(history_file) {
                    if self.config.verbose {
                        eprintln!("Warning: Could not save history: {}", e);
                    }
                }
            }
        }

        // Cleanup session
        self.session_manager.cleanup()?;

        if self.config.verbose {
            println!("👋 Thanks for using CURSED! Keep it fire! 🔥");
        } else {
            println!("Goodbye!");
        }

        Ok(())
    }
}

impl Default for CursedRepl {
    fn default() -> Self {
        Self::new()
    }
}
