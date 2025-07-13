// Test self-hosting error handling with comprehensive error context

slay parse_code(code tea) normie {
    // Simulate parser error
    bestie code == "bad_syntax" {
        yikes parse_error := "Parse error: invalid syntax";
        damn 0;
    }
    damn 1;
}

slay compile_code(parse_result normie) normie {
    // Test error propagation
    bestie parse_result == 0 {
        propagated_error := shook parse_code("bad_syntax");
        damn propagated_error;
    }
    damn 1;
}

slay self_hosting_test() lit {
    fam compilation_error {
        // This should trigger error recovery
        compile_result := compile_code(0);
        vibez.spill("Compilation successful");
    };
    
    vibez.spill("Error recovered successfully");
    damn based;
}

// Test error handling in self-hosting compiler
vibez.spill("Testing self-hosting error handling");
result := self_hosting_test();
vibez.spill("Test completed");
vibez.spill(result);
