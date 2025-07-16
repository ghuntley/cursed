# Type switch test for CURSED

sus value normie = 42

# Basic type switch with variable binding
sus result tea = typecheck value is {
    normie n -> "Found integer: " + n.toString()
    tea s -> "Found string: " + s
    lit b -> if b then "Found true" else "Found false"
    _ -> "Unknown type"
}

vibez.spill(result)

# Type switch with interface matching
collab NumberLike {
    slay toString() tea
}

sus another_value normie = 123

sus interface_result tea = typecheck another_value is {
    NumberLike num -> num.toString()
    tea s -> s
    _ -> "Not number-like"
}

vibez.spill(interface_result)
