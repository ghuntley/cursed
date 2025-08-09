// Complex expression test 20
sus result drip = (((20 + 5) * 2) - 1) / ((20 % 3) + 1)
sus bool_result lit = (result > 20) && (result < 200)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 20:", result, bool_result, nested)
