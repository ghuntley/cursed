# Generic Syntax Preprocessor

This document describes the preprocessor implementation for handling generic syntax in the CURSED programming language.

## Processing Flow

The preprocessor sits between the lexer and parser, enhancing tokens with contextual information about generic syntax.

```mermaid
graph TD
    A[Source Code] --> B[Lexer]
    B --> C[Token Stream]
    C --> D[Preprocessor]
    D --> E[Enhanced Token Stream]
    E --> F[Parser]
    F --> G[AST]
    
    subgraph "Preprocessor Component"
        D1["Buffer Tokens"] --> D2["Identify Patterns"]
        D2 --> D3["Process Generic Syntax"]
        D3 --> D4["Create Enhanced Tokens"]
        D4 --> D5["Handle Errors"]
    end
    
    D --> D1
    D5 --> E
```

## Token Processing Sequence

The following sequence diagram shows how tokens flow through the system:

```mermaid
sequenceDiagram
    participant SC as Source Code
    participant L as Lexer
    participant P as Preprocessor
    participant TS as TokenStream
    participant PA as Parser
    
    SC->>L: Input text
    L->>P: Raw tokens
    
    Note over P: Token buffer analysis
    
    P->>P: Identify generic type declaration
    P->>P: Identify generic function declaration
    P->>P: Identify generic function call
    P->>P: Process nested generics
    
    P->>TS: Enhanced tokens with metadata
    TS->>PA: Structured token stream
    PA->>PA: Process with context awareness
    
    Note over PA: Future Enhancement
```

## Component Structure

The preprocessor consists of several components working together:

```mermaid
classDiagram
    class Preprocessor {
        +lexer: Lexer
        +token_buffer: Vec<Token>
        +token_stream: TokenStream
        +new(lexer)
        +process()
        -process_buffer()
        -process_generic_type_declaration()
        -process_generic_function_declaration()
        -process_generic_function_call()
    }
    
    class TokenStream {
        +tokens: Vec<TokenWithContext>
        +position: usize
        +add_token()
        +add_token_with_metadata()
        +contains_generic_type_declaration()
        +contains_generic_function_declaration()
        +contains_generic_function_call()
        +contains_nested_generic_type()
        +contains_separate_brackets()
    }
    
    class TokenWithContext {
        +token: Token
        +location: SourceLocation
        +metadata: Option<TokenMetadata>
    }
    
    class TokenMetadata {
        <<enumeration>>
        GenericType
        GenericFunction
        GenericFunctionCall
        NestedGenericType
    }
    
    Preprocessor --> TokenStream : creates
    TokenStream --> TokenWithContext : contains
    TokenWithContext --> TokenMetadata : may have
```

## Generic Syntax Support

The preprocessor handles the following generic syntax patterns:

1. Generic type declarations:
   ```
   be_like Box[T] squad { ... }
   ```

2. Generic function declarations:
   ```
   slay foo[T](x normie) T { ... }
   ```

3. Generic function calls:
   ```
   foo[normie](42)
   ```

4. Nested generic types:
   ```
   be_like Pair[K, V[T]] squad { ... }
   ```

## Error Handling

The preprocessor provides detailed error messages for malformed generic syntax:

- Unclosed type parameter brackets
- Unexpected tokens in generic declarations
- Missing required tokens after type parameters