// Complex expression round-trip test 15
sus nested drip = (((15 + 3) * 2) - 1) / ((15 % 4) + 1)
sus conditional drip = ready (nested > 15) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(15, nested)

vibez.spill("Complex round-trip 15:", final)
