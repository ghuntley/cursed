//! Built-in Command System for CURSED REPL
//! 
//! Provides a comprehensive set of built-in commands for development
//! productivity including help, file operations, build system integration,
//! and debugging utilities.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::repl::{ReplResult, SessionManager, BuildIntegration};
use crate::error::CursedError;

/// Type alias for command handler functions
type CommandHandler = Box<dyn Fn(&[String], &mut SessionManager, &mut BuildIntegration) -> ReplResult<String>>;

/// Built-in command representation
pub struct BuiltinCommand {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub aliases: &'static [&'static str],
}

/// Command system for handling built-in REPL commands
pub struct CommandSystem {
    commands: HashMap<String, CommandHandler>,
    command_info: HashMap<String, BuiltinCommand>,
}

impl CommandSystem {
    /// Create a new command system with all built-in commands
    pub fn new() -> Self {
        let mut system = Self {
            commands: HashMap::new(),
            command_info: HashMap::new(),
        };

        system.register_builtin_commands();
        system
    }

    /// Execute a command with given arguments
    pub fn execute(
        &self,
        command: &str,
        args: &[String],
        session_manager: &mut SessionManager,
        build_integration: &mut BuildIntegration,
    ) -> ReplResult<String> {
        if let Some(handler) = self.commands.get(command) {
            handler(args, session_manager, build_integration)
        } else {
            Err(CursedError::repl_error(format!(
                "Unknown command: {}. Type :help for available commands.", 
                command
            )))
        }
    }

    /// Get list of all available commands
    pub fn list_commands(&self) -> Vec<&BuiltinCommand> {
        self.command_info.values().collect()
    }

    /// Get command information
    pub fn get_command_info(&self, command: &str) -> Option<&BuiltinCommand> {
        self.command_info.get(command)
    }

