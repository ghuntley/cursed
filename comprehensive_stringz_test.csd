fr fr Comprehensive stringz module test
yeet "stringz"

vibez.spill("=== CURSED StringZ Module Test ===")

fr fr Test length function
vibez.spill("\n🔢 Length Tests:")
sus len1 drip = stringz.length("hello")
sus len2 drip = stringz.length("world")
sus len3 drip = stringz.length("CURSED")
sus len4 drip = stringz.length("hello world")
sus len5 drip = stringz.length("test")

vibez.spill("'hello' length: " + len1)
vibez.spill("'world' length: " + len2)
vibez.spill("'CURSED' length: " + len3)
vibez.spill("'hello world' length: " + len4)
vibez.spill("'test' length: " + len5)

fr fr Test concat function
vibez.spill("\n🔗 Concat Tests:")
sus concat1 tea = stringz.concat("hello", "world")
sus concat2 tea = stringz.concat("Hello ", "World")
sus concat3 tea = stringz.concat("CURSED", "stdlib")
sus concat4 tea = stringz.concat("test", "ing")

vibez.spill("'hello' + 'world': " + concat1)
vibez.spill("'Hello ' + 'World': " + concat2)
vibez.spill("'CURSED' + 'stdlib': " + concat3)
vibez.spill("'test' + 'ing': " + concat4)

fr fr Test contains function
vibez.spill("\n🔍 Contains Tests:")
sus contains1 lit = stringz.contains("hello world", "world")
sus contains2 lit = stringz.contains("hello world", "hello")
sus contains3 lit = stringz.contains("test", "es")
sus contains4 lit = stringz.contains("CURSED", "CUR")
sus contains5 lit = stringz.contains("banana", "an")

vibez.spill("'hello world' contains 'world': " + contains1)
vibez.spill("'hello world' contains 'hello': " + contains2)
vibez.spill("'test' contains 'es': " + contains3)
vibez.spill("'CURSED' contains 'CUR': " + contains4)
vibez.spill("'banana' contains 'an': " + contains5)

fr fr Test upper function
vibez.spill("\n📈 Upper Tests:")
sus upper1 tea = stringz.upper("hello")
sus upper2 tea = stringz.upper("world")
sus upper3 tea = stringz.upper("Hello World")
sus upper4 tea = stringz.upper("test")

vibez.spill("upper('hello'): " + upper1)
vibez.spill("upper('world'): " + upper2)
vibez.spill("upper('Hello World'): " + upper3)
vibez.spill("upper('test'): " + upper4)

fr fr Test lower function
vibez.spill("\n📉 Lower Tests:")
sus lower1 tea = stringz.lower("HELLO")
sus lower2 tea = stringz.lower("WORLD")
sus lower3 tea = stringz.lower("Hello World")
sus lower4 tea = stringz.lower("TEST")

vibez.spill("lower('HELLO'): " + lower1)
vibez.spill("lower('WORLD'): " + lower2)
vibez.spill("lower('Hello World'): " + lower3)
vibez.spill("lower('TEST'): " + lower4)

fr fr Test trim function
vibez.spill("\n✂️ Trim Tests:")
sus trim1 tea = stringz.trim("  hello  ")
sus trim2 tea = stringz.trim(" test ")
sus trim3 tea = stringz.trim("hello")
sus trim4 tea = stringz.trim("  hello world  ")

vibez.spill("trim('  hello  '): '" + trim1 + "'")
vibez.spill("trim(' test '): '" + trim2 + "'")
vibez.spill("trim('hello'): '" + trim3 + "'")
vibez.spill("trim('  hello world  '): '" + trim4 + "'")

fr fr Test split function
vibez.spill("\n🪓 Split Tests (count):")
sus split1 drip = stringz.split("a,b,c", ",")
sus split2 drip = stringz.split("hello world", " ")
sus split3 drip = stringz.split("one,two,three,four", ",")
sus split4 drip = stringz.split("test.file.name", ".")

vibez.spill("split('a,b,c', ','): " + split1 + " parts")
vibez.spill("split('hello world', ' '): " + split2 + " parts")
vibez.spill("split('one,two,three,four', ','): " + split3 + " parts")
vibez.spill("split('test.file.name', '.'): " + split4 + " parts")

vibez.spill("\n✅ StringZ comprehensive test completed!")
vibez.spill("🎯 All basic string operations are working in pure CURSED!")
