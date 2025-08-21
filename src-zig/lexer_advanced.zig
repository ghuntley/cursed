//! Advanced lexer for complete CURSED language support
//!
//! This lexer tokenizes all CURSED language constructs including:
//! - Pattern matching keywords and operators
//! - Generic type syntax with constraints
//! - Interface inheritance keywords
//! - Advanced control flow constructs
//! - Channel and concurrency operators
//! - Error handling keywords
//! - All operators and punctuation

const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

/// Enhanced token kinds for complete CURSED language
pub const TokenKind = enum {
    // Literals
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharacterLiteral,
    BooleanLiteral,
    
    // Identifiers and keywords
    Identifier,
    
    // CURSED keywords (core language)
    Vibe,           // package/namespace
    Yeet,           // import
    Slay,           // function
    Sus,            // variable declaration
    Facts,          // constant declaration
    BeLike,         // type alias
    Squad,          // struct
    Collab,         // interface
    Flex,           // implementation
    Stan,           // goroutine
    Later,          // defer
    Damn,           // return
    Yolo,           // return (alternative)
    
    // Control flow
    Lowkey,         // if
    Highkey,        // else
    VibeCheck,      // match/switch
    Mood,           // case
    Basic,          // default
    Bestie,         // for
    Periodt,        // while
    Ghosted,        // break
    Simp,           // continue
    Ready,          // select
    
    // Types (basic)
    Normie,         // int
    Tea,            // string
    Lit,            // bool
    Meal,           // float
    Sip,            // byte
    Drip,           // int64
    Smol,           // int32
    Thicc,          // int64
    Txt,            // char
    Vibes,          // any/interface{}
    
    // Boolean literals
    Based,          // true
    Cringe,         // false
    
    // Channel and concurrency
    Dm,             // channel type (dm<T>)
    ChannelSend,    // <-
    ChannelRecv,    // <-
    
    // Error handling
    Yikes,          // error/exception
    Shook,          // try
    Fam,            // catch
    
    // Pattern matching
    Underscore,     // _
    DotDot,         // ..
    DotDotDot,      // ...
    
    // Operators (arithmetic)
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    StarStar,       // **
    
    // Operators (comparison)
    Equal,          // =
    EqualEqual,     // ==
    BangEqual,      // !=
    Less,           // <
    LessEqual,      // <=
    Greater,        // >
    GreaterEqual,   // >=
    Spaceship,      // <=>
    
    // Operators (logical)
    Bang,           // !
    AmpersandAmpersand, // &&
    PipePipe,       // ||
    
    // Operators (bitwise)
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    Tilde,          // ~
    LeftShift,      // <<
    RightShift,     // >>
    
    // Assignment operators
    PlusEqual,      // +=
    MinusEqual,     // -=
    StarEqual,      // *=
    SlashEqual,     // /=
    PercentEqual,   // %=
    AmpersandEqual, // &=
    PipeEqual,      // |=
    CaretEqual,     // ^=
    LeftShiftEqual, // <<=
    RightShiftEqual, // >>=
    
    // Punctuation
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    LeftAngle,      // <
    RightAngle,     // >
    
    // Delimiters
    Comma,          // ,
    Semicolon,      // ;
    Colon,          // :
    ColonColon,     // ::
    Dot,            // .
    DotQuestion,    // .?
    Question,       // ?
    QuestionQuestion, // ??
    
    // Arrows and pointers
    Arrow,          // ->
    FatArrow,       // =>
    LeftArrow,      // <-
    At,             // @
    Hash,           // #
    Dollar,         // $
    
    // Generic and type system
    Where,          // where
    Impl,           // impl
    Trait,          // trait
    Type,           // type
    Typeof,         // typeof
    Sizeof,         // sizeof
    Alignof,        // alignof
    
    // Lifetime and memory
    Lifetime,       // 'a, 'static, etc.
    Mut,            // mut
    Ref,            // ref
    Move,           // move
    Copy,           // copy
    
    // Visibility and attributes
    Pub,            // pub
    Priv,           // priv
    Protected,      // protected
    Internal,       // internal
    Export,         // export
    Extern,         // extern
    
    // Async/await
    Async,          // async
    Await,          // await
    Future,         // future
    Promise,        // promise
    
    // Pattern matching enhancements
    If,             // if (in patterns)
    Guard,          // guard
    When,           // when
    Is,             // is
    As,             // as
    In,             // in
    
    // Memory management
    New,            // new
    Delete,         // delete
    Alloc,          // alloc
    Free,           // free
    
    // Module system
    Module,         // module
    Use,            // use
    From,           // from
    Super,          // super
    Self,           // self
    SelfType,       // Self
    
    // Attributes and annotations
    Attribute,      // #[...]
    Deprecated,     // deprecated
    Inline,         // inline
    NoInline,       // noinline
    Pure,           // pure
    Const,          // const
    Static,         // static
    
    // Macros and metaprogramming
    Macro,          // macro
    Quote,          // quote
    Unquote,        // unquote
    Splice,         // splice
    
    // Comments and documentation
    Comment,        // // or /* */
    DocComment,     // /// or /** */
    LineComment,    // //
    BlockComment,   // /* */
    
    // Special tokens
    Newline,        // \n
    Whitespace,     // space, tab
    Eof,            // end of file
    Error,          // error token
    Unknown,        // unknown token
    
    // String interpolation
    StringStart,    // "
    StringEnd,      // "
    StringMid,      // middle part of interpolated string
    InterpolationStart, // ${
    InterpolationEnd,   // }
    
    // Numeric suffixes
    IntSuffix,      // i32, i64, u32, u64
    FloatSuffix,    // f32, f64
    
    // Range operators
    Range,          // ..
    RangeInclusive, // ..=
    RangeFrom,      // ..
    RangeTo,        // ..
    RangeFull,      // ..
    
    // Closure syntax
    ClosureStart,   // |
    ClosureEnd,     // |
    Lambda,         // =>
    
    // Advanced operators
    Pipeline,       // |>
    Compose,        // >>
    ComposeLeft,    // <<
    Apply,          // <|
    
    // Type constraints
    Implements,     // implements
    Extends,        // extends
    With,           // with
    Mixin,          // mixin
    
    // Error propagation
    Try,            // try
    Propagate,      // ?
    Unwrap,         // !
    UnwrapOr,       // ??
    
    // Package and versioning
    Package,        // package
    Version,        // version
    Feature,        // feature
    
    // Testing and debugging
    Test,           // test
    Bench,          // bench
    Debug,          // debug
    Assert,         // assert
    Panic,          // panic
    
    // Foreign function interface
    Ffi,            // ffi
    CCall,          // ccall
    StdCall,        // stdcall
    FastCall,       // fastcall
    
    // Compile-time evaluation
    Comptime,       // comptime
    Eval,           // eval
    
    // Struct and union keywords
    Struct,         // struct (alternative to squad)
    Union,          // union
    Enum,           // enum
    Variant,        // variant
    
    // Interface keywords  
    Interface,      // interface (alternative to collab)
    Vtable,         // vtable
    Dispatch,       // dispatch
    
    // Advanced control flow
    Match,          // match (alternative to vibe_check)
    Case,           // case (alternative to mood)
    Default,        // default (alternative to basic)
    Switch,         // switch
    
    // Loop enhancements
    For,            // for (alternative to bestie)
    While,          // while (alternative to periodt)
    Loop,           // loop
    Until,          // until
    Repeat,         // repeat
    
    // Branch control
    Break,          // break (alternative to ghosted)
    Continue,       // continue (alternative to simp)
    Return,         // return (alternative to damn/yolo)
    Yield,          // yield
    
    // Exception handling
    Throw,          // throw
    Catch,          // catch (alternative to fam)
    Finally,        // finally
    Ensure,         // ensure
    
    // Generator and coroutine
    Generator,      // generator
    Coroutine,      // coroutine
    Suspend,        // suspend
    Resume,         // resume
    
    // Advanced type system
    Generic,        // generic
    Template,       // template
    Typename,       // typename
    Auto,           // auto
    Var,            // var
    Let,            // let
    
    // Ownership and borrowing
    Own,            // own
    Borrow,         // borrow
    Shared,         // shared
    Unique,         // unique
    Weak,           // weak
    
    // Concurrency primitives
    Atomic,         // atomic
    Volatile,       // volatile
    Sync,           // sync
    Lock,           // lock
    Mutex,          // mutex
    Semaphore,      // semaphore
    
    // Channel operations
    Send,           // send
    Recv,           // recv
    Close,          // close
    Select,         // select (alternative to ready)
    
    // Unsafe operations
    Unsafe,         // unsafe
    Raw,            // raw
    Transmute,      // transmute
    Cast,           // cast
    
    // Reflection and metaprogramming
    Reflect,        // reflect
    TypeInfo,       // typeinfo
    FieldInfo,      // fieldinfo
    MethodInfo,     // methodinfo
};

