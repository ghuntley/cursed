fr fr Test 1: Loop that should never execute (false condition)
sus x drip = 5
bestie (x < 3) {
    vibez.spill("This should not print")
}
vibez.spill("x is still:", x)

fr fr Test 2: Loop with different condition
sus j drip = 0
bestie (j < 2) {
    vibez.spill("j is:", j)
    j = j + 1
}
vibez.spill("j final:", j)
