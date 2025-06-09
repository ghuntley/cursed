//! Integration tests for CURSED Language Server Protocol implementation
//! 
//! Tests the full LSP server functionality including protocol compliance,
//! feature correctness, and performance characteristics.

use cursed::lsp::{
    LspServer, LspServerBuilder, ServerMode,
    backend::CursedLanguageServer,
    document::DocumentManager,
    diagnostics::DiagnosticsProvider,
    completion::CompletionProvider,
    navigation::NavigationProvider,
    formatting::FormattingProvider,
    workspace::WorkspaceManager,
};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::{Child, Command};
use tokio::time::timeout;
use tower_lsp::lsp_types::*;
use tracing::{debug, info};

/// Test fixture for LSP integration tests
struct LspTestFixture {
    temp_dir: TempDir,
    server_process: Option<Child>,
    client_stream: Option<TcpStream>,
}

impl LspTestFixture {
    /// Create a new test fixture with a temporary workspace
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        
        // Create test CURSED files
        Self::create_test_files(&temp_dir).await?;
        
        Ok(Self {
            temp_dir,
            server_process: None,
            client_stream: None,
        })
    }

    /// Create test CURSED files in the workspace
    async fn create_test_files(temp_dir: &TempDir) -> Result<(), std::io::Error> {
        let workspace_path = temp_dir.path();
        
        // Main CURSED file
        std::fs::write(
            workspace_path.join("main.csd"),
            r#"
use "std/fmt"

squad Person {
    name: string,
    age: int,
}

collab Drawable {
    draw() -> void,
}

slay main() {
    facts person = Person {
        name: "Alice",
        age: 30,
    }
    
    println(person.name)
    
    sus counter = 0
    flex counter < 10 {
        counter = counter + 1
    }
    
    bestie i in 0..5 {
        lowkey i % 2 == 0 {
            println("Even: ", i)
        } highkey {
            println("Odd: ", i)
        }
    }
}

slay calculate(a: int, b: int) -> int {
    bounce a + b
}

yolo async_function() -> string {
    facts result = await some_async_operation()
    bounce result
}

squad Container<T> {
    value: T,
}

collab Iterator<T> {
    next() -> Option<T>,
}
"#,
        )?;

        // Library file
        std::fs::create_dir_all(workspace_path.join("lib"))?;
        std::fs::write(
            workspace_path.join("lib").join("utils.csd"),
            r#"
slay format_string(input: string) -> string {
    bounce "Formatted: " + input
}

squad Config {
    host: string,
    port: int,
    enabled: bool,
}

facts DEFAULT_CONFIG = Config {
    host: "localhost",
    port: 8080,
    enabled: true,
}

vibes Status {
    Active,
    Inactive,
    Pending,
}
"#,
        )?;

        // Package configuration
        std::fs::write(
            workspace_path.join("CursedPackage.toml"),
            r#"
[package]
name = "test-project"
version = "0.1.0"
authors = ["Test Author"]

[dependencies]
std = "*"
"#,
        )?;

        // Build configuration
        std::fs::write(
            workspace_path.join("CursedBuild.toml"),
            r#"
[build]
target = "native"
optimization = "release"

[features]
default = ["std"]
std = []
"#,
        )?;

        Ok(())
    }

    /// Start the LSP server process
    async fn start_server(&mut self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--bin", "cursed-lsp", "--", "--mode", "tcp", "--port", &port.to_string(), "--debug"]);
        cmd.current_dir(&self.temp_dir);
        
        let child = cmd.spawn()?;
        self.server_process = Some(child);
        
        // Wait for server to start
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        Ok(())
    }

    /// Connect to the LSP server
    async fn connect(&mut self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
        self.client_stream = Some(stream);
        Ok(())
    }

    /// Send LSP request and get response
    async fn send_request(&mut self, method: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
        if let Some(ref mut stream) = self.client_stream {
            let request = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": params
            });
            
            let content = serde_json::to_string(&request)?;
            let message = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
            
            stream.write_all(message.as_bytes()).await?;
            
            // Read response
            let mut buffer = String::new();
            stream.read_to_string(&mut buffer).await?;
            
            // Parse LSP response (simplified)
            if let Some(content_start) = buffer.find("\r\n\r\n") {
                let content = &buffer[content_start + 4..];
                let response: Value = serde_json::from_str(content)?;
                if let Some(result) = response.get("result") {
                    return Ok(result.clone());
                }
            }
        }
        
        Err("No connection or invalid response".into())
    }

    /// Get workspace path
    fn workspace_path(&self) -> PathBuf {
        self.temp_dir.path().to_path_buf()
    }
}

