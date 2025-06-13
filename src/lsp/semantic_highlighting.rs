//! Semantic highlighting implementation for CURSED language constructs
//! 
//! Provides comprehensive semantic highlighting for CURSED's Gen Z slang keywords,
//! type system, and language constructs.

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument};

use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::ast::*;

/// Enhanced semantic token types for CURSED language
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CursedSemanticTokenType {
    // Standard LSP types
    Namespace,
    Type,
    Class,
    Enum,
    Interface,
    Struct,
    Function,
    Variable,
    Property,
    Keyword,
    Comment,
    String,
    Number,
    Operator,
    Parameter,
    Macro,
    
    // CURSED-specific Gen Z slang types
    SlayKeyword,         // Function declarations
    SusKeyword,          // Variable declarations
    FactsKeyword,        // Constants
    LowkeyKeyword,       // If statements
    HighkeyKeyword,      // Else statements
    PeriodtKeyword,      // Switch statements
    BestieKeyword,       // Case statements
    FlexKeyword,         // Default case
    YoloKeyword,         // While/for loops
    StanKeyword,         // Goroutine spawning
    CrushKeyword,        // Channel operations
    SpillKeyword,        // Error/panic
    NoCapKeyword,        // Boolean true
    CapKeyword,          // Boolean false
    VibezKeyword,        // Return statements
    SkrrKeyword,         // Break statements
    YeetKeyword,         // Continue statements
    GurlKeyword,         // Finally blocks
    
    // Type system
    SquadKeyword,        // Struct definitions
    CollabKeyword,       // Interface definitions
    MapKeyword,          // Map types
    ArrayKeyword,        // Array types
    SliceKeyword,        // Slice types
    ChanKeyword,         // Channel types
    
    // Special constructs
    ImportKeyword,       // Import statements
    PackageKeyword,      // Package declarations
    GenericParam,        // Generic type parameters
    ErrorPropagation,    // ? operator
    NilValue,            // nil literal
    
    // Annotations and attributes
    Annotation,          // @annotations
    Pragma,              // #pragmas
    
    // String and formatting
    StringInterpolation, // String template expressions
    FormatSpecifier,     // Format string specifiers
    
    // Async and concurrency
    AsyncKeyword,        // Async operations
    AwaitKeyword,        // Await operations
    
    // Pattern matching
    MatchKeyword,        // Pattern matching
    WhenKeyword,         // When clauses
}

impl CursedSemanticTokenType {
    /// Convert to LSP semantic token type
    pub fn to_lsp_type(&self) -> SemanticTokenType {
        match self {
            Self::Namespace => SemanticTokenType::NAMESPACE,
            Self::Type => SemanticTokenType::TYPE,
            Self::Class => SemanticTokenType::CLASS,
            Self::Enum => SemanticTokenType::ENUM,
            Self::Interface => SemanticTokenType::INTERFACE,
            Self::Struct => SemanticTokenType::STRUCT,
            Self::Function => SemanticTokenType::FUNCTION,
            Self::Variable => SemanticTokenType::VARIABLE,
            Self::Property => SemanticTokenType::PROPERTY,
            Self::Keyword => SemanticTokenType::KEYWORD,
            Self::Comment => SemanticTokenType::COMMENT,
            Self::String => SemanticTokenType::STRING,
            Self::Number => SemanticTokenType::NUMBER,
            Self::Operator => SemanticTokenType::OPERATOR,
            Self::Parameter => SemanticTokenType::PARAMETER,
            Self::Macro => SemanticTokenType::MACRO,
            
            // Map CURSED-specific types to appropriate LSP types
            Self::SlayKeyword | Self::SusKeyword | Self::FactsKeyword |
            Self::LowkeyKeyword | Self::HighkeyKeyword | Self::PeriodtKeyword |
            Self::BestieKeyword | Self::FlexKeyword | Self::YoloKeyword |
            Self::StanKeyword | Self::CrushKeyword | Self::SpillKeyword |
            Self::NoCapKeyword | Self::CapKeyword | Self::VibezKeyword |
            Self::SkrrKeyword | Self::YeetKeyword | Self::GurlKeyword |
            Self::SquadKeyword | Self::CollabKeyword | Self::MapKeyword |
            Self::ArrayKeyword | Self::SliceKeyword | Self::ChanKeyword |
            Self::ImportKeyword | Self::PackageKeyword | Self::AsyncKeyword |
            Self::AwaitKeyword | Self::MatchKeyword | Self::WhenKeyword => SemanticTokenType::KEYWORD,
            
            Self::GenericParam => SemanticTokenType::TYPE_PARAMETER,
            Self::ErrorPropagation => SemanticTokenType::OPERATOR,
            Self::NilValue => SemanticTokenType::KEYWORD,
            Self::Annotation | Self::Pragma => SemanticTokenType::DECORATOR,
            Self::StringInterpolation | Self::FormatSpecifier => SemanticTokenType::STRING,
        }
    }
    
