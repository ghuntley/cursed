yeet "stringz"

vibez.spill("=== Comprehensive StringZ Test ===")

fr Test length function
sus len1 normie = stringz.length("hello")
sus len2 normie = stringz.length("")
sus len3 normie = stringz.length("test")
vibez.spill("Length of 'hello':", len1)
vibez.spill("Length of '':", len2)  
vibez.spill("Length of 'test':", len3)

fr Test concatenation
sus concat1 tea = stringz.concat("hello", "world")
sus concat2 tea = stringz.concat("", "test")
sus concat3 tea = stringz.concat("test", "")
vibez.spill("Concat 'hello' + 'world':", concat1)
vibez.spill("Concat '' + 'test':", concat2)
vibez.spill("Concat 'test' + '':", concat3)

fr Test contains
sus contains1 lit = stringz.contains("hello world", "world")
sus contains2 lit = stringz.contains("hello", "xyz")
sus contains3 lit = stringz.contains("programming", "gram")
vibez.spill("'hello world' contains 'world':", contains1)
vibez.spill("'hello' contains 'xyz':", contains2)
vibez.spill("'programming' contains 'gram':", contains3)

vibez.spill("=== All tests completed ===")
