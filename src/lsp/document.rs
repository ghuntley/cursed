// Document management for LSP server
// 
// Handles text synchronization, incremental updates, and document storage

use std::collections::HashMap;
use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument, warn};

/// Document information stored by the LSP server
#[derive(Debug, Clone)]
pub struct DocumentInfo {
    /// The document URI
    pub uri: Url,
    /// The document content as a rope for efficient editing
    pub content: Rope,
    /// The document version number
    pub version: i32,
    /// The document language ID
    pub language_id: String,
}

impl DocumentInfo {
    /// Create a new document info
    pub fn new(uri: Url, content: String, version: i32, language_id: String) -> Self {
        Self {
            uri,
            content: Rope::from_str(&content),
            version,
            language_id,
        }
    }

    /// Get the full text content as a string
    pub fn get_text(&self) -> String {
        self.content.to_string()
    }

    /// Get text at a specific range
    pub fn get_text_range(&self, range: Range) -> String {
        let start_line = range.start.line as usize;
        let start_char = range.start.character as usize;
        let end_line = range.end.line as usize;
        let end_char = range.end.character as usize;

        if start_line >= self.content.len_lines() {
            return String::new();
        }

        let start_line_start = self.content.line_to_char(start_line);
        let start_pos = start_line_start + start_char;

        let end_pos = if end_line >= self.content.len_lines() {
            self.content.len_chars()
        } else {
            let end_line_start = self.content.line_to_char(end_line);
            end_line_start + end_char
        };

        if start_pos >= end_pos {
            return String::new();
        }

        self.content.slice(start_pos..end_pos).to_string()
    }

    /// Get the character at a specific position
    pub fn get_char_at_position(&self, position: Position) -> Option<char> {
        let line = position.line as usize;
        let character = position.character as usize;

        if line >= self.content.len_lines() {
            return None;
        }

        let line_start = self.content.line_to_char(line);
        let char_pos = line_start + character;

        if char_pos >= self.content.len_chars() {
            return None;
        }

        self.content.char(char_pos).into()
    }

    /// Get the word at a specific position
    pub fn get_word_at_position(&self, position: Position) -> Option<String> {
        let line = position.line as usize;
        let character = position.character as usize;

        if line >= self.content.len_lines() {
            return None;
        }

        let line_text = self.content.line(line).to_string();
        let chars: Vec<char> = line_text.chars().collect();

        if character >= chars.len() {
            return None;
        }

        // Find word boundaries
        let mut start = character;
        let mut end = character;

        // Move start backward to find beginning of word
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }

        // Move end forward to find end of word
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }

        if start < end {
            Some(chars[start..end].iter().collect())
        } else {
            None
        }
    }

    /// Apply a text change to the document
    pub fn apply_change(&mut self, change: TextDocumentContentChangeEvent) {
        match change.range {
            Some(range) => {
                // Incremental change
                let start_line = range.start.line as usize;
                let start_char = range.start.character as usize;
                let end_line = range.end.line as usize;
                let end_char = range.end.character as usize;

                if start_line < self.content.len_lines() {
                    let start_line_start = self.content.line_to_char(start_line);
                    let start_pos = start_line_start + start_char;

                    let end_pos = if end_line >= self.content.len_lines() {
                        self.content.len_chars()
                    } else {
                        let end_line_start = self.content.line_to_char(end_line);
                        end_line_start + end_char
                    };

                    if start_pos <= end_pos && end_pos <= self.content.len_chars() {
                        self.content.remove(start_pos..end_pos);
                        self.content.insert(start_pos, &change.text);
                    }
                }
            }
            None => {
                // Full document change
                self.content = Rope::from_str(&change.text);
            }
        }
    }
}

/// Document manager for the LSP server
pub struct DocumentManager {
    /// Map of document URIs to document information
    documents: DashMap<Url, DocumentInfo>,
}

impl DocumentManager {
    /// Create a new document manager
    pub fn new() -> Self {
        Self {
            documents: DashMap::new(),
        }
    }

    /// Open a document
    #[instrument(skip(self, content))]
    pub async fn open_document(&self, uri: Url, content: String, version: i32) {
        debug!("Opening document: {}", uri);
        
        let language_id = Self::detect_language_id(&uri);
        let doc_info = DocumentInfo::new(uri.clone(), content, version, language_id);
        self.documents.insert(uri, doc_info);
    }

    /// Update a document with incremental changes
    #[instrument(skip(self, changes))]
    pub async fn update_document(
        &self,
        uri: Url,
        changes: Vec<TextDocumentContentChangeEvent>,
        version: i32,
    ) -> Option<String> {
        debug!("Updating document: {} (version {})", uri, version);
        
        if let Some(mut doc_entry) = self.documents.get_mut(&uri) {
            doc_entry.version = version;
            
            for change in changes {
                (*doc_entry).apply_change(change);
            }
            
            Some(doc_entry.get_text())
        } else {
            warn!("Attempted to update non-existent document: {}", uri);
            None
        }
    }

