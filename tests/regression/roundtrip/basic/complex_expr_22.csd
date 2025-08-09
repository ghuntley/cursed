// Complex expression round-trip test 22
sus nested drip = (((22 + 3) * 2) - 1) / ((22 % 4) + 1)
sus conditional drip = ready (nested > 22) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(22, nested)

vibez.spill("Complex round-trip 22:", final)
