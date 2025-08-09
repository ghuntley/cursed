// Complex expression round-trip test 11
sus nested drip = (((11 + 3) * 2) - 1) / ((11 % 4) + 1)
sus conditional drip = ready (nested > 11) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(11, nested)

vibez.spill("Complex round-trip 11:", final)
