#!/bin/bash

# Test parser function definition handling

set -e

# Test function definitions
cat > function_test.csd << 'EOF'
fr fr Test function definitions
slay simple_function() {
    vibez.spill("hello")
}

slay function_with_params(x drip, name tea) {
    vibez.spill(name)
}

slay function_with_return(x drip) drip {
    damn x * 2
}

slay complex_function(a drip, b meal, name tea) tea {
    sus result tea = name + " result"
    damn result
}

fr fr Test function calls
simple_function()
sus value drip = function_with_return(21)
EOF

# Test that functions are properly parsed
./cursed-unified function_test.csd

# Cleanup
rm -f function_test.csd

exit 0
