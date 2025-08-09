sus test_var drip = 42
sus name tea = "CURSED"

slay test_function(param drip) drip {
    vibez.spill("Testing:", param)
    damn param * 2
}

ready (test_var > 0) {
    sus result drip = test_function(test_var)
    vibez.spill("Result:", result)
}

bestie (test_var > 0) {
    test_var = test_var - 1
    vibez.spill("Countdown:", test_var)
}
