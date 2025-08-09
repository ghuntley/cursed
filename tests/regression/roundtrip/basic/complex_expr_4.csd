// Complex expression round-trip test 4
sus nested drip = (((4 + 3) * 2) - 1) / ((4 % 4) + 1)
sus conditional drip = ready (nested > 4) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(4, nested)

vibez.spill("Complex round-trip 4:", final)
