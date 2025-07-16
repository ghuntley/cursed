# Simple type switch test

sus value normie = 42

sus result tea = typecheck value is {
    normie -> "It's an integer"
    _ -> "Unknown type"
}

vibez.spill(result)
