sus x drip = 10
ready (x > 5) { vibez.spill("x is greater than 5") } otherwise { vibez.spill("x is not greater than 5") }

fr fr Test nested variables in conditions
sus a drip = 7
sus b drip = 3
ready (a + b > 8) {
    vibez.spill("Sum is greater than 8")
    sus result drip = a + b
    vibez.spill("Result:", result)
} otherwise {
    vibez.spill("Sum is not greater than 8")
}

fr fr Test boolean conditions
sus flag lit = based
ready (flag) {
    vibez.spill("Flag is true")
} otherwise {
    vibez.spill("Flag is false")
}

fr fr Test string conditions
sus name tea = "CURSED"
ready (name == "CURSED") {
    vibez.spill("Language name is correct")
}
