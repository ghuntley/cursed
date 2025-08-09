// Complex expression round-trip test 13
sus nested drip = (((13 + 3) * 2) - 1) / ((13 % 4) + 1)
sus conditional drip = ready (nested > 13) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(13, nested)

vibez.spill("Complex round-trip 13:", final)
