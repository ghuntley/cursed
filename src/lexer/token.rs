//! Token representation for the CURSED language lexical analysis
//!
//! This module defines the `Token` enum that represents all possible tokens
//! in the CURSED programming language. It includes identifiers, literals,
//! operators, keywords, and special tokens.

use crate::error::{Error, ErrorReporter, SourceLocation};
use crate::lexer::TokenType;

/// Represents a lexical token in the CURSED language
///
/// A token is the smallest unit of meaning in the language, such as
/// keywords, identifiers, literals, operators, and delimiters.
///
/// Tokens are created by the lexer during the lexical analysis phase
/// and consumed by the parser to build the abstract syntax tree.
#[derive(Debug, PartialEq, Clone)]
#[repr(u16)]
pub enum Token {
    // Special tokens
    Illegal(String),
    Eof,

    // Identifiers and literals
    Identifier(String),
    String(String),
    Int(i64),
    Float(f64),
    Byte(u8),   // byte literal (single byte value)
    Rune(char), // rune literal (Unicode code point)

    // Operators
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /
    Percent,  // %
    Lt,       // <
    Gt,       // >
    Less = 38,     // < (alias for Lt)
    Greater = 39,  // > (alias for Gt)
    Eq,       // ==
    NotEq,    // !=
    LtEq,     // <=
    GtEq,     // >=
    And,      // &&
    Or,       // ||
    Arrow,    // <-
    At,       // @ (for pointers)

    // Compound assignment operators
    PlusAssign,     // +=
    MinusAssign,    // -=
    AsteriskAssign, // *=
    SlashAssign,    // /=
    PercentAssign,  // %=
    BitAndAssign,   // &=
    BitOrAssign,    // |=
    BitXorAssign,   // ^=

    // Increment/decrement operators
    Inc, // ++
    Dec, // --

    // Bitwise operators
    BitAnd,     // &
    BitOr,      // |
    BitXor,     // ^
    BitCompl,   // ~
    ShiftLeft,  // <<
    ShiftRight, // >>

    // Special tokens
    DeclAssign, // :=
    Ellipsis,   // ...

    // Delimiters
    Comma,     // ,
    Semicolon, // ;
    Colon,     // :
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    Dot,       // .
    Question,  // ?

    // Keywords - CURSED uses Gen Z slang for keywords
    Vibe,      // package (vibe)
    Yeet,      // import (yeet)
    Slay,      // func (slay)
    Sus,       // var (sus)
    Facts,     // const (facts)
    Lowkey,    // if (lowkey)
    Highkey,   // else (highkey)
    Bestie,    // for (bestie)
    Periodt,   // while (periodt)
    VibeCheck, // switch (vibe_check)
    Mood,      // case (mood)
    Basic,     // default (basic)
    Ghosted,   // break (ghosted)
    Simp,      // continue (simp)
    BeLike,    // type (be_like)
    Squad,     // struct (squad)
    Collab,    // interface (collab)
    Tea,       // map (tea)
    Dm,        // chan (dm)
    Stan,      // go (stan)
    Flex,      // range (flex)
    Later,     // defer (later)
    Yolo,      // return (yolo)
    Based,     // true (based)
    Cap,       // nil (cap)
    Crew,      // array literal

    // Type tokens
    Smol,   // int8 (smol)
    Mid,    // int16 (mid)
    Normie, // int32 (normie)
    Thicc,  // int64 (thicc)
    Snack,  // float32 (snack)
    Meal,   // float64 (meal)
    Lit,    // bool (lit)
    Sip,    // char (sip)

    // Comment tokens
    LineComment,       // fr fr
    BlockCommentStart, // no cap
    BlockCommentEnd,   // on god
}

