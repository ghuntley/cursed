fr fr Test file for pointer functionality

slay main() {
    fr fr Test basic pointer creation and dereferencing
    sus x normie = 42;
    sus ptr = @x;
    sus y = @ptr;
    
    fr fr Test that y equals x via the pointer
    lowkey y == 42 {
        yolo 1;
    } highkey {
        yolo 0;
    }
    
    fr fr Test pointer modification
    @ptr = 100;
    lowkey x == 100 {
        yolo 1;
    } highkey {
        yolo 0;
    }
    
    fr fr Test pointer to struct
    sus person = Person{name: "John", age: 30};
    sus person_ptr = @person;
    @person_ptr.age = 31;
    
    lowkey person.age == 31 {
        yolo 1;
    } highkey {
        yolo 0;
    }
    
    fr fr Test nil pointer
    sus nil_ptr = cap;
    lowkey nil_ptr == cap {
        yolo 1;
    } highkey {
        yolo 0;
    }
    
    yolo 0;
}

be_like Person squad {
    name tea;
    age normie;
}