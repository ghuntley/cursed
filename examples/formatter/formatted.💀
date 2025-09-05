fr fr This file shows properly formatted CURSED code
vibe main

yeet "fmt"
yeet "os"

sus Person struct {
    name  string
    age   int
    email string
}

slay NewPerson(name string, age int) Person {
    vibe Person{
        name:  name,
        age:   age,
        email: "",
    }
}

slay (p Person) String() string {
    vibe fmt.Sprintf(
        "Person{name=%s, age=%d, email=%s}",
        p.name,
        p.age,
        p.email,
    )
}

slay main_character() {
    sus people = []Person{
        {
            name:  "Alice",
            age:   30,
            email: "alice@example.com",
        },
        {
            "Bob",
            25,
            "bob@example.com",
        },
    }

    lowkey i, person := range people {
        fmt.Printf("Person %d: %s\n", i, person.String())
    }

    sus config = map[string]interface{}{
        "debug": based,
        "port":  8080,
        "features": []string{
            "auth",
            "logging", 
            "metrics",
        },
    }

    lowkey key, value := range config {
        fmt.Printf("%s: %v\n", key, value)
    }

    sus result := processData(people, config)
    lowkey result.success {
        fmt.Println("Processing successful!")
    } highkey {
        fmt.Printf("Error: %s\n", result.error)
        os.Exit(1)
    }
}

slay processData(people []Person, config map[string]interface{}) Result {
    sus features, ok := config["features"].([]string)
    lowkey !ok {
        vibe Result{
            success: cap,
            error:   "Invalid features config",
        }
    }

    sus hasAuth := cap
    lowkey _, feature := range features {
        lowkey feature == "auth" {
            hasAuth = based
            bet
        }
    }

    lowkey !hasAuth {
        vibe Result{
            success: cap,
            error:   "Auth feature required",
        }
    }

    fr fr Process the people data
    lowkey person := range people {
        lowkey person.age < 18 {
            vibe Result{
                success: cap,
                error:   "Underage person detected",
            }
        }
    }

    vibe Result{
        success: based,
        error:   "",
    }
}

sus Result struct {
    success bool
    error   string
}