impl Token {
    /// Creates a new token from a token type and literal value
    ///
    /// This method converts a TokenType and a string literal into the appropriate
    /// Token variant. For tokens that contain values (like identifiers, strings,
    /// and numbers), it parses the literal string to create the appropriate
    /// typed value.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of token to create
    /// * `literal` - The string literal representing the token's value
    ///
    /// # Returns
    ///
    /// A new Token instance. For invalid literals, returns Token::Illegal.
    pub fn new(token_type: crate::lexer::TokenType, literal: &str) -> Self {
        match token_type {
            crate::lexer::TokenType::Illegal => Token::Illegal(literal.to_string()),
            TokenType::Eof => Token::Eof,
            TokenType::Identifier => Token::Identifier(literal.to_string()),
            TokenType::String => Token::String(literal.to_string()),
            TokenType::Int => {
                if let Ok(value) = literal.parse::<i64>() {
                    Token::Int(value)
                } else {
                    Token::Illegal(format!("Invalid integer: {}", literal))
                }
            }
            TokenType::Float => {
                if let Ok(value) = literal.parse::<f64>() {
                    Token::Float(value)
                } else {
                    Token::Illegal(format!("Invalid float: {}", literal))
                }
            }
            TokenType::Byte => {
                if literal.len() == 1 {
                    Token::Byte(literal.as_bytes()[0])
                } else {
                    Token::Illegal(format!("Invalid byte: {}", literal))
                }
            }
            TokenType::Rune => {
                if let Some(ch) = literal.chars().next() {
                    Token::Rune(ch)
                } else {
                    Token::Illegal(format!("Invalid rune: {}", literal))
                }
            }
            TokenType::Assign => Token::Assign,
            TokenType::Plus => Token::Plus,
            TokenType::Minus => Token::Minus,
            TokenType::Bang => Token::Bang,
            TokenType::Asterisk => Token::Asterisk,
            TokenType::Slash => Token::Slash,
            TokenType::Percent => Token::Percent,
            TokenType::Lt => Token::Lt,
            TokenType::Gt => Token::Gt,
            TokenType::Less => Token::Lt,  // Map Less to Lt
            TokenType::Greater => Token::Gt,  // Map Greater to Gt
            TokenType::Eq => Token::Eq,
            TokenType::NotEq => Token::NotEq,
            TokenType::LtEq => Token::LtEq,
            TokenType::GtEq => Token::GtEq,
            TokenType::And => Token::And,
            TokenType::Or => Token::Or,
            TokenType::Arrow => Token::Arrow,
            TokenType::At => Token::At,
            TokenType::PlusAssign => Token::PlusAssign,
            TokenType::MinusAssign => Token::MinusAssign,
            TokenType::AsteriskAssign => Token::AsteriskAssign,
            TokenType::SlashAssign => Token::SlashAssign,
            TokenType::PercentAssign => Token::PercentAssign,
            TokenType::BitAndAssign => Token::BitAndAssign,
            TokenType::BitOrAssign => Token::BitOrAssign,
            TokenType::BitXorAssign => Token::BitXorAssign,
            TokenType::Inc => Token::Inc,
            TokenType::Dec => Token::Dec,
            TokenType::BitAnd => Token::BitAnd,
            TokenType::BitOr => Token::BitOr,
            TokenType::BitXor => Token::BitXor,
            TokenType::BitCompl => Token::BitCompl,
            TokenType::ShiftLeft => Token::ShiftLeft,
            TokenType::ShiftRight => Token::ShiftRight,
            TokenType::DeclAssign => Token::DeclAssign,
            TokenType::Ellipsis => Token::Ellipsis,
            TokenType::Comma => Token::Comma,
            TokenType::Semicolon => Token::Semicolon,
            TokenType::Colon => Token::Colon,
            TokenType::LParen => Token::LParen,
            TokenType::RParen => Token::RParen,
            TokenType::LBrace => Token::LBrace,
            TokenType::RBrace => Token::RBrace,
            TokenType::LBracket => Token::LBracket,
            TokenType::RBracket => Token::RBracket,
            TokenType::Dot => Token::Dot,
            TokenType::Vibe => Token::Vibe,
            TokenType::Yeet => Token::Yeet,
            TokenType::Slay => Token::Slay,
            TokenType::Sus => Token::Sus,
            TokenType::Facts => Token::Facts,
            TokenType::Lowkey => Token::Lowkey,
            TokenType::Highkey => Token::Highkey,
            TokenType::Bestie => Token::Bestie,
            TokenType::Periodt => Token::Periodt,
            TokenType::VibeCheck => Token::VibeCheck,
            TokenType::Mood => Token::Mood,
            TokenType::Basic => Token::Basic,
            TokenType::Ghosted => Token::Ghosted,
            TokenType::Simp => Token::Simp,
            TokenType::BeLike => Token::BeLike,
            TokenType::Squad => Token::Squad,
            TokenType::Collab => Token::Collab,
            TokenType::Tea => Token::Tea,
            TokenType::Dm => Token::Dm,
            TokenType::Stan => Token::Stan,
            TokenType::Flex => Token::Flex,
            TokenType::Later => Token::Later,
            TokenType::Yolo => Token::Yolo,
            TokenType::Based => Token::Based,
            TokenType::Cap => Token::Cap,
            TokenType::Crew => Token::Crew,
            TokenType::Smol => Token::Smol,
            TokenType::Mid => Token::Mid,
            TokenType::Normie => Token::Normie,
            TokenType::Thicc => Token::Thicc,
            TokenType::Snack => Token::Snack,
            TokenType::Meal => Token::Meal,
            TokenType::Lit => Token::Lit,
            TokenType::Sip => Token::Sip,
            TokenType::LineComment => Token::LineComment,
            TokenType::BlockCommentStart => Token::BlockCommentStart,
            TokenType::BlockCommentEnd => Token::BlockCommentEnd,
            // For compatibility with tests
            TokenType::True => Token::Based,
            TokenType::Return => Token::Yolo,
            TokenType::If => Token::Lowkey,
            TokenType::While => Token::Periodt,
            TokenType::Break => Token::Ghosted,
            TokenType::Continue => Token::Simp,
            TokenType::For => Token::Bestie,
        }
    }

