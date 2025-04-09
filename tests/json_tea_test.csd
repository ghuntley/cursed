vibe main

yeet "vibez"    fr fr For printing results
yeet "json_tea" fr fr JSON encoding/decoding

fr fr Define some test structures
be_like Person squad {
    Name tea
    Age normie
    Email tea
    Active lit
}

be_like Address squad {
    Street tea
    City tea
    ZipCode tea
    Country tea
}

be_like ComplexObject squad {
    ID thicc
    Owner Person
    Addresses []Address
    Tags []tea
    Metadata tea[tea]
    NullField @normie fr fr Pointer to test null
}

slay main() {
    vibez.spill("Testing json_tea package")
    
    fr fr Test basic types
    test_basic_types()
    
    fr fr Test struct marshaling
    test_struct_marshaling()
    
    fr fr Test struct unmarshaling
    test_struct_unmarshaling()
    
    fr fr Test complex objects
    test_complex_objects()
    
    fr fr Test custom marshaling
    test_custom_marshaling()
    
    vibez.spill("All json_tea tests passed!")
}

fr fr Test marshaling and unmarshaling of basic types
slay test_basic_types() {
    vibez.spill("Testing basic type marshaling...")
    
    fr fr Test string
    tea str := "hello world"
    tea jsonStr, err := json_tea.Marshal(str)
    lowkey err != cap {
        vibez.spill("Error marshaling string:", err)
        yolo
    }
    vibez.spill("String JSON:", jsonStr)
    
    tea strBack := ""
    err = json_tea.Unmarshal(jsonStr, &strBack)
    lowkey err != cap {
        vibez.spill("Error unmarshaling string:", err)
        yolo
    }
    vibez.spill("Unmarshaled string:", strBack)
    lowkey strBack != str {
        vibez.spill("String mismatch! Got:", strBack, "Expected:", str)
        yolo
    }
    
    fr fr Test number
    tea num := 42
    tea jsonNum, err := json_tea.Marshal(num)
    lowkey err != cap {
        vibez.spill("Error marshaling number:", err)
        yolo
    }
    vibez.spill("Number JSON:", jsonNum)
    
    tea numBack := 0
    err = json_tea.Unmarshal(jsonNum, &numBack)
    lowkey err != cap {
        vibez.spill("Error unmarshaling number:", err)
        yolo
    }
    vibez.spill("Unmarshaled number:", numBack)
    lowkey numBack != num {
        vibez.spill("Number mismatch! Got:", numBack, "Expected:", num)
        yolo
    }
    
    fr fr Test boolean
    tea boolVal := based
    tea jsonBool, err := json_tea.Marshal(boolVal)
    lowkey err != cap {
        vibez.spill("Error marshaling boolean:", err)
        yolo
    }
    vibez.spill("Boolean JSON:", jsonBool)
    
    tea boolBack := cap
    err = json_tea.Unmarshal(jsonBool, &boolBack)
    lowkey err != cap {
        vibez.spill("Error unmarshaling boolean:", err)
        yolo
    }
    vibez.spill("Unmarshaled boolean:", boolBack)
    lowkey boolBack != boolVal {
        vibez.spill("Boolean mismatch! Got:", boolBack, "Expected:", boolVal)
        yolo
    }
    
    fr fr Test array
    tea arr := []normie{1, 2, 3, 4, 5}
    tea jsonArr, err := json_tea.Marshal(arr)
    lowkey err != cap {
        vibez.spill("Error marshaling array:", err)
        yolo
    }
    vibez.spill("Array JSON:", jsonArr)
    
    tea arrBack := []normie{}
    err = json_tea.Unmarshal(jsonArr, &arrBack)
    lowkey err != cap {
        vibez.spill("Error unmarshaling array:", err)
        yolo
    }
    vibez.spill("Unmarshaled array length:", len(arrBack))
    lowkey len(arrBack) != len(arr) {
        vibez.spill("Array length mismatch! Got:", len(arrBack), "Expected:", len(arr))
        yolo
    }
    
    fr fr Test map
    tea mp := tea[tea]{
        "key1": "value1",
        "key2": "value2",
    }
    tea jsonMap, err := json_tea.Marshal(mp)
    lowkey err != cap {
        vibez.spill("Error marshaling map:", err)
        yolo
    }
    vibez.spill("Map JSON:", jsonMap)
    
    tea mpBack := tea[tea]{}
    err = json_tea.Unmarshal(jsonMap, &mpBack)
    lowkey err != cap {
        vibez.spill("Error unmarshaling map:", err)
        yolo
    }
    vibez.spill("Unmarshaled map keys:", len(mpBack))
    lowkey len(mpBack) != len(mp) {
        vibez.spill("Map length mismatch! Got:", len(mpBack), "Expected:", len(mp))
        yolo
    }
    
    vibez.spill("Basic type tests passed!")
}

fr fr Test marshaling and unmarshaling of structs
slay test_struct_marshaling() {
    vibez.spill("Testing struct marshaling...")
    
    fr fr Create a test person
    tea person := Person{
        Name: "John Doe",
        Age: 30,
        Email: "john@example.com",
        Active: based,
    }
    
    fr fr Marshal to JSON
    tea jsonPerson, err := json_tea.Marshal(person)
    lowkey err != cap {
        vibez.spill("Error marshaling person:", err)
        yolo
    }
    vibez.spill("Person JSON:", jsonPerson)
    
    fr fr Verify JSON contains expected fields
    lowkey !json_tea.Contains(jsonPerson, "John Doe") {
        vibez.spill("Name field missing in JSON")
        yolo
    }
    
    lowkey !json_tea.Contains(jsonPerson, "30") {
        vibez.spill("Age field missing in JSON")
        yolo
    }
    
    lowkey !json_tea.Contains(jsonPerson, "john@example.com") {
        vibez.spill("Email field missing in JSON")
        yolo
    }
    
    vibez.spill("Struct marshaling test passed!")
}

