// Complex expression round-trip test 16
sus nested drip = (((16 + 3) * 2) - 1) / ((16 % 4) + 1)
sus conditional drip = ready (nested > 16) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(16, nested)

vibez.spill("Complex round-trip 16:", final)
