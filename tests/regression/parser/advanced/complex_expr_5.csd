// Complex expression test 5
sus result drip = (((5 + 5) * 2) - 1) / ((5 % 3) + 1)
sus bool_result lit = (result > 5) && (result < 50)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 5:", result, bool_result, nested)
