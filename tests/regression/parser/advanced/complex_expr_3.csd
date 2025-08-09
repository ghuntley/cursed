// Complex expression test 3
sus result drip = (((3 + 5) * 2) - 1) / ((3 % 3) + 1)
sus bool_result lit = (result > 3) && (result < 30)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 3:", result, bool_result, nested)
