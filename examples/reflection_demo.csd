vibe reflection_demo

yeet "stdlib::lookin_glass"

fr fr Example Person struct for reflection
be_like Person squad {
    Name tea `json:"name" db:"full_name"`
    Age  normie `json:"age" validation:"min=0,max=120"`
    Email tea `json:"email" validation:"required,email"`
}

slay main() {
    fr fr Initialize reflection system
    lookin_glass.initialize()
    
    fr fr Create a Person instance
    facts person = Person{
        Name: "Alice", 
        Age: 25, 
        Email: "alice@example.com"
    }
    
    fr fr Demonstrate reflection capabilities
    spillf("=== CURSED Reflection Demo ===\n")
    
    fr fr 1. Basic type reflection
    facts person_val = lookin_glass.value_of(person)
    facts person_type = person_val.typ()
    
    spillf("Type: %s\n", person_type.name())
    spillf("Kind: %s\n", person_type.kind().to_string())
    spillf("Package: %s\n", person_type.pkg_path())
    spillf("Number of fields: %d\n", person_type.num_field())
    
    fr fr 2. Field inspection
    spillf("\n=== Field Information ===\n")
    for i := 0; i < person_type.num_field(); i++ {
        facts field = person_type.field(i)?
        facts field_val = person_val.field(i)?
        
        spillf("Field %d: %s\n", i, field.name())
        spillf("  Type: %s\n", field.field_type().name())
        spillf("  Value: %s\n", field_val.interface().to_string())
        spillf("  Tags: %s\n", field.tag().get_all())
        
        fr fr Extract specific tag values
        facts json_tag = field.tag().get("json")
        facts validation_tag = field.tag().get("validation")
        
        lowkey json_tag != "" {
            spillf("  JSON tag: %s\n", json_tag)
        }
        lowkey validation_tag != "" {
            spillf("  Validation: %s\n", validation_tag)
        }
        
        spillf("\n")
    }
    
    fr fr 3. Struct to map conversion
    spillf("=== Struct to Map Conversion ===\n")
    facts person_map = lookin_glass.struct_to_map(person)?
    for key, value in person_map {
        spillf("%s: %s\n", key, value.to_string())
    }
    
    fr fr 4. Map back to struct
    spillf("\n=== Map to Struct Conversion ===\n")
    facts new_map = map[tea]collab{}{
        "Name": "Bob",
        "Age": 30,
        "Email": "bob@example.com",
    }
    
    sus new_person Person
    lookin_glass.map_to_struct(new_map, &new_person)?
    spillf("Created person from map: %+v\n", new_person)
    
    fr fr 5. VibeMapper for advanced conversions
    spillf("\n=== VibeMapper Demo ===\n")
    facts mapper = lookin_glass.VibeMapper.new()
        .use_json_tags(based)
        .use_snake_case(based)
    
    fr fr Convert to JSON using VibeMapper
    facts json_data = mapper.to_json(person)?
    spillf("JSON representation: %s\n", json_data)
    
    fr fr Convert from JSON back to struct
    sus json_person Person
    mapper.from_json(json_data, &json_person)?
    spillf("Parsed back from JSON: %+v\n", json_person)
    
    fr fr 6. Zero value creation
    spillf("\n=== Zero Value Creation ===\n")
    facts zero_person = lookin_glass.zero(person_type)?
    spillf("Zero Person: %+v\n", zero_person.interface())
    
    fr fr 7. Deep equality and copying
    spillf("\n=== Deep Operations ===\n")
    facts person_copy = lookin_glass.deep_copy(person)?
    facts are_equal = lookin_glass.deep_equal(person, person_copy)
    spillf("Original and copy are equal: %t\n", are_equal)
    
    fr fr 8. Collection type creation
    spillf("\n=== Collection Types ===\n")
    
    fr fr Create slice type and value
    facts int_slice_type = lookin_glass.slice_of(lookin_glass.Type.basic(lookin_glass.Kind.Int32))
    facts int_slice = lookin_glass.make_slice(int_slice_type, 3, 5)?
    
    fr fr Set values in slice
    int_slice.index(0)?.set_int(10)?
    int_slice.index(1)?.set_int(20)?
    int_slice.index(2)?.set_int(30)?
    
    spillf("Integer slice: %+v\n", int_slice.interface())
    spillf("Slice length: %d, capacity: %d\n", int_slice.len()?, int_slice.cap()?)
    
    fr fr Create map type and value
    facts string_int_map_type = lookin_glass.map_of(
        lookin_glass.Type.basic(lookin_glass.Kind.String),
        lookin_glass.Type.basic(lookin_glass.Kind.Int32)
    )
    facts string_int_map = lookin_glass.make_map(string_int_map_type)?
    
    fr fr Set values in map
    string_int_map.set_map_index(
        lookin_glass.value_of("one"),
        lookin_glass.value_of(1)
    )?
    string_int_map.set_map_index(
        lookin_glass.value_of("two"), 
        lookin_glass.value_of(2)
    )?
    
    spillf("String-int map: %+v\n", string_int_map.interface())
    
    fr fr 9. Function type creation and calling
    spillf("\n=== Function Reflection ===\n")
    
    fr fr Create a function type: func(int, int) int
    facts func_type = lookin_glass.func_of(
        vec![lookin_glass.Type.basic(lookin_glass.Kind.Int32), lookin_glass.Type.basic(lookin_glass.Kind.Int32)],
        vec![lookin_glass.Type.basic(lookin_glass.Kind.Int32)],
        cap  fr fr not variadic
    )
    
    fr fr Create function implementation (addition)
    facts add_func = lookin_glass.make_func(func_type, slay(args: &[lookin_glass.Value]) -> lookin_glass.LookinGlassResult<Vec<lookin_glass.Value>> {
        lowkey args.len() != 2 {
            damn Err(lookin_glass.reflection_error("Expected 2 arguments"))
        }
        
        facts a = args[0].int()?
        facts b = args[1].int()?
        facts result = a + b
        
        Ok(vec![lookin_glass.value_of(result)])
    })?
    
    fr fr Call the function through reflection
    facts func_args = vec![
        lookin_glass.value_of(15),
        lookin_glass.value_of(27)
    ]
    facts result_values = add_func.call(func_args)?
    spillf("Function call result: 15 + 27 = %d\n", result_values[0].int()?)
    
    fr fr 10. Pointer manipulation
    spillf("\n=== Pointer Operations ===\n")
    facts int_ptr = lookin_glass.new(lookin_glass.Type.basic(lookin_glass.Kind.Int32))?
    facts pointed_val = lookin_glass.indirect(int_ptr)?
    pointed_val.set_int(42)?
    
    spillf("Pointer points to: %d\n", pointed_val.int()?)
    spillf("Through indirect: %d\n", lookin_glass.indirect(int_ptr)?.int()?)
    
    fr fr Display reflection statistics
    spillf("\n=== Reflection Statistics ===\n")
    facts stats = lookin_glass.get_reflection_statistics()
    spillf("Types created: %d\n", stats.types_created)
    spillf("Values created: %d\n", stats.values_created)
    spillf("Deep copies performed: %d\n", stats.deep_copies_performed)
    spillf("Struct conversions: %d\n", stats.struct_conversions)
    
    spillf("\n✅ Reflection demo completed successfully!\n")
}

