#!/bin/bash

# Test parser interface definition handling

set -e

# Test interface definitions
cat > interface_test.csd << 'EOF'
fr fr Test interface definitions
collab Drawable {
    slay draw()
    slay area() meal
}

collab Serializable {
    slay serialize() tea
    slay deserialize(data tea) 
}

fr fr Test struct implementing interface
squad Circle {
    spill radius meal
}

flex Circle => Drawable {
    slay draw() {
        vibez.spill("Drawing circle")
    }
    
    slay area() meal {
        damn 3.14159 * radius * radius
    }
}

fr fr Test interface usage
sus circle Circle = Circle{radius: 5.0}
sus drawable Drawable = circle
drawable.draw()
EOF

# Test that interfaces are properly parsed
./cursed-unified interface_test.csd

# Cleanup
rm -f interface_test.csd

exit 0