impl Drop for LspTestFixture {
    fn drop(&mut self) {
        if let Some(mut child) = self.server_process.take() {
            let _ = child.kill();
        }
    }
}

#[tokio::test]
async fn test_lsp_server_startup() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19257; // Test port
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Test initialize request
    let init_params = json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {}
    });
    
    let response = fixture.send_request("initialize", init_params).await.unwrap();
    assert!(response.get("capabilities").is_some());
}

#[tokio::test]
async fn test_document_synchronization() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19258;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    let init_params = json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {
            "textDocument": {
                "synchronization": {
                    "didOpen": true,
                    "didChange": true,
                    "didClose": true
                }
            }
        }
    });
    
    fixture.send_request("initialize", init_params).await.unwrap();
    
    // Send initialized notification
    let initialized_params = json!({});
    fixture.send_request("initialized", initialized_params).await.unwrap();
    
    // Open document
    let file_uri = format!("file://{}", fixture.workspace_path().join("main.csd").display());
    let did_open_params = json!({
        "textDocument": {
            "uri": file_uri,
            "languageId": "cursed",
            "version": 1,
            "text": "slay main() {\n    print(\"hello\")\n}"
        }
    });
    
    fixture.send_request("textDocument/didOpen", did_open_params).await.unwrap();
    
    // Change document
    let did_change_params = json!({
        "textDocument": {
            "uri": file_uri,
            "version": 2
        },
        "contentChanges": [{
            "range": {
                "start": {"line": 1, "character": 11},
                "end": {"line": 1, "character": 18}
            },
            "text": "\"world\""
        }]
    });
    
    fixture.send_request("textDocument/didChange", did_change_params).await.unwrap();
}

#[tokio::test]
async fn test_completion_features() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19259;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize and open document
    let init_params = json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {
            "textDocument": {
                "completion": {
                    "completionItem": {
                        "snippetSupport": true
                    }
                }
            }
        }
    });
    
    fixture.send_request("initialize", init_params).await.unwrap();
    
    let file_uri = format!("file://{}", fixture.workspace_path().join("main.csd").display());
    let did_open_params = json!({
        "textDocument": {
            "uri": file_uri,
            "languageId": "cursed",
            "version": 1,
            "text": "slay main() {\n    sl\n}"
        }
    });
    
    fixture.send_request("textDocument/didOpen", did_open_params).await.unwrap();
    
    // Request completion
    let completion_params = json!({
        "textDocument": {
            "uri": file_uri
        },
        "position": {
            "line": 1,
            "character": 6
        }
    });
    
    let response = fixture.send_request("textDocument/completion", completion_params).await.unwrap();
    
    // Check that completion includes CURSED keywords
    if let Some(items) = response.as_array() {
        let labels: Vec<_> = items.iter()
            .filter_map(|item| item.get("label"))
            .filter_map(|label| label.as_str())
            .collect();
        
        assert!(labels.contains(&"slay"));
    }
}

#[tokio::test] 
async fn test_diagnostics() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19260;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    fixture.send_request("initialize", json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {}
    })).await.unwrap();
    
    // Open document with syntax error
    let file_uri = format!("file://{}", fixture.workspace_path().join("error.csd").display());
    let did_open_params = json!({
        "textDocument": {
            "uri": file_uri,
            "languageId": "cursed", 
            "version": 1,
            "text": "slay main( {\n    print(\"missing closing paren\"\n}"
        }
    });
    
    fixture.send_request("textDocument/didOpen", did_open_params).await.unwrap();
    
    // Wait for diagnostics to be published
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Diagnostics should be published as notifications, not responses
    // In a real test, we'd listen for the publishDiagnostics notification
}

