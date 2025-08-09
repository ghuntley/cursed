// Complex expression test 10
sus result drip = (((10 + 5) * 2) - 1) / ((10 % 3) + 1)
sus bool_result lit = (result > 10) && (result < 100)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 10:", result, bool_result, nested)
