fr fr Simple optimization test

vibez.spill("Testing optimization levels...")

fr fr Constant folding test
sus a drip = 5
sus b drip = 10
sus result drip = 50

vibez.spill("Result: ")
vibez.spill(result)

fr fr Dead code that should be eliminated at O1+
sus unused drip = 999

vibez.spill("Done")
