# Debug: Function calls
slay simple_func() drip {
    damn 100
}

slay add_nums(a drip, b drip) drip {
    damn a + b
}

vibez.spill("Testing function calls:")
sus result1 drip = simple_func()
vibez.spill(result1)

sus result2 drip = add_nums(5, 7)
vibez.spill(result2)
