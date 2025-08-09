// Complex expression test 16
sus result drip = (((16 + 5) * 2) - 1) / ((16 % 3) + 1)
sus bool_result lit = (result > 16) && (result < 160)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 16:", result, bool_result, nested)
