// Complex expression round-trip test 19
sus nested drip = (((19 + 3) * 2) - 1) / ((19 % 4) + 1)
sus conditional drip = ready (nested > 19) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(19, nested)

vibez.spill("Complex round-trip 19:", final)
