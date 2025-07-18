sus value tea = "hello"
sus result tea = match value {
    "hello" -> "found"
    _ -> "not found"
}
vibez.spill(result)
