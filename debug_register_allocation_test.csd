sus test_array drip[] = [1, 2, 3]
sus test_struct squad {
    name tea = "test"
    value drip = 42
}

slay test_channel_ops() lit {
    sus ch drip = dm normie(5)
    ch <- 42
    sus result drip = <-ch
    damn result == 42
}

sus count drip = 0
flex i ina 0..5 {
    count = count + i
}

vibez.spill("Array: " + tea(test_array[0]))
vibez.spill("Struct: " + test_struct.name)
vibez.spill("Channel test: " + tea(test_channel_ops()))
vibez.spill("Loop result: " + tea(count))
