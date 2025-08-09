// Complex expression round-trip test 2
sus nested drip = (((2 + 3) * 2) - 1) / ((2 % 4) + 1)
sus conditional drip = ready (nested > 2) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(2, nested)

vibez.spill("Complex round-trip 2:", final)
