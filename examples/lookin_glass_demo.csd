#!/usr/bin/env cursed

fr fr LookinGlass Reflection Demo - Comprehensive demonstration of CURSED reflection capabilities
fr fr This example shows how to use the LookinGlass package for runtime type inspection and manipulation

yeet "stdlib::lookin_glass"
yeet "stdlib::vibez"

fr fr Define some sample types for reflection
be_like Person squad {
    Name tea `json:"name" db:"full_name" validate:"required,min=2"`
    Age normie `json:"age,omitempty" validate:"min=0,max=150"`
    Email tea `json:"email" validate:"email"`
    Address *Address `json:"address,omitempty"`
}

be_like Address squad {
    Street tea `json:"street"`
    City tea `json:"city"`
    State tea `json:"state"`
    ZipCode tea `json:"zip_code" db:"postal_code"`
    Country tea `json:"country" default:"USA"`
}

be_like ContactInfo squad {
    Phone tea `json:"phone,omitempty" validate:"phone"`
    Website tea `json:"website,omitempty" validate:"url"`
    SocialMedia map[tea]tea `json:"social_media,omitempty"`
}

slay main() {
    vibez.spill("🔍 LookinGlass Reflection Demo - Examining Types at Runtime")
    vibez.spill("=" * 60)
    
    // Initialize the reflection system
    lookin_glass.initialize()
    
    // Demo 1: Basic Type Inspection
    demo_basic_type_inspection()
    
    // Demo 2: Struct Field Analysis
    demo_struct_field_analysis()
    
    // Demo 3: Tag Parsing and Metadata
    demo_tag_parsing()
    
    // Demo 4: Deep Operations
    demo_deep_operations()
    
    // Demo 5: VibeMapper Usage
    demo_vibe_mapper()
    
    // Demo 6: Dynamic Type Creation
    demo_dynamic_type_creation()
    
    // Demo 7: Collection Reflection
    demo_collection_reflection()
    
    vibez.spill("\n✨ LookinGlass reflection demo completed successfully!")
}

slay demo_basic_type_inspection() {
    vibez.spill("\n📋 Demo 1: Basic Type Inspection")
    vibez.spill("-" * 40)
    
    // Create some basic values
    sus name = "Alice"
    sus age = 25
    sus active = lit
    sus score = 95.5
    
    // Get types using reflection
    facts name_type = lookin_glass.type_of(name)
    facts age_type = lookin_glass.type_of(age)
    facts active_type = lookin_glass.type_of(active)
    facts score_type = lookin_glass.type_of(score)
    
    vibez.spill("Type of name ('$name'): $name_type")
    vibez.spill("Type of age ($age): $age_type")
    vibez.spill("Type of active ($active): $active_type") 
    vibez.spill("Type of score ($score): $score_type")
    
    // Check type properties
    vibez.spill("\nType Properties:")
    vibez.spill("- name_type.kind(): ${name_type.kind()}")
    vibez.spill("- name_type.is_basic(): ${name_type.is_basic()}")
    vibez.spill("- name_type.comparable(): ${name_type.comparable()}")
    vibez.spill("- age_type.size(): ${age_type.size()}")
    vibez.spill("- age_type.align(): ${age_type.align()}")
}

