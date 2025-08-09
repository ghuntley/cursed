// Complex expression round-trip test 21
sus nested drip = (((21 + 3) * 2) - 1) / ((21 % 4) + 1)
sus conditional drip = ready (nested > 21) ? nested + 5 : nested - 5
sus final drip = abs_normie(conditional) + max_drip(21, nested)

vibez.spill("Complex round-trip 21:", final)
