// Complex expression test 15
sus result drip = (((15 + 5) * 2) - 1) / ((15 % 3) + 1)
sus bool_result lit = (result > 15) && (result < 150)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 15:", result, bool_result, nested)
