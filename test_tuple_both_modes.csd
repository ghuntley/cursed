fr fr Test tuple operations in both interpretation and compilation modes
sus tuple1 (normie, tea, lit) = (42, "hello", based)
sus tuple2 (normie, normie) = (10, 20)

fr fr Test tuple access
sus first normie = tuple1.0
sus message tea = tuple1.1
sus flag lit = tuple1.2

fr fr Test arithmetic with tuple elements
sus sum normie = tuple2.0 + tuple2.1

vibez.spill("Tuple access tests:")
vibez.spill("First:", first)
vibez.spill("Message:", message)
vibez.spill("Flag:", flag)
vibez.spill("Sum:", sum)

fr fr Test tuple destructuring
(x, y) := tuple2
vibez.spill("Destructured values:", x, y)

fr fr Test nested tuples
sus nested ((normie, normie), (normie, normie)) = ((1, 2), (3, 4))
sus inner_first (normie, normie) = nested.0
sus value normie = inner_first.1
vibez.spill("Nested value:", value)
