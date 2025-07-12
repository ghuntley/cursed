sus tuple_basic := (1, "hello", based)
sus tuple_nested := ((1, 2), (3, 4))
sus tuple_mixed := (42, 3.14, 'x')

vibez.spill("Basic tuple:", tuple_basic)
vibez.spill("First element:", tuple_basic.0)
vibez.spill("Second element:", tuple_basic.1)
vibez.spill("Third element:", tuple_basic.2)

vibez.spill("Nested tuple:", tuple_nested)
vibez.spill("First nested element:", tuple_nested.0.0)
vibez.spill("Second nested element:", tuple_nested.1.1)

vibez.spill("Mixed tuple:", tuple_mixed)

(a, b, c) := tuple_basic
vibez.spill("Destructured a:", a)
vibez.spill("Destructured b:", b)
vibez.spill("Destructured c:", c)

(x, y) := (100, 200)
vibez.spill("Direct destructure x:", x)
vibez.spill("Direct destructure y:", y)
