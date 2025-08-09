// Complex expression test 4
sus result drip = (((4 + 5) * 2) - 1) / ((4 % 3) + 1)
sus bool_result lit = (result > 4) && (result < 40)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 4:", result, bool_result, nested)
