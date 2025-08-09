// Complex expression test 18
sus result drip = (((18 + 5) * 2) - 1) / ((18 % 3) + 1)
sus bool_result lit = (result > 18) && (result < 180)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 18:", result, bool_result, nested)
