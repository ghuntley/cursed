fr fr Testing all major CURSED language features

squad TestStruct {
    spill number drip
    spill text tea
}

collab TestInterface {
    slay test_method() drip
}

flex TestStruct => TestInterface {
    slay test_method() drip {
        damn number * 2
    }
}

slay test_function(param drip) drip {
    damn param + 10
}

slay feature_completeness_test() {
    fr fr Variables
    sus int_var drip = 42
    sus string_var tea = "test"
    sus bool_var lit = based
    
    fr fr Structs
    sus test_obj TestStruct = TestStruct{number: 100, text: "hello"}
    
    fr fr Functions
    sus func_result drip = test_function(5)
    
    fr fr Interface calls
    sus interface_result drip = test_obj.test_method()
    
    fr fr Control flow
    sus loop_counter drip = 0
    bestie (loop_counter < 3) {
        loop_counter = loop_counter + 1
    }
    
    fr fr Conditional
    if (int_var > 0) {
        vibez.spill("Positive number")
    }
    
    vibez.spill("Feature test results:")
    vibez.spill("Integer:", int_var)
    vibez.spill("String:", string_var)
    vibez.spill("Function result:", func_result)
    vibez.spill("Interface result:", interface_result)
    vibez.spill("Loop iterations:", loop_counter)
    vibez.spill("All features tested successfully!")
}

feature_completeness_test()