    /// Close a document
    #[instrument(skip(self))]
    pub async fn close_document(&self, uri: Url) {
        debug!("Closing document: {}", uri);
        self.documents.remove(&uri);
    }

    /// Get document content
    pub async fn get_document_content(&self, uri: &Url) -> Option<String> {
        self.documents.get(uri).map(|doc| doc.get_text())
    }

    /// Get document info
    pub async fn get_document_info(&self, uri: &Url) -> Option<DocumentInfo> {
        self.documents.get(uri).map(|doc| doc.clone())
    }

    /// Get all open document URIs
    pub async fn get_open_documents(&self) -> Vec<Url> {
        self.documents.iter().map(|entry| entry.key().clone()).collect()
    }

    /// Get document count
    pub async fn get_document_count(&self) -> usize {
        self.documents.len()
    }

    /// Check if document is open
    pub async fn is_document_open(&self, uri: &Url) -> bool {
        self.documents.contains_key(uri)
    }

    /// Get text at a specific range
    pub async fn get_text_range(&self, uri: &Url, range: Range) -> Option<String> {
        self.documents.get(uri).map(|doc| doc.get_text_range(range))
    }

    /// Get word at position
    pub async fn get_word_at_position(&self, uri: &Url, position: Position) -> Option<String> {
        self.documents.get(uri).and_then(|doc| doc.get_word_at_position(position))
    }

    /// Get character at position
    pub async fn get_char_at_position(&self, uri: &Url, position: Position) -> Option<char> {
        self.documents.get(uri).and_then(|doc| doc.get_char_at_position(position))
    }

    /// Detect language ID from URI
    fn detect_language_id(uri: &Url) -> String {
        if let Some(path) = uri.path().split('/').last() {
            if path.ends_with(".csd") {
                return "cursed".to_string();
            }
        }
        "plaintext".to_string()
    }

    /// Get document statistics
    pub async fn get_document_stats(&self, uri: &Url) -> Option<DocumentStats> {
        self.documents.get(uri).map(|doc| {
            let content = &doc.content;
            DocumentStats {
                lines: content.len_lines(),
                characters: content.len_chars(),
                words: content.to_string().split_whitespace().count(),
                bytes: content.to_string().len(),
            }
        })
    }
}

/// Document statistics
#[derive(Debug, Clone)]
pub struct DocumentStats {
    pub lines: usize,
    pub characters: usize,
    pub words: usize,
    pub bytes: usize,
}

impl Default for DocumentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_document_manager_basic_operations() {
        let manager = DocumentManager::new();
        let uri = Url::parse("file:///test.csd").unwrap();
        let content = "slay main() {\n    facts x = 42\n}".to_string();

        // Test opening document
        manager.open_document(uri.clone(), content.clone(), 1).await;
        assert!(manager.is_document_open(&uri).await);
        assert_eq!(manager.get_document_count().await, 1);

        // Test getting content
        let retrieved_content = manager.get_document_content(&uri).await;
        assert_eq!(retrieved_content, Some(content));

        // Test closing document
        manager.close_document(uri.clone()).await;
        assert!(!manager.is_document_open(&uri).await);
        assert_eq!(manager.get_document_count().await, 0);
    }

    #[tokio::test]
    async fn test_incremental_updates() {
        let manager = DocumentManager::new();
        let uri = Url::parse("file:///test.csd").unwrap();
        let initial_content = "slay main() {\n    facts x = 42\n}".to_string();

        manager.open_document(uri.clone(), initial_content, 1).await;

        // Test incremental change
        let change = TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position { line: 1, character: 11 },
                end: Position { line: 1, character: 13 },
            }),
            range_length: Some(2),
            text: "100".to_string(),
        };

        let updated_content = manager.update_document(uri.clone(), Vec::from([change]), 2).await;
        assert!(updated_content.is_some());
        assert!(updated_content.unwrap().contains("x = 100"));
    }

    #[test]
    fn test_document_info_word_extraction() {
        let uri = Url::parse("file:///test.csd").unwrap();
        let content = "slay main() {\n    facts variable_name = 42\n}".to_string();
        let doc = DocumentInfo::new(uri, content, 1, "cursed".to_string());

        // Test getting word at position
        let word = doc.get_word_at_position(Position { line: 1, character: 10 });
        assert_eq!(word, Some("variable_name".to_string()));

        // Test getting character at position
        let char = doc.get_char_at_position(Position { line: 0, character: 0 });
        assert_eq!(char, Some('s'));
    }
}
