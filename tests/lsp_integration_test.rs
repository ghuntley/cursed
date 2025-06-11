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
        // TODO: Add fields
    }
    async fn new(} {let temp_dir  =  TempDir::new()?;
use )
        
        // Create test CURSED files
        Self::create_test_files(&temp_dir).await?;
        
        Ok(Self {temp_dir,})
            server_process: None,
            client_stream: None);
    /// Create test CURSED files in the workspace
    async fn create_test_files() {
    // TODO: Implement test
    assert!(true);" : ""
collab Iterator<T> {next() -> Option<T>,;#,")""
            workspace_path.join(lib).join(utils csd,")""
            r#" format_string(input: string) -> string   {bounce  # :  + }""
facts DEFAULT_CONFIG = Config {host:  #"}""
name =  ", -, 0.1.0 authors = [", "]""
            workspace_path.join(CursedBuild .toml),")""
            r#", ""
optimization =  release, ""
std = []#"""
        cmd.args(&[run, --, binlsp  , ----" ,  tcp--", portdebug "]""
                 " 1,""
                 ", ""
            let message = format!(", ""
                if let Some(result) = response.get(result "     {return Ok(result.clone()""
        Err(")""
         rootUri: format!(file , ", init_params).await.unwrap()""
    assert!(true);")""
         rootUri: format!(: {"textDocument: { + ""
                     ""
                     didClose: true )"""
    fixture.send_request(: {""
             uri: file_uri,, :  cursed,"""
             version: 1, :  ""
    fixture.send_request(", " /didOpen, did_open_params).await.unwrap() + : 2},")""
         ""
             range: { + " 11},""
                 end: {, : 18},""
             text """
    fixture.send_request(",  /didChange, did_change_params).await.unwrap()file " ://{}, fixture.workspace_path().display()""
         """
             textDocument: { + : {""
                         snippetSupport: true , ", init_params}.await.unwrap();""
    let file_uri = format!(file ", " .csd).display()""
    let did_open_params = json!({textDocument: {", : file_uri,""
             "   +  :  ""
         ""
             line: 1, + " /completion, completion_params).await.unwrap()""
    fixture.send_request(initialize, json!({processId: null,, : format!(file ")}""
    let file_uri = format!(file ://{), fixture.workspace_path().join(error  .csd).display();, : {"uri: file_uri,", ,"}""
             " 1,""
             ":  slaymain({\\n    print(missing ""
         capabilities: {"}""
             , : true ""
    let did_open_params = json!({, : file_uri,"}""
             languageId:  , : 1,;""
             text , """
    let format_params = json!({textDocument: {uri: file_uri  + "fixed}""
             ");""
    let response = fixture.send_request(textDocument /formatting, format_params).await.unwrap()", : format!(file ://{), fixture.workspace_path().display()""
         , : {"}""
                 definition: true , : {""
             uri: file_uri,, :  cursed,"""
             version: 1, :  ", "} -> int   {\\n    bounce 42"\n")\nslay main() {\n    facts x = helper(}"\n""
    fixture.send_request(""
    let goto_params = json!({textDocument: { + " {""
             line: 5,", ""
    let response = fixture.send_request("textDocument /definition, goto_params).await.unwrap() + " format!(file  + ": {""
    let did_open_params = json!({, " {""
             languageId:  , ", ""
             text "  slaymain(} {\\n    print(hello ",  /didOpen, did_open_params).await.unwrap()")""
    let hover_params = json!({textDocument: {"}""
             uri: file_uri ", " {line: 1,)""
    let response = fixture.send_request(/hover, hover_params).await.unwrap() + " format!(file  + : {""
         ", " {uri: file_uri,, ", ""
             version: 1,""
              :  slaymain(} {\\n    facts x = 42"\n")"""
    fixture.send_request(",  /didOpen, did_open_params).await.unwrap()""
         position: { + " 10},""
         includeChildren: true,, : 3})""
    let response = fixture.send_request(cursed /getAstNode, ast_params).await.unwrap(), """
    let type_params = json!({textDocument: {uri: file_uri), , line: 1,")}""
             ", : 10""
    let response = fixture.send_request(",  /getTypeInfo, type_params).await.unwrap()""
         " ://{}, fixture.workspace_path().display()", " {}).await.unwrap()""
    let file_uri = format!(file  ://{), fixture.workspace_path().join(")""
    let large_file_content = std::fs::read_to_string(fixture.workspace_path().join(large .csd).unwrap(), : file_uri,""
             cursed,"""
    fixture.send_request(", /didOpen , did_open_params).await.unwrap()""
         position: { + " 5})""
         , : format!(")""
    let malformed_params = json!({invalid  :  request)""
         rootUri: format!(",  ://{), fixture.workspace_path().display()")", valid_params).await.unwrap();""
    assert!(response.get(capabilities.is_some()"}""
    fixture.send_request(initialize, json!({processId: null,, " format!(file ""
    let file_uri = format!(" .csd).display()""
    let did_open_params = json!({textDocument: { + "  cursed,}""
             ""
             text  :  ", " /didOpen, did_open_params).await.unwrap()""
        let completion_params = json!({textDocument: {uri: file_uri ", : {line: 1,")"}""
        fixture.send_request(" /completion, completion_params).await.unwrap()"  completion time: {:?}, avg_completion_time);""
    let content =  ",  main() {\\n    print(""
    std::fs::write(root_path.join(main  .csd),  ", )""
    std::fs::write(root_path.join(CursedPackage  .", test unwrap();""
    std::fs::write(root_path.join(").join(",  squad MyStruct {value: int).unwrap()")""
    assert!(symbols.iter().any(|s| s.name ==  ", ;""
    let struct_symbols = manager.search_symbols(""