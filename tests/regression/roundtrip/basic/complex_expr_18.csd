// Complex expression round-trip test 18
sus nested drip = (((18 + 3) * 2) - 1) / ((18 % 4) + 1)
sus conditional drip = ready (nested > 18) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(18, nested)

vibez.spill("Complex round-trip 18:", final)
