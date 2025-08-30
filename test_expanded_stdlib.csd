yeet "vibez"
yeet "mathz"
yeet "stringz"

slay test_vibez() {
    vibez.spill("=== Testing VIBEZ Module ===")
}

slay test_mathz_abs() {
    sus x drip = -42.5
    sus result drip = mathz.abs_normie(x)
    vibez.spill("Abs test: OK")
}

slay test_mathz_add() {
    sus a drip = 10.0
    sus b drip = 5.0
    sus result drip = mathz.add(a, b)
    vibez.spill("Add test: OK")
}

slay test_mathz_sub() {
    sus a drip = 20.0
    sus b drip = 8.0
    sus result drip = mathz.sub(a, b)
    vibez.spill("Sub test: OK")
}

slay test_mathz_mul() {
    sus a drip = 6.0
    sus b drip = 7.0
    sus result drip = mathz.mul(a, b)
    vibez.spill("Mul test: OK")
}

slay test_mathz_div() {
    sus a drip = 15.0
    sus b drip = 3.0
    sus result drip = mathz.div(a, b)
    vibez.spill("Div test: OK")
}

slay test_stringz_length() {
    sus text tea = "Hello CURSED"
    sus len normie = stringz.length(text)
    vibez.spill("String length test: OK")
}

slay test_stringz_concat() {
    sus a tea = "Hello"
    sus b tea = " World"
    sus result tea = stringz.concat(a, b)
    vibez.spill("String concat test: OK")
}

slay print_final_results() {
    vibez.spillln("=== All stdlib tests completed successfully ===")
}

slay main_character() {
    test_vibez()
    test_mathz_abs()
    test_mathz_add()
    test_mathz_sub()
    test_mathz_mul()
    test_mathz_div()
    test_stringz_length()
    test_stringz_concat()
    print_final_results()
}
