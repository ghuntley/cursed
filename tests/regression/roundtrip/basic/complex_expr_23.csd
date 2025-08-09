// Complex expression round-trip test 23
sus nested drip = (((23 + 3) * 2) - 1) / ((23 % 4) + 1)
sus conditional drip = ready (nested > 23) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(23, nested)

vibez.spill("Complex round-trip 23:", final)
