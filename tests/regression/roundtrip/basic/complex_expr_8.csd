// Complex expression round-trip test 8
sus nested drip = (((8 + 3) * 2) - 1) / ((8 % 4) + 1)
sus conditional drip = ready (nested > 8) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(8, nested)

vibez.spill("Complex round-trip 8:", final)
