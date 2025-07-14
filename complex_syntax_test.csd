# Complex syntax test for parser edge cases
yeet "testz"

# 1. Interface with method receivers and generics
interface Comparable[T] {
    slay compare(a T, b T) normie
    slay (receiver *T) equals(other T) lit
    slay (receiver T) less_than(other T) lit
}

# 2. Generic interface with constraints
interface Container[T: Comparable[T], U: Container[T]] extends Comparable[T] {
    slay (receiver *Container[T]) add(item T) lit
    slay (receiver Container[T]) get(index normie) T where T: Comparable[T]
    slay size() normie
}

# 3. Complex pattern matching with nested types
vibes match value {
    pattern Container[normie] => { vibez.spill("integer container") }
    pattern Container[tea] => { vibez.spill("string container") }
    pattern (x, y) where x > 0 && y < 10 => { vibez.spill("valid range") }
    _ => { vibez.spill("default case") }
}

# 4. Method receivers with generics
struct GenericStruct[T] {
    value T
}

slay (receiver *GenericStruct[T]) set_value(new_value T) {
    receiver.value = new_value
}

slay (receiver GenericStruct[T]) get_value() T {
    damn receiver.value
}

# 5. Complex type constraints and bounds
slay process_data[T: Comparable[T] + Container[T], U: Iterator[T]](
    data T,
    iterator U
) T where T: Clone + Display {
    # Implementation
    damn data
}

# 6. Advanced pattern matching in select statements
ready {
    case <- channel1 where value > 0 => {
        vibez.spill("positive value from channel1")
    }
    case value := <- channel2 => {
        vibes match value {
            pattern x where x % 2 == 0 => vibez.spill("even")
            pattern x where x % 2 == 1 => vibez.spill("odd")
            _ => vibez.spill("unknown")
        }
    }
    basic => {
        vibez.spill("default case")
    }
}

# 7. Error handling with complex patterns
yikes {
    sus result := risky_operation()
    vibes match result {
        Ok(value) => damn value
        Err(error) where error.code == 404 => {
            shook CustomError("not found")
        }
        Err(error) => {
            fam error
        }
    }
}

# 8. Nested generics with method receivers
interface Processor[T, U] {
    slay (receiver *Processor[T, U]) process(input T) U
}

struct DataProcessor[T: Serializable, U: Deserializable] implements Processor[T, U] {
    config ProcessorConfig[T, U]
}

slay (receiver *DataProcessor[T, U]) process(input T) U where T: Clone, U: Default {
    # Complex implementation
    damn U::default()
}

# 9. Complex tuple destructuring with generics
sus (x, y, z) := create_triple[normie](1, 2, 3)
sus (first, ...rest) := create_variadic[tea]("a", "b", "c", "d")

# 10. Advanced interface inheritance with constraints
interface Reader[T] {
    slay read() T
}

interface Writer[T] {
    slay write(data T) lit
}

interface ReadWriter[T: Serializable] extends Reader[T] + Writer[T] 
    where T: Clone + Display {
    slay (receiver *ReadWriter[T]) read_write(data T) T
}

vibez.spill("Complex syntax test complete")
