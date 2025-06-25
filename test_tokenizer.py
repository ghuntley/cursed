#!/usr/bin/env python3

# Simple test to validate our CURSED tokenizer logic

source = """
facts x = 42;
facts name = "CURSED";
slay greet(name) {
    facts greeting = "Hello";
}
"""

# Simulate the tokenizer logic
def tokenize(source):
    tokens = []
    i = 0
    line = 1
    column = 1
    
    while i < len(source):
        ch = source[i]
        
        # Skip whitespace
        if ch in ' \t\n\r':
            if ch == '\n':
                line += 1
                column = 1
            else:
                column += 1
            i += 1
            continue
        
        # Single character tokens
        if ch == '=':
            tokens.append(('Assign', '=', line, column))
        elif ch == '+':
            tokens.append(('Plus', '+', line, column))
        elif ch == '-':
            tokens.append(('Minus', '-', line, column))
        elif ch == '(':
            tokens.append(('LeftParen', '(', line, column))
        elif ch == ')':
            tokens.append(('RightParen', ')', line, column))
        elif ch == '{':
            tokens.append(('LeftBrace', '{', line, column))
        elif ch == '}':
            tokens.append(('RightBrace', '}', line, column))
        elif ch == ',':
            tokens.append(('Comma', ',', line, column))
        elif ch == ';':
            tokens.append(('Semicolon', ';', line, column))
        
        # String literals
        elif ch == '"':
            start = i + 1
            i += 1
            while i < len(source) and source[i] != '"':
                i += 1
            if i < len(source):
                literal = source[start:i]
                tokens.append(('String', literal, line, column))
            else:
                tokens.append(('Illegal', '"', line, column))
        
        # Identifiers and keywords
        elif ch.isalpha() or ch == '_':
            start = i
            while i < len(source) and (source[i].isalnum() or source[i] == '_'):
                i += 1
            literal = source[start:i]
            
            # Check for keywords
            if literal == 'facts':
                tokens.append(('Facts', literal, line, column))
            elif literal == 'slay':
                tokens.append(('Slay', literal, line, column))
            elif literal == 'lowkey':
                tokens.append(('Lowkey', literal, line, column))
            elif literal == 'yeet':
                tokens.append(('Yeet', literal, line, column))
            elif literal in ['true', 'false']:
                tokens.append(('Boolean', literal, line, column))
            else:
                tokens.append(('Identifier', literal, line, column))
            i -= 1  # Back up one since we'll increment at end of loop
        
        # Numbers
        elif ch.isdigit():
            start = i
            while i < len(source) and source[i].isdigit():
                i += 1
            literal = source[start:i]
            tokens.append(('Integer', literal, line, column))
            i -= 1  # Back up one since we'll increment at end of loop
        
        else:
            tokens.append(('Illegal', ch, line, column))
        
        i += 1
        column += 1
    
    return tokens

print("🔤 Testing CURSED Tokenizer Logic")
print("Source code:")
print(source)
print()

tokens = tokenize(source)
print(f"✅ Found {len(tokens)} tokens:")
for i, (token_type, literal, line, col) in enumerate(tokens):
    print(f"  {i + 1}: {token_type} '{literal}' at {line}:{col}")

print()
print("🎉 Tokenizer test completed!")
