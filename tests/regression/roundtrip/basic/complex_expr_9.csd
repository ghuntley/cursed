// Complex expression round-trip test 9
sus nested drip = (((9 + 3) * 2) - 1) / ((9 % 4) + 1)
sus conditional drip = ready (nested > 9) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(9, nested)

vibez.spill("Complex round-trip 9:", final)