    /// Get token type index for semantic tokens encoding
    pub fn to_index(&self) -> u32 {
        match self {
            Self::Namespace => 0,
            Self::Type => 1,
            Self::Class => 2,
            Self::Enum => 3,
            Self::Interface => 4,
            Self::Struct => 5,
            Self::Function => 6,
            Self::Variable => 7,
            Self::Property => 8,
            Self::Keyword => 9,
            Self::Comment => 10,
            Self::String => 11,
            Self::Number => 12,
            Self::Operator => 13,
            Self::Parameter => 14,
            Self::Macro => 15,
            Self::SlayKeyword => 16,
            Self::SusKeyword => 17,
            Self::FactsKeyword => 18,
            Self::LowkeyKeyword => 19,
            Self::HighkeyKeyword => 20,
            Self::PeriodtKeyword => 21,
            Self::BestieKeyword => 22,
            Self::FlexKeyword => 23,
            Self::YoloKeyword => 24,
            Self::StanKeyword => 25,
            Self::CrushKeyword => 26,
            Self::SpillKeyword => 27,
            Self::NoCapKeyword => 28,
            Self::CapKeyword => 29,
            Self::VibezKeyword => 30,
            Self::SkrrKeyword => 31,
            Self::YeetKeyword => 32,
            Self::GurlKeyword => 33,
            Self::SquadKeyword => 34,
            Self::CollabKeyword => 35,
            Self::MapKeyword => 36,
            Self::ArrayKeyword => 37,
            Self::SliceKeyword => 38,
            Self::ChanKeyword => 39,
            Self::ImportKeyword => 40,
            Self::PackageKeyword => 41,
            Self::GenericParam => 42,
            Self::ErrorPropagation => 43,
            Self::NilValue => 44,
            Self::Annotation => 45,
            Self::Pragma => 46,
            Self::StringInterpolation => 47,
            Self::FormatSpecifier => 48,
            Self::AsyncKeyword => 49,
            Self::AwaitKeyword => 50,
            Self::MatchKeyword => 51,
            Self::WhenKeyword => 52,
        }
    }
}

/// Semantic token modifiers for additional context
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CursedSemanticTokenModifier {
    Declaration,
    Definition,
    Readonly,
    Static,
    Deprecated,
    Async,
    Modification,
    Documentation,
    DefaultLibrary,
    
    // CURSED-specific modifiers
    Nullable,            // Nullable types
    Generic,             // Generic types/functions
    Goroutine,           // Goroutine-related
    Channel,             // Channel-related
    ErrorProne,          // Functions that can error
    Concurrent,          // Concurrent operations
    Immutable,           // Immutable data
    Reference,           // Reference types
    SlangKeyword,        // Gen Z slang keywords
}

impl CursedSemanticTokenModifier {
    /// Convert to LSP semantic token modifier
    pub fn to_lsp_modifier(&self) -> SemanticTokenModifier {
        match self {
            Self::Declaration => SemanticTokenModifier::DECLARATION,
            Self::Definition => SemanticTokenModifier::DEFINITION,
            Self::Readonly => SemanticTokenModifier::READONLY,
            Self::Static => SemanticTokenModifier::STATIC,
            Self::Deprecated => SemanticTokenModifier::DEPRECATED,
            Self::Async => SemanticTokenModifier::ASYNC,
            Self::Modification => SemanticTokenModifier::MODIFICATION,
            Self::Documentation => SemanticTokenModifier::DOCUMENTATION,
            Self::DefaultLibrary => SemanticTokenModifier::DEFAULT_LIBRARY,
            
            // Map CURSED-specific modifiers to appropriate LSP modifiers
            Self::Nullable => SemanticTokenModifier::READONLY,
            Self::Generic => SemanticTokenModifier::ABSTRACT,
            Self::Goroutine | Self::Channel | Self::Concurrent => SemanticTokenModifier::ASYNC,
            Self::ErrorProne => SemanticTokenModifier::DEPRECATED,
            Self::Immutable => SemanticTokenModifier::READONLY,
            Self::Reference => SemanticTokenModifier::READONLY,
            Self::SlangKeyword => SemanticTokenModifier::DEFAULT_LIBRARY,
        }
    }
    
