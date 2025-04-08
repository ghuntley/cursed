use crate::error::{Error, ErrorReporter, SourceLocation};

/// Token type for the CURSED language
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Special tokens
    Illegal(String),
    Eof,
    
    // Identifiers and literals
    Identifier(String),
    String(String),
    Int(i64),
    Float(f64),
    Byte(u8),    // byte literal (single byte value)
    Rune(char),  // rune literal (Unicode code point)
    
    // Operators
    Assign,      // =
    Plus,        // +
    Minus,       // -
    Bang,        // !
    Asterisk,    // *
    Slash,       // /
    Percent,     // %
    Lt,          // <
    Gt,          // >
    Eq,          // ==
    NotEq,       // !=
    LtEq,        // <=
    GtEq,        // >=
    And,         // &&
    Or,          // ||
    Arrow,       // <-
    At,          // @ (for pointers)
    
    // Compound assignment operators
    PlusAssign,   // +=
    MinusAssign,  // -=
    AsteriskAssign, // *=
    SlashAssign,  // /=
    PercentAssign, // %=
    BitAndAssign, // &=
    BitOrAssign,  // |=
    BitXorAssign, // ^=
    
    // Increment/decrement operators
    Inc,          // ++
    Dec,          // --
    
    // Bitwise operators
    BitAnd,       // &
    BitOr,        // |
    BitXor,       // ^
    BitCompl,     // ~
    ShiftLeft,    // <<
    ShiftRight,   // >>
    
    // Special tokens
    DeclAssign,   // :=
    Ellipsis,     // ...
    
    // Delimiters
    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [
    RBracket,    // ]
    Dot,         // .
    
    // Keywords - CURSED uses Gen Z slang for keywords
    Vibe,        // package (vibe)
    Yeet,        // import (yeet)
    Slay,        // func (slay)
    Sus,         // var (sus)
    Facts,       // const (facts)
    Lowkey,      // if (lowkey)
    Highkey,     // else (highkey)
    Bestie,      // for (bestie)
    Periodt,     // while (periodt)
    VibeCheck,   // switch (vibe_check)
    Mood,        // case (mood)
    Basic,       // default (basic)
    Ghosted,     // break (ghosted)
    Simp,        // continue (simp)
    BeLike,      // type (be_like)
    Squad,       // struct (squad)
    Collab,      // interface (collab)
    Tea,         // map (tea)
    Dm,          // chan (dm)
    Stan,        // go (stan)
    Flex,        // range (flex)
    Later,       // defer (later)
    Yolo,        // return (yolo)
    Based,       // true (based)
    Cap,         // nil (cap)
    Crew,        // array literal
    
    // Integer types
    Smol,        // int8 (smol)
    Mid,         // int16 (mid)
    Normie,      // int32 (normie)
    Thicc,       // int64 (thicc)
    
    // Comment tokens
    LineComment, // fr fr
    BlockCommentStart, // no cap
    BlockCommentEnd,   // on god
}

impl Token {
    /// Get the literal representation of the token
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
            
            // Integer types
            Token::Smol => String::from("smol"),
            Token::Mid => String::from("mid"),
            Token::Normie => String::from("normie"),
            Token::Thicc => String::from("thicc"),
            
            // Comment tokens
            Token::LineComment => String::from("LineComment"),
            Token::BlockCommentStart => String::from("BlockCommentStart"),
            Token::BlockCommentEnd => String::from("BlockCommentEnd"),
        }
    }
}