yeet "stringz"
yeet "arrayz"

# Test self-hosting compiler constructs
squad Token {
    spill type tea
    spill value tea  
    spill line drip
}

slay tokenize(source tea) []Token {
    # Simple tokenization simulation
    sus tokens []Token = []
    
    # Mock tokenization - in real implementation this would parse the source
    sus token Token = Token{type: "KEYWORD", value: "slay", line: 1}
    
    vibez.spill("Tokenizing: " + source)
    vibez.spill("Found token: " + token.type + " = " + token.value)
    
    damn tokens
}

# Test the tokenizer
sus test_source tea = "slay main() { }"
sus result []Token = tokenize(test_source)
vibez.spill("Tokenization complete")
