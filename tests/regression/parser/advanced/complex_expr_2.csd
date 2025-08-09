// Complex expression test 2
sus result drip = (((2 + 5) * 2) - 1) / ((2 % 3) + 1)
sus bool_result lit = (result > 2) && (result < 20)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 2:", result, bool_result, nested)
