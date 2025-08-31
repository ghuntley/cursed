sus "stdlib/mathz.csd" as mathz

slay test_math() {
    x := 10
    y := 5
    result := mathz.add_two(x, y)
    vibez.spill(result)
}

test_math()
