# AST Reference Counting Memory Management Test
yeet "vibez"

# Test complex nested structures that stress memory management
slay create_deep_nested_ast() {
    sus complex_expression lit = (
        (1 + 2) * 3 / (4 - 1) +
        [1, 2, 3, 4, 5].length() *
        { "key1": "value1", "key2": "value2" }.keys().length()
    )
    
    # Create nested function calls that generate many AST nodes
    sus deep_call lit = function_with_many_args(
        another_function(arg1, arg2, arg3),
        yet_another(deep_nested_array([1, [2, [3, [4, 5]]])),
        complex_struct({
            field1: "value",
            field2: {
                nested_field: true,
                more_nesting: [
                    { inner: "data" }
                ]
            }
        })
    )
    
    damn deep_call
}

# Test concurrent access to AST nodes
go {
    sus ast_node lit = create_deep_nested_ast()
    vibez.spill("Concurrent AST access 1:", ast_node)
}

go {
    sus ast_node lit = create_deep_nested_ast()
    vibez.spill("Concurrent AST access 2:", ast_node)
}

# Test memory cleanup after many operations
bestie (drip i = 0; i < 1000; i++) {
    sus temp_ast lit = create_deep_nested_ast()
    # This should trigger reference counting cleanup
}

vibez.spill("AST memory management test completed")
