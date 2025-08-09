// Control flow structures
ready (x > 5) {
    vibez.spill("greater")
} otherwise {
    vibez.spill("smaller")
}

bestie (i < 10) {
    vibez.spill(i)
    i = i + 1
}

ready (value) {
    1 => vibez.spill("one")
    2 => vibez.spill("two")
    _ => vibez.spill("other")
}
