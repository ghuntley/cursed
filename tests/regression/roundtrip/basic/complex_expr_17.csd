// Complex expression round-trip test 17
sus nested drip = (((17 + 3) * 2) - 1) / ((17 % 4) + 1)
sus conditional drip = ready (nested > 17) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(17, nested)

vibez.spill("Complex round-trip 17:", final)
