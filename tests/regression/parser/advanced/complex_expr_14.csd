// Complex expression test 14
sus result drip = (((14 + 5) * 2) - 1) / ((14 % 3) + 1)
sus bool_result lit = (result > 14) && (result < 140)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 14:", result, bool_result, nested)
