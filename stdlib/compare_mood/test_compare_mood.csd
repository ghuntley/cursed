yeet "testz"
yeet "compare_mood"

test_start("compare_mood comprehensive tests")

fr fr Test comparison constants
assert_eq_int(LessThan, -1)
assert_eq_int(Equal, 0)
assert_eq_int(GreaterThan, 1)

fr fr Test integer comparisons
assert_eq_int(CompareInt(5, 10), LessThan)
assert_eq_int(CompareInt(10, 5), GreaterThan)
assert_eq_int(CompareInt(7, 7), Equal)

fr fr Test string comparisons
assert_eq_int(CompareString("apple", "banana"), LessThan)
assert_eq_int(CompareString("banana", "apple"), GreaterThan)
assert_eq_int(CompareString("same", "same"), Equal)

fr fr Test float comparisons
assert_eq_int(CompareFloat(3.14, 2.71), GreaterThan)
assert_eq_int(CompareFloat(2.71, 3.14), LessThan)
assert_eq_int(CompareFloat(1.0, 1.0), Equal)

fr fr Test boolean comparisons
assert_eq_int(CompareBool(cap, based), LessThan)
assert_eq_int(CompareBool(based, cap), GreaterThan)
assert_eq_int(CompareBool(based, based), Equal)

fr fr Test equality functions
assert_true(EqualInt(42, 42))
assert_false(EqualInt(42, 43))
assert_true(EqualString("hello", "hello"))
assert_false(EqualString("hello", "world"))
assert_true(EqualFloat(3.14, 3.14))
assert_false(EqualFloat(3.14, 2.71))
assert_true(EqualBool(based, based))
assert_false(EqualBool(based, cap))

fr fr Test less than functions
assert_true(LessInt(5, 10))
assert_false(LessInt(10, 5))
assert_false(LessInt(5, 5))
assert_true(LessString("a", "b"))
assert_false(LessString("b", "a"))
assert_true(LessFloat(1.5, 2.5))
assert_false(LessFloat(2.5, 1.5))

fr fr Test greater than functions
assert_true(GreaterInt(10, 5))
assert_false(GreaterInt(5, 10))
assert_false(GreaterInt(5, 5))
assert_true(GreaterString("z", "a"))
assert_false(GreaterString("a", "z"))
assert_true(GreaterFloat(2.5, 1.5))
assert_false(GreaterFloat(1.5, 2.5))

fr fr Test less than or equal functions
assert_true(LessEqualInt(5, 10))
assert_true(LessEqualInt(5, 5))
assert_false(LessEqualInt(10, 5))
assert_true(LessEqualString("a", "b"))
assert_true(LessEqualString("a", "a"))
assert_true(LessEqualFloat(1.5, 2.5))
assert_true(LessEqualFloat(1.5, 1.5))

fr fr Test greater than or equal functions
assert_true(GreaterEqualInt(10, 5))
assert_true(GreaterEqualInt(5, 5))
assert_false(GreaterEqualInt(5, 10))
assert_true(GreaterEqualString("z", "a"))
assert_true(GreaterEqualString("a", "a"))
assert_true(GreaterEqualFloat(2.5, 1.5))
assert_true(GreaterEqualFloat(1.5, 1.5))

fr fr Test ThreeWay function
assert_eq_int(ThreeWay(based, cap), LessThan)
assert_eq_int(ThreeWay(cap, based), GreaterThan)
assert_eq_int(ThreeWay(cap, cap), Equal)

fr fr Test Min functions
assert_eq_int(MinInt(5, 10), 5)
assert_eq_int(MinInt(10, 5), 5)
assert_eq_int(MinInt(7, 7), 7)
sus min_float meal = MinFloat(3.14, 2.71)
assert_true(min_float == 2.71)
assert_eq_string(MinString("apple", "banana"), "apple")
assert_eq_string(MinString("zebra", "apple"), "apple")

fr fr Test Max functions
assert_eq_int(MaxInt(5, 10), 10)
assert_eq_int(MaxInt(10, 5), 10)
assert_eq_int(MaxInt(7, 7), 7)
sus max_float meal = MaxFloat(3.14, 2.71)
assert_true(max_float == 3.14)
assert_eq_string(MaxString("apple", "banana"), "banana")
assert_eq_string(MaxString("zebra", "apple"), "zebra")

fr fr Test Clamp functions
assert_eq_int(ClampInt(5, 1, 10), 5)
assert_eq_int(ClampInt(-5, 1, 10), 1)
assert_eq_int(ClampInt(15, 1, 10), 10)
sus clamped_float meal = ClampFloat(5.5, 1.0, 10.0)
assert_true(clamped_float == 5.5)
sus clamped_float2 meal = ClampFloat(-1.5, 1.0, 10.0)
assert_true(clamped_float2 == 1.0)

fr fr Test Sign functions
assert_eq_int(SignInt(42), 1)
assert_eq_int(SignInt(-42), -1)
assert_eq_int(SignInt(0), 0)
assert_eq_int(SignFloat(3.14), 1)
assert_eq_int(SignFloat(-3.14), -1)
assert_eq_int(SignFloat(0.0), 0)

fr fr Test Absolute value functions
assert_eq_int(AbsInt(42), 42)
assert_eq_int(AbsInt(-42), 42)
assert_eq_int(AbsInt(0), 0)
sus abs_float meal = AbsFloat(3.14)
assert_true(abs_float == 3.14)
sus abs_float2 meal = AbsFloat(-3.14)
assert_true(abs_float2 == 3.14)

fr fr Test Distance functions
assert_eq_int(DistanceInt(10, 5), 5)
assert_eq_int(DistanceInt(5, 10), 5)
assert_eq_int(DistanceInt(7, 7), 0)
sus distance_float meal = DistanceFloat(10.5, 5.2)
assert_true(distance_float == 5.3)

fr fr Test Between functions
assert_true(BetweenInt(5, 1, 10))
assert_true(BetweenInt(1, 1, 10))
assert_true(BetweenInt(10, 1, 10))
assert_false(BetweenInt(0, 1, 10))
assert_false(BetweenInt(11, 1, 10))
assert_true(BetweenFloat(5.5, 1.0, 10.0))
assert_true(BetweenFloat(1.0, 1.0, 10.0))
assert_false(BetweenFloat(0.5, 1.0, 10.0))

fr fr Test BetweenExclusive functions
assert_true(BetweenExclusiveInt(5, 1, 10))
assert_false(BetweenExclusiveInt(1, 1, 10))
assert_false(BetweenExclusiveInt(10, 1, 10))
assert_false(BetweenExclusiveInt(0, 1, 10))
assert_true(BetweenExclusiveFloat(5.5, 1.0, 10.0))
assert_false(BetweenExclusiveFloat(1.0, 1.0, 10.0))
assert_false(BetweenExclusiveFloat(10.0, 1.0, 10.0))

fr fr Test edge cases
assert_eq_int(CompareInt(0, 0), Equal)
assert_eq_int(CompareInt(-1, -1), Equal)
assert_eq_int(CompareInt(-5, -3), LessThan)
assert_eq_int(CompareInt(-3, -5), GreaterThan)

fr fr Test string edge cases
assert_eq_int(CompareString("", ""), Equal)
assert_eq_int(CompareString("", "a"), LessThan)
assert_eq_int(CompareString("a", ""), GreaterThan)

fr fr Test float edge cases
assert_eq_int(CompareFloat(0.0, 0.0), Equal)
assert_eq_int(CompareFloat(-1.0, -1.0), Equal)
assert_eq_int(CompareFloat(-2.5, -1.5), LessThan)

print_test_summary()
