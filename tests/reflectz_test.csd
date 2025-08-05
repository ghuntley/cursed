vibe main

yeet "vibez"    fr fr For printing results
yeet "reflectz" fr fr Reflection functionality

fr fr Test structures
be_like Person squad {
    Name tea
    Age normie
    Active lit
}

be_like Employee squad {
    ID thicc
    Person Person
    Department tea
    Salary meal
}

be_like Shape collab {
    Area() meal
    Perimeter() meal
}

be_like Rectangle squad {
    Width meal
    Height meal
}

slay (r Rectangle) Area() meal {
    damn r.Width * r.Height
}

slay (r Rectangle) Perimeter() meal {
    damn 2 * (r.Width + r.Height)
}

slay main() {
    vibez.spill("Testing reflectz package")
    
    fr fr Test basic type reflection
    test_basic_types()
    
    fr fr Test struct reflection
    test_struct_reflection()
    
    fr fr Test dynamic access to struct fields
    test_field_access()
    
    fr fr Test method reflection
    test_method_reflection()
    
    fr fr Test calling methods dynamically
    test_method_calling()
    
    vibez.spill("All reflectz tests passed!")
}

fr fr Test basic type reflection
slay test_basic_types() {
    vibez.spill("Testing basic type reflection...")
    
    fr fr Get type of integer
    tea num := 42
    tea numType := reflectz.TypeOf(num)
    vibez.spill("Type of 42:", numType.Name())
    lowkey numType.Name() != "normie" && numType.Name() != "int" && numType.Name() != "int32" {
        vibez.spill("Invalid type name for integer. Got '", numType.Name(), "'")
        damn
    }
    
    lowkey !numType.IsBasic() {
        vibez.spill("Integer should be a basic type")
        damn
    }
    
    fr fr Get type of string
    tea str := "hello"
    tea strType := reflectz.TypeOf(str)
    vibez.spill("Type of string:", strType.Name())
    lowkey strType.Name() != "tea" && strType.Name() != "string" {
        vibez.spill("Invalid type name for string. Got '", strType.Name(), "'")
        damn
    }
    
    fr fr Get type of float
    tea flt := 3.14
    tea fltType := reflectz.TypeOf(flt)
    vibez.spill("Type of float:", fltType.Name())
    lowkey fltType.Name() != "meal" && fltType.Name() != "float64" {
        vibez.spill("Invalid type name for float. Got '", fltType.Name(), "'")
        damn
    }
    
    fr fr Get type of boolean
    tea boolean := based
    tea boolType := reflectz.TypeOf(boolean)
    vibez.spill("Type of boolean:", boolType.Name())
    lowkey boolType.Name() != "lit" && boolType.Name() != "bool" {
        vibez.spill("Invalid type name for boolean. Got '", boolType.Name(), "'")
        damn
    }
    
    fr fr Get type of array
    tea arr := []normie{1, 2, 3}
    tea arrType := reflectz.TypeOf(arr)
    vibez.spill("Type of array:", arrType.Name())
    lowkey !arrType.IsArray() {
        vibez.spill("Array type not recognized as array")
        damn
    }
    
    vibez.spill("Basic type reflection tests passed!")
}

fr fr Test struct reflection
slay test_struct_reflection() {
    vibez.spill("Testing struct reflection...")
    
    fr fr Create a struct
    tea person := Person{
        Name: "John Doe",
        Age: 30,
        Active: based,
    }
    
    fr fr Get type information
    tea personType := reflectz.TypeOf(person)
    vibez.spill("Type of Person:", personType.Name())
    lowkey personType.Name() != "Person" {
        vibez.spill("Invalid type name for Person. Got '", personType.Name(), "'")
        damn
    }
    
    lowkey !personType.IsStruct() {
        vibez.spill("Person should be a struct type")
        damn
    }
    
    fr fr Get fields
    tea fields := personType.Fields()
    vibez.spill("Fields of Person:", fields)
    lowkey len(fields) != 3 {
        vibez.spill("Expected 3 fields, got", len(fields))
        damn
    }
    
    fr fr Check field types
    bestie i, field := range fields {
        vibez.spill("Field", i, ":", field.Name, "of type", field.Type.Name())
        
        fr fr Verify field types are correct
        lowkey field.Name == "Name" && field.Type.Name() != "tea" && field.Type.Name() != "string" {
            vibez.spill("Invalid type for Name field. Got '", field.Type.Name(), "'")
            damn
        }
        
        lowkey field.Name == "Age" && field.Type.Name() != "normie" && field.Type.Name() != "int" && field.Type.Name() != "int32" {
            vibez.spill("Invalid type for Age field. Got '", field.Type.Name(), "'")
            damn
        }
        
        lowkey field.Name == "Active" && field.Type.Name() != "lit" && field.Type.Name() != "bool" {
            vibez.spill("Invalid type for Active field. Got '", field.Type.Name(), "'")
            damn
        }
    }
    
    fr fr Nested struct
    tea employee := Employee{
        ID: 12345,
        Person: person,
        Department: "Engineering",
        Salary: 75000.0,
    }
    
    tea employeeType := reflectz.TypeOf(employee)
    vibez.spill("Type of Employee:", employeeType.Name())
    
    fr fr Get fields of nested struct
    tea empFields := employeeType.Fields()
    lowkey len(empFields) != 4 {
        vibez.spill("Expected 4 fields for Employee, got", len(empFields))
        damn
    }
    
    fr fr Find Person field in Employee
    tea personField := cap
    bestie _, field := range empFields {
        lowkey field.Name == "Person" {
            personField = field
            vibez.spill("Found Person field with type:", field.Type.Name())
            break
        }
    }
    
    lowkey personField == cap {
        vibez.spill("Could not find Person field in Employee")
        damn
    }
    
    vibez.spill("Struct reflection tests passed!")
}