    /// Get modifier bit for semantic tokens encoding
    pub fn to_bit(&self) -> u32 {
        match self {
            Self::Declaration => 1 << 0,
            Self::Definition => 1 << 1,
            Self::Readonly => 1 << 2,
            Self::Static => 1 << 3,
            Self::Deprecated => 1 << 4,
            Self::Async => 1 << 5,
            Self::Modification => 1 << 6,
            Self::Documentation => 1 << 7,
            Self::DefaultLibrary => 1 << 8,
            Self::Nullable => 1 << 9,
            Self::Generic => 1 << 10,
            Self::Goroutine => 1 << 11,
            Self::Channel => 1 << 12,
            Self::ErrorProne => 1 << 13,
            Self::Concurrent => 1 << 14,
            Self::Immutable => 1 << 15,
            Self::Reference => 1 << 16,
            Self::SlangKeyword => 1 << 17,
        }
    }
}

/// Semantic token with position and type information
#[derive(Debug, Clone)]
pub struct SemanticToken {
    pub line: u32,
    pub start: u32,
    pub length: u32,
    pub token_type: CursedSemanticTokenType,
    pub modifiers: Vec<CursedSemanticTokenModifier>,
}

impl SemanticToken {
    /// Create a new semantic token
    pub fn new(
        line: u32,
        start: u32,
        length: u32,
        token_type: CursedSemanticTokenType,
        modifiers: Vec<CursedSemanticTokenModifier>,
    ) -> Self {
        Self {
            line,
            start,
            length,
            token_type,
            modifiers,
        }
    }
    
    /// Get modifiers as bitmask
    pub fn modifiers_bitmask(&self) -> u32 {
        self.modifiers.iter().map(|m| m.to_bit()).fold(0, |acc, bit| acc | bit)
    }
}

/// Semantic highlighting provider for CURSED language
pub struct SemanticHighlightingProvider {
    /// Token type legend for LSP
    token_types: Vec<SemanticTokenType>,
    /// Token modifier legend for LSP
    token_modifiers: Vec<SemanticTokenModifier>,
    /// Keyword mapping
    keyword_map: HashMap<String, CursedSemanticTokenType>,
}

impl SemanticHighlightingProvider {
    /// Create a new semantic highlighting provider
    pub fn new() -> Self {
        let token_types = Self::build_token_types();
        let token_modifiers = Self::build_token_modifiers();
        let keyword_map = Self::build_keyword_map();
        
        Self {
            token_types,
            token_modifiers,
            keyword_map,
        }
    }
    
    /// Build token types legend
    fn build_token_types() -> Vec<SemanticTokenType> {
        vec![
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::TYPE,
            SemanticTokenType::CLASS,
            SemanticTokenType::ENUM,
            SemanticTokenType::INTERFACE,
            SemanticTokenType::STRUCT,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::KEYWORD,
            SemanticTokenType::COMMENT,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::PARAMETER,
            SemanticTokenType::MACRO,
            SemanticTokenType::DECORATOR,
            SemanticTokenType::TYPE_PARAMETER,
        ]
    }
    
    /// Build token modifiers legend
    fn build_token_modifiers() -> Vec<SemanticTokenModifier> {
        vec![
            SemanticTokenModifier::DECLARATION,
            SemanticTokenModifier::DEFINITION,
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::DEPRECATED,
            SemanticTokenModifier::ASYNC,
            SemanticTokenModifier::MODIFICATION,
            SemanticTokenModifier::DOCUMENTATION,
            SemanticTokenModifier::DEFAULT_LIBRARY,
            SemanticTokenModifier::ABSTRACT,
        ]
    }
    
