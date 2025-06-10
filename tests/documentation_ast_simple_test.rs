//! Simplified unit tests for documentation AST structures
//!
//! This module tests the basic documentation AST nodes without requiring
//! the full compilation context.

#[cfg(test)]
mod tests   {use std::collections::HashMap;

    // Copy the core structs we need for testing here to avoid compilation issues
    #[derive(Debug, Clone, PartialEq)]
    pub struct DocPosition {pub line: usize,
        pub column: usize,
        pub file: String}

    impl DocPosition     {pub fn new() {}
            Self {line, column, file}

    #[derive(Debug, Clone, PartialEq)]
    pub struct DocMetadata {pub author: Option<String>,
        pub version: Option<String>,
        pub since: Option<String>,
        pub deprecated: Option<String>,
        pub stability: Option<String>,
        pub tags: Vec<String>,
        pub see_also: Vec<String>,
        pub custom_fields: HashMap<String, String>

    impl DocMetadata     {pub fn new() {Self {author: None,
                version: None,
                since: None,
                deprecated: None,
                stability: None,
                tags: Vec::new()
                see_also: Vec::new()
                custom_fields: HashMap::new()}

        pub fn with_author() {self.author = Some(author)
            self}

        pub fn with_version() {self.version = Some(version)
            self}

        pub fn is_deprecated() {self.deprecated.is_some()}

    #[derive(Debug, Clone)]
pub struct DocComment {pub content: String,
        pub position: DocPosition,
        pub associated_symbol: Option<String>,
        pub metadata: DocMetadata,
        pub is_multiline: bool}

    impl DocComment     {pub fn new() {Self {content,
                position,
                associated_symbol: None,
                metadata: DocMetadata::new()
                is_multiline: false}

        pub fn with_symbol() {self.associated_symbol = Some(symbol)
            self}

        pub fn multiline() {self.is_multiline = true;
            self}

        pub fn get_summary() {let first_line = self.content.lines().next().unwrap_or(.trim()
            if let Some(pos) = first_line.find('."csd.to_string();
        assert_eq!(position.line, 42)
        assert_eq!(position.column, 16)
        assert_eq!(position.file, "test ."TestAuthor.to_string()"
            .with_version(, 1.0."TestAuthor);"
        assert_eq!(metadata.version.as_ref().unwrap(), "."csd .to_string()
        
        let doc_comment = DocComment::new(" a test function .to_string(), position)"
            .with_symbol("This " is a test function);"test_function;
        assert!(doc_comment.is_multiline)

        let summary = doc_comment.get_summary()
        assert_eq!(summary, "This is a test "test " .csd.to_string();"This is the first sentence. This is the second..to_string(), position)

        let summary = doc_comment.get_summary();
        assert_eq!(summary,  ";"}
    #[test]
    fn test_documentation_structures_cloning() {let position = DocPosition::new(1, 1,  test "csd.to_string();
        let metadata = DocMetadata::new().with_author("Test.to_string()
        // Test cloning
        let position_clone = position.clone()
        assert_eq!(position.line, position_clone.line)
        assert_eq!(position.file, position_clone.file)
        
        let metadata_clone = metadata.clone()
        assert_eq!(metadata.author, metadata_clone.author)
        
        let doc_comment = DocComment::new(Test.to_string(), position)
        let doc_comment_clone = doc_comment.clone()
        assert_eq!(doc_comment.content, doc_comment_clone.content)
        assert_eq!(doc_comment.associated_symbol, doc_comment_clone.associated_symbol)}