fr fr Test dynamic access to struct fields
slay test_field_access() {
    vibez.spill("Testing field access...")
    
    fr fr Create a struct
    tea person := Person{
        Name: "Jane Smith",
        Age: 25,
        Active: based,
    }
    
    fr fr Create a Value from person
    tea val := reflectz.ValueOf(person)
    
    fr fr Get fields by name
    tea nameField := val.FieldByName("Name")
    lowkey nameField.IsValid() == cap {
        vibez.spill("Could not find Name field")
        damn
    }
    
    fr fr Get field value
    tea nameValue := nameField.String()
    vibez.spill("Name field value:", nameValue)
    lowkey nameValue != "Jane Smith" {
        vibez.spill("Expected 'Jane Smith', got '", nameValue, "'")
        damn
    }
    
    fr fr Get Age field
    tea ageField := val.FieldByName("Age")
    tea ageValue := ageField.Int()
    vibez.spill("Age field value:", ageValue)
    lowkey ageValue != 25 {
        vibez.spill("Expected 25, got", ageValue)
        damn
    }
    
    fr fr Get all field values
    tea numFields := val.NumField()
    vibez.spill("Number of fields:", numFields)
    
    bestie i := 0; i < numFields; i++ {
        tea field := val.Field(i)
        tea fieldName := val.Type().Field(i).Name
        vibez.spill("Field", fieldName, "=", field.Interface())
    }
    
    fr fr Create a Value for a settable struct
    tea mutablePerson := Person{
        Name: "Original Name",
        Age: 30,
        Active: cap,
    }
    
    tea mutableVal := reflectz.ValueOf(&mutablePerson).Elem()
    
    fr fr Set field values
    tea nameFieldMut := mutableVal.FieldByName("Name")
    lowkey nameFieldMut.CanSet() {
        nameFieldMut.SetString("Modified Name")
        vibez.spill("Modified name to:", mutablePerson.Name)
        
        lowkey mutablePerson.Name != "Modified Name" {
            vibez.spill("Name field not modified. Expected 'Modified Name', got '", mutablePerson.Name, "'")
            damn
        }
    }
    
    vibez.spill("Field access tests passed!")
}

fr fr Test method reflection
slay test_method_reflection() {
    vibez.spill("Testing method reflection...")
    
    fr fr Create a Rectangle
    tea rect := Rectangle{Width: 10.0, Height: 5.0}
    
    fr fr Get type information
    tea rectType := reflectz.TypeOf(rect)
    vibez.spill("Type of Rectangle:", rectType.Name())
    
    fr fr Get methods
    tea methods := rectType.Methods()
    vibez.spill("Number of methods on Rectangle:", len(methods))
    
    fr fr Check specific methods
    bestie _, method := range methods {
        vibez.spill("Method:", method.Name)
        
        lowkey method.Name == "Area" || method.Name == "Perimeter" {
            fr fr Check return type
            tea returnType := method.Type.Out(0)
            vibez.spill("  Return type:", returnType.Name())
            
            lowkey returnType.Name() != "meal" && returnType.Name() != "float64" {
                vibez.spill("Invalid return type for", method.Name, ". Got '", returnType.Name(), "'")
                damn
            }
        }
    }
    
    fr fr Test interface reflection
    tea shapeType := reflectz.TypeOf((*Shape)(cap))
    vibez.spill("Shape interface has", shapeType.NumMethod(), "methods")
    
    fr fr Verify Rectangle implements Shape
    tea implsShape := reflectz.Implements(rectType, shapeType)
    vibez.spill("Rectangle implements Shape:", implsShape)
    lowkey !implsShape {
        vibez.spill("Rectangle should implement Shape interface")
        damn
    }
    
    vibez.spill("Method reflection tests passed!")
}

fr fr Test calling methods dynamically
slay test_method_calling() {
    vibez.spill("Testing method calling...")
    
    fr fr Create a Rectangle
    tea rect := Rectangle{Width: 4.0, Height: 3.0}
    
    fr fr Create a Value from rectangle
    tea val := reflectz.ValueOf(rect)
    
    fr fr Call Area method
    tea areaMethod := val.MethodByName("Area")
    lowkey areaMethod.IsValid() == cap {
        vibez.spill("Could not find Area method")
        damn
    }
    
    fr fr Call the method
    tea results := areaMethod.Call([]collab{})
    lowkey len(results) != 1 {
        vibez.spill("Expected 1 result, got", len(results))
        damn
    }
    
    fr fr Check result
    tea area := results[0].Float()
    vibez.spill("Area result:", area)
    
    fr fr Area should be Width * Height = 4 * 3 = 12
    lowkey area != 12.0 {
        vibez.spill("Expected area 12.0, got", area)
        damn
    }
    
    fr fr Call Perimeter method
    tea perimeterMethod := val.MethodByName("Perimeter")
    lowkey perimeterMethod.IsValid() == cap {
        vibez.spill("Could not find Perimeter method")
        damn
    }
    
    fr fr Call the method
    results = perimeterMethod.Call([]collab{})
    tea perimeter := results[0].Float()
    vibez.spill("Perimeter result:", perimeter)
    
    fr fr Perimeter should be 2 * (Width + Height) = 2 * (4 + 3) = 14
    lowkey perimeter != 14.0 {
        vibez.spill("Expected perimeter 14.0, got", perimeter)
        damn
    }
    
    vibez.spill("Method calling tests passed!")
}