#[tokio::test]
async fn test_formatting() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19261;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    fixture.send_request("initialize", json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {
            "textDocument": {
                "formatting": true
            }
        }
    })).await.unwrap();
    
    // Open unformatted document
    let file_uri = format!("file://{}", fixture.workspace_path().join("unformatted.csd").display());
    let did_open_params = json!({
        "textDocument": {
            "uri": file_uri,
            "languageId": "cursed",
            "version": 1,
            "text": "slay   main(){facts x=42;print(x);}"
        }
    });
    
    fixture.send_request("textDocument/didOpen", did_open_params).await.unwrap();
    
    // Request formatting
    let format_params = json!({
        "textDocument": {
            "uri": file_uri
        },
        "options": {
            "tabSize": 4,
            "insertSpaces": true
        }
    });
    
    let response = fixture.send_request("textDocument/formatting", format_params).await.unwrap();
    
    // Check that formatting edits are returned
    if let Some(edits) = response.as_array() {
        assert!(!edits.is_empty());
    }
}

#[tokio::test]
async fn test_goto_definition() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19262;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    fixture.send_request("initialize", json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {
            "textDocument": {
                "definition": true
            }
        }
    })).await.unwrap();
    
    // Open document with function call
    let file_uri = format!("file://{}", fixture.workspace_path().join("goto_test.csd").display());
    let did_open_params = json!({
        "textDocument": {
            "uri": file_uri,
            "languageId": "cursed",
            "version": 1,
            "text": "slay helper() -> int {\n    bounce 42\n}\n\nslay main() {\n    facts x = helper()\n}"
        }
    });
    
    fixture.send_request("textDocument/didOpen", did_open_params).await.unwrap();
    
    // Go to definition of 'helper' function call
    let goto_params = json!({
        "textDocument": {
            "uri": file_uri
        },
        "position": {
            "line": 5,
            "character": 14
        }
    });
    
    let response = fixture.send_request("textDocument/definition", goto_params).await.unwrap();
    
    // Check that definition location is returned
    if let Some(location) = response.as_object() {
        assert!(location.get("uri").is_some());
        assert!(location.get("range").is_some());
    }
}

#[tokio::test]
async fn test_hover_information() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19263;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    fixture.send_request("initialize", json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {
            "textDocument": {
                "hover": true
            }
        }
    })).await.unwrap();
    
    // Open document
    let file_uri = format!("file://{}", fixture.workspace_path().join("hover_test.csd").display());
    let did_open_params = json!({
        "textDocument": {
            "uri": file_uri,
            "languageId": "cursed",
            "version": 1,
            "text": "slay main() {\n    print(\"hello\")\n}"
        }
    });
    
    fixture.send_request("textDocument/didOpen", did_open_params).await.unwrap();
    
    // Request hover on 'print' function
    let hover_params = json!({
        "textDocument": {
            "uri": file_uri
        },
        "position": {
            "line": 1,
            "character": 4
        }
    });
    
    let response = fixture.send_request("textDocument/hover", hover_params).await.unwrap();
    
    // Check that hover information is returned
    if let Some(hover) = response.as_object() {
        assert!(hover.get("contents").is_some());
    }
}

#[tokio::test] 
async fn test_workspace_symbols() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19264;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    fixture.send_request("initialize", json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {
            "workspace": {
                "symbol": true
            }
        }
    })).await.unwrap();
    
    // Wait for workspace to be scanned
    tokio::time::sleep(Duration::from_millis(1000)).await;
    
    // Request workspace symbols
    let symbol_params = json!({
        "query": "main"
    });
    
    let response = fixture.send_request("workspace/symbol", symbol_params).await.unwrap();
    
    // Check that symbols are returned
    if let Some(symbols) = response.as_array() {
        assert!(!symbols.is_empty());
        
        // Check that main function is found
        let names: Vec<_> = symbols.iter()
            .filter_map(|sym| sym.get("name"))
            .filter_map(|name| name.as_str())
            .collect();
        
        assert!(names.contains(&"main"));
    }
}

