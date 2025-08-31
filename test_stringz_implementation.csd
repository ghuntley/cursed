fr fr Test the new stringz implementation
yeet "stringz"

fr fr Test length function
sus len1 drip = stringz.length("hello")
sus len2 drip = stringz.length("world") 
sus len3 drip = stringz.length("CURSED")
sus len4 drip = stringz.length("hello world")
sus len_empty drip = stringz.length("")

vibez.spill("Length tests:")
vibez.spill("'hello' length: " + len1)
vibez.spill("'world' length: " + len2)
vibez.spill("'CURSED' length: " + len3)
vibez.spill("'hello world' length: " + len4)
vibez.spill("'' length: " + len_empty)

fr fr Test concat function
sus concat1 tea = stringz.concat("hello", "world")
sus concat2 tea = stringz.concat("Hello ", "World")
sus concat3 tea = stringz.concat("CURSED", "stdlib")
sus concat_empty tea = stringz.concat("", "test")

vibez.spill("\nConcat tests:")
vibez.spill("'hello' + 'world': " + concat1)
vibez.spill("'Hello ' + 'World': " + concat2)
vibez.spill("'CURSED' + 'stdlib': " + concat3)
vibez.spill("'' + 'test': " + concat_empty)

fr fr Test contains function
sus contains1 lit = stringz.contains("hello world", "world")
sus contains2 lit = stringz.contains("hello world", "hello")
sus contains3 lit = stringz.contains("test", "es")
sus contains4 lit = stringz.contains("CURSED", "CUR")
sus contains_false lit = stringz.contains("hello", "xyz")

vibez.spill("\nContains tests:")
vibez.spill("'hello world' contains 'world': " + contains1)
vibez.spill("'hello world' contains 'hello': " + contains2)
vibez.spill("'test' contains 'es': " + contains3)
vibez.spill("'CURSED' contains 'CUR': " + contains4)
vibez.spill("'hello' contains 'xyz': " + contains_false)

fr fr Test upper function
sus upper1 tea = stringz.upper("hello")
sus upper2 tea = stringz.upper("world")
sus upper3 tea = stringz.upper("Hello World")

vibez.spill("\nUpper tests:")
vibez.spill("upper('hello'): " + upper1)
vibez.spill("upper('world'): " + upper2)
vibez.spill("upper('Hello World'): " + upper3)

fr fr Test lower function
sus lower1 tea = stringz.lower("HELLO")
sus lower2 tea = stringz.lower("WORLD")
sus lower3 tea = stringz.lower("Hello World")

vibez.spill("\nLower tests:")
vibez.spill("lower('HELLO'): " + lower1)
vibez.spill("lower('WORLD'): " + lower2)
vibez.spill("lower('Hello World'): " + lower3)

fr fr Test trim function
sus trim1 tea = stringz.trim("  hello  ")
sus trim2 tea = stringz.trim(" test ")
sus trim3 tea = stringz.trim("hello")

vibez.spill("\nTrim tests:")
vibez.spill("trim('  hello  '): '" + trim1 + "'")
vibez.spill("trim(' test '): '" + trim2 + "'")
vibez.spill("trim('hello'): '" + trim3 + "'")

fr fr Test split function
sus split1 drip = stringz.split("a,b,c", ",")
sus split2 drip = stringz.split("hello world", " ")
sus split3 drip = stringz.split("one,two,three,four", ",")

vibez.spill("\nSplit tests (returns count):")
vibez.spill("split('a,b,c', ','): " + split1)
vibez.spill("split('hello world', ' '): " + split2)
vibez.spill("split('one,two,three,four', ','): " + split3)

vibez.spill("\n✅ Stringz implementation tests completed!")
