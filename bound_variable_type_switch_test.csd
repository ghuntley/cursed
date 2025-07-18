# Type switch with bound variable
sus x normie = 100
sus result normie = typecheck x is {
    normie value -> value * 2
    _ -> 0
}

vibez.spill(result)
