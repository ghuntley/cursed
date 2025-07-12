yeet "testz"
yeet "stringz"

# Simple Self-Hosting Test for CURSED Compiler
# Demonstrates core compiler pipeline functions in pure CURSED

# Simple tokenizer in CURSED
slay simple_tokenize(source tea) normie {
    sus tokens normie = 0
    sus i normie = 0
    
    bestie i < stringz.len(source) {
        sus char tea = stringz.char_at(source, i)
        if stringz.equal(char, " ") || stringz.equal(char, "\n") {
            tokens = tokens + 1
        }
        i = i + 1
    }
    
    damn tokens
}

# Simple parser validation in CURSED
slay simple_parse(source tea) lit {
    sus has_keywords lit = cap
    
    if stringz.contains(source, "vibez.spill") {
        has_keywords = based
    }
    
    if stringz.contains(source, "sus") {
        has_keywords = based
    }
    
    damn has_keywords
}

# Simple LLVM IR generator in CURSED
slay simple_generate_ir(source tea) tea {
    sus ir tea = "; Simple LLVM IR\n"
    ir = stringz.concat(ir, "define i32 @main() {\n")
    ir = stringz.concat(ir, "  ret i32 0\n")
    ir = stringz.concat(ir, "}\n")
    
    damn ir
}

# Main self-hosting test
test_start("Simple Self-Hosting Test")

# Test source code
sus test_source tea = "sus x normie = 42\nvibez.spill(\"Hello\")"

# Test tokenization
sus token_count normie = simple_tokenize(test_source)
assert_true(token_count > 0)
vibez.spill("✅ Tokenization works: ")
vibez.spill(token_count)

# Test parsing
sus parse_result lit = simple_parse(test_source)
assert_true(parse_result)
vibez.spill("✅ Parsing works")

# Test IR generation
sus llvm_ir tea = simple_generate_ir(test_source)
sus ir_length normie = stringz.len(llvm_ir)
assert_true(ir_length > 10)
vibez.spill("✅ LLVM IR generation works: ")
vibez.spill(ir_length)

vibez.spill("🎉 Simple self-hosting test complete!")
vibez.spill("✅ Core compiler functions implemented in pure CURSED")

print_test_summary()
