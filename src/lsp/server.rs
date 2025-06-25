use crate::error::CursedError;
// Main LSP server implementation

use std::sync::Arc;
use tokio::net::{TcpListener, ToSocketAddrs};
use tower_lsp::{LspService, Server};
use tracing::{info, instrument, error};

use crate::lsp::backend::CursedLanguageServer;

/// LSP server configuration
#[derive(Debug, Clone)]
pub struct LspServerConfig {
    /// Server mode (stdio, tcp, or socket)
    pub mode: ServerMode,
    /// TCP port (if using TCP mode)
    pub port: Option<u16>,
    /// Socket path (if using socket mode)
    pub socket_path: Option<String>,
    /// Enable debug logging
    pub debug: bool,
    /// Maximum number of concurrent requests
    pub max_concurrent_requests: usize,
}

impl Default for LspServerConfig {
    fn default() -> Self {
        Self {
            mode: ServerMode::Stdio,
            port: None,
            socket_path: None,
            debug: false,
            max_concurrent_requests: 100,
        }
    }
}

/// Server communication mode
#[derive(Debug, Clone, PartialEq)]
pub enum ServerMode {
    /// Use stdin/stdout (default for most editors)
    Stdio,
    /// Use TCP connection
    Tcp,
    /// Use Unix domain socket
    Socket,
}

/// Main LSP server
pub struct LspServer {
    config: LspServerConfig,
}

impl LspServer {
    /// Create a new LSP server with the given configuration
    pub fn new(config: LspServerConfig) -> Self {
        Self { config }
    }

    /// Create a new LSP server with default configuration
    pub fn new_default() -> Self {
        Self::new(LspServerConfig::default())
    }

    /// Start the LSP server
    #[instrument(skip(self))]
    pub async fn start(self) -> crate::error::Result<()> {
        info!("Starting CURSED Language Server");
        info!("Server mode: {:?}", self.config.mode);
        
        match self.config.mode {
            ServerMode::Stdio => {
                info!("Starting LSP server on stdin/stdout");
                // Create the language server backend for stdio
                let (service, socket) = LspService::new(|client| CursedLanguageServer::new_with_client(Some(client)));
                Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
                    .serve(service)
                    .await;
            }
            ServerMode::Tcp => {
                let port = self.config.port.unwrap_or(9257); // WAZL in phone keypad
                let addr = format!("127.0.0.1:{}", port);
                info!("Starting LSP server on TCP {}", addr);
                
                let listener = TcpListener::bind(&addr).await?;
                info!("LSP server listening on {}", addr);
                
                loop {
                    let (stream, client_addr) = listener.accept().await?;
                    info!("Client connected from {}", client_addr);
                    
                    // Create a new service for each client connection
                    let (service, socket) = LspService::new(|client| CursedLanguageServer::new_with_client(Some(client)));
                    let (read, write) = tokio::io::split(stream);
                    let server = Server::new(read, write, socket);
                    
                    tokio::spawn(async move {
                        server.serve(service).await;
                        info!("Client {} disconnected", client_addr);
                    });
                }
            }
            ServerMode::Socket => {
                // Unix domain socket implementation would go here
                // For now, fall back to stdio
                error!("Socket mode not yet implemented, falling back to stdio");
                let (service, socket) = LspService::new(|client| CursedLanguageServer::new_with_client(Some(client)));
                Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
                    .serve(service)
                    .await;
            }
        }

        Ok(())
    }
}

/// Builder for LSP server configuration
pub struct LspServerBuilder {
    config: LspServerConfig,
}

impl LspServerBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: LspServerConfig::default(),
        }
    }

    /// Set the server mode
    pub fn mode(mut self, mode: ServerMode) -> Self {
        self.config.mode = mode;
        self
    }

    /// Set the TCP port (for TCP mode)
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = Some(port);
        self
    }

    /// Set the socket path (for socket mode)  
    pub fn socket_path(mut self, path: String) -> Self {
        self.config.socket_path = Some(path);
        self
    }

    /// Enable debug logging
    pub fn debug(mut self, debug: bool) -> Self {
        self.config.debug = debug;
        self
    }

    /// Set maximum concurrent requests
    pub fn max_concurrent_requests(mut self, max: usize) -> Self {
        self.config.max_concurrent_requests = max;
        self
    }

    /// Build the LSP server
    pub fn build(self) -> LspServer {
        LspServer::new(self.config)
    }
}

impl Default for LspServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