slay demo_struct_field_analysis() {
    vibez.spill("\n🏗️  Demo 2: Struct Field Analysis")
    vibez.spill("-" * 40)
    
    // Create a Person instance
    sus person = Person{
        Name: "Bob Johnson",
        Age: 32,
        Email: "bob@example.com",
        Address: &Address{
            Street: "456 Oak Avenue",
            City: "Springfield",
            State: "IL",
            ZipCode: "62701",
            Country: "USA"
        }
    }
    
    // Get the type and examine its structure
    facts person_type = lookin_glass.type_of(person)
    
    vibez.spill("Analyzing struct: ${person_type.name()}")
    vibez.spill("Package: ${person_type.pkg_path()}")
    vibez.spill("Number of fields: ${person_type.num_field()}")
    
    // Iterate through all fields
    vibez.spill("\nField Details:")
    yolo (sus i = 0; i < person_type.num_field(); i++) {
        facts field = person_type.field(i)
        facts field_value = lookin_glass.get_field(person, field.name())
        
        vibez.spill("  Field $i:")
        vibez.spill("    Name: ${field.name()}")
        vibez.spill("    Type: ${field.field_type()}")
        vibez.spill("    Exported: ${field.is_exported()}")
        vibez.spill("    Anonymous: ${field.is_anonymous()}")
        vibez.spill("    Current Value: $field_value")
        
        lowkey (field.tag().len() > 0) {
            vibez.spill("    Tags: ${field.tag()}")
        }
    }
    
    // Test field access by name
    vibez.spill("\nAccessing fields by name:")
    facts name_field = lookin_glass.get_field(person, "Name")
    facts age_field = lookin_glass.get_field(person, "Age")
    
    vibez.spill("- Name field value: $name_field")
    vibez.spill("- Age field value: $age_field")
    vibez.spill("- Has Email field: ${lookin_glass.has_field(person, 'Email')}")
    vibez.spill("- Has Phone field: ${lookin_glass.has_field(person, 'Phone')}")
}

slay demo_tag_parsing() {
    vibez.spill("\n🏷️  Demo 3: Tag Parsing and Metadata")
    vibez.spill("-" * 40)
    
    sus contact = ContactInfo{
        Phone: "+1-555-0123",
        Website: "https://example.com",
        SocialMedia: {
            "twitter": "@johndoe",
            "linkedin": "john-doe",
            "github": "johndoe"
        }
    }
    
    facts contact_type = lookin_glass.type_of(contact)
    facts all_tags = lookin_glass.get_tags(contact)
    
    vibez.spill("Examining tags for ContactInfo struct:")
    
    yolo (facts field_name, facts tags) in all_tags {
        vibez.spill("\nField: $field_name")
        yolo (facts tag_key, facts tag_value) in tags {
            vibez.spill("  $tag_key: '$tag_value'")
        }
        
        // Parse specific tag types
        facts field_info = contact_type.field_by_name(field_name)
        lowkey (field_info.has_tag("json")) {
            facts json_name = field_info.json_name()
            facts omit_empty = field_info.omit_empty()
            facts ignored = field_info.json_ignored()
            
            vibez.spill("  JSON Analysis:")
            vibez.spill("    - JSON name: ${json_name.unwrap_or('(none)')}")
            vibez.spill("    - Omit empty: $omit_empty")
            vibez.spill("    - Ignored: $ignored")
        }
        
        lowkey (field_info.has_tag("validate")) {
            facts validation_rules = field_info.validation_rules()
            vibez.spill("  Validation rules: $validation_rules")
        }
    }
}

slay demo_deep_operations() {
    vibez.spill("\n🔄 Demo 4: Deep Operations (Copy, Equal, Merge)")
    vibez.spill("-" * 40)
    
    // Create test data
    sus original_person = Person{
        Name: "Charlie Brown",
        Age: 28,
        Email: "charlie@peanuts.com",
        Address: &Address{
            Street: "123 Peanut Lane",
            City: "Peanutville",
            State: "CA",
            ZipCode: "90210"
        }
    }
    
    // Test deep copy
    vibez.spill("Testing deep copy...")
    facts copied_person = lookin_glass.deep_copy(original_person)
    facts are_equal = lookin_glass.deep_equal(original_person, copied_person)
    
    vibez.spill("Original and copy are equal: $are_equal")
    
    // Modify the copy to test independence
    copied_person.Name = "Charlie Brown Jr."
    copied_person.Age = 5
    
    facts still_equal = lookin_glass.deep_equal(original_person, copied_person)
    vibez.spill("After modification, still equal: $still_equal")
    
    vibez.spill("\nOriginal person name: ${original_person.Name}")
    vibez.spill("Copied person name: ${copied_person.Name}")
    
    // Test struct-to-map conversion
    vibez.spill("\nTesting struct-to-map conversion...")
    facts person_map = lookin_glass.struct_to_map(original_person)
    
    vibez.spill("Person as map:")
    yolo (facts key, facts value) in person_map {
        vibez.spill("  $key: $value")
    }
    
    // Test map-to-struct conversion
    person_map["name"] = "David Brown"
    person_map["age"] = 45
    
    facts person_from_map = lookin_glass.map_to_struct(person_map, lookin_glass.type_of(original_person))
    vibez.spill("\nReconstructed person from modified map:")
    vibez.spill("Name: ${person_from_map.Name}")
    vibez.spill("Age: ${person_from_map.Age}")
}