/// Token with location information
pub const Token = struct {
    kind: TokenKind,
    lexeme: []const u8,
    line: u32,
    column: u32,
    offset: u32,
    length: u32,
    
    pub fn init(kind: TokenKind, lexeme: []const u8, line: u32, column: u32, offset: u32) Token {
        return Token{
            .kind = kind,
            .lexeme = lexeme,
            .line = line,
            .column = column,
            .offset = offset,
            .length = @intCast(lexeme.len),
        };
    }
    
    pub fn isLiteral(self: Token) bool {
        return switch (self.kind) {
            .IntegerLiteral, .FloatLiteral, .StringLiteral, .CharacterLiteral, .BooleanLiteral => true,
            .Based, .Cringe => true,
            else => false,
        };
    }
    
    pub fn isKeyword(self: Token) bool {
        return switch (self.kind) {
            .Vibe, .Yeet, .Slay, .Sus, .Facts, .BeLike, .Squad, .Collab, .Flex,
            .Stan, .Later, .Damn, .Yolo, .Lowkey, .Highkey, .VibeCheck, .Mood,
            .Basic, .Bestie, .Periodt, .Ghosted, .Simp, .Ready, .Based, .Cringe,
            .Yikes, .Shook, .Fam => true,
            else => false,
        };
    }
    
    pub fn isOperator(self: Token) bool {
        return switch (self.kind) {
            .Plus, .Minus, .Star, .Slash, .Percent, .StarStar,
            .Equal, .EqualEqual, .BangEqual, .Less, .LessEqual, .Greater, .GreaterEqual,
            .Bang, .AmpersandAmpersand, .PipePipe, .Ampersand, .Pipe, .Caret, .Tilde,
            .LeftShift, .RightShift, .PlusEqual, .MinusEqual, .StarEqual, .SlashEqual => true,
            else => false,
        };
    }
    
    pub fn isDelimiter(self: Token) bool {
        return switch (self.kind) {
            .Comma, .Semicolon, .Colon, .Dot, .Question => true,
            else => false,
        };
    }
    
    pub fn isBrace(self: Token) bool {
        return switch (self.kind) {
            .LeftParen, .RightParen, .LeftBrace, .RightBrace, .LeftBracket, .RightBracket => true,
            else => false,
        };
    }
};

