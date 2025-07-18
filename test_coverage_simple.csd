// Simple CURSED test program for coverage testing

slay test_function(value normie) normie {
    lowkey (value > 0) {
        vibez.spill("Positive value")
        damn value * 2
    } highkey (value < 0) {
        vibez.spill("Negative value")
        damn value * -1
    } else {
        vibez.spill("Zero value")
        damn 0
    }
}

slay main() {
    sus x normie = 42
    sus result normie = test_function(x)
    vibez.spill("Result: " + result)
}
