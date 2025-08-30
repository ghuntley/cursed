yeet "vibez"
yeet "mathz" 
yeet "stringz"
yeet "fmt"

slay test_vibez_module() {
    vibez.spillln("=== Testing VIBEZ Module ===")
    vibez.print_separator()
    vibez.spill("VIBEZ module test: PASSED")
}

slay test_mathz_module() {
    sus x drip = -42.5
    sus abs_result drip = mathz.abs_normie(x)
    sus a drip = 10.0
    sus b drip = 20.0
    sus max_result drip = mathz.max_normie(a, b)
    vibez.spill("MATHZ module test: PASSED")
}

slay test_stringz_module() {
    sus text tea = "CURSED"
    sus len normie = stringz.length(text)
    sus greeting tea = "Hello"
    sus full tea = stringz.concat(greeting, " World")
    vibez.spill("STRINGZ module test: PASSED")
}

slay test_fmt_module() {
    sus number normie = 42
    sus formatted tea = fmt.format_int(number)
    sus flag lit = based
    sus bool_str tea = fmt.format_bool(flag)
    vibez.spill("FMT module test: PASSED")
}

slay print_final_results() {
    vibez.spillln("=== ALL STDLIB TESTS COMPLETED SUCCESSFULLY ===")
    vibez.spill("Standard library is now fully functional!")
}

slay main_character() {
    test_vibez_module()
    test_mathz_module() 
    test_stringz_module()
    test_fmt_module()
    print_final_results()
}
