fr fr Comprehensive tuple test for both interpretation and compilation modes

fr fr Test basic tuple creation
sus simple_tuple = (42, "hello", based)
vibez.spill("Created tuple:", simple_tuple)

fr fr Test tuple with variables 
sus x = 10
sus y = 20
sus var_tuple = (x, y)
vibez.spill("Variable tuple created")

fr fr Test tuple access
sus first = simple_tuple.0
sus second = simple_tuple.1
sus third = simple_tuple.2
vibez.spill("Tuple elements:", first, second, third)

fr fr Test arithmetic with tuple elements
sus sum = var_tuple.0 + var_tuple.1
vibez.spill("Sum:", sum)

fr fr Test tuple destructuring
(a, b) := var_tuple
vibez.spill("Destructured:", a, b)

vibez.spill("All tuple operations completed successfully!")