#[tokio::test]
async fn test_custom_cursed_methods() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19265;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    fixture.send_request("initialize", json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {}
    })).await.unwrap();
    
    // Open document
    let file_uri = format!("file://{}", fixture.workspace_path().join("main.csd").display());
    let did_open_params = json!({
        "textDocument": {
            "uri": file_uri,
            "languageId": "cursed",
            "version": 1,
            "text": "slay main() {\n    facts x = 42\n}"
        }
    });
    
    fixture.send_request("textDocument/didOpen", did_open_params).await.unwrap();
    
    // Test custom AST node method
    let ast_params = json!({
        "textDocument": {
            "uri": file_uri
        },
        "position": {
            "line": 1,
            "character": 10
        },
        "includeChildren": true,
        "maxDepth": 3
    });
    
    let response = fixture.send_request("cursed/getAstNode", ast_params).await.unwrap();
    assert!(response.get("node_type").is_some());
    
    // Test custom type info method
    let type_params = json!({
        "textDocument": {
            "uri": file_uri
        },
        "position": {
            "line": 1,
            "character": 10
        },
        "includeHierarchy": true
    });
    
    let response = fixture.send_request("cursed/getTypeInfo", type_params).await.unwrap();
    assert!(response.get("type").is_some());
}

#[tokio::test] 
async fn test_performance_large_file() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19266;
    
    // Create a large CURSED file
    let large_content = (0..1000)
        .map(|i| format!("slay function_{i}() -> int {{\n    bounce {i}\n}}\n"))
        .collect::<String>();
    
    std::fs::write(
        fixture.workspace_path().join("large.csd"),
        large_content,
    ).unwrap();
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    fixture.send_request("initialize", json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {}
    })).await.unwrap();
    
    // Open large document
    let file_uri = format!("file://{}", fixture.workspace_path().join("large.csd").display());
    let large_file_content = std::fs::read_to_string(fixture.workspace_path().join("large.csd")).unwrap();
    
    let start_time = std::time::Instant::now();
    
    let did_open_params = json!({
        "textDocument": {
            "uri": file_uri,
            "languageId": "cursed",
            "version": 1,
            "text": large_file_content
        }
    });
    
    fixture.send_request("textDocument/didOpen", did_open_params).await.unwrap();
    
    let open_duration = start_time.elapsed();
    
    // Test completion performance
    let completion_start = std::time::Instant::now();
    
    let completion_params = json!({
        "textDocument": {
            "uri": file_uri
        },
        "position": {
            "line": 500,
            "character": 5
        }
    });
    
    fixture.send_request("textDocument/completion", completion_params).await.unwrap();
    
    let completion_duration = completion_start.elapsed();
    
    // Performance assertions
    assert!(open_duration < Duration::from_secs(5), "File open took too long: {:?}", open_duration);
    assert!(completion_duration < Duration::from_secs(2), "Completion took too long: {:?}", completion_duration);
}

#[tokio::test]
async fn test_error_recovery() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19267;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    fixture.send_request("initialize", json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {}
    })).await.unwrap();
    
    // Send malformed request
    let malformed_params = json!({
        "invalid": "request"
    });
    
    let result = fixture.send_request("textDocument/completion", malformed_params).await;
    
    // Server should handle malformed request gracefully
    assert!(result.is_err() || result.unwrap().get("error").is_some());
    
    // Server should still be responsive after error
    let valid_params = json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {}
    });
    
    let response = fixture.send_request("initialize", valid_params).await.unwrap();
    assert!(response.get("capabilities").is_some());
}

