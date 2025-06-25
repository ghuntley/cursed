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

slay main() {
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
        "debug": true,
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
            success: false,
            error:   "Invalid features config",
        }
    }

    sus hasAuth := false
    lowkey _, feature := range features {
        lowkey feature == "auth" {
            hasAuth = true
            bet
        }
    }

    lowkey !hasAuth {
        vibe Result{
            success: false,
            error:   "Auth feature required",
        }
    }

    fr fr Process the people data
    lowkey person := range people {
        lowkey person.age < 18 {
            vibe Result{
                success: false,
                error:   "Underage person detected",
            }
        }
    }

    vibe Result{
        success: true,
        error:   "",
    }
}

sus Result struct {
    success bool
    error   string
}