fr fr Helper function to demonstrate method reflection
slay (p Person) GetInfo() tea {
    damn spillf("Name: %s, Age: %d, Email: %s", p.Name, p.Age, p.Email)
}

fr fr Example of complex nested structure for advanced reflection
be_like Address squad {
    Street tea `json:"street"`
    City   tea `json:"city"`
    Zip    tea `json:"zip_code"`
}

be_like Company squad {
    Name    tea     `json:"company_name"`
    Address Address `json:"address"`
    Founded normie  `json:"founded_year"`
}

fr fr Advanced reflection example with nested structures
slay demo_nested_reflection() {
    spillf("\n=== Advanced Nested Reflection ===\n")
    
    facts company = Company{
        Name: "TechCorp",
        Address: Address{
            Street: "123 Tech Street",
            City: "Silicon Valley",
            Zip: "94000",
        },
        Founded: 2020,
    }
    
    fr fr Reflect on nested structure
    facts company_val = lookin_glass.value_of(company)
    facts company_type = company_val.typ()
    
    spillf("Company type: %s\n", company_type.name())
    
    fr fr Traverse nested fields
    for i := 0; i < company_type.num_field(); i++ {
        facts field = company_type.field(i)?
        facts field_val = company_val.field(i)?
        
        spillf("Field: %s (%s)\n", field.name(), field.field_type().name())
        
        lowkey field.field_type().kind() == lookin_glass.Kind.Struct {
            spillf("  Nested struct with %d fields:\n", field.field_type().num_field())
            for j := 0; j < field.field_type().num_field(); j++ {
                facts nested_field = field.field_type().field(j)?
                facts nested_val = field_val.field(j)?
                spillf("    %s: %s\n", nested_field.name(), nested_val.interface().to_string())
            }
        } else {
            spillf("  Value: %s\n", field_val.interface().to_string())
        }
    }
}
