fr fr P0 Sprint 1: Simple Type Checker Demo
fr fr This program demonstrates the type checking capabilities

fr fr Basic variable declarations with type annotations
sus number drip = 42
sus text tea = "Hello CURSED"
sus flag lit = based
sus decimal vibes = 3.14

fr fr Test type checking for variables
spill("Number:", number)
spill("Text:", text)
spill("Flag:", flag)
spill("Decimal:", decimal)

fr fr Struct declaration and usage
squad Person {
    name tea
    age drip
    active lit
}

fr fr Create struct instance
sus person Person = Person{
    name: "Alice",
    age: 30,
    active: based
}

fr fr Field access
spill("Person name:", person.name)
spill("Person age:", person.age)

fr fr Array declaration and access
sus numbers []drip = [1, 2, 3, 4, 5]
spill("First number:", numbers[0])
spill("Array length:", len(numbers))

fr fr Function declaration
slay add(a drip, b drip) drip {
    damn a + b
}

fr fr Function call
sus sum drip = add(10, 20)
spill("Sum:", sum)

fr fr Control structures
ready (number > 0) {
    spill("✅ Number is positive")
} otherwise {
    spill("❌ Number is negative or zero")
}

fr fr Loop with type checking
sus i drip = 0
bestie (i < 3) {
    spill("Loop iteration:", i)
    i = i + 1
}

fr fr Arithmetic operations
sus result1 drip = number + 10
sus result2 vibes = decimal * 2.0
sus result3 lit = (number > 10)

spill("Arithmetic results:", result1, result2, result3)

fr fr String operations
sus greeting tea = "Hello" + " " + "World"
spill("Greeting:", greeting)

fr fr Type coercions (should work)
sus mixed_math vibes = number + decimal

fr fr Error cases (should be caught by type checker)
fr fr sus error1 = text + number   // Type mismatch
fr fr sus error2 drip = flag       // Type mismatch  
fr fr sus error3 = unknown_var     // Undeclared variable
fr fr person.unknown_field         // Unknown field

spill("✅ Type checker demo completed")
