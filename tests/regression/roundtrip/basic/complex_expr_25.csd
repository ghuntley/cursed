// Complex expression round-trip test 25
sus nested drip = (((25 + 3) * 2) - 1) / ((25 % 4) + 1)
sus conditional drip = ready (nested > 25) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(25, nested)

vibez.spill("Complex round-trip 25:", final)
