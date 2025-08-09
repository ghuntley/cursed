// Complex expression test 11
sus result drip = (((11 + 5) * 2) - 1) / ((11 % 3) + 1)
sus bool_result lit = (result > 11) && (result < 110)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 11:", result, bool_result, nested)
