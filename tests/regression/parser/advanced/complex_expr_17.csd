// Complex expression test 17
sus result drip = (((17 + 5) * 2) - 1) / ((17 % 3) + 1)
sus bool_result lit = (result > 17) && (result < 170)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 17:", result, bool_result, nested)
