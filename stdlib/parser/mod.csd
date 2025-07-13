yeet "testz"
yeet "core"
yeet "stringz"

# Token types for lexical analysis
sus TokenType := normie

# Token type constants
facts TokenIdentifier := 1
facts TokenNumber := 2
facts TokenString := 3
facts TokenKeyword := 4
facts TokenOperator := 5
facts TokenDelimiter := 6
facts TokenComment := 7
facts TokenEOF := 8
facts TokenError := 9

# AST Node types
sus ASTNodeType := normie

facts NodeProgram := 1
facts NodeStatement := 2
facts NodeExpression := 3
facts NodeDeclaration := 4
facts NodeFunction := 5
facts NodeVariable := 6
facts NodeBinary := 7
facts NodeUnary := 8
facts NodeCall := 9
facts NodeLiteral := 10

# Lexical analysis - tokenize source code
slay tokenize(source tea) []Token {
    sus tokens []Token
    sus line := 1
    sus column := 1
    sus i := 0
    
    while i < stringz.length(source) {
        sus ch := stringz.char_at(source, i)
        
        # Skip whitespace
        if stringz.is_whitespace(ch) {
            if ch == '\n' {
                line++
                column = 1
            } else {
                column++
            }
            i++
            continue
        }
        
        # Handle identifiers and keywords
        if stringz.is_alpha(ch) {
            sus start := i
            while i < stringz.length(source) && (stringz.is_alnum(stringz.char_at(source, i)) || stringz.char_at(source, i) == '_') {
                i++
            }
            sus value := stringz.substring(source, start, i)
            sus token_type := if is_keyword(value) { TokenKeyword } else { TokenIdentifier }
            
            sus token := Token{token_type, value, line, column}
            tokens = append(tokens, token)
            column += i - start
            continue
        }
        
        # Handle numbers
        if stringz.is_digit(ch) {
            sus start := i
            while i < stringz.length(source) && (stringz.is_digit(stringz.char_at(source, i)) || stringz.char_at(source, i) == '.') {
                i++
            }
            sus value := stringz.substring(source, start, i)
            sus token := Token{TokenNumber, value, line, column}
            tokens = append(tokens, token)
            column += i - start
            continue
        }
        
        # Handle strings
        if ch == '"' {
            sus start := i
            i++ # Skip opening quote
            while i < stringz.length(source) && stringz.char_at(source, i) != '"' {
                i++
            }
            if i < stringz.length(source) {
                i++ # Skip closing quote
            }
            sus value := stringz.substring(source, start + 1, i - 1)
            sus token := Token{TokenString, value, line, column}
            tokens = append(tokens, token)
            column += i - start
            continue
        }
        
        # Handle operators and delimiters
        if is_operator(ch) {
            sus value := stringz.char_to_string(ch)
            sus token := Token{TokenOperator, value, line, column}
            tokens = append(tokens, token)
            column++
            i++
            continue
        }
        
        if is_delimiter(ch) {
            sus value := stringz.char_to_string(ch)
            sus token := Token{TokenDelimiter, value, line, column}
            tokens = append(tokens, token)
            column++
            i++
            continue
        }
        
        # Handle comments
        if ch == '#' {
            sus start := i
            while i < stringz.length(source) && stringz.char_at(source, i) != '\n' {
                i++
            }
            sus value := stringz.substring(source, start, i)
            sus token := Token{TokenComment, value, line, column}
            tokens = append(tokens, token)
            continue
        }
        
        # Unknown character - create error token
        sus value := stringz.char_to_string(ch)
        sus token := Token{TokenError, value, line, column}
        tokens = append(tokens, token)
        column++
        i++
    }
    
    # Add EOF token
    sus eof_token := Token{TokenEOF, "", line, column}
    tokens = append(tokens, eof_token)
    
    damn tokens
}

# Check if string is a keyword
slay is_keyword(value tea) lit {
    damn value == "sus" || value == "slay" || value == "damn" || 
         value == "if" || value == "else" || value == "while" ||
         value == "yeet" || value == "vibez" || value == "based" ||
         value == "cringe" || value == "facts" || value == "be_like"
}

# Check if character is an operator
slay is_operator(ch sip) lit {
    damn ch == '+' || ch == '-' || ch == '*' || ch == '/' ||
         ch == '=' || ch == '!' || ch == '<' || ch == '>' ||
         ch == '&' || ch == '|' || ch == '^' || ch == '%'
}

# Check if character is a delimiter
slay is_delimiter(ch sip) lit {
    damn ch == '(' || ch == ')' || ch == '{' || ch == '}' ||
         ch == '[' || ch == ']' || ch == ';' || ch == ',' ||
         ch == '.' || ch == ':' || ch == '?'
}

# Main parsing entry point
slay parse_source(source tea) (*ASTNode, []tea) {
    sus tokens := tokenize(source)
    sus errors := make([]tea, 0)
    
    # Create simple AST node for now
    sus ast := &ASTNode{
        node_type: NodeProgram,
        value: "program",
        children: make([]*ASTNode, 0),
        line: 1,
        column: 1
    }
    
    damn ast, errors
}

# Type inference for expressions
slay infer_type(node *ASTNode, parser *Parser) tea {
    if node == cringe {
        damn "void"
    }
    
    if node.node_type == NodeLiteral {
        # Infer type from literal value
        if stringz.is_numeric(node.value) {
            if stringz.contains(node.value, ".") {
                damn "meal"
            } else {
                damn "normie"
            }
        }
        
        if node.value == "based" || node.value == "cap" {
            damn "lit"
        }
        
        damn "tea"
    }
    
    damn "void"
}
