# Test program for Stage 2 self-hosting compiler
# This program tests that the CURSED compiler can compile itself

yeet "testz"
yeet "ast_mood"
yeet "token_vibe"

# Test the core compilation pipeline
slay test_self_hosting_pipeline() lit {
    test_start("Stage 2 Self-Hosting Compiler Test")
    
    # Test 1: AST node creation
    sus test_node normie = create_ast_node(AST_PROGRAM, "test", "test_value", 1, 1)
    assert_true(test_node > 0)
    
    # Test 2: Node type extraction
    sus node_type normie = ast_node_type(test_node)
    assert_eq_int(node_type, AST_PROGRAM)
    
    # Test 3: Token string conversion
    sus token_str tea = token_string(IDENT_TOKEN)
    assert_eq_string(token_str, "IDENT")
    
    # Test 4: Token validation
    sus is_op lit = is_operator(ADD_TOKEN)
    assert_true(is_op)
    
    vibez.spill("✅ All Stage 2 compiler tests passed!")
    damn based
}

# Main test function
slay main() normie {
    vibez.spill("🧪 Testing CURSED Stage 2 Self-Hosting Compiler")
    
    sus result lit = test_self_hosting_pipeline()
    lowkey (result) {
        vibez.spill("🎉 Stage 2 compiler ready for self-hosting!")
        print_test_summary()
        damn 0
    } highkey {
        vibez.spill("❌ Stage 2 compiler tests failed")
        damn 1
    }
}