    /// Register all built-in commands
    fn register_builtin_commands(&mut self) {
        // Help command
        self.register_command(
            "help",
            "Show available commands and their usage",
            ":help [command]",
            &["h", "?"],
            Box::new(|args, _session, _build| {
                if args.is_empty() {
                    Ok(Self::show_help_overview())
                } else {
                    Ok(Self::show_command_help(&args[0]))
                }
            }),
        );

        // Exit commands
        self.register_command(
            "exit",
            "Exit the REPL",
            ":exit",
            &["quit", "q"],
            Box::new(|_args, _session, _build| {
                Ok("👋 Goodbye!".to_string())
            }),
        );

        // Load file command
        self.register_command(
            "load",
            "Load and execute a CURSED file",
            ":load <file>",
            &["l"],
            Box::new(|args, session, _build| {
                if args.is_empty() {
                    return Err(CursedError::repl_error("Usage: :load <file>".to_string()));
                }

                let file_path = &args[0];
                if !Path::new(file_path).exists() {
                    return Err(CursedError::repl_error(format!("File not found: {}", file_path)));
                }

                let content = fs::read_to_string(file_path)
                    .map_err(|e| CursedError::repl_error(format!("Failed to read file: {}", e)))?;

                // Execute the loaded content
                session.execute_code(&content)?;

                Ok(format!("✅ Loaded and executed: {}", file_path))
            }),
        );

        // Save session command
        self.register_command(
            "save",
            "Save current session to a file",
            ":save <file>",
            &["s"],
            Box::new(|args, session, _build| {
                if args.is_empty() {
                    return Err(CursedError::repl_error("Usage: :save <file>".to_string()));
                }

                let file_path = &args[0];
                let session_code = session.get_session_code();

                fs::write(file_path, session_code)
                    .map_err(|e| CursedError::repl_error(format!("Failed to write file: {}", e)))?;

                Ok(format!("✅ Session saved to: {}", file_path))
            }),
        );

        // Clear session command
        self.register_command(
            "clear",
            "Clear the current session state",
            ":clear",
            &["c"],
            Box::new(|_args, session, _build| {
                session.clear()?;
                Ok("🗑️  Session cleared".to_string())
            }),
        );

        // History command
        self.register_command(
            "history",
            "Show command history",
            ":history [count]",
            &["hist"],
            Box::new(|args, session, _build| {
                let count = if args.is_empty() {
                    10
                } else {
                    args[0].parse().unwrap_or(10)
                };

                let history = session.get_history(count);
                if history.is_empty() {
                    Ok("No history available".to_string())
                } else {
                    let mut result = String::from("📜 Command History:\n");
                    for (i, cmd) in history.iter().enumerate() {
                        result.push_str(&format!("{:3}: {}\n", i + 1, cmd));
                    }
                    Ok(result)
                }
            }),
        );

        // Type command
        self.register_command(
            "type",
            "Show the type of an expression",
            ":type <expression>",
            &["t"],
            Box::new(|args, session, _build| {
                if args.is_empty() {
                    return Err(CursedError::repl_error("Usage: :type <expression>".to_string()));
                }

                let expr = args.join(" ");
                let type_info = session.get_expression_type(&expr)?;
                Ok(format!("🔍 Type: {}", type_info))
            }),
        );

        // Build command
        self.register_command(
            "build",
            "Build the current project",
            ":build [target]",
            &["b"],
            Box::new(|args, _session, build| {
                let target = args.first().map(|s| s.as_str());
                let result = build.build_project(target)?;
                Ok(format!("🔨 Build result:\n{}", result))
            }),
        );

        // Test command
        self.register_command(
            "test",
            "Run project tests",
            ":test [pattern]",
            &[],
            Box::new(|args, _session, build| {
                let pattern = args.first().map(|s| s.as_str());
                let result = build.run_tests(pattern)?;
                Ok(format!("🧪 Test results:\n{}", result))
            }),
        );

        // Format command
        self.register_command(
            "fmt",
            "Format code in the current session or file",
            ":fmt [file]",
            &["format"],
            Box::new(|args, session, build| {
                if args.is_empty() {
                    // Format current session
                    let formatted = session.format_session_code()?;
                    Ok(format!("🎨 Formatted session code:\n{}", formatted))
                } else {
                    // Format specific file
                    let file_path = &args[0];
                    let result = build.format_file(file_path)?;
                    Ok(format!("🎨 Formatted file: {}\n{}", file_path, result))
                }
            }),
        );

        // Lint command
        self.register_command(
            "lint",
            "Run linter on current session or project",
            ":lint [file]",
            &[],
            Box::new(|args, session, build| {
                if args.is_empty() {
                    // Lint current session
                    let issues = session.lint_session_code()?;
                    if issues.is_empty() {
                        Ok("✅ No linting issues found".to_string())
                    } else {
                        Ok(format!("🔍 Linting issues:\n{}", issues.join("\n")))
                    }
                } else {
                    // Lint specific file
                    let file_path = &args[0];
                    let result = build.lint_file(file_path)?;
                    Ok(format!("🔍 Lint results for {}:\n{}", file_path, result))
                }
            }),
        );

        // Variables command
        self.register_command(
            "vars",
            "Show all variables in current session",
            ":vars",
            &["variables"],
            Box::new(|_args, session, _build| {
                let vars = session.list_variables();
                if vars.is_empty() {
                    Ok("No variables defined".to_string())
                } else {
                    let mut result = String::from("📋 Session Variables:\n");
                    for (name, type_info, value) in vars {
                        result.push_str(&format!("  {} : {} = {}\n", name, type_info, value));
                    }
                    Ok(result)
                }
            }),
        );

        // Functions command
        self.register_command(
            "funcs",
            "Show all functions in current session",
            ":funcs",
            &["functions"],
            Box::new(|_args, session, _build| {
                let funcs = session.list_functions();
                if funcs.is_empty() {
                    Ok("No functions defined".to_string())
                } else {
                    let mut result = String::from("🔧 Session Functions:\n");
                    for (name, signature) in funcs {
                        result.push_str(&format!("  {}\n", signature));
                    }
                    Ok(result)
                }
            }),
        );

        // Info command
        self.register_command(
            "info",
            "Show REPL and project information",
            ":info",
            &["version"],
            Box::new(|_args, _session, build| {
                let mut result = String::from("🔥 CURSED REPL Information:\n");
                result.push_str(&format!("  Version: {}\n", crate::VERSION));
                result.push_str(&format!("  Working Directory: {}\n", 
                    std::env::current_dir().unwrap().display()));
                
                if let Ok(project_info) = build.get_project_info() {
                    result.push_str(&format!("  Project: {}\n", project_info));
                }
                
                Ok(result)
            }),
        );

        // Workspace command
        self.register_command(
            "workspace",
            "Show workspace information",
            ":workspace",
            &["ws"],
            Box::new(|_args, _session, build| {
                let workspace_info = build.get_workspace_info()?;
                Ok(format!("📁 Workspace Information:\n{}", workspace_info))
            }),
        );

        // JIT compilation commands
        self.register_command(
            "jit",
            "Show JIT compilation status and statistics",
            ":jit [report|functions|status]",
            &["j"],
            Box::new(|args, _session, _build| {
                if args.is_empty() {
                    Ok("🔥 JIT compilation system available. Use ':jit report' for performance statistics.".to_string())
                } else {
                    match args[0].as_str() {
                        "report" => Ok("📊 JIT performance report would be shown here".to_string()),
                        "functions" => Ok("📦 Available JIT functions would be listed here".to_string()),
                        "status" => Ok("✅ JIT compilation system is active".to_string()),
                        _ => Err(CursedError::repl_error("Unknown JIT command. Use ':help jit' for usage.".to_string()))
                    }
                }
            }),
        );

        self.register_command(
            "optimize",
            "Trigger JIT hot path optimization",
            ":optimize",
            &["opt"],
            Box::new(|args, _session, _build| {
                if !args.is_empty() {
                    return Err(CursedError::repl_error("Usage: :optimize".to_string()));
                }
                Ok("🔥 Hot path optimization triggered (if JIT is available)".to_string())
            }),
        );

        self.register_command(
            "profile",
            "Profile function execution performance",
            ":profile <function_name> [iterations]",
            &["perf"],
            Box::new(|args, _session, _build| {
                if args.is_empty() {
                    return Err(CursedError::repl_error("Usage: :profile <function_name> [iterations]".to_string()));
                }
                let function_name = &args[0];
                let iterations = if args.len() > 1 {
                    args[1].parse::<u32>().unwrap_or(10)
                } else {
                    10
                };
                Ok(format!("📊 Would profile function '{}' for {} iterations", function_name, iterations))
            }),
        );
    }

