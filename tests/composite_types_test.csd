vibe test_composite;

fr fr Test file for composite types in CURSED

fr fr Struct test
be_like Person squad {
    name tea
    age normie
    hobbies []tea
}

fr fr Array test
slay test_arrays() normie {
    sus numbers = crew[1, 2, 3, 4, 5];
    sus first = numbers[0];
    sus last = numbers[4];
    
    lowkey first == 1 && last == 5 {
        damn 1;
    } highkey {
        damn 0;
    }
}

fr fr Slice test
slay test_slices() normie {
    sus names = crew["Alice", "Bob", "Charlie", "Dave", "Eve"];
    fr fr Accessing elements
    sus first_name = names[0];
    sus third_name = names[2];
    
    lowkey first_name == "Alice" && third_name == "Charlie" {
        damn 1;
    } highkey {
        damn 0;
    }
}

fr fr Map test
slay test_maps() normie {
    sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92};
    sus alice_score = scores["Alice"];
    sus bob_score = scores["Bob"];
    
    lowkey alice_score == 95 && bob_score == 87 {
        damn 1;
    } highkey {
        damn 0;
    }
}

fr fr Struct instantiation and access
slay test_structs() normie {
    sus person = be_like Person {
        name: "John",
        age: 30,
        hobbies: crew["reading", "coding", "gaming"]
    };
    
    lowkey person.name == "John" && person.age == 30 {
        damn 1;
    } highkey {
        damn 0;
    }
}

fr fr Main test function
slay main() normie {
    sus array_result = test_arrays();
    sus slice_result = test_slices();
    sus map_result = test_maps();
    sus struct_result = test_structs();
    
    sus total = array_result + slice_result + map_result + struct_result;
    
    fr fr All tests should pass (return 1)
    lowkey total == 4 {
        puts("All composite type tests passed!");
        damn 1;
    } highkey {
        puts("Some composite type tests failed!");
        damn 0;
    }
}