    /// Build keyword mapping for CURSED Gen Z slang
    fn build_keyword_map() -> HashMap<String, CursedSemanticTokenType> {
        let mut map = HashMap::new();
        
        // Function and variable declarations
        map.insert("slay".to_string(), CursedSemanticTokenType::SlayKeyword);
        map.insert("sus".to_string(), CursedSemanticTokenType::SusKeyword);
        map.insert("facts".to_string(), CursedSemanticTokenType::FactsKeyword);
        
        // Control flow
        map.insert("lowkey".to_string(), CursedSemanticTokenType::LowkeyKeyword);
        map.insert("highkey".to_string(), CursedSemanticTokenType::HighkeyKeyword);
        map.insert("periodt".to_string(), CursedSemanticTokenType::PeriodtKeyword);
        map.insert("bestie".to_string(), CursedSemanticTokenType::BestieKeyword);
        map.insert("flex".to_string(), CursedSemanticTokenType::FlexKeyword);
        map.insert("yolo".to_string(), CursedSemanticTokenType::YoloKeyword);
        
        // Concurrency
        map.insert("stan".to_string(), CursedSemanticTokenType::StanKeyword);
        map.insert("crush".to_string(), CursedSemanticTokenType::CrushKeyword);
        
        // Error handling
        map.insert("spill".to_string(), CursedSemanticTokenType::SpillKeyword);
        
        // Boolean values
        map.insert("no_cap".to_string(), CursedSemanticTokenType::NoCapKeyword);
        map.insert("cap".to_string(), CursedSemanticTokenType::CapKeyword);
        
        // Control flow statements
        map.insert("vibez".to_string(), CursedSemanticTokenType::VibezKeyword);
        map.insert("skrr".to_string(), CursedSemanticTokenType::SkrrKeyword);
        map.insert("yeet".to_string(), CursedSemanticTokenType::YeetKeyword);
        map.insert("gurl".to_string(), CursedSemanticTokenType::GurlKeyword);
        
        // Type system
        map.insert("squad".to_string(), CursedSemanticTokenType::SquadKeyword);
        map.insert("collab".to_string(), CursedSemanticTokenType::CollabKeyword);
        map.insert("map".to_string(), CursedSemanticTokenType::MapKeyword);
        map.insert("array".to_string(), CursedSemanticTokenType::ArrayKeyword);
        map.insert("slice".to_string(), CursedSemanticTokenType::SliceKeyword);
        map.insert("chan".to_string(), CursedSemanticTokenType::ChanKeyword);
        
        // Module system
        map.insert("import".to_string(), CursedSemanticTokenType::ImportKeyword);
        map.insert("package".to_string(), CursedSemanticTokenType::PackageKeyword);
        
        // Special values
        map.insert("nil".to_string(), CursedSemanticTokenType::NilValue);
        
        // Async operations
        map.insert("async".to_string(), CursedSemanticTokenType::AsyncKeyword);
        map.insert("await".to_string(), CursedSemanticTokenType::AwaitKeyword);
        
        // Pattern matching
        map.insert("match".to_string(), CursedSemanticTokenType::MatchKeyword);
        map.insert("when".to_string(), CursedSemanticTokenType::WhenKeyword);
        
        map
    }
    
    /// Get semantic tokens legend
    pub fn get_legend(&self) -> SemanticTokensLegend {
        SemanticTokensLegend {
            token_types: self.token_types.clone(),
            token_modifiers: self.token_modifiers.clone(),
        }
    }
    
