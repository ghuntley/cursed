// Complex expression round-trip test 24
sus nested drip = (((24 + 3) * 2) - 1) / ((24 % 4) + 1)
sus conditional drip = ready (nested > 24) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(24, nested)

vibez.spill("Complex round-trip 24:", final)
