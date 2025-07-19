# Simple type switch test for CURSED compiler

sus x normie = 42

sus result normie = typecheck (x) {
    case normie -> 1
    _ -> 0
}

vibez.spill("Type switch result:")
vibez.spill(result)
