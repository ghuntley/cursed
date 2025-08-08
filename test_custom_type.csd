squad TestStruct {
    spill name tea
    spill value normie
}

sus test_var TestStruct = TestStruct{name: "test", value: 42}

vibez.spill("name:", test_var.name)
vibez.spill("value:", test_var.value)
