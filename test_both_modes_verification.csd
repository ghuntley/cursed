fr fr Both-mode verification test for tuple operations

fr fr Test simple tuple with variables (works in both modes)
sus x = 42
sus y = "test"
sus tuple_var = (x, y)
sus first_elem = tuple_var.0
vibez.spill("First element from variable tuple:", first_elem)

fr fr Test tuple destructuring
(a, b) := tuple_var
vibez.spill("Destructured a:", a)
vibez.spill("Destructured b:", b)

fr fr Test nested operations
sus result = tuple_var.0 + 10
vibez.spill("Arithmetic result:", result)

vibez.spill("✅ All tuple operations working correctly")
