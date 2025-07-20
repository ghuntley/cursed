sus test_count normie = 0
sus pass_count normie = 0
sus fail_count normie = 0
sus current_test_name tea = ""

slay test_start(name tea) {
    current_test_name = name
    test_count = test_count + 1
    vibez.spill("Starting test: ", name)
}

vibez.spill("Testz module loaded")
