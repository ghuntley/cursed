// Complex expression test 6
sus result drip = (((6 + 5) * 2) - 1) / ((6 % 3) + 1)
sus bool_result lit = (result > 6) && (result < 60)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 6:", result, bool_result, nested)
