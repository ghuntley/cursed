yeet "compare_mood"

vibez.spill("Testing compare_mood module")

# Test constants
vibez.spill("LessThan constant:")
vibez.spill(LessThan)

vibez.spill("Equal constant:")
vibez.spill(Equal)

vibez.spill("GreaterThan constant:")
vibez.spill(GreaterThan)

# Test basic comparison
vibez.spill("Testing CompareInt(5, 10):")
sus result1 normie = CompareInt(5, 10)
vibez.spill(result1)

vibez.spill("Testing CompareInt(10, 5):")
sus result2 normie = CompareInt(10, 5)
vibez.spill(result2)

vibez.spill("Testing CompareInt(7, 7):")
sus result3 normie = CompareInt(7, 7)
vibez.spill(result3)

# Test min/max
vibez.spill("Testing MinInt(5, 10):")
sus min_result normie = MinInt(5, 10)
vibez.spill(min_result)

vibez.spill("Testing MaxInt(5, 10):")
sus max_result normie = MaxInt(5, 10)
vibez.spill(max_result)

# Test equality
vibez.spill("Testing EqualInt(42, 42):")
sus equal_result lit = EqualInt(42, 42)
vibez.spill(equal_result)

vibez.spill("Testing EqualInt(42, 43):")
sus not_equal_result lit = EqualInt(42, 43)
vibez.spill(not_equal_result)

vibez.spill("compare_mood module test completed successfully!")
