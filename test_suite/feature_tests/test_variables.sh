#!/bin/bash

# Test variable feature functionality

set -e

# Test all variable features
cat > variable_features_test.csd << 'EOF'
fr fr Test variable features

fr fr Basic variable declarations
sus int_var drip = 42
sus float_var meal = 3.14159
sus string_var tea = "Hello, CURSED!"
sus bool_var lit = based

fr fr Short variable declarations
x := 100
name := "World"
active := cringe

fr fr Tuple assignment
(a, b, c) := (1, 2.5, "three")

fr fr Variable reassignment
int_var = 84
float_var = 2.71828
string_var = "Updated"
bool_var = cringe

fr fr Array variables
sus numbers []drip = [1, 2, 3, 4, 5]
sus names []tea = ["Alice", "Bob", "Charlie"]

fr fr Map variables
sus ages map[tea]drip = {"Alice": 25, "Bob": 30}

fr fr Output all variables
vibez.spill("int_var:", int_var)
vibez.spill("float_var:", float_var)
vibez.spill("string_var:", string_var)
vibez.spill("bool_var:", bool_var)
vibez.spill("x:", x)
vibez.spill("name:", name)
vibez.spill("active:", active)
vibez.spill("a:", a)
vibez.spill("b:", b)
vibez.spill("c:", c)
vibez.spill("numbers[0]:", numbers[0])
vibez.spill("names[0]:", names[0])
vibez.spill("ages[\"Alice\"]:", ages["Alice"])
EOF

# Test interpretation mode
./cursed-unified variable_features_test.csd

# Test compilation mode
./cursed-unified --compile variable_features_test.csd
if [ -f ./variable_features_test ]; then
    ./variable_features_test
    rm -f variable_features_test
fi

# Cleanup
rm -f variable_features_test.csd

exit 0