slay demo_vibe_mapper() {
    vibez.spill("\n🗺️  Demo 5: VibeMapper - Advanced Mapping and JSON")
    vibez.spill("-" * 40)
    
    // Create a VibeMapper with custom configuration
    facts mapper = lookin_glass.VibeMapper()
        .use_json_tags(lit)
        .omit_empty(lit)
        .field_name_transformer(lookin_glass.camel_to_snake)
    
    sus person = Person{
        Name: "Emma Watson",
        Age: 33,
        Email: "emma@example.com"
    }
    
    // Convert to JSON
    vibez.spill("Converting to JSON...")
    facts json_bytes = mapper.to_json(person)
    facts json_string = tea.from_bytes(json_bytes)
    vibez.spill("JSON representation:")
    vibez.spill(json_string)
    
    // Convert back from JSON
    vibez.spill("\nConverting back from JSON...")
    facts person_from_json = mapper.from_json(json_bytes, lookin_glass.type_of(person))
    vibez.spill("Reconstructed person name: ${person_from_json.Name}")
    
    // Test field name transformations
    vibez.spill("\nField name transformations:")
    vibez.spill("camelCase -> snake_case:")
    vibez.spill("  firstName -> ${lookin_glass.camel_to_snake('firstName')}")
    vibez.spill("  XMLHttpRequest -> ${lookin_glass.camel_to_snake('XMLHttpRequest')}")
    
    vibez.spill("snake_case -> camelCase:")
    vibez.spill("  first_name -> ${lookin_glass.snake_to_camel('first_name')}")
    vibez.spill("  user_profile_data -> ${lookin_glass.snake_to_camel('user_profile_data')}")
    
    // Test merge operation
    sus person1 = Person{Name: "Alice", Age: 25}
    sus person2 = Person{Email: "alice@example.com"}
    
    facts merged = mapper.merge(person1, person2)
    vibez.spill("\nMerged person:")
    vibez.spill("Name: ${merged.Name}")
    vibez.spill("Age: ${merged.Age}")
    vibez.spill("Email: ${merged.Email}")
}

slay demo_dynamic_type_creation() {
    vibez.spill("\n🛠️  Demo 6: Dynamic Type Creation")
    vibez.spill("-" * 40)
    
    // Create types dynamically
    vibez.spill("Creating dynamic types...")
    
    // Create array type
    facts int_array_type = lookin_glass.array_of(lookin_glass.Type.basic(lookin_glass.Kind.Int32), 10)
    vibez.spill("Array type: $int_array_type")
    vibez.spill("Array length: ${int_array_type.len()}")
    
    // Create slice type
    facts string_slice_type = lookin_glass.slice_of(lookin_glass.Type.basic(lookin_glass.Kind.String))
    vibez.spill("Slice type: $string_slice_type")
    
    // Create map type
    facts string_int_map_type = lookin_glass.map_of(
        lookin_glass.Type.basic(lookin_glass.Kind.String),
        lookin_glass.Type.basic(lookin_glass.Kind.Int32)
    )
    vibez.spill("Map type: $string_int_map_type")
    
    // Create pointer type
    facts int_ptr_type = lookin_glass.ptr_to(lookin_glass.Type.basic(lookin_glass.Kind.Int32))
    vibez.spill("Pointer type: $int_ptr_type")
    
    // Create function type
    facts func_type = lookin_glass.func_of(
        [lookin_glass.Type.basic(lookin_glass.Kind.String), lookin_glass.Type.basic(lookin_glass.Kind.Int32)],
        [lookin_glass.Type.basic(lookin_glass.Kind.Bool)],
        cap
    )
    vibez.spill("Function type: $func_type")
    vibez.spill("Function input count: ${func_type.num_in()}")
    vibez.spill("Function output count: ${func_type.num_out()}")
    vibez.spill("Function is variadic: ${func_type.is_variadic()}")
}

