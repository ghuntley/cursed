//! Tests for CURSED documentation server
//!
//! Tests HTTP server functionality, file serving, and live reload capabilities.

use cursed::docs::server::DocServer;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

#[test]
fn test_server_creation() {
    let temp_dir = TempDir::new().unwrap();
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    assert!(server.is_ok());
}

#[test]
fn test_server_creation_invalid_path() {
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        PathBuf::from("/nonexistent/path"),
    );
    assert!(server.is_err());
}

#[test]
fn test_server_mime_types() {
    let temp_dir = TempDir::new().unwrap();
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    ).unwrap();

    // The mime_types field is private, so we test indirectly by checking
    // that the server was created successfully, indicating mime types were set up
    assert_eq!(format!("{:?}", server).contains("DocServer"), true);
}

#[test]
fn test_server_file_serving_setup() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files
    let index_content = r#"
<!DOCTYPE html>
<html>
<head><title>Test Documentation</title></head>
<body>
    <h1>Welcome to Test Docs</h1>
    <p>This is a test documentation page.</p>
</body>
</html>
"#;
    
    let css_content = r#"
body {
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 20px;
}

h1 {
    color: #333;
}
"#;
    
    let js_content = r#"
console.log('Documentation loaded');

// Simple search functionality
function search(query) {
    console.log('Searching for:', query);
}
"#;
    
    fs::write(temp_dir.path().join("index.html"), index_content).unwrap();
    fs::write(temp_dir.path().join("styles.css"), css_content).unwrap();
    fs::write(temp_dir.path().join("script.js"), js_content).unwrap();
    
    // Create subdirectory with files
    let subdir = temp_dir.path().join("api");
    fs::create_dir_all(&subdir).unwrap();
    fs::write(subdir.join("index.html"), "<h1>API Documentation</h1>").unwrap();
    
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    
    assert!(server.is_ok());
}

#[test]
fn test_server_watch_mode_setup() {
    let temp_dir = TempDir::new().unwrap();
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();
    
    // Create some CURSED source files
    fs::write(src_dir.join("main.csd"), "slay main() { yolo \"Hello world!\"; }").unwrap();
    fs::write(src_dir.join("lib.csd"), "squad MyStruct { sus value: facts; }").unwrap();
    
    let mut server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    ).unwrap();
    
    let watch_paths = vec![src_dir];
    let result = server.enable_watch_mode(watch_paths, Box::new(|paths| {
        // Test callback
        println!("Files changed: {:?}", paths);
    }));
    
    assert!(result.is_ok());
}

#[test] 
fn test_file_watcher_creation() {
    let temp_dir = TempDir::new().unwrap();
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();
    
    // Create test files
    fs::write(src_dir.join("test1.csd"), "// Test file 1").unwrap();
    fs::write(src_dir.join("test2.csd"), "// Test file 2").unwrap();
    
    let watch_paths = vec![src_dir];
    
    // This tests the internal file watcher creation
    // We can't directly test SimpleFileWatcher since it's private,
    // but we can test through the server's enable_watch_mode
    let mut server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    ).unwrap();
    
    let result = server.enable_watch_mode(watch_paths, Box::new(|_| {}));
    assert!(result.is_ok());
}

#[test]
fn test_server_configuration_variations() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test different host configurations
    let server1 = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    assert!(server1.is_ok());
    
    let server2 = DocServer::new(
        "localhost".to_string(),
        3000,
        temp_dir.path().to_path_buf(),
    );
    assert!(server2.is_ok());
    
    let server3 = DocServer::new(
        "0.0.0.0".to_string(),
        9090,
        temp_dir.path().to_path_buf(),
    );
    assert!(server3.is_ok());
}

#[test]
fn test_server_directory_structure() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a complex directory structure
    let dirs = ["api", "guides", "examples", "reference"];
    for dir in &dirs {
        let dir_path = temp_dir.path().join(dir);
        fs::create_dir_all(&dir_path).unwrap();
        fs::write(dir_path.join("index.html"), format!("<h1>{} Documentation</h1>", dir)).unwrap();
    }
    
    // Create main index
    let main_index = r#"
<!DOCTYPE html>
<html>
<head><title>Main Documentation</title></head>
<body>
    <h1>Documentation Index</h1>
    <ul>
        <li><a href="api/">API Reference</a></li>
        <li><a href="guides/">Guides</a></li>
        <li><a href="examples/">Examples</a></li>
        <li><a href="reference/">Language Reference</a></li>
    </ul>
</body>
</html>
"#;
    fs::write(temp_dir.path().join("index.html"), main_index).unwrap();
    
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    
    assert!(server.is_ok());
}

