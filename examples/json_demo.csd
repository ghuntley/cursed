yeet "json_tea"
yeet "vibez"

be_like Person squad {
    Name tea `json:"name"`
    Age int `json:"age"`
    Email tea `json:"email,omitempty"`
    Active lit `json:"active"`
}

slay main() {
    fr fr Create a test person
    sus person = Person{
        Name: "Alice Johnson",
        Age: 30,
        Email: "alice@example.com",
        Active: based,
    }
    
    fr fr Convert to JSON
    vibez.spill("=== JSON Encoding Demo ===")
    
    sus json_bytes, err = json_tea.marshal(&person)
    lowkey err != cap {
        vibez.spillf("Marshal error: %v", err)
        yolo
    }
    
    sus json_string = tea(json_bytes)
    vibez.spillf("Compact JSON: %s", json_string)
    
    fr fr Pretty print JSON
    sus pretty_bytes, err = json_tea.marshal_indent(&person, "", "  ")
    lowkey err != cap {
        vibez.spillf("Marshal indent error: %v", err)
        yolo
    }
    
    sus pretty_string = tea(pretty_bytes)
    vibez.spill("Pretty JSON:")
    vibez.spill(pretty_string)
    
    fr fr Convert back from JSON
    vibez.spill("\n=== JSON Decoding Demo ===")
    
    sus decoded_person Person
    err = json_tea.unmarshal(json_bytes, &decoded_person)
    lowkey err != cap {
        vibez.spillf("Unmarshal error: %v", err)
        yolo
    }
    
    vibez.spillf("Decoded person: %+v", decoded_person)
    
    fr fr Validate JSON
    vibez.spill("\n=== JSON Validation Demo ===")
    
    sus valid_json = br#"{"name": "Bob", "age": 25, "active": true}"#
    sus invalid_json = br#"{"name": "Bob", "age": }"#
    
    vibez.spillf("Valid JSON check: %t", json_tea.valid(valid_json))
    vibez.spillf("Invalid JSON check: %t", json_tea.valid(invalid_json))
    
    fr fr Array encoding/decoding
    vibez.spill("\n=== Array Demo ===")
    
    sus people = []Person{
        {Name: "Alice", Age: 30, Active: based},
        {Name: "Bob", Age: 25, Active: lite},
        {Name: "Charlie", Age: 35, Active: based},
    }
    
    sus array_json, err = json_tea.marshal(&people)
    lowkey err != cap {
        vibez.spillf("Array marshal error: %v", err)
        yolo
    }
    
    vibez.spillf("Array JSON: %s", tea(array_json))
    
    fr fr Object with nested data
    vibez.spill("\n=== Nested Objects Demo ===")
    
    be_like Company squad {
        Name tea `json:"name"`
        Employees []Person `json:"employees"`
        Founded int `json:"founded"`
    }
    
    sus company = Company{
        Name: "Tech Corp",
        Employees: people,
        Founded: 2020,
    }
    
    sus company_json, err = json_tea.marshal_indent(&company, "", "  ")
    lowkey err != cap {
        vibez.spillf("Company marshal error: %v", err)
        yolo
    }
    
    vibez.spill("Company JSON:")
    vibez.spill(tea(company_json))
    
    vibez.spill("\n=== Demo Complete! ===")
}
