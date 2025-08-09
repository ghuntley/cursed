// Complex expression round-trip test 1
sus nested drip = (((1 + 3) * 2) - 1) / ((1 % 4) + 1)
sus conditional drip = ready (nested > 1) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(1, nested)

vibez.spill("Complex round-trip 1:", final)
