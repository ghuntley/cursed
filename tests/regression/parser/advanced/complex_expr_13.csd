// Complex expression test 13
sus result drip = (((13 + 5) * 2) - 1) / ((13 % 3) + 1)
sus bool_result lit = (result > 13) && (result < 130)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 13:", result, bool_result, nested)
