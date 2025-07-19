// Test type switch variable binding scope management
sus x normie = 42
sus result tea = typecheck (x) {
    bound_int normie -> {
        vibez.spill(bound_int)  // Should be able to access bound_int with type normie
        "integer case"
    }
    bound_str tea -> {
        vibez.spill(bound_str)  // Should be able to access bound_str with type tea  
        "string case"
    }
    _ -> "unknown case"
}
vibez.spill(result)
