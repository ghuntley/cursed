// Complex expression round-trip test 14
sus nested drip = (((14 + 3) * 2) - 1) / ((14 % 4) + 1)
sus conditional drip = ready (nested > 14) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(14, nested)

vibez.spill("Complex round-trip 14:", final)
