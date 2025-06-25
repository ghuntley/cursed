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
    /// TCP port (if using TCP mode)
    /// Socket path (if using socket mode)
    /// Enable debug logging
    /// Maximum number of concurrent requests
impl Default for LspServerConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Server communication mode
#[derive(Debug, Clone, PartialEq)]
pub enum ServerMode {
    /// Use stdin/stdout (default for most editors)
    /// Use TCP connection
    /// Use Unix domain socket
/// Main LSP server
pub struct LspServer {
impl LspServer {
    /// Create a new LSP server with the given configuration
    pub fn new(config: LspServerConfig) -> Self {
        Self { config }
    }

    /// Create a new LSP server with default configuration
    pub fn new_default() -> Self {
        Self::new(LspServerConfig::default())
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
impl LspServerBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
        }
    }

    /// Set the server mode
    pub fn mode(mut self, mode: ServerMode) -> Self {
        self.config.mode = mode;
        self
    /// Set the TCP port (for TCP mode)
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = Some(port);
        self
    /// Set the socket path (for socket mode)  
    pub fn socket_path(mut self, path: String) -> Self {
        self.config.socket_path = Some(path);
        self
    /// Enable debug logging
    pub fn debug(mut self, debug: bool) -> Self {
        self.config.debug = debug;
        self
    /// Set maximum concurrent requests
    pub fn max_concurrent_requests(mut self, max: usize) -> Self {
        self.config.max_concurrent_requests = max;
        self
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