/// Lexer state for advanced tokenization
pub const LexerState = struct {
    input: []const u8,
    position: u32,
    current_char: ?u8,
    line: u32,
    column: u32,
    
    pub fn init(input: []const u8) LexerState {
        var state = LexerState{
            .input = input,
            .position = 0,
            .current_char = null,
            .line = 1,
            .column = 1,
        };
        
        if (input.len > 0) {
            state.current_char = input[0];
        }
        
        return state;
    }
    
    pub fn advance(self: *LexerState) void {
        if (self.current_char == '\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        
        self.position += 1;
        
        if (self.position >= self.input.len) {
            self.current_char = null;
        } else {
            self.current_char = self.input[self.position];
        }
    }
    
    pub fn peek(self: *LexerState, offset: u32) ?u8 {
        const pos = self.position + offset;
        if (pos >= self.input.len) {
            return null;
        }
        return self.input[pos];
    }
    
    pub fn peekChar(self: *LexerState) ?u8 {
        return self.peek(1);
    }
    
    pub fn isAtEnd(self: *LexerState) bool {
        return self.current_char == null;
    }
    
    pub fn match(self: *LexerState, expected: u8) bool {
        if (self.current_char != expected) {
            return false;
        }
        self.advance();
        return true;
    }
    
    pub fn matchString(self: *LexerState, expected: []const u8) bool {
        if (self.position + expected.len > self.input.len) {
            return false;
        }
        
        const slice = self.input[self.position..self.position + expected.len];
        if (std.mem.eql(u8, slice, expected)) {
            for (expected) |_| {
                self.advance();
            }
            return true;
        }
        
        return false;
    }
};

/// Advanced lexer with complete CURSED language support
pub const AdvancedLexer = struct {
    state: LexerState,
    tokens: ArrayList(Token),
    allocator: Allocator,
    keywords: std.StringHashMap(TokenKind),
    
    pub fn init(allocator: Allocator, input: []const u8) !AdvancedLexer {
        var lexer = AdvancedLexer{
            .state = LexerState.init(input),
            .tokens = .empty,
            .allocator = allocator,
            .keywords = std.StringHashMap(TokenKind).init(allocator),
        };
        
        try lexer.initializeKeywords();
        return lexer;
    }
    
    pub fn deinit(self: *AdvancedLexer) void {
        self.tokens.deinit();
        self.keywords.deinit();
    }
    
    fn initializeKeywords(self: *AdvancedLexer) !void {
        // Core CURSED keywords
        try self.keywords.put("vibe", .Vibe);
        try self.keywords.put("yeet", .Yeet);
        try self.keywords.put("slay", .Slay);
        try self.keywords.put("sus", .Sus);
        try self.keywords.put("facts", .Facts);
        try self.keywords.put("be_like", .BeLike);
        try self.keywords.put("squad", .Squad);
        try self.keywords.put("collab", .Collab);
        try self.keywords.put("flex", .Flex);
        try self.keywords.put("stan", .Stan);
        try self.keywords.put("later", .Later);
        try self.keywords.put("damn", .Damn);
        try self.keywords.put("yolo", .Yolo);
        
        // Control flow
        try self.keywords.put("lowkey", .Lowkey);
        try self.keywords.put("highkey", .Highkey);
        try self.keywords.put("vibe_check", .VibeCheck);
        try self.keywords.put("mood", .Mood);
        try self.keywords.put("basic", .Basic);
        try self.keywords.put("bestie", .Bestie);
        try self.keywords.put("periodt", .Periodt);
        try self.keywords.put("ghosted", .Ghosted);
        try self.keywords.put("simp", .Simp);
        try self.keywords.put("ready", .Ready);
        
        // Types
        try self.keywords.put("normie", .Normie);
        try self.keywords.put("tea", .Tea);
        try self.keywords.put("lit", .Lit);
        try self.keywords.put("meal", .Meal);
        try self.keywords.put("sip", .Sip);
        try self.keywords.put("drip", .Drip);
        try self.keywords.put("smol", .Smol);
        try self.keywords.put("thicc", .Thicc);
        try self.keywords.put("txt", .Txt);
        try self.keywords.put("vibes", .Vibes);
        
        // Boolean literals
        try self.keywords.put("based", .Based);
        try self.keywords.put("cringe", .Cringe);
        
        // Channel type
        try self.keywords.put("dm", .Dm);
        
        // Error handling
        try self.keywords.put("yikes", .Yikes);
        try self.keywords.put("shook", .Shook);
        try self.keywords.put("fam", .Fam);
        
        // Advanced keywords
        try self.keywords.put("where", .Where);
        try self.keywords.put("impl", .Impl);
        try self.keywords.put("trait", .Trait);
        try self.keywords.put("type", .Type);
        try self.keywords.put("typeof", .Typeof);
        try self.keywords.put("sizeof", .Sizeof);
        try self.keywords.put("alignof", .Alignof);
        
        // Visibility
        try self.keywords.put("pub", .Pub);
        try self.keywords.put("priv", .Priv);
        try self.keywords.put("protected", .Protected);
        try self.keywords.put("internal", .Internal);
        try self.keywords.put("export", .Export);
        try self.keywords.put("extern", .Extern);
        
        // Async/await
        try self.keywords.put("async", .Async);
        try self.keywords.put("await", .Await);
        try self.keywords.put("future", .Future);
        try self.keywords.put("promise", .Promise);
        
        // Pattern matching
        try self.keywords.put("if", .If);
        try self.keywords.put("guard", .Guard);
        try self.keywords.put("when", .When);
        try self.keywords.put("is", .Is);
        try self.keywords.put("as", .As);
        try self.keywords.put("in", .In);
        
        // Memory management
        try self.keywords.put("new", .New);
        try self.keywords.put("delete", .Delete);
        try self.keywords.put("alloc", .Alloc);
        try self.keywords.put("free", .Free);
        try self.keywords.put("mut", .Mut);
        try self.keywords.put("ref", .Ref);
        try self.keywords.put("move", .Move);
        try self.keywords.put("copy", .Copy);
        
        // Module system
        try self.keywords.put("module", .Module);
        try self.keywords.put("use", .Use);
        try self.keywords.put("from", .From);
        try self.keywords.put("super", .Super);
        try self.keywords.put("self", .Self);
        try self.keywords.put("Self", .SelfType);
        
        // Additional language constructs
        try self.keywords.put("struct", .Struct);
        try self.keywords.put("union", .Union);
        try self.keywords.put("enum", .Enum);
        try self.keywords.put("interface", .Interface);
        try self.keywords.put("match", .Match);
        try self.keywords.put("case", .Case);
        try self.keywords.put("default", .Default);
        try self.keywords.put("for", .For);
        try self.keywords.put("while", .While);
        try self.keywords.put("loop", .Loop);
        try self.keywords.put("break", .Break);
        try self.keywords.put("continue", .Continue);
        try self.keywords.put("return", .Return);
        try self.keywords.put("yield", .Yield);
        try self.keywords.put("throw", .Throw);
        try self.keywords.put("catch", .Catch);
        try self.keywords.put("finally", .Finally);
        try self.keywords.put("try", .Try);
        try self.keywords.put("unsafe", .Unsafe);
        try self.keywords.put("const", .Const);
        try self.keywords.put("static", .Static);
        try self.keywords.put("inline", .Inline);
        try self.keywords.put("pure", .Pure);
        try self.keywords.put("comptime", .Comptime);
        try self.keywords.put("test", .Test);
        try self.keywords.put("bench", .Bench);
        try self.keywords.put("debug", .Debug);
        try self.keywords.put("assert", .Assert);
        try self.keywords.put("panic", .Panic);
        
        // Type system enhancements
        try self.keywords.put("generic", .Generic);
        try self.keywords.put("template", .Template);
        try self.keywords.put("typename", .Typename);
        try self.keywords.put("auto", .Auto);
        try self.keywords.put("var", .Var);
        try self.keywords.put("let", .Let);
        
        // Concurrency
        try self.keywords.put("atomic", .Atomic);
        try self.keywords.put("volatile", .Volatile);
        try self.keywords.put("sync", .Sync);
        try self.keywords.put("lock", .Lock);
        try self.keywords.put("mutex", .Mutex);
        try self.keywords.put("send", .Send);
        try self.keywords.put("recv", .Recv);
        try self.keywords.put("close", .Close);
        try self.keywords.put("select", .Select);
        
        // Ownership and borrowing
        try self.keywords.put("own", .Own);
        try self.keywords.put("borrow", .Borrow);
        try self.keywords.put("shared", .Shared);
        try self.keywords.put("unique", .Unique);
        try self.keywords.put("weak", .Weak);
        
        // Interface operations
        try self.keywords.put("implements", .Implements);
        try self.keywords.put("extends", .Extends);
        try self.keywords.put("with", .With);
        try self.keywords.put("mixin", .Mixin);
    }
    
    pub fn tokenize(self: *AdvancedLexer) ![]Token {
        while (!self.state.isAtEnd()) {
            try self.scanToken();
        }
        
        // Add EOF token
        const eof_token = Token.init(.Eof, "", self.state.line, self.state.column, self.state.position);
        try self.tokens.append(eof_token);
        
        return self.tokens.toOwnedSlice();
    }
    
    fn scanToken(self: *AdvancedLexer) !void {
        const start_pos = self.state.position;
        const start_line = self.state.line;
        const start_column = self.state.column;
        
        const char = self.state.current_char orelse return;
        
        switch (char) {
            // Whitespace
            ' ', '\t', '\r' => {
                self.state.advance();
                return;
            },
            
            // Newline
            '\n' => {
                self.state.advance();
                try self.addToken(.Newline, start_pos, start_line, start_column);
                return;
            },
            
            // Single character tokens
            '(' => {
                self.state.advance();
                try self.addToken(.LeftParen, start_pos, start_line, start_column);
            },
            ')' => {
                self.state.advance();
                try self.addToken(.RightParen, start_pos, start_line, start_column);
            },
            '{' => {
                self.state.advance();
                try self.addToken(.LeftBrace, start_pos, start_line, start_column);
            },
            '}' => {
                self.state.advance();
                try self.addToken(.RightBrace, start_pos, start_line, start_column);
            },
            '[' => {
                self.state.advance();
                try self.addToken(.LeftBracket, start_pos, start_line, start_column);
            },
            ']' => {
                self.state.advance();
                try self.addToken(.RightBracket, start_pos, start_line, start_column);
            },
            ',' => {
                self.state.advance();
                try self.addToken(.Comma, start_pos, start_line, start_column);
            },
            ';' => {
                self.state.advance();
                try self.addToken(.Semicolon, start_pos, start_line, start_column);
            },
            '?' => {
                self.state.advance();
                if (self.state.match('?')) {
                    try self.addToken(.QuestionQuestion, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Question, start_pos, start_line, start_column);
                }
            },
            '@' => {
                self.state.advance();
                try self.addToken(.At, start_pos, start_line, start_column);
            },
            '#' => {
                self.state.advance();
                try self.addToken(.Hash, start_pos, start_line, start_column);
            },
            '$' => {
                self.state.advance();
                try self.addToken(.Dollar, start_pos, start_line, start_column);
            },
            '~' => {
                self.state.advance();
                try self.addToken(.Tilde, start_pos, start_line, start_column);
            },
            '^' => {
                self.state.advance();
                if (self.state.match('=')) {
                    try self.addToken(.CaretEqual, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Caret, start_pos, start_line, start_column);
                }
            },
            
            // Multi-character operators
            '+' => {
                self.state.advance();
                if (self.state.match('=')) {
                    try self.addToken(.PlusEqual, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Plus, start_pos, start_line, start_column);
                }
            },
            '-' => {
                self.state.advance();
                if (self.state.match('=')) {
                    try self.addToken(.MinusEqual, start_pos, start_line, start_column);
                } else if (self.state.match('>')) {
                    try self.addToken(.Arrow, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Minus, start_pos, start_line, start_column);
                }
            },
            '*' => {
                self.state.advance();
                if (self.state.match('=')) {
                    try self.addToken(.StarEqual, start_pos, start_line, start_column);
                } else if (self.state.match('*')) {
                    try self.addToken(.StarStar, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Star, start_pos, start_line, start_column);
                }
            },
            '/' => {
                self.state.advance();
                if (self.state.match('=')) {
                    try self.addToken(.SlashEqual, start_pos, start_line, start_column);
                } else if (self.state.match('/')) {
                    // Line comment
                    try self.scanLineComment();
                } else if (self.state.match('*')) {
                    // Block comment
                    try self.scanBlockComment();
                } else {
                    try self.addToken(.Slash, start_pos, start_line, start_column);
                }
            },
            '%' => {
                self.state.advance();
                if (self.state.match('=')) {
                    try self.addToken(.PercentEqual, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Percent, start_pos, start_line, start_column);
                }
            },
            '=' => {
                self.state.advance();
                if (self.state.match('=')) {
                    try self.addToken(.EqualEqual, start_pos, start_line, start_column);
                } else if (self.state.match('>')) {
                    try self.addToken(.FatArrow, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Equal, start_pos, start_line, start_column);
                }
            },
            '!' => {
                self.state.advance();
                if (self.state.match('=')) {
                    try self.addToken(.BangEqual, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Bang, start_pos, start_line, start_column);
                }
            },
            '<' => {
                self.state.advance();
                if (self.state.match('=')) {
                    if (self.state.match('>')) {
                        try self.addToken(.Spaceship, start_pos, start_line, start_column);
                    } else {
                        try self.addToken(.LessEqual, start_pos, start_line, start_column);
                    }
                } else if (self.state.match('<')) {
                    if (self.state.match('=')) {
                        try self.addToken(.LeftShiftEqual, start_pos, start_line, start_column);
                    } else {
                        try self.addToken(.LeftShift, start_pos, start_line, start_column);
                    }
                } else if (self.state.match('-')) {
                    try self.addToken(.LeftArrow, start_pos, start_line, start_column);
                } else if (self.state.match('|')) {
                    try self.addToken(.Apply, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Less, start_pos, start_line, start_column);
                }
            },
            '>' => {
                self.state.advance();
                if (self.state.match('=')) {
                    try self.addToken(.GreaterEqual, start_pos, start_line, start_column);
                } else if (self.state.match('>')) {
                    if (self.state.match('=')) {
                        try self.addToken(.RightShiftEqual, start_pos, start_line, start_column);
                    } else {
                        try self.addToken(.RightShift, start_pos, start_line, start_column);
                    }
                } else {
                    try self.addToken(.Greater, start_pos, start_line, start_column);
                }
            },
            '&' => {
                self.state.advance();
                if (self.state.match('&')) {
                    try self.addToken(.AmpersandAmpersand, start_pos, start_line, start_column);
                } else if (self.state.match('=')) {
                    try self.addToken(.AmpersandEqual, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Ampersand, start_pos, start_line, start_column);
                }
            },
            '|' => {
                self.state.advance();
                if (self.state.match('|')) {
                    try self.addToken(.PipePipe, start_pos, start_line, start_column);
                } else if (self.state.match('=')) {
                    try self.addToken(.PipeEqual, start_pos, start_line, start_column);
                } else if (self.state.match('>')) {
                    try self.addToken(.Pipeline, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Pipe, start_pos, start_line, start_column);
                }
            },
            ':' => {
                self.state.advance();
                if (self.state.match(':')) {
                    try self.addToken(.ColonColon, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Colon, start_pos, start_line, start_column);
                }
            },
            '.' => {
                self.state.advance();
                if (self.state.match('.')) {
                    if (self.state.match('.')) {
                        try self.addToken(.DotDotDot, start_pos, start_line, start_column);
                    } else if (self.state.match('=')) {
                        try self.addToken(.RangeInclusive, start_pos, start_line, start_column);
                    } else {
                        try self.addToken(.DotDot, start_pos, start_line, start_column);
                    }
                } else if (self.state.match('?')) {
                    try self.addToken(.DotQuestion, start_pos, start_line, start_column);
                } else {
                    try self.addToken(.Dot, start_pos, start_line, start_column);
                }
            },
            '_' => {
                self.state.advance();
                try self.addToken(.Underscore, start_pos, start_line, start_column);
            },
            
            // String literals
            '"' => {
                try self.scanString();
            },
            '\'' => {
                try self.scanCharacter();
            },
            
            // Numbers
            '0'...'9' => {
                try self.scanNumber();
            },
            
            // Identifiers and keywords
            'a'...'z', 'A'...'Z' => {
                try self.scanIdentifier();
            },
            
            else => {
                // Unknown character
                self.state.advance();
                try self.addToken(.Unknown, start_pos, start_line, start_column);
            },
        }
    }
    
    fn scanLineComment(self: *AdvancedLexer) !void {
        const start_pos = self.state.position - 2; // Account for '//'
        const start_line = self.state.line;
        const start_column = self.state.column - 2;
        
        // Consume characters until newline or EOF
        while (self.state.current_char != null and self.state.current_char != '\n') {
            self.state.advance();
        }
        
        try self.addToken(.LineComment, start_pos, start_line, start_column);
    }
    
    fn scanBlockComment(self: *AdvancedLexer) !void {
        const start_pos = self.state.position - 2; // Account for '/*'
        const start_line = self.state.line;
        const start_column = self.state.column - 2;
        
        // Consume characters until '*/' or EOF
        while (self.state.current_char != null) {
            if (self.state.current_char == '*' and self.state.peekChar() == '/') {
                self.state.advance(); // consume '*'
                self.state.advance(); // consume '/'
                break;
            }
            self.state.advance();
        }
        
        try self.addToken(.BlockComment, start_pos, start_line, start_column);
    }
    
    fn scanString(self: *AdvancedLexer) !void {
        const start_pos = self.state.position;
        const start_line = self.state.line;
        const start_column = self.state.column;
        
        self.state.advance(); // consume opening quote
        
        while (self.state.current_char != null and self.state.current_char != '"') {
            if (self.state.current_char == '\\') {
                self.state.advance(); // consume backslash
                if (self.state.current_char != null) {
                    self.state.advance(); // consume escaped character
                }
            } else {
                self.state.advance();
            }
        }
        
        if (self.state.current_char == '"') {
            self.state.advance(); // consume closing quote
        }
        
        try self.addToken(.StringLiteral, start_pos, start_line, start_column);
    }
    
    fn scanCharacter(self: *AdvancedLexer) !void {
        const start_pos = self.state.position;
        const start_line = self.state.line;
        const start_column = self.state.column;
        
        self.state.advance(); // consume opening quote
        
        if (self.state.current_char == '\\') {
            self.state.advance(); // consume backslash
            if (self.state.current_char != null) {
                self.state.advance(); // consume escaped character
            }
        } else if (self.state.current_char != null) {
            self.state.advance(); // consume character
        }
        
        if (self.state.current_char == '\'') {
            self.state.advance(); // consume closing quote
        }
        
        try self.addToken(.CharacterLiteral, start_pos, start_line, start_column);
    }
    
    fn scanNumber(self: *AdvancedLexer) !void {
        const start_pos = self.state.position;
        const start_line = self.state.line;
        const start_column = self.state.column;
        
        // Consume digits
        while (self.state.current_char != null and std.ascii.isDigit(self.state.current_char.?)) {
            self.state.advance();
        }
        
        var is_float = false;
        
        // Check for decimal point
        if (self.state.current_char == '.' and self.state.peekChar() != null and std.ascii.isDigit(self.state.peekChar().?)) {
            is_float = true;
            self.state.advance(); // consume '.'
            
            // Consume fractional digits
            while (self.state.current_char != null and std.ascii.isDigit(self.state.current_char.?)) {
                self.state.advance();
            }
        }
        
        // Check for scientific notation
        if (self.state.current_char == 'e' or self.state.current_char == 'E') {
            is_float = true;
            self.state.advance(); // consume 'e' or 'E'
            
            if (self.state.current_char == '+' or self.state.current_char == '-') {
                self.state.advance(); // consume sign
            }
            
            while (self.state.current_char != null and std.ascii.isDigit(self.state.current_char.?)) {
                self.state.advance();
            }
        }
        
        // Check for numeric suffixes
        if (self.state.current_char != null and std.ascii.isAlphabetic(self.state.current_char.?)) {
            while (self.state.current_char != null and (std.ascii.isAlphanumeric(self.state.current_char.?) or self.state.current_char == '_')) {
                self.state.advance();
            }
        }
        
        const token_kind = if (is_float) TokenKind.FloatLiteral else TokenKind.IntegerLiteral;
        try self.addToken(token_kind, start_pos, start_line, start_column);
    }
    
    fn scanIdentifier(self: *AdvancedLexer) !void {
        const start_pos = self.state.position;
        const start_line = self.state.line;
        const start_column = self.state.column;
        
        // Consume identifier characters
        while (self.state.current_char != null and 
               (std.ascii.isAlphanumeric(self.state.current_char.?) or self.state.current_char == '_')) {
            self.state.advance();
        }
        
        const text = self.state.input[start_pos..self.state.position];
        
        // Check if it's a keyword
        if (self.keywords.get(text)) |token_kind| {
            try self.addToken(token_kind, start_pos, start_line, start_column);
        } else {
            try self.addToken(.Identifier, start_pos, start_line, start_column);
        }
    }
    
    fn addToken(self: *AdvancedLexer, token_kind: TokenKind, start_pos: u32, line: u32, column: u32) !void {
        const lexeme = self.state.input[start_pos..self.state.position];
        const token = Token.init(token_kind, lexeme, line, column, start_pos);
        try self.tokens.append(token);
    }
};

/// Utility functions for token analysis
pub fn isAssignmentOperator(token_kind: TokenKind) bool {
    return switch (token_kind) {
        .Equal, .PlusEqual, .MinusEqual, .StarEqual, .SlashEqual, .PercentEqual,
        .AmpersandEqual, .PipeEqual, .CaretEqual, .LeftShiftEqual, .RightShiftEqual => true,
        else => false,
    };
}

pub fn isBinaryOperator(token_kind: TokenKind) bool {
    return switch (token_kind) {
        .Plus, .Minus, .Star, .Slash, .Percent, .StarStar,
        .EqualEqual, .BangEqual, .Less, .LessEqual, .Greater, .GreaterEqual, .Spaceship,
        .AmpersandAmpersand, .PipePipe, .Ampersand, .Pipe, .Caret,
        .LeftShift, .RightShift => true,
        else => false,
    };
}

pub fn isUnaryOperator(token_kind: TokenKind) bool {
    return switch (token_kind) {
        .Plus, .Minus, .Bang, .Tilde, .Star, .Ampersand => true,
        else => false,
    };
}

pub fn getOperatorPrecedence(token_kind: TokenKind) u8 {
    return switch (token_kind) {
        .PipePipe => 1,
        .AmpersandAmpersand => 2,
        .Pipe => 3,
        .Caret => 4,
        .Ampersand => 5,
        .EqualEqual, .BangEqual => 6,
        .Less, .LessEqual, .Greater, .GreaterEqual, .Spaceship => 7,
        .LeftShift, .RightShift => 8,
        .Plus, .Minus => 9,
        .Star, .Slash, .Percent => 10,
        .StarStar => 11,
        else => 0,
    };
}

pub fn isRightAssociative(token_kind: TokenKind) bool {
    return switch (token_kind) {
        .StarStar => true, // Exponentiation is right-associative
        else => false,
    };
}
