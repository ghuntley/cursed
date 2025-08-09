// Complex expression round-trip test 3
sus nested drip = (((3 + 3) * 2) - 1) / ((3 % 4) + 1)
sus conditional drip = ready (nested > 3) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(3, nested)

vibez.spill("Complex round-trip 3:", final)
