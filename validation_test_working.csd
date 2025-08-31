// Working validation test using correct CURSED syntax

slay test_arithmetic() {
    sus a normie = 15
    sus b normie = 25
    sus result normie = a + b
    yap("Testing arithmetic:")
    yap(result)
    
    sus mult normie = 6 * 7
    yap("6 * 7 =")
    yap(mult)
}

slay test_variables() {
    sus x normie = 42
    sus message tea = "Hello CURSED"
    yap("Testing variables:")
    yap(x)
    yap(message)
}

slay main_character() {
    yap("CURSED Validation Suite")
    yap("=====================")
    test_arithmetic()
    test_variables()
    yap("All tests completed!")
}
