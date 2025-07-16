//! CURSED LSP Server binary

use cursed::lsp::minimal_server::start_lsp_server;
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--version" {
        println!("CURSED LSP Server v1.0.0 - Language Server Protocol Support");
        println!("Features: Completion, Diagnostics, Hover, Go to Definition, References, Formatting");
        return;
    }

    if args.len() > 1 && args[1] == "--help" {
        println!("CURSED Language Server Protocol Server");
        println!();
        println!("USAGE:");
        println!("    cursed-lsp [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    --version    Show version information");
        println!("    --help       Show this help message");
        println!();
        println!("FEATURES:");
        println!("    • Syntax highlighting");
        println!("    • Error diagnostics");
        println!("    • Code completion");
        println!("    • Hover information");
        println!("    • Go to definition");
        println!("    • Find references");
        println!("    • Code formatting");
        println!("    • Workspace symbols");
        println!();
        println!("EDITOR SETUP:");
        println!("    VS Code: Install the CURSED language extension");
        println!("    Vim/Neovim: Configure with nvim-lspconfig");
        println!("    Emacs: Use lsp-mode configuration");
        return;
    }
    
    info!("Starting CURSED Language Server...");
    
    if let Err(err) = start_lsp_server().await {
        error!("LSP Server error: {}", err);
        std::process::exit(1);
    }
}
