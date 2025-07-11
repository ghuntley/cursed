yeet "string_pure"

vibez.spill("Testing Pure CURSED String Library")

// Test string length
sus len1 normie = string_len("hello")
sus len2 normie = string_len("")
sus len3 normie = string_len("CURSED")

vibez.spill("Length tests:")
vibez.spill("hello length: " + tea(len1))
vibez.spill("empty length: " + tea(len2))
vibez.spill("CURSED length: " + tea(len3))

// Test empty string check
sus empty1 lit = string_is_empty("")
sus empty2 lit = string_is_empty("hello")

vibez.spill("Empty tests:")
vibez.spill("empty string is empty: " + tea(empty1))
vibez.spill("hello is empty: " + tea(empty2))

// Test case conversion
sus upper1 tea = string_to_upper("a")
sus upper2 tea = string_to_upper("z")
sus lower1 tea = string_to_lower("A")
sus lower2 tea = string_to_lower("Z")

vibez.spill("Case conversion tests:")
vibez.spill("a to upper: " + upper1)
vibez.spill("z to upper: " + upper2)
vibez.spill("A to lower: " + lower1)
vibez.spill("Z to lower: " + lower2)

// Test string contains
sus contains1 lit = string_contains("hello world", "world")
sus contains2 lit = string_contains("hello world", "hello")
sus contains3 lit = string_contains("hello world", "xyz")

vibez.spill("Contains tests:")
vibez.spill("hello world contains world: " + tea(contains1))
vibez.spill("hello world contains hello: " + tea(contains2))
vibez.spill("hello world contains xyz: " + tea(contains3))

// Test string indexing
sus index1 normie = string_index_of("hello world", "world")
sus index2 normie = string_index_of("hello world", "hello")
sus index3 normie = string_index_of("hello world", "xyz")

vibez.spill("Index tests:")
vibez.spill("hello world index of world: " + tea(index1))
vibez.spill("hello world index of hello: " + tea(index2))
vibez.spill("hello world index of xyz: " + tea(index3))

// Test type conversion
sus int1 normie = string_to_int("123")
sus int2 normie = string_to_int("-456")
sus int3 normie = string_to_int("0")

vibez.spill("Integer conversion tests:")
vibez.spill("123 to int: " + tea(int1))
vibez.spill("-456 to int: " + tea(int2))
vibez.spill("0 to int: " + tea(int3))

// Test string from types
sus str1 tea = string_from_int(123)
sus str2 tea = string_from_int(-456)
sus str3 tea = string_from_bool(based)
sus str4 tea = string_from_bool(cap)

vibez.spill("String from types tests:")
vibez.spill("123 from int: " + str1)
vibez.spill("-456 from int: " + str2)
vibez.spill("true from bool: " + str3)
vibez.spill("false from bool: " + str4)

vibez.spill("Pure CURSED String Library tests completed!")
