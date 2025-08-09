// Complex expression test 12
sus result drip = (((12 + 5) * 2) - 1) / ((12 % 3) + 1)
sus bool_result lit = (result > 12) && (result < 120)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 12:", result, bool_result, nested)
