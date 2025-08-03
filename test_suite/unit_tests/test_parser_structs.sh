#!/bin/bash

# Test parser struct definition handling

set -e

# Test struct definitions
cat > struct_test.csd << 'EOF'
fr fr Test struct definitions
squad Point {
    spill x meal
    spill y meal
}

squad Person {
    spill name tea
    spill age drip
    spill active lit
}

fr fr Test struct instantiation
sus point Point = Point{x: 1.0, y: 2.0}
sus person Person = Person{
    name: "Alice",
    age: 30,
    active: based
}

fr fr Test field access
sus x_coord meal = point.x
sus person_name tea = person.name
EOF

# Test that structs are properly parsed
./cursed-unified struct_test.csd

# Cleanup
rm -f struct_test.csd

exit 0