    /// Generate semantic tokens for the given content
    #[instrument(skip(self, content))]
    pub async fn get_semantic_tokens(&self, content: &str) -> Result<Vec<SemanticToken>, String> {
        debug!("Generating semantic tokens for content");
        
        let mut tokens = Vec::new();
        let mut lexer = Lexer::new(content);
        
        // Tokenize the content
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    if token.token_type == TokenType::Eof {
                        break;
                    }
                    
                    if let Some(semantic_token) = self.token_to_semantic(&token) {
                        tokens.push(semantic_token);
                    }
                }
                Err(e) => {
                    debug!("Lexer error: {:?}", e);
                    break;
                }
            }
        }
        
        Ok(tokens)
    }
    
    /// Convert lexer token to semantic token
    fn token_to_semantic(&self, token: &Token) -> Option<SemanticToken> {
        let line = token.line as u32;
        let start = token.column as u32;
        let length = token.lexeme.len() as u32;
        
        let (token_type, modifiers) = match &token.token_type {
            TokenType::Identifier => {
                // Check if it's a CURSED keyword
                if let Some(&keyword_type) = self.keyword_map.get(&token.lexeme) {
                    (keyword_type, vec![CursedSemanticTokenModifier::SlangKeyword])
                } else {
                    (CursedSemanticTokenType::Variable, vec![])
                }
            }
            
            TokenType::String => (CursedSemanticTokenType::String, vec![]),
            TokenType::Number => (CursedSemanticTokenType::Number, vec![]),
            TokenType::Comment => (CursedSemanticTokenType::Comment, vec![]),
            
            // Operators
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash |
            TokenType::Percent | TokenType::Equal | TokenType::NotEqual |
            TokenType::LessThan | TokenType::LessThanEqual | TokenType::GreaterThan |
            TokenType::GreaterThanEqual | TokenType::LogicalAnd | TokenType::LogicalOr |
            TokenType::BitwiseAnd | TokenType::BitwiseOr | TokenType::BitwiseXor |
            TokenType::LeftShift | TokenType::RightShift => {
                (CursedSemanticTokenType::Operator, vec![])
            }
            
            // Error propagation operator
            TokenType::Question => {
                (CursedSemanticTokenType::ErrorPropagation, vec![])
            }
            
            // Annotations and pragmas
            TokenType::At => (CursedSemanticTokenType::Annotation, vec![]),
            TokenType::Hash => (CursedSemanticTokenType::Pragma, vec![]),
            
            // Other tokens don't get semantic highlighting
            _ => return None,
        };
        
        Some(SemanticToken::new(line, start, length, token_type, modifiers))
    }
    
    /// Convert semantic tokens to LSP semantic tokens format
    pub fn encode_semantic_tokens(&self, tokens: Vec<SemanticToken>) -> SemanticTokens {
        let mut data = Vec::new();
        let mut last_line = 0;
        let mut last_start = 0;
        
        for token in tokens {
            let delta_line = token.line - last_line;
            let delta_start = if delta_line == 0 {
                token.start - last_start
            } else {
                token.start
            };
            
            data.push(delta_line);
            data.push(delta_start);
            data.push(token.length);
            data.push(token.token_type.to_index());
            data.push(token.modifiers_bitmask());
            
            last_line = token.line;
            last_start = token.start;
        }
        
        SemanticTokens {
            result_id: None,
            data,
        }
    }
    
    /// Get semantic tokens for a range
    #[instrument(skip(self, content))]
    pub async fn get_semantic_tokens_range(
        &self,
        content: &str,
        range: Range,
    ) -> Result<Vec<SemanticToken>, String> {
        debug!("Generating semantic tokens for range {:?}", range);
        
        let all_tokens = self.get_semantic_tokens(content).await?;
        
        // Filter tokens within the range
        let filtered_tokens = all_tokens
            .into_iter()
            .filter(|token| {
                let token_line = token.line;
                let token_start = token.start;
                let token_end = token.start + token.length;
                
                // Check if token overlaps with range
                token_line >= range.start.line &&
                token_line <= range.end.line &&
                (token_line > range.start.line || token_end >= range.start.character) &&
                (token_line < range.end.line || token_start <= range.end.character)
            })
            .collect();
        
        Ok(filtered_tokens)
    }
}

impl Default for SemanticHighlightingProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_semantic_highlighting_basic() {
        let provider = SemanticHighlightingProvider::new();
        let content = r#"
            slay greet(name: string) -> string {
                sus greeting = "Hello, " + name;
                vibez greeting;
            }
            
            facts PI = 3.14159;
            
            lowkey (x > 0) {
                // This is a comment
                spill "positive number";
            }
        "#;
        
        let tokens = provider.get_semantic_tokens(content).await.unwrap();
        assert!(!tokens.is_empty());
        
        // Verify specific keywords are highlighted
        let keyword_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| matches!(
                t.token_type,
                CursedSemanticTokenType::SlayKeyword |
                CursedSemanticTokenType::SusKeyword |
                CursedSemanticTokenType::FactsKeyword |
                CursedSemanticTokenType::LowkeyKeyword |
                CursedSemanticTokenType::VibezKeyword |
                CursedSemanticTokenType::SpillKeyword
            ))
            .collect();
        
        assert!(keyword_tokens.len() >= 6);
    }
    
    #[tokio::test]
    async fn test_semantic_token_encoding() {
        let provider = SemanticHighlightingProvider::new();
        let tokens = vec![
            SemanticToken::new(
                0,
                0,
                4,
                CursedSemanticTokenType::SlayKeyword,
                vec![CursedSemanticTokenModifier::SlangKeyword],
            ),
            SemanticToken::new(
                0,
                5,
                5,
                CursedSemanticTokenType::Function,
                vec![CursedSemanticTokenModifier::Declaration],
            ),
        ];
        
        let encoded = provider.encode_semantic_tokens(tokens);
        assert!(!encoded.data.is_empty());
        assert_eq!(encoded.data.len(), 10); // 2 tokens * 5 values each
    }
    
    #[test]
    fn test_keyword_mapping() {
        let provider = SemanticHighlightingProvider::new();
        
        assert_eq!(
            provider.keyword_map.get("slay"),
            Some(&CursedSemanticTokenType::SlayKeyword)
        );
        assert_eq!(
            provider.keyword_map.get("sus"),
            Some(&CursedSemanticTokenType::SusKeyword)
        );
        assert_eq!(
            provider.keyword_map.get("lowkey"),
            Some(&CursedSemanticTokenType::LowkeyKeyword)
        );
    }
}
