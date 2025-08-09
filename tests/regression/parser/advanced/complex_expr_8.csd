// Complex expression test 8
sus result drip = (((8 + 5) * 2) - 1) / ((8 % 3) + 1)
sus bool_result lit = (result > 8) && (result < 80)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 8:", result, bool_result, nested)