/// Performance benchmark for LSP operations
#[tokio::test]
async fn benchmark_lsp_operations() {
    let mut fixture = LspTestFixture::new().await.unwrap();
    let port = 19268;
    
    fixture.start_server(port).await.unwrap();
    fixture.connect(port).await.unwrap();
    
    // Initialize
    fixture.send_request("initialize", json!({
        "processId": null,
        "rootUri": format!("file://{}", fixture.workspace_path().display()),
        "capabilities": {}
    })).await.unwrap();
    
    let file_uri = format!("file://{}", fixture.workspace_path().join("main.csd").display());
    let did_open_params = json!({
        "textDocument": {
            "uri": file_uri,
            "languageId": "cursed",
            "version": 1,
            "text": "slay main() {\n    facts x = 42\n    print(x)\n}"
        }
    });
    
    fixture.send_request("textDocument/didOpen", did_open_params).await.unwrap();
    
    // Benchmark completion requests
    let mut completion_times = Vec::new();
    for _ in 0..10 {
        let start = std::time::Instant::now();
        
        let completion_params = json!({
            "textDocument": {
                "uri": file_uri
            },
            "position": {
                "line": 1,
                "character": 8
            }
        });
        
        fixture.send_request("textDocument/completion", completion_params).await.unwrap();
        
        completion_times.push(start.elapsed());
    }
    
    let avg_completion_time = completion_times.iter().sum::<Duration>() / completion_times.len() as u32;
    
    info!("Average completion time: {:?}", avg_completion_time);
    info!("Completion times: {:?}", completion_times);
    
    // Performance requirements
    assert!(avg_completion_time < Duration::from_millis(100), 
           "Average completion time too slow: {:?}", avg_completion_time);
}

/// Test the document manager component in isolation  
#[tokio::test]
async fn test_document_manager() {
    let manager = DocumentManager::new();
    let uri = tower_lsp::lsp_types::Url::parse("file:///test.csd").unwrap();
    let content = "slay main() {\n    print(\"hello\")\n}".to_string();
    
    // Test opening document
    manager.open_document(uri.clone(), content.clone(), 1).await;
    assert!(manager.is_document_open(&uri).await);
    assert_eq!(manager.get_document_count().await, 1);
    
    // Test getting content
    let retrieved_content = manager.get_document_content(&uri).await;
    assert_eq!(retrieved_content, Some(content));
    
    // Test incremental updates
    let changes = vec![tower_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(tower_lsp::lsp_types::Range {
            start: tower_lsp::lsp_types::Position { line: 1, character: 11 },
            end: tower_lsp::lsp_types::Position { line: 1, character: 18 },
        }),
        range_length: Some(7),
        text: "\"world\"".to_string(),
    }];
    
    let updated_content = manager.update_document(uri.clone(), changes, 2).await;
    assert!(updated_content.is_some());
    assert!(updated_content.unwrap().contains("world"));
    
    // Test closing document
    manager.close_document(uri.clone()).await;
    assert!(!manager.is_document_open(&uri).await);
    assert_eq!(manager.get_document_count().await, 0);
}

/// Test diagnostics provider functionality
#[tokio::test]
async fn test_diagnostics_provider() {
    let provider = DiagnosticsProvider::new();
    
    // Test syntax diagnostics
    let content_with_error = "slay main( {\n    print(\"missing paren\"\n}";
    let diagnostics = provider.get_syntax_diagnostics(content_with_error).await;
    assert!(!diagnostics.is_empty());
    
    // Test style diagnostics  
    let non_cursed_style = "function main() {\n    var x = 42\n}";
    let lint_diagnostics = provider.get_lint_diagnostics(non_cursed_style).await;
    assert!(lint_diagnostics.iter().any(|d| 
        d.source == Some("cursed-style".to_string())
    ));
    
    // Test valid code
    let valid_content = "slay main() {\n    facts x = 42\n    print(x)\n}";
    let valid_diagnostics = provider.get_syntax_diagnostics(valid_content).await;
    // May have warnings but should not have errors
    assert!(!valid_diagnostics.iter().any(|d| 
        d.severity == Some(tower_lsp::lsp_types::DiagnosticSeverity::ERROR)
    ));
}

