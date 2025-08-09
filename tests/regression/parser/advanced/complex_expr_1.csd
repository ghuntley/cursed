// Complex expression test 1
sus result drip = (((1 + 5) * 2) - 1) / ((1 % 3) + 1)
sus bool_result lit = (result > 1) && (result < 10)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 1:", result, bool_result, nested)
