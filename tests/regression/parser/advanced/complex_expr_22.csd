// Complex expression test 22
sus result drip = (((22 + 5) * 2) - 1) / ((22 % 3) + 1)
sus bool_result lit = (result > 22) && (result < 220)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 22:", result, bool_result, nested)