slay demo_collection_reflection() {
    vibez.spill("\n📚 Demo 7: Collection Reflection")
    vibez.spill("-" * 40)
    
    // Create various collections
    sus numbers = [1, 2, 3, 4, 5]
    sus names = ["Alice", "Bob", "Charlie"]
    sus scores = {
        "Alice": 95,
        "Bob": 87,
        "Charlie": 92
    }
    
    // Reflect on slice
    facts numbers_type = lookin_glass.type_of(numbers)
    vibez.spill("Numbers slice:")
    vibez.spill("  Type: $numbers_type")
    vibez.spill("  Kind: ${numbers_type.kind()}")
    vibez.spill("  Element type: ${numbers_type.elem()}")
    vibez.spill("  Length: ${numbers.len()}")
    
    // Create slice using reflection
    facts int_slice_type = lookin_glass.slice_of(lookin_glass.Type.basic(lookin_glass.Kind.Int32))
    facts new_slice = lookin_glass.make_slice(int_slice_type, 3, 10)
    
    vibez.spill("\nDynamically created slice:")
    vibez.spill("  Length: ${new_slice.len()}")
    vibez.spill("  Capacity: ${new_slice.cap()}")
    
    // Reflect on map
    facts scores_type = lookin_glass.type_of(scores)
    vibez.spill("\nScores map:")
    vibez.spill("  Type: $scores_type")
    vibez.spill("  Key type: ${scores_type.key()}")
    vibez.spill("  Element type: ${scores_type.elem()}")
    vibez.spill("  Length: ${scores.len()}")
    
    // Create map using reflection
    facts string_int_map_type = lookin_glass.map_of(
        lookin_glass.Type.basic(lookin_glass.Kind.String),
        lookin_glass.Type.basic(lookin_glass.Kind.Int32)
    )
    facts new_map = lookin_glass.make_map(string_int_map_type)
    
    vibez.spill("\nDynamically created map:")
    vibez.spill("  Type: ${new_map.typ()}")
    vibez.spill("  Length: ${new_map.len()}")
    
    // Test zero value creation
    vibez.spill("\nZero values:")
    facts zero_bool = lookin_glass.zero(lookin_glass.Type.basic(lookin_glass.Kind.Bool))
    facts zero_int = lookin_glass.zero(lookin_glass.Type.basic(lookin_glass.Kind.Int32))
    facts zero_string = lookin_glass.zero(lookin_glass.Type.basic(lookin_glass.Kind.String))
    
    vibez.spill("  Zero bool: $zero_bool (is_zero: ${zero_bool.is_zero()})")
    vibez.spill("  Zero int: $zero_int (is_zero: ${zero_int.is_zero()})")
    vibez.spill("  Zero string: '$zero_string' (is_zero: ${zero_string.is_zero()})")
    
    // Test pointer creation and indirection
    facts int_value = lookin_glass.value_of(42)
    facts ptr_value = lookin_glass.new(lookin_glass.type_of(int_value))
    facts indirect_value = lookin_glass.indirect(ptr_value)
    
    vibez.spill("\nPointer operations:")
    vibez.spill("  Original value: $int_value")
    vibez.spill("  Pointer type: ${ptr_value.typ()}")
    vibez.spill("  Is nil: ${ptr_value.is_nil()}")
    vibez.spill("  Indirect value: $indirect_value")
}
