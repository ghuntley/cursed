fr fr Test comprehensive CURSED parsing functionality

yeet "testz"

slay test_basic_syntax() lit {
    sus x normie = 42
    sus message tea = "Hello, CURSED!"
    damn based
}

slay test_with_params(name tea, age normie) tea {
    damn name + " is " + age.to_string()
}

squad Person {
    spill name tea
    spill age normie
}

collab Greetable {
    slay greet() tea
}

flex Person => Greetable {
    slay greet() tea {
        damn "Hello, I'm " + self.name
    }
}

slay test_control_flow() {
    lowkey true {
        vibez.spill("If statement works")
    } highkey {
        vibez.spill("Else works too")
    }
    
    bestie i := 0; i < 5; i = i + 1 {
        vibez.spill("For loop iteration: " + i.to_string())
    }
    
    sus count normie = 0
    periodt count < 3 {
        vibez.spill("While loop: " + count.to_string())
        count = count + 1
    }
}

slay test_advanced_features() {
    sus numbers []normie = [1, 2, 3, 4, 5]
    sus person Person = Person{name: "Alice", age: 30}
    
    sus result normie = match numbers[0] {
        1 => 100,
        2 => 200,
        _ => 0
    }
    
    vibez.spill("Advanced features test complete")
}

slay test_error_handling() {
    fam {
        yikes CustomError = "Something went wrong"
        shook risky_operation()
    } catch(err) {
        vibez.spill("Caught error: " + err.message)
    }
}

slay test_concurrency() {
    sus ch = make_channel<normie>()
    
    stan {
        dm_send(ch, 42)
    }
    
    ready {
        mood value := dm_recv(ch):
            vibez.spill("Received: " + value.to_string())
        basic:
            vibez.spill("No value received")
    }
}

test_start("Comprehensive CURSED Parser Test")
test_basic_syntax()
test_control_flow()
test_advanced_features()
test_error_handling()
test_concurrency()
print_test_summary()
