// Complete program for round-trip testing
yeet "mathz"
yeet "stringz"

squad Person {
    spill name tea
    spill age drip
    
    slay greet() tea {
        damn "Hello, I'm " + name + " and I'm " + to_str_drip(age) + " years old"
    }
}

slay main() {
    sus people []Person = [
        Person{name: "Alice", age: 30},
        Person{name: "Bob", age: 25}
    ]
    
    sus i drip = 0
    bestie (i < len(people)) {
        sus greeting tea = people[i].greet()
        vibez.spill(greeting)
        i = i + 1
    }
    
    sus total_age drip = 0
    ready (person) in people {
        total_age = total_age + person.age
    }
    
    sus average normie = to_normie(total_age) / to_normie(len(people))
    vibez.spill("Average age:", average)
}

main()
