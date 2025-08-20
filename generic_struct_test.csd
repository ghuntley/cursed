fr fr Generic struct field test

fr fr Define generic struct
squad Container[T] {
    value T  
}

fr fr Create instance - this should now be handled by our generic resolution
sus container Container[drip] = Container { value: 123 }
vibez.spill("Container instance created")
vibez.spill("Container:", container)