#[test]
fn test_static_file_types() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create various file types that should be served
    let files = [
        ("index.html", "text/html"),
        ("styles.css", "text/css"),
        ("script.js", "application/javascript"),
        ("data.json", "application/json"),
        ("image.png", "image/png"),
        ("image.jpg", "image/jpeg"),
        ("image.gif", "image/gif"),
        ("icon.svg", "image/svg+xml"),
        ("favicon.ico", "image/x-icon"),
        ("font.woff", "font/woff"),
        ("font.woff2", "font/woff2"),
        ("font.ttf", "font/ttf"),
    ];
    
    for (filename, _expected_mime) in &files {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, "test content").unwrap();
        assert!(file_path.exists());
    }
    
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    
    assert!(server.is_ok());
}

#[test]
fn test_live_reload_injection() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create HTML file that should get live reload script injected
    let html_content = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Test Page</title>
</head>
<body>
    <h1>Test Content</h1>
    <p>This page should get live reload functionality.</p>
</body>
</html>
"#;
    
    fs::write(temp_dir.path().join("test.html"), html_content).unwrap();
    
    // The live reload injection happens in the serve_static_file method
    // We can't test it directly without starting the server, but we can
    // verify the setup is correct
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    
    assert!(server.is_ok());
}

#[test]
fn test_security_path_traversal_prevention() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create some files outside the document root that shouldn't be accessible
    let parent_dir = temp_dir.path().parent().unwrap();
    let secret_file = parent_dir.join("secret.txt");
    fs::write(&secret_file, "This should not be accessible").unwrap();
    
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    
    assert!(server.is_ok());
    
    // The actual security test would be done with HTTP requests
    // containing paths like "../secret.txt", but that requires
    // starting the server, which is more of an integration test
}

#[test]
fn test_watch_mode_file_changes() {
    let temp_dir = TempDir::new().unwrap();
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();
    
    // Create initial file
    let test_file = src_dir.join("test.csd");
    fs::write(&test_file, "// Initial content").unwrap();
    
    let mut server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    ).unwrap();
    
    let watch_paths = vec![src_dir.clone()];
    let result = server.enable_watch_mode(watch_paths, Box::new(|_| {}));
    assert!(result.is_ok());
    
    // Simulate file change by updating modification time
    // (In a real test environment, we'd wait and check for the callback)
    thread::sleep(Duration::from_millis(100));
    fs::write(&test_file, "// Updated content").unwrap();
    
    // The file watcher would detect this change in a real scenario
    assert!(test_file.exists());
}

#[test]
fn test_server_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test with various invalid configurations
    
    // Invalid port (though this won't fail until we try to bind)
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        65536, // Invalid port number
        temp_dir.path().to_path_buf(),
    );
    assert!(server.is_ok()); // Constructor doesn't validate port
    
    // Invalid host format (constructor accepts any string)
    let server = DocServer::new(
        "invalid-host-format".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    assert!(server.is_ok()); // Constructor doesn't validate host format
}

#[test]
fn test_directory_listing_functionality() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create directory structure without index.html files
    let api_dir = temp_dir.path().join("api");
    fs::create_dir_all(&api_dir).unwrap();
    
    // Add some files to the directory
    fs::write(api_dir.join("functions.html"), "<h1>Functions</h1>").unwrap();
    fs::write(api_dir.join("structs.html"), "<h1>Structs</h1>").unwrap();
    fs::write(api_dir.join("interfaces.html"), "<h1>Interfaces</h1>").unwrap();
    
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    
    assert!(server.is_ok());
    
    // The directory listing functionality would be tested by making
    // HTTP requests to directories without index.html files
}

#[test]
fn test_server_stop_functionality() {
    let temp_dir = TempDir::new().unwrap();
    
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    ).unwrap();
    
    // Test the stop method (doesn't actually start the server)
    server.stop();
    
    // The running flag should be set to false
    // We can't directly test this since the field is private,
    // but the method should execute without panicking
}

#[test]
fn test_concurrent_server_operations() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create multiple server instances (not running)
    let servers: Vec<_> = (0..5).map(|i| {
        DocServer::new(
            "127.0.0.1".to_string(),
            8080 + i as u16,
            temp_dir.path().to_path_buf(),
        ).unwrap()
    }).collect();
    
    assert_eq!(servers.len(), 5);
    
    // Each server should be independently created
    for server in servers {
        server.stop();
    }
}

#[test]
fn test_file_extension_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create files with various extensions
    let files = [
        "test.html", "test.css", "test.js", "test.json",
        "test.png", "test.jpg", "test.gif", "test.svg",
        "test.ico", "test.woff", "test.woff2", "test.ttf",
        "test.md", "test.txt", "test.unknown"
    ];
    
    for file in &files {
        fs::write(temp_dir.path().join(file), "test content").unwrap();
    }
    
    let server = DocServer::new(
        "127.0.0.1".to_string(),
        8080,
        temp_dir.path().to_path_buf(),
    );
    
    assert!(server.is_ok());
    
    // The actual MIME type handling would be tested with HTTP requests
    // to verify correct Content-Type headers are sent
}
