// Test that the parser can handle all AST node types

// Array literal
sus numbers = [1, 2, 3]

// Struct literal with field initializers
squad Person {
    spill name tea
    spill age drip
}

sus person = Person{
    name: "Alice",
    age: 30
}

// Method call
person.getName()

// Error expressions
yikes "Error message"
shook risky_function()
