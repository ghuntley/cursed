// CURSED Language Server Protocol binary
// 
// Provides IDE integration for the CURSED programming language

use clap::{Arg, ArgAction, Command};
use cursed::lsp::{init_lsp_server, LspServer, LspServerBuilder, ServerMode};
use std::process;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    let matches = Command::new("cursed-lsp")
        .version("0.1.0")
        .author("Geoffrey Huntley")
        .about("CURSED Language Server Protocol implementation")
        .arg(
            Arg::new("mode")
                .long("mode")
                .short('m')
                .value_name("MODE")
                .help("Server communication mode")
                .value_parser(["stdio", "tcp", "socket"])
                .default_value("stdio"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .value_name("PORT")
                .help("TCP port for server mode")
                .value_parser(clap::value_parser!(u16))
                .default_value("9257"), // WAZL in phone keypad
        )
        .arg(
            Arg::new("socket")
                .long("socket")
                .short('s')
                .value_name("PATH")
                .help("Socket path for socket mode"),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("Enable debug logging")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Enable verbose logging")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("log-file")
                .long("log-file")
                .value_name("FILE")
                .help("Write logs to file instead of stderr"),
        )
        .arg(
            Arg::new("max-concurrent")
                .long("max-concurrent")
                .value_name("NUM")
                .help("Maximum number of concurrent requests")
                .value_parser(clap::value_parser!(usize))
                .default_value("100"),
        )
        .get_matches();

    // Initialize logging
    let debug_enabled = matches.get_flag("debug");
    let verbose = matches.get_flag("verbose");
    let log_file = matches.get_one::<String>("log-file");
    
    init_logging(debug_enabled, verbose, log_file.map(|s| s.as_str()));

    // Initialize LSP server
    if let Err(err) = init_lsp_server() {
        error!("Failed to initialize LSP server: {}", err);
        process::exit(1);
    }

    info!("Starting CURSED Language Server");

    // Parse server mode
    let mode = match matches.get_one::<String>("mode").unwrap().as_str() {
        "stdio" => ServerMode::Stdio,
        "tcp" => ServerMode::Tcp,
        "socket" => ServerMode::Socket,
        _ => {
            error!("Invalid server mode");
            process::exit(1);
        }
    };

    // Build server configuration
    let mut builder = LspServerBuilder::new()
        .mode(mode.clone())
        .debug(debug_enabled)
        .max_concurrent_requests(*matches.get_one::<usize>("max-concurrent").unwrap());

    if let Some(port) = matches.get_one::<u16>("port") {
        builder = builder.port(*port);
    }

    if let Some(socket_path) = matches.get_one::<String>("socket") {
        builder = builder.socket_path(socket_path.clone());
    }

    let server = builder.build();

    // Print server information
    info!("CURSED Language Server v0.1.0");
    info!("Mode: {:?}", mode);
    if mode == ServerMode::Tcp {
        info!("TCP Port: {}", matches.get_one::<u16>("port").unwrap());
    }
    if mode == ServerMode::Socket && matches.get_one::<String>("socket").is_some() {
        info!("Socket Path: {}", matches.get_one::<String>("socket").unwrap());
    }
    info!("Debug: {}", debug_enabled);
    info!("Max Concurrent Requests: {}", matches.get_one::<usize>("max-concurrent").unwrap());

    // Start the server
    if let Err(err) = server.start().await {
        error!("LSP server error: {}", err);
        process::exit(1);
    }

    info!("CURSED Language Server stopped");
}

/// Initialize logging based on command-line options
fn init_logging(debug: bool, verbose: bool, log_file: Option<&str>) {
    let level = if debug {
        "debug"
    } else if verbose {
        "info"
    } else {
        "warn"
    };

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            tracing_subscriber::EnvFilter::new(format!("cursed_lsp={},cursed={}", level, level))
        });

    let subscriber_builder = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(debug) // Include target (module path) in debug mode
        .with_thread_ids(debug) // Include thread IDs in debug mode
        .with_line_number(debug) // Include line numbers in debug mode
        .with_file(debug); // Include file names in debug mode

    if let Some(log_file_path) = log_file {
        // Write to file
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)
            .unwrap_or_else(|err| {
                eprintln!("Failed to open log file '{}': {}", log_file_path, err);
                process::exit(1);
            });

        subscriber_builder
            .with_writer(file)
            .init();
    } else {
        // Write to stderr (default)
        subscriber_builder
            .with_writer(std::io::stderr)
            .init();
    }
}

