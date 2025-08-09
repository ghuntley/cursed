// Complex expression test 19
sus result drip = (((19 + 5) * 2) - 1) / ((19 % 3) + 1)
sus bool_result lit = (result > 19) && (result < 190)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 19:", result, bool_result, nested)
