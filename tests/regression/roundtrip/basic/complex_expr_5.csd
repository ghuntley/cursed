// Complex expression round-trip test 5
sus nested drip = (((5 + 3) * 2) - 1) / ((5 % 4) + 1)
sus conditional drip = ready (nested > 5) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(5, nested)

vibez.spill("Complex round-trip 5:", final)
