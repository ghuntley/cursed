// Complex expression round-trip test 12
sus nested drip = (((12 + 3) * 2) - 1) / ((12 % 4) + 1)
sus conditional drip = ready (nested > 12) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(12, nested)

vibez.spill("Complex round-trip 12:", final)
