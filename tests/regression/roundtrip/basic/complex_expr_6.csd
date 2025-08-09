// Complex expression round-trip test 6
sus nested drip = (((6 + 3) * 2) - 1) / ((6 % 4) + 1)
sus conditional drip = ready (nested > 6) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(6, nested)

vibez.spill("Complex round-trip 6:", final)
