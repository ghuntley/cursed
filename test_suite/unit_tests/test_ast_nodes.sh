#!/bin/bash

# Test AST node creation and structure

set -e

# Test AST structure with complex program
cat > ast_test.csd << 'EOF'
fr fr Test AST node creation
yeet "testz"

squad TestStruct {
    spill value drip
}

slay test_function(param drip) drip {
    sus local_var drip = param + 1
    
    bestie local_var > 0 {
        local_var = local_var - 1
    }
    
    match local_var {
        0 => damn 100
        x if x > 10 => damn x * 2
        _ => damn local_var
    }
}

sus instance TestStruct = TestStruct{value: 42}
sus result drip = test_function(instance.value)
vibez.spill(result)
EOF

# Test that complex AST is properly created
./cursed-unified ast_test.csd

# Cleanup
rm -f ast_test.csd

exit 0
