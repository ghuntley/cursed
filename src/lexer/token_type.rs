//! Token type classification for the CURSED language
//!
//! This module defines the `TokenType` enum which classifies all possible token types
//! without their associated values. It's used by the lexer to identify the category
//! of each token before creating the actual Token instance.

/// Represents the category or type of a token in the CURSED language
///
/// TokenType is used to classify tokens without their associated values.
/// This is different from the `Token` enum which includes the actual values
/// for tokens like identifiers, strings, and numbers.
///
/// TokenType is used primarily during lexical analysis to identify the type
/// of a token before creating the actual Token instance with its value.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Special tokens
    Illegal,
    Eof,

    // Identifiers and literals
    Identifier,
    String,
    Int,
    Float,
    Byte, // byte literal (single byte value)
    Rune, // rune literal (Unicode code point)

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
    True,      // true (based) - added for compatibility with tests
    Return,    // return (yolo) - added for compatibility with tests
    If,        // if (lowkey) - added for compatibility with tests
    While,     // while (periodt) - added for compatibility with tests
    Break,     // break (ghosted) - added for compatibility with tests
    Continue,  // continue (simp) - added for compatibility with tests
    For,       // for (bestie) - added for compatibility with tests

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
