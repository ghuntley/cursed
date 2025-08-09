// Complex expression round-trip test 7
sus nested drip = (((7 + 3) * 2) - 1) / ((7 % 4) + 1)
sus conditional drip = ready (nested > 7) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(7, nested)

vibez.spill("Complex round-trip 7:", final)
