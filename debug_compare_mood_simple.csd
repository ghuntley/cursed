yeet "compare_mood"

# Simple test without testz
sus result1 normie = CompareInt(5, 10)
vibez.spill("CompareInt(5, 10) = " + result1)

sus result2 normie = CompareInt(10, 5)
vibez.spill("CompareInt(10, 5) = " + result2)

sus result3 normie = CompareInt(7, 7)
vibez.spill("CompareInt(7, 7) = " + result3)

sus min_result normie = MinInt(5, 10)
vibez.spill("MinInt(5, 10) = " + min_result)

sus max_result normie = MaxInt(5, 10)
vibez.spill("MaxInt(5, 10) = " + max_result)

vibez.spill("compare_mood module test completed successfully!")
