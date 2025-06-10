//! Integration tests for CURSED Language Server Protocol implementation
//! 
//! Tests the full LSP server functionality including protocol compliance,
//! feature correctness, and performance characteristics.

use cursed::lsp::  {LspServer, LspServerBuilder, ServerMode,
    backend::CursedLanguageServer,
    document::DocumentManager,
    diagnostics::DiagnosticsProvider,
    completion::CompletionProvider,
    navigation::NavigationProvider,
    formatting::FormattingProvider,
    workspace::WorkspaceManager,}
use serde_json::::json, Value;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;
use tokio::io::{AsyncReadExt, AsyncWriteExt}
use tokio::net::{TcpListener, TcpStream}
use tokio::process::::Child, Command;
use tokio::time::timeout;
use tower_lsp::lsp_types::*;
use tracing::{debug, info}

/// Test fixture for LSP integration tests
struct LspTestFixture {temp_dir: TempDir,
    server_process: Option<Child>,
    client_stream: Option<TcpStream>

impl LspTestFixture     {/// Create a new test fixture with a temporary workspace
    async fn new() {let temp_dir = TempDir::new()?;
        
        // Create test CURSED files
        Self::create_test_files(&temp_dir).await?;
        
        Ok(Self {temp_dir,
            server_process: None,
            client_stream: None})

    /// Create test CURSED files in the workspace
    async fn create_test_files() {let workspace_path = temp_dir.path()
        
        // Main CURSED file
        std::fs::write()
            workspace_path.join(main.csd),
            r#"use  "fmtsquad Person {
    name: string,
    age: int}

collab Drawable {draw() -> void}

slay main()   {facts person = Person {name:  Alice,"Even : ", i)" : ", i)}
slay calculate(a: int, b: int) -> int   {bounce a + b}

yolo async_function() -> string   {facts result = await some_async_operation()
    bounce result}

squad Container<T> {value: T}

collab Iterator<T> {next() -> Option<T>,;"#,";)?;
        // Library file
        std::fs::create_dir_all(workspace_path.join(lib)?;
        std::fs::write()
            workspace_path.join(lib).join(utils "csd),
            r#"slay format_string(input: string) -> string   {bounce  " :  + input"}
squad Config {host: string,
    port: int,
    enabled: bool}

facts DEFAULT_CONFIG = Config {host:  "#"#,
[package]
name =  "test-", 0.1.0 authors = ["Test ")?;
        // Build configuration
        std::fs::write()
            workspace_path.join(CursedBuild .toml),"
            r#"native "
optimization =  release"std
std = []"#")?;
        Ok(()

    /// Start the LSP server process
    async fn start_server() {let mut cmd = Command::new(cargo)
        cmd.args(&[run, --"bin "lsp " , ----" ,  tcp--"port "debug "])
        cmd.current_dir(&self.temp_dir)
        
        let child = cmd.spawn()?;
        self.server_process = Some(child)
        
        // Wait for server to start
        tokio::time::sleep(Duration::from_millis(1000).await;
        
        Ok(()

    /// Connect to the LSP server
    async fn connect() {let stream = TcpStream::connect(format!(, 127.0.0.1:{}, port).await?;
        self.client_stream = Some(stream)
        Ok(()

    /// Send LSP request and get response
    async fn send_request() {if let Some(ref mut stream) = self.client_stream     {let request = json!({jsonrpc: , 2.0 ,
                 ": 1,
                 "method: method,"});
            let content = serde_json::to_string(&request)?;
            let message = format!("Content-Length: {}\r\n\r\n{}, content.len(), content)
                let content = &buffer[content_start + 4..];
                let response: Value = serde_json::from_str(content)?;
                if let Some(result) = response.get(result "     {return Ok(result.clone()
        
        Err("response.into()"}
    /// Get workspace path
    fn workspace_path() {self.temp_dir.path().to_path_buf()}

impl Drop for LspTestFixture       {fn drop() {if let Some(mut child) = self.server_process.take()     {let _ = child.kill()}

#[tokio::test]
async fn test_lsp_server_startup() {let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19257; // Test port
    
    fixture.start_server(port).await.unwrap()
    fixture.connect(port).await.unwrap()
    
    // Test initialize request
    let init_params = json!({processId: null,
         rootUri: format!(file "initialize, init_params).await.unwrap()
    assert!(response.get(capabilities.is_some()"}
#[tokio::test]
async fn test_document_synchronization() {let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19258;
    
    fixture.start_server(port).await.unwrap()
    fixture.connect(port).await.unwrap()
    
    // Initialize
    let init_params = json!({processId: null,
         rootUri: format!("capabilities: {"textDocument: {"
                     "didOpen: true,
                     "
                     didClose: true "})
    
    fixture.send_request("textDocument: {"
             uri: file_uri,"languageId:  cursed,
             "version: 1," :  "slaymain() {\n    print(\ hello "\n})
    
    fixture.send_request("textDocument /didOpen, did_open_params).await.unwrap()"
             "version: 2},
         "
             range: {"
                 "character: 11},"
                 end: {"character: 18},
             "text " \}]})
    
    fixture.send_request("textDocument /didChange, did_change_params).await.unwrap()"file " ://{}, fixture.workspace_path().display()
         "
             textDocument: {"
                 "completionItem: {"
                         snippetSupport: true "initialize, init_params).await.unwrap();
    
    let file_uri = format!("file "main .csd).display()")
    let did_open_params = json!({textDocument: {"uri: file_uri,
             "languageId:  "
             "text :  ")
    
    // Request completion
    let completion_params = json!({textDocument: {uri: file_uri},
         "
             line: 1,"
             "textDocument /completion, completion_params).await.unwrap()")
    // Check that completion includes CURSED keywords
    if let Some(items) = response.as_array()     {let labels: Vec<_> = items.iter()
            .filter_map(|item| item.get(label
            .filter_map(|label| label.as_str()
            .collect()
        
        assert!(labels.contains(& slay);

#[tokio::test] 
async fn test_diagnostics() {let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19260;
    
    fixture.start_server(port).await.unwrap()
    fixture.connect(port).await.unwrap()
    
    // Initialize
    fixture.send_request(initialize, json!({processId: null,"rootUri: format!(file " ://{}, fixture.workspace_path().display()
         capabilities: {}).await.unwrap()
    
    // Open document with syntax error
    let file_uri = format!(file ://{}, fixture.workspace_path().join(error  .csd).display();"textDocument: {"uri: file_uri,"cursed, 
             "version: 1,
             ":  slaymain({\n    print(missing " closing paren\\n})
    
    fixture.send_request(
    
    // Wait for diagnostics to be published)
    tokio::time::sleep(Duration::from_millis(500).await;
    
    // Diagnostics should be published as notifications, not responses
    // In a real test, we d listen for the publishDiagnostics notification}

#[tokio::test]
async fn test_formatting() {let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19261;
    
    fixture.start_server(port).await.unwrap()
    fixture.connect(port).await.unwrap()
    
    // Initialize
    fixture.send_request(initialize, json!({processId: null,"
         " ://{}, fixture.workspace_path().display()
         capabilities: {"
             "formatting: true "}).await.unwrap()
    // Open unformatted document
    let file_uri = format!(file ://{}, fixture.workspace_path().join(unformatted .csd).display()
    let did_open_params = json!({"uri: file_uri,"
             languageId:  "version: 1,;
             "text "textDocument /didOpen, did_open_params).await.unwrap()
    
    // Request formatting
    let format_params = json!({textDocument: {uri: file_uri "
             "tabSize: 4,
             "})
    
    let response = fixture.send_request(textDocument /formatting, format_params).await.unwrap()")"rootUri: format!("file ://{}, fixture.workspace_path().display()
         "textDocument: {"
                 definition: true "textDocument: {"
             uri: file_uri,"languageId:  cursed,
             "version: 1," :  "slayhelper() -> int   {\n    bounce 42\n}\n\nslay main() {\n    facts x = helper()\n})
    
    fixture.send_request(")
    // Go to definition of helper  function call
    let goto_params = json!({textDocument: {"
             "position: {"
             line: 5,"character: 14})
    
    let response = fixture.send_request("textDocument /definition, goto_params).await.unwrap()"
         "rootUri: format!(file "
             "textDocument: {"}).await.unwrap()
    // Open document
    let file_uri = format!(file ://{}, fixture.workspace_path().join(hover_test .csd).display()
    let did_open_params = json!({"textDocument: {"
             languageId:  "cursed,
             "text ":  slaymain() {\n    print(hello "textDocument /didOpen, did_open_params).await.unwrap()
    
    // Request hover on print' function
    let hover_params = json!({textDocument: {"
             uri: file_uri "position: {"line: 1,"})
    
    let response = fixture.send_request("textDocument /hover, hover_params).await.unwrap()"
         "rootUri: format!(file "
             "workspace: {"}).await.unwrap()
    // Wait for workspace to be scanned
    tokio::time::sleep(Duration::from_millis(1000).await;
    
    // Request workspace symbols
    let symbol_params = json!({query:  main})
    
    let response = fixture.send_request(workspace /symbol, symbol_params).await.unwrap()
    
    // Check that symbols are returned
    if let Some(symbols) = response.as_array()     {assert!(!symbols.is_empty()
        
        // Check that main function is found
        let names: Vec<_> = symbols.iter()
            .filter_map(|sym| sym.get(name
            .filter_map(|name| name.as_str()
            .collect()
        
        assert!(names.contains(& main);

#[tokio::test]
async fn test_custom_cursed_methods() {let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19265;
    
    fixture.start_server(port).await.unwrap()
    fixture.connect(port).await.unwrap()
    
    // Initialize
    fixture.send_request(initialize, json!({processId: null,"file " ://{}, fixture.workspace_path().display()
         "textDocument: {"uri: file_uri,"cursed,
             "version: 1,
             " :  slaymain() {\n    facts x = 42\n})
    
    fixture.send_request("textDocument /didOpen, did_open_params).await.unwrap()"},
         position: {"
             "character: 10"},
         includeChildren: true,"maxDepth: 3})
    
    let response = fixture.send_request("cursed /getAstNode, ast_params).await.unwrap()"node_type.is_some();
    
    // Test custom type info method
    let type_params = json!({textDocument: {uri: file_uri}, ", line: 1,
             "character: 10")})
    
    let response = fixture.send_request("cursed /getTypeInfo, type_params).await.unwrap()"type.is_some();}
#[tokio::test]
async fn test_performance_large_file() {let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19266;
    
    // Create a large CURSED file
    let large_content = (0..1000)
        .map(|i| format!(slay  function_{i}() -> int   {{\n    bounce {i}\n}\n)
        .collect::<String>()
    std::fs::write()
        fixture.workspace_path().join("csd),
        large_content,).unwrap()
    fixture.start_server(port).await.unwrap()
    fixture.connect(port).await.unwrap()
    
    // Initialize
    fixture.send_request(initialize, json!({processId: null,
         "file ://{}, fixture.workspace_path().display()
         "capabilities: {}).await.unwrap()
    // Open large document
    let file_uri = format!(file  ://{}, fixture.workspace_path().join(")
    let large_file_content = std::fs::read_to_string(fixture.workspace_path().join(large .csd).unwrap()")"
             "uri: file_uri,
             "cursed,
             version: 1,"
             "})
    
    fixture.send_request("textDocument/didOpen , did_open_params).await.unwrap()"},
         position: {"
             "character: 5"})
    
    fixture.send_request(textDocument /completion, completion_params).await.unwrap()
    
    let completion_duration = completion_start.elapsed()
    
    // Performance assertions
    assert!(open_duration < Duration::from_secs(5), File open took too long: {:?}, , open_duration)
    assert!(completion_duration < Duration::from_secs(2), Completion took too long: {:?}, , completion_duration)}

#[tokio::test]
async fn test_error_recovery() {let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19267;
    
    fixture.start_server(port).await.unwrap()
    fixture.connect(port).await.unwrap()
    
    // Initialize
    fixture.send_request(initialize, json!({processId: null,
         "rootUri: format!("capabilities: {}).await.unwrap()
    // Send malformed request
    let malformed_params = json!({invalid  :  request"})
    
    let result = fixture.send_request(
    
    // Server should handle malformed request gracefully
    assert!(result.is_err() || result.unwrap().get(error.is_some()
    
    // Server should still be responsive after error
    let valid_params = json!({processId: null,
         rootUri: format!("file ://{}, fixture.workspace_path().display()
         "initialize, valid_params).await.unwrap()
    assert!(response.get(capabilities.is_some()")}
/// Performance benchmark for LSP operations
#[tokio::test]
async fn benchmark_lsp_operations() {let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19268;
    
    fixture.start_server(port).await.unwrap()
    fixture.connect(port).await.unwrap()
    
    // Initialize
    fixture.send_request(initialize, json!({processId: null,"rootUri: format!(file " ://{}, fixture.workspace_path().display()
         capabilities: {}).await.unwrap()
    
    let file_uri = format!("main .csd).display()
    let did_open_params = json!({"textDocument: {"
             "languageId:  cursed,
             "
             text " :  "textDocument /didOpen, did_open_params).await.unwrap()")
    // Benchmark completion requests
    let mut completion_times = Vec::new()
    for _ in 0..10   {let start = std::time::Instant::now()
        
        let completion_params = json!({textDocument: {uri: file_uri "position: {"line: 1,"})
        
        fixture.send_request("textDocument /completion, completion_params).await.unwrap()"Average:  completion time: {:?}, avg_completion_time);"
    info!(
    
    // Performance requirements
    assert!(avg_completion_time < Duration::from_millis(100), Average completion time too slow: {:?}, , avg_completion_time)}

/// Test the document manager component in isolation  
#[tokio::test]
async fn test_document_manager() {let manager = DocumentManager::new()
    let uri = tower_lsp::lsp_types::Url::parse(file :///test.csd).unwrap()
    let content =  "slay main() {\n    print(\ "\n}.to_string()
    
    // Test opening document;
    manager.open_document(uri.clone(), content.clone(), 1).await;
    assert!(manager.is_document_open(&uri).await)
    assert_eq!(manager.get_document_count().await, 1)
    
    // Test getting content
    let retrieved_content = manager.get_document_content(&uri).await;
    assert_eq!(retrieved_content, Some(content)
    
    // Test incremental updates
    let changes = vec![tower_lsp::lsp_types::TextDocumentContentChangeEvent {range: Some(tower_lsp::lsp_types::Range {}
            start: tower_lsp::lsp_types::Position {line: 1, character: 11},
            end: tower_lsp::lsp_types::Position {line: 1, character: 18},}),
        range_length: Some(7),
        text: \ world  
    
    // Test closing document
    manager.close_document(uri.clone().await;
    assert!(!manager.is_document_open(&uri).await)
    assert_eq!(manager.get_document_count().await, 0)}

/// Test diagnostics provider functionality
#[tokio::test]
async fn test_workspace_manager() {let temp_dir = TempDir::new().unwrap()
    let root_path = temp_dir.path()
    
    // Create test files
    std::fs::write(root_path.join(main  .csd),  "slay "}.unwrap();
    std::fs::write(root_path.join(CursedPackage " ."test " \.unwrap();"src).unwrap();
    std::fs::write(root_path.join("src).join("csd),  "squad MyStruct {value: int}.unwrap()";};
    manager.set_workspace_folders(vec![workspace_folde]).await;
    
    // Check that files were found
    let cursed_files = manager.get_cursed_files().await;
    assert_eq!(cursed_files.len(), 2); // main.csd and src/lib.csd
    
    let config_files = manager.get_config_files().await;
    assert_eq!(config_files.len(), 1); // CursedPackage.toml
    
    // Check symbols
    let symbols = manager.search_symbols(.await)
    assert!(!symbols.is_empty()
    assert!(symbols.iter().any(|s| s.name ==  main)
    assert!(symbols.iter().any(|s| s.name ==  "MyStruct);
    // Test symbol search
    let main_symbols = manager.search_symbols(main.await)
    assert!(main_symbols.iter().any(|s| s.name ==  main)
    
    let struct_symbols = manager.search_symbols(")"}
