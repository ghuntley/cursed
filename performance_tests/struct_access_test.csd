squad TestStruct {
    spill field1 drip
    spill field2 drip
    spill field3 drip
    spill field4 drip
    spill field5 drip
}

slay test_struct_access() {
    sus test_obj TestStruct = TestStruct{
        field1: 10,
        field2: 20,
        field3: 30,
        field4: 40,
        field5: 50
    }
    
    sus total drip = 0
    sus i drip = 0
    bestie (i < 20000) {
        total = total + test_obj.field1 + test_obj.field2 + test_obj.field3 + test_obj.field4 + test_obj.field5
        i = i + 1
    }
    vibez.spill("Struct access test result:", total)
}

test_struct_access()
