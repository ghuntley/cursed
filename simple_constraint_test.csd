# Simple Generic Constraint Validation Test

yeet "vibez"
yeet "testz"

# Test numeric constraint
slay add<T>(a T, b T) T ready T: Numeric {
    damn a + b
}

sus result drip = add(5, 3)
vibez.spill("Numeric constraint test: 5 + 3 =", result)

# Test comparable constraint  
slay max<T>(a T, b T) T ready T: Comparable {
    ready (a > b) {
        damn a
    } otherwise {
        damn b
    }
}

sus max_val drip = max(10, 5)
vibez.spill("Comparable constraint test: max(10, 5) =", max_val)

# Test interface constraint
collab Drawable {
    slay draw(self) vibes
}

squad Circle {
    radius meal
}

slay draw(self Circle) vibes {
    vibez.spill("Drawing circle")
}

slay render<T>(obj T) vibes ready T: Drawable {
    obj.draw()
}

sus circle Circle = Circle{ .radius = 5.0 }
render(circle)

vibez.spill("All constraint validation tests passed!")

test_start("Constraint Validation")
assert_eq_int(result, 8)
assert_eq_int(max_val, 10) 
print_test_summary()