/// Test completion provider functionality
#[tokio::test]
async fn test_completion_provider() {
    let provider = CompletionProvider::new();
    
    // Test keyword completion
    let content = "sl";
    let position = tower_lsp::lsp_types::Position { line: 0, character: 2 };
    let completions = provider.get_completions(content, position).await;
    
    assert!(!completions.is_empty());
    assert!(completions.iter().any(|c| c.label == "slay"));
    
    // Test variable completion
    let content_with_vars = "facts my_var = 42\nprint(my";
    let position = tower_lsp::lsp_types::Position { line: 1, character: 8 };
    let completions = provider.get_completions(content_with_vars, position).await;
    
    assert!(completions.iter().any(|c| c.label == "my_var"));
    
    // Test function completion
    let content_for_funcs = "pr";
    let position = tower_lsp::lsp_types::Position { line: 0, character: 2 };
    let completions = provider.get_completions(content_for_funcs, position).await;
    
    assert!(completions.iter().any(|c| c.label == "print"));
    assert!(completions.iter().any(|c| c.label == "println"));
}

/// Test navigation provider functionality
#[tokio::test]  
async fn test_navigation_provider() {
    let provider = NavigationProvider::new();
    let uri = tower_lsp::lsp_types::Url::parse("file:///test.csd").unwrap();
    
    // Test hover for built-in function
    let content = "print(\"hello\")";
    let position = tower_lsp::lsp_types::Position { line: 0, character: 2 };
    let hover = provider.get_hover_info(content, position).await;
    
    assert!(hover.is_some());
    if let Some(hover) = hover {
        if let tower_lsp::lsp_types::HoverContents::Markup(markup) = hover.contents {
            assert!(markup.value.contains("print"));
        }
    }
    
    // Test go to definition
    let content_with_func = "slay helper() -> int {\n    bounce 42\n}\n\nslay main() {\n    facts x = helper()\n}";
    let position = tower_lsp::lsp_types::Position { line: 5, character: 14 };
    let definition = provider.get_definition(content_with_func, position, &uri).await;
    
    assert!(definition.is_some());
    if let Some(tower_lsp::lsp_types::GotoDefinitionResponse::Scalar(location)) = definition {
        assert_eq!(location.range.start.line, 0); // helper function is on first line
    }
    
    // Test find references
    let position = tower_lsp::lsp_types::Position { line: 0, character: 5 }; // On helper declaration
    let references = provider.find_references(content_with_func, position, &uri).await;
    
    assert_eq!(references.len(), 2); // Declaration + 1 usage
}

/// Test workspace manager functionality
#[tokio::test]
async fn test_workspace_manager() {
    let temp_dir = TempDir::new().unwrap();
    let root_path = temp_dir.path();
    
    // Create test files
    std::fs::write(root_path.join("main.csd"), "slay main() { print(\"hello\") }").unwrap();
    std::fs::write(root_path.join("CursedPackage.toml"), "[package]\nname = \"test\"").unwrap();
    std::fs::create_dir(root_path.join("src")).unwrap();
    std::fs::write(root_path.join("src").join("lib.csd"), "squad MyStruct { value: int }").unwrap();
    
    let manager = WorkspaceManager::new();
    let workspace_folder = tower_lsp::lsp_types::WorkspaceFolder {
        uri: tower_lsp::lsp_types::Url::from_file_path(root_path).unwrap(),
        name: "Test Workspace".to_string(),
    };
    
    manager.set_workspace_folders(vec![workspace_folder]).await;
    
    // Check that files were found
    let cursed_files = manager.get_cursed_files().await;
    assert_eq!(cursed_files.len(), 2); // main.csd and src/lib.csd
    
    let config_files = manager.get_config_files().await;
    assert_eq!(config_files.len(), 1); // CursedPackage.toml
    
    // Check symbols
    let symbols = manager.search_symbols("").await;
    assert!(!symbols.is_empty());
    assert!(symbols.iter().any(|s| s.name == "main"));
    assert!(symbols.iter().any(|s| s.name == "MyStruct"));
    
    // Test symbol search
    let main_symbols = manager.search_symbols("main").await;
    assert!(main_symbols.iter().any(|s| s.name == "main"));
    
    let struct_symbols = manager.search_symbols("Struct").await;
    assert!(struct_symbols.iter().any(|s| s.name == "MyStruct"));
}
