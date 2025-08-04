fr fr CURSED Tooling Infrastructure Demo
fr fr This file demonstrates various tooling features

yeet "testz"

fr fr @brief Demonstrates CURSED function documentation
fr fr @param name tea The name to greet
fr fr @param count drip Number of times to greet
fr fr @returns tea A greeting message
fr fr @example
fr fr   sus message = greet_person("Alice", 3)
fr fr   vibez.spill(message)
slay greet_person(name tea, count drip) tea {
    sus result tea = ""
    bestie i := 0; i < count; i = i + 1 {
        result = result + "Hello, " + name + "! "
    }
    damn result
}

fr fr @brief A sample data structure
fr fr @description Represents a person with basic information
squad Person {
    spill name tea
    spill age drip
    spill email tea
}

fr fr @brief Interface for drawable objects
collab Drawable {
    slay draw()
    slay get_area() meal
}

fr fr @brief Implementation of drawable for rectangles
squad Rectangle {
    spill width meal
    spill height meal
}

flex Rectangle => Drawable {
    slay draw() {
        vibez.spill("Drawing rectangle")
    }
    
    slay get_area() meal {
        damn width * height
    }
}

fr fr Test the tooling functionality
slay main() {
    test_start("Tooling Demo Test")
    
    fr fr Test function
    sus greeting = greet_person("CURSED", 2)
    assert_true(greeting.len() > 0)
    
    fr fr Test struct
    sus person = Person{
        name: "Developer",
        age: 25,
        email: "dev@cursed.dev"
    }
    assert_eq_string(person.name, "Developer")
    
    fr fr Test interface implementation
    sus rect = Rectangle{width: 10.0, height: 5.0}
    sus area = rect.get_area()
    assert_eq_float(area, 50.0)
    
    print_test_summary()
}

main()
