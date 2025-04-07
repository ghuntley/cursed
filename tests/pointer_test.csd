fr fr Test file for pointer functionality

slay main() {
    fr fr Test basic pointer creation and dereferencing
    sus x normie = 42
    sus ptr @normie = @x
    sus y normie = @ptr
    
    fr fr Test that y equals x via the pointer
    lowkey y == 42 {
        print("Basic pointer test passed")
    } highkey {
        print("Basic pointer test failed")
    }
    
    fr fr Test pointer modification
    @ptr = 100
    lowkey x == 100 {
        print("Pointer modification test passed")
    } highkey {
        print("Pointer modification test failed")
    }
    
    fr fr Test pointer to struct
    sus person Person = Person{name: "John", age: 30}
    sus person_ptr @Person = @person
    @person_ptr.age = 31
    
    lowkey person.age == 31 {
        print("Struct pointer test passed")
    } highkey {
        print("Struct pointer test failed")
    }
    
    fr fr Test nil pointer
    sus nil_ptr @normie = cap
    lowkey nil_ptr == cap {
        print("Nil pointer test passed")
    } highkey {
        print("Nil pointer test failed")
    }
}

be_like Person squad {
    name tea
    age normie
}

fr fr Simple print function for testing
slay print(message tea) {
    fr fr In real implementation this would print to console
}