    /// Gets the string literal representation of the token
    ///
    /// This method converts the token back to its string representation,
    /// which is useful for debugging, error reporting, and testing.
    ///
    /// # Returns
    ///
    /// A string representing the token's value or text.
    pub fn token_literal(&self) -> String {
        match self {
            Token::Illegal(s) => s.clone(),
            Token::Eof => String::from("EOF"),
            Token::Identifier(s) => s.clone(),
            Token::String(s) => s.clone(),
            Token::Int(i) => i.to_string(),
            Token::Float(f) => f.to_string(),
            Token::Byte(b) => format!("b'{}'", *b as char),
            Token::Rune(r) => format!("'{}'", r),

            // Operators
            Token::Assign => String::from("="),
            Token::Plus => String::from("+"),
            Token::Minus => String::from("-"),
            Token::Bang => String::from("!"),
            Token::Asterisk => String::from("*"),
            Token::Slash => String::from("/"),
            Token::Percent => String::from("%"),
            Token::Lt => String::from("<"),
            Token::Gt => String::from(">"),
            Token::Eq => String::from("=="),
            Token::NotEq => String::from("!="),
            Token::LtEq => String::from("<="),
            Token::GtEq => String::from(">="),
            Token::And => String::from("&&"),
            Token::Or => String::from("||"),
            Token::Arrow => String::from("<-"),
            Token::At => String::from("@"),

            // Compound assignment operators
            Token::PlusAssign => String::from("+="),
            Token::MinusAssign => String::from("-="),
            Token::AsteriskAssign => String::from("*="),
            Token::SlashAssign => String::from("/="),
            Token::PercentAssign => String::from("%="),
            Token::BitAndAssign => String::from("&="),
            Token::BitOrAssign => String::from("|="),
            Token::BitXorAssign => String::from("^="),

            // Increment/decrement operators
            Token::Inc => String::from("++"),
            Token::Dec => String::from("--"),

            // Bitwise operators
            Token::BitAnd => String::from("&"),
            Token::BitOr => String::from("|"),
            Token::BitXor => String::from("^"),
            Token::BitCompl => String::from("~"),
            Token::ShiftLeft => String::from("<<"),
            Token::ShiftRight => String::from(">>"),

            // Special tokens
            Token::DeclAssign => String::from(":="),
            Token::Ellipsis => String::from("..."),

            // Delimiters
            Token::Comma => String::from(","),
            Token::Semicolon => String::from(";"),
            Token::Colon => String::from(":"),
            Token::LParen => String::from("("),
            Token::RParen => String::from(")"),
            Token::LBrace => String::from("{"),
            Token::RBrace => String::from("}"),
            Token::LBracket => String::from("["),
            Token::RBracket => String::from("]"),
            Token::Dot => String::from("."),

            // Keywords
            Token::Vibe => String::from("vibe"),
            Token::Yeet => String::from("yeet"),
            Token::Slay => String::from("slay"),
            Token::Sus => String::from("sus"),
            Token::Facts => String::from("facts"),
            Token::Lowkey => String::from("lowkey"),
            Token::Highkey => String::from("highkey"),
            Token::Bestie => String::from("bestie"),
            Token::Periodt => String::from("periodt"),
            Token::VibeCheck => String::from("vibe_check"),
            Token::Mood => String::from("mood"),
            Token::Basic => String::from("basic"),
            Token::Ghosted => String::from("ghosted"),
            Token::Simp => String::from("simp"),
            Token::BeLike => String::from("be_like"),
            Token::Squad => String::from("squad"),
            Token::Collab => String::from("collab"),
            Token::Tea => String::from("tea"),
            Token::Dm => String::from("dm"),
            Token::Stan => String::from("stan"),
            Token::Flex => String::from("flex"),
            Token::Later => String::from("later"),
            Token::Yolo => String::from("yolo"),
            Token::Based => String::from("based"),
            Token::Cap => String::from("cap"),
            Token::Crew => String::from("crew"),

            // Type tokens
            Token::Smol => String::from("smol"),
            Token::Mid => String::from("mid"),
            Token::Normie => String::from("normie"),
            Token::Thicc => String::from("thicc"),
            Token::Snack => String::from("snack"),
            Token::Meal => String::from("meal"),
            Token::Lit => String::from("lit"),
            Token::Sip => String::from("sip"),

            // Comment tokens
            Token::LineComment => String::from("LineComment"),
            Token::BlockCommentStart => String::from("BlockCommentStart"),
            Token::BlockCommentEnd => String::from("BlockCommentEnd"),
        }
    }
}
