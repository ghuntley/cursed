sus x drip = 3
sus y drip = 7

ready (x > y) {
    vibez.spill("x is greater")
} otherwise {
    vibez.spill("y is greater or equal")
}

ready (x == 3 && y > 5) {
    vibez.spill("Complex condition true")
} otherwise {
    vibez.spill("Complex condition false")
}