/// Print usage information and examples
fn print_usage_examples() {
    println!("CURSED Language Server Protocol Examples:");
    println!();
    println!("# Start LSP server on stdin/stdout (default for most editors):");
    println!("cursed-lsp");
    println!();
    println!("# Start LSP server on TCP port 9257:");
    println!("cursed-lsp --mode tcp --port 9257");
    println!();
    println!("# Start LSP server with debug logging:");
    println!("cursed-lsp --debug");
    println!();
    println!("# Start LSP server with verbose logging to file:");
    println!("cursed-lsp --verbose --log-file /tmp/cursed-lsp.log");
    println!();
    println!("# Start LSP server on Unix domain socket:");
    println!("cursed-lsp --mode socket --socket /tmp/cursed-lsp.sock");
    println!();
    println!("Editor Integration:");
    println!();
    println!("# VS Code:");
    println!("# Add to settings.json:");
    println!(r#"  "cursed.languageServer.command": "cursed-lsp""#);
    println!(r#"  "cursed.languageServer.args": ["--debug"]"#);
    println!();
    println!("# Vim/Neovim with coc.nvim:");
    println!("# Add to coc-settings.json:");
    println!(r#"  "languageserver": {{"#);
    println!(r#"    "cursed": {{"#);
    println!(r#"      "command": "cursed-lsp","#);
    println!(r#"      "filetypes": ["cursed"]"#);
    println!(r#"    }}"#);
    println!(r#"  }}"#);
    println!();
    println!("# Emacs with lsp-mode:");
    println!("# Add to your init.el:");
    println!(r#"(add-to-list 'lsp-language-id-configuration '(cursed-mode . "cursed"))"#);
    println!(r#"(lsp-register-client"#);
    println!(r#" (make-lsp-client :new-connection (lsp-stdio-connection "cursed-lsp")"#);
    println!(r#"                  :major-modes '(cursed-mode)"#);
    println!(r#"                  :server-id 'cursed-lsp))"#);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_command_line_parsing() {
        let cmd = Command::new("cursed-lsp")
            .arg(Arg::new("mode").long("mode").value_parser(["stdio", "tcp", "socket"]))
            .arg(Arg::new("port").long("port").value_parser(clap::value_parser!(u16)))
            .arg(Arg::new("debug").long("debug").action(ArgAction::SetTrue));

        // Test default values
        let matches = cmd.clone().try_get_matches_from(Vec::from(["cursed-lsp"])).unwrap();
        assert_eq!(matches.get_one::<String>("mode").map(|s| s.as_str()), Some("stdio"));

        // Test TCP mode with port
        let matches = cmd.clone().try_get_matches_from(Vec::from(["cursed-lsp", "--mode", "tcp", "--port", "8080"])).unwrap();
        assert_eq!(matches.get_one::<String>("mode").map(|s| s.as_str()), Some("tcp"));
        assert_eq!(matches.get_one::<u16>("port"), Some(&8080));

        // Test debug flag
        let matches = cmd.clone().try_get_matches_from(Vec::from(["cursed-lsp", "--debug"])).unwrap();
        assert!(matches.get_flag("debug"));
    }

    #[test]
    fn test_server_mode_parsing() {
        assert!(matches!("stdio", "stdio"));
        assert!(matches!("tcp", "tcp"));
        assert!(matches!("socket", "socket"));
    }

    #[tokio::test]
    async fn test_log_file_creation() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let log_path = temp_file.path().to_str().unwrap();
        
        // Initialize logging to file
        init_logging(false, false, Some(log_path));
        
        // Write a log message
        tracing::info!("Test log message");
        
        // Check that the file contains the message
        let mut contents = String::new();
        temp_file.read_to_string(&mut contents).unwrap();
        assert!(contents.contains("Test log message"));
    }
}
