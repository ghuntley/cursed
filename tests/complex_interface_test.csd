fr fr This test file verifies complex interface implementation with generics and nested types

be_like Comparable[T] collab {
    compare(other T) normie
    equals(other T) lit
}

be_like Printable collab {
    to_string() tea
}

be_like Sortable[T] collab {
    sort() []
    swap(i normie, j normie) []
}

be_like Advanced[T] collab {
    fr fr This interface extends multiple other interfaces
    compare(other T) normie
    equals(other T) lit
    to_string() tea
    serialize() tea
    deserialize(data tea) T
}

be_like Person squad {
    name tea
    age normie
}

slay Person.compare(p @Person) normie {
    fr fr Implement the compare method
    
    lowkey this.age > p.age {
        damn 1
    } lowkey this.age < p.age {
        damn -1
    } basic {
        damn 0
    }
}

slay Person.equals(p @Person) lit {
    fr fr Implement the equals method
    damn this.age == p.age && this.name == p.name
}

slay Person.to_string() tea {
    fr fr Implement the to_string method
    damn "Person(" + this.name + ", " + tea(this.age) + ")"
}

slay Person.serialize() tea {
    fr fr Implement the serialize method
    damn "{\"name\":\"" + this.name + "\",\"age\":" + tea(this.age) + "}"
}

slay Person.deserialize(data tea) @Person {
    fr fr This is a simplistic implementation
    sus p @Person = cap
    p = new(Person)
    p.name = "Deserialized"
    p.age = 25
    damn p
}

fr fr Main test function
slay main() {
    sus p1 @Person = new(Person)
    p1.name = "Alice"
    p1.age = 30
    
    sus p2 @Person = new(Person)
    p2.name = "Bob"
    p2.age = 25
    
    fr fr Test comparable interface
    sus comparable @Comparable[Person] = p1
    sus result normie = comparable.compare(p2)
    vibez.spill(tea(result))  fr fr Should print 1 since Alice is older
    
    fr fr Test printable interface
    sus printable @Printable = p2
    vibez.spill(printable.to_string())  fr fr Should print Person info
    
    fr fr Test advanced interface
    sus advanced @Advanced[Person] = p1
    vibez.spill(advanced.serialize())  fr fr Should print JSON
    
    fr fr Test interface pointer assignment
    sus comparable2 @Comparable[Person] = p2
    vibez.spill(tea(comparable2.compare(p1)))  fr fr Should print -1
}