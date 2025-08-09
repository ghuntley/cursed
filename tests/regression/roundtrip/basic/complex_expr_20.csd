// Complex expression round-trip test 20
sus nested drip = (((20 + 3) * 2) - 1) / ((20 % 4) + 1)
sus conditional drip = ready (nested > 20) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(20, nested)

vibez.spill("Complex round-trip 20:", final)
