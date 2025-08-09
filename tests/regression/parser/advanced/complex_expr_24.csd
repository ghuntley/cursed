// Complex expression test 24
sus result drip = (((24 + 5) * 2) - 1) / ((24 % 3) + 1)
sus bool_result lit = (result > 24) && (result < 240)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 24:", result, bool_result, nested)
