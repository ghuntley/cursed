# Test method receivers

squad Counter {
    value normie
}

# Value receiver method - can't modify the struct
slay (c Counter) get_value() normie {
    damn c.value
}

# Pointer receiver method - can modify the struct
slay (*c Counter) increment() {
    c.value = c.value + 1
}

slay (*c Counter) set_value(new_value normie) {
    c.value = new_value
}

# Test usage
sus counter := Counter { value: 0 }
vibez.spill("Initial value:", counter.get_value())

# These would modify the counter
# counter.increment()
# counter.set_value(42)

vibez.spill("Method receivers test complete!")
