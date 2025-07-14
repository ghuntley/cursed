# Simple parser test
interface Simple {
    slay test() normie
}

struct MyStruct[T] {
    value T
}

slay (receiver *MyStruct[T]) get_value() T {
    damn receiver.value
}

vibez.spill("Simple parser test")
