// Complex expression test 7
sus result drip = (((7 + 5) * 2) - 1) / ((7 % 3) + 1)
sus bool_result lit = (result > 7) && (result < 70)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 7:", result, bool_result, nested)
