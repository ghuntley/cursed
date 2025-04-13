vibe main

yeet (
  "fmt"
  tea "strings"
  "math"
)

be_like Person squad {
  name tea
  age normie
  address @Address  fr fr pointer to Address
}

be_like Address squad {
  street tea
  city tea
  zipcode tea
}

be_like Container[T] squad {
  items []T
  capacity normie
}

facts (
  PI = 3.14159
  MAX_AGE = 120
)

slay greet(name tea) tea {
  yolo "Hello, " + name + "!"
}

slay createPerson(name tea, age normie) Person {
  yolo Person{
    name: name,
    age: age,
    address: cap
  }
}

slay process[T](items []T, fn slay(T) T) []T {
  sus result []T = crew{}
  bestie _, item := flex items {
    result = append(result, fn(item))
  }
  yolo result
}

slay main() {
  sus name tea = "World"
  sus greeting tea = greet(name)
  fmt.Println(greeting)
  
  sus person Person = createPerson("Alice", 30)
  fmt.Println(person.name, "is", person.age, "years old")
  
  sus numbers []normie = crew{1, 2, 3, 4, 5}
  sus doubled []normie = process(numbers, slay(n normie) normie {
    yolo n * 2
  })
  
  fmt.Println("Doubled numbers:")
  bestie i := 0; i < len(doubled); i++ {
    fmt.Println(doubled[i])
  }
  
  fr fr Using a channel
  sus ch dm<tea> = make(dm<tea>)
  
  stan slay() {
    ch <- "Hello from goroutine!"
  }()
  
  sus msg tea = <-ch
  fmt.Println(msg)
  
  vibe_check person.age {
    mood 18:
      fmt.Println("Just became an adult")
    mood 21:
      fmt.Println("Can legally drink in the US")
    mood 30, 40, 50:
      fmt.Println("Milestone age")
    basic:
      fmt.Println("Regular age")
  }
}