// Complex expression test 21
sus result drip = (((21 + 5) * 2) - 1) / ((21 % 3) + 1)
sus bool_result lit = (result > 21) && (result < 210)
sus nested drip = ready (bool_result) ? result * 2 : result / 2
vibez.spill("Complex 21:", result, bool_result, nested)
