# Simple type switch test
sus x normie = 42
sus result tea = typecheck x is {
    normie -> "integer"
    tea -> "string"
    _ -> "other"
}

vibez.spill(result)
