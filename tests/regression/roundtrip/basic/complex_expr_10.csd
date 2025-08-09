// Complex expression round-trip test 10
sus nested drip = (((10 + 3) * 2) - 1) / ((10 % 4) + 1)
sus conditional drip = ready (nested > 10) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(10, nested)

vibez.spill("Complex round-trip 10:", final)
