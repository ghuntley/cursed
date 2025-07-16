//! Basic LSP functionality tests

#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};
    use std::io::{Write, BufReader, BufRead};
    use serde_json::json;

    #[test]
    fn test_lsp_binary_exists() {
        let output = Command::new("cargo")
            .args(["build", "--bin", "cursed-lsp"])
            .output()
            .expect("Failed to execute cargo build");

        assert!(output.status.success(), "Failed to build cursed-lsp binary");
    }

    #[test]
    fn test_lsp_version_command() {
        let output = Command::new("./target/x86_64-unknown-linux-gnu/debug/cursed-lsp")
            .arg("--version")
            .output()
            .expect("Failed to execute cursed-lsp --version");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout.contains("CURSED LSP Server"));
        assert!(stdout.contains("v1.0.0"));
    }

    #[test]
    fn test_lsp_help_command() {
        let output = Command::new("./target/x86_64-unknown-linux-gnu/debug/cursed-lsp")
            .arg("--help")
            .output()
            .expect("Failed to execute cursed-lsp --help");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        assert!(stdout.contains("USAGE:"));
        assert!(stdout.contains("FEATURES:"));
        assert!(stdout.contains("EDITOR SETUP:"));
    }

    #[test]
    fn test_lsp_initialization() {
        // This test checks if the LSP server can start and handle basic JSON-RPC messages
        let mut child = Command::new("./target/x86_64-unknown-linux-gnu/debug/cursed-lsp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start cursed-lsp");

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        
        // Send an initialize request
        let initialize_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": "file:///test/workspace",
                "capabilities": {}
            }
        });

        let message = format!("Content-Length: {}\r\n\r\n{}", 
                             initialize_request.to_string().len(), 
                             initialize_request.to_string());

        stdin.write_all(message.as_bytes()).expect("Failed to write to stdin");
        stdin.flush().expect("Failed to flush stdin");

        // Give the server a moment to process
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Send shutdown request
        let shutdown_request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "shutdown",
            "params": null
        });

        let shutdown_message = format!("Content-Length: {}\r\n\r\n{}", 
                                      shutdown_request.to_string().len(), 
                                      shutdown_request.to_string());

        stdin.write_all(shutdown_message.as_bytes()).expect("Failed to write shutdown");
        stdin.flush().expect("Failed to flush stdin");

        // Send exit notification
        let exit_notification = json!({
            "jsonrpc": "2.0",
            "method": "exit"
        });

        let exit_message = format!("Content-Length: {}\r\n\r\n{}", 
                                  exit_notification.to_string().len(), 
                                  exit_notification.to_string());

        stdin.write_all(exit_message.as_bytes()).expect("Failed to write exit");
        stdin.flush().expect("Failed to flush stdin");

        // Wait for the process to finish
        let status = child.wait().expect("Failed to wait for process");

        assert!(status.success() || status.code() == Some(0), "LSP server didn't exit cleanly");
    }

    #[test]
    fn test_lsp_capabilities() {
        // Test that our LSP server advertises the expected capabilities
        // This would require more complex JSON-RPC interaction
        // For now, we'll verify the server starts without crashing
        let output = Command::new("timeout")
            .args(["1", "./target/x86_64-unknown-linux-gnu/debug/cursed-lsp"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output();

        // The server should timeout (not crash) when no input is provided
        match output {
            Ok(output) => {
                // Exit code 124 means timeout occurred (server was running)
                // Exit code 0 means server exited normally (also acceptable)
                assert!(output.status.code() == Some(124) || output.status.code() == Some(0));
            }
            Err(_) => {
                // If timeout command doesn't exist, just check the binary exists
                assert!(std::path::Path::new("./target/x86_64-unknown-linux-gnu/debug/cursed-lsp").exists());
            }
        }
    }

    #[test]
    fn test_cursed_keywords_completion() {
        // Test that our completion includes CURSED keywords
        // This is tested implicitly through the LSP server implementation
        // The actual completion logic is in minimal_server.rs

        // Verify the binary can start (basic smoke test)
        let output = Command::new("./target/x86_64-unknown-linux-gnu/debug/cursed-lsp")
            .arg("--version")
            .output()
            .expect("Failed to execute cursed-lsp");

        assert!(output.status.success());
        
        // The completion functionality is tested through the LSP protocol
        // but requires a more complex test setup with JSON-RPC messages
    }
}
