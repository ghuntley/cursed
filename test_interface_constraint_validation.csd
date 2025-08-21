# Test interface constraint validation implementation

# Define a simple interface
collab Display {
    slay show() tea
    slay format() tea
}

# Define a struct that implements the interface
squad MyStruct {
    value normie
}

# Implementation of Display for MyStruct
impl Display for MyStruct {
    slay show() tea {
        damn "MyStruct: " + this.value.to_string()
    }
    
    slay format() tea {
        damn this.show()
    }
}

# Generic function with interface constraint
slay print_displayable<T: Display>(item T) tea {
    damn item.show()
}

# Test the constraint validation
slay main() tea {
    sus my_struct MyStruct = MyStruct { value: 42 }
    print_displayable(my_struct)  # This should work
    
    # This should fail constraint validation:
    # print_displayable(42)  # normie doesn't implement Display
}
