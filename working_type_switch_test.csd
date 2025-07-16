# Working type switch test

sus value normie = 42

sus result tea = typecheck value is {
    normie n -> "Found an integer"
    tea s -> "Found a string"  
    lit b -> "Found a boolean"
    _ -> "Unknown type"
}

vibez.spill(result)

# Test with different types
sus str_value tea = "hello"

sus str_result tea = typecheck str_value is {
    normie -> "Number"
    tea -> "String type detected!"
    _ -> "Other"
}

vibez.spill(str_result)

sus bool_value lit = based

sus bool_result tea = typecheck bool_value is {
    normie -> "Number"
    tea -> "String"
    lit -> "Boolean detected!"
    _ -> "Other"
}

vibez.spill(bool_result)