fr fr Test unmarshaling JSON into structs
slay test_struct_unmarshaling() {
    vibez.spill("Testing struct unmarshaling...")
    
    fr fr JSON data for a person
    tea jsonPerson := `{
        "Name": "Jane Smith",
        "Age": 28,
        "Email": "jane@example.com",
        "Active": true
    }`
    
    fr fr Unmarshal to struct
    tea person := Person{}
    tea err := json_tea.Unmarshal(jsonPerson, &person)
    lowkey err != cap {
        vibez.spill("Error unmarshaling to person:", err)
        yolo
    }
    
    fr fr Verify fields
    vibez.spill("Unmarshaled person:")
    vibez.spill("  Name:", person.Name)
    vibez.spill("  Age:", person.Age)
    vibez.spill("  Email:", person.Email)
    vibez.spill("  Active:", person.Active)
    
    fr fr Check field values
    lowkey person.Name != "Jane Smith" {
        vibez.spill("Name field mismatch! Got:", person.Name, "Expected: Jane Smith")
        yolo
    }
    
    lowkey person.Age != 28 {
        vibez.spill("Age field mismatch! Got:", person.Age, "Expected: 28")
        yolo
    }
    
    lowkey !person.Active {
        vibez.spill("Active field mismatch! Expected: true")
        yolo
    }
    
    vibez.spill("Struct unmarshaling test passed!")
}

fr fr Test complex objects with nested structures
slay test_complex_objects() {
    vibez.spill("Testing complex objects...")
    
    fr fr Create complex object with nested structures
    tea complex := ComplexObject{
        ID: 123456789,
        Owner: Person{
            Name: "Bob Johnson", 
            Age: 42,
            Email: "bob@example.com",
            Active: based,
        },
        Addresses: []Address{
            Address{
                Street: "123 Main St",
                City: "New York",
                ZipCode: "10001",
                Country: "USA",
            },
            Address{
                Street: "456 Park Ave",
                City: "Boston",
                ZipCode: "02108",
                Country: "USA",
            },
        },
        Tags: []tea{"developer", "gamer", "musician"},
        Metadata: tea[tea]{
            "department": "Engineering",
            "level": "Senior",
            "startYear": "2015",
        },
        NullField: cap, fr fr Null pointer
    }
    
    fr fr Marshal to JSON
    tea jsonComplex, err := json_tea.Marshal(complex)
    lowkey err != cap {
        vibez.spill("Error marshaling complex object:", err)
        yolo
    }
    vibez.spill("Complex object JSON:", jsonComplex)
    
    fr fr Unmarshal back
    tea complexBack := ComplexObject{}
    err = json_tea.Unmarshal(jsonComplex, &complexBack)
    lowkey err != cap {
        vibez.spill("Error unmarshaling complex object:", err)
        yolo
    }
    
    fr fr Verify fields
    lowkey complexBack.ID != complex.ID {
        vibez.spill("ID mismatch!", complexBack.ID, "!=", complex.ID)
        yolo
    }
    
    lowkey complexBack.Owner.Name != complex.Owner.Name {
        vibez.spill("Owner name mismatch!", complexBack.Owner.Name, "!=", complex.Owner.Name)
        yolo
    }
    
    lowkey len(complexBack.Addresses) != len(complex.Addresses) {
        vibez.spill("Addresses length mismatch!", len(complexBack.Addresses), "!=", len(complex.Addresses))
        yolo
    }
    
    lowkey len(complexBack.Tags) != len(complex.Tags) {
        vibez.spill("Tags length mismatch!", len(complexBack.Tags), "!=", len(complex.Tags))
        yolo
    }
    
    lowkey len(complexBack.Metadata) != len(complex.Metadata) {
        vibez.spill("Metadata length mismatch!", len(complexBack.Metadata), "!=", len(complex.Metadata))
        yolo
    }
    
    fr fr Check addresses details
    lowkey complexBack.Addresses[0].City != "New York" {
        vibez.spill("First address city mismatch!", complexBack.Addresses[0].City, "!= New York")
        yolo
    }
    
    vibez.spill("Complex object test passed!")
}

fr fr Custom marshaling types example
be_like CustomJSON squad {
    rawData tea
}

fr fr Implement custom marshaler method
slay (c CustomJSON) MarshalJSON() (tea, tea) {
    yolo c.rawData, cap
}

fr fr Implement custom unmarshaler method
slay (c @CustomJSON) UnmarshalJSON(data tea) tea {
    c.rawData = data
    yolo cap
}

slay test_custom_marshaling() {
    vibez.spill("Testing custom marshaling...")
    
    fr fr Create an object with custom marshaling
    tea custom := CustomJSON{
        rawData: `{"custom":"data","with":"special format"}`,
    }
    
    fr fr Marshal
    tea jsonCustom, err := json_tea.Marshal(custom)
    lowkey err != cap {
        vibez.spill("Error marshaling custom object:", err)
        yolo
    }
    vibez.spill("Custom JSON:", jsonCustom)
    
    fr fr Unmarshal
    tea customBack := CustomJSON{}
    err = json_tea.Unmarshal(jsonCustom, &customBack)
    lowkey err != cap {
        vibez.spill("Error unmarshaling custom object:", err)
        yolo
    }
    
    fr fr Verify
    vibez.spill("Custom raw data:", customBack.rawData)
    
    fr fr Check if raw data matches
    lowkey !json_tea.Contains(customBack.rawData, "special format") {
        vibez.spill("Custom data doesn't match")
        yolo
    }
    
    vibez.spill("Custom marshaling test passed!")
}