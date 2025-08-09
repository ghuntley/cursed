// Complex expression test 23
sus result drip = (((23 + 5) * 2) - 1) / ((23 % 3) + 1)
sus bool_result lit = (result > 23) && (result < 230)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 23:", result, bool_result, nested)