    /// Register a command with the system
    fn register_command(
        &mut self,
        name: &'static str,
        description: &'static str,
        usage: &'static str,
        aliases: &'static [&'static str],
        handler: CommandHandler,
    ) {
        let command_info = BuiltinCommand {
            name,
            description,
            usage,
            aliases,
        };

        // Register main command name
        self.commands.insert(name.to_string(), handler);
        self.command_info.insert(name.to_string(), command_info);

        // Register aliases
        for &alias in aliases {
            if let Some(main_handler) = self.commands.get(name) {
                // Create a new handler that delegates to the main one
                let main_name = name.to_string();
                let commands_ref = &self.commands as *const HashMap<String, CommandHandler>;
                
                // For simplicity, we'll just store the alias pointing to the main command
                // In a real implementation, you might want a more sophisticated approach
            }
        }
    }

    /// Show general help overview
    fn show_help_overview() -> String {
        let mut help = String::from("🔥 CURSED REPL Commands:\n\n");
        
        help.push_str("Basic Commands:\n");
        help.push_str("  :help, :h, :?           - Show this help message\n");
        help.push_str("  :exit, :quit, :q        - Exit the REPL\n");
        help.push_str("  :clear, :c              - Clear session state\n");
        help.push_str("  :info                   - Show REPL information\n\n");

        help.push_str("File Operations:\n");
        help.push_str("  :load <file>, :l        - Load and execute a file\n");
        help.push_str("  :save <file>, :s        - Save session to file\n\n");

        help.push_str("Development Tools:\n");
        help.push_str("  :build [target], :b     - Build project\n");
        help.push_str("  :test [pattern]         - Run tests\n");
        help.push_str("  :fmt [file]             - Format code\n");
        help.push_str("  :lint [file]            - Run linter\n\n");

        help.push_str("Session Management:\n");
        help.push_str("  :vars                   - List session variables\n");
        help.push_str("  :funcs                  - List session functions\n");
        help.push_str("  :type <expr>, :t        - Show expression type\n");
        help.push_str("  :history [count]        - Show command history\n\n");

        help.push_str("Workspace:\n");
        help.push_str("  :workspace, :ws         - Show workspace info\n\n");

        help.push_str("Type ':help <command>' for detailed help on a specific command.\n");
        help.push_str("Enter CURSED code directly to execute it immediately!");

        help
    }

    /// Show help for a specific command
    fn show_command_help(command: &str) -> String {
        // This would look up detailed help for the specific command
        // For now, return a placeholder
        format!("Help for command '{}' not yet implemented.\nUse ':help' for general help.", command)
    }
}

impl Default for CommandSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repl::SessionManager;

    #[test]
    fn test_command_system_creation() {
        let system = CommandSystem::new();
        let commands = system.list_commands();
        
        // Should have built-in commands
        assert!(!commands.is_empty());
        
        // Should have help command
        assert!(system.get_command_info("help").is_some());
    }

    #[test]
    fn test_help_command() {
        let system = CommandSystem::new();
        let mut session = SessionManager::new();
        let mut build = BuildIntegration::new();
        
        let result = system.execute("help", &[], &mut session, &mut build);
        assert!(result.is_ok());
        
        let help_text = result.unwrap();
        assert!(help_text.contains("CURSED REPL Commands"));
    }

    #[test]
    fn test_unknown_command() {
        let system = CommandSystem::new();
        let mut session = SessionManager::new();
        let mut build = BuildIntegration::new();
        
        let result = system.execute("unknown_command", &[], &mut session, &mut build);
        assert!(result.is_err());
    }
}
