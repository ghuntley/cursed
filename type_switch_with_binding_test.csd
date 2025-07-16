# Type switch test with variable binding

sus value normie = 42

sus result tea = typecheck value is {
    normie n -> "Integer value: " + n.toString()
    tea s -> "String value: " + s
    _ -> "Unknown type"
}

vibez.spill(result)
