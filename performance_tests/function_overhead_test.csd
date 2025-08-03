slay simple_add(a drip, b drip) drip {
    damn a + b
}

slay test_function_calls() {
    sus total drip = 0
    sus i drip = 0
    bestie (i < 50000) {
        total = total + simple_add(i, i + 1)
        i = i + 1
    }
    vibez.spill("Function call test result:", total)
}

test_function_calls()
