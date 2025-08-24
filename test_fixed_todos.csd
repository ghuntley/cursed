yeet "vibez"

sus numbers []drip = [1, 2, 3, 4, 5]
sus other []drip = [1, 2, 3]

# Test array comparison fix
ready (numbers == numbers) {
    vibez.spill("✅ Array comparison works!")
}

# Test array iteration
sus sum drip = 0
bestie (sus i drip = 0; i < len(numbers); i = i + 1) {
    sum = sum + numbers[i]
}

vibez.spill("Sum of numbers:", sum)

# Test interface implementation
collab TestInterface {
    slay test_method(self &Self, value drip) drip
}

squad TestStruct {
    sus field drip
}

impl TestInterface for TestStruct {
    slay test_method(self &Self, value drip) drip {
        damn self.field + value
    }
}

sus test_obj TestStruct = TestStruct{field: 10}
sus result drip = test_obj.test_method(5)
vibez.spill("Interface method result:", result)
