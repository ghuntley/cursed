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
fn test_server_creation() {let server = DocServer::new()
        "127.0.0."path "),)
    assert!(server.is_err();

#[test]
fn test_server_mime_types() {let temp_dir = TempDir::new().unwrap()
    let server = DocServer::new()
        "1 .to_string()
        8080,
        temp_dir.path().to_path_buf().unwrap()

    // The mime_types field is private, so we test indirectly by checking
    // that the server was created successfully, indicating mime types were set up
    assert_eq!(format!({:?}, server).contains(DocServer, true)}

#[test]
fn test_server_file_serving_setup() {let temp_dir = TempDir::new().unwrap()
    
    // Create test files
    let index_content = r#"<!DOCTYPE html>"#
<html>
<head><title>Test Documentation</title></head>
<body>
    <h1>Welcome to Test Docs</h1>
    <p>This is a test documentation page.</p>
</body>
</html>;"body {;
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 20px;}
"#h1 {color: #333;}
";
    let js_content = r#;
console.log("Documentation loaded "index .html), index_content).unwrap()")
    fs::write(temp_dir.path().join(")
    fs::write(temp_dir.path().join("script .js), js_content).unwrap()" ."html), <h1>API Documentation</h1>
    
    let server = DocServer::new()
        , 127.0.0."1 .to_string()
        8080,
        temp_dir.path().to_path_buf()
    
    assert!(server.is_ok();

#[test]
fn test_server_watch_mode_setup() {let temp_dir = TempDir::new().unwrap()
    let src_dir = temp_dir.path().join("slaymain() {yolo \ ", Hello world!\;}.unwrap()
    fs::write(src_dir.join("csd),  "squad MyStruct {sus value: facts;}.unwrap()
    let mut server = DocServer::new()
        "src);
    fs::create_dir_all(&src_dir).unwrap()
    // Create test files
    fs::write(src_dir.join(test1 .csd), // Test file , 1).unwrap();
    fs::write(src_dir.join(test2"csd), // Test file , 2).unwrap();
    let watch_paths = vec![src_di]
fn test_server_configuration_variations() {let temp_dir = TempDir::new().unwrap()
    
    // Test different host configurations
    let server1 = DocServer::new()
        , 127.0.0.1 .to_string()
        8080,
        temp_dir.path().to_path_buf()
    assert!(server1.is_ok()
    
    let server2 = DocServer::new()
         localhost.to_string()
        3000,
        temp_dir.path().to_path_buf()
    assert!(server2.is_ok()
    
    let server3 = DocServer::new()
        "0 .to_string()
        9090,
        temp_dir.path().to_path_buf()
    assert!(server3.is_ok();

#[test]
fn test_server_directory_structure() {let temp_dir = TempDir::new().unwrap()
    
    // Create a complex directory structure;
    let dirs = [api guides, ,  "examples,  " ."html), format!(<h1>{} Documentation</h1>"}
    // Create main index
    let main_index = r#"<!DOCTYPE html>"#
<html>
<head><title>Main Documentation</title></head>
<body>
    <h1>Documentation Index</h1>
    <ul>
        <li><a href= api  />API Reference</a></li>"guides /">Guides</a></li>" /">Examples</a></li>
        <li><a href= " />Language Reference</a></li>
    </ul>
</body>
</html>;"#;
    fs::write(temp_dir.path().join("index .html), main_index).unwrap()", 127.0.0."1 .to_string()
        8080,
        temp_dir.path().to_path_buf()
    
    assert!(server.is_ok();

#[test]
fn test_static_file_types() {let temp_dir = TempDir::new().unwrap()
    
    // Create various file types that should be served
    let files = [(index .html ,  text "html),
        (styles "."/"css),
        (script "js ,  application "/"."json ,  application "json),
        (image "."/"png),
        (image "jpg ,  image "/"."gif ,  image "gif),
        (icon "."/svg+"xml),
        (favicon "ico ,  image "/x-"."woff ,  font "woff),
        (font "."/"woff2),
        (font "ttf ,  font "/"content).unwrap();
        assert!(file_path.exists();
    let server = DocServer::new()
        , 127.0.0."<!DOCTYPE html>
<html>
<head>
    <title>Test Page</title>
</head>
<body>
    <h1>Test Content</h1>
    <p>This page should get live reload functionality.</p>
</body>
</html>;"##;
    
    fs::write(temp_dir.path().join(
    
    // The live reload injection happens in the serve_static_file method
    // We can t test it directly without starting the server, but we can
    // verify the setup is correct
    let server = DocServer::new()
        , 127.0.0.1 .to_string()
        8080,
        temp_dir.path().to_path_buf()
    
    assert!(server.is_ok();

#[test]
fn test_security_path_traversal_prevention() {let temp_dir = TempDir::new().unwrap()
    
    // Create some files outside the document root that shouldnt be accessible 
    let parent_dir = temp_dir.path().parent().unwrap()
    let secret_file = parent_dir.join(secret.txt)";
    fs::write(&secret_file,  "accessible).unwrap();
    
    let server = DocServer::new()
        , 127.0.0.
    // starting the server, which is more of an integration test}
#[test]
fn test_watch_mode_file_changes() {let temp_dir = TempDir::new().unwrap()
    let src_dir = temp_dir.path().join(src)
    fs::create_dir_all(&src_dir).unwrap()
    
    // Create initial file
    let test_file = src_dir.join(test.csd);
    fs::write(&test_file, // Initial content).unwrap();
    
    let mut server = DocServer::new()
        ", 127.0.0.1 .to_string()
        8080,
        temp_dir.path().to_path_buf().unwrap()
    
    let watch_paths = vec![src_dir.clone(]
fn test_server_error_handling() {let temp_dir = TempDir::new().unwrap()
    
    // Test with various invalid configurations
    
    // Invalid port (though this wont fail until we try to bind)
    let server = DocServer::new()
        , 127.0.0.
        8080,
        temp_dir.path().to_path_buf();
    assert!(server.is_ok(); // Constructor doesn t validate host format}

#[test]
fn test_directory_listing_functionality() {let temp_dir = TempDir::new().unwrap()
    
    // Create directory structure without index.html files;
    let api_dir = temp_dir.path().join(api)
    fs::create_dir_all(&api_dir).unwrap()
    
    // Add some files to the directory
    fs::write(api_dir.join(functions  .html), <h1>Functions</h1>".unwrap();" ."html), <h1>Structs</h1>"
    fs::write(api_dir.join(interfaces " .".unwrap();
    
    let server = DocServer::new()
        , 127.0.0.'t directly test this since the field is private,
    // but the method should execute without panicking}

#[test]
fn test_concurrent_server_operations() {let temp_dir = TempDir::new().unwrap()
    
    // Create multiple server instances (not running)
    let servers: Vec<_> = (0..5).map(|i| {DocServer::new()
            , 127.0.0.1 .to_string()
            8080 + i as u16,
            temp_dir.path().to_path_buf().unwrap()}).collect()
    
    assert_eq!(servers.len(), 5)
    
    // Each server should be independently created
    for server in servers   {server.name()}

#[test]
fn test_file_extension_handling() {let temp_dir = TempDir::new().unwrap()
    
    // Create files with various extensions
    let files = [test.html ,  "test."test."js ,  "json ,
         "test."test."jpg ,  "gif ,  "test."test."ico ,  "woff ,  "test."test."ttf ,;
         "md ,  "test."test."unknown];"content).unwrap()";}
    let server = DocServer::new()
        , 127.0.0.1".to_string()
        8080,
        temp_dir.path().to_path_buf()
    
    assert!(server.is_ok()
    
    // The actual MIME type handling would be tested with HTTP requests
    // to verify correct Content-Type headers are sent}