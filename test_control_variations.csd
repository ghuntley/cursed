sus a drip = 8
ready (a == 8) {
    vibez.spill("a equals 8")
}

sus b drip = 2
ready (b > 10) {
    vibez.spill("b is big")
} otherwise {
    vibez.spill("b is small")
}

sus name tea = "test"
ready (name == "test") {
    vibez.spill("name is test")
} otherwise {
    vibez.spill("name is not test")
}
